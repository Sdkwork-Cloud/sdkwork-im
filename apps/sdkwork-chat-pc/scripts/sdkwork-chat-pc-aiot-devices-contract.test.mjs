import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const appRoot = path.resolve(__dirname, '..');
const devicesServicePath = path.join(
  appRoot,
  'packages',
  'sdkwork-clawchat-pc-devices',
  'src',
  'services',
  'DeviceService.ts',
);

const source = readFileSync(devicesServicePath, 'utf8');

assert.match(
  source,
  /from ['"]@sdkwork\/aiot-app-sdk['"]/u,
  'Claw Chat devices service must consume sdkwork-aiot-app-sdk for device management.',
);

assert.match(
  source,
  /iot\.devices\.list/u,
  'Claw Chat devices service must list devices through client.iot.devices.list.',
);

assert.match(
  source,
  /iot\.devices\.retrieve/u,
  'Claw Chat devices service must retrieve devices through client.iot.devices.retrieve.',
);

for (const forbidden of [
  /\bmockDevices\b/u,
  /\bMockDeviceService\b/u,
  /setTimeout\(/u,
  /Date\.now\(\)/u,
  /push\(newDevice\)/u,
  /splice\(/u,
]) {
  assert.doesNotMatch(
    source,
    forbidden,
    `Claw Chat devices service must not keep local mock device ownership: ${forbidden}.`,
  );
}

assert.match(
  source,
  /AIoT App SDK does not expose \$\{method\}/u,
  'Missing mutation support must be surfaced through a clear AIoT SDK capability gap message.',
);

for (const missingMethod of [
  'devices.create',
  'devices.update',
  'devices.delete',
  'devices.activate',
  'devices.agentBindings.create',
  'devices.agentBindings.delete',
]) {
  assert.match(
    source,
    new RegExp(`unsupportedAiotAppMethod\\('${missingMethod.replaceAll('.', '\\.')}'\\)`, 'u'),
    `Missing ${missingMethod} support must be surfaced as an AIoT SDK capability gap instead of fake local success.`,
  );
}

console.log('sdkwork chat pc AIoT devices SDK contract passed.');
