package com.sdkwork.craw.chat.backend

data class ConversationReadCursorView(
    val tenantId: String? = null,
    val conversationId: String? = null,
    val memberId: String? = null,
    val principalId: String? = null,
    val readSeq: Int? = null,
    val lastReadMessageId: String? = null,
    val updatedAt: String? = null,
    val unreadCount: Int? = null
)
