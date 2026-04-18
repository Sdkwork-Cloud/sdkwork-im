package com.sdkwork.craw.chat.backend

data class ConversationMember(
    val tenantId: String? = null,
    val conversationId: String? = null,
    val memberId: String? = null,
    val principalId: String? = null,
    val principalKind: String? = null,
    val role: String? = null,
    val state: String? = null,
    val invitedBy: String? = null,
    val joinedAt: String? = null,
    val removedAt: String? = null,
    val attributes: Map<String, String>? = null
)
