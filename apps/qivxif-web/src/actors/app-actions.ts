import { createOwner, login } from "../http/client.ts";
import { activePaneId as firstActivePaneId } from "../domain/tile-tree.ts";
import { renderShell } from "../ui/shell.ts";
import { storeAuthPayload } from "./auth-state.ts";
import { addCurrentNodeToBoard, createBoard, linkBoardNodes, moveBoardItem } from "./board-actions.ts";
import { withPaneContext } from "./pane-context.ts";
import { createBlogDraft, publishBlogPost, unpublishBlogPost } from "./publish-actions.ts";
import {
  blockProfile,
  clearSocialEdge,
  createShortPost,
  followProfile,
  muteProfile,
} from "./social-actions.ts";
import { loadLocalState, refreshCurrentNode } from "./state-loader.ts";
import { flushQueue, refreshQueueState } from "./sync.ts";
import { updateTextDraft } from "./tab-drafts.ts";
import { updatePaneScroll } from "./tab-scrolls.ts";
import { createTextNode, openNode, saveText, selectNode } from "./text-actions.ts";
import { movePane } from "./tile-move-actions.ts";
import {
  closePane,
  focusPane,
  maximizePane,
  openProductTab,
  splitPane,
  stackTab,
} from "./tile-actions.ts";

export function actionsFor(root, store, state) {
  return {
    addCurrentNodeToBoard: (context) =>
      run(root, store, state, () => addCurrentNodeToBoard(store, withPaneContext(state, context))),
    blockProfile: (target) => run(root, store, state, () => blockProfile(store, state, target)),
    clearSocialEdge: (edge, kind) => run(root, store, state, () => clearSocialEdge(store, state, edge, kind)),
    closePane: (paneId) => run(root, store, state, () => closePane(store, state, paneId)),
    createBoard: (context) => run(root, store, state, () => createBoard(store, withPaneContext(state, context))),
    createBlogDraft: (title) => run(root, store, state, () => createBlogDraft(store, state, title)),
    createOwner: (name, password) => run(root, store, state, () => createOwnerAccount(store, state, name, password)),
    createShortPost: (body) => run(root, store, state, () => createShortPost(store, state, body)),
    createTextNode: () => run(root, store, state, () => createTextNode(store, state)),
    followProfile: (target) => run(root, store, state, () => followProfile(store, state, target)),
    focusPane: (paneId) => run(root, store, state, () => focusPane(store, state, paneId)),
    linkBoardNodes: () => run(root, store, state, () => linkBoardNodes(store, state)),
    login: (name, password) => run(root, store, state, () => loginUser(store, state, name, password)),
    maximizePane: (paneId) => run(root, store, state, () => maximizePane(store, state, paneId)),
    moveBoardItem: () => run(root, store, state, () => moveBoardItem(store, state)),
    movePane: (source, target, zone) => run(root, store, state, () => movePane(store, state, source, target, zone)),
    muteProfile: (target) => run(root, store, state, () => muteProfile(store, state, target)),
    openNode: (nodeId) => run(root, store, state, () => openNode(store, state, nodeId)),
    openTab: (tabId, paneId) => openTab(root, store, state, tabId, paneId),
    publishBlogPost: (slug, summary) => run(root, store, state, () => publishBlogPost(store, state, slug, summary)),
    saveText: (content, nodeId, paneId) => run(root, store, state, () => saveText(store, state, content, nodeId, paneId)),
    selectNode: (nodeId) => run(root, store, state, () => selectNode(store, state, nodeId)),
    splitPane: (paneId, context) =>
      run(root, store, state, () => splitPane(store, withPaneContext(state, context), paneId)),
    stackTab: (paneId, context) =>
      run(root, store, state, () => stackTab(store, withPaneContext(state, context), paneId)),
    sync: () => run(root, store, state, () => flushQueue(store, state)),
    toggleCommandPalette: (open) => toggleCommandPalette(root, store, state, open),
    toggleTabChooser: (paneId) => toggleTabChooser(root, store, state, paneId),
    unpublishBlogPost: () => run(root, store, state, () => unpublishBlogPost(store, state)),
    updatePaneScroll: (paneId, scrollTop) => updatePaneScroll(store, state, paneId, scrollTop),
    updateTextDraft: (paneId, content) => updateTextDraft(store, state, paneId, content),
  };
}

async function run(root, store, state, action) {
  const actions = actionsFor(root, store, state);
  try {
    await action();
    await loadLocalState(store, state);
    await refreshQueueState(store, state);
    await flushQueue(store, state);
    await loadLocalState(store, state);
    if (state.online && state.currentNodeId) {
      const activeTabId = state.activeTabId;
      await refreshCurrentNode(store, state);
      if (activeTabId !== "editor" && state.activeTabId === "editor") {
        state.activeTabId = activeTabId;
      }
    }
  } catch (error) {
    state.lastError = error.api?.message ?? error.api?.code ?? String(error);
    if (state.activeTabId === "setup") {
      state.setupError = state.lastError;
    }
  }
  renderShell(root, state, actions);
}

async function loginUser(store, state, name, password) {
  await storeAuthPayload(store, state, await login(name, password));
  state.activeTabId = "welcome";
}

async function createOwnerAccount(store, state, name, password) {
  await storeAuthPayload(store, state, await createOwner(name, password));
  state.setupRequired = false;
  state.setupChecked = true;
  state.setupError = "";
  state.activeTabId = "welcome";
}

function openTab(root, store, state, tabId, paneId) {
  if (paneId) {
    run(root, store, state, () => openProductTab(store, state, paneId, tabId));
    return;
  }
  state.activeTabId = tabId;
  state.tabChooserOpen = false;
  renderShell(root, state, actionsFor(root, store, state));
}

function toggleTabChooser(root, store, state, paneId = "") {
  const targetPaneId = paneId || chooserPaneId(state);
  const samePane = state.tabChooserOpen && state.tabChooserPaneId === targetPaneId;
  state.tabChooserOpen = !samePane;
  state.tabChooserPaneId = state.tabChooserOpen ? targetPaneId : "";
  renderShell(root, state, actionsFor(root, store, state));
}

function toggleCommandPalette(root, store, state, open) {
  state.commandPaletteOpen = typeof open === "boolean" ? open : !state.commandPaletteOpen;
  renderShell(root, state, actionsFor(root, store, state));
}

function chooserPaneId(state) {
  return state.activePaneId || firstActivePaneId(state.layout?.root) || localPaneId(state);
}

function localPaneId(state) {
  const panel = state.setupRequired ? "setup" : state.activeTabId;
  return `local_${paneKindForPanel(panel)}`;
}

function paneKindForPanel(panel) {
  const kinds = {
    board: "graph_board",
    diagnostics: "diagnostics",
    editor: "text_editor",
    graph: "graph_node",
    history: "history",
    login: "login",
    publish: "publishing",
    settings: "settings",
    setup: "setup",
    social: "social_feed",
    sync: "sync_status",
    welcome: "welcome",
  };
  return kinds[panel] ?? "welcome";
}
