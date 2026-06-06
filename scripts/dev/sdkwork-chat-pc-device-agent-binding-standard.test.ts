import assert from 'node:assert/strict';
import { deviceService } from '../../apps/sdkwork-chat-pc/packages/sdkwork-clawchat-pc-devices/src/services/DeviceService';

async function main(): Promise<void> {
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
