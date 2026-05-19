import { test, expect } from '@playwright/test';
import { readFileSync } from 'node:fs';

test('proof canvas exposes required public proof panels', async ({ page }) => {
  const html = readFileSync('public/proof/index.html', 'utf8');
  await page.setContent(html);

  await expect(page.getByRole('heading', { name: 'StealthEye Cloud Proof Canvas' })).toBeVisible();
  await expect(page.locator('main[data-stealtheye="proof-canvas"]')).toBeVisible();

  const panelKinds = await page.locator('section[data-panel-kind]').evaluateAll((sections) =>
    sections.map((section) => section.getAttribute('data-panel-kind')),
  );

  expect(panelKinds).toEqual([
    'mission',
    'ci',
    'browser',
    'seal',
    'relay',
    'worker',
    'decision',
  ]);
});
