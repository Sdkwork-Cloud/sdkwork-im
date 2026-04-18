import 'paths.dart';
import '../http/client.dart';

class ProtocolApi {
  final HttpClient _client;

  ProtocolApi(this._client);

  /// get_api_v1_control_protocol_governance
  Future<dynamic> getApiV1ControlProtocolGovernance(
    {
      Map<String, dynamic>? params,
      Map<String, String>? headers,
    }
  ) {
    return _client.get(
      backendApiPath('/api/v1/control/protocol-governance'),
      params: params,
      headers: headers,
    );
  }

  /// get_api_v1_control_protocol_registry
  Future<dynamic> getApiV1ControlProtocolRegistry(
    {
      Map<String, dynamic>? params,
      Map<String, String>? headers,
    }
  ) {
    return _client.get(
      backendApiPath('/api/v1/control/protocol-registry'),
      params: params,
      headers: headers,
    );
  }
}
