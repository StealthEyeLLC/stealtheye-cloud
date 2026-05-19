import { test, expect } from '@playwright/test';
import { mkdirSync, writeFileSync } from 'node:fs';
import { resolve, join } from 'node:path';

const artifactDir = '.stealtheye/mobile/artifacts';
const previewPath = resolve('public/previews/s7-mobile-playtest/index.html');

test('S7 mobile preview supports tap and swipe replay with artifacts', async ({ page }) => {
  mkdirSync(artifactDir, { recursive: true });
  const consoleErrors: string[] = [];
  const networkFailures: string[] = [];

  page.on('console', (message) => {
    if (message.type() === 'error') consoleErrors.push(message.text());
  });
  page.on('requestfailed', (request) => {
    networkFailures.push(`${request.method()} ${request.url()}`);
  });

  await page.goto(`file://${previewPath}`);
  await expect(page.getByRole('heading', { name: 'StealthEye Cloud S7 Mobile Playtest' })).toBeVisible();
  await expect(page.locator('[data-stealtheye="s7-mobile-playtest"]')).toBeVisible();

  const arena = page.getByLabel('mobile playtest arena');
  const box = await arena.boundingBox();
  if (!box) throw new Error('mobile arena bounding box unavailable');

  await page.touchscreen.tap(box.x + box.width * 0.25, box.y + box.height * 0.35);
  await page.mouse.move(box.x + box.width * 0.25, box.y + box.height * 0.35);
  await page.mouse.down();
  await page.mouse.move(box.x + box.width * 0.78, box.y + box.height * 0.72, { steps: 8 });
  await page.mouse.up();

  const state = await page.evaluate(() => (window as any).__STEALTHEYE_MOBILE_STATE__);
  expect(state.taps).toBeGreaterThanOrEqual(2);
  expect(state.swipes).toBeGreaterThanOrEqual(1);
  expect(consoleErrors).toEqual([]);
  expect(networkFailures).toEqual([]);

  const screenshotPath = join(artifactDir, 's7-mobile-playtest.png');
  await page.screenshot({ path: screenshotPath, fullPage: true });

  const summary = {
    schema_version: 'MobilePlaytestActivationV0',
    mission_id: 's7-first-real-activations',
    lane: 'mobile_browser_game_preview',
    preview_source_path: 'public/previews/s7-mobile-playtest/index.html',
    playtest_link_artifact: join(artifactDir, 's7-mobile-playtest-link.txt'),
    viewport: page.viewportSize() ? { ...page.viewportSize(), is_mobile: true, has_touch: true } : { width: 390, height: 844, is_mobile: true, has_touch: true },
    input_replay: ['touchscreen.tap', 'mouse.swipe.compatibility_replay'],
    artifacts: [
      screenshotPath,
      join(artifactDir, 's7-mobile-playtest-console-errors.json'),
      join(artifactDir, 's7-mobile-playtest-network-failures.json'),
      join(artifactDir, 's7-mobile-playtest-state.json')
    ],
    console_errors: consoleErrors,
    network_failures: networkFailures,
    final_state: state,
    status: 'success'
  };

  writeFileSync(join(artifactDir, 's7-mobile-playtest-summary.json'), JSON.stringify(summary, null, 2));
  writeFileSync(join(artifactDir, 's7-mobile-playtest-summary.md'), '# S7 Mobile Playtest Summary\n\nStatus: success\n');
  writeFileSync(join(artifactDir, 's7-mobile-playtest-console-errors.json'), JSON.stringify(consoleErrors, null, 2));
  writeFileSync(join(artifactDir, 's7-mobile-playtest-network-failures.json'), JSON.stringify(networkFailures, null, 2));
  writeFileSync(join(artifactDir, 's7-mobile-playtest-state.json'), JSON.stringify(state, null, 2));
  writeFileSync(join(artifactDir, 's7-mobile-playtest-link.txt'), 'public/previews/s7-mobile-playtest/index.html\n');
});
