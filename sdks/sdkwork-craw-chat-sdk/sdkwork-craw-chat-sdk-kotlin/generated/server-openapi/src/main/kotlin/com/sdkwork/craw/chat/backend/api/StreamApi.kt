package com.sdkwork.craw.chat.backend.api

import com.fasterxml.jackson.core.type.TypeReference
import com.sdkwork.craw.chat.backend.*
import com.sdkwork.craw.chat.backend.http.HttpClient

class StreamApi(private val client: HttpClient) {

    /** Open a stream session */
    suspend fun open_(body: OpenStreamRequest): StreamSession? {
        val raw = client.post(ApiPaths.backendPath("/streams"), body, null, null, "application/json")
        return client.convertValue(raw, object : TypeReference<StreamSession>() {})
    }

    /** List stream frames */
    suspend fun listStreamFrames(streamId: String, params: Map<String, Any>? = null): StreamFrameWindow? {
        val raw = client.get(ApiPaths.backendPath("/streams/$streamId/frames"), params)
        return client.convertValue(raw, object : TypeReference<StreamFrameWindow>() {})
    }

    /** Append a frame to a stream */
    suspend fun appendStreamFrame(streamId: String, body: AppendStreamFrameRequest): StreamFrame? {
        val raw = client.post(ApiPaths.backendPath("/streams/$streamId/frames"), body, null, null, "application/json")
        return client.convertValue(raw, object : TypeReference<StreamFrame>() {})
    }

    /** Checkpoint a stream session */
    suspend fun checkpoint(streamId: String, body: CheckpointStreamRequest): StreamSession? {
        val raw = client.post(ApiPaths.backendPath("/streams/$streamId/checkpoint"), body, null, null, "application/json")
        return client.convertValue(raw, object : TypeReference<StreamSession>() {})
    }

    /** Complete a stream session */
    suspend fun complete(streamId: String, body: CompleteStreamRequest): StreamSession? {
        val raw = client.post(ApiPaths.backendPath("/streams/$streamId/complete"), body, null, null, "application/json")
        return client.convertValue(raw, object : TypeReference<StreamSession>() {})
    }

    /** Abort a stream session */
    suspend fun abort(streamId: String, body: AbortStreamRequest): StreamSession? {
        val raw = client.post(ApiPaths.backendPath("/streams/$streamId/abort"), body, null, null, "application/json")
        return client.convertValue(raw, object : TypeReference<StreamSession>() {})
    }
}
