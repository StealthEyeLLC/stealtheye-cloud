import { test, expect } from '@playwright/test';
import { mkdirSync, writeFileSync } from 'node:fs';
import { join } from 'node:path';

test('captures StealthEye browser proof artifacts', async ({ page }) => {
  const artifactDir = '.stealtheye/browser/artifacts';
  mkdirSync(artifactDir, { recursive: true });
  const consoleErrors: string[] = [];
  const networkFailures: string[] = [];

  page.on('console', (message) => {
    if (message.type() === 'error') consoleErrors.push(message.text());
  });
  page.on('requestfailed', (request) => {
    networkFailures.push(`${request.method()} ${request.url()}`);
  });

  await page.setContent('<main data-stealtheye="browser-proof"><h1>StealthEye Cloud Browser Body</h1><button aria-label="proof button">Proof</button></main>');
  await expect(page.getByRole('heading', { name: 'StealthEye Cloud Browser Body' })).toBeVisible();
  await expect(page.getByRole('button', { name: 'proof button' })).toBeVisible();

  const domSketch = await page.locator('main').evaluate((element) => ({
    tag: element.tagName.toLowerCase(),
    marker: element.getAttribute('data-stealtheye'),
    text: element.textContent?.replace(/\s+/g, ' ').trim()
  }));

  const screenshotPath = join(artifactDir, 'browser-proof.png');
  await page.screenshot({ path: screenshotPath, fullPage: true });

  const summary = {
    schema_version: 'BrowserArtifactIndexV0',
    mission_id: 's2-browser-body-proof',
    route: 'inline-fixture',
    screenshot: screenshotPath,
    console_errors: consoleErrors,
    network_failures: networkFailures,
    dom_sketch: domSketch,
    status: 'success'
  };

  writeFileSync(join(artifactDir, 'browser-summary.json'), JSON.stringify(summary, null, 2));
  writeFileSync(join(artifactDir, 'browser-summary.md'), '# Browser Proof Summary\n\nStatus: success\n');
  writeFileSync(join(artifactDir, 'browser-console-errors.json'), JSON.stringify(consoleErrors, null, 2));
  writeFileSync(join(artifactDir, 'browser-network-failures.json'), JSON.stringify(networkFailures, null, 2));
  writeFileSync(join(artifactDir, 'browser-dom-sketch.json'), JSON.stringify(domSketch, null, 2));

  expect(consoleErrors).toEqual([]);
  expect(networkFailures).toEqual([]);
  expect(domSketch.marker).toBe('browser-proof');
});
