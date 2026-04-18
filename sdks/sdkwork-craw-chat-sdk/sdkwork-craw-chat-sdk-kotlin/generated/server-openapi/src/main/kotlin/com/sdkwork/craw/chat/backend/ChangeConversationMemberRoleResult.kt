package com.sdkwork.craw.chat.backend

data class ChangeConversationMemberRoleResult(
    val eventId: String? = null,
    val changedAt: String? = null,
    val previousMember: ConversationMember? = null,
    val updatedMember: ConversationMember? = null
)
