package com.sdkwork.craw.chat.backend

data class PortalUserView(
    val id: String? = null,
    val login: String? = null,
    val name: String? = null,
    val role: String? = null,
    val email: String? = null,
    val actorKind: String? = null,
    val clientKind: String? = null,
    val permissions: List<String>? = null
)
