import Foundation

public struct ApiError: Codable {
    public let code: String?
    public let message: String?


    public init(code: String? = nil, message: String? = nil) {
        self.code = code
        self.message = message
    }
}

public struct PortalLoginRequest: Codable {
    public let tenantId: String?
    public let login: String?
    public let password: String?
    public let deviceId: String?
    public let sessionId: String?
    public let clientKind: String?


    public init(tenantId: String? = nil, login: String? = nil, password: String? = nil, deviceId: String? = nil, sessionId: String? = nil, clientKind: String? = nil) {
        self.tenantId = tenantId
        self.login = login
        self.password = password
        self.deviceId = deviceId
        self.sessionId = sessionId
        self.clientKind = clientKind
    }
}

public struct PortalUserView: Codable {
    public let id: String?
    public let login: String?
    public let name: String?
    public let role: String?
    public let email: String?
    public let actorKind: String?
    public let clientKind: String?
    public let permissions: [String]?


    public init(id: String? = nil, login: String? = nil, name: String? = nil, role: String? = nil, email: String? = nil, actorKind: String? = nil, clientKind: String? = nil, permissions: [String]? = nil) {
        self.id = id
        self.login = login
        self.name = name
        self.role = role
        self.email = email
        self.actorKind = actorKind
        self.clientKind = clientKind
        self.permissions = permissions
    }
}

public struct PortalWorkspaceView: Codable {
    public let name: String?
    public let slug: String?
    public let tier: String?
    public let region: String?
    public let supportPlan: String?
    public let seats: Int?
    public let activeBrands: Int?
    public let uptime: String?


    public init(name: String? = nil, slug: String? = nil, tier: String? = nil, region: String? = nil, supportPlan: String? = nil, seats: Int? = nil, activeBrands: Int? = nil, uptime: String? = nil) {
        self.name = name
        self.slug = slug
        self.tier = tier
        self.region = region
        self.supportPlan = supportPlan
        self.seats = seats
        self.activeBrands = activeBrands
        self.uptime = uptime
    }
}

public struct PortalLoginResponse: Codable {
    public let accessToken: String?
    public let refreshToken: String?
    public let expiresAt: Int?
    public let user: PortalUserView?
    public let workspace: PortalWorkspaceView?


    public init(accessToken: String? = nil, refreshToken: String? = nil, expiresAt: Int? = nil, user: PortalUserView? = nil, workspace: PortalWorkspaceView? = nil) {
        self.accessToken = accessToken
        self.refreshToken = refreshToken
        self.expiresAt = expiresAt
        self.user = user
        self.workspace = workspace
    }
}

public struct PortalMeResponse: Codable {
    public let tenantId: String?
    public let user: PortalUserView?
    public let workspace: PortalWorkspaceView?


    public init(tenantId: String? = nil, user: PortalUserView? = nil, workspace: PortalWorkspaceView? = nil) {
        self.tenantId = tenantId
        self.user = user
        self.workspace = workspace
    }
}

public struct ResumeSessionRequest: Codable {
    public let deviceId: String?
    public let lastSeenSyncSeq: Int?


    public init(deviceId: String? = nil, lastSeenSyncSeq: Int? = nil) {
        self.deviceId = deviceId
        self.lastSeenSyncSeq = lastSeenSyncSeq
    }
}

public struct PresenceDeviceRequest: Codable {
    public let deviceId: String?


    public init(deviceId: String? = nil) {
        self.deviceId = deviceId
    }
}

public struct RegisterDeviceRequest: Codable {
    public let deviceId: String?


    public init(deviceId: String? = nil) {
        self.deviceId = deviceId
    }
}

public struct RealtimeSubscriptionItemInput: Codable {
    public let scopeType: String?
    public let scopeId: String?
    public let eventTypes: [String]?


    public init(scopeType: String? = nil, scopeId: String? = nil, eventTypes: [String]? = nil) {
        self.scopeType = scopeType
        self.scopeId = scopeId
        self.eventTypes = eventTypes
    }
}

public struct SyncRealtimeSubscriptionsRequest: Codable {
    public let deviceId: String?
    public let items: [RealtimeSubscriptionItemInput]?


    public init(deviceId: String? = nil, items: [RealtimeSubscriptionItemInput]? = nil) {
        self.deviceId = deviceId
        self.items = items
    }
}

public struct AckRealtimeEventsRequest: Codable {
    public let deviceId: String?
    public let ackedSeq: Int?


    public init(deviceId: String? = nil, ackedSeq: Int? = nil) {
        self.deviceId = deviceId
        self.ackedSeq = ackedSeq
    }
}

public struct CreateConversationRequest: Codable {
    public let conversationId: String?
    public let conversationType: String?


    public init(conversationId: String? = nil, conversationType: String? = nil) {
        self.conversationId = conversationId
        self.conversationType = conversationType
    }
}

public struct CreateAgentDialogRequest: Codable {
    public let conversationId: String?
    public let agentId: String?


    public init(conversationId: String? = nil, agentId: String? = nil) {
        self.conversationId = conversationId
        self.agentId = agentId
    }
}

public struct CreateAgentHandoffRequest: Codable {
    public let conversationId: String?
    public let targetId: String?
    public let targetKind: String?
    public let handoffSessionId: String?
    public let handoffReason: String?


    public init(conversationId: String? = nil, targetId: String? = nil, targetKind: String? = nil, handoffSessionId: String? = nil, handoffReason: String? = nil) {
        self.conversationId = conversationId
        self.targetId = targetId
        self.targetKind = targetKind
        self.handoffSessionId = handoffSessionId
        self.handoffReason = handoffReason
    }
}

public struct CreateSystemChannelRequest: Codable {
    public let conversationId: String?
    public let subscriberId: String?


    public init(conversationId: String? = nil, subscriberId: String? = nil) {
        self.conversationId = conversationId
        self.subscriberId = subscriberId
    }
}

public struct AddConversationMemberRequest: Codable {
    public let principalId: String?
    public let principalKind: String?
    public let role: String?


    public init(principalId: String? = nil, principalKind: String? = nil, role: String? = nil) {
        self.principalId = principalId
        self.principalKind = principalKind
        self.role = role
    }
}

public struct RemoveConversationMemberRequest: Codable {
    public let memberId: String?


    public init(memberId: String? = nil) {
        self.memberId = memberId
    }
}

public struct TransferConversationOwnerRequest: Codable {
    public let memberId: String?


    public init(memberId: String? = nil) {
        self.memberId = memberId
    }
}

public struct ChangeConversationMemberRoleRequest: Codable {
    public let memberId: String?
    public let role: String?


    public init(memberId: String? = nil, role: String? = nil) {
        self.memberId = memberId
        self.role = role
    }
}

public struct UpdateReadCursorRequest: Codable {
    public let readSeq: Int?
    public let lastReadMessageId: String?


    public init(readSeq: Int? = nil, lastReadMessageId: String? = nil) {
        self.readSeq = readSeq
        self.lastReadMessageId = lastReadMessageId
    }
}

public struct PostMessageRequest: Codable {
    public let clientMsgId: String?
    public let summary: String?
    public let text: String?
    public let parts: [ContentPart]?
    public let renderHints: [String: String]?


    public init(clientMsgId: String? = nil, summary: String? = nil, text: String? = nil, parts: [ContentPart]? = nil, renderHints: [String: String]? = nil) {
        self.clientMsgId = clientMsgId
        self.summary = summary
        self.text = text
        self.parts = parts
        self.renderHints = renderHints
    }
}

public struct EditMessageRequest: Codable {
    public let summary: String?
    public let text: String?
    public let parts: [ContentPart]?
    public let renderHints: [String: String]?


    public init(summary: String? = nil, text: String? = nil, parts: [ContentPart]? = nil, renderHints: [String: String]? = nil) {
        self.summary = summary
        self.text = text
        self.parts = parts
        self.renderHints = renderHints
    }
}

public struct AttachMediaRequest: Codable {
    public let conversationId: String?
    public let clientMsgId: String?
    public let summary: String?
    public let text: String?
    public let renderHints: [String: String]?


    public init(conversationId: String? = nil, clientMsgId: String? = nil, summary: String? = nil, text: String? = nil, renderHints: [String: String]? = nil) {
        self.conversationId = conversationId
        self.clientMsgId = clientMsgId
        self.summary = summary
        self.text = text
        self.renderHints = renderHints
    }
}

public struct DevicePresenceView: Codable {
    public let tenantId: String?
    public let principalId: String?
    public let deviceId: String?
    public let platform: String?
    public let sessionId: String?
    public let status: String?
    public let lastSyncSeq: Int?
    public let lastResumeAt: String?
    public let lastSeenAt: String?


    public init(tenantId: String? = nil, principalId: String? = nil, deviceId: String? = nil, platform: String? = nil, sessionId: String? = nil, status: String? = nil, lastSyncSeq: Int? = nil, lastResumeAt: String? = nil, lastSeenAt: String? = nil) {
        self.tenantId = tenantId
        self.principalId = principalId
        self.deviceId = deviceId
        self.platform = platform
        self.sessionId = sessionId
        self.status = status
        self.lastSyncSeq = lastSyncSeq
        self.lastResumeAt = lastResumeAt
        self.lastSeenAt = lastSeenAt
    }
}

public struct PresenceSnapshotView: Codable {
    public let tenantId: String?
    public let principalId: String?
    public let currentDeviceId: String?
    public let devices: [DevicePresenceView]?


    public init(tenantId: String? = nil, principalId: String? = nil, currentDeviceId: String? = nil, devices: [DevicePresenceView]? = nil) {
        self.tenantId = tenantId
        self.principalId = principalId
        self.currentDeviceId = currentDeviceId
        self.devices = devices
    }
}

public struct SessionResumeView: Codable {
    public let tenantId: String?
    public let actorId: String?
    public let actorKind: String?
    public let sessionId: String?
    public let deviceId: String?
    public let resumeRequired: Bool?
    public let resumeFromSyncSeq: Int?
    public let latestSyncSeq: Int?
    public let resumedAt: String?
    public let presence: PresenceSnapshotView?


    public init(tenantId: String? = nil, actorId: String? = nil, actorKind: String? = nil, sessionId: String? = nil, deviceId: String? = nil, resumeRequired: Bool? = nil, resumeFromSyncSeq: Int? = nil, latestSyncSeq: Int? = nil, resumedAt: String? = nil, presence: PresenceSnapshotView? = nil) {
        self.tenantId = tenantId
        self.actorId = actorId
        self.actorKind = actorKind
        self.sessionId = sessionId
        self.deviceId = deviceId
        self.resumeRequired = resumeRequired
        self.resumeFromSyncSeq = resumeFromSyncSeq
        self.latestSyncSeq = latestSyncSeq
        self.resumedAt = resumedAt
        self.presence = presence
    }
}

public struct RealtimeSubscription: Codable {
    public let scopeType: String?
    public let scopeId: String?
    public let eventTypes: [String]?
    public let subscribedAt: String?


    public init(scopeType: String? = nil, scopeId: String? = nil, eventTypes: [String]? = nil, subscribedAt: String? = nil) {
        self.scopeType = scopeType
        self.scopeId = scopeId
        self.eventTypes = eventTypes
        self.subscribedAt = subscribedAt
    }
}

public struct RealtimeSubscriptionSnapshot: Codable {
    public let tenantId: String?
    public let principalId: String?
    public let deviceId: String?
    public let items: [RealtimeSubscription]?
    public let syncedAt: String?


    public init(tenantId: String? = nil, principalId: String? = nil, deviceId: String? = nil, items: [RealtimeSubscription]? = nil, syncedAt: String? = nil) {
        self.tenantId = tenantId
        self.principalId = principalId
        self.deviceId = deviceId
        self.items = items
        self.syncedAt = syncedAt
    }
}

public struct RealtimeEvent: Codable {
    public let tenantId: String?
    public let principalId: String?
    public let deviceId: String?
    public let realtimeSeq: Int?
    public let scopeType: String?
    public let scopeId: String?
    public let eventType: String?
    public let deliveryClass: String?
    public let payload: String?
    public let occurredAt: String?


    public init(tenantId: String? = nil, principalId: String? = nil, deviceId: String? = nil, realtimeSeq: Int? = nil, scopeType: String? = nil, scopeId: String? = nil, eventType: String? = nil, deliveryClass: String? = nil, payload: String? = nil, occurredAt: String? = nil) {
        self.tenantId = tenantId
        self.principalId = principalId
        self.deviceId = deviceId
        self.realtimeSeq = realtimeSeq
        self.scopeType = scopeType
        self.scopeId = scopeId
        self.eventType = eventType
        self.deliveryClass = deliveryClass
        self.payload = payload
        self.occurredAt = occurredAt
    }
}

public struct RealtimeEventWindow: Codable {
    public let deviceId: String?
    public let items: [RealtimeEvent]?
    public let nextAfterSeq: Int?
    public let hasMore: Bool?
    public let ackedThroughSeq: Int?
    public let trimmedThroughSeq: Int?


    public init(deviceId: String? = nil, items: [RealtimeEvent]? = nil, nextAfterSeq: Int? = nil, hasMore: Bool? = nil, ackedThroughSeq: Int? = nil, trimmedThroughSeq: Int? = nil) {
        self.deviceId = deviceId
        self.items = items
        self.nextAfterSeq = nextAfterSeq
        self.hasMore = hasMore
        self.ackedThroughSeq = ackedThroughSeq
        self.trimmedThroughSeq = trimmedThroughSeq
    }
}

public struct RealtimeAckState: Codable {
    public let tenantId: String?
    public let principalId: String?
    public let deviceId: String?
    public let ackedThroughSeq: Int?
    public let trimmedThroughSeq: Int?
    public let retainedEventCount: Int?
    public let ackedAt: String?


    public init(tenantId: String? = nil, principalId: String? = nil, deviceId: String? = nil, ackedThroughSeq: Int? = nil, trimmedThroughSeq: Int? = nil, retainedEventCount: Int? = nil, ackedAt: String? = nil) {
        self.tenantId = tenantId
        self.principalId = principalId
        self.deviceId = deviceId
        self.ackedThroughSeq = ackedThroughSeq
        self.trimmedThroughSeq = trimmedThroughSeq
        self.retainedEventCount = retainedEventCount
        self.ackedAt = ackedAt
    }
}

public struct ConversationMember: Codable {
    public let tenantId: String?
    public let conversationId: String?
    public let memberId: String?
    public let principalId: String?
    public let principalKind: String?
    public let role: String?
    public let state: String?
    public let invitedBy: String?
    public let joinedAt: String?
    public let removedAt: String?
    public let attributes: [String: String]?


    public init(tenantId: String? = nil, conversationId: String? = nil, memberId: String? = nil, principalId: String? = nil, principalKind: String? = nil, role: String? = nil, state: String? = nil, invitedBy: String? = nil, joinedAt: String? = nil, removedAt: String? = nil, attributes: [String: String]? = nil) {
        self.tenantId = tenantId
        self.conversationId = conversationId
        self.memberId = memberId
        self.principalId = principalId
        self.principalKind = principalKind
        self.role = role
        self.state = state
        self.invitedBy = invitedBy
        self.joinedAt = joinedAt
        self.removedAt = removedAt
        self.attributes = attributes
    }
}

public struct ConversationReadCursorView: Codable {
    public let tenantId: String?
    public let conversationId: String?
    public let memberId: String?
    public let principalId: String?
    public let readSeq: Int?
    public let lastReadMessageId: String?
    public let updatedAt: String?
    public let unreadCount: Int?


    public init(tenantId: String? = nil, conversationId: String? = nil, memberId: String? = nil, principalId: String? = nil, readSeq: Int? = nil, lastReadMessageId: String? = nil, updatedAt: String? = nil, unreadCount: Int? = nil) {
        self.tenantId = tenantId
        self.conversationId = conversationId
        self.memberId = memberId
        self.principalId = principalId
        self.readSeq = readSeq
        self.lastReadMessageId = lastReadMessageId
        self.updatedAt = updatedAt
        self.unreadCount = unreadCount
    }
}

public struct ChangeAgentHandoffStatusView: Codable {
    public let id: String?
    public let kind: String?


    public init(id: String? = nil, kind: String? = nil) {
        self.id = id
        self.kind = kind
    }
}

public struct AgentHandoffStateView: Codable {
    public let tenantId: String?
    public let conversationId: String?
    public let status: String?
    public let source: ChangeAgentHandoffStatusView?
    public let target: ChangeAgentHandoffStatusView?
    public let handoffSessionId: String?
    public let handoffReason: String?
    public let acceptedAt: String?
    public let acceptedBy: ChangeAgentHandoffStatusView?
    public let resolvedAt: String?
    public let resolvedBy: ChangeAgentHandoffStatusView?
    public let closedAt: String?
    public let closedBy: ChangeAgentHandoffStatusView?


    public init(tenantId: String? = nil, conversationId: String? = nil, status: String? = nil, source: ChangeAgentHandoffStatusView? = nil, target: ChangeAgentHandoffStatusView? = nil, handoffSessionId: String? = nil, handoffReason: String? = nil, acceptedAt: String? = nil, acceptedBy: ChangeAgentHandoffStatusView? = nil, resolvedAt: String? = nil, resolvedBy: ChangeAgentHandoffStatusView? = nil, closedAt: String? = nil, closedBy: ChangeAgentHandoffStatusView? = nil) {
        self.tenantId = tenantId
        self.conversationId = conversationId
        self.status = status
        self.source = source
        self.target = target
        self.handoffSessionId = handoffSessionId
        self.handoffReason = handoffReason
        self.acceptedAt = acceptedAt
        self.acceptedBy = acceptedBy
        self.resolvedAt = resolvedAt
        self.resolvedBy = resolvedBy
        self.closedAt = closedAt
        self.closedBy = closedBy
    }
}

public struct ConversationActorView: Codable {
    public let id: String?
    public let kind: String?


    public init(id: String? = nil, kind: String? = nil) {
        self.id = id
        self.kind = kind
    }
}

public struct ConversationAgentHandoffView: Codable {
    public let status: String?
    public let source: ConversationActorView?
    public let target: ConversationActorView?
    public let handoffSessionId: String?
    public let handoffReason: String?
    public let acceptedAt: String?
    public let acceptedBy: ConversationActorView?
    public let resolvedAt: String?
    public let resolvedBy: ConversationActorView?
    public let closedAt: String?
    public let closedBy: ConversationActorView?


    public init(status: String? = nil, source: ConversationActorView? = nil, target: ConversationActorView? = nil, handoffSessionId: String? = nil, handoffReason: String? = nil, acceptedAt: String? = nil, acceptedBy: ConversationActorView? = nil, resolvedAt: String? = nil, resolvedBy: ConversationActorView? = nil, closedAt: String? = nil, closedBy: ConversationActorView? = nil) {
        self.status = status
        self.source = source
        self.target = target
        self.handoffSessionId = handoffSessionId
        self.handoffReason = handoffReason
        self.acceptedAt = acceptedAt
        self.acceptedBy = acceptedBy
        self.resolvedAt = resolvedAt
        self.resolvedBy = resolvedBy
        self.closedAt = closedAt
        self.closedBy = closedBy
    }
}

public struct ConversationInboxEntry: Codable {
    public let tenantId: String?
    public let principalId: String?
    public let memberId: String?
    public let conversationId: String?
    public let conversationType: String?
    public let messageCount: Int?
    public let lastMessageId: String?
    public let lastMessageSeq: Int?
    public let lastSenderId: String?
    public let lastSenderKind: String?
    public let lastSummary: String?
    public let unreadCount: Int?
    public let lastActivityAt: String?
    public let agentHandoff: ConversationAgentHandoffView?


    public init(tenantId: String? = nil, principalId: String? = nil, memberId: String? = nil, conversationId: String? = nil, conversationType: String? = nil, messageCount: Int? = nil, lastMessageId: String? = nil, lastMessageSeq: Int? = nil, lastSenderId: String? = nil, lastSenderKind: String? = nil, lastSummary: String? = nil, unreadCount: Int? = nil, lastActivityAt: String? = nil, agentHandoff: ConversationAgentHandoffView? = nil) {
        self.tenantId = tenantId
        self.principalId = principalId
        self.memberId = memberId
        self.conversationId = conversationId
        self.conversationType = conversationType
        self.messageCount = messageCount
        self.lastMessageId = lastMessageId
        self.lastMessageSeq = lastMessageSeq
        self.lastSenderId = lastSenderId
        self.lastSenderKind = lastSenderKind
        self.lastSummary = lastSummary
        self.unreadCount = unreadCount
        self.lastActivityAt = lastActivityAt
        self.agentHandoff = agentHandoff
    }
}

public struct DeviceSyncFeedEntry: Codable {
    public let tenantId: String?
    public let principalId: String?
    public let deviceId: String?
    public let syncSeq: Int?
    public let originEventId: String?
    public let originEventType: String?
    public let conversationId: String?
    public let messageId: String?
    public let messageSeq: Int?
    public let memberId: String?
    public let readSeq: Int?
    public let lastReadMessageId: String?
    public let actorId: String?
    public let actorKind: String?
    public let actorDeviceId: String?
    public let summary: String?
    public let payloadSchema: String?
    public let payload: String?
    public let occurredAt: String?


    public init(tenantId: String? = nil, principalId: String? = nil, deviceId: String? = nil, syncSeq: Int? = nil, originEventId: String? = nil, originEventType: String? = nil, conversationId: String? = nil, messageId: String? = nil, messageSeq: Int? = nil, memberId: String? = nil, readSeq: Int? = nil, lastReadMessageId: String? = nil, actorId: String? = nil, actorKind: String? = nil, actorDeviceId: String? = nil, summary: String? = nil, payloadSchema: String? = nil, payload: String? = nil, occurredAt: String? = nil) {
        self.tenantId = tenantId
        self.principalId = principalId
        self.deviceId = deviceId
        self.syncSeq = syncSeq
        self.originEventId = originEventId
        self.originEventType = originEventType
        self.conversationId = conversationId
        self.messageId = messageId
        self.messageSeq = messageSeq
        self.memberId = memberId
        self.readSeq = readSeq
        self.lastReadMessageId = lastReadMessageId
        self.actorId = actorId
        self.actorKind = actorKind
        self.actorDeviceId = actorDeviceId
        self.summary = summary
        self.payloadSchema = payloadSchema
        self.payload = payload
        self.occurredAt = occurredAt
    }
}

public struct ListMembersResponse: Codable {
    public let items: [ConversationMember]?


    public init(items: [ConversationMember]? = nil) {
        self.items = items
    }
}

public struct InboxResponse: Codable {
    public let items: [ConversationInboxEntry]?


    public init(items: [ConversationInboxEntry]? = nil) {
        self.items = items
    }
}

public struct DeviceSyncFeedResponse: Codable {
    public let items: [DeviceSyncFeedEntry]?


    public init(items: [DeviceSyncFeedEntry]? = nil) {
        self.items = items
    }
}

public struct TimelineViewEntry: Codable {
    public let tenantId: String?
    public let conversationId: String?
    public let messageId: String?
    public let messageSeq: Int?
    public let summary: String?


    public init(tenantId: String? = nil, conversationId: String? = nil, messageId: String? = nil, messageSeq: Int? = nil, summary: String? = nil) {
        self.tenantId = tenantId
        self.conversationId = conversationId
        self.messageId = messageId
        self.messageSeq = messageSeq
        self.summary = summary
    }
}

public struct TimelineResponse: Codable {
    public let items: [TimelineViewEntry]?


    public init(items: [TimelineViewEntry]? = nil) {
        self.items = items
    }
}

public struct SummarySenderView: Codable {
    public let id: String?
    public let kind: String?


    public init(id: String? = nil, kind: String? = nil) {
        self.id = id
        self.kind = kind
    }
}

public struct ConversationSummaryView: Codable {
    public let tenantId: String?
    public let conversationId: String?
    public let messageCount: Int?
    public let lastMessageId: String?
    public let lastMessageSeq: Int?
    public let lastSenderId: String?
    public let lastSenderKind: String?
    public let lastSender: SummarySenderView?
    public let lastSummary: String?
    public let lastMessageAt: String?
    public let agentHandoff: ConversationAgentHandoffView?


    public init(tenantId: String? = nil, conversationId: String? = nil, messageCount: Int? = nil, lastMessageId: String? = nil, lastMessageSeq: Int? = nil, lastSenderId: String? = nil, lastSenderKind: String? = nil, lastSender: SummarySenderView? = nil, lastSummary: String? = nil, lastMessageAt: String? = nil, agentHandoff: ConversationAgentHandoffView? = nil) {
        self.tenantId = tenantId
        self.conversationId = conversationId
        self.messageCount = messageCount
        self.lastMessageId = lastMessageId
        self.lastMessageSeq = lastMessageSeq
        self.lastSenderId = lastSenderId
        self.lastSenderKind = lastSenderKind
        self.lastSender = lastSender
        self.lastSummary = lastSummary
        self.lastMessageAt = lastMessageAt
        self.agentHandoff = agentHandoff
    }
}

public struct RegisteredDeviceView: Codable {
    public let tenantId: String?
    public let principalId: String?
    public let deviceId: String?
    public let registeredAt: String?


    public init(tenantId: String? = nil, principalId: String? = nil, deviceId: String? = nil, registeredAt: String? = nil) {
        self.tenantId = tenantId
        self.principalId = principalId
        self.deviceId = deviceId
        self.registeredAt = registeredAt
    }
}

public struct CreateConversationResult: Codable {
    public let conversationId: String?
    public let eventId: String?


    public init(conversationId: String? = nil, eventId: String? = nil) {
        self.conversationId = conversationId
        self.eventId = eventId
    }
}

public struct TransferConversationOwnerResult: Codable {
    public let eventId: String?
    public let transferredAt: String?
    public let previousOwner: ConversationMember?
    public let newOwner: ConversationMember?


    public init(eventId: String? = nil, transferredAt: String? = nil, previousOwner: ConversationMember? = nil, newOwner: ConversationMember? = nil) {
        self.eventId = eventId
        self.transferredAt = transferredAt
        self.previousOwner = previousOwner
        self.newOwner = newOwner
    }
}

public struct ChangeConversationMemberRoleResult: Codable {
    public let eventId: String?
    public let changedAt: String?
    public let previousMember: ConversationMember?
    public let updatedMember: ConversationMember?


    public init(eventId: String? = nil, changedAt: String? = nil, previousMember: ConversationMember? = nil, updatedMember: ConversationMember? = nil) {
        self.eventId = eventId
        self.changedAt = changedAt
        self.previousMember = previousMember
        self.updatedMember = updatedMember
    }
}

public struct PostMessageResult: Codable {
    public let messageId: String?
    public let messageSeq: Int?
    public let eventId: String?


    public init(messageId: String? = nil, messageSeq: Int? = nil, eventId: String? = nil) {
        self.messageId = messageId
        self.messageSeq = messageSeq
        self.eventId = eventId
    }
}

public struct MessageMutationResult: Codable {
    public let conversationId: String?
    public let messageId: String?
    public let messageSeq: Int?
    public let eventId: String?


    public init(conversationId: String? = nil, messageId: String? = nil, messageSeq: Int? = nil, eventId: String? = nil) {
        self.conversationId = conversationId
        self.messageId = messageId
        self.messageSeq = messageSeq
        self.eventId = eventId
    }
}

public struct Sender: Codable {
    public let id: String?
    public let kind: String?
    public let memberId: String?
    public let deviceId: String?
    public let sessionId: String?
    public let metadata: [String: String]?


    public init(id: String? = nil, kind: String? = nil, memberId: String? = nil, deviceId: String? = nil, sessionId: String? = nil, metadata: [String: String]? = nil) {
        self.id = id
        self.kind = kind
        self.memberId = memberId
        self.deviceId = deviceId
        self.sessionId = sessionId
        self.metadata = metadata
    }
}

public struct MessageBody: Codable {
    public let summary: String?
    public let parts: [ContentPart]?
    public let renderHints: [String: String]?


    public init(summary: String? = nil, parts: [ContentPart]? = nil, renderHints: [String: String]? = nil) {
        self.summary = summary
        self.parts = parts
        self.renderHints = renderHints
    }
}

public struct ContentPart: Codable {
    public let kind: String?
    public let text: String?
    public let schemaRef: String?
    public let encoding: String?
    public let payload: String?
    public let mediaAssetId: String?
    public let resource: MediaResource?
    public let signalType: String?
    public let streamId: String?
    public let streamType: String?
    public let state: String?


    public init(kind: String? = nil, text: String? = nil, schemaRef: String? = nil, encoding: String? = nil, payload: String? = nil, mediaAssetId: String? = nil, resource: MediaResource? = nil, signalType: String? = nil, streamId: String? = nil, streamType: String? = nil, state: String? = nil) {
        self.kind = kind
        self.text = text
        self.schemaRef = schemaRef
        self.encoding = encoding
        self.payload = payload
        self.mediaAssetId = mediaAssetId
        self.resource = resource
        self.signalType = signalType
        self.streamId = streamId
        self.streamType = streamType
        self.state = state
    }
}

public struct CreateUploadRequest: Codable {
    public let mediaAssetId: String?
    public let resource: MediaResource?


    public init(mediaAssetId: String? = nil, resource: MediaResource? = nil) {
        self.mediaAssetId = mediaAssetId
        self.resource = resource
    }
}

public struct CompleteUploadRequest: Codable {
    public let bucket: String?
    public let objectKey: String?
    public let storageProvider: String?
    public let url: String?
    public let checksum: String?


    public init(bucket: String? = nil, objectKey: String? = nil, storageProvider: String? = nil, url: String? = nil, checksum: String? = nil) {
        self.bucket = bucket
        self.objectKey = objectKey
        self.storageProvider = storageProvider
        self.url = url
        self.checksum = checksum
    }
}

public struct MediaDownloadUrlResponse: Codable {
    public let mediaAssetId: String?
    public let storageProvider: String?
    public let downloadUrl: String?
    public let expiresInSeconds: Int?


    public init(mediaAssetId: String? = nil, storageProvider: String? = nil, downloadUrl: String? = nil, expiresInSeconds: Int? = nil) {
        self.mediaAssetId = mediaAssetId
        self.storageProvider = storageProvider
        self.downloadUrl = downloadUrl
        self.expiresInSeconds = expiresInSeconds
    }
}

public struct MediaResource: Codable {
    public let id: Int?
    public let uuid: String?
    public let url: String?
    public let bytes: [Int]?
    public let localFile: String?
    public let base64: String?
    public let type: String?
    public let mimeType: String?
    public let size: Int?
    public let name: String?
    public let extension_: String?
    public let tags: [String: String]?
    public let metadata: [String: String]?
    public let prompt: String?


    public init(id: Int? = nil, uuid: String? = nil, url: String? = nil, bytes: [Int]? = nil, localFile: String? = nil, base64: String? = nil, type: String? = nil, mimeType: String? = nil, size: Int? = nil, name: String? = nil, extension_: String? = nil, tags: [String: String]? = nil, metadata: [String: String]? = nil, prompt: String? = nil) {
        self.id = id
        self.uuid = uuid
        self.url = url
        self.bytes = bytes
        self.localFile = localFile
        self.base64 = base64
        self.type = type
        self.mimeType = mimeType
        self.size = size
        self.name = name
        self.extension_ = extension_
        self.tags = tags
        self.metadata = metadata
        self.prompt = prompt
    }
}

public struct MediaAsset: Codable {
    public let tenantId: String?
    public let principalId: String?
    public let principalKind: String?
    public let mediaAssetId: String?
    public let bucket: String?
    public let objectKey: String?
    public let storageProvider: String?
    public let checksum: String?
    public let processingState: String?
    public let resource: MediaResource?
    public let createdAt: String?
    public let completedAt: String?


    public init(tenantId: String? = nil, principalId: String? = nil, principalKind: String? = nil, mediaAssetId: String? = nil, bucket: String? = nil, objectKey: String? = nil, storageProvider: String? = nil, checksum: String? = nil, processingState: String? = nil, resource: MediaResource? = nil, createdAt: String? = nil, completedAt: String? = nil) {
        self.tenantId = tenantId
        self.principalId = principalId
        self.principalKind = principalKind
        self.mediaAssetId = mediaAssetId
        self.bucket = bucket
        self.objectKey = objectKey
        self.storageProvider = storageProvider
        self.checksum = checksum
        self.processingState = processingState
        self.resource = resource
        self.createdAt = createdAt
        self.completedAt = completedAt
    }
}

public struct OpenStreamRequest: Codable {
    public let streamId: String?
    public let streamType: String?
    public let scopeKind: String?
    public let scopeId: String?
    public let durabilityClass: String?
    public let schemaRef: String?


    public init(streamId: String? = nil, streamType: String? = nil, scopeKind: String? = nil, scopeId: String? = nil, durabilityClass: String? = nil, schemaRef: String? = nil) {
        self.streamId = streamId
        self.streamType = streamType
        self.scopeKind = scopeKind
        self.scopeId = scopeId
        self.durabilityClass = durabilityClass
        self.schemaRef = schemaRef
    }
}

public struct CheckpointStreamRequest: Codable {
    public let frameSeq: Int?


    public init(frameSeq: Int? = nil) {
        self.frameSeq = frameSeq
    }
}

public struct CompleteStreamRequest: Codable {
    public let frameSeq: Int?
    public let resultMessageId: String?


    public init(frameSeq: Int? = nil, resultMessageId: String? = nil) {
        self.frameSeq = frameSeq
        self.resultMessageId = resultMessageId
    }
}

public struct AbortStreamRequest: Codable {
    public let frameSeq: Int?
    public let reason: String?


    public init(frameSeq: Int? = nil, reason: String? = nil) {
        self.frameSeq = frameSeq
        self.reason = reason
    }
}

public struct AppendStreamFrameRequest: Codable {
    public let frameSeq: Int?
    public let frameType: String?
    public let schemaRef: String?
    public let encoding: String?
    public let payload: String?
    public let attributes: [String: String]?


    public init(frameSeq: Int? = nil, frameType: String? = nil, schemaRef: String? = nil, encoding: String? = nil, payload: String? = nil, attributes: [String: String]? = nil) {
        self.frameSeq = frameSeq
        self.frameType = frameType
        self.schemaRef = schemaRef
        self.encoding = encoding
        self.payload = payload
        self.attributes = attributes
    }
}

public struct StreamSession: Codable {
    public let tenantId: String?
    public let streamId: String?
    public let streamType: String?
    public let scopeKind: String?
    public let scopeId: String?
    public let durabilityClass: String?
    public let orderingScope: String?
    public let schemaRef: String?
    public let state: String?
    public let lastFrameSeq: Int?
    public let lastCheckpointSeq: Int?
    public let resultMessageId: String?
    public let openedAt: String?
    public let closedAt: String?
    public let expiresAt: String?


    public init(tenantId: String? = nil, streamId: String? = nil, streamType: String? = nil, scopeKind: String? = nil, scopeId: String? = nil, durabilityClass: String? = nil, orderingScope: String? = nil, schemaRef: String? = nil, state: String? = nil, lastFrameSeq: Int? = nil, lastCheckpointSeq: Int? = nil, resultMessageId: String? = nil, openedAt: String? = nil, closedAt: String? = nil, expiresAt: String? = nil) {
        self.tenantId = tenantId
        self.streamId = streamId
        self.streamType = streamType
        self.scopeKind = scopeKind
        self.scopeId = scopeId
        self.durabilityClass = durabilityClass
        self.orderingScope = orderingScope
        self.schemaRef = schemaRef
        self.state = state
        self.lastFrameSeq = lastFrameSeq
        self.lastCheckpointSeq = lastCheckpointSeq
        self.resultMessageId = resultMessageId
        self.openedAt = openedAt
        self.closedAt = closedAt
        self.expiresAt = expiresAt
    }
}

public struct StreamFrame: Codable {
    public let tenantId: String?
    public let streamId: String?
    public let streamType: String?
    public let scopeKind: String?
    public let scopeId: String?
    public let frameSeq: Int?
    public let frameType: String?
    public let schemaRef: String?
    public let encoding: String?
    public let payload: String?
    public let sender: Sender?
    public let attributes: [String: String]?
    public let occurredAt: String?


    public init(tenantId: String? = nil, streamId: String? = nil, streamType: String? = nil, scopeKind: String? = nil, scopeId: String? = nil, frameSeq: Int? = nil, frameType: String? = nil, schemaRef: String? = nil, encoding: String? = nil, payload: String? = nil, sender: Sender? = nil, attributes: [String: String]? = nil, occurredAt: String? = nil) {
        self.tenantId = tenantId
        self.streamId = streamId
        self.streamType = streamType
        self.scopeKind = scopeKind
        self.scopeId = scopeId
        self.frameSeq = frameSeq
        self.frameType = frameType
        self.schemaRef = schemaRef
        self.encoding = encoding
        self.payload = payload
        self.sender = sender
        self.attributes = attributes
        self.occurredAt = occurredAt
    }
}

public struct StreamFrameWindow: Codable {
    public let items: [StreamFrame]?
    public let nextAfterFrameSeq: Int?
    public let hasMore: Bool?


    public init(items: [StreamFrame]? = nil, nextAfterFrameSeq: Int? = nil, hasMore: Bool? = nil) {
        self.items = items
        self.nextAfterFrameSeq = nextAfterFrameSeq
        self.hasMore = hasMore
    }
}

public struct CreateRtcSessionRequest: Codable {
    public let rtcSessionId: String?
    public let conversationId: String?
    public let rtcMode: String?


    public init(rtcSessionId: String? = nil, conversationId: String? = nil, rtcMode: String? = nil) {
        self.rtcSessionId = rtcSessionId
        self.conversationId = conversationId
        self.rtcMode = rtcMode
    }
}

public struct InviteRtcSessionRequest: Codable {
    public let signalingStreamId: String?


    public init(signalingStreamId: String? = nil) {
        self.signalingStreamId = signalingStreamId
    }
}

public struct UpdateRtcSessionRequest: Codable {
    public let artifactMessageId: String?


    public init(artifactMessageId: String? = nil) {
        self.artifactMessageId = artifactMessageId
    }
}

public struct PostRtcSignalRequest: Codable {
    public let signalType: String?
    public let schemaRef: String?
    public let payload: String?
    public let signalingStreamId: String?


    public init(signalType: String? = nil, schemaRef: String? = nil, payload: String? = nil, signalingStreamId: String? = nil) {
        self.signalType = signalType
        self.schemaRef = schemaRef
        self.payload = payload
        self.signalingStreamId = signalingStreamId
    }
}

public struct IssueRtcParticipantCredentialRequest: Codable {
    public let participantId: String?


    public init(participantId: String? = nil) {
        self.participantId = participantId
    }
}

public struct RtcSession: Codable {
    public let tenantId: String?
    public let rtcSessionId: String?
    public let conversationId: String?
    public let rtcMode: String?
    public let initiatorId: String?
    public let providerPluginId: String?
    public let providerSessionId: String?
    public let accessEndpoint: String?
    public let providerRegion: String?
    public let state: String?
    public let signalingStreamId: String?
    public let artifactMessageId: String?
    public let startedAt: String?
    public let endedAt: String?


    public init(tenantId: String? = nil, rtcSessionId: String? = nil, conversationId: String? = nil, rtcMode: String? = nil, initiatorId: String? = nil, providerPluginId: String? = nil, providerSessionId: String? = nil, accessEndpoint: String? = nil, providerRegion: String? = nil, state: String? = nil, signalingStreamId: String? = nil, artifactMessageId: String? = nil, startedAt: String? = nil, endedAt: String? = nil) {
        self.tenantId = tenantId
        self.rtcSessionId = rtcSessionId
        self.conversationId = conversationId
        self.rtcMode = rtcMode
        self.initiatorId = initiatorId
        self.providerPluginId = providerPluginId
        self.providerSessionId = providerSessionId
        self.accessEndpoint = accessEndpoint
        self.providerRegion = providerRegion
        self.state = state
        self.signalingStreamId = signalingStreamId
        self.artifactMessageId = artifactMessageId
        self.startedAt = startedAt
        self.endedAt = endedAt
    }
}

public struct RtcSignalEvent: Codable {
    public let tenantId: String?
    public let rtcSessionId: String?
    public let conversationId: String?
    public let rtcMode: String?
    public let signalType: String?
    public let schemaRef: String?
    public let payload: String?
    public let sender: Sender?
    public let signalingStreamId: String?
    public let occurredAt: String?


    public init(tenantId: String? = nil, rtcSessionId: String? = nil, conversationId: String? = nil, rtcMode: String? = nil, signalType: String? = nil, schemaRef: String? = nil, payload: String? = nil, sender: Sender? = nil, signalingStreamId: String? = nil, occurredAt: String? = nil) {
        self.tenantId = tenantId
        self.rtcSessionId = rtcSessionId
        self.conversationId = conversationId
        self.rtcMode = rtcMode
        self.signalType = signalType
        self.schemaRef = schemaRef
        self.payload = payload
        self.sender = sender
        self.signalingStreamId = signalingStreamId
        self.occurredAt = occurredAt
    }
}

public struct RtcParticipantCredential: Codable {
    public let tenantId: String?
    public let rtcSessionId: String?
    public let participantId: String?
    public let credential: String?
    public let expiresAt: String?


    public init(tenantId: String? = nil, rtcSessionId: String? = nil, participantId: String? = nil, credential: String? = nil, expiresAt: String? = nil) {
        self.tenantId = tenantId
        self.rtcSessionId = rtcSessionId
        self.participantId = participantId
        self.credential = credential
        self.expiresAt = expiresAt
    }
}

public struct RtcRecordingArtifact: Codable {
    public let tenantId: String?
    public let rtcSessionId: String?
    public let bucket: String?
    public let objectKey: String?
    public let storageProvider: String?
    public let playbackUrl: String?


    public init(tenantId: String? = nil, rtcSessionId: String? = nil, bucket: String? = nil, objectKey: String? = nil, storageProvider: String? = nil, playbackUrl: String? = nil) {
        self.tenantId = tenantId
        self.rtcSessionId = rtcSessionId
        self.bucket = bucket
        self.objectKey = objectKey
        self.storageProvider = storageProvider
        self.playbackUrl = playbackUrl
    }
}
