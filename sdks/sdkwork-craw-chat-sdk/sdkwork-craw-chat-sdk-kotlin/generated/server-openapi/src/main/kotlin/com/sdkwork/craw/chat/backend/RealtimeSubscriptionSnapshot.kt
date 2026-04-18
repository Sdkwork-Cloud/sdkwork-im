package com.sdkwork.craw.chat.backend

data class RealtimeSubscriptionSnapshot(
    val tenantId: String? = null,
    val principalId: String? = null,
    val deviceId: String? = null,
    val items: List<RealtimeSubscription>? = null,
    val syncedAt: String? = null
)
