import 'package:backend_sdk/backend_sdk.dart';

import 'builders.dart';
import 'context.dart';
import 'types.dart';

class CrawChatMessagesModule {
  final CrawChatSdkContext context;

  CrawChatMessagesModule(this.context);

  Future<MessageMutationResult?> edit(
    String messageId,
    EditMessageRequest body,
  ) {
    return context.backendClient.message.edit(messageId, body);
  }

  Future<MessageMutationResult?> editText(
    String messageId, {
    required String text,
    CrawChatTextEditOptions options = const CrawChatTextEditOptions(),
  }) {
    return edit(
      messageId,
      CrawChatBuilders.textEdit(text: text, options: options),
    );
  }

  Future<MessageMutationResult?> recall(String messageId) {
    return context.backendClient.message.recall(messageId);
  }
}
