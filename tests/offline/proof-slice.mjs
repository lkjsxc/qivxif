import { createRequire } from "node:module";

const require = createRequire(import.meta.url);
const { chromium } = require("playwright-core");

const base = process.env.QIVXIF_E2E_BASE ?? "http://127.0.0.1:8080";
const browserPath = process.env.QIVXIF_BROWSER ?? "/usr/bin/chromium";
const proofText = "offline proof text";

const browser = await chromium.launch({
  executablePath: browserPath,
  headless: true,
  args: ["--no-sandbox", "--disable-dev-shm-usage"],
});

try {
  const context = await browser.newContext({ baseURL: base });
  const page = await context.newPage();
  const browserEvents = captureBrowserEvents(page);
  await loadShell(page);
  await serviceWorkerReady(page);
  await page.reload({ waitUntil: "domcontentloaded" });
  await onlineShellReady(page);
  await login(page, browserEvents);

  await context.setOffline(true);
  await page.getByRole("button", { name: "Create text node" }).click();
  await page.locator(".editor").fill(proofText);
  await page.getByRole("button", { name: "Save text operation" }).click();
  await waitForLocalOps(page, 2);
  await waitForText(page, "Queued: 2", browserEvents);
  await page.getByText("Sync: offline").waitFor();
  await page.getByRole("button", { name: "Split pane" }).click();
  await page.getByText("Layout panes: 2").waitFor();
  await page.getByRole("button", { name: "Stack tab" }).click();
  await page.getByText("Layout panes: 3").waitFor();
  await page.getByRole("button", { name: "Maximize pane" }).click();
  await page.getByText(/^Maximized: nod_/).waitFor();
  await page.getByRole("button", { name: "Close pane" }).click();
  await page.getByText("Layout panes: 2").waitFor();
  await page.getByRole("button", { name: "Create board" }).click();
  await page.getByText(/^Active board: nod_/).waitFor();
  await page.getByRole("button", { name: "Add current node to board" }).click();
  await page.getByText("Board items: 1").waitFor();
  await page.getByRole("button", { name: "Move board item" }).click();
  await page.getByText("@ 160,144").waitFor();
  const localBefore = await localState(page);
  assert(localBefore.ops.length > 10, "workspace and board operations were not queued");
  const nodeId = localBefore.nodes.find((item) => item.kind === "text")?.id;
  const boardId = localBefore.workspace.find((item) => item.id === "active_board")?.node_id;
  const layoutId = localBefore.workspace.find((item) => item.id === "workspace_model")?.layout_node_id;
  assert(nodeId, "local node id missing");
  assert(boardId, "local board id missing");
  assert(layoutId, "local layout id missing");

  await page.reload({ waitUntil: "domcontentloaded" });
  await waitForText(page, "Layout panes: 2", browserEvents);
  await waitForText(page, "Board items: 1", browserEvents);
  await page.getByRole("tab", { name: "Editor" }).first().click();
  assert(
    (await page.locator(".editor").inputValue()) === proofText,
    "offline text was not restored",
  );
  const status = await serverNodeStatus(context, nodeId);
  assert(status !== 200, "server accepted offline node before flush");
  const boardStatus = await serverNodeStatus(context, boardId);
  assert(boardStatus !== 200, "server accepted offline board before flush");

  await context.setOffline(false);
  await page.getByRole("tab", { name: "Home" }).first().click();
  await page.getByRole("button", { name: "Flush queue" }).click();
  await page.getByText("Queued: 0").waitFor({ timeout: 15000 });
  await page.getByRole("tab", { name: "History" }).first().click();
  await page.getByText(/^node\.create #/).waitFor();
  await page.getByText(/^text\.restore #/).waitFor();

  const second = await browser.newContext({ baseURL: base });
  const secondPage = await second.newPage();
  const secondEvents = captureBrowserEvents(secondPage);
  await loadShell(secondPage);
  await login(secondPage, secondEvents);
  await secondPage.getByRole("tab", { name: "Home" }).first().click();
  await secondPage.getByLabel("Server node id").fill(nodeId);
  await secondPage.getByRole("button", { name: "Open node" }).click();
  await secondPage.waitForFunction(
    (expected) => document.querySelector(".editor")?.value === expected,
    proofText,
  );
  await secondPage.getByText(/^node\.create #/).waitFor();
  await secondPage.getByText(/^text\.restore #/).waitFor();
  await secondPage.getByRole("tab", { name: "Home" }).first().click();
  await secondPage.getByLabel("Server node id").fill(boardId);
  await secondPage.getByRole("button", { name: "Open node" }).click();
  await secondPage.getByText("Board items: 1").waitFor();
  await secondPage.getByText("@ 160,144").waitFor();
  await secondPage.getByRole("tab", { name: "Home" }).first().click();
  await secondPage.getByLabel("Server node id").fill(layoutId);
  await secondPage.getByRole("button", { name: "Open node" }).click();
  await secondPage.getByText("Layout panes: 2").waitFor();
  await second.close();
  await context.close();
} finally {
  await browser.close();
}

async function loadShell(page) {
  await page.goto("/", { waitUntil: "domcontentloaded" });
  await page.locator(".workspace").waitFor();
  await onlineShellReady(page);
}

async function onlineShellReady(page) {
  await page.getByText(/Capabilities: .*server\.health/).waitFor();
}

async function serviceWorkerReady(page) {
  await page.evaluate(() => navigator.serviceWorker.ready.then(() => true));
  await page.waitForFunction(() => navigator.serviceWorker.controller !== null);
}

async function login(page, browserEvents = []) {
  await page.getByLabel("Login name").fill("admin");
  await page.getByLabel("Password").fill("secret");
  await page.getByRole("button", { name: "Login" }).click();
  try {
    await page.getByText("Signed in as admin").waitFor();
  } catch (error) {
    const body = await page.locator("body").innerText();
    throw new Error(`login did not complete\n${body}\n${browserEvents.join("\n")}`);
  }
}

async function localState(page) {
  return page.evaluate(async () => {
    const db = await new Promise((resolve, reject) => {
      const request = indexedDB.open("qivxif", 1);
      request.onerror = () => reject(request.error);
      request.onsuccess = () => resolve(request.result);
    });
    const read = (name) =>
      new Promise((resolve, reject) => {
        const call = db.transaction(name, "readonly").objectStore(name).getAll();
        call.onerror = () => reject(call.error);
        call.onsuccess = () => resolve(call.result);
      });
    return {
      nodes: await read("nodes"),
      edges: await read("edges"),
      ops: await read("ops"),
      text: await read("text_snapshots"),
      workspace: await read("workspace_layout"),
    };
  });
}

async function waitForLocalOps(page, count) {
  await page.waitForFunction(async (expected) => {
    const db = await new Promise((resolve, reject) => {
      const request = indexedDB.open("qivxif", 1);
      request.onerror = () => reject(request.error);
      request.onsuccess = () => resolve(request.result);
    });
    return new Promise((resolve, reject) => {
      const call = db.transaction("ops", "readonly").objectStore("ops").getAll();
      call.onerror = () => reject(call.error);
      call.onsuccess = () => resolve(call.result.length === expected);
    });
  }, count);
}

function captureBrowserEvents(page) {
  const events = [];
  page.on("console", (message) => events.push(`console ${message.type()}: ${message.text()}`));
  page.on("pageerror", (error) => events.push(`pageerror: ${error.message}`));
  page.on("requestfailed", (request) => {
    events.push(`requestfailed: ${request.url()} ${request.failure()?.errorText}`);
  });
  return events;
}

async function waitForText(page, value, browserEvents = []) {
  try {
    await page.getByText(value).waitFor({ timeout: 5000 });
  } catch (error) {
    const body = await page.locator("body").innerText();
    const local = JSON.stringify(await localState(page));
    throw new Error(`${value} was not visible\n${body}\n${local}\n${browserEvents.join("\n")}`);
  }
}

async function serverNodeStatus(context, nodeId) {
  const cookies = await context.cookies(base);
  const cookie = cookies.map((item) => `${item.name}=${item.value}`).join("; ");
  const response = await fetch(`${base}/api/nodes/${nodeId}`, {
    headers: { cookie },
  });
  return response.status;
}

function assert(condition, message) {
  if (!condition) {
    throw new Error(message);
  }
}
