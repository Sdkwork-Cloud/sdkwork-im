import '../http/client.dart';

import 'paths.dart';


class RtcApi {
  final HttpClient _client;

  RtcApi(this._client);

  /// Map RTC provider callback
  Future<Map<String, dynamic>?> providerCallbacksCreate() async {
    final response = await _client.post(ApiPaths.appPath('/rtc/provider_callbacks'));
    return response;
  }

  /// Retrieve RTC provider health
  Future<Map<String, dynamic>?> providerHealthRetrieve() async {
    final response = await _client.get(ApiPaths.appPath('/rtc/provider_health'));
    return response;
  }
}
