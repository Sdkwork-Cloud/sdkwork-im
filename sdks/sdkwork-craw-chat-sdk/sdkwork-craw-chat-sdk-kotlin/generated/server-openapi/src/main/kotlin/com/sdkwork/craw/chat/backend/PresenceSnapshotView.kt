package com.sdkwork.craw.chat.backend

data class PresenceSnapshotView(
    val tenantId: String? = null,
    val principalId: String? = null,
    val currentDeviceId: String? = null,
    val devices: List<DevicePresenceView>? = null
)
