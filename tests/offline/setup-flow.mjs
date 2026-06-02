import { createRequire } from "node:module";

const require = createRequire(import.meta.url);
const { chromium } = require("playwright-core");

const base = process.env.QIVXIF_E2E_BASE ?? "http://127.0.0.1:8080";
const browserPath = process.env.QIVXIF_BROWSER ?? "/usr/bin/chromium";

const browser = await chromium.launch({
  executablePath: browserPath,
  headless: true,
  args: ["--no-sandbox", "--disable-dev-shm-usage"],
});

try {
  const context = await browser.newContext({ baseURL: base });
  const page = await context.newPage();
  const events = captureBrowserEvents(page);
  await page.goto("/", { waitUntil: "domcontentloaded" });
  await page.locator(".app-shell").waitFor();
  await page.locator(".app-header").waitFor();
  await page.locator(".tile-grid").waitFor();
  await page.locator(".tile").first().waitFor();
  await waitForText(page, "Setup required", events);
  await page.getByRole("tab", { name: "Setup" }).waitFor();
  await page.getByLabel("Name").fill("admin");
  await page.getByLabel("Password").fill("secret");
  await page.getByRole("button", { name: "Create owner account" }).click();
  await waitForText(page, "Signed in as admin", events, 15000);
  await page.getByRole("tab", { name: "Welcome" }).waitFor();
  await page.getByRole("button", { name: "Create text node" }).waitFor();
  await page.getByRole("button", { name: "New tab" }).click();
  await page.locator("article.tile").first().locator(".tab-chooser").getByRole("button", { name: "Settings" }).click();
  await page.getByRole("tab", { name: "Settings" }).waitFor();
  await page.reload({ waitUntil: "domcontentloaded" });
  await waitForText(page, "Signed in as admin", events, 15000);
  await page.getByRole("tab", { name: "Welcome" }).waitFor();
  await context.close();
} finally {
  await browser.close();
}

function captureBrowserEvents(page) {
  const events = [];
  page.on("console", (message) => events.push(`console ${message.type()}: ${message.text()}`));
  page.on("pageerror", (error) => events.push(`pageerror: ${error.message}`));
  page.on("requestfailed", (request) => {
    events.push(`requestfailed: ${request.url()} ${request.failure()?.errorText}`);
  });
  return events;
}

async function waitForText(page, value, events = [], timeout = 5000) {
  try {
    await page.getByText(value).waitFor({ timeout });
  } catch (error) {
    const bodyText = await page.locator("body").innerText();
    throw new Error(`${value} missing\n${bodyText}\n${events.join("\n")}`);
  }
}
