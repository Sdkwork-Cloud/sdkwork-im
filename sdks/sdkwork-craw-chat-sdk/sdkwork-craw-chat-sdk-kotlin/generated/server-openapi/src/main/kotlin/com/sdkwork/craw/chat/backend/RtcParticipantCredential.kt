package com.sdkwork.craw.chat.backend

data class RtcParticipantCredential(
    val tenantId: String? = null,
    val rtcSessionId: String? = null,
    val participantId: String? = null,
    val credential: String? = null,
    val expiresAt: String? = null
)
