import '../http/client.dart';
import '../models.dart';
import 'paths.dart';
import 'response_helpers.dart';

class ProtocolApi {
  final AdminHttpClient _httpClient;

  ProtocolApi(this._httpClient);

  Future<JsonObject> getProtocolGovernance() async {
    final response = await _httpClient.get(
      AdminApiPaths.control('/protocol-governance'),
    );
    return sdkworkRequireJsonObject(
      response,
      operationName: 'protocol.getProtocolGovernance',
    );
  }

  Future<JsonObject> getProtocolRegistry() async {
    final response = await _httpClient.get(
      AdminApiPaths.control('/protocol-registry'),
    );
    return sdkworkRequireJsonObject(
      response,
      operationName: 'protocol.getProtocolRegistry',
    );
  }
}
