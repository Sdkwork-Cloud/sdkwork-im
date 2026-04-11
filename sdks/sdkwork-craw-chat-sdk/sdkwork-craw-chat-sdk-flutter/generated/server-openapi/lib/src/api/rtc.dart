import '../http/client.dart';
import '../models.dart';
import 'paths.dart';
import 'response_helpers.dart';

class RtcApi {
  final HttpClient _client;

  RtcApi(this._client);

  /// Create an RTC session
  Future<RtcSession?> createRtcSession(CreateRtcSessionRequest body) async {
    final payload = body.toJson();
    final response = await _client.post(ApiPaths.backendPath('/rtc/sessions'), body: payload, contentType: 'application/json');
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : RtcSession.fromJson(map);
    })();
  }

  /// Invite participants into an RTC session
  Future<RtcSession?> inviteRtcSession(String rtcSessionId, InviteRtcSessionRequest body) async {
    final payload = body.toJson();
    final response = await _client.post(ApiPaths.backendPath('/rtc/sessions/$rtcSessionId/invite'), body: payload, contentType: 'application/json');
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : RtcSession.fromJson(map);
    })();
  }

  /// Accept an RTC session
  Future<RtcSession?> acceptRtcSession(String rtcSessionId, UpdateRtcSessionRequest body) async {
    final payload = body.toJson();
    final response = await _client.post(ApiPaths.backendPath('/rtc/sessions/$rtcSessionId/accept'), body: payload, contentType: 'application/json');
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : RtcSession.fromJson(map);
    })();
  }

  /// Reject an RTC session
  Future<RtcSession?> rejectRtcSession(String rtcSessionId, UpdateRtcSessionRequest body) async {
    final payload = body.toJson();
    final response = await _client.post(ApiPaths.backendPath('/rtc/sessions/$rtcSessionId/reject'), body: payload, contentType: 'application/json');
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : RtcSession.fromJson(map);
    })();
  }

  /// End an RTC session
  Future<RtcSession?> endRtcSession(String rtcSessionId, UpdateRtcSessionRequest body) async {
    final payload = body.toJson();
    final response = await _client.post(ApiPaths.backendPath('/rtc/sessions/$rtcSessionId/end'), body: payload, contentType: 'application/json');
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : RtcSession.fromJson(map);
    })();
  }

  /// Post an RTC signaling event
  Future<RtcSignalEvent?> postRtcSignal(String rtcSessionId, PostRtcSignalRequest body) async {
    final payload = body.toJson();
    final response = await _client.post(ApiPaths.backendPath('/rtc/sessions/$rtcSessionId/signals'), body: payload, contentType: 'application/json');
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : RtcSignalEvent.fromJson(map);
    })();
  }

  /// Issue an RTC participant credential
  Future<RtcParticipantCredential?> issueRtcParticipantCredential(String rtcSessionId, IssueRtcParticipantCredentialRequest body) async {
    final payload = body.toJson();
    final response = await _client.post(ApiPaths.backendPath('/rtc/sessions/$rtcSessionId/credentials'), body: payload, contentType: 'application/json');
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : RtcParticipantCredential.fromJson(map);
    })();
  }

  /// Get the RTC recording artifact
  Future<RtcRecordingArtifact?> getRtcRecordingArtifact(String rtcSessionId) async {
    final response = await _client.get(ApiPaths.backendPath('/rtc/sessions/$rtcSessionId/artifacts/recording'));
    return (() {
      final map = sdkworkResponseAsMap(response);
      return map == null ? null : RtcRecordingArtifact.fromJson(map);
    })();
  }
}
