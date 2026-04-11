import '../http/client.dart';
import '../models.dart';
import 'paths.dart';
import 'response_helpers.dart';

class MessageApi {
  final HttpClient _client;

  MessageApi(this._client);

  /// Edit a posted message
  Future<MessageMutationResult?> edit(String messageId, EditMessageRequest body) async {
    final payload = body.toJson();
    final response = await _client.post(ApiPaths.backendPath('/messages/$messageId/edit'), body: payload, contentType: 'application/json');
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : MessageMutationResult.fromJson(map);
    })();
  }

  /// Recall a posted message
  Future<MessageMutationResult?> recall(String messageId) async {
    final response = await _client.post(ApiPaths.backendPath('/messages/$messageId/recall'));
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : MessageMutationResult.fromJson(map);
    })();
  }
}
