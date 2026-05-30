import Foundation
import SDKworkCommon

public class SdkworkAppClient {
    private let httpClient: HttpClient
    public let portal: PortalApi
    public let device: DeviceApi
    public let presence: PresenceApi
    public let realtime: RealtimeApi
    public let social: SocialApi
    public let chat: ChatApi
    public let media: MediaApi
    public let stream: StreamApi
    public let rtc: RtcApi
    public let notification: NotificationApi
    public let automation: AutomationApi

    public init(baseURL: String) {
        self.httpClient = HttpClient(baseURL: baseURL)
        self.portal = PortalApi(client: httpClient)
        self.device = DeviceApi(client: httpClient)
        self.presence = PresenceApi(client: httpClient)
        self.realtime = RealtimeApi(client: httpClient)
        self.social = SocialApi(client: httpClient)
        self.chat = ChatApi(client: httpClient)
        self.media = MediaApi(client: httpClient)
        self.stream = StreamApi(client: httpClient)
        self.rtc = RtcApi(client: httpClient)
        self.notification = NotificationApi(client: httpClient)
        self.automation = AutomationApi(client: httpClient)
    }

    public init(config: SdkConfig) {
        self.httpClient = HttpClient(config: config)
        self.portal = PortalApi(client: httpClient)
        self.device = DeviceApi(client: httpClient)
        self.presence = PresenceApi(client: httpClient)
        self.realtime = RealtimeApi(client: httpClient)
        self.social = SocialApi(client: httpClient)
        self.chat = ChatApi(client: httpClient)
        self.media = MediaApi(client: httpClient)
        self.stream = StreamApi(client: httpClient)
        self.rtc = RtcApi(client: httpClient)
        self.notification = NotificationApi(client: httpClient)
        self.automation = AutomationApi(client: httpClient)
    }


    public func setAuthToken(_ token: String) -> SdkworkAppClient {
        httpClient.setAuthToken(token)
        return self
    }

    public func setAccessToken(_ token: String) -> SdkworkAppClient {
        httpClient.setAccessToken(token)
        return self
    }

    public func setHeader(_ key: String, value: String) -> SdkworkAppClient {
        httpClient.setHeader(key, value: value)
        return self
    }
}
