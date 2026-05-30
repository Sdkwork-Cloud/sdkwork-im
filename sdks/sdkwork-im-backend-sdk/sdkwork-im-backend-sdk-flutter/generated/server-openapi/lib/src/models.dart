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

class ActivateFriendshipRequest {
  final String? directChatId;
  final String? establishedAt;
  final String? eventId;
  final String? friendshipId;
  final String? initiatorUserId;
  final String? peerUserId;

  ActivateFriendshipRequest({
    this.directChatId,
    this.establishedAt,
    this.eventId,
    this.friendshipId,
    this.initiatorUserId,
    this.peerUserId
  });

  factory ActivateFriendshipRequest.fromJson(Map<String, dynamic> json) {
    return ActivateFriendshipRequest(
      directChatId: json['directChatId']?.toString(),
      establishedAt: json['establishedAt']?.toString(),
      eventId: json['eventId']?.toString(),
      friendshipId: json['friendshipId']?.toString(),
      initiatorUserId: json['initiatorUserId']?.toString(),
      peerUserId: json['peerUserId']?.toString()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'directChatId': directChatId,
      'establishedAt': establishedAt,
      'eventId': eventId,
      'friendshipId': friendshipId,
      'initiatorUserId': initiatorUserId,
      'peerUserId': peerUserId,
    };
  }
}

class ApplySharedChannelPolicyRequest {
  final String? appliedAt;
  final String? channelId;
  final String? connectionId;
  final String? conversationId;
  final String? eventId;
  final String? historyVisibility;
  final String? policyId;
  final int? policyVersion;

  ApplySharedChannelPolicyRequest({
    this.appliedAt,
    this.channelId,
    this.connectionId,
    this.conversationId,
    this.eventId,
    this.historyVisibility,
    this.policyId,
    this.policyVersion
  });

  factory ApplySharedChannelPolicyRequest.fromJson(Map<String, dynamic> json) {
    return ApplySharedChannelPolicyRequest(
      appliedAt: json['appliedAt']?.toString(),
      channelId: json['channelId']?.toString(),
      connectionId: json['connectionId']?.toString(),
      conversationId: json['conversationId']?.toString(),
      eventId: json['eventId']?.toString(),
      historyVisibility: json['historyVisibility']?.toString(),
      policyId: json['policyId']?.toString(),
      policyVersion: json['policyVersion'] is int ? json['policyVersion'] : null
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'appliedAt': appliedAt,
      'channelId': channelId,
      'connectionId': connectionId,
      'conversationId': conversationId,
      'eventId': eventId,
      'historyVisibility': historyVisibility,
      'policyId': policyId,
      'policyVersion': policyVersion,
    };
  }
}

class BindDirectChatRequest {
  final String? boundAt;
  final String? conversationId;
  final String? directChatId;
  final String? eventId;
  final String? leftActorId;
  final String? rightActorId;

  BindDirectChatRequest({
    this.boundAt,
    this.conversationId,
    this.directChatId,
    this.eventId,
    this.leftActorId,
    this.rightActorId
  });

  factory BindDirectChatRequest.fromJson(Map<String, dynamic> json) {
    return BindDirectChatRequest(
      boundAt: json['boundAt']?.toString(),
      conversationId: json['conversationId']?.toString(),
      directChatId: json['directChatId']?.toString(),
      eventId: json['eventId']?.toString(),
      leftActorId: json['leftActorId']?.toString(),
      rightActorId: json['rightActorId']?.toString()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'boundAt': boundAt,
      'conversationId': conversationId,
      'directChatId': directChatId,
      'eventId': eventId,
      'leftActorId': leftActorId,
      'rightActorId': rightActorId,
    };
  }
}

class BindExternalMemberLinkRequest {
  final String? connectionId;
  final String? eventId;
  final String? externalDisplayName;
  final String? externalMemberId;
  final String? linkId;
  final String? linkedAt;
  final String? localActorId;
  final String? localActorKind;

  BindExternalMemberLinkRequest({
    this.connectionId,
    this.eventId,
    this.externalDisplayName,
    this.externalMemberId,
    this.linkId,
    this.linkedAt,
    this.localActorId,
    this.localActorKind
  });

  factory BindExternalMemberLinkRequest.fromJson(Map<String, dynamic> json) {
    return BindExternalMemberLinkRequest(
      connectionId: json['connectionId']?.toString(),
      eventId: json['eventId']?.toString(),
      externalDisplayName: json['externalDisplayName']?.toString(),
      externalMemberId: json['externalMemberId']?.toString(),
      linkId: json['linkId']?.toString(),
      linkedAt: json['linkedAt']?.toString(),
      localActorId: json['localActorId']?.toString(),
      localActorKind: json['localActorKind']?.toString()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'connectionId': connectionId,
      'eventId': eventId,
      'externalDisplayName': externalDisplayName,
      'externalMemberId': externalMemberId,
      'linkId': linkId,
      'linkedAt': linkedAt,
      'localActorId': localActorId,
      'localActorKind': localActorKind,
    };
  }
}

class BlockUserRequest {
  final String? blockId;
  final String? blockedUserId;
  final String? blockerUserId;
  final String? directChatId;
  final String? effectiveAt;
  final String? eventId;
  final String? expiresAt;
  final String? scope;

  BlockUserRequest({
    this.blockId,
    this.blockedUserId,
    this.blockerUserId,
    this.directChatId,
    this.effectiveAt,
    this.eventId,
    this.expiresAt,
    this.scope
  });

  factory BlockUserRequest.fromJson(Map<String, dynamic> json) {
    return BlockUserRequest(
      blockId: json['blockId']?.toString(),
      blockedUserId: json['blockedUserId']?.toString(),
      blockerUserId: json['blockerUserId']?.toString(),
      directChatId: json['directChatId']?.toString(),
      effectiveAt: json['effectiveAt']?.toString(),
      eventId: json['eventId']?.toString(),
      expiresAt: json['expiresAt']?.toString(),
      scope: json['scope']?.toString()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'blockId': blockId,
      'blockedUserId': blockedUserId,
      'blockerUserId': blockerUserId,
      'directChatId': directChatId,
      'effectiveAt': effectiveAt,
      'eventId': eventId,
      'expiresAt': expiresAt,
      'scope': scope,
    };
  }
}

class BusinessPolicyVocabularyResponse {
  final String? capabilityFlagsField;
  final String? historyVisibilityField;
  final List<String>? historyVisibilityModes;
  final String? policyVersionField;
  final String? retentionPolicyRefField;
  final List<String>? retentionPolicyScopes;

  BusinessPolicyVocabularyResponse({
    this.capabilityFlagsField,
    this.historyVisibilityField,
    this.historyVisibilityModes,
    this.policyVersionField,
    this.retentionPolicyRefField,
    this.retentionPolicyScopes
  });

  factory BusinessPolicyVocabularyResponse.fromJson(Map<String, dynamic> json) {
    return BusinessPolicyVocabularyResponse(
      capabilityFlagsField: json['capabilityFlagsField']?.toString(),
      historyVisibilityField: json['historyVisibilityField']?.toString(),
      historyVisibilityModes: (() {
        final list = _sdkworkAsList(json['historyVisibilityModes']);
        if (list == null) {
          return null;
        }
        return list
            .map((item) => item?.toString())
            .whereType<String>()
            .toList();
      })(),
      policyVersionField: json['policyVersionField']?.toString(),
      retentionPolicyRefField: json['retentionPolicyRefField']?.toString(),
      retentionPolicyScopes: (() {
        final list = _sdkworkAsList(json['retentionPolicyScopes']);
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
      'capabilityFlagsField': capabilityFlagsField,
      'historyVisibilityField': historyVisibilityField,
      'historyVisibilityModes': historyVisibilityModes?.map((item) => item).toList(),
      'policyVersionField': policyVersionField,
      'retentionPolicyRefField': retentionPolicyRefField,
      'retentionPolicyScopes': retentionPolicyScopes?.map((item) => item).toList(),
    };
  }
}

class CapabilityProfileResponse {
  final List<String>? enabledCapabilities;
  final List<String>? experimentalCapabilities;
  final String? profileId;
  final String? releaseChannel;

  CapabilityProfileResponse({
    this.enabledCapabilities,
    this.experimentalCapabilities,
    this.profileId,
    this.releaseChannel
  });

  factory CapabilityProfileResponse.fromJson(Map<String, dynamic> json) {
    return CapabilityProfileResponse(
      enabledCapabilities: (() {
        final list = _sdkworkAsList(json['enabledCapabilities']);
        if (list == null) {
          return null;
        }
        return list
            .map((item) => item?.toString())
            .whereType<String>()
            .toList();
      })(),
      experimentalCapabilities: (() {
        final list = _sdkworkAsList(json['experimentalCapabilities']);
        if (list == null) {
          return null;
        }
        return list
            .map((item) => item?.toString())
            .whereType<String>()
            .toList();
      })(),
      profileId: json['profileId']?.toString(),
      releaseChannel: json['releaseChannel']?.toString()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'enabledCapabilities': enabledCapabilities?.map((item) => item).toList(),
      'experimentalCapabilities': experimentalCapabilities?.map((item) => item).toList(),
      'profileId': profileId,
      'releaseChannel': releaseChannel,
    };
  }
}

class ClientCompatibilityResponse {
  final List<String>? blockedExperimentalCapabilities;
  final String? clientType;
  final String? minimumProtocolVersion;
  final List<String>? supportedBindings;
  final List<String>? supportedCapabilities;
  final List<String>? supportedCodecs;

  ClientCompatibilityResponse({
    this.blockedExperimentalCapabilities,
    this.clientType,
    this.minimumProtocolVersion,
    this.supportedBindings,
    this.supportedCapabilities,
    this.supportedCodecs
  });

  factory ClientCompatibilityResponse.fromJson(Map<String, dynamic> json) {
    return ClientCompatibilityResponse(
      blockedExperimentalCapabilities: (() {
        final list = _sdkworkAsList(json['blockedExperimentalCapabilities']);
        if (list == null) {
          return null;
        }
        return list
            .map((item) => item?.toString())
            .whereType<String>()
            .toList();
      })(),
      clientType: json['clientType']?.toString(),
      minimumProtocolVersion: json['minimumProtocolVersion']?.toString(),
      supportedBindings: (() {
        final list = _sdkworkAsList(json['supportedBindings']);
        if (list == null) {
          return null;
        }
        return list
            .map((item) => item?.toString())
            .whereType<String>()
            .toList();
      })(),
      supportedCapabilities: (() {
        final list = _sdkworkAsList(json['supportedCapabilities']);
        if (list == null) {
          return null;
        }
        return list
            .map((item) => item?.toString())
            .whereType<String>()
            .toList();
      })(),
      supportedCodecs: (() {
        final list = _sdkworkAsList(json['supportedCodecs']);
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
      'blockedExperimentalCapabilities': blockedExperimentalCapabilities?.map((item) => item).toList(),
      'clientType': clientType,
      'minimumProtocolVersion': minimumProtocolVersion,
      'supportedBindings': supportedBindings?.map((item) => item).toList(),
      'supportedCapabilities': supportedCapabilities?.map((item) => item).toList(),
      'supportedCodecs': supportedCodecs?.map((item) => item).toList(),
    };
  }
}

class EffectiveProtocolSnapshotResponse {
  final List<String>? allowedBindings;
  final List<String>? allowedCodecs;
  final List<String>? enabledCapabilities;
  final bool? killSwitchActive;
  final List<String>? precedence;
  final String? protocolVersion;
  final String? quotaProfileId;
  final String? releaseChannel;

  EffectiveProtocolSnapshotResponse({
    this.allowedBindings,
    this.allowedCodecs,
    this.enabledCapabilities,
    this.killSwitchActive,
    this.precedence,
    this.protocolVersion,
    this.quotaProfileId,
    this.releaseChannel
  });

  factory EffectiveProtocolSnapshotResponse.fromJson(Map<String, dynamic> json) {
    return EffectiveProtocolSnapshotResponse(
      allowedBindings: (() {
        final list = _sdkworkAsList(json['allowedBindings']);
        if (list == null) {
          return null;
        }
        return list
            .map((item) => item?.toString())
            .whereType<String>()
            .toList();
      })(),
      allowedCodecs: (() {
        final list = _sdkworkAsList(json['allowedCodecs']);
        if (list == null) {
          return null;
        }
        return list
            .map((item) => item?.toString())
            .whereType<String>()
            .toList();
      })(),
      enabledCapabilities: (() {
        final list = _sdkworkAsList(json['enabledCapabilities']);
        if (list == null) {
          return null;
        }
        return list
            .map((item) => item?.toString())
            .whereType<String>()
            .toList();
      })(),
      killSwitchActive: json['killSwitchActive'] is bool ? json['killSwitchActive'] : null,
      precedence: (() {
        final list = _sdkworkAsList(json['precedence']);
        if (list == null) {
          return null;
        }
        return list
            .map((item) => item?.toString())
            .whereType<String>()
            .toList();
      })(),
      protocolVersion: json['protocolVersion']?.toString(),
      quotaProfileId: json['quotaProfileId']?.toString(),
      releaseChannel: json['releaseChannel']?.toString()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'allowedBindings': allowedBindings?.map((item) => item).toList(),
      'allowedCodecs': allowedCodecs?.map((item) => item).toList(),
      'enabledCapabilities': enabledCapabilities?.map((item) => item).toList(),
      'killSwitchActive': killSwitchActive,
      'precedence': precedence?.map((item) => item).toList(),
      'protocolVersion': protocolVersion,
      'quotaProfileId': quotaProfileId,
      'releaseChannel': releaseChannel,
    };
  }
}

class EstablishExternalConnectionRequest {
  final String? connectionId;
  final String? connectionKind;
  final String? establishedAt;
  final String? eventId;
  final String? externalOrgName;
  final String? externalTenantId;

  EstablishExternalConnectionRequest({
    this.connectionId,
    this.connectionKind,
    this.establishedAt,
    this.eventId,
    this.externalOrgName,
    this.externalTenantId
  });

  factory EstablishExternalConnectionRequest.fromJson(Map<String, dynamic> json) {
    return EstablishExternalConnectionRequest(
      connectionId: json['connectionId']?.toString(),
      connectionKind: json['connectionKind']?.toString(),
      establishedAt: json['establishedAt']?.toString(),
      eventId: json['eventId']?.toString(),
      externalOrgName: json['externalOrgName']?.toString(),
      externalTenantId: json['externalTenantId']?.toString()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'connectionId': connectionId,
      'connectionKind': connectionKind,
      'establishedAt': establishedAt,
      'eventId': eventId,
      'externalOrgName': externalOrgName,
      'externalTenantId': externalTenantId,
    };
  }
}

class KillSwitchResponse {
  final bool? active;
  final List<String>? disabledBindings;
  final List<String>? disabledCapabilities;
  final List<String>? disabledCodecs;
  final String? reason;
  final String? ruleId;

  KillSwitchResponse({
    this.active,
    this.disabledBindings,
    this.disabledCapabilities,
    this.disabledCodecs,
    this.reason,
    this.ruleId
  });

  factory KillSwitchResponse.fromJson(Map<String, dynamic> json) {
    return KillSwitchResponse(
      active: json['active'] is bool ? json['active'] : null,
      disabledBindings: (() {
        final list = _sdkworkAsList(json['disabledBindings']);
        if (list == null) {
          return null;
        }
        return list
            .map((item) => item?.toString())
            .whereType<String>()
            .toList();
      })(),
      disabledCapabilities: (() {
        final list = _sdkworkAsList(json['disabledCapabilities']);
        if (list == null) {
          return null;
        }
        return list
            .map((item) => item?.toString())
            .whereType<String>()
            .toList();
      })(),
      disabledCodecs: (() {
        final list = _sdkworkAsList(json['disabledCodecs']);
        if (list == null) {
          return null;
        }
        return list
            .map((item) => item?.toString())
            .whereType<String>()
            .toList();
      })(),
      reason: json['reason']?.toString(),
      ruleId: json['ruleId']?.toString()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'active': active,
      'disabledBindings': disabledBindings?.map((item) => item).toList(),
      'disabledCapabilities': disabledCapabilities?.map((item) => item).toList(),
      'disabledCodecs': disabledCodecs?.map((item) => item).toList(),
      'reason': reason,
      'ruleId': ruleId,
    };
  }
}

class MigrateRoutesRequest {
  final String? targetNodeId;

  MigrateRoutesRequest({
    this.targetNodeId
  });

  factory MigrateRoutesRequest.fromJson(Map<String, dynamic> json) {
    return MigrateRoutesRequest(
      targetNodeId: json['targetNodeId']?.toString()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'targetNodeId': targetNodeId,
    };
  }
}

class ProtocolGovernanceResponse {
  final BusinessPolicyVocabularyResponse? businessPolicyVocabulary;
  final CapabilityProfileResponse? capabilityProfile;
  final EffectiveProtocolSnapshotResponse? effectiveSnapshot;
  final KillSwitchResponse? killSwitch;
  final QuotaProfileResponse? quotaProfile;
  final RolloutPolicyResponse? rolloutPolicy;
  final SdkCompatibilityBaselineResponse? sdkCompatibilityBaseline;

  ProtocolGovernanceResponse({
    this.businessPolicyVocabulary,
    this.capabilityProfile,
    this.effectiveSnapshot,
    this.killSwitch,
    this.quotaProfile,
    this.rolloutPolicy,
    this.sdkCompatibilityBaseline
  });

  factory ProtocolGovernanceResponse.fromJson(Map<String, dynamic> json) {
    return ProtocolGovernanceResponse(
      businessPolicyVocabulary: (() {
        final map = _sdkworkAsMap(json['businessPolicyVocabulary']);
        return map == null ? null : BusinessPolicyVocabularyResponse.fromJson(map);
      })(),
      capabilityProfile: (() {
        final map = _sdkworkAsMap(json['capabilityProfile']);
        return map == null ? null : CapabilityProfileResponse.fromJson(map);
      })(),
      effectiveSnapshot: (() {
        final map = _sdkworkAsMap(json['effectiveSnapshot']);
        return map == null ? null : EffectiveProtocolSnapshotResponse.fromJson(map);
      })(),
      killSwitch: (() {
        final map = _sdkworkAsMap(json['killSwitch']);
        return map == null ? null : KillSwitchResponse.fromJson(map);
      })(),
      quotaProfile: (() {
        final map = _sdkworkAsMap(json['quotaProfile']);
        return map == null ? null : QuotaProfileResponse.fromJson(map);
      })(),
      rolloutPolicy: (() {
        final map = _sdkworkAsMap(json['rolloutPolicy']);
        return map == null ? null : RolloutPolicyResponse.fromJson(map);
      })(),
      sdkCompatibilityBaseline: (() {
        final map = _sdkworkAsMap(json['sdkCompatibilityBaseline']);
        return map == null ? null : SdkCompatibilityBaselineResponse.fromJson(map);
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'businessPolicyVocabulary': businessPolicyVocabulary?.toJson(),
      'capabilityProfile': capabilityProfile?.toJson(),
      'effectiveSnapshot': effectiveSnapshot?.toJson(),
      'killSwitch': killSwitch?.toJson(),
      'quotaProfile': quotaProfile?.toJson(),
      'rolloutPolicy': rolloutPolicy?.toJson(),
      'sdkCompatibilityBaseline': sdkCompatibilityBaseline?.toJson(),
    };
  }
}

class ProtocolRegistryResponse {
  final List<String>? bindings;
  final List<String>? codecs;
  final List<ClientCompatibilityResponse>? compatibilityMatrix;
  final String? protocolVersion;
  final List<ProtocolSchemaResponse>? schemas;

  ProtocolRegistryResponse({
    this.bindings,
    this.codecs,
    this.compatibilityMatrix,
    this.protocolVersion,
    this.schemas
  });

  factory ProtocolRegistryResponse.fromJson(Map<String, dynamic> json) {
    return ProtocolRegistryResponse(
      bindings: (() {
        final list = _sdkworkAsList(json['bindings']);
        if (list == null) {
          return null;
        }
        return list
            .map((item) => item?.toString())
            .whereType<String>()
            .toList();
      })(),
      codecs: (() {
        final list = _sdkworkAsList(json['codecs']);
        if (list == null) {
          return null;
        }
        return list
            .map((item) => item?.toString())
            .whereType<String>()
            .toList();
      })(),
      compatibilityMatrix: (() {
        final list = _sdkworkAsList(json['compatibilityMatrix']);
        if (list == null) {
          return null;
        }
        return list
            .map((item) => (() {
        final map = _sdkworkAsMap(item);
        return map == null ? null : ClientCompatibilityResponse.fromJson(map);
      })())
            .whereType<ClientCompatibilityResponse>()
            .toList();
      })(),
      protocolVersion: json['protocolVersion']?.toString(),
      schemas: (() {
        final list = _sdkworkAsList(json['schemas']);
        if (list == null) {
          return null;
        }
        return list
            .map((item) => (() {
        final map = _sdkworkAsMap(item);
        return map == null ? null : ProtocolSchemaResponse.fromJson(map);
      })())
            .whereType<ProtocolSchemaResponse>()
            .toList();
      })()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'bindings': bindings?.map((item) => item).toList(),
      'codecs': codecs?.map((item) => item).toList(),
      'compatibilityMatrix': compatibilityMatrix?.map((item) => item.toJson()).toList(),
      'protocolVersion': protocolVersion,
      'schemas': schemas?.map((item) => item.toJson()).toList(),
    };
  }
}

class ProtocolSchemaResponse {
  final List<String>? bindingProtocols;
  final String? kind;
  final List<String>? requiredCapabilities;
  final String? schema;
  final String? stage;
  final List<String>? supportedConsumers;

  ProtocolSchemaResponse({
    this.bindingProtocols,
    this.kind,
    this.requiredCapabilities,
    this.schema,
    this.stage,
    this.supportedConsumers
  });

  factory ProtocolSchemaResponse.fromJson(Map<String, dynamic> json) {
    return ProtocolSchemaResponse(
      bindingProtocols: (() {
        final list = _sdkworkAsList(json['bindingProtocols']);
        if (list == null) {
          return null;
        }
        return list
            .map((item) => item?.toString())
            .whereType<String>()
            .toList();
      })(),
      kind: json['kind']?.toString(),
      requiredCapabilities: (() {
        final list = _sdkworkAsList(json['requiredCapabilities']);
        if (list == null) {
          return null;
        }
        return list
            .map((item) => item?.toString())
            .whereType<String>()
            .toList();
      })(),
      schema: json['schema']?.toString(),
      stage: json['stage']?.toString(),
      supportedConsumers: (() {
        final list = _sdkworkAsList(json['supportedConsumers']);
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
      'bindingProtocols': bindingProtocols?.map((item) => item).toList(),
      'kind': kind,
      'requiredCapabilities': requiredCapabilities?.map((item) => item).toList(),
      'schema': schema,
      'stage': stage,
      'supportedConsumers': supportedConsumers?.map((item) => item).toList(),
    };
  }
}

class ProviderBindingCommitResponse {


  ProviderBindingCommitResponse();

  factory ProviderBindingCommitResponse.fromJson(Map<String, dynamic> json) {
    return ProviderBindingCommitResponse();
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{};
  }
}

class ProviderBindingsResponse {


  ProviderBindingsResponse();

  factory ProviderBindingsResponse.fromJson(Map<String, dynamic> json) {
    return ProviderBindingsResponse();
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{};
  }
}

class ProviderPolicyDiffResponse {


  ProviderPolicyDiffResponse();

  factory ProviderPolicyDiffResponse.fromJson(Map<String, dynamic> json) {
    return ProviderPolicyDiffResponse();
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{};
  }
}

class ProviderPolicyHistoryResponse {


  ProviderPolicyHistoryResponse();

  factory ProviderPolicyHistoryResponse.fromJson(Map<String, dynamic> json) {
    return ProviderPolicyHistoryResponse();
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{};
  }
}

class ProviderPolicyRollbackRequest {
  final int? targetVersion;

  ProviderPolicyRollbackRequest({
    this.targetVersion
  });

  factory ProviderPolicyRollbackRequest.fromJson(Map<String, dynamic> json) {
    return ProviderPolicyRollbackRequest(
      targetVersion: json['targetVersion'] is int ? json['targetVersion'] : null
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'targetVersion': targetVersion,
    };
  }
}

class ProviderRegistrySnapshotResponse {


  ProviderRegistrySnapshotResponse();

  factory ProviderRegistrySnapshotResponse.fromJson(Map<String, dynamic> json) {
    return ProviderRegistrySnapshotResponse();
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{};
  }
}

class QuotaProfileResponse {
  final int? maxConcurrentSessionsPerTenant;
  final int? maxInflightMessages;
  final int? maxPayloadBytes;
  final int? maxSubscriptionsPerSession;
  final String? profileId;

  QuotaProfileResponse({
    this.maxConcurrentSessionsPerTenant,
    this.maxInflightMessages,
    this.maxPayloadBytes,
    this.maxSubscriptionsPerSession,
    this.profileId
  });

  factory QuotaProfileResponse.fromJson(Map<String, dynamic> json) {
    return QuotaProfileResponse(
      maxConcurrentSessionsPerTenant: json['maxConcurrentSessionsPerTenant'] is int ? json['maxConcurrentSessionsPerTenant'] : null,
      maxInflightMessages: json['maxInflightMessages'] is int ? json['maxInflightMessages'] : null,
      maxPayloadBytes: json['maxPayloadBytes'] is int ? json['maxPayloadBytes'] : null,
      maxSubscriptionsPerSession: json['maxSubscriptionsPerSession'] is int ? json['maxSubscriptionsPerSession'] : null,
      profileId: json['profileId']?.toString()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'maxConcurrentSessionsPerTenant': maxConcurrentSessionsPerTenant,
      'maxInflightMessages': maxInflightMessages,
      'maxPayloadBytes': maxPayloadBytes,
      'maxSubscriptionsPerSession': maxSubscriptionsPerSession,
      'profileId': profileId,
    };
  }
}

class RolloutPolicyResponse {
  final String? cellSelector;
  final bool? operatorOverride;
  final String? policyId;
  final String? regionSelector;
  final String? releaseChannel;
  final List<String>? tenantAllowlist;
  final int? trafficPercent;

  RolloutPolicyResponse({
    this.cellSelector,
    this.operatorOverride,
    this.policyId,
    this.regionSelector,
    this.releaseChannel,
    this.tenantAllowlist,
    this.trafficPercent
  });

  factory RolloutPolicyResponse.fromJson(Map<String, dynamic> json) {
    return RolloutPolicyResponse(
      cellSelector: json['cellSelector']?.toString(),
      operatorOverride: json['operatorOverride'] is bool ? json['operatorOverride'] : null,
      policyId: json['policyId']?.toString(),
      regionSelector: json['regionSelector']?.toString(),
      releaseChannel: json['releaseChannel']?.toString(),
      tenantAllowlist: (() {
        final list = _sdkworkAsList(json['tenantAllowlist']);
        if (list == null) {
          return null;
        }
        return list
            .map((item) => item?.toString())
            .whereType<String>()
            .toList();
      })(),
      trafficPercent: json['trafficPercent'] is int ? json['trafficPercent'] : null
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'cellSelector': cellSelector,
      'operatorOverride': operatorOverride,
      'policyId': policyId,
      'regionSelector': regionSelector,
      'releaseChannel': releaseChannel,
      'tenantAllowlist': tenantAllowlist?.map((item) => item).toList(),
      'trafficPercent': trafficPercent,
    };
  }
}

class RouteMigrationResult {
  final int? migratedRouteCount;
  final String? sourceDrainStatus;
  final String? sourceNodeId;
  final String? sourceRebalanceState;
  final String? targetDrainStatus;
  final String? targetNodeId;
  final String? targetRebalanceState;

  RouteMigrationResult({
    this.migratedRouteCount,
    this.sourceDrainStatus,
    this.sourceNodeId,
    this.sourceRebalanceState,
    this.targetDrainStatus,
    this.targetNodeId,
    this.targetRebalanceState
  });

  factory RouteMigrationResult.fromJson(Map<String, dynamic> json) {
    return RouteMigrationResult(
      migratedRouteCount: json['migratedRouteCount'] is int ? json['migratedRouteCount'] : null,
      sourceDrainStatus: json['sourceDrainStatus']?.toString(),
      sourceNodeId: json['sourceNodeId']?.toString(),
      sourceRebalanceState: json['sourceRebalanceState']?.toString(),
      targetDrainStatus: json['targetDrainStatus']?.toString(),
      targetNodeId: json['targetNodeId']?.toString(),
      targetRebalanceState: json['targetRebalanceState']?.toString()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'migratedRouteCount': migratedRouteCount,
      'sourceDrainStatus': sourceDrainStatus,
      'sourceNodeId': sourceNodeId,
      'sourceRebalanceState': sourceRebalanceState,
      'targetDrainStatus': targetDrainStatus,
      'targetNodeId': targetNodeId,
      'targetRebalanceState': targetRebalanceState,
    };
  }
}

class RouteNodeLifecycle {
  final String? drainStatus;
  final String? nodeId;
  final int? ownedRouteCount;
  final String? rebalanceState;

  RouteNodeLifecycle({
    this.drainStatus,
    this.nodeId,
    this.ownedRouteCount,
    this.rebalanceState
  });

  factory RouteNodeLifecycle.fromJson(Map<String, dynamic> json) {
    return RouteNodeLifecycle(
      drainStatus: json['drainStatus']?.toString(),
      nodeId: json['nodeId']?.toString(),
      ownedRouteCount: json['ownedRouteCount'] is int ? json['ownedRouteCount'] : null,
      rebalanceState: json['rebalanceState']?.toString()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'drainStatus': drainStatus,
      'nodeId': nodeId,
      'ownedRouteCount': ownedRouteCount,
      'rebalanceState': rebalanceState,
    };
  }
}

class SdkCompatibilityBaselineResponse {
  final String? appSdkFamily;
  final String? backendSdkFamily;
  final String? imSdkFamily;
  final String? rtcSdkFamily;
  final List<String>? matrixClientTypes;
  final String? protocolGovernancePath;
  final String? protocolRegistryPath;

  SdkCompatibilityBaselineResponse({
    this.appSdkFamily,
    this.backendSdkFamily,
    this.imSdkFamily,
    this.rtcSdkFamily,
    this.matrixClientTypes,
    this.protocolGovernancePath,
    this.protocolRegistryPath
  });

  factory SdkCompatibilityBaselineResponse.fromJson(Map<String, dynamic> json) {
    return SdkCompatibilityBaselineResponse(
      appSdkFamily: json['appSdkFamily']?.toString(),
      backendSdkFamily: json['backendSdkFamily']?.toString(),
      imSdkFamily: json['imSdkFamily']?.toString(),
      rtcSdkFamily: json['rtcSdkFamily']?.toString(),
      matrixClientTypes: (() {
        final list = _sdkworkAsList(json['matrixClientTypes']);
        if (list == null) {
          return null;
        }
        return list
            .map((item) => item?.toString())
            .whereType<String>()
            .toList();
      })(),
      protocolGovernancePath: json['protocolGovernancePath']?.toString(),
      protocolRegistryPath: json['protocolRegistryPath']?.toString()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'appSdkFamily': appSdkFamily,
      'backendSdkFamily': backendSdkFamily,
      'imSdkFamily': imSdkFamily,
      'rtcSdkFamily': rtcSdkFamily,
      'matrixClientTypes': matrixClientTypes?.map((item) => item).toList(),
      'protocolGovernancePath': protocolGovernancePath,
      'protocolRegistryPath': protocolRegistryPath,
    };
  }
}

class AcceptFriendRequestRequest {
  final String? acceptedAt;
  final String? acceptedByUserId;
  final String? eventId;

  AcceptFriendRequestRequest({
    this.acceptedAt,
    this.acceptedByUserId,
    this.eventId
  });

  factory AcceptFriendRequestRequest.fromJson(Map<String, dynamic> json) {
    return AcceptFriendRequestRequest(
      acceptedAt: json['acceptedAt']?.toString(),
      acceptedByUserId: json['acceptedByUserId']?.toString(),
      eventId: json['eventId']?.toString()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'acceptedAt': acceptedAt,
      'acceptedByUserId': acceptedByUserId,
      'eventId': eventId,
    };
  }
}

class DeclineFriendRequestRequest {
  final String? declinedAt;
  final String? declinedByUserId;
  final String? eventId;

  DeclineFriendRequestRequest({
    this.declinedAt,
    this.declinedByUserId,
    this.eventId
  });

  factory DeclineFriendRequestRequest.fromJson(Map<String, dynamic> json) {
    return DeclineFriendRequestRequest(
      declinedAt: json['declinedAt']?.toString(),
      declinedByUserId: json['declinedByUserId']?.toString(),
      eventId: json['eventId']?.toString()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'declinedAt': declinedAt,
      'declinedByUserId': declinedByUserId,
      'eventId': eventId,
    };
  }
}

class CancelFriendRequestRequest {
  final String? canceledAt;
  final String? canceledByUserId;
  final String? eventId;

  CancelFriendRequestRequest({
    this.canceledAt,
    this.canceledByUserId,
    this.eventId
  });

  factory CancelFriendRequestRequest.fromJson(Map<String, dynamic> json) {
    return CancelFriendRequestRequest(
      canceledAt: json['canceledAt']?.toString(),
      canceledByUserId: json['canceledByUserId']?.toString(),
      eventId: json['eventId']?.toString()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'canceledAt': canceledAt,
      'canceledByUserId': canceledByUserId,
      'eventId': eventId,
    };
  }
}

class RemoveFriendshipRequest {
  final String? eventId;
  final String? removedAt;
  final String? removedByUserId;

  RemoveFriendshipRequest({
    this.eventId,
    this.removedAt,
    this.removedByUserId
  });

  factory RemoveFriendshipRequest.fromJson(Map<String, dynamic> json) {
    return RemoveFriendshipRequest(
      eventId: json['eventId']?.toString(),
      removedAt: json['removedAt']?.toString(),
      removedByUserId: json['removedByUserId']?.toString()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'eventId': eventId,
      'removedAt': removedAt,
      'removedByUserId': removedByUserId,
    };
  }
}

class SocialDirectChatCommitResponse {


  SocialDirectChatCommitResponse();

  factory SocialDirectChatCommitResponse.fromJson(Map<String, dynamic> json) {
    return SocialDirectChatCommitResponse();
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{};
  }
}

class SocialDirectChatSnapshotResponse {


  SocialDirectChatSnapshotResponse();

  factory SocialDirectChatSnapshotResponse.fromJson(Map<String, dynamic> json) {
    return SocialDirectChatSnapshotResponse();
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{};
  }
}

class SocialExternalConnectionCommitResponse {


  SocialExternalConnectionCommitResponse();

  factory SocialExternalConnectionCommitResponse.fromJson(Map<String, dynamic> json) {
    return SocialExternalConnectionCommitResponse();
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{};
  }
}

class SocialExternalConnectionSnapshotResponse {


  SocialExternalConnectionSnapshotResponse();

  factory SocialExternalConnectionSnapshotResponse.fromJson(Map<String, dynamic> json) {
    return SocialExternalConnectionSnapshotResponse();
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{};
  }
}

class SocialExternalMemberLinkCommitResponse {


  SocialExternalMemberLinkCommitResponse();

  factory SocialExternalMemberLinkCommitResponse.fromJson(Map<String, dynamic> json) {
    return SocialExternalMemberLinkCommitResponse();
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{};
  }
}

class SocialExternalMemberLinkSnapshotResponse {


  SocialExternalMemberLinkSnapshotResponse();

  factory SocialExternalMemberLinkSnapshotResponse.fromJson(Map<String, dynamic> json) {
    return SocialExternalMemberLinkSnapshotResponse();
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{};
  }
}

class SocialFriendRequestCommitResponse {


  SocialFriendRequestCommitResponse();

  factory SocialFriendRequestCommitResponse.fromJson(Map<String, dynamic> json) {
    return SocialFriendRequestCommitResponse();
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{};
  }
}

class SocialFriendRequestSnapshotResponse {


  SocialFriendRequestSnapshotResponse();

  factory SocialFriendRequestSnapshotResponse.fromJson(Map<String, dynamic> json) {
    return SocialFriendRequestSnapshotResponse();
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{};
  }
}

class SocialFriendshipCommitResponse {


  SocialFriendshipCommitResponse();

  factory SocialFriendshipCommitResponse.fromJson(Map<String, dynamic> json) {
    return SocialFriendshipCommitResponse();
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{};
  }
}

class SocialFriendshipSnapshotResponse {


  SocialFriendshipSnapshotResponse();

  factory SocialFriendshipSnapshotResponse.fromJson(Map<String, dynamic> json) {
    return SocialFriendshipSnapshotResponse();
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{};
  }
}

class SocialRuntimeRepairResponse {


  SocialRuntimeRepairResponse();

  factory SocialRuntimeRepairResponse.fromJson(Map<String, dynamic> json) {
    return SocialRuntimeRepairResponse();
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{};
  }
}

class SocialSharedChannelPolicyCommitResponse {


  SocialSharedChannelPolicyCommitResponse();

  factory SocialSharedChannelPolicyCommitResponse.fromJson(Map<String, dynamic> json) {
    return SocialSharedChannelPolicyCommitResponse();
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{};
  }
}

class SocialSharedChannelPolicySnapshotResponse {


  SocialSharedChannelPolicySnapshotResponse();

  factory SocialSharedChannelPolicySnapshotResponse.fromJson(Map<String, dynamic> json) {
    return SocialSharedChannelPolicySnapshotResponse();
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{};
  }
}

class SocialSharedChannelSyncDeadLetterInventoryResponse {


  SocialSharedChannelSyncDeadLetterInventoryResponse();

  factory SocialSharedChannelSyncDeadLetterInventoryResponse.fromJson(Map<String, dynamic> json) {
    return SocialSharedChannelSyncDeadLetterInventoryResponse();
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{};
  }
}

class SocialSharedChannelSyncDeadLetterRequeueResponse {


  SocialSharedChannelSyncDeadLetterRequeueResponse();

  factory SocialSharedChannelSyncDeadLetterRequeueResponse.fromJson(Map<String, dynamic> json) {
    return SocialSharedChannelSyncDeadLetterRequeueResponse();
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{};
  }
}

class SocialSharedChannelSyncDeadLetterTargetedRequeueRequest {
  final List<String>? requestKeys;

  SocialSharedChannelSyncDeadLetterTargetedRequeueRequest({
    this.requestKeys
  });

  factory SocialSharedChannelSyncDeadLetterTargetedRequeueRequest.fromJson(Map<String, dynamic> json) {
    return SocialSharedChannelSyncDeadLetterTargetedRequeueRequest(
      requestKeys: (() {
        final list = _sdkworkAsList(json['requestKeys']);
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
      'requestKeys': requestKeys?.map((item) => item).toList(),
    };
  }
}

class SocialSharedChannelSyncDeadLetterTargetedRequeueResponse {


  SocialSharedChannelSyncDeadLetterTargetedRequeueResponse();

  factory SocialSharedChannelSyncDeadLetterTargetedRequeueResponse.fromJson(Map<String, dynamic> json) {
    return SocialSharedChannelSyncDeadLetterTargetedRequeueResponse();
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{};
  }
}

class SocialSharedChannelSyncDeliveredInventoryResponse {


  SocialSharedChannelSyncDeliveredInventoryResponse();

  factory SocialSharedChannelSyncDeliveredInventoryResponse.fromJson(Map<String, dynamic> json) {
    return SocialSharedChannelSyncDeliveredInventoryResponse();
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{};
  }
}

class SocialSharedChannelSyncDeliveryStateInventoryResponse {


  SocialSharedChannelSyncDeliveryStateInventoryResponse();

  factory SocialSharedChannelSyncDeliveryStateInventoryResponse.fromJson(Map<String, dynamic> json) {
    return SocialSharedChannelSyncDeliveryStateInventoryResponse();
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{};
  }
}

class SocialSharedChannelSyncPendingClaimResponse {


  SocialSharedChannelSyncPendingClaimResponse();

  factory SocialSharedChannelSyncPendingClaimResponse.fromJson(Map<String, dynamic> json) {
    return SocialSharedChannelSyncPendingClaimResponse();
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{};
  }
}

class SocialSharedChannelSyncPendingInventoryResponse {


  SocialSharedChannelSyncPendingInventoryResponse();

  factory SocialSharedChannelSyncPendingInventoryResponse.fromJson(Map<String, dynamic> json) {
    return SocialSharedChannelSyncPendingInventoryResponse();
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{};
  }
}

class SocialSharedChannelSyncPendingReleaseResponse {


  SocialSharedChannelSyncPendingReleaseResponse();

  factory SocialSharedChannelSyncPendingReleaseResponse.fromJson(Map<String, dynamic> json) {
    return SocialSharedChannelSyncPendingReleaseResponse();
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{};
  }
}

class SocialSharedChannelSyncPendingStaleReclaimResponse {


  SocialSharedChannelSyncPendingStaleReclaimResponse();

  factory SocialSharedChannelSyncPendingStaleReclaimResponse.fromJson(Map<String, dynamic> json) {
    return SocialSharedChannelSyncPendingStaleReclaimResponse();
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{};
  }
}

class SocialSharedChannelSyncPendingTakeoverResponse {


  SocialSharedChannelSyncPendingTakeoverResponse();

  factory SocialSharedChannelSyncPendingTakeoverResponse.fromJson(Map<String, dynamic> json) {
    return SocialSharedChannelSyncPendingTakeoverResponse();
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{};
  }
}

class SocialSharedChannelSyncPendingTargetedClaimRequest {
  final List<String>? requestKeys;

  SocialSharedChannelSyncPendingTargetedClaimRequest({
    this.requestKeys
  });

  factory SocialSharedChannelSyncPendingTargetedClaimRequest.fromJson(Map<String, dynamic> json) {
    return SocialSharedChannelSyncPendingTargetedClaimRequest(
      requestKeys: (() {
        final list = _sdkworkAsList(json['requestKeys']);
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
      'requestKeys': requestKeys?.map((item) => item).toList(),
    };
  }
}

class SocialSharedChannelSyncPendingTargetedReleaseRequest {
  final List<String>? requestKeys;

  SocialSharedChannelSyncPendingTargetedReleaseRequest({
    this.requestKeys
  });

  factory SocialSharedChannelSyncPendingTargetedReleaseRequest.fromJson(Map<String, dynamic> json) {
    return SocialSharedChannelSyncPendingTargetedReleaseRequest(
      requestKeys: (() {
        final list = _sdkworkAsList(json['requestKeys']);
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
      'requestKeys': requestKeys?.map((item) => item).toList(),
    };
  }
}

class SocialSharedChannelSyncPendingTargetedTakeoverRequest {
  final bool? allowLegacyUntracked;
  final List<String>? requestKeys;

  SocialSharedChannelSyncPendingTargetedTakeoverRequest({
    this.allowLegacyUntracked,
    this.requestKeys
  });

  factory SocialSharedChannelSyncPendingTargetedTakeoverRequest.fromJson(Map<String, dynamic> json) {
    return SocialSharedChannelSyncPendingTargetedTakeoverRequest(
      allowLegacyUntracked: json['allowLegacyUntracked'] is bool ? json['allowLegacyUntracked'] : null,
      requestKeys: (() {
        final list = _sdkworkAsList(json['requestKeys']);
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
      'allowLegacyUntracked': allowLegacyUntracked,
      'requestKeys': requestKeys?.map((item) => item).toList(),
    };
  }
}

class SocialSharedChannelSyncRepairResponse {


  SocialSharedChannelSyncRepairResponse();

  factory SocialSharedChannelSyncRepairResponse.fromJson(Map<String, dynamic> json) {
    return SocialSharedChannelSyncRepairResponse();
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{};
  }
}

class SocialSharedChannelSyncTargetedRepublishRequest {
  final List<String>? requestKeys;

  SocialSharedChannelSyncTargetedRepublishRequest({
    this.requestKeys
  });

  factory SocialSharedChannelSyncTargetedRepublishRequest.fromJson(Map<String, dynamic> json) {
    return SocialSharedChannelSyncTargetedRepublishRequest(
      requestKeys: (() {
        final list = _sdkworkAsList(json['requestKeys']);
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
      'requestKeys': requestKeys?.map((item) => item).toList(),
    };
  }
}

class SocialSharedChannelSyncTargetedRepublishResponse {


  SocialSharedChannelSyncTargetedRepublishResponse();

  factory SocialSharedChannelSyncTargetedRepublishResponse.fromJson(Map<String, dynamic> json) {
    return SocialSharedChannelSyncTargetedRepublishResponse();
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{};
  }
}

class SocialUserBlockCommitResponse {


  SocialUserBlockCommitResponse();

  factory SocialUserBlockCommitResponse.fromJson(Map<String, dynamic> json) {
    return SocialUserBlockCommitResponse();
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{};
  }
}

class SocialUserBlockSnapshotResponse {


  SocialUserBlockSnapshotResponse();

  factory SocialUserBlockSnapshotResponse.fromJson(Map<String, dynamic> json) {
    return SocialUserBlockSnapshotResponse();
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{};
  }
}

class SubmitFriendRequestRequest {
  final String? eventId;
  final String? requestId;
  final String? requestMessage;
  final String? requestedAt;
  final String? requesterUserId;
  final String? targetUserId;

  SubmitFriendRequestRequest({
    this.eventId,
    this.requestId,
    this.requestMessage,
    this.requestedAt,
    this.requesterUserId,
    this.targetUserId
  });

  factory SubmitFriendRequestRequest.fromJson(Map<String, dynamic> json) {
    return SubmitFriendRequestRequest(
      eventId: json['eventId']?.toString(),
      requestId: json['requestId']?.toString(),
      requestMessage: json['requestMessage']?.toString(),
      requestedAt: json['requestedAt']?.toString(),
      requesterUserId: json['requesterUserId']?.toString(),
      targetUserId: json['targetUserId']?.toString()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'eventId': eventId,
      'requestId': requestId,
      'requestMessage': requestMessage,
      'requestedAt': requestedAt,
      'requesterUserId': requesterUserId,
      'targetUserId': targetUserId,
    };
  }
}

class UpsertProviderBindingPolicyRequest {
  final String? domain;
  final int? expectedBaseVersion;
  final String? pluginId;
  final String? tenantId;

  UpsertProviderBindingPolicyRequest({
    this.domain,
    this.expectedBaseVersion,
    this.pluginId,
    this.tenantId
  });

  factory UpsertProviderBindingPolicyRequest.fromJson(Map<String, dynamic> json) {
    return UpsertProviderBindingPolicyRequest(
      domain: json['domain']?.toString(),
      expectedBaseVersion: json['expectedBaseVersion'] is int ? json['expectedBaseVersion'] : null,
      pluginId: json['pluginId']?.toString(),
      tenantId: json['tenantId']?.toString()
    );
  }

  Map<String, dynamic> toJson() {
    return <String, dynamic>{
      'domain': domain,
      'expectedBaseVersion': expectedBaseVersion,
      'pluginId': pluginId,
      'tenantId': tenantId,
    };
  }
}
