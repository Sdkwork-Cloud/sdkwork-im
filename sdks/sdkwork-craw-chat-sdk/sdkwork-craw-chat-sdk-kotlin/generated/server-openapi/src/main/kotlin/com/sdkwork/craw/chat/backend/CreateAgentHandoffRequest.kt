package com.sdkwork.craw.chat.backend

data class CreateAgentHandoffRequest(
    val conversationId: String? = null,
    val targetId: String? = null,
    val targetKind: String? = null,
    val handoffSessionId: String? = null,
    val handoffReason: String? = null
)
