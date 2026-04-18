import 'paths.dart';
import '../http/client.dart';

class ProvidersApi {
  final HttpClient _client;

  ProvidersApi(this._client);

  /// get_api_v1_control_provider_bindings
  Future<dynamic> getApiV1ControlProviderBindings(
    {
      Map<String, dynamic>? params,
      Map<String, String>? headers,
    }
  ) {
    return _client.get(
      backendApiPath('/api/v1/control/provider-bindings'),
      params: params,
      headers: headers,
    );
  }

  /// post_api_v1_control_provider_bindings
  Future<dynamic> postApiV1ControlProviderBindings(
    {
      dynamic body,
      Map<String, dynamic>? params,
      Map<String, String>? headers,
      String? contentType,
    }
  ) {
    return _client.post(
      backendApiPath('/api/v1/control/provider-bindings'),
      body: body,
      params: params,
      headers: headers,
      contentType: contentType,
    );
  }

  /// get_api_v1_control_provider_policies
  Future<dynamic> getApiV1ControlProviderPolicies(
    {
      Map<String, dynamic>? params,
      Map<String, String>? headers,
    }
  ) {
    return _client.get(
      backendApiPath('/api/v1/control/provider-policies'),
      params: params,
      headers: headers,
    );
  }

  /// get_api_v1_control_provider_policies_diff
  Future<dynamic> getApiV1ControlProviderPoliciesDiff(
    {
      Map<String, dynamic>? params,
      Map<String, String>? headers,
    }
  ) {
    return _client.get(
      backendApiPath('/api/v1/control/provider-policies/diff'),
      params: params,
      headers: headers,
    );
  }

  /// post_api_v1_control_provider_policies_preview
  Future<dynamic> postApiV1ControlProviderPoliciesPreview(
    {
      dynamic body,
      Map<String, dynamic>? params,
      Map<String, String>? headers,
      String? contentType,
    }
  ) {
    return _client.post(
      backendApiPath('/api/v1/control/provider-policies/preview'),
      body: body,
      params: params,
      headers: headers,
      contentType: contentType,
    );
  }

  /// post_api_v1_control_provider_policies_rollback
  Future<dynamic> postApiV1ControlProviderPoliciesRollback(
    {
      dynamic body,
      Map<String, dynamic>? params,
      Map<String, String>? headers,
      String? contentType,
    }
  ) {
    return _client.post(
      backendApiPath('/api/v1/control/provider-policies/rollback'),
      body: body,
      params: params,
      headers: headers,
      contentType: contentType,
    );
  }

  /// get_api_v1_control_provider_registry
  Future<dynamic> getApiV1ControlProviderRegistry(
    {
      Map<String, dynamic>? params,
      Map<String, String>? headers,
    }
  ) {
    return _client.get(
      backendApiPath('/api/v1/control/provider-registry'),
      params: params,
      headers: headers,
    );
  }
}
