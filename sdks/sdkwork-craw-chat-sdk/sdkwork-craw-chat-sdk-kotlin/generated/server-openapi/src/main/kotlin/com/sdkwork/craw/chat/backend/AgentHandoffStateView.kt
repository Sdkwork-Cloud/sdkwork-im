package com.sdkwork.craw.chat.backend

data class AgentHandoffStateView(
    val tenantId: String? = null,
    val conversationId: String? = null,
    val status: String? = null,
    val source: ChangeAgentHandoffStatusView? = null,
    val target: ChangeAgentHandoffStatusView? = null,
    val handoffSessionId: String? = null,
    val handoffReason: String? = null,
    val acceptedAt: String? = null,
    val acceptedBy: ChangeAgentHandoffStatusView? = null,
    val resolvedAt: String? = null,
    val resolvedBy: ChangeAgentHandoffStatusView? = null,
    val closedAt: String? = null,
    val closedBy: ChangeAgentHandoffStatusView? = null
)
