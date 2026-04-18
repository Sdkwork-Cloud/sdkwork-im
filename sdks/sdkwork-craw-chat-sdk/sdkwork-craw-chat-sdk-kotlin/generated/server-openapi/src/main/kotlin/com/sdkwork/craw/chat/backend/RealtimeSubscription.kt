package com.sdkwork.craw.chat.backend

data class RealtimeSubscription(
    val scopeType: String? = null,
    val scopeId: String? = null,
    val eventTypes: List<String>? = null,
    val subscribedAt: String? = null
)
