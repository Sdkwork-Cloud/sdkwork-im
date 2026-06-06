import 'package:sdkwork_common_flutter/sdkwork_common_flutter.dart';
import 'src/http/client.dart';
import 'src/api/automation.dart';
import 'src/api/device.dart';
import 'src/api/notification.dart';
import 'src/api/portal.dart';
import 'src/api/provider.dart';
import 'src/api/iot.dart';
import 'src/api/rtc.dart';

class SdkworkImAppClient {
  final HttpClient _httpClient;

  late final AutomationApi automation;
  late final DeviceApi device;
  late final NotificationApi notification;
  late final PortalApi portal;
  late final ProviderApi provider;
  late final IotApi iot;
  late final RtcApi rtc;

  SdkworkImAppClient({
    required SdkConfig config,
  }) : _httpClient = HttpClient(config: config) {
    automation = AutomationApi(_httpClient);
    device = DeviceApi(_httpClient);
    notification = NotificationApi(_httpClient);
    portal = PortalApi(_httpClient);
    provider = ProviderApi(_httpClient);
    iot = IotApi(_httpClient);
    rtc = RtcApi(_httpClient);
  }

  factory SdkworkImAppClient.withBaseUrl({
    required String baseUrl,
    String? authToken,
    String? accessToken,
    Map<String, String>? headers,
    int timeout = 30000,
  }) {
    return SdkworkImAppClient(
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

typedef SdkworkAppClient = SdkworkImAppClient;
