package com.sdkwork.craw.chat.backend

data class MediaResource(
    val id: Int? = null,
    val uuid: String? = null,
    val url: String? = null,
    val bytes: List<Int>? = null,
    val localFile: String? = null,
    val base64: String? = null,
    val type: String? = null,
    val mimeType: String? = null,
    val size: Int? = null,
    val name: String? = null,
    val extension: String? = null,
    val tags: Map<String, String>? = null,
    val metadata: Map<String, String>? = null,
    val prompt: String? = null
)
