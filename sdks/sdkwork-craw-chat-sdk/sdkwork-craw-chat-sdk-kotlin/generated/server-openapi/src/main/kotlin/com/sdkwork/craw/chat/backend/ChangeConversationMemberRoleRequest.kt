package com.sdkwork.craw.chat.backend

data class ChangeConversationMemberRoleRequest(
    val memberId: String? = null,
    val role: String? = null
)
