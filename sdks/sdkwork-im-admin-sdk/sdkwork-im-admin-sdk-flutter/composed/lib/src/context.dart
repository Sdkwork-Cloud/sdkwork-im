import 'package:im_admin_backend_sdk/im_admin_backend_sdk.dart';

class ImAdminSdkClientContext {
  final SdkworkBackendClient backendClient;

  ImAdminSdkClientContext(this.backendClient);

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
