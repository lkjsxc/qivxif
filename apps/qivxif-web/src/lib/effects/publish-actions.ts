import { generateId } from "../ids.ts";
import { isSyncableLayout } from "../domain/tile-layout-validation.ts";
import { replaceTabInLayout } from "../domain/tile-tab-update.ts";
import { reserveActorSeq } from "./actor-seq.ts";
import {
  blogPostCreateEntry,
  nodeCreateEntry,
  publishPostEntry,
  textRestoreEntry,
  unpublishPostEntry,
} from "./local-events.ts";
import { queueLayout } from "./tile-helpers.ts";

export async function createBlogDraft(store, state, title) {
  requireAuth(state);
  const safeTitle = title.trim() || "Untitled post";
  const bodySeq = await reserveActorSeq(store);
  const body = nodeCreateEntry(bodySeq, "text", { title: `${safeTitle} body` });
  const docId = generateId("txt");
  const content = `# ${safeTitle}\n\n`;
  const textSeq = await reserveActorSeq(store);
  const restored = textRestoreEntry(textSeq, body.node.id, docId, state.auth.user.actor_id, content);
  const postSeq = await reserveActorSeq(store);
  const post = blogPostCreateEntry(postSeq, body.node.id, safeTitle);

  await store.put("events", body.entry);
  await store.put("nodes", body.node);
  await store.put("sync_cursors", { id: `text_doc:${body.node.id}`, doc_id: docId });
  await store.put("events", restored.entry);
  await store.put("text_snapshots", {
    dirty: true,
    id: body.node.id,
    state: { content },
  });
  await store.put("events", post.entry);
  await store.put("nodes", post.node);
  await store.put("tile_layout", { id: "current_node", node_id: body.node.id });
  await store.put("tile_layout", { id: "current_blog_post", node_id: post.node.id });
  state.currentNodeId = body.node.id;
  state.currentBlogPostId = post.node.id;
  state.currentBlogPost = post.node;
  state.activeTabId = "editor";
  state.text = content;
  await convertActivePaneToEditor(store, state, body.node.id, `${safeTitle} body`);
}

async function convertActivePaneToEditor(store, state, bodyNodeId, title) {
  const model = await store.get("tile_layout", "tile_model");
  if (!state.activePaneId || !isSyncableLayout(model?.layout)) {
    return;
  }
  const next = replaceTabInLayout(model.layout, state.activePaneId, {
    pane_kind: "text_editor",
    target_node_id: bodyNodeId,
    title,
  });
  await queueLayout(store, state, model.layout_node_id, next);
}

export async function publishBlogPost(store, state, slug, summary) {
  requireAuth(state);
  const safeSlug = slug.trim();
  if (!safeSlug) {
    throw new Error("slug is required");
  }
  const postId = await currentPostId(store, state);
  const actorSeq = await reserveActorSeq(store);
  const queued = publishPostEntry(actorSeq, postId, safeSlug, summary.trim());
  await store.put("events", queued.entry);
  await store.put("sync_cursors", {
    id: "last_public_route",
    path: `/@${state.auth.user.name}/${safeSlug}`,
  });
}

export async function unpublishBlogPost(store, state) {
  requireAuth(state);
  const postId = await currentPostId(store, state);
  const actorSeq = await reserveActorSeq(store);
  const queued = unpublishPostEntry(actorSeq, postId, "browser command");
  await store.put("events", queued.entry);
}

async function currentPostId(store, state) {
  if (state.currentBlogPostId) {
    return state.currentBlogPostId;
  }
  const current = state.currentNodeId ? await store.get("nodes", state.currentNodeId) : null;
  if (current?.kind === "blog_post") {
    return current.id;
  }
  throw new Error("select a blog draft first");
}

function requireAuth(state) {
  if (!state.auth?.user?.actor_id) {
    throw new Error("login is required");
  }
}
