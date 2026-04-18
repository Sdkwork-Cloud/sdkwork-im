Map<String, dynamic>? _sdkworkAsMap(dynamic value) {
  if (value is Map<String, dynamic>) {
    return value;
  }
  if (value is Map) {
    return value.map((key, item) => MapEntry(key.toString(), item));
  }
  return null;
}

List<dynamic>? _sdkworkAsList(dynamic value) {
  return value is List ? value : null;
}

class ApiError {
  final String? code;
  final String? message;

  ApiError({
    this.code,
    this.message
  });

  factory ApiError.fromJson(Map<String, dynamic> json) {
    return ApiError(
      code: json['code']?.toString(),
      message: json['message']?.toString()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'code': code,
      'message': message,
    };
  }
}

class PortalLoginRequest {
  final String? tenantId;
  final String? login;
  final String? password;
  final String? deviceId;
  final String? sessionId;
  final String? clientKind;

  PortalLoginRequest({
    this.tenantId,
    this.login,
    this.password,
    this.deviceId,
    this.sessionId,
    this.clientKind
  });

  factory PortalLoginRequest.fromJson(Map<String, dynamic> json) {
    return PortalLoginRequest(
      tenantId: json['tenantId']?.toString(),
      login: json['login']?.toString(),
      password: json['password']?.toString(),
      deviceId: json['deviceId']?.toString(),
      sessionId: json['sessionId']?.toString(),
      clientKind: json['clientKind']?.toString()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'tenantId': tenantId,
      'login': login,
      'password': password,
      'deviceId': deviceId,
      'sessionId': sessionId,
      'clientKind': clientKind,
    };
  }
}

class PortalUserView {
  final String? id;
  final String? login;
  final String? name;
  final String? role;
  final String? email;
  final String? actorKind;
  final String? clientKind;
  final List<String>? permissions;

  PortalUserView({
    this.id,
    this.login,
    this.name,
    this.role,
    this.email,
    this.actorKind,
    this.clientKind,
    this.permissions
  });

  factory PortalUserView.fromJson(Map<String, dynamic> json) {
    return PortalUserView(
      id: json['id']?.toString(),
      login: json['login']?.toString(),
      name: json['name']?.toString(),
      role: json['role']?.toString(),
      email: json['email']?.toString(),
      actorKind: json['actorKind']?.toString(),
      clientKind: json['clientKind']?.toString(),
      permissions: (() {
        final list = _sdkworkAsList(json['permissions']);
        if (list == null) {
          return null;
        }
        return list
            .map((item) => item?.toString())
            .whereType<String>()
            .toList();
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'id': id,
      'login': login,
      'name': name,
      'role': role,
      'email': email,
      'actorKind': actorKind,
      'clientKind': clientKind,
      'permissions': permissions?.map((item) => item).toList(),
    };
  }
}

class PortalWorkspaceView {
  final String? name;
  final String? slug;
  final String? tier;
  final String? region;
  final String? supportPlan;
  final int? seats;
  final int? activeBrands;
  final String? uptime;

  PortalWorkspaceView({
    this.name,
    this.slug,
    this.tier,
    this.region,
    this.supportPlan,
    this.seats,
    this.activeBrands,
    this.uptime
  });

  factory PortalWorkspaceView.fromJson(Map<String, dynamic> json) {
    return PortalWorkspaceView(
      name: json['name']?.toString(),
      slug: json['slug']?.toString(),
      tier: json['tier']?.toString(),
      region: json['region']?.toString(),
      supportPlan: json['supportPlan']?.toString(),
      seats: json['seats'] is int ? json['seats'] : null,
      activeBrands: json['activeBrands'] is int ? json['activeBrands'] : null,
      uptime: json['uptime']?.toString()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'name': name,
      'slug': slug,
      'tier': tier,
      'region': region,
      'supportPlan': supportPlan,
      'seats': seats,
      'activeBrands': activeBrands,
      'uptime': uptime,
    };
  }
}

class PortalLoginResponse {
  final String? accessToken;
  final String? refreshToken;
  final int? expiresAt;
  final PortalUserView? user;
  final PortalWorkspaceView? workspace;

  PortalLoginResponse({
    this.accessToken,
    this.refreshToken,
    this.expiresAt,
    this.user,
    this.workspace
  });

  factory PortalLoginResponse.fromJson(Map<String, dynamic> json) {
    return PortalLoginResponse(
      accessToken: json['accessToken']?.toString(),
      refreshToken: json['refreshToken']?.toString(),
      expiresAt: json['expiresAt'] is int ? json['expiresAt'] : null,
      user: (() {
        final map = _sdkworkAsMap(json['user']);
        return map == null ? null : PortalUserView.fromJson(map);
      })(),
      workspace: (() {
        final map = _sdkworkAsMap(json['workspace']);
        return map == null ? null : PortalWorkspaceView.fromJson(map);
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'accessToken': accessToken,
      'refreshToken': refreshToken,
      'expiresAt': expiresAt,
      'user': user?.toJson(),
      'workspace': workspace?.toJson(),
    };
  }
}

class PortalMeResponse {
  final String? tenantId;
  final PortalUserView? user;
  final PortalWorkspaceView? workspace;

  PortalMeResponse({
    this.tenantId,
    this.user,
    this.workspace
  });

  factory PortalMeResponse.fromJson(Map<String, dynamic> json) {
    return PortalMeResponse(
      tenantId: json['tenantId']?.toString(),
      user: (() {
        final map = _sdkworkAsMap(json['user']);
        return map == null ? null : PortalUserView.fromJson(map);
      })(),
      workspace: (() {
        final map = _sdkworkAsMap(json['workspace']);
        return map == null ? null : PortalWorkspaceView.fromJson(map);
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'tenantId': tenantId,
      'user': user?.toJson(),
      'workspace': workspace?.toJson(),
    };
  }
}

class ResumeSessionRequest {
  final String? deviceId;
  final int? lastSeenSyncSeq;

  ResumeSessionRequest({
    this.deviceId,
    this.lastSeenSyncSeq
  });

  factory ResumeSessionRequest.fromJson(Map<String, dynamic> json) {
    return ResumeSessionRequest(
      deviceId: json['deviceId']?.toString(),
      lastSeenSyncSeq: json['lastSeenSyncSeq'] is int ? json['lastSeenSyncSeq'] : null
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'deviceId': deviceId,
      'lastSeenSyncSeq': lastSeenSyncSeq,
    };
  }
}

class PresenceDeviceRequest {
  final String? deviceId;

  PresenceDeviceRequest({
    this.deviceId
  });

  factory PresenceDeviceRequest.fromJson(Map<String, dynamic> json) {
    return PresenceDeviceRequest(
      deviceId: json['deviceId']?.toString()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'deviceId': deviceId,
    };
  }
}

class RegisterDeviceRequest {
  final String? deviceId;

  RegisterDeviceRequest({
    this.deviceId
  });

  factory RegisterDeviceRequest.fromJson(Map<String, dynamic> json) {
    return RegisterDeviceRequest(
      deviceId: json['deviceId']?.toString()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'deviceId': deviceId,
    };
  }
}

class RealtimeSubscriptionItemInput {
  final String? scopeType;
  final String? scopeId;
  final List<String>? eventTypes;

  RealtimeSubscriptionItemInput({
    this.scopeType,
    this.scopeId,
    this.eventTypes
  });

  factory RealtimeSubscriptionItemInput.fromJson(Map<String, dynamic> json) {
    return RealtimeSubscriptionItemInput(
      scopeType: json['scopeType']?.toString(),
      scopeId: json['scopeId']?.toString(),
      eventTypes: (() {
        final list = _sdkworkAsList(json['eventTypes']);
        if (list == null) {
          return null;
        }
        return list
            .map((item) => item?.toString())
            .whereType<String>()
            .toList();
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'scopeType': scopeType,
      'scopeId': scopeId,
      'eventTypes': eventTypes?.map((item) => item).toList(),
    };
  }
}

class SyncRealtimeSubscriptionsRequest {
  final String? deviceId;
  final List<RealtimeSubscriptionItemInput>? items;

  SyncRealtimeSubscriptionsRequest({
    this.deviceId,
    this.items
  });

  factory SyncRealtimeSubscriptionsRequest.fromJson(Map<String, dynamic> json) {
    return SyncRealtimeSubscriptionsRequest(
      deviceId: json['deviceId']?.toString(),
      items: (() {
        final list = _sdkworkAsList(json['items']);
        if (list == null) {
          return null;
        }
        return list
            .map((item) => (() {
        final map = _sdkworkAsMap(item);
        return map == null ? null : RealtimeSubscriptionItemInput.fromJson(map);
      })())
            .whereType<RealtimeSubscriptionItemInput>()
            .toList();
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'deviceId': deviceId,
      'items': items?.map((item) => item.toJson()).toList(),
    };
  }
}

class AckRealtimeEventsRequest {
  final String? deviceId;
  final int? ackedSeq;

  AckRealtimeEventsRequest({
    this.deviceId,
    this.ackedSeq
  });

  factory AckRealtimeEventsRequest.fromJson(Map<String, dynamic> json) {
    return AckRealtimeEventsRequest(
      deviceId: json['deviceId']?.toString(),
      ackedSeq: json['ackedSeq'] is int ? json['ackedSeq'] : null
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'deviceId': deviceId,
      'ackedSeq': ackedSeq,
    };
  }
}

class CreateConversationRequest {
  final String? conversationId;
  final String? conversationType;

  CreateConversationRequest({
    this.conversationId,
    this.conversationType
  });

  factory CreateConversationRequest.fromJson(Map<String, dynamic> json) {
    return CreateConversationRequest(
      conversationId: json['conversationId']?.toString(),
      conversationType: json['conversationType']?.toString()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'conversationId': conversationId,
      'conversationType': conversationType,
    };
  }
}

class CreateAgentDialogRequest {
  final String? conversationId;
  final String? agentId;

  CreateAgentDialogRequest({
    this.conversationId,
    this.agentId
  });

  factory CreateAgentDialogRequest.fromJson(Map<String, dynamic> json) {
    return CreateAgentDialogRequest(
      conversationId: json['conversationId']?.toString(),
      agentId: json['agentId']?.toString()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'conversationId': conversationId,
      'agentId': agentId,
    };
  }
}

class CreateAgentHandoffRequest {
  final String? conversationId;
  final String? targetId;
  final String? targetKind;
  final String? handoffSessionId;
  final String? handoffReason;

  CreateAgentHandoffRequest({
    this.conversationId,
    this.targetId,
    this.targetKind,
    this.handoffSessionId,
    this.handoffReason
  });

  factory CreateAgentHandoffRequest.fromJson(Map<String, dynamic> json) {
    return CreateAgentHandoffRequest(
      conversationId: json['conversationId']?.toString(),
      targetId: json['targetId']?.toString(),
      targetKind: json['targetKind']?.toString(),
      handoffSessionId: json['handoffSessionId']?.toString(),
      handoffReason: json['handoffReason']?.toString()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'conversationId': conversationId,
      'targetId': targetId,
      'targetKind': targetKind,
      'handoffSessionId': handoffSessionId,
      'handoffReason': handoffReason,
    };
  }
}

class CreateSystemChannelRequest {
  final String? conversationId;
  final String? subscriberId;

  CreateSystemChannelRequest({
    this.conversationId,
    this.subscriberId
  });

  factory CreateSystemChannelRequest.fromJson(Map<String, dynamic> json) {
    return CreateSystemChannelRequest(
      conversationId: json['conversationId']?.toString(),
      subscriberId: json['subscriberId']?.toString()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'conversationId': conversationId,
      'subscriberId': subscriberId,
    };
  }
}

class AddConversationMemberRequest {
  final String? principalId;
  final String? principalKind;
  final String? role;

  AddConversationMemberRequest({
    this.principalId,
    this.principalKind,
    this.role
  });

  factory AddConversationMemberRequest.fromJson(Map<String, dynamic> json) {
    return AddConversationMemberRequest(
      principalId: json['principalId']?.toString(),
      principalKind: json['principalKind']?.toString(),
      role: json['role']?.toString()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'principalId': principalId,
      'principalKind': principalKind,
      'role': role,
    };
  }
}

class RemoveConversationMemberRequest {
  final String? memberId;

  RemoveConversationMemberRequest({
    this.memberId
  });

  factory RemoveConversationMemberRequest.fromJson(Map<String, dynamic> json) {
    return RemoveConversationMemberRequest(
      memberId: json['memberId']?.toString()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'memberId': memberId,
    };
  }
}

class TransferConversationOwnerRequest {
  final String? memberId;

  TransferConversationOwnerRequest({
    this.memberId
  });

  factory TransferConversationOwnerRequest.fromJson(Map<String, dynamic> json) {
    return TransferConversationOwnerRequest(
      memberId: json['memberId']?.toString()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'memberId': memberId,
    };
  }
}

class ChangeConversationMemberRoleRequest {
  final String? memberId;
  final String? role;

  ChangeConversationMemberRoleRequest({
    this.memberId,
    this.role
  });

  factory ChangeConversationMemberRoleRequest.fromJson(Map<String, dynamic> json) {
    return ChangeConversationMemberRoleRequest(
      memberId: json['memberId']?.toString(),
      role: json['role']?.toString()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'memberId': memberId,
      'role': role,
    };
  }
}

class UpdateReadCursorRequest {
  final int? readSeq;
  final String? lastReadMessageId;

  UpdateReadCursorRequest({
    this.readSeq,
    this.lastReadMessageId
  });

  factory UpdateReadCursorRequest.fromJson(Map<String, dynamic> json) {
    return UpdateReadCursorRequest(
      readSeq: json['readSeq'] is int ? json['readSeq'] : null,
      lastReadMessageId: json['lastReadMessageId']?.toString()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'readSeq': readSeq,
      'lastReadMessageId': lastReadMessageId,
    };
  }
}

class PostMessageRequest {
  final String? clientMsgId;
  final String? summary;
  final String? text;
  final List<ContentPart>? parts;
  final Map<String, String>? renderHints;

  PostMessageRequest({
    this.clientMsgId,
    this.summary,
    this.text,
    this.parts,
    this.renderHints
  });

  factory PostMessageRequest.fromJson(Map<String, dynamic> json) {
    return PostMessageRequest(
      clientMsgId: json['clientMsgId']?.toString(),
      summary: json['summary']?.toString(),
      text: json['text']?.toString(),
      parts: (() {
        final list = _sdkworkAsList(json['parts']);
        if (list == null) {
          return null;
        }
        return list
            .map((item) => (() {
        final map = _sdkworkAsMap(item);
        return map == null ? null : ContentPart.fromJson(map);
      })())
            .whereType<ContentPart>()
            .toList();
      })(),
      renderHints: (() {
        final map = _sdkworkAsMap(json['renderHints']);
        if (map == null) {
          return null;
        }
        final result = <String, String>{};
        map.forEach((key, item) {
          final deserialized = item?.toString();
          if (deserialized is String) {
            result[key] = deserialized;
          }
        });
        return result;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'clientMsgId': clientMsgId,
      'summary': summary,
      'text': text,
      'parts': parts?.map((item) => item.toJson()).toList(),
      'renderHints': renderHints?.map((key, item) => MapEntry(key, item)),
    };
  }
}

class EditMessageRequest {
  final String? summary;
  final String? text;
  final List<ContentPart>? parts;
  final Map<String, String>? renderHints;

  EditMessageRequest({
    this.summary,
    this.text,
    this.parts,
    this.renderHints
  });

  factory EditMessageRequest.fromJson(Map<String, dynamic> json) {
    return EditMessageRequest(
      summary: json['summary']?.toString(),
      text: json['text']?.toString(),
      parts: (() {
        final list = _sdkworkAsList(json['parts']);
        if (list == null) {
          return null;
        }
        return list
            .map((item) => (() {
        final map = _sdkworkAsMap(item);
        return map == null ? null : ContentPart.fromJson(map);
      })())
            .whereType<ContentPart>()
            .toList();
      })(),
      renderHints: (() {
        final map = _sdkworkAsMap(json['renderHints']);
        if (map == null) {
          return null;
        }
        final result = <String, String>{};
        map.forEach((key, item) {
          final deserialized = item?.toString();
          if (deserialized is String) {
            result[key] = deserialized;
          }
        });
        return result;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'summary': summary,
      'text': text,
      'parts': parts?.map((item) => item.toJson()).toList(),
      'renderHints': renderHints?.map((key, item) => MapEntry(key, item)),
    };
  }
}

class AttachMediaRequest {
  final String? conversationId;
  final String? clientMsgId;
  final String? summary;
  final String? text;
  final Map<String, String>? renderHints;

  AttachMediaRequest({
    this.conversationId,
    this.clientMsgId,
    this.summary,
    this.text,
    this.renderHints
  });

  factory AttachMediaRequest.fromJson(Map<String, dynamic> json) {
    return AttachMediaRequest(
      conversationId: json['conversationId']?.toString(),
      clientMsgId: json['clientMsgId']?.toString(),
      summary: json['summary']?.toString(),
      text: json['text']?.toString(),
      renderHints: (() {
        final map = _sdkworkAsMap(json['renderHints']);
        if (map == null) {
          return null;
        }
        final result = <String, String>{};
        map.forEach((key, item) {
          final deserialized = item?.toString();
          if (deserialized is String) {
            result[key] = deserialized;
          }
        });
        return result;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'conversationId': conversationId,
      'clientMsgId': clientMsgId,
      'summary': summary,
      'text': text,
      'renderHints': renderHints?.map((key, item) => MapEntry(key, item)),
    };
  }
}

class DevicePresenceView {
  final String? tenantId;
  final String? principalId;
  final String? deviceId;
  final String? platform;
  final String? sessionId;
  final String? status;
  final int? lastSyncSeq;
  final String? lastResumeAt;
  final String? lastSeenAt;

  DevicePresenceView({
    this.tenantId,
    this.principalId,
    this.deviceId,
    this.platform,
    this.sessionId,
    this.status,
    this.lastSyncSeq,
    this.lastResumeAt,
    this.lastSeenAt
  });

  factory DevicePresenceView.fromJson(Map<String, dynamic> json) {
    return DevicePresenceView(
      tenantId: json['tenantId']?.toString(),
      principalId: json['principalId']?.toString(),
      deviceId: json['deviceId']?.toString(),
      platform: json['platform']?.toString(),
      sessionId: json['sessionId']?.toString(),
      status: json['status']?.toString(),
      lastSyncSeq: json['lastSyncSeq'] is int ? json['lastSyncSeq'] : null,
      lastResumeAt: json['lastResumeAt']?.toString(),
      lastSeenAt: json['lastSeenAt']?.toString()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'tenantId': tenantId,
      'principalId': principalId,
      'deviceId': deviceId,
      'platform': platform,
      'sessionId': sessionId,
      'status': status,
      'lastSyncSeq': lastSyncSeq,
      'lastResumeAt': lastResumeAt,
      'lastSeenAt': lastSeenAt,
    };
  }
}

class PresenceSnapshotView {
  final String? tenantId;
  final String? principalId;
  final String? currentDeviceId;
  final List<DevicePresenceView>? devices;

  PresenceSnapshotView({
    this.tenantId,
    this.principalId,
    this.currentDeviceId,
    this.devices
  });

  factory PresenceSnapshotView.fromJson(Map<String, dynamic> json) {
    return PresenceSnapshotView(
      tenantId: json['tenantId']?.toString(),
      principalId: json['principalId']?.toString(),
      currentDeviceId: json['currentDeviceId']?.toString(),
      devices: (() {
        final list = _sdkworkAsList(json['devices']);
        if (list == null) {
          return null;
        }
        return list
            .map((item) => (() {
        final map = _sdkworkAsMap(item);
        return map == null ? null : DevicePresenceView.fromJson(map);
      })())
            .whereType<DevicePresenceView>()
            .toList();
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'tenantId': tenantId,
      'principalId': principalId,
      'currentDeviceId': currentDeviceId,
      'devices': devices?.map((item) => item.toJson()).toList(),
    };
  }
}

class SessionResumeView {
  final String? tenantId;
  final String? actorId;
  final String? actorKind;
  final String? sessionId;
  final String? deviceId;
  final bool? resumeRequired;
  final int? resumeFromSyncSeq;
  final int? latestSyncSeq;
  final String? resumedAt;
  final PresenceSnapshotView? presence;

  SessionResumeView({
    this.tenantId,
    this.actorId,
    this.actorKind,
    this.sessionId,
    this.deviceId,
    this.resumeRequired,
    this.resumeFromSyncSeq,
    this.latestSyncSeq,
    this.resumedAt,
    this.presence
  });

  factory SessionResumeView.fromJson(Map<String, dynamic> json) {
    return SessionResumeView(
      tenantId: json['tenantId']?.toString(),
      actorId: json['actorId']?.toString(),
      actorKind: json['actorKind']?.toString(),
      sessionId: json['sessionId']?.toString(),
      deviceId: json['deviceId']?.toString(),
      resumeRequired: json['resumeRequired'] is bool ? json['resumeRequired'] : null,
      resumeFromSyncSeq: json['resumeFromSyncSeq'] is int ? json['resumeFromSyncSeq'] : null,
      latestSyncSeq: json['latestSyncSeq'] is int ? json['latestSyncSeq'] : null,
      resumedAt: json['resumedAt']?.toString(),
      presence: (() {
        final map = _sdkworkAsMap(json['presence']);
        return map == null ? null : PresenceSnapshotView.fromJson(map);
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'tenantId': tenantId,
      'actorId': actorId,
      'actorKind': actorKind,
      'sessionId': sessionId,
      'deviceId': deviceId,
      'resumeRequired': resumeRequired,
      'resumeFromSyncSeq': resumeFromSyncSeq,
      'latestSyncSeq': latestSyncSeq,
      'resumedAt': resumedAt,
      'presence': presence?.toJson(),
    };
  }
}

class RealtimeSubscription {
  final String? scopeType;
  final String? scopeId;
  final List<String>? eventTypes;
  final String? subscribedAt;

  RealtimeSubscription({
    this.scopeType,
    this.scopeId,
    this.eventTypes,
    this.subscribedAt
  });

  factory RealtimeSubscription.fromJson(Map<String, dynamic> json) {
    return RealtimeSubscription(
      scopeType: json['scopeType']?.toString(),
      scopeId: json['scopeId']?.toString(),
      eventTypes: (() {
        final list = _sdkworkAsList(json['eventTypes']);
        if (list == null) {
          return null;
        }
        return list
            .map((item) => item?.toString())
            .whereType<String>()
            .toList();
      })(),
      subscribedAt: json['subscribedAt']?.toString()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'scopeType': scopeType,
      'scopeId': scopeId,
      'eventTypes': eventTypes?.map((item) => item).toList(),
      'subscribedAt': subscribedAt,
    };
  }
}

class RealtimeSubscriptionSnapshot {
  final String? tenantId;
  final String? principalId;
  final String? deviceId;
  final List<RealtimeSubscription>? items;
  final String? syncedAt;

  RealtimeSubscriptionSnapshot({
    this.tenantId,
    this.principalId,
    this.deviceId,
    this.items,
    this.syncedAt
  });

  factory RealtimeSubscriptionSnapshot.fromJson(Map<String, dynamic> json) {
    return RealtimeSubscriptionSnapshot(
      tenantId: json['tenantId']?.toString(),
      principalId: json['principalId']?.toString(),
      deviceId: json['deviceId']?.toString(),
      items: (() {
        final list = _sdkworkAsList(json['items']);
        if (list == null) {
          return null;
        }
        return list
            .map((item) => (() {
        final map = _sdkworkAsMap(item);
        return map == null ? null : RealtimeSubscription.fromJson(map);
      })())
            .whereType<RealtimeSubscription>()
            .toList();
      })(),
      syncedAt: json['syncedAt']?.toString()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'tenantId': tenantId,
      'principalId': principalId,
      'deviceId': deviceId,
      'items': items?.map((item) => item.toJson()).toList(),
      'syncedAt': syncedAt,
    };
  }
}

class RealtimeEvent {
  final String? tenantId;
  final String? principalId;
  final String? deviceId;
  final int? realtimeSeq;
  final String? scopeType;
  final String? scopeId;
  final String? eventType;
  final String? deliveryClass;
  final String? payload;
  final String? occurredAt;

  RealtimeEvent({
    this.tenantId,
    this.principalId,
    this.deviceId,
    this.realtimeSeq,
    this.scopeType,
    this.scopeId,
    this.eventType,
    this.deliveryClass,
    this.payload,
    this.occurredAt
  });

  factory RealtimeEvent.fromJson(Map<String, dynamic> json) {
    return RealtimeEvent(
      tenantId: json['tenantId']?.toString(),
      principalId: json['principalId']?.toString(),
      deviceId: json['deviceId']?.toString(),
      realtimeSeq: json['realtimeSeq'] is int ? json['realtimeSeq'] : null,
      scopeType: json['scopeType']?.toString(),
      scopeId: json['scopeId']?.toString(),
      eventType: json['eventType']?.toString(),
      deliveryClass: json['deliveryClass']?.toString(),
      payload: json['payload']?.toString(),
      occurredAt: json['occurredAt']?.toString()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'tenantId': tenantId,
      'principalId': principalId,
      'deviceId': deviceId,
      'realtimeSeq': realtimeSeq,
      'scopeType': scopeType,
      'scopeId': scopeId,
      'eventType': eventType,
      'deliveryClass': deliveryClass,
      'payload': payload,
      'occurredAt': occurredAt,
    };
  }
}

class RealtimeEventWindow {
  final String? deviceId;
  final List<RealtimeEvent>? items;
  final int? nextAfterSeq;
  final bool? hasMore;
  final int? ackedThroughSeq;
  final int? trimmedThroughSeq;

  RealtimeEventWindow({
    this.deviceId,
    this.items,
    this.nextAfterSeq,
    this.hasMore,
    this.ackedThroughSeq,
    this.trimmedThroughSeq
  });

  factory RealtimeEventWindow.fromJson(Map<String, dynamic> json) {
    return RealtimeEventWindow(
      deviceId: json['deviceId']?.toString(),
      items: (() {
        final list = _sdkworkAsList(json['items']);
        if (list == null) {
          return null;
        }
        return list
            .map((item) => (() {
        final map = _sdkworkAsMap(item);
        return map == null ? null : RealtimeEvent.fromJson(map);
      })())
            .whereType<RealtimeEvent>()
            .toList();
      })(),
      nextAfterSeq: json['nextAfterSeq'] is int ? json['nextAfterSeq'] : null,
      hasMore: json['hasMore'] is bool ? json['hasMore'] : null,
      ackedThroughSeq: json['ackedThroughSeq'] is int ? json['ackedThroughSeq'] : null,
      trimmedThroughSeq: json['trimmedThroughSeq'] is int ? json['trimmedThroughSeq'] : null
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'deviceId': deviceId,
      'items': items?.map((item) => item.toJson()).toList(),
      'nextAfterSeq': nextAfterSeq,
      'hasMore': hasMore,
      'ackedThroughSeq': ackedThroughSeq,
      'trimmedThroughSeq': trimmedThroughSeq,
    };
  }
}

class RealtimeAckState {
  final String? tenantId;
  final String? principalId;
  final String? deviceId;
  final int? ackedThroughSeq;
  final int? trimmedThroughSeq;
  final int? retainedEventCount;
  final String? ackedAt;

  RealtimeAckState({
    this.tenantId,
    this.principalId,
    this.deviceId,
    this.ackedThroughSeq,
    this.trimmedThroughSeq,
    this.retainedEventCount,
    this.ackedAt
  });

  factory RealtimeAckState.fromJson(Map<String, dynamic> json) {
    return RealtimeAckState(
      tenantId: json['tenantId']?.toString(),
      principalId: json['principalId']?.toString(),
      deviceId: json['deviceId']?.toString(),
      ackedThroughSeq: json['ackedThroughSeq'] is int ? json['ackedThroughSeq'] : null,
      trimmedThroughSeq: json['trimmedThroughSeq'] is int ? json['trimmedThroughSeq'] : null,
      retainedEventCount: json['retainedEventCount'] is int ? json['retainedEventCount'] : null,
      ackedAt: json['ackedAt']?.toString()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'tenantId': tenantId,
      'principalId': principalId,
      'deviceId': deviceId,
      'ackedThroughSeq': ackedThroughSeq,
      'trimmedThroughSeq': trimmedThroughSeq,
      'retainedEventCount': retainedEventCount,
      'ackedAt': ackedAt,
    };
  }
}

class ConversationMember {
  final String? tenantId;
  final String? conversationId;
  final String? memberId;
  final String? principalId;
  final String? principalKind;
  final String? role;
  final String? state;
  final String? invitedBy;
  final String? joinedAt;
  final String? removedAt;
  final Map<String, String>? attributes;

  ConversationMember({
    this.tenantId,
    this.conversationId,
    this.memberId,
    this.principalId,
    this.principalKind,
    this.role,
    this.state,
    this.invitedBy,
    this.joinedAt,
    this.removedAt,
    this.attributes
  });

  factory ConversationMember.fromJson(Map<String, dynamic> json) {
    return ConversationMember(
      tenantId: json['tenantId']?.toString(),
      conversationId: json['conversationId']?.toString(),
      memberId: json['memberId']?.toString(),
      principalId: json['principalId']?.toString(),
      principalKind: json['principalKind']?.toString(),
      role: json['role']?.toString(),
      state: json['state']?.toString(),
      invitedBy: json['invitedBy']?.toString(),
      joinedAt: json['joinedAt']?.toString(),
      removedAt: json['removedAt']?.toString(),
      attributes: (() {
        final map = _sdkworkAsMap(json['attributes']);
        if (map == null) {
          return null;
        }
        final result = <String, String>{};
        map.forEach((key, item) {
          final deserialized = item?.toString();
          if (deserialized is String) {
            result[key] = deserialized;
          }
        });
        return result;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'tenantId': tenantId,
      'conversationId': conversationId,
      'memberId': memberId,
      'principalId': principalId,
      'principalKind': principalKind,
      'role': role,
      'state': state,
      'invitedBy': invitedBy,
      'joinedAt': joinedAt,
      'removedAt': removedAt,
      'attributes': attributes?.map((key, item) => MapEntry(key, item)),
    };
  }
}

class ConversationReadCursorView {
  final String? tenantId;
  final String? conversationId;
  final String? memberId;
  final String? principalId;
  final int? readSeq;
  final String? lastReadMessageId;
  final String? updatedAt;
  final int? unreadCount;

  ConversationReadCursorView({
    this.tenantId,
    this.conversationId,
    this.memberId,
    this.principalId,
    this.readSeq,
    this.lastReadMessageId,
    this.updatedAt,
    this.unreadCount
  });

  factory ConversationReadCursorView.fromJson(Map<String, dynamic> json) {
    return ConversationReadCursorView(
      tenantId: json['tenantId']?.toString(),
      conversationId: json['conversationId']?.toString(),
      memberId: json['memberId']?.toString(),
      principalId: json['principalId']?.toString(),
      readSeq: json['readSeq'] is int ? json['readSeq'] : null,
      lastReadMessageId: json['lastReadMessageId']?.toString(),
      updatedAt: json['updatedAt']?.toString(),
      unreadCount: json['unreadCount'] is int ? json['unreadCount'] : null
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'tenantId': tenantId,
      'conversationId': conversationId,
      'memberId': memberId,
      'principalId': principalId,
      'readSeq': readSeq,
      'lastReadMessageId': lastReadMessageId,
      'updatedAt': updatedAt,
      'unreadCount': unreadCount,
    };
  }
}

class ChangeAgentHandoffStatusView {
  final String? id;
  final String? kind;

  ChangeAgentHandoffStatusView({
    this.id,
    this.kind
  });

  factory ChangeAgentHandoffStatusView.fromJson(Map<String, dynamic> json) {
    return ChangeAgentHandoffStatusView(
      id: json['id']?.toString(),
      kind: json['kind']?.toString()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'id': id,
      'kind': kind,
    };
  }
}

class AgentHandoffStateView {
  final String? tenantId;
  final String? conversationId;
  final String? status;
  final ChangeAgentHandoffStatusView? source;
  final ChangeAgentHandoffStatusView? target;
  final String? handoffSessionId;
  final String? handoffReason;
  final String? acceptedAt;
  final ChangeAgentHandoffStatusView? acceptedBy;
  final String? resolvedAt;
  final ChangeAgentHandoffStatusView? resolvedBy;
  final String? closedAt;
  final ChangeAgentHandoffStatusView? closedBy;

  AgentHandoffStateView({
    this.tenantId,
    this.conversationId,
    this.status,
    this.source,
    this.target,
    this.handoffSessionId,
    this.handoffReason,
    this.acceptedAt,
    this.acceptedBy,
    this.resolvedAt,
    this.resolvedBy,
    this.closedAt,
    this.closedBy
  });

  factory AgentHandoffStateView.fromJson(Map<String, dynamic> json) {
    return AgentHandoffStateView(
      tenantId: json['tenantId']?.toString(),
      conversationId: json['conversationId']?.toString(),
      status: json['status']?.toString(),
      source: (() {
        final map = _sdkworkAsMap(json['source']);
        return map == null ? null : ChangeAgentHandoffStatusView.fromJson(map);
      })(),
      target: (() {
        final map = _sdkworkAsMap(json['target']);
        return map == null ? null : ChangeAgentHandoffStatusView.fromJson(map);
      })(),
      handoffSessionId: json['handoffSessionId']?.toString(),
      handoffReason: json['handoffReason']?.toString(),
      acceptedAt: json['acceptedAt']?.toString(),
      acceptedBy: (() {
        final map = _sdkworkAsMap(json['acceptedBy']);
        return map == null ? null : ChangeAgentHandoffStatusView.fromJson(map);
      })(),
      resolvedAt: json['resolvedAt']?.toString(),
      resolvedBy: (() {
        final map = _sdkworkAsMap(json['resolvedBy']);
        return map == null ? null : ChangeAgentHandoffStatusView.fromJson(map);
      })(),
      closedAt: json['closedAt']?.toString(),
      closedBy: (() {
        final map = _sdkworkAsMap(json['closedBy']);
        return map == null ? null : ChangeAgentHandoffStatusView.fromJson(map);
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'tenantId': tenantId,
      'conversationId': conversationId,
      'status': status,
      'source': source?.toJson(),
      'target': target?.toJson(),
      'handoffSessionId': handoffSessionId,
      'handoffReason': handoffReason,
      'acceptedAt': acceptedAt,
      'acceptedBy': acceptedBy?.toJson(),
      'resolvedAt': resolvedAt,
      'resolvedBy': resolvedBy?.toJson(),
      'closedAt': closedAt,
      'closedBy': closedBy?.toJson(),
    };
  }
}

class ConversationActorView {
  final String? id;
  final String? kind;

  ConversationActorView({
    this.id,
    this.kind
  });

  factory ConversationActorView.fromJson(Map<String, dynamic> json) {
    return ConversationActorView(
      id: json['id']?.toString(),
      kind: json['kind']?.toString()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'id': id,
      'kind': kind,
    };
  }
}

class ConversationAgentHandoffView {
  final String? status;
  final ConversationActorView? source;
  final ConversationActorView? target;
  final String? handoffSessionId;
  final String? handoffReason;
  final String? acceptedAt;
  final ConversationActorView? acceptedBy;
  final String? resolvedAt;
  final ConversationActorView? resolvedBy;
  final String? closedAt;
  final ConversationActorView? closedBy;

  ConversationAgentHandoffView({
    this.status,
    this.source,
    this.target,
    this.handoffSessionId,
    this.handoffReason,
    this.acceptedAt,
    this.acceptedBy,
    this.resolvedAt,
    this.resolvedBy,
    this.closedAt,
    this.closedBy
  });

  factory ConversationAgentHandoffView.fromJson(Map<String, dynamic> json) {
    return ConversationAgentHandoffView(
      status: json['status']?.toString(),
      source: (() {
        final map = _sdkworkAsMap(json['source']);
        return map == null ? null : ConversationActorView.fromJson(map);
      })(),
      target: (() {
        final map = _sdkworkAsMap(json['target']);
        return map == null ? null : ConversationActorView.fromJson(map);
      })(),
      handoffSessionId: json['handoffSessionId']?.toString(),
      handoffReason: json['handoffReason']?.toString(),
      acceptedAt: json['acceptedAt']?.toString(),
      acceptedBy: (() {
        final map = _sdkworkAsMap(json['acceptedBy']);
        return map == null ? null : ConversationActorView.fromJson(map);
      })(),
      resolvedAt: json['resolvedAt']?.toString(),
      resolvedBy: (() {
        final map = _sdkworkAsMap(json['resolvedBy']);
        return map == null ? null : ConversationActorView.fromJson(map);
      })(),
      closedAt: json['closedAt']?.toString(),
      closedBy: (() {
        final map = _sdkworkAsMap(json['closedBy']);
        return map == null ? null : ConversationActorView.fromJson(map);
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'status': status,
      'source': source?.toJson(),
      'target': target?.toJson(),
      'handoffSessionId': handoffSessionId,
      'handoffReason': handoffReason,
      'acceptedAt': acceptedAt,
      'acceptedBy': acceptedBy?.toJson(),
      'resolvedAt': resolvedAt,
      'resolvedBy': resolvedBy?.toJson(),
      'closedAt': closedAt,
      'closedBy': closedBy?.toJson(),
    };
  }
}

class ConversationInboxEntry {
  final String? tenantId;
  final String? principalId;
  final String? memberId;
  final String? conversationId;
  final String? conversationType;
  final int? messageCount;
  final String? lastMessageId;
  final int? lastMessageSeq;
  final String? lastSenderId;
  final String? lastSenderKind;
  final String? lastSummary;
  final int? unreadCount;
  final String? lastActivityAt;
  final ConversationAgentHandoffView? agentHandoff;

  ConversationInboxEntry({
    this.tenantId,
    this.principalId,
    this.memberId,
    this.conversationId,
    this.conversationType,
    this.messageCount,
    this.lastMessageId,
    this.lastMessageSeq,
    this.lastSenderId,
    this.lastSenderKind,
    this.lastSummary,
    this.unreadCount,
    this.lastActivityAt,
    this.agentHandoff
  });

  factory ConversationInboxEntry.fromJson(Map<String, dynamic> json) {
    return ConversationInboxEntry(
      tenantId: json['tenantId']?.toString(),
      principalId: json['principalId']?.toString(),
      memberId: json['memberId']?.toString(),
      conversationId: json['conversationId']?.toString(),
      conversationType: json['conversationType']?.toString(),
      messageCount: json['messageCount'] is int ? json['messageCount'] : null,
      lastMessageId: json['lastMessageId']?.toString(),
      lastMessageSeq: json['lastMessageSeq'] is int ? json['lastMessageSeq'] : null,
      lastSenderId: json['lastSenderId']?.toString(),
      lastSenderKind: json['lastSenderKind']?.toString(),
      lastSummary: json['lastSummary']?.toString(),
      unreadCount: json['unreadCount'] is int ? json['unreadCount'] : null,
      lastActivityAt: json['lastActivityAt']?.toString(),
      agentHandoff: (() {
        final map = _sdkworkAsMap(json['agentHandoff']);
        return map == null ? null : ConversationAgentHandoffView.fromJson(map);
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'tenantId': tenantId,
      'principalId': principalId,
      'memberId': memberId,
      'conversationId': conversationId,
      'conversationType': conversationType,
      'messageCount': messageCount,
      'lastMessageId': lastMessageId,
      'lastMessageSeq': lastMessageSeq,
      'lastSenderId': lastSenderId,
      'lastSenderKind': lastSenderKind,
      'lastSummary': lastSummary,
      'unreadCount': unreadCount,
      'lastActivityAt': lastActivityAt,
      'agentHandoff': agentHandoff?.toJson(),
    };
  }
}

class DeviceSyncFeedEntry {
  final String? tenantId;
  final String? principalId;
  final String? deviceId;
  final int? syncSeq;
  final String? originEventId;
  final String? originEventType;
  final String? conversationId;
  final String? messageId;
  final int? messageSeq;
  final String? memberId;
  final int? readSeq;
  final String? lastReadMessageId;
  final String? actorId;
  final String? actorKind;
  final String? actorDeviceId;
  final String? summary;
  final String? payloadSchema;
  final String? payload;
  final String? occurredAt;

  DeviceSyncFeedEntry({
    this.tenantId,
    this.principalId,
    this.deviceId,
    this.syncSeq,
    this.originEventId,
    this.originEventType,
    this.conversationId,
    this.messageId,
    this.messageSeq,
    this.memberId,
    this.readSeq,
    this.lastReadMessageId,
    this.actorId,
    this.actorKind,
    this.actorDeviceId,
    this.summary,
    this.payloadSchema,
    this.payload,
    this.occurredAt
  });

  factory DeviceSyncFeedEntry.fromJson(Map<String, dynamic> json) {
    return DeviceSyncFeedEntry(
      tenantId: json['tenantId']?.toString(),
      principalId: json['principalId']?.toString(),
      deviceId: json['deviceId']?.toString(),
      syncSeq: json['syncSeq'] is int ? json['syncSeq'] : null,
      originEventId: json['originEventId']?.toString(),
      originEventType: json['originEventType']?.toString(),
      conversationId: json['conversationId']?.toString(),
      messageId: json['messageId']?.toString(),
      messageSeq: json['messageSeq'] is int ? json['messageSeq'] : null,
      memberId: json['memberId']?.toString(),
      readSeq: json['readSeq'] is int ? json['readSeq'] : null,
      lastReadMessageId: json['lastReadMessageId']?.toString(),
      actorId: json['actorId']?.toString(),
      actorKind: json['actorKind']?.toString(),
      actorDeviceId: json['actorDeviceId']?.toString(),
      summary: json['summary']?.toString(),
      payloadSchema: json['payloadSchema']?.toString(),
      payload: json['payload']?.toString(),
      occurredAt: json['occurredAt']?.toString()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'tenantId': tenantId,
      'principalId': principalId,
      'deviceId': deviceId,
      'syncSeq': syncSeq,
      'originEventId': originEventId,
      'originEventType': originEventType,
      'conversationId': conversationId,
      'messageId': messageId,
      'messageSeq': messageSeq,
      'memberId': memberId,
      'readSeq': readSeq,
      'lastReadMessageId': lastReadMessageId,
      'actorId': actorId,
      'actorKind': actorKind,
      'actorDeviceId': actorDeviceId,
      'summary': summary,
      'payloadSchema': payloadSchema,
      'payload': payload,
      'occurredAt': occurredAt,
    };
  }
}

class ListMembersResponse {
  final List<ConversationMember>? items;

  ListMembersResponse({
    this.items
  });

  factory ListMembersResponse.fromJson(Map<String, dynamic> json) {
    return ListMembersResponse(
      items: (() {
        final list = _sdkworkAsList(json['items']);
        if (list == null) {
          return null;
        }
        return list
            .map((item) => (() {
        final map = _sdkworkAsMap(item);
        return map == null ? null : ConversationMember.fromJson(map);
      })())
            .whereType<ConversationMember>()
            .toList();
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'items': items?.map((item) => item.toJson()).toList(),
    };
  }
}

class InboxResponse {
  final List<ConversationInboxEntry>? items;

  InboxResponse({
    this.items
  });

  factory InboxResponse.fromJson(Map<String, dynamic> json) {
    return InboxResponse(
      items: (() {
        final list = _sdkworkAsList(json['items']);
        if (list == null) {
          return null;
        }
        return list
            .map((item) => (() {
        final map = _sdkworkAsMap(item);
        return map == null ? null : ConversationInboxEntry.fromJson(map);
      })())
            .whereType<ConversationInboxEntry>()
            .toList();
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'items': items?.map((item) => item.toJson()).toList(),
    };
  }
}

class DeviceSyncFeedResponse {
  final List<DeviceSyncFeedEntry>? items;

  DeviceSyncFeedResponse({
    this.items
  });

  factory DeviceSyncFeedResponse.fromJson(Map<String, dynamic> json) {
    return DeviceSyncFeedResponse(
      items: (() {
        final list = _sdkworkAsList(json['items']);
        if (list == null) {
          return null;
        }
        return list
            .map((item) => (() {
        final map = _sdkworkAsMap(item);
        return map == null ? null : DeviceSyncFeedEntry.fromJson(map);
      })())
            .whereType<DeviceSyncFeedEntry>()
            .toList();
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'items': items?.map((item) => item.toJson()).toList(),
    };
  }
}

class TimelineViewEntry {
  final String? tenantId;
  final String? conversationId;
  final String? messageId;
  final int? messageSeq;
  final String? summary;

  TimelineViewEntry({
    this.tenantId,
    this.conversationId,
    this.messageId,
    this.messageSeq,
    this.summary
  });

  factory TimelineViewEntry.fromJson(Map<String, dynamic> json) {
    return TimelineViewEntry(
      tenantId: json['tenantId']?.toString(),
      conversationId: json['conversationId']?.toString(),
      messageId: json['messageId']?.toString(),
      messageSeq: json['messageSeq'] is int ? json['messageSeq'] : null,
      summary: json['summary']?.toString()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'tenantId': tenantId,
      'conversationId': conversationId,
      'messageId': messageId,
      'messageSeq': messageSeq,
      'summary': summary,
    };
  }
}

class TimelineResponse {
  final List<TimelineViewEntry>? items;

  TimelineResponse({
    this.items
  });

  factory TimelineResponse.fromJson(Map<String, dynamic> json) {
    return TimelineResponse(
      items: (() {
        final list = _sdkworkAsList(json['items']);
        if (list == null) {
          return null;
        }
        return list
            .map((item) => (() {
        final map = _sdkworkAsMap(item);
        return map == null ? null : TimelineViewEntry.fromJson(map);
      })())
            .whereType<TimelineViewEntry>()
            .toList();
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'items': items?.map((item) => item.toJson()).toList(),
    };
  }
}

class SummarySenderView {
  final String? id;
  final String? kind;

  SummarySenderView({
    this.id,
    this.kind
  });

  factory SummarySenderView.fromJson(Map<String, dynamic> json) {
    return SummarySenderView(
      id: json['id']?.toString(),
      kind: json['kind']?.toString()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'id': id,
      'kind': kind,
    };
  }
}

class ConversationSummaryView {
  final String? tenantId;
  final String? conversationId;
  final int? messageCount;
  final String? lastMessageId;
  final int? lastMessageSeq;
  final String? lastSenderId;
  final String? lastSenderKind;
  final SummarySenderView? lastSender;
  final String? lastSummary;
  final String? lastMessageAt;
  final ConversationAgentHandoffView? agentHandoff;

  ConversationSummaryView({
    this.tenantId,
    this.conversationId,
    this.messageCount,
    this.lastMessageId,
    this.lastMessageSeq,
    this.lastSenderId,
    this.lastSenderKind,
    this.lastSender,
    this.lastSummary,
    this.lastMessageAt,
    this.agentHandoff
  });

  factory ConversationSummaryView.fromJson(Map<String, dynamic> json) {
    return ConversationSummaryView(
      tenantId: json['tenantId']?.toString(),
      conversationId: json['conversationId']?.toString(),
      messageCount: json['messageCount'] is int ? json['messageCount'] : null,
      lastMessageId: json['lastMessageId']?.toString(),
      lastMessageSeq: json['lastMessageSeq'] is int ? json['lastMessageSeq'] : null,
      lastSenderId: json['lastSenderId']?.toString(),
      lastSenderKind: json['lastSenderKind']?.toString(),
      lastSender: (() {
        final map = _sdkworkAsMap(json['lastSender']);
        return map == null ? null : SummarySenderView.fromJson(map);
      })(),
      lastSummary: json['lastSummary']?.toString(),
      lastMessageAt: json['lastMessageAt']?.toString(),
      agentHandoff: (() {
        final map = _sdkworkAsMap(json['agentHandoff']);
        return map == null ? null : ConversationAgentHandoffView.fromJson(map);
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'tenantId': tenantId,
      'conversationId': conversationId,
      'messageCount': messageCount,
      'lastMessageId': lastMessageId,
      'lastMessageSeq': lastMessageSeq,
      'lastSenderId': lastSenderId,
      'lastSenderKind': lastSenderKind,
      'lastSender': lastSender?.toJson(),
      'lastSummary': lastSummary,
      'lastMessageAt': lastMessageAt,
      'agentHandoff': agentHandoff?.toJson(),
    };
  }
}

class RegisteredDeviceView {
  final String? tenantId;
  final String? principalId;
  final String? deviceId;
  final String? registeredAt;

  RegisteredDeviceView({
    this.tenantId,
    this.principalId,
    this.deviceId,
    this.registeredAt
  });

  factory RegisteredDeviceView.fromJson(Map<String, dynamic> json) {
    return RegisteredDeviceView(
      tenantId: json['tenantId']?.toString(),
      principalId: json['principalId']?.toString(),
      deviceId: json['deviceId']?.toString(),
      registeredAt: json['registeredAt']?.toString()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'tenantId': tenantId,
      'principalId': principalId,
      'deviceId': deviceId,
      'registeredAt': registeredAt,
    };
  }
}

class CreateConversationResult {
  final String? conversationId;
  final String? eventId;

  CreateConversationResult({
    this.conversationId,
    this.eventId
  });

  factory CreateConversationResult.fromJson(Map<String, dynamic> json) {
    return CreateConversationResult(
      conversationId: json['conversationId']?.toString(),
      eventId: json['eventId']?.toString()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'conversationId': conversationId,
      'eventId': eventId,
    };
  }
}

class TransferConversationOwnerResult {
  final String? eventId;
  final String? transferredAt;
  final ConversationMember? previousOwner;
  final ConversationMember? newOwner;

  TransferConversationOwnerResult({
    this.eventId,
    this.transferredAt,
    this.previousOwner,
    this.newOwner
  });

  factory TransferConversationOwnerResult.fromJson(Map<String, dynamic> json) {
    return TransferConversationOwnerResult(
      eventId: json['eventId']?.toString(),
      transferredAt: json['transferredAt']?.toString(),
      previousOwner: (() {
        final map = _sdkworkAsMap(json['previousOwner']);
        return map == null ? null : ConversationMember.fromJson(map);
      })(),
      newOwner: (() {
        final map = _sdkworkAsMap(json['newOwner']);
        return map == null ? null : ConversationMember.fromJson(map);
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'eventId': eventId,
      'transferredAt': transferredAt,
      'previousOwner': previousOwner?.toJson(),
      'newOwner': newOwner?.toJson(),
    };
  }
}

class ChangeConversationMemberRoleResult {
  final String? eventId;
  final String? changedAt;
  final ConversationMember? previousMember;
  final ConversationMember? updatedMember;

  ChangeConversationMemberRoleResult({
    this.eventId,
    this.changedAt,
    this.previousMember,
    this.updatedMember
  });

  factory ChangeConversationMemberRoleResult.fromJson(Map<String, dynamic> json) {
    return ChangeConversationMemberRoleResult(
      eventId: json['eventId']?.toString(),
      changedAt: json['changedAt']?.toString(),
      previousMember: (() {
        final map = _sdkworkAsMap(json['previousMember']);
        return map == null ? null : ConversationMember.fromJson(map);
      })(),
      updatedMember: (() {
        final map = _sdkworkAsMap(json['updatedMember']);
        return map == null ? null : ConversationMember.fromJson(map);
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'eventId': eventId,
      'changedAt': changedAt,
      'previousMember': previousMember?.toJson(),
      'updatedMember': updatedMember?.toJson(),
    };
  }
}

class PostMessageResult {
  final String? messageId;
  final int? messageSeq;
  final String? eventId;

  PostMessageResult({
    this.messageId,
    this.messageSeq,
    this.eventId
  });

  factory PostMessageResult.fromJson(Map<String, dynamic> json) {
    return PostMessageResult(
      messageId: json['messageId']?.toString(),
      messageSeq: json['messageSeq'] is int ? json['messageSeq'] : null,
      eventId: json['eventId']?.toString()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'messageId': messageId,
      'messageSeq': messageSeq,
      'eventId': eventId,
    };
  }
}

class MessageMutationResult {
  final String? conversationId;
  final String? messageId;
  final int? messageSeq;
  final String? eventId;

  MessageMutationResult({
    this.conversationId,
    this.messageId,
    this.messageSeq,
    this.eventId
  });

  factory MessageMutationResult.fromJson(Map<String, dynamic> json) {
    return MessageMutationResult(
      conversationId: json['conversationId']?.toString(),
      messageId: json['messageId']?.toString(),
      messageSeq: json['messageSeq'] is int ? json['messageSeq'] : null,
      eventId: json['eventId']?.toString()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'conversationId': conversationId,
      'messageId': messageId,
      'messageSeq': messageSeq,
      'eventId': eventId,
    };
  }
}

class Sender {
  final String? id;
  final String? kind;
  final String? memberId;
  final String? deviceId;
  final String? sessionId;
  final Map<String, String>? metadata;

  Sender({
    this.id,
    this.kind,
    this.memberId,
    this.deviceId,
    this.sessionId,
    this.metadata
  });

  factory Sender.fromJson(Map<String, dynamic> json) {
    return Sender(
      id: json['id']?.toString(),
      kind: json['kind']?.toString(),
      memberId: json['memberId']?.toString(),
      deviceId: json['deviceId']?.toString(),
      sessionId: json['sessionId']?.toString(),
      metadata: (() {
        final map = _sdkworkAsMap(json['metadata']);
        if (map == null) {
          return null;
        }
        final result = <String, String>{};
        map.forEach((key, item) {
          final deserialized = item?.toString();
          if (deserialized is String) {
            result[key] = deserialized;
          }
        });
        return result;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'id': id,
      'kind': kind,
      'memberId': memberId,
      'deviceId': deviceId,
      'sessionId': sessionId,
      'metadata': metadata?.map((key, item) => MapEntry(key, item)),
    };
  }
}

class MessageBody {
  final String? summary;
  final List<ContentPart>? parts;
  final Map<String, String>? renderHints;

  MessageBody({
    this.summary,
    this.parts,
    this.renderHints
  });

  factory MessageBody.fromJson(Map<String, dynamic> json) {
    return MessageBody(
      summary: json['summary']?.toString(),
      parts: (() {
        final list = _sdkworkAsList(json['parts']);
        if (list == null) {
          return null;
        }
        return list
            .map((item) => (() {
        final map = _sdkworkAsMap(item);
        return map == null ? null : ContentPart.fromJson(map);
      })())
            .whereType<ContentPart>()
            .toList();
      })(),
      renderHints: (() {
        final map = _sdkworkAsMap(json['renderHints']);
        if (map == null) {
          return null;
        }
        final result = <String, String>{};
        map.forEach((key, item) {
          final deserialized = item?.toString();
          if (deserialized is String) {
            result[key] = deserialized;
          }
        });
        return result;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'summary': summary,
      'parts': parts?.map((item) => item.toJson()).toList(),
      'renderHints': renderHints?.map((key, item) => MapEntry(key, item)),
    };
  }
}

class ContentPart {
  final String? kind;
  final String? text;
  final String? schemaRef;
  final String? encoding;
  final String? payload;
  final String? mediaAssetId;
  final MediaResource? resource;
  final String? signalType;
  final String? streamId;
  final String? streamType;
  final String? state;

  ContentPart({
    this.kind,
    this.text,
    this.schemaRef,
    this.encoding,
    this.payload,
    this.mediaAssetId,
    this.resource,
    this.signalType,
    this.streamId,
    this.streamType,
    this.state
  });

  factory ContentPart.fromJson(Map<String, dynamic> json) {
    return ContentPart(
      kind: json['kind']?.toString(),
      text: json['text']?.toString(),
      schemaRef: json['schemaRef']?.toString(),
      encoding: json['encoding']?.toString(),
      payload: json['payload']?.toString(),
      mediaAssetId: json['mediaAssetId']?.toString(),
      resource: (() {
        final map = _sdkworkAsMap(json['resource']);
        return map == null ? null : MediaResource.fromJson(map);
      })(),
      signalType: json['signalType']?.toString(),
      streamId: json['streamId']?.toString(),
      streamType: json['streamType']?.toString(),
      state: json['state']?.toString()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'kind': kind,
      'text': text,
      'schemaRef': schemaRef,
      'encoding': encoding,
      'payload': payload,
      'mediaAssetId': mediaAssetId,
      'resource': resource?.toJson(),
      'signalType': signalType,
      'streamId': streamId,
      'streamType': streamType,
      'state': state,
    };
  }
}

class CreateUploadRequest {
  final String? mediaAssetId;
  final MediaResource? resource;

  CreateUploadRequest({
    this.mediaAssetId,
    this.resource
  });

  factory CreateUploadRequest.fromJson(Map<String, dynamic> json) {
    return CreateUploadRequest(
      mediaAssetId: json['mediaAssetId']?.toString(),
      resource: (() {
        final map = _sdkworkAsMap(json['resource']);
        return map == null ? null : MediaResource.fromJson(map);
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'mediaAssetId': mediaAssetId,
      'resource': resource?.toJson(),
    };
  }
}

class CompleteUploadRequest {
  final String? bucket;
  final String? objectKey;
  final String? storageProvider;
  final String? url;
  final String? checksum;

  CompleteUploadRequest({
    this.bucket,
    this.objectKey,
    this.storageProvider,
    this.url,
    this.checksum
  });

  factory CompleteUploadRequest.fromJson(Map<String, dynamic> json) {
    return CompleteUploadRequest(
      bucket: json['bucket']?.toString(),
      objectKey: json['objectKey']?.toString(),
      storageProvider: json['storageProvider']?.toString(),
      url: json['url']?.toString(),
      checksum: json['checksum']?.toString()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'bucket': bucket,
      'objectKey': objectKey,
      'storageProvider': storageProvider,
      'url': url,
      'checksum': checksum,
    };
  }
}

class MediaUploadSession {
  final String? assetId;
  final String? storageProvider;
  final String? bucket;
  final String? objectKey;
  final String? method;
  final String? url;
  final Map<String, String>? headers;
  final String? expiresAt;

  MediaUploadSession({
    this.assetId,
    this.storageProvider,
    this.bucket,
    this.objectKey,
    this.method,
    this.url,
    this.headers,
    this.expiresAt
  });

  factory MediaUploadSession.fromJson(Map<String, dynamic> json) {
    return MediaUploadSession(
      assetId: json['assetId']?.toString(),
      storageProvider: json['storageProvider']?.toString(),
      bucket: json['bucket']?.toString(),
      objectKey: json['objectKey']?.toString(),
      method: json['method']?.toString(),
      url: json['url']?.toString(),
      headers: (() {
        final map = _sdkworkAsMap(json['headers']);
        if (map == null) {
          return null;
        }
        final result = <String, String>{};
        map.forEach((key, item) {
          final deserialized = item?.toString();
          if (deserialized is String) {
            result[key] = deserialized;
          }
        });
        return result;
      })(),
      expiresAt: json['expiresAt']?.toString()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'assetId': assetId,
      'storageProvider': storageProvider,
      'bucket': bucket,
      'objectKey': objectKey,
      'method': method,
      'url': url,
      'headers': headers?.map((key, item) => MapEntry(key, item)),
      'expiresAt': expiresAt,
    };
  }
}

class MediaUploadMutationResponse {
  final String? tenantId;
  final String? principalId;
  final String? principalKind;
  final String? mediaAssetId;
  final String? bucket;
  final String? objectKey;
  final String? storageProvider;
  final String? checksum;
  final String? processingState;
  final MediaResource? resource;
  final String? createdAt;
  final String? completedAt;
  final MediaUploadSession? upload;
  final String? requestKey;
  final String? deliveryStatus;
  final String? proofVersion;

  MediaUploadMutationResponse({
    this.tenantId,
    this.principalId,
    this.principalKind,
    this.mediaAssetId,
    this.bucket,
    this.objectKey,
    this.storageProvider,
    this.checksum,
    this.processingState,
    this.resource,
    this.createdAt,
    this.completedAt,
    this.upload,
    this.requestKey,
    this.deliveryStatus,
    this.proofVersion
  });

  factory MediaUploadMutationResponse.fromJson(Map<String, dynamic> json) {
    return MediaUploadMutationResponse(
      tenantId: json['tenantId']?.toString(),
      principalId: json['principalId']?.toString(),
      principalKind: json['principalKind']?.toString(),
      mediaAssetId: json['mediaAssetId']?.toString(),
      bucket: json['bucket']?.toString(),
      objectKey: json['objectKey']?.toString(),
      storageProvider: json['storageProvider']?.toString(),
      checksum: json['checksum']?.toString(),
      processingState: json['processingState']?.toString(),
      resource: (() {
        final map = _sdkworkAsMap(json['resource']);
        return map == null ? null : MediaResource.fromJson(map);
      })(),
      createdAt: json['createdAt']?.toString(),
      completedAt: json['completedAt']?.toString(),
      upload: (() {
        final map = _sdkworkAsMap(json['upload']);
        return map == null ? null : MediaUploadSession.fromJson(map);
      })(),
      requestKey: json['requestKey']?.toString(),
      deliveryStatus: json['deliveryStatus']?.toString(),
      proofVersion: json['proofVersion']?.toString()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'tenantId': tenantId,
      'principalId': principalId,
      'principalKind': principalKind,
      'mediaAssetId': mediaAssetId,
      'bucket': bucket,
      'objectKey': objectKey,
      'storageProvider': storageProvider,
      'checksum': checksum,
      'processingState': processingState,
      'resource': resource?.toJson(),
      'createdAt': createdAt,
      'completedAt': completedAt,
      'upload': upload?.toJson(),
      'requestKey': requestKey,
      'deliveryStatus': deliveryStatus,
      'proofVersion': proofVersion,
    };
  }
}

class MediaDownloadUrlResponse {
  final String? mediaAssetId;
  final String? storageProvider;
  final String? downloadUrl;
  final int? expiresInSeconds;

  MediaDownloadUrlResponse({
    this.mediaAssetId,
    this.storageProvider,
    this.downloadUrl,
    this.expiresInSeconds
  });

  factory MediaDownloadUrlResponse.fromJson(Map<String, dynamic> json) {
    return MediaDownloadUrlResponse(
      mediaAssetId: json['mediaAssetId']?.toString(),
      storageProvider: json['storageProvider']?.toString(),
      downloadUrl: json['downloadUrl']?.toString(),
      expiresInSeconds: json['expiresInSeconds'] is int ? json['expiresInSeconds'] : null
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'mediaAssetId': mediaAssetId,
      'storageProvider': storageProvider,
      'downloadUrl': downloadUrl,
      'expiresInSeconds': expiresInSeconds,
    };
  }
}

class MediaResource {
  final int? id;
  final String? uuid;
  final String? url;
  final List<int>? bytes;
  final String? localFile;
  final String? base64;
  final String? type;
  final String? mimeType;
  final int? size;
  final String? name;
  final String? extension_;
  final Map<String, String>? tags;
  final Map<String, String>? metadata;
  final String? prompt;

  MediaResource({
    this.id,
    this.uuid,
    this.url,
    this.bytes,
    this.localFile,
    this.base64,
    this.type,
    this.mimeType,
    this.size,
    this.name,
    this.extension_,
    this.tags,
    this.metadata,
    this.prompt
  });

  factory MediaResource.fromJson(Map<String, dynamic> json) {
    return MediaResource(
      id: json['id'] is int ? json['id'] : null,
      uuid: json['uuid']?.toString(),
      url: json['url']?.toString(),
      bytes: (() {
        final list = _sdkworkAsList(json['bytes']);
        if (list == null) {
          return null;
        }
        return list
            .map((item) => item is int ? item : null)
            .whereType<int>()
            .toList();
      })(),
      localFile: json['localFile']?.toString(),
      base64: json['base64']?.toString(),
      type: json['type']?.toString(),
      mimeType: json['mimeType']?.toString(),
      size: json['size'] is int ? json['size'] : null,
      name: json['name']?.toString(),
      extension_: json['extension']?.toString(),
      tags: (() {
        final map = _sdkworkAsMap(json['tags']);
        if (map == null) {
          return null;
        }
        final result = <String, String>{};
        map.forEach((key, item) {
          final deserialized = item?.toString();
          if (deserialized is String) {
            result[key] = deserialized;
          }
        });
        return result;
      })(),
      metadata: (() {
        final map = _sdkworkAsMap(json['metadata']);
        if (map == null) {
          return null;
        }
        final result = <String, String>{};
        map.forEach((key, item) {
          final deserialized = item?.toString();
          if (deserialized is String) {
            result[key] = deserialized;
          }
        });
        return result;
      })(),
      prompt: json['prompt']?.toString()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'id': id,
      'uuid': uuid,
      'url': url,
      'bytes': bytes?.map((item) => item).toList(),
      'localFile': localFile,
      'base64': base64,
      'type': type,
      'mimeType': mimeType,
      'size': size,
      'name': name,
      'extension': extension_,
      'tags': tags?.map((key, item) => MapEntry(key, item)),
      'metadata': metadata?.map((key, item) => MapEntry(key, item)),
      'prompt': prompt,
    };
  }
}

class MediaAsset {
  final String? tenantId;
  final String? principalId;
  final String? principalKind;
  final String? mediaAssetId;
  final String? bucket;
  final String? objectKey;
  final String? storageProvider;
  final String? checksum;
  final String? processingState;
  final MediaResource? resource;
  final String? createdAt;
  final String? completedAt;

  MediaAsset({
    this.tenantId,
    this.principalId,
    this.principalKind,
    this.mediaAssetId,
    this.bucket,
    this.objectKey,
    this.storageProvider,
    this.checksum,
    this.processingState,
    this.resource,
    this.createdAt,
    this.completedAt
  });

  factory MediaAsset.fromJson(Map<String, dynamic> json) {
    return MediaAsset(
      tenantId: json['tenantId']?.toString(),
      principalId: json['principalId']?.toString(),
      principalKind: json['principalKind']?.toString(),
      mediaAssetId: json['mediaAssetId']?.toString(),
      bucket: json['bucket']?.toString(),
      objectKey: json['objectKey']?.toString(),
      storageProvider: json['storageProvider']?.toString(),
      checksum: json['checksum']?.toString(),
      processingState: json['processingState']?.toString(),
      resource: (() {
        final map = _sdkworkAsMap(json['resource']);
        return map == null ? null : MediaResource.fromJson(map);
      })(),
      createdAt: json['createdAt']?.toString(),
      completedAt: json['completedAt']?.toString()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'tenantId': tenantId,
      'principalId': principalId,
      'principalKind': principalKind,
      'mediaAssetId': mediaAssetId,
      'bucket': bucket,
      'objectKey': objectKey,
      'storageProvider': storageProvider,
      'checksum': checksum,
      'processingState': processingState,
      'resource': resource?.toJson(),
      'createdAt': createdAt,
      'completedAt': completedAt,
    };
  }
}

class OpenStreamRequest {
  final String? streamId;
  final String? streamType;
  final String? scopeKind;
  final String? scopeId;
  final String? durabilityClass;
  final String? schemaRef;

  OpenStreamRequest({
    this.streamId,
    this.streamType,
    this.scopeKind,
    this.scopeId,
    this.durabilityClass,
    this.schemaRef
  });

  factory OpenStreamRequest.fromJson(Map<String, dynamic> json) {
    return OpenStreamRequest(
      streamId: json['streamId']?.toString(),
      streamType: json['streamType']?.toString(),
      scopeKind: json['scopeKind']?.toString(),
      scopeId: json['scopeId']?.toString(),
      durabilityClass: json['durabilityClass']?.toString(),
      schemaRef: json['schemaRef']?.toString()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'streamId': streamId,
      'streamType': streamType,
      'scopeKind': scopeKind,
      'scopeId': scopeId,
      'durabilityClass': durabilityClass,
      'schemaRef': schemaRef,
    };
  }
}

class CheckpointStreamRequest {
  final int? frameSeq;

  CheckpointStreamRequest({
    this.frameSeq
  });

  factory CheckpointStreamRequest.fromJson(Map<String, dynamic> json) {
    return CheckpointStreamRequest(
      frameSeq: json['frameSeq'] is int ? json['frameSeq'] : null
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'frameSeq': frameSeq,
    };
  }
}

class CompleteStreamRequest {
  final int? frameSeq;
  final String? resultMessageId;

  CompleteStreamRequest({
    this.frameSeq,
    this.resultMessageId
  });

  factory CompleteStreamRequest.fromJson(Map<String, dynamic> json) {
    return CompleteStreamRequest(
      frameSeq: json['frameSeq'] is int ? json['frameSeq'] : null,
      resultMessageId: json['resultMessageId']?.toString()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'frameSeq': frameSeq,
      'resultMessageId': resultMessageId,
    };
  }
}

class AbortStreamRequest {
  final int? frameSeq;
  final String? reason;

  AbortStreamRequest({
    this.frameSeq,
    this.reason
  });

  factory AbortStreamRequest.fromJson(Map<String, dynamic> json) {
    return AbortStreamRequest(
      frameSeq: json['frameSeq'] is int ? json['frameSeq'] : null,
      reason: json['reason']?.toString()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'frameSeq': frameSeq,
      'reason': reason,
    };
  }
}

class AppendStreamFrameRequest {
  final int? frameSeq;
  final String? frameType;
  final String? schemaRef;
  final String? encoding;
  final String? payload;
  final Map<String, String>? attributes;

  AppendStreamFrameRequest({
    this.frameSeq,
    this.frameType,
    this.schemaRef,
    this.encoding,
    this.payload,
    this.attributes
  });

  factory AppendStreamFrameRequest.fromJson(Map<String, dynamic> json) {
    return AppendStreamFrameRequest(
      frameSeq: json['frameSeq'] is int ? json['frameSeq'] : null,
      frameType: json['frameType']?.toString(),
      schemaRef: json['schemaRef']?.toString(),
      encoding: json['encoding']?.toString(),
      payload: json['payload']?.toString(),
      attributes: (() {
        final map = _sdkworkAsMap(json['attributes']);
        if (map == null) {
          return null;
        }
        final result = <String, String>{};
        map.forEach((key, item) {
          final deserialized = item?.toString();
          if (deserialized is String) {
            result[key] = deserialized;
          }
        });
        return result;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'frameSeq': frameSeq,
      'frameType': frameType,
      'schemaRef': schemaRef,
      'encoding': encoding,
      'payload': payload,
      'attributes': attributes?.map((key, item) => MapEntry(key, item)),
    };
  }
}

class StreamSession {
  final String? tenantId;
  final String? streamId;
  final String? streamType;
  final String? scopeKind;
  final String? scopeId;
  final String? durabilityClass;
  final String? orderingScope;
  final String? schemaRef;
  final String? state;
  final int? lastFrameSeq;
  final int? lastCheckpointSeq;
  final String? resultMessageId;
  final String? openedAt;
  final String? closedAt;
  final String? expiresAt;

  StreamSession({
    this.tenantId,
    this.streamId,
    this.streamType,
    this.scopeKind,
    this.scopeId,
    this.durabilityClass,
    this.orderingScope,
    this.schemaRef,
    this.state,
    this.lastFrameSeq,
    this.lastCheckpointSeq,
    this.resultMessageId,
    this.openedAt,
    this.closedAt,
    this.expiresAt
  });

  factory StreamSession.fromJson(Map<String, dynamic> json) {
    return StreamSession(
      tenantId: json['tenantId']?.toString(),
      streamId: json['streamId']?.toString(),
      streamType: json['streamType']?.toString(),
      scopeKind: json['scopeKind']?.toString(),
      scopeId: json['scopeId']?.toString(),
      durabilityClass: json['durabilityClass']?.toString(),
      orderingScope: json['orderingScope']?.toString(),
      schemaRef: json['schemaRef']?.toString(),
      state: json['state']?.toString(),
      lastFrameSeq: json['lastFrameSeq'] is int ? json['lastFrameSeq'] : null,
      lastCheckpointSeq: json['lastCheckpointSeq'] is int ? json['lastCheckpointSeq'] : null,
      resultMessageId: json['resultMessageId']?.toString(),
      openedAt: json['openedAt']?.toString(),
      closedAt: json['closedAt']?.toString(),
      expiresAt: json['expiresAt']?.toString()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'tenantId': tenantId,
      'streamId': streamId,
      'streamType': streamType,
      'scopeKind': scopeKind,
      'scopeId': scopeId,
      'durabilityClass': durabilityClass,
      'orderingScope': orderingScope,
      'schemaRef': schemaRef,
      'state': state,
      'lastFrameSeq': lastFrameSeq,
      'lastCheckpointSeq': lastCheckpointSeq,
      'resultMessageId': resultMessageId,
      'openedAt': openedAt,
      'closedAt': closedAt,
      'expiresAt': expiresAt,
    };
  }
}

class StreamFrame {
  final String? tenantId;
  final String? streamId;
  final String? streamType;
  final String? scopeKind;
  final String? scopeId;
  final int? frameSeq;
  final String? frameType;
  final String? schemaRef;
  final String? encoding;
  final String? payload;
  final Sender? sender;
  final Map<String, String>? attributes;
  final String? occurredAt;

  StreamFrame({
    this.tenantId,
    this.streamId,
    this.streamType,
    this.scopeKind,
    this.scopeId,
    this.frameSeq,
    this.frameType,
    this.schemaRef,
    this.encoding,
    this.payload,
    this.sender,
    this.attributes,
    this.occurredAt
  });

  factory StreamFrame.fromJson(Map<String, dynamic> json) {
    return StreamFrame(
      tenantId: json['tenantId']?.toString(),
      streamId: json['streamId']?.toString(),
      streamType: json['streamType']?.toString(),
      scopeKind: json['scopeKind']?.toString(),
      scopeId: json['scopeId']?.toString(),
      frameSeq: json['frameSeq'] is int ? json['frameSeq'] : null,
      frameType: json['frameType']?.toString(),
      schemaRef: json['schemaRef']?.toString(),
      encoding: json['encoding']?.toString(),
      payload: json['payload']?.toString(),
      sender: (() {
        final map = _sdkworkAsMap(json['sender']);
        return map == null ? null : Sender.fromJson(map);
      })(),
      attributes: (() {
        final map = _sdkworkAsMap(json['attributes']);
        if (map == null) {
          return null;
        }
        final result = <String, String>{};
        map.forEach((key, item) {
          final deserialized = item?.toString();
          if (deserialized is String) {
            result[key] = deserialized;
          }
        });
        return result;
      })(),
      occurredAt: json['occurredAt']?.toString()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'tenantId': tenantId,
      'streamId': streamId,
      'streamType': streamType,
      'scopeKind': scopeKind,
      'scopeId': scopeId,
      'frameSeq': frameSeq,
      'frameType': frameType,
      'schemaRef': schemaRef,
      'encoding': encoding,
      'payload': payload,
      'sender': sender?.toJson(),
      'attributes': attributes?.map((key, item) => MapEntry(key, item)),
      'occurredAt': occurredAt,
    };
  }
}

class StreamFrameWindow {
  final List<StreamFrame>? items;
  final int? nextAfterFrameSeq;
  final bool? hasMore;

  StreamFrameWindow({
    this.items,
    this.nextAfterFrameSeq,
    this.hasMore
  });

  factory StreamFrameWindow.fromJson(Map<String, dynamic> json) {
    return StreamFrameWindow(
      items: (() {
        final list = _sdkworkAsList(json['items']);
        if (list == null) {
          return null;
        }
        return list
            .map((item) => (() {
        final map = _sdkworkAsMap(item);
        return map == null ? null : StreamFrame.fromJson(map);
      })())
            .whereType<StreamFrame>()
            .toList();
      })(),
      nextAfterFrameSeq: json['nextAfterFrameSeq'] is int ? json['nextAfterFrameSeq'] : null,
      hasMore: json['hasMore'] is bool ? json['hasMore'] : null
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'items': items?.map((item) => item.toJson()).toList(),
      'nextAfterFrameSeq': nextAfterFrameSeq,
      'hasMore': hasMore,
    };
  }
}

class CreateRtcSessionRequest {
  final String? rtcSessionId;
  final String? conversationId;
  final String? rtcMode;

  CreateRtcSessionRequest({
    this.rtcSessionId,
    this.conversationId,
    this.rtcMode
  });

  factory CreateRtcSessionRequest.fromJson(Map<String, dynamic> json) {
    return CreateRtcSessionRequest(
      rtcSessionId: json['rtcSessionId']?.toString(),
      conversationId: json['conversationId']?.toString(),
      rtcMode: json['rtcMode']?.toString()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'rtcSessionId': rtcSessionId,
      'conversationId': conversationId,
      'rtcMode': rtcMode,
    };
  }
}

class InviteRtcSessionRequest {
  final String? signalingStreamId;

  InviteRtcSessionRequest({
    this.signalingStreamId
  });

  factory InviteRtcSessionRequest.fromJson(Map<String, dynamic> json) {
    return InviteRtcSessionRequest(
      signalingStreamId: json['signalingStreamId']?.toString()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'signalingStreamId': signalingStreamId,
    };
  }
}

class UpdateRtcSessionRequest {
  final String? artifactMessageId;

  UpdateRtcSessionRequest({
    this.artifactMessageId
  });

  factory UpdateRtcSessionRequest.fromJson(Map<String, dynamic> json) {
    return UpdateRtcSessionRequest(
      artifactMessageId: json['artifactMessageId']?.toString()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'artifactMessageId': artifactMessageId,
    };
  }
}

class PostRtcSignalRequest {
  final String? signalType;
  final String? schemaRef;
  final String? payload;
  final String? signalingStreamId;

  PostRtcSignalRequest({
    this.signalType,
    this.schemaRef,
    this.payload,
    this.signalingStreamId
  });

  factory PostRtcSignalRequest.fromJson(Map<String, dynamic> json) {
    return PostRtcSignalRequest(
      signalType: json['signalType']?.toString(),
      schemaRef: json['schemaRef']?.toString(),
      payload: json['payload']?.toString(),
      signalingStreamId: json['signalingStreamId']?.toString()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'signalType': signalType,
      'schemaRef': schemaRef,
      'payload': payload,
      'signalingStreamId': signalingStreamId,
    };
  }
}

class IssueRtcParticipantCredentialRequest {
  final String? participantId;

  IssueRtcParticipantCredentialRequest({
    this.participantId
  });

  factory IssueRtcParticipantCredentialRequest.fromJson(Map<String, dynamic> json) {
    return IssueRtcParticipantCredentialRequest(
      participantId: json['participantId']?.toString()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'participantId': participantId,
    };
  }
}

class RtcSession {
  final String? tenantId;
  final String? rtcSessionId;
  final String? conversationId;
  final String? rtcMode;
  final String? initiatorId;
  final String? providerPluginId;
  final String? providerSessionId;
  final String? accessEndpoint;
  final String? providerRegion;
  final String? state;
  final String? signalingStreamId;
  final String? artifactMessageId;
  final String? startedAt;
  final String? endedAt;

  RtcSession({
    this.tenantId,
    this.rtcSessionId,
    this.conversationId,
    this.rtcMode,
    this.initiatorId,
    this.providerPluginId,
    this.providerSessionId,
    this.accessEndpoint,
    this.providerRegion,
    this.state,
    this.signalingStreamId,
    this.artifactMessageId,
    this.startedAt,
    this.endedAt
  });

  factory RtcSession.fromJson(Map<String, dynamic> json) {
    return RtcSession(
      tenantId: json['tenantId']?.toString(),
      rtcSessionId: json['rtcSessionId']?.toString(),
      conversationId: json['conversationId']?.toString(),
      rtcMode: json['rtcMode']?.toString(),
      initiatorId: json['initiatorId']?.toString(),
      providerPluginId: json['providerPluginId']?.toString(),
      providerSessionId: json['providerSessionId']?.toString(),
      accessEndpoint: json['accessEndpoint']?.toString(),
      providerRegion: json['providerRegion']?.toString(),
      state: json['state']?.toString(),
      signalingStreamId: json['signalingStreamId']?.toString(),
      artifactMessageId: json['artifactMessageId']?.toString(),
      startedAt: json['startedAt']?.toString(),
      endedAt: json['endedAt']?.toString()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'tenantId': tenantId,
      'rtcSessionId': rtcSessionId,
      'conversationId': conversationId,
      'rtcMode': rtcMode,
      'initiatorId': initiatorId,
      'providerPluginId': providerPluginId,
      'providerSessionId': providerSessionId,
      'accessEndpoint': accessEndpoint,
      'providerRegion': providerRegion,
      'state': state,
      'signalingStreamId': signalingStreamId,
      'artifactMessageId': artifactMessageId,
      'startedAt': startedAt,
      'endedAt': endedAt,
    };
  }
}

class RtcSignalEvent {
  final String? tenantId;
  final String? rtcSessionId;
  final String? conversationId;
  final String? rtcMode;
  final String? signalType;
  final String? schemaRef;
  final String? payload;
  final Sender? sender;
  final String? signalingStreamId;
  final String? occurredAt;

  RtcSignalEvent({
    this.tenantId,
    this.rtcSessionId,
    this.conversationId,
    this.rtcMode,
    this.signalType,
    this.schemaRef,
    this.payload,
    this.sender,
    this.signalingStreamId,
    this.occurredAt
  });

  factory RtcSignalEvent.fromJson(Map<String, dynamic> json) {
    return RtcSignalEvent(
      tenantId: json['tenantId']?.toString(),
      rtcSessionId: json['rtcSessionId']?.toString(),
      conversationId: json['conversationId']?.toString(),
      rtcMode: json['rtcMode']?.toString(),
      signalType: json['signalType']?.toString(),
      schemaRef: json['schemaRef']?.toString(),
      payload: json['payload']?.toString(),
      sender: (() {
        final map = _sdkworkAsMap(json['sender']);
        return map == null ? null : Sender.fromJson(map);
      })(),
      signalingStreamId: json['signalingStreamId']?.toString(),
      occurredAt: json['occurredAt']?.toString()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'tenantId': tenantId,
      'rtcSessionId': rtcSessionId,
      'conversationId': conversationId,
      'rtcMode': rtcMode,
      'signalType': signalType,
      'schemaRef': schemaRef,
      'payload': payload,
      'sender': sender?.toJson(),
      'signalingStreamId': signalingStreamId,
      'occurredAt': occurredAt,
    };
  }
}

class RtcParticipantCredential {
  final String? tenantId;
  final String? rtcSessionId;
  final String? participantId;
  final String? credential;
  final String? expiresAt;

  RtcParticipantCredential({
    this.tenantId,
    this.rtcSessionId,
    this.participantId,
    this.credential,
    this.expiresAt
  });

  factory RtcParticipantCredential.fromJson(Map<String, dynamic> json) {
    return RtcParticipantCredential(
      tenantId: json['tenantId']?.toString(),
      rtcSessionId: json['rtcSessionId']?.toString(),
      participantId: json['participantId']?.toString(),
      credential: json['credential']?.toString(),
      expiresAt: json['expiresAt']?.toString()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'tenantId': tenantId,
      'rtcSessionId': rtcSessionId,
      'participantId': participantId,
      'credential': credential,
      'expiresAt': expiresAt,
    };
  }
}

class RtcRecordingArtifact {
  final String? tenantId;
  final String? rtcSessionId;
  final String? bucket;
  final String? objectKey;
  final String? storageProvider;
  final String? playbackUrl;

  RtcRecordingArtifact({
    this.tenantId,
    this.rtcSessionId,
    this.bucket,
    this.objectKey,
    this.storageProvider,
    this.playbackUrl
  });

  factory RtcRecordingArtifact.fromJson(Map<String, dynamic> json) {
    return RtcRecordingArtifact(
      tenantId: json['tenantId']?.toString(),
      rtcSessionId: json['rtcSessionId']?.toString(),
      bucket: json['bucket']?.toString(),
      objectKey: json['objectKey']?.toString(),
      storageProvider: json['storageProvider']?.toString(),
      playbackUrl: json['playbackUrl']?.toString()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'tenantId': tenantId,
      'rtcSessionId': rtcSessionId,
      'bucket': bucket,
      'objectKey': objectKey,
      'storageProvider': storageProvider,
      'playbackUrl': playbackUrl,
    };
  }
}
