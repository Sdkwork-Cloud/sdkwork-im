import 'package:sdkwork_common_flutter/sdkwork_common_flutter.dart';
import 'src/http/client.dart';
import 'src/api/portal.dart';
import 'src/api/device.dart';
import 'src/api/presence.dart';
import 'src/api/realtime.dart';
import 'src/api/social.dart';
import 'src/api/chat.dart';
import 'src/api/media.dart';
import 'src/api/stream.dart';
import 'src/api/rtc.dart';
import 'src/api/notification.dart';
import 'src/api/automation.dart';

class SdkworkAppClient {
  final HttpClient _httpClient;

  late final PortalApi portal;
  late final DeviceApi device;
  late final PresenceApi presence;
  late final RealtimeApi realtime;
  late final SocialApi social;
  late final ChatApi chat;
  late final MediaApi media;
  late final StreamApi stream;
  late final RtcApi rtc;
  late final NotificationApi notification;
  late final AutomationApi automation;

  SdkworkAppClient({
    required SdkConfig config,
  }) : _httpClient = HttpClient(config: config) {
    portal = PortalApi(_httpClient);
    device = DeviceApi(_httpClient);
    presence = PresenceApi(_httpClient);
    realtime = RealtimeApi(_httpClient);
    social = SocialApi(_httpClient);
    chat = ChatApi(_httpClient);
    media = MediaApi(_httpClient);
    stream = StreamApi(_httpClient);
    rtc = RtcApi(_httpClient);
    notification = NotificationApi(_httpClient);
    automation = AutomationApi(_httpClient);
  }

  factory SdkworkAppClient.withBaseUrl({
    required String baseUrl,
    String? authToken,
    String? accessToken,
    Map<String, String>? headers,
    int timeout = 30000,
  }) {
    return SdkworkAppClient(
      config: SdkConfig(
        baseUrl: baseUrl,
        timeout: timeout,
        headers: headers ?? const {},
        authToken: authToken,
        accessToken: accessToken,
      ),
    );
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
