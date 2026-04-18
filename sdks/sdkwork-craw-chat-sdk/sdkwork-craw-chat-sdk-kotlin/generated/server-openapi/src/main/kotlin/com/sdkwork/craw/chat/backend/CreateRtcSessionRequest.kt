package com.sdkwork.craw.chat.backend

data class CreateRtcSessionRequest(
    val rtcSessionId: String? = null,
    val conversationId: String? = null,
    val rtcMode: String? = null
)
