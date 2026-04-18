package com.sdkwork.craw.chat.backend

data class AbortStreamRequest(
    val frameSeq: Int? = null,
    val reason: String? = null
)
