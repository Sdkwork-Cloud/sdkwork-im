import 'package:backend_sdk/backend_sdk.dart';

import 'builders.dart';
import 'context.dart';
import 'types.dart';

class CrawChatRtcModule {
  final CrawChatSdkContext context;

  CrawChatRtcModule(this.context);

  Future<RtcSession?> create(CreateRtcSessionRequest body) {
    return context.backendClient.rtc.createRtcSession(body);
  }

  Future<RtcSession?> invite(
    String rtcSessionId,
    InviteRtcSessionRequest body,
  ) {
    return context.backendClient.rtc.inviteRtcSession(rtcSessionId, body);
  }

  Future<RtcSession?> accept(
    String rtcSessionId,
    UpdateRtcSessionRequest body,
  ) {
    return context.backendClient.rtc.acceptRtcSession(rtcSessionId, body);
  }

  Future<RtcSession?> reject(
    String rtcSessionId,
    UpdateRtcSessionRequest body,
  ) {
    return context.backendClient.rtc.rejectRtcSession(rtcSessionId, body);
  }

  Future<RtcSession?> end(
    String rtcSessionId,
    UpdateRtcSessionRequest body,
  ) {
    return context.backendClient.rtc.endRtcSession(rtcSessionId, body);
  }

  Future<RtcSignalEvent?> postSignal(
    String rtcSessionId,
    PostRtcSignalRequest body,
  ) {
    return context.backendClient.rtc.postRtcSignal(rtcSessionId, body);
  }

  Future<RtcSignalEvent?> postJsonSignal(
    String rtcSessionId, {
    required String signalType,
    required CrawChatPostJsonSignalOptions options,
  }) {
    return postSignal(
      rtcSessionId,
      CrawChatBuilders.jsonRtcSignal(
        signalType: signalType,
        options: options,
      ),
    );
  }

  Future<RtcParticipantCredential?> issueParticipantCredential(
    String rtcSessionId,
    IssueRtcParticipantCredentialRequest body,
  ) {
    return context.backendClient.rtc.issueRtcParticipantCredential(
      rtcSessionId,
      body,
    );
  }

  Future<RtcRecordingArtifact?> getRecordingArtifact(String rtcSessionId) {
    return context.backendClient.rtc.getRtcRecordingArtifact(rtcSessionId);
  }
}
