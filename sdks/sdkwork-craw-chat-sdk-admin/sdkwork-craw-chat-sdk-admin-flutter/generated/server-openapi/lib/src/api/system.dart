import 'paths.dart';
import '../http/client.dart';

class SystemApi {
  final HttpClient _client;

  SystemApi(this._client);

  /// get_healthz
  Future<dynamic> getHealthz(
    {
      Map<String, dynamic>? params,
      Map<String, String>? headers,
    }
  ) {
    return _client.get(
      backendApiPath('/healthz'),
      params: params,
      headers: headers,
    );
  }
}
