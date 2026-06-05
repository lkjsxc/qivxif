import { createRequire } from "node:module";
import {
  captureBrowserEvents,
  loadShell,
  login,
  openShellTab,
  reloadShell,
  waitForLocalStore,
  waitForText,
} from "./browser-helpers.mjs";

const require = createRequire(import.meta.url);
const { chromium } = require("playwright-core");

const base = process.env.QIVXIF_E2E_BASE ?? "http://127.0.0.1:8080";
const browserPath = process.env.QIVXIF_BROWSER ?? "/usr/bin/chromium";
const title = "Offline Post";
const body = "# Offline Post\n\nPublished <body>";
const slug = "offline-post";

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
  await login(page, browserEvents);
  await page.keyboard.press("Control+K");
  await waitForText(page, "Command palette", browserEvents);
  await page.getByLabel("Search commands").fill("settings");
  assert((await page.getByRole("button", { name: "Open graph" }).count()) === 0, "palette filter kept graph visible");
  await page.getByRole("button", { name: "Open settings" }).click();
  await waitForText(page, "Signed in as admin", browserEvents);

  await context.setOffline(true);
  await page.locator("article.tile").first().getByRole("button", { name: "Add tab" }).click({ force: true });
  await page.locator(".tab-body:not([hidden]) .new-tab-panel").getByRole("button", { name: /Publishing/ }).click({ force: true });
  await page.locator(".tab-panel.publish").waitFor({ timeout: 90000 });
  await page.locator("#publish-title").fill(title);
  await page.locator("form.publish-draft-form").evaluate((form) => form.requestSubmit());
  await waitForLocalStore(page, "tile_layout", "(rows) => rows.some((row) => row.id === 'current_blog_post' && row.node_id)");
  await waitForLocalStore(page, "events", "(rows) => rows.length >= 3", null, 30000);
  await page.locator("textarea.editor").first().fill(body, { force: true });
  await clickByText(page, "Save text event");
  await waitForLocalStore(page, "events", "(rows) => rows.length >= 4", null, 30000);
  await openShellTab(page, "Publish");
  await page.waitForFunction(() => document.querySelector("form.publish-submit-form") !== null, { timeout: 90000 });
  assert(await submitPublish(page), "publish form was not available");
  await waitForLocalStore(page, "events", "(rows) => rows.length >= 5", null, 30000);
  assert((await publicStatus(slug)) === 404, "server published while browser was offline");

  await context.setOffline(false);
  await reloadShell(page);
  await waitForLocalStore(page, "events", "(rows) => rows.length >= 5", null, 30000);
  await clickByText(page, "Flush queue");
  await waitForLocalStore(
    page,
    "events",
    "(rows) => rows.every((entry) => entry.status !== 'dirty' && entry.status !== 'pending_validation')",
    null,
    120000,
  );
  const publishedStatus = await publicStatus(slug);
  assert(publishedStatus === 200 || publishedStatus === 404, `unexpected public status ${publishedStatus}`);
  await context.close();
} finally {
  await browser.close();
}

async function clickByText(page, text) {
  await page.evaluate((label) => {
    const button = [...document.querySelectorAll("button")].find(
      (entry) => entry.textContent?.trim() === label,
    );
    button?.click();
  }, text);
}

function submitPublish(page) {
  return page.evaluate(({ slugValue, summaryValue }) => {
    const slugInput = document.querySelector("#publish-slug");
    const summaryInput = document.querySelector("#publish-summary");
    const form = document.querySelector("form.publish-submit-form");
    if (!slugInput || !summaryInput || !form) return false;
    slugInput.value = slugValue;
    summaryInput.value = summaryValue;
    form.requestSubmit();
    return true;
  }, { slugValue: slug, summaryValue: "offline summary" });
}

async function publicStatus(slugValue) {
  const response = await fetch(`${base}/@admin/${slugValue}`);
  return response.status;
}

function assert(condition, message) {
  if (!condition) throw new Error(message);
}
