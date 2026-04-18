package com.sdkwork.craw.chat.backend

data class PortalLoginResponse(
    val accessToken: String? = null,
    val refreshToken: String? = null,
    val expiresAt: Int? = null,
    val user: PortalUserView? = null,
    val workspace: PortalWorkspaceView? = null
)
