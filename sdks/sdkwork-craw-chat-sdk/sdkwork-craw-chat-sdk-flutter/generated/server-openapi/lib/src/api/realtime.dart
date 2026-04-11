import '../http/client.dart';
import '../models.dart';
import 'paths.dart';
import 'response_helpers.dart';

class RealtimeApi {
  final HttpClient _client;

  RealtimeApi(this._client);

  /// Replace realtime subscriptions for the current device
  Future<RealtimeSubscriptionSnapshot?> syncRealtimeSubscriptions(SyncRealtimeSubscriptionsRequest body) async {
    final payload = body.toJson();
    final response = await _client.post(ApiPaths.backendPath('/realtime/subscriptions/sync'), body: payload, contentType: 'application/json');
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : RealtimeSubscriptionSnapshot.fromJson(map);
    })();
  }

  /// Pull realtime events for the current device
  Future<RealtimeEventWindow?> listRealtimeEvents(Map<String, dynamic>? params) async {
    final response = await _client.get(ApiPaths.backendPath('/realtime/events'), params: params);
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : RealtimeEventWindow.fromJson(map);
    })();
  }

  /// Ack realtime events for the current device
  Future<RealtimeAckState?> ackRealtimeEvents(AckRealtimeEventsRequest body) async {
    final payload = body.toJson();
    final response = await _client.post(ApiPaths.backendPath('/realtime/events/ack'), body: payload, contentType: 'application/json');
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : RealtimeAckState.fromJson(map);
    })();
  }
}
