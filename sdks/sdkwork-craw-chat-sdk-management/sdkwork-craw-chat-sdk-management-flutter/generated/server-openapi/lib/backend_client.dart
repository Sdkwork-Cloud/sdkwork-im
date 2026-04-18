import 'package:sdkwork_common_flutter/sdkwork_common_flutter.dart';
import 'src/http/client.dart';
import 'src/api/auth.dart';
import 'src/api/users.dart';
import 'src/api/marketing.dart';
import 'src/api/tenants.dart';
import 'src/api/access.dart';
import 'src/api/routing.dart';
import 'src/api/catalog.dart';
import 'src/api/usage.dart';
import 'src/api/billing.dart';
import 'src/api/operations.dart';

class SdkworkBackendConfig {
  final String baseUrl;
  final String? apiKey;
  final String? authToken;
  final String? accessToken;
  final Map<String, String> headers;
  final int timeout;
  final String apiKeyHeader;
  final bool apiKeyAsBearer;

  const SdkworkBackendConfig({
    required this.baseUrl,
    this.apiKey,
    this.authToken,
    this.accessToken,
    this.headers = const <String, String>{},
    this.timeout = 30000,
    this.apiKeyHeader = 'Authorization',
    this.apiKeyAsBearer = true,
  });

  SdkConfig toSdkConfig() {
    return SdkConfig(
      baseUrl: baseUrl,
      timeout: timeout,
      headers: headers,
      apiKey: apiKey,
      apiKeyHeader: apiKeyHeader,
      apiKeyAsBearer: apiKeyAsBearer,
      authToken: authToken,
      accessToken: accessToken,
    );
  }
}

class SdkworkBackendClient {
  final HttpClient _httpClient;

  late final AuthApi auth;
  late final UsersApi users;
  late final MarketingApi marketing;
  late final TenantsApi tenants;
  late final AccessApi access;
  late final RoutingApi routing;
  late final CatalogApi catalog;
  late final UsageApi usage;
  late final BillingApi billing;
  late final OperationsApi operations;

  SdkworkBackendClient({
    required SdkworkBackendConfig config,
  }) : _httpClient = HttpClient(config: config.toSdkConfig()) {
    auth = AuthApi(_httpClient);
    users = UsersApi(_httpClient);
    marketing = MarketingApi(_httpClient);
    tenants = TenantsApi(_httpClient);
    access = AccessApi(_httpClient);
    routing = RoutingApi(_httpClient);
    catalog = CatalogApi(_httpClient);
    usage = UsageApi(_httpClient);
    billing = BillingApi(_httpClient);
    operations = OperationsApi(_httpClient);
  }

  factory SdkworkBackendClient.withBaseUrl({
    required String baseUrl,
    String? apiKey,
    String? authToken,
    String? accessToken,
    Map<String, String>? headers,
    int timeout = 30000,
    String apiKeyHeader = 'Authorization',
    bool apiKeyAsBearer = true,
  }) {
    return SdkworkBackendClient(
      config: SdkworkBackendConfig(
        baseUrl: baseUrl,
        apiKey: apiKey,
        authToken: authToken,
        accessToken: accessToken,
        headers: headers ?? const <String, String>{},
        timeout: timeout,
        apiKeyHeader: apiKeyHeader,
        apiKeyAsBearer: apiKeyAsBearer,
      ),
    );
  }

  void setApiKey(String apiKey) {
    _httpClient.setApiKey(apiKey);
  }

  void setAuthToken(String token) {
    _httpClient.setAuthToken(token);
  }

  void setAccessToken(String token) {
    _httpClient.setAccessToken(token);
  }

  void setHeader(String key, String value) {
    _httpClient.setHeader(key, value);
  }
}
