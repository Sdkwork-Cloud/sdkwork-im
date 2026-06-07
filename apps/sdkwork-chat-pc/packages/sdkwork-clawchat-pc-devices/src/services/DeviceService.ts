import type {
  AiotDevice,
  SdkworkAiotAppClient,
} from '@sdkwork/aiot-app-sdk';
import type {
  AiotDeviceCreateRequest,
  AiotDeviceUpdateRequest,
  SdkworkAiotBackendClient,
} from '@sdkwork/aiot-backend-sdk';
import {
  getAiotAppSdkClientWithSession,
  readAppSdkSessionTokens,
  resolveAppSdkOrganizationId,
  resolveAppSdkTenantId,
  resolveAppSdkUserId,
} from '@sdkwork/clawchat-pc-core';

export type DeviceType = 'camera' | 'speaker' | 'display' | 'sensor' | 'other';
export type DeviceStatus = 'online' | 'offline' | 'error' | 'unactivated';

export interface Device {
  id: string;
  name: string;
  type: DeviceType;
  status: DeviceStatus;
  agentId?: string;
  macAddress: string;
  firmwareVersion: string;
}

export interface DeviceService {
  getDevices(): Promise<Device[]>;
  getDevice(id: string): Promise<Device | undefined>;
  addDevice(device: Omit<Device, 'id'>): Promise<Device>;
  updateDevice(id: string, device: Partial<Device>): Promise<void>;
  deleteDevice(id: string): Promise<void>;
  bindAgent(deviceId: string, agentId: string): Promise<void>;
  unbindAgent(deviceId: string): Promise<void>;
  activateDevice(deviceId: string, activationCode: string): Promise<void>;
}

export interface AiotDeviceServiceContext {
  tenantId: string;
  organizationId: string;
  userId?: string;
  dataScope?: string;
  permissionScope: string;
}

export interface AiotDeviceServiceOptions {
  backendClient?: SdkworkAiotBackendClient;
  client?: SdkworkAiotAppClient;
  context?: Partial<AiotDeviceServiceContext>;
}

interface AiotDevicesListParams {
  xSdkworkTenantId: string;
  xSdkworkOrganizationId: string;
  xSdkworkUserId?: string;
  xSdkworkDataScope?: string;
  xSdkworkPermissionScope: string;
}

const DEFAULT_AIOT_CONTEXT: AiotDeviceServiceContext = {
  tenantId: '20001',
  organizationId: '30001',
  permissionScope: 'iot.devices.read',
};
const DEFAULT_AIOT_PRODUCT_ID = 'sdkwork-chat-pc-device';

let configuredBackendClient: SdkworkAiotBackendClient | undefined;
let configuredClient: SdkworkAiotAppClient | undefined;
let configuredContext: Partial<AiotDeviceServiceContext> = {};

function readRecord(value: unknown): Record<string, unknown> {
  return value && typeof value === 'object' && !Array.isArray(value)
    ? value as Record<string, unknown>
    : {};
}

function readString(value: unknown, fallback = ''): string {
  if (typeof value === 'string') {
    return value;
  }
  if (typeof value === 'number' || typeof value === 'boolean') {
    return String(value);
  }
  return fallback;
}

function normalizeDeviceType(value: unknown, fallback: unknown): DeviceType {
  const type = readString(value, readString(fallback)).trim().toLowerCase();
  if (type === 'camera' || type === 'speaker' || type === 'display' || type === 'sensor') {
    return type;
  }
  return 'other';
}

function normalizeDeviceStatus(value: unknown): DeviceStatus {
  const status = readString(value).trim().toLowerCase();
  if (status === 'online') {
    return 'online';
  }
  if (status === 'error' || status === 'warning' || status === 'degraded' || status === 'alarm') {
    return 'error';
  }
  if (status === 'unactivated' || status === 'inactive' || status === 'pending_activation') {
    return 'unactivated';
  }
  return 'offline';
}

function resolveDeviceId(device: AiotDevice): string {
  return device.deviceId || device.id;
}

function createDeviceId(device: Omit<Device, 'id'>): string {
  const seed = readString(device.macAddress, device.name)
    .trim()
    .toLowerCase()
    .replace(/[^a-z0-9._-]+/g, '-')
    .replace(/^-+|-+$/g, '');
  if (seed) {
    return `pc-${seed}`.slice(0, 96);
  }

  const entropy =
    typeof crypto !== 'undefined' && typeof crypto.randomUUID === 'function'
      ? crypto.randomUUID()
      : `${Date.now()}-${Math.random().toString(36).slice(2, 10)}`;
  return `pc-${entropy}`.slice(0, 96);
}

function buildDeviceMetadata(device: Partial<Device>): Record<string, unknown> {
  return {
    ...(device.type ? { type: device.type } : {}),
    ...(device.agentId !== undefined ? { agentId: device.agentId } : {}),
    ...(device.macAddress !== undefined ? { macAddress: device.macAddress } : {}),
    ...(device.firmwareVersion !== undefined ? { firmwareVersion: device.firmwareVersion } : {}),
  };
}

function toCreateRequest(device: Omit<Device, 'id'>): AiotDeviceCreateRequest {
  return {
    deviceId: createDeviceId(device),
    displayName: device.name,
    productId: DEFAULT_AIOT_PRODUCT_ID,
    clientId: device.macAddress,
    chipFamily: device.type,
  };
}

function toUpdateRequest(device: Partial<Device>): AiotDeviceUpdateRequest {
  return {
    ...(device.name !== undefined ? { displayName: device.name } : {}),
    ...(device.status !== undefined ? { status: device.status } : {}),
    metadata: buildDeviceMetadata(device),
  };
}

function mapAiotDevice(device: AiotDevice): Device {
  const metadata = readRecord(device.metadata);
  const deviceId = resolveDeviceId(device);
  return {
    id: deviceId,
    name: device.displayName || deviceId,
    type: normalizeDeviceType(metadata.type, device.chipFamily),
    status: normalizeDeviceStatus(device.status),
    ...(readString(metadata.agentId) ? { agentId: readString(metadata.agentId) } : {}),
    macAddress: readString(metadata.macAddress, readString(metadata.mac)),
    firmwareVersion: readString(metadata.firmwareVersion),
  };
}

function resolveContext(
  permissionScope: string,
  overrides?: Partial<AiotDeviceServiceContext>,
): AiotDeviceServiceContext {
  const session = readAppSdkSessionTokens();
  return {
    ...DEFAULT_AIOT_CONTEXT,
    tenantId: resolveAppSdkTenantId(session) ?? DEFAULT_AIOT_CONTEXT.tenantId,
    organizationId: resolveAppSdkOrganizationId(session) ?? DEFAULT_AIOT_CONTEXT.organizationId,
    ...(resolveAppSdkUserId(session) ? { userId: resolveAppSdkUserId(session) } : {}),
    ...configuredContext,
    ...(overrides ?? {}),
    permissionScope,
  };
}

function toListParams(context: AiotDeviceServiceContext): AiotDevicesListParams {
  return {
    xSdkworkTenantId: context.tenantId,
    xSdkworkOrganizationId: context.organizationId,
    xSdkworkUserId: context.userId,
    xSdkworkDataScope: context.dataScope,
    xSdkworkPermissionScope: context.permissionScope,
  };
}

function getClient(override?: SdkworkAiotAppClient): SdkworkAiotAppClient {
  return override ?? configuredClient ?? getAiotAppSdkClientWithSession();
}

function getBackendClient(override?: SdkworkAiotBackendClient): SdkworkAiotBackendClient {
  const client = override ?? configuredBackendClient;
  if (!client) {
    throw new Error('AIoT backend SDK client is required for device mutation operations.');
  }
  return client;
}

class AiotDeviceService implements DeviceService {
  constructor(private readonly options: AiotDeviceServiceOptions = {}) {}

  async getDevices(): Promise<Device[]> {
    const context = resolveContext('iot.devices.read', this.options.context);
    const response = await getClient(this.options.client).iot.devices.list(toListParams(context));
    return Array.isArray(response.data) ? response.data.map(mapAiotDevice) : [];
  }

  async getDevice(id: string): Promise<Device | undefined> {
    const context = resolveContext('iot.devices.read', this.options.context);
    try {
      const response = await getClient(this.options.client).iot.devices.retrieve(id, toListParams(context));
      return response.data ? mapAiotDevice(response.data) : undefined;
    } catch {
      return undefined;
    }
  }

  async addDevice(device: Omit<Device, 'id'>): Promise<Device> {
    const backendClient = getBackendClient(this.options.backendClient);
    const response = await backendClient.iot.devices.create(
      toCreateRequest(device),
    );
    const created = mapAiotDevice(response.data as AiotDevice);
    await backendClient.iot.devices.update(created.id, {
      metadata: buildDeviceMetadata(device),
      status: device.status,
    });
    return {
      ...created,
      ...device,
    };
  }

  async updateDevice(id: string, device: Partial<Device>): Promise<void> {
    await getBackendClient(this.options.backendClient).iot.devices.update(
      id,
      toUpdateRequest(device),
    );
  }

  async deleteDevice(id: string): Promise<void> {
    await getBackendClient(this.options.backendClient).iot.devices.delete(id);
  }

  async bindAgent(deviceId: string, agentId: string): Promise<void> {
    await getBackendClient(this.options.backendClient).iot.devices.twin.update(deviceId, {
      desired: { agentId },
    });
  }

  async unbindAgent(deviceId: string): Promise<void> {
    await getBackendClient(this.options.backendClient).iot.devices.twin.update(deviceId, {
      desired: { agentId: null },
    });
  }

  async activateDevice(deviceId: string, activationCode: string): Promise<void> {
    const backendClient = getBackendClient(this.options.backendClient);
    const commands = await backendClient.iot.devices.commands.list(deviceId);
    for (const command of commands.data) {
      if (
        command.commandName === 'activate'
        && (command.status === 'queued' || command.status === 'pending')
      ) {
        await backendClient.iot.devices.commands.cancel(deviceId, command.commandId).catch(() => undefined);
      }
    }
    await backendClient.iot.devices.twin.update(deviceId, {
      desired: {
        activationCode,
        activationRequestedAt: new Date().toISOString(),
      },
    });
  }
}

export function configureDeviceService(options: AiotDeviceServiceOptions = {}): void {
  configuredBackendClient = options.backendClient ?? configuredBackendClient;
  configuredClient = options.client ?? configuredClient;
  configuredContext = {
    ...configuredContext,
    ...(options.context ?? {}),
  };
}

export function createDeviceService(options: AiotDeviceServiceOptions = {}): DeviceService {
  return new AiotDeviceService(options);
}

export const deviceService: DeviceService = createDeviceService();
