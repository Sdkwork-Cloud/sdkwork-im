package com.sdkwork.craw.chat.backend

data class MessageMutationResult(
    val conversationId: String? = null,
    val messageId: String? = null,
    val messageSeq: Int? = null,
    val eventId: String? = null
)
