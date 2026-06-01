const APP_CACHE = "qivxif-app-shell";
const APP_ASSETS = [
  "/",
  "/index.html",
  "/main.js",
  "/ids.js",
  "/http/client.js",
  "/actors/app-shell.js",
  "/actors/local-operations.js",
  "/actors/sync.js",
  "/store/indexed-db.js",
  "/ui/workspace.js",
  "/ui/sync-status-pane.js",
  "/styles.css",
  "/manifest.json",
  "/service-worker.js",
];

self.addEventListener("install", (event) => {
  event.waitUntil(
    caches
      .open(APP_CACHE)
      .then((cache) => cache.addAll(APP_ASSETS))
      .then(() => self.skipWaiting()),
  );
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
    event.respondWith(
      fetch(event.request).catch(() => cachedAppAsset("/index.html")),
    );
    return;
  }
  event.respondWith(
    cachedAppAsset(event.request).then((cached) => cached || fetch(event.request)),
  );
});

function cachedAppAsset(requestOrPath) {
  if (typeof requestOrPath === "string") {
    return caches.match(requestOrPath);
  }
  const url = new URL(requestOrPath.url);
  return caches
    .match(requestOrPath)
    .then((cached) => cached || caches.match(url.pathname));
}
