export async function assertPaneScrollRestores(page, paneId, expectedText) {
  await page.setViewportSize({ width: 980, height: 360 });
  const top = await page.evaluate(() => {
    const body = document.querySelector("article.tile .tab-body");
    if (!body) {
      throw new Error("active tab body missing");
    }
    body.scrollTop = Math.min(140, body.scrollHeight - body.clientHeight);
    body.dispatchEvent(new Event("scroll", { bubbles: true }));
    return body.scrollTop;
  });
  assert(top > 0, "pane body did not scroll");
  await waitForPaneScrollSnapshot(page, paneId, top);
  await page.reload({ waitUntil: "domcontentloaded" });
  await page.locator(".workspace").waitFor();
  await page.waitForFunction(
    (expected) => document.querySelector("article.tile .editor")?.value === expected,
    expectedText,
  );
  await page.waitForFunction((expected) => {
    const top = document.querySelector("article.tile .tab-body")?.scrollTop ?? 0;
    return Math.abs(top - expected) <= 1;
  }, top);
  await page.setViewportSize({ width: 1280, height: 720 });
}

async function waitForPaneScrollSnapshot(page, paneId, top) {
  await page.waitForFunction(async ({ paneId, top }) => {
    const db = await new Promise((resolve, reject) => {
      const request = indexedDB.open("qivxif", 3);
      request.onerror = () => reject(request.error);
      request.onsuccess = () => resolve(request.result);
    });
    return new Promise((resolve, reject) => {
      const call = db.transaction("tab_snapshots", "readonly")
        .objectStore("tab_snapshots")
        .get(`pane_scroll:${paneId}`);
      call.onerror = () => reject(call.error);
      call.onsuccess = () => resolve(call.result?.scroll_top === top);
    });
  }, { paneId, top });
}

function assert(condition, message) {
  if (!condition) {
    throw new Error(message);
  }
}
