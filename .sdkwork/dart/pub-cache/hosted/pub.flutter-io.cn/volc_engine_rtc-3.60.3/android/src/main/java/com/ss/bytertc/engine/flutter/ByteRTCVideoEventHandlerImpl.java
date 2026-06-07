// Copyright (c) 2026 Beijing Volcano Engine Technology Ltd.

package com.ss.bytertc.engine.flutter;

import com.ss.bytertc.engine.live.MixedStreamLayoutRegionImageWaterMarkConfig;
import com.ss.bytertc.engine.data.AudioRoute;
import com.ss.bytertc.engine.data.AudioVADType;
import com.ss.bytertc.engine.data.DeadLockMsg;
import com.ss.bytertc.engine.data.FrameUpdateInfo;
import com.ss.bytertc.engine.data.LocalAudioPropertiesInfo;
import com.ss.bytertc.engine.live.MixedStreamTaskErrorCode;
import com.ss.bytertc.engine.live.MixedStreamTaskEvent;
import com.ss.bytertc.engine.live.MixedStreamTaskInfo;
import com.ss.bytertc.engine.live.SingleStreamTaskErrorCode;
import com.ss.bytertc.engine.live.SingleStreamTaskEvent;
import com.ss.bytertc.engine.type.EffectErrorType;
import com.ss.bytertc.engine.type.LocalProxyState;
import com.ss.bytertc.engine.type.LocalProxyType;
import com.ss.bytertc.engine.type.LocalProxyError;
import com.ss.bytertc.engine.data.RecordingInfo;
import com.ss.bytertc.engine.data.RecordingProgress;
import com.ss.bytertc.engine.data.RemoteAudioPropertiesInfo;
import com.ss.bytertc.engine.data.StreamInfo;
import com.ss.bytertc.engine.type.SnapshotErrorCode;
import com.ss.bytertc.engine.SysStats;
import com.ss.bytertc.engine.data.LocalAudioStreamState;
import com.ss.bytertc.engine.data.LocalAudioStreamError;
import com.ss.bytertc.engine.data.RemoteAudioState;
import com.ss.bytertc.engine.data.RemoteAudioStateChangeReason;
import com.ss.bytertc.engine.data.StreamSyncInfoConfig;
import com.ss.bytertc.engine.data.VideoFrameInfo;
import com.ss.bytertc.engine.data.VideoSuperResolutionMode;
import com.ss.bytertc.engine.data.VideoSuperResolutionModeChangedReason;
import com.ss.bytertc.engine.data.VideoDenoiseMode;
import com.ss.bytertc.engine.data.VideoDenoiseModeChangedReason;
import com.ss.bytertc.engine.type.AudioDeviceType;
import com.ss.bytertc.engine.type.AudioDumpStatus;
import com.ss.bytertc.engine.type.AudioRecordingErrorCode;
import com.ss.bytertc.engine.type.AudioRecordingState;
import com.ss.bytertc.engine.type.EchoTestResult;
import com.ss.bytertc.engine.type.FirstFramePlayState;
import com.ss.bytertc.engine.type.FirstFrameSendState;
import com.ss.bytertc.engine.type.LocalVideoStreamError;
import com.ss.bytertc.engine.type.LocalVideoStreamState;
import com.ss.bytertc.engine.type.NetworkDetectionLinkType;
import com.ss.bytertc.engine.type.NetworkDetectionStopReason;
import com.ss.bytertc.engine.type.PerformanceAlarmMode;
import com.ss.bytertc.engine.type.PerformanceAlarmReason;
import com.ss.bytertc.engine.type.PublicStreamErrorCode;
import com.ss.bytertc.engine.type.RecordingErrorCode;
import com.ss.bytertc.engine.type.RecordingState;
import com.ss.bytertc.engine.type.RemoteStreamSwitch;
import com.ss.bytertc.engine.type.RemoteVideoState;
import com.ss.bytertc.engine.type.RenderError;
import com.ss.bytertc.engine.type.SEIStreamUpdateEvent;
import com.ss.bytertc.engine.type.RemoteVideoStateChangeReason;
import com.ss.bytertc.engine.type.RtcUser;
import com.ss.bytertc.engine.type.SourceWantedData;
import com.ss.bytertc.engine.type.VideoDeviceType;
import com.ss.bytertc.engine.type.HardwareEchoDetectionResult;
import com.ss.bytertc.engine.type.AudioAEDType;
import com.ss.bytertc.engine.utils.LogUtil.LogLevel;
import com.ss.bytertc.engine.IAudioSource;
import com.ss.bytertc.engine.IVideoSource;
import java.nio.ByteBuffer;
import org.json.JSONObject;
import com.ss.bytertc.engine.handler.IRTCEngineEventHandler;
import com.volcengine.VolcApiEngine.BeanFactory;

public class ByteRTCVideoEventHandlerImpl extends IRTCEngineEventHandler implements BeanFactory.EventReceiver {
  public BeanFactory.EventEmitter ee;

  public ByteRTCVideoEventHandlerImpl(BeanFactory.EventEmitter ee) {
    this.ee = ee;
  }

  @Override
  public void onLoggerMessage(LogLevel level, String msg, Throwable throwable) {
    this.ee.sendEvent("onLoggerMessage", level, msg, throwable);
  }

  @Override
  public void onWarning(int warn) {
    this.ee.sendEvent("onWarning", warn);
  }

  @Override
  public void onError(int err) {
    this.ee.sendEvent("onError", err);
  }

  @Override
  public void onDeadLockError(DeadLockMsg deadLockMsg) {
    this.ee.sendEvent("onDeadLockError", deadLockMsg);
  }

  @Override
  public void onExtensionAccessError(String extensionName, String msg) {
    this.ee.sendEvent("onExtensionAccessError", extensionName, msg);
  }

  @Override
  public void onSysStats(SysStats stats) {
    this.ee.sendEvent("onSysStats", stats);
  }

  @Override
  public void onNetworkTypeChanged(int type) {
    this.ee.sendEvent("onNetworkTypeChanged", type);
  }

  @Override
  public void onUserStartVideoCapture(String streamId, StreamInfo streamInfo) {
    this.ee.sendEvent("onUserStartVideoCapture", streamId, streamInfo);
  }

  @Override
  public void onUserStopVideoCapture(String streamId, StreamInfo streamInfo) {
    this.ee.sendEvent("onUserStopVideoCapture", streamId, streamInfo);
  }

  @Override
  public void onUserStartAudioCapture(String streamId, StreamInfo streamInfo) {
    this.ee.sendEvent("onUserStartAudioCapture", streamId, streamInfo);
  }

  @Override
  public void onUserStopAudioCapture(String streamId, StreamInfo streamInfo) {
    this.ee.sendEvent("onUserStopAudioCapture", streamId, streamInfo);
  }

  @Override
  public void onLocalAudioStateChanged(IAudioSource audioSource, LocalAudioStreamState state, LocalAudioStreamError error) {
    this.ee.sendEvent("onLocalAudioStateChanged", audioSource, state, error);
  }

  @Override
  public void onRemoteAudioStateChanged(String streamId, StreamInfo streamInfo, RemoteAudioState state, RemoteAudioStateChangeReason reason) {
    this.ee.sendEvent("onRemoteAudioStateChanged", streamId, streamInfo, state, reason);
  }

  @Override
  public void onLocalVideoStateChanged(IVideoSource videoSource, LocalVideoStreamState state, LocalVideoStreamError error) {
    this.ee.sendEvent("onLocalVideoStateChanged", "source", state, error);
  }

  @Override
  public void onRemoteVideoStateChanged(String streamId, StreamInfo streamInfo, RemoteVideoState videoState, RemoteVideoStateChangeReason videoStateReason) {
    this.ee.sendEvent("onRemoteVideoStateChanged", streamId, streamInfo, videoState, videoStateReason);
  }

  @Override
  public void onRemoteVideoSuperResolutionModeChanged(String streamId, StreamInfo streamInfo, VideoSuperResolutionMode mode, VideoSuperResolutionModeChangedReason reason) {
    this.ee.sendEvent("onRemoteVideoSuperResolutionModeChanged", streamId, streamInfo, mode, reason);
  }

  @Override
  public void onVideoDenoiseModeChanged(VideoDenoiseMode mode, VideoDenoiseModeChangedReason reason) {
    this.ee.sendEvent("onVideoDenoiseModeChanged", mode, reason);
  }

  @Override
  public void onFirstRemoteVideoFrameRendered(String streamId, StreamInfo streamInfo, VideoFrameInfo frameInfo) {
    this.ee.sendEvent("onFirstRemoteVideoFrameRendered", streamId, streamInfo, frameInfo);
  }

  @Override
  public void onFirstRemoteVideoFrameDecoded(String streamId, StreamInfo streamInfo, VideoFrameInfo frameInfo) {
    this.ee.sendEvent("onFirstRemoteVideoFrameDecoded", streamId, streamInfo, frameInfo);
  }

  @Override
  public void onFirstLocalVideoFrameCaptured(IVideoSource videoSource, VideoFrameInfo frameInfo) {
    this.ee.sendEvent("onFirstLocalVideoFrameCaptured", "source", frameInfo);
  }

  @Override
  public void onLocalVideoSizeChanged(IVideoSource videoSource, VideoFrameInfo frameInfo) {
    this.ee.sendEvent("onLocalVideoSizeChanged", "source", frameInfo);
  }

  @Override
  public void onRemoteVideoSizeChanged(String streamId, StreamInfo streamInfo, VideoFrameInfo frameInfo) {
    this.ee.sendEvent("onRemoteVideoSizeChanged", streamId, streamInfo, frameInfo);
  }

  @Override
  public void onConnectionStateChanged(int state, int reason) {
    this.ee.sendEvent("onConnectionStateChanged", state, reason);
  }

  @Override
  public void onAudioRouteChanged(AudioRoute route) {
    this.ee.sendEvent("onAudioRouteChanged", route);
  }

  @Override
  public void onFirstLocalAudioFrame(IAudioSource audioSource) {
    this.ee.sendEvent("onFirstLocalAudioFrame", audioSource);
  }

  @Override
  public void onFirstRemoteAudioFrame(String streamId, StreamInfo streamInfo) {
    this.ee.sendEvent("onFirstRemoteAudioFrame", streamId, streamInfo);
  }

  @Override
  public void onLogReport(String logType, JSONObject logContent) {
    this.ee.sendEvent("onLogReport", logType, logContent);
  }

  @Override
  public void onSEIMessageReceived(String streamId, StreamInfo streamInfo, ByteBuffer message) {
    this.ee.sendEvent("onSEIMessageReceived", streamId, streamInfo, message);
  }

  @Override
  public void onSEIStreamUpdate(String streamId, StreamInfo streamInfo, SEIStreamUpdateEvent event) {
    this.ee.sendEvent("onSEIStreamUpdate", streamId, streamInfo, event);
  }

  @Override
  public void onLoginResult(String uid, int errorCode, int elapsed) {
    this.ee.sendEvent("onLoginResult", uid, errorCode, elapsed);
  }

  @Override
  public void onLogout(int reason) {
    this.ee.sendEvent("onLogout", reason);
  }

  @Override
  public void onServerParamsSetResult(int error) {
    this.ee.sendEvent("onServerParamsSetResult", error);
  }

  @Override
  public void onGetPeerOnlineStatus(String peerUserId, int status) {
    this.ee.sendEvent("onGetPeerOnlineStatus", peerUserId, status);
  }

  @Override
  public void onUserMessageReceivedOutsideRoom(String uid, String message) {
    this.ee.sendEvent("onUserMessageReceivedOutsideRoom", uid, message);
  }

  @Override
  public void onUserBinaryMessageReceivedOutsideRoom(String uid, ByteBuffer message) {
    this.ee.sendEvent("onUserBinaryMessageReceivedOutsideRoom", uid, message);
  }

  @Override
  public void onUserMessageReceivedOutsideRoom(long msgid, String uid, String message) {
    this.ee.sendEvent("onUserMessageReceivedOutsideRoom", msgid, uid, message);
  }

  @Override
  public void onUserBinaryMessageReceivedOutsideRoom(long msgid, String uid, ByteBuffer message) {
    this.ee.sendEvent("onUserBinaryMessageReceivedOutsideRoom", msgid, uid, message);
  }

  @Override
  public void onUserMessageSendResultOutsideRoom(long msgid, int error) {
    this.ee.sendEvent("onUserMessageSendResultOutsideRoom", msgid, error);
  }

  @Override
  public void onServerMessageSendResult(long msgid, int error, ByteBuffer message) {
    this.ee.sendEvent("onServerMessageSendResult", msgid, error, message);
  }

  @Override
  public void onNetworkDetectionResult(NetworkDetectionLinkType type, int quality, int rtt, double lostRate, int bitrate, int jitter) {
    this.ee.sendEvent("onNetworkDetectionResult", type, quality, rtt, lostRate, bitrate, jitter);
  }

  @Override
  public void onNetworkDetectionStopped(NetworkDetectionStopReason reason) {
    this.ee.sendEvent("onNetworkDetectionStopped", reason);
  }

  @Override
  public void onAudioDeviceStateChanged(String deviceID, AudioDeviceType deviceType, int deviceState, int deviceError) {
    this.ee.sendEvent("onAudioDeviceStateChanged", deviceID, deviceType, deviceState, deviceError);
  }

  @Override
  public void onVideoDeviceStateChanged(String deviceID, VideoDeviceType deviceType, int deviceState, int deviceError) {
    this.ee.sendEvent("onVideoDeviceStateChanged", deviceID, deviceType, deviceState, deviceError);
  }

  @Override
  public void onAudioDeviceWarning(String deviceID, AudioDeviceType deviceType, int deviceWarning) {
    this.ee.sendEvent("onAudioDeviceWarning", deviceID, deviceType, deviceWarning);
  }

  @Override
  public void onVideoDeviceWarning(String deviceID, VideoDeviceType deviceType, int deviceWarning) {
    this.ee.sendEvent("onVideoDeviceWarning", deviceID, deviceType, deviceWarning);
  }

  @Override
  public void onRecordingStateUpdate(IVideoSource videoSource, RecordingState state, RecordingErrorCode errorCode, RecordingInfo info) {
    this.ee.sendEvent("onRecordingStateUpdate", "source", state, errorCode, info);
  }

  @Override
  public void onRecordingProgressUpdate(IVideoSource videoSource, RecordingProgress progress, RecordingInfo info) {
    this.ee.sendEvent("onRecordingProgressUpdate", "source", progress, info);
  }

  @Override
  public void onAudioRecordingStateUpdate(AudioRecordingState state, AudioRecordingErrorCode errorCode) {
    this.ee.sendEvent("onAudioRecordingStateUpdate", state, errorCode);
  }

  @Override
  public void onAudioMixingPlayingProgress(int mixId, long progress) {
    this.ee.sendEvent("onAudioMixingPlayingProgress", mixId, progress);
  }

  @Override
  public void onLocalAudioPropertiesReport(LocalAudioPropertiesInfo[] audioPropertiesInfos) {
    this.ee.sendEvent("onLocalAudioPropertiesReport", new Object[]{audioPropertiesInfos});
  }

  @Override
  public void onAudioVADStateUpdate(AudioVADType state) {
    this.ee.sendEvent("onAudioVADStateUpdate", state);
  }

  @Override
  public void onAudioAEDStateUpdate(AudioAEDType state) {
    this.ee.sendEvent("onAudioAEDStateUpdate", state);
  }

  @Override
  public void onAudioPlaybackDeviceTestVolume(int volume) {
    this.ee.sendEvent("onAudioPlaybackDeviceTestVolume", volume);
  }

  @Override
  public void onRemoteAudioPropertiesReport(RemoteAudioPropertiesInfo[] audioPropertiesInfos, int totalRemoteVolume) {
    this.ee.sendEvent("onRemoteAudioPropertiesReport", audioPropertiesInfos, totalRemoteVolume);
  }

  @Override
  public void onActiveSpeaker(String roomId, String uid) {
    this.ee.sendEvent("onActiveSpeaker", roomId, uid);
  }

  @Override
  public void onPushPublicStreamResult(String roomId, String publicStreamId, PublicStreamErrorCode error) {
    this.ee.sendEvent("onPushPublicStreamResult", roomId, publicStreamId, error);
  }

  @Override
  public void onEchoTestResult(EchoTestResult result) {
    this.ee.sendEvent("onEchoTestResult", result);
  }

  @Override
  public void onCloudProxyConnected(int interval) {
    this.ee.sendEvent("onCloudProxyConnected", interval);
  }

  @Override
  public void onAudioDumpStateChanged(AudioDumpStatus status) {
    this.ee.sendEvent("onAudioDumpStateChanged", status);
  }

  @Override
  public void onNetworkTimeSynchronized() {
    this.ee.sendEvent("onNetworkTimeSynchronized");
  }

  @Override
  public void onLicenseWillExpire(int days) {
    this.ee.sendEvent("onLicenseWillExpire", days);
  }

  @Override
  public void onHardwareEchoDetectionResult(HardwareEchoDetectionResult hardwareEchoDetectionResult) {
    this.ee.sendEvent("onHardwareEchoDetectionResult", hardwareEchoDetectionResult);
  }

  @Override
  public void onLocalProxyStateChanged(LocalProxyType localProxyType, LocalProxyState localProxyState, LocalProxyError localProxyError) {
    this.ee.sendEvent("onLocalProxyStateChanged", localProxyType, localProxyState, localProxyError);
  }

  @Override
  public void onEffectError(EffectErrorType error, String msg) {
    this.ee.sendEvent("onEffectError", error, msg);
  }

  @Override
  public void onStreamSyncInfoReceived(String streamId, StreamInfo streamInfo, StreamSyncInfoConfig.SyncInfoStreamType streamType, ByteBuffer data) {
    this.ee.sendEvent("onStreamSyncInfoReceived", streamId, streamInfo, streamType, data);
  }

  @Override
  public void onRemoteRenderError(String streamId, StreamInfo streamInfo, RenderError error, String message) {
    this.ee.sendEvent("onRemoteRenderError", streamId, streamInfo, error, message);
  }

  @Override
  public void onExternalScreenFrameUpdate(FrameUpdateInfo info) {
    this.ee.sendEvent("onExternalScreenFrameUpdate", info);
  }

  @Override
  public void onRemoteSnapshotTakenToFile(String streamId, StreamInfo streamInfo, String filePath, int width, int height, SnapshotErrorCode errorCode, long taskId) {
    this.ee.sendEvent("onRemoteSnapshotTakenToFile", streamId, streamInfo, filePath, width, height, errorCode, taskId);
  }

  @Override
  public void onLocalSnapshotTakenToFile(IVideoSource videoSource, String filePath, int width, int height, SnapshotErrorCode errorCode, long taskId) {
    this.ee.sendEvent("onLocalSnapshotTakenToFile", "source", filePath, width, height, errorCode, taskId);
  }

  @Override
  public void onAudioFrameSendStateChanged(String streamId, StreamInfo streamInfo, RtcUser user, FirstFrameSendState state) {
    this.ee.sendEvent("onAudioFrameSendStateChanged", streamId, streamInfo, user, state);
  }

  @Override
  public void onVideoFrameSendStateChanged(String streamId, StreamInfo streamInfo, RtcUser user, FirstFrameSendState state) {
    this.ee.sendEvent("onVideoFrameSendStateChanged", streamId, streamInfo, user, state);
  }

  @Override
  public void onAudioFramePlayStateChanged(String streamId, StreamInfo streamInfo, RtcUser user, FirstFramePlayState state) {
    this.ee.sendEvent("onAudioFramePlayStateChanged", streamId, streamInfo, user, state);
  }

  @Override
  public void onVideoFramePlayStateChanged(String streamId, StreamInfo streamInfo, RtcUser user, FirstFramePlayState state) {
    this.ee.sendEvent("onVideoFramePlayStateChanged", streamId, streamInfo, user, state);
  }

  @Override
  public void onSimulcastSubscribeFallback(String streamId, StreamInfo streamInfo, RemoteStreamSwitch event) {
    this.ee.sendEvent("onSimulcastSubscribeFallback", streamId, streamInfo, event);
  }

  @Override
  public void onPerformanceAlarms(String streamId, StreamInfo streamInfo, PerformanceAlarmMode mode, PerformanceAlarmReason reason, SourceWantedData data) {
    this.ee.sendEvent("onPerformanceAlarms", streamId, streamInfo, mode, reason, data);
  }

  @Override
  public void onRemoteAudioPropertiesReportEx(RemoteAudioPropertiesInfo[] audioPropertiesInfos) {
    this.ee.sendEvent("onRemoteAudioPropertiesReportEx", new Object[]{audioPropertiesInfos});
  }

  @Override
  public void onMixedStreamEvent(MixedStreamTaskInfo info, MixedStreamTaskEvent event, MixedStreamTaskErrorCode error) {
    this.ee.sendEvent("onMixedStreamEvent", info, event, error);
  }

  @Override
  public void onSingleStreamEvent(String taskId, SingleStreamTaskEvent event, SingleStreamTaskErrorCode error) {
    this.ee.sendEvent("onSingleStreamEvent", taskId, event, error);
  }

  @Override
  public void onExperimentalCallback(String param) {
    this.ee.sendEvent("onExperimentalCallback", param);
  }

}