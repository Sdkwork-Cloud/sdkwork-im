import 'package:backend_sdk/backend_sdk.dart';

class CrawChatSdkContext {
  final SdkworkBackendClient backendClient;

  CrawChatSdkContext(this.backendClient);

  void setAuthToken(String token) {
    backendClient.setAuthToken(token);
  }

  void clearAuthToken() {
    backendClient.setAuthToken('');
  }
}
