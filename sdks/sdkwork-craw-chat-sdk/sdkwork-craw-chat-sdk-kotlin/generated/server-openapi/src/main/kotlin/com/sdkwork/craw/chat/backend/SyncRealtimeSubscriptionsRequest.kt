package com.sdkwork.craw.chat.backend

data class SyncRealtimeSubscriptionsRequest(
    val deviceId: String? = null,
    val items: List<RealtimeSubscriptionItemInput>? = null
)
