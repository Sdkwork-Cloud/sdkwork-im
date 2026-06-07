/// Copyright (c) 2026 Beijing Volcano Engine Technology Ltd.

// ignore_for_file: camel_case_types, annotate_overrides, null_check_always_fails, unused_import, non_constant_identifier_names
import 'package:hybrid_runtime/hybrid_runtime.dart';
import 'dart:async';
import 'dart:typed_data';
import 'keytype.dart';
import 'types.dart';
import 'errorcode.dart';

class IMediaPlayerAudioFrameObserver extends NativeObserverClass {
  static const _$namespace =
      r'com.ss.bytertc.engine.IMediaPlayerAudioFrameObserver';
  static get codegen_$namespace => _$namespace;

  IMediaPlayerAudioFrameObserver([NativeClassOptions? options])
      : super(options == null
            ? const NativeClassOptions([],
                className: _$namespace,
                instanceType: InstanceType.manual,
                bridgeKey: 'com.volcengine.rtc.hybrid_runtime',
                methodMap: {r"onFrame": r"onFrame"})
            : NativeClassOptions.fromMap({
                ...options.toMap(),
                'bridgeKey': 'com.volcengine.rtc.hybrid_runtime',
              })) {
    registerEvent(r"onFrame", onFrame);
  }

  /// @detail callback
  /// @brief 当本地音频文件混音时，回调播放的音频帧。
  /// @param playerId 播放器 ID
  /// @param frame 参看 IAudioFrame{@link #IAudioFrame}。
  ///

  FutureOr<void> onFrame(int playerId, IAudioFrame frame) async {}
}

class IAudioFrameObserver extends NativeObserverClass {
  static const _$namespace = r'com.ss.bytertc.engine.IAudioFrameObserver';
  static get codegen_$namespace => _$namespace;

  IAudioFrameObserver([NativeClassOptions? options])
      : super(options == null
            ? const NativeClassOptions([],
                className: _$namespace,
                instanceType: InstanceType.manual,
                bridgeKey: 'com.volcengine.rtc.hybrid_runtime',
                methodMap: {
                  r"onRecordAudioFrame": r"onRecordAudioFrame",
                  r"onPlaybackAudioFrame": r"onPlaybackAudioFrame",
                  r"onRemoteUserAudioFrame": r"onRemoteUserAudioFrame",
                  r"onMixedAudioFrame": r"onMixedAudioFrame",
                  r"onCaptureMixedAudioFrame": r"onCaptureMixedAudioFrame"
                })
            : NativeClassOptions.fromMap({
                ...options.toMap(),
                'bridgeKey': 'com.volcengine.rtc.hybrid_runtime',
              })) {
    registerEvent(r"onRecordAudioFrame", onRecordAudioFrame);

    registerEvent(r"onPlaybackAudioFrame", onPlaybackAudioFrame);

    registerEvent(r"onRemoteUserAudioFrame", onRemoteUserAudioFrame);

    registerEvent(r"onMixedAudioFrame", onMixedAudioFrame);

    registerEvent(r"onCaptureMixedAudioFrame", onCaptureMixedAudioFrame);
  }

  /// @detail callback
  /// @author wangjunzheng
  /// @brief 返回麦克风录制的音频数据
  /// @param audioFrame 音频数据, 参看 IAudioFrame{@link #IAudioFrame}。
  ///

  FutureOr<void> onRecordAudioFrame(IAudioFrame audioFrame) async {}

  /// @detail callback
  /// @author wangjunzheng
  /// @brief 返回订阅的所有远端用户混音后的音频数据。
  /// @param audioFrame 音频数据, 参看 IAudioFrame{@link #IAudioFrame}。
  ///

  FutureOr<void> onPlaybackAudioFrame(IAudioFrame audioFrame) async {}

  /// @detail callback
  /// @author wangjunzheng
  /// @brief 返回远端单个用户的音频数据
  /// @param streamId 远端流 ID。
  /// @param streamInfo 远端流信息，参看 StreamInfo{@link #StreamInfo}。
  /// @param audioFrame 音频数据，参看 IAudioFrame{@link #IAudioFrame}。
  /// @note 此回调在播放线程调用。不要在此回调中做任何耗时的事情，否则可能会影响整个音频播放链路。
  ///

  FutureOr<void> onRemoteUserAudioFrame(
      String streamId, StreamInfo streamInfo, IAudioFrame audioFrame) async {}

  /// @detail callback
  /// @author wangjunzheng
  /// @brief 返回本地麦克风录制和订阅的所有远端用户混音后的音频数据
  /// @param audioFrame 音频数据, 参看 IAudioFrame{@link #IAudioFrame}。
  ///

  FutureOr<void> onMixedAudioFrame(IAudioFrame audioFrame) async {}

  /// @detail callback
  /// @author huanghao
  /// @brief 返回本地麦克风录制的音频数据，本地 `MediaPlayer` / `EffectPlayer` 播放音频文件混音后的音频数据。
  /// @param audioFrame 音频数据, 参看 IAudioFrame{@link #IAudioFrame}。
  ///

  FutureOr<void> onCaptureMixedAudioFrame(IAudioFrame audioFrame) async {}
}

class ISnapshotResultCallback extends NativeObserverClass {
  static const _$namespace =
      r'com.ss.bytertc.engine.video.ISnapshotResultCallback';
  static get codegen_$namespace => _$namespace;

  ISnapshotResultCallback([NativeClassOptions? options])
      : super(options == null
            ? const NativeClassOptions([],
                className: _$namespace,
                instanceType: InstanceType.manual,
                bridgeKey: 'com.volcengine.rtc.hybrid_runtime',
                methodMap: {
                  r"onTakeRemoteSnapshotResult": r"onTakeRemoteSnapshotResult"
                })
            : NativeClassOptions.fromMap({
                ...options.toMap(),
                'bridgeKey': 'com.volcengine.rtc.hybrid_runtime',
              })) {
    registerEvent(r"onTakeRemoteSnapshotResult", onTakeRemoteSnapshotResult);
  }

  /// @detail callback
  /// @brief 调用 takeRemoteSnapshot{@link #RTCEngine#takeRemoteSnapshot} 截取视频画面时，收到此回调。
  /// @param taskId 远端截图任务的编号。和 takeRemoteSnapshot{@link #RTCEngine#takeRemoteSnapshot} 的返回值一致。
  /// @param streamId 截图的远端流 ID。
  /// @param streamInfo 截图的远端流信息，参看 StreamInfo{@link #StreamInfo}。
  /// @param image 截图。你可以保存为文件，或对其进行二次处理。截图失败时，为空。
  /// @param errorCode 截图错误码： <br>
  ///        - 0: 成功
  ///        - -1: 截图错误。生成图片数据失败或 RGBA 编码失败
  ///        - -2: 截图错误。流无效。
  ///        - -3: 截图错误。截图超时,超时时间 1 秒。
  ///

  FutureOr<void> onTakeRemoteSnapshotResult(long taskId, String streamId,
      StreamInfo streamInfo, Bitmap image, int errorCode) async {}
}

class IRTCRoomEventHandler extends NativeObserverClass {
  static const _$namespace =
      r'com.ss.bytertc.engine.handler.IRTCRoomEventHandler';
  static get codegen_$namespace => _$namespace;

  IRTCRoomEventHandler([NativeClassOptions? options])
      : super(options == null
            ? const NativeClassOptions([],
                className: _$namespace,
                instanceType: InstanceType.manual,
                bridgeKey: 'com.volcengine.rtc.hybrid_runtime',
                methodMap: {
                  r"onLeaveRoom": r"onLeaveRoom",
                  r"onRoomStateChangedWithReason":
                      r"onRoomStateChangedWithReason",
                  r"onRoomStateChanged": r"onRoomStateChanged",
                  r"onStreamStateChanged": r"onStreamStateChanged",
                  r"onAVSyncStateChange": r"onAVSyncStateChange",
                  r"onRoomStats": r"onRoomStats",
                  r"onRoomEvent": r"onRoomEvent",
                  r"onUserJoined": r"onUserJoined",
                  r"onUserLeave": r"onUserLeave",
                  r"onTokenWillExpire": r"onTokenWillExpire",
                  r"onPublishPrivilegeTokenWillExpire":
                      r"onPublishPrivilegeTokenWillExpire",
                  r"onSubscribePrivilegeTokenWillExpire":
                      r"onSubscribePrivilegeTokenWillExpire",
                  r"onVideoPublishStateChanged": r"onVideoPublishStateChanged",
                  r"onAudioPublishStateChanged": r"onAudioPublishStateChanged",
                  r"onVideoSubscribeStateChanged":
                      r"onVideoSubscribeStateChanged",
                  r"onAudioSubscribeStateChanged":
                      r"onAudioSubscribeStateChanged",
                  r"onLocalStreamStats": r"onLocalStreamStats",
                  r"onRemoteStreamStats": r"onRemoteStreamStats",
                  r"onStreamSubscribed": r"onStreamSubscribed",
                  r"onStreamPublishSuccess": r"onStreamPublishSuccess",
                  r"onAVSyncEvent": r"onAVSyncEvent",
                  r"onUserPublishStreamVideo": r"onUserPublishStreamVideo",
                  r"onUserPublishStreamAudio": r"onUserPublishStreamAudio",
                  r"onRoomMessageReceived": r"onRoomMessageReceived",
                  r"onRoomBinaryMessageReceived":
                      r"onRoomBinaryMessageReceived",
                  r"onUserMessageReceived": r"onUserMessageReceived",
                  r"onUserBinaryMessageReceived":
                      r"onUserBinaryMessageReceived",
                  r"onUserMessageSendResult": r"onUserMessageSendResult",
                  r"onRoomMessageSendResult": r"onRoomMessageSendResult",
                  r"onVideoStreamBanned": r"onVideoStreamBanned",
                  r"onAudioStreamBanned": r"onAudioStreamBanned",
                  r"onForwardStreamStateChanged":
                      r"onForwardStreamStateChanged",
                  r"onForwardStreamEvent": r"onForwardStreamEvent",
                  r"onNetworkQuality": r"onNetworkQuality",
                  r"onSetRoomExtraInfoResult": r"onSetRoomExtraInfoResult",
                  r"onRoomExtraInfoUpdate": r"onRoomExtraInfoUpdate",
                  r"onRoomStreamExtraInfoUpdate":
                      r"onRoomStreamExtraInfoUpdate",
                  r"onUserVisibilityChanged": r"onUserVisibilityChanged",
                  r"onSubtitleStateChanged": r"onSubtitleStateChanged",
                  r"onSubtitleMessageReceived": r"onSubtitleMessageReceived",
                  r"onRoomWarning": r"onRoomWarning"
                })
            : NativeClassOptions.fromMap({
                ...options.toMap(),
                'bridgeKey': 'com.volcengine.rtc.hybrid_runtime',
              })) {
    registerEvent(r"onLeaveRoom", onLeaveRoom);

    registerEvent(
        r"onRoomStateChangedWithReason", onRoomStateChangedWithReason);

    registerEvent(r"onRoomStateChanged", onRoomStateChanged);

    registerEvent(r"onStreamStateChanged", onStreamStateChanged);

    registerEvent(r"onAVSyncStateChange", onAVSyncStateChange);

    registerEvent(r"onRoomStats", onRoomStats);

    registerEvent(r"onRoomEvent", onRoomEvent);

    registerEvent(r"onUserJoined", onUserJoined);

    registerEvent(r"onUserLeave", onUserLeave);

    registerEvent(r"onTokenWillExpire", onTokenWillExpire);

    registerEvent(r"onPublishPrivilegeTokenWillExpire",
        onPublishPrivilegeTokenWillExpire);

    registerEvent(r"onSubscribePrivilegeTokenWillExpire",
        onSubscribePrivilegeTokenWillExpire);

    registerEvent(r"onVideoPublishStateChanged", onVideoPublishStateChanged);

    registerEvent(r"onAudioPublishStateChanged", onAudioPublishStateChanged);

    registerEvent(
        r"onVideoSubscribeStateChanged", onVideoSubscribeStateChanged);

    registerEvent(
        r"onAudioSubscribeStateChanged", onAudioSubscribeStateChanged);

    registerEvent(r"onLocalStreamStats", onLocalStreamStats);

    registerEvent(r"onRemoteStreamStats", onRemoteStreamStats);

    registerEvent(r"onStreamSubscribed", onStreamSubscribed);

    registerEvent(r"onStreamPublishSuccess", onStreamPublishSuccess);

    registerEvent(r"onAVSyncEvent", onAVSyncEvent);

    registerEvent(r"onUserPublishStreamVideo", onUserPublishStreamVideo);

    registerEvent(r"onUserPublishStreamAudio", onUserPublishStreamAudio);

    registerEvent(r"onRoomMessageReceived", onRoomMessageReceived);

    registerEvent(r"onRoomBinaryMessageReceived", onRoomBinaryMessageReceived);

    registerEvent(r"onUserMessageReceived", onUserMessageReceived);

    registerEvent(r"onUserBinaryMessageReceived", onUserBinaryMessageReceived);

    registerEvent(r"onUserMessageSendResult", onUserMessageSendResult);

    registerEvent(r"onRoomMessageSendResult", onRoomMessageSendResult);

    registerEvent(r"onVideoStreamBanned", onVideoStreamBanned);

    registerEvent(r"onAudioStreamBanned", onAudioStreamBanned);

    registerEvent(r"onForwardStreamStateChanged", onForwardStreamStateChanged);

    registerEvent(r"onForwardStreamEvent", onForwardStreamEvent);

    registerEvent(r"onNetworkQuality", onNetworkQuality);

    registerEvent(r"onSetRoomExtraInfoResult", onSetRoomExtraInfoResult);

    registerEvent(r"onRoomExtraInfoUpdate", onRoomExtraInfoUpdate);

    registerEvent(r"onRoomStreamExtraInfoUpdate", onRoomStreamExtraInfoUpdate);

    registerEvent(r"onUserVisibilityChanged", onUserVisibilityChanged);

    registerEvent(r"onSubtitleStateChanged", onSubtitleStateChanged);

    registerEvent(r"onSubtitleMessageReceived", onSubtitleMessageReceived);

    registerEvent(r"onRoomWarning", onRoomWarning);
  }

  /// @detail callback
  /// @region 多房间
  /// @author shenpengliang
  /// @brief 离开房间成功回调。 <br>
  ///        用户调用 leaveRoom{@link #RTSRoom#leaveRoom} 方法后，SDK 会停止所有的发布订阅流，并在释放所有通话相关的音视频资源后，通过此回调通知用户离开房间成功。
  /// @param stats 保留参数，目前为空。
  /// @note
  ///       - 用户调用 leaveRoom{@link #RTSRoom#leaveRoom} 方法离开房间后，如果立即调用 destroy{@link #RTCRoom#destroy} 销毁房间实例或 destroyRTCEngine{@link #RTCEngine#destroyRTCEngine} 方法销毁 RTC 引擎，则将无法收到此回调事件。
  ///       - 离开房间后，如果 App 需要使用系统音视频设备，则建议在收到此回调后再初始化音视频设备，否则可能由于 SDK 占用音视频设备导致初始化失败。
  /// @order 2
  ///

  FutureOr<void> onLeaveRoom(RTCRoomStats stats) async {}

  /// @hidden
  /// @detail callback
  /// @author shenpengliang
  /// @brief 房间状态改变回调，加入房间、异常退出房间、发生房间相关的警告或错误时会收到此回调。
  /// @param roomId 房间 ID。
  /// @param uid 用户 ID。
  /// @param state 房间状态码。参见 RoomState{@link #RoomState}。 <br>
  ///              - 0: 加入房间成功。
  ///              - 1: 加入房间失败、异常退房、发生房间相关的警告或错误。
  ///              - 2: 离开房间。
  /// @param reason 房间状态发生变化的原因。参见 RoomStateChangeReason{@link #RoomStateChangeReason}。
  ///

  FutureOr<void> onRoomStateChangedWithReason(String roomId, String uid,
      RoomState state, RoomStateChangeReason reason) async {}

  /// @detail callback
  /// @region 多房间
  /// @author shenpengliang
  /// @brief 房间状态改变回调，加入房间、异常退出房间、发生房间相关的警告或错误时会收到此回调。
  /// @param roomId 房间 ID。
  /// @param uid 用户 ID。
  /// @param state 房间状态码。 <br>
  ///              - 0: 加入房间成功。
  ///              - !0: 加入房间失败、异常退房、发生房间相关的警告或错误。具体原因参看 ErrorCode{@link #ErrorCode} 及 WarningCode{@link #WarningCode}。
  /// @param extraInfo 额外信息，如 `{"elapsed":1187,"join_type":0}`。 <br>
  ///                  `join_type` 表示加入房间的类型，`0`为首次进房，`1`为重连进房。 <br>
  ///                  `elapsed` 表示加入房间耗时，即本地用户从调用 joinRoom{@link #RTCRoom#joinRoom} 到加入房间成功所经历的时间间隔，单位为 ms。
  /// @order 0
  ///

  FutureOr<void> onRoomStateChanged(
      String roomId, String uid, int state, String extraInfo) async {}

  /// @detail callback
  /// @author shenpengliang
  /// @brief 流状态改变回调，发生流相关的警告或错误时会收到此回调。
  /// @param roomId 房间 ID。
  /// @param uid 用户 ID。
  /// @param state 流状态码，参看 ErrorCode{@link #ErrorCode} 及 WarningCode{@link #WarningCode}。
  /// @param extraInfo 附加信息，目前为空。
  ///

  FutureOr<void> onStreamStateChanged(
      String roomId, String uid, int state, String extraInfo) async {}

  /// @detail callback
  /// @author wangzhanqiang
  /// @brief 发布端调用 setMultiDeviceAVSync{@link #RTCRoom#setMultiDeviceAVSync} 后音视频同步状态发生改变时，会收到此回调。
  /// @param state 音视频同步状态，参看 AVSyncState{@link #AVSyncState}。
  ///

  FutureOr<void> onAVSyncStateChange(AVSyncState state) async {}

  /// @detail callback
  /// @author yejing
  /// @brief 房间内通话统计信息回调。 <br>
  ///        用户进房开始通话后，每 2s 收到一次本回调。
  /// @param stats 房间内的汇总统计数据。详见 RTCRoomStats{@link #RTCRoomStats}。
  ///

  FutureOr<void> onRoomStats(RTCRoomStats stats) async {}

  /// @hidden
  /// @detail callback
  /// @region 多房间
  /// @valid since 3.60.
  /// @author taoshasha
  /// @brief 房间事件回调。
  /// @param roomId 房间 ID。
  /// @param uid 用户 ID。
  /// @param state 房间事件状态。详见 RoomEvent{@link #RoomEvent}。
  /// @param info 房间封禁时，包含封禁时间。详见 RoomEventInfo{@link #RoomEventInfo}。
  /// @order 0
  ///

  FutureOr<void> onRoomEvent(
      String roomId, String uid, RoomEvent state, RoomEventInfo info) async {}

  /// @detail callback
  /// @author shenpengliang
  /// @brief 远端可见用户加入房间，或房内不可见用户切换为可见的回调。 <br>
  ///        1.远端用户调用 setUserVisibility{@link #RTCRoom#setUserVisibility} 方法将自身设为可见后加入房间时，房间内其他用户将收到该事件。 <br>
  ///        2.远端可见用户断网后重新连入房间时，房间内其他用户将收到该事件。 <br>
  ///        3.房间内隐身远端用户调用 setUserVisibility{@link #RTCRoom#setUserVisibility} 方法切换至可见时，房间内其他用户将收到该事件。 <br>
  ///        4.新进房用户也会收到进房前已在房内的可见用户的进房回调通知。
  /// @param userInfo 用户信息。参看 UserInfo{@link #UserInfo}。
  ///

  FutureOr<void> onUserJoined(UserInfo userInfo) async {}

  /// @detail callback
  /// @author shenpengliang
  /// @brief 远端用户离开房间，或切至不可见时，房间内其他用户会收到此事件
  /// @param uid 离开房间，或切至不可见的的远端用户 ID。
  /// @param reason 用户离开房间的原因： <br>
  ///              - 0: 远端用户调用 leaveRoom{@link #RTSRoom#leaveRoom} 主动退出房间。
  ///              - 1: 远端用户因 Token 过期或网络原因等掉线。详细信息请参看[连接状态提示](https://www.volcengine.com/docs/6348/95376)
  ///              - 2: 远端用户调用 setUserVisibility{@link #RTCRoom#setUserVisibility} 切换至不可见状态。
  ///              - 3: 服务端调用 OpenAPI 将该远端用户踢出房间。
  ///

  FutureOr<void> onUserLeave(String uid, int reason) async {}

  /// @detail callback
  /// @author shenpengliang
  /// @brief 当 SDK 检测到 Token 的进房权限将在 30 秒内过期时，触发该回调。
  ///        收到该回调后，你需调用 updateToken{@link #RTSRoom#updateToken} 更新 Token 进房权限。
  /// @note 若 Token 进房权限过期且未及时更新： <br>
  ///        - 用户此时尝试进房会收到 onRoomStateChanged{@link #IRTCRoomEventHandler#onRoomStateChanged} 回调，提示错误码为 `-1000` Token 无效；
  ///        - 用户已在房间内则会被移出房间，本地用户会收到 onRoomStateChanged{@link #IRTCRoomEventHandler#onRoomStateChanged} 回调，提示错误码为 `-1009` Token 无效，同时远端用户会收到 onUserLeave{@link #IRTCRoomEventHandler#onUserLeave} 回调，提示原因为 `1` Token 进房权限过期。
  ///

  FutureOr<void> onTokenWillExpire() async {}

  /// @detail callback
  /// @author qipengxiang
  /// @brief Token 发布权限过期前 30 秒将触发该回调。 <br>
  ///        收到该回调后，你需调用 updateToken{@link #RTSRoom#updateToken} 更新 Token 发布权限。
  /// @note  Token 发布权限过期后：
  ///        - 已发布流或尝试发布流时，本端会收到 onVideoPublishStateChanged{@link #IRTCRoomEventHandler#onVideoPublishStateChanged}、onAudioPublishStateChanged{@link #IRTCRoomEventHandler#onAudioPublishStateChanged} 回调，提示`kPublishStateChangeReasonNoPublishPermission`，没有发布权限。
  ///        - 发布中的流将停止发布。远端用户会收到 onUserPublishStreamVideo{@link #IRTCRoomEventHandler#onUserPublishStreamVideo}、onUserPublishStreamAudio{@link #IRTCRoomEventHandler#onUserPublishStreamAudio} 回调，提示该流已停止发布。
  ///

  FutureOr<void> onPublishPrivilegeTokenWillExpire() async {}

  /// @detail callback
  /// @author qipengxiang
  /// @brief Token 订阅权限过期前 30 秒将触发该回调。 <br>
  ///        收到该回调后，你需调用 updateToken{@link #RTSRoom#updateToken} 更新 Token 订阅权限有效期。
  /// @note 若收到该回调后未及时更新 Token，Token 订阅权限过期后，尝试新订阅流会失败，已订阅的流会取消订阅，可通过 onVideoSubscribeStateChanged{@link #IRTCRoomEventHandler#onVideoSubscribeStateChanged}、onAudioSubscribeStateChanged{@link #IRTCRoomEventHandler#onAudioSubscribeStateChanged} 回调，提示错误码为 `-1003` 没有订阅权限。
  ///

  FutureOr<void> onSubscribePrivilegeTokenWillExpire() async {}

  /// @detail callback
  /// @valid since 3.60. 自 3.60 起，该回调替代了 `onStreamStateChanged` 方法中的本地视频发布状态变更通知功能。如果你已升级至 3.60 及以上版本 SDK，且还在使用该方法，请迁移至该回调。
  /// @author xuyiling.x10
  /// @brief 视频发布状态改变回调。
  /// @param streamId 流 ID，用于标识特定的视频流。
  /// @param streamInfo 流信息结构体，包含房间 ID、用户 ID 等详细信息，参看 StreamInfo{@link #StreamInfo}。
  /// @param state 发布状态码，参看 PublishState{@link #PublishState}。
  /// @param reason 本地视频流发布状态改变的具体原因，参看 PublishStateChangeReason{@link #PublishStateChangeReason}。
  /// @order 0
  ///

  FutureOr<void> onVideoPublishStateChanged(
      String streamId,
      StreamInfo streamInfo,
      PublishState state,
      PublishStateChangeReason reason) async {}

  /// @detail callback
  /// @valid since 3.60. 自 3.60 起，该回调替代了 `onStreamStateChanged` 方法中的本地音频发布状态变更通知功能。如果你已升级至 3.60 及以上版本 SDK，且还在使用该方法，请迁移至该回调。
  /// @author xuyiling.x10
  /// @brief 音频发布状态改变回调。
  /// @param streamId 流 ID，用于标识特定的音频流。
  /// @param streamInfo 流信息结构体，包含房间 ID、用户 ID 等详细信息，参看 StreamInfo{@link #StreamInfo}。
  /// @param state 发布状态码，参看 PublishState{@link #PublishState}。
  /// @param reason 本地音频流发布状态改变的具体原因，参看 PublishStateChangeReason{@link #PublishStateChangeReason}。
  /// @order 0
  ///

  FutureOr<void> onAudioPublishStateChanged(
      String streamId,
      StreamInfo streamInfo,
      PublishState state,
      PublishStateChangeReason reason) async {}

  /// @detail callback
  /// @valid since 3.60.
  /// @region 房间管理
  /// @author xuyiling.x10
  /// @brief 视频订阅状态发生改变回调。
  /// @param streamId 流 ID，用于标识特定的视频流。
  /// @param streamInfo 流信息结构体，包含房间 ID、用户 ID 等详细信息，参看 StreamInfo{@link #StreamInfo}。
  /// @param state 订阅状态码，参看 SubscribeState{@link #SubscribeState}。
  /// @param reason 视频订阅状态改变的具体原因，参看 SubscribeStateChangeReason{@link #SubscribeStateChangeReason}。
  /// @note 当调用 subscribeStreamVideo{@link #RTCRoom#subscribeStreamVideo} 后会收到该通知。
  /// @order 0
  ///

  FutureOr<void> onVideoSubscribeStateChanged(
      String streamId,
      StreamInfo streamInfo,
      SubscribeState state,
      SubscribeStateChangeReason reason) async {}

  /// @detail callback
  /// @valid since 3.60.
  /// @region 房间管理
  /// @author xuyiling.x10
  /// @brief 音频订阅状态发生改变回调。
  /// @param streamId 流 ID，用于标识特定的音频流。
  /// @param streamInfo 流信息结构体，包含房间 ID、用户 ID 等详细信息，参看 StreamInfo{@link #StreamInfo}。
  /// @param state 订阅状态码，参看 SubscribeState{@link #SubscribeState}。
  /// @param reason 音频订阅状态改变的具体原因，参看 SubscribeStateChangeReason{@link #SubscribeStateChangeReason}。
  /// @note 当调用 subscribeStreamAudio{@link #RTCRoom#subscribeStreamAudio} 后会收到该通知。
  /// @order 0
  ///

  FutureOr<void> onAudioSubscribeStateChanged(
      String streamId,
      StreamInfo streamInfo,
      SubscribeState state,
      SubscribeStateChangeReason reason) async {}

  /// @detail callback
  /// @author yejing
  /// @brief 本地流数据统计以及网络质量回调。 <br>
  ///        本地用户发布流成功后，SDK 会周期性（2s）的通过此回调事件通知用户发布的流在此次统计周期内的质量统计信息。 <br>
  ///        统计信息通过 LocalStreamStats{@link #LocalStreamStats} 类型的回调参数传递给用户，其中包括发送音视频比特率、发送帧率、编码帧率，网络质量等。
  /// @param streamId 流 ID，用于标识特定的本地流。
  /// @param streamInfo 流信息结构体，包含房间 ID、用户 ID 等详细信息，参看 StreamInfo{@link #StreamInfo}。
  /// @param stats 音视频流以及网络状况统计信息。参见 LocalStreamStats{@link #LocalStreamStats}。
  ///

  FutureOr<void> onLocalStreamStats(
      String streamId, StreamInfo streamInfo, LocalStreamStats stats) async {}

  /// @detail callback
  /// @author yejing
  /// @brief 本地订阅的远端音/视频流数据统计以及网络质量回调。 <br>
  ///        本地用户订阅流成功后，SDK 会周期性（2s）的通过此回调事件通知用户订阅的流在此次统计周期内的质量统计信息，包括：发送音视频比特率、发送帧率、编码帧率，网络质量等。
  /// @param streamId 流 ID，用于标识特定的远端流。
  /// @param streamInfo 流信息结构体，包含房间 ID、用户 ID 等详细信息，参看 StreamInfo{@link #StreamInfo}。
  /// @param stats 音视频流以及网络状况统计信息。参见 RemoteStreamStats{@link #RemoteStreamStats}。
  ///

  FutureOr<void> onRemoteStreamStats(
      String streamId, StreamInfo streamInfo, RemoteStreamStats stats) async {}

  /// @detail callback
  /// @author shenpengliang
  /// @brief 关于订阅媒体流状态改变的回调
  /// @param stateCode 订阅媒体流状态，参看 SubscribeState{@link #SubscribeState}
  /// @param userId 流发布用户的用户 ID
  /// @param info 流的属性，参看 SubscribeConfig{@link #SubscribeConfig}
  /// @note 本地用户收到该回调的时机：调用 subscribeStreamVideo{@link #RTCRoom#subscribeStreamVideo} 订阅/取消订阅指定远端摄像头音视频流后。
  ///

  FutureOr<void> onStreamSubscribed(
      SubscribeState stateCode, String userId, SubscribeConfig info) async {}

  /// @hidden for internal use only
  /// @detail callback
  /// @author shenpengliang
  /// @brief 当发布流成功的时候回调该事
  /// @param uid 流发布用户的用户 ID
  /// @param isScreen 流的标识
  ///

  FutureOr<void> onStreamPublishSuccess(String uid, boolean isScreen) async {}

  /// @detail callback
  /// @valid since 3.60.
  /// @author xuyiling.x10
  /// @brief 发布端调用 setMultiDeviceAVSync{@link #RTCRoom#setMultiDeviceAVSync} 后音视频同步状态发生错误时，会收到此回调。
  /// @param roomId 房间 ID。
  /// @param uid 用户 ID。
  /// @param eventCode 音视频同步状态错误，参看 AVSyncEvent{@link #AVSyncEvent}。
  ///

  FutureOr<void> onAVSyncEvent(
      String roomId, String uid, AVSyncEvent eventCode) async {}

  /// @detail callback
  /// @valid since 3.60. 自版本 3.60 起，该回调替换了 `onUserPublishStream`、`onUserUnpublishStream`、`onUserPublishScreen` 和 `onUserUnpublishScreen` 方法。如果您已升级到 SDK 版本 3.60 或以上，且仍在使用这两个方法，请迁移至该回调。
  /// @region 房间管理
  /// @author xuyiling.x10
  /// @brief 房间内远端用户发布或取消发布视频流的回调。
  /// @param streamId 流 ID，用于标识特定的视频流。
  /// @param streamInfo 流信息结构体，包含房间 ID、用户 ID 等详细信息，参看 StreamInfo{@link #StreamInfo}。
  /// @param isPublish 远端用户是否发布视频流。
  ///         + `true`：已发布。
  ///         + `false`：已取消发布或未发布。
  /// @note 当房间内的远端用户调用 publishStreamVideo{@link #RTCRoom#publishStreamVideo} 发布或取消发布由摄像头采集的媒体流时，本地用户会收到该回调，此时本地用户可以自行选择是否调用 subscribeStreamVideo{@link #RTCRoom#subscribeStreamVideo} 订阅或取消订阅此流。
  ///

  FutureOr<void> onUserPublishStreamVideo(
      String streamId, StreamInfo streamInfo, boolean isPublish) async {}

  /// @detail callback
  /// @valid since 3.60. 自 3.60 起，该回调替代了 `onUserPublishStream`、`onUserUnpublishStream`、`onUserPublishScreen` 和 `onUserUnpublishScreen` 方法来实现下述功能。如果你已升级至 3.60 及以上版本 SDK，且还在使用这两个方法，请迁移至该回调。
  /// @author xuyiling.x10
  /// @brief 房间内远端用户发布或取消发布音频流的回调。
  /// @param streamId 流 ID，用于标识特定的音频流。
  /// @param streamInfo 流信息结构体，包含房间 ID、用户 ID 等详细信息，参看 StreamInfo{@link #StreamInfo}。
  /// @param isPublish 远端用户是否发布音频流。
  ///         + `true`：已发布。
  ///         + `false`：已取消发布或未发布。
  /// @note 当房间内的远端用户调用 publishStreamAudio{@link #RTCRoom#publishStreamAudio} 发布或取消发布音频流时，本地用户会收到该回调，此时本地用户可以自行选择是否调用 subscribeStreamAudio{@link #RTCRoom#subscribeStreamAudio} 订阅或取消订阅此流。
  ///

  FutureOr<void> onUserPublishStreamAudio(
      String streamId, StreamInfo streamInfo, boolean isPublish) async {}

  /// @detail callback
  /// @author hanchenchen.c
  /// @brief 接收到房间内广播消息的回调。 <br>
  ///        房间内其他用户调用 sendRoomMessage{@link #RTSRoom#sendRoomMessage} 发送广播消息时，收到此回调。
  /// @param msgid 消息编号。
  /// @param uid 消息发送者 ID。
  /// @param message 收到的消息内容。
  ///

  FutureOr<void> onRoomMessageReceived(
      long msgid, String uid, String message) async {}

  /// @detail callback
  /// @author hanchenchen.c
  /// @brief 收到房间内广播二进制消息的回调。 <br>
  ///        房间内其他用户调用 sendRoomBinaryMessage{@link #RTSRoom#sendRoomBinaryMessage} 发送广播二进制消息时，收到此回调。
  /// @param msgid 消息编号。
  /// @param uid 消息发送者 ID。
  /// @param message 收到的二进制消息内容。
  ///

  FutureOr<void> onRoomBinaryMessageReceived(
      long msgid, String uid, Uint8List message) async {}

  /// @detail callback
  /// @author hanchenchen.c
  /// @brief 收到来自房间中其他用户通过 sendUserMessage{@link #RTSRoom#sendUserMessage} 发来的点对点文本消息时，会收到此回调。
  /// @param msgid 消息编号。
  /// @param uid 消息发送者的用户 ID。
  /// @param message 收到的文本消息内容。
  ///

  FutureOr<void> onUserMessageReceived(
      long msgid, String uid, String message) async {}

  /// @detail callback
  /// @author hanchenchen.c
  /// @brief 收到来自房间中其他用户通过 sendUserBinaryMessage{@link #RTSRoom#sendUserBinaryMessage} 发来的点对点二进制消息时，会收到此回调。
  /// @param msgid 消息编号。
  /// @param uid 消息发送者的用户 ID。
  /// @param message 收到的二进制消息内容。
  ///

  FutureOr<void> onUserBinaryMessageReceived(
      long msgid, String uid, Uint8List message) async {}

  /// @detail callback
  /// @author hanchenchen.c
  /// @brief 向房间内单个用户发送文本或二进制消息后（P2P），消息发送方会收到该消息发送结果回调。
  /// @param msgid 本条消息的 ID。
  /// @param error 文本或二进制消息发送结果，详见 UserMessageSendResult{@link #UserMessageSendResult}
  /// @note 调用 sendUserMessage{@link #RTSRoom#sendUserMessage} 或 sendUserBinaryMessage{@link #RTSRoom#sendUserBinaryMessage} 接口，才能收到此回调。
  ///

  FutureOr<void> onUserMessageSendResult(
      long msgid, UserMessageSendResult error) async {}

  /// @detail callback
  /// @author hanchenchen.c
  /// @brief 调用 sendRoomMessage{@link #RTSRoom#sendRoomMessage} 或 sendRoomBinaryMessage{@link #RTSRoom#sendRoomBinaryMessage} 向房间内群发文本或二进制消息后，消息发送方会收到该消息发送结果回调。
  /// @param msgid 本条消息的 ID。
  /// @param error 消息发送结果，详见 RoomMessageSendResult{@link #RoomMessageSendResult}
  ///

  FutureOr<void> onRoomMessageSendResult(
      long msgid, RoomMessageSendResult error) async {}

  /// @detail callback
  /// @author qipengxiang
  /// @brief 通过调用服务端 BanUserStream/UnbanUserStream 方法禁用/解禁指定房间内指定用户视频流的发送时，触发此回调。
  /// @param uid 被禁用/解禁的视频流用户 ID
  /// @param banned 视频流发送状态 <br>
  ///        - true: 视频流发送被禁用
  ///        - false: 视频流发送被解禁
  /// @note
  ///        - 房间内指定用户被禁止/解禁视频流发送时，房间内所有用户都会收到该回调。
  ///        - 若被封禁用户断网或退房后再进房，则依然是封禁状态，且房间内所有人会再次收到该回调。
  ///        - 指定用户被封禁后，房间内其他用户退房后再进房，会再次收到该回调。
  ///        - 同一房间解散后再次创建，房间内状态清空。
  ///

  FutureOr<void> onVideoStreamBanned(String uid, boolean banned) async {}

  /// @detail callback
  /// @author qipengxiang
  /// @brief 通过调用服务端 BanUserStream/UnbanUserStream 方法禁用/解禁指定房间内指定用户音频流的发送时，触发此回调。
  /// @param uid 被禁用/解禁的音频流用户 ID
  /// @param banned 音频流发送状态 <br>
  ///        - true: 音频流发送被禁用
  ///        - false: 音频流发送被解禁
  /// @note
  ///        - 房间内指定用户被禁止/解禁音频流发送时，房间内所有用户都会收到该回调。
  ///        - 若被封禁用户断网或退房后再进房，则依然是封禁状态，且房间内所有人会再次收到该回调。
  ///        - 指定用户被封禁后，房间内其他用户退房后再进房，会再次收到该回调。
  ///        - 在控制台开启音频选路后，只有被封禁/解禁用户会收到该回调。
  ///        - 同一房间解散后再次创建，房间内状态清空。
  ///

  FutureOr<void> onAudioStreamBanned(String uid, boolean banned) async {}

  /// @detail callback
  /// @author shenpengliang
  /// @brief 跨房间媒体流转发状态和错误回调
  /// @param stateInfos 跨房间媒体流转发目标房间信息数组，详见 ForwardStreamStateInfo{@link #ForwardStreamStateInfo}
  ///

  FutureOr<void> onForwardStreamStateChanged(
      Array<ForwardStreamStateInfo> stateInfos) async {}

  /// @detail callback
  /// @author shenpengliang
  /// @brief 跨房间媒体流转发事件回调
  /// @param eventInfos 跨房间媒体流转发目标房间事件数组，详见 ForwardStreamEventInfo{@link #ForwardStreamEventInfo}
  ///

  FutureOr<void> onForwardStreamEvent(
      Array<ForwardStreamEventInfo> eventInfos) async {}

  /// @detail callback
  /// @author chengchao.cc951119
  /// @brief 加入房间并发布或订阅流后， 以每 2 秒一次的频率，报告本地用户和已订阅的远端用户的上下行网络质量信息。
  /// @param localQuality 本地网络质量，详见 NetworkQualityStats{@link #NetworkQualityStats}。
  /// @param remoteQualities 已订阅用户的网络质量，详见 NetworkQualityStats{@link #NetworkQualityStats}。
  /// @note 更多通话中的监测接口，详见[通话中质量监测](https://www.volcengine.com/docs/6348/106866)。
  ///

  FutureOr<void> onNetworkQuality(NetworkQualityStats localQuality,
      Array<NetworkQualityStats> remoteQualities) async {}

  /// @valid since 3.52
  /// @detail callback
  /// @author lichangfeng.rtc
  /// @brief 调用 setRoomExtraInfo{@link #RTCRoom#setRoomExtraInfo} 设置房间附加信息结果的回调。
  /// @param taskId 调用 setRoomExtraInfo 的任务编号。
  /// @param result 设置房间附加信息的结果，详见 SetRoomExtraInfoResult{@link #SetRoomExtraInfoResult}
  ///

  FutureOr<void> onSetRoomExtraInfoResult(
      long taskId, SetRoomExtraInfoResult result) async {}

  /// @valid since 3.52
  /// @detail callback
  /// @author lichangfeng.rtc
  /// @brief 接收同一房间内，其他用户调用 setRoomExtraInfo{@link #RTCRoom#setRoomExtraInfo} 设置的房间附加信息的回调。
  /// @param key 房间附加信息的键值
  /// @param value 房间附加信息的内容
  /// @param lastUpdateUserId 最后更新本条信息的用户 ID。
  /// @param lastUpdateTimeMs 最后更新本条信息的 Unix 时间，单位：毫秒。
  /// @note 新进房的用户会收到进房前房间内已有的全部附加信息通知。
  ///

  FutureOr<void> onRoomExtraInfoUpdate(String key, String value,
      String lastUpdateUserId, long lastUpdateTimeMs) async {}

  /// @valid since 3.54
  /// @detail callback
  /// @brief 接收同一房间内，其他用户调用 setStreamExtraInfo{@link #RTCRoom#setStreamExtraInfo} 设置的流附加信息的回调。
  /// @param streamId 流附加信息的流 ID
  /// @param streamInfo 流附加信息的流信息
  /// @param extraInfo 流附加信息
  /// @note 新进房的用户会收到进房前房间内已有的全部附加信息通知。
  ///

  FutureOr<void> onRoomStreamExtraInfoUpdate(
      String streamId, StreamInfo streamInfo, String extraInfo) async {}

  /// @valid since 3.54
  /// @detail callback
  /// @author caocun
  /// @brief 用户调用 setUserVisibility{@link #RTCRoom#setUserVisibility} 设置用户可见性的回调。
  /// @param currentUserVisibility 当前用户的可见性。 <br>
  ///        - true: 可见，用户可以在房间内发布音视频流，房间中的其他用户将收到用户的行为通知，例如进房、开启视频采集和退房。
  ///        - false: 不可见，用户不可以在房间内发布音视频流，房间中的其他用户不会收到用户的行为通知，例如进房、开启视频采集和退房。
  /// @param errorCode 设置用户可见性错误码，参看 UserVisibilityChangeError{@link #UserVisibilityChangeError}。
  ///

  FutureOr<void> onUserVisibilityChanged(boolean currentUserVisibility,
      UserVisibilityChangeError errorCode) async {}

  /// @detail callback
  /// @author qiaoxingwang
  /// @brief 字幕状态发生改变回调。 <br>
  ///         当用户调用 startSubtitle{@link #RTCRoom#startSubtitle} 和 stopSubtitle{@link #RTCRoom#stopSubtitle} 使字幕状态发生改变或字幕任务出现错误时，触发该回调。
  /// @param state 字幕状态。参看 SubtitleState{@link #SubtitleState}。
  /// @param errorCode 字幕任务错误码。参看 SubtitleErrorCode{@link #SubtitleErrorCode}。
  /// @param errorMessage 与第三方服务有关的错误信息。
  ///

  FutureOr<void> onSubtitleStateChanged(SubtitleState state,
      SubtitleErrorCode errorCode, String errorMessage) async {}

  /// @detail callback
  /// @author qiaoxingwang
  /// @brief 字幕相关内容回调。 <br>
  ///         当用户成功调用 startSubtitle{@link #RTCRoom#startSubtitle} 后会收到此回调，通知字幕的相关信息。
  /// @param subtitles 字幕消息内容。参看 SubtitleMessage{@link #SubtitleMessage}。
  ///

  FutureOr<void> onSubtitleMessageReceived(
      Array<SubtitleMessage> subtitles) async {}

  /// @hidden
  /// @deprecated since 3.41 and will be deleted in 3.51, use onRoomStateChanged{@link #IRTCRoomEventHandler#onRoomStateChanged} and onStreamStateChanged instead.
  /// @detail callback
  /// @author shenpengliang
  /// @brief 发生警告回调。
  /// @param warn 警告代码，参见 WarningCode{@link #WarningCode}
  /// @note SDK 运行时出现了（网络或媒体相关的）警告。SDK 通常会自动恢复，警告信息可以忽略。
  ///

  FutureOr<void> onRoomWarning(int warn) async {}
}

class IRTSRoomEventHandler extends NativeObserverClass {
  static const _$namespace =
      r'com.ss.bytertc.engine.handler.IRTSRoomEventHandler';
  static get codegen_$namespace => _$namespace;

  IRTSRoomEventHandler([NativeClassOptions? options])
      : super(options == null
            ? const NativeClassOptions([],
                className: _$namespace,
                instanceType: InstanceType.manual,
                bridgeKey: 'com.volcengine.rtc.hybrid_runtime',
                methodMap: {
                  r"onLeaveRoom": r"onLeaveRoom",
                  r"onRoomStateChanged": r"onRoomStateChanged",
                  r"onUserJoined": r"onUserJoined",
                  r"onUserLeave": r"onUserLeave",
                  r"onRoomMessageReceived": r"onRoomMessageReceived",
                  r"onRoomBinaryMessageReceived":
                      r"onRoomBinaryMessageReceived",
                  r"onUserMessageReceived": r"onUserMessageReceived",
                  r"onUserBinaryMessageReceived":
                      r"onUserBinaryMessageReceived",
                  r"onUserMessageSendResult": r"onUserMessageSendResult",
                  r"onRoomMessageSendResult": r"onRoomMessageSendResult"
                })
            : NativeClassOptions.fromMap({
                ...options.toMap(),
                'bridgeKey': 'com.volcengine.rtc.hybrid_runtime',
              })) {
    registerEvent(r"onLeaveRoom", onLeaveRoom);

    registerEvent(r"onRoomStateChanged", onRoomStateChanged);

    registerEvent(r"onUserJoined", onUserJoined);

    registerEvent(r"onUserLeave", onUserLeave);

    registerEvent(r"onRoomMessageReceived", onRoomMessageReceived);

    registerEvent(r"onRoomBinaryMessageReceived", onRoomBinaryMessageReceived);

    registerEvent(r"onUserMessageReceived", onUserMessageReceived);

    registerEvent(r"onUserBinaryMessageReceived", onUserBinaryMessageReceived);

    registerEvent(r"onUserMessageSendResult", onUserMessageSendResult);

    registerEvent(r"onRoomMessageSendResult", onRoomMessageSendResult);
  }

  /// @detail callback
  /// @brief 离开房间成功回调。 <br>
  ///        用户调用 leaveRoom{@link #RTSRoom#leaveRoom} 方法后，SDK 会停止所有的发布订阅流，并在释放所有通话相关的音视频资源后，通过此回调通知用户离开房间成功。
  /// @param stats 保留参数，目前为空。
  /// @note
  ///       - 用户调用 leaveRoom{@link #RTSRoom#leaveRoom} 方法离开房间后，如果立即调用 destroy{@link #RTSRoom#destroy} 销毁房间实例或 destroyRTCEngine{@link #RTCEngine#destroyRTCEngine} 方法销毁 RTC 引擎，则将无法收到此回调事件。
  ///       - 离开房间后，如果 App 需要使用系统音视频设备，则建议在收到此回调后再初始化音视频设备，否则可能由于 SDK 占用音视频设备导致初始化失败。
  ///

  FutureOr<void> onLeaveRoom(RTCRoomStats stats) async {}

  /// @detail callback
  /// @brief 房间状态改变回调，加入房间、异常退出房间、发生房间相关的警告或错误时会收到此回调。
  /// @param roomId 房间 ID。
  /// @param uid 用户 ID。
  /// @param state 房间状态码。 <br>
  ///              - 0: 加入房间成功。
  ///              - !0: 加入房间失败、异常退房、发生房间相关的警告或错误。具体原因参看 ErrorCode{@link #ErrorCode} 及 WarningCode{@link #WarningCode}。
  /// @param extraInfo 额外信息，如 `{"elapsed":1187,"join_type":0}`。 <br>
  ///                  `join_type` 表示加入房间的类型，`0`为首次进房，`1`为重连进房。 <br>
  ///                  `elapsed` 表示加入房间耗时，即本地用户从调用 joinRoom{@link #RTCRoom#joinRoom} 到加入房间成功所经历的时间间隔，单位为 ms。
  ///

  FutureOr<void> onRoomStateChanged(
      String roomId, String uid, int state, String extraInfo) async {}

  /// @detail callback
  /// @brief 远端可见用户加入房间，或房内不可见用户切换为可见的回调。 <br>
  ///        1.远端用户调用 setUserVisibility{@link #RTCRoom#setUserVisibility} 方法将自身设为可见后加入房间时，房间内其他用户将收到该事件。 <br>
  ///        2.远端可见用户断网后重新连入房间时，房间内其他用户将收到该事件。 <br>
  ///        3.房间内隐身远端用户调用 setUserVisibility{@link #RTCRoom#setUserVisibility} 方法切换至可见时，房间内其他用户将收到该事件。 <br>
  ///        4.新进房用户也会收到进房前已在房内的可见用户的进房回调通知。
  /// @param userInfo 用户信息。参看 UserInfo{@link #UserInfo}。
  ///

  FutureOr<void> onUserJoined(UserInfo userInfo) async {}

  /// @detail callback
  /// @brief 远端用户离开房间，或切至不可见时，房间内其他用户会收到此事件
  /// @param uid 离开房间，或切至不可见的的远端用户 ID。
  /// @param reason 用户离开房间的原因： <br>
  ///              - 0: 远端用户调用 leaveRoom{@link #RTSRoom#leaveRoom} 主动退出房间。
  ///              - 1: 远端用户因 Token 过期或网络原因等掉线。详细信息请参看[连接状态提示](https://www.volcengine.com/docs/6348/95376)
  ///              - 2: 远端用户调用 setUserVisibility{@link #RTCRoom#setUserVisibility} 切换至不可见状态。
  ///              - 3: 服务端调用 OpenAPI 将该远端用户踢出房间。
  ///

  FutureOr<void> onUserLeave(String uid, int reason) async {}

  /// @detail callback
  /// @author hanchenchen.c
  /// @brief 接收到房间内广播消息的回调。 <br>
  ///        房间内其他用户调用 sendRoomMessage{@link #RTSRoom#sendRoomMessage} 发送广播消息时，收到此回调。
  /// @param msgid 消息编号
  /// @param uid 消息发送者 ID
  /// @param message 收到的消息内容
  ///

  FutureOr<void> onRoomMessageReceived(
      long msgid, String uid, String message) async {}

  /// @detail callback
  /// @author hanchenchen.c
  /// @brief 收到房间内广播二进制消息的回调。 <br>
  ///        房间内其他用户调用 sendRoomBinaryMessage{@link #RTSRoom#sendRoomBinaryMessage} 发送广播二进制消息时，收到此回调。
  /// @param msgid 消息编号
  /// @param uid 消息发送者 ID
  /// @param message 收到的二进制消息内容
  ///

  FutureOr<void> onRoomBinaryMessageReceived(
      long msgid, String uid, ByteBuffer message) async {}

  /// @detail callback
  /// @author hanchenchen.c
  /// @brief 收到来自房间中其他用户通过 sendUserMessage{@link #RTSRoom#sendUserMessage} 发来的点对点文本消息时，会收到此回调。
  /// @param msgid 消息编号
  /// @param uid 消息发送者的用户 ID。
  /// @param message 收到的文本消息内容。
  ///

  FutureOr<void> onUserMessageReceived(
      long msgid, String uid, String message) async {}

  /// @detail callback
  /// @author hanchenchen.c
  /// @brief 收到来自房间中其他用户通过 sendUserBinaryMessage{@link #RTSRoom#sendUserBinaryMessage} 发来的点对点二进制消息时，会收到此回调。
  /// @param msgid 消息编号
  /// @param uid 消息发送者的用户 ID。
  /// @param message 收到的二进制消息内容。
  ///

  FutureOr<void> onUserBinaryMessageReceived(
      long msgid, String uid, ByteBuffer message) async {}

  /// @detail callback
  /// @brief 向房间内单个用户发送文本或二进制消息后（P2P），消息发送方会收到该消息发送结果回调。
  /// @param msgid 本条消息的 ID。
  /// @param error 文本或二进制消息发送结果，详见 UserMessageSendResult{@link #UserMessageSendResult}
  /// @note 调用 sendUserMessage{@link #RTSRoom#sendUserMessage} 或 sendUserBinaryMessage{@link #RTSRoom#sendUserBinaryMessage} 接口，才能收到此回调。
  ///

  FutureOr<void> onUserMessageSendResult(long msgid, int error) async {}

  /// @detail callback
  /// @brief 调用 sendRoomMessage{@link #RTSRoom#sendRoomMessage} 或 sendRoomBinaryMessage{@link #RTSRoom#sendRoomBinaryMessage} 向房间内群发文本或二进制消息后，消息发送方会收到该消息发送结果回调。
  /// @param msgid 本条消息的 ID。
  /// @param error 消息发送结果，详见 RoomMessageSendResult{@link #RoomMessageSendResult}
  ///

  FutureOr<void> onRoomMessageSendResult(long msgid, int error) async {}
}

class IRemoteEncodedVideoFrameObserver extends NativeObserverClass {
  static const _$namespace =
      r'com.ss.bytertc.engine.mediaio.IRemoteEncodedVideoFrameObserver';
  static get codegen_$namespace => _$namespace;

  IRemoteEncodedVideoFrameObserver([NativeClassOptions? options])
      : super(options == null
            ? const NativeClassOptions([],
                className: _$namespace,
                instanceType: InstanceType.manual,
                bridgeKey: 'com.volcengine.rtc.hybrid_runtime',
                methodMap: {
                  r"onRemoteEncodedVideoFrame": r"onRemoteEncodedVideoFrame"
                })
            : NativeClassOptions.fromMap({
                ...options.toMap(),
                'bridgeKey': 'com.volcengine.rtc.hybrid_runtime',
              })) {
    registerEvent(r"onRemoteEncodedVideoFrame", onRemoteEncodedVideoFrame);
  }

  /// @detail callback
  /// @author wangzhanqiang
  /// @brief 调用 registerRemoteEncodedVideoFrameObserver{@link #RTCEngine#registerRemoteEncodedVideoFrameObserver} 后，SDK 监测到远端编码后视频数据时，触发该回调
  /// @param streamId 收到的远端流 ID
  /// @param streamInfo 收到的远端流信息，参看 StreamInfo{@link #StreamInfo}
  /// @param encodedVideoFrame 收到的远端视频帧信息，参看 RTCEncodedVideoFrame{@link #RTCEncodedVideoFrame}
  /// @note encodedVideoFrame 只在回调函数作用域内有效，不要存储该参数并在其它函数内访问该参数的内存数据
  ///

  FutureOr<void> onRemoteEncodedVideoFrame(String streamId,
      StreamInfo streamInfo, RTCEncodedVideoFrame encodedVideoFrame) async {}
}

class IAudioFileFrameObserver extends NativeObserverClass {
  static const _$namespace = r'com.ss.bytertc.engine.IAudioFileFrameObserver';
  static get codegen_$namespace => _$namespace;

  IAudioFileFrameObserver([NativeClassOptions? options])
      : super(options == null
            ? const NativeClassOptions([],
                className: _$namespace,
                instanceType: InstanceType.manual,
                bridgeKey: 'com.volcengine.rtc.hybrid_runtime',
                methodMap: {r"onAudioFileFrame": r"onAudioFileFrame"})
            : NativeClassOptions.fromMap({
                ...options.toMap(),
                'bridgeKey': 'com.volcengine.rtc.hybrid_runtime',
              })) {
    registerEvent(r"onAudioFileFrame", onAudioFileFrame);
  }

  /// @detail callback
  /// @author majun.lvhiei
  /// @brief 当本地音频文件混音时，回调播放的音频帧。
  /// @param mixID 混音 ID。
  /// @param audioFrame 参看 IAudioFrame{@link #IAudioFrame}。
  ///

  FutureOr<void> onAudioFileFrame(int mixID, IAudioFrame audioFrame) async {}
}

class IAudioEffectPlayerEventHandler extends NativeObserverClass {
  static const _$namespace =
      r'com.ss.bytertc.engine.IAudioEffectPlayerEventHandler';
  static get codegen_$namespace => _$namespace;

  IAudioEffectPlayerEventHandler([NativeClassOptions? options])
      : super(options == null
            ? const NativeClassOptions([],
                className: _$namespace,
                instanceType: InstanceType.manual,
                bridgeKey: 'com.volcengine.rtc.hybrid_runtime',
                methodMap: {
                  r"onAudioEffectPlayerStateChanged":
                      r"onAudioEffectPlayerStateChanged"
                })
            : NativeClassOptions.fromMap({
                ...options.toMap(),
                'bridgeKey': 'com.volcengine.rtc.hybrid_runtime',
              })) {
    registerEvent(
        r"onAudioEffectPlayerStateChanged", onAudioEffectPlayerStateChanged);
  }

  /// @detail callback
  /// @brief 播放状态改变时回调。
  /// @param effectId IAudioEffectPlayer{@link #IAudioEffectPlayer} 的 ID。通过 getAudioEffectPlayer{@link #RTCEngine#getAudioEffectPlayer} 设置。
  /// @param state 混音状态。参考 PlayerState{@link #PlayerState}。
  /// @param error 错误码。参考 PlayerError{@link #PlayerError}。
  /// @order 0
  ///

  FutureOr<void> onAudioEffectPlayerStateChanged(
      int effectId, PlayerState state, PlayerError error) async {}
}

class IWTNStreamEventHandler extends NativeObserverClass {
  static const _$namespace = r'com.ss.bytertc.engine.IWTNStreamEventHandler';
  static get codegen_$namespace => _$namespace;

  IWTNStreamEventHandler([NativeClassOptions? options])
      : super(options == null
            ? const NativeClassOptions([],
                className: _$namespace,
                instanceType: InstanceType.manual,
                bridgeKey: 'com.volcengine.rtc.hybrid_runtime',
                methodMap: {
                  r"onWTNRemoteVideoStats": r"onWTNRemoteVideoStats",
                  r"onWTNRemoteAudioStats": r"onWTNRemoteAudioStats",
                  r"onWTNVideoSubscribeStateChanged":
                      r"onWTNVideoSubscribeStateChanged",
                  r"onWTNAudioSubscribeStateChanged":
                      r"onWTNAudioSubscribeStateChanged",
                  r"onWTNFirstRemoteAudioFrame": r"onWTNFirstRemoteAudioFrame",
                  r"onWTNFirstRemoteVideoFrameDecoded":
                      r"onWTNFirstRemoteVideoFrameDecoded",
                  r"onWTNSEIMessageReceived": r"onWTNSEIMessageReceived",
                  r"onWTNDataMessageReceived": r"onWTNDataMessageReceived"
                })
            : NativeClassOptions.fromMap({
                ...options.toMap(),
                'bridgeKey': 'com.volcengine.rtc.hybrid_runtime',
              })) {
    registerEvent(r"onWTNRemoteVideoStats", onWTNRemoteVideoStats);

    registerEvent(r"onWTNRemoteAudioStats", onWTNRemoteAudioStats);

    registerEvent(
        r"onWTNVideoSubscribeStateChanged", onWTNVideoSubscribeStateChanged);

    registerEvent(
        r"onWTNAudioSubscribeStateChanged", onWTNAudioSubscribeStateChanged);

    registerEvent(r"onWTNFirstRemoteAudioFrame", onWTNFirstRemoteAudioFrame);

    registerEvent(r"onWTNFirstRemoteVideoFrameDecoded",
        onWTNFirstRemoteVideoFrameDecoded);

    registerEvent(r"onWTNSEIMessageReceived", onWTNSEIMessageReceived);

    registerEvent(r"onWTNDataMessageReceived", onWTNDataMessageReceived);
  }

  /// @detail callback
  /// @valid since 3.60.
  /// @author hanchenchen
  /// @brief 通话中本地设备接收订阅的远端 WTN 视频流的流 ID 以及远端 WTN 视频流统计信息。
  /// @param streamId WTN 流 ID
  /// @param stats 远端 WTN 视频流的统计信息，详见 RemoteVideoStats{@link #RemoteVideoStats}。
  /// @order 0
  ///

  FutureOr<void> onWTNRemoteVideoStats(
      String streamId, RemoteVideoStats stats) async {}

  /// @detail callback
  /// @valid since 3.60.
  /// @author hanchenchen
  /// @brief 通话中本地设备接收订阅的远端 WTN 音频流的流 ID 以及远端 WTN 音频流统计信息。
  /// @param streamId WTN 流 ID
  /// @param stats 远端 WTN 音频流的统计信息，详见 RemoteAudioStats{@link #RemoteAudioStats}。
  /// @order 1
  ///

  FutureOr<void> onWTNRemoteAudioStats(
      String streamId, RemoteAudioStats stats) async {}

  /// @detail callback
  /// @valid since 3.60. 自 3.60 起，该回调替代了 `onPlayPublicStreamResult` 方法中的 WTN 视频流订阅状态变化通知功能。如果你已升级至 3.60 及以上版本，并且仍在使用该方法，请迁移到此回调。
  /// @author hanchenchen
  /// @brief WTN 视频流订阅状态变化回调 <br>
  ///        通过 subscribeWTNVideoStream{@link #IWTNStream#subscribeWTNVideoStream} 订阅 WTN 视频流后，可以通过本回调获取订阅结果。
  /// @param streamId WTN 视频流的 ID
  /// @param stateCode 视频流状态码，参看 WTNSubscribeState{@link #WTNSubscribeState}。
  /// @param reason 订阅状态发生变化的原因，参看 WTNSubscribeStateChangeReason{@link #WTNSubscribeStateChangeReason}。
  /// @order 2
  ///

  FutureOr<void> onWTNVideoSubscribeStateChanged(
      String streamId,
      WTNSubscribeState stateCode,
      WTNSubscribeStateChangeReason reason) async {}

  /// @detail callback
  /// @valid since 3.60. 自 3.60 起，该回调替代了 `onPlayPublicStreamResult` 方法中的 WTN 音频流订阅状态变化通知功能。如果你已升级至 3.60 及以上版本，并且仍在使用该方法，请迁移到此回调。
  /// @author hanchenchen
  /// @brief WTN 音频流订阅状态变化回调 <br>
  ///        通过 subscribeWTNAudioStream{@link #IWTNStream#subscribeWTNAudioStream} 订阅 WTN 音频流后，可以通过本回调获取订阅结果。
  /// @param streamId WTN 音频流的 ID
  /// @param stateCode 音频流状态码，参看 WTNSubscribeState{@link #WTNSubscribeState}。
  /// @param reason 订阅状态发生变化的原因. See WTNSubscribeStateChangeReason{@link #WTNSubscribeStateChangeReason}.
  /// @order 2
  ///

  FutureOr<void> onWTNAudioSubscribeStateChanged(
      String streamId,
      WTNSubscribeState stateCode,
      WTNSubscribeStateChangeReason reason) async {}

  /// @author hanchenchen
  /// @detail callback
  /// @valid since 3.60. 自 3.60 起，该回调替代了 `onFirstPublicStreamAudioFrame`。如果你已升级至 3.60 及以上版本，并且仍在使用该方法，请迁移到此回调。
  /// @brief WTN 流的首帧音频解码成功 <br>
  ///        关于订阅 WTN 音频流，详见 subscribeWTNAudioStream{@link #IWTNStream#subscribeWTNAudioStream}。
  /// @param streamId WTN 流 ID
  /// @order 3
  ///

  FutureOr<void> onWTNFirstRemoteAudioFrame(String streamId) async {}

  /// @detail callback
  /// @valid since 3.60. 自 3.60 起，该回调替代了 `onFirstPublicStreamVideoFrameDecoded`。如果你已升级至 3.60 及以上版本，并且仍在使用该方法，请迁移到此回调。
  /// @author hanchenchen
  /// @brief WTN 流的首帧视频解码成功 <br>
  ///        关于订阅 WTN 流，详见 subscribeWTNVideoStream{@link #IWTNStream#subscribeWTNVideoStream}。
  /// @param streamId WTN 流 ID
  /// @param info 视频帧信息。详见 VideoFrameInfo{@link #VideoFrameInfo}。
  /// @order 4
  ///

  FutureOr<void> onWTNFirstRemoteVideoFrameDecoded(
      String streamId, VideoFrameInfo info) async {}

  /// @detail callback
  /// @valid since 3.60. 自 3.60 起，该回调替换了 `onPublicStreamSEIMessageReceived` 来实现下述功能。如果你已升级至 3.60 及以上版本，并且仍在使用该方法，请迁移到此回调。
  /// @author hanchenchen
  /// @brief 回调 WTN 流中包含的 SEI 信息。 <br>
  ///        调用 subscribeWTNAudioStream{@link #IWTNStream#subscribeWTNAudioStream}/subscribeWTNVideoStream{@link #IWTNStream#subscribeWTNVideoStream}接口拉 WTN 音频流/视频流后，通过此回调收到 WTN 流中的 SEI 消息。
  /// @param streamId WTN 流 ID。
  /// @param channelId SEI 的消息传输通道，取值范围 `[0 - 255]`。通过此参数，你可以为不同接受方设置不同的 ChannelID，这样不同接收方可以根据回调中的 ChannelID 选择应关注的 SEI 信息。
  /// @param message 收到的 SEI 消息内容。 <br>
  ///                通过调用客户端 `sendSEIMessage` 插入的 SEI 信息。 <br>
  ///                当 WTN 流中的多路视频流均包含有 SEI 信息：SEI 不互相冲突时，将通过多次回调分别发送；SEI 在同一帧有冲突时，则只有一条流中的 SEI 信息被透传并融合到 WTN 流中。
  /// @order 5
  ///

  FutureOr<void> onWTNSEIMessageReceived(
      String streamId, int channelId, ByteBuffer message) async {}

  /// @detail callback
  /// @valid since 3.60.
  /// @author hanchenchen
  /// @brief 回调 WTN 流中包含的数据信息。 <br>
  ///        通过 subscribeWTNAudioStream{@link #IWTNStream#subscribeWTNAudioStream}/subscribeWTNVideoStream{@link #IWTNStream#subscribeWTNVideoStream} 订阅 WTN 流后，通过监听本回调获取 WTN 流中的数据消息，包括调用 Open API 发送的 SEI 消息和音量回调。
  /// @param streamId  WTN 流 ID
  /// @param message 收到的数据消息内容，如下： <br>
  /// - 调用 WTN 流 OpenAPI 发送的自定义消息。
  /// - 媒体流音量变化，需要通过 WTN 流 OpenAPI 开启回调。JSON 格式说明如下：<br/>
  /// {<br/>
  /// "Type" : "VolumeIndication", //具体业务类型<br/>
  /// "VolumeInfos" : [ // 业务类型对应信息<br/>
  /// {<br/>
  /// "RoomId":"1000001", // 房间 ID<br/>
  /// "UserId":"1000001", // 用户 ID<br/>
  /// "StreamType":0, // 0:摄像头流；1:屏幕流<br/>
  /// "LinearVolume":1 // 线性音量大小<br/>
  /// }<br/>
  /// @param sourceType 数据消息来源，参看 DataMessageSourceType{@link #DataMessageSourceType}。
  /// @note 通过调用客户端 API 插入的 SEI 信息，应通过回调 onWTNSEIMessageReceived{@link #IWTNStreamEventHandler#onWTNSEIMessageReceived} 获取。
  /// @order 6
  ///

  FutureOr<void> onWTNDataMessageReceived(String streamId, ByteBuffer message,
      DataMessageSourceType sourceType) async {}
}

class IMediaPlayerEventHandler extends NativeObserverClass {
  static const _$namespace = r'com.ss.bytertc.engine.IMediaPlayerEventHandler';
  static get codegen_$namespace => _$namespace;

  IMediaPlayerEventHandler([NativeClassOptions? options])
      : super(options == null
            ? const NativeClassOptions([],
                className: _$namespace,
                instanceType: InstanceType.manual,
                bridgeKey: 'com.volcengine.rtc.hybrid_runtime',
                methodMap: {
                  r"onMediaPlayerStateChanged": r"onMediaPlayerStateChanged",
                  r"onMediaPlayerPlayingProgress":
                      r"onMediaPlayerPlayingProgress",
                  r"onMediaPlayerEvent": r"onMediaPlayerEvent"
                })
            : NativeClassOptions.fromMap({
                ...options.toMap(),
                'bridgeKey': 'com.volcengine.rtc.hybrid_runtime',
              })) {
    registerEvent(r"onMediaPlayerStateChanged", onMediaPlayerStateChanged);

    registerEvent(
        r"onMediaPlayerPlayingProgress", onMediaPlayerPlayingProgress);

    registerEvent(r"onMediaPlayerEvent", onMediaPlayerEvent);
  }

  /// @detail callback
  /// @brief 播放状态改变时回调。
  /// @param playerId IMediaPlayer{@link #IMediaPlayer} 的 ID。通过 getMediaPlayer{@link #RTCEngine#getMediaPlayer} 设置。
  /// @param state 混音状态。参考 PlayerState{@link #PlayerState}。
  /// @param error 错误码。参考 PlayerError{@link #PlayerError}。
  /// @order 2
  ///

  FutureOr<void> onMediaPlayerStateChanged(
      int playerId, PlayerState state, PlayerError error) async {}

  /// @detail callback
  /// @brief 播放进度周期性回调。回调周期通过 setProgressInterval{@link #IMediaPlayer#setProgressInterval} 设置。
  /// @param playerId IMediaPlayer{@link #IMediaPlayer} 的 ID。通过 getMediaPlayer{@link #RTCEngine#getMediaPlayer} 设置。
  /// @param progress 进度。单位 ms。
  /// @order 3
  ///

  FutureOr<void> onMediaPlayerPlayingProgress(
      int playerId, long progress) async {}

  /// @valid since 3.59
  /// @detail callback
  /// @author wangfeng.1004
  /// @brief 播放事件回调。调用 selectAudioTrack{@link #IMediaPlayer#selectAudioTrack} 和 setPosition{@link #IMediaPlayer#setPosition} 后，会触发此回调。
  /// @param playerId IMediaPlayer{@link #IMediaPlayer} 的 ID。通过 getMediaPlayer{@link #RTCEngine#getMediaPlayer} 设置。
  /// @param event 播放器事件。参看 PlayerEvent{@link #PlayerEvent}。
  /// @param message 事件描述信息，可能为空。
  ///

  FutureOr<void> onMediaPlayerEvent(
      int playerId, PlayerEvent event, String message) async {}
}

class IRTCEncryptionHandler extends NativeObserverClass {
  static const _$namespace =
      r'com.ss.bytertc.engine.handler.IRTCEncryptionHandler';
  static get codegen_$namespace => _$namespace;

  IRTCEncryptionHandler([NativeClassOptions? options])
      : super(options == null
            ? const NativeClassOptions([],
                className: _$namespace,
                instanceType: InstanceType.manual,
                bridgeKey: 'com.volcengine.rtc.hybrid_runtime',
                methodMap: {
                  r"onEncryptData": r"onEncryptData",
                  r"onDecryptData": r"onDecryptData"
                })
            : NativeClassOptions.fromMap({
                ...options.toMap(),
                'bridgeKey': 'com.volcengine.rtc.hybrid_runtime',
              })) {
    registerEvent(r"onEncryptData", onEncryptData);

    registerEvent(r"onDecryptData", onDecryptData);
  }

  /// @detail callback
  /// @brief 自定义加密。 <br>
  ///        使用设定的自定义加密方式，对编码后传输前的音视频帧数据进行加密。 <br>
  ///        暂不支持对原始音视频帧进行加密。
  /// @param data 未加密的数据。
  /// @return 加密后的数据
  ///

  FutureOr<ArrayBuffer> onEncryptData(ArrayBuffer data) async {
    throw UnimplementedError();
  }

  /// @detail callback
  /// @brief 自定义解密。 <br>
  ///        对自定义加密后的音视频帧数据进行解密。关于自定义加密，参看 onEncryptData{@link #IRTCEncryptionHandler#onEncryptData}。
  /// @param data 加密过的数据。
  /// @return 解密后的数据
  ///

  FutureOr<ArrayBuffer> onDecryptData(ArrayBuffer data) async {
    throw UnimplementedError();
  }
}

class IFaceDetectionObserver extends NativeObserverClass {
  static const _$namespace =
      r'com.ss.bytertc.engine.video.IFaceDetectionObserver';
  static get codegen_$namespace => _$namespace;

  IFaceDetectionObserver([NativeClassOptions? options])
      : super(options == null
            ? const NativeClassOptions([],
                className: _$namespace,
                instanceType: InstanceType.manual,
                bridgeKey: 'com.volcengine.rtc.hybrid_runtime',
                methodMap: {
                  r"onFaceDetectResult": r"onFaceDetectResult",
                  r"onExpressionDetectResult": r"onExpressionDetectResult"
                })
            : NativeClassOptions.fromMap({
                ...options.toMap(),
                'bridgeKey': 'com.volcengine.rtc.hybrid_runtime',
              })) {
    registerEvent(r"onFaceDetectResult", onFaceDetectResult);

    registerEvent(r"onExpressionDetectResult", onExpressionDetectResult);
  }

  /// @detail callback
  /// @author wangjunlin.3182
  /// @brief 特效 SDK 进行人脸检测结果的回调。 <br>
  ///        调用 enableFaceDetection{@link #IVideoEffect#enableFaceDetection} 注册了 IFaceDetectionObserver{@link #IFaceDetectionObserver}，并使用 RTC SDK 中包含的特效 SDK 进行视频特效处理时，你会收到此回调。
  /// @param result 人脸检测结果, 参看 FaceDetectionResult{@link #FaceDetectionResult}。
  ///

  FutureOr<void> onFaceDetectResult(FaceDetectionResult result) async {}

  /// @hidden for internal use only
  /// @detail callback
  /// @author zhushufan.ref
  /// @brief 特效 SDK 进行人像属性检测结果的回调。 <br>
  ///        调用 registerFaceDetectionObserver 注册了 IFaceDetectionObserver{@link #IFaceDetectionObserver}，并调用 setVideoEffectExpressionDetect{@link #IVideoEffect#setVideoEffectExpressionDetect} 设置开启人像属性检测后，你会收到此回调。
  /// @param result 人像属性检测结果, 参看 ExpressionDetectResult{@link #ExpressionDetectResult}。
  ///

  FutureOr<void> onExpressionDetectResult(
      ExpressionDetectResult result) async {}
}

class IMediaPlayerCustomSourceProvider extends NativeObserverClass {
  static const _$namespace =
      r'com.ss.bytertc.engine.IMediaPlayerCustomSourceProvider';
  static get codegen_$namespace => _$namespace;

  IMediaPlayerCustomSourceProvider([NativeClassOptions? options])
      : super(options == null
            ? const NativeClassOptions([],
                className: _$namespace,
                instanceType: InstanceType.manual,
                bridgeKey: 'com.volcengine.rtc.hybrid_runtime',
                methodMap: {r"onReadData": r"onReadData", r"onSeek": r"onSeek"})
            : NativeClassOptions.fromMap({
                ...options.toMap(),
                'bridgeKey': 'com.volcengine.rtc.hybrid_runtime',
              })) {
    registerEvent(r"onReadData", onReadData);

    registerEvent(r"onSeek", onSeek);
  }

  /// @valid since 3.53
  /// @detail callback
  /// @author songxiaomeng.19
  /// @brief 调用 openWithCustomSource{@link #IMediaPlayer#openWithCustomSource} 接口播放用户传入的内存音频数据时，会触发此回调，用户需要写入音频数据。
  /// @param buffer 内存地址。在该地址中写入音频数据，写入音频数据的大小不超过 bufferSize 中填入的数值。支持的音频数据格式有: mp3，aac，m4a，3gp，wav。
  /// @param bufferSize 音频数据大小，单位为字节。如果你想停止播放内存音频数据，可在 bufferSize 中填入小于或等于 0 的数，此时 SDK 会停止调用此接口。
  /// @return 返回实际读取的音频数据大小。
  /// @note 若 openWithCustomSource{@link #IMediaPlayer#openWithCustomSource} 接口调用失败，请在 buffer 和 bufferSize 两个参数中填入 0。 此时 SDK 会停止调用此接口。
  ///

  FutureOr<int> onReadData(ByteBuffer buffer, int bufferSize) async {
    throw UnimplementedError();
  }

  /// @valid since 3.53
  /// @detail callback
  /// @author songxiaomeng.19
  /// @brief 根据设置好的内存音频数据的读取位置和读取偏移量对音频数据进行偏移，以便 SDK 读取和分析音频数据。 <br>
  ///        在调用 openWithCustomSource{@link #IMediaPlayer#openWithCustomSource} 接口传入内存音频数据，或者调用 setPosition{@link #IMediaPlayer#setPosition} 设置了音频数据的起始播放位置后，SDK 会对音频数据进行读取和分析，此时会触发该回调，你需要根据参数中设置的起始读取位置和偏移量进行操作。
  /// @param offset 音频数据读取偏移量，单位为字节，取值可正可负。
  /// @param whence 音频数据的起始读取位置。参看 MediaPlayerCustomSourceSeekWhence{@link #MediaPlayerCustomSourceSeekWhence}
  /// @return
  ///         定位成功，返回偏移后的位置信息，或返回音频数据的大小。 <br>
  ///         定位失败，返回 -1。
  ///

  FutureOr<long> onSeek(
      long offset, MediaPlayerCustomSourceSeekWhence whence) async {
    throw UnimplementedError();
  }
}

class IExternalVideoEncoderEventHandler extends NativeObserverClass {
  static const _$namespace =
      r'com.ss.bytertc.engine.handler.IExternalVideoEncoderEventHandler';
  static get codegen_$namespace => _$namespace;

  IExternalVideoEncoderEventHandler([NativeClassOptions? options])
      : super(options == null
            ? const NativeClassOptions([],
                className: _$namespace,
                instanceType: InstanceType.manual,
                bridgeKey: 'com.volcengine.rtc.hybrid_runtime',
                methodMap: {
                  r"onStart": r"onStart",
                  r"onStop": r"onStop",
                  r"onRateUpdate": r"onRateUpdate",
                  r"onRequestKeyFrame": r"onRequestKeyFrame",
                  r"onActiveVideoLayer": r"onActiveVideoLayer"
                })
            : NativeClassOptions.fromMap({
                ...options.toMap(),
                'bridgeKey': 'com.volcengine.rtc.hybrid_runtime',
              })) {
    registerEvent(r"onStart", onStart);

    registerEvent(r"onStop", onStop);

    registerEvent(r"onRateUpdate", onRateUpdate);

    registerEvent(r"onRequestKeyFrame", onRequestKeyFrame);

    registerEvent(r"onActiveVideoLayer", onActiveVideoLayer);
  }

  /// @detail callback
  /// @author wangzhanqiang
  /// @brief 提示自定义编码帧可以开始推送的回调。 <br>
  ///        收到该回调后，你即可调用 pushExternalEncodedVideoFrame{@link #RTCEngine#pushExternalEncodedVideoFrame} 向 SDK 推送自定义编码视频帧
  /// @param streamId 可以推送的编码流的 ID
  /// @param streamInfo 可以推送的编码流的属性
  ///

  FutureOr<void> onStart(String streamId, StreamInfo streamInfo) async {}

  /// @detail callback
  /// @author wangzhanqiang
  /// @brief 当收到该回调时，你需停止向 SDK 推送自定义编码视频帧
  /// @param streamId 需停止推送的编码流的 ID
  /// @param streamInfo 需停止推送的编码流的属性
  ///

  FutureOr<void> onStop(String streamId, StreamInfo streamInfo) async {}

  /// @detail callback
  /// @author wangzhanqiang
  /// @brief 当自定义编码流的帧率或码率发生变化时，触发该回调
  /// @param streamId 发生变化的编码流的 ID
  /// @param streamInfo 发生变化的编码流的属性
  /// @param videoIndex 对应编码流的下标
  /// @param fps 变化后的帧率，单位：fps
  /// @param bitrateKbps 变化后的码率，单位：kbps
  ///

  FutureOr<void> onRateUpdate(String streamId, StreamInfo streamInfo,
      int videoIndex, int fps, int bitrateKbps) async {}

  /// @detail callback
  /// @author wangzhanqiang
  /// @brief 提示流发布端需重新生成关键帧的回调
  /// @param streamId 远端编码流的 ID
  /// @param streamInfo 远端编码流的属性
  /// @param videoIndex 对应编码流的下标
  ///

  FutureOr<void> onRequestKeyFrame(
      String streamId, StreamInfo streamInfo, int videoIndex) async {}

  /// @valid since 3.56
  /// @detail callback
  /// @author wangqianqian.1104
  /// @brief 作为自定义编码视频流的发送端，你会在视频流可发送状态发生变化时，收到此回调。 <br>
  ///        你可以根据此回调的提示，仅对可发送的视频流进行编码，以降低本端视频编码性能消耗。此回调会根据多个因素综合判断触发，包括：本端设备性能和本端网络性能，以及按需订阅场景下，远端用户是否订阅。
  /// @param streamId 编码流的 ID
  /// @param streamInfo 编码流的属性
  /// @param videoIndex 对应编码流的下标
  /// @param active 该路流可发送状态
  /// @note 要收到此回调，必须调用 setVideoSourceType{@link #RTCEngine#setVideoSourceType} 设置视频源是自定义编码，且通过 setExternalVideoEncoderEventHandler{@link #RTCEngine#setExternalVideoEncoderEventHandler} 设置了回调句柄。
  ///

  FutureOr<void> onActiveVideoLayer(String streamId, StreamInfo streamInfo,
      int videoIndex, boolean active) async {}
}

class ILocalEncodedVideoFrameObserver extends NativeObserverClass {
  static const _$namespace =
      r'com.ss.bytertc.engine.mediaio.ILocalEncodedVideoFrameObserver';
  static get codegen_$namespace => _$namespace;

  ILocalEncodedVideoFrameObserver([NativeClassOptions? options])
      : super(options == null
            ? const NativeClassOptions([],
                className: _$namespace,
                instanceType: InstanceType.manual,
                bridgeKey: 'com.volcengine.rtc.hybrid_runtime',
                methodMap: {
                  r"onLocalEncodedVideoFrame": r"onLocalEncodedVideoFrame"
                })
            : NativeClassOptions.fromMap({
                ...options.toMap(),
                'bridgeKey': 'com.volcengine.rtc.hybrid_runtime',
              })) {
    registerEvent(r"onLocalEncodedVideoFrame", onLocalEncodedVideoFrame);
  }

  /// @detail callback
  /// @brief 调用 registerLocalEncodedVideoFrameObserver{@link #RTCEngine#registerLocalEncodedVideoFrameObserver} 后，SDK 每次使用内部采集，采集到一帧视频帧，或收到一帧外部视频帧时，都会回调该事件。
  /// @param videoSource 预留参数
  /// @param encodedVideoFrame 本地视频帧信息，参看 RTCEncodedVideoFrame{@link #RTCEncodedVideoFrame}
  /// @note encodedVideoFrame 只在回调函数作用域内有效，不要存储该参数并在其它函数内访问该参数的内存数据

  FutureOr<void> onLocalEncodedVideoFrame(
      dynamic videoSource, RTCEncodedVideoFrame encodedVideoFrame) async {}
}

class IRTCEngineEventHandler extends NativeObserverClass {
  static const _$namespace =
      r'com.ss.bytertc.engine.handler.IRTCEngineEventHandler';
  static get codegen_$namespace => _$namespace;

  IRTCEngineEventHandler([NativeClassOptions? options])
      : super(options == null
            ? const NativeClassOptions([],
                className: _$namespace,
                instanceType: InstanceType.manual,
                bridgeKey: 'com.volcengine.rtc.hybrid_runtime',
                methodMap: {
                  r"onWarning": r"onWarning",
                  r"onError": r"onError",
                  r"onExtensionAccessError": r"onExtensionAccessError",
                  r"onSysStats": r"onSysStats",
                  r"onNetworkTypeChanged": r"onNetworkTypeChanged",
                  r"onUserStartVideoCapture": r"onUserStartVideoCapture",
                  r"onUserStopVideoCapture": r"onUserStopVideoCapture",
                  r"onUserStartAudioCapture": r"onUserStartAudioCapture",
                  r"onUserStopAudioCapture": r"onUserStopAudioCapture",
                  r"onLocalAudioStateChanged": r"onLocalAudioStateChanged",
                  r"onRemoteAudioStateChanged": r"onRemoteAudioStateChanged",
                  r"onLocalVideoStateChanged": r"onLocalVideoStateChanged",
                  r"onRemoteVideoStateChanged": r"onRemoteVideoStateChanged",
                  r"onRemoteVideoSuperResolutionModeChanged":
                      r"onRemoteVideoSuperResolutionModeChanged",
                  r"onVideoDenoiseModeChanged": r"onVideoDenoiseModeChanged",
                  r"onFirstRemoteVideoFrameRendered":
                      r"onFirstRemoteVideoFrameRendered",
                  r"onFirstRemoteVideoFrameDecoded":
                      r"onFirstRemoteVideoFrameDecoded",
                  r"onFirstLocalVideoFrameCaptured":
                      r"onFirstLocalVideoFrameCaptured",
                  r"onLocalVideoSizeChanged": r"onLocalVideoSizeChanged",
                  r"onRemoteVideoSizeChanged": r"onRemoteVideoSizeChanged",
                  r"onConnectionStateChanged": r"onConnectionStateChanged",
                  r"onAudioRouteChanged": r"onAudioRouteChanged",
                  r"onFirstLocalAudioFrame": r"onFirstLocalAudioFrame",
                  r"onFirstRemoteAudioFrame": r"onFirstRemoteAudioFrame",
                  r"onLogReport": r"onLogReport",
                  r"onSEIMessageReceived": r"onSEIMessageReceived",
                  r"onSEIStreamUpdate": r"onSEIStreamUpdate",
                  r"onLoginResult": r"onLoginResult",
                  r"onLogout": r"onLogout",
                  r"onServerParamsSetResult": r"onServerParamsSetResult",
                  r"onGetPeerOnlineStatus": r"onGetPeerOnlineStatus",
                  r"onUserMessageReceivedOutsideRoom":
                      r"onUserMessageReceivedOutsideRoom",
                  r"onUserBinaryMessageReceivedOutsideRoom":
                      r"onUserBinaryMessageReceivedOutsideRoom",
                  r"onUserMessageSendResultOutsideRoom":
                      r"onUserMessageSendResultOutsideRoom",
                  r"onServerMessageSendResult": r"onServerMessageSendResult",
                  r"onNetworkDetectionResult": r"onNetworkDetectionResult",
                  r"onNetworkDetectionStopped": r"onNetworkDetectionStopped",
                  r"onAudioDeviceStateChanged": r"onAudioDeviceStateChanged",
                  r"onVideoDeviceStateChanged": r"onVideoDeviceStateChanged",
                  r"onAudioDeviceWarning": r"onAudioDeviceWarning",
                  r"onVideoDeviceWarning": r"onVideoDeviceWarning",
                  r"onRecordingStateUpdate": r"onRecordingStateUpdate",
                  r"onRecordingProgressUpdate": r"onRecordingProgressUpdate",
                  r"onAudioRecordingStateUpdate":
                      r"onAudioRecordingStateUpdate",
                  r"onAudioMixingPlayingProgress":
                      r"onAudioMixingPlayingProgress",
                  r"onLocalAudioPropertiesReport":
                      r"onLocalAudioPropertiesReport",
                  r"onAudioPlaybackDeviceTestVolume":
                      r"onAudioPlaybackDeviceTestVolume",
                  r"onRemoteAudioPropertiesReport":
                      r"onRemoteAudioPropertiesReport",
                  r"onActiveSpeaker": r"onActiveSpeaker",
                  r"onEchoTestResult": r"onEchoTestResult",
                  r"onCloudProxyConnected": r"onCloudProxyConnected",
                  r"onAudioDumpStateChanged": r"onAudioDumpStateChanged",
                  r"onNetworkTimeSynchronized": r"onNetworkTimeSynchronized",
                  r"onLicenseWillExpire": r"onLicenseWillExpire",
                  r"onHardwareEchoDetectionResult":
                      r"onHardwareEchoDetectionResult",
                  r"onLocalProxyStateChanged": r"onLocalProxyStateChanged",
                  r"onEffectError": r"onEffectError",
                  r"onStreamSyncInfoReceived": r"onStreamSyncInfoReceived",
                  r"onExternalScreenFrameUpdate":
                      r"onExternalScreenFrameUpdate",
                  r"onRemoteSnapshotTakenToFile":
                      r"onRemoteSnapshotTakenToFile",
                  r"onAudioFrameSendStateChanged":
                      r"onAudioFrameSendStateChanged",
                  r"onVideoFrameSendStateChanged":
                      r"onVideoFrameSendStateChanged",
                  r"onAudioFramePlayStateChanged":
                      r"onAudioFramePlayStateChanged",
                  r"onVideoFramePlayStateChanged":
                      r"onVideoFramePlayStateChanged",
                  r"onSimulcastSubscribeFallback":
                      r"onSimulcastSubscribeFallback",
                  r"onPerformanceAlarms": r"onPerformanceAlarms",
                  r"onRemoteAudioPropertiesReportEx":
                      r"onRemoteAudioPropertiesReportEx",
                  r"onMixedStreamEvent": r"onMixedStreamEvent",
                  r"onSingleStreamEvent": r"onSingleStreamEvent",
                  r"onExperimentalCallback": r"onExperimentalCallback",
                  r"onPushPublicStreamResult": r"onPushPublicStreamResult"
                })
            : NativeClassOptions.fromMap({
                ...options.toMap(),
                'bridgeKey': 'com.volcengine.rtc.hybrid_runtime',
              })) {
    registerEvent(r"onWarning", onWarning);

    registerEvent(r"onError", onError);

    registerEvent(r"onExtensionAccessError", onExtensionAccessError);

    registerEvent(r"onSysStats", onSysStats);

    registerEvent(r"onNetworkTypeChanged", onNetworkTypeChanged);

    registerEvent(r"onUserStartVideoCapture", onUserStartVideoCapture);

    registerEvent(r"onUserStopVideoCapture", onUserStopVideoCapture);

    registerEvent(r"onUserStartAudioCapture", onUserStartAudioCapture);

    registerEvent(r"onUserStopAudioCapture", onUserStopAudioCapture);

    registerEvent(r"onLocalAudioStateChanged", onLocalAudioStateChanged);

    registerEvent(r"onRemoteAudioStateChanged", onRemoteAudioStateChanged);

    registerEvent(r"onLocalVideoStateChanged", onLocalVideoStateChanged);

    registerEvent(r"onRemoteVideoStateChanged", onRemoteVideoStateChanged);

    registerEvent(r"onRemoteVideoSuperResolutionModeChanged",
        onRemoteVideoSuperResolutionModeChanged);

    registerEvent(r"onVideoDenoiseModeChanged", onVideoDenoiseModeChanged);

    registerEvent(
        r"onFirstRemoteVideoFrameRendered", onFirstRemoteVideoFrameRendered);

    registerEvent(
        r"onFirstRemoteVideoFrameDecoded", onFirstRemoteVideoFrameDecoded);

    registerEvent(
        r"onFirstLocalVideoFrameCaptured", onFirstLocalVideoFrameCaptured);

    registerEvent(r"onLocalVideoSizeChanged", onLocalVideoSizeChanged);

    registerEvent(r"onRemoteVideoSizeChanged", onRemoteVideoSizeChanged);

    registerEvent(r"onConnectionStateChanged", onConnectionStateChanged);

    registerEvent(r"onAudioRouteChanged", onAudioRouteChanged);

    registerEvent(r"onFirstLocalAudioFrame", onFirstLocalAudioFrame);

    registerEvent(r"onFirstRemoteAudioFrame", onFirstRemoteAudioFrame);

    registerEvent(r"onLogReport", onLogReport);

    registerEvent(r"onSEIMessageReceived", onSEIMessageReceived);

    registerEvent(r"onSEIStreamUpdate", onSEIStreamUpdate);

    registerEvent(r"onLoginResult", onLoginResult);

    registerEvent(r"onLogout", onLogout);

    registerEvent(r"onServerParamsSetResult", onServerParamsSetResult);

    registerEvent(r"onGetPeerOnlineStatus", onGetPeerOnlineStatus);

    registerEvent(
        r"onUserMessageReceivedOutsideRoom", onUserMessageReceivedOutsideRoom);

    registerEvent(r"onUserBinaryMessageReceivedOutsideRoom",
        onUserBinaryMessageReceivedOutsideRoom);

    registerEvent(r"onUserMessageSendResultOutsideRoom",
        onUserMessageSendResultOutsideRoom);

    registerEvent(r"onServerMessageSendResult", onServerMessageSendResult);

    registerEvent(r"onNetworkDetectionResult", onNetworkDetectionResult);

    registerEvent(r"onNetworkDetectionStopped", onNetworkDetectionStopped);

    registerEvent(r"onAudioDeviceStateChanged", onAudioDeviceStateChanged);

    registerEvent(r"onVideoDeviceStateChanged", onVideoDeviceStateChanged);

    registerEvent(r"onAudioDeviceWarning", onAudioDeviceWarning);

    registerEvent(r"onVideoDeviceWarning", onVideoDeviceWarning);

    registerEvent(r"onRecordingStateUpdate", onRecordingStateUpdate);

    registerEvent(r"onRecordingProgressUpdate", onRecordingProgressUpdate);

    registerEvent(r"onAudioRecordingStateUpdate", onAudioRecordingStateUpdate);

    registerEvent(
        r"onAudioMixingPlayingProgress", onAudioMixingPlayingProgress);

    registerEvent(
        r"onLocalAudioPropertiesReport", onLocalAudioPropertiesReport);

    registerEvent(
        r"onAudioPlaybackDeviceTestVolume", onAudioPlaybackDeviceTestVolume);

    registerEvent(
        r"onRemoteAudioPropertiesReport", onRemoteAudioPropertiesReport);

    registerEvent(r"onActiveSpeaker", onActiveSpeaker);

    registerEvent(r"onEchoTestResult", onEchoTestResult);

    registerEvent(r"onCloudProxyConnected", onCloudProxyConnected);

    registerEvent(r"onAudioDumpStateChanged", onAudioDumpStateChanged);

    registerEvent(r"onNetworkTimeSynchronized", onNetworkTimeSynchronized);

    registerEvent(r"onLicenseWillExpire", onLicenseWillExpire);

    registerEvent(
        r"onHardwareEchoDetectionResult", onHardwareEchoDetectionResult);

    registerEvent(r"onLocalProxyStateChanged", onLocalProxyStateChanged);

    registerEvent(r"onEffectError", onEffectError);

    registerEvent(r"onStreamSyncInfoReceived", onStreamSyncInfoReceived);

    registerEvent(r"onExternalScreenFrameUpdate", onExternalScreenFrameUpdate);

    registerEvent(r"onRemoteSnapshotTakenToFile", onRemoteSnapshotTakenToFile);

    registerEvent(
        r"onAudioFrameSendStateChanged", onAudioFrameSendStateChanged);

    registerEvent(
        r"onVideoFrameSendStateChanged", onVideoFrameSendStateChanged);

    registerEvent(
        r"onAudioFramePlayStateChanged", onAudioFramePlayStateChanged);

    registerEvent(
        r"onVideoFramePlayStateChanged", onVideoFramePlayStateChanged);

    registerEvent(
        r"onSimulcastSubscribeFallback", onSimulcastSubscribeFallback);

    registerEvent(r"onPerformanceAlarms", onPerformanceAlarms);

    registerEvent(
        r"onRemoteAudioPropertiesReportEx", onRemoteAudioPropertiesReportEx);

    registerEvent(r"onMixedStreamEvent", onMixedStreamEvent);

    registerEvent(r"onSingleStreamEvent", onSingleStreamEvent);

    registerEvent(r"onExperimentalCallback", onExperimentalCallback);

    registerEvent(r"onPushPublicStreamResult", onPushPublicStreamResult);
  }

  /// @detail callback
  /// @author panjian.fishing
  /// @brief 发生警告回调。 <br>
  ///        SDK 运行时出现了警告。SDK 通常会自动恢复，警告信息可以忽略。
  /// @param warn 警告代码，参见 WarningCode{@link #WarningCode}
  ///

  FutureOr<void> onWarning(WarningCode warn) async {}

  /// @detail callback
  /// @author panjian.fishing
  /// @brief 发生错误回调。 <br>
  ///        SDK 运行时出现了网络或媒体相关的错误，且无法自动恢复时触发此回调。 <br>
  ///        你可能需要干预.
  /// @param err 错误代码，详情定义见: ErrorCode{@link #ErrorCode}
  ///

  FutureOr<void> onError(ErrorCode err) async {}

  /// @valid since 3.52
  /// @detail callback
  /// @author zhanyunqiao
  /// @brief 当访问插件失败时，收到此回调。 <br>
  ///        RTC SDK 将一些功能封装成插件。当使用这些功能时，如果插件不存在，功能将无法使用。
  /// @param extensionName 插件名字
  /// @param msg 失败说明
  ///

  FutureOr<void> onExtensionAccessError(
      String extensionName, String msg) async {}

  /// @detail callback
  /// @author wangzhanqiang
  /// @brief 每 2 秒发生回调，通知当前 cpu，内存使用的信息。
  /// @param stats cpu，内存信息。详见 SysStats{@link #SysStats} 数据类型。
  ///

  FutureOr<void> onSysStats(SysStats stats) async {}

  /// @detail callback
  /// @author hanchenchen.c
  /// @brief SDK 当前网络连接类型改变回调。当 SDK 的当前网络连接类型发生改变时回调该事件。
  /// @param type <br>
  ///        网络类型。 <br>
  ///        - -1： 网络连接类型未知。
  ///        - 0： 网络连接已断开。
  ///        - 1： LAN
  ///        - 2： Wi-Fi,包含热点
  ///        - 3： 2G 移动网络
  ///        - 4： 3G 移动网络
  ///        - 5： 4G 移动网络
  ///        - 6： 5G 移动网络
  ///

  FutureOr<void> onNetworkTypeChanged(NetworkType type) async {}

  /// @detail callback
  /// @author liuyangyang
  /// @brief 房间内的可见用户调用 startVideoCapture{@link #RTCEngine#startVideoCapture} 开启内部视频采集时，房间内其他用户会收到此回调。
  /// @param streamId 视频流 ID
  /// @param streamInfo 视频流信息，详见 StreamInfo{@link #StreamInfo}。
  ///

  FutureOr<void> onUserStartVideoCapture(
      String streamId, StreamInfo streamInfo) async {}

  /// @detail callback
  /// @author liuyangyang
  /// @brief 房间内的可见用户调用 stopVideoCapture{@link #RTCEngine#stopVideoCapture} 关闭内部视频采集时，房间内其他用户会收到此回调。 <br>
  ///        若发布视频数据前未开启采集，房间内所有可见用户会收到此回调。
  /// @param streamId 视频流 ID
  /// @param streamInfo 视频流信息，详见 StreamInfo{@link #StreamInfo}。
  ///

  FutureOr<void> onUserStopVideoCapture(
      String streamId, StreamInfo streamInfo) async {}

  /// @detail callback
  /// @author dixing
  /// @brief 房间内的用户调用 startAudioCapture{@link #RTCEngine#startAudioCapture} 开启音频采集时，房间内其他用户会收到此回调。
  /// @param streamId 开启音频采集的远端流 ID
  /// @param streamInfo 开启音频采集的远端流信息，详见 StreamInfo{@link #StreamInfo}
  ///

  FutureOr<void> onUserStartAudioCapture(
      String streamId, StreamInfo streamInfo) async {}

  /// @detail callback
  /// @author dixing
  /// @brief 房间内的用户调用 stopAudioCapture{@link #RTCEngine#stopAudioCapture} 关闭音频采集时，房间内其他用户会收到此回调。
  /// @param streamId 流 ID
  /// @param streamInfo 流信息，详见 StreamInfo{@link #StreamInfo}
  ///

  FutureOr<void> onUserStopAudioCapture(
      String streamId, StreamInfo streamInfo) async {}

  /// @detail callback
  /// @author zhangyuanyuan.0101
  /// @brief 本地音频流的状态发生改变时，收到此回调。
  /// @param audioSource 预留参数
  /// @param state 本地音频设备的状态，详见 LocalAudioStreamState{@link #LocalAudioStreamState}
  /// @param error 本地音频流状态改变时的错误码，详见 LocalAudioStreamError{@link #LocalAudioStreamError}
  ///

  FutureOr<void> onLocalAudioStateChanged(Map audioSource,
      LocalAudioStreamState state, LocalAudioStreamError error) async {}

  /// @detail callback
  /// @author zhangyuanyuan.0101
  /// @brief 用户订阅来自远端的音频流状态发生改变时，会收到此回调，了解当前的远端音频流状态。
  /// @param streamId 远端流 ID
  /// @param streamInfo 远端流信息, 详见 StreamInfo{@link #StreamInfo}
  /// @param state 远端音频流状态，详见 RemoteAudioState{@link #RemoteAudioState}
  /// @param reason 远端音频流状态改变的原因，详见 RemoteAudioStateChangeReason{@link #RemoteAudioStateChangeReason}
  ///

  FutureOr<void> onRemoteAudioStateChanged(
      String streamId,
      StreamInfo streamInfo,
      RemoteAudioState state,
      RemoteAudioStateChangeReason reason) async {}

  /// @detail callback
  /// @author shenpengliang
  /// @brief 本地视频流的状态发生改变时，收到该事件。
  /// @param videoSource  预留参数
  /// @param state 本地视频流状态，参看 LocalVideoStreamState{@link #LocalVideoStreamState}
  /// @param error 本地视频状态改变时的错误码，参看 LocalVideoStreamError{@link #LocalVideoStreamError}
  ///

  FutureOr<void> onLocalVideoStateChanged(Map videoSource,
      LocalVideoStreamState state, LocalVideoStreamError error) async {}

  /// @detail callback
  /// @author shenpengliang
  /// @brief 远端视频流的状态发生改变时，房间内订阅此流的用户会收到该事件。
  /// @param streamId 远端流 ID
  /// @param streamInfo 远端流信息, 详见 StreamInfo{@link #StreamInfo}
  /// @param videoState 远端视频流状态，参看 RemoteVideoState{@link #RemoteVideoState}
  /// @param videoStateReason 远端视频流状态改变原因，参看 RemoteVideoStateChangeReason{@link #RemoteVideoStateChangeReason}
  /// @note 本回调仅适用于主流，不适用于屏幕流。
  ///

  FutureOr<void> onRemoteVideoStateChanged(
      String streamId,
      StreamInfo streamInfo,
      RemoteVideoState videoState,
      RemoteVideoStateChangeReason videoStateReason) async {}

  /// @hidden for internal use only
  /// @valid since 3.54
  /// @detail callback
  /// @author yinkaisheng
  /// @brief 远端视频流的超分状态发生改变时，房间内订阅此流的用户会收到该回调。
  /// @param streamId 远端流 ID
  /// @param streamInfo 远端流信息, 详见 StreamInfo{@link #StreamInfo}
  /// @param mode 超分模式，参看 VideoSuperResolutionMode{@link #VideoSuperResolutionMode}。
  /// @param reason 超分模式改变原因，参看 VideoSuperResolutionModeChangedReason{@link #VideoSuperResolutionModeChangedReason}。
  ///

  FutureOr<void> onRemoteVideoSuperResolutionModeChanged(
      String streamId,
      StreamInfo streamInfo,
      VideoSuperResolutionMode mode,
      VideoSuperResolutionModeChangedReason reason) async {}

  /// @hidden for internal use only
  /// @valid since 3.54
  /// @detail callback
  /// @author Yujianli
  /// @brief 降噪模式状态变更回调。当降噪模式的运行状态发生改变，SDK 会触发该回调，提示用户降噪模式改变后的运行状态及状态发生改变的原因。
  /// @param mode 视频降噪模式，参看 VideoDenoiseMode{@link #VideoDenoiseMode}。
  /// @param reason 视频降噪模式改变的原因，参看 VideoDenoiseModeChangedReason{@link #VideoDenoiseModeChangedReason}。
  ///

  FutureOr<void> onVideoDenoiseModeChanged(
      VideoDenoiseMode mode, VideoDenoiseModeChangedReason reason) async {}

  /// @detail callback
  /// @author zhushufan.ref
  /// @brief SDK 内部渲染成功远端视频流首帧后，收到此回调。包含以下情况： <br>
  ///        1. 发布端首次发布视频 <br>
  ///        2. 在 1 条件下，发布端取消发布视频后，再次发布视频 <br>
  ///        3. 在 1 条件下，发布端关闭视频采集后，再次打开采集（或使用外部源时，停止推流后再次推流） <br>
  ///        4. 在 1 条件下，订阅端取消订阅视频后，再次订阅视频（调用接口包括 subscribeAllStreamsVideo{@link #RTCRoom-subscribeAllStreamsVideo}，pauseAllSubscribedStreamVideo{@link #RTCRoom#pauseAllSubscribedStreamVideo}。
  /// @param streamId 远端流 ID
  /// @param streamInfo 远端流信息, 详见 StreamInfo{@link #StreamInfo}
  /// @param frameInfo 视频帧信息，参看 VideoFrameInfo{@link #VideoFrameInfo}
  /// @note 仅在采用内部渲染时，符合上述策略。
  ///

  FutureOr<void> onFirstRemoteVideoFrameRendered(
      String streamId, StreamInfo streamInfo, VideoFrameInfo frameInfo) async {}

  /// @detail callback
  /// @author zhushufan.ref
  /// @brief 订阅端接收并解码远端视频流首帧时，收到此回调。包含以下情况： <br>
  ///        1. 发布端发布视频，包含首次发布和取消后再次发布。<br>
  ///        2. 发布端关闭视频采集后，再次打开采集。使用外部源时，停止推流后再次推流。<br>
  ///        3. 发布端发布视频后，订阅端取消订阅视频后，又再次订阅视频。
  /// @param streamId 远端流 ID
  /// @param streamInfo 远端流信息, 详见 StreamInfo{@link #StreamInfo}
  /// @param frameInfo 视频帧信息，参看 VideoFrameInfo{@link #VideoFrameInfo}
  /// @note
  ///       - 用户刚收到房间内订阅的每一路视频流时，都会收到该回调。
  ///       - 摄像头流、屏幕流，内部采集、外部源，自动订阅和手动订阅的视频流，都符合上述策略。
  ///

  FutureOr<void> onFirstRemoteVideoFrameDecoded(
      String streamId, StreamInfo streamInfo, VideoFrameInfo frameInfo) async {}

  /// @detail callback
  /// @author zhangzhenyu.samuel
  /// @brief RTC SDK 在本地完成第一帧视频帧或屏幕视频帧采集时，收到此回调。
  /// @param videoSource 预留参数。
  /// @param frameInfo 视频信息，参看 VideoFrameInfo{@link #VideoFrameInfo}
  /// @note 对于采集到的本地视频帧，你可以调用 setLocalVideoCanvas{@link #RTCEngine#setLocalVideoCanvas} 或 setLocalVideoSink{@link #RTCEngine#setLocalVideoSink} 在本地渲染。
  ///

  FutureOr<void> onFirstLocalVideoFrameCaptured(
      Map videoSource, VideoFrameInfo frameInfo) async {}

  /// @detail callback
  /// @author zhushufan.ref
  /// @brief 本地预览视频大小或旋转信息发生改变时，收到此回调。
  /// @param videoSource 预留参数。
  /// @param frameInfo 视频帧信息，参看 VideoFrameInfo{@link #VideoFrameInfo}
  ///

  FutureOr<void> onLocalVideoSizeChanged(
      Map videoSource, VideoFrameInfo frameInfo) async {}

  /// @detail callback
  /// @author zhushufan.ref
  /// @brief 远端视频大小或旋转信息发生改变时，房间内订阅此视频流的用户会收到此回调。
  /// @param streamId 远端流 ID
  /// @param streamInfo 远端流信息, 详见 StreamInfo{@link #StreamInfo}
  /// @param frameInfo 视频帧信息，参看 VideoFrameInfo{@link #VideoFrameInfo}
  ///

  FutureOr<void> onRemoteVideoSizeChanged(
      String streamId, StreamInfo streamInfo, VideoFrameInfo frameInfo) async {}

  /// @detail callback
  /// @author hanchenchen.c
  /// @brief 回调 SDK 与信令服务器连接状态相关事件。当 SDK 与信令服务器的网络连接状态改变时回调该事件。
  /// @param state <br>
  ///        当前 SDK 与信令服务器连接状态。 详细定义参见 ConnectionState{@link #ConnectionState}
  /// @param reason <br>
  ///        引起信令服务器连接状态发生改变的原因，目前未启用固定为 -1 。
  /// @note 更多信息参见 [连接状态提示](https://www.volcengine.com/docs/6348/95376)。
  ///

  FutureOr<void> onConnectionStateChanged(ConnectionState state) async {}

  /// @detail callback
  /// @author dixing
  /// @brief 音频播放路由变化时，收到该回调。
  /// @param route 新的音频播放路由，详见 AudioRoute{@link #AudioRoute}
  /// @note 插拔音频外设，或调用 setAudioRoute{@link #RTCEngine#setAudioRoute} 都可能触发音频路由切换，详见[移动端设置音频路由](https://www.volcengine.com/docs/6348/117836) 。
  ///

  FutureOr<void> onAudioRouteChanged(AudioRoute route) async {}

  /// @detail callback
  /// @author zhangyuanyuan.0101
  /// @brief 发布音频流时，采集到第一帧音频帧，收到该回调。
  /// @param audioSource 预留参数
  /// @note 如果发布音频流时，未开启本地音频采集，SDK 会推送静音帧，也会收到此回调。
  ///

  FutureOr<void> onFirstLocalAudioFrame(Map audioSource) async {}

  /// @detail callback
  /// @author zhangyuanyuan.0101
  /// @brief 订阅端接收并解码远端音频流首帧时，收到此回调。包含以下情况： <br>
  ///        1. 发布端发布音频，包含首次发布和取消后再次发布。<br>
  ///        2. 发布端关闭音频采集后，再次打开采集。使用外部源时，停止推流后再次推流。<br>
  ///        3. 发布端发布音频后，订阅端取消订阅音频后，又再次订阅音频。
  /// @param streamId 远端流 ID
  /// @param streamInfo 远端流信息, 详见 StreamInfo{@link #StreamInfo}
  /// @note
  ///        - 用户刚收到房间内订阅的每一路音频流时，都会收到该回调。
  ///        - 摄像头流、屏幕流，内部采集、外部源、自动订阅和手动订阅的音频流，都符合上述策略。
  ///

  FutureOr<void> onFirstRemoteAudioFrame(
      String streamId, StreamInfo streamInfo) async {}

  /// @detail callback
  /// @author chenweiming.push
  /// @brief 上报日志时回调该事件。
  /// @param logType <br>
  ///        日志类型。
  /// @param logContent <br>
  ///        日志内容。
  ///

  FutureOr<void> onLogReport(String logType, JSONObject logContent) async {}

  /// @detail callback
  /// @author wangzhanqiang
  /// @brief 收到通过调用 sendSEIMessage{@link #RTCEngine#sendSEIMessage} 发送带有 SEI 消息的视频帧时，收到此回调。
  /// @param streamId 包含 SEI 发送者的流 ID
  /// @param streamInfo 包含 SEI 发送者的流信息, 详见 StreamInfo{@link #StreamInfo}
  /// @param message 收到的 SEI 消息内容
  ///

  FutureOr<void> onSEIMessageReceived(
      String streamId, StreamInfo streamInfo, Uint8List message) async {}

  /// @detail callback
  /// @author wangzhanqiang
  /// @brief 黑帧视频流发布状态回调。 <br>
  ///        在语音通话场景下，本地用户调用 sendSEIMessage{@link #RTCEngine#sendSEIMessage} 通过黑帧视频流发送 SEI 数据时，流的发送状态会通过该回调通知远端用户。 <br>
  ///        你可以通过此回调判断携带 SEI 数据的视频帧为黑帧，从而不对该视频帧进行渲染。
  /// @param streamId 远端流 ID
  /// @param streamInfo 远端流信息，参看 StreamInfo{@link #StreamInfo}。
  /// @param event 黑帧视频流状态，参看 SEIStreamUpdateEvent{@link #SEIStreamUpdateEvent}
  ///

  FutureOr<void> onSEIStreamUpdate(String streamId, StreamInfo streamInfo,
      SEIStreamUpdateEvent event) async {}

  /// @detail callback
  /// @author hanchenchen.c
  /// @brief 登录结果回调
  /// @param uid <br>
  ///        登录用户 ID
  /// @param errorCode <br>
  ///        登录结果 <br>
  ///        详见 LoginErrorCode{@link #LoginErrorCode}。
  /// @param elapsed <br>
  ///        从调用 login{@link #RTCEngine#login} 接口开始到返回结果所用时长。 <br>
  ///        单位为 ms。
  /// @note 调用 login{@link #RTCEngine#login} 后，会收到此回调。
  ///

  FutureOr<void> onLoginResult(
      String uid, LoginErrorCode errorCode, int elapsed) async {}

  /// @detail callback
  /// @author hanchenchen.c
  /// @brief 登出结果回调
  /// @param reason 用户登出的原因，参看 LogoutReason{@link #LogoutReason}
  /// @note 在以下两种情况下会收到此回调：调用 logout{@link #RTCEngine#logout} 接口主动退出；或其他用户以相同 UserId 进行 `login` 导致本地用户被动登出。
  ///

  FutureOr<void> onLogout(LogoutReason reason) async {}

  /// @detail callback
  /// @author hanchenchen.c
  /// @brief 设置应用服务器参数的返回结果
  /// @param error <br>
  ///        设置结果 <br>
  ///        - 返回 200，设置成功
  ///        - 返回其他，设置失败，详见 UserMessageSendResult{@link #UserMessageSendResult}
  /// @note 调用 setServerParams{@link #RTCEngine#setServerParams} 后，会收到此回调。
  ///

  FutureOr<void> onServerParamsSetResult(int error) async {}

  /// @detail callback
  /// @author hanchenchen.c
  /// @brief 查询对端或本端用户登录状态的返回结果
  /// @param peerUserId <br>
  ///        需要查询的用户 ID
  /// @param status <br>
  ///        查询的用户登录状态 <br>
  ///        详见 UserOnlineStatus{@link #UserOnlineStatus}.
  /// @note 必须先调用 getPeerOnlineStatus{@link #RTCEngine#getPeerOnlineStatus}，才能收到此回调。
  ///

  FutureOr<void> onGetPeerOnlineStatus(
      String peerUserId, UserOnlineStatus status) async {}

  /// @detail callback
  /// @author hanchenchen.c
  /// @brief 收到房间外用户调用 sendUserMessageOutsideRoom{@link #RTCEngine#sendUserMessageOutsideRoom} 发来的文本消息时，会收到此回调。
  /// @param msgid 消息编号。
  /// @param uid 消息发送者 ID。
  /// @param message 收到的文本消息内容。
  ///

  FutureOr<void> onUserMessageReceivedOutsideRoom(
      long msgid, String uid, String message) async {}

  /// @detail callback
  /// @author hanchenchen.c
  /// @brief 收到房间外用户调用 sendUserBinaryMessageOutsideRoom{@link #RTCEngine#sendUserBinaryMessageOutsideRoom} 发来的二进制消息时，会收到此回调。
  /// @param msgid 消息编号。
  /// @param uid 消息发送者 ID。
  /// @param message 收到的二进制消息内容。
  ///

  FutureOr<void> onUserBinaryMessageReceivedOutsideRoom(
      long msgid, String uid, Uint8List message) async {}

  /// @detail callback
  /// @author hanchenchen.c
  /// @brief 给房间外指定的用户发送消息的结果回调。<br>
  ///        当调用 sendUserMessageOutsideRoom{@link #RTCEngine#sendUserMessageOutsideRoom} 或 sendUserBinaryMessageOutsideRoom{@link #RTCEngine#sendUserBinaryMessageOutsideRoom} 发送消息后，会收到此回调。
  /// @param msgid 消息 ID。<br>
  ///        所有的 P2P 和 P2Server 消息共用一个 ID 序列。
  /// @param error 消息发送结果。详见 UserMessageSendResult{@link #UserMessageSendResult}。
  ///

  FutureOr<void> onUserMessageSendResultOutsideRoom(
      long msgid, UserMessageSendResult error) async {}

  /// @detail callback
  /// @author hanchenchen.c
  /// @brief 给应用服务器发送消息的回调。
  /// @param msgid 本条消息的 ID <br>
  ///        所有的 P2P 和 P2Server 消息共用一个 ID 序列。
  /// @param error 消息发送结果，详见 UserMessageSendResult{@link #UserMessageSendResult}。
  /// @param message 应用服务器收到 HTTP 请求后，在 ACK 中返回的信息。消息不超过 64 KB。
  /// @note 本回调为异步回调。当调用 sendServerMessage{@link #RTCEngine#sendServerMessage} 或 sendServerBinaryMessage{@link #RTCEngine#sendServerBinaryMessage} 接口发送消息后，会收到此回调。
  ///

  FutureOr<void> onServerMessageSendResult(
      long msgid, UserMessageSendResult error, Uint8List message) async {}

  /// @detail callback
  /// @author hanchenchen.c
  /// @brief 通话前网络探测结果。 <br>
  ///        成功调用 startNetworkDetection{@link #RTCEngine#startNetworkDetection} 接口开始探测后，会在 3s 内首次收到该回调，之后每 2s 收到一次该回调。
  /// @param type 探测网络类型为上行/下行
  /// @param quality 探测网络的质量，参看 NetworkQuality{@link #NetworkQuality}。
  /// @param rtt 探测网络的 RTT，单位：ms
  /// @param lostRate 探测网络的丢包率
  /// @param bitrate 探测网络的带宽，单位：kbps
  /// @param jitter 探测网络的抖动,单位：ms
  ///

  FutureOr<void> onNetworkDetectionResult(
      NetworkDetectionLinkType type,
      NetworkQuality quality,
      int rtt,
      double lostRate,
      int bitrate,
      int jitter) async {}

  /// @detail callback
  /// @author hanchenchen.c
  /// @brief 通话前网络探测结束 <br>
  ///        以下情况将停止探测并收到本一次本回调： <br>
  ///        1. 当调用 stopNetworkDetection{@link #RTCEngine#stopNetworkDetection} 接口停止探测后，会收到一次该回调； <br>
  ///        2. 当收到远端/本端音频首帧后，停止探测； <br>
  ///        3. 当探测超过 3 分钟后，停止探测； <br>
  ///        4. 当探测链路断开一定时间之后，停止探测。
  /// @param reason <br>
  ///        停止探测的原因类型,参考 NetworkDetectionStopReason{@link #NetworkDetectionStopReason}
  ///

  FutureOr<void> onNetworkDetectionStopped(
      NetworkDetectionStopReason reason) async {}

  /// @detail callback
  /// @author dixing
  /// @brief 音频设备状态回调。提示音频采集、音频播放等媒体设备的状态。
  /// @param deviceID 设备 ID
  /// @param deviceType 设备类型，详见 AudioDeviceType{@link #AudioDeviceType}。
  /// @param deviceState 设备状态，详见 MediaDeviceState{@link #MediaDeviceState}。
  /// @param deviceError 设备错误类型，详见 MediaDeviceError{@link #MediaDeviceError}。
  ///

  FutureOr<void> onAudioDeviceStateChanged(
      String deviceID,
      AudioDeviceType deviceType,
      MediaDeviceState deviceState,
      MediaDeviceError deviceError) async {}

  /// @detail callback
  /// @author liuyangyang
  /// @brief 视频设备状态回调。提示摄像头视频采集、屏幕视频采集等媒体设备的状态。
  /// @param deviceID 设备 ID
  /// @param deviceType 设备类型，详见 VideoDeviceType{@link #VideoDeviceType}。
  /// @param deviceState 设备状态，详见 MediaDeviceState{@link #MediaDeviceState}。
  /// @param deviceError 设备错误类型，详见 MediaDeviceError{@link #MediaDeviceError}。
  ///

  FutureOr<void> onVideoDeviceStateChanged(
      String deviceID,
      VideoDeviceType deviceType,
      MediaDeviceState deviceState,
      MediaDeviceError deviceError) async {}

  /// @detail callback
  /// @author dixing
  /// @brief 音频设备警告回调。音频设备包括音频采集设备、音频渲染设备等。
  /// @param deviceID 设备 ID
  /// @param deviceType 参看 AudioDeviceType{@link #AudioDeviceType}
  /// @param deviceWarning 参看 MediaDeviceWarning{@link #MediaDeviceWarning}
  ///

  FutureOr<void> onAudioDeviceWarning(String deviceID,
      AudioDeviceType deviceType, MediaDeviceWarning deviceWarning) async {}

  /// @detail callback
  /// @author liuyangyang
  /// @brief 视频设备警告回调，包括视频采集等设备。
  /// @param deviceID 设备 ID
  /// @param deviceType 参看 VideoDeviceType{@link #VideoDeviceType}
  /// @param deviceWarning 参看 MediaDeviceWarning{@link #MediaDeviceWarning}
  ///

  FutureOr<void> onVideoDeviceWarning(String deviceID,
      VideoDeviceType deviceType, MediaDeviceWarning deviceWarning) async {}

  /// @detail callback
  /// @author wangzhanqiang
  /// @brief 获取本地录制状态回调。 <br>
  ///        该回调由 startFileRecording{@link #RTCEngine#startFileRecording} 或 stopFileRecording{@link #RTCEngine#stopFileRecording} 触发。
  /// @param videoSource 预留参数。
  /// @param state 录制状态，参看 RecordingState{@link #RecordingState}
  /// @param errorCode 录制错误码，参看 RecordingErrorCode{@link #RecordingErrorCode}
  /// @param info 录制文件的详细信息，参看 RecordingInfo{@link #RecordingInfo}
  ///

  FutureOr<void> onRecordingStateUpdate(Map videoSource, RecordingState state,
      RecordingErrorCode errorCode, RecordingInfo info) async {}

  /// @detail callback
  /// @author wangzhanqiang
  /// @brief 本地录制进度回调。 <br>
  ///        该回调由 startFileRecording{@link #RTCEngine#startFileRecording} 触发，录制状态正常时，系统每秒钟都会通过该回调提示录制进度。
  /// @param videoSource 预留参数。
  /// @param progress 录制进度，参看 RecordingProgress{@link #RecordingProgress}
  /// @param info 录制文件的详细信息，参看 RecordingInfo{@link #RecordingInfo}
  ///

  FutureOr<void> onRecordingProgressUpdate(
      Map videoSource, RecordingProgress progress, RecordingInfo info) async {}

  /// @detail callback
  /// @author huangshouqin
  /// @brief 调用 startAudioRecording{@link #RTCEngine#startAudioRecording} 或 stopAudioRecording{@link #RTCEngine#stopAudioRecording} 改变音频文件录制状态时，收到此回调。
  /// @param state 录制状态，参看 AudioRecordingState{@link #AudioRecordingState}
  /// @param errorCode 录制错误码，参看 AudioRecordingErrorCode{@link #AudioRecordingErrorCode}
  ///

  FutureOr<void> onAudioRecordingStateUpdate(
      AudioRecordingState state, AudioRecordingErrorCode errorCode) async {}

  /// @detail callback
  /// @author songxiaomeng.19
  /// @brief 混音音频文件播放进度回调
  /// @param mixId 混音 ID
  /// @param progress 当前混音音频文件播放进度，单位毫秒
  /// @note 调用 setAudioMixingProgressInterval 将时间间隔设为大于 0 的值后，或调用 startAudioMixing 将 AudioMixingConfig 中的时间间隔设为大于 0 的值后，SDK 会按照设置的时间间隔回调该事件。
  ///

  FutureOr<void> onAudioMixingPlayingProgress(int mixId, long progress) async {}

  /// @detail callback
  /// @author wangjunzheng
  /// @brief 调用 enableAudioPropertiesReport{@link #RTCEngine#enableAudioPropertiesReport} 后，你会周期性地收到此回调，了解本地音频的瞬时相关信息。 <br>
  ///        本地音频包括使用 RTC SDK 内部机制采集的麦克风音频，屏幕音频和本地混音音频信息。
  /// @param audioPropertiesInfos 本地音频信息，详见 LocalAudioPropertiesInfo{@link #LocalAudioPropertiesInfo} 。
  ///

  FutureOr<void> onLocalAudioPropertiesReport(
      Array<LocalAudioPropertiesInfo> audioPropertiesInfos) async {}

  /// @detail callback
  /// @author dixing
  /// @brief 回调音频设备测试时的播放音量
  /// @param volume 音频设备测试播放音量。取值范围：[0,255]
  /// @note 调用 startAudioPlaybackDeviceTest{@link #IRTCAudioDeviceManager#startAudioPlaybackDeviceTest} 或 startAudioDeviceRecordTest{@link #IRTCAudioDeviceManager#startAudioDeviceRecordTest}，开始播放音频文件或录音时，将开启该回调。本回调为周期性回调，回调周期由上述接口的 `interval` 参数指定。
  ///

  FutureOr<void> onAudioPlaybackDeviceTestVolume(int volume) async {}

  /// @detail callback
  /// @author gongzhengduo
  /// @brief 远端用户进房后，本地调用 enableAudioPropertiesReport{@link #RTCEngine#enableAudioPropertiesReport} ，根据设置的 interval 值，本地会周期性地收到此回调，了解订阅的远端用户的瞬时音频信息。 <br>
  ///        远端用户的音频包括使用 RTC SDK 内部机制/自定义机制采集的麦克风音频和屏幕音频。
  /// @param audioPropertiesInfos 远端音频信息，其中包含音频流属性、房间 ID、用户 ID ，详见 RemoteAudioPropertiesInfo{@link #RemoteAudioPropertiesInfo}。
  /// @param totalRemoteVolume 所有订阅的远端流混音后的音量，范围是 [0,255]。 <br>
  ///       - [0,25] 接近无声；
  ///       - [25,75] 为低音量；
  ///       - [76,204] 为中音量；
  ///       - [205,255] 为高音量。
  ///

  FutureOr<void> onRemoteAudioPropertiesReport(
      Array<RemoteAudioPropertiesInfo> audioPropertiesInfos,
      int totalRemoteVolume) async {}

  /// @detail callback
  /// @author gongzhengduo
  /// @brief 调用 enableAudioPropertiesReport{@link #RTCEngine#enableAudioPropertiesReport} 后，根据设置的 `AudioPropertiesConfig.interval`，你会周期性地收到此回调，获取房间内的最活跃用户信息。
  /// @param roomId 房间 ID
  /// @param uid 最活跃用户（ActiveSpeaker）的用户 ID
  ///

  FutureOr<void> onActiveSpeaker(String roomId, String uid) async {}

  /// @detail callback
  /// @region 网络管理
  /// @author qipengxiang
  /// @brief 关于音视频回路测试结果的回调。
  /// @param result 测试结果，参看 EchoTestResult{@link #EchoTestResult}
  /// @note 该回调触发的时机包括： <br>
  ///        - 检测过程中采集设备发生错误时；
  ///        - 检测成功后；
  ///        - 非设备原因导致检测过程中未接收到音/视频回放，停止检测后。
  ///

  FutureOr<void> onEchoTestResult(EchoTestResult result) async {}

  /// @detail callback
  /// @author daining.nemo
  /// @brief 调用 startCloudProxy{@link #RTCEngine#startCloudProxy} 开启云代理，SDK 首次成功连接云代理服务器时，回调此事件。
  /// @param interval 从开启云代理到连接成功经过的时间，单位为 ms
  ///

  FutureOr<void> onCloudProxyConnected(int interval) async {}

  /// @hidden for internal use only
  /// @detail callback
  /// @author songxiaomeng.19
  /// @brief 音频 dump 状态改变回调
  /// @param status 音频 dump 状态，参见 AudioDumpStatus{@link #AudioDumpStatus}
  /// @note 本回调用于内部排查音质相关异常问题，开发者无需关注。
  ///

  FutureOr<void> onAudioDumpStateChanged(AudioDumpStatus status) async {}

  /// @detail callback
  /// @author songxiaomeng.19
  /// @brief 首次调用 getNetworkTimeInfo{@link #RTCEngine#getNetworkTimeInfo} 后，SDK 内部启动网络时间同步，同步完成时会触发此回调。
  ///

  FutureOr<void> onNetworkTimeSynchronized() async {}

  /// @hidden internal use only
  /// @detail callback
  /// @author wangyu.1705
  /// @brief license 过期时间提醒
  /// @param days 即将过期剩余天数
  ///

  FutureOr<void> onLicenseWillExpire(int days) async {}

  /// @detail callback
  /// @author zhangcaining
  /// @brief 通话前回声检测结果回调。
  /// @param hardwareEchoDetectionResult 参见 HardwareEchoDetectionResult{@link #HardwareEchoDetectionResult}
  /// @note
  ///        - 通话前调用 startHardwareEchoDetection{@link #RTCEngine#startHardwareEchoDetection} 后，将触发本回调返回检测结果。
  ///        - 建议在收到检测结果后，调用 stopHardwareEchoDetection{@link #RTCEngine#stopHardwareEchoDetection} 停止检测，释放对音频设备的占用。
  ///        - 如果 SDK 在通话中检测到回声，将通过 onAudioDeviceWarning{@link #IRTCEngineEventHandler#onAudioDeviceWarning} 回调 `MEDIA_DEVICE_WARNING_DETECT_LEAK_ECHO`。
  ///

  FutureOr<void> onHardwareEchoDetectionResult(
      HardwareEchoDetectionResult hardwareEchoDetectionResult) async {}

  /// @detail callback
  /// @author keshixing.rtc
  /// @brief 本地代理状态发生改变回调。调用 setLocalProxy{@link #RTCEngine#setLocalProxy} 设置本地代理后，SDK 会触发此回调，通知代理连接的状态。
  /// @param localProxyType 本地代理类型。参看 LocalProxyType{@link #LocalProxyType} 。
  /// @param localProxyState 本地代理状态。参看 LocalProxyState{@link #LocalProxyState}。
  /// @param localProxyError 本地代理错误。参看 LocalProxyError{@link #LocalProxyError}。
  ///

  FutureOr<void> onLocalProxyStateChanged(LocalProxyType localProxyType,
      LocalProxyState localProxyState, LocalProxyError localProxyError) async {}

  /// @hidden internal use only
  /// @detail callback
  /// @author wangqianqian.1104
  /// @brief 当特效设置失败时，收到此回调。
  /// @param error 特效错误类型。参看 EffectErrorType{@link #EffectErrorType}。
  /// @param msg 错误信息。
  ///

  FutureOr<void> onEffectError(EffectErrorType error, String msg) async {}

  /// @detail callback
  /// @author wangjunzheng
  /// @brief 音频流同步信息回调。可以通过此回调，在远端用户调用 sendStreamSyncInfo{@link #RTCEngine#sendStreamSyncInfo} 发送音频流同步消息后，收到远端发送的音频流同步信息。
  /// @param streamId 远端流 ID
  /// @param streamInfo 远端流信息，详见 StreamInfo{@link #StreamInfo}
  /// @param streamType 媒体流类型，详见 SyncInfoStreamType{@link #SyncInfoStreamType}
  /// @param data 消息内容。
  ///

  FutureOr<void> onStreamSyncInfoReceived(
      String streamId,
      StreamInfo streamInfo,
      SyncInfoStreamType streamType,
      Uint8List data) async {}

  /// @hidden
  /// @detail callback
  /// @author zhoubohui
  /// @brief 外部采集时，调用 setOriginalScreenVideoInfo 设置屏幕或窗口大小改变前的分辨率后，若屏幕采集模式为智能模式，你将收到此回调，根据 RTC 智能决策合适的帧率和分辨率积（宽*高）重新采集。
  /// @param info RTC 智能决策后合适的帧率和分辨率积（宽*高）。参看 FrameUpdateInfo{@link #FrameUpdateInfo}。
  ///

  FutureOr<void> onExternalScreenFrameUpdate(FrameUpdateInfo info) async {}

  /// @detail callback
  /// @valid since 3.60.
  /// @brief 调用 takeRemoteSnapshotToFile{@link #RTCEngine#takeRemoteSnapshotToFile} 截取视频画面时，会收到此回调报告截图是否成功，以及截取的图片信息。
  /// @param streamId 被截图的视频流 ID。
  /// @param streamInfo 被截图的视频流信息，参看 StreamInfo{@link #StreamInfo}。
  /// @param filePath 截图文件的保存路径。
  /// @param width 截图图像的宽度。单位：像素。
  /// @param height 截图图像的高度。单位：像素。
  /// @param errorCode 截图错误码。参看 SnapshotErrorCode{@link #SnapshotErrorCode}。
  /// @param taskId 截图任务的编号。和 takeRemoteSnapshotToFile{@link #RTCEngine#takeRemoteSnapshotToFile} 的返回值一致。
  ///

  FutureOr<void> onRemoteSnapshotTakenToFile(
      String streamId,
      StreamInfo streamInfo,
      String filePath,
      int width,
      int height,
      SnapshotErrorCode errorCode,
      long taskId) async {}

  /// @detail callback
  /// @author wangjunzheng
  /// @brief 本地音频首帧发送状态发生改变时，收到此回调。
  /// @param streamId 流 ID
  /// @param streamInfo 流信息，参看 StreamInfo{@link #StreamInfo}
  /// @param user 用户信息，参看 RtcUser{@link #RtcUser}
  /// @param state 首帧发送状态，详见 FirstFrameSendState{@link #FirstFrameSendState}
  ///

  FutureOr<void> onAudioFrameSendStateChanged(String streamId,
      StreamInfo streamInfo, RtcUser user, FirstFrameSendState state) async {}

  /// @detail callback
  /// @author wangfujun
  /// @brief 视频首帧发送状态发生改变时，收到此回调。
  /// @param streamId 流 ID
  /// @param streamInfo 流信息，参看 StreamInfo{@link #StreamInfo}
  /// @param user 用户信息，参看 RtcUser{@link #RtcUser}
  /// @param state 首帧发送状态，详见 FirstFrameSendState{@link #FirstFrameSendState}
  ///

  FutureOr<void> onVideoFrameSendStateChanged(String streamId,
      StreamInfo streamInfo, RtcUser user, FirstFrameSendState state) async {}

  /// @detail callback
  /// @author wangjunzheng
  /// @brief 音频首帧播放状态发生改变时，收到此回调。
  /// @param streamId 流 ID
  /// @param streamInfo 流信息，参看 StreamInfo{@link #StreamInfo}
  /// @param user 用户信息，参看 RtcUser{@link #RtcUser}
  /// @param state 首帧播放状态，详见 FirstFramePlayState{@link #FirstFramePlayState}
  ///

  FutureOr<void> onAudioFramePlayStateChanged(String streamId,
      StreamInfo streamInfo, RtcUser user, FirstFramePlayState state) async {}

  /// @detail callback
  /// @author wangfujun
  /// @brief 远端视频流的首帧播放状态改变时，收到此回调。
  /// @param streamId 流 ID
  /// @param streamInfo 流信息，参看 StreamInfo{@link #StreamInfo}
  /// @param user 用户信息，参看 RtcUser{@link #RtcUser}
  /// @param state 首帧播放状态，详见 FirstFramePlayState{@link #FirstFramePlayState}
  ///

  FutureOr<void> onVideoFramePlayStateChanged(String streamId,
      StreamInfo streamInfo, RtcUser user, FirstFramePlayState state) async {}

  /// @detail callback
  /// @author wangfujun
  /// @region 音视频回退
  /// @brief 音视频流因网络环境变化等原因发生回退，或从回退中恢复时，触发该回调。
  /// @param streamId 流 ID
  /// @param streamInfo 流信息，参看 StreamInfo{@link #StreamInfo}
  /// @param event 音视频流发生变化的信息。参看 RemoteStreamSwitch{@link #RemoteStreamSwitch}。
  ///

  FutureOr<void> onSimulcastSubscribeFallback(
      String streamId, StreamInfo streamInfo, RemoteStreamSwitch event) async {}

  /// @detail callback
  /// @author panjian.fishing
  /// @brief 本地未通过 setPublishFallbackOption{@link #RTCEngine#setPublishFallbackOption} 开启发布性能回退，检测到设备性能不足时，收到此回调。 <br>
  ///        本地通过 setPublishFallbackOption{@link #RTCEngine#setPublishFallbackOption} 开启发布性能回退，因设备性能/网络原因，造成发布性能回退/恢复时，收到此回调。
  /// @param streamId 流 ID
  /// @param streamInfo 流信息，参看 StreamInfo{@link #StreamInfo}
  /// @param mode 指示本地是否开启发布回退功能。参看 PerformanceAlarmMode{@link #PerformanceAlarmMode} <br>
  ///             - 当发布端未开启发布性能回退时，mode 值为 NORMAL。
  ///             - 当发布端开启发布性能回退时，mode 值为 SIMULCAST。
  /// @param reason 告警原因，参看 PerformanceAlarmReason{@link #PerformanceAlarmReason}
  /// @param data 性能回退相关数据，详见 SourceWantedData{@link #SourceWantedData}。
  ///

  FutureOr<void> onPerformanceAlarms(
      String streamId,
      StreamInfo streamInfo,
      PerformanceAlarmMode mode,
      PerformanceAlarmReason reason,
      SourceWantedData data) async {}

  /// @hidden internal use only
  /// @detail callback
  /// @author lizheng
  ///

  FutureOr<void> onRemoteAudioPropertiesReportEx(
      Array<RemoteAudioPropertiesInfo> audioPropertiesInfos) async {}

  /// @valid since 3.60. 自 3.60 起，该回调替代了 `onStreamMixingEvent` 和 `onPushPublicStreamResult` 方法来实现下述功能。如果你已升级至 3.60 及以上版本，并且仍在使用这两个方法，请迁移到此回调。
  /// @detail callback
  /// @author lizheng
  /// @brief 合流转推 CDN / WTN 流状态回调
  /// @param info 任务详情，参看 MixedStreamTaskInfo{@link #MixedStreamTaskInfo}。
  /// @param event 任务事件，参看 MixedStreamTaskEvent{@link #MixedStreamTaskEvent}。
  /// @param error 任务错误码，参看 MixedStreamTaskErrorCode{@link #MixedStreamTaskErrorCode}
  ///

  FutureOr<void> onMixedStreamEvent(MixedStreamTaskInfo info,
      MixedStreamTaskEvent event, MixedStreamTaskErrorCode error) async {}

  /// @valid since 3.60.
  /// @detail callback
  /// @author lizheng
  /// @brief 单流转推 CDN 状态回调
  /// @param taskId 任务 ID
  /// @param event 任务状态, 参看 SingleStreamTaskEvent{@link #SingleStreamTaskEvent}
  /// @param error 错误码，参看 SingleStreamTaskErrorCode{@link #SingleStreamTaskErrorCode}
  ///

  FutureOr<void> onSingleStreamEvent(String taskId, SingleStreamTaskEvent event,
      SingleStreamTaskErrorCode error) async {}

  /// @hidden internal use only
  /// @valid since 3.60.
  /// @detail callback
  /// @author hegangjie
  /// @brief 试验性接口回调
  /// @param param 回调内容(JSON string)
  ///

  FutureOr<void> onExperimentalCallback(String param) async {}

  /// @deprecated since 3.60, use onMixedStreamEvent{@link #IRTCEngineEventHandler#onMixedStreamEvent} instead.
  /// @detail callback
  /// @author qipengxiang
  /// @brief WTN 流发布结果回调。 <br>
  ///        调用 startPushMixedStream{@link #RTCEngine#startPushMixedStream} 接口发布WTN 流后，启动结果通过此回调方法通知用户。
  /// @param roomId 发布WTN 流的房间 ID
  /// @param publicStreamId WTN 流 ID
  /// @param error WTN 流发布结果状态码。详见 PublicStreamErrorCode{@link #PublicStreamErrorCode}。
  ///

  FutureOr<void> onPushPublicStreamResult(String roomId, String publicStreamId,
      PublicStreamErrorCode error) async {}
}

class IAudioFrameProcessor extends NativeObserverClass {
  static const _$namespace = r'com.ss.bytertc.engine.IAudioFrameProcessor';
  static get codegen_$namespace => _$namespace;

  IAudioFrameProcessor([NativeClassOptions? options])
      : super(options == null
            ? const NativeClassOptions([],
                className: _$namespace,
                instanceType: InstanceType.manual,
                bridgeKey: 'com.volcengine.rtc.hybrid_runtime',
                methodMap: {
                  r"onProcessRecordAudioFrame": r"onProcessRecordAudioFrame",
                  r"onProcessPlayBackAudioFrame":
                      r"onProcessPlayBackAudioFrame",
                  r"onProcessRemoteUserAudioFrame":
                      r"onProcessRemoteUserAudioFrame",
                  r"onProcessEarMonitorAudioFrame":
                      r"onProcessEarMonitorAudioFrame",
                  r"onProcessScreenAudioFrame": r"onProcessScreenAudioFrame"
                })
            : NativeClassOptions.fromMap({
                ...options.toMap(),
                'bridgeKey': 'com.volcengine.rtc.hybrid_runtime',
              })) {
    registerEvent(r"onProcessRecordAudioFrame", onProcessRecordAudioFrame);

    registerEvent(r"onProcessPlayBackAudioFrame", onProcessPlayBackAudioFrame);

    registerEvent(
        r"onProcessRemoteUserAudioFrame", onProcessRemoteUserAudioFrame);

    registerEvent(
        r"onProcessEarMonitorAudioFrame", onProcessEarMonitorAudioFrame);

    registerEvent(r"onProcessScreenAudioFrame", onProcessScreenAudioFrame);
  }

  /// @detail callback
  /// @author majun.lvhiei
  /// @brief 回调本地采集的音频帧地址，供自定义音频处理。
  /// @param audioFrame 音频帧地址，参看 IAudioFrame{@link #IAudioFrame}。
  /// @return
  ///        - 0： 成功。
  ///        - < 0： 失败。
  /// @note
  ///        - 完成自定义音频处理后，SDK 会对处理后的音频帧进行编码，并传输到远端。此音频处理不会影响软件耳返音频数据。
  ///        - 要启用此回调，必须调用 enableAudioProcessor{@link #RTCEngine#enableAudioProcessor}，并在参数中选择本地采集的音频，每 10 ms 收到此回调。
  ///

  FutureOr<int> onProcessRecordAudioFrame(IAudioFrame audioFrame) async {
    throw UnimplementedError();
  }

  /// @detail callback
  /// @author majun.lvhiei
  /// @brief 回调远端音频混音的音频帧地址，供自定义音频处理。
  /// @param audioFrame 音频帧地址，参看 IAudioFrame{@link #IAudioFrame}
  /// @note 调用 enableAudioProcessor{@link #RTCEngine#enableAudioProcessor}，并在参数中选择远端音频流的的混音音频时，每 10 ms 收到此回调。
  ///

  FutureOr<int> onProcessPlayBackAudioFrame(IAudioFrame audioFrame) async {
    throw UnimplementedError();
  }

  /// @detail callback
  /// @author majun.lvhiei
  /// @brief 回调单个远端用户的音频帧地址，供自定义音频处理。
  /// @param streamId 远端流 ID。
  /// @param streamInfo 远端流信息, 参看 StreamInfo{@link #StreamInfo}。
  /// @param audioFrame 音频帧地址，参看 IAudioFrame{@link #IAudioFrame}。
  /// @note 调用 enableAudioProcessor{@link #RTCEngine#enableAudioProcessor}，并在参数中选择各个远端音频流时，每 10 ms 收到此回调。
  ///

  FutureOr<int> onProcessRemoteUserAudioFrame(
      String streamId, StreamInfo streamInfo, IAudioFrame audioFrame) async {
    throw UnimplementedError();
  }

  /// @valid since 3.50
  /// @detail callback
  /// @author songxiaomeng.19
  /// @brief 软件耳返音频数据的回调。你可根据此回调自定义处理音频。 <br>
  ///        软件耳返音频中包含通过调用 `setVoiceReverbType` 和 `setVoiceChangerType` 设置的音频特效。
  /// @param audioFrame 音频帧地址。参看 IAudioFrame{@link #IAudioFrame}。
  /// @return
  ///        - 0： 成功。
  ///        - < 0： 失败。
  /// @note
  ///        - 此数据处理只影响软件耳返音频数据。
  ///        - 要启用此回调，必须调用 enableAudioProcessor{@link #RTCEngine#enableAudioProcessor}，并选择软件耳返音频，每 10 ms 收到此回调。
  ///

  FutureOr<int> onProcessEarMonitorAudioFrame(IAudioFrame audioFrame) async {
    throw UnimplementedError();
  }

  /// @detail callback
  /// @author zhangcaining
  /// @brief 屏幕共享的音频帧地址回调。你可根据此回调自定义处理音频。
  /// @param audioFrame 音频帧地址，参看 IAudioFrame{@link #IAudioFrame}。
  /// @note 调用 enableAudioProcessor{@link #RTCEngine#enableAudioProcessor}，把返回给音频处理器的音频类型设置为屏幕共享音频后，每 10 ms 收到此回调。
  ///

  FutureOr<int> onProcessScreenAudioFrame(IAudioFrame audioFrame) async {
    throw UnimplementedError();
  }
}

class IClientMixedStreamObserver extends NativeObserverClass {
  static const _$namespace =
      r'com.ss.bytertc.engine.live.IClientMixedStreamObserver';
  static get codegen_$namespace => _$namespace;

  IClientMixedStreamObserver([NativeClassOptions? options])
      : super(options == null
            ? const NativeClassOptions([],
                className: _$namespace,
                instanceType: InstanceType.manual,
                bridgeKey: 'com.volcengine.rtc.hybrid_runtime',
                methodMap: {
                  r"onClientMixedStreamEvent": r"onClientMixedStreamEvent",
                  r"onMixedAudioFrame": r"onMixedAudioFrame",
                  r"onMixedVideoFrame": r"onMixedVideoFrame",
                  r"onMixedDataFrame": r"onMixedDataFrame",
                  r"onMixedFirstAudioFrame": r"onMixedFirstAudioFrame",
                  r"onMixedFirstVideoFrame": r"onMixedFirstVideoFrame"
                })
            : NativeClassOptions.fromMap({
                ...options.toMap(),
                'bridgeKey': 'com.volcengine.rtc.hybrid_runtime',
              })) {
    registerEvent(r"onClientMixedStreamEvent", onClientMixedStreamEvent);

    registerEvent(r"onMixedAudioFrame", onMixedAudioFrame);

    registerEvent(r"onMixedVideoFrame", onMixedVideoFrame);

    registerEvent(r"onMixedDataFrame", onMixedDataFrame);

    registerEvent(r"onMixedFirstAudioFrame", onMixedFirstAudioFrame);

    registerEvent(r"onMixedFirstVideoFrame", onMixedFirstVideoFrame);
  }

  /// @hidden for internal use only
  /// @detail callback
  /// @author liujingchao
  /// @brief 转推直播状态回调
  /// @param eventType 转推直播任务状态, 参看 ByteRTCStreamMixingEvent{@link #ByteRTCStreamMixingEvent}
  /// @param taskId 转推直播任务 ID
  /// @param error 转推直播错误码，参看 MixedStreamTaskErrorCode{@link #MixedStreamTaskErrorCode}
  /// @param mixType 转推直播类型，参看 MixedStreamType{@link #MixedStreamType}
  ///

  FutureOr<void> onClientMixedStreamEvent(
      MixedStreamTaskInfo info,
      MixedStreamType type,
      MixedStreamTaskEvent event,
      MixedStreamTaskErrorCode error) async {}

  /// @hidden for internal use only
  /// @detail callback
  /// @author liujingchao
  /// @brief 端云一体合流音频 PCM 回调
  /// @param taskId 转推直播任务 ID
  /// @param audioFrame PCM 编码的合流音频数据帧
  /// @param frameNum PCM 编码的音频数据帧数
  /// @param timeStampMs 时间戳，单位毫秒
  /// @note 收到该回调的周期为每 10 毫秒一次，并且每次的音频数据量为 10 毫秒数据量。
  ///

  FutureOr<void> onMixedAudioFrame(String taskId, ArrayBuffer audioFrame,
      int frameNum, long timeStampMs) async {}

  /// @hidden for internal use only
  /// @detail callback
  /// @author liujingchao
  /// @brief 端云一体合流视频 YUV 回调
  /// @param taskId 转推直播任务 ID
  /// @param videoFrame YUV 合流视频数据帧，参看 IVideoFrame{@link #IVideoFrame}
  /// @note
  ///        - 收到该回调的周期与视频的帧间隔一致。
  ///

  FutureOr<void> onMixedVideoFrame(
      String taskId, IVideoFrame videoFrame) async {}

  /// @hidden for internal use only
  /// @detail callback
  /// @author liujingchao
  /// @brief 端云一体合流视频 SEI 数据回调
  /// @param taskId 转推直播任务 ID
  /// @param dataFrame SEI 数据
  /// @param time 时间信息
  ///

  FutureOr<void> onMixedDataFrame(
      String taskId, ArrayBuffer dataFrame, long time) async {}

  /// @hidden for internal use only
  ///

  FutureOr<void> onMixedFirstAudioFrame(String taskId) async {}

  /// @hidden for internal use only
  ///

  FutureOr<void> onMixedFirstVideoFrame(String taskId) async {}
}
