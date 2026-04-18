package com.sdkwork.craw.chat.backend

data class MediaAsset(
    val tenantId: String? = null,
    val principalId: String? = null,
    val principalKind: String? = null,
    val mediaAssetId: String? = null,
    val bucket: String? = null,
    val objectKey: String? = null,
    val storageProvider: String? = null,
    val checksum: String? = null,
    val processingState: String? = null,
    val resource: MediaResource? = null,
    val createdAt: String? = null,
    val completedAt: String? = null
)
