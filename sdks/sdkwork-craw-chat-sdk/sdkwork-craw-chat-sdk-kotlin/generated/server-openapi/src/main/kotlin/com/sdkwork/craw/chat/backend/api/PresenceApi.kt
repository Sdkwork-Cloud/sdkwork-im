package com.sdkwork.craw.chat.backend.api

import com.fasterxml.jackson.core.type.TypeReference
import com.sdkwork.craw.chat.backend.*
import com.sdkwork.craw.chat.backend.http.HttpClient

class PresenceApi(private val client: HttpClient) {

    /** Refresh device presence */
    suspend fun heartbeat(body: PresenceDeviceRequest): PresenceSnapshotView? {
        val raw = client.post(ApiPaths.backendPath("/presence/heartbeat"), body, null, null, "application/json")
        return client.convertValue(raw, object : TypeReference<PresenceSnapshotView>() {})
    }

    /** Get current presence */
    suspend fun getPresenceMe(): PresenceSnapshotView? {
        val raw = client.get(ApiPaths.backendPath("/presence/me"))
        return client.convertValue(raw, object : TypeReference<PresenceSnapshotView>() {})
    }
}
