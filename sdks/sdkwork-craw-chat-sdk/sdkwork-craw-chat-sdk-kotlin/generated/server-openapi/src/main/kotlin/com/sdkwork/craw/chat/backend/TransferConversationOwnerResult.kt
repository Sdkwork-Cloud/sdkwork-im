package com.sdkwork.craw.chat.backend

data class TransferConversationOwnerResult(
    val eventId: String? = null,
    val transferredAt: String? = null,
    val previousOwner: ConversationMember? = null,
    val newOwner: ConversationMember? = null
)
