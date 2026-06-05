import { reserveActorSeq } from "./actor-seq.ts";
import { edgeCreateEntry, nodeCreateEntry } from "./local-events.ts";

const CHUNK_SIZE = 1024 * 1024;

export async function importMediaFile(store, state, file: File) {
  requireAuth(state);
  const chunks = await writeChunks(file);
  const contentHash = await digestText(chunks.map((chunk) => chunk.hash).join(""));
  const created = nodeCreateEntry(await reserveActorSeq(store), "media_asset", {
    content_hash: contentHash,
    filename: file.name,
    mime_type: file.type || "application/octet-stream",
    processing_state: "local",
    size: String(file.size),
    title: file.name,
  });
  const asset = {
    id: created.node.id,
    chunks,
    content_hash: contentHash,
    filename: file.name,
    mime_type: file.type || "application/octet-stream",
    size: file.size,
  };
  await store.put("events", created.entry);
  await store.put("nodes", created.node);
  await store.put("media_assets", asset);
  for (const chunk of chunks) await store.put("media_chunks", { ...chunk, asset_id: created.node.id });
  state.currentMediaAssetId = created.node.id;
  state.activeTabId = "media";
  if (state.currentNodeId) await attachMediaToNode(store, state, created.node.id, state.currentNodeId);
}

export async function attachMediaToNode(store, state, assetId = state.currentMediaAssetId, nodeId = state.currentNodeId) {
  requireAuth(state);
  if (!assetId || !nodeId) throw new Error("media asset and target node are required");
  const edge = edgeCreateEntry(await reserveActorSeq(store), nodeId, assetId, "media_attachment", {
    attachment_kind: "reference",
  });
  await store.put("events", edge.entry);
  await store.put("edges", edge.edge);
}

async function writeChunks(file: File) {
  const root = await navigator.storage.getDirectory();
  const dir = await root.getDirectoryHandle("qivxif-media", { create: true });
  const chunks = [];
  for (let offset = 0, index = 0; offset < file.size; offset += CHUNK_SIZE, index += 1) {
    const bytes = await file.slice(offset, offset + CHUNK_SIZE).arrayBuffer();
    const hash = await digestBytes(bytes);
    await writeChunk(dir, hash, bytes);
    chunks.push({ hash, id: `${file.name}:${index}`, index, size: bytes.byteLength, stored: true });
  }
  return chunks;
}

async function writeChunk(dir: FileSystemDirectoryHandle, hash: string, bytes: ArrayBuffer) {
  const handle = await dir.getFileHandle(hash, { create: true });
  const writable = await handle.createWritable();
  await writable.write(bytes);
  await writable.close();
}

async function digestBytes(bytes: ArrayBuffer) {
  const digest = await crypto.subtle.digest("SHA-256", bytes);
  return hex(digest);
}

async function digestText(text: string) {
  return digestBytes(new TextEncoder().encode(text));
}

function hex(bytes: ArrayBuffer) {
  return [...new Uint8Array(bytes)].map((byte) => byte.toString(16).padStart(2, "0")).join("");
}

function requireAuth(state) {
  if (!state.auth?.user?.actor_id) throw new Error("login is required");
}
