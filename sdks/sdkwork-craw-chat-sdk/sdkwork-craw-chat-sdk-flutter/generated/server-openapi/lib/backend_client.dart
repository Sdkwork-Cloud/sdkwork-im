import 'package:sdkwork_common_flutter/sdkwork_common_flutter.dart';
import 'src/http/client.dart';
import 'src/api/auth.dart';
import 'src/api/portal.dart';
import 'src/api/session.dart';
import 'src/api/presence.dart';
import 'src/api/realtime.dart';
import 'src/api/device.dart';
import 'src/api/inbox.dart';
import 'src/api/conversation.dart';
import 'src/api/message.dart';
import 'src/api/media.dart';
import 'src/api/stream.dart';
import 'src/api/rtc.dart';

class SdkworkBackendConfig {
  final String baseUrl;
  final String? authToken;
  final Map<String, String> headers;
  final int timeout;

  const SdkworkBackendConfig({
    required this.baseUrl,
    this.authToken,
    this.headers = const <String, String>{},
    this.timeout = 30000,
  });

  SdkConfig toSdkConfig() {
    return SdkConfig(
      baseUrl: baseUrl,
      timeout: timeout,
      headers: headers,
      authToken: authToken,
    );
  }
}

class SdkworkBackendClient {
  final HttpClient _httpClient;

  late final AuthApi auth;
  late final PortalApi portal;
  late final SessionApi session;
  late final PresenceApi presence;
  late final RealtimeApi realtime;
  late final DeviceApi device;
  late final InboxApi inbox;
  late final ConversationApi conversation;
  late final MessageApi message;
  late final MediaApi media;
  late final StreamApi stream;
  late final RtcApi rtc;

  SdkworkBackendClient({
    required SdkworkBackendConfig config,
  }) : _httpClient = HttpClient(config: config.toSdkConfig()) {
    auth = AuthApi(_httpClient);
    portal = PortalApi(_httpClient);
    session = SessionApi(_httpClient);
    presence = PresenceApi(_httpClient);
    realtime = RealtimeApi(_httpClient);
    device = DeviceApi(_httpClient);
    inbox = InboxApi(_httpClient);
    conversation = ConversationApi(_httpClient);
    message = MessageApi(_httpClient);
    media = MediaApi(_httpClient);
    stream = StreamApi(_httpClient);
    rtc = RtcApi(_httpClient);
  }

  factory SdkworkBackendClient.withBaseUrl({
    required String baseUrl,
    String? authToken,
    Map<String, String>? headers,
    int timeout = 30000,
  }) {
    return SdkworkBackendClient(
      config: SdkworkBackendConfig(
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
