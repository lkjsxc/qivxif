export async function serverInfo() {
  return getEnvelope("/api/server-info");
}

export async function login(name, password) {
  return postEnvelope("/api/auth/login", { name, password });
}

export async function setupStatus() {
  return getEnvelope("/api/setup");
}

export async function createOwner(name, password) {
  return postEnvelope("/api/setup/owner", { name, password });
}

export async function node(nodeId) {
  return getEnvelope(`/api/nodes/${nodeId}`);
}

export async function nodeHistory(nodeId) {
  return getEnvelope(`/api/nodes/${nodeId}/history`);
}

export async function neighborhood(nodeId) {
  return getEnvelope(`/api/graph/neighborhood?node_id=${encodeURIComponent(nodeId)}&depth=2&limit=100`);
}

export async function sendQueued(entry, csrfToken) {
  return postEnvelope(entry.route.path, entry.request, csrfToken);
}

export async function text(nodeId) {
  return getEnvelope(`/api/text/${nodeId}`);
}

const REQUEST_TIMEOUT_MS = 15000;

async function getEnvelope(path) {
  const response = await fetchWithTimeout(path, { credentials: "include" });
  return readEnvelope(response);
}

async function postEnvelope(path, body, csrfToken = "") {
  const headers = { "content-type": "application/json" };
  if (csrfToken) {
    headers["x-qivxif-csrf"] = csrfToken;
  }
  const response = await fetchWithTimeout(path, {
    body: JSON.stringify(body),
    credentials: "include",
    headers,
    method: "POST",
  });
  return readEnvelope(response);
}

async function fetchWithTimeout(path, init) {
  const controller = new AbortController();
  const timer = setTimeout(() => controller.abort(), REQUEST_TIMEOUT_MS);
  try {
    return await fetch(path, { ...init, signal: controller.signal });
  } finally {
    clearTimeout(timer);
  }
}

async function readEnvelope(response) {
  const text = await response.text();
  const envelope = parseEnvelope(response, text);
  if (!response.ok || envelope.error) {
    throw apiFailure(envelope);
  }
  return envelope.payload;
}

function parseEnvelope(response, text) {
  try {
    return text ? JSON.parse(text) : {};
  } catch {
    return {
      error: {
        code: `http.${response.status}`,
        message: text || response.statusText || "request failed",
      },
    };
  }
}

function apiFailure(envelope) {
  const error: Error & { api?: unknown } = new Error(envelope.error?.message ?? "request failed");
  error.api = envelope.error;
  return error;
}
