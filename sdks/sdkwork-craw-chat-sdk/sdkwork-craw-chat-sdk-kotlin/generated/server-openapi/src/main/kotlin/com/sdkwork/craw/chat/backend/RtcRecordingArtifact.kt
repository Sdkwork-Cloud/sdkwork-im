package com.sdkwork.craw.chat.backend

data class RtcRecordingArtifact(
    val tenantId: String? = null,
    val rtcSessionId: String? = null,
    val bucket: String? = null,
    val objectKey: String? = null,
    val storageProvider: String? = null,
    val playbackUrl: String? = null
)
