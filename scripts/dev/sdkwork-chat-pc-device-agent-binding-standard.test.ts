import assert from 'node:assert/strict';
import {
  createDeviceService,
  type AiotDeviceServiceOptions,
} from '../../apps/sdkwork-chat-pc/packages/sdkwork-clawchat-pc-devices/src/services/DeviceService';

interface FakeAiotDevice {
  chipFamily?: string;
  deviceId: string;
  displayName: string;
  id: string;
  metadata: Record<string, unknown>;
  status: string;
}

async function main(): Promise<void> {
  const devices = new Map<string, FakeAiotDevice>([
    [
      'dev-001',
      {
        chipFamily: 'camera',
        deviceId: 'dev-001',
        displayName: 'Door Camera',
        id: 'dev-001',
        metadata: {
          agentId: 'agent.device.guard',
          firmwareVersion: '1.0.0',
          macAddress: '00:11:22:33:44:55',
        },
        status: 'online',
      },
    ],
    [
      'dev-002',
      {
        chipFamily: 'display',
        deviceId: 'dev-002',
        displayName: 'Lobby Display',
        id: 'dev-002',
        metadata: {
          firmwareVersion: '1.1.0',
          macAddress: '00:11:22:33:44:66',
        },
        status: 'offline',
      },
    ],
  ]);
  const appClient = {
    iot: {
      devices: {
        list: async () => ({ data: Array.from(devices.values()) }),
        retrieve: async (id: string) => ({ data: devices.get(id) }),
      },
    },
  };
  const backendClient = {
    iot: {
      devices: {
        commands: {
          cancel: async () => ({ data: undefined }),
          list: async () => ({ data: [] }),
        },
        create: async () => ({ data: devices.get('dev-002') }),
        delete: async (id: string) => {
          devices.delete(id);
          return { data: undefined };
        },
        twin: {
          update: async (id: string, request: { desired?: { agentId?: string | null } }) => {
            const device = devices.get(id);
            if (device && request.desired && Object.prototype.hasOwnProperty.call(request.desired, 'agentId')) {
              if (request.desired.agentId) {
                device.metadata.agentId = request.desired.agentId;
              } else {
                delete device.metadata.agentId;
              }
            }
            return { data: device };
          },
        },
        update: async (id: string, request: { metadata?: Record<string, unknown>; status?: string }) => {
          const device = devices.get(id);
          if (device) {
            device.metadata = {
              ...device.metadata,
              ...(request.metadata ?? {}),
            };
            device.status = request.status ?? device.status;
          }
          return { data: device };
        },
      },
    },
  };
  const deviceService = createDeviceService({
    backendClient: backendClient as AiotDeviceServiceOptions['backendClient'],
    client: appClient as AiotDeviceServiceOptions['client'],
  });

  const initialDevices = await deviceService.getDevices();
  assert.equal(
    initialDevices
      .map((device) => device.agentId)
      .filter((agentId): agentId is string => Boolean(agentId))
      .every((agentId) => /^agent\.[a-z0-9_-]+(?:\.[a-z0-9_-]+)*$/u.test(agentId)),
    true,
    'device agent bindings must use standard agent. ids in local product data',
  );

  await deviceService.bindAgent('dev-002', 'agent.device.concierge');
  const boundDevice = await deviceService.getDevice('dev-002');
  assert.equal(
    boundDevice?.agentId,
    'agent.device.concierge',
    'device agent binding must store standard agent. ids',
  );

  await assert.rejects(
    () => deviceService.bindAgent('dev-002', 'agent-legacy'),
    /Device agent binding id must use the standard agent\./,
    'device agent binding must reject legacy agent-* ids',
  );
  const afterInvalidBind = await deviceService.getDevice('dev-002');
  assert.equal(
    afterInvalidBind?.agentId,
    'agent.device.concierge',
    'invalid legacy agent ids must not overwrite the device binding',
  );

  console.log('sdkwork-chat-pc device agent binding standard contract passed');
}

void main();
