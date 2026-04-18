import 'package:craw_chat_admin_backend_sdk/craw_chat_admin_backend_sdk.dart';

typedef CrawChatAdminQueryParams = QueryParams;

class CrawChatAdminSdkClientOptions {
  final CrawChatAdminBackendClient backendClient;

  const CrawChatAdminSdkClientOptions({
    required this.backendClient,
  });
}
