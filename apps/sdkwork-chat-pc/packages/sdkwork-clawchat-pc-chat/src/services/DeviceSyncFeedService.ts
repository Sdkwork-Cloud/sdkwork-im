import type {
  DeviceSyncFeedResponse,
  ImSdkClient,
} from '@sdkwork/im-sdk';

export type DeviceSyncFeedEntry = DeviceSyncFeedResponse['items'][number];

export interface DeviceSyncFeedPageResult {
  deviceId: string;
  entries: DeviceSyncFeedEntry[];
  nextAfterSeq: number;
  trimmedThroughSeq: number;
}

export const DEVICE_SYNC_PAGE_LIMIT = 100;

const SDKWORK_CHAT_PC_DEVICE_ID_KEY = 'sdkwork-chat-pc:im-device-id:v1';
const SDKWORK_CHAT_PC_DEVICE_SYNC_SEQ_KEY_PREFIX = 'sdkwork-chat-pc:im-device-sync-after-seq:v1';
const LOCAL_DEVICE_ID_PATTERN = /^d_[a-z0-9]+_[a-z0-9]+$/u;

function getStorage(): Storage | undefined {
  if (typeof window === 'undefined') {
    return undefined;
  }
  return window.localStorage;
}

function createDeviceId(): string {
  const timestamp = Date.now().toString(36);
  const entropy = Math.random().toString(36).slice(2, 10).padEnd(8, '0');
  return `d_${timestamp}_${entropy}`;
}

function isLocalDeviceId(value: string): boolean {
  return LOCAL_DEVICE_ID_PATTERN.test(value.trim());
}

export function resolveSdkworkChatPcDeviceId(): string {
  const storage = getStorage();
  if (!storage) {
    return createDeviceId();
  }

  const existing = storage.getItem(SDKWORK_CHAT_PC_DEVICE_ID_KEY);
  if (existing && isLocalDeviceId(existing)) {
    return existing.trim();
  }

  const deviceId = createDeviceId();
  storage.setItem(SDKWORK_CHAT_PC_DEVICE_ID_KEY, deviceId);
  return deviceId;
}

export function readDeviceSyncAfterSeq(namespace: string, deviceId: string): number {
  const storage = getStorage();
  if (!storage) {
    return 0;
  }
  const value = storage.getItem(deviceSyncSeqKey(namespace, deviceId));
  if (!value) {
    return 0;
  }
  const parsed = Number(value);
  return Number.isFinite(parsed) && parsed > 0 ? parsed : 0;
}

export function writeDeviceSyncAfterSeq(
  namespace: string,
  deviceId: string,
  afterSeq: number,
): void {
  const storage = getStorage();
  if (!storage || !Number.isFinite(afterSeq) || afterSeq < 0) {
    return;
  }
  storage.setItem(deviceSyncSeqKey(namespace, deviceId), String(afterSeq));
}

export async function retrieveDeviceSyncFeedWindow(
  client: ImSdkClient,
  namespace: string,
  deviceId: string,
  memoryAfterSeq: Map<string, number>,
): Promise<DeviceSyncFeedPageResult> {
  const entries: DeviceSyncFeedEntry[] = [];
  let afterSeq = memoryAfterSeq.get(deviceId) ?? readDeviceSyncAfterSeq(namespace, deviceId);
  let nextAfterSeq = afterSeq;
  let trimmedThroughSeq = 0;

  await client.device.registrations.create({ deviceId });

  while (true) {
    const response = await client.device.syncFeed.retrieve(deviceId, {
      afterSeq,
      limit: DEVICE_SYNC_PAGE_LIMIT,
    });
    entries.push(...response.items);
    trimmedThroughSeq = Math.max(trimmedThroughSeq, response.trimmedThroughSeq);

    const responseNextAfterSeq = typeof response.nextAfterSeq === 'number'
      ? response.nextAfterSeq
      : response.items.at(-1)?.syncSeq;
    if (typeof responseNextAfterSeq === 'number' && responseNextAfterSeq > nextAfterSeq) {
      nextAfterSeq = responseNextAfterSeq;
    }

    if (
      !response.hasMore
      || typeof responseNextAfterSeq !== 'number'
      || responseNextAfterSeq <= afterSeq
    ) {
      break;
    }

    afterSeq = responseNextAfterSeq;
  }

  memoryAfterSeq.set(deviceId, nextAfterSeq);
  writeDeviceSyncAfterSeq(namespace, deviceId, nextAfterSeq);

  return {
    deviceId,
    entries,
    nextAfterSeq,
    trimmedThroughSeq,
  };
}

export function parseDeviceSyncPayload(entry: DeviceSyncFeedEntry): Record<string, unknown> {
  if (!entry.payload) {
    return {};
  }
  try {
    const parsed: unknown = JSON.parse(entry.payload);
    return parsed && typeof parsed === 'object' && !Array.isArray(parsed)
      ? parsed as Record<string, unknown>
      : {};
  } catch {
    return {};
  }
}

export function pickDeviceSyncString(...values: unknown[]): string | undefined {
  for (const value of values) {
    if (typeof value === 'string' && value.trim().length > 0) {
      return value.trim();
    }
  }
  return undefined;
}

export function toDeviceSyncRecord(value: unknown): Record<string, unknown> {
  return value && typeof value === 'object' && !Array.isArray(value)
    ? value as Record<string, unknown>
    : {};
}

function deviceSyncSeqKey(namespace: string, deviceId: string): string {
  return `${SDKWORK_CHAT_PC_DEVICE_SYNC_SEQ_KEY_PREFIX}:${namespace}:${deviceId}`;
}
