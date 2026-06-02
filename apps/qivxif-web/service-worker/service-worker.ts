const APP_CACHE = "qivxif-app-shell";
const CORE_ASSETS = ["/", "/index.html", "/manifest.json"];

self.addEventListener("install", (event) => {
  event.waitUntil(cacheAppShell().then(() => self.skipWaiting()));
});

self.addEventListener("activate", (event) => {
  event.waitUntil(
    caches.keys().then((keys) => Promise.all(keys.filter((k) => k !== APP_CACHE).map((k) => caches.delete(k)))).then(() =>
      self.clients.claim(),
    ),
  );
});

self.addEventListener("fetch", (event) => {
  const url = new URL(event.request.url);
  if (url.pathname.startsWith("/api/")) {
    return;
  }
  if (url.pathname.startsWith("/_app/")) {
    event.respondWith(fetch(event.request));
    return;
  }
  if (event.request.mode === "navigate") {
    event.respondWith(fetch(event.request).catch(() => cachedPath("/index.html")));
    return;
  }
  event.respondWith(staticAsset(event.request));
});

async function cacheAppShell() {
  const cache = await caches.open(APP_CACHE);
  await cache.addAll(CORE_ASSETS);
}

async function staticAsset(request) {
  const cached = await caches.match(request);
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

function cachedPath(path) {
  return caches.match(path);
}
