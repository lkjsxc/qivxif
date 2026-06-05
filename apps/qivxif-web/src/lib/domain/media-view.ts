export function mediaAssets(state) {
  const byId = new Map((state.mediaAssets ?? []).map((asset: any) => [asset.id, asset]));
  for (const node of state.nodes ?? []) {
    if (node.kind !== "media_asset" || byId.has(node.id)) continue;
    byId.set(node.id, { id: node.id, ...node.metadata_map, dirty: node.dirty });
  }
  return [...byId.values()].sort((left: any, right: any) =>
    String(left.filename).localeCompare(String(right.filename)),
  );
}

export function mediaAttachments(state, assetId: string) {
  return (state.edges ?? []).filter((edge) => edge.kind === "media_attachment" && edge.to_node === assetId);
}

export function formatBytes(value: unknown) {
  const bytes = Number(value ?? 0);
  if (!Number.isFinite(bytes)) return "0 B";
  if (bytes < 1024) return `${bytes} B`;
  if (bytes < 1024 * 1024) return `${Math.round(bytes / 1024)} KiB`;
  return `${Math.round(bytes / 1024 / 1024)} MiB`;
}
