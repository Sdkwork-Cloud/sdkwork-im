package com.sdkwork.craw.chat.backend

data class CompleteUploadRequest(
    val bucket: String? = null,
    val objectKey: String? = null,
    val storageProvider: String? = null,
    val url: String? = null,
    val checksum: String? = null
)
