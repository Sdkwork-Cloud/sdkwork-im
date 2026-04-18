package com.sdkwork.craw.chat.backend

data class ConversationInboxEntry(
    val tenantId: String? = null,
    val principalId: String? = null,
    val memberId: String? = null,
    val conversationId: String? = null,
    val conversationType: String? = null,
    val messageCount: Int? = null,
    val lastMessageId: String? = null,
    val lastMessageSeq: Int? = null,
    val lastSenderId: String? = null,
    val lastSenderKind: String? = null,
    val lastSummary: String? = null,
    val unreadCount: Int? = null,
    val lastActivityAt: String? = null,
    val agentHandoff: ConversationAgentHandoffView? = null
)
