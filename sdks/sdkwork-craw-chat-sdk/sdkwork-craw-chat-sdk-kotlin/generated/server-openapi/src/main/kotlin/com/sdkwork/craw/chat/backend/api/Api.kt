package com.sdkwork.craw.chat.backend.api

import com.sdkwork.craw.chat.backend.http.HttpClient

/**
 * API modules for sdkwork-craw-chat-sdk
 */
class Api(private val client: HttpClient) {
    val auth: AuthApi = AuthApi(client)
    val portal: PortalApi = PortalApi(client)
    val session: SessionApi = SessionApi(client)
    val presence: PresenceApi = PresenceApi(client)
    val realtime: RealtimeApi = RealtimeApi(client)
    val device: DeviceApi = DeviceApi(client)
    val inbox: InboxApi = InboxApi(client)
    val conversation: ConversationApi = ConversationApi(client)
    val message: MessageApi = MessageApi(client)
    val media: MediaApi = MediaApi(client)
    val stream: StreamApi = StreamApi(client)
    val rtc: RtcApi = RtcApi(client)
}
