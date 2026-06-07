package com.sdkwork.im.sdk.generated.api

import com.sdkwork.im.sdk.generated.http.HttpClient

/**
 * API modules for sdkwork-im-sdk
 */
class Api(private val client: HttpClient) {
    val device: DeviceApi = DeviceApi(client)
    val presence: PresenceApi = PresenceApi(client)
    val realtime: RealtimeApi = RealtimeApi(client)
    val rtc: RtcApi = RtcApi(client)
    val social: SocialApi = SocialApi(client)
    val chat: ChatApi = ChatApi(client)
    val streams: StreamsApi = StreamsApi(client)
}
