const APP_CACHE = "qivxif-app-shell";
const CORE_ASSETS = ["/", "/index.html", "/main.js", "/styles.css", "/asset-manifest.json"];

self.addEventListener("install", (event) => {
  event.waitUntil(cacheAppShell().then(() => self.skipWaiting()));
});

self.addEventListener("activate", (event) => {
  event.waitUntil(self.clients.claim());
});

self.addEventListener("fetch", (event) => {
  const url = new URL(event.request.url);
  if (url.pathname.startsWith("/api/")) {
    return;
  }
  if (event.request.mode === "navigate") {
    event.respondWith(fetch(event.request).catch(() => cachedAppAsset("/index.html")));
    return;
  }
  event.respondWith(staticAsset(event.request));
});

async function cacheAppShell() {
  const cache = await caches.open(APP_CACHE);
  const assets = await manifestAssets();
  await cache.addAll([...new Set([...CORE_ASSETS, ...assets])]);
}

async function manifestAssets() {
  try {
    const response = await fetch("/asset-manifest.json", { cache: "no-store" });
    return await response.json();
  } catch (_error) {
    return CORE_ASSETS;
  }
}

async function staticAsset(request) {
  const cached = await cachedAppAsset(request);
  if (cached) {
    return cached;
  }
  const response = await fetch(request);
  if (response.ok && request.method === "GET") {
    const cache = await caches.open(APP_CACHE);
    cache.put(request, response.clone());
  }
  return response;
}

function cachedAppAsset(requestOrPath) {
  if (typeof requestOrPath === "string") {
    return caches.match(requestOrPath);
  }
  const url = new URL(requestOrPath.url);
  return caches
    .match(requestOrPath)
    .then((cached) => cached || caches.match(url.pathname));
}
