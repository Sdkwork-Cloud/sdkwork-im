import 'package:backend_sdk/backend_sdk.dart';

import 'context.dart';

class CrawChatAuthModule {
  final CrawChatSdkContext context;

  CrawChatAuthModule(this.context);

  Future<PortalLoginResponse?> login(PortalLoginRequest body) async {
    final session = await context.backendClient.auth.login(body);
    final accessToken = session?.accessToken;
    if (accessToken != null && accessToken.isNotEmpty) {
      useToken(accessToken);
    }
    return session;
  }

  Future<PortalMeResponse?> me() {
    return context.backendClient.auth.me();
  }

  void useToken(String token) {
    context.setAuthToken(token);
  }

  void clearToken() {
    context.clearAuthToken();
  }
}
