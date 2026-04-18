package com.sdkwork.craw.chat.backend

data class ResumeSessionRequest(
    val deviceId: String? = null,
    val lastSeenSyncSeq: Int? = null
)
