import Foundation

/// API modules for sdkwork-craw-chat-sdk
public struct API {
    public static let auth = AuthApi.self
    public static let portal = PortalApi.self
    public static let session = SessionApi.self
    public static let presence = PresenceApi.self
    public static let realtime = RealtimeApi.self
    public static let device = DeviceApi.self
    public static let inbox = InboxApi.self
    public static let conversation = ConversationApi.self
    public static let message = MessageApi.self
    public static let media = MediaApi.self
    public static let stream = StreamApi.self
    public static let rtc = RtcApi.self
}
