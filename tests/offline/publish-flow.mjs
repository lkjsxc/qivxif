import { createRequire } from "node:module";
import {
  captureBrowserEvents,
  loadShell,
  login,
  openShellTab,
  reloadShell,
  waitForText,
} from "./browser-helpers.mjs";
import { createPublishedPost, expectPublishSlugConflict } from "./publish-helpers.mjs";

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
  await page.locator("article.tile").first().locator(".new-tab-panel").getByRole("button", { name: /Publishing/ }).click({ force: true });
  await page.locator(".tab-panel.publish").waitFor({ timeout: 90000 });
  await page.locator("#publish-title").fill(title);
  await page.locator("form.publish-draft-form").evaluate((form) => form.requestSubmit());
  await page.waitForFunction(async () => {
    const db = await new Promise((resolve, reject) => {
      const request = indexedDB.open("qivxif", 4);
      request.onerror = () => reject(request.error);
      request.onsuccess = () => resolve(request.result);
    });
    const draft = await new Promise((resolve, reject) => {
      const call = db.transaction("tile_layout", "readonly").objectStore("tile_layout").get("current_blog_post");
      call.onerror = () => reject(call.error);
      call.onsuccess = () => resolve(call.result);
    });
    return Boolean(draft?.node_id);
  }, null, { timeout: 90000 });
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
    return events.length >= 3;
  }, null, { timeout: 30000 });
  await page.locator("textarea.editor").first().fill(body, { force: true });
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
    return events.length >= 4;
  }, null, { timeout: 30000 });
  await page.waitForFunction(() => document.querySelector("form.publish-submit-form") !== null, {
    timeout: 90000,
  });
  const published = await page.evaluate(
    ({ slugValue, summaryValue }) => {
      const slugInput = document.querySelector("#publish-slug");
      const summaryInput = document.querySelector("#publish-summary");
      const form = document.querySelector("form.publish-submit-form");
      if (!slugInput || !summaryInput || !form) {
        return false;
      }
      slugInput.value = slugValue;
      summaryInput.value = summaryValue;
      form.requestSubmit();
      return true;
    },
    { slugValue: slug, summaryValue: "offline summary" },
  );
  assert(published, "publish form was not available");
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
    return events.length >= 5;
  }, null, { timeout: 30000 });
  assert((await publicStatus(slug)) === 404, "server published while browser was offline");

  await context.setOffline(false);
  await reloadShell(page);
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
    return events.length >= 5;
  }, null, { timeout: 30000 });

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
  const publishedStatus = await publicStatus(slug);
  assert(publishedStatus === 200 || publishedStatus === 404, `unexpected public status ${publishedStatus}`);
  await context.close();
} finally {
  await browser.close();
}

async function waitForQueuedAtLeast(page, minimum, events = [], timeout = 5000) {
  try {
    await page.waitForFunction((count) => {
      const match = document.body.textContent.match(/Queued: (\d+)/);
      return match && Number(match[1]) >= count;
    }, minimum, { timeout });
  } catch (error) {
    const bodyText = await page.locator("body").innerText();
    throw new Error(`queued count below ${minimum}\n${bodyText}\n${events.join("\n")}`);
  }
}

async function publicStatus(slugValue) {
  const response = await fetch(`${base}/@admin/${slugValue}`);
  return response.status;
}

async function publicHtml(slugValue) {
  const response = await fetch(`${base}/@admin/${slugValue}`);
  assert(response.status === 200, `public route returned ${response.status}`);
  return response.text();
}

async function homeFeed(context) {
  const cookies = await context.cookies(base);
  const cookie = cookies.map((item) => `${item.name}=${item.value}`).join("; ");
  const response = await fetch(`${base}/api/feed/home?limit=10`, {
    headers: { cookie },
  });
  assert(response.status === 200, `feed route returned ${response.status}`);
  return (await response.json()).payload;
}

function assert(condition, message) {
  if (!condition) {
    throw new Error(message);
  }
}
