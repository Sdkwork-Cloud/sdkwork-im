import type {
  AckResponse,
  AddConversationMemberRequest,
  BindDirectChatRequest,
  ContactPreferencesView,
  ContactRecommendationView,
  ContactTagsResponse,
  ContactTagView,
  ContactsResponse,
  CreateAgentDialogRequest,
  CreateContactRecommendationRequest,
  CreateContactTagRequest,
  CreateConversationRequest,
  CreateConversationResult,
  DeleteContactTagResponse,
  DeleteMessageFavoriteResponse,
  DeviceSyncFeedResponse,
  FavoriteMessageRequest,
  FavoriteMessagesResponse,
  InboxResponse,
  ListMembersResponse,
  MessageFavoriteView,
  MessageFavoriteType,
  MessageInteractionSummaryView,
  MessagePinMutationResult,
  MessageReactionMutationResult,
  MessageReactionRequest,
  MessageVisibilityMutationResult,
  PinnedMessagesResponse,
  PostMessageRequest,
  PostedMessageResponse,
  QueryParams,
  ReadCursorView,
  RegisterDeviceRequest,
  RegisteredDeviceView,
  RtcSession,
  SocialFriendRequestAcceptanceResponse,
  SocialFriendRequestListResponse,
  SocialFriendRequestMutationResponse,
  SocialFriendshipMutationResponse,
  SocialUserSearchResponse,
  TimelineResponse,
  UpdateContactPreferencesRequest,
  UpdateContactTagRequest,
  UpdateConversationPreferencesRequest,
  UpdateConversationProfileRequest,
  ConversationProfileView,
} from '@sdkwork/im-sdk-generated';

export type { QueryParams };

export interface ImTransportClientLike {
  chat: {
    contacts: {
      list(params?: QueryParams): Promise<ContactsResponse>;
    };
    inbox: {
      retrieve(params?: QueryParams): Promise<InboxResponse>;
    };
    conversations: {
      create(body: CreateConversationRequest): Promise<CreateConversationResult>;
      agentDialogs: {
        create(body: CreateAgentDialogRequest): Promise<CreateConversationResult>;
      };
      directChats: {
        bind(body: BindDirectChatRequest): Promise<CreateConversationResult>;
      };
      members: {
        list(conversationId: string | number, params?: QueryParams): Promise<ListMembersResponse>;
        add(conversationId: string | number, body: AddConversationMemberRequest): Promise<unknown>;
        remove(conversationId: string | number, body: unknown): Promise<unknown>;
        leave(conversationId: string | number): Promise<unknown>;
      };
      messages: {
        list(conversationId: string | number, params?: QueryParams): Promise<TimelineResponse>;
        create(conversationId: string | number, body: PostMessageRequest): Promise<PostedMessageResponse>;
        interactionSummary: {
          retrieve(conversationId: string | number, messageId: string | number): Promise<MessageInteractionSummaryView>;
        };
      };
      pins: {
        list(conversationId: string | number): Promise<PinnedMessagesResponse>;
      };
      preferences: {
        retrieve(conversationId: string | number): Promise<import('@sdkwork/im-sdk-generated').ConversationPreferencesView>;
        update(conversationId: string | number, body: UpdateConversationPreferencesRequest): Promise<import('@sdkwork/im-sdk-generated').ConversationPreferencesView>;
      };
      profile: {
        retrieve(conversationId: string | number): Promise<ConversationProfileView>;
        update(conversationId: string | number, body: UpdateConversationProfileRequest): Promise<ConversationProfileView>;
      };
      readCursor: {
        update(conversationId: string | number, body: { readSeq: number }): Promise<ReadCursorView>;
      };
    };
    messages: {
      edit(messageId: string | number, body: unknown): Promise<PostedMessageResponse>;
      reactions: {
        create(messageId: string | number, body: MessageReactionRequest): Promise<MessageReactionMutationResult>;
        delete(messageId: string | number, body: MessageReactionRequest): Promise<MessageReactionMutationResult>;
      };
      pin: {
        create(messageId: string | number): Promise<MessagePinMutationResult>;
        delete(messageId: string | number): Promise<MessagePinMutationResult>;
      };
      visibility: {
        delete(messageId: string | number): Promise<MessageVisibilityMutationResult>;
      };
      favorites: {
        list(params?: QueryParams & { favoriteType?: MessageFavoriteType }): Promise<FavoriteMessagesResponse>;
        create(messageId: string | number, body: FavoriteMessageRequest): Promise<MessageFavoriteView>;
        delete(favoriteId: string | number): Promise<DeleteMessageFavoriteResponse>;
      };
    };
  };
  device: {
    registrations: {
      create(body: RegisterDeviceRequest): Promise<RegisteredDeviceView>;
    };
    syncFeed: {
      retrieve(deviceId: string | number, params?: QueryParams & { afterSeq?: number; limit?: number }): Promise<DeviceSyncFeedResponse>;
    };
  };
  rtc: {
    sessions: {
      retrieve(rtcSessionId: string | number): Promise<RtcSession>;
    };
  };
  social: {
    users: {
      list(params?: { q?: string; limit?: number; cursor?: string; }): Promise<SocialUserSearchResponse>;
    };
    friendRequests: {
      list(params?: QueryParams & { direction?: string; status?: string }): Promise<SocialFriendRequestListResponse>;
      create(body: { targetUserId: string; requestMessage?: string }): Promise<SocialFriendRequestMutationResponse>;
      accept(requestId: string | number): Promise<SocialFriendRequestAcceptanceResponse>;
      decline(requestId: string | number): Promise<SocialFriendRequestMutationResponse>;
      cancel(requestId: string | number): Promise<SocialFriendRequestMutationResponse>;
    };
    friendships: {
      remove(friendshipId: string | number): Promise<SocialFriendshipMutationResponse>;
    };
    contacts: {
      preferences: {
        retrieve(targetUserId: string | number): Promise<ContactPreferencesView>;
        update(targetUserId: string | number, body: UpdateContactPreferencesRequest): Promise<ContactPreferencesView>;
      };
      tags: {
        list(params?: QueryParams): Promise<ContactTagsResponse>;
        create(body: CreateContactTagRequest): Promise<ContactTagView>;
        update(
          tagId: string | number,
          body: UpdateContactTagRequest,
        ): Promise<ContactTagView>;
        delete(tagId: string | number): Promise<DeleteContactTagResponse>;
      };
      recommendations: {
        create(
          targetUserId: string | number,
          body: CreateContactRecommendationRequest,
        ): Promise<ContactRecommendationView>;
      };
    };
  };
  setAuthToken?(token: string): unknown;
  setAccessToken?(token: string): unknown;
  setTokenManager?(manager: unknown): unknown;
}

export type TransportAckResponse = AckResponse;
