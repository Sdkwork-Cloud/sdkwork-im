import 'paths.dart';
import '../http/client.dart';

class AccessApi {
  final HttpClient _client;

  AccessApi(this._client);

  /// listApiKeyGroups
  Future<dynamic> listApiKeyGroups(
    {
      Map<String, dynamic>? params,
      Map<String, String>? headers,
    }
  ) {
    return _client.get(
      backendApiPath('/api/admin/api-key-groups'),
      params: params,
      headers: headers,
    );
  }

  /// createApiKeyGroup
  Future<dynamic> createApiKeyGroup(
    {
      dynamic body,
      Map<String, dynamic>? params,
      Map<String, String>? headers,
      String? contentType,
    }
  ) {
    return _client.post(
      backendApiPath('/api/admin/api-key-groups'),
      body: body,
      params: params,
      headers: headers,
      contentType: contentType,
    );
  }

  /// deleteApiKeyGroup
  Future<dynamic> deleteApiKeyGroup(
    Object groupId,
    {
      Map<String, dynamic>? params,
      Map<String, String>? headers,
    }
  ) {
    return _client.delete(
      backendApiPath('/api/admin/api-key-groups/${Uri.encodeComponent(String(groupId))}'),
      params: params,
      headers: headers,
    );
  }

  /// updateApiKeyGroup
  Future<dynamic> updateApiKeyGroup(
    Object groupId,
    {
      dynamic body,
      Map<String, dynamic>? params,
      Map<String, String>? headers,
      String? contentType,
    }
  ) {
    return _client.patch(
      backendApiPath('/api/admin/api-key-groups/${Uri.encodeComponent(String(groupId))}'),
      body: body,
      params: params,
      headers: headers,
      contentType: contentType,
    );
  }

  /// updateApiKeyGroupStatus
  Future<dynamic> updateApiKeyGroupStatus(
    Object groupId,
    {
      dynamic body,
      Map<String, dynamic>? params,
      Map<String, String>? headers,
      String? contentType,
    }
  ) {
    return _client.post(
      backendApiPath('/api/admin/api-key-groups/${Uri.encodeComponent(String(groupId))}/status'),
      body: body,
      params: params,
      headers: headers,
      contentType: contentType,
    );
  }

  /// listApiKeys
  Future<dynamic> listApiKeys(
    {
      Map<String, dynamic>? params,
      Map<String, String>? headers,
    }
  ) {
    return _client.get(
      backendApiPath('/api/admin/api-keys'),
      params: params,
      headers: headers,
    );
  }

  /// createApiKey
  Future<dynamic> createApiKey(
    {
      dynamic body,
      Map<String, dynamic>? params,
      Map<String, String>? headers,
      String? contentType,
    }
  ) {
    return _client.post(
      backendApiPath('/api/admin/api-keys'),
      body: body,
      params: params,
      headers: headers,
      contentType: contentType,
    );
  }

  /// deleteApiKey
  Future<dynamic> deleteApiKey(
    Object hashedKey,
    {
      Map<String, dynamic>? params,
      Map<String, String>? headers,
    }
  ) {
    return _client.delete(
      backendApiPath('/api/admin/api-keys/${Uri.encodeComponent(String(hashedKey))}'),
      params: params,
      headers: headers,
    );
  }

  /// updateApiKey
  Future<dynamic> updateApiKey(
    Object hashedKey,
    {
      dynamic body,
      Map<String, dynamic>? params,
      Map<String, String>? headers,
      String? contentType,
    }
  ) {
    return _client.put(
      backendApiPath('/api/admin/api-keys/${Uri.encodeComponent(String(hashedKey))}'),
      body: body,
      params: params,
      headers: headers,
      contentType: contentType,
    );
  }

  /// updateApiKeyStatus
  Future<dynamic> updateApiKeyStatus(
    Object hashedKey,
    {
      dynamic body,
      Map<String, dynamic>? params,
      Map<String, String>? headers,
      String? contentType,
    }
  ) {
    return _client.post(
      backendApiPath('/api/admin/api-keys/${Uri.encodeComponent(String(hashedKey))}/status'),
      body: body,
      params: params,
      headers: headers,
      contentType: contentType,
    );
  }
}
