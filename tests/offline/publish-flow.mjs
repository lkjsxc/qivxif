import { createRequire } from "node:module";
import { captureBrowserEvents, openShellTab, waitForText } from "./browser-helpers.mjs";

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
  const context = await browser.newContext({ baseURL: base });
  const page = await context.newPage();
  const browserEvents = captureBrowserEvents(page);
  await loadShell(page);
  await login(page, browserEvents);
  await page.keyboard.press("Control+K");
  await waitForText(page, "Command palette", browserEvents);
  await page.getByRole("button", { name: "Open settings" }).click();
  await waitForText(page, "Account: admin", browserEvents);

  await context.setOffline(true);
  await openShellTab(page, "Publish");
  await page.getByLabel("Blog title").fill(title);
  await page.getByRole("button", { name: "Create blog draft" }).click();
  await waitForQueuedAtLeast(page, 3, browserEvents);
  await openShellTab(page, "Editor");
  await page.locator(".editor").fill(body);
  await page.getByRole("button", { name: "Save text event" }).click();
  await waitForQueuedAtLeast(page, 4, browserEvents);
  await openShellTab(page, "Publish");
  await page.getByLabel("Slug").fill(slug);
  await page.getByLabel("Summary").fill("offline summary");
  await page.getByRole("button", { name: "Publish draft" }).click();
  await waitForQueuedAtLeast(page, 5, browserEvents);
  assert((await publicStatus(slug)) === 404, "server published while browser was offline");

  await page.reload({ waitUntil: "domcontentloaded" });
  await openShellTab(page, "Publish");
  await waitForText(page, `Draft: ${title}`, browserEvents);
  await waitForQueuedAtLeast(page, 5, browserEvents);
  await openShellTab(page, "Editor");
  assert((await page.locator(".editor").inputValue()) === body, "draft body did not reload");

  await context.setOffline(false);
  await openShellTab(page, "Welcome");
  await page.getByRole("button", { name: "Flush queue" }).click();
  await waitForText(page, "Queued: 0", browserEvents, 15000);
  await openShellTab(page, "Publish");
  await waitForText(page, "State: published", browserEvents);
  const html = await publicHtml(slug);
  assert(html.includes("<h1>Offline Post</h1>"), "public heading missing");
  assert(html.includes("Published &lt;body&gt;"), "public body was not escaped");

  const second = await browser.newContext({ baseURL: base });
  const secondPage = await second.newPage();
  await secondPage.goto(`/@admin/${slug}`, { waitUntil: "domcontentloaded" });
  await secondPage.getByText("Published <body>").waitFor();
  await second.close();

  await page.getByRole("button", { name: "Unpublish" }).click();
  await openShellTab(page, "Publish");
  await waitForText(page, "State: unpublished", browserEvents, 15000);
  assert((await publicStatus(slug)) === 404, "unpublished post stayed public");

  await context.setOffline(true);
  await openShellTab(page, "Social");
  await page.getByLabel("Short post").fill("offline social post");
  await page.getByRole("button", { name: "Create short post" }).click();
  await waitForQueuedAtLeast(page, 1, browserEvents);
  await waitForText(page, "offline social post", browserEvents);
  await page.reload({ waitUntil: "domcontentloaded" });
  await openShellTab(page, "Social");
  await waitForText(page, "offline social post", browserEvents);

  await context.setOffline(false);
  await openShellTab(page, "Welcome");
  await page.getByRole("button", { name: "Flush queue" }).click();
  await waitForText(page, "Queued: 0", browserEvents, 15000);
  const feed = await homeFeed(context);
  assert(feed.items.some((item) => item.body === "offline social post"), "feed item missing");
  await context.close();
} finally {
  await browser.close();
}

async function loadShell(page) {
  await page.goto("/", { waitUntil: "domcontentloaded" });
  await page.locator(".workspace").waitFor();
  await page.getByText(/Capabilities: .*server\.health/).waitFor();
  await serviceWorkerReady(page);
  await page.reload({ waitUntil: "domcontentloaded" });
  await page.getByText(/Capabilities: .*publish\.blog/).waitFor();
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
    const bodyText = await page.locator("body").innerText();
    throw new Error(`login failed\n${bodyText}\n${browserEvents.join("\n")}`);
  }
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
