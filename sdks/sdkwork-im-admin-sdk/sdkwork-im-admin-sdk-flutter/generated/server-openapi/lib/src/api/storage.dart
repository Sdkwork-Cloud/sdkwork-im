import 'paths.dart';
import '../http/client.dart';

class StorageApi {
  final HttpClient _client;

  StorageApi(this._client);

  /// listStorageAuditTrail
  Future<dynamic> listStorageAuditTrail(
    {
      Map<String, dynamic>? params,
      Map<String, String>? headers,
    }
  ) {
    return _client.get(
      backendApiPath('/api/admin/storage/audit'),
      params: params,
      headers: headers,
    );
  }

  /// getGlobalStorageConfig
  Future<dynamic> getGlobalStorageConfig(
    {
      Map<String, dynamic>? params,
      Map<String, String>? headers,
    }
  ) {
    return _client.get(
      backendApiPath('/api/admin/storage/config'),
      params: params,
      headers: headers,
    );
  }

  /// saveGlobalStorageConfig
  Future<dynamic> saveGlobalStorageConfig(
    {
      dynamic body,
      Map<String, dynamic>? params,
      Map<String, String>? headers,
      String? contentType,
    }
  ) {
    return _client.post(
      backendApiPath('/api/admin/storage/config'),
      body: body,
      params: params,
      headers: headers,
      contentType: contentType,
    );
  }

  /// deleteTenantStorageConfig
  Future<dynamic> deleteTenantStorageConfig(
    Object tenantId,
    {
      Map<String, dynamic>? params,
      Map<String, String>? headers,
    }
  ) {
    return _client.delete(
      backendApiPath('/api/admin/storage/config/tenants/${Uri.encodeComponent(String(tenantId))}'),
      params: params,
      headers: headers,
    );
  }

  /// getTenantStorageConfig
  Future<dynamic> getTenantStorageConfig(
    Object tenantId,
    {
      Map<String, dynamic>? params,
      Map<String, String>? headers,
    }
  ) {
    return _client.get(
      backendApiPath('/api/admin/storage/config/tenants/${Uri.encodeComponent(String(tenantId))}'),
      params: params,
      headers: headers,
    );
  }

  /// saveTenantStorageConfig
  Future<dynamic> saveTenantStorageConfig(
    Object tenantId,
    {
      dynamic body,
      Map<String, dynamic>? params,
      Map<String, String>? headers,
      String? contentType,
    }
  ) {
    return _client.post(
      backendApiPath('/api/admin/storage/config/tenants/${Uri.encodeComponent(String(tenantId))}'),
      body: body,
      params: params,
      headers: headers,
      contentType: contentType,
    );
  }

  /// getTenantEffectiveStorageConfig
  Future<dynamic> getTenantEffectiveStorageConfig(
    Object tenantId,
    {
      Map<String, dynamic>? params,
      Map<String, String>? headers,
    }
  ) {
    return _client.get(
      backendApiPath('/api/admin/storage/effective/tenants/${Uri.encodeComponent(String(tenantId))}'),
      params: params,
      headers: headers,
    );
  }

  /// listStorageProviders
  Future<dynamic> listStorageProviders(
    {
      Map<String, dynamic>? params,
      Map<String, String>? headers,
    }
  ) {
    return _client.get(
      backendApiPath('/api/admin/storage/providers'),
      params: params,
      headers: headers,
    );
  }

  /// validateGlobalStorageConfig
  Future<dynamic> validateGlobalStorageConfig(
    {
      dynamic body,
      Map<String, dynamic>? params,
      Map<String, String>? headers,
      String? contentType,
    }
  ) {
    return _client.post(
      backendApiPath('/api/admin/storage/validate'),
      body: body,
      params: params,
      headers: headers,
      contentType: contentType,
    );
  }

  /// validateTenantStorageConfig
  Future<dynamic> validateTenantStorageConfig(
    Object tenantId,
    {
      dynamic body,
      Map<String, dynamic>? params,
      Map<String, String>? headers,
      String? contentType,
    }
  ) {
    return _client.post(
      backendApiPath('/api/admin/storage/validate/tenants/${Uri.encodeComponent(String(tenantId))}'),
      body: body,
      params: params,
      headers: headers,
      contentType: contentType,
    );
  }
}
