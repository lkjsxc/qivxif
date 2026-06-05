import type { JsonRecord, LocalStore, StorageDiagnostics } from "./types.ts";

export type LocalRepositories = ReturnType<typeof repositoriesFor>;

export function repositoriesFor(store: LocalStore) {
  return {
    cache: {
      recordEntry: (entry: JsonRecord) => store.put("cache_entries", entry),
      recordJournal: (entry: JsonRecord) => store.put("cache_journal", entry),
    },
    diagnostics: { read: (): Promise<StorageDiagnostics> => store.diagnostics() },
    events: {
      append: (entry: JsonRecord) => store.put("events", entry),
      listAll: () => store.all("events"),
      listNonAccepted: async () =>
        (await store.all("events"))
          .filter((entry) => entry.status !== "accepted")
          .sort((left, right) => Number(left.actor_seq ?? 0) - Number(right.actor_seq ?? 0)),
      markAccepted: (eventId: string, payload: any) =>
        store.put("accepted_events", { id: eventId, payload, status: "accepted" }),
      markPending: (eventId: string) => putStatus(store, eventId, "pending_validation"),
      markRejected: (eventId: string, error: any) => putStatus(store, eventId, "rejected", error),
    },
    graph: {
      getNode: (nodeId: string) => store.get("nodes", nodeId),
      listEdges: () => store.all("edges"),
      listNodes: () => store.all("nodes"),
      putEdgeProjection: (edge: JsonRecord) => store.put("edges", edge),
      putNodeProjection: (node: JsonRecord) => store.put("nodes", node),
    },
    inventory: { read: () => store.diagnostics().then((item) => item.inventory) },
    tabs: {
      loadAll: () => store.all("tab_snapshots"),
      saveDraft: (paneId: string, content: string) =>
        store.put("tab_snapshots", { content, id: `draft:${paneId}`, pane_id: paneId }),
      saveScroll: (paneId: string, scrollTop: number) =>
        store.put("tab_snapshots", { id: `scroll:${paneId}`, pane_id: paneId, scrollTop }),
    },
    text: {
      getSnapshot: (nodeId: string) => store.get("text_snapshots", nodeId),
      putSnapshot: (nodeId: string, snapshot: JsonRecord) =>
        store.put("text_snapshots", { ...snapshot, id: nodeId }),
    },
    tile: {
      getLayout: () => store.get("tile_layout", "tile_model"),
      putLayout: (record: JsonRecord) => store.put("tile_layout", record),
      setActiveGraphMap: (nodeId: string) => store.put("tile_layout", { id: "active_graph_map", node_id: nodeId }),
      setCurrentBlogPost: (nodeId: string) =>
        store.put("tile_layout", { id: "current_blog_post", node_id: nodeId }),
    },
    workspace: {
      load: () => store.get("local_workspace", "workspace"),
      save: (snapshot: JsonRecord) => store.put("local_workspace", { ...snapshot, id: "workspace" }),
    },
  };
}

async function putStatus(store: LocalStore, eventId: string, status: string, error?: any) {
  const entry = await store.get("events", eventId);
  if (entry) await store.put("events", { ...entry, last_error: error ?? entry.last_error, status });
}
