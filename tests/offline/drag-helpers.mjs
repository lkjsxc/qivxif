export async function dragSecondTileTabToFirstCenter(page) {
  await page.evaluate(() => {
    const tiles = document.querySelectorAll("article.tile");
    const sourceTab = tiles[1]?.querySelector('[role="tab"]');
    const targetTile = tiles[0];
    if (!sourceTab || !targetTile) {
      throw new Error("drag test needs two tiles");
    }
    const dataTransfer = new DataTransfer();
    sourceTab.dispatchEvent(new DragEvent("dragstart", { bubbles: true, dataTransfer }));
    const box = targetTile.getBoundingClientRect();
    const eventInit = {
      bubbles: true,
      cancelable: true,
      clientX: box.left + box.width / 2,
      clientY: box.top + box.height / 2,
      dataTransfer,
    };
    targetTile.dispatchEvent(new DragEvent("dragover", eventInit));
    targetTile.dispatchEvent(new DragEvent("drop", eventInit));
  });
  await page.waitForFunction(() => document.querySelectorAll("article.tile").length === 1);
}

export async function reorderSecondTabBeforeFirst(page) {
  const before = await firstTileTabPaneIds(page);
  assert(before.length >= 2, "reorder test needs two tabs");
  await page.evaluate(() => {
    const firstTileTabs = () => [...document.querySelector("article.tile").querySelectorAll('[role="tab"]')];
    const tabs = firstTileTabs();
    const sourceTab = tabs[1];
    const targetTab = tabs[0];
    const dataTransfer = new DataTransfer();
    sourceTab.dispatchEvent(new DragEvent("dragstart", { bubbles: true, dataTransfer }));
    const box = targetTab.getBoundingClientRect();
    const eventInit = {
      bubbles: true,
      cancelable: true,
      clientX: box.left + 2,
      clientY: box.top + box.height / 2,
      dataTransfer,
    };
    targetTab.dispatchEvent(new DragEvent("dragover", eventInit));
    targetTab.dispatchEvent(new DragEvent("drop", eventInit));
  });
  await waitForFirstTab(page, before[1]);
  return firstTileTabPaneIds(page);
}

export async function longPressFirstTabAfterSecond(page) {
  const before = await firstTileTabPaneIds(page);
  assert(before.length >= 2, "long-press reorder test needs two tabs");
  await page.evaluate(async () => {
    const firstTileTabs = () => [...document.querySelector("article.tile").querySelectorAll('[role="tab"]')];
    const tabs = firstTileTabs();
    const sourceTab = tabs[0];
    const targetTab = tabs[1];
    const start = sourceTab.getBoundingClientRect();
    const end = targetTab.getBoundingClientRect();
    const pointer = {
      bubbles: true,
      button: 0,
      cancelable: true,
      pointerId: 31,
      pointerType: "touch",
    };
    sourceTab.dispatchEvent(
      new PointerEvent("pointerdown", {
        ...pointer,
        clientX: start.left + start.width / 2,
        clientY: start.top + start.height / 2,
      }),
    );
    await new Promise((resolve) => setTimeout(resolve, 460));
    const endPoint = { ...pointer, clientX: end.right - 2, clientY: end.top + end.height / 2 };
    sourceTab.dispatchEvent(new PointerEvent("pointermove", endPoint));
    sourceTab.dispatchEvent(new PointerEvent("pointerup", endPoint));
  });
  await waitForFirstTab(page, before[1]);
  return firstTileTabPaneIds(page);
}

export async function shortTouchDoesNotArmTabDrag(page) {
  const armed = await page.evaluate(async () => {
    const firstTileTabs = () => [...document.querySelector("article.tile").querySelectorAll('[role="tab"]')];
    const tab = firstTileTabs()[0];
    const box = tab.getBoundingClientRect();
    const pointer = {
      bubbles: true,
      button: 0,
      cancelable: true,
      clientX: box.left + box.width / 2,
      clientY: box.top + box.height / 2,
      pointerId: 32,
      pointerType: "touch",
    };
    tab.dispatchEvent(new PointerEvent("pointerdown", pointer));
    await new Promise((resolve) => setTimeout(resolve, 120));
    tab.dispatchEvent(new PointerEvent("pointerup", pointer));
    return document.body.classList.contains("tab-drag-armed") || Boolean(tab.dataset.dragArmed);
  });
  assert(!armed, "short touch armed tab drag");
}

export async function assertIndependentTextDrafts(page, savedText) {
  const draft = "pane-local unsaved draft";
  const editor = page.locator("article.tile").first().locator(".editor");
  const draftPane = await selectedPaneId(page);
  assert(draftPane, "draft pane is missing");
  await editor.fill(draft);
  await clickOtherTab(page, draftPane);
  await page.waitForFunction(
    (expected) => document.querySelector("article.tile .editor")?.value === expected,
    savedText,
  );
  await page.evaluate((paneId) => {
    const firstTileTabs = () => [...document.querySelector("article.tile").querySelectorAll('[role="tab"]')];
    firstTileTabs().find((tab) => tab.dataset.paneId === paneId)?.click();
  }, draftPane);
  await page.waitForFunction(
    (expected) => document.querySelector("article.tile .editor")?.value === expected,
    draft,
  );
}

async function firstTileTabPaneIds(page) {
  return page.evaluate(() => {
    const firstTileTabs = () => [...document.querySelector("article.tile").querySelectorAll('[role="tab"]')];
    return firstTileTabs().map((tab) => tab.dataset.paneId);
  });
}

async function selectedPaneId(page) {
  return page.evaluate(() => {
    const firstTileTabs = () => [...document.querySelector("article.tile").querySelectorAll('[role="tab"]')];
    return firstTileTabs().find((tab) => tab.getAttribute("aria-selected") === "true")?.dataset.paneId;
  });
}

async function clickOtherTab(page, paneId) {
  await page.evaluate((current) => {
    const firstTileTabs = () => [...document.querySelector("article.tile").querySelectorAll('[role="tab"]')];
    firstTileTabs().find((tab) => tab.dataset.paneId !== current)?.click();
  }, paneId);
}

async function waitForFirstTab(page, paneId) {
  await page.waitForFunction((expected) => {
    const firstTileTabs = () => [...document.querySelector("article.tile").querySelectorAll('[role="tab"]')];
    return firstTileTabs()[0]?.dataset.paneId === expected;
  }, paneId);
}

function assert(condition, message) {
  if (!condition) {
    throw new Error(message);
  }
}
