package com.sdkwork.im.backend.api.generated

import com.sdkwork.common.core.SdkConfig
import com.sdkwork.im.backend.api.generated.http.HttpClient
import com.sdkwork.im.backend.api.generated.api.OpsApi
import com.sdkwork.im.backend.api.generated.api.AuditApi
import com.sdkwork.im.backend.api.generated.api.AutomationApi
import com.sdkwork.im.backend.api.generated.api.ControlApi
import com.sdkwork.im.backend.api.generated.api.AdminApi

class SdkworkBackendClient {
    private val httpClient: HttpClient

    lateinit var ops: OpsApi
    lateinit var audit: AuditApi
    lateinit var automation: AutomationApi
    lateinit var control: ControlApi
    lateinit var admin: AdminApi

    constructor(baseUrl: String) {
        this.httpClient = HttpClient(baseUrl)
        ops = OpsApi(httpClient)
        audit = AuditApi(httpClient)
        automation = AutomationApi(httpClient)
        control = ControlApi(httpClient)
        admin = AdminApi(httpClient)
    }

    constructor(config: SdkConfig) {
        this.httpClient = HttpClient(config)
        ops = OpsApi(httpClient)
        audit = AuditApi(httpClient)
        automation = AutomationApi(httpClient)
        control = ControlApi(httpClient)
        admin = AdminApi(httpClient)
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
