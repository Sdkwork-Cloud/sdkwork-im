library craw_chat_sdk_admin;

export 'package:craw_chat_admin_backend_sdk/craw_chat_admin_backend_sdk.dart';
export 'src/context.dart';

import 'package:craw_chat_admin_backend_sdk/craw_chat_admin_backend_sdk.dart';

import 'src/context.dart';

class CrawChatAdminClientOptions {
  final SdkworkBackendClient backendClient;

  const CrawChatAdminClientOptions({
    required this.backendClient,
  });
}

class CrawChatAdminClient {
  final CrawChatAdminClientContext _context;

  final SdkworkBackendClient backendClient;

  late final ClusterApi cluster;
  late final ProtocolApi protocol;
  late final ProvidersApi providers;
  late final SocialApi social;
  late final SystemApi system;

  CrawChatAdminClient(CrawChatAdminClientOptions options)
      : backendClient = options.backendClient,
        _context = CrawChatAdminClientContext(options.backendClient) {
    cluster = options.backendClient.cluster;
    protocol = options.backendClient.protocol;
    providers = options.backendClient.providers;
    social = options.backendClient.social;
    system = options.backendClient.system;
  }

  factory CrawChatAdminClient.create({
    SdkworkBackendClient? backendClient,
    SdkworkBackendConfig? backendConfig,
    String? baseUrl,
    String? apiKey,
    String? authToken,
    String? accessToken,
    Map<String, String>? headers,
    int timeout = 30000,
  }) {
    final resolvedConfig = backendConfig ??
        (baseUrl == null
            ? null
            : SdkworkBackendConfig(
                baseUrl: baseUrl,
                apiKey: apiKey,
                authToken: authToken,
                accessToken: accessToken,
                headers: headers ?? const <String, String>{},
                timeout: timeout,
              ));

    if (backendClient == null && resolvedConfig == null) {
      throw ArgumentError(
        'Provide backendClient or baseUrl/backendConfig when creating CrawChatAdminClient.',
      );
    }

    final resolvedBackendClient =
        backendClient ?? SdkworkBackendClient(config: resolvedConfig!);

    return CrawChatAdminClient(
      CrawChatAdminClientOptions(
        backendClient: resolvedBackendClient,
      ),
    );
  }

  void setApiKey(String apiKey) {
    _context.setApiKey(apiKey);
  }

  void setAuthToken(String token) {
    _context.setAuthToken(token);
  }

  void setAccessToken(String token) {
    _context.setAccessToken(token);
  }
}

CrawChatAdminClient createCrawChatAdminClient({
  SdkworkBackendClient? backendClient,
  SdkworkBackendConfig? backendConfig,
  String? baseUrl,
  String? apiKey,
  String? authToken,
  String? accessToken,
  Map<String, String>? headers,
  int timeout = 30000,
}) {
  return CrawChatAdminClient.create(
    backendClient: backendClient,
    backendConfig: backendConfig,
    baseUrl: baseUrl,
    apiKey: apiKey,
    authToken: authToken,
    accessToken: accessToken,
    headers: headers,
    timeout: timeout,
  );
}
