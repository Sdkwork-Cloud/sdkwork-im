package com.sdkwork.craw.chat.backend

data class AttachMediaRequest(
    val conversationId: String? = null,
    val clientMsgId: String? = null,
    val summary: String? = null,
    val text: String? = null,
    val renderHints: Map<String, String>? = null
)
