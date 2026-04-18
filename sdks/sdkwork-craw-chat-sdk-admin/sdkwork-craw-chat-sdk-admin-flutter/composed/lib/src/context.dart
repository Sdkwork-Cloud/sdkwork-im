import 'package:craw_chat_admin_backend_sdk/craw_chat_admin_backend_sdk.dart';

class CrawChatAdminSdkContext {
  final CrawChatAdminBackendClient backendClient;

  CrawChatAdminSdkContext(this.backendClient);

  void setAuthToken(String token) {
    backendClient.setAuthToken(token);
  }
}
