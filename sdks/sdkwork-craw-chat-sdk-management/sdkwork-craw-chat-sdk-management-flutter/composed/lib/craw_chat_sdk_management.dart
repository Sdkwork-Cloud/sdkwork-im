library craw_chat_sdk_management;

export 'package:craw_chat_management_backend_sdk/craw_chat_management_backend_sdk.dart';
export 'src/context.dart';

import 'package:craw_chat_management_backend_sdk/craw_chat_management_backend_sdk.dart';

import 'src/context.dart';

class CrawChatManagementClientOptions {
  final SdkworkBackendClient backendClient;

  const CrawChatManagementClientOptions({
    required this.backendClient,
  });
}

class CrawChatManagementClient {
  final CrawChatManagementClientContext _context;

  final SdkworkBackendClient backendClient;

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

  CrawChatManagementClient(CrawChatManagementClientOptions options)
      : backendClient = options.backendClient,
        _context = CrawChatManagementClientContext(options.backendClient) {
    auth = options.backendClient.auth;
    users = options.backendClient.users;
    marketing = options.backendClient.marketing;
    tenants = options.backendClient.tenants;
    access = options.backendClient.access;
    routing = options.backendClient.routing;
    catalog = options.backendClient.catalog;
    usage = options.backendClient.usage;
    billing = options.backendClient.billing;
    operations = options.backendClient.operations;
  }

  factory CrawChatManagementClient.create({
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
        'Provide backendClient or baseUrl/backendConfig when creating CrawChatManagementClient.',
      );
    }

    final resolvedBackendClient =
        backendClient ?? SdkworkBackendClient(config: resolvedConfig!);

    return CrawChatManagementClient(
      CrawChatManagementClientOptions(
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

CrawChatManagementClient createCrawChatManagementClient({
  SdkworkBackendClient? backendClient,
  SdkworkBackendConfig? backendConfig,
  String? baseUrl,
  String? apiKey,
  String? authToken,
  String? accessToken,
  Map<String, String>? headers,
  int timeout = 30000,
}) {
  return CrawChatManagementClient.create(
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
