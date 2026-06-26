import type { ImLiveConnection, ImRealtimeScopeSubscription, ImSubscription } from "@sdkwork/im-sdk";
import { getImSdkClient } from "@sdkwork/im-h5-core";
import { readImH5IamSessionTokens } from "@sdkwork/im-h5-core";

const INBOX_REALTIME_EVENT_TYPES = [
  "message.posted",
  "conversation.updated",
  "conversation.created",
  "conversation.member_joined",
  "conversation.member_role_changed",
  "conversation.member_removed",
  "conversation.member_left",
  "conversation.owner_transferred",
] as const;

type ConversationMessageHandler = (message: unknown) => void;
type InboxRefreshHandler = () => void;

let sharedConnection: ImLiveConnection | null = null;
let sharedConnectionPromise: Promise<ImLiveConnection> | null = null;
let sharedConnectionIsOpen = false;
const conversationHandlers = new Map<string, Set<ConversationMessageHandler>>();
const conversationUnsubs = new Map<string, ImSubscription>();
const inboxHandlers = new Map<string, Set<InboxRefreshHandler>>();
const inboxUnsubs = new Map<string, ImSubscription>();

function realtimeScopeKey(scopeType: string, scopeId: string): string {
  return `${scopeType}:${scopeId}`;
}

async function ensureLiveConnection(): Promise<ImLiveConnection> {
  if (sharedConnection) {
    return sharedConnection;
  }
  if (!sharedConnectionPromise) {
    sharedConnectionPromise = getImSdkClient()
      .connect({
        subscriptions: {
          conversations: [],
          scopes: [],
        },
      })
      .then((connection) => {
        sharedConnection = connection;
        connection.lifecycle.onStateChange((state) => {
          sharedConnectionIsOpen = state.status === "open";
          if (sharedConnectionIsOpen) {
            syncLiveSubscriptions(connection);
          }
          if (state.status === "closed" || state.status === "error") {
            sharedConnection = null;
            sharedConnectionPromise = null;
            sharedConnectionIsOpen = false;
          }
        });
        return connection;
      })
      .catch((error: unknown) => {
        sharedConnectionPromise = null;
        throw error;
      });
  }
  return sharedConnectionPromise;
}

function buildScopeSubscriptions(): ImRealtimeScopeSubscription[] {
  const scopes: ImRealtimeScopeSubscription[] = [];
  for (const scopeKey of inboxHandlers.keys()) {
    const [scopeType, scopeId] = scopeKey.split(":");
    if (!scopeType || !scopeId) {
      continue;
    }
    scopes.push({
      scopeType,
      scopeId,
      eventTypes: [...INBOX_REALTIME_EVENT_TYPES],
    });
  }
  return scopes;
}

function syncLiveSubscriptions(connection: ImLiveConnection): void {
  if (!sharedConnectionIsOpen) {
    return;
  }
  connection.subscriptions.syncConversations(Array.from(conversationHandlers.keys()));
  connection.subscriptions.syncScopes(buildScopeSubscriptions());
}

function teardownConnectionIfIdle(): void {
  if (conversationHandlers.size > 0 || inboxHandlers.size > 0) {
    return;
  }
  sharedConnection?.disconnect(1000, "no live subscriptions");
  sharedConnection = null;
  sharedConnectionPromise = null;
  sharedConnectionIsOpen = false;
}

export function disposeChatLiveConnection(): void {
  for (const unsubscribe of conversationUnsubs.values()) {
    unsubscribe();
  }
  for (const unsubscribe of inboxUnsubs.values()) {
    unsubscribe();
  }
  conversationUnsubs.clear();
  inboxUnsubs.clear();
  conversationHandlers.clear();
  inboxHandlers.clear();
  sharedConnection?.disconnect(1000, "session ended");
  sharedConnection = null;
  sharedConnectionPromise = null;
  sharedConnectionIsOpen = false;
}

export function resolveInboxRealtimeUserId(): string | null {
  const session = readImH5IamSessionTokens();
  const userId =
    session?.context?.userId?.trim()
    ?? session?.user?.userId?.trim()
    ?? session?.user?.id?.trim()
    ?? null;
  return userId || null;
}

export async function subscribeConversationLiveMessages(
  conversationId: string,
  handler: ConversationMessageHandler,
): Promise<() => void> {
  const connection = await ensureLiveConnection();
  let handlers = conversationHandlers.get(conversationId);
  if (!handlers) {
    handlers = new Set();
    conversationHandlers.set(conversationId, handlers);
    const unsubscribe = connection.messages.onConversation(conversationId, (message) => {
      for (const activeHandler of handlers ?? []) {
        activeHandler(message);
      }
    });
    conversationUnsubs.set(conversationId, unsubscribe);
    syncLiveSubscriptions(connection);
  }

  handlers.add(handler);
  return () => {
    const activeHandlers = conversationHandlers.get(conversationId);
    if (!activeHandlers) {
      return;
    }
    activeHandlers.delete(handler);
    if (activeHandlers.size > 0) {
      return;
    }

    conversationUnsubs.get(conversationId)?.();
    conversationUnsubs.delete(conversationId);
    conversationHandlers.delete(conversationId);
    if (sharedConnection) {
      syncLiveSubscriptions(sharedConnection);
    }
    teardownConnectionIfIdle();
  };
}

export async function subscribeInboxLiveRefresh(
  handler: InboxRefreshHandler,
  userId = resolveInboxRealtimeUserId(),
): Promise<() => void> {
  if (!userId) {
    return () => undefined;
  }

  const scopeKey = realtimeScopeKey("user", userId);
  const connection = await ensureLiveConnection();
  let handlers = inboxHandlers.get(scopeKey);
  if (!handlers) {
    handlers = new Set();
    inboxHandlers.set(scopeKey, handlers);
    const unsubscribe = connection.events.onScope("user", userId, () => {
      for (const activeHandler of handlers ?? []) {
        activeHandler();
      }
    });
    inboxUnsubs.set(scopeKey, unsubscribe);
    syncLiveSubscriptions(connection);
  }

  handlers.add(handler);
  return () => {
    const activeHandlers = inboxHandlers.get(scopeKey);
    if (!activeHandlers) {
      return;
    }
    activeHandlers.delete(handler);
    if (activeHandlers.size > 0) {
      return;
    }

    inboxUnsubs.get(scopeKey)?.();
    inboxUnsubs.delete(scopeKey);
    inboxHandlers.delete(scopeKey);
    if (sharedConnection) {
      syncLiveSubscriptions(sharedConnection);
    }
    teardownConnectionIfIdle();
  };
}
