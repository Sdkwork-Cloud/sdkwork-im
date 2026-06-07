import Foundation

/// API modules for sdkwork-im-sdk
public struct API {
    public static let presence = PresenceApi.self
    public static let realtime = RealtimeApi.self
    public static let rtc = RtcApi.self
    public static let social = SocialApi.self
    public static let chat = ChatApi.self
    public static let streams = StreamsApi.self
}
