import '../http/client.dart';

import 'paths.dart';


class IotApi {
  final HttpClient _client;

  IotApi(this._client);

  /// Retrieve IoT access provider health
  Future<Map<String, dynamic>?> accessProviderHealthRetrieve() async {
    final response = await _client.get(ApiPaths.appPath('/iot/access/provider_health'));
    return response;
  }

  /// Retrieve IoT protocol provider health
  Future<Map<String, dynamic>?> protocolProviderHealthRetrieve() async {
    final response = await _client.get(ApiPaths.appPath('/iot/protocol/provider_health'));
    return response;
  }

  /// Ingest IoT protocol uplink
  Future<Map<String, dynamic>?> protocolUplinkCreate() async {
    final response = await _client.post(ApiPaths.appPath('/iot/protocol/uplink'));
    return response;
  }

  /// Ingest IoT protocol downlink
  Future<Map<String, dynamic>?> protocolDownlinkCreate() async {
    final response = await _client.post(ApiPaths.appPath('/iot/protocol/downlink'));
    return response;
  }
}
