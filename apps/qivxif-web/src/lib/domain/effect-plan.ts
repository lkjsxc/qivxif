export type EffectPlan =
  | { type: "PersistWorkspace" }
  | { type: "AppendDirtyEvent"; entry: any }
  | { type: "FlushQueue" }
  | { type: "RegisterServiceWorker" }
  | { type: "LoadStorageDiagnostics" }
  | { type: "FetchServerInfo" }
  | { type: "StartSyncActor" };

export type ReducerResult<State> = {
  effects: EffectPlan[];
  state: State;
};

export function result<State>(state: State, effects: EffectPlan[] = []): ReducerResult<State> {
  return { effects, state };
}
