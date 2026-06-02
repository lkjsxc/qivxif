import { writable } from "svelte/store";
import { initialWorkspaceState } from "../domain/workspace-state.ts";
import type { createController } from "./controller.ts";

export type WorkspaceController = Awaited<ReturnType<typeof createController>>;
export type WorkspaceState = ReturnType<WorkspaceController["state"]>;
export type WorkspaceActions = Record<string, (...args: never[]) => unknown>;

export const workspaceState = writable<WorkspaceState>(initialWorkspaceState());
export const workspaceActions = writable<WorkspaceActions>({});

export function bindController(controller: WorkspaceController) {
  return controller.subscribe((state, actions) => {
    workspaceState.set(state);
    workspaceActions.set(actions as WorkspaceActions);
  });
}
