package com.sdkwork.craw.chat.backend

data class PortalWorkspaceView(
    val name: String? = null,
    val slug: String? = null,
    val tier: String? = null,
    val region: String? = null,
    val supportPlan: String? = null,
    val seats: Int? = null,
    val activeBrands: Int? = null,
    val uptime: String? = null
)
