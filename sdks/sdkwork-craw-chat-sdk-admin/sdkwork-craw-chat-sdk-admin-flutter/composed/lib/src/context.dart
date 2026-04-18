import 'package:craw_chat_admin_backend_sdk/craw_chat_admin_backend_sdk.dart';

class CrawChatAdminClientContext {
  final SdkworkBackendClient backendClient;

  CrawChatAdminClientContext(this.backendClient);

  void setApiKey(String apiKey) {
    backendClient.setApiKey(apiKey);
  }

  void setAuthToken(String token) {
    backendClient.setAuthToken(token);
  }

  void setAccessToken(String token) {
    backendClient.setAccessToken(token);
  }
}
