import 'paths.dart';
import '../http/client.dart';

class TenantsApi {
  final HttpClient _client;

  TenantsApi(this._client);

  /// listProjects
  Future<dynamic> listProjects(
    {
      Map<String, dynamic>? params,
      Map<String, String>? headers,
    }
  ) {
    return _client.get(
      backendApiPath('/api/admin/projects'),
      params: params,
      headers: headers,
    );
  }

  /// saveProject
  Future<dynamic> saveProject(
    {
      dynamic body,
      Map<String, dynamic>? params,
      Map<String, String>? headers,
      String? contentType,
    }
  ) {
    return _client.post(
      backendApiPath('/api/admin/projects'),
      body: body,
      params: params,
      headers: headers,
      contentType: contentType,
    );
  }

  /// deleteProject
  Future<dynamic> deleteProject(
    Object projectId,
    {
      Map<String, dynamic>? params,
      Map<String, String>? headers,
    }
  ) {
    return _client.delete(
      backendApiPath('/api/admin/projects/${Uri.encodeComponent(String(projectId))}'),
      params: params,
      headers: headers,
    );
  }

  /// listTenants
  Future<dynamic> listTenants(
    {
      Map<String, dynamic>? params,
      Map<String, String>? headers,
    }
  ) {
    return _client.get(
      backendApiPath('/api/admin/tenants'),
      params: params,
      headers: headers,
    );
  }

  /// saveTenant
  Future<dynamic> saveTenant(
    {
      dynamic body,
      Map<String, dynamic>? params,
      Map<String, String>? headers,
      String? contentType,
    }
  ) {
    return _client.post(
      backendApiPath('/api/admin/tenants'),
      body: body,
      params: params,
      headers: headers,
      contentType: contentType,
    );
  }

  /// deleteTenant
  Future<dynamic> deleteTenant(
    Object tenantId,
    {
      Map<String, dynamic>? params,
      Map<String, String>? headers,
    }
  ) {
    return _client.delete(
      backendApiPath('/api/admin/tenants/${Uri.encodeComponent(String(tenantId))}'),
      params: params,
      headers: headers,
    );
  }
}
