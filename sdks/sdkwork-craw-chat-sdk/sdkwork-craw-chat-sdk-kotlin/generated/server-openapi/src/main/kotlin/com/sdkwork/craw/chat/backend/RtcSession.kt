package com.sdkwork.craw.chat.backend

data class RtcSession(
    val tenantId: String? = null,
    val rtcSessionId: String? = null,
    val conversationId: String? = null,
    val rtcMode: String? = null,
    val initiatorId: String? = null,
    val providerPluginId: String? = null,
    val providerSessionId: String? = null,
    val accessEndpoint: String? = null,
    val providerRegion: String? = null,
    val state: String? = null,
    val signalingStreamId: String? = null,
    val artifactMessageId: String? = null,
    val startedAt: String? = null,
    val endedAt: String? = null
)
