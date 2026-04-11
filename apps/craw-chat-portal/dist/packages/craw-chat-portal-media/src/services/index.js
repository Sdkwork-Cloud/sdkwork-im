import { readPortalMediaBoard } from '../repository/index.js';
import {
  assertPortalBulletItems,
  assertPortalClusterItems,
  assertPortalHeroSnapshot,
  assertPortalSnapshotRecord,
  assertPortalTableItems,
} from '../../../craw-chat-portal-commons/src/index.js';

export async function buildPortalMediaViewModel() {
  const snapshot = await readPortalMediaBoard();
  assertPortalSnapshotRecord('Media snapshot', snapshot);
  assertPortalHeroSnapshot('Media snapshot', snapshot.hero);
  assertPortalTableItems('Media snapshot assets', snapshot.assets, [
    'asset',
    'type',
    'state',
    'queue',
    'owner',
  ]);
  assertPortalTableItems('Media snapshot rtcSessions', snapshot.rtcSessions, [
    'room',
    'region',
    'participants',
    'state',
    'note',
  ]);
  assertPortalClusterItems('Media snapshot providers', snapshot.providers);
  assertPortalBulletItems('Media snapshot streams', snapshot.streams);
  return snapshot;
}
