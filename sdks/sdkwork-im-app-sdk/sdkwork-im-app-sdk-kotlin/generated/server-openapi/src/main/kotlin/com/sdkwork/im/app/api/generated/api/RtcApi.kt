package com.sdkwork.im.app.api.generated.api

import com.fasterxml.jackson.core.type.TypeReference
import com.fasterxml.jackson.databind.ObjectMapper
import com.fasterxml.jackson.module.kotlin.registerKotlinModule
import com.sdkwork.im.app.api.generated.*
import com.sdkwork.im.app.api.generated.http.HttpClient

class RtcApi(private val client: HttpClient) {

    /** Map RTC provider callback */
    suspend fun providerCallbacksCreate(): Map<String, Any>? {
        val raw = client.post(ApiPaths.appPath("/rtc/provider_callbacks"), null)
        return client.convertValue(raw, object : TypeReference<Map<String, Any>>() {})
    }

    /** Retrieve RTC provider health */
    suspend fun providerHealthRetrieve(): Map<String, Any>? {
        val raw = client.get(ApiPaths.appPath("/rtc/provider_health"))
        return client.convertValue(raw, object : TypeReference<Map<String, Any>>() {})
    }



}
