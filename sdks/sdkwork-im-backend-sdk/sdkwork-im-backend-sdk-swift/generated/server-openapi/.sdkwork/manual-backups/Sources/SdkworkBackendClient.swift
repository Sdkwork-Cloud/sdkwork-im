import Foundation
import SDKworkCommon

public class SdkworkBackendClient {
    private let httpClient: HttpClient
    public let ops: OpsApi
    public let audit: AuditApi
    public let provider: ProviderApi
    public let iot: IotApi
    public let rtc: RtcApi
    public let automation: AutomationApi

    public init(baseURL: String) {
        self.httpClient = HttpClient(baseURL: baseURL)
        self.ops = OpsApi(client: httpClient)
        self.audit = AuditApi(client: httpClient)
        self.provider = ProviderApi(client: httpClient)
        self.iot = IotApi(client: httpClient)
        self.rtc = RtcApi(client: httpClient)
        self.automation = AutomationApi(client: httpClient)
    }

    public init(config: SdkConfig) {
        self.httpClient = HttpClient(config: config)
        self.ops = OpsApi(client: httpClient)
        self.audit = AuditApi(client: httpClient)
        self.provider = ProviderApi(client: httpClient)
        self.iot = IotApi(client: httpClient)
        self.rtc = RtcApi(client: httpClient)
        self.automation = AutomationApi(client: httpClient)
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
