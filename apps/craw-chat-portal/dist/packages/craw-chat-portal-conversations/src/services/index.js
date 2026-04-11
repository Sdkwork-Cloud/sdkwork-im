import { readPortalConversationsBoard } from '../repository/index.js';
import {
  assertPortalBulletItems,
  assertPortalHeroSnapshot,
  assertPortalProgressItems,
  assertPortalSnapshotRecord,
  assertPortalTableItems,
} from '../../../craw-chat-portal-commons/src/index.js';

export async function buildPortalConversationsViewModel() {
  const snapshot = await readPortalConversationsBoard();
  assertPortalSnapshotRecord('Conversations snapshot', snapshot);
  assertPortalHeroSnapshot('Conversations snapshot', snapshot.hero);
  assertPortalProgressItems('Conversations snapshot pipeline', snapshot.pipeline);
  assertPortalTableItems('Conversations snapshot handoffs', snapshot.handoffs, [
    'conversation',
    'owner',
    'next',
    'wait',
    'priority',
  ]);
  assertPortalTableItems('Conversations snapshot watchlist', snapshot.watchlist, [
    'topic',
    'customer',
    'unread',
    'sentiment',
    'sla',
  ]);
  assertPortalBulletItems('Conversations snapshot systemChannels', snapshot.systemChannels);
  return snapshot;
}
