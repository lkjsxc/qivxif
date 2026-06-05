export type ResourceClass =
  | "dirty_event"
  | "accepted_event"
  | "text_snapshot"
  | "tab_snapshot"
  | "tile_layout"
  | "graph_index"
  | "graph_view_cache"
  | "media_original"
  | "media_chunk"
  | "media_thumbnail"
  | "service_worker_asset"
  | "feed_window"
  | "profile_cache"
  | "sync_cursor"
  | "upload_session"
  | "download_session"
  | "preview_cache";

export type ResourceEntry = {
  id: string;
  className: ResourceClass;
  bytes: number;
  protected: boolean;
  reason: string;
};

export type ResourcePlan = {
  entries: ResourceEntry[];
  actions: string[];
  protectedBytes: number;
  prunableBytes: number;
  summary: string;
};

export function resourceSnapshot(state: any) {
  return {
    acceptedCount: Number(state.acceptedCount ?? 0),
    activePaneId: state.activePaneId ?? "",
    cache: state.storageStatus?.cache ?? { protected: 0, prunable: 0 },
    layoutDirty: Boolean(state.layoutDirty),
    mediaNodes: (state.nodes ?? []).filter((node) => node.kind === "media_asset"),
    queueEntries: state.queueEntries ?? [],
    serviceWorkerReady: Boolean(state.serviceWorkerReady),
    tabDrafts: state.tabDrafts ?? {},
    tabScrolls: state.tabScrolls ?? {},
    textSnapshots: state.textSnapshots ?? {},
    usage: Number(state.storageStatus?.usage ?? 0),
    quota: Number(state.storageStatus?.quota ?? 0),
  };
}

export function planResources(snapshot: any): ResourcePlan {
  const entries = [
    ...queueResources(snapshot.queueEntries),
    ...tabResources(snapshot),
    ...textResources(snapshot.textSnapshots),
    ...mediaResources(snapshot.mediaNodes),
    layoutResource(snapshot),
    serviceWorkerResource(snapshot.serviceWorkerReady),
    cacheResource("cache:protected", "graph_view_cache", snapshot.cache.protected, true, "cache protected"),
    cacheResource("cache:prunable", "preview_cache", snapshot.cache.prunable, false, "cache prunable"),
  ].filter(Boolean) as ResourceEntry[];
  const protectedBytes = sum(entries.filter((entry) => entry.protected));
  const prunableBytes = sum(entries.filter((entry) => !entry.protected));
  const actions = chooseActions(snapshot, prunableBytes);
  return {
    actions,
    entries,
    protectedBytes,
    prunableBytes,
    summary: `${entries.length} resources · ${actions.length} planned actions`,
  };
}

function queueResources(entries: any[]): ResourceEntry[] {
  return entries.map((entry) => ({
    bytes: jsonBytes(entry),
    className: entry.status === "accepted" ? "accepted_event" : "dirty_event",
    id: entry.id,
    protected: entry.status !== "accepted",
    reason: entry.status !== "accepted" ? "local event not accepted" : "accepted event evidence",
  }));
}

function tabResources(snapshot: any): ResourceEntry[] {
  const draftEntries = Object.entries(snapshot.tabDrafts).map(([paneId, content]) => ({
    bytes: String(content ?? "").length,
    className: "tab_snapshot" as ResourceClass,
    id: `draft:${paneId}`,
    protected: paneId === snapshot.activePaneId,
    reason: paneId === snapshot.activePaneId ? "active tab draft" : "inactive tab draft",
  }));
  const scrollEntries = Object.keys(snapshot.tabScrolls).map((paneId) => ({
    bytes: 32,
    className: "tab_snapshot" as ResourceClass,
    id: `scroll:${paneId}`,
    protected: paneId === snapshot.activePaneId,
    reason: paneId === snapshot.activePaneId ? "active tab scroll" : "inactive tab scroll",
  }));
  return [...draftEntries, ...scrollEntries];
}

function textResources(snapshots: Record<string, any>): ResourceEntry[] {
  return Object.entries(snapshots).map(([nodeId, snapshot]) => ({
    bytes: jsonBytes(snapshot),
    className: "text_snapshot",
    id: `text:${nodeId}`,
    protected: Boolean(snapshot?.dirty),
    reason: snapshot?.dirty ? "dirty editor snapshot" : "accepted text snapshot",
  }));
}

function mediaResources(nodes: any[]): ResourceEntry[] {
  return nodes.map((node) => ({
    bytes: Number(node.metadata_map?.size ?? 0),
    className: "media_original",
    id: node.id,
    protected: Boolean(node.dirty || node.metadata_map?.pinned),
    reason: node.dirty ? "dirty media metadata" : node.metadata_map?.pinned ? "user pinned" : "owned media metadata",
  }));
}

function layoutResource(snapshot: any): ResourceEntry {
  return {
    bytes: 512,
    className: "tile_layout",
    id: "tile_model",
    protected: true,
    reason: snapshot.layoutDirty ? "dirty layout" : "active workspace layout",
  };
}

function serviceWorkerResource(ready: boolean): ResourceEntry {
  return {
    bytes: 0,
    className: "service_worker_asset",
    id: "service-worker",
    protected: ready,
    reason: ready ? "offline shell ready" : "offline shell pending",
  };
}

function cacheResource(id: string, className: ResourceClass, bytes: number, protectedEntry: boolean, reason: string) {
  return { bytes: Number(bytes ?? 0), className, id, protected: protectedEntry, reason };
}

function chooseActions(snapshot: any, prunableBytes: number): string[] {
  const pressure = snapshot.quota > 0 && snapshot.usage / snapshot.quota > 0.8;
  const actions = [];
  if (pressure && prunableBytes > 0) actions.push("evict prunable cache");
  if ((snapshot.queueEntries ?? []).some((entry) => entry.status === "dirty")) actions.push("flush dirty events");
  if (!snapshot.serviceWorkerReady) actions.push("check service worker assets");
  return actions;
}

function jsonBytes(value: any) {
  return JSON.stringify(value ?? {}).length;
}

function sum(entries: ResourceEntry[]) {
  return entries.reduce((total, entry) => total + Number(entry.bytes ?? 0), 0);
}
