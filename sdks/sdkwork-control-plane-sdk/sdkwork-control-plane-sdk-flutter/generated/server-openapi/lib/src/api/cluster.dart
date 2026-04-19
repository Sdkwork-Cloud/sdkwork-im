import 'paths.dart';
import '../http/client.dart';

class ClusterApi {
  final HttpClient _client;

  ClusterApi(this._client);

  /// post_api_v1_control_nodes_node_id_activate
  Future<dynamic> postApiV1ControlNodesNodeIdActivate(
    Object nodeId,
    {
      dynamic body,
      Map<String, dynamic>? params,
      Map<String, String>? headers,
      String? contentType,
    }
  ) {
    return _client.post(
      backendApiPath('/api/v1/control/nodes/${Uri.encodeComponent(String(nodeId))}/activate'),
      body: body,
      params: params,
      headers: headers,
      contentType: contentType,
    );
  }

  /// post_api_v1_control_nodes_node_id_drain
  Future<dynamic> postApiV1ControlNodesNodeIdDrain(
    Object nodeId,
    {
      dynamic body,
      Map<String, dynamic>? params,
      Map<String, String>? headers,
      String? contentType,
    }
  ) {
    return _client.post(
      backendApiPath('/api/v1/control/nodes/${Uri.encodeComponent(String(nodeId))}/drain'),
      body: body,
      params: params,
      headers: headers,
      contentType: contentType,
    );
  }

  /// post_api_v1_control_nodes_node_id_routes_migrate
  Future<dynamic> postApiV1ControlNodesNodeIdRoutesMigrate(
    Object nodeId,
    {
      dynamic body,
      Map<String, dynamic>? params,
      Map<String, String>? headers,
      String? contentType,
    }
  ) {
    return _client.post(
      backendApiPath('/api/v1/control/nodes/${Uri.encodeComponent(String(nodeId))}/routes/migrate'),
      body: body,
      params: params,
      headers: headers,
      contentType: contentType,
    );
  }
}
