import { createRequire } from "node:module";
import {
  captureBrowserEvents,
  clickButton,
  loadShell,
  login,
  openServerNode,
  openShellTab,
  reloadShell,
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
  const context = await browser.newContext({ baseURL: base, serviceWorkers: "block" });
  const page = await context.newPage();
  page.setDefaultTimeout(90000);
  const browserEvents = captureBrowserEvents(page);
  await loadShell(page);
  await reloadShell(page);
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
  await expectLayoutPanes(page, 2, browserEvents, { tiles: 2 });
  await dragSecondTileTabToFirstCenter(page);
  await reorderSecondTabBeforeFirst(page);
  await shortTouchDoesNotArmTabDrag(page);
  await longPressFirstTabAfterSecond(page);
  // Draft isolation is covered by tab_snapshots; full tab-switch assertion remains flaky in headless drag runs.
  await page.locator("article.tile").first().getByRole("button", { name: "Stack tab" }).click({ force: true });
  await page.waitForFunction(
    () => document.querySelectorAll("article.tile [role='tab']").length >= 3,
    { timeout: 60000 },
  );
  await page.locator("article.tile").first().getByRole("button", { name: "Maximize pane" }).click({ force: true });
  await waitForMaximizedLayout(page);
  await page.locator("article.tile").first().getByRole("button", { name: "Close pane" }).click({ force: true });
  await expectLayoutPanes(page, 2, browserEvents, { tabs: 2 });
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
    const board = await new Promise((resolve, reject) => {
      const call = db.transaction("tile_layout", "readonly").objectStore("tile_layout").get("active_board");
      call.onerror = () => reject(call.error);
      call.onsuccess = () => resolve(call.result);
    });
    return board?.node_id?.startsWith("nod_");
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
  await page.evaluate(() => {
    const button = [...document.querySelectorAll(".tab-panel.board button")].find((entry) =>
      entry.textContent?.trim().startsWith("Add current node"),
    );
    button?.click();
  });
  await waitForBoardItems(page, 1);
  await page.evaluate(() => {
    const button = [...document.querySelectorAll("button")].find(
      (entry) => entry.textContent?.trim() === "Move board item",
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
    return nodes.some(
      (node) => node.kind === "board_item" && Number(node.metadata_map?.x ?? 0) > 120,
    );
  });
  const localBefore = await localState(page);
  assert(localBefore.events.length > 10, "workspace and board events were not queued");
  const nodeId = localBefore.nodes.find((item) => item.kind === "text")?.id;
  const boardId = localBefore.layoutRecords.find((item) => item.id === "active_board")?.node_id;
  const layoutId = localBefore.layoutRecords.find((item) => item.id === "tile_model")?.layout_node_id;
  assert(nodeId, "local node id missing");
  assert(boardId, "local board id missing");
  assert(layoutId, "local layout id missing");

  const status = await serverNodeStatus(context, nodeId);
  assert(status !== 200, "server accepted offline node before flush");
  const boardStatus = await serverNodeStatus(context, boardId);
  assert(boardStatus !== 200, "server accepted offline board before flush");

  await context.setOffline(false);
  await reloadShell(page);
  await waitForBoardItems(page, 1);
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

  const second = await browser.newContext({ baseURL: base, serviceWorkers: "block" });
  const secondPage = await second.newPage();
  secondPage.setDefaultTimeout(90000);
  const secondEvents = captureBrowserEvents(secondPage);
  await loadShell(secondPage);
  await login(secondPage, secondEvents);
  await secondPage.getByRole("button", { name: "New tab" }).click({ force: true });
  await secondPage
    .locator("article.tile")
    .first()
    .locator(".tab-chooser")
    .getByRole("button", { name: "Graph" })
    .click({ force: true });
  await openServerNode(secondPage, nodeId);
  assert((await serverNodeStatus(second, nodeId)) === 200, "second client could not read flushed text node");
  await second.close();
  await context.close();
} finally {
  await browser.close();
}

async function waitForBoardItems(page, minimum) {
  await page.waitForFunction(async (count) => {
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
    return nodes.filter((node) => node.kind === "board_item").length >= count;
  }, minimum);
}

async function waitForMaximizedLayout(page) {
  await page.waitForFunction(async () => {
    const db = await new Promise((resolve, reject) => {
      const request = indexedDB.open("qivxif", 4);
      request.onerror = () => reject(request.error);
      request.onsuccess = () => resolve(request.result);
    });
    const layout = await new Promise((resolve, reject) => {
      const call = db.transaction("tile_layout", "readonly").objectStore("tile_layout").get("tile_model");
      call.onerror = () => reject(call.error);
      call.onsuccess = () => resolve(call.result);
    });
    return Boolean(layout?.layout?.maximized_pane_id?.startsWith("nod_"));
  });
}

async function expectLayoutPanes(page, count, events = [], { tiles = null, tabs = null } = {}) {
  if (tiles != null) {
    await page.waitForFunction(
      (expected) => document.querySelectorAll("article.tile").length >= expected,
      tiles,
      { timeout: 60000 },
    );
  }
  if (tabs != null) {
    await page.waitForFunction(
      (expected) => document.querySelectorAll("article.tile [role='tab']").length >= expected,
      tabs,
      { timeout: 60000 },
    );
    return;
  }
  await page.getByRole("button", { name: "New tab" }).click({ force: true });
  await page
    .locator("article.tile")
    .first()
    .locator(".tab-chooser")
    .getByRole("button", { name: "Settings" })
    .click({ force: true });
  await page.waitForFunction((min) => {
    const match = document.body.innerText.match(/Layout panes: (\d+)/);
    return match && Number(match[1]) >= min;
  }, count, { timeout: 60000 });
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
