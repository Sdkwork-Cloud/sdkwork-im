import '../http/client.dart';
import '../models.dart';
import 'paths.dart';
import 'response_helpers.dart';

class NodesApi {
  final AdminHttpClient _httpClient;

  NodesApi(this._httpClient);

  Future<JsonObject> activateNode(Identifier nodeId) async {
    final response = await _httpClient.post(
      AdminApiPaths.control('/nodes/${encodeIdentifier(nodeId)}/activate'),
    );
    return sdkworkRequireJsonObject(
      response,
      operationName: 'nodes.activateNode',
    );
  }

  Future<JsonObject> drainNode(Identifier nodeId) async {
    final response = await _httpClient.post(
      AdminApiPaths.control('/nodes/${encodeIdentifier(nodeId)}/drain'),
    );
    return sdkworkRequireJsonObject(
      response,
      operationName: 'nodes.drainNode',
    );
  }

  Future<JsonObject> migrateNodeRoutes(
    Identifier nodeId,
    JsonObject body,
  ) async {
    final response = await _httpClient.post(
      AdminApiPaths.control('/nodes/${encodeIdentifier(nodeId)}/routes/migrate'),
      body: body,
    );
    return sdkworkRequireJsonObject(
      response,
      operationName: 'nodes.migrateNodeRoutes',
    );
  }
}
