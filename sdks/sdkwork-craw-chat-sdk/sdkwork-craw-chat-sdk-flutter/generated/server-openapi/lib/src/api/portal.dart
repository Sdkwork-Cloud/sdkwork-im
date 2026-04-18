import '../http/client.dart';
import '../models.dart';
import 'paths.dart';
import 'response_helpers.dart';

class PortalApi {
  final HttpClient _client;

  PortalApi(this._client);

  /// Read the tenant portal home snapshot
  Future<Map<String, dynamic>?> getHome() async {
    final response = await _client.get(ApiPaths.backendPath('/portal/home'));
    return response;
  }

  /// Read the tenant portal sign-in snapshot
  Future<Map<String, dynamic>?> getAuth() async {
    final response = await _client.get(ApiPaths.backendPath('/portal/auth'));
    return response;
  }

  /// Read the current tenant workspace snapshot
  Future<PortalWorkspaceView?> getWorkspace() async {
    final response = await _client.get(ApiPaths.backendPath('/portal/workspace'));
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : PortalWorkspaceView.fromJson(map);
    })();
  }

  /// Read the tenant dashboard snapshot
  Future<Map<String, dynamic>?> getDashboard() async {
    final response = await _client.get(ApiPaths.backendPath('/portal/dashboard'));
    return response;
  }

  /// Read the tenant conversations snapshot
  Future<Map<String, dynamic>?> getConversations() async {
    final response = await _client.get(ApiPaths.backendPath('/portal/conversations'));
    return response;
  }

  /// Read the tenant realtime snapshot
  Future<Map<String, dynamic>?> getRealtime() async {
    final response = await _client.get(ApiPaths.backendPath('/portal/realtime'));
    return response;
  }

  /// Read the tenant media snapshot
  Future<Map<String, dynamic>?> getMedia() async {
    final response = await _client.get(ApiPaths.backendPath('/portal/media'));
    return response;
  }

  /// Read the tenant automation snapshot
  Future<Map<String, dynamic>?> getAutomation() async {
    final response = await _client.get(ApiPaths.backendPath('/portal/automation'));
    return response;
  }

  /// Read the tenant governance snapshot
  Future<Map<String, dynamic>?> getGovernance() async {
    final response = await _client.get(ApiPaths.backendPath('/portal/governance'));
    return response;
  }
}
