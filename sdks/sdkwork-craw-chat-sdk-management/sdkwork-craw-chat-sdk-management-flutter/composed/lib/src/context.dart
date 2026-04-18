import 'package:craw_chat_management_backend_sdk/craw_chat_management_backend_sdk.dart';

class CrawChatManagementClientContext {
  final SdkworkBackendClient backendClient;

  CrawChatManagementClientContext(this.backendClient);

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
