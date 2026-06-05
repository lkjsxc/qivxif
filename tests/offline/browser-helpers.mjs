export async function shellReady(page, timeout = 90000) {
  await page.waitForFunction(
    () => document.querySelector("section.app-shell.workspace") !== null,
    { timeout },
  );
  await page.getByText(/Capabilities: .*server\.health/).waitFor({ timeout });
}

export async function loadShell(page) {
  await page.goto("/", { waitUntil: "domcontentloaded" });
  await shellReady(page);
}

export async function reloadShell(page) {
  await page.reload({ waitUntil: "domcontentloaded" });
  await shellReady(page);
}

export async function setupOwner(page, events = [], { name = "admin", password = "secret" } = {}) {
  await waitForText(page, "Setup required", events, 60000);
  await page.getByRole("heading", { name: "Setup" }).waitFor({ timeout: 30000 });
  await page.locator("#setup-name").fill(name);
  await page.locator("#setup-password").fill(password);
  await page.locator("form.setup-form").evaluate((form) => form.requestSubmit());
  await page.locator(".header-status").getByText(`Signed in as ${name}`).waitFor({ timeout: 60000 });
}

export async function login(page, events = [], { name = "admin", password = "secret" } = {}) {
  const signedIn = page.locator(".header-status").getByText(`Signed in as ${name}`);
  if (await signedIn.isVisible().catch(() => false)) {
    return;
  }
  if (!(await page.locator("form.login").isVisible().catch(() => false))) {
    const openLogin = page.getByRole("button", { name: "Open login" });
    if (await openLogin.isVisible().catch(() => false)) {
      await openLogin.click({ force: true });
    }
  }
  await page.locator("form.login").waitFor({ timeout: 60000 });
  await page.locator("#login-name").fill(name);
  await page.locator("#login-password").fill(password);
  await page.locator("form.login").evaluate((form) => form.requestSubmit());
  try {
    await signedIn.waitFor({ timeout: 60000 });
  } catch (error) {
    const bodyText = await page.locator("body").innerText();
    throw new Error(`login failed\n${bodyText}\n${events.join("\n")}`);
  }
}

export function captureBrowserEvents(page) {
  const events = [];
  page.on("console", (message) => events.push(`console ${message.type()}: ${message.text()}`));
  page.on("pageerror", (error) => events.push(`pageerror: ${error.message}`));
  page.on("requestfailed", (request) => {
    events.push(`requestfailed: ${request.url()} ${request.failure()?.errorText}`);
  });
  return events;
}

export async function clickButton(page, name) {
  await page.getByRole("button", { name }).first().click({ force: true });
}

export async function waitForBodyText(page, pattern, timeout = 90000) {
  await page.waitForFunction(
    (source) => new RegExp(source).test(document.body.innerText),
    pattern.source,
    { timeout },
  );
}

export async function waitForText(page, value, events = [], timeout = 5000) {
  try {
    await page.getByText(value).first().waitFor({ timeout });
  } catch (error) {
    const bodyText = await page.locator("body").innerText();
    throw new Error(`${value} missing\n${bodyText}\n${events.join("\n")}`);
  }
}

export async function openShellTab(page, name) {
  const tile = page.locator("article.tile").first();
  await tile.waitFor();
  const selectedName = tabTitle(name);
  const existingIndex = await waitForExistingTab(page, selectedName);
  if (existingIndex >= 0) {
    await selectedTabInTile(page, selectedName, existingIndex);
    return;
  }
  await tile.getByRole("button", { exact: true, name: "Add tab" }).click({ force: true });
  try {
    await tile.locator(".tab-body:not([hidden]) .new-tab-panel").getByRole("button", { name }).click({ force: true });
  } catch (error) {
    const bodyText = await page.locator("body").innerText();
    const tabs = JSON.stringify(await tabSnapshot(page));
    throw new Error(`${name} chooser item missing\n${tabs}\n${bodyText}`);
  }
  await selectedTabInTile(page, selectedName, 0);
}

async function waitForExistingTab(page, name) {
  try {
    await page.waitForFunction((label) => {
      const labelOf = (tab) => tab.getAttribute("aria-label") ?? tab.textContent?.trim();
      return [...document.querySelectorAll("article.tile")].some((tile) => {
        return [...tile.querySelectorAll('[role="tab"]')].some((tab) => {
          return labelOf(tab) === label;
        });
      });
    }, name, { timeout: 2000 });
  } catch (error) {
    return -1;
  }
  return selectExistingTab(page, name);
}

async function selectExistingTab(page, name) {
  return page.evaluate((label) => {
    const labelOf = (tab) => tab.getAttribute("aria-label") ?? tab.textContent?.trim();
    const tiles = [...document.querySelectorAll("article.tile")];
    for (const [index, tile] of tiles.entries()) {
      const tabs = [...tile.querySelectorAll('[role="tab"]')];
      const tab = tabs.find((item) => labelOf(item) === label);
      if (!tab) {
        continue;
      }
      if (tab.getAttribute("aria-selected") !== "true") {
        tab.click();
      }
      return index;
    }
    return -1;
  }, name);
}

async function tabSnapshot(page) {
  return page.evaluate(() => {
    return [...document.querySelectorAll("article.tile")].map((tile) => {
      return [...tile.querySelectorAll('[role="tab"], button')].map((item) => {
        return {
          selected: item.getAttribute("aria-selected"),
          text: item.getAttribute("aria-label") ?? item.textContent?.trim(),
          type: item.getAttribute("role") ?? item.tagName.toLowerCase(),
        };
      });
    });
  });
}

async function tileLayoutSnapshot(page) {
  const rows = await readLocalStore(page, "tile_layout");
  return rows.find((row) => row.id === "tile_model");
}

export async function readLocalStore(page, name) {
  return page.evaluate(async (storeName) => {
    return window.__qivxifStorageDebug?.all(storeName) ?? [];
  }, name);
}

export async function waitForLocalStore(page, name, predicateSource, arg, timeout = 90000) {
  await page.waitForFunction(async ({ name, predicateSource, arg }) => {
    const rows = await window.__qivxifStorageDebug?.all(name);
    if (!rows) return false;
    return Function("rows", "arg", `return (${predicateSource})(rows, arg);`)(rows, arg);
  }, { arg, name, predicateSource }, { timeout });
}

export async function openServerNode(page, nodeId, timeout = 90000) {
  await page.waitForSelector("form.open-node input[name='nodeId'], #graph-node-id", { timeout });
  await page.locator("form.open-node input[name='nodeId'], #graph-node-id").first().fill(nodeId);
  await page.locator("form.open-node").first().evaluate((form) => form.requestSubmit());
}

async function selectedTabInTile(page, name, index) {
  try {
    await page.waitForFunction(({ index, label }) => {
      const labelOf = (tab) => tab.getAttribute("aria-label") ?? tab.textContent?.trim();
      const tile = document.querySelectorAll("article.tile")[index];
      return [...(tile?.querySelectorAll('[role="tab"]') ?? [])].some((tab) => {
        return labelOf(tab) === label && tab.getAttribute("aria-selected") === "true";
      });
    }, { index, label: name });
  } catch (error) {
    const bodyText = await page.locator("body").innerText();
    const tabs = JSON.stringify(await tabSnapshot(page));
    const layout = JSON.stringify(await tileLayoutSnapshot(page));
    throw new Error(`${name} tab was not selected\n${tabs}\n${layout}\n${bodyText}`);
  }
}

function tabTitle(name) {
  return {
    Graph: "Graph Node",
    Publish: "Publishing",
    Sync: "Sync Status",
  }[name] ?? name;
}
