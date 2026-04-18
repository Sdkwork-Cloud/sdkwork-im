package com.sdkwork.craw.chat.backend.api

import com.fasterxml.jackson.core.type.TypeReference
import com.sdkwork.craw.chat.backend.*
import com.sdkwork.craw.chat.backend.http.HttpClient

class SessionApi(private val client: HttpClient) {

    /** Resume the current app session */
    suspend fun resume(body: ResumeSessionRequest): SessionResumeView? {
        val raw = client.post(ApiPaths.backendPath("/sessions/resume"), body, null, null, "application/json")
        return client.convertValue(raw, object : TypeReference<SessionResumeView>() {})
    }

    /** Disconnect the current app session device route */
    suspend fun disconnect(body: PresenceDeviceRequest): PresenceSnapshotView? {
        val raw = client.post(ApiPaths.backendPath("/sessions/disconnect"), body, null, null, "application/json")
        return client.convertValue(raw, object : TypeReference<PresenceSnapshotView>() {})
    }
}
