import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const appRoot = path.resolve(__dirname, '..');
const repoRoot = path.resolve(appRoot, '..', '..');
const aiotDevicePackage = path.resolve(
  appRoot,
  '..',
  '..',
  '..',
  'sdkwork-aiot',
  'apps',
  'sdkwork-aiot-pc',
  'packages',
  'sdkwork-aiot-pc-console-device',
  'package.json',
);
const aiotIotPackage = path.resolve(
  appRoot,
  '..',
  '..',
  '..',
  'sdkwork-aiot',
  'apps',
  'sdkwork-aiot-pc',
  'packages',
  'sdkwork-aiot-pc-console-iot',
  'package.json',
);
const crawChatDevicePackage = path.resolve(
  appRoot,
  'packages',
  'sdkwork-clawchat-pc-devices',
  'package.json',
);
const crawChatDeviceService = path.resolve(
  appRoot,
  'packages',
  'sdkwork-clawchat-pc-devices',
  'src',
  'services',
  'DeviceService.ts',
);

function readJson(filePath) {
  return JSON.parse(readFileSync(filePath, 'utf8'));
}

function readText(filePath) {
  return readFileSync(filePath, 'utf8');
}

function readRepoText(relativePath) {
  return readText(path.join(repoRoot, relativePath));
}

const devicePackage = readJson(aiotDevicePackage);
const iotPackage = readJson(aiotIotPackage);
const crawChatDevicePackageJson = readJson(crawChatDevicePackage);

assert.equal(
  devicePackage.sdkwork?.product,
  'sdkwork-aiot',
  'Canonical device package must live in the sdkwork-aiot product workspace.',
);
assert.ok(
  Array.isArray(devicePackage.sdkwork?.supersedes) && devicePackage.sdkwork.supersedes.includes('@sdkwork/device-pc-react'),
  '@sdkwork/aiot-pc-console-device must supersede the legacy @sdkwork/device-pc-react package.',
);
assert.equal(
  devicePackage.dependencies?.['@sdkwork/aiot-app-sdk'],
  'workspace:*',
  '@sdkwork/aiot-pc-console-device must depend on @sdkwork/aiot-app-sdk for canonical device catalog integration.',
);

assert.equal(
  iotPackage.sdkwork?.product,
  'sdkwork-aiot',
  'Canonical IoT package must live in the sdkwork-aiot product workspace.',
);
assert.ok(
  Array.isArray(iotPackage.sdkwork?.supersedes) && iotPackage.sdkwork.supersedes.includes('@sdkwork/iot-pc-react'),
  '@sdkwork/aiot-pc-console-iot must supersede the legacy @sdkwork/iot-pc-react package.',
);

const aiotDeviceServiceSource = readText(
  path.resolve(
    appRoot,
    '..',
    '..',
    '..',
    'sdkwork-aiot',
    'apps',
    'sdkwork-aiot-pc',
    'packages',
    'sdkwork-aiot-pc-console-device',
    'src',
    'device-service.ts',
  ),
);
assert.match(
  aiotDeviceServiceSource,
  /from\s+["']@sdkwork\/aiot-app-sdk["']/u,
  'Canonical device service must consume sdkwork-aiot-app-sdk.',
);
assert.match(
  aiotDeviceServiceSource,
  /\.iot\.devices\.list\s*\(/u,
  'Canonical device service must list devices through client.iot.devices.list.',
);
assert.doesNotMatch(
  aiotDeviceServiceSource,
  /\bfetch\s*\(/u,
  'Canonical device service must not use raw fetch.',
);

const aiotIotServiceSource = readText(
  path.resolve(
    appRoot,
    '..',
    '..',
    '..',
    'sdkwork-aiot',
    'apps',
    'sdkwork-aiot-pc',
    'packages',
    'sdkwork-aiot-pc-console-iot',
    'src',
    'iot-service.ts',
  ),
);
assert.match(
  aiotIotServiceSource,
  /from\s+["']@sdkwork\/aiot-app-sdk["']/u,
  'Canonical IoT service must consume sdkwork-aiot-app-sdk.',
);
assert.match(
  aiotIotServiceSource,
  /\.iot\.devices\.list\s*\(/u,
  'Canonical IoT service must load fleet nodes through client.iot.devices.list.',
);
assert.doesNotMatch(
  aiotIotServiceSource,
  /\bfetch\s*\(/u,
  'Canonical IoT service must not use raw fetch.',
);

assert.equal(
  crawChatDevicePackageJson.dependencies?.['@sdkwork/aiot-app-sdk'],
  'workspace:*',
  'Craw Chat device bridge must depend on @sdkwork/aiot-app-sdk for AIoT device reads.',
);
assert.equal(
  crawChatDevicePackageJson.dependencies?.['@sdkwork/aiot-backend-sdk'],
  'workspace:*',
  'Craw Chat device bridge must depend on @sdkwork/aiot-backend-sdk for AIoT device mutations.',
);

const crawChatDeviceServiceSource = readText(crawChatDeviceService);
assert.match(
  crawChatDeviceServiceSource,
  /from\s+["']@sdkwork\/aiot-app-sdk["']/u,
  'Craw Chat device bridge must consume sdkwork-aiot app SDK for user-visible reads.',
);
assert.match(
  crawChatDeviceServiceSource,
  /from\s+["']@sdkwork\/aiot-backend-sdk["']/u,
  'Craw Chat device bridge must consume sdkwork-aiot backend SDK for device operations.',
);
for (const requiredCall of [
  /\.iot\.devices\.create\s*\(/u,
  /\.iot\.devices\.update\s*\(/u,
  /\.iot\.devices\.delete\s*\(/u,
  /\.iot\.devices\.twin\.update\s*\(/u,
  /\.iot\.devices\.commands\.list\s*\(/u,
  /\.iot\.devices\.commands\.cancel\s*\(/u,
]) {
  assert.match(
    crawChatDeviceServiceSource,
    requiredCall,
    `Craw Chat device bridge must route device operations through AIoT backend SDK: ${requiredCall}`,
  );
}
assert.doesNotMatch(
  crawChatDeviceServiceSource,
  /unsupportedAiotAppMethod/u,
  'Craw Chat device bridge must not keep unsupported AIoT app method placeholders.',
);
assert.doesNotMatch(
  crawChatDeviceServiceSource,
  /\bfetch\s*\(/u,
  'Craw Chat device bridge must not use raw fetch.',
);
assert.doesNotMatch(
  crawChatDeviceServiceSource,
  /\/im\/v3\/api\/device|\/im\/v3\/api\/devices/u,
  'Craw Chat device bridge must not call retired Craw Chat IM device APIs.',
);

const rootCargoSource = readRepoText('Cargo.toml');
const localMinimalCargoSource = readRepoText('services/local-minimal-node/Cargo.toml');
const sessionGatewayCargoSource = readRepoText('services/session-gateway/Cargo.toml');
const imPlatformCargoSource = readRepoText('crates/im-platform-contracts/Cargo.toml');
const imPlatformExportsSource = readRepoText('crates/im-platform-contracts/src/lib.rs');
const imPlatformProviderSource = readRepoText('crates/im-platform-contracts/src/provider.rs');
const localMinimalBuildSource = readRepoText('services/local-minimal-node/src/node/build.rs');
const localMinimalNodeSource = readRepoText('services/local-minimal-node/src/node.rs');

for (const retiredRustMember of [
  'adapters/iot-access-local',
  'adapters/iot-mqtt',
  'crates/craw-chat-contract-iot',
]) {
  assert.doesNotMatch(
    rootCargoSource,
    new RegExp(retiredRustMember.replace(/[.*+?^${}()|[\]\\]/g, '\\$&')),
    `Craw Chat Rust workspace must remove retired local device/IOT member ${retiredRustMember}.`,
  );
}

for (const [label, source] of [
  ['local-minimal-node Cargo.toml', localMinimalCargoSource],
  ['session-gateway Cargo.toml', sessionGatewayCargoSource],
  ['im-platform-contracts Cargo.toml', imPlatformCargoSource],
]) {
  assert.doesNotMatch(
    source,
    /im-adapter-iot-access-local|im-adapter-iot-mqtt|craw-chat-contract-iot/u,
    `${label} must not depend on retired Craw Chat-owned device/IOT crates.`,
  );
}

for (const [label, source] of [
  ['im-platform-contracts exports', imPlatformExportsSource],
  ['im-platform-contracts provider contracts', imPlatformProviderSource],
  ['local-minimal-node runtime build', localMinimalBuildSource],
]) {
  assert.doesNotMatch(
    source,
    /DeviceAccessProvider|IotProtocolAdapter|DeviceTwin|DeviceSubject|iot-access-local|iot-mqtt/u,
    `${label} must not retain Craw Chat-owned device/IOT provider or twin contracts.`,
  );
}

for (const [dependencyName, source] of [
  ['sdkwork-aiot-contract', rootCargoSource],
  ['sdkwork-aiot-http-api', rootCargoSource],
  ['sdkwork-aiot-runtime', rootCargoSource],
  ['sdkwork-aiot-transport', rootCargoSource],
]) {
  assert.match(
    source,
    new RegExp(`${dependencyName}\\s*=\\s*\\{\\s*path\\s*=\\s*"\\.\\./sdkwork-aiot/`),
    `Craw Chat Rust workspace must integrate ${dependencyName} from the sibling sdkwork-aiot root.`,
  );
  assert.match(
    localMinimalCargoSource,
    new RegExp(`${dependencyName}\\.workspace\\s*=\\s*true`),
    `local-minimal-node must consume ${dependencyName} through workspace dependencies.`,
  );
}

assert.match(
  localMinimalNodeSource,
  /mod aiot_bridge;/u,
  'local-minimal-node must mount the SDKWork AIoT Rust backend bridge.',
);
assert.match(
  localMinimalBuildSource,
  /\/app\/v3\/api\/iot/u,
  'local-minimal-node must mount the AIoT app API prefix from the sdkwork-aiot bridge.',
);
assert.match(
  localMinimalBuildSource,
  /\/backend\/v3\/api\/iot/u,
  'local-minimal-node must mount the AIoT backend API prefix from the sdkwork-aiot bridge.',
);

console.log('sdkwork chat pc AIoT devices SDK contract passed.');
