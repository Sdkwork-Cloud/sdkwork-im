import Foundation
import SDKworkCommon

public class SdkworkAppClient {
    private let httpClient: HttpClient
    public let automation: AutomationApi
    public let device: DeviceApi
    public let notification: NotificationApi
    public let portal: PortalApi
    public let provider: ProviderApi
    public let iot: IotApi
    public let rtc: RtcApi

    public init(baseURL: String) {
        self.httpClient = HttpClient(baseURL: baseURL)
        self.automation = AutomationApi(client: httpClient)
        self.device = DeviceApi(client: httpClient)
        self.notification = NotificationApi(client: httpClient)
        self.portal = PortalApi(client: httpClient)
        self.provider = ProviderApi(client: httpClient)
        self.iot = IotApi(client: httpClient)
        self.rtc = RtcApi(client: httpClient)
    }

    public init(config: SdkConfig) {
        self.httpClient = HttpClient(config: config)
        self.automation = AutomationApi(client: httpClient)
        self.device = DeviceApi(client: httpClient)
        self.notification = NotificationApi(client: httpClient)
        self.portal = PortalApi(client: httpClient)
        self.provider = ProviderApi(client: httpClient)
        self.iot = IotApi(client: httpClient)
        self.rtc = RtcApi(client: httpClient)
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
