library im_admin_sdk;

export 'package:im_admin_backend_sdk/im_admin_backend_sdk.dart';
export 'src/context.dart';

import 'package:im_admin_backend_sdk/im_admin_backend_sdk.dart';

import 'src/context.dart';

class ImAdminSdkClientOptions {
  final SdkworkBackendClient backendClient;

  const ImAdminSdkClientOptions({
    required this.backendClient,
  });
}

class ImAdminSdkClient {
  final ImAdminSdkClientContext _context;

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
  late final StorageApi storage;

  ImAdminSdkClient(ImAdminSdkClientOptions options)
      : backendClient = options.backendClient,
        _context = ImAdminSdkClientContext(options.backendClient) {
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
    storage = options.backendClient.storage;
  }

  factory ImAdminSdkClient.create({
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
        'Provide backendClient or baseUrl/backendConfig when creating ImAdminSdkClient.',
      );
    }

    final resolvedBackendClient =
        backendClient ?? SdkworkBackendClient(config: resolvedConfig!);

    return ImAdminSdkClient(
      ImAdminSdkClientOptions(
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

ImAdminSdkClient createImAdminSdkClient({
  SdkworkBackendClient? backendClient,
  SdkworkBackendConfig? backendConfig,
  String? baseUrl,
  String? apiKey,
  String? authToken,
  String? accessToken,
  Map<String, String>? headers,
  int timeout = 30000,
}) {
  return ImAdminSdkClient.create(
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
