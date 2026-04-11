import { readPortalRealtimeBoard } from '../repository/index.js';
import {
  assertPortalBulletItems,
  assertPortalClusterItems,
  assertPortalHeroSnapshot,
  assertPortalProgressItems,
  assertPortalSnapshotRecord,
  assertPortalTableItems,
} from '../../../craw-chat-portal-commons/src/index.js';

export async function buildPortalRealtimeViewModel() {
  const snapshot = await readPortalRealtimeBoard();
  assertPortalSnapshotRecord('Realtime snapshot', snapshot);
  assertPortalHeroSnapshot('Realtime snapshot', snapshot.hero);
  assertPortalClusterItems('Realtime snapshot posture', snapshot.posture);
  assertPortalProgressItems('Realtime snapshot subscriptions', snapshot.subscriptions);
  assertPortalTableItems('Realtime snapshot devices', snapshot.devices, [
    'owner',
    'device',
    'sync',
    'lag',
    'state',
  ]);
  assertPortalBulletItems('Realtime snapshot events', snapshot.events);
  return snapshot;
}
