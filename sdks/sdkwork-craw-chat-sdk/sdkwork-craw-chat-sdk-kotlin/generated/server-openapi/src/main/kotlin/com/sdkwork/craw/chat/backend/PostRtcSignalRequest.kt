package com.sdkwork.craw.chat.backend

data class PostRtcSignalRequest(
    val signalType: String? = null,
    val schemaRef: String? = null,
    val payload: String? = null,
    val signalingStreamId: String? = null
)
