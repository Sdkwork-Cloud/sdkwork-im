import 'paths.dart';
import '../http/client.dart';

class OperationsApi {
  final HttpClient _client;

  OperationsApi(this._client);

  /// reloadExtensionRuntimes
  Future<dynamic> reloadExtensionRuntimes(
    {
      dynamic body,
      Map<String, dynamic>? params,
      Map<String, String>? headers,
      String? contentType,
    }
  ) {
    return _client.post(
      backendApiPath('/api/admin/extensions/runtime-reloads'),
      body: body,
      params: params,
      headers: headers,
      contentType: contentType,
    );
  }

  /// listRuntimeStatuses
  Future<dynamic> listRuntimeStatuses(
    {
      Map<String, dynamic>? params,
      Map<String, String>? headers,
    }
  ) {
    return _client.get(
      backendApiPath('/api/admin/extensions/runtime-statuses'),
      params: params,
      headers: headers,
    );
  }

  /// listRateLimitPolicies
  Future<dynamic> listRateLimitPolicies(
    {
      Map<String, dynamic>? params,
      Map<String, String>? headers,
    }
  ) {
    return _client.get(
      backendApiPath('/api/admin/gateway/rate-limit-policies'),
      params: params,
      headers: headers,
    );
  }

  /// createRateLimitPolicy
  Future<dynamic> createRateLimitPolicy(
    {
      dynamic body,
      Map<String, dynamic>? params,
      Map<String, String>? headers,
      String? contentType,
    }
  ) {
    return _client.post(
      backendApiPath('/api/admin/gateway/rate-limit-policies'),
      body: body,
      params: params,
      headers: headers,
      contentType: contentType,
    );
  }

  /// listRateLimitWindows
  Future<dynamic> listRateLimitWindows(
    {
      Map<String, dynamic>? params,
      Map<String, String>? headers,
    }
  ) {
    return _client.get(
      backendApiPath('/api/admin/gateway/rate-limit-windows'),
      params: params,
      headers: headers,
    );
  }
}
