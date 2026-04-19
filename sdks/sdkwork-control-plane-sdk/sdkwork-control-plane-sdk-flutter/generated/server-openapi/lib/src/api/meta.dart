import '../http/client.dart';
import '../models.dart';
import 'paths.dart';
import 'response_helpers.dart';

class MetaApi {
  final AdminHttpClient _httpClient;

  MetaApi(this._httpClient);

  Future<JsonObject> getHealthz() async {
    final response = await _httpClient.get(AdminApiPaths.backendPath('/healthz'));
    return sdkworkRequireJsonObject(
      response,
      operationName: 'meta.getHealthz',
    );
  }
}
