export type WorkspaceCommand =
  | { type: "bootstrap" }
  | { type: "focusPane"; paneId: string }
  | { type: "openNewTab"; paneId?: string }
  | { type: "convertNewTab"; paneId: string; tabId: string }
  | { type: "closePane"; paneId: string }
  | { type: "splitPane"; paneId: string; context?: any; direction?: string }
  | { type: "stackTab"; context?: any; paneId: string }
  | { type: "maximizePane"; paneId: string }
  | { type: "restorePane"; paneId: string }
  | { type: "movePane"; source: string; target: string; zone: string }
  | { type: "reorderTab"; source: string; target: string; zone: string }
  | { type: "resizeSplit"; paneId: string; sizes: number[] }
  | { type: "createOwner"; name: string; password: string }
  | { type: "login"; name: string; password: string }
  | { type: "logout" }
  | { type: "createTextNode" }
  | { type: "openNode"; nodeId: string }
  | { type: "saveTextDraft"; content: string; paneId: string }
  | { type: "saveText"; content: string; nodeId?: string; paneId?: string }
  | { type: "updatePaneScroll"; paneId: string; scrollTop: number }
  | { type: "createGraphMap"; context?: any }
  | { type: "addCurrentNodeToGraphMap"; context?: any }
  | { type: "moveGraphMapItem" }
  | { type: "linkGraphMapNodes" }
  | { type: "createShortPost"; body: string }
  | { type: "followProfile"; target: string }
  | { type: "clearSocialEdge"; edge: string; kind: string }
  | { type: "createBlogDraft"; title: string }
  | { type: "publishBlogPost"; slug: string; summary: string }
  | { type: "unpublishBlogPost" }
  | { type: "flushSyncQueue" }
  | { type: "refreshDiagnostics" }
  | { type: "toggleCommandPalette"; open?: boolean }
  | { type: "openTab"; paneId?: string; tabId: string }
  | { type: "toggleTabChooser"; paneId?: string }
  | { type: "sync" }
  | { type: "updateTextDraft"; paneId: string; content: string };
