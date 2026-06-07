package com.sdkwork.im.sdk.generated

import com.sdkwork.common.core.SdkConfig
import com.sdkwork.im.sdk.generated.http.HttpClient
import com.sdkwork.im.sdk.generated.api.DeviceApi
import com.sdkwork.im.sdk.generated.api.PresenceApi
import com.sdkwork.im.sdk.generated.api.RealtimeApi
import com.sdkwork.im.sdk.generated.api.RtcApi
import com.sdkwork.im.sdk.generated.api.SocialApi
import com.sdkwork.im.sdk.generated.api.ChatApi
import com.sdkwork.im.sdk.generated.api.StreamsApi

open class SdkworkImClient {
    private val httpClient: HttpClient

    lateinit var device: DeviceApi
    lateinit var presence: PresenceApi
    lateinit var realtime: RealtimeApi
    lateinit var rtc: RtcApi
    lateinit var social: SocialApi
    lateinit var chat: ChatApi
    lateinit var streams: StreamsApi

    constructor(baseUrl: String) {
        this.httpClient = HttpClient(baseUrl)
        device = DeviceApi(httpClient)
        presence = PresenceApi(httpClient)
        realtime = RealtimeApi(httpClient)
        rtc = RtcApi(httpClient)
        social = SocialApi(httpClient)
        chat = ChatApi(httpClient)
        streams = StreamsApi(httpClient)
    }

    constructor(config: SdkConfig) {
        this.httpClient = HttpClient(config)
        device = DeviceApi(httpClient)
        presence = PresenceApi(httpClient)
        realtime = RealtimeApi(httpClient)
        rtc = RtcApi(httpClient)
        social = SocialApi(httpClient)
        chat = ChatApi(httpClient)
        streams = StreamsApi(httpClient)
    }
    fun setAuthToken(token: String): SdkworkImClient {
        httpClient.setAuthToken(token)
        return this
    }

    fun setAccessToken(token: String): SdkworkImClient {
        httpClient.setAccessToken(token)
        return this
    }

    fun setHeader(key: String, value: String): SdkworkImClient {
        httpClient.setHeader(key, value)
        return this
    }
}
