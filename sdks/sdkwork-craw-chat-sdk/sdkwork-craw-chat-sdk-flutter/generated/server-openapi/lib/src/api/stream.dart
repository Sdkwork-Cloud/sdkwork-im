import '../http/client.dart';
import '../models.dart';
import 'paths.dart';
import 'response_helpers.dart';

class StreamApi {
  final HttpClient _client;

  StreamApi(this._client);

  /// Open a stream session
  Future<StreamSession?> open(OpenStreamRequest body) async {
    final payload = body.toJson();
    final response = await _client.post(ApiPaths.backendPath('/streams'), body: payload, contentType: 'application/json');
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : StreamSession.fromJson(map);
    })();
  }

  /// List stream frames
  Future<StreamFrameWindow?> listStreamFrames(String streamId, Map<String, dynamic>? params) async {
    final response = await _client.get(ApiPaths.backendPath('/streams/$streamId/frames'), params: params);
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : StreamFrameWindow.fromJson(map);
    })();
  }

  /// Append a frame to a stream
  Future<StreamFrame?> appendStreamFrame(String streamId, AppendStreamFrameRequest body) async {
    final payload = body.toJson();
    final response = await _client.post(ApiPaths.backendPath('/streams/$streamId/frames'), body: payload, contentType: 'application/json');
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : StreamFrame.fromJson(map);
    })();
  }

  /// Checkpoint a stream session
  Future<StreamSession?> checkpoint(String streamId, CheckpointStreamRequest body) async {
    final payload = body.toJson();
    final response = await _client.post(ApiPaths.backendPath('/streams/$streamId/checkpoint'), body: payload, contentType: 'application/json');
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : StreamSession.fromJson(map);
    })();
  }

  /// Complete a stream session
  Future<StreamSession?> complete(String streamId, CompleteStreamRequest body) async {
    final payload = body.toJson();
    final response = await _client.post(ApiPaths.backendPath('/streams/$streamId/complete'), body: payload, contentType: 'application/json');
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : StreamSession.fromJson(map);
    })();
  }

  /// Abort a stream session
  Future<StreamSession?> abort(String streamId, AbortStreamRequest body) async {
    final payload = body.toJson();
    final response = await _client.post(ApiPaths.backendPath('/streams/$streamId/abort'), body: payload, contentType: 'application/json');
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : StreamSession.fromJson(map);
    })();
  }
}
