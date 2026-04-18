package com.sdkwork.craw.chat.backend

data class RealtimeAckState(
    val tenantId: String? = null,
    val principalId: String? = null,
    val deviceId: String? = null,
    val ackedThroughSeq: Int? = null,
    val trimmedThroughSeq: Int? = null,
    val retainedEventCount: Int? = null,
    val ackedAt: String? = null
)
