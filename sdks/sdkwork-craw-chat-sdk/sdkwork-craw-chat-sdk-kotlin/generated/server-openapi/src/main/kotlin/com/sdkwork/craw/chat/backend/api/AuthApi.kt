package com.sdkwork.craw.chat.backend.api

import com.fasterxml.jackson.core.type.TypeReference
import com.sdkwork.craw.chat.backend.*
import com.sdkwork.craw.chat.backend.http.HttpClient

class AuthApi(private val client: HttpClient) {

    /** Sign in to the tenant portal */
    suspend fun login(body: PortalLoginRequest): PortalLoginResponse? {
        val raw = client.post(ApiPaths.backendPath("/auth/login"), body, null, null, "application/json")
        return client.convertValue(raw, object : TypeReference<PortalLoginResponse>() {})
    }

    /** Read the current portal session */
    suspend fun me(): PortalMeResponse? {
        val raw = client.get(ApiPaths.backendPath("/auth/me"))
        return client.convertValue(raw, object : TypeReference<PortalMeResponse>() {})
    }
}
