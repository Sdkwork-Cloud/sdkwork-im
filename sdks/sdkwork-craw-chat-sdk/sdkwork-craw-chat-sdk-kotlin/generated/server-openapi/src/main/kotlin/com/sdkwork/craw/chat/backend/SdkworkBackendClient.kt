package com.sdkwork.craw.chat.backend

import com.sdkwork.common.core.SdkConfig
import com.sdkwork.craw.chat.backend.http.HttpClient
import com.sdkwork.craw.chat.backend.api.AuthApi
import com.sdkwork.craw.chat.backend.api.PortalApi
import com.sdkwork.craw.chat.backend.api.SessionApi
import com.sdkwork.craw.chat.backend.api.PresenceApi
import com.sdkwork.craw.chat.backend.api.RealtimeApi
import com.sdkwork.craw.chat.backend.api.DeviceApi
import com.sdkwork.craw.chat.backend.api.InboxApi
import com.sdkwork.craw.chat.backend.api.ConversationApi
import com.sdkwork.craw.chat.backend.api.MessageApi
import com.sdkwork.craw.chat.backend.api.MediaApi
import com.sdkwork.craw.chat.backend.api.StreamApi
import com.sdkwork.craw.chat.backend.api.RtcApi

class SdkworkBackendClient {
    private val httpClient: HttpClient

    lateinit var auth: AuthApi
    lateinit var portal: PortalApi
    lateinit var session: SessionApi
    lateinit var presence: PresenceApi
    lateinit var realtime: RealtimeApi
    lateinit var device: DeviceApi
    lateinit var inbox: InboxApi
    lateinit var conversation: ConversationApi
    lateinit var message: MessageApi
    lateinit var media: MediaApi
    lateinit var stream: StreamApi
    lateinit var rtc: RtcApi

    constructor(baseUrl: String) {
        this.httpClient = HttpClient(baseUrl)
        auth = AuthApi(httpClient)
        portal = PortalApi(httpClient)
        session = SessionApi(httpClient)
        presence = PresenceApi(httpClient)
        realtime = RealtimeApi(httpClient)
        device = DeviceApi(httpClient)
        inbox = InboxApi(httpClient)
        conversation = ConversationApi(httpClient)
        message = MessageApi(httpClient)
        media = MediaApi(httpClient)
        stream = StreamApi(httpClient)
        rtc = RtcApi(httpClient)
    }

    constructor(config: SdkConfig) {
        this.httpClient = HttpClient(config)
        auth = AuthApi(httpClient)
        portal = PortalApi(httpClient)
        session = SessionApi(httpClient)
        presence = PresenceApi(httpClient)
        realtime = RealtimeApi(httpClient)
        device = DeviceApi(httpClient)
        inbox = InboxApi(httpClient)
        conversation = ConversationApi(httpClient)
        message = MessageApi(httpClient)
        media = MediaApi(httpClient)
        stream = StreamApi(httpClient)
        rtc = RtcApi(httpClient)
    }

    fun setApiKey(apiKey: String): SdkworkBackendClient {
        httpClient.setApiKey(apiKey)
        return this
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
