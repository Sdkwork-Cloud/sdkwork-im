import type {
  AiotDevice,
  SdkworkAiotAppClient,
} from '@sdkwork/aiot-app-sdk';
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

function unsupportedAiotAppMethod(method: string): Error {
  return new Error(
    `AIoT App SDK does not expose ${method}. Add this capability to sdkwork-aiot App OpenAPI and regenerate @sdkwork/aiot-app-sdk before wiring this device workflow.`,
  );
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

  async addDevice(_device: Omit<Device, 'id'>): Promise<Device> {
    throw unsupportedAiotAppMethod('devices.create');
  }

  async updateDevice(_id: string, _device: Partial<Device>): Promise<void> {
    throw unsupportedAiotAppMethod('devices.update');
  }

  async deleteDevice(_id: string): Promise<void> {
    throw unsupportedAiotAppMethod('devices.delete');
  }

  async bindAgent(_deviceId: string, _agentId: string): Promise<void> {
    throw unsupportedAiotAppMethod('devices.agentBindings.create');
  }

  async unbindAgent(_deviceId: string): Promise<void> {
    throw unsupportedAiotAppMethod('devices.agentBindings.delete');
  }

  async activateDevice(_deviceId: string, _activationCode: string): Promise<void> {
    throw unsupportedAiotAppMethod('devices.activate');
  }
}

export function configureDeviceService(options: AiotDeviceServiceOptions = {}): void {
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
