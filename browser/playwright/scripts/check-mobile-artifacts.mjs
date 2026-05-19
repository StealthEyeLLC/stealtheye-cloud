import { existsSync, readFileSync } from 'node:fs';

const required = [
  '.stealtheye/mobile/artifacts/s7-mobile-playtest-summary.json',
  '.stealtheye/mobile/artifacts/s7-mobile-playtest-summary.md',
  '.stealtheye/mobile/artifacts/s7-mobile-playtest-console-errors.json',
  '.stealtheye/mobile/artifacts/s7-mobile-playtest-network-failures.json',
  '.stealtheye/mobile/artifacts/s7-mobile-playtest-state.json',
  '.stealtheye/mobile/artifacts/s7-mobile-playtest-link.txt',
  '.stealtheye/mobile/artifacts/s7-mobile-playtest.png'
];

const missing = required.filter((path) => !existsSync(path));
if (missing.length > 0) {
  console.error(`Missing mobile artifacts: ${missing.join(', ')}`);
  process.exit(1);
}

const summary = JSON.parse(readFileSync('.stealtheye/mobile/artifacts/s7-mobile-playtest-summary.json', 'utf8'));
if (summary.schema_version !== 'MobilePlaytestActivationV0') {
  console.error('mobile summary schema_version must be MobilePlaytestActivationV0');
  process.exit(1);
}
if (summary.status !== 'success') {
  console.error(`mobile summary status must be success, got ${summary.status}`);
  process.exit(1);
}
if (summary.lane !== 'mobile_browser_game_preview') {
  console.error('mobile summary lane is invalid');
  process.exit(1);
}
if (!Array.isArray(summary.input_replay) || !summary.input_replay.includes('touchscreen.tap')) {
  console.error('mobile input replay must include touchscreen.tap');
  process.exit(1);
}
if (!summary.final_state || summary.final_state.swipes < 1) {
  console.error('mobile final state must prove swipe replay');
  process.exit(1);
}
if ((summary.console_errors || []).length !== 0 || (summary.network_failures || []).length !== 0) {
  console.error('mobile proof captured console or network failures');
  process.exit(1);
}

console.log('PASS: S7 mobile artifacts validated');
