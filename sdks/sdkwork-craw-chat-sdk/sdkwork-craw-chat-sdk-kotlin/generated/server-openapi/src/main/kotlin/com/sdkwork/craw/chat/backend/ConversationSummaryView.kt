package com.sdkwork.craw.chat.backend

data class ConversationSummaryView(
    val tenantId: String? = null,
    val conversationId: String? = null,
    val messageCount: Int? = null,
    val lastMessageId: String? = null,
    val lastMessageSeq: Int? = null,
    val lastSenderId: String? = null,
    val lastSenderKind: String? = null,
    val lastSender: SummarySenderView? = null,
    val lastSummary: String? = null,
    val lastMessageAt: String? = null,
    val agentHandoff: ConversationAgentHandoffView? = null
)
