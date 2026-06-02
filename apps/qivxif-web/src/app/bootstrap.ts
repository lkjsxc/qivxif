import { renderShell } from "../ui/shell.ts";
import { createController } from "./controller.ts";

export async function bootstrapApp(root) {
  if (!root) {
    return;
  }
  const controller = await createController(root);
  controller.subscribe((state, actions) => renderShell(root, state, actions));
  await controller.start();
}
