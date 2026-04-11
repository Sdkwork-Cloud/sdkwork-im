import type { EditMessageRequest, EditTextMessageOptions, MessageMutationResult } from './types.js';
import type { CrawChatSdkContext } from './sdk-context.js';
export declare class CrawChatMessagesModule {
    private readonly context;
    constructor(context: CrawChatSdkContext);
    edit(messageId: string | number, body: EditMessageRequest): Promise<MessageMutationResult>;
    editText(messageId: string | number, text: string, options?: EditTextMessageOptions): Promise<MessageMutationResult>;
    recall(messageId: string | number): Promise<MessageMutationResult>;
}
//# sourceMappingURL=messages-module.d.ts.map