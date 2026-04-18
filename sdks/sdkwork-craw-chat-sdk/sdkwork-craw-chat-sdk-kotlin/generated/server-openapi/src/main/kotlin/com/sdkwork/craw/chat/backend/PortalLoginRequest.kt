package com.sdkwork.craw.chat.backend

data class PortalLoginRequest(
    val tenantId: String? = null,
    val login: String? = null,
    val password: String? = null,
    val deviceId: String? = null,
    val sessionId: String? = null,
    val clientKind: String? = null
)
