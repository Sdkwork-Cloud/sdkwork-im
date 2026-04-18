package com.sdkwork.craw.chat.backend

data class MessageBody(
    val summary: String? = null,
    val parts: List<ContentPart>? = null,
    val renderHints: Map<String, String>? = null
)
