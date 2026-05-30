import 'package:sdkwork_common_flutter/sdkwork_common_flutter.dart';
import 'src/http/client.dart';
import 'src/api/ops.dart';
import 'src/api/audit.dart';
import 'src/api/provider.dart';
import 'src/api/iot.dart';
import 'src/api/rtc.dart';
import 'src/api/automation.dart';

class SdkworkBackendClient {
  final HttpClient _httpClient;

  late final OpsApi ops;
  late final AuditApi audit;
  late final ProviderApi provider;
  late final IotApi iot;
  late final RtcApi rtc;
  late final AutomationApi automation;

  SdkworkBackendClient({
    required SdkConfig config,
  }) : _httpClient = HttpClient(config: config) {
    ops = OpsApi(_httpClient);
    audit = AuditApi(_httpClient);
    provider = ProviderApi(_httpClient);
    iot = IotApi(_httpClient);
    rtc = RtcApi(_httpClient);
    automation = AutomationApi(_httpClient);
  }

  factory SdkworkBackendClient.withBaseUrl({
    required String baseUrl,
    String? authToken,
    String? accessToken,
    Map<String, String>? headers,
    int timeout = 30000,
  }) {
    return SdkworkBackendClient(
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
