import '../http/client.dart';
import '../models.dart';
import 'paths.dart';
import 'response_helpers.dart';

class DeviceApi {
  final HttpClient _client;

  DeviceApi(this._client);

  /// Register the current device
  Future<RegisteredDeviceView?> register(RegisterDeviceRequest body) async {
    final payload = body.toJson();
    final response = await _client.post(ApiPaths.backendPath('/devices/register'), body: payload, contentType: 'application/json');
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : RegisteredDeviceView.fromJson(map);
    })();
  }

  /// Get device sync feed entries
  Future<DeviceSyncFeedResponse?> getDeviceSyncFeed(String deviceId, Map<String, dynamic>? params) async {
    final response = await _client.get(ApiPaths.backendPath('/devices/$deviceId/sync-feed'), params: params);
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : DeviceSyncFeedResponse.fromJson(map);
    })();
  }
}
