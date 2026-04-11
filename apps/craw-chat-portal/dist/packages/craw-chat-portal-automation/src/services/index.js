import { readPortalAutomationBoard } from '../repository/index.js';
import {
  assertPortalBulletItems,
  assertPortalClusterItems,
  assertPortalHeroSnapshot,
  assertPortalSnapshotRecord,
  assertPortalTableItems,
} from '../../../craw-chat-portal-commons/src/index.js';

export async function buildPortalAutomationViewModel() {
  const snapshot = await readPortalAutomationBoard();
  assertPortalSnapshotRecord('Automation snapshot', snapshot);
  assertPortalHeroSnapshot('Automation snapshot', snapshot.hero);
  assertPortalClusterItems('Automation snapshot summary', snapshot.summary);
  assertPortalTableItems('Automation snapshot executions', snapshot.executions, [
    'flow',
    'owner',
    'state',
    'age',
    'impact',
  ]);
  assertPortalTableItems('Automation snapshot notifications', snapshot.notifications, [
    'task',
    'channel',
    'state',
    'drift',
  ]);
  assertPortalBulletItems('Automation snapshot playbooks', snapshot.playbooks);
  return snapshot;
}
