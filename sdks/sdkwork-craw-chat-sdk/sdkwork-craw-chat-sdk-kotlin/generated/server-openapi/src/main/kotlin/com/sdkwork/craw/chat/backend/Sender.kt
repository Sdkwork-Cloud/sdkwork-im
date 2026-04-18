package com.sdkwork.craw.chat.backend

data class Sender(
    val id: String? = null,
    val kind: String? = null,
    val memberId: String? = null,
    val deviceId: String? = null,
    val sessionId: String? = null,
    val metadata: Map<String, String>? = null
)
