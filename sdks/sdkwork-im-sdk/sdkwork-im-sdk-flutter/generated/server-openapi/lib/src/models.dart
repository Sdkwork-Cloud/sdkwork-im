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

class AckResponse {
  final bool ok;

  AckResponse({
    required this.ok
  });

  factory AckResponse.fromJson(Map<String, dynamic> json) {
    return AckResponse(
      ok: (() {
        final value = json['ok'];
        if (value is! bool) {
          throw FormatException('AckResponse.ok is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'ok': ok,
    };
  }
}

class DeviceSessionView {
  final String tenantId;
  final String principalId;
  final String principalKind;
  final String deviceId;
  final String resumedAt;

  DeviceSessionView({
    required this.tenantId,
    required this.principalId,
    required this.principalKind,
    required this.deviceId,
    required this.resumedAt
  });

  factory DeviceSessionView.fromJson(Map<String, dynamic> json) {
    return DeviceSessionView(
      tenantId: (() {
        final value = json['tenantId']?.toString();
        if (value == null) {
          throw FormatException('DeviceSessionView.tenantId is required');
        }
        return value;
      })(),
      principalId: (() {
        final value = json['principalId']?.toString();
        if (value == null) {
          throw FormatException('DeviceSessionView.principalId is required');
        }
        return value;
      })(),
      principalKind: (() {
        final value = json['principalKind']?.toString();
        if (value == null) {
          throw FormatException('DeviceSessionView.principalKind is required');
        }
        return value;
      })(),
      deviceId: (() {
        final value = json['deviceId']?.toString();
        if (value == null) {
          throw FormatException('DeviceSessionView.deviceId is required');
        }
        return value;
      })(),
      resumedAt: (() {
        final value = json['resumedAt']?.toString();
        if (value == null) {
          throw FormatException('DeviceSessionView.resumedAt is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'tenantId': tenantId,
      'principalId': principalId,
      'principalKind': principalKind,
      'deviceId': deviceId,
      'resumedAt': resumedAt,
    };
  }
}

class DeviceSessionDisconnectResponse {
  final String deviceId;
  final bool disconnected;

  DeviceSessionDisconnectResponse({
    required this.deviceId,
    required this.disconnected
  });

  factory DeviceSessionDisconnectResponse.fromJson(Map<String, dynamic> json) {
    return DeviceSessionDisconnectResponse(
      deviceId: (() {
        final value = json['deviceId']?.toString();
        if (value == null) {
          throw FormatException('DeviceSessionDisconnectResponse.deviceId is required');
        }
        return value;
      })(),
      disconnected: (() {
        final value = json['disconnected'];
        if (value is! bool) {
          throw FormatException('DeviceSessionDisconnectResponse.disconnected is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'deviceId': deviceId,
      'disconnected': disconnected,
    };
  }
}

class ResumeDeviceSessionRequest {
  final String? deviceId;
  final int? lastSeenSyncSeq;

  ResumeDeviceSessionRequest({
    this.deviceId,
    this.lastSeenSyncSeq
  });

  factory ResumeDeviceSessionRequest.fromJson(Map<String, dynamic> json) {
    return ResumeDeviceSessionRequest(
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

class DevicePresenceRequest {
  final String? deviceId;

  DevicePresenceRequest({
    this.deviceId
  });

  factory DevicePresenceRequest.fromJson(Map<String, dynamic> json) {
    return DevicePresenceRequest(
      deviceId: json['deviceId']?.toString()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'deviceId': deviceId,
    };
  }
}

class PresenceView {
  final String tenantId;
  final String principalId;
  final String principalKind;
  final String deviceId;
  final String status;
  final String updatedAt;

  PresenceView({
    required this.tenantId,
    required this.principalId,
    required this.principalKind,
    required this.deviceId,
    required this.status,
    required this.updatedAt
  });

  factory PresenceView.fromJson(Map<String, dynamic> json) {
    return PresenceView(
      tenantId: (() {
        final value = json['tenantId']?.toString();
        if (value == null) {
          throw FormatException('PresenceView.tenantId is required');
        }
        return value;
      })(),
      principalId: (() {
        final value = json['principalId']?.toString();
        if (value == null) {
          throw FormatException('PresenceView.principalId is required');
        }
        return value;
      })(),
      principalKind: (() {
        final value = json['principalKind']?.toString();
        if (value == null) {
          throw FormatException('PresenceView.principalKind is required');
        }
        return value;
      })(),
      deviceId: (() {
        final value = json['deviceId']?.toString();
        if (value == null) {
          throw FormatException('PresenceView.deviceId is required');
        }
        return value;
      })(),
      status: (() {
        final value = json['status']?.toString();
        if (value == null) {
          throw FormatException('PresenceView.status is required');
        }
        return value;
      })(),
      updatedAt: (() {
        final value = json['updatedAt']?.toString();
        if (value == null) {
          throw FormatException('PresenceView.updatedAt is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'tenantId': tenantId,
      'principalId': principalId,
      'principalKind': principalKind,
      'deviceId': deviceId,
      'status': status,
      'updatedAt': updatedAt,
    };
  }
}

class RealtimeSubscriptionSyncRequest {
  final String? deviceId;
  final List<String>? conversations;

  RealtimeSubscriptionSyncRequest({
    this.deviceId,
    this.conversations
  });

  factory RealtimeSubscriptionSyncRequest.fromJson(Map<String, dynamic> json) {
    return RealtimeSubscriptionSyncRequest(
      deviceId: json['deviceId']?.toString(),
      conversations: (() {
        final list = _sdkworkAsList(json['conversations']);
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
      'deviceId': deviceId,
      'conversations': conversations?.map((item) => item).toList(),
    };
  }
}

class RealtimeSubscriptionSyncResponse {
  final List<String> subscriptions;

  RealtimeSubscriptionSyncResponse({
    required this.subscriptions
  });

  factory RealtimeSubscriptionSyncResponse.fromJson(Map<String, dynamic> json) {
    return RealtimeSubscriptionSyncResponse(
      subscriptions: (() {
        final list = _sdkworkAsList(json['subscriptions']);
        if (list == null) {
          throw FormatException('RealtimeSubscriptionSyncResponse.subscriptions is required');
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
      'subscriptions': subscriptions.map((item) => item).toList(),
    };
  }
}

class RealtimeEventAckRequest {
  final List<String> eventIds;

  RealtimeEventAckRequest({
    required this.eventIds
  });

  factory RealtimeEventAckRequest.fromJson(Map<String, dynamic> json) {
    return RealtimeEventAckRequest(
      eventIds: (() {
        final list = _sdkworkAsList(json['eventIds']);
        if (list == null) {
          throw FormatException('RealtimeEventAckRequest.eventIds is required');
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
      'eventIds': eventIds.map((item) => item).toList(),
    };
  }
}

class RealtimeEventView {
  final String eventId;
  final String scope;
  final String scopeId;
  final String eventType;
  final String? payload;
  final String occurredAt;

  RealtimeEventView({
    required this.eventId,
    required this.scope,
    required this.scopeId,
    required this.eventType,
    this.payload,
    required this.occurredAt
  });

  factory RealtimeEventView.fromJson(Map<String, dynamic> json) {
    return RealtimeEventView(
      eventId: (() {
        final value = json['eventId']?.toString();
        if (value == null) {
          throw FormatException('RealtimeEventView.eventId is required');
        }
        return value;
      })(),
      scope: (() {
        final value = json['scope']?.toString();
        if (value == null) {
          throw FormatException('RealtimeEventView.scope is required');
        }
        return value;
      })(),
      scopeId: (() {
        final value = json['scopeId']?.toString();
        if (value == null) {
          throw FormatException('RealtimeEventView.scopeId is required');
        }
        return value;
      })(),
      eventType: (() {
        final value = json['eventType']?.toString();
        if (value == null) {
          throw FormatException('RealtimeEventView.eventType is required');
        }
        return value;
      })(),
      payload: json['payload']?.toString(),
      occurredAt: (() {
        final value = json['occurredAt']?.toString();
        if (value == null) {
          throw FormatException('RealtimeEventView.occurredAt is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'eventId': eventId,
      'scope': scope,
      'scopeId': scopeId,
      'eventType': eventType,
      'payload': payload,
      'occurredAt': occurredAt,
    };
  }
}

class RealtimeEventsResponse {
  final List<RealtimeEventView> items;
  final String? nextCursor;
  final bool hasMore;

  RealtimeEventsResponse({
    required this.items,
    this.nextCursor,
    required this.hasMore
  });

  factory RealtimeEventsResponse.fromJson(Map<String, dynamic> json) {
    return RealtimeEventsResponse(
      items: (() {
        final list = _sdkworkAsList(json['items']);
        if (list == null) {
          throw FormatException('RealtimeEventsResponse.items is required');
        }
        return list
            .map((item) => (() {
        final map = _sdkworkAsMap(item);
        return map == null ? null : RealtimeEventView.fromJson(map);
      })())
            .whereType<RealtimeEventView>()
            .toList();
      })(),
      nextCursor: json['nextCursor']?.toString(),
      hasMore: (() {
        final value = json['hasMore'];
        if (value is! bool) {
          throw FormatException('RealtimeEventsResponse.hasMore is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'items': items.map((item) => item.toJson()).toList(),
      'nextCursor': nextCursor,
      'hasMore': hasMore,
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

class RegisteredDeviceView {
  final String tenantId;
  final String principalId;
  final String principalKind;
  final String deviceId;
  final String registeredAt;

  RegisteredDeviceView({
    required this.tenantId,
    required this.principalId,
    required this.principalKind,
    required this.deviceId,
    required this.registeredAt
  });

  factory RegisteredDeviceView.fromJson(Map<String, dynamic> json) {
    return RegisteredDeviceView(
      tenantId: (() {
        final value = json['tenantId']?.toString();
        if (value == null) {
          throw FormatException('RegisteredDeviceView.tenantId is required');
        }
        return value;
      })(),
      principalId: (() {
        final value = json['principalId']?.toString();
        if (value == null) {
          throw FormatException('RegisteredDeviceView.principalId is required');
        }
        return value;
      })(),
      principalKind: (() {
        final value = json['principalKind']?.toString();
        if (value == null) {
          throw FormatException('RegisteredDeviceView.principalKind is required');
        }
        return value;
      })(),
      deviceId: (() {
        final value = json['deviceId']?.toString();
        if (value == null) {
          throw FormatException('RegisteredDeviceView.deviceId is required');
        }
        return value;
      })(),
      registeredAt: (() {
        final value = json['registeredAt']?.toString();
        if (value == null) {
          throw FormatException('RegisteredDeviceView.registeredAt is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'tenantId': tenantId,
      'principalId': principalId,
      'principalKind': principalKind,
      'deviceId': deviceId,
      'registeredAt': registeredAt,
    };
  }
}

class DeviceSyncFeedEntry {
  final String tenantId;
  final String principalId;
  final String principalKind;
  final String? deviceId;
  final int syncSeq;
  final String eventId;
  final String originEventType;
  final String? actorId;
  final String? conversationId;
  final String? messageId;
  final int? messageSeq;
  final String? payload;
  final int? readSeq;
  final String? summary;
  final String occurredAt;

  DeviceSyncFeedEntry({
    required this.tenantId,
    required this.principalId,
    required this.principalKind,
    this.deviceId,
    required this.syncSeq,
    required this.eventId,
    required this.originEventType,
    this.actorId,
    this.conversationId,
    this.messageId,
    this.messageSeq,
    this.payload,
    this.readSeq,
    this.summary,
    required this.occurredAt
  });

  factory DeviceSyncFeedEntry.fromJson(Map<String, dynamic> json) {
    return DeviceSyncFeedEntry(
      tenantId: (() {
        final value = json['tenantId']?.toString();
        if (value == null) {
          throw FormatException('DeviceSyncFeedEntry.tenantId is required');
        }
        return value;
      })(),
      principalId: (() {
        final value = json['principalId']?.toString();
        if (value == null) {
          throw FormatException('DeviceSyncFeedEntry.principalId is required');
        }
        return value;
      })(),
      principalKind: (() {
        final value = json['principalKind']?.toString();
        if (value == null) {
          throw FormatException('DeviceSyncFeedEntry.principalKind is required');
        }
        return value;
      })(),
      deviceId: json['deviceId']?.toString(),
      syncSeq: (() {
        final value = json['syncSeq'];
        if (value is! int) {
          throw FormatException('DeviceSyncFeedEntry.syncSeq is required');
        }
        return value;
      })(),
      eventId: (() {
        final value = json['eventId']?.toString();
        if (value == null) {
          throw FormatException('DeviceSyncFeedEntry.eventId is required');
        }
        return value;
      })(),
      originEventType: (() {
        final value = json['originEventType']?.toString();
        if (value == null) {
          throw FormatException('DeviceSyncFeedEntry.originEventType is required');
        }
        return value;
      })(),
      actorId: json['actorId']?.toString(),
      conversationId: json['conversationId']?.toString(),
      messageId: json['messageId']?.toString(),
      messageSeq: json['messageSeq'] is int ? json['messageSeq'] : null,
      payload: json['payload']?.toString(),
      readSeq: json['readSeq'] is int ? json['readSeq'] : null,
      summary: json['summary']?.toString(),
      occurredAt: (() {
        final value = json['occurredAt']?.toString();
        if (value == null) {
          throw FormatException('DeviceSyncFeedEntry.occurredAt is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'tenantId': tenantId,
      'principalId': principalId,
      'principalKind': principalKind,
      'deviceId': deviceId,
      'syncSeq': syncSeq,
      'eventId': eventId,
      'originEventType': originEventType,
      'actorId': actorId,
      'conversationId': conversationId,
      'messageId': messageId,
      'messageSeq': messageSeq,
      'payload': payload,
      'readSeq': readSeq,
      'summary': summary,
      'occurredAt': occurredAt,
    };
  }
}

class DeviceSyncFeedResponse {
  final List<DeviceSyncFeedEntry> items;
  final int? nextAfterSeq;
  final bool hasMore;
  final int trimmedThroughSeq;

  DeviceSyncFeedResponse({
    required this.items,
    this.nextAfterSeq,
    required this.hasMore,
    required this.trimmedThroughSeq
  });

  factory DeviceSyncFeedResponse.fromJson(Map<String, dynamic> json) {
    return DeviceSyncFeedResponse(
      items: (() {
        final list = _sdkworkAsList(json['items']);
        if (list == null) {
          throw FormatException('DeviceSyncFeedResponse.items is required');
        }
        return list
            .map((item) => (() {
        final map = _sdkworkAsMap(item);
        return map == null ? null : DeviceSyncFeedEntry.fromJson(map);
      })())
            .whereType<DeviceSyncFeedEntry>()
            .toList();
      })(),
      nextAfterSeq: json['nextAfterSeq'] is int ? json['nextAfterSeq'] : null,
      hasMore: (() {
        final value = json['hasMore'];
        if (value is! bool) {
          throw FormatException('DeviceSyncFeedResponse.hasMore is required');
        }
        return value;
      })(),
      trimmedThroughSeq: (() {
        final value = json['trimmedThroughSeq'];
        if (value is! int) {
          throw FormatException('DeviceSyncFeedResponse.trimmedThroughSeq is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'items': items.map((item) => item.toJson()).toList(),
      'nextAfterSeq': nextAfterSeq,
      'hasMore': hasMore,
      'trimmedThroughSeq': trimmedThroughSeq,
    };
  }
}

class RtcSession {
  final String tenantId;
  final String rtcSessionId;
  final String? conversationId;
  final String? providerPluginId;
  final String? providerSessionId;
  final String rtcMode;
  final String state;
  final String createdAt;
  final String updatedAt;

  RtcSession({
    required this.tenantId,
    required this.rtcSessionId,
    this.conversationId,
    this.providerPluginId,
    this.providerSessionId,
    required this.rtcMode,
    required this.state,
    required this.createdAt,
    required this.updatedAt
  });

  factory RtcSession.fromJson(Map<String, dynamic> json) {
    return RtcSession(
      tenantId: (() {
        final value = json['tenantId']?.toString();
        if (value == null) {
          throw FormatException('RtcSession.tenantId is required');
        }
        return value;
      })(),
      rtcSessionId: (() {
        final value = json['rtcSessionId']?.toString();
        if (value == null) {
          throw FormatException('RtcSession.rtcSessionId is required');
        }
        return value;
      })(),
      conversationId: json['conversationId']?.toString(),
      providerPluginId: json['providerPluginId']?.toString(),
      providerSessionId: json['providerSessionId']?.toString(),
      rtcMode: (() {
        final value = json['rtcMode']?.toString();
        if (value == null) {
          throw FormatException('RtcSession.rtcMode is required');
        }
        return value;
      })(),
      state: (() {
        final value = json['state']?.toString();
        if (value == null) {
          throw FormatException('RtcSession.state is required');
        }
        return value;
      })(),
      createdAt: (() {
        final value = json['createdAt']?.toString();
        if (value == null) {
          throw FormatException('RtcSession.createdAt is required');
        }
        return value;
      })(),
      updatedAt: (() {
        final value = json['updatedAt']?.toString();
        if (value == null) {
          throw FormatException('RtcSession.updatedAt is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'tenantId': tenantId,
      'rtcSessionId': rtcSessionId,
      'conversationId': conversationId,
      'providerPluginId': providerPluginId,
      'providerSessionId': providerSessionId,
      'rtcMode': rtcMode,
      'state': state,
      'createdAt': createdAt,
      'updatedAt': updatedAt,
    };
  }
}

class CreateRtcSessionRequest {
  final String? conversationId;
  final String? mediaKind;

  CreateRtcSessionRequest({
    this.conversationId,
    this.mediaKind
  });

  factory CreateRtcSessionRequest.fromJson(Map<String, dynamic> json) {
    return CreateRtcSessionRequest(
      conversationId: json['conversationId']?.toString(),
      mediaKind: json['mediaKind']?.toString()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'conversationId': conversationId,
      'mediaKind': mediaKind,
    };
  }
}

class Sender {
  final String id;
  final String kind;
  final String? principalId;
  final String? principalKind;
  final String? displayName;
  final String? avatarUrl;

  Sender({
    required this.id,
    required this.kind,
    this.principalId,
    this.principalKind,
    this.displayName,
    this.avatarUrl
  });

  factory Sender.fromJson(Map<String, dynamic> json) {
    return Sender(
      id: (() {
        final value = json['id']?.toString();
        if (value == null) {
          throw FormatException('Sender.id is required');
        }
        return value;
      })(),
      kind: (() {
        final value = json['kind']?.toString();
        if (value == null) {
          throw FormatException('Sender.kind is required');
        }
        return value;
      })(),
      principalId: json['principalId']?.toString(),
      principalKind: json['principalKind']?.toString(),
      displayName: json['displayName']?.toString(),
      avatarUrl: json['avatarUrl']?.toString()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'id': id,
      'kind': kind,
      'principalId': principalId,
      'principalKind': principalKind,
      'displayName': displayName,
      'avatarUrl': avatarUrl,
    };
  }
}

class MessageReplyReference {
  final String messageId;
  final String senderDisplayName;
  final String contentPreview;

  MessageReplyReference({
    required this.messageId,
    required this.senderDisplayName,
    required this.contentPreview
  });

  factory MessageReplyReference.fromJson(Map<String, dynamic> json) {
    return MessageReplyReference(
      messageId: (() {
        final value = json['messageId']?.toString();
        if (value == null) {
          throw FormatException('MessageReplyReference.messageId is required');
        }
        return value;
      })(),
      senderDisplayName: (() {
        final value = json['senderDisplayName']?.toString();
        if (value == null) {
          throw FormatException('MessageReplyReference.senderDisplayName is required');
        }
        return value;
      })(),
      contentPreview: (() {
        final value = json['contentPreview']?.toString();
        if (value == null) {
          throw FormatException('MessageReplyReference.contentPreview is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'messageId': messageId,
      'senderDisplayName': senderDisplayName,
      'contentPreview': contentPreview,
    };
  }
}

class DriveReference {
  final String driveUri;
  final String spaceId;
  final String nodeId;
  final String? nodeVersion;

  DriveReference({
    required this.driveUri,
    required this.spaceId,
    required this.nodeId,
    this.nodeVersion
  });

  factory DriveReference.fromJson(Map<String, dynamic> json) {
    return DriveReference(
      driveUri: (() {
        final value = json['driveUri']?.toString();
        if (value == null) {
          throw FormatException('DriveReference.driveUri is required');
        }
        return value;
      })(),
      spaceId: (() {
        final value = json['spaceId']?.toString();
        if (value == null) {
          throw FormatException('DriveReference.spaceId is required');
        }
        return value;
      })(),
      nodeId: (() {
        final value = json['nodeId']?.toString();
        if (value == null) {
          throw FormatException('DriveReference.nodeId is required');
        }
        return value;
      })(),
      nodeVersion: json['nodeVersion']?.toString()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'driveUri': driveUri,
      'spaceId': spaceId,
      'nodeId': nodeId,
      'nodeVersion': nodeVersion,
    };
  }
}

class MediaResource {
  final String? id;
  final String? kind;
  final String? mediaKind;
  final String source;
  final String uri;
  final String? publicUrl;
  final String? url;
  final String? name;
  final String? title;
  final String? fileName;
  final String? mimeType;
  final int? size;
  final String? sizeBytes;
  final String? fileSize;
  final int? durationSeconds;
  final MediaResource? poster;
  final List<MediaResource>? thumbnails;

  MediaResource({
    this.id,
    this.kind,
    this.mediaKind,
    required this.source,
    required this.uri,
    this.publicUrl,
    this.url,
    this.name,
    this.title,
    this.fileName,
    this.mimeType,
    this.size,
    this.sizeBytes,
    this.fileSize,
    this.durationSeconds,
    this.poster,
    this.thumbnails
  });

  factory MediaResource.fromJson(Map<String, dynamic> json) {
    return MediaResource(
      id: json['id']?.toString(),
      kind: json['kind']?.toString(),
      mediaKind: json['mediaKind']?.toString(),
      source: (() {
        final value = json['source']?.toString();
        if (value == null) {
          throw FormatException('MediaResource.source is required');
        }
        return value;
      })(),
      uri: (() {
        final value = json['uri']?.toString();
        if (value == null) {
          throw FormatException('MediaResource.uri is required');
        }
        return value;
      })(),
      publicUrl: json['publicUrl']?.toString(),
      url: json['url']?.toString(),
      name: json['name']?.toString(),
      title: json['title']?.toString(),
      fileName: json['fileName']?.toString(),
      mimeType: json['mimeType']?.toString(),
      size: json['size'] is int ? json['size'] : null,
      sizeBytes: json['sizeBytes']?.toString(),
      fileSize: json['fileSize']?.toString(),
      durationSeconds: json['durationSeconds'] is int ? json['durationSeconds'] : null,
      poster: (() {
        final map = _sdkworkAsMap(json['poster']);
        return map == null ? null : MediaResource.fromJson(map);
      })(),
      thumbnails: (() {
        final list = _sdkworkAsList(json['thumbnails']);
        if (list == null) {
          return null;
        }
        return list
            .map((item) => (() {
        final map = _sdkworkAsMap(item);
        return map == null ? null : MediaResource.fromJson(map);
      })())
            .whereType<MediaResource>()
            .toList();
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'id': id,
      'kind': kind,
      'mediaKind': mediaKind,
      'source': source,
      'uri': uri,
      'publicUrl': publicUrl,
      'url': url,
      'name': name,
      'title': title,
      'fileName': fileName,
      'mimeType': mimeType,
      'size': size,
      'sizeBytes': sizeBytes,
      'fileSize': fileSize,
      'durationSeconds': durationSeconds,
      'poster': poster?.toJson(),
      'thumbnails': thumbnails?.map((item) => item.toJson()).toList(),
    };
  }
}

abstract class ContentPart {
  const ContentPart();

  factory ContentPart.fromJson(Map<String, dynamic> json) {
    switch (json['kind']?.toString()) {
      case 'text':
        return TextContentPart.fromJson(json);
      case 'data':
        return DataContentPart.fromJson(json);
      case 'media':
        return MediaContentPart.fromJson(json);
      case 'signal':
        return SignalContentPart.fromJson(json);
      case 'stream_ref':
        return StreamRefContentPart.fromJson(json);
      default:
        return UnknownContentPart(json);
    }
  }

  Map<String, dynamic> toJson();
}

class UnknownContentPart implements ContentPart {
  final Map<String, dynamic> raw;

  const UnknownContentPart(this.raw);

  @override
  Map<String, dynamic> toJson() {
    return raw;
  }
}

class MessageBody {
  final String? text;
  final List<ContentPart> parts;
  final MessageReplyReference? replyTo;
  final Map<String, dynamic>? renderHints;
  final String? summary;
  final Map<String, dynamic>? metadata;

  MessageBody({
    this.text,
    required this.parts,
    this.replyTo,
    this.renderHints,
    this.summary,
    this.metadata
  });

  factory MessageBody.fromJson(Map<String, dynamic> json) {
    return MessageBody(
      text: json['text']?.toString(),
      parts: (() {
        final list = _sdkworkAsList(json['parts']);
        if (list == null) {
          throw FormatException('MessageBody.parts is required');
        }
        return list
            .map((item) => (() {
        final map = _sdkworkAsMap(item);
        return map == null ? null : ContentPart.fromJson(map);
      })())
            .whereType<ContentPart>()
            .toList();
      })(),
      replyTo: (() {
        final map = _sdkworkAsMap(json['replyTo']);
        return map == null ? null : MessageReplyReference.fromJson(map);
      })(),
      renderHints: _sdkworkAsMap(json['renderHints']),
      summary: json['summary']?.toString(),
      metadata: _sdkworkAsMap(json['metadata'])
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'text': text,
      'parts': parts.map((item) => item.toJson()).toList(),
      'replyTo': replyTo?.toJson(),
      'renderHints': renderHints,
      'summary': summary,
      'metadata': metadata,
    };
  }
}

class TimelineViewEntry {
  final String tenantId;
  final String conversationId;
  final String messageId;
  final int messageSeq;
  final String? summary;
  final Sender sender;
  final MessageBody body;
  final String messageType;
  final String deliveryMode;
  final String? clientMsgId;
  final String? streamSessionId;
  final String? rtcSessionId;
  final String occurredAt;
  final String? committedAt;

  TimelineViewEntry({
    required this.tenantId,
    required this.conversationId,
    required this.messageId,
    required this.messageSeq,
    this.summary,
    required this.sender,
    required this.body,
    required this.messageType,
    required this.deliveryMode,
    this.clientMsgId,
    this.streamSessionId,
    this.rtcSessionId,
    required this.occurredAt,
    this.committedAt
  });

  factory TimelineViewEntry.fromJson(Map<String, dynamic> json) {
    return TimelineViewEntry(
      tenantId: (() {
        final value = json['tenantId']?.toString();
        if (value == null) {
          throw FormatException('TimelineViewEntry.tenantId is required');
        }
        return value;
      })(),
      conversationId: (() {
        final value = json['conversationId']?.toString();
        if (value == null) {
          throw FormatException('TimelineViewEntry.conversationId is required');
        }
        return value;
      })(),
      messageId: (() {
        final value = json['messageId']?.toString();
        if (value == null) {
          throw FormatException('TimelineViewEntry.messageId is required');
        }
        return value;
      })(),
      messageSeq: (() {
        final value = json['messageSeq'];
        if (value is! int) {
          throw FormatException('TimelineViewEntry.messageSeq is required');
        }
        return value;
      })(),
      summary: json['summary']?.toString(),
      sender: (() {
        final map = _sdkworkAsMap(json['sender']);
        if (map == null) {
          throw FormatException('TimelineViewEntry.sender is required');
        }
        return Sender.fromJson(map);
      })(),
      body: (() {
        final map = _sdkworkAsMap(json['body']);
        if (map == null) {
          throw FormatException('TimelineViewEntry.body is required');
        }
        return MessageBody.fromJson(map);
      })(),
      messageType: (() {
        final value = json['messageType']?.toString();
        if (value == null) {
          throw FormatException('TimelineViewEntry.messageType is required');
        }
        return value;
      })(),
      deliveryMode: (() {
        final value = json['deliveryMode']?.toString();
        if (value == null) {
          throw FormatException('TimelineViewEntry.deliveryMode is required');
        }
        return value;
      })(),
      clientMsgId: json['clientMsgId']?.toString(),
      streamSessionId: json['streamSessionId']?.toString(),
      rtcSessionId: json['rtcSessionId']?.toString(),
      occurredAt: (() {
        final value = json['occurredAt']?.toString();
        if (value == null) {
          throw FormatException('TimelineViewEntry.occurredAt is required');
        }
        return value;
      })(),
      committedAt: json['committedAt']?.toString()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'tenantId': tenantId,
      'conversationId': conversationId,
      'messageId': messageId,
      'messageSeq': messageSeq,
      'summary': summary,
      'sender': sender.toJson(),
      'body': body.toJson(),
      'messageType': messageType,
      'deliveryMode': deliveryMode,
      'clientMsgId': clientMsgId,
      'streamSessionId': streamSessionId,
      'rtcSessionId': rtcSessionId,
      'occurredAt': occurredAt,
      'committedAt': committedAt,
    };
  }
}

class TimelineResponse {
  final List<TimelineViewEntry> items;
  final int? nextAfterSeq;
  final bool hasMore;

  TimelineResponse({
    required this.items,
    this.nextAfterSeq,
    required this.hasMore
  });

  factory TimelineResponse.fromJson(Map<String, dynamic> json) {
    return TimelineResponse(
      items: (() {
        final list = _sdkworkAsList(json['items']);
        if (list == null) {
          throw FormatException('TimelineResponse.items is required');
        }
        return list
            .map((item) => (() {
        final map = _sdkworkAsMap(item);
        return map == null ? null : TimelineViewEntry.fromJson(map);
      })())
            .whereType<TimelineViewEntry>()
            .toList();
      })(),
      nextAfterSeq: json['nextAfterSeq'] is int ? json['nextAfterSeq'] : null,
      hasMore: (() {
        final value = json['hasMore'];
        if (value is! bool) {
          throw FormatException('TimelineResponse.hasMore is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'items': items.map((item) => item.toJson()).toList(),
      'nextAfterSeq': nextAfterSeq,
      'hasMore': hasMore,
    };
  }
}

class PostMessageRequest {
  final String? text;
  final List<ContentPart>? parts;
  final MessageReplyReference? replyTo;
  final String? clientMsgId;
  final String? summary;
  final Map<String, dynamic>? renderHints;

  PostMessageRequest({
    this.text,
    this.parts,
    this.replyTo,
    this.clientMsgId,
    this.summary,
    this.renderHints
  });

  factory PostMessageRequest.fromJson(Map<String, dynamic> json) {
    return PostMessageRequest(
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
      replyTo: (() {
        final map = _sdkworkAsMap(json['replyTo']);
        return map == null ? null : MessageReplyReference.fromJson(map);
      })(),
      clientMsgId: json['clientMsgId']?.toString(),
      summary: json['summary']?.toString(),
      renderHints: _sdkworkAsMap(json['renderHints'])
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'text': text,
      'parts': parts?.map((item) => item.toJson()).toList(),
      'replyTo': replyTo?.toJson(),
      'clientMsgId': clientMsgId,
      'summary': summary,
      'renderHints': renderHints,
    };
  }
}

class EditMessageRequest {
  final String? text;
  final List<ContentPart>? parts;
  final MessageReplyReference? replyTo;

  EditMessageRequest({
    this.text,
    this.parts,
    this.replyTo
  });

  factory EditMessageRequest.fromJson(Map<String, dynamic> json) {
    return EditMessageRequest(
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
      replyTo: (() {
        final map = _sdkworkAsMap(json['replyTo']);
        return map == null ? null : MessageReplyReference.fromJson(map);
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'text': text,
      'parts': parts?.map((item) => item.toJson()).toList(),
      'replyTo': replyTo?.toJson(),
    };
  }
}

class PostedMessageResponse {
  final String conversationId;
  final String messageId;
  final int messageSeq;
  final MessageBody body;
  final String occurredAt;

  PostedMessageResponse({
    required this.conversationId,
    required this.messageId,
    required this.messageSeq,
    required this.body,
    required this.occurredAt
  });

  factory PostedMessageResponse.fromJson(Map<String, dynamic> json) {
    return PostedMessageResponse(
      conversationId: (() {
        final value = json['conversationId']?.toString();
        if (value == null) {
          throw FormatException('PostedMessageResponse.conversationId is required');
        }
        return value;
      })(),
      messageId: (() {
        final value = json['messageId']?.toString();
        if (value == null) {
          throw FormatException('PostedMessageResponse.messageId is required');
        }
        return value;
      })(),
      messageSeq: (() {
        final value = json['messageSeq'];
        if (value is! int) {
          throw FormatException('PostedMessageResponse.messageSeq is required');
        }
        return value;
      })(),
      body: (() {
        final map = _sdkworkAsMap(json['body']);
        if (map == null) {
          throw FormatException('PostedMessageResponse.body is required');
        }
        return MessageBody.fromJson(map);
      })(),
      occurredAt: (() {
        final value = json['occurredAt']?.toString();
        if (value == null) {
          throw FormatException('PostedMessageResponse.occurredAt is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'conversationId': conversationId,
      'messageId': messageId,
      'messageSeq': messageSeq,
      'body': body.toJson(),
      'occurredAt': occurredAt,
    };
  }
}

class MessageReactionRequest {
  final String reactionKey;

  MessageReactionRequest({
    required this.reactionKey
  });

  factory MessageReactionRequest.fromJson(Map<String, dynamic> json) {
    return MessageReactionRequest(
      reactionKey: (() {
        final value = json['reactionKey']?.toString();
        if (value == null) {
          throw FormatException('MessageReactionRequest.reactionKey is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'reactionKey': reactionKey,
    };
  }
}

class MessageReactionCountView {
  final String reactionKey;
  final int count;

  MessageReactionCountView({
    required this.reactionKey,
    required this.count
  });

  factory MessageReactionCountView.fromJson(Map<String, dynamic> json) {
    return MessageReactionCountView(
      reactionKey: (() {
        final value = json['reactionKey']?.toString();
        if (value == null) {
          throw FormatException('MessageReactionCountView.reactionKey is required');
        }
        return value;
      })(),
      count: (() {
        final value = json['count'];
        if (value is! int) {
          throw FormatException('MessageReactionCountView.count is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'reactionKey': reactionKey,
      'count': count,
    };
  }
}

class InteractionActorView {
  final String id;
  final String kind;

  InteractionActorView({
    required this.id,
    required this.kind
  });

  factory InteractionActorView.fromJson(Map<String, dynamic> json) {
    return InteractionActorView(
      id: (() {
        final value = json['id']?.toString();
        if (value == null) {
          throw FormatException('InteractionActorView.id is required');
        }
        return value;
      })(),
      kind: (() {
        final value = json['kind']?.toString();
        if (value == null) {
          throw FormatException('InteractionActorView.kind is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'id': id,
      'kind': kind,
    };
  }
}

class MessagePinView {
  final InteractionActorView pinnedBy;
  final String pinnedAt;

  MessagePinView({
    required this.pinnedBy,
    required this.pinnedAt
  });

  factory MessagePinView.fromJson(Map<String, dynamic> json) {
    return MessagePinView(
      pinnedBy: (() {
        final map = _sdkworkAsMap(json['pinnedBy']);
        if (map == null) {
          throw FormatException('MessagePinView.pinnedBy is required');
        }
        return InteractionActorView.fromJson(map);
      })(),
      pinnedAt: (() {
        final value = json['pinnedAt']?.toString();
        if (value == null) {
          throw FormatException('MessagePinView.pinnedAt is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'pinnedBy': pinnedBy.toJson(),
      'pinnedAt': pinnedAt,
    };
  }
}

class MessageInteractionSummaryView {
  final String tenantId;
  final String conversationId;
  final String messageId;
  final int messageSeq;
  final int totalReactionCount;
  final List<MessageReactionCountView> reactionCounts;
  final MessagePinView? pin;

  MessageInteractionSummaryView({
    required this.tenantId,
    required this.conversationId,
    required this.messageId,
    required this.messageSeq,
    required this.totalReactionCount,
    required this.reactionCounts,
    this.pin
  });

  factory MessageInteractionSummaryView.fromJson(Map<String, dynamic> json) {
    return MessageInteractionSummaryView(
      tenantId: (() {
        final value = json['tenantId']?.toString();
        if (value == null) {
          throw FormatException('MessageInteractionSummaryView.tenantId is required');
        }
        return value;
      })(),
      conversationId: (() {
        final value = json['conversationId']?.toString();
        if (value == null) {
          throw FormatException('MessageInteractionSummaryView.conversationId is required');
        }
        return value;
      })(),
      messageId: (() {
        final value = json['messageId']?.toString();
        if (value == null) {
          throw FormatException('MessageInteractionSummaryView.messageId is required');
        }
        return value;
      })(),
      messageSeq: (() {
        final value = json['messageSeq'];
        if (value is! int) {
          throw FormatException('MessageInteractionSummaryView.messageSeq is required');
        }
        return value;
      })(),
      totalReactionCount: (() {
        final value = json['totalReactionCount'];
        if (value is! int) {
          throw FormatException('MessageInteractionSummaryView.totalReactionCount is required');
        }
        return value;
      })(),
      reactionCounts: (() {
        final list = _sdkworkAsList(json['reactionCounts']);
        if (list == null) {
          throw FormatException('MessageInteractionSummaryView.reactionCounts is required');
        }
        return list
            .map((item) => (() {
        final map = _sdkworkAsMap(item);
        return map == null ? null : MessageReactionCountView.fromJson(map);
      })())
            .whereType<MessageReactionCountView>()
            .toList();
      })(),
      pin: (() {
        final map = _sdkworkAsMap(json['pin']);
        return map == null ? null : MessagePinView.fromJson(map);
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'tenantId': tenantId,
      'conversationId': conversationId,
      'messageId': messageId,
      'messageSeq': messageSeq,
      'totalReactionCount': totalReactionCount,
      'reactionCounts': reactionCounts.map((item) => item.toJson()).toList(),
      'pin': pin?.toJson(),
    };
  }
}

class MessageReactionMutationResult {
  final String tenantId;
  final String conversationId;
  final String messageId;
  final String reactionKey;
  final int count;
  final String updatedAt;

  MessageReactionMutationResult({
    required this.tenantId,
    required this.conversationId,
    required this.messageId,
    required this.reactionKey,
    required this.count,
    required this.updatedAt
  });

  factory MessageReactionMutationResult.fromJson(Map<String, dynamic> json) {
    return MessageReactionMutationResult(
      tenantId: (() {
        final value = json['tenantId']?.toString();
        if (value == null) {
          throw FormatException('MessageReactionMutationResult.tenantId is required');
        }
        return value;
      })(),
      conversationId: (() {
        final value = json['conversationId']?.toString();
        if (value == null) {
          throw FormatException('MessageReactionMutationResult.conversationId is required');
        }
        return value;
      })(),
      messageId: (() {
        final value = json['messageId']?.toString();
        if (value == null) {
          throw FormatException('MessageReactionMutationResult.messageId is required');
        }
        return value;
      })(),
      reactionKey: (() {
        final value = json['reactionKey']?.toString();
        if (value == null) {
          throw FormatException('MessageReactionMutationResult.reactionKey is required');
        }
        return value;
      })(),
      count: (() {
        final value = json['count'];
        if (value is! int) {
          throw FormatException('MessageReactionMutationResult.count is required');
        }
        return value;
      })(),
      updatedAt: (() {
        final value = json['updatedAt']?.toString();
        if (value == null) {
          throw FormatException('MessageReactionMutationResult.updatedAt is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'tenantId': tenantId,
      'conversationId': conversationId,
      'messageId': messageId,
      'reactionKey': reactionKey,
      'count': count,
      'updatedAt': updatedAt,
    };
  }
}

class MessagePinMutationResult {
  final String tenantId;
  final String conversationId;
  final String messageId;
  final bool isPinned;
  final String updatedAt;

  MessagePinMutationResult({
    required this.tenantId,
    required this.conversationId,
    required this.messageId,
    required this.isPinned,
    required this.updatedAt
  });

  factory MessagePinMutationResult.fromJson(Map<String, dynamic> json) {
    return MessagePinMutationResult(
      tenantId: (() {
        final value = json['tenantId']?.toString();
        if (value == null) {
          throw FormatException('MessagePinMutationResult.tenantId is required');
        }
        return value;
      })(),
      conversationId: (() {
        final value = json['conversationId']?.toString();
        if (value == null) {
          throw FormatException('MessagePinMutationResult.conversationId is required');
        }
        return value;
      })(),
      messageId: (() {
        final value = json['messageId']?.toString();
        if (value == null) {
          throw FormatException('MessagePinMutationResult.messageId is required');
        }
        return value;
      })(),
      isPinned: (() {
        final value = json['isPinned'];
        if (value is! bool) {
          throw FormatException('MessagePinMutationResult.isPinned is required');
        }
        return value;
      })(),
      updatedAt: (() {
        final value = json['updatedAt']?.toString();
        if (value == null) {
          throw FormatException('MessagePinMutationResult.updatedAt is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'tenantId': tenantId,
      'conversationId': conversationId,
      'messageId': messageId,
      'isPinned': isPinned,
      'updatedAt': updatedAt,
    };
  }
}

class MessageVisibilityMutationResult {
  final String tenantId;
  final String conversationId;
  final String messageId;
  final int messageSeq;
  final String principalKind;
  final String principalId;
  final bool isDeleted;
  final String updatedAt;

  MessageVisibilityMutationResult({
    required this.tenantId,
    required this.conversationId,
    required this.messageId,
    required this.messageSeq,
    required this.principalKind,
    required this.principalId,
    required this.isDeleted,
    required this.updatedAt
  });

  factory MessageVisibilityMutationResult.fromJson(Map<String, dynamic> json) {
    return MessageVisibilityMutationResult(
      tenantId: (() {
        final value = json['tenantId']?.toString();
        if (value == null) {
          throw FormatException('MessageVisibilityMutationResult.tenantId is required');
        }
        return value;
      })(),
      conversationId: (() {
        final value = json['conversationId']?.toString();
        if (value == null) {
          throw FormatException('MessageVisibilityMutationResult.conversationId is required');
        }
        return value;
      })(),
      messageId: (() {
        final value = json['messageId']?.toString();
        if (value == null) {
          throw FormatException('MessageVisibilityMutationResult.messageId is required');
        }
        return value;
      })(),
      messageSeq: (() {
        final value = json['messageSeq'];
        if (value is! int) {
          throw FormatException('MessageVisibilityMutationResult.messageSeq is required');
        }
        return value;
      })(),
      principalKind: (() {
        final value = json['principalKind']?.toString();
        if (value == null) {
          throw FormatException('MessageVisibilityMutationResult.principalKind is required');
        }
        return value;
      })(),
      principalId: (() {
        final value = json['principalId']?.toString();
        if (value == null) {
          throw FormatException('MessageVisibilityMutationResult.principalId is required');
        }
        return value;
      })(),
      isDeleted: (() {
        final value = json['isDeleted'];
        if (value is! bool) {
          throw FormatException('MessageVisibilityMutationResult.isDeleted is required');
        }
        return value;
      })(),
      updatedAt: (() {
        final value = json['updatedAt']?.toString();
        if (value == null) {
          throw FormatException('MessageVisibilityMutationResult.updatedAt is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'tenantId': tenantId,
      'conversationId': conversationId,
      'messageId': messageId,
      'messageSeq': messageSeq,
      'principalKind': principalKind,
      'principalId': principalId,
      'isDeleted': isDeleted,
      'updatedAt': updatedAt,
    };
  }
}

class FavoriteMessageRequest {
  final String conversationId;
  final String favoriteType;
  final String title;
  final String contentPreview;
  final String sourceDisplayName;

  FavoriteMessageRequest({
    required this.conversationId,
    required this.favoriteType,
    required this.title,
    required this.contentPreview,
    required this.sourceDisplayName
  });

  factory FavoriteMessageRequest.fromJson(Map<String, dynamic> json) {
    return FavoriteMessageRequest(
      conversationId: (() {
        final value = json['conversationId']?.toString();
        if (value == null) {
          throw FormatException('FavoriteMessageRequest.conversationId is required');
        }
        return value;
      })(),
      favoriteType: (() {
        final value = json['favoriteType']?.toString();
        if (value == null) {
          throw FormatException('FavoriteMessageRequest.favoriteType is required');
        }
        return value;
      })(),
      title: (() {
        final value = json['title']?.toString();
        if (value == null) {
          throw FormatException('FavoriteMessageRequest.title is required');
        }
        return value;
      })(),
      contentPreview: (() {
        final value = json['contentPreview']?.toString();
        if (value == null) {
          throw FormatException('FavoriteMessageRequest.contentPreview is required');
        }
        return value;
      })(),
      sourceDisplayName: (() {
        final value = json['sourceDisplayName']?.toString();
        if (value == null) {
          throw FormatException('FavoriteMessageRequest.sourceDisplayName is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'conversationId': conversationId,
      'favoriteType': favoriteType,
      'title': title,
      'contentPreview': contentPreview,
      'sourceDisplayName': sourceDisplayName,
    };
  }
}

class MessageFavoriteView {
  final String tenantId;
  final String principalKind;
  final String principalId;
  final String favoriteId;
  final String favoriteType;
  final String conversationId;
  final String messageId;
  final int messageSeq;
  final String title;
  final String contentPreview;
  final String sourceDisplayName;
  final String favoritedAt;

  MessageFavoriteView({
    required this.tenantId,
    required this.principalKind,
    required this.principalId,
    required this.favoriteId,
    required this.favoriteType,
    required this.conversationId,
    required this.messageId,
    required this.messageSeq,
    required this.title,
    required this.contentPreview,
    required this.sourceDisplayName,
    required this.favoritedAt
  });

  factory MessageFavoriteView.fromJson(Map<String, dynamic> json) {
    return MessageFavoriteView(
      tenantId: (() {
        final value = json['tenantId']?.toString();
        if (value == null) {
          throw FormatException('MessageFavoriteView.tenantId is required');
        }
        return value;
      })(),
      principalKind: (() {
        final value = json['principalKind']?.toString();
        if (value == null) {
          throw FormatException('MessageFavoriteView.principalKind is required');
        }
        return value;
      })(),
      principalId: (() {
        final value = json['principalId']?.toString();
        if (value == null) {
          throw FormatException('MessageFavoriteView.principalId is required');
        }
        return value;
      })(),
      favoriteId: (() {
        final value = json['favoriteId']?.toString();
        if (value == null) {
          throw FormatException('MessageFavoriteView.favoriteId is required');
        }
        return value;
      })(),
      favoriteType: (() {
        final value = json['favoriteType']?.toString();
        if (value == null) {
          throw FormatException('MessageFavoriteView.favoriteType is required');
        }
        return value;
      })(),
      conversationId: (() {
        final value = json['conversationId']?.toString();
        if (value == null) {
          throw FormatException('MessageFavoriteView.conversationId is required');
        }
        return value;
      })(),
      messageId: (() {
        final value = json['messageId']?.toString();
        if (value == null) {
          throw FormatException('MessageFavoriteView.messageId is required');
        }
        return value;
      })(),
      messageSeq: (() {
        final value = json['messageSeq'];
        if (value is! int) {
          throw FormatException('MessageFavoriteView.messageSeq is required');
        }
        return value;
      })(),
      title: (() {
        final value = json['title']?.toString();
        if (value == null) {
          throw FormatException('MessageFavoriteView.title is required');
        }
        return value;
      })(),
      contentPreview: (() {
        final value = json['contentPreview']?.toString();
        if (value == null) {
          throw FormatException('MessageFavoriteView.contentPreview is required');
        }
        return value;
      })(),
      sourceDisplayName: (() {
        final value = json['sourceDisplayName']?.toString();
        if (value == null) {
          throw FormatException('MessageFavoriteView.sourceDisplayName is required');
        }
        return value;
      })(),
      favoritedAt: (() {
        final value = json['favoritedAt']?.toString();
        if (value == null) {
          throw FormatException('MessageFavoriteView.favoritedAt is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'tenantId': tenantId,
      'principalKind': principalKind,
      'principalId': principalId,
      'favoriteId': favoriteId,
      'favoriteType': favoriteType,
      'conversationId': conversationId,
      'messageId': messageId,
      'messageSeq': messageSeq,
      'title': title,
      'contentPreview': contentPreview,
      'sourceDisplayName': sourceDisplayName,
      'favoritedAt': favoritedAt,
    };
  }
}

class FavoriteMessagesResponse {
  final List<MessageFavoriteView> items;
  final String? nextCursor;
  final bool hasMore;

  FavoriteMessagesResponse({
    required this.items,
    this.nextCursor,
    required this.hasMore
  });

  factory FavoriteMessagesResponse.fromJson(Map<String, dynamic> json) {
    return FavoriteMessagesResponse(
      items: (() {
        final list = _sdkworkAsList(json['items']);
        if (list == null) {
          throw FormatException('FavoriteMessagesResponse.items is required');
        }
        return list
            .map((item) => (() {
        final map = _sdkworkAsMap(item);
        return map == null ? null : MessageFavoriteView.fromJson(map);
      })())
            .whereType<MessageFavoriteView>()
            .toList();
      })(),
      nextCursor: json['nextCursor']?.toString(),
      hasMore: (() {
        final value = json['hasMore'];
        if (value is! bool) {
          throw FormatException('FavoriteMessagesResponse.hasMore is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'items': items.map((item) => item.toJson()).toList(),
      'nextCursor': nextCursor,
      'hasMore': hasMore,
    };
  }
}

class DeleteMessageFavoriteResponse {
  final String favoriteId;
  final bool deleted;

  DeleteMessageFavoriteResponse({
    required this.favoriteId,
    required this.deleted
  });

  factory DeleteMessageFavoriteResponse.fromJson(Map<String, dynamic> json) {
    return DeleteMessageFavoriteResponse(
      favoriteId: (() {
        final value = json['favoriteId']?.toString();
        if (value == null) {
          throw FormatException('DeleteMessageFavoriteResponse.favoriteId is required');
        }
        return value;
      })(),
      deleted: (() {
        final value = json['deleted'];
        if (value is! bool) {
          throw FormatException('DeleteMessageFavoriteResponse.deleted is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'favoriteId': favoriteId,
      'deleted': deleted,
    };
  }
}

class ConversationPreferencesView {
  final String tenantId;
  final String conversationId;
  final String principalKind;
  final String principalId;
  final bool isPinned;
  final bool isMuted;
  final bool isMarkedUnread;
  final bool isHidden;
  final String updatedAt;

  ConversationPreferencesView({
    required this.tenantId,
    required this.conversationId,
    required this.principalKind,
    required this.principalId,
    required this.isPinned,
    required this.isMuted,
    required this.isMarkedUnread,
    required this.isHidden,
    required this.updatedAt
  });

  factory ConversationPreferencesView.fromJson(Map<String, dynamic> json) {
    return ConversationPreferencesView(
      tenantId: (() {
        final value = json['tenantId']?.toString();
        if (value == null) {
          throw FormatException('ConversationPreferencesView.tenantId is required');
        }
        return value;
      })(),
      conversationId: (() {
        final value = json['conversationId']?.toString();
        if (value == null) {
          throw FormatException('ConversationPreferencesView.conversationId is required');
        }
        return value;
      })(),
      principalKind: (() {
        final value = json['principalKind']?.toString();
        if (value == null) {
          throw FormatException('ConversationPreferencesView.principalKind is required');
        }
        return value;
      })(),
      principalId: (() {
        final value = json['principalId']?.toString();
        if (value == null) {
          throw FormatException('ConversationPreferencesView.principalId is required');
        }
        return value;
      })(),
      isPinned: (() {
        final value = json['isPinned'];
        if (value is! bool) {
          throw FormatException('ConversationPreferencesView.isPinned is required');
        }
        return value;
      })(),
      isMuted: (() {
        final value = json['isMuted'];
        if (value is! bool) {
          throw FormatException('ConversationPreferencesView.isMuted is required');
        }
        return value;
      })(),
      isMarkedUnread: (() {
        final value = json['isMarkedUnread'];
        if (value is! bool) {
          throw FormatException('ConversationPreferencesView.isMarkedUnread is required');
        }
        return value;
      })(),
      isHidden: (() {
        final value = json['isHidden'];
        if (value is! bool) {
          throw FormatException('ConversationPreferencesView.isHidden is required');
        }
        return value;
      })(),
      updatedAt: (() {
        final value = json['updatedAt']?.toString();
        if (value == null) {
          throw FormatException('ConversationPreferencesView.updatedAt is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'tenantId': tenantId,
      'conversationId': conversationId,
      'principalKind': principalKind,
      'principalId': principalId,
      'isPinned': isPinned,
      'isMuted': isMuted,
      'isMarkedUnread': isMarkedUnread,
      'isHidden': isHidden,
      'updatedAt': updatedAt,
    };
  }
}

class UpdateConversationPreferencesRequest {
  final bool? isPinned;
  final bool? isMuted;
  final bool? isMarkedUnread;
  final bool? isHidden;

  UpdateConversationPreferencesRequest({
    this.isPinned,
    this.isMuted,
    this.isMarkedUnread,
    this.isHidden
  });

  factory UpdateConversationPreferencesRequest.fromJson(Map<String, dynamic> json) {
    return UpdateConversationPreferencesRequest(
      isPinned: json['isPinned'] is bool ? json['isPinned'] : null,
      isMuted: json['isMuted'] is bool ? json['isMuted'] : null,
      isMarkedUnread: json['isMarkedUnread'] is bool ? json['isMarkedUnread'] : null,
      isHidden: json['isHidden'] is bool ? json['isHidden'] : null
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'isPinned': isPinned,
      'isMuted': isMuted,
      'isMarkedUnread': isMarkedUnread,
      'isHidden': isHidden,
    };
  }
}

class ConversationProfileView {
  final String tenantId;
  final String conversationId;
  final String displayName;
  final String avatarUrl;
  final String notice;
  final String updatedAt;
  final String? updatedByPrincipalKind;
  final String? updatedByPrincipalId;

  ConversationProfileView({
    required this.tenantId,
    required this.conversationId,
    required this.displayName,
    required this.avatarUrl,
    required this.notice,
    required this.updatedAt,
    this.updatedByPrincipalKind,
    this.updatedByPrincipalId
  });

  factory ConversationProfileView.fromJson(Map<String, dynamic> json) {
    return ConversationProfileView(
      tenantId: (() {
        final value = json['tenantId']?.toString();
        if (value == null) {
          throw FormatException('ConversationProfileView.tenantId is required');
        }
        return value;
      })(),
      conversationId: (() {
        final value = json['conversationId']?.toString();
        if (value == null) {
          throw FormatException('ConversationProfileView.conversationId is required');
        }
        return value;
      })(),
      displayName: (() {
        final value = json['displayName']?.toString();
        if (value == null) {
          throw FormatException('ConversationProfileView.displayName is required');
        }
        return value;
      })(),
      avatarUrl: (() {
        final value = json['avatarUrl']?.toString();
        if (value == null) {
          throw FormatException('ConversationProfileView.avatarUrl is required');
        }
        return value;
      })(),
      notice: (() {
        final value = json['notice']?.toString();
        if (value == null) {
          throw FormatException('ConversationProfileView.notice is required');
        }
        return value;
      })(),
      updatedAt: (() {
        final value = json['updatedAt']?.toString();
        if (value == null) {
          throw FormatException('ConversationProfileView.updatedAt is required');
        }
        return value;
      })(),
      updatedByPrincipalKind: json['updatedByPrincipalKind']?.toString(),
      updatedByPrincipalId: json['updatedByPrincipalId']?.toString()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'tenantId': tenantId,
      'conversationId': conversationId,
      'displayName': displayName,
      'avatarUrl': avatarUrl,
      'notice': notice,
      'updatedAt': updatedAt,
      'updatedByPrincipalKind': updatedByPrincipalKind,
      'updatedByPrincipalId': updatedByPrincipalId,
    };
  }
}

class UpdateConversationProfileRequest {
  final String? displayName;
  final String? avatarUrl;
  final String? notice;

  UpdateConversationProfileRequest({
    this.displayName,
    this.avatarUrl,
    this.notice
  });

  factory UpdateConversationProfileRequest.fromJson(Map<String, dynamic> json) {
    return UpdateConversationProfileRequest(
      displayName: json['displayName']?.toString(),
      avatarUrl: json['avatarUrl']?.toString(),
      notice: json['notice']?.toString()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'displayName': displayName,
      'avatarUrl': avatarUrl,
      'notice': notice,
    };
  }
}

class ConversationSummaryView {
  final String tenantId;
  final String conversationId;
  final int messageCount;
  final int lastMessageSeq;
  final String? lastSummary;
  final String? lastMessageAt;

  ConversationSummaryView({
    required this.tenantId,
    required this.conversationId,
    required this.messageCount,
    required this.lastMessageSeq,
    this.lastSummary,
    this.lastMessageAt
  });

  factory ConversationSummaryView.fromJson(Map<String, dynamic> json) {
    return ConversationSummaryView(
      tenantId: (() {
        final value = json['tenantId']?.toString();
        if (value == null) {
          throw FormatException('ConversationSummaryView.tenantId is required');
        }
        return value;
      })(),
      conversationId: (() {
        final value = json['conversationId']?.toString();
        if (value == null) {
          throw FormatException('ConversationSummaryView.conversationId is required');
        }
        return value;
      })(),
      messageCount: (() {
        final value = json['messageCount'];
        if (value is! int) {
          throw FormatException('ConversationSummaryView.messageCount is required');
        }
        return value;
      })(),
      lastMessageSeq: (() {
        final value = json['lastMessageSeq'];
        if (value is! int) {
          throw FormatException('ConversationSummaryView.lastMessageSeq is required');
        }
        return value;
      })(),
      lastSummary: json['lastSummary']?.toString(),
      lastMessageAt: json['lastMessageAt']?.toString()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'tenantId': tenantId,
      'conversationId': conversationId,
      'messageCount': messageCount,
      'lastMessageSeq': lastMessageSeq,
      'lastSummary': lastSummary,
      'lastMessageAt': lastMessageAt,
    };
  }
}

class ConversationInboxEntry {
  final String tenantId;
  final String conversationId;
  final bool? agentHandoff;
  final String conversationType;
  final String lastActivityAt;
  final String? lastMessageId;
  final String? lastSenderId;
  final int messageCount;
  final int lastMessageSeq;
  final String? lastSummary;
  final String? lastMessageAt;
  final int unreadCount;

  ConversationInboxEntry({
    required this.tenantId,
    required this.conversationId,
    this.agentHandoff,
    required this.conversationType,
    required this.lastActivityAt,
    this.lastMessageId,
    this.lastSenderId,
    required this.messageCount,
    required this.lastMessageSeq,
    this.lastSummary,
    this.lastMessageAt,
    required this.unreadCount
  });

  factory ConversationInboxEntry.fromJson(Map<String, dynamic> json) {
    return ConversationInboxEntry(
      tenantId: (() {
        final value = json['tenantId']?.toString();
        if (value == null) {
          throw FormatException('ConversationInboxEntry.tenantId is required');
        }
        return value;
      })(),
      conversationId: (() {
        final value = json['conversationId']?.toString();
        if (value == null) {
          throw FormatException('ConversationInboxEntry.conversationId is required');
        }
        return value;
      })(),
      agentHandoff: json['agentHandoff'] is bool ? json['agentHandoff'] : null,
      conversationType: (() {
        final value = json['conversationType']?.toString();
        if (value == null) {
          throw FormatException('ConversationInboxEntry.conversationType is required');
        }
        return value;
      })(),
      lastActivityAt: (() {
        final value = json['lastActivityAt']?.toString();
        if (value == null) {
          throw FormatException('ConversationInboxEntry.lastActivityAt is required');
        }
        return value;
      })(),
      lastMessageId: json['lastMessageId']?.toString(),
      lastSenderId: json['lastSenderId']?.toString(),
      messageCount: (() {
        final value = json['messageCount'];
        if (value is! int) {
          throw FormatException('ConversationInboxEntry.messageCount is required');
        }
        return value;
      })(),
      lastMessageSeq: (() {
        final value = json['lastMessageSeq'];
        if (value is! int) {
          throw FormatException('ConversationInboxEntry.lastMessageSeq is required');
        }
        return value;
      })(),
      lastSummary: json['lastSummary']?.toString(),
      lastMessageAt: json['lastMessageAt']?.toString(),
      unreadCount: (() {
        final value = json['unreadCount'];
        if (value is! int) {
          throw FormatException('ConversationInboxEntry.unreadCount is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'tenantId': tenantId,
      'conversationId': conversationId,
      'agentHandoff': agentHandoff,
      'conversationType': conversationType,
      'lastActivityAt': lastActivityAt,
      'lastMessageId': lastMessageId,
      'lastSenderId': lastSenderId,
      'messageCount': messageCount,
      'lastMessageSeq': lastMessageSeq,
      'lastSummary': lastSummary,
      'lastMessageAt': lastMessageAt,
      'unreadCount': unreadCount,
    };
  }
}

class InboxResponse {
  final List<ConversationInboxEntry> items;
  final String? nextCursor;
  final bool hasMore;

  InboxResponse({
    required this.items,
    this.nextCursor,
    required this.hasMore
  });

  factory InboxResponse.fromJson(Map<String, dynamic> json) {
    return InboxResponse(
      items: (() {
        final list = _sdkworkAsList(json['items']);
        if (list == null) {
          throw FormatException('InboxResponse.items is required');
        }
        return list
            .map((item) => (() {
        final map = _sdkworkAsMap(item);
        return map == null ? null : ConversationInboxEntry.fromJson(map);
      })())
            .whereType<ConversationInboxEntry>()
            .toList();
      })(),
      nextCursor: json['nextCursor']?.toString(),
      hasMore: (() {
        final value = json['hasMore'];
        if (value is! bool) {
          throw FormatException('InboxResponse.hasMore is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'items': items.map((item) => item.toJson()).toList(),
      'nextCursor': nextCursor,
      'hasMore': hasMore,
    };
  }
}

class ContactView {
  final String tenantId;
  final String ownerUserId;
  final String targetUserId;
  final String contactType;
  final String relationshipState;
  final String friendshipId;
  final String? directChatId;
  final String? conversationId;
  final String establishedAt;
  final String lastInteractionAt;

  ContactView({
    required this.tenantId,
    required this.ownerUserId,
    required this.targetUserId,
    required this.contactType,
    required this.relationshipState,
    required this.friendshipId,
    this.directChatId,
    this.conversationId,
    required this.establishedAt,
    required this.lastInteractionAt
  });

  factory ContactView.fromJson(Map<String, dynamic> json) {
    return ContactView(
      tenantId: (() {
        final value = json['tenantId']?.toString();
        if (value == null) {
          throw FormatException('ContactView.tenantId is required');
        }
        return value;
      })(),
      ownerUserId: (() {
        final value = json['ownerUserId']?.toString();
        if (value == null) {
          throw FormatException('ContactView.ownerUserId is required');
        }
        return value;
      })(),
      targetUserId: (() {
        final value = json['targetUserId']?.toString();
        if (value == null) {
          throw FormatException('ContactView.targetUserId is required');
        }
        return value;
      })(),
      contactType: (() {
        final value = json['contactType']?.toString();
        if (value == null) {
          throw FormatException('ContactView.contactType is required');
        }
        return value;
      })(),
      relationshipState: (() {
        final value = json['relationshipState']?.toString();
        if (value == null) {
          throw FormatException('ContactView.relationshipState is required');
        }
        return value;
      })(),
      friendshipId: (() {
        final value = json['friendshipId']?.toString();
        if (value == null) {
          throw FormatException('ContactView.friendshipId is required');
        }
        return value;
      })(),
      directChatId: json['directChatId']?.toString(),
      conversationId: json['conversationId']?.toString(),
      establishedAt: (() {
        final value = json['establishedAt']?.toString();
        if (value == null) {
          throw FormatException('ContactView.establishedAt is required');
        }
        return value;
      })(),
      lastInteractionAt: (() {
        final value = json['lastInteractionAt']?.toString();
        if (value == null) {
          throw FormatException('ContactView.lastInteractionAt is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'tenantId': tenantId,
      'ownerUserId': ownerUserId,
      'targetUserId': targetUserId,
      'contactType': contactType,
      'relationshipState': relationshipState,
      'friendshipId': friendshipId,
      'directChatId': directChatId,
      'conversationId': conversationId,
      'establishedAt': establishedAt,
      'lastInteractionAt': lastInteractionAt,
    };
  }
}

class ContactsResponse {
  final List<ContactView> items;
  final String? nextCursor;
  final bool hasMore;

  ContactsResponse({
    required this.items,
    this.nextCursor,
    required this.hasMore
  });

  factory ContactsResponse.fromJson(Map<String, dynamic> json) {
    return ContactsResponse(
      items: (() {
        final list = _sdkworkAsList(json['items']);
        if (list == null) {
          throw FormatException('ContactsResponse.items is required');
        }
        return list
            .map((item) => (() {
        final map = _sdkworkAsMap(item);
        return map == null ? null : ContactView.fromJson(map);
      })())
            .whereType<ContactView>()
            .toList();
      })(),
      nextCursor: json['nextCursor']?.toString(),
      hasMore: (() {
        final value = json['hasMore'];
        if (value is! bool) {
          throw FormatException('ContactsResponse.hasMore is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'items': items.map((item) => item.toJson()).toList(),
      'nextCursor': nextCursor,
      'hasMore': hasMore,
    };
  }
}

class ContactPreferencesView {
  final String tenantId;
  final String ownerUserId;
  final String targetUserId;
  final bool isStarred;
  final String remark;
  final bool isBlocked;
  final String updatedAt;

  ContactPreferencesView({
    required this.tenantId,
    required this.ownerUserId,
    required this.targetUserId,
    required this.isStarred,
    required this.remark,
    required this.isBlocked,
    required this.updatedAt
  });

  factory ContactPreferencesView.fromJson(Map<String, dynamic> json) {
    return ContactPreferencesView(
      tenantId: (() {
        final value = json['tenantId']?.toString();
        if (value == null) {
          throw FormatException('ContactPreferencesView.tenantId is required');
        }
        return value;
      })(),
      ownerUserId: (() {
        final value = json['ownerUserId']?.toString();
        if (value == null) {
          throw FormatException('ContactPreferencesView.ownerUserId is required');
        }
        return value;
      })(),
      targetUserId: (() {
        final value = json['targetUserId']?.toString();
        if (value == null) {
          throw FormatException('ContactPreferencesView.targetUserId is required');
        }
        return value;
      })(),
      isStarred: (() {
        final value = json['isStarred'];
        if (value is! bool) {
          throw FormatException('ContactPreferencesView.isStarred is required');
        }
        return value;
      })(),
      remark: (() {
        final value = json['remark']?.toString();
        if (value == null) {
          throw FormatException('ContactPreferencesView.remark is required');
        }
        return value;
      })(),
      isBlocked: (() {
        final value = json['isBlocked'];
        if (value is! bool) {
          throw FormatException('ContactPreferencesView.isBlocked is required');
        }
        return value;
      })(),
      updatedAt: (() {
        final value = json['updatedAt']?.toString();
        if (value == null) {
          throw FormatException('ContactPreferencesView.updatedAt is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'tenantId': tenantId,
      'ownerUserId': ownerUserId,
      'targetUserId': targetUserId,
      'isStarred': isStarred,
      'remark': remark,
      'isBlocked': isBlocked,
      'updatedAt': updatedAt,
    };
  }
}

class UpdateContactPreferencesRequest {
  final bool? isStarred;
  final String? remark;
  final bool? isBlocked;

  UpdateContactPreferencesRequest({
    this.isStarred,
    this.remark,
    this.isBlocked
  });

  factory UpdateContactPreferencesRequest.fromJson(Map<String, dynamic> json) {
    return UpdateContactPreferencesRequest(
      isStarred: json['isStarred'] is bool ? json['isStarred'] : null,
      remark: json['remark']?.toString(),
      isBlocked: json['isBlocked'] is bool ? json['isBlocked'] : null
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'isStarred': isStarred,
      'remark': remark,
      'isBlocked': isBlocked,
    };
  }
}

class ContactTagView {
  final String tenantId;
  final String ownerUserId;
  final String tagId;
  final String name;
  final String color;
  final int count;
  final String bg;
  final String border;
  final String createdAt;
  final String updatedAt;

  ContactTagView({
    required this.tenantId,
    required this.ownerUserId,
    required this.tagId,
    required this.name,
    required this.color,
    required this.count,
    required this.bg,
    required this.border,
    required this.createdAt,
    required this.updatedAt
  });

  factory ContactTagView.fromJson(Map<String, dynamic> json) {
    return ContactTagView(
      tenantId: (() {
        final value = json['tenantId']?.toString();
        if (value == null) {
          throw FormatException('ContactTagView.tenantId is required');
        }
        return value;
      })(),
      ownerUserId: (() {
        final value = json['ownerUserId']?.toString();
        if (value == null) {
          throw FormatException('ContactTagView.ownerUserId is required');
        }
        return value;
      })(),
      tagId: (() {
        final value = json['tagId']?.toString();
        if (value == null) {
          throw FormatException('ContactTagView.tagId is required');
        }
        return value;
      })(),
      name: (() {
        final value = json['name']?.toString();
        if (value == null) {
          throw FormatException('ContactTagView.name is required');
        }
        return value;
      })(),
      color: (() {
        final value = json['color']?.toString();
        if (value == null) {
          throw FormatException('ContactTagView.color is required');
        }
        return value;
      })(),
      count: (() {
        final value = json['count'];
        if (value is! int) {
          throw FormatException('ContactTagView.count is required');
        }
        return value;
      })(),
      bg: (() {
        final value = json['bg']?.toString();
        if (value == null) {
          throw FormatException('ContactTagView.bg is required');
        }
        return value;
      })(),
      border: (() {
        final value = json['border']?.toString();
        if (value == null) {
          throw FormatException('ContactTagView.border is required');
        }
        return value;
      })(),
      createdAt: (() {
        final value = json['createdAt']?.toString();
        if (value == null) {
          throw FormatException('ContactTagView.createdAt is required');
        }
        return value;
      })(),
      updatedAt: (() {
        final value = json['updatedAt']?.toString();
        if (value == null) {
          throw FormatException('ContactTagView.updatedAt is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'tenantId': tenantId,
      'ownerUserId': ownerUserId,
      'tagId': tagId,
      'name': name,
      'color': color,
      'count': count,
      'bg': bg,
      'border': border,
      'createdAt': createdAt,
      'updatedAt': updatedAt,
    };
  }
}

class ContactTagsResponse {
  final List<ContactTagView> items;
  final String? nextCursor;
  final bool hasMore;

  ContactTagsResponse({
    required this.items,
    this.nextCursor,
    required this.hasMore
  });

  factory ContactTagsResponse.fromJson(Map<String, dynamic> json) {
    return ContactTagsResponse(
      items: (() {
        final list = _sdkworkAsList(json['items']);
        if (list == null) {
          throw FormatException('ContactTagsResponse.items is required');
        }
        return list
            .map((item) => (() {
        final map = _sdkworkAsMap(item);
        return map == null ? null : ContactTagView.fromJson(map);
      })())
            .whereType<ContactTagView>()
            .toList();
      })(),
      nextCursor: json['nextCursor']?.toString(),
      hasMore: (() {
        final value = json['hasMore'];
        if (value is! bool) {
          throw FormatException('ContactTagsResponse.hasMore is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'items': items.map((item) => item.toJson()).toList(),
      'nextCursor': nextCursor,
      'hasMore': hasMore,
    };
  }
}

class CreateContactTagRequest {
  final String name;
  final String color;
  final int? count;
  final String? bg;
  final String? border;

  CreateContactTagRequest({
    required this.name,
    required this.color,
    this.count,
    this.bg,
    this.border
  });

  factory CreateContactTagRequest.fromJson(Map<String, dynamic> json) {
    return CreateContactTagRequest(
      name: (() {
        final value = json['name']?.toString();
        if (value == null) {
          throw FormatException('CreateContactTagRequest.name is required');
        }
        return value;
      })(),
      color: (() {
        final value = json['color']?.toString();
        if (value == null) {
          throw FormatException('CreateContactTagRequest.color is required');
        }
        return value;
      })(),
      count: json['count'] is int ? json['count'] : null,
      bg: json['bg']?.toString(),
      border: json['border']?.toString()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'name': name,
      'color': color,
      'count': count,
      'bg': bg,
      'border': border,
    };
  }
}

class UpdateContactTagRequest {
  final String? name;
  final String? color;
  final int? count;
  final String? bg;
  final String? border;

  UpdateContactTagRequest({
    this.name,
    this.color,
    this.count,
    this.bg,
    this.border
  });

  factory UpdateContactTagRequest.fromJson(Map<String, dynamic> json) {
    return UpdateContactTagRequest(
      name: json['name']?.toString(),
      color: json['color']?.toString(),
      count: json['count'] is int ? json['count'] : null,
      bg: json['bg']?.toString(),
      border: json['border']?.toString()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'name': name,
      'color': color,
      'count': count,
      'bg': bg,
      'border': border,
    };
  }
}

class DeleteContactTagResponse {
  final String tagId;
  final bool deleted;

  DeleteContactTagResponse({
    required this.tagId,
    required this.deleted
  });

  factory DeleteContactTagResponse.fromJson(Map<String, dynamic> json) {
    return DeleteContactTagResponse(
      tagId: (() {
        final value = json['tagId']?.toString();
        if (value == null) {
          throw FormatException('DeleteContactTagResponse.tagId is required');
        }
        return value;
      })(),
      deleted: (() {
        final value = json['deleted'];
        if (value is! bool) {
          throw FormatException('DeleteContactTagResponse.deleted is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'tagId': tagId,
      'deleted': deleted,
    };
  }
}

class ContactRecommendationView {
  final String tenantId;
  final String ownerUserId;
  final String targetUserId;
  final String recommendationId;
  final String? targetConversationId;
  final String createdAt;

  ContactRecommendationView({
    required this.tenantId,
    required this.ownerUserId,
    required this.targetUserId,
    required this.recommendationId,
    this.targetConversationId,
    required this.createdAt
  });

  factory ContactRecommendationView.fromJson(Map<String, dynamic> json) {
    return ContactRecommendationView(
      tenantId: (() {
        final value = json['tenantId']?.toString();
        if (value == null) {
          throw FormatException('ContactRecommendationView.tenantId is required');
        }
        return value;
      })(),
      ownerUserId: (() {
        final value = json['ownerUserId']?.toString();
        if (value == null) {
          throw FormatException('ContactRecommendationView.ownerUserId is required');
        }
        return value;
      })(),
      targetUserId: (() {
        final value = json['targetUserId']?.toString();
        if (value == null) {
          throw FormatException('ContactRecommendationView.targetUserId is required');
        }
        return value;
      })(),
      recommendationId: (() {
        final value = json['recommendationId']?.toString();
        if (value == null) {
          throw FormatException('ContactRecommendationView.recommendationId is required');
        }
        return value;
      })(),
      targetConversationId: json['targetConversationId']?.toString(),
      createdAt: (() {
        final value = json['createdAt']?.toString();
        if (value == null) {
          throw FormatException('ContactRecommendationView.createdAt is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'tenantId': tenantId,
      'ownerUserId': ownerUserId,
      'targetUserId': targetUserId,
      'recommendationId': recommendationId,
      'targetConversationId': targetConversationId,
      'createdAt': createdAt,
    };
  }
}

class CreateContactRecommendationRequest {
  final String? targetConversationId;

  CreateContactRecommendationRequest({
    this.targetConversationId
  });

  factory CreateContactRecommendationRequest.fromJson(Map<String, dynamic> json) {
    return CreateContactRecommendationRequest(
      targetConversationId: json['targetConversationId']?.toString()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'targetConversationId': targetConversationId,
    };
  }
}

class SocialUserSearchResult {
  final String tenantId;
  final String userId;
  final String displayName;
  final String relationshipState;
  final String? avatarUrl;
  final String? email;
  final String? phone;
  final Map<String, dynamic>? metadata;

  SocialUserSearchResult({
    required this.tenantId,
    required this.userId,
    required this.displayName,
    required this.relationshipState,
    this.avatarUrl,
    this.email,
    this.phone,
    this.metadata
  });

  factory SocialUserSearchResult.fromJson(Map<String, dynamic> json) {
    return SocialUserSearchResult(
      tenantId: (() {
        final value = json['tenantId']?.toString();
        if (value == null) {
          throw FormatException('SocialUserSearchResult.tenantId is required');
        }
        return value;
      })(),
      userId: (() {
        final value = json['userId']?.toString();
        if (value == null) {
          throw FormatException('SocialUserSearchResult.userId is required');
        }
        return value;
      })(),
      displayName: (() {
        final value = json['displayName']?.toString();
        if (value == null) {
          throw FormatException('SocialUserSearchResult.displayName is required');
        }
        return value;
      })(),
      relationshipState: (() {
        final value = json['relationshipState']?.toString();
        if (value == null) {
          throw FormatException('SocialUserSearchResult.relationshipState is required');
        }
        return value;
      })(),
      avatarUrl: json['avatarUrl']?.toString(),
      email: json['email']?.toString(),
      phone: json['phone']?.toString(),
      metadata: _sdkworkAsMap(json['metadata'])
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'tenantId': tenantId,
      'userId': userId,
      'displayName': displayName,
      'relationshipState': relationshipState,
      'avatarUrl': avatarUrl,
      'email': email,
      'phone': phone,
      'metadata': metadata,
    };
  }
}

class SocialUserSearchResponse {
  final List<SocialUserSearchResult> items;
  final String? nextCursor;
  final bool hasMore;

  SocialUserSearchResponse({
    required this.items,
    this.nextCursor,
    required this.hasMore
  });

  factory SocialUserSearchResponse.fromJson(Map<String, dynamic> json) {
    return SocialUserSearchResponse(
      items: (() {
        final list = _sdkworkAsList(json['items']);
        if (list == null) {
          throw FormatException('SocialUserSearchResponse.items is required');
        }
        return list
            .map((item) => (() {
        final map = _sdkworkAsMap(item);
        return map == null ? null : SocialUserSearchResult.fromJson(map);
      })())
            .whereType<SocialUserSearchResult>()
            .toList();
      })(),
      nextCursor: json['nextCursor']?.toString(),
      hasMore: (() {
        final value = json['hasMore'];
        if (value is! bool) {
          throw FormatException('SocialUserSearchResponse.hasMore is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'items': items.map((item) => item.toJson()).toList(),
      'nextCursor': nextCursor,
      'hasMore': hasMore,
    };
  }
}

class SubmitFriendRequestRequest {
  final String targetUserId;
  final String? requestMessage;

  SubmitFriendRequestRequest({
    required this.targetUserId,
    this.requestMessage
  });

  factory SubmitFriendRequestRequest.fromJson(Map<String, dynamic> json) {
    return SubmitFriendRequestRequest(
      targetUserId: (() {
        final value = json['targetUserId']?.toString();
        if (value == null) {
          throw FormatException('SubmitFriendRequestRequest.targetUserId is required');
        }
        return value;
      })(),
      requestMessage: json['requestMessage']?.toString()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'targetUserId': targetUserId,
      'requestMessage': requestMessage,
    };
  }
}

class FriendRequest {
  final String tenantId;
  final String requestId;
  final String requesterUserId;
  final String targetUserId;
  final String status;
  final String? requestMessage;
  final String createdAt;
  final String updatedAt;

  FriendRequest({
    required this.tenantId,
    required this.requestId,
    required this.requesterUserId,
    required this.targetUserId,
    required this.status,
    this.requestMessage,
    required this.createdAt,
    required this.updatedAt
  });

  factory FriendRequest.fromJson(Map<String, dynamic> json) {
    return FriendRequest(
      tenantId: (() {
        final value = json['tenantId']?.toString();
        if (value == null) {
          throw FormatException('FriendRequest.tenantId is required');
        }
        return value;
      })(),
      requestId: (() {
        final value = json['requestId']?.toString();
        if (value == null) {
          throw FormatException('FriendRequest.requestId is required');
        }
        return value;
      })(),
      requesterUserId: (() {
        final value = json['requesterUserId']?.toString();
        if (value == null) {
          throw FormatException('FriendRequest.requesterUserId is required');
        }
        return value;
      })(),
      targetUserId: (() {
        final value = json['targetUserId']?.toString();
        if (value == null) {
          throw FormatException('FriendRequest.targetUserId is required');
        }
        return value;
      })(),
      status: (() {
        final value = json['status']?.toString();
        if (value == null) {
          throw FormatException('FriendRequest.status is required');
        }
        return value;
      })(),
      requestMessage: json['requestMessage']?.toString(),
      createdAt: (() {
        final value = json['createdAt']?.toString();
        if (value == null) {
          throw FormatException('FriendRequest.createdAt is required');
        }
        return value;
      })(),
      updatedAt: (() {
        final value = json['updatedAt']?.toString();
        if (value == null) {
          throw FormatException('FriendRequest.updatedAt is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'tenantId': tenantId,
      'requestId': requestId,
      'requesterUserId': requesterUserId,
      'targetUserId': targetUserId,
      'status': status,
      'requestMessage': requestMessage,
      'createdAt': createdAt,
      'updatedAt': updatedAt,
    };
  }
}

class Friendship {
  final String tenantId;
  final String friendshipId;
  final String initiatorUserId;
  final String leftUserId;
  final String rightUserId;
  final String userHighId;
  final String userLowId;
  final String status;
  final String createdAt;

  Friendship({
    required this.tenantId,
    required this.friendshipId,
    required this.initiatorUserId,
    required this.leftUserId,
    required this.rightUserId,
    required this.userHighId,
    required this.userLowId,
    required this.status,
    required this.createdAt
  });

  factory Friendship.fromJson(Map<String, dynamic> json) {
    return Friendship(
      tenantId: (() {
        final value = json['tenantId']?.toString();
        if (value == null) {
          throw FormatException('Friendship.tenantId is required');
        }
        return value;
      })(),
      friendshipId: (() {
        final value = json['friendshipId']?.toString();
        if (value == null) {
          throw FormatException('Friendship.friendshipId is required');
        }
        return value;
      })(),
      initiatorUserId: (() {
        final value = json['initiatorUserId']?.toString();
        if (value == null) {
          throw FormatException('Friendship.initiatorUserId is required');
        }
        return value;
      })(),
      leftUserId: (() {
        final value = json['leftUserId']?.toString();
        if (value == null) {
          throw FormatException('Friendship.leftUserId is required');
        }
        return value;
      })(),
      rightUserId: (() {
        final value = json['rightUserId']?.toString();
        if (value == null) {
          throw FormatException('Friendship.rightUserId is required');
        }
        return value;
      })(),
      userHighId: (() {
        final value = json['userHighId']?.toString();
        if (value == null) {
          throw FormatException('Friendship.userHighId is required');
        }
        return value;
      })(),
      userLowId: (() {
        final value = json['userLowId']?.toString();
        if (value == null) {
          throw FormatException('Friendship.userLowId is required');
        }
        return value;
      })(),
      status: (() {
        final value = json['status']?.toString();
        if (value == null) {
          throw FormatException('Friendship.status is required');
        }
        return value;
      })(),
      createdAt: (() {
        final value = json['createdAt']?.toString();
        if (value == null) {
          throw FormatException('Friendship.createdAt is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'tenantId': tenantId,
      'friendshipId': friendshipId,
      'initiatorUserId': initiatorUserId,
      'leftUserId': leftUserId,
      'rightUserId': rightUserId,
      'userHighId': userHighId,
      'userLowId': userLowId,
      'status': status,
      'createdAt': createdAt,
    };
  }
}

class DirectChat {
  final String tenantId;
  final String directChatId;
  final String conversationId;
  final String status;

  DirectChat({
    required this.tenantId,
    required this.directChatId,
    required this.conversationId,
    required this.status
  });

  factory DirectChat.fromJson(Map<String, dynamic> json) {
    return DirectChat(
      tenantId: (() {
        final value = json['tenantId']?.toString();
        if (value == null) {
          throw FormatException('DirectChat.tenantId is required');
        }
        return value;
      })(),
      directChatId: (() {
        final value = json['directChatId']?.toString();
        if (value == null) {
          throw FormatException('DirectChat.directChatId is required');
        }
        return value;
      })(),
      conversationId: (() {
        final value = json['conversationId']?.toString();
        if (value == null) {
          throw FormatException('DirectChat.conversationId is required');
        }
        return value;
      })(),
      status: (() {
        final value = json['status']?.toString();
        if (value == null) {
          throw FormatException('DirectChat.status is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'tenantId': tenantId,
      'directChatId': directChatId,
      'conversationId': conversationId,
      'status': status,
    };
  }
}

class SocialFriendRequestMutationResponse {
  final FriendRequest friendRequest;

  SocialFriendRequestMutationResponse({
    required this.friendRequest
  });

  factory SocialFriendRequestMutationResponse.fromJson(Map<String, dynamic> json) {
    return SocialFriendRequestMutationResponse(
      friendRequest: (() {
        final map = _sdkworkAsMap(json['friendRequest']);
        if (map == null) {
          throw FormatException('SocialFriendRequestMutationResponse.friendRequest is required');
        }
        return FriendRequest.fromJson(map);
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'friendRequest': friendRequest.toJson(),
    };
  }
}

class SocialFriendRequestListResponse {
  final List<FriendRequest> items;
  final String? nextCursor;

  SocialFriendRequestListResponse({
    required this.items,
    this.nextCursor
  });

  factory SocialFriendRequestListResponse.fromJson(Map<String, dynamic> json) {
    return SocialFriendRequestListResponse(
      items: (() {
        final list = _sdkworkAsList(json['items']);
        if (list == null) {
          throw FormatException('SocialFriendRequestListResponse.items is required');
        }
        return list
            .map((item) => (() {
        final map = _sdkworkAsMap(item);
        return map == null ? null : FriendRequest.fromJson(map);
      })())
            .whereType<FriendRequest>()
            .toList();
      })(),
      nextCursor: json['nextCursor']?.toString()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'items': items.map((item) => item.toJson()).toList(),
      'nextCursor': nextCursor,
    };
  }
}

class SocialFriendRequestAcceptanceResponse {
  final FriendRequest friendRequest;
  final Friendship friendship;
  final DirectChat directChat;
  final CreateConversationResult conversation;

  SocialFriendRequestAcceptanceResponse({
    required this.friendRequest,
    required this.friendship,
    required this.directChat,
    required this.conversation
  });

  factory SocialFriendRequestAcceptanceResponse.fromJson(Map<String, dynamic> json) {
    return SocialFriendRequestAcceptanceResponse(
      friendRequest: (() {
        final map = _sdkworkAsMap(json['friendRequest']);
        if (map == null) {
          throw FormatException('SocialFriendRequestAcceptanceResponse.friendRequest is required');
        }
        return FriendRequest.fromJson(map);
      })(),
      friendship: (() {
        final map = _sdkworkAsMap(json['friendship']);
        if (map == null) {
          throw FormatException('SocialFriendRequestAcceptanceResponse.friendship is required');
        }
        return Friendship.fromJson(map);
      })(),
      directChat: (() {
        final map = _sdkworkAsMap(json['directChat']);
        if (map == null) {
          throw FormatException('SocialFriendRequestAcceptanceResponse.directChat is required');
        }
        return DirectChat.fromJson(map);
      })(),
      conversation: (() {
        final map = _sdkworkAsMap(json['conversation']);
        if (map == null) {
          throw FormatException('SocialFriendRequestAcceptanceResponse.conversation is required');
        }
        return CreateConversationResult.fromJson(map);
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'friendRequest': friendRequest.toJson(),
      'friendship': friendship.toJson(),
      'directChat': directChat.toJson(),
      'conversation': conversation.toJson(),
    };
  }
}

class SocialFriendshipMutationResponse {
  final Friendship friendship;

  SocialFriendshipMutationResponse({
    required this.friendship
  });

  factory SocialFriendshipMutationResponse.fromJson(Map<String, dynamic> json) {
    return SocialFriendshipMutationResponse(
      friendship: (() {
        final map = _sdkworkAsMap(json['friendship']);
        if (map == null) {
          throw FormatException('SocialFriendshipMutationResponse.friendship is required');
        }
        return Friendship.fromJson(map);
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'friendship': friendship.toJson(),
    };
  }
}

class CreateConversationRequest {
  final String? conversationId;
  final String? conversationType;
  final String? kind;
  final String? title;
  final List<String>? memberIds;

  CreateConversationRequest({
    this.conversationId,
    this.conversationType,
    this.kind,
    this.title,
    this.memberIds
  });

  factory CreateConversationRequest.fromJson(Map<String, dynamic> json) {
    return CreateConversationRequest(
      conversationId: json['conversationId']?.toString(),
      conversationType: json['conversationType']?.toString(),
      kind: json['kind']?.toString(),
      title: json['title']?.toString(),
      memberIds: (() {
        final list = _sdkworkAsList(json['memberIds']);
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
      'conversationId': conversationId,
      'conversationType': conversationType,
      'kind': kind,
      'title': title,
      'memberIds': memberIds?.map((item) => item).toList(),
    };
  }
}

class CreateAgentDialogRequest {
  final String agentId;
  final String? conversationId;
  final String? title;

  CreateAgentDialogRequest({
    required this.agentId,
    this.conversationId,
    this.title
  });

  factory CreateAgentDialogRequest.fromJson(Map<String, dynamic> json) {
    return CreateAgentDialogRequest(
      agentId: (() {
        final value = json['agentId']?.toString();
        if (value == null) {
          throw FormatException('CreateAgentDialogRequest.agentId is required');
        }
        return value;
      })(),
      conversationId: json['conversationId']?.toString(),
      title: json['title']?.toString()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'agentId': agentId,
      'conversationId': conversationId,
      'title': title,
    };
  }
}

class BindDirectChatRequest {
  final String? conversationId;
  final String? directChatId;
  final String? leftActorId;
  final String? leftActorKind;
  final String? rightActorId;
  final String? rightActorKind;
  final String? targetUserId;

  BindDirectChatRequest({
    this.conversationId,
    this.directChatId,
    this.leftActorId,
    this.leftActorKind,
    this.rightActorId,
    this.rightActorKind,
    this.targetUserId
  });

  factory BindDirectChatRequest.fromJson(Map<String, dynamic> json) {
    return BindDirectChatRequest(
      conversationId: json['conversationId']?.toString(),
      directChatId: json['directChatId']?.toString(),
      leftActorId: json['leftActorId']?.toString(),
      leftActorKind: json['leftActorKind']?.toString(),
      rightActorId: json['rightActorId']?.toString(),
      rightActorKind: json['rightActorKind']?.toString(),
      targetUserId: json['targetUserId']?.toString()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'conversationId': conversationId,
      'directChatId': directChatId,
      'leftActorId': leftActorId,
      'leftActorKind': leftActorKind,
      'rightActorId': rightActorId,
      'rightActorKind': rightActorKind,
      'targetUserId': targetUserId,
    };
  }
}

class CreateConversationResult {
  final String tenantId;
  final String conversationId;
  final String kind;
  final String createdAt;

  CreateConversationResult({
    required this.tenantId,
    required this.conversationId,
    required this.kind,
    required this.createdAt
  });

  factory CreateConversationResult.fromJson(Map<String, dynamic> json) {
    return CreateConversationResult(
      tenantId: (() {
        final value = json['tenantId']?.toString();
        if (value == null) {
          throw FormatException('CreateConversationResult.tenantId is required');
        }
        return value;
      })(),
      conversationId: (() {
        final value = json['conversationId']?.toString();
        if (value == null) {
          throw FormatException('CreateConversationResult.conversationId is required');
        }
        return value;
      })(),
      kind: (() {
        final value = json['kind']?.toString();
        if (value == null) {
          throw FormatException('CreateConversationResult.kind is required');
        }
        return value;
      })(),
      createdAt: (() {
        final value = json['createdAt']?.toString();
        if (value == null) {
          throw FormatException('CreateConversationResult.createdAt is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'tenantId': tenantId,
      'conversationId': conversationId,
      'kind': kind,
      'createdAt': createdAt,
    };
  }
}

class AddConversationMemberRequest {
  final String principalId;
  final String principalKind;
  final String role;
  final Map<String, dynamic>? attributes;

  AddConversationMemberRequest({
    required this.principalId,
    required this.principalKind,
    required this.role,
    this.attributes
  });

  factory AddConversationMemberRequest.fromJson(Map<String, dynamic> json) {
    return AddConversationMemberRequest(
      principalId: (() {
        final value = json['principalId']?.toString();
        if (value == null) {
          throw FormatException('AddConversationMemberRequest.principalId is required');
        }
        return value;
      })(),
      principalKind: (() {
        final value = json['principalKind']?.toString();
        if (value == null) {
          throw FormatException('AddConversationMemberRequest.principalKind is required');
        }
        return value;
      })(),
      role: (() {
        final value = json['role']?.toString();
        if (value == null) {
          throw FormatException('AddConversationMemberRequest.role is required');
        }
        return value;
      })(),
      attributes: _sdkworkAsMap(json['attributes'])
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'principalId': principalId,
      'principalKind': principalKind,
      'role': role,
      'attributes': attributes,
    };
  }
}

class RemoveConversationMemberRequest {
  final String memberId;

  RemoveConversationMemberRequest({
    required this.memberId
  });

  factory RemoveConversationMemberRequest.fromJson(Map<String, dynamic> json) {
    return RemoveConversationMemberRequest(
      memberId: (() {
        final value = json['memberId']?.toString();
        if (value == null) {
          throw FormatException('RemoveConversationMemberRequest.memberId is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'memberId': memberId,
    };
  }
}

class TransferConversationOwnerRequest {
  final String memberId;

  TransferConversationOwnerRequest({
    required this.memberId
  });

  factory TransferConversationOwnerRequest.fromJson(Map<String, dynamic> json) {
    return TransferConversationOwnerRequest(
      memberId: (() {
        final value = json['memberId']?.toString();
        if (value == null) {
          throw FormatException('TransferConversationOwnerRequest.memberId is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'memberId': memberId,
    };
  }
}

class ChangeConversationMemberRoleRequest {
  final String memberId;
  final String role;

  ChangeConversationMemberRoleRequest({
    required this.memberId,
    required this.role
  });

  factory ChangeConversationMemberRoleRequest.fromJson(Map<String, dynamic> json) {
    return ChangeConversationMemberRoleRequest(
      memberId: (() {
        final value = json['memberId']?.toString();
        if (value == null) {
          throw FormatException('ChangeConversationMemberRoleRequest.memberId is required');
        }
        return value;
      })(),
      role: (() {
        final value = json['role']?.toString();
        if (value == null) {
          throw FormatException('ChangeConversationMemberRoleRequest.role is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'memberId': memberId,
      'role': role,
    };
  }
}

class ConversationMember {
  final String tenantId;
  final String conversationId;
  final String memberId;
  final String principalId;
  final String principalKind;
  final String role;
  final String state;
  final String joinedAt;

  ConversationMember({
    required this.tenantId,
    required this.conversationId,
    required this.memberId,
    required this.principalId,
    required this.principalKind,
    required this.role,
    required this.state,
    required this.joinedAt
  });

  factory ConversationMember.fromJson(Map<String, dynamic> json) {
    return ConversationMember(
      tenantId: (() {
        final value = json['tenantId']?.toString();
        if (value == null) {
          throw FormatException('ConversationMember.tenantId is required');
        }
        return value;
      })(),
      conversationId: (() {
        final value = json['conversationId']?.toString();
        if (value == null) {
          throw FormatException('ConversationMember.conversationId is required');
        }
        return value;
      })(),
      memberId: (() {
        final value = json['memberId']?.toString();
        if (value == null) {
          throw FormatException('ConversationMember.memberId is required');
        }
        return value;
      })(),
      principalId: (() {
        final value = json['principalId']?.toString();
        if (value == null) {
          throw FormatException('ConversationMember.principalId is required');
        }
        return value;
      })(),
      principalKind: (() {
        final value = json['principalKind']?.toString();
        if (value == null) {
          throw FormatException('ConversationMember.principalKind is required');
        }
        return value;
      })(),
      role: (() {
        final value = json['role']?.toString();
        if (value == null) {
          throw FormatException('ConversationMember.role is required');
        }
        return value;
      })(),
      state: (() {
        final value = json['state']?.toString();
        if (value == null) {
          throw FormatException('ConversationMember.state is required');
        }
        return value;
      })(),
      joinedAt: (() {
        final value = json['joinedAt']?.toString();
        if (value == null) {
          throw FormatException('ConversationMember.joinedAt is required');
        }
        return value;
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
      'joinedAt': joinedAt,
    };
  }
}

class ListMembersResponse {
  final List<ConversationMember> items;
  final String? nextCursor;
  final bool hasMore;

  ListMembersResponse({
    required this.items,
    this.nextCursor,
    required this.hasMore
  });

  factory ListMembersResponse.fromJson(Map<String, dynamic> json) {
    return ListMembersResponse(
      items: (() {
        final list = _sdkworkAsList(json['items']);
        if (list == null) {
          throw FormatException('ListMembersResponse.items is required');
        }
        return list
            .map((item) => (() {
        final map = _sdkworkAsMap(item);
        return map == null ? null : ConversationMember.fromJson(map);
      })())
            .whereType<ConversationMember>()
            .toList();
      })(),
      nextCursor: json['nextCursor']?.toString(),
      hasMore: (() {
        final value = json['hasMore'];
        if (value is! bool) {
          throw FormatException('ListMembersResponse.hasMore is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'items': items.map((item) => item.toJson()).toList(),
      'nextCursor': nextCursor,
      'hasMore': hasMore,
    };
  }
}

class MemberDirectoryResponse {
  final List<ConversationMember> items;

  MemberDirectoryResponse({
    required this.items
  });

  factory MemberDirectoryResponse.fromJson(Map<String, dynamic> json) {
    return MemberDirectoryResponse(
      items: (() {
        final list = _sdkworkAsList(json['items']);
        if (list == null) {
          throw FormatException('MemberDirectoryResponse.items is required');
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
      'items': items.map((item) => item.toJson()).toList(),
    };
  }
}

class ReadCursorView {
  final String tenantId;
  final String conversationId;
  final String principalId;
  final int readSeq;
  final String updatedAt;

  ReadCursorView({
    required this.tenantId,
    required this.conversationId,
    required this.principalId,
    required this.readSeq,
    required this.updatedAt
  });

  factory ReadCursorView.fromJson(Map<String, dynamic> json) {
    return ReadCursorView(
      tenantId: (() {
        final value = json['tenantId']?.toString();
        if (value == null) {
          throw FormatException('ReadCursorView.tenantId is required');
        }
        return value;
      })(),
      conversationId: (() {
        final value = json['conversationId']?.toString();
        if (value == null) {
          throw FormatException('ReadCursorView.conversationId is required');
        }
        return value;
      })(),
      principalId: (() {
        final value = json['principalId']?.toString();
        if (value == null) {
          throw FormatException('ReadCursorView.principalId is required');
        }
        return value;
      })(),
      readSeq: (() {
        final value = json['readSeq'];
        if (value is! int) {
          throw FormatException('ReadCursorView.readSeq is required');
        }
        return value;
      })(),
      updatedAt: (() {
        final value = json['updatedAt']?.toString();
        if (value == null) {
          throw FormatException('ReadCursorView.updatedAt is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'tenantId': tenantId,
      'conversationId': conversationId,
      'principalId': principalId,
      'readSeq': readSeq,
      'updatedAt': updatedAt,
    };
  }
}

class UpdateReadCursorRequest {
  final int readSeq;

  UpdateReadCursorRequest({
    required this.readSeq
  });

  factory UpdateReadCursorRequest.fromJson(Map<String, dynamic> json) {
    return UpdateReadCursorRequest(
      readSeq: (() {
        final value = json['readSeq'];
        if (value is! int) {
          throw FormatException('UpdateReadCursorRequest.readSeq is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'readSeq': readSeq,
    };
  }
}

class PinnedMessagesResponse {
  final List<MessageInteractionSummaryView> items;

  PinnedMessagesResponse({
    required this.items
  });

  factory PinnedMessagesResponse.fromJson(Map<String, dynamic> json) {
    return PinnedMessagesResponse(
      items: (() {
        final list = _sdkworkAsList(json['items']);
        if (list == null) {
          throw FormatException('PinnedMessagesResponse.items is required');
        }
        return list
            .map((item) => (() {
        final map = _sdkworkAsMap(item);
        return map == null ? null : MessageInteractionSummaryView.fromJson(map);
      })())
            .whereType<MessageInteractionSummaryView>()
            .toList();
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'items': items.map((item) => item.toJson()).toList(),
    };
  }
}

class StreamView {
  final String tenantId;
  final String streamId;
  final String state;
  final String openedAt;

  StreamView({
    required this.tenantId,
    required this.streamId,
    required this.state,
    required this.openedAt
  });

  factory StreamView.fromJson(Map<String, dynamic> json) {
    return StreamView(
      tenantId: (() {
        final value = json['tenantId']?.toString();
        if (value == null) {
          throw FormatException('StreamView.tenantId is required');
        }
        return value;
      })(),
      streamId: (() {
        final value = json['streamId']?.toString();
        if (value == null) {
          throw FormatException('StreamView.streamId is required');
        }
        return value;
      })(),
      state: (() {
        final value = json['state']?.toString();
        if (value == null) {
          throw FormatException('StreamView.state is required');
        }
        return value;
      })(),
      openedAt: (() {
        final value = json['openedAt']?.toString();
        if (value == null) {
          throw FormatException('StreamView.openedAt is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'tenantId': tenantId,
      'streamId': streamId,
      'state': state,
      'openedAt': openedAt,
    };
  }
}

class OpenStreamRequest {
  final String streamType;
  final String? conversationId;

  OpenStreamRequest({
    required this.streamType,
    this.conversationId
  });

  factory OpenStreamRequest.fromJson(Map<String, dynamic> json) {
    return OpenStreamRequest(
      streamType: (() {
        final value = json['streamType']?.toString();
        if (value == null) {
          throw FormatException('OpenStreamRequest.streamType is required');
        }
        return value;
      })(),
      conversationId: json['conversationId']?.toString()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'streamType': streamType,
      'conversationId': conversationId,
    };
  }
}

class StreamFrameView {
  final String streamId;
  final int frameSeq;
  final String payload;
  final String createdAt;

  StreamFrameView({
    required this.streamId,
    required this.frameSeq,
    required this.payload,
    required this.createdAt
  });

  factory StreamFrameView.fromJson(Map<String, dynamic> json) {
    return StreamFrameView(
      streamId: (() {
        final value = json['streamId']?.toString();
        if (value == null) {
          throw FormatException('StreamFrameView.streamId is required');
        }
        return value;
      })(),
      frameSeq: (() {
        final value = json['frameSeq'];
        if (value is! int) {
          throw FormatException('StreamFrameView.frameSeq is required');
        }
        return value;
      })(),
      payload: (() {
        final value = json['payload']?.toString();
        if (value == null) {
          throw FormatException('StreamFrameView.payload is required');
        }
        return value;
      })(),
      createdAt: (() {
        final value = json['createdAt']?.toString();
        if (value == null) {
          throw FormatException('StreamFrameView.createdAt is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'streamId': streamId,
      'frameSeq': frameSeq,
      'payload': payload,
      'createdAt': createdAt,
    };
  }
}

class StreamFramesResponse {
  final List<StreamFrameView> items;
  final String? nextCursor;
  final bool hasMore;

  StreamFramesResponse({
    required this.items,
    this.nextCursor,
    required this.hasMore
  });

  factory StreamFramesResponse.fromJson(Map<String, dynamic> json) {
    return StreamFramesResponse(
      items: (() {
        final list = _sdkworkAsList(json['items']);
        if (list == null) {
          throw FormatException('StreamFramesResponse.items is required');
        }
        return list
            .map((item) => (() {
        final map = _sdkworkAsMap(item);
        return map == null ? null : StreamFrameView.fromJson(map);
      })())
            .whereType<StreamFrameView>()
            .toList();
      })(),
      nextCursor: json['nextCursor']?.toString(),
      hasMore: (() {
        final value = json['hasMore'];
        if (value is! bool) {
          throw FormatException('StreamFramesResponse.hasMore is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'items': items.map((item) => item.toJson()).toList(),
      'nextCursor': nextCursor,
      'hasMore': hasMore,
    };
  }
}

class AppendStreamFrameRequest {
  final String payload;

  AppendStreamFrameRequest({
    required this.payload
  });

  factory AppendStreamFrameRequest.fromJson(Map<String, dynamic> json) {
    return AppendStreamFrameRequest(
      payload: (() {
        final value = json['payload']?.toString();
        if (value == null) {
          throw FormatException('AppendStreamFrameRequest.payload is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'payload': payload,
    };
  }
}

class ProblemDetail {
  final String type;
  final String title;
  final int status;
  final String detail;
  final String? code;
  final String? message;
  final String? traceId;
  final bool? retryable;

  ProblemDetail({
    required this.type,
    required this.title,
    required this.status,
    required this.detail,
    this.code,
    this.message,
    this.traceId,
    this.retryable
  });

  factory ProblemDetail.fromJson(Map<String, dynamic> json) {
    return ProblemDetail(
      type: (() {
        final value = json['type']?.toString();
        if (value == null) {
          throw FormatException('ProblemDetail.type is required');
        }
        return value;
      })(),
      title: (() {
        final value = json['title']?.toString();
        if (value == null) {
          throw FormatException('ProblemDetail.title is required');
        }
        return value;
      })(),
      status: (() {
        final value = json['status'];
        if (value is! int) {
          throw FormatException('ProblemDetail.status is required');
        }
        return value;
      })(),
      detail: (() {
        final value = json['detail']?.toString();
        if (value == null) {
          throw FormatException('ProblemDetail.detail is required');
        }
        return value;
      })(),
      code: json['code']?.toString(),
      message: json['message']?.toString(),
      traceId: json['traceId']?.toString(),
      retryable: json['retryable'] is bool ? json['retryable'] : null
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'type': type,
      'title': title,
      'status': status,
      'detail': detail,
      'code': code,
      'message': message,
      'traceId': traceId,
      'retryable': retryable,
    };
  }
}

class TextContentPart implements ContentPart {
  final String kind;
  final String text;

  TextContentPart({
    required this.kind,
    required this.text
  });

  factory TextContentPart.fromJson(Map<String, dynamic> json) {
    return TextContentPart(
      kind: (() {
        final value = json['kind']?.toString();
        if (value == null) {
          throw FormatException('TextContentPart.kind is required');
        }
        return value;
      })(),
      text: (() {
        final value = json['text']?.toString();
        if (value == null) {
          throw FormatException('TextContentPart.text is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'kind': kind,
      'text': text,
    };
  }
}

class DataContentPart implements ContentPart {
  final String kind;
  final String schemaRef;
  final String encoding;
  final String payload;

  DataContentPart({
    required this.kind,
    required this.schemaRef,
    required this.encoding,
    required this.payload
  });

  factory DataContentPart.fromJson(Map<String, dynamic> json) {
    return DataContentPart(
      kind: (() {
        final value = json['kind']?.toString();
        if (value == null) {
          throw FormatException('DataContentPart.kind is required');
        }
        return value;
      })(),
      schemaRef: (() {
        final value = json['schemaRef']?.toString();
        if (value == null) {
          throw FormatException('DataContentPart.schemaRef is required');
        }
        return value;
      })(),
      encoding: (() {
        final value = json['encoding']?.toString();
        if (value == null) {
          throw FormatException('DataContentPart.encoding is required');
        }
        return value;
      })(),
      payload: (() {
        final value = json['payload']?.toString();
        if (value == null) {
          throw FormatException('DataContentPart.payload is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'kind': kind,
      'schemaRef': schemaRef,
      'encoding': encoding,
      'payload': payload,
    };
  }
}

class MediaContentPart implements ContentPart {
  final String kind;
  final DriveReference drive;
  final MediaResource resource;
  final String? mediaRole;

  MediaContentPart({
    required this.kind,
    required this.drive,
    required this.resource,
    this.mediaRole
  });

  factory MediaContentPart.fromJson(Map<String, dynamic> json) {
    return MediaContentPart(
      kind: (() {
        final value = json['kind']?.toString();
        if (value == null) {
          throw FormatException('MediaContentPart.kind is required');
        }
        return value;
      })(),
      drive: (() {
        final map = _sdkworkAsMap(json['drive']);
        if (map == null) {
          throw FormatException('MediaContentPart.drive is required');
        }
        return DriveReference.fromJson(map);
      })(),
      resource: (() {
        final map = _sdkworkAsMap(json['resource']);
        if (map == null) {
          throw FormatException('MediaContentPart.resource is required');
        }
        return MediaResource.fromJson(map);
      })(),
      mediaRole: json['mediaRole']?.toString()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'kind': kind,
      'drive': drive.toJson(),
      'resource': resource.toJson(),
      'mediaRole': mediaRole,
    };
  }
}

class SignalContentPart implements ContentPart {
  final String kind;
  final String signalType;
  final String? schemaRef;
  final String payload;

  SignalContentPart({
    required this.kind,
    required this.signalType,
    this.schemaRef,
    required this.payload
  });

  factory SignalContentPart.fromJson(Map<String, dynamic> json) {
    return SignalContentPart(
      kind: (() {
        final value = json['kind']?.toString();
        if (value == null) {
          throw FormatException('SignalContentPart.kind is required');
        }
        return value;
      })(),
      signalType: (() {
        final value = json['signalType']?.toString();
        if (value == null) {
          throw FormatException('SignalContentPart.signalType is required');
        }
        return value;
      })(),
      schemaRef: json['schemaRef']?.toString(),
      payload: (() {
        final value = json['payload']?.toString();
        if (value == null) {
          throw FormatException('SignalContentPart.payload is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'kind': kind,
      'signalType': signalType,
      'schemaRef': schemaRef,
      'payload': payload,
    };
  }
}

class StreamRefContentPart implements ContentPart {
  final String kind;
  final String streamId;
  final String streamType;
  final String state;

  StreamRefContentPart({
    required this.kind,
    required this.streamId,
    required this.streamType,
    required this.state
  });

  factory StreamRefContentPart.fromJson(Map<String, dynamic> json) {
    return StreamRefContentPart(
      kind: (() {
        final value = json['kind']?.toString();
        if (value == null) {
          throw FormatException('StreamRefContentPart.kind is required');
        }
        return value;
      })(),
      streamId: (() {
        final value = json['streamId']?.toString();
        if (value == null) {
          throw FormatException('StreamRefContentPart.streamId is required');
        }
        return value;
      })(),
      streamType: (() {
        final value = json['streamType']?.toString();
        if (value == null) {
          throw FormatException('StreamRefContentPart.streamType is required');
        }
        return value;
      })(),
      state: (() {
        final value = json['state']?.toString();
        if (value == null) {
          throw FormatException('StreamRefContentPart.state is required');
        }
        return value;
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'kind': kind,
      'streamId': streamId,
      'streamType': streamType,
      'state': state,
    };
  }
}
