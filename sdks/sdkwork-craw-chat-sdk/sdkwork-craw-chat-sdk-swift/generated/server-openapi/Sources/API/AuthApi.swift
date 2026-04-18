import Foundation

public class AuthApi {
    private let client: HttpClient
    
    public init(client: HttpClient) {
        self.client = client
    }

    /// Sign in to the tenant portal
    public func login(body: PortalLoginRequest) async throws -> PortalLoginResponse? {
        return try await client.post(ApiPaths.backendPath("/auth/login"), body: body, params: nil, headers: nil, contentType: "application/json", responseType: PortalLoginResponse.self)
    }

    /// Read the current portal session
    public func me() async throws -> PortalMeResponse? {
        return try await client.get(ApiPaths.backendPath("/auth/me"), responseType: PortalMeResponse.self)
    }
}
