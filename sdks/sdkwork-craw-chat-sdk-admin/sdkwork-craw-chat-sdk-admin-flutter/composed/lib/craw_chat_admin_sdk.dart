library craw_chat_admin_sdk;

export 'package:craw_chat_admin_backend_sdk/craw_chat_admin_backend_sdk.dart';

export 'src/context.dart';
export 'src/meta_module.dart';
export 'src/nodes_module.dart';
export 'src/protocol_module.dart';
export 'src/providers_module.dart';
export 'src/social_module.dart';
export 'src/social_runtime_module.dart';
export 'src/types.dart';

import 'package:craw_chat_admin_backend_sdk/craw_chat_admin_backend_sdk.dart';

import 'src/context.dart';
import 'src/meta_module.dart';
import 'src/nodes_module.dart';
import 'src/protocol_module.dart';
import 'src/providers_module.dart';
import 'src/social_module.dart';
import 'src/social_runtime_module.dart';
import 'src/types.dart';

class CrawChatAdminSdkClient {
  final CrawChatAdminSdkContext _context;

  final CrawChatAdminBackendClient backendClient;

  late final CrawChatAdminMetaModule meta;
  late final CrawChatAdminProtocolModule protocol;
  late final CrawChatAdminProvidersModule providers;
  late final CrawChatAdminSocialModule social;
  late final CrawChatAdminSocialRuntimeModule socialRuntime;
  late final CrawChatAdminNodesModule nodes;

  CrawChatAdminSdkClient(CrawChatAdminSdkClientOptions options)
      : backendClient = options.backendClient,
        _context = CrawChatAdminSdkContext(options.backendClient) {
    meta = CrawChatAdminMetaModule(_context);
    protocol = CrawChatAdminProtocolModule(_context);
    providers = CrawChatAdminProvidersModule(_context);
    social = CrawChatAdminSocialModule(_context);
    socialRuntime = CrawChatAdminSocialRuntimeModule(_context);
    nodes = CrawChatAdminNodesModule(_context);
  }

  factory CrawChatAdminSdkClient.create({
    CrawChatAdminBackendClient? backendClient,
    String? baseUrl,
    String? authToken,
    Map<String, String>? headers,
    int timeout = defaultTimeoutMs,
  }) {
    final resolvedConfig = baseUrl == null
        ? null
        : CrawChatAdminBackendConfig(
            baseUrl: baseUrl,
            authToken: authToken,
            headers: headers ?? const <String, String>{},
            timeout: timeout,
          );

    if (backendClient == null && resolvedConfig == null) {
      throw ArgumentError(
        'Provide backendClient or baseUrl when creating CrawChatAdminSdkClient.',
      );
    }

    final resolvedBackendClient =
        backendClient ?? CrawChatAdminBackendClient(config: resolvedConfig!);

    return CrawChatAdminSdkClient(
      CrawChatAdminSdkClientOptions(
        backendClient: resolvedBackendClient,
      ),
    );
  }

  void setAuthToken(String token) {
    _context.setAuthToken(token);
  }
}
