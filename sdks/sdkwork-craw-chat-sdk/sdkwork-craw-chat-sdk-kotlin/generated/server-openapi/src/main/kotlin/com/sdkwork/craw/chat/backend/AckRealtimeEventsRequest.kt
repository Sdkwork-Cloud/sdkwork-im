package com.sdkwork.craw.chat.backend

data class AckRealtimeEventsRequest(
    val deviceId: String? = null,
    val ackedSeq: Int? = null
)
