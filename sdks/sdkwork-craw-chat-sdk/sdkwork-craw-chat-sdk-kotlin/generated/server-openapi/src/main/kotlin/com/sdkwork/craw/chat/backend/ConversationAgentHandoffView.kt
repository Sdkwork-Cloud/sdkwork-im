package com.sdkwork.craw.chat.backend

data class ConversationAgentHandoffView(
    val status: String? = null,
    val source: ConversationActorView? = null,
    val target: ConversationActorView? = null,
    val handoffSessionId: String? = null,
    val handoffReason: String? = null,
    val acceptedAt: String? = null,
    val acceptedBy: ConversationActorView? = null,
    val resolvedAt: String? = null,
    val resolvedBy: ConversationActorView? = null,
    val closedAt: String? = null,
    val closedBy: ConversationActorView? = null
)
