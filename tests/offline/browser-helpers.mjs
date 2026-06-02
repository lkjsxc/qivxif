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

export async function waitForBodyText(page, pattern, timeout = 30000) {
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
  await tile.getByRole("button", { exact: true, name: "+" }).click({ force: true });
  try {
    await tile.locator(".tab-chooser").getByRole("button", { exact: true, name }).click({ force: true });
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
      return [...document.querySelectorAll("article.tile")].some((tile) => {
        return [...tile.querySelectorAll('[role="tab"]')].some((tab) => {
          return tab.textContent?.trim() === label;
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
    const tiles = [...document.querySelectorAll("article.tile")];
    for (const [index, tile] of tiles.entries()) {
      const tabs = [...tile.querySelectorAll('[role="tab"]')];
      const tab = tabs.find((item) => item.textContent?.trim() === label);
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
          text: item.textContent?.trim(),
          type: item.getAttribute("role") ?? item.tagName.toLowerCase(),
        };
      });
    });
  });
}

export async function openServerNode(page, nodeId) {
  await page.waitForFunction((id) => {
    const labels = [...document.querySelectorAll("label")];
    const label = labels.find((entry) => {
      return entry.textContent?.includes("Server node id") && entry.offsetParent !== null;
    });
    const input = label?.querySelector("input");
    const form = input?.closest("form");
    if (!input || !form) {
      return false;
    }
    input.value = id;
    input.dispatchEvent(new Event("input", { bubbles: true }));
    form.requestSubmit();
    return true;
  }, nodeId);
}

async function selectedTabInTile(page, name, index) {
  try {
    await page.waitForFunction(({ index, label }) => {
      const tile = document.querySelectorAll("article.tile")[index];
      return [...(tile?.querySelectorAll('[role="tab"]') ?? [])].some((tab) => {
        return tab.textContent?.trim() === label && tab.getAttribute("aria-selected") === "true";
      });
    }, { index, label: name });
  } catch (error) {
    const bodyText = await page.locator("body").innerText();
    throw new Error(`${name} tab was not selected\n${bodyText}`);
  }
}

function tabTitle(name) {
  return {
    Graph: "Graph Node",
    Publish: "Publishing",
    Sync: "Sync Status",
  }[name] ?? name;
}
