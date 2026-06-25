import 'package:im_sdk_generated/im_client.dart';

import 'im_realtime.dart';

class ImSdkComposedClient {
  ImSdkComposedClient({
    required this.transport,
    required this.websocketBaseUrl,
    this.accessToken,
    this.authToken,
    this.headers = const {},
  });

  final SdkworkImClient transport;
  final String websocketBaseUrl;
  String? accessToken;
  String? authToken;
  final Map<String, String> headers;

  ImLiveConnection connect({ImConnectOptions options = const ImConnectOptions()}) {
    return createImLiveConnection(
      ImCreateLiveConnectionParams(
        websocketBaseUrl: websocketBaseUrl,
        accessToken: accessToken,
        authToken: authToken,
        headers: headers,
        options: options,
      ),
    );
  }
}
