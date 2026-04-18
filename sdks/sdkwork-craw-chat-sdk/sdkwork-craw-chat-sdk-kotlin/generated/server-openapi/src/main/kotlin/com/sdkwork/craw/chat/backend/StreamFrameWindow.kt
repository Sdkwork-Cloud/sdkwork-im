package com.sdkwork.craw.chat.backend

data class StreamFrameWindow(
    val items: List<StreamFrame>? = null,
    val nextAfterFrameSeq: Int? = null,
    val hasMore: Boolean? = null
)
