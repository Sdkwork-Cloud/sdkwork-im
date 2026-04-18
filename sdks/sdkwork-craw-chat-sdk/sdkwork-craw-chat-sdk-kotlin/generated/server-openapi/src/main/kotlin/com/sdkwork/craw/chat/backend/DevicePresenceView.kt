package com.sdkwork.craw.chat.backend

data class DevicePresenceView(
    val tenantId: String? = null,
    val principalId: String? = null,
    val deviceId: String? = null,
    val platform: String? = null,
    val sessionId: String? = null,
    val status: String? = null,
    val lastSyncSeq: Int? = null,
    val lastResumeAt: String? = null,
    val lastSeenAt: String? = null
)
