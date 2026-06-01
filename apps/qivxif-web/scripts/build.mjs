import { cp, mkdir, readdir, readFile, rm, writeFile } from "node:fs/promises";
import { dirname, join, relative, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const root = dirname(fileURLToPath(import.meta.url));
const app = join(root, "..");
const src = join(app, "src");
const dist = resolve(app, process.env.QIVXIF_WEB_DIST_DIR ?? "dist");

await rm(dist, { recursive: true, force: true });
await mkdir(dist, { recursive: true });
await cp(join(app, "public"), dist, { recursive: true });
await cp(join(app, "index.html"), join(dist, "index.html"));
await copyModules(src);
await copyServiceWorker();
await writeAssetManifest();

async function copyModules(dir) {
  for (const entry of await readdir(dir, { withFileTypes: true })) {
    const path = join(dir, entry.name);
    if (entry.isDirectory()) {
      await copyModules(path);
    } else if (entry.name.endsWith(".ts")) {
      await copyModule(path);
    }
  }
}

async function copyModule(source) {
  const target = join(dist, relative(src, source)).replace(/\.ts$/, ".js");
  await mkdir(dirname(target), { recursive: true });
  const text = await readFile(source, "utf8");
  await writeFile(target, text.replaceAll(".ts\"", ".js\""));
}

async function copyServiceWorker() {
  const text = await readFile(join(app, "service-worker", "service-worker.ts"), "utf8");
  await writeFile(join(dist, "service-worker.js"), text);
}

async function writeAssetManifest() {
  const assets = await distAssets(dist);
  await writeFile(join(dist, "asset-manifest.json"), JSON.stringify(assets.sort(), null, 2));
}

async function distAssets(dir) {
  const assets = [];
  for (const entry of await readdir(dir, { withFileTypes: true })) {
    const path = join(dir, entry.name);
    if (entry.isDirectory()) {
      assets.push(...(await distAssets(path)));
    } else {
      assets.push(`/${relative(dist, path).replaceAll("\\", "/")}`);
    }
  }
  return assets;
}
