package com.sdkwork.im.app.api.generated

import com.sdkwork.common.core.SdkConfig
import com.sdkwork.im.app.api.generated.http.HttpClient
import com.sdkwork.im.app.api.generated.api.AutomationApi
import com.sdkwork.im.app.api.generated.api.DeviceApi
import com.sdkwork.im.app.api.generated.api.NotificationApi
import com.sdkwork.im.app.api.generated.api.PortalApi
import com.sdkwork.im.app.api.generated.api.ProviderApi
import com.sdkwork.im.app.api.generated.api.IotApi
import com.sdkwork.im.app.api.generated.api.RtcApi

class SdkworkAppClient {
    private val httpClient: HttpClient

    lateinit var automation: AutomationApi
    lateinit var device: DeviceApi
    lateinit var notification: NotificationApi
    lateinit var portal: PortalApi
    lateinit var provider: ProviderApi
    lateinit var iot: IotApi
    lateinit var rtc: RtcApi

    constructor(baseUrl: String) {
        this.httpClient = HttpClient(baseUrl)
        automation = AutomationApi(httpClient)
        device = DeviceApi(httpClient)
        notification = NotificationApi(httpClient)
        portal = PortalApi(httpClient)
        provider = ProviderApi(httpClient)
        iot = IotApi(httpClient)
        rtc = RtcApi(httpClient)
    }

    constructor(config: SdkConfig) {
        this.httpClient = HttpClient(config)
        automation = AutomationApi(httpClient)
        device = DeviceApi(httpClient)
        notification = NotificationApi(httpClient)
        portal = PortalApi(httpClient)
        provider = ProviderApi(httpClient)
        iot = IotApi(httpClient)
        rtc = RtcApi(httpClient)
    }


    fun setAuthToken(token: String): SdkworkAppClient {
        httpClient.setAuthToken(token)
        return this
    }

    fun setAccessToken(token: String): SdkworkAppClient {
        httpClient.setAccessToken(token)
        return this
    }

    fun setHeader(key: String, value: String): SdkworkAppClient {
        httpClient.setHeader(key, value)
        return this
    }
}
