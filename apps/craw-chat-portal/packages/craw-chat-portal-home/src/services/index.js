import {
  assertPortalBulletItems,
  assertPortalHeroSnapshot,
  assertPortalSnapshotRecord,
  assertPortalText,
} from '../../../craw-chat-portal-commons/src/index.js';
import { readPortalHomeSnapshot } from '../repository/index.js';

export async function buildPortalHomeViewModel() {
  const snapshot = await readPortalHomeSnapshot();
  assertPortalSnapshotRecord('Home snapshot', snapshot);
  assertPortalHeroSnapshot('Home snapshot', snapshot.hero);
  assertPortalText('Home snapshot hero.eyebrow', snapshot.hero.eyebrow);
  assertPortalBulletItems('Home snapshot pillars', snapshot.pillars);
  return snapshot;
}
