import { readPortalGovernanceBoard } from '../repository/index.js';
import {
  assertPortalBulletItems,
  assertPortalClusterItems,
  assertPortalHeroSnapshot,
  assertPortalSnapshotRecord,
  assertPortalTableItems,
} from '../../../craw-chat-portal-commons/src/index.js';

export async function buildPortalGovernanceViewModel() {
  const snapshot = await readPortalGovernanceBoard();
  assertPortalSnapshotRecord('Governance snapshot', snapshot);
  assertPortalHeroSnapshot('Governance snapshot', snapshot.hero);
  assertPortalTableItems('Governance snapshot auditRecords', snapshot.auditRecords, [
    'action',
    'actor',
    'scope',
    'status',
  ]);
  assertPortalClusterItems('Governance snapshot providerHealth', snapshot.providerHealth);
  assertPortalBulletItems('Governance snapshot diagnostics', snapshot.diagnostics);
  assertPortalBulletItems('Governance snapshot checklist', snapshot.checklist);
  return snapshot;
}
