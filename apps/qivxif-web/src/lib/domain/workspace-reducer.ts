import { result, type ReducerResult } from "./effect-plan.ts";
import type { WorkspaceCommand } from "./workspace-command.ts";

export function reduceWorkspace(state: any, command: WorkspaceCommand): ReducerResult<any> {
  if (command.type === "toggleCommandPalette") {
    return result({ ...state, commandPaletteOpen: command.open ?? !state.commandPaletteOpen });
  }
  if (command.type === "refreshDiagnostics") {
    return result(state, [{ type: "LoadStorageDiagnostics" }]);
  }
  if (command.type === "flushSyncQueue") {
    return result(state, [{ type: "FlushQueue" }]);
  }
  if (command.type === "bootstrap") {
    return result(state, [
      { type: "LoadStorageDiagnostics" },
      { type: "RegisterServiceWorker" },
      { type: "FetchServerInfo" },
    ]);
  }
  return result(state);
}
