import '../http/client.dart';
import '../models.dart';
import 'paths.dart';
import 'response_helpers.dart';

class InboxApi {
  final HttpClient _client;

  InboxApi(this._client);

  /// Get inbox entries
  Future<InboxResponse?> getInbox() async {
    final response = await _client.get(ApiPaths.backendPath('/inbox'));
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : InboxResponse.fromJson(map);
    })();
  }
}
