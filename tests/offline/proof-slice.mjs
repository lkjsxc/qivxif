import { createRequire } from "node:module";
import { captureBrowserEvents, openServerNode, openShellTab, waitForText } from "./browser-helpers.mjs";
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
  await page.getByRole("button", { name: "Create text node" }).click();
  await page.locator(".editor").fill(proofText);
  await page.getByRole("button", { name: "Save text event" }).click();
  await waitForLocalEvents(page, 2);
  await waitForText(page, "Queued: 2", browserEvents);
  await page.getByText("Sync: offline").first().waitFor();
  await page.getByRole("button", { name: "Split pane" }).first().click();
  await waitForText(page, "Layout panes: 2", browserEvents);
  await dragSecondTileTabToFirstCenter(page);
  await waitForText(page, "Layout panes: 2", browserEvents);
  await reorderSecondTabBeforeFirst(page);
  await shortTouchDoesNotArmTabDrag(page);
  await longPressFirstTabAfterSecond(page);
  await assertIndependentTextDrafts(page, proofText);
  await page.getByRole("button", { name: "Stack tab" }).first().click();
  await waitForText(page, "Layout panes: 3", browserEvents);
  await page.getByRole("button", { name: "Maximize pane" }).first().click();
  await page.getByText(/^Maximized: nod_/).first().waitFor();
  await page.getByRole("button", { name: "Close pane" }).first().click();
  await waitForText(page, "Layout panes: 2", browserEvents);
  await page.getByRole("button", { name: "Create board" }).first().click();
  await openShellTab(page, "Board");
  await page.getByText(/^Active board: nod_/).first().waitFor();
  await page.getByRole("button", { name: "Add current node to board" }).first().click();
  await page.getByText("Board items: 1").first().waitFor();
  await page.getByRole("button", { name: "Move board item" }).first().click();
  await page.getByText("@ 160,144").first().waitFor();
  const localBefore = await localState(page);
  assert(localBefore.events.length > 10, "workspace and board events were not queued");
  const nodeId = localBefore.nodes.find((item) => item.kind === "text")?.id;
  const boardId = localBefore.layoutRecords.find((item) => item.id === "active_board")?.node_id;
  const layoutId = localBefore.layoutRecords.find((item) => item.id === "tile_model")?.layout_node_id;
  assert(nodeId, "local node id missing");
  assert(boardId, "local board id missing");
  assert(layoutId, "local layout id missing");

  await page.reload({ waitUntil: "domcontentloaded" });
  await waitForText(page, "Layout panes: 3", browserEvents);
  await waitForText(page, "Board items: 1", browserEvents);
  await openShellTab(page, "Editor");
  assert(
    (await page.locator("article.tile").first().locator(".editor").inputValue()) === proofText,
    "offline text was not restored",
  );
  const status = await serverNodeStatus(context, nodeId);
  assert(status !== 200, "server accepted offline node before flush");
  const boardStatus = await serverNodeStatus(context, boardId);
  assert(boardStatus !== 200, "server accepted offline board before flush");

  await context.setOffline(false);
  await openShellTab(page, "Welcome");
  await page.getByRole("button", { name: "Flush queue" }).click();
  try {
    await waitForText(page, "Queued: 0", browserEvents, 30000);
  } catch (error) {
    throw new Error(`${error.message}\n${JSON.stringify(await localState(page), null, 2)}`);
  }
  await openShellTab(page, "History");
  await page.getByText(/^node\.create #/).first().waitFor();
  await page.getByText(/^text\.restore #/).first().waitFor();

  const second = await browser.newContext({ baseURL: base });
  const secondPage = await second.newPage();
  const secondEvents = captureBrowserEvents(secondPage);
  await loadShell(secondPage);
  await login(secondPage, secondEvents);
  await openShellTab(secondPage, "Welcome");
  await openServerNode(secondPage, nodeId);
  try {
    await secondPage.waitForFunction(
      (expected) => document.querySelector(".editor")?.value === expected,
      proofText,
    );
  } catch (error) {
    const body = await secondPage.locator("body").innerText();
    throw new Error(`server text did not open\n${body}\n${secondEvents.join("\n")}`);
  }
  await secondPage.getByText(/^node\.create #/).first().waitFor();
  await secondPage.getByText(/^text\.restore #/).first().waitFor();
  await openShellTab(secondPage, "Welcome");
  await openServerNode(secondPage, boardId);
  await openShellTab(secondPage, "Board");
  await secondPage.getByText("Board items: 1").first().waitFor();
  await secondPage.getByText("@ 160,144").first().waitFor();
  await openShellTab(secondPage, "Welcome");
  await openServerNode(secondPage, layoutId);
  await secondPage.getByText("Layout panes: 3").first().waitFor();
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
      const request = indexedDB.open("qivxif", 3);
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

async function waitForLocalEvents(page, count) {
  await page.waitForFunction(async (expected) => {
    const db = await new Promise((resolve, reject) => {
      const request = indexedDB.open("qivxif", 3);
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
