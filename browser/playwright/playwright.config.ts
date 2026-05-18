import { defineConfig } from '@playwright/test';

export default defineConfig({
  testDir: './tests',
  reporter: [['list'], ['json', { outputFile: '../../.stealtheye/browser/playwright-report.json' }]],
  use: {
    browserName: 'chromium',
    trace: 'retain-on-failure',
    screenshot: 'only-on-failure'
  },
  outputDir: '../../.stealtheye/browser/test-results'
});
