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

class ProblemDetail {
  final String? type;
  final String? title;
  final int? status;
  final String? detail;
  final String? code;
  final String? message;
  final String? traceId;
  final bool? retryable;

  ProblemDetail({
    this.type,
    this.title,
    this.status,
    this.detail,
    this.code,
    this.message,
    this.traceId,
    this.retryable
  });

  factory ProblemDetail.fromJson(Map<String, dynamic> json) {
    return ProblemDetail(
      type: json['type']?.toString(),
      title: json['title']?.toString(),
      status: json['status'] is int ? json['status'] : null,
      detail: json['detail']?.toString(),
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

class AgentSubject {
  final String? agentId;
  final String? sessionId;
  final Map<String, String>? metadata;

  AgentSubject({
    this.agentId,
    this.sessionId,
    this.metadata
  });

  factory AgentSubject.fromJson(Map<String, dynamic> json) {
    return AgentSubject(
      agentId: json['agent_id']?.toString(),
      sessionId: json['session_id']?.toString(),
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
      'agent_id': agentId,
      'session_id': sessionId,
      'metadata': metadata?.map((key, item) => MapEntry(key, item)),
    };
  }
}

class AgentToolCall {
  final String? tenantId;
  final String? executionId;
  final String? agentId;
  final String? toolCallId;
  final String? toolName;
  final String? argumentsPayload;
  final String? resultPayload;
  final String? state;
  final String? requestedAt;
  final String? completedAt;

  AgentToolCall({
    this.tenantId,
    this.executionId,
    this.agentId,
    this.toolCallId,
    this.toolName,
    this.argumentsPayload,
    this.resultPayload,
    this.state,
    this.requestedAt,
    this.completedAt
  });

  factory AgentToolCall.fromJson(Map<String, dynamic> json) {
    return AgentToolCall(
      tenantId: json['tenantId']?.toString(),
      executionId: json['executionId']?.toString(),
      agentId: json['agentId']?.toString(),
      toolCallId: json['toolCallId']?.toString(),
      toolName: json['toolName']?.toString(),
      argumentsPayload: json['argumentsPayload']?.toString(),
      resultPayload: json['resultPayload']?.toString(),
      state: json['state']?.toString(),
      requestedAt: json['requestedAt']?.toString(),
      completedAt: json['completedAt']?.toString()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'tenantId': tenantId,
      'executionId': executionId,
      'agentId': agentId,
      'toolCallId': toolCallId,
      'toolName': toolName,
      'argumentsPayload': argumentsPayload,
      'resultPayload': resultPayload,
      'state': state,
      'requestedAt': requestedAt,
      'completedAt': completedAt,
    };
  }
}

class AppendAgentResponseDeltaRequest {
  final int? frameSeq;
  final String? frameType;
  final String? schemaRef;
  final String? encoding;
  final String? payload;
  final Map<String, String>? attributes;

  AppendAgentResponseDeltaRequest({
    this.frameSeq,
    this.frameType,
    this.schemaRef,
    this.encoding,
    this.payload,
    this.attributes
  });

  factory AppendAgentResponseDeltaRequest.fromJson(Map<String, dynamic> json) {
    return AppendAgentResponseDeltaRequest(
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

class AutomationExecution {
  final String? tenantId;
  final String? principalId;
  final String? principalKind;
  final String? executionId;
  final String? triggerType;
  final String? targetKind;
  final String? targetRef;
  final String? inputPayload;
  final String? outputPayload;
  final String? state;
  final int? retryCount;
  final String? requestedAt;
  final String? completedAt;
  final String? failureReason;

  AutomationExecution({
    this.tenantId,
    this.principalId,
    this.principalKind,
    this.executionId,
    this.triggerType,
    this.targetKind,
    this.targetRef,
    this.inputPayload,
    this.outputPayload,
    this.state,
    this.retryCount,
    this.requestedAt,
    this.completedAt,
    this.failureReason
  });

  factory AutomationExecution.fromJson(Map<String, dynamic> json) {
    return AutomationExecution(
      tenantId: json['tenantId']?.toString(),
      principalId: json['principalId']?.toString(),
      principalKind: json['principalKind']?.toString(),
      executionId: json['executionId']?.toString(),
      triggerType: json['triggerType']?.toString(),
      targetKind: json['targetKind']?.toString(),
      targetRef: json['targetRef']?.toString(),
      inputPayload: json['inputPayload']?.toString(),
      outputPayload: json['outputPayload']?.toString(),
      state: json['state']?.toString(),
      retryCount: json['retryCount'] is int ? json['retryCount'] : null,
      requestedAt: json['requestedAt']?.toString(),
      completedAt: json['completedAt']?.toString(),
      failureReason: json['failureReason']?.toString()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'tenantId': tenantId,
      'principalId': principalId,
      'principalKind': principalKind,
      'executionId': executionId,
      'triggerType': triggerType,
      'targetKind': targetKind,
      'targetRef': targetRef,
      'inputPayload': inputPayload,
      'outputPayload': outputPayload,
      'state': state,
      'retryCount': retryCount,
      'requestedAt': requestedAt,
      'completedAt': completedAt,
      'failureReason': failureReason,
    };
  }
}

class AutomationExecutionRequestResponse {
  final String? tenantId;
  final String? principalId;
  final String? principalKind;
  final String? executionId;
  final String? triggerType;
  final String? targetKind;
  final String? targetRef;
  final String? inputPayload;
  final String? outputPayload;
  final String? state;
  final int? retryCount;
  final String? requestedAt;
  final String? completedAt;
  final String? failureReason;
  final String? requestKey;
  final String? deliveryStatus;
  final String? proofVersion;

  AutomationExecutionRequestResponse({
    this.tenantId,
    this.principalId,
    this.principalKind,
    this.executionId,
    this.triggerType,
    this.targetKind,
    this.targetRef,
    this.inputPayload,
    this.outputPayload,
    this.state,
    this.retryCount,
    this.requestedAt,
    this.completedAt,
    this.failureReason,
    this.requestKey,
    this.deliveryStatus,
    this.proofVersion
  });

  factory AutomationExecutionRequestResponse.fromJson(Map<String, dynamic> json) {
    return AutomationExecutionRequestResponse(
      tenantId: json['tenantId']?.toString(),
      principalId: json['principalId']?.toString(),
      principalKind: json['principalKind']?.toString(),
      executionId: json['executionId']?.toString(),
      triggerType: json['triggerType']?.toString(),
      targetKind: json['targetKind']?.toString(),
      targetRef: json['targetRef']?.toString(),
      inputPayload: json['inputPayload']?.toString(),
      outputPayload: json['outputPayload']?.toString(),
      state: json['state']?.toString(),
      retryCount: json['retryCount'] is int ? json['retryCount'] : null,
      requestedAt: json['requestedAt']?.toString(),
      completedAt: json['completedAt']?.toString(),
      failureReason: json['failureReason']?.toString(),
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
      'executionId': executionId,
      'triggerType': triggerType,
      'targetKind': targetKind,
      'targetRef': targetRef,
      'inputPayload': inputPayload,
      'outputPayload': outputPayload,
      'state': state,
      'retryCount': retryCount,
      'requestedAt': requestedAt,
      'completedAt': completedAt,
      'failureReason': failureReason,
      'requestKey': requestKey,
      'deliveryStatus': deliveryStatus,
      'proofVersion': proofVersion,
    };
  }
}

class CompleteAgentResponseRequest {
  final int? frameSeq;
  final String? resultMessageId;

  CompleteAgentResponseRequest({
    this.frameSeq,
    this.resultMessageId
  });

  factory CompleteAgentResponseRequest.fromJson(Map<String, dynamic> json) {
    return CompleteAgentResponseRequest(
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

class CompleteAgentToolCallRequest {
  final String? resultPayload;

  CompleteAgentToolCallRequest({
    this.resultPayload
  });

  factory CompleteAgentToolCallRequest.fromJson(Map<String, dynamic> json) {
    return CompleteAgentToolCallRequest(
      resultPayload: json['resultPayload']?.toString()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'resultPayload': resultPayload,
    };
  }
}

class DeviceTwinView {
  final String? tenantId;
  final String? deviceId;
  final String? desiredStateJson;
  final String? reportedStateJson;
  final String? updatedAt;

  DeviceTwinView({
    this.tenantId,
    this.deviceId,
    this.desiredStateJson,
    this.reportedStateJson,
    this.updatedAt
  });

  factory DeviceTwinView.fromJson(Map<String, dynamic> json) {
    return DeviceTwinView(
      tenantId: json['tenantId']?.toString(),
      deviceId: json['deviceId']?.toString(),
      desiredStateJson: json['desiredStateJson']?.toString(),
      reportedStateJson: json['reportedStateJson']?.toString(),
      updatedAt: json['updatedAt']?.toString()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'tenantId': tenantId,
      'deviceId': deviceId,
      'desiredStateJson': desiredStateJson,
      'reportedStateJson': reportedStateJson,
      'updatedAt': updatedAt,
    };
  }
}

class NotificationTask {
  final String? tenantId;
  final String? notificationId;
  final String? sourceEventId;
  final String? sourceEventType;
  final String? category;
  final String? channel;
  final String? recipientId;
  final String? recipientKind;
  final String? status;
  final String? title;
  final String? body;
  final String? payload;
  final String? requestedAt;
  final String? dispatchedAt;
  final String? failureReason;

  NotificationTask({
    this.tenantId,
    this.notificationId,
    this.sourceEventId,
    this.sourceEventType,
    this.category,
    this.channel,
    this.recipientId,
    this.recipientKind,
    this.status,
    this.title,
    this.body,
    this.payload,
    this.requestedAt,
    this.dispatchedAt,
    this.failureReason
  });

  factory NotificationTask.fromJson(Map<String, dynamic> json) {
    return NotificationTask(
      tenantId: json['tenantId']?.toString(),
      notificationId: json['notificationId']?.toString(),
      sourceEventId: json['sourceEventId']?.toString(),
      sourceEventType: json['sourceEventType']?.toString(),
      category: json['category']?.toString(),
      channel: json['channel']?.toString(),
      recipientId: json['recipientId']?.toString(),
      recipientKind: json['recipientKind']?.toString(),
      status: json['status']?.toString(),
      title: json['title']?.toString(),
      body: json['body']?.toString(),
      payload: json['payload']?.toString(),
      requestedAt: json['requestedAt']?.toString(),
      dispatchedAt: json['dispatchedAt']?.toString(),
      failureReason: json['failureReason']?.toString()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'tenantId': tenantId,
      'notificationId': notificationId,
      'sourceEventId': sourceEventId,
      'sourceEventType': sourceEventType,
      'category': category,
      'channel': channel,
      'recipientId': recipientId,
      'recipientKind': recipientKind,
      'status': status,
      'title': title,
      'body': body,
      'payload': payload,
      'requestedAt': requestedAt,
      'dispatchedAt': dispatchedAt,
      'failureReason': failureReason,
    };
  }
}

class NotificationListResponse {
  final List<NotificationTask>? items;

  NotificationListResponse({
    this.items
  });

  factory NotificationListResponse.fromJson(Map<String, dynamic> json) {
    return NotificationListResponse(
      items: (() {
        final list = _sdkworkAsList(json['items']);
        if (list == null) {
          return null;
        }
        return list
            .map((item) => (() {
        final map = _sdkworkAsMap(item);
        return map == null ? null : NotificationTask.fromJson(map);
      })())
            .whereType<NotificationTask>()
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

class NotificationRequestResponse {
  final String? tenantId;
  final String? notificationId;
  final String? sourceEventId;
  final String? sourceEventType;
  final String? category;
  final String? channel;
  final String? recipientId;
  final String? recipientKind;
  final String? status;
  final String? title;
  final String? body;
  final String? payload;
  final String? requestedAt;
  final String? dispatchedAt;
  final String? failureReason;
  final String? requestKey;
  final String? deliveryStatus;
  final String? proofVersion;

  NotificationRequestResponse({
    this.tenantId,
    this.notificationId,
    this.sourceEventId,
    this.sourceEventType,
    this.category,
    this.channel,
    this.recipientId,
    this.recipientKind,
    this.status,
    this.title,
    this.body,
    this.payload,
    this.requestedAt,
    this.dispatchedAt,
    this.failureReason,
    this.requestKey,
    this.deliveryStatus,
    this.proofVersion
  });

  factory NotificationRequestResponse.fromJson(Map<String, dynamic> json) {
    return NotificationRequestResponse(
      tenantId: json['tenantId']?.toString(),
      notificationId: json['notificationId']?.toString(),
      sourceEventId: json['sourceEventId']?.toString(),
      sourceEventType: json['sourceEventType']?.toString(),
      category: json['category']?.toString(),
      channel: json['channel']?.toString(),
      recipientId: json['recipientId']?.toString(),
      recipientKind: json['recipientKind']?.toString(),
      status: json['status']?.toString(),
      title: json['title']?.toString(),
      body: json['body']?.toString(),
      payload: json['payload']?.toString(),
      requestedAt: json['requestedAt']?.toString(),
      dispatchedAt: json['dispatchedAt']?.toString(),
      failureReason: json['failureReason']?.toString(),
      requestKey: json['requestKey']?.toString(),
      deliveryStatus: json['deliveryStatus']?.toString(),
      proofVersion: json['proofVersion']?.toString()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'tenantId': tenantId,
      'notificationId': notificationId,
      'sourceEventId': sourceEventId,
      'sourceEventType': sourceEventType,
      'category': category,
      'channel': channel,
      'recipientId': recipientId,
      'recipientKind': recipientKind,
      'status': status,
      'title': title,
      'body': body,
      'payload': payload,
      'requestedAt': requestedAt,
      'dispatchedAt': dispatchedAt,
      'failureReason': failureReason,
      'requestKey': requestKey,
      'deliveryStatus': deliveryStatus,
      'proofVersion': proofVersion,
    };
  }
}

class RequestAgentToolCallRequest {
  final String? executionId;
  final String? toolCallId;
  final String? toolName;
  final String? argumentsPayload;

  RequestAgentToolCallRequest({
    this.executionId,
    this.toolCallId,
    this.toolName,
    this.argumentsPayload
  });

  factory RequestAgentToolCallRequest.fromJson(Map<String, dynamic> json) {
    return RequestAgentToolCallRequest(
      executionId: json['executionId']?.toString(),
      toolCallId: json['toolCallId']?.toString(),
      toolName: json['toolName']?.toString(),
      argumentsPayload: json['argumentsPayload']?.toString()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'executionId': executionId,
      'toolCallId': toolCallId,
      'toolName': toolName,
      'argumentsPayload': argumentsPayload,
    };
  }
}

class RequestAutomationExecution {
  final String? executionId;
  final String? triggerType;
  final String? targetKind;
  final String? targetRef;
  final String? inputPayload;

  RequestAutomationExecution({
    this.executionId,
    this.triggerType,
    this.targetKind,
    this.targetRef,
    this.inputPayload
  });

  factory RequestAutomationExecution.fromJson(Map<String, dynamic> json) {
    return RequestAutomationExecution(
      executionId: json['executionId']?.toString(),
      triggerType: json['triggerType']?.toString(),
      targetKind: json['targetKind']?.toString(),
      targetRef: json['targetRef']?.toString(),
      inputPayload: json['inputPayload']?.toString()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'executionId': executionId,
      'triggerType': triggerType,
      'targetKind': targetKind,
      'targetRef': targetRef,
      'inputPayload': inputPayload,
    };
  }
}

class RequestNotification {
  final String? notificationId;
  final String? sourceEventId;
  final String? sourceEventType;
  final String? category;
  final String? channel;
  final String? recipientId;
  final String? recipientKind;
  final String? title;
  final String? body;
  final String? payload;

  RequestNotification({
    this.notificationId,
    this.sourceEventId,
    this.sourceEventType,
    this.category,
    this.channel,
    this.recipientId,
    this.recipientKind,
    this.title,
    this.body,
    this.payload
  });

  factory RequestNotification.fromJson(Map<String, dynamic> json) {
    return RequestNotification(
      notificationId: json['notificationId']?.toString(),
      sourceEventId: json['sourceEventId']?.toString(),
      sourceEventType: json['sourceEventType']?.toString(),
      category: json['category']?.toString(),
      channel: json['channel']?.toString(),
      recipientId: json['recipientId']?.toString(),
      recipientKind: json['recipientKind']?.toString(),
      title: json['title']?.toString(),
      body: json['body']?.toString(),
      payload: json['payload']?.toString()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'notificationId': notificationId,
      'sourceEventId': sourceEventId,
      'sourceEventType': sourceEventType,
      'category': category,
      'channel': channel,
      'recipientId': recipientId,
      'recipientKind': recipientKind,
      'title': title,
      'body': body,
      'payload': payload,
    };
  }
}

class StartAgentResponseRequest {
  final String? executionId;
  final String? streamId;
  final String? streamType;
  final String? conversationId;
  final String? schemaRef;
  final String? memberId;
  final AgentSubject? agent;

  StartAgentResponseRequest({
    this.executionId,
    this.streamId,
    this.streamType,
    this.conversationId,
    this.schemaRef,
    this.memberId,
    this.agent
  });

  factory StartAgentResponseRequest.fromJson(Map<String, dynamic> json) {
    return StartAgentResponseRequest(
      executionId: json['executionId']?.toString(),
      streamId: json['streamId']?.toString(),
      streamType: json['streamType']?.toString(),
      conversationId: json['conversationId']?.toString(),
      schemaRef: json['schemaRef']?.toString(),
      memberId: json['memberId']?.toString(),
      agent: (() {
        final map = _sdkworkAsMap(json['agent']);
        return map == null ? null : AgentSubject.fromJson(map);
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'executionId': executionId,
      'streamId': streamId,
      'streamType': streamType,
      'conversationId': conversationId,
      'schemaRef': schemaRef,
      'memberId': memberId,
      'agent': agent?.toJson(),
    };
  }
}

class UpdateDeviceTwinDesiredRequest {
  final String? desiredStateJson;

  UpdateDeviceTwinDesiredRequest({
    this.desiredStateJson
  });

  factory UpdateDeviceTwinDesiredRequest.fromJson(Map<String, dynamic> json) {
    return UpdateDeviceTwinDesiredRequest(
      desiredStateJson: json['desiredStateJson']?.toString()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'desiredStateJson': desiredStateJson,
    };
  }
}

class UpdateDeviceTwinReportedRequest {
  final String? reportedStateJson;

  UpdateDeviceTwinReportedRequest({
    this.reportedStateJson
  });

  factory UpdateDeviceTwinReportedRequest.fromJson(Map<String, dynamic> json) {
    return UpdateDeviceTwinReportedRequest(
      reportedStateJson: json['reportedStateJson']?.toString()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'reportedStateJson': reportedStateJson,
    };
  }
}
