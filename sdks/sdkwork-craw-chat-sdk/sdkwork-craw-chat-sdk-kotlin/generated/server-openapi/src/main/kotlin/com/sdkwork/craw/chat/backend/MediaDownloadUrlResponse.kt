package com.sdkwork.craw.chat.backend

data class MediaDownloadUrlResponse(
    val mediaAssetId: String? = null,
    val storageProvider: String? = null,
    val downloadUrl: String? = null,
    val expiresInSeconds: Int? = null
)
