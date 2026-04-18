package com.sdkwork.craw.chat.backend

data class RealtimeEvent(
    val tenantId: String? = null,
    val principalId: String? = null,
    val deviceId: String? = null,
    val realtimeSeq: Int? = null,
    val scopeType: String? = null,
    val scopeId: String? = null,
    val eventType: String? = null,
    val deliveryClass: String? = null,
    val payload: String? = null,
    val occurredAt: String? = null
)
