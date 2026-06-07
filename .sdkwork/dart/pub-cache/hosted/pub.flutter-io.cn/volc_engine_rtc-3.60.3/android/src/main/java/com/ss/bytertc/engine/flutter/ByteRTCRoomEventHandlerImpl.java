// Copyright (c) 2026 Beijing Volcano Engine Technology Ltd.

package com.ss.bytertc.engine.flutter;

import com.ss.bytertc.engine.RTCStream;
import com.ss.bytertc.engine.SubscribeConfig;
import com.ss.bytertc.engine.UserInfo;
import com.ss.bytertc.engine.data.ForwardStreamEventInfo;
import com.ss.bytertc.engine.data.ForwardStreamStateInfo;
import com.ss.bytertc.engine.data.PublishState;
import com.ss.bytertc.engine.data.PublishStateChangeReason;
import com.ss.bytertc.engine.data.StreamInfo;
import com.ss.bytertc.engine.data.SubscribeState;
import com.ss.bytertc.engine.data.SubscribeStateChangeReason;
import com.ss.bytertc.engine.data.AVSyncState;
import com.ss.bytertc.engine.data.AVSyncEvent;
import com.ss.bytertc.engine.handler.IRTCRoomEventHandler;
import com.ss.bytertc.engine.type.LocalStreamStats;
import com.ss.bytertc.engine.type.NetworkQualityStats;
import com.ss.bytertc.engine.type.RTCRoomStats;
import com.ss.bytertc.engine.type.RemoteStreamStats;
import com.ss.bytertc.engine.type.RoomEvent;
import com.ss.bytertc.engine.type.RoomEventInfo;
import com.ss.bytertc.engine.type.RoomStateChangeReason;
import com.ss.bytertc.engine.type.RoomState;
import com.ss.bytertc.engine.type.SetRoomExtraInfoResult;
import com.ss.bytertc.engine.type.UserVisibilityChangeError;
import com.ss.bytertc.engine.type.StreamRemoveReason;
import com.ss.bytertc.engine.type.SubtitleMessage;
import com.ss.bytertc.engine.type.SubtitleErrorCode;
import com.ss.bytertc.engine.type.SubtitleState;
import java.nio.ByteBuffer;
import com.volcengine.VolcApiEngine.BeanFactory;

public class ByteRTCRoomEventHandlerImpl extends IRTCRoomEventHandler implements BeanFactory.EventReceiver {
  public BeanFactory.EventEmitter ee;

  public ByteRTCRoomEventHandlerImpl(BeanFactory.EventEmitter ee) {
    this.ee = ee;
  }

  @Override
  public void onLeaveRoom(RTCRoomStats stats) {
      this.ee.sendEvent("onLeaveRoom", stats);
  }

  @Override
  public void onRoomStateChangedWithReason(String roomId, String uid, RoomState state, RoomStateChangeReason reason) {
     this.ee.sendEvent("onRoomStateChangedWithReason", roomId, uid, state, reason);
  }

  @Override
  public void onRoomStateChanged(String roomId, String uid, int state, String extraInfo) {
     this.ee.sendEvent("onRoomStateChanged", roomId, uid, state, extraInfo);
  }

  @Override
  public void onStreamStateChanged(String roomId, String uid, int state, String extraInfo) {
    this.ee.sendEvent("onStreamStateChanged", roomId, uid, state, extraInfo);
  }

  @Override
  public void onRoomWarning(int warn) {
     this.ee.sendEvent("onRoomWarning", warn);
  }

  @Override
  public void onAVSyncStateChange(AVSyncState state) {
     this.ee.sendEvent("onAVSyncStateChange", state);
  }

  @Override
  public void onRoomStats(RTCRoomStats stats) {
     this.ee.sendEvent("onRoomStats", stats);
  }

  @Override
  public void onRoomEvent(String roomId, String uid, RoomEvent state, RoomEventInfo info) {
     this.ee.sendEvent("onRoomEvent", roomId, uid, state, info);
  }

  @Override
  public void onUserJoined(UserInfo userInfo) {
      this.ee.sendEvent("onUserJoined", userInfo);
  }

  @Override
  public void onUserLeave(String uid, int reason) {
      this.ee.sendEvent("onUserLeave", uid, reason);
  }

  @Override
  public void onTokenWillExpire() {
     this.ee.sendEvent("onTokenWillExpire");
  }

  @Override
  public void onPublishPrivilegeTokenWillExpire() {
     this.ee.sendEvent("onPublishPrivilegeTokenWillExpire");
  }

  @Override
  public void onSubscribePrivilegeTokenWillExpire() {
     this.ee.sendEvent("onSubscribePrivilegeTokenWillExpire");
  }

  @Override
  public void onVideoPublishStateChanged(String streamId, StreamInfo streamInfo, PublishState state, PublishStateChangeReason reason) {
     this.ee.sendEvent("onVideoPublishStateChanged", streamId, streamInfo, state, reason);
  }

  @Override
  public void onAudioPublishStateChanged(String streamId, StreamInfo streamInfo, PublishState state, PublishStateChangeReason reason) {
     this.ee.sendEvent("onAudioPublishStateChanged", streamId, streamInfo, state, reason);
  }

  @Override
  public void onVideoSubscribeStateChanged(String streamId, StreamInfo streamInfo, SubscribeState state, SubscribeStateChangeReason reason) {
     this.ee.sendEvent("onVideoSubscribeStateChanged", streamId, streamInfo, state, reason);
  }

  @Override
  public void onAudioSubscribeStateChanged(String streamId, StreamInfo streamInfo, SubscribeState state, SubscribeStateChangeReason reason) {
    this.ee.sendEvent("onAudioSubscribeStateChanged", streamId, streamInfo, state, reason);
  }

  @Override
  public void onLocalStreamStats(String streamId, StreamInfo streamInfo, LocalStreamStats stats) {
    this.ee.sendEvent("onLocalStreamStats", streamId, streamInfo, stats);
  }

  @Override
  public void onRemoteStreamStats(String streamId, StreamInfo streamInfo, RemoteStreamStats stats) {
    this.ee.sendEvent("onRemoteStreamStats", streamId, streamInfo, stats);
  }

  @Override
  public void onStreamRemove(RTCStream stream, StreamRemoveReason reason) {
    this.ee.sendEvent("onStreamRemove", stream, reason);
  }

  @Override
  public void onStreamAdd(RTCStream stream) {
    this.ee.sendEvent("onStreamAdd", stream);
  }

  @Override
  public void onStreamSubscribed(int stateCode, String userId, SubscribeConfig info) {
     this.ee.sendEvent("onStreamSubscribed", stateCode, userId, info);
  }

  @Override
  public void onStreamPublishSuccess(String uid, boolean isScreen) {
     this.ee.sendEvent("onStreamPublishSuccess", uid, isScreen);
  }

  @Override
  public void onAVSyncEvent(String roomId, String uid, AVSyncEvent eventCode) {
     this.ee.sendEvent("onAVSyncEvent", roomId, uid, eventCode);
  }

  @Override
  public void onUserPublishStreamVideo(String streamId, StreamInfo streamInfo, boolean isPublish) {
     this.ee.sendEvent("onUserPublishStreamVideo", streamId, streamInfo, isPublish);
  }

  @Override
  public void onUserPublishStreamAudio(String streamId, StreamInfo streamInfo, boolean isPublish) {
     this.ee.sendEvent("onUserPublishStreamAudio", streamId, streamInfo, isPublish);
  }

  @Override
  public void onRoomMessageReceived(String uid, String message) {
     this.ee.sendEvent("onRoomMessageReceived", uid, message);
  }

  @Override
  public void onRoomBinaryMessageReceived(String uid, ByteBuffer message) {
     this.ee.sendEvent("onRoomBinaryMessageReceived", uid, message);
  }

  @Override
  public void onUserMessageReceived(String uid, String message) {
     this.ee.sendEvent("onUserMessageReceived", uid, message);
  }

  @Override
  public void onUserBinaryMessageReceived(String uid, ByteBuffer message) {
     this.ee.sendEvent("onUserBinaryMessageReceived", uid, message);
  }

  @Override
  public void onRoomMessageReceived(long msgid, String uid, String message) {
     this.ee.sendEvent("onRoomMessageReceived", msgid, uid, message);
  }

  @Override
  public void onRoomBinaryMessageReceived(long msgid, String uid, ByteBuffer message) {
     this.ee.sendEvent("onRoomBinaryMessageReceived", msgid, uid, message);
  }

  @Override
  public void onUserMessageReceived(long msgid, String uid, String message) {
     this.ee.sendEvent("onUserMessageReceived", msgid, uid, message);
  }

  @Override
  public void onUserBinaryMessageReceived(long msgid, String uid, ByteBuffer message) {
     this.ee.sendEvent("onUserBinaryMessageReceived", msgid, uid, message);
  }

  @Override
  public void onUserMessageSendResult(long msgid, int error) {
     this.ee.sendEvent("onUserMessageSendResult", msgid, error);
  }

  @Override
  public void onRoomMessageSendResult(long msgid, int error) {
     this.ee.sendEvent("onRoomMessageSendResult", msgid, error);
  }

  @Override
  public void onVideoStreamBanned(String uid, boolean banned) {
     this.ee.sendEvent("onVideoStreamBanned", uid, banned);
  }

  @Override
  public void onAudioStreamBanned(String uid, boolean banned) {
     this.ee.sendEvent("onAudioStreamBanned", uid, banned);
  }

  @Override
  public void onForwardStreamStateChanged(ForwardStreamStateInfo[] stateInfos) {
    this.ee.sendEvent("onForwardStreamStateChanged", new Object[]{stateInfos});
  }

  @Override
  public void onForwardStreamEvent(ForwardStreamEventInfo[] eventInfos) {
    this.ee.sendEvent("onForwardStreamEvent", new Object[]{eventInfos});
  }

  @Override
  public void onNetworkQuality(NetworkQualityStats localQuality, NetworkQualityStats[] remoteQualities) {
     this.ee.sendEvent("onNetworkQuality", localQuality, remoteQualities);
  }

  @Override
  public void onSetRoomExtraInfoResult(long taskId, SetRoomExtraInfoResult result) {
     this.ee.sendEvent("onSetRoomExtraInfoResult", taskId, result);
  }

  @Override
  public void onRoomExtraInfoUpdate(String key,String value,String lastUpdateUserId, long lastUpdateTimeMs) {
     this.ee.sendEvent("onRoomExtraInfoUpdate", key, value, lastUpdateUserId, lastUpdateTimeMs);
  }

  @Override
  public void onRoomStreamExtraInfoUpdate(String streamId, StreamInfo streamInfo, String extraInfo) {
     this.ee.sendEvent("onRoomStreamExtraInfoUpdate", streamId, streamInfo, extraInfo);
  }

  @Override
  public void onUserVisibilityChanged(boolean currentUserVisibility, UserVisibilityChangeError errorCode) {
     this.ee.sendEvent("onUserVisibilityChanged", currentUserVisibility, errorCode);
  }

  @Override
  public void onSubtitleStateChanged(SubtitleState state, SubtitleErrorCode errorCode, String errorMessage) {
     this.ee.sendEvent("onSubtitleStateChanged", state, errorCode, errorMessage);
  }

  @Override
  public void onSubtitleMessageReceived(SubtitleMessage[] subtitles) {
     this.ee.sendEvent("onSubtitleMessageReceived", new Object[]{subtitles});
  }

}