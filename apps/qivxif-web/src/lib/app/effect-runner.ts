import type { EffectPlan } from "../domain/effect-plan.ts";
import type { AppPorts } from "./ports.ts";

export async function runEffectPlans(ports: AppPorts, state: any, effects: EffectPlan[]) {
  for (const effect of effects) {
    await runEffectPlan(ports, state, effect);
  }
}

async function runEffectPlan(ports: AppPorts, state: any, effect: EffectPlan) {
  if (effect.type === "LoadStorageDiagnostics") {
    state.storageStatus = await ports.storage.diagnostics();
  } else if (effect.type === "RegisterServiceWorker") {
    await ports.serviceWorker.register();
    state.serviceWorkerReady = true;
  } else if (effect.type === "FetchServerInfo") {
    const payload = await ports.server.info();
    state.capabilities = payload.capabilities?.capabilities ?? [];
    state.online = true;
  } else if (effect.type === "FlushQueue") {
    await ports.sync.flush(state);
  } else if (effect.type === "PersistWorkspace") {
    await ports.storage.store.put("local_workspace", { id: "workspace", layout: state.layout });
  }
}
