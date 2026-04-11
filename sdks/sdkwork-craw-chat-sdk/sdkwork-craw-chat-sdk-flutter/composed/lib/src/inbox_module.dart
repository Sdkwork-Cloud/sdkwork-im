import 'package:backend_sdk/backend_sdk.dart';

import 'context.dart';

class CrawChatInboxModule {
  final CrawChatSdkContext context;

  CrawChatInboxModule(this.context);

  Future<InboxResponse?> list() {
    return context.backendClient.inbox.getInbox();
  }
}
