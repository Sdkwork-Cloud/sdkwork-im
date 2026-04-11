import { readPortalDashboardSnapshot } from '../repository/index.js';
import {
  assertPortalBulletItems,
  assertPortalClusterItems,
  assertPortalHeroSnapshot,
  assertPortalProgressItems,
  assertPortalSnapshotRecord,
  assertPortalTableItems,
} from '../../../craw-chat-portal-commons/src/index.js';

export async function buildPortalDashboardViewModel() {
  const snapshot = await readPortalDashboardSnapshot();
  assertPortalSnapshotRecord('Dashboard snapshot', snapshot);
  assertPortalHeroSnapshot('Dashboard snapshot', snapshot.hero);
  assertPortalTableItems('Dashboard snapshot hero.kpis', snapshot.hero.kpis, ['label', 'value']);
  assertPortalProgressItems('Dashboard snapshot pressure', snapshot.pressure);
  assertPortalClusterItems('Dashboard snapshot posture', snapshot.posture);
  assertPortalBulletItems('Dashboard snapshot priorities', snapshot.priorities);
  assertPortalBulletItems('Dashboard snapshot timeline', snapshot.timeline);
  return snapshot;
}
