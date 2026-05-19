import { defineConfig, devices } from '@playwright/test';

export default defineConfig({
  testDir: './mobile-tests',
  reporter: [['list'], ['json', { outputFile: '../../.stealtheye/mobile/playwright-mobile-report.json' }]],
  use: {
    ...devices['iPhone 13'],
    browserName: 'chromium',
    trace: 'retain-on-failure',
    screenshot: 'only-on-failure'
  },
  outputDir: '../../.stealtheye/mobile/test-results'
});
