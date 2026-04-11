import '../http/client.dart';
import '../models.dart';
import 'paths.dart';
import 'response_helpers.dart';

class PresenceApi {
  final HttpClient _client;

  PresenceApi(this._client);

  /// Refresh device presence
  Future<PresenceSnapshotView?> heartbeat(PresenceDeviceRequest body) async {
    final payload = body.toJson();
    final response = await _client.post(ApiPaths.backendPath('/presence/heartbeat'), body: payload, contentType: 'application/json');
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : PresenceSnapshotView.fromJson(map);
    })();
  }

  /// Get current presence
  Future<PresenceSnapshotView?> getPresenceMe() async {
    final response = await _client.get(ApiPaths.backendPath('/presence/me'));
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : PresenceSnapshotView.fromJson(map);
    })();
  }
}
