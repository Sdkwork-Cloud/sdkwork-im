package com.sdkwork.craw.chat.backend

data class AppendStreamFrameRequest(
    val frameSeq: Int? = null,
    val frameType: String? = null,
    val schemaRef: String? = null,
    val encoding: String? = null,
    val payload: String? = null,
    val attributes: Map<String, String>? = null
)
