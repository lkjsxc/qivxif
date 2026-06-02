import { copyFile, mkdir, readdir, readFile, writeFile } from "node:fs/promises";
import { dirname, join, relative, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const root = resolve(dirname(fileURLToPath(import.meta.url)), "..");
const dist = resolve(root, process.env.QIVXIF_WEB_DIST_DIR ?? "dist");

await mkdir(dist, { recursive: true });

const swSource = join(root, "service-worker", "service-worker.ts");
const swText = await readFile(swSource, "utf8");
await writeFile(join(dist, "service-worker.js"), swText.replaceAll(".ts\"", ".js\""));

await copyFile(join(root, "static", "manifest.json"), join(dist, "manifest.json"));
await writeAssetManifest(dist);

async function writeAssetManifest(distDir) {
  const assets = [];
  async function walk(dir) {
    for (const entry of await readdir(dir, { withFileTypes: true })) {
      const path = join(dir, entry.name);
      if (entry.isDirectory()) {
        await walk(path);
      } else {
        assets.push("/" + relative(distDir, path).split("\\").join("/"));
      }
    }
  }
  await walk(distDir);
  await writeFile(join(distDir, "asset-manifest.json"), JSON.stringify(assets.sort(), null, 2));
}
