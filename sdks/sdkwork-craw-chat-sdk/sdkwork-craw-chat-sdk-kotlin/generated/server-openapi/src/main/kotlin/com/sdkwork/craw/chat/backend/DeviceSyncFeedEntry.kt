package com.sdkwork.craw.chat.backend

data class DeviceSyncFeedEntry(
    val tenantId: String? = null,
    val principalId: String? = null,
    val deviceId: String? = null,
    val syncSeq: Int? = null,
    val originEventId: String? = null,
    val originEventType: String? = null,
    val conversationId: String? = null,
    val messageId: String? = null,
    val messageSeq: Int? = null,
    val memberId: String? = null,
    val readSeq: Int? = null,
    val lastReadMessageId: String? = null,
    val actorId: String? = null,
    val actorKind: String? = null,
    val actorDeviceId: String? = null,
    val summary: String? = null,
    val payloadSchema: String? = null,
    val payload: String? = null,
    val occurredAt: String? = null
)
