import { copyFile, mkdir, readFile, writeFile } from "node:fs/promises";
import { dirname, join, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const root = resolve(dirname(fileURLToPath(import.meta.url)), "..");
const dist = resolve(root, process.env.QIVXIF_WEB_DIST_DIR ?? "dist");

await mkdir(dist, { recursive: true });

const swSource = join(root, "service-worker", "service-worker.ts");
const swText = await readFile(swSource, "utf8");
await writeFile(join(dist, "service-worker.js"), swText.replaceAll(".ts\"", ".js\""));

await copyFile(join(root, "static", "manifest.json"), join(dist, "manifest.json"));
