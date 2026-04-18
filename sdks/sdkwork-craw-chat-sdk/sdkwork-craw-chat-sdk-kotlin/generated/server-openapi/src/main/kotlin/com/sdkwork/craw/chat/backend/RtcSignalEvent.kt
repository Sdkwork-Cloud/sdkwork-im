package com.sdkwork.craw.chat.backend

data class RtcSignalEvent(
    val tenantId: String? = null,
    val rtcSessionId: String? = null,
    val conversationId: String? = null,
    val rtcMode: String? = null,
    val signalType: String? = null,
    val schemaRef: String? = null,
    val payload: String? = null,
    val sender: Sender? = null,
    val signalingStreamId: String? = null,
    val occurredAt: String? = null
)
