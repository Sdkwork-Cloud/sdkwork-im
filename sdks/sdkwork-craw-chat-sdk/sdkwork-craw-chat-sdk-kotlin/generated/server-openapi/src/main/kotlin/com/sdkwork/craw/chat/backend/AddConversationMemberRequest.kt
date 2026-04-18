package com.sdkwork.craw.chat.backend

data class AddConversationMemberRequest(
    val principalId: String? = null,
    val principalKind: String? = null,
    val role: String? = null
)
