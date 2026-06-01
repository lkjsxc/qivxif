export async function serverInfo() {
  return getEnvelope("/api/server-info");
}

export async function login(name, password) {
  return postEnvelope("/api/auth/login", { name, password });
}

export async function node(nodeId) {
  return getEnvelope(`/api/nodes/${nodeId}`);
}

export async function nodeHistory(nodeId) {
  return getEnvelope(`/api/nodes/${nodeId}/history`);
}

export async function sendQueued(entry, csrfToken) {
  return postEnvelope(entry.route.path, entry.request, csrfToken);
}

export async function text(nodeId) {
  return getEnvelope(`/api/text/${nodeId}`);
}

async function getEnvelope(path) {
  const response = await fetch(path, { credentials: "include" });
  return readEnvelope(response);
}

async function postEnvelope(path, body, csrfToken) {
  const headers = { "content-type": "application/json" };
  if (csrfToken) {
    headers["x-qivxif-csrf"] = csrfToken;
  }
  const response = await fetch(path, {
    body: JSON.stringify(body),
    credentials: "include",
    headers,
    method: "POST",
  });
  return readEnvelope(response);
}

async function readEnvelope(response) {
  const envelope = await response.json();
  if (!response.ok || envelope.error) {
    throw apiFailure(envelope);
  }
  return envelope.payload;
}

function apiFailure(envelope) {
  const error = new Error(envelope.error?.message ?? "request failed");
  error.api = envelope.error;
  return error;
}
