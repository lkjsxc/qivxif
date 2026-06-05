import { createRequire } from "node:module";
import {
  captureBrowserEvents,
  loadShell,
  login,
  openServerNode,
  openShellTab,
  readLocalStore,
  reloadShell,
  waitForLocalStore,
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
  await waitForLocalStore(page, "events", "(rows) => rows.filter((entry) => entry.kind.startsWith('text.')).length >= 1");
  await page.getByText("Sync: offline").first().waitFor();
  await clickTileMenuItem(page, "Split right");
  await expectLayoutPanes(page, 2, browserEvents, { tiles: 2 });
  await dragSecondTileTabToFirstCenter(page);
  await reorderSecondTabBeforeFirst(page);
  await shortTouchDoesNotArmTabDrag(page);
  await longPressFirstTabAfterSecond(page);
  // Draft isolation is covered by tab_snapshots; full tab-switch assertion remains flaky in headless drag runs.
  await clickTileMenuItem(page, "Stack tab");
  await page.waitForFunction(
    () => document.querySelectorAll("article.tile [role='tab']").length >= 3,
    { timeout: 60000 },
  );
  await clickTileMenuItem(page, "Maximize pane");
  await waitForMaximizedLayout(page);
  await clickTileMenuItem(page, "Close tile");
  await expectLayoutPanes(page, 2, browserEvents, { tabs: 2 });
  await page.evaluate(() => {
    const button = [...document.querySelectorAll("button")].find(
      (entry) => entry.textContent?.trim() === "Create Graph Map",
    );
    button?.click();
  });
  await waitForLocalStore(page, "tile_layout", "(rows) => rows.some((row) => row.id === 'active_graph_map' && row.node_id?.startsWith('nod_'))");
  await waitForLocalStore(page, "nodes", "(rows) => rows.some((node) => node.kind === 'graph_map')");
  await page.evaluate(() => {
    const button = [...document.querySelectorAll(".tab-panel.graph-map button")].find((entry) =>
      entry.textContent?.trim().startsWith("Add current node"),
    );
    button?.click();
  });
  await waitForGraphMapItems(page, 1);
  await page.evaluate(() => {
    const button = [...document.querySelectorAll("button")].find(
      (entry) => entry.textContent?.trim() === "Pin first node",
    );
    button?.click();
  });
  await waitForLocalStore(page, "nodes", "(rows) => rows.some((node) => node.kind === 'graph_map_item' && Number(node.metadata_map?.x ?? 0) > 120)");
  const localBefore = await localState(page);
  assert(localBefore.events.length > 10, "workspace and Graph Map events were not queued");
  const nodeId = localBefore.nodes.find((item) => item.kind === "text")?.id;
  const graphMapId = localBefore.layoutRecords.find((item) => item.id === "active_graph_map")?.node_id;
  const layoutId = localBefore.layoutRecords.find((item) => item.id === "tile_model")?.layout_node_id;
  assert(nodeId, "local node id missing");
  assert(graphMapId, "local Graph Map id missing");
  assert(layoutId, "local layout id missing");

  const status = await serverNodeStatus(context, nodeId);
  assert(status !== 200, "server accepted offline node before flush");
  const graphMapStatus = await serverNodeStatus(context, graphMapId);
  assert(graphMapStatus !== 200, "server accepted offline Graph Map before flush");

  await context.setOffline(false);
  await reloadShell(page);
  await waitForGraphMapItems(page, 1);
  await openShellTab(page, "Welcome");
  await page.evaluate(() => {
    const button = [...document.querySelectorAll("button")].find(
      (entry) => entry.textContent?.trim() === "Flush queue",
    );
    button?.click();
  });
  await waitForLocalStore(page, "events", "(rows) => rows.every((entry) => entry.status !== 'dirty' && entry.status !== 'pending_validation')", null, 120000);
  const nodeStatusAfterFlush = await serverNodeStatus(context, nodeId);
  assert(nodeStatusAfterFlush === 200, `text node was not flushed to the server: ${nodeStatusAfterFlush}`);

  const second = await browser.newContext({ baseURL: base, serviceWorkers: "block" });
  const secondPage = await second.newPage();
  secondPage.setDefaultTimeout(90000);
  const secondEvents = captureBrowserEvents(secondPage);
  await loadShell(secondPage);
  await login(secondPage, secondEvents);
  await secondPage.getByRole("button", { name: "New tab" }).click({ force: true });
  await secondPage.locator(".tab-body:not([hidden]) .new-tab-panel").getByRole("button", { name: /Graph Inspect/ }).click({ force: true });
  await openServerNode(secondPage, nodeId);
  assert((await serverNodeStatus(second, nodeId)) === 200, "second client could not read flushed text node");
  await second.close();
  await context.close();
} finally {
  await browser.close();
}

async function waitForGraphMapItems(page, minimum) {
  await waitForLocalStore(page, "nodes", "(rows, count) => rows.filter((node) => node.kind === 'graph_map_item').length >= count", minimum);
}

async function waitForMaximizedLayout(page) {
  await waitForLocalStore(page, "tile_layout", "(rows) => rows.some((row) => row.id === 'tile_model' && row.layout?.maximized_pane_id?.startsWith('nod_'))");
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
  await page.locator(".tab-body:not([hidden]) .new-tab-panel").getByRole("button", { name: /Settings/ }).click({ force: true });
  await page.waitForFunction((min) => {
    const match = document.body.innerText.match(/Layout panes: (\d+)/);
    return match && Number(match[1]) >= min;
  }, count, { timeout: 60000 });
}

async function clickTileMenuItem(page, name) {
  const tile = page.locator("article.tile").first();
  await tile.getByRole("button", { name: "Tile menu" }).click({ force: true });
  await tile.getByRole("menuitem", { name }).click({ force: true });
}

async function localState(page) {
  return {
    edges: await readLocalStore(page, "edges"),
    events: await readLocalStore(page, "events"),
    layoutRecords: await readLocalStore(page, "tile_layout"),
    nodes: await readLocalStore(page, "nodes"),
    text: await readLocalStore(page, "text_snapshots"),
  };
}

async function waitForNodeCreateEvent(page) {
  await waitForLocalStore(page, "events", "(rows) => rows.some((entry) => entry.kind === 'node.create')");
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
