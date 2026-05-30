package com.sdkwork.im.backend.api.generated

import com.sdkwork.common.core.SdkConfig
import com.sdkwork.im.backend.api.generated.http.HttpClient
import com.sdkwork.im.backend.api.generated.api.OpsApi
import com.sdkwork.im.backend.api.generated.api.AuditApi
import com.sdkwork.im.backend.api.generated.api.ProviderApi
import com.sdkwork.im.backend.api.generated.api.IotApi
import com.sdkwork.im.backend.api.generated.api.RtcApi
import com.sdkwork.im.backend.api.generated.api.AutomationApi

class SdkworkBackendClient {
    private val httpClient: HttpClient

    lateinit var ops: OpsApi
    lateinit var audit: AuditApi
    lateinit var provider: ProviderApi
    lateinit var iot: IotApi
    lateinit var rtc: RtcApi
    lateinit var automation: AutomationApi

    constructor(baseUrl: String) {
        this.httpClient = HttpClient(baseUrl)
        ops = OpsApi(httpClient)
        audit = AuditApi(httpClient)
        provider = ProviderApi(httpClient)
        iot = IotApi(httpClient)
        rtc = RtcApi(httpClient)
        automation = AutomationApi(httpClient)
    }

    constructor(config: SdkConfig) {
        this.httpClient = HttpClient(config)
        ops = OpsApi(httpClient)
        audit = AuditApi(httpClient)
        provider = ProviderApi(httpClient)
        iot = IotApi(httpClient)
        rtc = RtcApi(httpClient)
        automation = AutomationApi(httpClient)
    }


    fun setAuthToken(token: String): SdkworkBackendClient {
        httpClient.setAuthToken(token)
        return this
    }

    fun setAccessToken(token: String): SdkworkBackendClient {
        httpClient.setAccessToken(token)
        return this
    }

    fun setHeader(key: String, value: String): SdkworkBackendClient {
        httpClient.setHeader(key, value)
        return this
    }
}
