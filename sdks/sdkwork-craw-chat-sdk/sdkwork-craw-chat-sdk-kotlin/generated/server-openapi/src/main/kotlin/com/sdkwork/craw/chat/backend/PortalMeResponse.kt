package com.sdkwork.craw.chat.backend

data class PortalMeResponse(
    val tenantId: String? = null,
    val user: PortalUserView? = null,
    val workspace: PortalWorkspaceView? = null
)
