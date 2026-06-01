const APP_CACHE = "qivxif-app-shell";
const APP_ASSETS = [
  "/",
  "/index.html",
  "/main.js",
  "/ids.js",
  "/api/client.js",
  "/actors/app-shell.js",
  "/actors/sync.js",
  "/store/indexed-db.js",
  "/ui/workspace.js",
  "/ui/sync-status-pane.js",
  "/styles.css",
  "/manifest.json",
];

self.addEventListener("install", (event) => {
  event.waitUntil(caches.open(APP_CACHE).then((cache) => cache.addAll(APP_ASSETS)));
});

self.addEventListener("fetch", (event) => {
  const url = new URL(event.request.url);
  if (url.pathname.startsWith("/api/")) {
    return;
  }
  if (event.request.mode === "navigate") {
    event.respondWith(fetch(event.request).catch(() => caches.match("/index.html")));
    return;
  }
  event.respondWith(caches.match(event.request).then((cached) => cached || fetch(event.request)));
});
