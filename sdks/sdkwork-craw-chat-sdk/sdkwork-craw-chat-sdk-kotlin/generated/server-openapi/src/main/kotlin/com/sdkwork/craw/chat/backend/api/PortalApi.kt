package com.sdkwork.craw.chat.backend.api

import com.fasterxml.jackson.core.type.TypeReference
import com.sdkwork.craw.chat.backend.*
import com.sdkwork.craw.chat.backend.http.HttpClient

class PortalApi(private val client: HttpClient) {

    /** Read the tenant portal home snapshot */
    suspend fun getHome(): Map<String, Any>? {
        val raw = client.get(ApiPaths.backendPath("/portal/home"))
        return client.convertValue(raw, object : TypeReference<Map<String, Any>>() {})
    }

    /** Read the tenant portal sign-in snapshot */
    suspend fun getAuth(): Map<String, Any>? {
        val raw = client.get(ApiPaths.backendPath("/portal/auth"))
        return client.convertValue(raw, object : TypeReference<Map<String, Any>>() {})
    }

    /** Read the current tenant workspace snapshot */
    suspend fun getWorkspace(): PortalWorkspaceView? {
        val raw = client.get(ApiPaths.backendPath("/portal/workspace"))
        return client.convertValue(raw, object : TypeReference<PortalWorkspaceView>() {})
    }

    /** Read the tenant dashboard snapshot */
    suspend fun getDashboard(): Map<String, Any>? {
        val raw = client.get(ApiPaths.backendPath("/portal/dashboard"))
        return client.convertValue(raw, object : TypeReference<Map<String, Any>>() {})
    }

    /** Read the tenant conversations snapshot */
    suspend fun getConversations(): Map<String, Any>? {
        val raw = client.get(ApiPaths.backendPath("/portal/conversations"))
        return client.convertValue(raw, object : TypeReference<Map<String, Any>>() {})
    }

    /** Read the tenant realtime snapshot */
    suspend fun getRealtime(): Map<String, Any>? {
        val raw = client.get(ApiPaths.backendPath("/portal/realtime"))
        return client.convertValue(raw, object : TypeReference<Map<String, Any>>() {})
    }

    /** Read the tenant media snapshot */
    suspend fun getMedia(): Map<String, Any>? {
        val raw = client.get(ApiPaths.backendPath("/portal/media"))
        return client.convertValue(raw, object : TypeReference<Map<String, Any>>() {})
    }

    /** Read the tenant automation snapshot */
    suspend fun getAutomation(): Map<String, Any>? {
        val raw = client.get(ApiPaths.backendPath("/portal/automation"))
        return client.convertValue(raw, object : TypeReference<Map<String, Any>>() {})
    }

    /** Read the tenant governance snapshot */
    suspend fun getGovernance(): Map<String, Any>? {
        val raw = client.get(ApiPaths.backendPath("/portal/governance"))
        return client.convertValue(raw, object : TypeReference<Map<String, Any>>() {})
    }
}
