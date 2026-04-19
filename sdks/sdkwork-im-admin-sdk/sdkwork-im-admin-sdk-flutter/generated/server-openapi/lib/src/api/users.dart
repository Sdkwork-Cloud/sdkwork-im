import 'paths.dart';
import '../http/client.dart';

class UsersApi {
  final HttpClient _client;

  UsersApi(this._client);

  /// listOperatorUsers
  Future<dynamic> listOperatorUsers(
    {
      Map<String, dynamic>? params,
      Map<String, String>? headers,
    }
  ) {
    return _client.get(
      backendApiPath('/api/admin/users/operators'),
      params: params,
      headers: headers,
    );
  }

  /// saveOperatorUser
  Future<dynamic> saveOperatorUser(
    {
      dynamic body,
      Map<String, dynamic>? params,
      Map<String, String>? headers,
      String? contentType,
    }
  ) {
    return _client.post(
      backendApiPath('/api/admin/users/operators'),
      body: body,
      params: params,
      headers: headers,
      contentType: contentType,
    );
  }

  /// deleteOperatorUser
  Future<dynamic> deleteOperatorUser(
    Object userId,
    {
      Map<String, dynamic>? params,
      Map<String, String>? headers,
    }
  ) {
    return _client.delete(
      backendApiPath('/api/admin/users/operators/${Uri.encodeComponent(String(userId))}'),
      params: params,
      headers: headers,
    );
  }

  /// resetOperatorUserPassword
  Future<dynamic> resetOperatorUserPassword(
    Object userId,
    {
      dynamic body,
      Map<String, dynamic>? params,
      Map<String, String>? headers,
      String? contentType,
    }
  ) {
    return _client.post(
      backendApiPath('/api/admin/users/operators/${Uri.encodeComponent(String(userId))}/password'),
      body: body,
      params: params,
      headers: headers,
      contentType: contentType,
    );
  }

  /// updateOperatorUserStatus
  Future<dynamic> updateOperatorUserStatus(
    Object userId,
    {
      dynamic body,
      Map<String, dynamic>? params,
      Map<String, String>? headers,
      String? contentType,
    }
  ) {
    return _client.post(
      backendApiPath('/api/admin/users/operators/${Uri.encodeComponent(String(userId))}/status'),
      body: body,
      params: params,
      headers: headers,
      contentType: contentType,
    );
  }

  /// listPortalUsers
  Future<dynamic> listPortalUsers(
    {
      Map<String, dynamic>? params,
      Map<String, String>? headers,
    }
  ) {
    return _client.get(
      backendApiPath('/api/admin/users/portal'),
      params: params,
      headers: headers,
    );
  }

  /// savePortalUser
  Future<dynamic> savePortalUser(
    {
      dynamic body,
      Map<String, dynamic>? params,
      Map<String, String>? headers,
      String? contentType,
    }
  ) {
    return _client.post(
      backendApiPath('/api/admin/users/portal'),
      body: body,
      params: params,
      headers: headers,
      contentType: contentType,
    );
  }

  /// deletePortalUser
  Future<dynamic> deletePortalUser(
    Object userId,
    {
      Map<String, dynamic>? params,
      Map<String, String>? headers,
    }
  ) {
    return _client.delete(
      backendApiPath('/api/admin/users/portal/${Uri.encodeComponent(String(userId))}'),
      params: params,
      headers: headers,
    );
  }

  /// resetPortalUserPassword
  Future<dynamic> resetPortalUserPassword(
    Object userId,
    {
      dynamic body,
      Map<String, dynamic>? params,
      Map<String, String>? headers,
      String? contentType,
    }
  ) {
    return _client.post(
      backendApiPath('/api/admin/users/portal/${Uri.encodeComponent(String(userId))}/password'),
      body: body,
      params: params,
      headers: headers,
      contentType: contentType,
    );
  }

  /// updatePortalUserStatus
  Future<dynamic> updatePortalUserStatus(
    Object userId,
    {
      dynamic body,
      Map<String, dynamic>? params,
      Map<String, String>? headers,
      String? contentType,
    }
  ) {
    return _client.post(
      backendApiPath('/api/admin/users/portal/${Uri.encodeComponent(String(userId))}/status'),
      body: body,
      params: params,
      headers: headers,
      contentType: contentType,
    );
  }
}
