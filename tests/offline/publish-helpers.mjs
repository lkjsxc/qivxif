import { openShellTab, waitForText } from "./browser-helpers.mjs";

export async function createPublishedPost(page, input, events = []) {
  await openShellTab(page, "Publish");
  await page.getByLabel("Blog title").fill(input.title);
  await page.getByRole("button", { name: "Create blog draft" }).click();
  await waitForText(page, `Draft: ${input.title}`, events, 15000);
  await openShellTab(page, "Publish");
  await page.getByLabel("Slug").fill(input.slug);
  await page.getByLabel("Summary").fill(input.summary);
  await page.getByRole("button", { name: "Publish draft" }).click();
  await waitForText(page, "State: published", events, 15000);
}

export async function expectPublishSlugConflict(page, input, events = []) {
  await openShellTab(page, "Publish");
  await page.getByLabel("Blog title").fill(input.title);
  await page.getByRole("button", { name: "Create blog draft" }).click();
  await waitForText(page, `Draft: ${input.title}`, events, 15000);
  await page.getByLabel("Slug").fill(input.slug);
  await page.getByLabel("Summary").fill(input.summary);
  await page.getByRole("button", { name: "Publish draft" }).click();
  await waitForText(page, "Rejected: 1", events, 15000);
  await openShellTab(page, "Sync");
  await waitForText(page, "Event: publish.post rejected", events, 15000);
  await waitForText(page, "Error: publish.slug_conflict", events, 15000);
}
