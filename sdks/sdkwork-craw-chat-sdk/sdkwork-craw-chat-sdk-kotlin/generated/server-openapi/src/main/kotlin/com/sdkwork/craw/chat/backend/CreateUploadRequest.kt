package com.sdkwork.craw.chat.backend

data class CreateUploadRequest(
    val mediaAssetId: String? = null,
    val resource: MediaResource? = null
)
