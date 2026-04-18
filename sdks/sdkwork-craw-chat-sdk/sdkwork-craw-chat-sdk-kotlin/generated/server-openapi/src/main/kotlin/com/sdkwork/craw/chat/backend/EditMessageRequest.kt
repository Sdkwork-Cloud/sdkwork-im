package com.sdkwork.craw.chat.backend

data class EditMessageRequest(
    val summary: String? = null,
    val text: String? = null,
    val parts: List<ContentPart>? = null,
    val renderHints: Map<String, String>? = null
)
