import {
  buildAgentClientMessage,
  buildAgentHandoffClientMessage,
  buildAgentStateClientMessage,
  buildAiImageGenerationClientMessage,
  buildAiTextClientMessage,
  buildAiVideoGenerationClientMessage,
  buildCardClientMessage,
  buildContactClientMessage,
  buildCustomClientMessage,
  buildDataClientMessage,
  buildLinkClientMessage,
  buildLocationClientMessage,
  buildMediaClientMessage,
  buildMusicClientMessage,
  buildSignalClientMessage,
  buildStickerClientMessage,
  buildStreamReferenceClientMessage,
  buildTextClientMessage,
  buildToolResultClientMessage,
  buildVoiceClientMessage,
  buildWorkflowEventClientMessage,
  buildTextEditRequest,
} from './builders.js';
import { decodeMessageBody } from './message-codec.js';
import { performPresignedMediaUpload } from './media-upload-runtime.js';
import type {
  CrawChatClientMessage,
  CrawChatCreateAgentInput,
  CrawChatCreateAgentHandoffInput,
  CrawChatCreateAgentStateInput,
  CrawChatCreateAiImageGenerationInput,
  CrawChatCreateAiTextInput,
  CrawChatCreateAiVideoGenerationInput,
  CrawChatCreateCardInput,
  CrawChatCreateContactInput,
  CrawChatCreateCustomInput,
  CrawChatCreateDataInput,
  CrawChatCreateLinkInput,
  CrawChatCreateLocationInput,
  CrawChatCreateMediaInput,
  CrawChatCreateMusicInput,
  CrawChatCreateSignalInput,
  CrawChatCreateStickerInput,
  CrawChatCreateStreamReferenceInput,
  CrawChatCreateTextInput,
  CrawChatCreateToolResultInput,
  CrawChatCreateVoiceInput,
  CrawChatCreateWorkflowEventInput,
  CrawChatDecodableMessageBody,
  CrawChatDecodedMessage,
  CrawChatMessageKind,
  CrawChatUploadAndSendMessageOptions,
  CrawChatUploadAndSendMessageResult,
  EditMessageRequest,
  EditTextMessageOptions,
  MessageMutationResult,
  PostMessageResult,
} from './types.js';
import type { CrawChatSdkContext } from './sdk-context.js';

export class CrawChatMessagesModule {
  constructor(private readonly context: CrawChatSdkContext) {}

  createText(
    input: CrawChatCreateTextInput,
  ): CrawChatClientMessage<'text'> {
    return buildTextClientMessage(
      input.conversationId,
      input.text,
      withoutConversationId(input),
    );
  }

  createImage(
    input: CrawChatCreateMediaInput,
  ): CrawChatClientMessage<'image'> {
    return buildMediaClientMessage('image', input.conversationId, withoutConversationId(input));
  }

  createVideo(
    input: CrawChatCreateMediaInput,
  ): CrawChatClientMessage<'video'> {
    return buildMediaClientMessage('video', input.conversationId, withoutConversationId(input));
  }

  createAudio(
    input: CrawChatCreateMediaInput,
  ): CrawChatClientMessage<'audio'> {
    return buildMediaClientMessage('audio', input.conversationId, withoutConversationId(input));
  }

  createFile(
    input: CrawChatCreateMediaInput,
  ): CrawChatClientMessage<'file'> {
    return buildMediaClientMessage('file', input.conversationId, withoutConversationId(input));
  }

  createData(
    input: CrawChatCreateDataInput,
  ): CrawChatClientMessage<'data'> {
    return buildDataClientMessage(input.conversationId, withoutConversationId(input));
  }

  createSignal(
    input: CrawChatCreateSignalInput,
  ): CrawChatClientMessage<'signal'> {
    return buildSignalClientMessage(input.conversationId, withoutConversationId(input));
  }

  createStreamReference(
    input: CrawChatCreateStreamReferenceInput,
  ): CrawChatClientMessage<'stream_ref'> {
    return buildStreamReferenceClientMessage(
      input.conversationId,
      withoutConversationId(input),
    );
  }

  createLocation(
    input: CrawChatCreateLocationInput,
  ): CrawChatClientMessage<'location'> {
    return buildLocationClientMessage(input.conversationId, withoutConversationId(input));
  }

  createLink(
    input: CrawChatCreateLinkInput,
  ): CrawChatClientMessage<'link'> {
    return buildLinkClientMessage(input.conversationId, withoutConversationId(input));
  }

  createCard(
    input: CrawChatCreateCardInput,
  ): CrawChatClientMessage<'card'> {
    return buildCardClientMessage(input.conversationId, withoutConversationId(input));
  }

  createMusic(
    input: CrawChatCreateMusicInput,
  ): CrawChatClientMessage<'music'> {
    return buildMusicClientMessage(input.conversationId, withoutConversationId(input));
  }

  createContact(
    input: CrawChatCreateContactInput,
  ): CrawChatClientMessage<'contact'> {
    return buildContactClientMessage(input.conversationId, withoutConversationId(input));
  }

  createSticker(
    input: CrawChatCreateStickerInput,
  ): CrawChatClientMessage<'sticker'> {
    return buildStickerClientMessage(input.conversationId, withoutConversationId(input));
  }

  createVoice(
    input: CrawChatCreateVoiceInput,
  ): CrawChatClientMessage<'voice'> {
    return buildVoiceClientMessage(input.conversationId, withoutConversationId(input));
  }

  createAgent(
    input: CrawChatCreateAgentInput,
  ): CrawChatClientMessage<'agent'> {
    return buildAgentClientMessage(input.conversationId, withoutConversationId(input));
  }

  createAgentState(
    input: CrawChatCreateAgentStateInput,
  ): CrawChatClientMessage<'agent_state'> {
    return buildAgentStateClientMessage(input.conversationId, withoutConversationId(input));
  }

  createAgentHandoff(
    input: CrawChatCreateAgentHandoffInput,
  ): CrawChatClientMessage<'agent_handoff'> {
    return buildAgentHandoffClientMessage(input.conversationId, withoutConversationId(input));
  }

  createCustom(
    input: CrawChatCreateCustomInput,
  ): CrawChatClientMessage<'custom'> {
    return buildCustomClientMessage(input.conversationId, withoutConversationId(input));
  }

  createAiText(
    input: CrawChatCreateAiTextInput,
  ): CrawChatClientMessage<'ai_text'> {
    return buildAiTextClientMessage(input.conversationId, withoutConversationId(input));
  }

  createAiImageGeneration(
    input: CrawChatCreateAiImageGenerationInput,
  ): CrawChatClientMessage<'ai_image_generation'> {
    return buildAiImageGenerationClientMessage(
      input.conversationId,
      withoutConversationId(input),
    );
  }

  createAiVideoGeneration(
    input: CrawChatCreateAiVideoGenerationInput,
  ): CrawChatClientMessage<'ai_video_generation'> {
    return buildAiVideoGenerationClientMessage(
      input.conversationId,
      withoutConversationId(input),
    );
  }

  createToolResult(
    input: CrawChatCreateToolResultInput,
  ): CrawChatClientMessage<'tool_result'> {
    return buildToolResultClientMessage(input.conversationId, withoutConversationId(input));
  }

  createWorkflowEvent(
    input: CrawChatCreateWorkflowEventInput,
  ): CrawChatClientMessage<'workflow_event'> {
    return buildWorkflowEventClientMessage(input.conversationId, withoutConversationId(input));
  }

  decode(body: CrawChatDecodableMessageBody): CrawChatDecodedMessage {
    return decodeMessageBody(body);
  }

  send<TKind extends CrawChatMessageKind>(
    message: CrawChatClientMessage<TKind>,
  ): Promise<PostMessageResult> {
    if (message.target.channel === 'system') {
      return this.context.backendClient.conversation.publishSystemChannelMessage(
        message.target.conversationId,
        message.body,
      );
    }

    return this.context.backendClient.conversation.postConversationMessage(
      message.target.conversationId,
      message.body,
    );
  }

  async uploadAndSend<TKind extends CrawChatMessageKind>(
    options: CrawChatUploadAndSendMessageOptions<TKind>,
  ): Promise<CrawChatUploadAndSendMessageResult<TKind>> {
    const uploaded = await performPresignedMediaUpload(this.context, options.upload);
    const message = options.createMessage(uploaded);
    const delivery = await this.send(message);

    return {
      ...uploaded,
      message,
      delivery,
    };
  }

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

function withoutConversationId<T extends { conversationId: string | number }>(
  input: T,
): Omit<T, 'conversationId'> {
  const { conversationId: _conversationId, ...rest } = input;
  return rest;
}
