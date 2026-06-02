/** @typedef {import("./workspace-state.ts").ReturnType<typeof import("./workspace-state.ts").initialWorkspaceState>} WorkspaceState */

/**
 * @typedef {(
 *   | { type: "focusPane"; paneId: string }
 *   | { type: "splitPane"; paneId: string; direction?: string }
 *   | { type: "stackTab"; paneId: string }
 *   | { type: "closePane"; paneId: string }
 *   | { type: "maximizePane"; paneId: string }
 *   | { type: "movePane"; source: string; target: string; zone: string }
 *   | { type: "resizeSplit"; paneId: string; sizes: number[] }
 *   | { type: "openTab"; tabId: string; paneId: string }
 *   | { type: "toggleTabChooser"; paneId: string }
 *   | { type: "toggleCommandPalette"; open: boolean }
 *   | { type: "createOwner"; name: string; password: string }
 *   | { type: "login"; name: string; password: string }
 *   | { type: "sync" }
 *   | { type: "updateTextDraft"; paneId: string; content: string }
 *   | { type: "updatePaneScroll"; paneId: string; scrollTop: number }
 * )} WorkspaceCommand
 */

export {};
