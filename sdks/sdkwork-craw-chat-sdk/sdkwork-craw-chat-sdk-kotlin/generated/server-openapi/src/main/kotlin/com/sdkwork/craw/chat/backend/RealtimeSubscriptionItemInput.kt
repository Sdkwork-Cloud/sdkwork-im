package com.sdkwork.craw.chat.backend

data class RealtimeSubscriptionItemInput(
    val scopeType: String? = null,
    val scopeId: String? = null,
    val eventTypes: List<String>? = null
)
