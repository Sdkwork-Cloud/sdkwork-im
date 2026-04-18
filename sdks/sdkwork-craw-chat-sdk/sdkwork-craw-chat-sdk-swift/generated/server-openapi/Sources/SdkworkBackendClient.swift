import Foundation
import SDKworkCommon

public class SdkworkBackendClient {
    private let httpClient: HttpClient
    public let auth: AuthApi
    public let portal: PortalApi
    public let session: SessionApi
    public let presence: PresenceApi
    public let realtime: RealtimeApi
    public let device: DeviceApi
    public let inbox: InboxApi
    public let conversation: ConversationApi
    public let message: MessageApi
    public let media: MediaApi
    public let stream: StreamApi
    public let rtc: RtcApi

    public init(baseURL: String) {
        self.httpClient = HttpClient(baseURL: baseURL)
        self.auth = AuthApi(client: httpClient)
        self.portal = PortalApi(client: httpClient)
        self.session = SessionApi(client: httpClient)
        self.presence = PresenceApi(client: httpClient)
        self.realtime = RealtimeApi(client: httpClient)
        self.device = DeviceApi(client: httpClient)
        self.inbox = InboxApi(client: httpClient)
        self.conversation = ConversationApi(client: httpClient)
        self.message = MessageApi(client: httpClient)
        self.media = MediaApi(client: httpClient)
        self.stream = StreamApi(client: httpClient)
        self.rtc = RtcApi(client: httpClient)
    }

    public init(config: SdkConfig) {
        self.httpClient = HttpClient(config: config)
        self.auth = AuthApi(client: httpClient)
        self.portal = PortalApi(client: httpClient)
        self.session = SessionApi(client: httpClient)
        self.presence = PresenceApi(client: httpClient)
        self.realtime = RealtimeApi(client: httpClient)
        self.device = DeviceApi(client: httpClient)
        self.inbox = InboxApi(client: httpClient)
        self.conversation = ConversationApi(client: httpClient)
        self.message = MessageApi(client: httpClient)
        self.media = MediaApi(client: httpClient)
        self.stream = StreamApi(client: httpClient)
        self.rtc = RtcApi(client: httpClient)
    }

    public func setApiKey(_ apiKey: String) -> SdkworkBackendClient {
        httpClient.setApiKey(apiKey)
        return self
    }

    public func setAuthToken(_ token: String) -> SdkworkBackendClient {
        httpClient.setAuthToken(token)
        return self
    }

    public func setAccessToken(_ token: String) -> SdkworkBackendClient {
        httpClient.setAccessToken(token)
        return self
    }

    public func setHeader(_ key: String, value: String) -> SdkworkBackendClient {
        httpClient.setHeader(key, value: value)
        return self
    }
}
