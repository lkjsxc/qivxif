const NODE_ID = /^nod_[0-9a-f]{64}$/;
const EVENT_ID = /^evt_[0-9a-f]{64}$/;
const U16_MAX = 65535;

export function isNodeId(value: unknown): value is string {
  return typeof value === "string" && NODE_ID.test(value);
}

export function isEventId(value: unknown): value is string {
  return typeof value === "string" && EVENT_ID.test(value);
}

export function isSyncableTileLayoutRequest(request: unknown): boolean {
  if (!isRecord(request)) {
    return false;
  }
  return (
    isEventId(request.event_id) &&
    Number.isSafeInteger(request.actor_seq) &&
    Number(request.actor_seq) >= 0 &&
    isNodeId(request.layout_node_id) &&
    isSyncableLayout(request.layout)
  );
}

export function isSyncableLayout(layout: unknown): boolean {
  if (!isRecord(layout)) {
    return false;
  }
  return isOptionalNodeId(layout.maximized_pane_id) && isSyncableTile(layout.root);
}

function isSyncableTile(tile: unknown): boolean {
  if (!isRecord(tile)) {
    return false;
  }
  if (tile.kind === "stack") {
    return isSyncableStack(tile);
  }
  if (tile.kind === "split") {
    return isSyncableSplit(tile);
  }
  return false;
}

function isSyncableStack(tile: Record<string, unknown>): boolean {
  return (
    Array.isArray(tile.tabs) &&
    tile.tabs.length > 0 &&
    Number.isSafeInteger(tile.active) &&
    Number(tile.active) >= 0 &&
    tile.tabs.every(isSyncableTab)
  );
}

function isSyncableSplit(tile: Record<string, unknown>): boolean {
  return (
    (tile.axis === "row" || tile.axis === "column") &&
    Array.isArray(tile.children) &&
    tile.children.length > 0 &&
    isSyncableSizes(tile.sizes, tile.children.length) &&
    tile.children.every(isSyncableTile)
  );
}

function isSyncableTab(tab: unknown): boolean {
  if (!isRecord(tab)) {
    return false;
  }
  return (
    isNodeId(tab.pane_node_id) &&
    isOptionalNodeId(tab.target_node_id) &&
    typeof tab.pane_kind === "string" &&
    typeof tab.title === "string"
  );
}

function isSyncableSizes(sizes: unknown, count: number): boolean {
  return (
    Array.isArray(sizes) &&
    sizes.length === count &&
    sizes.every((size) => Number.isInteger(size) && Number(size) >= 0 && Number(size) <= U16_MAX)
  );
}

function isOptionalNodeId(value: unknown): boolean {
  return value == null || isNodeId(value);
}

function isRecord(value: unknown): value is Record<string, unknown> {
  return Boolean(value) && typeof value === "object";
}
