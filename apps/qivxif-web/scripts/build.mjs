import { cp, mkdir, readFile, rm, writeFile } from "node:fs/promises";
import { dirname, join, relative, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const root = dirname(fileURLToPath(import.meta.url));
const app = join(root, "..");
const dist = resolve(app, process.env.QIVXIF_WEB_DIST_DIR ?? "dist");

await rm(dist, { recursive: true, force: true });
await mkdir(dist, { recursive: true });
await cp(join(app, "public"), dist, { recursive: true });
await cp(join(app, "index.html"), join(dist, "index.html"));
await copyModule(join(app, "src", "main.ts"));
await copyModule(join(app, "src", "ids.ts"));
await copyModule(join(app, "src", "http", "client.ts"));
await copyModule(join(app, "src", "actors", "app-shell.ts"));
await copyModule(join(app, "src", "actors", "actor-seq.ts"));
await copyModule(join(app, "src", "actors", "board-actions.ts"));
await copyModule(join(app, "src", "actors", "local-operations.ts"));
await copyModule(join(app, "src", "actors", "state-loader.ts"));
await copyModule(join(app, "src", "actors", "sync.ts"));
await copyModule(join(app, "src", "actors", "workspace-actions.ts"));
await copyModule(join(app, "src", "store", "indexed-db.ts"));
await copyModule(join(app, "src", "ui", "board.ts"));
await copyModule(join(app, "src", "ui", "workspace.ts"));
await copyModule(join(app, "src", "ui", "sync-status-pane.ts"));
await copyServiceWorker();

async function copyModule(source) {
  const target = join(dist, relative(join(app, "src"), source)).replace(/\.ts$/, ".js");
  await mkdir(dirname(target), { recursive: true });
  const text = await readFile(source, "utf8");
  await writeFile(target, text.replaceAll(".ts\"", ".js\""));
}

async function copyServiceWorker() {
  const text = await readFile(join(app, "service-worker", "service-worker.ts"), "utf8");
  await writeFile(join(dist, "service-worker.js"), text);
}
