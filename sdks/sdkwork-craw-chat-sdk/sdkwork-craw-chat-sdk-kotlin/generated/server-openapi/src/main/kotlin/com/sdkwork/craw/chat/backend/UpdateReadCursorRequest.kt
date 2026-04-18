package com.sdkwork.craw.chat.backend

data class UpdateReadCursorRequest(
    val readSeq: Int? = null,
    val lastReadMessageId: String? = null
)
