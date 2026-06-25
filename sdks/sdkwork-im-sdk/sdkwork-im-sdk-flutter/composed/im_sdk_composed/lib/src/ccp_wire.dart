import 'dart:convert';

const imCcpWebSocketSubprotocol = 'sdkwork-im.ccp.ws.v1';
const imRealtimeWsPath = '/im/v3/api/realtime/ws';

const _ccpProtocol = <String, dynamic>{'family': 'ccp', 'major': 1, 'minor': 0};
const _ccpWsBinding = 'Ws1';

class ImCcpAuthBindContext {
  const ImCcpAuthBindContext({
    required this.principalId,
    required this.actorKind,
    this.deviceId,
    this.sessionId,
  });

  final String principalId;
  final String actorKind;
  final String? deviceId;
  final String? sessionId;
}

Map<String, dynamic> _encodeCcpEnvelope(
  String schema,
  String kind,
  Map<String, dynamic> payload, {
  String? traceId,
}) {
  return <String, dynamic>{
    'protocol': Map<String, dynamic>.from(_ccpProtocol),
    'binding': _ccpWsBinding,
    'kind': kind,
    'schema': schema,
    'scope': null,
    'route': null,
    'flags': <String>[],
    if (traceId != null) 'trace_id': traceId,
    'payload': jsonEncode(payload),
  };
}

String encodeCcpControlFrame(
  String schema,
  String controlType,
  Map<String, dynamic> data, {
  String? traceId,
}) {
  return jsonEncode(_encodeCcpEnvelope(
    schema,
    'control',
    <String, dynamic>{'type': controlType, 'data': data},
    traceId: traceId,
  ));
}

String encodeCcpBusinessFrame(String schema, String kind, Map<String, dynamic> payload) {
  return jsonEncode(_encodeCcpEnvelope(schema, kind, payload));
}

Map<String, dynamic>? decodeCcpEnvelope(String raw) {
  try {
    final parsed = jsonDecode(raw);
    if (parsed is! Map) {
      return null;
    }
    final envelope = Map<String, dynamic>.from(parsed);
    if (envelope['payload'] is! String || envelope['schema'] is! String) {
      return null;
    }
    return envelope;
  } catch (_) {
    return null;
  }
}

Map<String, dynamic>? parseCcpEnvelopePayload(Map<String, dynamic> envelope) {
  try {
    final parsed = jsonDecode(envelope['payload'] as String);
    if (parsed is Map) {
      return Map<String, dynamic>.from(parsed);
    }
  } catch (_) {}
  return null;
}

String unwrapInboundRealtimeFrame(String raw) {
  final envelope = decodeCcpEnvelope(raw);
  if (envelope == null) {
    return raw;
  }
  return envelope['payload'] as String;
}

String encodeCcpHelloFrame(String requestId) {
  return encodeCcpControlFrame(
    'cc.control.hello.v1',
    'hello',
    <String, dynamic>{
      'protocol': Map<String, dynamic>.from(_ccpProtocol),
      'binding': _ccpWsBinding,
      'capabilities': <String, dynamic>{'items': <String>['payload.json', 'session.resume']},
      'trace_id': requestId,
    },
    traceId: requestId,
  );
}

String encodeCcpAuthBindFrame(ImCcpAuthBindContext context) {
  return encodeCcpControlFrame('cc.control.auth_bind.v1', 'auth_bind', <String, dynamic>{
    'principal_id': context.principalId,
    'device_id': context.deviceId,
    'session_id': context.sessionId,
    'actor_kind': context.actorKind,
  });
}

String encodeCcpHeartbeatFrame(int sequence) {
  return encodeCcpControlFrame('cc.control.heartbeat.v1', 'heartbeat', <String, dynamic>{
    'sequence': sequence,
  });
}

bool isCcpHelloAckEnvelope(String raw) {
  final envelope = decodeCcpEnvelope(raw);
  return envelope?['schema'] == 'cc.control.hello_ack.v1';
}

bool isCcpAuthOkEnvelope(String raw) {
  final envelope = decodeCcpEnvelope(raw);
  return envelope?['schema'] == 'cc.control.auth_ok.v1';
}

Map<String, dynamic>? decodeJwtPayload(String? token) {
  if (token == null || token.isEmpty) {
    return null;
  }
  final parts = token.split('.');
  if (parts.length < 2) {
    return null;
  }
  try {
    final normalized = parts[1].replaceAll('-', '+').replaceAll('_', '/');
    final padding = '=' * ((4 - normalized.length % 4) % 4);
    final decoded = utf8.decode(base64Decode(normalized + padding));
    final parsed = jsonDecode(decoded);
    if (parsed is Map) {
      return Map<String, dynamic>.from(parsed);
    }
  } catch (_) {}
  return null;
}

ImCcpAuthBindContext? resolveCcpAuthBindContext({
  String? accessToken,
  Map<String, dynamic>? authOk,
  String? deviceId,
}) {
  final jwtClaims = decodeJwtPayload(accessToken);
  final principalId = _pickString([
    authOk?['principalId'],
    jwtClaims?['user_id'],
    jwtClaims?['userId'],
  ]);
  if (principalId == null) {
    return null;
  }
  return ImCcpAuthBindContext(
    principalId: principalId,
    deviceId: _pickString([
      authOk?['deviceId'],
      deviceId,
      jwtClaims?['device_id'],
      jwtClaims?['deviceId'],
    ]),
    sessionId: _pickString([
      authOk?['sessionId'],
      jwtClaims?['session_id'],
      jwtClaims?['sessionId'],
    ]),
    actorKind: _pickString([
      authOk?['actorKind'],
      jwtClaims?['subject_type'],
    ]) ?? 'user',
  );
}

String? _pickString(List<dynamic> values) {
  for (final value in values) {
    if (value is String && value.trim().isNotEmpty) {
      return value.trim();
    }
  }
  return null;
}
