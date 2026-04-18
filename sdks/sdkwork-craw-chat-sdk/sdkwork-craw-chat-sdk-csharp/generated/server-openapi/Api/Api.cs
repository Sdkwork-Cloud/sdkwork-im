namespace Sdkwork.CrawChat.BackendSdk.Api
{
    /// <summary>
    /// API modules for sdkwork-craw-chat-sdk
    /// </summary>
    public static class Api
    {
        public static AuthApi? Auth { get; set; }
        public static PortalApi? Portal { get; set; }
        public static SessionApi? Session { get; set; }
        public static PresenceApi? Presence { get; set; }
        public static RealtimeApi? Realtime { get; set; }
        public static DeviceApi? Device { get; set; }
        public static InboxApi? Inbox { get; set; }
        public static ConversationApi? Conversation { get; set; }
        public static MessageApi? Message { get; set; }
        public static MediaApi? Media { get; set; }
        public static StreamApi? Stream { get; set; }
        public static RtcApi? Rtc { get; set; }
    }
}
