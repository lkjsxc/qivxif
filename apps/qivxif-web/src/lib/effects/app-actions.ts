import { createOwner, login } from "./api-client.ts";
import { activePaneId as firstActivePaneId } from "../domain/tile-tree.ts";
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
import { localStoreDiagnostics, saveLocalWorkspace } from "./indexed-db.ts";
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
  openNewTabChooser,
  openProductTab,
  resizeSplit,
  splitPane,
  stackTab,
} from "./tile-actions.ts";

export function actionsFor(store, state, notify = () => {}) {
  const run = (action) => runAction(store, state, notify, action);

  return {
    addCurrentNodeToBoard: (context) => run(() => addCurrentNodeToBoard(store, withPaneContext(state, context))),
    blockProfile: (target) => run(() => blockProfile(store, state, target)),
    clearSocialEdge: (edge, kind) => run(() => clearSocialEdge(store, state, edge, kind)),
    closePane: (paneId) => run(() => closePane(store, state, paneId)),
    createBoard: (context) => run(() => createBoard(store, withPaneContext(state, context))),
    createBlogDraft: (title) => run(() => createBlogDraft(store, state, title)),
    createOwner: (name, password) => run(() => createOwnerAccount(store, state, name, password)),
    createShortPost: (body) => run(() => createShortPost(store, state, body)),
    createTextNode: () => run(() => createTextNode(store, state)),
    followProfile: (target) => run(() => followProfile(store, state, target)),
    focusPane: (paneId) => run(() => focusPane(store, state, paneId)),
    linkBoardNodes: () => run(() => linkBoardNodes(store, state)),
    login: (name, password) => run(() => loginUser(store, state, name, password)),
    maximizePane: (paneId) => run(() => maximizePane(store, state, paneId)),
    moveBoardItem: () => run(() => moveBoardItem(store, state)),
    movePane: (source, target, zone) => run(() => movePane(store, state, source, target, zone)),
    muteProfile: (target) => run(() => muteProfile(store, state, target)),
    openNewTabChooser: (paneId) => run(() => openNewTabChooser(store, state, paneId || chooserPaneId(state))),
    openNode: (nodeId) => run(() => openNode(store, state, nodeId)),
    openTab: (tabId, paneId) => {
      if (paneId) {
        return run(() => openProductTab(store, state, paneId, tabId));
      }
      state.activeTabId = tabId;
      state.tabChooserOpen = false;
      notify();
    },
    publishBlogPost: (slug, summary) => run(() => publishBlogPost(store, state, slug, summary)),
    resizeSplit: (paneId, sizes) => run(() => resizeSplit(store, state, paneId, sizes)),
    saveText: (content, nodeId, paneId) => run(() => saveText(store, state, content, nodeId, paneId)),
    selectNode: (nodeId) => run(() => selectNode(store, state, nodeId)),
    splitPane: (paneId, context: any = {}) => {
      const scoped = withPaneContext(state, context);
      return run(() => splitPane(store, scoped, paneId, context.direction ?? "right"));
    },
    stackTab: (paneId, context) => run(() => stackTab(store, withPaneContext(state, context), paneId)),
    sync: () => run(() => flushQueue(store, state)),
    toggleCommandPalette: (open) => {
      state.commandPaletteOpen = typeof open === "boolean" ? open : !state.commandPaletteOpen;
      notify();
    },
    toggleTabChooser: (paneId = "") => {
      const targetPaneId = paneId || state.activePaneId || chooserPaneId(state);
      return run(() => openNewTabChooser(store, state, targetPaneId));
    },
    unpublishBlogPost: () => run(() => unpublishBlogPost(store, state)),
    updatePaneScroll: (paneId, scrollTop) => {
      updatePaneScroll(store, state, paneId, scrollTop);
    },
    updateTextDraft: (paneId, content) => {
      updateTextDraft(store, state, paneId, content);
      notify();
    },
  };
}

async function runAction(store, state, notify, action) {
  try {
    await action();
    notify();
    await loadLocalState(store, state);
    await refreshQueueState(store, state);
    state.storageStatus = await localStoreDiagnostics(store);
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
    const failure = error as any;
    state.lastError = failure.api?.message ?? failure.api?.code ?? String(error);
    if (state.setupRequired || state.activeTabId === "setup") {
      state.setupError = state.lastError;
    }
  }
  await saveLocalWorkspace(store, state).catch(() => {});
  notify();
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

function chooserPaneId(state) {
  return state.activePaneId || firstActivePaneId(state.layout?.root) || localPaneId(state);
}

function localPaneId(state) {
  const panel = state.setupRequired ? "setup" : state.activeTabId;
  return `local_${paneKindForPanel(panel)}`;
}

function paneKindForPanel(panel) {
  const kinds: Record<string, string> = {
    board: "graph_board",
    diagnostics: "diagnostics",
    editor: "text_editor",
    graph: "graph_node",
    history: "history",
    login: "login",
    "new-tab": "new_tab",
    publish: "publishing",
    settings: "settings",
    setup: "setup",
    social: "social_feed",
    sync: "sync_status",
    welcome: "welcome",
  };
  return kinds[panel] ?? "welcome";
}
