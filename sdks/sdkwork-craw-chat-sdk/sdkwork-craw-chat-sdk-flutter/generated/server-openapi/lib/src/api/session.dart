import '../http/client.dart';
import '../models.dart';
import 'paths.dart';
import 'response_helpers.dart';

class SessionApi {
  final HttpClient _client;

  SessionApi(this._client);

  /// Resume the current app session
  Future<SessionResumeView?> resume(ResumeSessionRequest body) async {
    final payload = body.toJson();
    final response = await _client.post(ApiPaths.backendPath('/sessions/resume'), body: payload, contentType: 'application/json');
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : SessionResumeView.fromJson(map);
    })();
  }

  /// Disconnect the current app session device route
  Future<PresenceSnapshotView?> disconnect(PresenceDeviceRequest body) async {
    final payload = body.toJson();
    final response = await _client.post(ApiPaths.backendPath('/sessions/disconnect'), body: payload, contentType: 'application/json');
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : PresenceSnapshotView.fromJson(map);
    })();
  }
}
