package com.sdkwork.craw.chat.backend

data class ContentPart(
    val kind: String? = null,
    val text: String? = null,
    val schemaRef: String? = null,
    val encoding: String? = null,
    val payload: String? = null,
    val mediaAssetId: String? = null,
    val resource: MediaResource? = null,
    val signalType: String? = null,
    val streamId: String? = null,
    val streamType: String? = null,
    val state: String? = null
)
