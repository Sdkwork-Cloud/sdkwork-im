import 'paths.dart';
import '../http/client.dart';

class AuthApi {
  final HttpClient _client;

  AuthApi(this._client);

  /// loginAdminUser
  Future<dynamic> loginAdminUser(
    {
      dynamic body,
      Map<String, dynamic>? params,
      Map<String, String>? headers,
      String? contentType,
    }
  ) {
    return _client.post(
      backendApiPath('/api/admin/auth/login'),
      body: body,
      params: params,
      headers: headers,
      contentType: contentType,
    );
  }

  /// getAdminMe
  Future<dynamic> getAdminMe(
    {
      Map<String, dynamic>? params,
      Map<String, String>? headers,
    }
  ) {
    return _client.get(
      backendApiPath('/api/admin/auth/me'),
      params: params,
      headers: headers,
    );
  }
}
