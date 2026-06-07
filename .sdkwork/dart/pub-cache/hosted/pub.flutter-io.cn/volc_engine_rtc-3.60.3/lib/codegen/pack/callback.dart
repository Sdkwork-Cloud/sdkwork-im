/// Copyright (c) 2026 Beijing Volcano Engine Technology Ltd.

// ignore_for_file: camel_case_types, annotate_overrides, null_check_always_fails, unused_import, non_constant_identifier_names
import 'package:hybrid_runtime/hybrid_runtime.dart';
import 'dart:io';
import 'dart:async';
import 'dart:typed_data';
import '../android/index.dart' as $p_a;
import '../ios/index.dart' as $p_i;
import 'keytype.dart';
import 'errorcode.dart';
import 'api.dart';
import 'types.dart';

/// @detail callback
/// @author zhangyuanyuan.0101
/// @brief 本地音频文件混音的音频帧观察者。

class IMediaPlayerAudioFrameObserver {
  IMediaPlayerAudioFrameObserver({this.onFrame});

  /// @detail callback
  /// @brief 当本地音频文件混音时，回调播放的音频帧。
  /// @param playerId 播放器 ID
  /// @param frame 参看 IAudioFrame{@link #IAudioFrame}。
  ///
  FutureOr<void> Function(int playerId)? onFrame;
}

/// @detail callback
/// @brief 音频数据回调观察者 <br>
/// 注意：回调函数是在 SDK 内部线程（非 UI 线程）同步抛出来的，请不要做耗时操作或直接操作 UI，否则可能导致 app 崩溃。 <br>
/// 本接口类中的回调周期均为 20 ms。

class IAudioFrameObserver {
  IAudioFrameObserver(
      {this.onRecordAudioFrame,
      this.onPlaybackAudioFrame,
      this.onRemoteUserAudioFrame,
      this.onMixedAudioFrame,
      this.onCaptureMixedAudioFrame});

  /// @detail callback
  /// @author wangjunzheng
  /// @brief 返回麦克风录制的音频数据
  /// @param audioFrame 音频数据, 参看 IAudioFrame{@link #IAudioFrame}。
  ///
  FutureOr<void> Function(AudioFrame audioFrame)? onRecordAudioFrame;

  /// @detail callback
  /// @author wangjunzheng
  /// @brief 返回订阅的所有远端用户混音后的音频数据。
  /// @param audioFrame 音频数据, 参看 IAudioFrame{@link #IAudioFrame}。
  ///
  FutureOr<void> Function(AudioFrame audioFrame)? onPlaybackAudioFrame;

  /// @detail callback
  /// @author wangjunzheng
  /// @brief 返回远端单个用户的音频数据
  /// @param streamId 远端流 ID。
  /// @param streamInfo 远端流信息，参看 StreamInfo{@link #StreamInfo}。
  /// @param audioFrame 音频数据，参看 IAudioFrame{@link #IAudioFrame}。
  /// @note 此回调在播放线程调用。不要在此回调中做任何耗时的事情，否则可能会影响整个音频播放链路。
  ///
  FutureOr<void> Function(String streamId, AudioFrame audioFrame)?
      onRemoteUserAudioFrame;

  /// @detail callback
  /// @author wangjunzheng
  /// @brief 返回本地麦克风录制和订阅的所有远端用户混音后的音频数据
  /// @param audioFrame 音频数据, 参看 IAudioFrame{@link #IAudioFrame}。
  ///
  FutureOr<void> Function(AudioFrame audioFrame)? onMixedAudioFrame;

  /// @detail callback
  /// @author huanghao
  /// @brief 返回本地麦克风录制的音频数据，本地 `MediaPlayer` / `EffectPlayer` 播放音频文件混音后的音频数据。
  /// @param audioFrame 音频数据, 参看 IAudioFrame{@link #IAudioFrame}。
  ///
  FutureOr<void> Function(AudioFrame audioFrame)? onCaptureMixedAudioFrame;
}

/// @detail callback
/// @brief 房间事件回调接口 <br>
/// 注意：回调函数是在 SDK 内部线程（非 UI 线程）同步抛出来的，请不要做耗时操作或直接操作 UI，否则可能导致 app 崩溃。

class IRTCRoomEventHandler {
  IRTCRoomEventHandler(
      {this.onLeaveRoom,
      this.onRoomStateChangedWithReason,
      this.onRoomStateChanged,
      this.onStreamStateChanged,
      this.onAVSyncStateChange,
      this.onRoomStats,
      this.onRoomEvent,
      this.onUserJoined,
      this.onUserLeave,
      this.onVideoPublishStateChanged,
      this.onAudioPublishStateChanged,
      this.onVideoSubscribeStateChanged,
      this.onAudioSubscribeStateChanged,
      this.onLocalStreamStats,
      this.onRemoteStreamStats,
      this.onStreamPublishSuccess,
      this.onAVSyncEvent,
      this.onUserPublishStreamVideo,
      this.onUserPublishStreamAudio,
      this.onRoomMessageReceived,
      this.onRoomBinaryMessageReceived,
      this.onUserMessageReceived,
      this.onUserBinaryMessageReceived,
      this.onUserMessageSendResult,
      this.onRoomMessageSendResult,
      this.onVideoStreamBanned,
      this.onAudioStreamBanned,
      this.onForwardStreamStateChanged,
      this.onForwardStreamEvent,
      this.onNetworkQuality,
      this.onSetRoomExtraInfoResult,
      this.onRoomExtraInfoUpdate,
      this.onRoomStreamExtraInfoUpdate,
      this.onUserVisibilityChanged,
      this.onSubtitleStateChanged,
      this.onSubtitleMessageReceived,
      this.onRoomWarning,
      this.onTokenWillExpire,
      this.onPublishPrivilegeTokenWillExpire,
      this.onSubscribePrivilegeTokenWillExpire,
      this.onStreamSubscribed});

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
  FutureOr<void> Function(RTCRoomStats stats)? onLeaveRoom;

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
  FutureOr<void> Function(String roomId, String uid, RoomState state,
      RoomStateChangeReason reason)? onRoomStateChangedWithReason;

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
  FutureOr<void> Function(
          String roomId, String uid, int state, String extraInfo)?
      onRoomStateChanged;

  /// @detail callback
  /// @author shenpengliang
  /// @brief 流状态改变回调，发生流相关的警告或错误时会收到此回调。
  /// @param roomId 房间 ID。
  /// @param uid 用户 ID。
  /// @param state 流状态码，参看 ErrorCode{@link #ErrorCode} 及 WarningCode{@link #WarningCode}。
  /// @param extraInfo 附加信息，目前为空。
  ///
  FutureOr<void> Function(
          String roomId, String uid, int state, String extraInfo)?
      onStreamStateChanged;

  /// @detail callback
  /// @author wangzhanqiang
  /// @brief 发布端调用 setMultiDeviceAVSync{@link #RTCRoom#setMultiDeviceAVSync} 后音视频同步状态发生改变时，会收到此回调。
  /// @param state 音视频同步状态，参看 AVSyncState{@link #AVSyncState}。
  ///
  FutureOr<void> Function(AVSyncState state)? onAVSyncStateChange;

  /// @detail callback
  /// @author yejing
  /// @brief 房间内通话统计信息回调。 <br>
  ///        用户进房开始通话后，每 2s 收到一次本回调。
  /// @param stats 房间内的汇总统计数据。详见 RTCRoomStats{@link #RTCRoomStats}。
  ///
  FutureOr<void> Function(RTCRoomStats stats)? onRoomStats;

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
  FutureOr<void> Function(
          String roomId, String uid, RoomEvent state, RoomEventInfo info)?
      onRoomEvent;

  /// @detail callback
  /// @author shenpengliang
  /// @brief 远端可见用户加入房间，或房内不可见用户切换为可见的回调。 <br>
  ///        1.远端用户调用 setUserVisibility{@link #RTCRoom#setUserVisibility} 方法将自身设为可见后加入房间时，房间内其他用户将收到该事件。 <br>
  ///        2.远端可见用户断网后重新连入房间时，房间内其他用户将收到该事件。 <br>
  ///        3.房间内隐身远端用户调用 setUserVisibility{@link #RTCRoom#setUserVisibility} 方法切换至可见时，房间内其他用户将收到该事件。 <br>
  ///        4.新进房用户也会收到进房前已在房内的可见用户的进房回调通知。
  /// @param userInfo 用户信息。参看 UserInfo{@link #UserInfo}。
  ///
  FutureOr<void> Function(UserInfo userInfo)? onUserJoined;

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
  FutureOr<void> Function(String uid, int reason)? onUserLeave;

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
  FutureOr<void> Function(
          String streamId, PublishState state, PublishStateChangeReason reason)?
      onVideoPublishStateChanged;

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
  FutureOr<void> Function(
          String streamId, PublishState state, PublishStateChangeReason reason)?
      onAudioPublishStateChanged;

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
  FutureOr<void> Function(String streamId, SubscribeState state,
      SubscribeStateChangeReason reason)? onVideoSubscribeStateChanged;

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
  FutureOr<void> Function(String streamId, SubscribeState state,
      SubscribeStateChangeReason reason)? onAudioSubscribeStateChanged;

  /// @detail callback
  /// @author yejing
  /// @brief 本地流数据统计以及网络质量回调。 <br>
  ///        本地用户发布流成功后，SDK 会周期性（2s）的通过此回调事件通知用户发布的流在此次统计周期内的质量统计信息。 <br>
  ///        统计信息通过 LocalStreamStats{@link #LocalStreamStats} 类型的回调参数传递给用户，其中包括发送音视频比特率、发送帧率、编码帧率，网络质量等。
  /// @param streamId 流 ID，用于标识特定的本地流。
  /// @param streamInfo 流信息结构体，包含房间 ID、用户 ID 等详细信息，参看 StreamInfo{@link #StreamInfo}。
  /// @param stats 音视频流以及网络状况统计信息。参见 LocalStreamStats{@link #LocalStreamStats}。
  ///
  FutureOr<void> Function(
          String streamId, StreamInfo streamInfo, LocalStreamStats stats)?
      onLocalStreamStats;

  /// @detail callback
  /// @author yejing
  /// @brief 本地订阅的远端音/视频流数据统计以及网络质量回调。 <br>
  ///        本地用户订阅流成功后，SDK 会周期性（2s）的通过此回调事件通知用户订阅的流在此次统计周期内的质量统计信息，包括：发送音视频比特率、发送帧率、编码帧率，网络质量等。
  /// @param streamId 流 ID，用于标识特定的远端流。
  /// @param streamInfo 流信息结构体，包含房间 ID、用户 ID 等详细信息，参看 StreamInfo{@link #StreamInfo}。
  /// @param stats 音视频流以及网络状况统计信息。参见 RemoteStreamStats{@link #RemoteStreamStats}。
  ///
  FutureOr<void> Function(
          String streamId, StreamInfo streamInfo, RemoteStreamStats stats)?
      onRemoteStreamStats;

  /// @hidden for internal use only
  /// @detail callback
  /// @author shenpengliang
  /// @brief 当发布流成功的时候回调该事
  /// @param uid 流发布用户的用户 ID
  /// @param isScreen 流的标识
  ///
  FutureOr<void> Function(bool isScreen)? onStreamPublishSuccess;

  /// @detail callback
  /// @valid since 3.60.
  /// @author xuyiling.x10
  /// @brief 发布端调用 setMultiDeviceAVSync{@link #RTCRoom#setMultiDeviceAVSync} 后音视频同步状态发生错误时，会收到此回调。
  /// @param roomId 房间 ID。
  /// @param uid 用户 ID。
  /// @param eventCode 音视频同步状态错误，参看 AVSyncEvent{@link #AVSyncEvent}。
  ///
  FutureOr<void> Function(String roomId, String userId, AVSyncEvent eventCode)?
      onAVSyncEvent;

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
  FutureOr<void> Function(
          String streamId, StreamInfo streamInfo, bool isPublish)?
      onUserPublishStreamVideo;

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
  FutureOr<void> Function(
          String streamId, StreamInfo streamInfo, bool isPublish)?
      onUserPublishStreamAudio;

  /// @detail callback
  /// @author hanchenchen.c
  /// @brief 接收到房间内广播消息的回调。 <br>
  ///        房间内其他用户调用 sendRoomMessage{@link #RTSRoom#sendRoomMessage} 发送广播消息时，收到此回调。
  /// @param msgid 消息编号。
  /// @param uid 消息发送者 ID。
  /// @param message 收到的消息内容。
  ///
  FutureOr<void> Function(int msgid, String uid, String message)?
      onRoomMessageReceived;

  /// @detail callback
  /// @author hanchenchen.c
  /// @brief 收到房间内广播二进制消息的回调。 <br>
  ///        房间内其他用户调用 sendRoomBinaryMessage{@link #RTSRoom#sendRoomBinaryMessage} 发送广播二进制消息时，收到此回调。
  /// @param msgid 消息编号。
  /// @param uid 消息发送者 ID。
  /// @param message 收到的二进制消息内容。
  ///
  FutureOr<void> Function(int msgid, String uid, Uint8List message)?
      onRoomBinaryMessageReceived;

  /// @detail callback
  /// @author hanchenchen.c
  /// @brief 收到来自房间中其他用户通过 sendUserMessage{@link #RTSRoom#sendUserMessage} 发来的点对点文本消息时，会收到此回调。
  /// @param msgid 消息编号。
  /// @param uid 消息发送者的用户 ID。
  /// @param message 收到的文本消息内容。
  ///
  FutureOr<void> Function(int msgid, String uid, String message)?
      onUserMessageReceived;

  /// @detail callback
  /// @author hanchenchen.c
  /// @brief 收到来自房间中其他用户通过 sendUserBinaryMessage{@link #RTSRoom#sendUserBinaryMessage} 发来的点对点二进制消息时，会收到此回调。
  /// @param msgid 消息编号。
  /// @param uid 消息发送者的用户 ID。
  /// @param message 收到的二进制消息内容。
  ///
  FutureOr<void> Function(int msgid, String uid, Uint8List message)?
      onUserBinaryMessageReceived;

  /// @detail callback
  /// @author hanchenchen.c
  /// @brief 向房间内单个用户发送文本或二进制消息后（P2P），消息发送方会收到该消息发送结果回调。
  /// @param msgid 本条消息的 ID。
  /// @param error 文本或二进制消息发送结果，详见 UserMessageSendResult{@link #UserMessageSendResult}
  /// @note 调用 sendUserMessage{@link #RTSRoom#sendUserMessage} 或 sendUserBinaryMessage{@link #RTSRoom#sendUserBinaryMessage} 接口，才能收到此回调。
  ///
  FutureOr<void> Function(int msgid, UserMessageSendResult error)?
      onUserMessageSendResult;

  /// @detail callback
  /// @author hanchenchen.c
  /// @brief 调用 sendRoomMessage{@link #RTSRoom#sendRoomMessage} 或 sendRoomBinaryMessage{@link #RTSRoom#sendRoomBinaryMessage} 向房间内群发文本或二进制消息后，消息发送方会收到该消息发送结果回调。
  /// @param msgid 本条消息的 ID。
  /// @param error 消息发送结果，详见 RoomMessageSendResult{@link #RoomMessageSendResult}
  ///
  FutureOr<void> Function(int msgid, RoomMessageSendResult error)?
      onRoomMessageSendResult;

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
  FutureOr<void> Function(String uid, bool banned)? onVideoStreamBanned;

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
  FutureOr<void> Function(String uid, bool banned)? onAudioStreamBanned;

  /// @detail callback
  /// @author shenpengliang
  /// @brief 跨房间媒体流转发状态和错误回调
  /// @param stateInfos 跨房间媒体流转发目标房间信息数组，详见 ForwardStreamStateInfo{@link #ForwardStreamStateInfo}
  ///
  FutureOr<void> Function(List<ForwardStreamStateInfo> infos)?
      onForwardStreamStateChanged;

  /// @detail callback
  /// @author shenpengliang
  /// @brief 跨房间媒体流转发事件回调
  /// @param eventInfos 跨房间媒体流转发目标房间事件数组，详见 ForwardStreamEventInfo{@link #ForwardStreamEventInfo}
  ///
  FutureOr<void> Function(List<ForwardStreamEventInfo> eventInfos)?
      onForwardStreamEvent;

  /// @detail callback
  /// @author chengchao.cc951119
  /// @brief 加入房间并发布或订阅流后， 以每 2 秒一次的频率，报告本地用户和已订阅的远端用户的上下行网络质量信息。
  /// @param localQuality 本地网络质量，详见 NetworkQualityStats{@link #NetworkQualityStats}。
  /// @param remoteQualities 已订阅用户的网络质量，详见 NetworkQualityStats{@link #NetworkQualityStats}。
  /// @note 更多通话中的监测接口，详见[通话中质量监测](https://www.volcengine.com/docs/6348/106866)。
  ///
  FutureOr<void> Function(NetworkQualityStats localQuality,
      List<NetworkQualityStats> remoteQualities)? onNetworkQuality;

  /// @valid since 3.52
  /// @detail callback
  /// @author lichangfeng.rtc
  /// @brief 调用 setRoomExtraInfo{@link #RTCRoom#setRoomExtraInfo} 设置房间附加信息结果的回调。
  /// @param taskId 调用 setRoomExtraInfo 的任务编号。
  /// @param result 设置房间附加信息的结果，详见 SetRoomExtraInfoResult{@link #SetRoomExtraInfoResult}
  ///
  FutureOr<void> Function(int taskId, SetRoomExtraInfoResult result)?
      onSetRoomExtraInfoResult;

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
  FutureOr<void> Function(String key, String value, String lastUpdateUserId,
      int lastUpdateTimeMs)? onRoomExtraInfoUpdate;

  /// @valid since 3.54
  /// @detail callback
  /// @brief 接收同一房间内，其他用户调用 setStreamExtraInfo{@link #RTCRoom#setStreamExtraInfo} 设置的流附加信息的回调。
  /// @param streamId 流附加信息的流 ID
  /// @param streamInfo 流附加信息的流信息
  /// @param extraInfo 流附加信息
  /// @note 新进房的用户会收到进房前房间内已有的全部附加信息通知。
  ///
  FutureOr<void> Function(
          String streamId, StreamInfo streamInfo, String extraInfo)?
      onRoomStreamExtraInfoUpdate;

  /// @valid since 3.54
  /// @detail callback
  /// @author caocun
  /// @brief 用户调用 setUserVisibility{@link #RTCRoom#setUserVisibility} 设置用户可见性的回调。
  /// @param currentUserVisibility 当前用户的可见性。 <br>
  ///        - true: 可见，用户可以在房间内发布音视频流，房间中的其他用户将收到用户的行为通知，例如进房、开启视频采集和退房。
  ///        - false: 不可见，用户不可以在房间内发布音视频流，房间中的其他用户不会收到用户的行为通知，例如进房、开启视频采集和退房。
  /// @param errorCode 设置用户可见性错误码，参看 UserVisibilityChangeError{@link #UserVisibilityChangeError}。
  ///
  FutureOr<void> Function(
          bool currentUserVisibility, UserVisibilityChangeError errorCode)?
      onUserVisibilityChanged;

  /// @detail callback
  /// @author qiaoxingwang
  /// @brief 字幕状态发生改变回调。 <br>
  ///         当用户调用 startSubtitle{@link #RTCRoom#startSubtitle} 和 stopSubtitle{@link #RTCRoom#stopSubtitle} 使字幕状态发生改变或字幕任务出现错误时，触发该回调。
  /// @param state 字幕状态。参看 SubtitleState{@link #SubtitleState}。
  /// @param errorCode 字幕任务错误码。参看 SubtitleErrorCode{@link #SubtitleErrorCode}。
  /// @param errorMessage 与第三方服务有关的错误信息。
  ///
  FutureOr<void> Function(SubtitleState state, SubtitleErrorCode errorCode,
      String errorMessage)? onSubtitleStateChanged;

  /// @detail callback
  /// @author qiaoxingwang
  /// @brief 字幕相关内容回调。 <br>
  ///         当用户成功调用 startSubtitle{@link #RTCRoom#startSubtitle} 后会收到此回调，通知字幕的相关信息。
  /// @param subtitles 字幕消息内容。参看 SubtitleMessage{@link #SubtitleMessage}。
  ///
  FutureOr<void> Function(List<SubtitleMessage> subtitles)?
      onSubtitleMessageReceived;

  /// @hidden
  /// @deprecated since 3.41 and will be deleted in 3.51, use onRoomStateChanged{@link #IRTCRoomEventHandler#onRoomStateChanged} and onStreamStateChanged instead.
  /// @detail callback
  /// @author shenpengliang
  /// @brief 发生警告回调。
  /// @param warn 警告代码，参见 WarningCode{@link #WarningCode}
  /// @note SDK 运行时出现了（网络或媒体相关的）警告。SDK 通常会自动恢复，警告信息可以忽略。
  ///
  FutureOr<void> Function(RTCRoom rtcRoom, WarningCode? warningCode)?
      onRoomWarning;

  /// @detail callback
  /// @author shenpengliang
  /// @brief 当 SDK 检测到 Token 的进房权限将在 30 秒内过期时，触发该回调。
  ///        收到该回调后，你需调用 updateToken{@link #RTSRoom#updateToken} 更新 Token 进房权限。
  /// @note 若 Token 进房权限过期且未及时更新： <br>
  ///        - 用户此时尝试进房会收到 onRoomStateChanged{@link #IRTCRoomEventHandler#onRoomStateChanged} 回调，提示错误码为 `-1000` Token 无效；
  ///        - 用户已在房间内则会被移出房间，本地用户会收到 onRoomStateChanged{@link #IRTCRoomEventHandler#onRoomStateChanged} 回调，提示错误码为 `-1009` Token 无效，同时远端用户会收到 onUserLeave{@link #IRTCRoomEventHandler#onUserLeave} 回调，提示原因为 `1` Token 进房权限过期。
  ///
  FutureOr<void> Function()? onTokenWillExpire;

  /// @detail callback
  /// @author qipengxiang
  /// @brief Token 发布权限过期前 30 秒将触发该回调。 <br>
  ///        收到该回调后，你需调用 updateToken{@link #RTSRoom#updateToken} 更新 Token 发布权限。
  /// @note  Token 发布权限过期后：
  ///        - 已发布流或尝试发布流时，本端会收到 onVideoPublishStateChanged{@link #IRTCRoomEventHandler#onVideoPublishStateChanged}、onAudioPublishStateChanged{@link #IRTCRoomEventHandler#onAudioPublishStateChanged} 回调，提示`kPublishStateChangeReasonNoPublishPermission`，没有发布权限。
  ///        - 发布中的流将停止发布。远端用户会收到 onUserPublishStreamVideo{@link #IRTCRoomEventHandler#onUserPublishStreamVideo}、onUserPublishStreamAudio{@link #IRTCRoomEventHandler#onUserPublishStreamAudio} 回调，提示该流已停止发布。
  ///
  FutureOr<void> Function()? onPublishPrivilegeTokenWillExpire;

  /// @detail callback
  /// @author qipengxiang
  /// @brief Token 订阅权限过期前 30 秒将触发该回调。 <br>
  ///        收到该回调后，你需调用 updateToken{@link #RTSRoom#updateToken} 更新 Token 订阅权限有效期。
  /// @note 若收到该回调后未及时更新 Token，Token 订阅权限过期后，尝试新订阅流会失败，已订阅的流会取消订阅，可通过 onVideoSubscribeStateChanged{@link #IRTCRoomEventHandler#onVideoSubscribeStateChanged}、onAudioSubscribeStateChanged{@link #IRTCRoomEventHandler#onAudioSubscribeStateChanged} 回调，提示错误码为 `-1003` 没有订阅权限。
  ///
  FutureOr<void> Function()? onSubscribePrivilegeTokenWillExpire;

  /// @platform android
  /// @detail callback
  /// @author shenpengliang
  /// @brief 关于订阅媒体流状态改变的回调
  /// @param stateCode 订阅媒体流状态，参看 SubscribeState{@link #SubscribeState}
  /// @param userId 流发布用户的用户 ID
  /// @param info 流的属性，参看 SubscribeConfig{@link #SubscribeConfig}
  /// @note 本地用户收到该回调的时机：调用 subscribeStreamVideo{@link #RTCRoom#subscribeStreamVideo} 订阅/取消订阅指定远端摄像头音视频流后。
  ///
  FutureOr<void> Function(
          SubscribeState stateCode, String userId, SubscribeConfig info)?
      onStreamSubscribed;
}

/// @detail callback
/// @brief IRTSRoomEventHandler Class<br>
/// 注意：回调函数是在 SDK 内部线程（非 UI 线程）同步抛出来的，请不要做耗时操作或直接操作 UI，否则可能导致 app 崩溃。

class IRTSRoomEventHandler {
  IRTSRoomEventHandler(
      {this.onLeaveRoom,
      this.onRoomStateChanged,
      this.onUserJoined,
      this.onUserLeave,
      this.onRoomMessageReceived,
      this.onRoomBinaryMessageReceived,
      this.onUserMessageReceived,
      this.onUserBinaryMessageReceived,
      this.onUserMessageSendResult,
      this.onRoomMessageSendResult});

  /// @platform android
  /// @detail callback
  /// @brief 离开房间成功回调。 <br>
  ///        用户调用 leaveRoom{@link #RTSRoom#leaveRoom} 方法后，SDK 会停止所有的发布订阅流，并在释放所有通话相关的音视频资源后，通过此回调通知用户离开房间成功。
  /// @param stats 保留参数，目前为空。
  /// @note
  ///       - 用户调用 leaveRoom{@link #RTSRoom#leaveRoom} 方法离开房间后，如果立即调用 destroy{@link #RTSRoom#destroy} 销毁房间实例或 destroyRTCEngine{@link #RTCEngine#destroyRTCEngine} 方法销毁 RTC 引擎，则将无法收到此回调事件。
  ///       - 离开房间后，如果 App 需要使用系统音视频设备，则建议在收到此回调后再初始化音视频设备，否则可能由于 SDK 占用音视频设备导致初始化失败。
  ///
  FutureOr<void> Function(RTCRoomStats stats)? onLeaveRoom;

  /// @platform android
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
  FutureOr<void> Function(
          String roomId, String uid, int state, String extraInfo)?
      onRoomStateChanged;

  /// @platform android
  /// @detail callback
  /// @brief 远端可见用户加入房间，或房内不可见用户切换为可见的回调。 <br>
  ///        1.远端用户调用 setUserVisibility{@link #RTCRoom#setUserVisibility} 方法将自身设为可见后加入房间时，房间内其他用户将收到该事件。 <br>
  ///        2.远端可见用户断网后重新连入房间时，房间内其他用户将收到该事件。 <br>
  ///        3.房间内隐身远端用户调用 setUserVisibility{@link #RTCRoom#setUserVisibility} 方法切换至可见时，房间内其他用户将收到该事件。 <br>
  ///        4.新进房用户也会收到进房前已在房内的可见用户的进房回调通知。
  /// @param userInfo 用户信息。参看 UserInfo{@link #UserInfo}。
  ///
  FutureOr<void> Function(UserInfo userInfo)? onUserJoined;

  /// @platform android
  /// @detail callback
  /// @brief 远端用户离开房间，或切至不可见时，房间内其他用户会收到此事件
  /// @param uid 离开房间，或切至不可见的的远端用户 ID。
  /// @param reason 用户离开房间的原因： <br>
  ///              - 0: 远端用户调用 leaveRoom{@link #RTSRoom#leaveRoom} 主动退出房间。
  ///              - 1: 远端用户因 Token 过期或网络原因等掉线。详细信息请参看[连接状态提示](https://www.volcengine.com/docs/6348/95376)
  ///              - 2: 远端用户调用 setUserVisibility{@link #RTCRoom#setUserVisibility} 切换至不可见状态。
  ///              - 3: 服务端调用 OpenAPI 将该远端用户踢出房间。
  ///
  FutureOr<void> Function(String uid, int reason)? onUserLeave;

  /// @platform android
  /// @detail callback
  /// @author hanchenchen.c
  /// @brief 接收到房间内广播消息的回调。 <br>
  ///        房间内其他用户调用 sendRoomMessage{@link #RTSRoom#sendRoomMessage} 发送广播消息时，收到此回调。
  /// @param msgid 消息编号
  /// @param uid 消息发送者 ID
  /// @param message 收到的消息内容
  ///
  FutureOr<void> Function(long msgid, String uid, String message)?
      onRoomMessageReceived;

  /// @platform android
  /// @detail callback
  /// @author hanchenchen.c
  /// @brief 收到房间内广播二进制消息的回调。 <br>
  ///        房间内其他用户调用 sendRoomBinaryMessage{@link #RTSRoom#sendRoomBinaryMessage} 发送广播二进制消息时，收到此回调。
  /// @param msgid 消息编号
  /// @param uid 消息发送者 ID
  /// @param message 收到的二进制消息内容
  ///
  FutureOr<void> Function(long msgid, String uid, ByteBuffer message)?
      onRoomBinaryMessageReceived;

  /// @platform android
  /// @detail callback
  /// @author hanchenchen.c
  /// @brief 收到来自房间中其他用户通过 sendUserMessage{@link #RTSRoom#sendUserMessage} 发来的点对点文本消息时，会收到此回调。
  /// @param msgid 消息编号
  /// @param uid 消息发送者的用户 ID。
  /// @param message 收到的文本消息内容。
  ///
  FutureOr<void> Function(long msgid, String uid, String message)?
      onUserMessageReceived;

  /// @platform android
  /// @detail callback
  /// @author hanchenchen.c
  /// @brief 收到来自房间中其他用户通过 sendUserBinaryMessage{@link #RTSRoom#sendUserBinaryMessage} 发来的点对点二进制消息时，会收到此回调。
  /// @param msgid 消息编号
  /// @param uid 消息发送者的用户 ID。
  /// @param message 收到的二进制消息内容。
  ///
  FutureOr<void> Function(long msgid, String uid, ByteBuffer message)?
      onUserBinaryMessageReceived;

  /// @platform android
  /// @detail callback
  /// @brief 向房间内单个用户发送文本或二进制消息后（P2P），消息发送方会收到该消息发送结果回调。
  /// @param msgid 本条消息的 ID。
  /// @param error 文本或二进制消息发送结果，详见 UserMessageSendResult{@link #UserMessageSendResult}
  /// @note 调用 sendUserMessage{@link #RTSRoom#sendUserMessage} 或 sendUserBinaryMessage{@link #RTSRoom#sendUserBinaryMessage} 接口，才能收到此回调。
  ///
  FutureOr<void> Function(long msgid, int error)? onUserMessageSendResult;

  /// @platform android
  /// @detail callback
  /// @brief 调用 sendRoomMessage{@link #RTSRoom#sendRoomMessage} 或 sendRoomBinaryMessage{@link #RTSRoom#sendRoomBinaryMessage} 向房间内群发文本或二进制消息后，消息发送方会收到该消息发送结果回调。
  /// @param msgid 本条消息的 ID。
  /// @param error 消息发送结果，详见 RoomMessageSendResult{@link #RoomMessageSendResult}
  ///
  FutureOr<void> Function(long msgid, int error)? onRoomMessageSendResult;
}

/// @detail callback
/// @brief 远端编码后视频数据监测器 <br>
/// 注意：回调函数是在 SDK 内部线程（非 UI 线程）同步抛出来的，请不要做耗时操作或直接操作 UI，否则可能导致 app 崩溃。

class IRemoteEncodedVideoFrameObserver {
  IRemoteEncodedVideoFrameObserver({this.onRemoteEncodedVideoFrame});

  /// @detail callback
  /// @author wangzhanqiang
  /// @brief 调用 registerRemoteEncodedVideoFrameObserver{@link #RTCEngine#registerRemoteEncodedVideoFrameObserver} 后，SDK 监测到远端编码后视频数据时，触发该回调
  /// @param streamId 收到的远端流 ID
  /// @param streamInfo 收到的远端流信息，参看 StreamInfo{@link #StreamInfo}
  /// @param encodedVideoFrame 收到的远端视频帧信息，参看 RTCEncodedVideoFrame{@link #RTCEncodedVideoFrame}
  /// @note encodedVideoFrame 只在回调函数作用域内有效，不要存储该参数并在其它函数内访问该参数的内存数据
  ///
  FutureOr<void> Function(String streamId)? onRemoteEncodedVideoFrame;
}

/// @detail callback
/// @brief IAudioEffectPlayer{@link #IAudioEffectPlayer} 对应的回调句柄。你必须调用 setEventHandler{@link #IAudioEffectPlayer#setEventHandler} 完成设置后，才能收到对应回调。

class IAudioEffectPlayerEventHandler {
  IAudioEffectPlayerEventHandler({this.onAudioEffectPlayerStateChanged});

  /// @detail callback
  /// @brief 播放状态改变时回调。
  /// @param effectId IAudioEffectPlayer{@link #IAudioEffectPlayer} 的 ID。通过 getAudioEffectPlayer{@link #RTCEngine#getAudioEffectPlayer} 设置。
  /// @param state 混音状态。参考 PlayerState{@link #PlayerState}。
  /// @param error 错误码。参考 PlayerError{@link #PlayerError}。
  /// @order 0
  ///
  FutureOr<void> Function(int effectId, PlayerState state, PlayerError error)?
      onAudioEffectPlayerStateChanged;
}

/// @detail callback
/// @brief WTN 事件回调接口。 <br>
/// 注意：回调函数是在 SDK 内部线程（非 UI 线程）同步抛出来的，请不要做耗时操作或直接操作 UI，否则可能导致 app 崩溃。

class IWTNStreamEventHandler {
  IWTNStreamEventHandler(
      {this.onWTNRemoteVideoStats,
      this.onWTNRemoteAudioStats,
      this.onWTNVideoSubscribeStateChanged,
      this.onWTNAudioSubscribeStateChanged,
      this.onWTNFirstRemoteAudioFrame,
      this.onWTNFirstRemoteVideoFrameDecoded,
      this.onWTNSEIMessageReceived,
      this.onWTNDataMessageReceived});

  /// @detail callback
  /// @valid since 3.60.
  /// @author hanchenchen
  /// @brief 通话中本地设备接收订阅的远端 WTN 视频流的流 ID 以及远端 WTN 视频流统计信息。
  /// @param streamId WTN 流 ID
  /// @param stats 远端 WTN 视频流的统计信息，详见 RemoteVideoStats{@link #RemoteVideoStats}。
  /// @order 0
  ///
  FutureOr<void> Function(String streamId, RemoteVideoStats stats)?
      onWTNRemoteVideoStats;

  /// @detail callback
  /// @valid since 3.60.
  /// @author hanchenchen
  /// @brief 通话中本地设备接收订阅的远端 WTN 音频流的流 ID 以及远端 WTN 音频流统计信息。
  /// @param streamId WTN 流 ID
  /// @param stats 远端 WTN 音频流的统计信息，详见 RemoteAudioStats{@link #RemoteAudioStats}。
  /// @order 1
  ///
  FutureOr<void> Function(String streamId, RemoteAudioStats stats)?
      onWTNRemoteAudioStats;

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
  FutureOr<void> Function(String streamId, WTNSubscribeState state,
      WTNSubscribeStateChangeReason reason)? onWTNVideoSubscribeStateChanged;

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
  FutureOr<void> Function(String streamId, WTNSubscribeState state,
      WTNSubscribeStateChangeReason reason)? onWTNAudioSubscribeStateChanged;

  /// @author hanchenchen
  /// @detail callback
  /// @valid since 3.60. 自 3.60 起，该回调替代了 `onFirstPublicStreamAudioFrame`。如果你已升级至 3.60 及以上版本，并且仍在使用该方法，请迁移到此回调。
  /// @brief WTN 流的首帧音频解码成功 <br>
  ///        关于订阅 WTN 音频流，详见 subscribeWTNAudioStream{@link #IWTNStream#subscribeWTNAudioStream}。
  /// @param streamId WTN 流 ID
  /// @order 3
  ///
  FutureOr<void> Function(String streamId)? onWTNFirstRemoteAudioFrame;

  /// @detail callback
  /// @valid since 3.60. 自 3.60 起，该回调替代了 `onFirstPublicStreamVideoFrameDecoded`。如果你已升级至 3.60 及以上版本，并且仍在使用该方法，请迁移到此回调。
  /// @author hanchenchen
  /// @brief WTN 流的首帧视频解码成功 <br>
  ///        关于订阅 WTN 流，详见 subscribeWTNVideoStream{@link #IWTNStream#subscribeWTNVideoStream}。
  /// @param streamId WTN 流 ID
  /// @param info 视频帧信息。详见 VideoFrameInfo{@link #VideoFrameInfo}。
  /// @order 4
  ///
  FutureOr<void> Function(String streamId, VideoFrameInfo info)?
      onWTNFirstRemoteVideoFrameDecoded;

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
  FutureOr<void> Function(String streamId, int channelId, Uint8List message)?
      onWTNSEIMessageReceived;

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
  FutureOr<void> Function(
          String streamId, Uint8List message, DataMessageSourceType sourceType)?
      onWTNDataMessageReceived;
}

/// @detail callback
/// @brief IMediaPlayer{@link #IMediaPlayer} 对应的回调句柄。你必须调用 setEventHandler{@link #IMediaPlayer#setEventHandler} 完成设置后，才能收到对应回调。

class IMediaPlayerEventHandler {
  IMediaPlayerEventHandler(
      {this.onMediaPlayerStateChanged,
      this.onMediaPlayerPlayingProgress,
      this.onMediaPlayerEvent});

  /// @detail callback
  /// @brief 播放状态改变时回调。
  /// @param playerId IMediaPlayer{@link #IMediaPlayer} 的 ID。通过 getMediaPlayer{@link #RTCEngine#getMediaPlayer} 设置。
  /// @param state 混音状态。参考 PlayerState{@link #PlayerState}。
  /// @param error 错误码。参考 PlayerError{@link #PlayerError}。
  /// @order 2
  ///
  FutureOr<void> Function(int playerId, PlayerState state, PlayerError error)?
      onMediaPlayerStateChanged;

  /// @detail callback
  /// @brief 播放进度周期性回调。回调周期通过 setProgressInterval{@link #IMediaPlayer#setProgressInterval} 设置。
  /// @param playerId IMediaPlayer{@link #IMediaPlayer} 的 ID。通过 getMediaPlayer{@link #RTCEngine#getMediaPlayer} 设置。
  /// @param progress 进度。单位 ms。
  /// @order 3
  ///
  FutureOr<void> Function(int playerId, int progress)?
      onMediaPlayerPlayingProgress;

  /// @valid since 3.59
  /// @detail callback
  /// @author wangfeng.1004
  /// @brief 播放事件回调。调用 selectAudioTrack{@link #IMediaPlayer#selectAudioTrack} 和 setPosition{@link #IMediaPlayer#setPosition} 后，会触发此回调。
  /// @param playerId IMediaPlayer{@link #IMediaPlayer} 的 ID。通过 getMediaPlayer{@link #RTCEngine#getMediaPlayer} 设置。
  /// @param event 播放器事件。参看 PlayerEvent{@link #PlayerEvent}。
  /// @param message 事件描述信息，可能为空。
  ///
  FutureOr<void> Function(int playerId, PlayerEvent event, String message)?
      onMediaPlayerEvent;
}

/// @detail callback
/// @brief 加密事件回调接口 <br>
/// 注意：回调函数是在 SDK 内部线程（非 UI 线程）同步抛出来的，请不要做耗时操作或直接操作 UI，否则可能导致 app 崩溃。

class IRTCEncryptionHandler {
  IRTCEncryptionHandler();
}

/// @detail callback
/// @author wangjunlin.3182
/// @brief 人脸检测结果回调观察者 <br>
/// 注意：回调函数是在 SDK 内部线程（非 UI 线程）同步抛出来的，请不要做耗时操作或直接操作 UI，否则可能导致 app 崩溃。

class IFaceDetectionObserver {
  IFaceDetectionObserver(
      {this.onFaceDetectResult, this.onExpressionDetectResult});

  /// @detail callback
  /// @author wangjunlin.3182
  /// @brief 特效 SDK 进行人脸检测结果的回调。 <br>
  ///        调用 enableFaceDetection{@link #IVideoEffect#enableFaceDetection} 注册了 IFaceDetectionObserver{@link #IFaceDetectionObserver}，并使用 RTC SDK 中包含的特效 SDK 进行视频特效处理时，你会收到此回调。
  /// @param result 人脸检测结果, 参看 FaceDetectionResult{@link #FaceDetectionResult}。
  ///
  FutureOr<void> Function(FaceDetectionResult result)? onFaceDetectResult;

  /// @hidden for internal use only
  /// @detail callback
  /// @author zhushufan.ref
  /// @brief 特效 SDK 进行人像属性检测结果的回调。 <br>
  ///        调用 registerFaceDetectionObserver 注册了 IFaceDetectionObserver{@link #IFaceDetectionObserver}，并调用 setVideoEffectExpressionDetect{@link #IVideoEffect#setVideoEffectExpressionDetect} 设置开启人像属性检测后，你会收到此回调。
  /// @param result 人像属性检测结果, 参看 ExpressionDetectResult{@link #ExpressionDetectResult}。
  ///
  FutureOr<void> Function(ExpressionDetectResult result)?
      onExpressionDetectResult;
}

/// @detail callback
/// @brief 内存播放数据源回调

class IMediaPlayerCustomSourceProvider {
  IMediaPlayerCustomSourceProvider();
}

/// @detail callback
/// @brief 自定义编码帧回调类 <br>
/// 注意：回调函数是在 SDK 内部线程（非 UI 线程）同步抛出来的，请不要做耗时操作或直接操作 UI，否则可能导致 app 崩溃。

class IExternalVideoEncoderEventHandler {
  IExternalVideoEncoderEventHandler(
      {this.onStart,
      this.onStop,
      this.onRateUpdate,
      this.onRequestKeyFrame,
      this.onActiveVideoLayer});

  /// @detail callback
  /// @author wangzhanqiang
  /// @brief 提示自定义编码帧可以开始推送的回调。 <br>
  ///        收到该回调后，你即可调用 pushExternalEncodedVideoFrame{@link #RTCEngine#pushExternalEncodedVideoFrame} 向 SDK 推送自定义编码视频帧
  /// @param streamId 可以推送的编码流的 ID
  /// @param streamInfo 可以推送的编码流的属性
  ///
  FutureOr<void> Function(String streamId)? onStart;

  /// @detail callback
  /// @author wangzhanqiang
  /// @brief 当收到该回调时，你需停止向 SDK 推送自定义编码视频帧
  /// @param streamId 需停止推送的编码流的 ID
  /// @param streamInfo 需停止推送的编码流的属性
  ///
  FutureOr<void> Function(String streamId)? onStop;

  /// @detail callback
  /// @author wangzhanqiang
  /// @brief 当自定义编码流的帧率或码率发生变化时，触发该回调
  /// @param streamId 发生变化的编码流的 ID
  /// @param streamInfo 发生变化的编码流的属性
  /// @param videoIndex 对应编码流的下标
  /// @param fps 变化后的帧率，单位：fps
  /// @param bitrateKbps 变化后的码率，单位：kbps
  ///
  FutureOr<void> Function(
      String streamId, int videoIndex, int fps, int bitrateKbps)? onRateUpdate;

  /// @detail callback
  /// @author wangzhanqiang
  /// @brief 提示流发布端需重新生成关键帧的回调
  /// @param streamId 远端编码流的 ID
  /// @param streamInfo 远端编码流的属性
  /// @param videoIndex 对应编码流的下标
  ///
  FutureOr<void> Function(String streamId, int videoIndex)? onRequestKeyFrame;

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
  FutureOr<void> Function(String streamId, int videoIndex, bool active)?
      onActiveVideoLayer;
}

/// @detail callback
/// @brief 本地视频帧监测器 <br>
/// 注意：回调函数是在 SDK 内部线程（非 UI 线程）同步抛出来的，请不要做耗时操作或直接操作 UI，否则可能导致 app 崩溃。

class ILocalEncodedVideoFrameObserver {
  ILocalEncodedVideoFrameObserver({this.onLocalEncodedVideoFrame});

  /// @detail callback
  /// @brief 调用 registerLocalEncodedVideoFrameObserver{@link #RTCEngine#registerLocalEncodedVideoFrameObserver} 后，SDK 每次使用内部采集，采集到一帧视频帧，或收到一帧外部视频帧时，都会回调该事件。
  /// @param videoSource 预留参数
  /// @param encodedVideoFrame 本地视频帧信息，参看 RTCEncodedVideoFrame{@link #RTCEncodedVideoFrame}
  /// @note encodedVideoFrame 只在回调函数作用域内有效，不要存储该参数并在其它函数内访问该参数的内存数据
  FutureOr<void> Function(dynamic videoSource)? onLocalEncodedVideoFrame;
}

/// @detail callback
/// @brief 音视频引擎事件回调接口 <br>
/// 注意：回调函数是在 SDK 内部线程（非 UI 线程）同步抛出来的，请不要做耗时操作或直接操作 UI，否则可能导致 app 崩溃。

class IRTCEngineEventHandler {
  IRTCEngineEventHandler(
      {this.onWarning,
      this.onError,
      this.onExtensionAccessError,
      this.onSysStats,
      this.onNetworkTypeChanged,
      this.onUserStartVideoCapture,
      this.onUserStopVideoCapture,
      this.onUserStartAudioCapture,
      this.onUserStopAudioCapture,
      this.onLocalAudioStateChanged,
      this.onRemoteAudioStateChanged,
      this.onLocalVideoStateChanged,
      this.onRemoteVideoStateChanged,
      this.onRemoteVideoSuperResolutionModeChanged,
      this.onVideoDenoiseModeChanged,
      this.onFirstRemoteVideoFrameRendered,
      this.onFirstRemoteVideoFrameDecoded,
      this.onFirstLocalVideoFrameCaptured,
      this.onLocalVideoSizeChanged,
      this.onRemoteVideoSizeChanged,
      this.onConnectionStateChanged,
      this.onAudioRouteChanged,
      this.onFirstLocalAudioFrame,
      this.onFirstRemoteAudioFrame,
      this.onSEIMessageReceived,
      this.onSEIStreamUpdate,
      this.onLoginResult,
      this.onLogout,
      this.onServerParamsSetResult,
      this.onGetPeerOnlineStatus,
      this.onUserMessageReceivedOutsideRoom,
      this.onUserBinaryMessageReceivedOutsideRoom,
      this.onUserMessageSendResultOutsideRoom,
      this.onServerMessageSendResult,
      this.onNetworkDetectionResult,
      this.onNetworkDetectionStopped,
      this.onAudioDeviceStateChanged,
      this.onVideoDeviceStateChanged,
      this.onAudioDeviceWarning,
      this.onVideoDeviceWarning,
      this.onRecordingStateUpdate,
      this.onRecordingProgressUpdate,
      this.onAudioRecordingStateUpdate,
      this.onAudioMixingPlayingProgress,
      this.onLocalAudioPropertiesReport,
      this.onAudioPlaybackDeviceTestVolume,
      this.onRemoteAudioPropertiesReport,
      this.onActiveSpeaker,
      this.onEchoTestResult,
      this.onCloudProxyConnected,
      this.onAudioDumpStateChanged,
      this.onLicenseWillExpire,
      this.onHardwareEchoDetectionResult,
      this.onLocalProxyStateChanged,
      this.onEffectError,
      this.onStreamSyncInfoReceived,
      this.onExternalScreenFrameUpdate,
      this.onRemoteSnapshotTakenToFile,
      this.onAudioFrameSendStateChanged,
      this.onVideoFrameSendStateChanged,
      this.onAudioFramePlayStateChanged,
      this.onVideoFramePlayStateChanged,
      this.onSimulcastSubscribeFallback,
      this.onPerformanceAlarms,
      this.onRemoteAudioPropertiesReportEx,
      this.onMixedStreamEvent,
      this.onSingleStreamEvent,
      this.onExperimentalCallback,
      this.onPushPublicStreamResult,
      this.onLogReport,
      this.onNetworkTimeSynchronized});

  /// @detail callback
  /// @author panjian.fishing
  /// @brief 发生警告回调。 <br>
  ///        SDK 运行时出现了警告。SDK 通常会自动恢复，警告信息可以忽略。
  /// @param warn 警告代码，参见 WarningCode{@link #WarningCode}
  ///
  FutureOr<void> Function(WarningCode code)? onWarning;

  /// @detail callback
  /// @author panjian.fishing
  /// @brief 发生错误回调。 <br>
  ///        SDK 运行时出现了网络或媒体相关的错误，且无法自动恢复时触发此回调。 <br>
  ///        你可能需要干预.
  /// @param err 错误代码，详情定义见: ErrorCode{@link #ErrorCode}
  ///
  FutureOr<void> Function(ErrorCode code)? onError;

  /// @valid since 3.52
  /// @detail callback
  /// @author zhanyunqiao
  /// @brief 当访问插件失败时，收到此回调。 <br>
  ///        RTC SDK 将一些功能封装成插件。当使用这些功能时，如果插件不存在，功能将无法使用。
  /// @param extensionName 插件名字
  /// @param msg 失败说明
  ///
  FutureOr<void> Function(String extensionName, String msg)?
      onExtensionAccessError;

  /// @detail callback
  /// @author wangzhanqiang
  /// @brief 每 2 秒发生回调，通知当前 cpu，内存使用的信息。
  /// @param stats cpu，内存信息。详见 SysStats{@link #SysStats} 数据类型。
  ///
  FutureOr<void> Function(SysStats stats)? onSysStats;

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
  FutureOr<void> Function(NetworkType type)? onNetworkTypeChanged;

  /// @detail callback
  /// @author liuyangyang
  /// @brief 房间内的可见用户调用 startVideoCapture{@link #RTCEngine#startVideoCapture} 开启内部视频采集时，房间内其他用户会收到此回调。
  /// @param streamId 视频流 ID
  /// @param streamInfo 视频流信息，详见 StreamInfo{@link #StreamInfo}。
  ///
  FutureOr<void> Function(String streamId, StreamInfo info)?
      onUserStartVideoCapture;

  /// @detail callback
  /// @author liuyangyang
  /// @brief 房间内的可见用户调用 stopVideoCapture{@link #RTCEngine#stopVideoCapture} 关闭内部视频采集时，房间内其他用户会收到此回调。 <br>
  ///        若发布视频数据前未开启采集，房间内所有可见用户会收到此回调。
  /// @param streamId 视频流 ID
  /// @param streamInfo 视频流信息，详见 StreamInfo{@link #StreamInfo}。
  ///
  FutureOr<void> Function(String streamId, StreamInfo info)?
      onUserStopVideoCapture;

  /// @detail callback
  /// @author dixing
  /// @brief 房间内的用户调用 startAudioCapture{@link #RTCEngine#startAudioCapture} 开启音频采集时，房间内其他用户会收到此回调。
  /// @param streamId 开启音频采集的远端流 ID
  /// @param streamInfo 开启音频采集的远端流信息，详见 StreamInfo{@link #StreamInfo}
  ///
  FutureOr<void> Function(String streamId, StreamInfo info)?
      onUserStartAudioCapture;

  /// @detail callback
  /// @author dixing
  /// @brief 房间内的用户调用 stopAudioCapture{@link #RTCEngine#stopAudioCapture} 关闭音频采集时，房间内其他用户会收到此回调。
  /// @param streamId 流 ID
  /// @param streamInfo 流信息，详见 StreamInfo{@link #StreamInfo}
  ///
  FutureOr<void> Function(String streamId, StreamInfo info)?
      onUserStopAudioCapture;

  /// @detail callback
  /// @author zhangyuanyuan.0101
  /// @brief 本地音频流的状态发生改变时，收到此回调。
  /// @param audioSource 预留参数
  /// @param state 本地音频设备的状态，详见 LocalAudioStreamState{@link #LocalAudioStreamState}
  /// @param error 本地音频流状态改变时的错误码，详见 LocalAudioStreamError{@link #LocalAudioStreamError}
  ///
  FutureOr<void> Function(dynamic audioSource, LocalAudioStreamState state,
      LocalAudioStreamError error)? onLocalAudioStateChanged;

  /// @detail callback
  /// @author zhangyuanyuan.0101
  /// @brief 用户订阅来自远端的音频流状态发生改变时，会收到此回调，了解当前的远端音频流状态。
  /// @param streamId 远端流 ID
  /// @param streamInfo 远端流信息, 详见 StreamInfo{@link #StreamInfo}
  /// @param state 远端音频流状态，详见 RemoteAudioState{@link #RemoteAudioState}
  /// @param reason 远端音频流状态改变的原因，详见 RemoteAudioStateChangeReason{@link #RemoteAudioStateChangeReason}
  ///
  FutureOr<void> Function(
      String streamId,
      StreamInfo streamInfo,
      RemoteAudioState state,
      RemoteAudioStateChangeReason reason)? onRemoteAudioStateChanged;

  /// @detail callback
  /// @author shenpengliang
  /// @brief 本地视频流的状态发生改变时，收到该事件。
  /// @param videoSource  预留参数
  /// @param state 本地视频流状态，参看 LocalVideoStreamState{@link #LocalVideoStreamState}
  /// @param error 本地视频状态改变时的错误码，参看 LocalVideoStreamError{@link #LocalVideoStreamError}
  ///
  FutureOr<void> Function(dynamic videoSource, LocalVideoStreamState state,
      LocalVideoStreamError error)? onLocalVideoStateChanged;

  /// @detail callback
  /// @author shenpengliang
  /// @brief 远端视频流的状态发生改变时，房间内订阅此流的用户会收到该事件。
  /// @param streamId 远端流 ID
  /// @param streamInfo 远端流信息, 详见 StreamInfo{@link #StreamInfo}
  /// @param videoState 远端视频流状态，参看 RemoteVideoState{@link #RemoteVideoState}
  /// @param videoStateReason 远端视频流状态改变原因，参看 RemoteVideoStateChangeReason{@link #RemoteVideoStateChangeReason}
  /// @note 本回调仅适用于主流，不适用于屏幕流。
  ///
  FutureOr<void> Function(
      String streamId,
      StreamInfo streamInfo,
      RemoteVideoState state,
      RemoteVideoStateChangeReason videoStateReason)? onRemoteVideoStateChanged;

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
  FutureOr<void> Function(
          String streamId,
          StreamInfo streamInfo,
          VideoSuperResolutionMode mode,
          VideoSuperResolutionModeChangedReason reason)?
      onRemoteVideoSuperResolutionModeChanged;

  /// @hidden for internal use only
  /// @valid since 3.54
  /// @detail callback
  /// @author Yujianli
  /// @brief 降噪模式状态变更回调。当降噪模式的运行状态发生改变，SDK 会触发该回调，提示用户降噪模式改变后的运行状态及状态发生改变的原因。
  /// @param mode 视频降噪模式，参看 VideoDenoiseMode{@link #VideoDenoiseMode}。
  /// @param reason 视频降噪模式改变的原因，参看 VideoDenoiseModeChangedReason{@link #VideoDenoiseModeChangedReason}。
  ///
  FutureOr<void> Function(
          VideoDenoiseMode mode, VideoDenoiseModeChangedReason reason)?
      onVideoDenoiseModeChanged;

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
  FutureOr<void> Function(
          String streamId, StreamInfo streamInfo, VideoFrameInfo frameInfo)?
      onFirstRemoteVideoFrameRendered;

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
  FutureOr<void> Function(
          String streamId, StreamInfo streamInfo, VideoFrameInfo frameInfo)?
      onFirstRemoteVideoFrameDecoded;

  /// @detail callback
  /// @author zhangzhenyu.samuel
  /// @brief RTC SDK 在本地完成第一帧视频帧或屏幕视频帧采集时，收到此回调。
  /// @param videoSource 预留参数。
  /// @param frameInfo 视频信息，参看 VideoFrameInfo{@link #VideoFrameInfo}
  /// @note 对于采集到的本地视频帧，你可以调用 setLocalVideoCanvas{@link #RTCEngine#setLocalVideoCanvas} 或 setLocalVideoSink{@link #RTCEngine#setLocalVideoSink} 在本地渲染。
  ///
  FutureOr<void> Function(dynamic videoSource, VideoFrameInfo frameInfo)?
      onFirstLocalVideoFrameCaptured;

  /// @detail callback
  /// @author zhushufan.ref
  /// @brief 本地预览视频大小或旋转信息发生改变时，收到此回调。
  /// @param videoSource 预留参数。
  /// @param frameInfo 视频帧信息，参看 VideoFrameInfo{@link #VideoFrameInfo}
  ///
  FutureOr<void> Function(dynamic videoSource, VideoFrameInfo frameInfo)?
      onLocalVideoSizeChanged;

  /// @detail callback
  /// @author zhushufan.ref
  /// @brief 远端视频大小或旋转信息发生改变时，房间内订阅此视频流的用户会收到此回调。
  /// @param streamId 远端流 ID
  /// @param streamInfo 远端流信息, 详见 StreamInfo{@link #StreamInfo}
  /// @param frameInfo 视频帧信息，参看 VideoFrameInfo{@link #VideoFrameInfo}
  ///
  FutureOr<void> Function(
          String streamId, StreamInfo streamInfo, VideoFrameInfo frameInfo)?
      onRemoteVideoSizeChanged;

  /// @detail callback
  /// @author hanchenchen.c
  /// @brief 回调 SDK 与信令服务器连接状态相关事件。当 SDK 与信令服务器的网络连接状态改变时回调该事件。
  /// @param state <br>
  ///        当前 SDK 与信令服务器连接状态。 详细定义参见 ConnectionState{@link #ConnectionState}
  /// @param reason <br>
  ///        引起信令服务器连接状态发生改变的原因，目前未启用固定为 -1 。
  /// @note 更多信息参见 [连接状态提示](https://www.volcengine.com/docs/6348/95376)。
  ///
  FutureOr<void> Function(int state, int? reason)? onConnectionStateChanged;

  /// @detail callback
  /// @author dixing
  /// @brief 音频播放路由变化时，收到该回调。
  /// @param route 新的音频播放路由，详见 AudioRoute{@link #AudioRoute}
  /// @note 插拔音频外设，或调用 setAudioRoute{@link #RTCEngine#setAudioRoute} 都可能触发音频路由切换，详见[移动端设置音频路由](https://www.volcengine.com/docs/6348/117836) 。
  ///
  FutureOr<void> Function(AudioRoute device)? onAudioRouteChanged;

  /// @detail callback
  /// @author zhangyuanyuan.0101
  /// @brief 发布音频流时，采集到第一帧音频帧，收到该回调。
  /// @param audioSource 预留参数
  /// @note 如果发布音频流时，未开启本地音频采集，SDK 会推送静音帧，也会收到此回调。
  ///
  FutureOr<void> Function(dynamic audioSource)? onFirstLocalAudioFrame;

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
  FutureOr<void> Function(String streamId, StreamInfo streamInfo)?
      onFirstRemoteAudioFrame;

  /// @detail callback
  /// @author wangzhanqiang
  /// @brief 收到通过调用 sendSEIMessage{@link #RTCEngine#sendSEIMessage} 发送带有 SEI 消息的视频帧时，收到此回调。
  /// @param streamId 包含 SEI 发送者的流 ID
  /// @param streamInfo 包含 SEI 发送者的流信息, 详见 StreamInfo{@link #StreamInfo}
  /// @param message 收到的 SEI 消息内容
  ///
  FutureOr<void> Function(
          String streamId, StreamInfo streamInfo, Uint8List message)?
      onSEIMessageReceived;

  /// @detail callback
  /// @author wangzhanqiang
  /// @brief 黑帧视频流发布状态回调。 <br>
  ///        在语音通话场景下，本地用户调用 sendSEIMessage{@link #RTCEngine#sendSEIMessage} 通过黑帧视频流发送 SEI 数据时，流的发送状态会通过该回调通知远端用户。 <br>
  ///        你可以通过此回调判断携带 SEI 数据的视频帧为黑帧，从而不对该视频帧进行渲染。
  /// @param streamId 远端流 ID
  /// @param streamInfo 远端流信息，参看 StreamInfo{@link #StreamInfo}。
  /// @param event 黑帧视频流状态，参看 SEIStreamUpdateEvent{@link #SEIStreamUpdateEvent}
  ///
  FutureOr<void> Function(
          String streamId, StreamInfo streamInfo, SEIStreamUpdateEvent event)?
      onSEIStreamUpdate;

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
  FutureOr<void> Function(String uid, LoginErrorCode errorCode, int elapsed)?
      onLoginResult;

  /// @detail callback
  /// @author hanchenchen.c
  /// @brief 登出结果回调
  /// @param reason 用户登出的原因，参看 LogoutReason{@link #LogoutReason}
  /// @note 在以下两种情况下会收到此回调：调用 logout{@link #RTCEngine#logout} 接口主动退出；或其他用户以相同 UserId 进行 `login` 导致本地用户被动登出。
  ///
  FutureOr<void> Function(LogoutReason reason)? onLogout;

  /// @detail callback
  /// @author hanchenchen.c
  /// @brief 设置应用服务器参数的返回结果
  /// @param error <br>
  ///        设置结果 <br>
  ///        - 返回 200，设置成功
  ///        - 返回其他，设置失败，详见 UserMessageSendResult{@link #UserMessageSendResult}
  /// @note 调用 setServerParams{@link #RTCEngine#setServerParams} 后，会收到此回调。
  ///
  FutureOr<void> Function(int error)? onServerParamsSetResult;

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
  FutureOr<void> Function(String peerUserId, UserOnlineStatus status)?
      onGetPeerOnlineStatus;

  /// @detail callback
  /// @author hanchenchen.c
  /// @brief 收到房间外用户调用 sendUserMessageOutsideRoom{@link #RTCEngine#sendUserMessageOutsideRoom} 发来的文本消息时，会收到此回调。
  /// @param msgid 消息编号。
  /// @param uid 消息发送者 ID。
  /// @param message 收到的文本消息内容。
  ///
  FutureOr<void> Function(int msgid, String uid, String message)?
      onUserMessageReceivedOutsideRoom;

  /// @detail callback
  /// @author hanchenchen.c
  /// @brief 收到房间外用户调用 sendUserBinaryMessageOutsideRoom{@link #RTCEngine#sendUserBinaryMessageOutsideRoom} 发来的二进制消息时，会收到此回调。
  /// @param msgid 消息编号。
  /// @param uid 消息发送者 ID。
  /// @param message 收到的二进制消息内容。
  ///
  FutureOr<void> Function(int msgid, String uid, Uint8List message)?
      onUserBinaryMessageReceivedOutsideRoom;

  /// @detail callback
  /// @author hanchenchen.c
  /// @brief 给房间外指定的用户发送消息的结果回调。<br>
  ///        当调用 sendUserMessageOutsideRoom{@link #RTCEngine#sendUserMessageOutsideRoom} 或 sendUserBinaryMessageOutsideRoom{@link #RTCEngine#sendUserBinaryMessageOutsideRoom} 发送消息后，会收到此回调。
  /// @param msgid 消息 ID。<br>
  ///        所有的 P2P 和 P2Server 消息共用一个 ID 序列。
  /// @param error 消息发送结果。详见 UserMessageSendResult{@link #UserMessageSendResult}。
  ///
  FutureOr<void> Function(int msgid, UserMessageSendResult error)?
      onUserMessageSendResultOutsideRoom;

  /// @detail callback
  /// @author hanchenchen.c
  /// @brief 给应用服务器发送消息的回调。
  /// @param msgid 本条消息的 ID <br>
  ///        所有的 P2P 和 P2Server 消息共用一个 ID 序列。
  /// @param error 消息发送结果，详见 UserMessageSendResult{@link #UserMessageSendResult}。
  /// @param message 应用服务器收到 HTTP 请求后，在 ACK 中返回的信息。消息不超过 64 KB。
  /// @note 本回调为异步回调。当调用 sendServerMessage{@link #RTCEngine#sendServerMessage} 或 sendServerBinaryMessage{@link #RTCEngine#sendServerBinaryMessage} 接口发送消息后，会收到此回调。
  ///
  FutureOr<void> Function(
          int msgid, UserMessageSendResult error, Uint8List message)?
      onServerMessageSendResult;

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
  FutureOr<void> Function(
      NetworkDetectionLinkType type,
      NetworkQuality quality,
      int rtt,
      double lostRate,
      int bitrate,
      int jitter)? onNetworkDetectionResult;

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
  FutureOr<void> Function(NetworkDetectionStopReason reason)?
      onNetworkDetectionStopped;

  /// @detail callback
  /// @author dixing
  /// @brief 音频设备状态回调。提示音频采集、音频播放等媒体设备的状态。
  /// @param deviceID 设备 ID
  /// @param deviceType 设备类型，详见 AudioDeviceType{@link #AudioDeviceType}。
  /// @param deviceState 设备状态，详见 MediaDeviceState{@link #MediaDeviceState}。
  /// @param deviceError 设备错误类型，详见 MediaDeviceError{@link #MediaDeviceError}。
  ///
  FutureOr<void> Function(
      String deviceId,
      AudioDeviceType deviceType,
      MediaDeviceState deviceState,
      MediaDeviceError deviceError)? onAudioDeviceStateChanged;

  /// @detail callback
  /// @author liuyangyang
  /// @brief 视频设备状态回调。提示摄像头视频采集、屏幕视频采集等媒体设备的状态。
  /// @param deviceID 设备 ID
  /// @param deviceType 设备类型，详见 VideoDeviceType{@link #VideoDeviceType}。
  /// @param deviceState 设备状态，详见 MediaDeviceState{@link #MediaDeviceState}。
  /// @param deviceError 设备错误类型，详见 MediaDeviceError{@link #MediaDeviceError}。
  ///
  FutureOr<void> Function(
      String deviceId,
      VideoDeviceType deviceType,
      MediaDeviceState deviceState,
      MediaDeviceError deviceError)? onVideoDeviceStateChanged;

  /// @detail callback
  /// @author dixing
  /// @brief 音频设备警告回调。音频设备包括音频采集设备、音频渲染设备等。
  /// @param deviceID 设备 ID
  /// @param deviceType 参看 AudioDeviceType{@link #AudioDeviceType}
  /// @param deviceWarning 参看 MediaDeviceWarning{@link #MediaDeviceWarning}
  ///
  FutureOr<void> Function(String deviceId, AudioDeviceType deviceType,
      MediaDeviceWarning deviceWarning)? onAudioDeviceWarning;

  /// @detail callback
  /// @author liuyangyang
  /// @brief 视频设备警告回调，包括视频采集等设备。
  /// @param deviceID 设备 ID
  /// @param deviceType 参看 VideoDeviceType{@link #VideoDeviceType}
  /// @param deviceWarning 参看 MediaDeviceWarning{@link #MediaDeviceWarning}
  ///
  FutureOr<void> Function(String deviceId, VideoDeviceType deviceType,
      MediaDeviceWarning deviceWarning)? onVideoDeviceWarning;

  /// @detail callback
  /// @author wangzhanqiang
  /// @brief 获取本地录制状态回调。 <br>
  ///        该回调由 startFileRecording{@link #RTCEngine#startFileRecording} 或 stopFileRecording{@link #RTCEngine#stopFileRecording} 触发。
  /// @param videoSource 预留参数。
  /// @param state 录制状态，参看 RecordingState{@link #RecordingState}
  /// @param errorCode 录制错误码，参看 RecordingErrorCode{@link #RecordingErrorCode}
  /// @param info 录制文件的详细信息，参看 RecordingInfo{@link #RecordingInfo}
  ///
  FutureOr<void> Function(dynamic videoSource, RecordingState state,
      RecordingErrorCode errorCode, RecordingInfo info)? onRecordingStateUpdate;

  /// @detail callback
  /// @author wangzhanqiang
  /// @brief 本地录制进度回调。 <br>
  ///        该回调由 startFileRecording{@link #RTCEngine#startFileRecording} 触发，录制状态正常时，系统每秒钟都会通过该回调提示录制进度。
  /// @param videoSource 预留参数。
  /// @param progress 录制进度，参看 RecordingProgress{@link #RecordingProgress}
  /// @param info 录制文件的详细信息，参看 RecordingInfo{@link #RecordingInfo}
  ///
  FutureOr<void> Function(
          dynamic videoSource, RecordingProgress process, RecordingInfo info)?
      onRecordingProgressUpdate;

  /// @detail callback
  /// @author huangshouqin
  /// @brief 调用 startAudioRecording{@link #RTCEngine#startAudioRecording} 或 stopAudioRecording{@link #RTCEngine#stopAudioRecording} 改变音频文件录制状态时，收到此回调。
  /// @param state 录制状态，参看 AudioRecordingState{@link #AudioRecordingState}
  /// @param errorCode 录制错误码，参看 AudioRecordingErrorCode{@link #AudioRecordingErrorCode}
  ///
  FutureOr<void> Function(
          AudioRecordingState state, AudioRecordingErrorCode errorCode)?
      onAudioRecordingStateUpdate;

  /// @detail callback
  /// @author songxiaomeng.19
  /// @brief 混音音频文件播放进度回调
  /// @param mixId 混音 ID
  /// @param progress 当前混音音频文件播放进度，单位毫秒
  /// @note 调用 setAudioMixingProgressInterval 将时间间隔设为大于 0 的值后，或调用 startAudioMixing 将 AudioMixingConfig 中的时间间隔设为大于 0 的值后，SDK 会按照设置的时间间隔回调该事件。
  ///
  FutureOr<void> Function(int mixId, int progress)?
      onAudioMixingPlayingProgress;

  /// @detail callback
  /// @author wangjunzheng
  /// @brief 调用 enableAudioPropertiesReport{@link #RTCEngine#enableAudioPropertiesReport} 后，你会周期性地收到此回调，了解本地音频的瞬时相关信息。 <br>
  ///        本地音频包括使用 RTC SDK 内部机制采集的麦克风音频，屏幕音频和本地混音音频信息。
  /// @param audioPropertiesInfos 本地音频信息，详见 LocalAudioPropertiesInfo{@link #LocalAudioPropertiesInfo} 。
  ///
  FutureOr<void> Function(List<LocalAudioPropertiesInfo> audioPropertiesInfos)?
      onLocalAudioPropertiesReport;

  /// @detail callback
  /// @author dixing
  /// @brief 回调音频设备测试时的播放音量
  /// @param volume 音频设备测试播放音量。取值范围：[0,255]
  /// @note 调用 startAudioPlaybackDeviceTest{@link #IRTCAudioDeviceManager#startAudioPlaybackDeviceTest} 或 startAudioDeviceRecordTest{@link #IRTCAudioDeviceManager#startAudioDeviceRecordTest}，开始播放音频文件或录音时，将开启该回调。本回调为周期性回调，回调周期由上述接口的 `interval` 参数指定。
  ///
  FutureOr<void> Function(int volume)? onAudioPlaybackDeviceTestVolume;

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
  FutureOr<void> Function(List<RemoteAudioPropertiesInfo> audioPropertiesInfos,
      int totalRemoteVolume)? onRemoteAudioPropertiesReport;

  /// @detail callback
  /// @author gongzhengduo
  /// @brief 调用 enableAudioPropertiesReport{@link #RTCEngine#enableAudioPropertiesReport} 后，根据设置的 `AudioPropertiesConfig.interval`，你会周期性地收到此回调，获取房间内的最活跃用户信息。
  /// @param roomId 房间 ID
  /// @param uid 最活跃用户（ActiveSpeaker）的用户 ID
  ///
  FutureOr<void> Function(String roomId, String userId)? onActiveSpeaker;

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
  FutureOr<void> Function(EchoTestResult result)? onEchoTestResult;

  /// @detail callback
  /// @author daining.nemo
  /// @brief 调用 startCloudProxy{@link #RTCEngine#startCloudProxy} 开启云代理，SDK 首次成功连接云代理服务器时，回调此事件。
  /// @param interval 从开启云代理到连接成功经过的时间，单位为 ms
  ///
  FutureOr<void> Function(int interval)? onCloudProxyConnected;

  /// @hidden for internal use only
  /// @detail callback
  /// @author songxiaomeng.19
  /// @brief 音频 dump 状态改变回调
  /// @param status 音频 dump 状态，参见 AudioDumpStatus{@link #AudioDumpStatus}
  /// @note 本回调用于内部排查音质相关异常问题，开发者无需关注。
  ///
  FutureOr<void> Function(AudioDumpStatus status)? onAudioDumpStateChanged;

  /// @hidden internal use only
  /// @detail callback
  /// @author wangyu.1705
  /// @brief license 过期时间提醒
  /// @param days 即将过期剩余天数
  ///
  FutureOr<void> Function(int days)? onLicenseWillExpire;

  /// @detail callback
  /// @author zhangcaining
  /// @brief 通话前回声检测结果回调。
  /// @param hardwareEchoDetectionResult 参见 HardwareEchoDetectionResult{@link #HardwareEchoDetectionResult}
  /// @note
  ///        - 通话前调用 startHardwareEchoDetection{@link #RTCEngine#startHardwareEchoDetection} 后，将触发本回调返回检测结果。
  ///        - 建议在收到检测结果后，调用 stopHardwareEchoDetection{@link #RTCEngine#stopHardwareEchoDetection} 停止检测，释放对音频设备的占用。
  ///        - 如果 SDK 在通话中检测到回声，将通过 onAudioDeviceWarning{@link #IRTCEngineEventHandler#onAudioDeviceWarning} 回调 `MEDIA_DEVICE_WARNING_DETECT_LEAK_ECHO`。
  ///
  FutureOr<void> Function(HardwareEchoDetectionResult result)?
      onHardwareEchoDetectionResult;

  /// @detail callback
  /// @author keshixing.rtc
  /// @brief 本地代理状态发生改变回调。调用 setLocalProxy{@link #RTCEngine#setLocalProxy} 设置本地代理后，SDK 会触发此回调，通知代理连接的状态。
  /// @param localProxyType 本地代理类型。参看 LocalProxyType{@link #LocalProxyType} 。
  /// @param localProxyState 本地代理状态。参看 LocalProxyState{@link #LocalProxyState}。
  /// @param localProxyError 本地代理错误。参看 LocalProxyError{@link #LocalProxyError}。
  ///
  FutureOr<void> Function(
      LocalProxyType localProxyType,
      LocalProxyState localProxyState,
      LocalProxyError localProxyError)? onLocalProxyStateChanged;

  /// @hidden internal use only
  /// @detail callback
  /// @author wangqianqian.1104
  /// @brief 当特效设置失败时，收到此回调。
  /// @param error 特效错误类型。参看 EffectErrorType{@link #EffectErrorType}。
  /// @param msg 错误信息。
  ///
  FutureOr<void> Function(EffectErrorType error, String msg)? onEffectError;

  /// @detail callback
  /// @author wangjunzheng
  /// @brief 音频流同步信息回调。可以通过此回调，在远端用户调用 sendStreamSyncInfo{@link #RTCEngine#sendStreamSyncInfo} 发送音频流同步消息后，收到远端发送的音频流同步信息。
  /// @param streamId 远端流 ID
  /// @param streamInfo 远端流信息，详见 StreamInfo{@link #StreamInfo}
  /// @param streamType 媒体流类型，详见 SyncInfoStreamType{@link #SyncInfoStreamType}
  /// @param data 消息内容。
  ///
  FutureOr<void> Function(String streamId, StreamInfo streamInfo,
      SyncInfoStreamType streamType, Uint8List data)? onStreamSyncInfoReceived;

  /// @hidden
  /// @detail callback
  /// @author zhoubohui
  /// @brief 外部采集时，调用 setOriginalScreenVideoInfo 设置屏幕或窗口大小改变前的分辨率后，若屏幕采集模式为智能模式，你将收到此回调，根据 RTC 智能决策合适的帧率和分辨率积（宽*高）重新采集。
  /// @param info RTC 智能决策后合适的帧率和分辨率积（宽*高）。参看 FrameUpdateInfo{@link #FrameUpdateInfo}。
  ///
  FutureOr<void> Function(
          FrameUpdateInfo engine, FrameUpdateInfo? frameUpdateInfo)?
      onExternalScreenFrameUpdate;

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
  FutureOr<void> Function(
      String streamId,
      String filePath,
      int width,
      int height,
      SnapshotErrorCode errorCode,
      int taskId)? onRemoteSnapshotTakenToFile;

  /// @detail callback
  /// @author wangjunzheng
  /// @brief 本地音频首帧发送状态发生改变时，收到此回调。
  /// @param streamId 流 ID
  /// @param streamInfo 流信息，参看 StreamInfo{@link #StreamInfo}
  /// @param user 用户信息，参看 RtcUser{@link #RtcUser}
  /// @param state 首帧发送状态，详见 FirstFrameSendState{@link #FirstFrameSendState}
  ///
  FutureOr<void> Function(String streamId, StreamInfo streamInfo, RtcUser user,
      FirstFrameSendState state)? onAudioFrameSendStateChanged;

  /// @detail callback
  /// @author wangfujun
  /// @brief 视频首帧发送状态发生改变时，收到此回调。
  /// @param streamId 流 ID
  /// @param streamInfo 流信息，参看 StreamInfo{@link #StreamInfo}
  /// @param user 用户信息，参看 RtcUser{@link #RtcUser}
  /// @param state 首帧发送状态，详见 FirstFrameSendState{@link #FirstFrameSendState}
  ///
  FutureOr<void> Function(String streamId, StreamInfo streamInfo, RtcUser user,
      FirstFrameSendState state)? onVideoFrameSendStateChanged;

  /// @detail callback
  /// @author wangjunzheng
  /// @brief 音频首帧播放状态发生改变时，收到此回调。
  /// @param streamId 流 ID
  /// @param streamInfo 流信息，参看 StreamInfo{@link #StreamInfo}
  /// @param user 用户信息，参看 RtcUser{@link #RtcUser}
  /// @param state 首帧播放状态，详见 FirstFramePlayState{@link #FirstFramePlayState}
  ///
  FutureOr<void> Function(String streamId, StreamInfo streamInfo, RtcUser user,
      FirstFramePlayState state)? onAudioFramePlayStateChanged;

  /// @detail callback
  /// @author wangfujun
  /// @brief 远端视频流的首帧播放状态改变时，收到此回调。
  /// @param streamId 流 ID
  /// @param streamInfo 流信息，参看 StreamInfo{@link #StreamInfo}
  /// @param user 用户信息，参看 RtcUser{@link #RtcUser}
  /// @param state 首帧播放状态，详见 FirstFramePlayState{@link #FirstFramePlayState}
  ///
  FutureOr<void> Function(String streamId, StreamInfo streamInfo, RtcUser user,
      FirstFramePlayState state)? onVideoFramePlayStateChanged;

  /// @detail callback
  /// @author wangfujun
  /// @region 音视频回退
  /// @brief 音视频流因网络环境变化等原因发生回退，或从回退中恢复时，触发该回调。
  /// @param streamId 流 ID
  /// @param streamInfo 流信息，参看 StreamInfo{@link #StreamInfo}
  /// @param event 音视频流发生变化的信息。参看 RemoteStreamSwitch{@link #RemoteStreamSwitch}。
  ///
  FutureOr<void> Function(
          String streamId, StreamInfo streamInfo, RemoteStreamSwitch event)?
      onSimulcastSubscribeFallback;

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
  FutureOr<void> Function(
      String streamId,
      StreamInfo streamInfo,
      PerformanceAlarmMode mode,
      PerformanceAlarmReason reason,
      SourceWantedData data)? onPerformanceAlarms;

  /// @hidden internal use only
  /// @detail callback
  /// @author lizheng
  ///
  FutureOr<void> Function(List<RemoteAudioPropertiesInfo> audioPropertiesInfos,
      int? totalRemoteVolume)? onRemoteAudioPropertiesReportEx;

  /// @valid since 3.60. 自 3.60 起，该回调替代了 `onStreamMixingEvent` 和 `onPushPublicStreamResult` 方法来实现下述功能。如果你已升级至 3.60 及以上版本，并且仍在使用这两个方法，请迁移到此回调。
  /// @detail callback
  /// @author lizheng
  /// @brief 合流转推 CDN / WTN 流状态回调
  /// @param info 任务详情，参看 MixedStreamTaskInfo{@link #MixedStreamTaskInfo}。
  /// @param event 任务事件，参看 MixedStreamTaskEvent{@link #MixedStreamTaskEvent}。
  /// @param error 任务错误码，参看 MixedStreamTaskErrorCode{@link #MixedStreamTaskErrorCode}
  ///
  FutureOr<void> Function(MixedStreamTaskInfo info, MixedStreamTaskEvent event,
      MixedStreamTaskErrorCode error)? onMixedStreamEvent;

  /// @valid since 3.60.
  /// @detail callback
  /// @author lizheng
  /// @brief 单流转推 CDN 状态回调
  /// @param taskId 任务 ID
  /// @param event 任务状态, 参看 SingleStreamTaskEvent{@link #SingleStreamTaskEvent}
  /// @param error 错误码，参看 SingleStreamTaskErrorCode{@link #SingleStreamTaskErrorCode}
  ///
  FutureOr<void> Function(String taskId, SingleStreamTaskEvent event,
      SingleStreamTaskErrorCode error)? onSingleStreamEvent;

  /// @hidden internal use only
  /// @valid since 3.60.
  /// @detail callback
  /// @author hegangjie
  /// @brief 试验性接口回调
  /// @param param 回调内容(JSON string)
  ///
  FutureOr<void> Function(String param)? onExperimentalCallback;

  /// @deprecated since 3.60, use onMixedStreamEvent{@link #IRTCEngineEventHandler#onMixedStreamEvent} instead.
  /// @detail callback
  /// @author qipengxiang
  /// @brief WTN 流发布结果回调。 <br>
  ///        调用 startPushMixedStream{@link #RTCEngine#startPushMixedStream} 接口发布WTN 流后，启动结果通过此回调方法通知用户。
  /// @param roomId 发布WTN 流的房间 ID
  /// @param publicStreamId WTN 流 ID
  /// @param error WTN 流发布结果状态码。详见 PublicStreamErrorCode{@link #PublicStreamErrorCode}。
  ///
  FutureOr<void> Function(
          String roomId, String streamId, PublicStreamErrorCode errorCode)?
      onPushPublicStreamResult;

  /// @platform android
  /// @detail callback
  /// @author chenweiming.push
  /// @brief 上报日志时回调该事件。
  /// @param logType <br>
  ///        日志类型。
  /// @param logContent <br>
  ///        日志内容。
  ///
  FutureOr<void> Function(String logType, JSONObject logContent)? onLogReport;

  /// @platform android
  /// @detail callback
  /// @author songxiaomeng.19
  /// @brief 首次调用 getNetworkTimeInfo{@link #RTCEngine#getNetworkTimeInfo} 后，SDK 内部启动网络时间同步，同步完成时会触发此回调。
  ///
  FutureOr<void> Function()? onNetworkTimeSynchronized;
}

/// @detail callback
/// @author majun.lvhiei
/// @brief 自定义音频处理器。 <br>
/// 注意：回调函数是在 SDK 内部线程（非 UI 线程）同步抛出来的，请不要做耗时操作或直接操作 UI，否则可能导致 app 崩溃。

class IAudioFrameProcessor {
  IAudioFrameProcessor();
}

/// @hidden for internal use only
/// @detail callback
/// @brief 转推直播观察者。

class IClientMixedStreamObserver {
  IClientMixedStreamObserver(
      {this.onClientMixedStreamEvent,
      this.onMixedAudioFrame,
      this.onMixedVideoFrame,
      this.onMixedDataFrame,
      this.onMixedFirstAudioFrame,
      this.onMixedFirstVideoFrame});

  /// @hidden for internal use only
  /// @detail callback
  /// @author liujingchao
  /// @brief 转推直播状态回调
  /// @param eventType 转推直播任务状态, 参看 ByteRTCStreamMixingEvent{@link #ByteRTCStreamMixingEvent}
  /// @param taskId 转推直播任务 ID
  /// @param error 转推直播错误码，参看 MixedStreamTaskErrorCode{@link #MixedStreamTaskErrorCode}
  /// @param mixType 转推直播类型，参看 MixedStreamType{@link #MixedStreamType}
  ///
  FutureOr<void> Function(MixedStreamTaskInfo info, MixedStreamType type,
      MixedStreamTaskEvent event)? onClientMixedStreamEvent;

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
  FutureOr<void> Function(String taskId, Uint8List audioFrame, int timeStampMs)?
      onMixedAudioFrame;

  /// @hidden for internal use only
  /// @detail callback
  /// @author liujingchao
  /// @brief 端云一体合流视频 YUV 回调
  /// @param taskId 转推直播任务 ID
  /// @param videoFrame YUV 合流视频数据帧，参看 IVideoFrame{@link #IVideoFrame}
  /// @note
  ///        - 收到该回调的周期与视频的帧间隔一致。
  ///
  FutureOr<void> Function(String taskId, IVideoFrame videoFrame)?
      onMixedVideoFrame;

  /// @hidden for internal use only
  /// @detail callback
  /// @author liujingchao
  /// @brief 端云一体合流视频 SEI 数据回调
  /// @param taskId 转推直播任务 ID
  /// @param dataFrame SEI 数据
  /// @param time 时间信息
  ///
  FutureOr<void> Function(String taskId, Uint8List dataFrame)? onMixedDataFrame;

  /// @hidden for internal use only
  ///
  FutureOr<void> Function(String taskId)? onMixedFirstAudioFrame;

  /// @hidden for internal use only
  ///
  FutureOr<void> Function(String taskId)? onMixedFirstVideoFrame;
}

class android_IMediaPlayerAudioFrameObserver
    extends $p_a.IMediaPlayerAudioFrameObserver {
  android_IMediaPlayerAudioFrameObserver();

  FutureOr<void> onFrame(dynamic playerId, dynamic frame) async {
    if ($instance == null || $instance is! IMediaPlayerAudioFrameObserver) {
      return;
    }
    return ($instance as IMediaPlayerAudioFrameObserver)
        .onFrame
        ?.call(int.tryParse(playerId.toString()) ?? 0);
  }
}

class ios_IMediaPlayerAudioFrameObserver
    extends $p_i.ByteRTCMediaPlayerAudioFrameObserver {
  ios_IMediaPlayerAudioFrameObserver();

  FutureOr<void> onFrame$audioFrame(
      dynamic playerId, dynamic audioFrame) async {
    if ($instance == null || $instance is! IMediaPlayerAudioFrameObserver) {
      return;
    }
    return ($instance as IMediaPlayerAudioFrameObserver)
        .onFrame
        ?.call(int.tryParse(playerId.toString()) ?? 0);
  }
}

class android_IAudioFrameObserver extends $p_a.IAudioFrameObserver {
  android_IAudioFrameObserver();

  FutureOr<void> onRecordAudioFrame(dynamic audioFrame) async {
    if ($instance == null || $instance is! IAudioFrameObserver) {
      return;
    }
    return ($instance as IAudioFrameObserver).onRecordAudioFrame?.call(
        packObject(
            audioFrame,
            () => AudioFrame.fromMap(AudioFrame.deepPackedMapValues(
                AudioFrame.mapMemberToConstructorParams(audioFrame)))));
  }

  FutureOr<void> onPlaybackAudioFrame(dynamic audioFrame) async {
    if ($instance == null || $instance is! IAudioFrameObserver) {
      return;
    }
    return ($instance as IAudioFrameObserver).onPlaybackAudioFrame?.call(
        packObject(
            audioFrame,
            () => AudioFrame.fromMap(AudioFrame.deepPackedMapValues(
                AudioFrame.mapMemberToConstructorParams(audioFrame)))));
  }

  FutureOr<void> onRemoteUserAudioFrame(
      dynamic streamId, dynamic streamInfo, dynamic audioFrame) async {
    if ($instance == null || $instance is! IAudioFrameObserver) {
      return;
    }
    return ($instance as IAudioFrameObserver).onRemoteUserAudioFrame?.call(
        streamId.toString(),
        packObject(
            audioFrame,
            () => AudioFrame.fromMap(AudioFrame.deepPackedMapValues(
                AudioFrame.mapMemberToConstructorParams(audioFrame)))));
  }

  FutureOr<void> onMixedAudioFrame(dynamic audioFrame) async {
    if ($instance == null || $instance is! IAudioFrameObserver) {
      return;
    }
    return ($instance as IAudioFrameObserver).onMixedAudioFrame?.call(
        packObject(
            audioFrame,
            () => AudioFrame.fromMap(AudioFrame.deepPackedMapValues(
                AudioFrame.mapMemberToConstructorParams(audioFrame)))));
  }

  FutureOr<void> onCaptureMixedAudioFrame(dynamic audioFrame) async {
    if ($instance == null || $instance is! IAudioFrameObserver) {
      return;
    }
    return ($instance as IAudioFrameObserver).onCaptureMixedAudioFrame?.call(
        packObject(
            audioFrame,
            () => AudioFrame.fromMap(AudioFrame.deepPackedMapValues(
                AudioFrame.mapMemberToConstructorParams(audioFrame)))));
  }
}

class ios_IAudioFrameObserver extends $p_i.ByteRTCAudioFrameObserver {
  ios_IAudioFrameObserver();

  FutureOr<void> onRecordAudioFrame(dynamic audioFrame) async {
    if ($instance == null || $instance is! IAudioFrameObserver) {
      return;
    }
    return ($instance as IAudioFrameObserver).onRecordAudioFrame?.call(
        packObject(
            audioFrame,
            () => AudioFrame.fromMap(AudioFrame.deepPackedMapValues(
                AudioFrame.mapMemberToConstructorParams(audioFrame)))));
  }

  FutureOr<void> onPlaybackAudioFrame(dynamic audioFrame) async {
    if ($instance == null || $instance is! IAudioFrameObserver) {
      return;
    }
    return ($instance as IAudioFrameObserver).onPlaybackAudioFrame?.call(
        packObject(
            audioFrame,
            () => AudioFrame.fromMap(AudioFrame.deepPackedMapValues(
                AudioFrame.mapMemberToConstructorParams(audioFrame)))));
  }

  FutureOr<void> onRemoteUserAudioFrame$info$audioFrame(
      dynamic streamId, dynamic info, dynamic audioFrame) async {
    if ($instance == null || $instance is! IAudioFrameObserver) {
      return;
    }
    return ($instance as IAudioFrameObserver).onRemoteUserAudioFrame?.call(
        streamId.toString(),
        packObject(
            audioFrame,
            () => AudioFrame.fromMap(AudioFrame.deepPackedMapValues(
                AudioFrame.mapMemberToConstructorParams(audioFrame)))));
  }

  FutureOr<void> onMixedAudioFrame(dynamic audioFrame) async {
    if ($instance == null || $instance is! IAudioFrameObserver) {
      return;
    }
    return ($instance as IAudioFrameObserver).onMixedAudioFrame?.call(
        packObject(
            audioFrame,
            () => AudioFrame.fromMap(AudioFrame.deepPackedMapValues(
                AudioFrame.mapMemberToConstructorParams(audioFrame)))));
  }

  FutureOr<void> onCaptureMixedAudioFrame(dynamic audioFrame) async {
    if ($instance == null || $instance is! IAudioFrameObserver) {
      return;
    }
    return ($instance as IAudioFrameObserver).onCaptureMixedAudioFrame?.call(
        packObject(
            audioFrame,
            () => AudioFrame.fromMap(AudioFrame.deepPackedMapValues(
                AudioFrame.mapMemberToConstructorParams(audioFrame)))));
  }
}

class android_IRTCRoomEventHandler extends $p_a.IRTCRoomEventHandler {
  android_IRTCRoomEventHandler();

  FutureOr<void> onLeaveRoom(dynamic stats) async {
    if ($instance == null || $instance is! IRTCRoomEventHandler) {
      return;
    }
    return ($instance as IRTCRoomEventHandler).onLeaveRoom?.call(packObject(
        stats,
        () => RTCRoomStats.fromMap(RTCRoomStats.deepPackedMapValues(
            RTCRoomStats.mapMemberToConstructorParams(stats)))));
  }

  FutureOr<void> onRoomStateChangedWithReason(
      dynamic roomId, dynamic uid, dynamic state, dynamic reason) async {
    if ($instance == null || $instance is! IRTCRoomEventHandler) {
      return;
    }
    return ($instance as IRTCRoomEventHandler)
        .onRoomStateChangedWithReason
        ?.call(
            roomId.toString(),
            uid.toString(),
            t_RoomState.android_to_code($p_a.RoomState.values
                .firstWhere((t) => t.$value == state || t.name == state)),
            t_RoomStateChangeReason.android_to_code($p_a
                .RoomStateChangeReason.values
                .firstWhere((t) => t.$value == reason || t.name == reason)));
  }

  FutureOr<void> onRoomStateChanged(
      dynamic roomId, dynamic uid, dynamic state, dynamic extraInfo) async {
    if ($instance == null || $instance is! IRTCRoomEventHandler) {
      return;
    }
    return ($instance as IRTCRoomEventHandler).onRoomStateChanged?.call(
        roomId.toString(),
        uid.toString(),
        int.tryParse(state.toString()) ?? 0,
        extraInfo.toString());
  }

  FutureOr<void> onStreamStateChanged(
      dynamic roomId, dynamic uid, dynamic state, dynamic extraInfo) async {
    if ($instance == null || $instance is! IRTCRoomEventHandler) {
      return;
    }
    return ($instance as IRTCRoomEventHandler).onStreamStateChanged?.call(
        roomId.toString(),
        uid.toString(),
        int.tryParse(state.toString()) ?? 0,
        extraInfo.toString());
  }

  FutureOr<void> onAVSyncStateChange(dynamic state) async {
    if ($instance == null || $instance is! IRTCRoomEventHandler) {
      return;
    }
    return ($instance as IRTCRoomEventHandler).onAVSyncStateChange?.call(
        t_AVSyncState.android_to_code($p_a.AVSyncState.values
            .firstWhere((t) => t.$value == state || t.name == state)));
  }

  FutureOr<void> onRoomStats(dynamic stats) async {
    if ($instance == null || $instance is! IRTCRoomEventHandler) {
      return;
    }
    return ($instance as IRTCRoomEventHandler).onRoomStats?.call(packObject(
        stats,
        () => RTCRoomStats.fromMap(RTCRoomStats.deepPackedMapValues(
            RTCRoomStats.mapMemberToConstructorParams(stats)))));
  }

  FutureOr<void> onRoomEvent(
      dynamic roomId, dynamic uid, dynamic state, dynamic info) async {
    if ($instance == null || $instance is! IRTCRoomEventHandler) {
      return;
    }
    return ($instance as IRTCRoomEventHandler).onRoomEvent?.call(
        roomId.toString(),
        uid.toString(),
        t_RoomEvent.android_to_code($p_a.RoomEvent.values
            .firstWhere((t) => t.$value == state || t.name == state)),
        packObject(
            info,
            () => RoomEventInfo.fromMap(RoomEventInfo.deepPackedMapValues(
                RoomEventInfo.mapMemberToConstructorParams(info)))));
  }

  FutureOr<void> onUserJoined(dynamic userInfo) async {
    if ($instance == null || $instance is! IRTCRoomEventHandler) {
      return;
    }
    return ($instance as IRTCRoomEventHandler).onUserJoined?.call(packObject(
        userInfo,
        () => UserInfo.fromMap(UserInfo.deepPackedMapValues(
            UserInfo.mapMemberToConstructorParams(userInfo)))));
  }

  FutureOr<void> onUserLeave(dynamic uid, dynamic reason) async {
    if ($instance == null || $instance is! IRTCRoomEventHandler) {
      return;
    }
    return ($instance as IRTCRoomEventHandler)
        .onUserLeave
        ?.call(uid.toString(), int.tryParse(reason.toString()) ?? 0);
  }

  FutureOr<void> onVideoPublishStateChanged(dynamic streamId,
      dynamic streamInfo, dynamic state, dynamic reason) async {
    if ($instance == null || $instance is! IRTCRoomEventHandler) {
      return;
    }
    return ($instance as IRTCRoomEventHandler).onVideoPublishStateChanged?.call(
        streamId.toString(),
        t_PublishState.android_to_code($p_a.PublishState.values
            .firstWhere((t) => t.$value == state || t.name == state)),
        t_PublishStateChangeReason.android_to_code($p_a
            .PublishStateChangeReason.values
            .firstWhere((t) => t.$value == reason || t.name == reason)));
  }

  FutureOr<void> onAudioPublishStateChanged(dynamic streamId,
      dynamic streamInfo, dynamic state, dynamic reason) async {
    if ($instance == null || $instance is! IRTCRoomEventHandler) {
      return;
    }
    return ($instance as IRTCRoomEventHandler).onAudioPublishStateChanged?.call(
        streamId.toString(),
        t_PublishState.android_to_code($p_a.PublishState.values
            .firstWhere((t) => t.$value == state || t.name == state)),
        t_PublishStateChangeReason.android_to_code($p_a
            .PublishStateChangeReason.values
            .firstWhere((t) => t.$value == reason || t.name == reason)));
  }

  FutureOr<void> onVideoSubscribeStateChanged(dynamic streamId,
      dynamic streamInfo, dynamic state, dynamic reason) async {
    if ($instance == null || $instance is! IRTCRoomEventHandler) {
      return;
    }
    return ($instance as IRTCRoomEventHandler)
        .onVideoSubscribeStateChanged
        ?.call(
            streamId.toString(),
            t_SubscribeState.android_to_code($p_a.SubscribeState.values
                .firstWhere((t) => t.$value == state || t.name == state)),
            t_SubscribeStateChangeReason.android_to_code($p_a
                .SubscribeStateChangeReason.values
                .firstWhere((t) => t.$value == reason || t.name == reason)));
  }

  FutureOr<void> onAudioSubscribeStateChanged(dynamic streamId,
      dynamic streamInfo, dynamic state, dynamic reason) async {
    if ($instance == null || $instance is! IRTCRoomEventHandler) {
      return;
    }
    return ($instance as IRTCRoomEventHandler)
        .onAudioSubscribeStateChanged
        ?.call(
            streamId.toString(),
            t_SubscribeState.android_to_code($p_a.SubscribeState.values
                .firstWhere((t) => t.$value == state || t.name == state)),
            t_SubscribeStateChangeReason.android_to_code($p_a
                .SubscribeStateChangeReason.values
                .firstWhere((t) => t.$value == reason || t.name == reason)));
  }

  FutureOr<void> onLocalStreamStats(
      dynamic streamId, dynamic streamInfo, dynamic stats) async {
    if ($instance == null || $instance is! IRTCRoomEventHandler) {
      return;
    }
    return ($instance as IRTCRoomEventHandler).onLocalStreamStats?.call(
        streamId.toString(),
        packObject(
            streamInfo,
            () => StreamInfo.fromMap(StreamInfo.deepPackedMapValues(
                StreamInfo.mapMemberToConstructorParams(streamInfo)))),
        packObject(
            stats,
            () => LocalStreamStats.fromMap(LocalStreamStats.deepPackedMapValues(
                LocalStreamStats.mapMemberToConstructorParams(stats)))));
  }

  FutureOr<void> onRemoteStreamStats(
      dynamic streamId, dynamic streamInfo, dynamic stats) async {
    if ($instance == null || $instance is! IRTCRoomEventHandler) {
      return;
    }
    return ($instance as IRTCRoomEventHandler).onRemoteStreamStats?.call(
        streamId.toString(),
        packObject(
            streamInfo,
            () => StreamInfo.fromMap(StreamInfo.deepPackedMapValues(
                StreamInfo.mapMemberToConstructorParams(streamInfo)))),
        packObject(
            stats,
            () => RemoteStreamStats.fromMap(
                RemoteStreamStats.deepPackedMapValues(
                    RemoteStreamStats.mapMemberToConstructorParams(stats)))));
  }

  FutureOr<void> onStreamPublishSuccess(dynamic uid, dynamic isScreen) async {
    if ($instance == null || $instance is! IRTCRoomEventHandler) {
      return;
    }
    return ($instance as IRTCRoomEventHandler)
        .onStreamPublishSuccess
        ?.call(isScreen);
  }

  FutureOr<void> onAVSyncEvent(
      dynamic roomId, dynamic uid, dynamic eventCode) async {
    if ($instance == null || $instance is! IRTCRoomEventHandler) {
      return;
    }
    return ($instance as IRTCRoomEventHandler).onAVSyncEvent?.call(
        roomId.toString(),
        uid.toString(),
        t_AVSyncEvent.android_to_code($p_a.AVSyncEvent.values
            .firstWhere((t) => t.$value == eventCode || t.name == eventCode)));
  }

  FutureOr<void> onUserPublishStreamVideo(
      dynamic streamId, dynamic streamInfo, dynamic isPublish) async {
    if ($instance == null || $instance is! IRTCRoomEventHandler) {
      return;
    }
    return ($instance as IRTCRoomEventHandler).onUserPublishStreamVideo?.call(
        streamId.toString(),
        packObject(
            streamInfo,
            () => StreamInfo.fromMap(StreamInfo.deepPackedMapValues(
                StreamInfo.mapMemberToConstructorParams(streamInfo)))),
        isPublish);
  }

  FutureOr<void> onUserPublishStreamAudio(
      dynamic streamId, dynamic streamInfo, dynamic isPublish) async {
    if ($instance == null || $instance is! IRTCRoomEventHandler) {
      return;
    }
    return ($instance as IRTCRoomEventHandler).onUserPublishStreamAudio?.call(
        streamId.toString(),
        packObject(
            streamInfo,
            () => StreamInfo.fromMap(StreamInfo.deepPackedMapValues(
                StreamInfo.mapMemberToConstructorParams(streamInfo)))),
        isPublish);
  }

  FutureOr<void> onRoomMessageReceived(
      dynamic msgid, dynamic uid, dynamic message) async {
    if ($instance == null || $instance is! IRTCRoomEventHandler) {
      return;
    }
    return ($instance as IRTCRoomEventHandler).onRoomMessageReceived?.call(
        int.tryParse(msgid.toString()) ?? 0,
        uid.toString(),
        message.toString());
  }

  FutureOr<void> onRoomBinaryMessageReceived(
      dynamic msgid, dynamic uid, dynamic message) async {
    if ($instance == null || $instance is! IRTCRoomEventHandler) {
      return;
    }
    return ($instance as IRTCRoomEventHandler)
        .onRoomBinaryMessageReceived
        ?.call(int.tryParse(msgid.toString()) ?? 0, uid.toString(), message);
  }

  FutureOr<void> onUserMessageReceived(
      dynamic msgid, dynamic uid, dynamic message) async {
    if ($instance == null || $instance is! IRTCRoomEventHandler) {
      return;
    }
    return ($instance as IRTCRoomEventHandler).onUserMessageReceived?.call(
        int.tryParse(msgid.toString()) ?? 0,
        uid.toString(),
        message.toString());
  }

  FutureOr<void> onUserBinaryMessageReceived(
      dynamic msgid, dynamic uid, dynamic message) async {
    if ($instance == null || $instance is! IRTCRoomEventHandler) {
      return;
    }
    return ($instance as IRTCRoomEventHandler)
        .onUserBinaryMessageReceived
        ?.call(int.tryParse(msgid.toString()) ?? 0, uid.toString(), message);
  }

  FutureOr<void> onUserMessageSendResult(dynamic msgid, dynamic error) async {
    if ($instance == null || $instance is! IRTCRoomEventHandler) {
      return;
    }
    return ($instance as IRTCRoomEventHandler).onUserMessageSendResult?.call(
        int.tryParse(msgid.toString()) ?? 0,
        t_UserMessageSendResult.android_to_code($p_a
            .UserMessageSendResult.values
            .firstWhere((t) => t.$value == error || t.name == error)));
  }

  FutureOr<void> onRoomMessageSendResult(dynamic msgid, dynamic error) async {
    if ($instance == null || $instance is! IRTCRoomEventHandler) {
      return;
    }
    return ($instance as IRTCRoomEventHandler).onRoomMessageSendResult?.call(
        int.tryParse(msgid.toString()) ?? 0,
        t_RoomMessageSendResult.android_to_code($p_a
            .RoomMessageSendResult.values
            .firstWhere((t) => t.$value == error || t.name == error)));
  }

  FutureOr<void> onVideoStreamBanned(dynamic uid, dynamic banned) async {
    if ($instance == null || $instance is! IRTCRoomEventHandler) {
      return;
    }
    return ($instance as IRTCRoomEventHandler)
        .onVideoStreamBanned
        ?.call(uid.toString(), banned);
  }

  FutureOr<void> onAudioStreamBanned(dynamic uid, dynamic banned) async {
    if ($instance == null || $instance is! IRTCRoomEventHandler) {
      return;
    }
    return ($instance as IRTCRoomEventHandler)
        .onAudioStreamBanned
        ?.call(uid.toString(), banned);
  }

  FutureOr<void> onForwardStreamStateChanged(List<dynamic> stateInfos) async {
    if ($instance == null || $instance is! IRTCRoomEventHandler) {
      return;
    }
    return ($instance as IRTCRoomEventHandler)
        .onForwardStreamStateChanged
        ?.call(stateInfos
            .map((e) => packObject(
                e,
                () => ForwardStreamStateInfo.fromMap(
                    ForwardStreamStateInfo.deepPackedMapValues(
                        ForwardStreamStateInfo.mapMemberToConstructorParams(
                            e)))))
            .toList());
  }

  FutureOr<void> onForwardStreamEvent(List<dynamic> eventInfos) async {
    if ($instance == null || $instance is! IRTCRoomEventHandler) {
      return;
    }
    return ($instance as IRTCRoomEventHandler).onForwardStreamEvent?.call(
        eventInfos
            .map((e) => packObject(
                e,
                () => ForwardStreamEventInfo.fromMap(
                    ForwardStreamEventInfo.deepPackedMapValues(
                        ForwardStreamEventInfo.mapMemberToConstructorParams(
                            e)))))
            .toList());
  }

  FutureOr<void> onNetworkQuality(
      dynamic localQuality, List<dynamic> remoteQualities) async {
    if ($instance == null || $instance is! IRTCRoomEventHandler) {
      return;
    }
    return ($instance as IRTCRoomEventHandler).onNetworkQuality?.call(
        packObject(
            localQuality,
            () => NetworkQualityStats.fromMap(
                NetworkQualityStats.deepPackedMapValues(
                    NetworkQualityStats.mapMemberToConstructorParams(
                        localQuality)))),
        remoteQualities
            .map((e) => packObject(
                e,
                () => NetworkQualityStats.fromMap(
                    NetworkQualityStats.deepPackedMapValues(
                        NetworkQualityStats.mapMemberToConstructorParams(e)))))
            .toList());
  }

  FutureOr<void> onSetRoomExtraInfoResult(
      dynamic taskId, dynamic result) async {
    if ($instance == null || $instance is! IRTCRoomEventHandler) {
      return;
    }
    return ($instance as IRTCRoomEventHandler).onSetRoomExtraInfoResult?.call(
        int.tryParse(taskId.toString()) ?? 0,
        t_SetRoomExtraInfoResult.android_to_code($p_a
            .SetRoomExtraInfoResult.values
            .firstWhere((t) => t.$value == result || t.name == result)));
  }

  FutureOr<void> onRoomExtraInfoUpdate(dynamic key, dynamic value,
      dynamic lastUpdateUserId, dynamic lastUpdateTimeMs) async {
    if ($instance == null || $instance is! IRTCRoomEventHandler) {
      return;
    }
    return ($instance as IRTCRoomEventHandler).onRoomExtraInfoUpdate?.call(
        key.toString(),
        value.toString(),
        lastUpdateUserId.toString(),
        int.tryParse(lastUpdateTimeMs.toString()) ?? 0);
  }

  FutureOr<void> onRoomStreamExtraInfoUpdate(
      dynamic streamId, dynamic streamInfo, dynamic extraInfo) async {
    if ($instance == null || $instance is! IRTCRoomEventHandler) {
      return;
    }
    return ($instance as IRTCRoomEventHandler)
        .onRoomStreamExtraInfoUpdate
        ?.call(
            streamId.toString(),
            packObject(
                streamInfo,
                () => StreamInfo.fromMap(StreamInfo.deepPackedMapValues(
                    StreamInfo.mapMemberToConstructorParams(streamInfo)))),
            extraInfo.toString());
  }

  FutureOr<void> onUserVisibilityChanged(
      dynamic currentUserVisibility, dynamic errorCode) async {
    if ($instance == null || $instance is! IRTCRoomEventHandler) {
      return;
    }
    return ($instance as IRTCRoomEventHandler).onUserVisibilityChanged?.call(
        currentUserVisibility,
        t_UserVisibilityChangeError.android_to_code($p_a
            .UserVisibilityChangeError.values
            .firstWhere((t) => t.$value == errorCode || t.name == errorCode)));
  }

  FutureOr<void> onSubtitleStateChanged(
      dynamic state, dynamic errorCode, dynamic errorMessage) async {
    if ($instance == null || $instance is! IRTCRoomEventHandler) {
      return;
    }
    return ($instance as IRTCRoomEventHandler).onSubtitleStateChanged?.call(
        t_SubtitleState.android_to_code($p_a.SubtitleState.values
            .firstWhere((t) => t.$value == state || t.name == state)),
        t_SubtitleErrorCode.android_to_code($p_a.SubtitleErrorCode.values
            .firstWhere((t) => t.$value == errorCode || t.name == errorCode)),
        errorMessage.toString());
  }

  FutureOr<void> onSubtitleMessageReceived(List<dynamic> subtitles) async {
    if ($instance == null || $instance is! IRTCRoomEventHandler) {
      return;
    }
    return ($instance as IRTCRoomEventHandler).onSubtitleMessageReceived?.call(
        subtitles
            .map((e) => packObject(
                e,
                () => SubtitleMessage.fromMap(
                    SubtitleMessage.deepPackedMapValues(
                        SubtitleMessage.mapMemberToConstructorParams(e)))))
            .toList());
  }

  FutureOr<void> onRoomWarning(dynamic warn) async {
    if ($instance == null || $instance is! IRTCRoomEventHandler) {
      return;
    }
    return ($instance as IRTCRoomEventHandler)
        .onRoomWarning
        ?.call(packObject(warn, () => RTCRoom()), null);
  }

  FutureOr<void> onTokenWillExpire() async {
    if ($instance == null || $instance is! IRTCRoomEventHandler) {
      return;
    }
    return ($instance as IRTCRoomEventHandler).onTokenWillExpire?.call();
  }

  FutureOr<void> onPublishPrivilegeTokenWillExpire() async {
    if ($instance == null || $instance is! IRTCRoomEventHandler) {
      return;
    }
    return ($instance as IRTCRoomEventHandler)
        .onPublishPrivilegeTokenWillExpire
        ?.call();
  }

  FutureOr<void> onSubscribePrivilegeTokenWillExpire() async {
    if ($instance == null || $instance is! IRTCRoomEventHandler) {
      return;
    }
    return ($instance as IRTCRoomEventHandler)
        .onSubscribePrivilegeTokenWillExpire
        ?.call();
  }

  FutureOr<void> onStreamSubscribed(
      dynamic stateCode, dynamic userId, dynamic info) async {
    if ($instance == null || $instance is! IRTCRoomEventHandler) {
      return;
    }
    return ($instance as IRTCRoomEventHandler).onStreamSubscribed?.call(
        t_SubscribeState.android_to_code($p_a.SubscribeState.values
            .firstWhere((t) => t.$value == stateCode || t.name == stateCode)),
        userId.toString(),
        packObject(
            info,
            () => SubscribeConfig.fromMap(SubscribeConfig.deepPackedMapValues(
                SubscribeConfig.mapMemberToConstructorParams(info)))));
  }
}

class ios_IRTCRoomEventHandler extends $p_i.ByteRTCRoomDelegate {
  ios_IRTCRoomEventHandler();

  FutureOr<void> rtcRoom$onLeaveRoom(dynamic rtcRoom, dynamic stats) async {
    if ($instance == null || $instance is! IRTCRoomEventHandler) {
      return;
    }
    return ($instance as IRTCRoomEventHandler).onLeaveRoom?.call(packObject(
        stats,
        () => RTCRoomStats.fromMap(RTCRoomStats.deepPackedMapValues(
            RTCRoomStats.mapMemberToConstructorParams(stats)))));
  }

  FutureOr<void> rtcRoom$onRoomStateChangedWithReason$withUid$state$reason(
      dynamic rtcRoom,
      dynamic roomId,
      dynamic uid,
      dynamic state,
      dynamic reason) async {
    if ($instance == null || $instance is! IRTCRoomEventHandler) {
      return;
    }
    return ($instance as IRTCRoomEventHandler)
        .onRoomStateChangedWithReason
        ?.call(
            roomId.toString(),
            uid.toString(),
            t_RoomState.ios_to_code($p_i.ByteRTCRoomState.values
                .firstWhere((t) => t.$value == state || t.name == state)),
            t_RoomStateChangeReason.ios_to_code($p_i
                .ByteRTCRoomStateChangeReason.values
                .firstWhere((t) => t.$value == reason || t.name == reason)));
  }

  FutureOr<void> rtcRoom$onRoomStateChanged$withUid$state$extraInfo(
      dynamic rtcRoom,
      dynamic roomId,
      dynamic uid,
      dynamic state,
      dynamic extraInfo) async {
    if ($instance == null || $instance is! IRTCRoomEventHandler) {
      return;
    }
    return ($instance as IRTCRoomEventHandler).onRoomStateChanged?.call(
        roomId.toString(),
        uid.toString(),
        int.tryParse(state.toString()) ?? 0,
        extraInfo.toString());
  }

  FutureOr<void> rtcRoom$onStreamStateChanged$withUid$state$extraInfo(
      dynamic rtcRoom,
      dynamic roomId,
      dynamic uid,
      dynamic state,
      dynamic extraInfo) async {
    if ($instance == null || $instance is! IRTCRoomEventHandler) {
      return;
    }
    return ($instance as IRTCRoomEventHandler).onStreamStateChanged?.call(
        roomId.toString(),
        uid.toString(),
        int.tryParse(state.toString()) ?? 0,
        extraInfo.toString());
  }

  FutureOr<void> rtcRoom$onAVSyncStateChange(
      dynamic rtcRoom, dynamic state) async {
    if ($instance == null || $instance is! IRTCRoomEventHandler) {
      return;
    }
    return ($instance as IRTCRoomEventHandler).onAVSyncStateChange?.call(
        t_AVSyncState.ios_to_code($p_i.ByteRTCAVSyncState.values
            .firstWhere((t) => t.$value == state || t.name == state)));
  }

  FutureOr<void> rtcRoom$onRoomStats(dynamic rtcRoom, dynamic stats) async {
    if ($instance == null || $instance is! IRTCRoomEventHandler) {
      return;
    }
    return ($instance as IRTCRoomEventHandler).onRoomStats?.call(packObject(
        stats,
        () => RTCRoomStats.fromMap(RTCRoomStats.deepPackedMapValues(
            RTCRoomStats.mapMemberToConstructorParams(stats)))));
  }

  FutureOr<void> rtcRoom$onRoomEvent$uid$state$info(dynamic rtcRoom,
      dynamic roomId, dynamic uid, dynamic state, dynamic info) async {
    if ($instance == null || $instance is! IRTCRoomEventHandler) {
      return;
    }
    return ($instance as IRTCRoomEventHandler).onRoomEvent?.call(
        roomId.toString(),
        uid.toString(),
        t_RoomEvent.ios_to_code($p_i.ByteRTCRoomEvent.values
            .firstWhere((t) => t.$value == state || t.name == state)),
        packObject(
            info,
            () => RoomEventInfo.fromMap(RoomEventInfo.deepPackedMapValues(
                RoomEventInfo.mapMemberToConstructorParams(info)))));
  }

  FutureOr<void> rtcRoom$onUserJoined(dynamic rtcRoom, dynamic userInfo) async {
    if ($instance == null || $instance is! IRTCRoomEventHandler) {
      return;
    }
    return ($instance as IRTCRoomEventHandler).onUserJoined?.call(packObject(
        userInfo,
        () => UserInfo.fromMap(UserInfo.deepPackedMapValues(
            UserInfo.mapMemberToConstructorParams(userInfo)))));
  }

  FutureOr<void> rtcRoom$onUserLeave$reason(
      dynamic rtcRoom, dynamic uid, dynamic reason) async {
    if ($instance == null || $instance is! IRTCRoomEventHandler) {
      return;
    }
    return ($instance as IRTCRoomEventHandler)
        .onUserLeave
        ?.call(uid.toString(), int.tryParse(reason.toString()) ?? 0);
  }

  FutureOr<void> rtcRoom$onVideoPublishStateChanged$info$state$reason(
      dynamic rtcRoom,
      dynamic streamId,
      dynamic info,
      dynamic state,
      dynamic reason) async {
    if ($instance == null || $instance is! IRTCRoomEventHandler) {
      return;
    }
    return ($instance as IRTCRoomEventHandler).onVideoPublishStateChanged?.call(
        streamId.toString(),
        t_PublishState.ios_to_code($p_i.ByteRTCPublishState.values
            .firstWhere((t) => t.$value == state || t.name == state)),
        t_PublishStateChangeReason.ios_to_code($p_i
            .ByteRTCPublishStateChangeReason.values
            .firstWhere((t) => t.$value == reason || t.name == reason)));
  }

  FutureOr<void> rtcRoom$onAudioPublishStateChanged$info$state$reason(
      dynamic rtcRoom,
      dynamic streamId,
      dynamic info,
      dynamic state,
      dynamic reason) async {
    if ($instance == null || $instance is! IRTCRoomEventHandler) {
      return;
    }
    return ($instance as IRTCRoomEventHandler).onAudioPublishStateChanged?.call(
        streamId.toString(),
        t_PublishState.ios_to_code($p_i.ByteRTCPublishState.values
            .firstWhere((t) => t.$value == state || t.name == state)),
        t_PublishStateChangeReason.ios_to_code($p_i
            .ByteRTCPublishStateChangeReason.values
            .firstWhere((t) => t.$value == reason || t.name == reason)));
  }

  FutureOr<void> rtcRoom$onVideoSubscribeStateChanged$info$state$reason(
      dynamic rtcRoom,
      dynamic streamId,
      dynamic info,
      dynamic state,
      dynamic reason) async {
    if ($instance == null || $instance is! IRTCRoomEventHandler) {
      return;
    }
    return ($instance as IRTCRoomEventHandler)
        .onVideoSubscribeStateChanged
        ?.call(
            streamId.toString(),
            t_SubscribeState.ios_to_code($p_i.ByteRTCSubscribeState.values
                .firstWhere((t) => t.$value == state || t.name == state)),
            t_SubscribeStateChangeReason.ios_to_code($p_i
                .ByteRTCSubscribeStateChangeReason.values
                .firstWhere((t) => t.$value == reason || t.name == reason)));
  }

  FutureOr<void> rtcRoom$onAudioSubscribeStateChanged$info$state$reason(
      dynamic rtcRoom,
      dynamic streamId,
      dynamic info,
      dynamic state,
      dynamic reason) async {
    if ($instance == null || $instance is! IRTCRoomEventHandler) {
      return;
    }
    return ($instance as IRTCRoomEventHandler)
        .onAudioSubscribeStateChanged
        ?.call(
            streamId.toString(),
            t_SubscribeState.ios_to_code($p_i.ByteRTCSubscribeState.values
                .firstWhere((t) => t.$value == state || t.name == state)),
            t_SubscribeStateChangeReason.ios_to_code($p_i
                .ByteRTCSubscribeStateChangeReason.values
                .firstWhere((t) => t.$value == reason || t.name == reason)));
  }

  FutureOr<void> rtcRoom$onLocalStreamStats$info$stats(
      dynamic rtcRoom, dynamic streamId, dynamic info, dynamic stats) async {
    if ($instance == null || $instance is! IRTCRoomEventHandler) {
      return;
    }
    return ($instance as IRTCRoomEventHandler).onLocalStreamStats?.call(
        streamId.toString(),
        packObject(
            info,
            () => StreamInfo.fromMap(StreamInfo.deepPackedMapValues(
                StreamInfo.mapMemberToConstructorParams(info)))),
        packObject(
            stats,
            () => LocalStreamStats.fromMap(LocalStreamStats.deepPackedMapValues(
                LocalStreamStats.mapMemberToConstructorParams(stats)))));
  }

  FutureOr<void> rtcRoom$onRemoteStreamStats$info$stats(
      dynamic rtcRoom, dynamic streamId, dynamic info, dynamic stats) async {
    if ($instance == null || $instance is! IRTCRoomEventHandler) {
      return;
    }
    return ($instance as IRTCRoomEventHandler).onRemoteStreamStats?.call(
        streamId.toString(),
        packObject(
            info,
            () => StreamInfo.fromMap(StreamInfo.deepPackedMapValues(
                StreamInfo.mapMemberToConstructorParams(info)))),
        packObject(
            stats,
            () => RemoteStreamStats.fromMap(
                RemoteStreamStats.deepPackedMapValues(
                    RemoteStreamStats.mapMemberToConstructorParams(stats)))));
  }

  FutureOr<void> rtcRoom$onStreamPublishSuccess$isScreen(
      dynamic rtcRoom, dynamic userId, dynamic isScreen) async {
    if ($instance == null || $instance is! IRTCRoomEventHandler) {
      return;
    }
    return ($instance as IRTCRoomEventHandler)
        .onStreamPublishSuccess
        ?.call(isScreen);
  }

  FutureOr<void> rtcRoom$onAVSyncEvent$userId$eventCode(dynamic rtcRoom,
      dynamic roomId, dynamic userId, dynamic eventCode) async {
    if ($instance == null || $instance is! IRTCRoomEventHandler) {
      return;
    }
    return ($instance as IRTCRoomEventHandler).onAVSyncEvent?.call(
        roomId.toString(),
        userId.toString(),
        t_AVSyncEvent.ios_to_code($p_i.ByteRTCAVSyncEvent.values
            .firstWhere((t) => t.$value == eventCode || t.name == eventCode)));
  }

  FutureOr<void> rtcRoom$onUserPublishStreamVideo$info$isPublish(
      dynamic rtcRoom,
      dynamic streamId,
      dynamic info,
      dynamic isPublish) async {
    if ($instance == null || $instance is! IRTCRoomEventHandler) {
      return;
    }
    return ($instance as IRTCRoomEventHandler).onUserPublishStreamVideo?.call(
        streamId.toString(),
        packObject(
            info,
            () => StreamInfo.fromMap(StreamInfo.deepPackedMapValues(
                StreamInfo.mapMemberToConstructorParams(info)))),
        isPublish);
  }

  FutureOr<void> rtcRoom$onUserPublishStreamAudio$info$isPublish(
      dynamic rtcRoom,
      dynamic streamId,
      dynamic info,
      dynamic isPublish) async {
    if ($instance == null || $instance is! IRTCRoomEventHandler) {
      return;
    }
    return ($instance as IRTCRoomEventHandler).onUserPublishStreamAudio?.call(
        streamId.toString(),
        packObject(
            info,
            () => StreamInfo.fromMap(StreamInfo.deepPackedMapValues(
                StreamInfo.mapMemberToConstructorParams(info)))),
        isPublish);
  }

  FutureOr<void> rtcRoom$onRoomMessageReceived$uid$message(
      dynamic rtcRoom, dynamic msgid, dynamic uid, dynamic message) async {
    if ($instance == null || $instance is! IRTCRoomEventHandler) {
      return;
    }
    return ($instance as IRTCRoomEventHandler).onRoomMessageReceived?.call(
        int.tryParse(msgid.toString()) ?? 0,
        uid.toString(),
        message.toString());
  }

  FutureOr<void> rtcRoom$onRoomBinaryMessageReceived$uid$message(
      dynamic rtcRoom, dynamic msgid, dynamic uid, dynamic message) async {
    if ($instance == null || $instance is! IRTCRoomEventHandler) {
      return;
    }
    return ($instance as IRTCRoomEventHandler)
        .onRoomBinaryMessageReceived
        ?.call(int.tryParse(msgid.toString()) ?? 0, uid.toString(), message);
  }

  FutureOr<void> rtcRoom$onUserMessageReceived$uid$message(
      dynamic rtcRoom, dynamic msgid, dynamic uid, dynamic message) async {
    if ($instance == null || $instance is! IRTCRoomEventHandler) {
      return;
    }
    return ($instance as IRTCRoomEventHandler).onUserMessageReceived?.call(
        int.tryParse(msgid.toString()) ?? 0,
        uid.toString(),
        message.toString());
  }

  FutureOr<void> rtcRoom$onUserBinaryMessageReceived$uid$message(
      dynamic rtcRoom, dynamic msgid, dynamic uid, dynamic message) async {
    if ($instance == null || $instance is! IRTCRoomEventHandler) {
      return;
    }
    return ($instance as IRTCRoomEventHandler)
        .onUserBinaryMessageReceived
        ?.call(int.tryParse(msgid.toString()) ?? 0, uid.toString(), message);
  }

  FutureOr<void> rtcRoom$onUserMessageSendResult$error(
      dynamic rtcRoom, dynamic msgid, dynamic error) async {
    if ($instance == null || $instance is! IRTCRoomEventHandler) {
      return;
    }
    return ($instance as IRTCRoomEventHandler).onUserMessageSendResult?.call(
        int.tryParse(msgid.toString()) ?? 0,
        t_UserMessageSendResult.ios_to_code($p_i
            .ByteRTCUserMessageSendResult.values
            .firstWhere((t) => t.$value == error || t.name == error)));
  }

  FutureOr<void> rtcRoom$onRoomMessageSendResult$error(
      dynamic rtcRoom, dynamic msgid, dynamic error) async {
    if ($instance == null || $instance is! IRTCRoomEventHandler) {
      return;
    }
    return ($instance as IRTCRoomEventHandler).onRoomMessageSendResult?.call(
        int.tryParse(msgid.toString()) ?? 0,
        t_RoomMessageSendResult.ios_to_code($p_i
            .ByteRTCRoomMessageSendResult.values
            .firstWhere((t) => t.$value == error || t.name == error)));
  }

  FutureOr<void> rtcRoom$onVideoStreamBanned$isBanned(
      dynamic rtcRoom, dynamic uid, dynamic banned) async {
    if ($instance == null || $instance is! IRTCRoomEventHandler) {
      return;
    }
    return ($instance as IRTCRoomEventHandler)
        .onVideoStreamBanned
        ?.call(uid.toString(), banned);
  }

  FutureOr<void> rtcRoom$onAudioStreamBanned$isBanned(
      dynamic rtcRoom, dynamic uid, dynamic banned) async {
    if ($instance == null || $instance is! IRTCRoomEventHandler) {
      return;
    }
    return ($instance as IRTCRoomEventHandler)
        .onAudioStreamBanned
        ?.call(uid.toString(), banned);
  }

  FutureOr<void> rtcRoom$onForwardStreamStateChanged(
      dynamic rtcRoom, List<dynamic> infos) async {
    if ($instance == null || $instance is! IRTCRoomEventHandler) {
      return;
    }
    return ($instance as IRTCRoomEventHandler)
        .onForwardStreamStateChanged
        ?.call(infos
            .map((e) => packObject(
                e,
                () => ForwardStreamStateInfo.fromMap(
                    ForwardStreamStateInfo.deepPackedMapValues(
                        ForwardStreamStateInfo.mapMemberToConstructorParams(
                            e)))))
            .toList());
  }

  FutureOr<void> rtcRoom$onForwardStreamEvent(
      dynamic rtcRoom, List<dynamic> infos) async {
    if ($instance == null || $instance is! IRTCRoomEventHandler) {
      return;
    }
    return ($instance as IRTCRoomEventHandler).onForwardStreamEvent?.call(
        rtcRoom
            .map((e) => packObject(
                e,
                () => ForwardStreamEventInfo.fromMap(
                    ForwardStreamEventInfo.deepPackedMapValues(
                        ForwardStreamEventInfo.mapMemberToConstructorParams(
                            e)))))
            .toList());
  }

  FutureOr<void> rtcRoom$onNetworkQuality$remoteQualities(dynamic rtcRoom,
      dynamic localQuality, List<dynamic> remoteQualities) async {
    if ($instance == null || $instance is! IRTCRoomEventHandler) {
      return;
    }
    return ($instance as IRTCRoomEventHandler).onNetworkQuality?.call(
        packObject(
            localQuality,
            () => NetworkQualityStats.fromMap(
                NetworkQualityStats.deepPackedMapValues(
                    NetworkQualityStats.mapMemberToConstructorParams(
                        localQuality)))),
        remoteQualities
            .map((e) => packObject(
                e,
                () => NetworkQualityStats.fromMap(
                    NetworkQualityStats.deepPackedMapValues(
                        NetworkQualityStats.mapMemberToConstructorParams(e)))))
            .toList());
  }

  FutureOr<void> rtcRoom$onSetRoomExtraInfoResult$result(
      dynamic rtcRoom, dynamic taskId, dynamic result) async {
    if ($instance == null || $instance is! IRTCRoomEventHandler) {
      return;
    }
    return ($instance as IRTCRoomEventHandler).onSetRoomExtraInfoResult?.call(
        int.tryParse(taskId.toString()) ?? 0,
        t_SetRoomExtraInfoResult.ios_to_code($p_i
            .ByteRTCSetRoomExtraInfoResult.values
            .firstWhere((t) => t.$value == result || t.name == result)));
  }

  FutureOr<void>
      rtcRoom$onRoomExtraInfoUpdate$value$lastUpdateUserId$lastUpdateTimeMs(
          dynamic rtcRoom,
          dynamic key,
          dynamic value,
          dynamic lastUpdateUserId,
          dynamic lastUpdateTimeMs) async {
    if ($instance == null || $instance is! IRTCRoomEventHandler) {
      return;
    }
    return ($instance as IRTCRoomEventHandler).onRoomExtraInfoUpdate?.call(
        key.toString(),
        value.toString(),
        lastUpdateUserId.toString(),
        int.tryParse(lastUpdateTimeMs.toString()) ?? 0);
  }

  FutureOr<void> rtcRoom$onRoomStreamExtraInfoUpdate$info$extraInfo(
      dynamic rtcRoom,
      dynamic streamId,
      dynamic streamInfo,
      dynamic extraInfo) async {
    if ($instance == null || $instance is! IRTCRoomEventHandler) {
      return;
    }
    return ($instance as IRTCRoomEventHandler)
        .onRoomStreamExtraInfoUpdate
        ?.call(
            streamId.toString(),
            packObject(
                streamInfo,
                () => StreamInfo.fromMap(StreamInfo.deepPackedMapValues(
                    StreamInfo.mapMemberToConstructorParams(streamInfo)))),
            extraInfo.toString());
  }

  FutureOr<void> rtcRoom$onUserVisibilityChanged$errorCode(
      dynamic rtcRoom, dynamic currentUserVisibility, dynamic errorCode) async {
    if ($instance == null || $instance is! IRTCRoomEventHandler) {
      return;
    }
    return ($instance as IRTCRoomEventHandler).onUserVisibilityChanged?.call(
        currentUserVisibility,
        t_UserVisibilityChangeError.ios_to_code($p_i
            .ByteRTCUserVisibilityChangeError.values
            .firstWhere((t) => t.$value == errorCode || t.name == errorCode)));
  }

  FutureOr<void> rtcRoom$onSubtitleStateChanged$errorCode$errorMessage(
      dynamic rtcRoom,
      dynamic state,
      dynamic errorCode,
      dynamic errorMessage) async {
    if ($instance == null || $instance is! IRTCRoomEventHandler) {
      return;
    }
    return ($instance as IRTCRoomEventHandler).onSubtitleStateChanged?.call(
        t_SubtitleState.ios_to_code($p_i.ByteRTCSubtitleState.values
            .firstWhere((t) => t.$value == state || t.name == state)),
        t_SubtitleErrorCode.ios_to_code($p_i.ByteRTCSubtitleErrorCode.values
            .firstWhere((t) => t.$value == errorCode || t.name == errorCode)),
        errorMessage.toString());
  }

  FutureOr<void> rtcRoom$onSubtitleMessageReceived(
      dynamic rtcRoom, List<dynamic> subtitles) async {
    if ($instance == null || $instance is! IRTCRoomEventHandler) {
      return;
    }
    return ($instance as IRTCRoomEventHandler).onSubtitleMessageReceived?.call(
        subtitles
            .map((e) => packObject(
                e,
                () => SubtitleMessage.fromMap(
                    SubtitleMessage.deepPackedMapValues(
                        SubtitleMessage.mapMemberToConstructorParams(e)))))
            .toList());
  }

  FutureOr<void> rtcRoom$onRoomWarning(
      dynamic rtcRoom, dynamic warningCode) async {
    if ($instance == null || $instance is! IRTCRoomEventHandler) {
      return;
    }
    return ($instance as IRTCRoomEventHandler).onRoomWarning?.call(
        packObject(rtcRoom, () => RTCRoom()),
        t_WarningCode.ios_to_code($p_i.ByteRTCWarningCode.values.firstWhere(
            (t) => t.$value == warningCode || t.name == warningCode)));
  }

  FutureOr<void> onTokenWillExpire(dynamic rtcRoom) async {
    if ($instance == null || $instance is! IRTCRoomEventHandler) {
      return;
    }
    return ($instance as IRTCRoomEventHandler).onTokenWillExpire?.call();
  }

  FutureOr<void> onPublishPrivilegeTokenWillExpire(dynamic rtcRoom) async {
    if ($instance == null || $instance is! IRTCRoomEventHandler) {
      return;
    }
    return ($instance as IRTCRoomEventHandler)
        .onPublishPrivilegeTokenWillExpire
        ?.call();
  }

  FutureOr<void> onSubscribePrivilegeTokenWillExpire(dynamic rtcRoom) async {
    if ($instance == null || $instance is! IRTCRoomEventHandler) {
      return;
    }
    return ($instance as IRTCRoomEventHandler)
        .onSubscribePrivilegeTokenWillExpire
        ?.call();
  }
}

class android_IRTSRoomEventHandler extends $p_a.IRTSRoomEventHandler {
  android_IRTSRoomEventHandler();

  FutureOr<void> onLeaveRoom(dynamic stats) async {
    if ($instance == null || $instance is! IRTSRoomEventHandler) {
      return;
    }
    return ($instance as IRTSRoomEventHandler).onLeaveRoom?.call(packObject(
        stats,
        () => RTCRoomStats.fromMap(RTCRoomStats.deepPackedMapValues(
            RTCRoomStats.mapMemberToConstructorParams(stats)))));
  }

  FutureOr<void> onRoomStateChanged(
      dynamic roomId, dynamic uid, dynamic state, dynamic extraInfo) async {
    if ($instance == null || $instance is! IRTSRoomEventHandler) {
      return;
    }
    return ($instance as IRTSRoomEventHandler).onRoomStateChanged?.call(
        roomId.toString(),
        uid.toString(),
        int.tryParse(state.toString()) ?? 0,
        extraInfo.toString());
  }

  FutureOr<void> onUserJoined(dynamic userInfo) async {
    if ($instance == null || $instance is! IRTSRoomEventHandler) {
      return;
    }
    return ($instance as IRTSRoomEventHandler).onUserJoined?.call(packObject(
        userInfo,
        () => UserInfo.fromMap(UserInfo.deepPackedMapValues(
            UserInfo.mapMemberToConstructorParams(userInfo)))));
  }

  FutureOr<void> onUserLeave(dynamic uid, dynamic reason) async {
    if ($instance == null || $instance is! IRTSRoomEventHandler) {
      return;
    }
    return ($instance as IRTSRoomEventHandler)
        .onUserLeave
        ?.call(uid.toString(), int.tryParse(reason.toString()) ?? 0);
  }

  FutureOr<void> onRoomMessageReceived(
      dynamic msgid, dynamic uid, dynamic message) async {
    if ($instance == null || $instance is! IRTSRoomEventHandler) {
      return;
    }
    return ($instance as IRTSRoomEventHandler).onRoomMessageReceived?.call(
        int.tryParse(msgid.toString()) ?? 0,
        uid.toString(),
        message.toString());
  }

  FutureOr<void> onRoomBinaryMessageReceived(
      dynamic msgid, dynamic uid, dynamic message) async {
    if ($instance == null || $instance is! IRTSRoomEventHandler) {
      return;
    }
    return ($instance as IRTSRoomEventHandler)
        .onRoomBinaryMessageReceived
        ?.call(int.tryParse(msgid.toString()) ?? 0, uid.toString(), message);
  }

  FutureOr<void> onUserMessageReceived(
      dynamic msgid, dynamic uid, dynamic message) async {
    if ($instance == null || $instance is! IRTSRoomEventHandler) {
      return;
    }
    return ($instance as IRTSRoomEventHandler).onUserMessageReceived?.call(
        int.tryParse(msgid.toString()) ?? 0,
        uid.toString(),
        message.toString());
  }

  FutureOr<void> onUserBinaryMessageReceived(
      dynamic msgid, dynamic uid, dynamic message) async {
    if ($instance == null || $instance is! IRTSRoomEventHandler) {
      return;
    }
    return ($instance as IRTSRoomEventHandler)
        .onUserBinaryMessageReceived
        ?.call(int.tryParse(msgid.toString()) ?? 0, uid.toString(), message);
  }

  FutureOr<void> onUserMessageSendResult(dynamic msgid, dynamic error) async {
    if ($instance == null || $instance is! IRTSRoomEventHandler) {
      return;
    }
    return ($instance as IRTSRoomEventHandler).onUserMessageSendResult?.call(
        int.tryParse(msgid.toString()) ?? 0,
        int.tryParse(error.toString()) ?? 0);
  }

  FutureOr<void> onRoomMessageSendResult(dynamic msgid, dynamic error) async {
    if ($instance == null || $instance is! IRTSRoomEventHandler) {
      return;
    }
    return ($instance as IRTSRoomEventHandler).onRoomMessageSendResult?.call(
        int.tryParse(msgid.toString()) ?? 0,
        int.tryParse(error.toString()) ?? 0);
  }
}

class ios_IRTSRoomEventHandler extends $p_i.ByteRTCRTSRoomDelegate {
  ios_IRTSRoomEventHandler();
}

class android_IRemoteEncodedVideoFrameObserver
    extends $p_a.IRemoteEncodedVideoFrameObserver {
  android_IRemoteEncodedVideoFrameObserver();

  FutureOr<void> onRemoteEncodedVideoFrame(
      dynamic streamId, dynamic streamInfo, dynamic encodedVideoFrame) async {
    if ($instance == null || $instance is! IRemoteEncodedVideoFrameObserver) {
      return;
    }
    return ($instance as IRemoteEncodedVideoFrameObserver)
        .onRemoteEncodedVideoFrame
        ?.call(streamId.toString());
  }
}

class ios_IRemoteEncodedVideoFrameObserver
    extends $p_i.ByteRTCRemoteEncodedVideoFrameObserver {
  ios_IRemoteEncodedVideoFrameObserver();

  FutureOr<void> onRemoteEncodedVideoFrame$info$withEncodedVideoFrame(
      dynamic streamId, dynamic info, dynamic videoFrame) async {
    if ($instance == null || $instance is! IRemoteEncodedVideoFrameObserver) {
      return;
    }
    return ($instance as IRemoteEncodedVideoFrameObserver)
        .onRemoteEncodedVideoFrame
        ?.call(streamId.toString());
  }
}

class android_IAudioEffectPlayerEventHandler
    extends $p_a.IAudioEffectPlayerEventHandler {
  android_IAudioEffectPlayerEventHandler();

  FutureOr<void> onAudioEffectPlayerStateChanged(
      dynamic effectId, dynamic state, dynamic error) async {
    if ($instance == null || $instance is! IAudioEffectPlayerEventHandler) {
      return;
    }
    return ($instance as IAudioEffectPlayerEventHandler)
        .onAudioEffectPlayerStateChanged
        ?.call(
            int.tryParse(effectId.toString()) ?? 0,
            t_PlayerState.android_to_code($p_a.PlayerState.values
                .firstWhere((t) => t.$value == state || t.name == state)),
            t_PlayerError.android_to_code($p_a.PlayerError.values
                .firstWhere((t) => t.$value == error || t.name == error)));
  }
}

class ios_IAudioEffectPlayerEventHandler
    extends $p_i.ByteRTCAudioEffectPlayerEventHandler {
  ios_IAudioEffectPlayerEventHandler();

  FutureOr<void> onAudioEffectPlayerStateChanged$state$error(
      dynamic effectId, dynamic state, dynamic error) async {
    if ($instance == null || $instance is! IAudioEffectPlayerEventHandler) {
      return;
    }
    return ($instance as IAudioEffectPlayerEventHandler)
        .onAudioEffectPlayerStateChanged
        ?.call(
            int.tryParse(effectId.toString()) ?? 0,
            t_PlayerState.ios_to_code($p_i.ByteRTCPlayerState.values
                .firstWhere((t) => t.$value == state || t.name == state)),
            t_PlayerError.ios_to_code($p_i.ByteRTCPlayerError.values
                .firstWhere((t) => t.$value == error || t.name == error)));
  }
}

class android_IWTNStreamEventHandler extends $p_a.IWTNStreamEventHandler {
  android_IWTNStreamEventHandler();

  FutureOr<void> onWTNRemoteVideoStats(dynamic streamId, dynamic stats) async {
    if ($instance == null || $instance is! IWTNStreamEventHandler) {
      return;
    }
    return ($instance as IWTNStreamEventHandler).onWTNRemoteVideoStats?.call(
        streamId.toString(),
        packObject(
            stats,
            () => RemoteVideoStats.fromMap(RemoteVideoStats.deepPackedMapValues(
                RemoteVideoStats.mapMemberToConstructorParams(stats)))));
  }

  FutureOr<void> onWTNRemoteAudioStats(dynamic streamId, dynamic stats) async {
    if ($instance == null || $instance is! IWTNStreamEventHandler) {
      return;
    }
    return ($instance as IWTNStreamEventHandler).onWTNRemoteAudioStats?.call(
        streamId.toString(),
        packObject(
            stats,
            () => RemoteAudioStats.fromMap(RemoteAudioStats.deepPackedMapValues(
                RemoteAudioStats.mapMemberToConstructorParams(stats)))));
  }

  FutureOr<void> onWTNVideoSubscribeStateChanged(
      dynamic streamId, dynamic stateCode, dynamic reason) async {
    if ($instance == null || $instance is! IWTNStreamEventHandler) {
      return;
    }
    return ($instance as IWTNStreamEventHandler)
        .onWTNVideoSubscribeStateChanged
        ?.call(
            streamId.toString(),
            t_WTNSubscribeState.android_to_code($p_a.WTNSubscribeState.values
                .firstWhere(
                    (t) => t.$value == stateCode || t.name == stateCode)),
            t_WTNSubscribeStateChangeReason.android_to_code($p_a
                .WTNSubscribeStateChangeReason.values
                .firstWhere((t) => t.$value == reason || t.name == reason)));
  }

  FutureOr<void> onWTNAudioSubscribeStateChanged(
      dynamic streamId, dynamic stateCode, dynamic reason) async {
    if ($instance == null || $instance is! IWTNStreamEventHandler) {
      return;
    }
    return ($instance as IWTNStreamEventHandler)
        .onWTNAudioSubscribeStateChanged
        ?.call(
            streamId.toString(),
            t_WTNSubscribeState.android_to_code($p_a.WTNSubscribeState.values
                .firstWhere(
                    (t) => t.$value == stateCode || t.name == stateCode)),
            t_WTNSubscribeStateChangeReason.android_to_code($p_a
                .WTNSubscribeStateChangeReason.values
                .firstWhere((t) => t.$value == reason || t.name == reason)));
  }

  FutureOr<void> onWTNFirstRemoteAudioFrame(dynamic streamId) async {
    if ($instance == null || $instance is! IWTNStreamEventHandler) {
      return;
    }
    return ($instance as IWTNStreamEventHandler)
        .onWTNFirstRemoteAudioFrame
        ?.call(streamId.toString());
  }

  FutureOr<void> onWTNFirstRemoteVideoFrameDecoded(
      dynamic streamId, dynamic info) async {
    if ($instance == null || $instance is! IWTNStreamEventHandler) {
      return;
    }
    return ($instance as IWTNStreamEventHandler)
        .onWTNFirstRemoteVideoFrameDecoded
        ?.call(
            streamId.toString(),
            packObject(
                info,
                () => VideoFrameInfo.fromMap(VideoFrameInfo.deepPackedMapValues(
                    VideoFrameInfo.mapMemberToConstructorParams(info)))));
  }

  FutureOr<void> onWTNSEIMessageReceived(
      dynamic streamId, dynamic channelId, dynamic message) async {
    if ($instance == null || $instance is! IWTNStreamEventHandler) {
      return;
    }
    return ($instance as IWTNStreamEventHandler).onWTNSEIMessageReceived?.call(
        streamId.toString(), int.tryParse(channelId.toString()) ?? 0, message);
  }

  FutureOr<void> onWTNDataMessageReceived(
      dynamic streamId, dynamic message, dynamic sourceType) async {
    if ($instance == null || $instance is! IWTNStreamEventHandler) {
      return;
    }
    return ($instance as IWTNStreamEventHandler).onWTNDataMessageReceived?.call(
        streamId.toString(),
        message,
        t_DataMessageSourceType.android_to_code(
            $p_a.DataMessageSourceType.values.firstWhere(
                (t) => t.$value == sourceType || t.name == sourceType)));
  }
}

class ios_IWTNStreamEventHandler extends $p_i.ByteRTCWTNStreamDelegate {
  ios_IWTNStreamEventHandler();

  FutureOr<void> onWTNRemoteVideoStats$videoStats(
      dynamic streamId, dynamic videoStats) async {
    if ($instance == null || $instance is! IWTNStreamEventHandler) {
      return;
    }
    return ($instance as IWTNStreamEventHandler).onWTNRemoteVideoStats?.call(
        streamId.toString(),
        packObject(
            videoStats,
            () => RemoteVideoStats.fromMap(RemoteVideoStats.deepPackedMapValues(
                RemoteVideoStats.mapMemberToConstructorParams(videoStats)))));
  }

  FutureOr<void> onWTNRemoteAudioStats$audioStats(
      dynamic streamId, dynamic audioStats) async {
    if ($instance == null || $instance is! IWTNStreamEventHandler) {
      return;
    }
    return ($instance as IWTNStreamEventHandler).onWTNRemoteAudioStats?.call(
        streamId.toString(),
        packObject(
            audioStats,
            () => RemoteAudioStats.fromMap(RemoteAudioStats.deepPackedMapValues(
                RemoteAudioStats.mapMemberToConstructorParams(audioStats)))));
  }

  FutureOr<void> onWTNVideoSubscribeStateChanged$state$reason(
      dynamic streamId, dynamic state, dynamic reason) async {
    if ($instance == null || $instance is! IWTNStreamEventHandler) {
      return;
    }
    return ($instance as IWTNStreamEventHandler)
        .onWTNVideoSubscribeStateChanged
        ?.call(
            streamId.toString(),
            t_WTNSubscribeState.ios_to_code($p_i.ByteRTCWTNSubscribeState.values
                .firstWhere((t) => t.$value == state || t.name == state)),
            t_WTNSubscribeStateChangeReason.ios_to_code($p_i
                .ByteRTCWTNSubscribeStateChangeReason.values
                .firstWhere((t) => t.$value == reason || t.name == reason)));
  }

  FutureOr<void> onWTNAudioSubscribeStateChanged$state$reason(
      dynamic streamId, dynamic state, dynamic reason) async {
    if ($instance == null || $instance is! IWTNStreamEventHandler) {
      return;
    }
    return ($instance as IWTNStreamEventHandler)
        .onWTNAudioSubscribeStateChanged
        ?.call(
            streamId.toString(),
            t_WTNSubscribeState.ios_to_code($p_i.ByteRTCWTNSubscribeState.values
                .firstWhere((t) => t.$value == state || t.name == state)),
            t_WTNSubscribeStateChangeReason.ios_to_code($p_i
                .ByteRTCWTNSubscribeStateChangeReason.values
                .firstWhere((t) => t.$value == reason || t.name == reason)));
  }

  FutureOr<void> onWTNFirstRemoteAudioFrame(dynamic streamId) async {
    if ($instance == null || $instance is! IWTNStreamEventHandler) {
      return;
    }
    return ($instance as IWTNStreamEventHandler)
        .onWTNFirstRemoteAudioFrame
        ?.call(streamId.toString());
  }

  FutureOr<void> onWTNFirstRemoteVideoFrameDecoded$withFrameInfo(
      dynamic streamId, dynamic frameInfo) async {
    if ($instance == null || $instance is! IWTNStreamEventHandler) {
      return;
    }
    return ($instance as IWTNStreamEventHandler)
        .onWTNFirstRemoteVideoFrameDecoded
        ?.call(
            streamId.toString(),
            packObject(
                frameInfo,
                () => VideoFrameInfo.fromMap(VideoFrameInfo.deepPackedMapValues(
                    VideoFrameInfo.mapMemberToConstructorParams(frameInfo)))));
  }

  FutureOr<void> onWTNSEIMessageReceived$andChannelId$andMessage(
      dynamic streamId, dynamic channelId, dynamic message) async {
    if ($instance == null || $instance is! IWTNStreamEventHandler) {
      return;
    }
    return ($instance as IWTNStreamEventHandler).onWTNSEIMessageReceived?.call(
        streamId.toString(), int.tryParse(channelId.toString()) ?? 0, message);
  }

  FutureOr<void> onWTNDataMessageReceived$andMessage$andSourceType(
      dynamic streamId, dynamic message, dynamic sourceType) async {
    if ($instance == null || $instance is! IWTNStreamEventHandler) {
      return;
    }
    return ($instance as IWTNStreamEventHandler).onWTNDataMessageReceived?.call(
        streamId.toString(),
        message,
        t_DataMessageSourceType.ios_to_code(
            $p_i.ByteRTCDataMessageSourceType.values.firstWhere(
                (t) => t.$value == sourceType || t.name == sourceType)));
  }
}

class android_IMediaPlayerEventHandler extends $p_a.IMediaPlayerEventHandler {
  android_IMediaPlayerEventHandler();

  FutureOr<void> onMediaPlayerStateChanged(
      dynamic playerId, dynamic state, dynamic error) async {
    if ($instance == null || $instance is! IMediaPlayerEventHandler) {
      return;
    }
    return ($instance as IMediaPlayerEventHandler)
        .onMediaPlayerStateChanged
        ?.call(
            int.tryParse(playerId.toString()) ?? 0,
            t_PlayerState.android_to_code($p_a.PlayerState.values
                .firstWhere((t) => t.$value == state || t.name == state)),
            t_PlayerError.android_to_code($p_a.PlayerError.values
                .firstWhere((t) => t.$value == error || t.name == error)));
  }

  FutureOr<void> onMediaPlayerPlayingProgress(
      dynamic playerId, dynamic progress) async {
    if ($instance == null || $instance is! IMediaPlayerEventHandler) {
      return;
    }
    return ($instance as IMediaPlayerEventHandler)
        .onMediaPlayerPlayingProgress
        ?.call(int.tryParse(playerId.toString()) ?? 0,
            int.tryParse(progress.toString()) ?? 0);
  }

  FutureOr<void> onMediaPlayerEvent(
      dynamic playerId, dynamic event, dynamic message) async {
    if ($instance == null || $instance is! IMediaPlayerEventHandler) {
      return;
    }
    return ($instance as IMediaPlayerEventHandler).onMediaPlayerEvent?.call(
        int.tryParse(playerId.toString()) ?? 0,
        t_PlayerEvent.android_to_code($p_a.PlayerEvent.values
            .firstWhere((t) => t.$value == event || t.name == event)),
        message.toString());
  }
}

class ios_IMediaPlayerEventHandler extends $p_i.ByteRTCMediaPlayerEventHandler {
  ios_IMediaPlayerEventHandler();

  FutureOr<void> onMediaPlayerStateChanged$state$error(
      dynamic playerId, dynamic state, dynamic error) async {
    if ($instance == null || $instance is! IMediaPlayerEventHandler) {
      return;
    }
    return ($instance as IMediaPlayerEventHandler)
        .onMediaPlayerStateChanged
        ?.call(
            int.tryParse(playerId.toString()) ?? 0,
            t_PlayerState.ios_to_code($p_i.ByteRTCPlayerState.values
                .firstWhere((t) => t.$value == state || t.name == state)),
            t_PlayerError.ios_to_code($p_i.ByteRTCPlayerError.values
                .firstWhere((t) => t.$value == error || t.name == error)));
  }

  FutureOr<void> onMediaPlayerPlayingProgress$progress(
      dynamic playerId, dynamic progress) async {
    if ($instance == null || $instance is! IMediaPlayerEventHandler) {
      return;
    }
    return ($instance as IMediaPlayerEventHandler)
        .onMediaPlayerPlayingProgress
        ?.call(int.tryParse(playerId.toString()) ?? 0,
            int.tryParse(progress.toString()) ?? 0);
  }

  FutureOr<void> onMediaPlayerEvent$event$message(
      dynamic playerId, dynamic event, dynamic message) async {
    if ($instance == null || $instance is! IMediaPlayerEventHandler) {
      return;
    }
    return ($instance as IMediaPlayerEventHandler).onMediaPlayerEvent?.call(
        int.tryParse(playerId.toString()) ?? 0,
        t_PlayerEvent.ios_to_code($p_i.ByteRTCPlayerEvent.values
            .firstWhere((t) => t.$value == event || t.name == event)),
        message.toString());
  }
}

class android_IRTCEncryptionHandler extends $p_a.IRTCEncryptionHandler {
  android_IRTCEncryptionHandler();
}

class ios_IRTCEncryptionHandler extends $p_i.ByteRTCEncryptHandler {
  ios_IRTCEncryptionHandler();
}

class android_IFaceDetectionObserver extends $p_a.IFaceDetectionObserver {
  android_IFaceDetectionObserver();

  FutureOr<void> onFaceDetectResult(dynamic result) async {
    if ($instance == null || $instance is! IFaceDetectionObserver) {
      return;
    }
    return ($instance as IFaceDetectionObserver).onFaceDetectResult?.call(
        packObject(
            result,
            () => FaceDetectionResult.fromMap(
                FaceDetectionResult.deepPackedMapValues(
                    FaceDetectionResult.mapMemberToConstructorParams(
                        result)))));
  }

  FutureOr<void> onExpressionDetectResult(dynamic result) async {
    if ($instance == null || $instance is! IFaceDetectionObserver) {
      return;
    }
    return ($instance as IFaceDetectionObserver).onExpressionDetectResult?.call(
        packObject(
            result,
            () => ExpressionDetectResult.fromMap(
                ExpressionDetectResult.deepPackedMapValues(
                    ExpressionDetectResult.mapMemberToConstructorParams(
                        result)))));
  }
}

class ios_IFaceDetectionObserver extends $p_i.ByteRTCFaceDetectionObserver {
  ios_IFaceDetectionObserver();

  FutureOr<void> onFaceDetectResult(dynamic result) async {
    if ($instance == null || $instance is! IFaceDetectionObserver) {
      return;
    }
    return ($instance as IFaceDetectionObserver).onFaceDetectResult?.call(
        packObject(
            result,
            () => FaceDetectionResult.fromMap(
                FaceDetectionResult.deepPackedMapValues(
                    FaceDetectionResult.mapMemberToConstructorParams(
                        result)))));
  }

  FutureOr<void> onExpressionDetectResult(dynamic result) async {
    if ($instance == null || $instance is! IFaceDetectionObserver) {
      return;
    }
    return ($instance as IFaceDetectionObserver).onExpressionDetectResult?.call(
        packObject(
            result,
            () => ExpressionDetectResult.fromMap(
                ExpressionDetectResult.deepPackedMapValues(
                    ExpressionDetectResult.mapMemberToConstructorParams(
                        result)))));
  }
}

class android_IMediaPlayerCustomSourceProvider
    extends $p_a.IMediaPlayerCustomSourceProvider {
  android_IMediaPlayerCustomSourceProvider();
}

class ios_IMediaPlayerCustomSourceProvider
    extends $p_i.ByteRTCMediaPlayerCustomSourceProvider {
  ios_IMediaPlayerCustomSourceProvider();
}

class android_IExternalVideoEncoderEventHandler
    extends $p_a.IExternalVideoEncoderEventHandler {
  android_IExternalVideoEncoderEventHandler();

  FutureOr<void> onStart(dynamic streamId, dynamic streamInfo) async {
    if ($instance == null || $instance is! IExternalVideoEncoderEventHandler) {
      return;
    }
    return ($instance as IExternalVideoEncoderEventHandler)
        .onStart
        ?.call(streamId.toString());
  }

  FutureOr<void> onStop(dynamic streamId, dynamic streamInfo) async {
    if ($instance == null || $instance is! IExternalVideoEncoderEventHandler) {
      return;
    }
    return ($instance as IExternalVideoEncoderEventHandler)
        .onStop
        ?.call(streamId.toString());
  }

  FutureOr<void> onRateUpdate(dynamic streamId, dynamic streamInfo,
      dynamic videoIndex, dynamic fps, dynamic bitrateKbps) async {
    if ($instance == null || $instance is! IExternalVideoEncoderEventHandler) {
      return;
    }
    return ($instance as IExternalVideoEncoderEventHandler).onRateUpdate?.call(
        streamId.toString(),
        int.tryParse(videoIndex.toString()) ?? 0,
        int.tryParse(fps.toString()) ?? 0,
        int.tryParse(bitrateKbps.toString()) ?? 0);
  }

  FutureOr<void> onRequestKeyFrame(
      dynamic streamId, dynamic streamInfo, dynamic videoIndex) async {
    if ($instance == null || $instance is! IExternalVideoEncoderEventHandler) {
      return;
    }
    return ($instance as IExternalVideoEncoderEventHandler)
        .onRequestKeyFrame
        ?.call(streamId.toString(), int.tryParse(videoIndex.toString()) ?? 0);
  }

  FutureOr<void> onActiveVideoLayer(dynamic streamId, dynamic streamInfo,
      dynamic videoIndex, dynamic active) async {
    if ($instance == null || $instance is! IExternalVideoEncoderEventHandler) {
      return;
    }
    return ($instance as IExternalVideoEncoderEventHandler)
        .onActiveVideoLayer
        ?.call(streamId.toString(), int.tryParse(videoIndex.toString()) ?? 0,
            active);
  }
}

class ios_IExternalVideoEncoderEventHandler
    extends $p_i.ByteRTCExternalVideoEncoderEventHandler {
  ios_IExternalVideoEncoderEventHandler();

  FutureOr<void> onStart$info(dynamic streamId, dynamic info) async {
    if ($instance == null || $instance is! IExternalVideoEncoderEventHandler) {
      return;
    }
    return ($instance as IExternalVideoEncoderEventHandler)
        .onStart
        ?.call(streamId.toString());
  }

  FutureOr<void> onStop$info(dynamic streamId, dynamic info) async {
    if ($instance == null || $instance is! IExternalVideoEncoderEventHandler) {
      return;
    }
    return ($instance as IExternalVideoEncoderEventHandler)
        .onStop
        ?.call(streamId.toString());
  }

  FutureOr<void> onRateUpdate$info$withVideoIndex$withFps$withBitRate(
      dynamic streamId,
      dynamic info,
      dynamic videoIndex,
      dynamic fps,
      dynamic bitRateKps) async {
    if ($instance == null || $instance is! IExternalVideoEncoderEventHandler) {
      return;
    }
    return ($instance as IExternalVideoEncoderEventHandler).onRateUpdate?.call(
        streamId.toString(),
        int.tryParse(videoIndex.toString()) ?? 0,
        int.tryParse(fps.toString()) ?? 0,
        int.tryParse(bitRateKps.toString()) ?? 0);
  }

  FutureOr<void> onRequestKeyFrame$info$withVideoIndex(
      dynamic streamId, dynamic info, dynamic videoIndex) async {
    if ($instance == null || $instance is! IExternalVideoEncoderEventHandler) {
      return;
    }
    return ($instance as IExternalVideoEncoderEventHandler)
        .onRequestKeyFrame
        ?.call(streamId.toString(), int.tryParse(videoIndex.toString()) ?? 0);
  }

  FutureOr<void> onActiveVideoLayer$info$withVideoIndex$withActive(
      dynamic streamId,
      dynamic info,
      dynamic videoIndex,
      dynamic active) async {
    if ($instance == null || $instance is! IExternalVideoEncoderEventHandler) {
      return;
    }
    return ($instance as IExternalVideoEncoderEventHandler)
        .onActiveVideoLayer
        ?.call(streamId.toString(), int.tryParse(videoIndex.toString()) ?? 0,
            active);
  }
}

class android_ILocalEncodedVideoFrameObserver
    extends $p_a.ILocalEncodedVideoFrameObserver {
  android_ILocalEncodedVideoFrameObserver();

  FutureOr<void> onLocalEncodedVideoFrame(
      dynamic videoSource, dynamic encodedVideoFrame) async {
    if ($instance == null || $instance is! ILocalEncodedVideoFrameObserver) {
      return;
    }
    return ($instance as ILocalEncodedVideoFrameObserver)
        .onLocalEncodedVideoFrame
        ?.call(videoSource);
  }
}

class ios_ILocalEncodedVideoFrameObserver
    extends $p_i.ByteRTCLocalEncodedVideoFrameObserver {
  ios_ILocalEncodedVideoFrameObserver();

  FutureOr<void> onLocalEncodedVideoFrame$Frame(
      dynamic videoSource, dynamic frame) async {
    if ($instance == null || $instance is! ILocalEncodedVideoFrameObserver) {
      return;
    }
    return ($instance as ILocalEncodedVideoFrameObserver)
        .onLocalEncodedVideoFrame
        ?.call(videoSource);
  }
}

class android_IRTCEngineEventHandler extends $p_a.IRTCEngineEventHandler {
  android_IRTCEngineEventHandler();

  FutureOr<void> onWarning(dynamic warn) async {
    if ($instance == null || $instance is! IRTCEngineEventHandler) {
      return;
    }
    return ($instance as IRTCEngineEventHandler).onWarning?.call(
        t_WarningCode.android_to_code($p_a.WarningCode.values
            .firstWhere((t) => t.$value == warn || t.name == warn)));
  }

  FutureOr<void> onError(dynamic err) async {
    if ($instance == null || $instance is! IRTCEngineEventHandler) {
      return;
    }
    return ($instance as IRTCEngineEventHandler).onError?.call(
        t_ErrorCode.android_to_code($p_a.ErrorCode.values
            .firstWhere((t) => t.$value == err || t.name == err)));
  }

  FutureOr<void> onExtensionAccessError(
      dynamic extensionName, dynamic msg) async {
    if ($instance == null || $instance is! IRTCEngineEventHandler) {
      return;
    }
    return ($instance as IRTCEngineEventHandler)
        .onExtensionAccessError
        ?.call(extensionName.toString(), msg.toString());
  }

  FutureOr<void> onSysStats(dynamic stats) async {
    if ($instance == null || $instance is! IRTCEngineEventHandler) {
      return;
    }
    return ($instance as IRTCEngineEventHandler).onSysStats?.call(packObject(
        stats,
        () => SysStats.fromMap(SysStats.deepPackedMapValues(
            SysStats.mapMemberToConstructorParams(stats)))));
  }

  FutureOr<void> onNetworkTypeChanged(dynamic type) async {
    if ($instance == null || $instance is! IRTCEngineEventHandler) {
      return;
    }
    return ($instance as IRTCEngineEventHandler).onNetworkTypeChanged?.call(
        t_NetworkType.android_to_code($p_a.NetworkType.values
            .firstWhere((t) => t.$value == type || t.name == type)));
  }

  FutureOr<void> onUserStartVideoCapture(
      dynamic streamId, dynamic streamInfo) async {
    if ($instance == null || $instance is! IRTCEngineEventHandler) {
      return;
    }
    return ($instance as IRTCEngineEventHandler).onUserStartVideoCapture?.call(
        streamId.toString(),
        packObject(
            streamInfo,
            () => StreamInfo.fromMap(StreamInfo.deepPackedMapValues(
                StreamInfo.mapMemberToConstructorParams(streamInfo)))));
  }

  FutureOr<void> onUserStopVideoCapture(
      dynamic streamId, dynamic streamInfo) async {
    if ($instance == null || $instance is! IRTCEngineEventHandler) {
      return;
    }
    return ($instance as IRTCEngineEventHandler).onUserStopVideoCapture?.call(
        streamId.toString(),
        packObject(
            streamInfo,
            () => StreamInfo.fromMap(StreamInfo.deepPackedMapValues(
                StreamInfo.mapMemberToConstructorParams(streamInfo)))));
  }

  FutureOr<void> onUserStartAudioCapture(
      dynamic streamId, dynamic streamInfo) async {
    if ($instance == null || $instance is! IRTCEngineEventHandler) {
      return;
    }
    return ($instance as IRTCEngineEventHandler).onUserStartAudioCapture?.call(
        streamId.toString(),
        packObject(
            streamInfo,
            () => StreamInfo.fromMap(StreamInfo.deepPackedMapValues(
                StreamInfo.mapMemberToConstructorParams(streamInfo)))));
  }

  FutureOr<void> onUserStopAudioCapture(
      dynamic streamId, dynamic streamInfo) async {
    if ($instance == null || $instance is! IRTCEngineEventHandler) {
      return;
    }
    return ($instance as IRTCEngineEventHandler).onUserStopAudioCapture?.call(
        streamId.toString(),
        packObject(
            streamInfo,
            () => StreamInfo.fromMap(StreamInfo.deepPackedMapValues(
                StreamInfo.mapMemberToConstructorParams(streamInfo)))));
  }

  FutureOr<void> onLocalAudioStateChanged(
      dynamic audioSource, dynamic state, dynamic error) async {
    if ($instance == null || $instance is! IRTCEngineEventHandler) {
      return;
    }
    return ($instance as IRTCEngineEventHandler).onLocalAudioStateChanged?.call(
        audioSource,
        t_LocalAudioStreamState.android_to_code($p_a
            .LocalAudioStreamState.values
            .firstWhere((t) => t.$value == state || t.name == state)),
        t_LocalAudioStreamError.android_to_code($p_a
            .LocalAudioStreamError.values
            .firstWhere((t) => t.$value == error || t.name == error)));
  }

  FutureOr<void> onRemoteAudioStateChanged(dynamic streamId, dynamic streamInfo,
      dynamic state, dynamic reason) async {
    if ($instance == null || $instance is! IRTCEngineEventHandler) {
      return;
    }
    return ($instance as IRTCEngineEventHandler)
        .onRemoteAudioStateChanged
        ?.call(
            streamId.toString(),
            packObject(
                streamInfo,
                () => StreamInfo.fromMap(StreamInfo.deepPackedMapValues(
                    StreamInfo.mapMemberToConstructorParams(streamInfo)))),
            t_RemoteAudioState.android_to_code($p_a.RemoteAudioState.values
                .firstWhere((t) => t.$value == state || t.name == state)),
            t_RemoteAudioStateChangeReason.android_to_code($p_a
                .RemoteAudioStateChangeReason.values
                .firstWhere((t) => t.$value == reason || t.name == reason)));
  }

  FutureOr<void> onLocalVideoStateChanged(
      dynamic videoSource, dynamic state, dynamic error) async {
    if ($instance == null || $instance is! IRTCEngineEventHandler) {
      return;
    }
    return ($instance as IRTCEngineEventHandler).onLocalVideoStateChanged?.call(
        videoSource,
        t_LocalVideoStreamState.android_to_code($p_a
            .LocalVideoStreamState.values
            .firstWhere((t) => t.$value == state || t.name == state)),
        t_LocalVideoStreamError.android_to_code($p_a
            .LocalVideoStreamError.values
            .firstWhere((t) => t.$value == error || t.name == error)));
  }

  FutureOr<void> onRemoteVideoStateChanged(dynamic streamId, dynamic streamInfo,
      dynamic videoState, dynamic videoStateReason) async {
    if ($instance == null || $instance is! IRTCEngineEventHandler) {
      return;
    }
    return ($instance as IRTCEngineEventHandler)
        .onRemoteVideoStateChanged
        ?.call(
            streamId.toString(),
            packObject(
                streamInfo,
                () => StreamInfo.fromMap(StreamInfo.deepPackedMapValues(
                    StreamInfo.mapMemberToConstructorParams(streamInfo)))),
            t_RemoteVideoState.android_to_code($p_a
                .RemoteVideoState.values
                .firstWhere(
                    (t) => t.$value == videoState || t.name == videoState)),
            t_RemoteVideoStateChangeReason.android_to_code(
                $p_a.RemoteVideoStateChangeReason.values.firstWhere((t) =>
                    t.$value == videoStateReason ||
                    t.name == videoStateReason)));
  }

  FutureOr<void> onRemoteVideoSuperResolutionModeChanged(dynamic streamId,
      dynamic streamInfo, dynamic mode, dynamic reason) async {
    if ($instance == null || $instance is! IRTCEngineEventHandler) {
      return;
    }
    return ($instance as IRTCEngineEventHandler)
        .onRemoteVideoSuperResolutionModeChanged
        ?.call(
            streamId.toString(),
            packObject(
                streamInfo,
                () => StreamInfo.fromMap(StreamInfo.deepPackedMapValues(
                    StreamInfo.mapMemberToConstructorParams(streamInfo)))),
            t_VideoSuperResolutionMode.android_to_code($p_a
                .VideoSuperResolutionMode.values
                .firstWhere((t) => t.$value == mode || t.name == mode)),
            t_VideoSuperResolutionModeChangedReason.android_to_code($p_a
                .VideoSuperResolutionModeChangedReason.values
                .firstWhere((t) => t.$value == reason || t.name == reason)));
  }

  FutureOr<void> onVideoDenoiseModeChanged(dynamic mode, dynamic reason) async {
    if ($instance == null || $instance is! IRTCEngineEventHandler) {
      return;
    }
    return ($instance as IRTCEngineEventHandler)
        .onVideoDenoiseModeChanged
        ?.call(
            t_VideoDenoiseMode.android_to_code($p_a.VideoDenoiseMode.values
                .firstWhere((t) => t.$value == mode || t.name == mode)),
            t_VideoDenoiseModeChangedReason.android_to_code($p_a
                .VideoDenoiseModeChangedReason.values
                .firstWhere((t) => t.$value == reason || t.name == reason)));
  }

  FutureOr<void> onFirstRemoteVideoFrameRendered(
      dynamic streamId, dynamic streamInfo, dynamic frameInfo) async {
    if ($instance == null || $instance is! IRTCEngineEventHandler) {
      return;
    }
    return ($instance as IRTCEngineEventHandler)
        .onFirstRemoteVideoFrameRendered
        ?.call(
            streamId.toString(),
            packObject(
                streamInfo,
                () => StreamInfo.fromMap(StreamInfo.deepPackedMapValues(
                    StreamInfo.mapMemberToConstructorParams(streamInfo)))),
            packObject(
                frameInfo,
                () => VideoFrameInfo.fromMap(VideoFrameInfo.deepPackedMapValues(
                    VideoFrameInfo.mapMemberToConstructorParams(frameInfo)))));
  }

  FutureOr<void> onFirstRemoteVideoFrameDecoded(
      dynamic streamId, dynamic streamInfo, dynamic frameInfo) async {
    if ($instance == null || $instance is! IRTCEngineEventHandler) {
      return;
    }
    return ($instance as IRTCEngineEventHandler)
        .onFirstRemoteVideoFrameDecoded
        ?.call(
            streamId.toString(),
            packObject(
                streamInfo,
                () => StreamInfo.fromMap(StreamInfo.deepPackedMapValues(
                    StreamInfo.mapMemberToConstructorParams(streamInfo)))),
            packObject(
                frameInfo,
                () => VideoFrameInfo.fromMap(VideoFrameInfo.deepPackedMapValues(
                    VideoFrameInfo.mapMemberToConstructorParams(frameInfo)))));
  }

  FutureOr<void> onFirstLocalVideoFrameCaptured(
      dynamic videoSource, dynamic frameInfo) async {
    if ($instance == null || $instance is! IRTCEngineEventHandler) {
      return;
    }
    return ($instance as IRTCEngineEventHandler)
        .onFirstLocalVideoFrameCaptured
        ?.call(
            videoSource,
            packObject(
                frameInfo,
                () => VideoFrameInfo.fromMap(VideoFrameInfo.deepPackedMapValues(
                    VideoFrameInfo.mapMemberToConstructorParams(frameInfo)))));
  }

  FutureOr<void> onLocalVideoSizeChanged(
      dynamic videoSource, dynamic frameInfo) async {
    if ($instance == null || $instance is! IRTCEngineEventHandler) {
      return;
    }
    return ($instance as IRTCEngineEventHandler).onLocalVideoSizeChanged?.call(
        videoSource,
        packObject(
            frameInfo,
            () => VideoFrameInfo.fromMap(VideoFrameInfo.deepPackedMapValues(
                VideoFrameInfo.mapMemberToConstructorParams(frameInfo)))));
  }

  FutureOr<void> onRemoteVideoSizeChanged(
      dynamic streamId, dynamic streamInfo, dynamic frameInfo) async {
    if ($instance == null || $instance is! IRTCEngineEventHandler) {
      return;
    }
    return ($instance as IRTCEngineEventHandler).onRemoteVideoSizeChanged?.call(
        streamId.toString(),
        packObject(
            streamInfo,
            () => StreamInfo.fromMap(StreamInfo.deepPackedMapValues(
                StreamInfo.mapMemberToConstructorParams(streamInfo)))),
        packObject(
            frameInfo,
            () => VideoFrameInfo.fromMap(VideoFrameInfo.deepPackedMapValues(
                VideoFrameInfo.mapMemberToConstructorParams(frameInfo)))));
  }

  FutureOr<void> onConnectionStateChanged(dynamic state) async {
    if ($instance == null || $instance is! IRTCEngineEventHandler) {
      return;
    }
    return ($instance as IRTCEngineEventHandler)
        .onConnectionStateChanged
        ?.call(int.tryParse(state.toString()) ?? 0, null);
  }

  FutureOr<void> onAudioRouteChanged(dynamic route) async {
    if ($instance == null || $instance is! IRTCEngineEventHandler) {
      return;
    }
    return ($instance as IRTCEngineEventHandler).onAudioRouteChanged?.call(
        t_AudioRoute.android_to_code($p_a.AudioRoute.values
            .firstWhere((t) => t.$value == route || t.name == route)));
  }

  FutureOr<void> onFirstLocalAudioFrame(dynamic audioSource) async {
    if ($instance == null || $instance is! IRTCEngineEventHandler) {
      return;
    }
    return ($instance as IRTCEngineEventHandler)
        .onFirstLocalAudioFrame
        ?.call(audioSource);
  }

  FutureOr<void> onFirstRemoteAudioFrame(
      dynamic streamId, dynamic streamInfo) async {
    if ($instance == null || $instance is! IRTCEngineEventHandler) {
      return;
    }
    return ($instance as IRTCEngineEventHandler).onFirstRemoteAudioFrame?.call(
        streamId.toString(),
        packObject(
            streamInfo,
            () => StreamInfo.fromMap(StreamInfo.deepPackedMapValues(
                StreamInfo.mapMemberToConstructorParams(streamInfo)))));
  }

  FutureOr<void> onSEIMessageReceived(
      dynamic streamId, dynamic streamInfo, dynamic message) async {
    if ($instance == null || $instance is! IRTCEngineEventHandler) {
      return;
    }
    return ($instance as IRTCEngineEventHandler).onSEIMessageReceived?.call(
        streamId.toString(),
        packObject(
            streamInfo,
            () => StreamInfo.fromMap(StreamInfo.deepPackedMapValues(
                StreamInfo.mapMemberToConstructorParams(streamInfo)))),
        message);
  }

  FutureOr<void> onSEIStreamUpdate(
      dynamic streamId, dynamic streamInfo, dynamic event) async {
    if ($instance == null || $instance is! IRTCEngineEventHandler) {
      return;
    }
    return ($instance as IRTCEngineEventHandler).onSEIStreamUpdate?.call(
        streamId.toString(),
        packObject(
            streamInfo,
            () => StreamInfo.fromMap(StreamInfo.deepPackedMapValues(
                StreamInfo.mapMemberToConstructorParams(streamInfo)))),
        t_SEIStreamUpdateEvent.android_to_code($p_a.SEIStreamUpdateEvent.values
            .firstWhere((t) => t.$value == event || t.name == event)));
  }

  FutureOr<void> onLoginResult(
      dynamic uid, dynamic errorCode, dynamic elapsed) async {
    if ($instance == null || $instance is! IRTCEngineEventHandler) {
      return;
    }
    return ($instance as IRTCEngineEventHandler).onLoginResult?.call(
        uid.toString(),
        t_LoginErrorCode.android_to_code($p_a.LoginErrorCode.values
            .firstWhere((t) => t.$value == errorCode || t.name == errorCode)),
        int.tryParse(elapsed.toString()) ?? 0);
  }

  FutureOr<void> onLogout(dynamic reason) async {
    if ($instance == null || $instance is! IRTCEngineEventHandler) {
      return;
    }
    return ($instance as IRTCEngineEventHandler).onLogout?.call(
        t_LogoutReason.android_to_code($p_a.LogoutReason.values
            .firstWhere((t) => t.$value == reason || t.name == reason)));
  }

  FutureOr<void> onServerParamsSetResult(dynamic error) async {
    if ($instance == null || $instance is! IRTCEngineEventHandler) {
      return;
    }
    return ($instance as IRTCEngineEventHandler)
        .onServerParamsSetResult
        ?.call(int.tryParse(error.toString()) ?? 0);
  }

  FutureOr<void> onGetPeerOnlineStatus(
      dynamic peerUserId, dynamic status) async {
    if ($instance == null || $instance is! IRTCEngineEventHandler) {
      return;
    }
    return ($instance as IRTCEngineEventHandler).onGetPeerOnlineStatus?.call(
        peerUserId.toString(),
        t_UserOnlineStatus.android_to_code($p_a.UserOnlineStatus.values
            .firstWhere((t) => t.$value == status || t.name == status)));
  }

  FutureOr<void> onUserMessageReceivedOutsideRoom(
      dynamic msgid, dynamic uid, dynamic message) async {
    if ($instance == null || $instance is! IRTCEngineEventHandler) {
      return;
    }
    return ($instance as IRTCEngineEventHandler)
        .onUserMessageReceivedOutsideRoom
        ?.call(int.tryParse(msgid.toString()) ?? 0, uid.toString(),
            message.toString());
  }

  FutureOr<void> onUserBinaryMessageReceivedOutsideRoom(
      dynamic msgid, dynamic uid, dynamic message) async {
    if ($instance == null || $instance is! IRTCEngineEventHandler) {
      return;
    }
    return ($instance as IRTCEngineEventHandler)
        .onUserBinaryMessageReceivedOutsideRoom
        ?.call(int.tryParse(msgid.toString()) ?? 0, uid.toString(), message);
  }

  FutureOr<void> onUserMessageSendResultOutsideRoom(
      dynamic msgid, dynamic error) async {
    if ($instance == null || $instance is! IRTCEngineEventHandler) {
      return;
    }
    return ($instance as IRTCEngineEventHandler)
        .onUserMessageSendResultOutsideRoom
        ?.call(
            int.tryParse(msgid.toString()) ?? 0,
            t_UserMessageSendResult.android_to_code($p_a
                .UserMessageSendResult.values
                .firstWhere((t) => t.$value == error || t.name == error)));
  }

  FutureOr<void> onServerMessageSendResult(
      dynamic msgid, dynamic error, dynamic message) async {
    if ($instance == null || $instance is! IRTCEngineEventHandler) {
      return;
    }
    return ($instance as IRTCEngineEventHandler)
        .onServerMessageSendResult
        ?.call(
            int.tryParse(msgid.toString()) ?? 0,
            t_UserMessageSendResult.android_to_code($p_a
                .UserMessageSendResult.values
                .firstWhere((t) => t.$value == error || t.name == error)),
            message);
  }

  FutureOr<void> onNetworkDetectionResult(dynamic type, dynamic quality,
      dynamic rtt, dynamic lostRate, dynamic bitrate, dynamic jitter) async {
    if ($instance == null || $instance is! IRTCEngineEventHandler) {
      return;
    }
    return ($instance as IRTCEngineEventHandler).onNetworkDetectionResult?.call(
        t_NetworkDetectionLinkType.android_to_code($p_a
            .NetworkDetectionLinkType.values
            .firstWhere((t) => t.$value == type || t.name == type)),
        t_NetworkQuality.android_to_code($p_a.NetworkQuality.values
            .firstWhere((t) => t.$value == quality || t.name == quality)),
        int.tryParse(rtt.toString()) ?? 0,
        double.tryParse(lostRate.toString()) ?? 0,
        int.tryParse(bitrate.toString()) ?? 0,
        int.tryParse(jitter.toString()) ?? 0);
  }

  FutureOr<void> onNetworkDetectionStopped(dynamic reason) async {
    if ($instance == null || $instance is! IRTCEngineEventHandler) {
      return;
    }
    return ($instance as IRTCEngineEventHandler)
        .onNetworkDetectionStopped
        ?.call(t_NetworkDetectionStopReason.android_to_code($p_a
            .NetworkDetectionStopReason.values
            .firstWhere((t) => t.$value == reason || t.name == reason)));
  }

  FutureOr<void> onAudioDeviceStateChanged(dynamic deviceID, dynamic deviceType,
      dynamic deviceState, dynamic deviceError) async {
    if ($instance == null || $instance is! IRTCEngineEventHandler) {
      return;
    }
    return ($instance as IRTCEngineEventHandler)
        .onAudioDeviceStateChanged
        ?.call(
            deviceID.toString(),
            t_AudioDeviceType.android_to_code($p_a.AudioDeviceType.values
                .firstWhere((t) =>
                    t.$value == deviceType || t.name == deviceType)),
            t_MediaDeviceState.android_to_code($p_a
                .MediaDeviceState.values
                .firstWhere((t) =>
                    t.$value == deviceState || t.name == deviceState)),
            t_MediaDeviceError.android_to_code(
                $p_a.MediaDeviceError.values.firstWhere(
                    (t) => t.$value == deviceError || t.name == deviceError)));
  }

  FutureOr<void> onVideoDeviceStateChanged(dynamic deviceID, dynamic deviceType,
      dynamic deviceState, dynamic deviceError) async {
    if ($instance == null || $instance is! IRTCEngineEventHandler) {
      return;
    }
    return ($instance as IRTCEngineEventHandler)
        .onVideoDeviceStateChanged
        ?.call(
            deviceID.toString(),
            t_VideoDeviceType.android_to_code($p_a.VideoDeviceType.values
                .firstWhere((t) =>
                    t.$value == deviceType || t.name == deviceType)),
            t_MediaDeviceState.android_to_code($p_a
                .MediaDeviceState.values
                .firstWhere((t) =>
                    t.$value == deviceState || t.name == deviceState)),
            t_MediaDeviceError.android_to_code(
                $p_a.MediaDeviceError.values.firstWhere(
                    (t) => t.$value == deviceError || t.name == deviceError)));
  }

  FutureOr<void> onAudioDeviceWarning(
      dynamic deviceID, dynamic deviceType, dynamic deviceWarning) async {
    if ($instance == null || $instance is! IRTCEngineEventHandler) {
      return;
    }
    return ($instance as IRTCEngineEventHandler).onAudioDeviceWarning?.call(
        deviceID.toString(),
        t_AudioDeviceType.android_to_code($p_a.AudioDeviceType.values
            .firstWhere((t) => t.$value == deviceType || t.name == deviceType)),
        t_MediaDeviceWarning.android_to_code($p_a.MediaDeviceWarning.values
            .firstWhere(
                (t) => t.$value == deviceWarning || t.name == deviceWarning)));
  }

  FutureOr<void> onVideoDeviceWarning(
      dynamic deviceID, dynamic deviceType, dynamic deviceWarning) async {
    if ($instance == null || $instance is! IRTCEngineEventHandler) {
      return;
    }
    return ($instance as IRTCEngineEventHandler).onVideoDeviceWarning?.call(
        deviceID.toString(),
        t_VideoDeviceType.android_to_code($p_a.VideoDeviceType.values
            .firstWhere((t) => t.$value == deviceType || t.name == deviceType)),
        t_MediaDeviceWarning.android_to_code($p_a.MediaDeviceWarning.values
            .firstWhere(
                (t) => t.$value == deviceWarning || t.name == deviceWarning)));
  }

  FutureOr<void> onRecordingStateUpdate(dynamic videoSource, dynamic state,
      dynamic errorCode, dynamic info) async {
    if ($instance == null || $instance is! IRTCEngineEventHandler) {
      return;
    }
    return ($instance as IRTCEngineEventHandler).onRecordingStateUpdate?.call(
        videoSource,
        t_RecordingState.android_to_code($p_a.RecordingState.values
            .firstWhere((t) => t.$value == state || t.name == state)),
        t_RecordingErrorCode.android_to_code($p_a.RecordingErrorCode.values
            .firstWhere((t) => t.$value == errorCode || t.name == errorCode)),
        packObject(
            info,
            () => RecordingInfo.fromMap(RecordingInfo.deepPackedMapValues(
                RecordingInfo.mapMemberToConstructorParams(info)))));
  }

  FutureOr<void> onRecordingProgressUpdate(
      dynamic videoSource, dynamic progress, dynamic info) async {
    if ($instance == null || $instance is! IRTCEngineEventHandler) {
      return;
    }
    return ($instance as IRTCEngineEventHandler)
        .onRecordingProgressUpdate
        ?.call(
            videoSource,
            packObject(
                progress,
                () => RecordingProgress.fromMap(
                    RecordingProgress.deepPackedMapValues(
                        RecordingProgress.mapMemberToConstructorParams(
                            progress)))),
            packObject(
                info,
                () => RecordingInfo.fromMap(RecordingInfo.deepPackedMapValues(
                    RecordingInfo.mapMemberToConstructorParams(info)))));
  }

  FutureOr<void> onAudioRecordingStateUpdate(
      dynamic state, dynamic errorCode) async {
    if ($instance == null || $instance is! IRTCEngineEventHandler) {
      return;
    }
    return ($instance as IRTCEngineEventHandler)
        .onAudioRecordingStateUpdate
        ?.call(
            t_AudioRecordingState.android_to_code($p_a
                .AudioRecordingState.values
                .firstWhere((t) => t.$value == state || t.name == state)),
            t_AudioRecordingErrorCode.android_to_code(
                $p_a.AudioRecordingErrorCode.values.firstWhere(
                    (t) => t.$value == errorCode || t.name == errorCode)));
  }

  FutureOr<void> onAudioMixingPlayingProgress(
      dynamic mixId, dynamic progress) async {
    if ($instance == null || $instance is! IRTCEngineEventHandler) {
      return;
    }
    return ($instance as IRTCEngineEventHandler)
        .onAudioMixingPlayingProgress
        ?.call(int.tryParse(mixId.toString()) ?? 0,
            int.tryParse(progress.toString()) ?? 0);
  }

  FutureOr<void> onLocalAudioPropertiesReport(
      List<dynamic> audioPropertiesInfos) async {
    if ($instance == null || $instance is! IRTCEngineEventHandler) {
      return;
    }
    return ($instance as IRTCEngineEventHandler)
        .onLocalAudioPropertiesReport
        ?.call(audioPropertiesInfos
            .map((e) => packObject(
                e,
                () => LocalAudioPropertiesInfo.fromMap(
                    LocalAudioPropertiesInfo.deepPackedMapValues(
                        LocalAudioPropertiesInfo.mapMemberToConstructorParams(
                            e)))))
            .toList());
  }

  FutureOr<void> onAudioPlaybackDeviceTestVolume(dynamic volume) async {
    if ($instance == null || $instance is! IRTCEngineEventHandler) {
      return;
    }
    return ($instance as IRTCEngineEventHandler)
        .onAudioPlaybackDeviceTestVolume
        ?.call(int.tryParse(volume.toString()) ?? 0);
  }

  FutureOr<void> onRemoteAudioPropertiesReport(
      List<dynamic> audioPropertiesInfos, dynamic totalRemoteVolume) async {
    if ($instance == null || $instance is! IRTCEngineEventHandler) {
      return;
    }
    return ($instance as IRTCEngineEventHandler)
        .onRemoteAudioPropertiesReport
        ?.call(
            audioPropertiesInfos
                .map((e) => packObject(
                    e,
                    () => RemoteAudioPropertiesInfo.fromMap(
                        RemoteAudioPropertiesInfo.deepPackedMapValues(
                            RemoteAudioPropertiesInfo
                                .mapMemberToConstructorParams(e)))))
                .toList(),
            int.tryParse(totalRemoteVolume.toString()) ?? 0);
  }

  FutureOr<void> onActiveSpeaker(dynamic roomId, dynamic uid) async {
    if ($instance == null || $instance is! IRTCEngineEventHandler) {
      return;
    }
    return ($instance as IRTCEngineEventHandler)
        .onActiveSpeaker
        ?.call(roomId.toString(), uid.toString());
  }

  FutureOr<void> onEchoTestResult(dynamic result) async {
    if ($instance == null || $instance is! IRTCEngineEventHandler) {
      return;
    }
    return ($instance as IRTCEngineEventHandler).onEchoTestResult?.call(
        t_EchoTestResult.android_to_code($p_a.EchoTestResult.values
            .firstWhere((t) => t.$value == result || t.name == result)));
  }

  FutureOr<void> onCloudProxyConnected(dynamic interval) async {
    if ($instance == null || $instance is! IRTCEngineEventHandler) {
      return;
    }
    return ($instance as IRTCEngineEventHandler)
        .onCloudProxyConnected
        ?.call(int.tryParse(interval.toString()) ?? 0);
  }

  FutureOr<void> onAudioDumpStateChanged(dynamic status) async {
    if ($instance == null || $instance is! IRTCEngineEventHandler) {
      return;
    }
    return ($instance as IRTCEngineEventHandler).onAudioDumpStateChanged?.call(
        t_AudioDumpStatus.android_to_code($p_a.AudioDumpStatus.values
            .firstWhere((t) => t.$value == status || t.name == status)));
  }

  FutureOr<void> onLicenseWillExpire(dynamic days) async {
    if ($instance == null || $instance is! IRTCEngineEventHandler) {
      return;
    }
    return ($instance as IRTCEngineEventHandler)
        .onLicenseWillExpire
        ?.call(int.tryParse(days.toString()) ?? 0);
  }

  FutureOr<void> onHardwareEchoDetectionResult(
      dynamic hardwareEchoDetectionResult) async {
    if ($instance == null || $instance is! IRTCEngineEventHandler) {
      return;
    }
    return ($instance as IRTCEngineEventHandler)
        .onHardwareEchoDetectionResult
        ?.call(t_HardwareEchoDetectionResult.android_to_code(
            $p_a.HardwareEchoDetectionResult.values.firstWhere((t) =>
                t.$value == hardwareEchoDetectionResult ||
                t.name == hardwareEchoDetectionResult)));
  }

  FutureOr<void> onLocalProxyStateChanged(dynamic localProxyType,
      dynamic localProxyState, dynamic localProxyError) async {
    if ($instance == null || $instance is! IRTCEngineEventHandler) {
      return;
    }
    return ($instance as IRTCEngineEventHandler).onLocalProxyStateChanged?.call(
        t_LocalProxyType.android_to_code($p_a.LocalProxyType.values.firstWhere(
            (t) => t.$value == localProxyType || t.name == localProxyType)),
        t_LocalProxyState.android_to_code($p_a.LocalProxyState.values
            .firstWhere((t) =>
                t.$value == localProxyState || t.name == localProxyState)),
        t_LocalProxyError.android_to_code($p_a.LocalProxyError.values
            .firstWhere((t) =>
                t.$value == localProxyError || t.name == localProxyError)));
  }

  FutureOr<void> onEffectError(dynamic error, dynamic msg) async {
    if ($instance == null || $instance is! IRTCEngineEventHandler) {
      return;
    }
    return ($instance as IRTCEngineEventHandler).onEffectError?.call(
        t_EffectErrorType.android_to_code($p_a.EffectErrorType.values
            .firstWhere((t) => t.$value == error || t.name == error)),
        msg.toString());
  }

  FutureOr<void> onStreamSyncInfoReceived(dynamic streamId, dynamic streamInfo,
      dynamic streamType, dynamic data) async {
    if ($instance == null || $instance is! IRTCEngineEventHandler) {
      return;
    }
    return ($instance as IRTCEngineEventHandler).onStreamSyncInfoReceived?.call(
        streamId.toString(),
        packObject(
            streamInfo,
            () => StreamInfo.fromMap(StreamInfo.deepPackedMapValues(
                StreamInfo.mapMemberToConstructorParams(streamInfo)))),
        t_SyncInfoStreamType.android_to_code($p_a.SyncInfoStreamType.values
            .firstWhere((t) => t.$value == streamType || t.name == streamType)),
        data);
  }

  FutureOr<void> onExternalScreenFrameUpdate(dynamic info) async {
    if ($instance == null || $instance is! IRTCEngineEventHandler) {
      return;
    }
    return ($instance as IRTCEngineEventHandler)
        .onExternalScreenFrameUpdate
        ?.call(
            packObject(
                info,
                () => FrameUpdateInfo.fromMap(
                    FrameUpdateInfo.deepPackedMapValues(
                        FrameUpdateInfo.mapMemberToConstructorParams(info)))),
            null);
  }

  FutureOr<void> onRemoteSnapshotTakenToFile(
      dynamic streamId,
      dynamic streamInfo,
      dynamic filePath,
      dynamic width,
      dynamic height,
      dynamic errorCode,
      dynamic taskId) async {
    if ($instance == null || $instance is! IRTCEngineEventHandler) {
      return;
    }
    return ($instance as IRTCEngineEventHandler)
        .onRemoteSnapshotTakenToFile
        ?.call(
            streamId.toString(),
            filePath.toString(),
            int.tryParse(width.toString()) ?? 0,
            int.tryParse(height.toString()) ?? 0,
            t_SnapshotErrorCode.android_to_code($p_a.SnapshotErrorCode.values
                .firstWhere(
                    (t) => t.$value == errorCode || t.name == errorCode)),
            int.tryParse(taskId.toString()) ?? 0);
  }

  FutureOr<void> onAudioFrameSendStateChanged(
      dynamic streamId, dynamic streamInfo, dynamic user, dynamic state) async {
    if ($instance == null || $instance is! IRTCEngineEventHandler) {
      return;
    }
    return ($instance
            as IRTCEngineEventHandler)
        .onAudioFrameSendStateChanged
        ?.call(
            streamId.toString(),
            packObject(
                streamInfo,
                () => StreamInfo.fromMap(
                    StreamInfo.deepPackedMapValues(
                        StreamInfo.mapMemberToConstructorParams(streamInfo)))),
            packObject(
                user,
                () => RtcUser.fromMap(RtcUser
                    .deepPackedMapValues(RtcUser.mapMemberToConstructorParams(
                        user)))),
            t_FirstFrameSendState.android_to_code($p_a
                .FirstFrameSendState.values
                .firstWhere((t) => t.$value == state || t.name == state)));
  }

  FutureOr<void> onVideoFrameSendStateChanged(
      dynamic streamId, dynamic streamInfo, dynamic user, dynamic state) async {
    if ($instance == null || $instance is! IRTCEngineEventHandler) {
      return;
    }
    return ($instance
            as IRTCEngineEventHandler)
        .onVideoFrameSendStateChanged
        ?.call(
            streamId.toString(),
            packObject(
                streamInfo,
                () => StreamInfo.fromMap(
                    StreamInfo.deepPackedMapValues(
                        StreamInfo.mapMemberToConstructorParams(streamInfo)))),
            packObject(
                user,
                () => RtcUser.fromMap(RtcUser
                    .deepPackedMapValues(RtcUser.mapMemberToConstructorParams(
                        user)))),
            t_FirstFrameSendState.android_to_code($p_a
                .FirstFrameSendState.values
                .firstWhere((t) => t.$value == state || t.name == state)));
  }

  FutureOr<void> onAudioFramePlayStateChanged(
      dynamic streamId, dynamic streamInfo, dynamic user, dynamic state) async {
    if ($instance == null || $instance is! IRTCEngineEventHandler) {
      return;
    }
    return ($instance
            as IRTCEngineEventHandler)
        .onAudioFramePlayStateChanged
        ?.call(
            streamId.toString(),
            packObject(
                streamInfo,
                () => StreamInfo.fromMap(
                    StreamInfo.deepPackedMapValues(
                        StreamInfo.mapMemberToConstructorParams(streamInfo)))),
            packObject(
                user,
                () => RtcUser.fromMap(RtcUser
                    .deepPackedMapValues(RtcUser.mapMemberToConstructorParams(
                        user)))),
            t_FirstFramePlayState.android_to_code($p_a
                .FirstFramePlayState.values
                .firstWhere((t) => t.$value == state || t.name == state)));
  }

  FutureOr<void> onVideoFramePlayStateChanged(
      dynamic streamId, dynamic streamInfo, dynamic user, dynamic state) async {
    if ($instance == null || $instance is! IRTCEngineEventHandler) {
      return;
    }
    return ($instance
            as IRTCEngineEventHandler)
        .onVideoFramePlayStateChanged
        ?.call(
            streamId.toString(),
            packObject(
                streamInfo,
                () => StreamInfo.fromMap(
                    StreamInfo.deepPackedMapValues(
                        StreamInfo.mapMemberToConstructorParams(streamInfo)))),
            packObject(
                user,
                () => RtcUser.fromMap(RtcUser
                    .deepPackedMapValues(RtcUser.mapMemberToConstructorParams(
                        user)))),
            t_FirstFramePlayState.android_to_code($p_a
                .FirstFramePlayState.values
                .firstWhere((t) => t.$value == state || t.name == state)));
  }

  FutureOr<void> onSimulcastSubscribeFallback(
      dynamic streamId, dynamic streamInfo, dynamic event) async {
    if ($instance == null || $instance is! IRTCEngineEventHandler) {
      return;
    }
    return ($instance
            as IRTCEngineEventHandler)
        .onSimulcastSubscribeFallback
        ?.call(
            streamId.toString(),
            packObject(
                streamInfo,
                () => StreamInfo.fromMap(StreamInfo.deepPackedMapValues(
                    StreamInfo.mapMemberToConstructorParams(streamInfo)))),
            packObject(
                event,
                () => RemoteStreamSwitch.fromMap(
                    RemoteStreamSwitch.deepPackedMapValues(
                        RemoteStreamSwitch.mapMemberToConstructorParams(
                            event)))));
  }

  FutureOr<void> onPerformanceAlarms(dynamic streamId, dynamic streamInfo,
      dynamic mode, dynamic reason, dynamic data) async {
    if ($instance == null || $instance is! IRTCEngineEventHandler) {
      return;
    }
    return ($instance as IRTCEngineEventHandler).onPerformanceAlarms?.call(
        streamId.toString(),
        packObject(
            streamInfo,
            () => StreamInfo.fromMap(StreamInfo.deepPackedMapValues(
                StreamInfo.mapMemberToConstructorParams(streamInfo)))),
        t_PerformanceAlarmMode.android_to_code($p_a.PerformanceAlarmMode.values
            .firstWhere((t) => t.$value == mode || t.name == mode)),
        t_PerformanceAlarmReason.android_to_code($p_a
            .PerformanceAlarmReason.values
            .firstWhere((t) => t.$value == reason || t.name == reason)),
        packObject(
            data,
            () => SourceWantedData.fromMap(SourceWantedData.deepPackedMapValues(
                SourceWantedData.mapMemberToConstructorParams(data)))));
  }

  FutureOr<void> onRemoteAudioPropertiesReportEx(
      List<dynamic> audioPropertiesInfos) async {
    if ($instance == null || $instance is! IRTCEngineEventHandler) {
      return;
    }
    return ($instance as IRTCEngineEventHandler)
        .onRemoteAudioPropertiesReportEx
        ?.call(
            audioPropertiesInfos
                .map((e) => packObject(
                    e,
                    () => RemoteAudioPropertiesInfo.fromMap(
                        RemoteAudioPropertiesInfo.deepPackedMapValues(
                            RemoteAudioPropertiesInfo
                                .mapMemberToConstructorParams(e)))))
                .toList(),
            null);
  }

  FutureOr<void> onMixedStreamEvent(
      dynamic info, dynamic event, dynamic error) async {
    if ($instance == null || $instance is! IRTCEngineEventHandler) {
      return;
    }
    return ($instance as IRTCEngineEventHandler).onMixedStreamEvent?.call(
        packObject(info, () => MixedStreamTaskInfo()),
        t_MixedStreamTaskEvent.android_to_code($p_a.MixedStreamTaskEvent.values
            .firstWhere((t) => t.$value == event || t.name == event)),
        t_MixedStreamTaskErrorCode.android_to_code($p_a
            .MixedStreamTaskErrorCode.values
            .firstWhere((t) => t.$value == error || t.name == error)));
  }

  FutureOr<void> onSingleStreamEvent(
      dynamic taskId, dynamic event, dynamic error) async {
    if ($instance == null || $instance is! IRTCEngineEventHandler) {
      return;
    }
    return ($instance as IRTCEngineEventHandler).onSingleStreamEvent?.call(
        taskId.toString(),
        t_SingleStreamTaskEvent.android_to_code($p_a
            .SingleStreamTaskEvent.values
            .firstWhere((t) => t.$value == event || t.name == event)),
        t_SingleStreamTaskErrorCode.android_to_code($p_a
            .SingleStreamTaskErrorCode.values
            .firstWhere((t) => t.$value == error || t.name == error)));
  }

  FutureOr<void> onExperimentalCallback(dynamic param) async {
    if ($instance == null || $instance is! IRTCEngineEventHandler) {
      return;
    }
    return ($instance as IRTCEngineEventHandler)
        .onExperimentalCallback
        ?.call(param.toString());
  }

  FutureOr<void> onPushPublicStreamResult(
      dynamic roomId, dynamic publicStreamId, dynamic error) async {
    if ($instance == null || $instance is! IRTCEngineEventHandler) {
      return;
    }
    return ($instance as IRTCEngineEventHandler).onPushPublicStreamResult?.call(
        roomId.toString(),
        publicStreamId.toString(),
        t_PublicStreamErrorCode.android_to_code($p_a
            .PublicStreamErrorCode.values
            .firstWhere((t) => t.$value == error || t.name == error)));
  }

  FutureOr<void> onLogReport(dynamic logType, dynamic logContent) async {
    if ($instance == null || $instance is! IRTCEngineEventHandler) {
      return;
    }
    return ($instance as IRTCEngineEventHandler)
        .onLogReport
        ?.call(logType.toString(), logContent);
  }

  FutureOr<void> onNetworkTimeSynchronized() async {
    if ($instance == null || $instance is! IRTCEngineEventHandler) {
      return;
    }
    return ($instance as IRTCEngineEventHandler)
        .onNetworkTimeSynchronized
        ?.call();
  }
}

class ios_IRTCEngineEventHandler extends $p_i.ByteRTCEngineDelegate {
  ios_IRTCEngineEventHandler();

  FutureOr<void> rtcEngine$onWarning(dynamic engine, dynamic code) async {
    if ($instance == null || $instance is! IRTCEngineEventHandler) {
      return;
    }
    return ($instance as IRTCEngineEventHandler).onWarning?.call(
        t_WarningCode.ios_to_code($p_i.ByteRTCWarningCode.values
            .firstWhere((t) => t.$value == code || t.name == code)));
  }

  FutureOr<void> rtcEngine$onError(dynamic engine, dynamic errorCode) async {
    if ($instance == null || $instance is! IRTCEngineEventHandler) {
      return;
    }
    return ($instance as IRTCEngineEventHandler).onError?.call(
        t_ErrorCode.ios_to_code($p_i.ByteRTCErrorCode.values
            .firstWhere((t) => t.$value == errorCode || t.name == errorCode)));
  }

  FutureOr<void> rtcEngine$onExtensionAccessError$msg(
      dynamic engine, dynamic extensionName, dynamic msg) async {
    if ($instance == null || $instance is! IRTCEngineEventHandler) {
      return;
    }
    return ($instance as IRTCEngineEventHandler)
        .onExtensionAccessError
        ?.call(extensionName.toString(), msg.toString());
  }

  FutureOr<void> rtcEngine$onSysStats(dynamic engine, dynamic stats) async {
    if ($instance == null || $instance is! IRTCEngineEventHandler) {
      return;
    }
    return ($instance as IRTCEngineEventHandler).onSysStats?.call(packObject(
        stats,
        () => SysStats.fromMap(SysStats.deepPackedMapValues(
            SysStats.mapMemberToConstructorParams(stats)))));
  }

  FutureOr<void> rtcEngine$onNetworkTypeChanged(
      dynamic engine, dynamic type) async {
    if ($instance == null || $instance is! IRTCEngineEventHandler) {
      return;
    }
    return ($instance as IRTCEngineEventHandler).onNetworkTypeChanged?.call(
        t_NetworkType.ios_to_code($p_i.ByteRTCNetworkType.values
            .firstWhere((t) => t.$value == type || t.name == type)));
  }

  FutureOr<void> rtcEngine$onUserStartVideoCapture$info(
      dynamic engine, dynamic streamId, dynamic info) async {
    if ($instance == null || $instance is! IRTCEngineEventHandler) {
      return;
    }
    return ($instance as IRTCEngineEventHandler).onUserStartVideoCapture?.call(
        streamId.toString(),
        packObject(
            info,
            () => StreamInfo.fromMap(StreamInfo.deepPackedMapValues(
                StreamInfo.mapMemberToConstructorParams(info)))));
  }

  FutureOr<void> rtcEngine$onUserStopVideoCapture$info(
      dynamic engine, dynamic streamId, dynamic info) async {
    if ($instance == null || $instance is! IRTCEngineEventHandler) {
      return;
    }
    return ($instance as IRTCEngineEventHandler).onUserStopVideoCapture?.call(
        streamId.toString(),
        packObject(
            info,
            () => StreamInfo.fromMap(StreamInfo.deepPackedMapValues(
                StreamInfo.mapMemberToConstructorParams(info)))));
  }

  FutureOr<void> rtcEngine$onUserStartAudioCapture$info(
      dynamic engine, dynamic streamId, dynamic info) async {
    if ($instance == null || $instance is! IRTCEngineEventHandler) {
      return;
    }
    return ($instance as IRTCEngineEventHandler).onUserStartAudioCapture?.call(
        streamId.toString(),
        packObject(
            info,
            () => StreamInfo.fromMap(StreamInfo.deepPackedMapValues(
                StreamInfo.mapMemberToConstructorParams(info)))));
  }

  FutureOr<void> rtcEngine$onUserStopAudioCapture$info(
      dynamic engine, dynamic streamId, dynamic info) async {
    if ($instance == null || $instance is! IRTCEngineEventHandler) {
      return;
    }
    return ($instance as IRTCEngineEventHandler).onUserStopAudioCapture?.call(
        streamId.toString(),
        packObject(
            info,
            () => StreamInfo.fromMap(StreamInfo.deepPackedMapValues(
                StreamInfo.mapMemberToConstructorParams(info)))));
  }

  FutureOr<void> rtcEngine$onLocalAudioStateChanged$state$error(
      dynamic engine, dynamic audioSource, dynamic state, dynamic error) async {
    if ($instance == null || $instance is! IRTCEngineEventHandler) {
      return;
    }
    return ($instance as IRTCEngineEventHandler).onLocalAudioStateChanged?.call(
        audioSource,
        t_LocalAudioStreamState.ios_to_code($p_i
            .ByteRTCLocalAudioStreamState.values
            .firstWhere((t) => t.$value == state || t.name == state)),
        t_LocalAudioStreamError.ios_to_code($p_i
            .ByteRTCLocalAudioStreamError.values
            .firstWhere((t) => t.$value == error || t.name == error)));
  }

  FutureOr<void> rtcEngine$onRemoteAudioStateChanged$info$state$reason(
      dynamic engine,
      dynamic streamId,
      dynamic info,
      dynamic state,
      dynamic reason) async {
    if ($instance == null || $instance is! IRTCEngineEventHandler) {
      return;
    }
    return ($instance as IRTCEngineEventHandler)
        .onRemoteAudioStateChanged
        ?.call(
            streamId.toString(),
            packObject(
                info,
                () => StreamInfo.fromMap(StreamInfo.deepPackedMapValues(
                    StreamInfo.mapMemberToConstructorParams(info)))),
            t_RemoteAudioState.ios_to_code($p_i.ByteRTCRemoteAudioState.values
                .firstWhere((t) => t.$value == state || t.name == state)),
            t_RemoteAudioStateChangeReason.ios_to_code($p_i
                .ByteRTCRemoteAudioStateChangeReason.values
                .firstWhere((t) => t.$value == reason || t.name == reason)));
  }

  FutureOr<void>
      rtcEngine$onLocalVideoStateChanged$withStreamState$withStreamError(
          dynamic engine,
          dynamic videoSource,
          dynamic state,
          dynamic error) async {
    if ($instance == null || $instance is! IRTCEngineEventHandler) {
      return;
    }
    return ($instance as IRTCEngineEventHandler).onLocalVideoStateChanged?.call(
        videoSource,
        t_LocalVideoStreamState.ios_to_code($p_i
            .ByteRTCLocalVideoStreamState.values
            .firstWhere((t) => t.$value == state || t.name == state)),
        t_LocalVideoStreamError.ios_to_code($p_i
            .ByteRTCLocalVideoStreamError.values
            .firstWhere((t) => t.$value == error || t.name == error)));
  }

  FutureOr<void>
      rtcEngine$onRemoteVideoStateChanged$info$withVideoState$withVideoStateReason(
          dynamic engine,
          dynamic streamId,
          dynamic info,
          dynamic state,
          dynamic reason) async {
    if ($instance == null || $instance is! IRTCEngineEventHandler) {
      return;
    }
    return ($instance as IRTCEngineEventHandler)
        .onRemoteVideoStateChanged
        ?.call(
            streamId.toString(),
            packObject(
                info,
                () => StreamInfo.fromMap(StreamInfo.deepPackedMapValues(
                    StreamInfo.mapMemberToConstructorParams(info)))),
            t_RemoteVideoState.ios_to_code($p_i.ByteRTCRemoteVideoState.values
                .firstWhere((t) => t.$value == state || t.name == state)),
            t_RemoteVideoStateChangeReason.ios_to_code($p_i
                .ByteRTCRemoteVideoStateChangeReason.values
                .firstWhere((t) => t.$value == reason || t.name == reason)));
  }

  FutureOr<void>
      rtcEngine$onRemoteVideoSuperResolutionModeChanged$info$withMode$withReason(
          dynamic engine,
          dynamic streamId,
          dynamic info,
          dynamic mode,
          dynamic reason) async {
    if ($instance == null || $instance is! IRTCEngineEventHandler) {
      return;
    }
    return ($instance as IRTCEngineEventHandler)
        .onRemoteVideoSuperResolutionModeChanged
        ?.call(
            streamId.toString(),
            packObject(
                info,
                () => StreamInfo.fromMap(StreamInfo.deepPackedMapValues(
                    StreamInfo.mapMemberToConstructorParams(info)))),
            t_VideoSuperResolutionMode.ios_to_code($p_i
                .ByteRTCVideoSuperResolutionMode.values
                .firstWhere((t) => t.$value == mode || t.name == mode)),
            t_VideoSuperResolutionModeChangedReason.ios_to_code($p_i
                .ByteRTCVideoSuperResolutionModeChangedReason.values
                .firstWhere((t) => t.$value == reason || t.name == reason)));
  }

  FutureOr<void> rtcEngine$onVideoDenoiseModeChanged$withReason(
      dynamic engine, dynamic mode, dynamic reason) async {
    if ($instance == null || $instance is! IRTCEngineEventHandler) {
      return;
    }
    return ($instance as IRTCEngineEventHandler)
        .onVideoDenoiseModeChanged
        ?.call(
            t_VideoDenoiseMode.ios_to_code($p_i.ByteRTCVideoDenoiseMode.values
                .firstWhere((t) => t.$value == mode || t.name == mode)),
            t_VideoDenoiseModeChangedReason.ios_to_code($p_i
                .ByteRTCVideoDenoiseModeChangedReason.values
                .firstWhere((t) => t.$value == reason || t.name == reason)));
  }

  FutureOr<void> rtcEngine$onFirstRemoteVideoFrameRendered$info$withFrameInfo(
      dynamic engine, dynamic streamId, dynamic info, dynamic frameInfo) async {
    if ($instance == null || $instance is! IRTCEngineEventHandler) {
      return;
    }
    return ($instance as IRTCEngineEventHandler)
        .onFirstRemoteVideoFrameRendered
        ?.call(
            streamId.toString(),
            packObject(
                info,
                () => StreamInfo.fromMap(StreamInfo.deepPackedMapValues(
                    StreamInfo.mapMemberToConstructorParams(info)))),
            packObject(
                frameInfo,
                () => VideoFrameInfo.fromMap(VideoFrameInfo.deepPackedMapValues(
                    VideoFrameInfo.mapMemberToConstructorParams(frameInfo)))));
  }

  FutureOr<void> rtcEngine$onFirstRemoteVideoFrameDecoded$info$withFrameInfo(
      dynamic engine, dynamic streamId, dynamic info, dynamic frameInfo) async {
    if ($instance == null || $instance is! IRTCEngineEventHandler) {
      return;
    }
    return ($instance as IRTCEngineEventHandler)
        .onFirstRemoteVideoFrameDecoded
        ?.call(
            streamId.toString(),
            packObject(
                info,
                () => StreamInfo.fromMap(StreamInfo.deepPackedMapValues(
                    StreamInfo.mapMemberToConstructorParams(info)))),
            packObject(
                frameInfo,
                () => VideoFrameInfo.fromMap(VideoFrameInfo.deepPackedMapValues(
                    VideoFrameInfo.mapMemberToConstructorParams(frameInfo)))));
  }

  FutureOr<void> rtcEngine$onFirstLocalVideoFrameCaptured$withFrameInfo(
      dynamic engine, dynamic videoSource, dynamic frameInfo) async {
    if ($instance == null || $instance is! IRTCEngineEventHandler) {
      return;
    }
    return ($instance as IRTCEngineEventHandler)
        .onFirstLocalVideoFrameCaptured
        ?.call(
            videoSource,
            packObject(
                frameInfo,
                () => VideoFrameInfo.fromMap(VideoFrameInfo.deepPackedMapValues(
                    VideoFrameInfo.mapMemberToConstructorParams(frameInfo)))));
  }

  FutureOr<void> rtcEngine$onLocalVideoSizeChanged$withFrameInfo(
      dynamic engine, dynamic videoSource, dynamic frameInfo) async {
    if ($instance == null || $instance is! IRTCEngineEventHandler) {
      return;
    }
    return ($instance as IRTCEngineEventHandler).onLocalVideoSizeChanged?.call(
        videoSource,
        packObject(
            frameInfo,
            () => VideoFrameInfo.fromMap(VideoFrameInfo.deepPackedMapValues(
                VideoFrameInfo.mapMemberToConstructorParams(frameInfo)))));
  }

  FutureOr<void> rtcEngine$onRemoteVideoSizeChanged$info$withFrameInfo(
      dynamic engine, dynamic streamId, dynamic info, dynamic frameInfo) async {
    if ($instance == null || $instance is! IRTCEngineEventHandler) {
      return;
    }
    return ($instance as IRTCEngineEventHandler).onRemoteVideoSizeChanged?.call(
        streamId.toString(),
        packObject(
            info,
            () => StreamInfo.fromMap(StreamInfo.deepPackedMapValues(
                StreamInfo.mapMemberToConstructorParams(info)))),
        packObject(
            frameInfo,
            () => VideoFrameInfo.fromMap(VideoFrameInfo.deepPackedMapValues(
                VideoFrameInfo.mapMemberToConstructorParams(frameInfo)))));
  }

  FutureOr<void> rtcEngine$onConnectionStateChanged(
      dynamic engine, dynamic state) async {
    if ($instance == null || $instance is! IRTCEngineEventHandler) {
      return;
    }
    return ($instance as IRTCEngineEventHandler).onConnectionStateChanged?.call(
        int.tryParse(state.toString()) ?? 0,
        int.tryParse(state.toString()) ?? 0);
  }

  FutureOr<void> rtcEngine$onAudioRouteChanged(
      dynamic engine, dynamic device) async {
    if ($instance == null || $instance is! IRTCEngineEventHandler) {
      return;
    }
    return ($instance as IRTCEngineEventHandler).onAudioRouteChanged?.call(
        t_AudioRoute.ios_to_code($p_i.ByteRTCAudioRoute.values
            .firstWhere((t) => t.$value == device || t.name == device)));
  }

  FutureOr<void> rtcEngine$onFirstLocalAudioFrame(
      dynamic engine, dynamic audioSource) async {
    if ($instance == null || $instance is! IRTCEngineEventHandler) {
      return;
    }
    return ($instance as IRTCEngineEventHandler)
        .onFirstLocalAudioFrame
        ?.call(audioSource);
  }

  FutureOr<void> rtcEngine$onFirstRemoteAudioFrame$info(
      dynamic engine, dynamic streamId, dynamic info) async {
    if ($instance == null || $instance is! IRTCEngineEventHandler) {
      return;
    }
    return ($instance as IRTCEngineEventHandler).onFirstRemoteAudioFrame?.call(
        streamId.toString(),
        packObject(
            info,
            () => StreamInfo.fromMap(StreamInfo.deepPackedMapValues(
                StreamInfo.mapMemberToConstructorParams(info)))));
  }

  FutureOr<void> rtcEngine$onSEIMessageReceived$info$andMessage(
      dynamic engine, dynamic streamId, dynamic info, dynamic message) async {
    if ($instance == null || $instance is! IRTCEngineEventHandler) {
      return;
    }
    return ($instance as IRTCEngineEventHandler).onSEIMessageReceived?.call(
        streamId.toString(),
        packObject(
            info,
            () => StreamInfo.fromMap(StreamInfo.deepPackedMapValues(
                StreamInfo.mapMemberToConstructorParams(info)))),
        message);
  }

  FutureOr<void> rtcEngine$onSEIStreamUpdate$info$eventType(
      dynamic engine, dynamic streamId, dynamic info, dynamic eventType) async {
    if ($instance == null || $instance is! IRTCEngineEventHandler) {
      return;
    }
    return ($instance as IRTCEngineEventHandler).onSEIStreamUpdate?.call(
        streamId.toString(),
        packObject(
            info,
            () => StreamInfo.fromMap(StreamInfo.deepPackedMapValues(
                StreamInfo.mapMemberToConstructorParams(info)))),
        t_SEIStreamUpdateEvent.ios_to_code($p_i.ByteRTCSEIStreamEventType.values
            .firstWhere((t) => t.$value == eventType || t.name == eventType)));
  }

  FutureOr<void> rtcEngine$onLoginResult$errorCode$elapsed(
      dynamic engine, dynamic uid, dynamic errorCode, dynamic elapsed) async {
    if ($instance == null || $instance is! IRTCEngineEventHandler) {
      return;
    }
    return ($instance as IRTCEngineEventHandler).onLoginResult?.call(
        uid.toString(),
        t_LoginErrorCode.ios_to_code($p_i.ByteRTCLoginErrorCode.values
            .firstWhere((t) => t.$value == errorCode || t.name == errorCode)),
        int.tryParse(elapsed.toString()) ?? 0);
  }

  FutureOr<void> rtcEngine$onLogout(dynamic engine, dynamic reason) async {
    if ($instance == null || $instance is! IRTCEngineEventHandler) {
      return;
    }
    return ($instance as IRTCEngineEventHandler).onLogout?.call(
        t_LogoutReason.ios_to_code($p_i.ByteRTCLogoutReason.values
            .firstWhere((t) => t.$value == reason || t.name == reason)));
  }

  FutureOr<void> rtcEngine$onServerParamsSetResult(
      dynamic engine, dynamic errorCode) async {
    if ($instance == null || $instance is! IRTCEngineEventHandler) {
      return;
    }
    return ($instance as IRTCEngineEventHandler)
        .onServerParamsSetResult
        ?.call(int.tryParse(engine.toString()) ?? 0);
  }

  FutureOr<void> rtcEngine$onGetPeerOnlineStatus$status(
      dynamic engine, dynamic peerUserId, dynamic status) async {
    if ($instance == null || $instance is! IRTCEngineEventHandler) {
      return;
    }
    return ($instance as IRTCEngineEventHandler).onGetPeerOnlineStatus?.call(
        peerUserId.toString(),
        t_UserOnlineStatus.ios_to_code($p_i.ByteRTCUserOnlineStatus.values
            .firstWhere((t) => t.$value == status || t.name == status)));
  }

  FutureOr<void> rtcEngine$onUserMessageReceivedOutsideRoom$uid$message(
      dynamic engine, dynamic msgid, dynamic uid, dynamic message) async {
    if ($instance == null || $instance is! IRTCEngineEventHandler) {
      return;
    }
    return ($instance as IRTCEngineEventHandler)
        .onUserMessageReceivedOutsideRoom
        ?.call(int.tryParse(msgid.toString()) ?? 0, uid.toString(),
            message.toString());
  }

  FutureOr<void> rtcEngine$onUserBinaryMessageReceivedOutsideRoom$uid$message(
      dynamic engine, dynamic msgid, dynamic uid, dynamic message) async {
    if ($instance == null || $instance is! IRTCEngineEventHandler) {
      return;
    }
    return ($instance as IRTCEngineEventHandler)
        .onUserBinaryMessageReceivedOutsideRoom
        ?.call(int.tryParse(msgid.toString()) ?? 0, uid.toString(), message);
  }

  FutureOr<void> rtcEngine$onUserMessageSendResultOutsideRoom$error(
      dynamic engine, dynamic msgid, dynamic error) async {
    if ($instance == null || $instance is! IRTCEngineEventHandler) {
      return;
    }
    return ($instance as IRTCEngineEventHandler)
        .onUserMessageSendResultOutsideRoom
        ?.call(
            int.tryParse(msgid.toString()) ?? 0,
            t_UserMessageSendResult.ios_to_code($p_i
                .ByteRTCUserMessageSendResult.values
                .firstWhere((t) => t.$value == error || t.name == error)));
  }

  FutureOr<void> rtcEngine$onServerMessageSendResult$error$message(
      dynamic engine, dynamic msgid, dynamic error, dynamic message) async {
    if ($instance == null || $instance is! IRTCEngineEventHandler) {
      return;
    }
    return ($instance as IRTCEngineEventHandler)
        .onServerMessageSendResult
        ?.call(
            int.tryParse(msgid.toString()) ?? 0,
            t_UserMessageSendResult.ios_to_code($p_i
                .ByteRTCUserMessageSendResult.values
                .firstWhere((t) => t.$value == error || t.name == error)),
            message);
  }

  FutureOr<void>
      rtcEngine$onNetworkDetectionResult$quality$rtt$lostRate$bitrate$jitter(
          dynamic engine,
          dynamic type,
          dynamic quality,
          dynamic rtt,
          dynamic lostRate,
          dynamic bitrate,
          dynamic jitter) async {
    if ($instance == null || $instance is! IRTCEngineEventHandler) {
      return;
    }
    return ($instance as IRTCEngineEventHandler).onNetworkDetectionResult?.call(
        t_NetworkDetectionLinkType.ios_to_code($p_i
            .ByteRTCNetworkDetectionLinkType.values
            .firstWhere((t) => t.$value == type || t.name == type)),
        t_NetworkQuality.ios_to_code($p_i.ByteRTCNetworkQuality.values
            .firstWhere((t) => t.$value == quality || t.name == quality)),
        int.tryParse(rtt.toString()) ?? 0,
        double.tryParse(lostRate.toString()) ?? 0,
        int.tryParse(bitrate.toString()) ?? 0,
        int.tryParse(jitter.toString()) ?? 0);
  }

  FutureOr<void> rtcEngine$onNetworkDetectionStopped(
      dynamic engine, dynamic errorCode) async {
    if ($instance == null || $instance is! IRTCEngineEventHandler) {
      return;
    }
    return ($instance as IRTCEngineEventHandler)
        .onNetworkDetectionStopped
        ?.call(t_NetworkDetectionStopReason.ios_to_code($p_i
            .ByteRTCNetworkDetectionStopReason.values
            .firstWhere((t) => t.$value == errorCode || t.name == errorCode)));
  }

  FutureOr<void>
      rtcEngine$onAudioDeviceStateChanged$device_type$device_state$device_error(
          dynamic engine,
          dynamic deviceID,
          dynamic deviceType,
          dynamic deviceState,
          dynamic deviceError) async {
    if ($instance == null || $instance is! IRTCEngineEventHandler) {
      return;
    }
    return ($instance as IRTCEngineEventHandler)
        .onAudioDeviceStateChanged
        ?.call(
            deviceID.toString(),
            t_AudioDeviceType.ios_to_code($p_i.ByteRTCAudioDeviceType.values
                .firstWhere(
                    (t) => t.$value == deviceType || t.name == deviceType)),
            t_MediaDeviceState.ios_to_code($p_i.ByteRTCMediaDeviceState.values
                .firstWhere(
                    (t) => t.$value == deviceState || t.name == deviceState)),
            t_MediaDeviceError.ios_to_code($p_i.ByteRTCMediaDeviceError.values
                .firstWhere(
                    (t) => t.$value == deviceError || t.name == deviceError)));
  }

  FutureOr<void>
      rtcEngine$onVideoDeviceStateChanged$device_type$device_state$device_error(
          dynamic engine,
          dynamic deviceID,
          dynamic deviceType,
          dynamic deviceState,
          dynamic deviceError) async {
    if ($instance == null || $instance is! IRTCEngineEventHandler) {
      return;
    }
    return ($instance as IRTCEngineEventHandler)
        .onVideoDeviceStateChanged
        ?.call(
            deviceID.toString(),
            t_VideoDeviceType.ios_to_code($p_i.ByteRTCVideoDeviceType.values
                .firstWhere(
                    (t) => t.$value == deviceType || t.name == deviceType)),
            t_MediaDeviceState.ios_to_code($p_i.ByteRTCMediaDeviceState.values
                .firstWhere(
                    (t) => t.$value == deviceState || t.name == deviceState)),
            t_MediaDeviceError.ios_to_code($p_i.ByteRTCMediaDeviceError.values
                .firstWhere(
                    (t) => t.$value == deviceError || t.name == deviceError)));
  }

  FutureOr<void> rtcEngine$onAudioDeviceWarning$deviceType$deviceWarning(
      dynamic engine,
      dynamic deviceId,
      dynamic deviceType,
      dynamic deviceWarning) async {
    if ($instance == null || $instance is! IRTCEngineEventHandler) {
      return;
    }
    return ($instance as IRTCEngineEventHandler).onAudioDeviceWarning?.call(
        deviceId.toString(),
        t_AudioDeviceType.ios_to_code($p_i.ByteRTCAudioDeviceType.values
            .firstWhere((t) => t.$value == deviceType || t.name == deviceType)),
        t_MediaDeviceWarning.ios_to_code($p_i.ByteRTCMediaDeviceWarning.values
            .firstWhere(
                (t) => t.$value == deviceWarning || t.name == deviceWarning)));
  }

  FutureOr<void> rtcEngine$onVideoDeviceWarning$deviceType$deviceWarning(
      dynamic engine,
      dynamic deviceId,
      dynamic deviceType,
      dynamic deviceWarning) async {
    if ($instance == null || $instance is! IRTCEngineEventHandler) {
      return;
    }
    return ($instance as IRTCEngineEventHandler).onVideoDeviceWarning?.call(
        deviceId.toString(),
        t_VideoDeviceType.ios_to_code($p_i.ByteRTCVideoDeviceType.values
            .firstWhere((t) => t.$value == deviceType || t.name == deviceType)),
        t_MediaDeviceWarning.ios_to_code($p_i.ByteRTCMediaDeviceWarning.values
            .firstWhere(
                (t) => t.$value == deviceWarning || t.name == deviceWarning)));
  }

  FutureOr<void>
      rtcEngine$onRecordingStateUpdate$state$error_code$recording_info(
          dynamic engine,
          dynamic videoSource,
          dynamic state,
          dynamic errorCode,
          dynamic recordingInfo) async {
    if ($instance == null || $instance is! IRTCEngineEventHandler) {
      return;
    }
    return ($instance as IRTCEngineEventHandler).onRecordingStateUpdate?.call(
        videoSource,
        t_RecordingState.ios_to_code($p_i.ByteRTCRecordingState.values
            .firstWhere((t) => t.$value == state || t.name == state)),
        t_RecordingErrorCode.ios_to_code($p_i.ByteRTCRecordingErrorCode.values
            .firstWhere((t) => t.$value == errorCode || t.name == errorCode)),
        packObject(
            recordingInfo,
            () => RecordingInfo.fromMap(RecordingInfo.deepPackedMapValues(
                RecordingInfo.mapMemberToConstructorParams(recordingInfo)))));
  }

  FutureOr<void> rtcEngine$onRecordingProgressUpdate$process$recording_info(
      dynamic engine,
      dynamic videoSource,
      dynamic process,
      dynamic recordingInfo) async {
    if ($instance == null || $instance is! IRTCEngineEventHandler) {
      return;
    }
    return ($instance as IRTCEngineEventHandler)
        .onRecordingProgressUpdate
        ?.call(
            videoSource,
            packObject(
                process,
                () => RecordingProgress.fromMap(
                    RecordingProgress.deepPackedMapValues(
                        RecordingProgress.mapMemberToConstructorParams(
                            process)))),
            packObject(
                recordingInfo,
                () => RecordingInfo.fromMap(RecordingInfo.deepPackedMapValues(
                    RecordingInfo.mapMemberToConstructorParams(
                        recordingInfo)))));
  }

  FutureOr<void> rtcEngine$onAudioRecordingStateUpdate$error_code(
      dynamic engine, dynamic state, dynamic errorCode) async {
    if ($instance == null || $instance is! IRTCEngineEventHandler) {
      return;
    }
    return ($instance as IRTCEngineEventHandler)
        .onAudioRecordingStateUpdate
        ?.call(
            t_AudioRecordingState.ios_to_code($p_i
                .ByteRTCAudioRecordingState.values
                .firstWhere((t) => t.$value == state || t.name == state)),
            t_AudioRecordingErrorCode.ios_to_code(
                $p_i.ByteRTCAudioRecordingErrorCode.values.firstWhere(
                    (t) => t.$value == errorCode || t.name == errorCode)));
  }

  FutureOr<void> rtcEngine$onAudioMixingPlayingProgress$progress(
      dynamic engine, dynamic mixId, dynamic progress) async {
    if ($instance == null || $instance is! IRTCEngineEventHandler) {
      return;
    }
    return ($instance as IRTCEngineEventHandler)
        .onAudioMixingPlayingProgress
        ?.call(int.tryParse(mixId.toString()) ?? 0,
            int.tryParse(progress.toString()) ?? 0);
  }

  FutureOr<void> rtcEngine$onLocalAudioPropertiesReport(
      dynamic engine, List<dynamic> audioPropertiesInfos) async {
    if ($instance == null || $instance is! IRTCEngineEventHandler) {
      return;
    }
    return ($instance as IRTCEngineEventHandler)
        .onLocalAudioPropertiesReport
        ?.call(audioPropertiesInfos
            .map((e) => packObject(
                e,
                () => LocalAudioPropertiesInfo.fromMap(
                    LocalAudioPropertiesInfo.deepPackedMapValues(
                        LocalAudioPropertiesInfo.mapMemberToConstructorParams(
                            e)))))
            .toList());
  }

  FutureOr<void> rtcEngine$onAudioPlaybackDeviceTestVolume(
      dynamic engine, dynamic volume) async {
    if ($instance == null || $instance is! IRTCEngineEventHandler) {
      return;
    }
    return ($instance as IRTCEngineEventHandler)
        .onAudioPlaybackDeviceTestVolume
        ?.call(int.tryParse(volume.toString()) ?? 0);
  }

  FutureOr<void> rtcEngine$onRemoteAudioPropertiesReport$totalRemoteVolume(
      dynamic engine,
      List<dynamic> audioPropertiesInfos,
      dynamic totalRemoteVolume) async {
    if ($instance == null || $instance is! IRTCEngineEventHandler) {
      return;
    }
    return ($instance as IRTCEngineEventHandler)
        .onRemoteAudioPropertiesReport
        ?.call(
            audioPropertiesInfos
                .map((e) => packObject(
                    e,
                    () => RemoteAudioPropertiesInfo.fromMap(
                        RemoteAudioPropertiesInfo.deepPackedMapValues(
                            RemoteAudioPropertiesInfo
                                .mapMemberToConstructorParams(e)))))
                .toList(),
            int.tryParse(totalRemoteVolume.toString()) ?? 0);
  }

  FutureOr<void> rtcEngine$onActiveSpeaker$uid(
      dynamic engine, dynamic roomId, dynamic uid) async {
    if ($instance == null || $instance is! IRTCEngineEventHandler) {
      return;
    }
    return ($instance as IRTCEngineEventHandler)
        .onActiveSpeaker
        ?.call(roomId.toString(), uid.toString());
  }

  FutureOr<void> rtcEngine$onEchoTestResult(
      dynamic engine, dynamic result) async {
    if ($instance == null || $instance is! IRTCEngineEventHandler) {
      return;
    }
    return ($instance as IRTCEngineEventHandler).onEchoTestResult?.call(
        t_EchoTestResult.ios_to_code($p_i.ByteRTCEchoTestResult.values
            .firstWhere((t) => t.$value == result || t.name == result)));
  }

  FutureOr<void> rtcEngine$onCloudProxyConnected(
      dynamic engine, dynamic interval) async {
    if ($instance == null || $instance is! IRTCEngineEventHandler) {
      return;
    }
    return ($instance as IRTCEngineEventHandler)
        .onCloudProxyConnected
        ?.call(int.tryParse(interval.toString()) ?? 0);
  }

  FutureOr<void> rtcEngine$onAudioDumpStateChanged(
      dynamic engine, dynamic status) async {
    if ($instance == null || $instance is! IRTCEngineEventHandler) {
      return;
    }
    return ($instance as IRTCEngineEventHandler).onAudioDumpStateChanged?.call(
        t_AudioDumpStatus.ios_to_code($p_i.ByteRTCAudioDumpStatus.values
            .firstWhere((t) => t.$value == status || t.name == status)));
  }

  FutureOr<void> rtcEngine$onLicenseWillExpire(
      dynamic engine, dynamic days) async {
    if ($instance == null || $instance is! IRTCEngineEventHandler) {
      return;
    }
    return ($instance as IRTCEngineEventHandler)
        .onLicenseWillExpire
        ?.call(int.tryParse(days.toString()) ?? 0);
  }

  FutureOr<void> rtcEngine$onHardwareEchoDetectionResult(
      dynamic engine, dynamic result) async {
    if ($instance == null || $instance is! IRTCEngineEventHandler) {
      return;
    }
    return ($instance as IRTCEngineEventHandler)
        .onHardwareEchoDetectionResult
        ?.call(t_HardwareEchoDetectionResult.ios_to_code($p_i
            .ByteRTCHardwareEchoDetectionResult.values
            .firstWhere((t) => t.$value == result || t.name == result)));
  }

  FutureOr<void>
      rtcEngine$onLocalProxyStateChanged$withProxyState$withProxyError(
          dynamic engine, dynamic type, dynamic state, dynamic error) async {
    if ($instance == null || $instance is! IRTCEngineEventHandler) {
      return;
    }
    return ($instance as IRTCEngineEventHandler).onLocalProxyStateChanged?.call(
        t_LocalProxyType.ios_to_code($p_i.ByteRTCLocalProxyType.values
            .firstWhere((t) => t.$value == type || t.name == type)),
        t_LocalProxyState.ios_to_code($p_i.ByteRTCLocalProxyState.values
            .firstWhere((t) => t.$value == state || t.name == state)),
        t_LocalProxyError.ios_to_code($p_i.ByteRTCLocalProxyError.values
            .firstWhere((t) => t.$value == error || t.name == error)));
  }

  FutureOr<void> rtcEngine$onEffectError$msg(
      dynamic engine, dynamic error, dynamic msg) async {
    if ($instance == null || $instance is! IRTCEngineEventHandler) {
      return;
    }
    return ($instance as IRTCEngineEventHandler).onEffectError?.call(
        t_EffectErrorType.ios_to_code($p_i.ByteRTCEffectErrorType.values
            .firstWhere((t) => t.$value == error || t.name == error)),
        msg.toString());
  }

  FutureOr<void> rtcEngine$onStreamSyncInfoReceived$info$streamType$data(
      dynamic engine,
      dynamic streamId,
      dynamic info,
      dynamic streamType,
      dynamic data) async {
    if ($instance == null || $instance is! IRTCEngineEventHandler) {
      return;
    }
    return ($instance as IRTCEngineEventHandler).onStreamSyncInfoReceived?.call(
        streamId.toString(),
        packObject(
            info,
            () => StreamInfo.fromMap(StreamInfo.deepPackedMapValues(
                StreamInfo.mapMemberToConstructorParams(info)))),
        t_SyncInfoStreamType.ios_to_code($p_i.ByteRTCSyncInfoStreamType.values
            .firstWhere((t) => t.$value == streamType || t.name == streamType)),
        data);
  }

  FutureOr<void> rtcEngine$onExternalScreenFrameUpdate(
      dynamic engine, dynamic frameUpdateInfo) async {
    if ($instance == null || $instance is! IRTCEngineEventHandler) {
      return;
    }
    return ($instance as IRTCEngineEventHandler)
        .onExternalScreenFrameUpdate
        ?.call(
            packObject(
                engine,
                () => FrameUpdateInfo.fromMap(
                    FrameUpdateInfo.deepPackedMapValues(
                        FrameUpdateInfo.mapMemberToConstructorParams(engine)))),
            packObject(
                frameUpdateInfo,
                () => FrameUpdateInfo.fromMap(
                    FrameUpdateInfo.deepPackedMapValues(
                        FrameUpdateInfo.mapMemberToConstructorParams(
                            frameUpdateInfo)))));
  }

  FutureOr<void>
      rtcEngine$onRemoteSnapshotTakenToFile$info$filePath$width$height$errorCode$taskId(
          dynamic engine,
          dynamic streamId,
          dynamic info,
          dynamic filePath,
          dynamic width,
          dynamic height,
          dynamic errorCode,
          dynamic taskId) async {
    if ($instance == null || $instance is! IRTCEngineEventHandler) {
      return;
    }
    return ($instance as IRTCEngineEventHandler)
        .onRemoteSnapshotTakenToFile
        ?.call(
            streamId.toString(),
            filePath.toString(),
            int.tryParse(width.toString()) ?? 0,
            int.tryParse(height.toString()) ?? 0,
            t_SnapshotErrorCode.ios_to_code($p_i.ByteRTCSnapshotErrorCode.values
                .firstWhere(
                    (t) => t.$value == errorCode || t.name == errorCode)),
            int.tryParse(taskId.toString()) ?? 0);
  }

  FutureOr<void> rtcEngine$onAudioFrameSendStateChanged$info$rtcUser$state(
      dynamic engine,
      dynamic streamId,
      dynamic info,
      dynamic user,
      dynamic state) async {
    if ($instance == null || $instance is! IRTCEngineEventHandler) {
      return;
    }
    return ($instance
            as IRTCEngineEventHandler)
        .onAudioFrameSendStateChanged
        ?.call(
            streamId.toString(),
            packObject(
                info,
                () => StreamInfo.fromMap(
                    StreamInfo.deepPackedMapValues(
                        StreamInfo.mapMemberToConstructorParams(info)))),
            packObject(
                user,
                () => RtcUser.fromMap(RtcUser
                    .deepPackedMapValues(RtcUser.mapMemberToConstructorParams(
                        user)))),
            t_FirstFrameSendState.ios_to_code($p_i
                .ByteRTCFirstFrameSendState.values
                .firstWhere((t) => t.$value == state || t.name == state)));
  }

  FutureOr<void> rtcEngine$onVideoFrameSendStateChanged$info$rtcUser$state(
      dynamic engine,
      dynamic streamId,
      dynamic info,
      dynamic user,
      dynamic state) async {
    if ($instance == null || $instance is! IRTCEngineEventHandler) {
      return;
    }
    return ($instance
            as IRTCEngineEventHandler)
        .onVideoFrameSendStateChanged
        ?.call(
            streamId.toString(),
            packObject(
                info,
                () => StreamInfo.fromMap(
                    StreamInfo.deepPackedMapValues(
                        StreamInfo.mapMemberToConstructorParams(info)))),
            packObject(
                user,
                () => RtcUser.fromMap(RtcUser
                    .deepPackedMapValues(RtcUser.mapMemberToConstructorParams(
                        user)))),
            t_FirstFrameSendState.ios_to_code($p_i
                .ByteRTCFirstFrameSendState.values
                .firstWhere((t) => t.$value == state || t.name == state)));
  }

  FutureOr<void> rtcEngine$onAudioFramePlayStateChanged$info$rtcUser$state(
      dynamic engine,
      dynamic streamId,
      dynamic info,
      dynamic user,
      dynamic state) async {
    if ($instance == null || $instance is! IRTCEngineEventHandler) {
      return;
    }
    return ($instance
            as IRTCEngineEventHandler)
        .onAudioFramePlayStateChanged
        ?.call(
            streamId.toString(),
            packObject(
                info,
                () => StreamInfo.fromMap(
                    StreamInfo.deepPackedMapValues(
                        StreamInfo.mapMemberToConstructorParams(info)))),
            packObject(
                user,
                () => RtcUser.fromMap(RtcUser
                    .deepPackedMapValues(RtcUser.mapMemberToConstructorParams(
                        user)))),
            t_FirstFramePlayState.ios_to_code($p_i
                .ByteRTCFirstFramePlayState.values
                .firstWhere((t) => t.$value == state || t.name == state)));
  }

  FutureOr<void> rtcEngine$onVideoFramePlayStateChanged$info$rtcUser$state(
      dynamic engine,
      dynamic streamId,
      dynamic info,
      dynamic user,
      dynamic state) async {
    if ($instance == null || $instance is! IRTCEngineEventHandler) {
      return;
    }
    return ($instance
            as IRTCEngineEventHandler)
        .onVideoFramePlayStateChanged
        ?.call(
            streamId.toString(),
            packObject(
                info,
                () => StreamInfo.fromMap(
                    StreamInfo.deepPackedMapValues(
                        StreamInfo.mapMemberToConstructorParams(info)))),
            packObject(
                user,
                () => RtcUser.fromMap(RtcUser
                    .deepPackedMapValues(RtcUser.mapMemberToConstructorParams(
                        user)))),
            t_FirstFramePlayState.ios_to_code($p_i
                .ByteRTCFirstFramePlayState.values
                .firstWhere((t) => t.$value == state || t.name == state)));
  }

  FutureOr<void> rtcEngine$onSimulcastSubscribeFallback$info$event(
      dynamic engine, dynamic streamId, dynamic info, dynamic event) async {
    if ($instance == null || $instance is! IRTCEngineEventHandler) {
      return;
    }
    return ($instance as IRTCEngineEventHandler)
        .onSimulcastSubscribeFallback
        ?.call(
            streamId.toString(),
            packObject(
                info,
                () =>
                    StreamInfo.fromMap(
                        StreamInfo.deepPackedMapValues(
                            StreamInfo.mapMemberToConstructorParams(info)))),
            packObject(
                event,
                () => RemoteStreamSwitch.fromMap(
                    RemoteStreamSwitch.deepPackedMapValues(
                        RemoteStreamSwitch.mapMemberToConstructorParams(
                            event)))));
  }

  FutureOr<void>
      rtcEngine$onPerformanceAlarms$info$mode$reason$sourceWantedData(
          dynamic engine,
          dynamic streamId,
          dynamic info,
          dynamic mode,
          dynamic reason,
          dynamic data) async {
    if ($instance == null || $instance is! IRTCEngineEventHandler) {
      return;
    }
    return ($instance as IRTCEngineEventHandler).onPerformanceAlarms?.call(
        streamId.toString(),
        packObject(
            info,
            () => StreamInfo.fromMap(StreamInfo.deepPackedMapValues(
                StreamInfo.mapMemberToConstructorParams(info)))),
        t_PerformanceAlarmMode.ios_to_code($p_i
            .ByteRTCPerformanceAlarmMode.values
            .firstWhere((t) => t.$value == mode || t.name == mode)),
        t_PerformanceAlarmReason.ios_to_code($p_i
            .ByteRTCPerformanceAlarmReason.values
            .firstWhere((t) => t.$value == reason || t.name == reason)),
        packObject(
            data,
            () => SourceWantedData.fromMap(SourceWantedData.deepPackedMapValues(
                SourceWantedData.mapMemberToConstructorParams(data)))));
  }

  FutureOr<void> rtcEngine$onRemoteAudioPropertiesReportEx(
      dynamic engine, List<dynamic> audioPropertiesInfos) async {
    if ($instance == null || $instance is! IRTCEngineEventHandler) {
      return;
    }
    return ($instance as IRTCEngineEventHandler)
        .onRemoteAudioPropertiesReportEx
        ?.call(
            audioPropertiesInfos
                .map((e) => packObject(
                    e,
                    () => RemoteAudioPropertiesInfo.fromMap(
                        RemoteAudioPropertiesInfo.deepPackedMapValues(
                            RemoteAudioPropertiesInfo
                                .mapMemberToConstructorParams(e)))))
                .toList(),
            int.tryParse((await audioPropertiesInfos).toString()) ?? 0);
  }

  FutureOr<void> rtcEngine$onMixedStreamEvent$withMixedStreamInfo$withErrorCode(
      dynamic engine, dynamic event, dynamic info, dynamic errorCode) async {
    if ($instance == null || $instance is! IRTCEngineEventHandler) {
      return;
    }
    return ($instance as IRTCEngineEventHandler).onMixedStreamEvent?.call(
        packObject(info, () => MixedStreamTaskInfo()),
        t_MixedStreamTaskEvent.ios_to_code($p_i
            .ByteRTCMixedStreamTaskEvent.values
            .firstWhere((t) => t.$value == event || t.name == event)),
        t_MixedStreamTaskErrorCode.ios_to_code($p_i
            .ByteRTCMixedStreamTaskErrorCode.values
            .firstWhere((t) => t.$value == errorCode || t.name == errorCode)));
  }

  FutureOr<void> rtcEngine$onSingleStreamEvent$withTaskId$withErrorCode(
      dynamic engine, dynamic event, dynamic taskId, dynamic errorCode) async {
    if ($instance == null || $instance is! IRTCEngineEventHandler) {
      return;
    }
    return ($instance as IRTCEngineEventHandler).onSingleStreamEvent?.call(
        taskId.toString(),
        t_SingleStreamTaskEvent.ios_to_code($p_i
            .ByteRTCSingleStreamTaskEvent.values
            .firstWhere((t) => t.$value == event || t.name == event)),
        t_SingleStreamTaskErrorCode.ios_to_code($p_i
            .ByteRTCSingleStreamTaskErrorCode.values
            .firstWhere((t) => t.$value == errorCode || t.name == errorCode)));
  }

  FutureOr<void> rtcEngine$onExperimentalCallback(
      dynamic engine, dynamic param) async {
    if ($instance == null || $instance is! IRTCEngineEventHandler) {
      return;
    }
    return ($instance as IRTCEngineEventHandler)
        .onExperimentalCallback
        ?.call(param.toString());
  }

  FutureOr<void> rtcEngine$onPushPublicStreamResult$publicStreamId$errorCode(
      dynamic engine,
      dynamic roomId,
      dynamic streamId,
      dynamic errorCode) async {
    if ($instance == null || $instance is! IRTCEngineEventHandler) {
      return;
    }
    return ($instance as IRTCEngineEventHandler).onPushPublicStreamResult?.call(
        roomId.toString(),
        streamId.toString(),
        t_PublicStreamErrorCode.ios_to_code($p_i
            .ByteRTCPublicStreamErrorCode.values
            .firstWhere((t) => t.$value == errorCode || t.name == errorCode)));
  }
}

class android_IAudioFrameProcessor extends $p_a.IAudioFrameProcessor {
  android_IAudioFrameProcessor();
}

class ios_IAudioFrameProcessor extends $p_i.ByteRTCAudioFrameProcessor {
  ios_IAudioFrameProcessor();
}

class android_IClientMixedStreamObserver
    extends $p_a.IClientMixedStreamObserver {
  android_IClientMixedStreamObserver();

  FutureOr<void> onClientMixedStreamEvent(
      dynamic info, dynamic type, dynamic event, dynamic error) async {
    if ($instance == null || $instance is! IClientMixedStreamObserver) {
      return;
    }
    return ($instance as IClientMixedStreamObserver)
        .onClientMixedStreamEvent
        ?.call(
            packObject(info, () => MixedStreamTaskInfo()),
            t_MixedStreamType
                .android_to_code(
                    $p_a
                        .MixedStreamType.values
                        .firstWhere((t) => t.$value == type || t.name == type)),
            t_MixedStreamTaskEvent.android_to_code($p_a
                .MixedStreamTaskEvent.values
                .firstWhere((t) => t.$value == event || t.name == event)));
  }

  FutureOr<void> onMixedAudioFrame(dynamic taskId, dynamic audioFrame,
      dynamic frameNum, dynamic timeStampMs) async {
    if ($instance == null || $instance is! IClientMixedStreamObserver) {
      return;
    }
    return ($instance as IClientMixedStreamObserver).onMixedAudioFrame?.call(
        taskId.toString(),
        audioFrame,
        int.tryParse(timeStampMs.toString()) ?? 0);
  }

  FutureOr<void> onMixedVideoFrame(dynamic taskId, dynamic videoFrame) async {
    if ($instance == null || $instance is! IClientMixedStreamObserver) {
      return;
    }
    return ($instance as IClientMixedStreamObserver)
        .onMixedVideoFrame
        ?.call(taskId.toString(), packObject(videoFrame, () => IVideoFrame()));
  }

  FutureOr<void> onMixedDataFrame(
      dynamic taskId, dynamic dataFrame, dynamic time) async {
    if ($instance == null || $instance is! IClientMixedStreamObserver) {
      return;
    }
    return ($instance as IClientMixedStreamObserver)
        .onMixedDataFrame
        ?.call(taskId.toString(), dataFrame);
  }

  FutureOr<void> onMixedFirstAudioFrame(dynamic taskId) async {
    if ($instance == null || $instance is! IClientMixedStreamObserver) {
      return;
    }
    return ($instance as IClientMixedStreamObserver)
        .onMixedFirstAudioFrame
        ?.call(taskId.toString());
  }

  FutureOr<void> onMixedFirstVideoFrame(dynamic taskId) async {
    if ($instance == null || $instance is! IClientMixedStreamObserver) {
      return;
    }
    return ($instance as IClientMixedStreamObserver)
        .onMixedFirstVideoFrame
        ?.call(taskId.toString());
  }
}

class ios_IClientMixedStreamObserver
    extends $p_i.ByteRTCClientMixedStreamDelegate {
  ios_IClientMixedStreamObserver();

  FutureOr<void>
      onClientMixedStreamEvent$withTaskInfo$withMixedType$withErrorCode(
          dynamic event, dynamic info, dynamic type, dynamic errorCode) async {
    if ($instance == null || $instance is! IClientMixedStreamObserver) {
      return;
    }
    return ($instance as IClientMixedStreamObserver)
        .onClientMixedStreamEvent
        ?.call(
            packObject(info, () => MixedStreamTaskInfo()),
            t_MixedStreamType
                .ios_to_code(
                    $p_i
                        .ByteRTCMixedStreamType.values
                        .firstWhere((t) => t.$value == type || t.name == type)),
            t_MixedStreamTaskEvent.ios_to_code($p_i
                .ByteRTCMixedStreamTaskEvent.values
                .firstWhere((t) => t.$value == event || t.name == event)));
  }

  FutureOr<void> onMixedAudioFrame$withTimestamp$withTaskId(
      dynamic audioFrame, dynamic timeStamp, dynamic taskId) async {
    if ($instance == null || $instance is! IClientMixedStreamObserver) {
      return;
    }
    return ($instance as IClientMixedStreamObserver).onMixedAudioFrame?.call(
        taskId.toString(), audioFrame, int.tryParse(timeStamp.toString()) ?? 0);
  }

  FutureOr<void> onMixedVideoFrame$withTaskId(
      dynamic videoFrame, dynamic taskId) async {
    if ($instance == null || $instance is! IClientMixedStreamObserver) {
      return;
    }
    return ($instance as IClientMixedStreamObserver)
        .onMixedVideoFrame
        ?.call(taskId.toString(), packObject(videoFrame, () => IVideoFrame()));
  }

  FutureOr<void> onMixedDataFrame$withTaskId(
      dynamic dataFrame, dynamic taskId) async {
    if ($instance == null || $instance is! IClientMixedStreamObserver) {
      return;
    }
    return ($instance as IClientMixedStreamObserver)
        .onMixedDataFrame
        ?.call(taskId.toString(), dataFrame);
  }

  FutureOr<void> onMixedFirstAudioFrame(dynamic taskId) async {
    if ($instance == null || $instance is! IClientMixedStreamObserver) {
      return;
    }
    return ($instance as IClientMixedStreamObserver)
        .onMixedFirstAudioFrame
        ?.call(taskId.toString());
  }

  FutureOr<void> onMixedFirstVideoFrame(dynamic taskId) async {
    if ($instance == null || $instance is! IClientMixedStreamObserver) {
      return;
    }
    return ($instance as IClientMixedStreamObserver)
        .onMixedFirstVideoFrame
        ?.call(taskId.toString());
  }
}
