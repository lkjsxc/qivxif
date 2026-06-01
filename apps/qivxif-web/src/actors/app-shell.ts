import { createOwner, login, serverInfo, setupStatus } from "../http/client.ts";
import { openLocalStore } from "../store/indexed-db.ts";
import { renderShell } from "../ui/shell.ts";
import { initialState } from "./app-state.ts";
import { storeAuthPayload } from "./auth-state.ts";
import { addCurrentNodeToBoard, createBoard, linkBoardNodes, moveBoardItem } from "./board-actions.ts";
import { installKeyboardShortcuts } from "./keyboard-shortcuts.ts";
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

export async function startAppShell(root) {
  if (!root) {
    return;
  }
  const state = initialState();
  renderShell(root, state, {});
  await refreshSetupStatus(state);
  const store = await openLocalStore();
  installKeyboardShortcuts(() => actionsFor(root, store, state), state);
  await loadLocalState(store, state);
  await refreshQueueState(store, state);
  selectInitialTab(state);
  await refreshServerInfo(state);
  await registerServiceWorker(state);
  if (state.auth && !state.setupRequired) {
    await flushQueue(store, state);
  }
  renderShell(root, state, actionsFor(root, store, state));
}

async function refreshSetupStatus(state) {
  try {
    const status = await setupStatus();
    state.setupChecked = true;
    state.setupRequired = Boolean(status.required);
    state.setupError = "";
    if (state.setupRequired) {
      state.activeTabId = "setup";
    }
  } catch (error) {
    state.online = false;
    state.setupError = String(error);
    state.lastError = String(error);
  }
}

async function refreshServerInfo(state) {
  try {
    const payload = await serverInfo();
    state.capabilities = payload.capabilities?.capabilities ?? [];
    state.online = true;
  } catch (error) {
    state.online = false;
    state.lastError = String(error);
  }
}

async function registerServiceWorker(state) {
  if (!("serviceWorker" in navigator)) {
    return;
  }
  try {
    await navigator.serviceWorker.register("/service-worker.js");
  } catch (error) {
    state.lastError = String(error);
  }
}

function actionsFor(root, store, state) {
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

function selectInitialTab(state) {
  if (state.setupRequired) {
    state.activeTabId = "setup";
  } else if (!state.auth) {
    state.activeTabId = "login";
  } else if (state.activeBoardId) {
    state.activeTabId = "board";
  }
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
  const samePane = state.tabChooserOpen && state.tabChooserPaneId === paneId;
  state.tabChooserOpen = !samePane;
  state.tabChooserPaneId = state.tabChooserOpen ? paneId : "";
  renderShell(root, state, actionsFor(root, store, state));
}

function toggleCommandPalette(root, store, state, open) {
  state.commandPaletteOpen = typeof open === "boolean" ? open : !state.commandPaletteOpen;
  renderShell(root, state, actionsFor(root, store, state));
}
