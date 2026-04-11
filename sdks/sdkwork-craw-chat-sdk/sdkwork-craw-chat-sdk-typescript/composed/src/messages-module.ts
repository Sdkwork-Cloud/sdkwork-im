import { buildTextEditRequest } from './builders.js';
import type {
  EditMessageRequest,
  EditTextMessageOptions,
  MessageMutationResult,
} from './types.js';
import type { CrawChatSdkContext } from './sdk-context.js';

export class CrawChatMessagesModule {
  constructor(private readonly context: CrawChatSdkContext) {}

  edit(
    messageId: string | number,
    body: EditMessageRequest,
  ): Promise<MessageMutationResult> {
    return this.context.backendClient.message.edit(messageId, body);
  }

  editText(
    messageId: string | number,
    text: string,
    options: EditTextMessageOptions = {},
  ): Promise<MessageMutationResult> {
    return this.edit(messageId, buildTextEditRequest(text, options));
  }

  recall(messageId: string | number): Promise<MessageMutationResult> {
    return this.context.backendClient.message.recall(messageId);
  }
}
