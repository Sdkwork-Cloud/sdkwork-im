package com.sdkwork.craw.chat.backend

data class OpenStreamRequest(
    val streamId: String? = null,
    val streamType: String? = null,
    val scopeKind: String? = null,
    val scopeId: String? = null,
    val durabilityClass: String? = null,
    val schemaRef: String? = null
)
