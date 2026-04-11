import {
  arrayField,
  field,
  objectField,
  type ApiSchemaDefinitionMap,
  type ApiSchemaField,
} from "./schema-types";

const devicePresenceFields: ApiSchemaField[] = [
  field("tenantId", "string", "Tenant identifier.", { required: true }),
  field("principalId", "string", "Principal identifier.", { required: true }),
  field("deviceId", "string", "Device identifier.", { required: true }),
  field("platform", "string | null", "Client platform label."),
  field("sessionId", "string | null", "Active session identifier."),
  field("status", "string", "Device presence state. Supported values: online, offline.", {
    required: true,
  }),
  field("lastSyncSeq", "uint64", "Latest device sync sequence acknowledged by the service.", {
    required: true,
  }),
  field("lastResumeAt", "date-time string | null", "Most recent resume timestamp."),
  field("lastSeenAt", "date-time string | null", "Most recent heartbeat timestamp."),
];

const presenceSnapshotFields: ApiSchemaField[] = [
  field("tenantId", "string", "Tenant identifier.", { required: true }),
  field("principalId", "string", "Principal identifier.", { required: true }),
  field("currentDeviceId", "string | null", "Current active device identifier."),
  arrayField("devices", "DevicePresenceView", "Presence snapshots for all known devices.", devicePresenceFields, {
    required: true,
    summary: "View nested fields for devices",
  }),
];

const realtimeSubscriptionItemInputFields: ApiSchemaField[] = [
  field("scopeType", "string", "Subscription scope type, for example conversation.", {
    required: true,
  }),
  field("scopeId", "string", "Subscription scope identifier.", { required: true }),
  field("eventTypes", "string[]", "Event types to include. An empty array subscribes to all events in scope."),
];

const realtimeSubscriptionFields: ApiSchemaField[] = [
  field("scopeType", "string", "Resolved subscription scope type.", { required: true }),
  field("scopeId", "string", "Resolved subscription scope identifier.", { required: true }),
  field("eventTypes", "string[]", "Subscribed event type list.", { required: true }),
  field("subscribedAt", "date-time string", "Subscription sync timestamp.", { required: true }),
];

const realtimeEventFields: ApiSchemaField[] = [
  field("tenantId", "string", "Tenant identifier.", { required: true }),
  field("principalId", "string", "Target principal identifier.", { required: true }),
  field("deviceId", "string", "Target device identifier.", { required: true }),
  field("realtimeSeq", "uint64", "Realtime event sequence number.", { required: true }),
  field("scopeType", "string", "Event scope type.", { required: true }),
  field("scopeId", "string", "Event scope identifier.", { required: true }),
  field("eventType", "string", "Event type.", { required: true }),
  field("deliveryClass", "string", "Delivery class. Current implementations commonly use ephemeral.", {
    required: true,
  }),
  field("payload", "string", "Serialized event payload JSON.", { required: true }),
  field("occurredAt", "date-time string", "Event timestamp.", { required: true }),
];

const registeredDeviceViewFields: ApiSchemaField[] = [
  field("tenantId", "string", "Tenant identifier.", { required: true }),
  field("principalId", "string", "Owning principal identifier.", { required: true }),
  field("deviceId", "string", "Device identifier.", { required: true }),
  field("registeredAt", "date-time string", "Registration or latest bind timestamp.", {
    required: true,
  }),
];

const deviceSyncFeedEntryFields: ApiSchemaField[] = [
  field("tenantId", "string", "Tenant identifier.", { required: true }),
  field("principalId", "string", "Principal identifier.", { required: true }),
  field("deviceId", "string", "Device identifier.", { required: true }),
  field("syncSeq", "uint64", "Device sync sequence number.", { required: true }),
  field("originEventId", "string", "Source event identifier.", { required: true }),
  field("originEventType", "string", "Source event type.", { required: true }),
  field("conversationId", "string | null", "Related conversation identifier."),
  field("messageId", "string | null", "Related message identifier."),
  field("messageSeq", "uint64 | null", "Related message sequence number."),
  field("memberId", "string | null", "Related conversation member identifier."),
  field("readSeq", "uint64 | null", "Read cursor sequence number."),
  field("lastReadMessageId", "string | null", "Related last-read message identifier."),
  field("actorId", "string | null", "Actor identifier that produced the sync item."),
  field("actorKind", "string | null", "Actor kind that produced the sync item."),
  field("actorDeviceId", "string | null", "Actor device identifier."),
  field("summary", "string | null", "Human-readable sync item summary."),
  field("payloadSchema", "string | null", "Payload schema identifier."),
  field("payload", "string | null", "Serialized sync payload JSON."),
  field("occurredAt", "date-time string", "Event timestamp.", { required: true }),
];

export const appSessionSchemas: ApiSchemaDefinitionMap = {
  DevicePresenceView: {
    fields: devicePresenceFields,
  },
  PresenceSnapshotView: {
    fields: presenceSnapshotFields,
  },
  ResumeSessionRequest: {
    fields: [
      field("deviceId", "string | null", "Device identifier to resume."),
      field("lastSeenSyncSeq", "uint64 | null", "Last sync sequence already consumed by the client."),
    ],
  },
  SessionResumeView: {
    fields: [
      field("tenantId", "string", "Tenant identifier.", { required: true }),
      field("actorId", "string", "Current actor identifier.", { required: true }),
      field("actorKind", "string", "Current actor kind.", { required: true }),
      field("sessionId", "string | null", "Active session identifier."),
      field("deviceId", "string", "Active device identifier.", { required: true }),
      field("resumeRequired", "boolean", "Whether the client must execute a replay or recovery flow.", {
        required: true,
      }),
      field("resumeFromSyncSeq", "uint64", "Recommended sync replay starting point.", {
        required: true,
      }),
      field("latestSyncSeq", "uint64", "Latest sync sequence known by the server.", {
        required: true,
      }),
      field("resumedAt", "date-time string", "Resume timestamp.", { required: true }),
      objectField("presence", "Presence snapshot after resuming.", presenceSnapshotFields, {
        required: true,
        summary: "View nested fields for presence",
      }),
    ],
  },
  PresenceDeviceRequest: {
    fields: [field("deviceId", "string | null", "Target device identifier.")],
  },
  RegisterDeviceRequest: {
    fields: [field("deviceId", "string | null", "Device identifier to register or reactivate.")],
  },
  RegisteredDeviceView: {
    fields: registeredDeviceViewFields,
  },
  RealtimeSubscriptionItemInput: {
    fields: realtimeSubscriptionItemInputFields,
  },
  SyncRealtimeSubscriptionsRequest: {
    fields: [
      field("deviceId", "string | null", "Target device identifier."),
      arrayField(
        "items",
        "RealtimeSubscriptionItemInput",
        "Requested subscription set. Missing or empty means no active subscriptions.",
        realtimeSubscriptionItemInputFields,
        {
          summary: "View nested fields for items",
        },
      ),
    ],
  },
  RealtimeSubscription: {
    fields: realtimeSubscriptionFields,
  },
  RealtimeSubscriptionSnapshot: {
    fields: [
      field("tenantId", "string", "Tenant identifier.", { required: true }),
      field("principalId", "string", "Principal identifier.", { required: true }),
      field("deviceId", "string", "Device identifier.", { required: true }),
      arrayField("items", "RealtimeSubscription", "Resolved realtime subscriptions for the device.", realtimeSubscriptionFields, {
        required: true,
        summary: "View nested fields for items",
      }),
      field("syncedAt", "date-time string", "Subscription sync timestamp.", { required: true }),
    ],
  },
  RealtimeEvent: {
    fields: realtimeEventFields,
  },
  RealtimeEventWindow: {
    fields: [
      field("deviceId", "string", "Device identifier.", { required: true }),
      arrayField("items", "RealtimeEvent", "Realtime events in the current fetch window.", realtimeEventFields, {
        required: true,
        summary: "View nested fields for items",
      }),
      field("nextAfterSeq", "uint64 | null", "Cursor to use for the next fetch request."),
      field("hasMore", "boolean", "Whether more events are available.", { required: true }),
      field("ackedThroughSeq", "uint64", "Highest acknowledged event sequence.", { required: true }),
      field("trimmedThroughSeq", "uint64", "Highest event sequence already trimmed from retention.", {
        required: true,
      }),
    ],
  },
  AckRealtimeEventsRequest: {
    fields: [
      field("deviceId", "string | null", "Target device identifier."),
      field("ackedSeq", "uint64", "Highest event sequence confirmed by the client.", {
        required: true,
      }),
    ],
  },
  RealtimeAckState: {
    fields: [
      field("tenantId", "string", "Tenant identifier.", { required: true }),
      field("principalId", "string", "Principal identifier.", { required: true }),
      field("deviceId", "string", "Device identifier.", { required: true }),
      field("ackedThroughSeq", "uint64", "Highest acknowledged event sequence.", { required: true }),
      field("trimmedThroughSeq", "uint64", "Highest trimmed event sequence.", { required: true }),
      field("retainedEventCount", "uint64", "Number of retained events still available in the device window.", {
        required: true,
      }),
      field("ackedAt", "date-time string", "ACK timestamp.", { required: true }),
    ],
  },
  DeviceSyncFeedEntry: {
    fields: deviceSyncFeedEntryFields,
  },
  DeviceSyncFeedResponse: {
    fields: [
      arrayField("items", "DeviceSyncFeedEntry", "Device sync feed entries.", deviceSyncFeedEntryFields, {
        required: true,
        summary: "View nested fields for items",
      }),
    ],
  },
};
