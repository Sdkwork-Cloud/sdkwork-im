import '../http/client.dart';
import '../models.dart';
import 'paths.dart';
import 'response_helpers.dart';

class AuthApi {
  final HttpClient _client;

  AuthApi(this._client);

  /// Sign in to the tenant portal
  Future<PortalLoginResponse?> login(PortalLoginRequest body) async {
    final payload = body.toJson();
    final response = await _client.post(ApiPaths.backendPath('/auth/login'), body: payload, contentType: 'application/json');
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : PortalLoginResponse.fromJson(map);
    })();
  }

  /// Read the current portal session
  Future<PortalMeResponse?> me() async {
    final response = await _client.get(ApiPaths.backendPath('/auth/me'));
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : PortalMeResponse.fromJson(map);
    })();
  }
}
