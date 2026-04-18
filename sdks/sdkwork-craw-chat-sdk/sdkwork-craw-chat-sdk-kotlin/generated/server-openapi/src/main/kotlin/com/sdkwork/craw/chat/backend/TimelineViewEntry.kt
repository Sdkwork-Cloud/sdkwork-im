package com.sdkwork.craw.chat.backend

data class TimelineViewEntry(
    val tenantId: String? = null,
    val conversationId: String? = null,
    val messageId: String? = null,
    val messageSeq: Int? = null,
    val summary: String? = null
)
