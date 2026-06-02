import { createRequire } from "node:module";
import {
  captureBrowserEvents,
  loadShell,
  reloadShell,
  setupOwner,
} from "./browser-helpers.mjs";

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
  const context = await browser.newContext({ baseURL: base, serviceWorkers: "block" });
  const page = await context.newPage();
  page.setDefaultTimeout(90000);
  const events = captureBrowserEvents(page);
  await loadShell(page);
  await setupOwner(page, events);
  await page.getByRole("tab", { name: "Welcome" }).waitFor();
  await page.getByRole("button", { name: "Create text node" }).waitFor();
  await page.getByRole("button", { name: "New tab" }).click({ force: true });
  await page
    .locator("article.tile")
    .first()
    .locator(".tab-chooser")
    .getByRole("button", { name: "Settings" })
    .click({ force: true });
  await page.getByRole("tab", { name: "Settings" }).waitFor();
  await reloadShell(page);
  await page.locator(".header-status").getByText("Signed in as admin").waitFor({ timeout: 60000 });
  await page.getByRole("tab", { name: "Welcome" }).click({ force: true });
  await page.getByRole("button", { name: "Create text node" }).waitFor();
  await context.close();
} finally {
  await browser.close();
}
