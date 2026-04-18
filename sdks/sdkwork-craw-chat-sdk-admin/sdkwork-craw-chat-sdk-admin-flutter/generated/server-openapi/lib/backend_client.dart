import 'package:sdkwork_common_flutter/sdkwork_common_flutter.dart';
import 'src/http/client.dart';
import 'src/api/cluster.dart';
import 'src/api/protocol.dart';
import 'src/api/providers.dart';
import 'src/api/social.dart';
import 'src/api/system.dart';

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

  late final ClusterApi cluster;
  late final ProtocolApi protocol;
  late final ProvidersApi providers;
  late final SocialApi social;
  late final SystemApi system;

  SdkworkBackendClient({
    required SdkworkBackendConfig config,
  }) : _httpClient = HttpClient(config: config.toSdkConfig()) {
    cluster = ClusterApi(_httpClient);
    protocol = ProtocolApi(_httpClient);
    providers = ProvidersApi(_httpClient);
    social = SocialApi(_httpClient);
    system = SystemApi(_httpClient);
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
