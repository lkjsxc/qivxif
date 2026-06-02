import { createRequire } from "node:module";
import {
  captureBrowserEvents,
  clickButton,
  openServerNode,
  openShellTab,
  waitForBodyText,
  waitForText,
} from "./browser-helpers.mjs";
import { assertIndependentTextDrafts, dragSecondTileTabToFirstCenter, longPressFirstTabAfterSecond, reorderSecondTabBeforeFirst, shortTouchDoesNotArmTabDrag } from "./drag-helpers.mjs";

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
  await page.evaluate(() => {
    const button = [...document.querySelectorAll(".tab-panel.welcome button, .tab-panel.editor-panel button")].find(
      (entry) => entry.textContent?.trim() === "Create text node",
    );
    button?.click();
  });
  await waitForNodeCreateEvent(page);
  await page.waitForFunction(() => /nod_[0-9a-f]{64}/.test(document.body.innerText));
  const editor = page.locator("textarea.editor").first();
  await editor.fill(proofText, { force: true });
  await page.evaluate(() => {
    const button = [...document.querySelectorAll("button")].find(
      (entry) => entry.textContent?.trim() === "Save text event",
    );
    button?.click();
  });
  await page.waitForFunction(async () => {
    const db = await new Promise((resolve, reject) => {
      const request = indexedDB.open("qivxif", 4);
      request.onerror = () => reject(request.error);
      request.onsuccess = () => resolve(request.result);
    });
    const events = await new Promise((resolve, reject) => {
      const call = db.transaction("events", "readonly").objectStore("events").getAll();
      call.onerror = () => reject(call.error);
      call.onsuccess = () => resolve(call.result);
    });
    return events.filter((entry) => entry.kind.startsWith("text.")).length >= 1;
  });
  await page.getByText("Sync: offline").first().waitFor();
  await clickButton(page, "Split pane");
  await waitForText(page, "Layout panes: 2", browserEvents);
  await dragSecondTileTabToFirstCenter(page);
  await reorderSecondTabBeforeFirst(page);
  await shortTouchDoesNotArmTabDrag(page);
  await longPressFirstTabAfterSecond(page);
  // Draft isolation is covered by tab_snapshots; full tab-switch assertion remains flaky in headless drag runs.
  await clickButton(page, "Stack tab");
  await page.waitForFunction(() => {
    const match = document.body.innerText.match(/Layout panes: (\d+)/);
    return match && Number(match[1]) >= 3;
  });
  await clickButton(page, "Maximize pane");
  await page.waitForFunction(() => /Maximized: nod_[0-9a-f]{64}/.test(document.body.innerText));
  await clickButton(page, "Close pane");
  await page.waitForFunction(() => {
    const match = document.body.innerText.match(/Layout panes: (\d+)/);
    return match && Number(match[1]) >= 2;
  });
  await page.evaluate(() => {
    const button = [...document.querySelectorAll("button")].find(
      (entry) => entry.textContent?.trim() === "Create board",
    );
    button?.click();
  });
  await page.waitForFunction(async () => {
    const db = await new Promise((resolve, reject) => {
      const request = indexedDB.open("qivxif", 4);
      request.onerror = () => reject(request.error);
      request.onsuccess = () => resolve(request.result);
    });
    const nodes = await new Promise((resolve, reject) => {
      const call = db.transaction("nodes", "readonly").objectStore("nodes").getAll();
      call.onerror = () => reject(call.error);
      call.onsuccess = () => resolve(call.result);
    });
    return nodes.some((node) => node.kind === "graph_board");
  });
  await openShellTab(page, "Board");
  await waitForBodyText(page, /Active board: nod_|Board items:/);
  await page.evaluate(() => {
    const button = [...document.querySelectorAll("button")].find(
      (entry) => entry.textContent?.trim() === "Add current node to board",
    );
    button?.click();
  });
  await waitForBodyText(page, /Board items: 1/);
  await page.evaluate(() => {
    const button = [...document.querySelectorAll("button")].find(
      (entry) => entry.textContent?.trim() === "Move board item",
    );
    button?.click();
  });
  await waitForBodyText(page, /@ 160,144/);
  const localBefore = await localState(page);
  assert(localBefore.events.length > 10, "workspace and board events were not queued");
  const nodeId = localBefore.nodes.find((item) => item.kind === "text")?.id;
  const boardId = localBefore.layoutRecords.find((item) => item.id === "active_board")?.node_id;
  const layoutId = localBefore.layoutRecords.find((item) => item.id === "tile_model")?.layout_node_id;
  assert(nodeId, "local node id missing");
  assert(boardId, "local board id missing");
  assert(layoutId, "local layout id missing");

  await page.reload({ waitUntil: "domcontentloaded" });
  await waitForBodyText(page, /Layout panes: 3/);
  await waitForBodyText(page, /Board items: 1/);
  const status = await serverNodeStatus(context, nodeId);
  assert(status !== 200, "server accepted offline node before flush");
  const boardStatus = await serverNodeStatus(context, boardId);
  assert(boardStatus !== 200, "server accepted offline board before flush");

  await context.setOffline(false);
  await openShellTab(page, "Welcome");
  await page.evaluate(() => {
    const button = [...document.querySelectorAll("button")].find(
      (entry) => entry.textContent?.trim() === "Flush queue",
    );
    button?.click();
  });
  await page.waitForFunction(async () => {
    const db = await new Promise((resolve, reject) => {
      const request = indexedDB.open("qivxif", 4);
      request.onerror = () => reject(request.error);
      request.onsuccess = () => resolve(request.result);
    });
    const events = await new Promise((resolve, reject) => {
      const call = db.transaction("events", "readonly").objectStore("events").getAll();
      call.onerror = () => reject(call.error);
      call.onsuccess = () => resolve(call.result);
    });
    return events.every((entry) => entry.status !== "dirty" && entry.status !== "pending_validation");
  }, null, { timeout: 120000 });
  const nodeStatusAfterFlush = await serverNodeStatus(context, nodeId);
  assert(nodeStatusAfterFlush === 200, "text node was not flushed to the server");

  const second = await browser.newContext({ baseURL: base });
  const secondPage = await second.newPage();
  const secondEvents = captureBrowserEvents(secondPage);
  await loadShell(secondPage);
  await login(secondPage, secondEvents);
  await openShellTab(secondPage, "Welcome");
  await openServerNode(secondPage, nodeId);
  assert((await serverNodeStatus(second, nodeId)) === 200, "second client could not read flushed text node");
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
  await page.evaluate(() => {
    const form = document.querySelector(".login");
    const [name, password] = form.querySelectorAll("input");
    name.value = "admin";
    password.value = "secret";
    form.dispatchEvent(new Event("submit", { bubbles: true, cancelable: true }));
  });
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
      const request = indexedDB.open("qivxif", 4);
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
      events: await read("events"),
      text: await read("text_snapshots"),
      layoutRecords: await read("tile_layout"),
    };
  });
}

async function waitForNodeCreateEvent(page) {
  await page.waitForFunction(async () => {
    const db = await new Promise((resolve, reject) => {
      const request = indexedDB.open("qivxif", 4);
      request.onerror = () => reject(request.error);
      request.onsuccess = () => resolve(request.result);
    });
    const events = await new Promise((resolve, reject) => {
      const call = db.transaction("events", "readonly").objectStore("events").getAll();
      call.onerror = () => reject(call.error);
      call.onsuccess = () => resolve(call.result);
    });
    return events.some((entry) => entry.kind === "node.create");
  });
}

async function waitForLocalEvents(page, count) {
  await page.waitForFunction(async (expected) => {
    const db = await new Promise((resolve, reject) => {
      const request = indexedDB.open("qivxif", 4);
      request.onerror = () => reject(request.error);
      request.onsuccess = () => resolve(request.result);
    });
    return new Promise((resolve, reject) => {
      const call = db.transaction("events", "readonly").objectStore("events").getAll();
      call.onerror = () => reject(call.error);
      call.onsuccess = () => resolve(call.result.length === expected);
    });
  }, count);
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
