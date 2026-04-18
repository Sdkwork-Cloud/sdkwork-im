import 'src/api/meta.dart';
import 'src/api/nodes.dart';
import 'src/api/protocol.dart';
import 'src/api/providers.dart';
import 'src/api/social.dart';
import 'src/api/social_runtime.dart';
import 'src/http/client.dart';
import 'src/models.dart';

class CrawChatAdminBackendClient {
  final AdminHttpClient _httpClient;

  late final MetaApi meta;
  late final ProtocolApi protocol;
  late final ProvidersApi providers;
  late final SocialApi social;
  late final SocialRuntimeApi socialRuntime;
  late final NodesApi nodes;

  CrawChatAdminBackendClient({
    required CrawChatAdminBackendConfig config,
  }) : _httpClient = AdminHttpClient(config: config) {
    meta = MetaApi(_httpClient);
    protocol = ProtocolApi(_httpClient);
    providers = ProvidersApi(_httpClient);
    social = SocialApi(_httpClient);
    socialRuntime = SocialRuntimeApi(_httpClient);
    nodes = NodesApi(_httpClient);
  }

  factory CrawChatAdminBackendClient.withBaseUrl({
    required String baseUrl,
    String? authToken,
    Map<String, String>? headers,
    int timeout = defaultTimeoutMs,
  }) {
    return CrawChatAdminBackendClient(
      config: CrawChatAdminBackendConfig(
        baseUrl: baseUrl,
        authToken: authToken,
        headers: headers ?? const <String, String>{},
        timeout: timeout,
      ),
    );
  }

  void setAuthToken(String token) {
    _httpClient.setAuthToken(token);
  }

  void setHeader(String key, String value) {
    _httpClient.setHeader(key, value);
  }
}
