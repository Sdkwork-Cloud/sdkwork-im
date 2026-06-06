import assert from 'node:assert/strict';
import { resolveSdkworkChatPcDeviceId } from '../../apps/sdkwork-chat-pc/packages/sdkwork-clawchat-pc-chat/src/services/DeviceSyncFeedService';

const DEVICE_ID_STORAGE_KEY = 'sdkwork-chat-pc:im-device-id:v1';

class MemoryStorage implements Storage {
  private readonly data = new Map<string, string>();

  get length(): number {
    return this.data.size;
  }

  clear(): void {
    this.data.clear();
  }

  getItem(key: string): string | null {
    return this.data.get(key) ?? null;
  }

  key(index: number): string | null {
    return Array.from(this.data.keys())[index] ?? null;
  }

  removeItem(key: string): void {
    this.data.delete(key);
  }

  setItem(key: string, value: string): void {
    this.data.set(key, String(value));
  }
}

const localStorage = new MemoryStorage();
Object.defineProperty(globalThis, 'window', {
  configurable: true,
  value: { localStorage },
});

function assertDeviceIdIsPathSafe(deviceId: string): void {
  assert.match(
    deviceId,
    /^d_[a-z0-9]+_[a-z0-9]+$/u,
    'local IM device ids must be compact path-safe ids with no app or project name embedded',
  );
  assert.doesNotMatch(
    deviceId,
    /sdkwork-chat-pc/u,
    'local IM device ids must not leak the app/project directory name into /devices/{deviceId} URLs',
  );
}

async function main(): Promise<void> {
  localStorage.clear();

  const generatedDeviceId = resolveSdkworkChatPcDeviceId();
  assertDeviceIdIsPathSafe(generatedDeviceId);
  assert.equal(localStorage.getItem(DEVICE_ID_STORAGE_KEY), generatedDeviceId);

  localStorage.setItem(DEVICE_ID_STORAGE_KEY, 'sdkwork-chat-pc-1780523380759-zhyejitm');
  const migratedDeviceId = resolveSdkworkChatPcDeviceId();
  assertDeviceIdIsPathSafe(migratedDeviceId);
  assert.notEqual(migratedDeviceId, 'sdkwork-chat-pc-1780523380759-zhyejitm');
  assert.equal(localStorage.getItem(DEVICE_ID_STORAGE_KEY), migratedDeviceId);

  console.log('sdkwork-chat-pc device id URL path contract passed');
}

void main();
