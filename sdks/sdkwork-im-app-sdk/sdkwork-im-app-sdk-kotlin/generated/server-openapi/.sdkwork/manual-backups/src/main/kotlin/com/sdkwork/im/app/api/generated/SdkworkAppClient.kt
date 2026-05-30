package com.sdkwork.im.app.api.generated

import com.sdkwork.common.core.SdkConfig
import com.sdkwork.im.app.api.generated.http.HttpClient
import com.sdkwork.im.app.api.generated.api.PortalApi
import com.sdkwork.im.app.api.generated.api.DeviceApi
import com.sdkwork.im.app.api.generated.api.PresenceApi
import com.sdkwork.im.app.api.generated.api.RealtimeApi
import com.sdkwork.im.app.api.generated.api.SocialApi
import com.sdkwork.im.app.api.generated.api.ChatApi
import com.sdkwork.im.app.api.generated.api.MediaApi
import com.sdkwork.im.app.api.generated.api.StreamApi
import com.sdkwork.im.app.api.generated.api.RtcApi
import com.sdkwork.im.app.api.generated.api.NotificationApi
import com.sdkwork.im.app.api.generated.api.AutomationApi

class SdkworkAppClient {
    private val httpClient: HttpClient

    lateinit var portal: PortalApi
    lateinit var device: DeviceApi
    lateinit var presence: PresenceApi
    lateinit var realtime: RealtimeApi
    lateinit var social: SocialApi
    lateinit var chat: ChatApi
    lateinit var media: MediaApi
    lateinit var stream: StreamApi
    lateinit var rtc: RtcApi
    lateinit var notification: NotificationApi
    lateinit var automation: AutomationApi

    constructor(baseUrl: String) {
        this.httpClient = HttpClient(baseUrl)
        portal = PortalApi(httpClient)
        device = DeviceApi(httpClient)
        presence = PresenceApi(httpClient)
        realtime = RealtimeApi(httpClient)
        social = SocialApi(httpClient)
        chat = ChatApi(httpClient)
        media = MediaApi(httpClient)
        stream = StreamApi(httpClient)
        rtc = RtcApi(httpClient)
        notification = NotificationApi(httpClient)
        automation = AutomationApi(httpClient)
    }

    constructor(config: SdkConfig) {
        this.httpClient = HttpClient(config)
        portal = PortalApi(httpClient)
        device = DeviceApi(httpClient)
        presence = PresenceApi(httpClient)
        realtime = RealtimeApi(httpClient)
        social = SocialApi(httpClient)
        chat = ChatApi(httpClient)
        media = MediaApi(httpClient)
        stream = StreamApi(httpClient)
        rtc = RtcApi(httpClient)
        notification = NotificationApi(httpClient)
        automation = AutomationApi(httpClient)
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
