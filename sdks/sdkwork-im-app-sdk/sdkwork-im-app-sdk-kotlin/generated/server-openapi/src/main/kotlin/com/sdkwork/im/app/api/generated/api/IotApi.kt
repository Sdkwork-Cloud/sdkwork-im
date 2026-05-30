package com.sdkwork.im.app.api.generated.api

import com.fasterxml.jackson.core.type.TypeReference
import com.fasterxml.jackson.databind.ObjectMapper
import com.fasterxml.jackson.module.kotlin.registerKotlinModule
import com.sdkwork.im.app.api.generated.*
import com.sdkwork.im.app.api.generated.http.HttpClient

class IotApi(private val client: HttpClient) {

    /** Retrieve IoT access provider health */
    suspend fun accessProviderHealthRetrieve(): Map<String, Any>? {
        val raw = client.get(ApiPaths.appPath("/iot/access/provider_health"))
        return client.convertValue(raw, object : TypeReference<Map<String, Any>>() {})
    }

    /** Retrieve IoT protocol provider health */
    suspend fun protocolProviderHealthRetrieve(): Map<String, Any>? {
        val raw = client.get(ApiPaths.appPath("/iot/protocol/provider_health"))
        return client.convertValue(raw, object : TypeReference<Map<String, Any>>() {})
    }

    /** Ingest IoT protocol uplink */
    suspend fun protocolUplinkCreate(): Map<String, Any>? {
        val raw = client.post(ApiPaths.appPath("/iot/protocol/uplink"), null)
        return client.convertValue(raw, object : TypeReference<Map<String, Any>>() {})
    }

    /** Ingest IoT protocol downlink */
    suspend fun protocolDownlinkCreate(): Map<String, Any>? {
        val raw = client.post(ApiPaths.appPath("/iot/protocol/downlink"), null)
        return client.convertValue(raw, object : TypeReference<Map<String, Any>>() {})
    }



}
