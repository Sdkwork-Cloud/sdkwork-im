/// Copyright (c) 2026 Beijing Volcano Engine Technology Ltd.

// ignore_for_file: camel_case_types, annotate_overrides, null_check_always_fails, unused_import, non_constant_identifier_names
import 'package:hybrid_runtime/hybrid_runtime.dart';
import 'dart:async';
import 'dart:typed_data';
import 'api.dart';
import 'types.dart';
import 'keytype.dart';
import 'errorcode.dart';
import 'external.dart';

class ByteRTCVideoProcessorDelegate extends NativeObserverClass {
  static const _$namespace = r'ByteRTCVideoProcessorDelegate';

  ByteRTCVideoProcessorDelegate([NativeClassOptions? options])
      : super(options == null
            ? const NativeClassOptions([],
                className: _$namespace,
                instanceType: InstanceType.manual,
                bridgeKey: 'com.volcengine.rtc.hybrid_runtime',
                methodMap: {})
            : NativeClassOptions.fromMap({
                ...options.toMap(),
                'bridgeKey': 'com.volcengine.rtc.hybrid_runtime',
              })) {}
}

class ByteRTCKTVPlayerDelegate extends NativeObserverClass {
  static const _$namespace = r'ByteRTCKTVPlayerDelegate';

  ByteRTCKTVPlayerDelegate([NativeClassOptions? options])
      : super(options == null
            ? const NativeClassOptions([],
                className: _$namespace,
                instanceType: InstanceType.manual,
                bridgeKey: 'com.volcengine.rtc.hybrid_runtime',
                methodMap: {
                  r"ktvPlayer$onPlayProgress$progress":
                      r"ktvPlayer:onPlayProgress:progress:",
                  r"ktvPlayer$onPlayStateChanged$state$error":
                      r"ktvPlayer:onPlayStateChanged:state:error:"
                })
            : NativeClassOptions.fromMap({
                ...options.toMap(),
                'bridgeKey': 'com.volcengine.rtc.hybrid_runtime',
              })) {
    registerEvent(r"ktvPlayer:onPlayProgress:progress:",
        ktvPlayer$onPlayProgress$progress);

    registerEvent(r"ktvPlayer:onPlayStateChanged:state:error:",
        ktvPlayer$onPlayStateChanged$state$error);
  }

  /// @detail callback
  /// @author lihuan.wuti2ha
  /// @brief 音乐播放进度回调。
  /// @param ktvPlayer 当前 ktvPlayer 对象，参看 ByteRTCKTVPlayer{@link #ByteRTCKTVPlayer}。
  /// @param musicId 音乐 ID。
  /// @param progress 音乐播放进度，单位为毫秒。

  FutureOr<void> ktvPlayer$onPlayProgress$progress(
      ByteRTCKTVPlayer ktvPlayer, NSString musicId, int64_t progress) async {}

  /// @detail callback
  /// @author lihuan.wuti2ha
  /// @brief 音乐播放状态改变回调。
  /// @param ktvPlayer 当前 ktvPlayer 对象，参看 ByteRTCKTVPlayer{@link #ByteRTCKTVPlayer}。
  /// @param musicId 音乐 ID。
  /// @param state 音乐播放状态，参看 ByteRTCPlayState{@link #ByteRTCPlayState}。
  /// @param error 错误码，参看 ByteRTCKTVPlayerErrorCode{@link #ByteRTCKTVPlayerErrorCode}。
  /// @note
  ///       此回调被触发的时机汇总如下： <br>
  ///       - 调用 playMusic:audioTrackType:audioPlayType:{@link #ByteRTCKTVPlayer#playMusic:audioTrackType:audioPlayType} 成功后，会触发 playState 值为 ByteRTCPlayStatePlaying 的回调；否则会触发 playState 值为 ByteRTCPlayStateFailed 的回调。
  ///       - 使用相同的音乐 ID 重复调用 playMusic:audioTrackType:audioPlayType:{@link #ByteRTCKTVPlayer#playMusic:audioTrackType:audioPlayType} 后，后一次播放会覆盖前一次，且会触发 playState 值为 ByteRTCPlayStatePlaying 的回调，表示后一次音乐播放已开始。
  ///       - 调用 pauseMusic:{@link #ByteRTCKTVPlayer#pauseMusic} 方法暂停播放成功后，会触发 playState 值为 ByteRTCPlayStatePaused 的回调；否则触发 playState 值为 ByteRTCPlayStateFailed 的回调。
  ///       - 调用 resumeMusic:{@link #ByteRTCKTVPlayer#resumeMusic} 方法恢复播放成功后，会触发 playState 值为 ByteRTCPlayStatePlaying 的回调；否则触发 playState 值为 ByteRTCPlayStateFailed 的回调。
  ///       - 调用 stopMusic:{@link #ByteRTCKTVPlayer#stopMusic} 方法停止播放成功后，会触发 playState 值为 ByteRTCPlayStateStoped 的回调；否则触发 playState 值为 ByteRTCPlayStateFailed 的回调。
  ///       - 音乐播放结束会触发 playState 值为 ByteRTCPlayStateFinished 的回调。

  FutureOr<void> ktvPlayer$onPlayStateChanged$state$error(
      ByteRTCKTVPlayer ktvPlayer,
      NSString musicId,
      ByteRTCPlayState state,
      ByteRTCKTVPlayerErrorCode error) async {}
}

class ByteRTCMediaMetadataObserver extends NativeObserverClass {
  static const _$namespace = r'ByteRTCMediaMetadataObserver';

  ByteRTCMediaMetadataObserver([NativeClassOptions? options])
      : super(options == null
            ? const NativeClassOptions([],
                className: _$namespace,
                instanceType: InstanceType.manual,
                bridgeKey: 'com.volcengine.rtc.hybrid_runtime',
                methodMap: {
                  r"receiveVideoFrameFromUID$withExtendedData$atTimestamp":
                      r"receiveVideoFrameFromUID:withExtendedData:atTimestamp:"
                })
            : NativeClassOptions.fromMap({
                ...options.toMap(),
                'bridgeKey': 'com.volcengine.rtc.hybrid_runtime',
              })) {
    registerEvent(r"receiveVideoFrameFromUID:withExtendedData:atTimestamp:",
        receiveVideoFrameFromUID$withExtendedData$atTimestamp);
  }

  /// @detail callback
  /// @author wangjunlin.3182
  /// @brief 当 SDK 收到的视频帧包含 medatada 时，会回调该接口
  /// @param uid 当前帧所属的用户 ID
  /// @param extendedData metadata
  /// @param timestamp 包含 metadata 视频帧的时间戳，单位为微秒
  /// @note 回调中不能做长时间逻辑处理，以免影响视频卡顿

  FutureOr<void> receiveVideoFrameFromUID$withExtendedData$atTimestamp(
      NSString uid, NSData extendedData, NSTimeInterval timestamp) async {}
}

class ByteRTCVideoFrameConsumerObserver extends NativeObserverClass {
  static const _$namespace = r'ByteRTCVideoFrameConsumerObserver';

  ByteRTCVideoFrameConsumerObserver([NativeClassOptions? options])
      : super(options == null
            ? const NativeClassOptions([],
                className: _$namespace,
                instanceType: InstanceType.manual,
                bridgeKey: 'com.volcengine.rtc.hybrid_runtime',
                methodMap: {
                  r"consumeYUV420Buffer$width$rotation$timestamp":
                      r"consumeYUV420Buffer:width:rotation:timestamp:",
                  r"consumeRGBABuffer$width$rotation$timestamp":
                      r"consumeRGBABuffer:width:rotation:timestamp:"
                })
            : NativeClassOptions.fromMap({
                ...options.toMap(),
                'bridgeKey': 'com.volcengine.rtc.hybrid_runtime',
              })) {
    registerEvent(r"consumeYUV420Buffer:width:rotation:timestamp:",
        consumeYUV420Buffer$width$rotation$timestamp);

    registerEvent(r"consumeRGBABuffer:width:rotation:timestamp:",
        consumeRGBABuffer$width$rotation$timestamp);
  }

  /// @brief 输出 yuv420 数据
  /// @param ocFrame 数据 Buffer
  /// @param width 视频宽度
  /// @param rotation 视频旋转角度
  /// @param timestamp 时间戳

  FutureOr<void> consumeYUV420Buffer$width$rotation$timestamp(
      CVPixelBufferRef ocFrame,
      NSInteger width,
      NSInteger rotation,
      long timestamp) async {}

  /// @brief 输出 RGBA 数据
  /// @param ocFrame 数据 Buffer
  /// @param width 视频宽度
  /// @param rotation 视频旋转角度
  /// @param timestamp 时间戳

  FutureOr<void> consumeRGBABuffer$width$rotation$timestamp(
      CVPixelBufferRef ocFrame,
      NSInteger width,
      NSInteger rotation,
      long timestamp) async {}
}

class ByteRTCMonitorDelegate extends NativeObserverClass {
  static const _$namespace = r'ByteRTCMonitorDelegate';

  ByteRTCMonitorDelegate([NativeClassOptions? options])
      : super(options == null
            ? const NativeClassOptions([],
                className: _$namespace,
                instanceType: InstanceType.manual,
                bridgeKey: 'com.volcengine.rtc.hybrid_runtime',
                methodMap: {
                  r"onMonitorLog$withType": r"onMonitorLog:withType:",
                  r"onVerboseLogWithLevel$filename$tag$line$functionName$format":
                      r"onVerboseLogWithLevel:filename:tag:line:functionName:format:"
                })
            : NativeClassOptions.fromMap({
                ...options.toMap(),
                'bridgeKey': 'com.volcengine.rtc.hybrid_runtime',
              })) {
    registerEvent(r"onMonitorLog:withType:", onMonitorLog$withType);

    registerEvent(
        r"onVerboseLogWithLevel:filename:tag:line:functionName:format:",
        onVerboseLogWithLevel$filename$tag$line$functionName$format);
  }

  /// @detail callback
  /// @brief 埋点日志回调
  /// @param data <br>
  ///        具体的埋点内容
  /// @param type <br>
  ///        埋点的类型

  FutureOr<void> onMonitorLog$withType(
      NSDictionary data, NSString type) async {}

  /// @detail callback
  /// @brief 输出更多的调试信息。
  /// @param level 日志等级，参看 ByteRTCLogLevel{@link #ByteRTCLogLevel}。
  /// @param filename 日志文件名称。
  /// @param tag 日志标签。
  /// @param line 行数
  /// @param funcName 函数名称
  /// @param format 格式

  FutureOr<void> onVerboseLogWithLevel$filename$tag$line$functionName$format(
      ByteRTCLogLevel level,
      NSString filename,
      NSString tag,
      int line,
      NSString funcName,
      NSString format) async {}
}

class ByteRTCRemoteEncodedVideoFrameObserver extends NativeObserverClass {
  static const _$namespace = r'ByteRTCRemoteEncodedVideoFrameObserver';

  ByteRTCRemoteEncodedVideoFrameObserver([NativeClassOptions? options])
      : super(options == null
            ? const NativeClassOptions([],
                className: _$namespace,
                instanceType: InstanceType.manual,
                bridgeKey: 'com.volcengine.rtc.hybrid_runtime',
                methodMap: {
                  r"onRemoteEncodedVideoFrame$info$withEncodedVideoFrame":
                      r"onRemoteEncodedVideoFrame:info:withEncodedVideoFrame:"
                })
            : NativeClassOptions.fromMap({
                ...options.toMap(),
                'bridgeKey': 'com.volcengine.rtc.hybrid_runtime',
              })) {
    registerEvent(r"onRemoteEncodedVideoFrame:info:withEncodedVideoFrame:",
        onRemoteEncodedVideoFrame$info$withEncodedVideoFrame);
  }

  /// @detail callback
  /// @author wangzhanqiang
  /// @brief 调用 registerRemoteEncodedVideoFrameObserver:{@link #ByteRTCEngine#registerRemoteEncodedVideoFrameObserver} 后，SDK 监测到远端编码后视频数据时，触发该回调
  /// @param streamId 收到的远端流 ID
  /// @param info 收到的远端流信息，参看 ByteRTCStreamInfo{@link #ByteRTCStreamInfo}
  /// @param videoFrame 收到的远端视频帧信息，参看 ByteRTCEncodedVideoFrame{@link #ByteRTCEncodedVideoFrame}

  FutureOr<void> onRemoteEncodedVideoFrame$info$withEncodedVideoFrame(
      NSString streamId,
      ByteRTCStreamInfo info,
      ByteRTCEncodedVideoFrame videoFrame) async {}
}

class ByteRTCSingScoringDelegate extends NativeObserverClass {
  static const _$namespace = r'ByteRTCSingScoringDelegate';

  ByteRTCSingScoringDelegate([NativeClassOptions? options])
      : super(options == null
            ? const NativeClassOptions([],
                className: _$namespace,
                instanceType: InstanceType.manual,
                bridgeKey: 'com.volcengine.rtc.hybrid_runtime',
                methodMap: {r"onCurrentScoringInfo": r"onCurrentScoringInfo:"})
            : NativeClassOptions.fromMap({
                ...options.toMap(),
                'bridgeKey': 'com.volcengine.rtc.hybrid_runtime',
              })) {
    registerEvent(r"onCurrentScoringInfo:", onCurrentScoringInfo);
  }

  /// @detail callback
  /// @author wangjunzheng
  /// @brief 实时评分信息回调。调用 startSingScoring:scoringInfoInterval:{@link #ByteRTCSingScoringManager#startSingScoring:scoringInfoInterval} 后，会收到该回调。
  /// @param info 实时评分信息。详见 ByteRTCSingScoringRealtimeInfo{@link #ByteRTCSingScoringRealtimeInfo}。

  FutureOr<void> onCurrentScoringInfo(
      ByteRTCSingScoringRealtimeInfo info) async {}
}

class ByteRTCGameRoomDelegate extends NativeObserverClass {
  static const _$namespace = r'ByteRTCGameRoomDelegate';

  ByteRTCGameRoomDelegate([NativeClassOptions? options])
      : super(options == null
            ? const NativeClassOptions([],
                className: _$namespace,
                instanceType: InstanceType.manual,
                bridgeKey: 'com.volcengine.rtc.hybrid_runtime',
                methodMap: {
                  r"rtcRoom$onRoomStateChangedWithReason$withUid$state$reason":
                      r"rtcRoom:onRoomStateChangedWithReason:withUid:state:reason:",
                  r"rtcRoom$onRoomStateChanged$withUid$state$extraInfo":
                      r"rtcRoom:onRoomStateChanged:withUid:state:extraInfo:",
                  r"rtcRoom$onStreamStateChanged$withUid$state$extraInfo":
                      r"rtcRoom:onStreamStateChanged:withUid:state:extraInfo:",
                  r"rtcRoom$onLeaveRoom": r"rtcRoom:onLeaveRoom:",
                  r"rtcRoom$onAVSyncStateChange":
                      r"rtcRoom:onAVSyncStateChange:",
                  r"rtcRoom$onVideoPublishStateChanged$info$state$reason":
                      r"rtcRoom:onVideoPublishStateChanged:info:state:reason:",
                  r"rtcRoom$onAudioPublishStateChanged$info$state$reason":
                      r"rtcRoom:onAudioPublishStateChanged:info:state:reason:",
                  r"rtcRoom$onScreenVideoPublishStateChanged$userId$state$reason":
                      r"rtcRoom:onScreenVideoPublishStateChanged:userId:state:reason:",
                  r"rtcRoom$onScreenAudioPublishStateChanged$userId$state$reason":
                      r"rtcRoom:onScreenAudioPublishStateChanged:userId:state:reason:",
                  r"rtcRoom$onVideoSubscribeStateChanged$info$state$reason":
                      r"rtcRoom:onVideoSubscribeStateChanged:info:state:reason:",
                  r"rtcRoom$onAudioSubscribeStateChanged$info$state$reason":
                      r"rtcRoom:onAudioSubscribeStateChanged:info:state:reason:",
                  r"rtcRoom$onScreenVideoSubscribeStateChanged$userId$state$reason":
                      r"rtcRoom:onScreenVideoSubscribeStateChanged:userId:state:reason:",
                  r"rtcRoom$onScreenAudioSubscribeStateChanged$userId$state$reason":
                      r"rtcRoom:onScreenAudioSubscribeStateChanged:userId:state:reason:",
                  r"rtcRoom$onRoomStats": r"rtcRoom:onRoomStats:",
                  r"rtcRoom$onRoomEvent$uid$state$info":
                      r"rtcRoom:onRoomEvent:uid:state:info:",
                  r"rtcRoom$onLocalStreamStats$info$stats":
                      r"rtcRoom:onLocalStreamStats:info:stats:",
                  r"rtcRoom$onRemoteStreamStats$info$stats":
                      r"rtcRoom:onRemoteStreamStats:info:stats:",
                  r"rtcRoom$onUserJoined": r"rtcRoom:onUserJoined:",
                  r"rtcRoom$onUserLeave$reason": r"rtcRoom:onUserLeave:reason:",
                  r"onTokenWillExpire": r"onTokenWillExpire:",
                  r"onPublishPrivilegeTokenWillExpire":
                      r"onPublishPrivilegeTokenWillExpire:",
                  r"onSubscribePrivilegeTokenWillExpire":
                      r"onSubscribePrivilegeTokenWillExpire:",
                  r"rtcRoom$onStreamPublishSuccess$isScreen":
                      r"rtcRoom:onStreamPublishSuccess:isScreen:",
                  r"rtcRoom$onAVSyncEvent$userId$eventCode":
                      r"rtcRoom:onAVSyncEvent:userId:eventCode:",
                  r"rtcRoom$onUserPublishStreamVideo$info$isPublish":
                      r"rtcRoom:onUserPublishStreamVideo:info:isPublish:",
                  r"rtcRoom$onUserPublishStreamAudio$info$isPublish":
                      r"rtcRoom:onUserPublishStreamAudio:info:isPublish:",
                  r"rtcRoom$onUserPublishScreenVideo$uid$isPublish":
                      r"rtcRoom:onUserPublishScreenVideo:uid:isPublish:",
                  r"rtcRoom$onUserPublishScreenAudio$uid$isPublish":
                      r"rtcRoom:onUserPublishScreenAudio:uid:isPublish:",
                  r"rtcRoom$onRoomMessageReceived$message":
                      r"rtcRoom:onRoomMessageReceived:message:",
                  r"rtcRoom$onRoomBinaryMessageReceived$message":
                      r"rtcRoom:onRoomBinaryMessageReceived:message:",
                  r"rtcRoom$onUserMessageReceived$message":
                      r"rtcRoom:onUserMessageReceived:message:",
                  r"rtcRoom$onUserBinaryMessageReceived$message":
                      r"rtcRoom:onUserBinaryMessageReceived:message:",
                  r"rtcRoom$onUserMessageSendResult$error":
                      r"rtcRoom:onUserMessageSendResult:error:",
                  r"rtcRoom$onRoomMessageSendResult$error":
                      r"rtcRoom:onRoomMessageSendResult:error:",
                  r"rtcRoom$onSetRoomExtraInfoResult$result":
                      r"rtcRoom:onSetRoomExtraInfoResult:result:",
                  r"rtcRoom$onRoomExtraInfoUpdate$value$lastUpdateUserId$lastUpdateTimeMs":
                      r"rtcRoom:onRoomExtraInfoUpdate:value:lastUpdateUserId:lastUpdateTimeMs:",
                  r"rtcRoom$onUserVisibilityChanged$errorCode":
                      r"rtcRoom:onUserVisibilityChanged:errorCode:",
                  r"rtcRoom$onVideoStreamBanned$isBanned":
                      r"rtcRoom:onVideoStreamBanned:isBanned:",
                  r"rtcRoom$onAudioStreamBanned$isBanned":
                      r"rtcRoom:onAudioStreamBanned:isBanned:",
                  r"rtcRoom$onForwardStreamStateChanged":
                      r"rtcRoom:onForwardStreamStateChanged:",
                  r"rtcRoom$onForwardStreamEvent":
                      r"rtcRoom:onForwardStreamEvent:",
                  r"rtcRoom$onNetworkQuality$remoteQualities":
                      r"rtcRoom:onNetworkQuality:remoteQualities:",
                  r"rtcRoom$onSubtitleStateChanged$errorCode$errorMessage":
                      r"rtcRoom:onSubtitleStateChanged:errorCode:errorMessage:",
                  r"rtcRoom$onSubtitleMessageReceived":
                      r"rtcRoom:onSubtitleMessageReceived:",
                  r"rtcRoom$onRoomWarning": r"rtcRoom:onRoomWarning:",
                  r"rtcRoom$onStreamAdd": r"rtcRoom:onStreamAdd:",
                  r"rtcRoom$onStreamRemove$stream$reason":
                      r"rtcRoom:onStreamRemove:stream:reason:"
                })
            : NativeClassOptions.fromMap({
                ...options.toMap(),
                'bridgeKey': 'com.volcengine.rtc.hybrid_runtime',
              })) {
    registerEvent(r"rtcRoom:onRoomStateChangedWithReason:withUid:state:reason:",
        rtcRoom$onRoomStateChangedWithReason$withUid$state$reason);

    registerEvent(r"rtcRoom:onRoomStateChanged:withUid:state:extraInfo:",
        rtcRoom$onRoomStateChanged$withUid$state$extraInfo);

    registerEvent(r"rtcRoom:onStreamStateChanged:withUid:state:extraInfo:",
        rtcRoom$onStreamStateChanged$withUid$state$extraInfo);

    registerEvent(r"rtcRoom:onLeaveRoom:", rtcRoom$onLeaveRoom);

    registerEvent(r"rtcRoom:onAVSyncStateChange:", rtcRoom$onAVSyncStateChange);

    registerEvent(r"rtcRoom:onVideoPublishStateChanged:info:state:reason:",
        rtcRoom$onVideoPublishStateChanged$info$state$reason);

    registerEvent(r"rtcRoom:onAudioPublishStateChanged:info:state:reason:",
        rtcRoom$onAudioPublishStateChanged$info$state$reason);

    registerEvent(
        r"rtcRoom:onScreenVideoPublishStateChanged:userId:state:reason:",
        rtcRoom$onScreenVideoPublishStateChanged$userId$state$reason);

    registerEvent(
        r"rtcRoom:onScreenAudioPublishStateChanged:userId:state:reason:",
        rtcRoom$onScreenAudioPublishStateChanged$userId$state$reason);

    registerEvent(r"rtcRoom:onVideoSubscribeStateChanged:info:state:reason:",
        rtcRoom$onVideoSubscribeStateChanged$info$state$reason);

    registerEvent(r"rtcRoom:onAudioSubscribeStateChanged:info:state:reason:",
        rtcRoom$onAudioSubscribeStateChanged$info$state$reason);

    registerEvent(
        r"rtcRoom:onScreenVideoSubscribeStateChanged:userId:state:reason:",
        rtcRoom$onScreenVideoSubscribeStateChanged$userId$state$reason);

    registerEvent(
        r"rtcRoom:onScreenAudioSubscribeStateChanged:userId:state:reason:",
        rtcRoom$onScreenAudioSubscribeStateChanged$userId$state$reason);

    registerEvent(r"rtcRoom:onRoomStats:", rtcRoom$onRoomStats);

    registerEvent(r"rtcRoom:onRoomEvent:uid:state:info:",
        rtcRoom$onRoomEvent$uid$state$info);

    registerEvent(r"rtcRoom:onLocalStreamStats:info:stats:",
        rtcRoom$onLocalStreamStats$info$stats);

    registerEvent(r"rtcRoom:onRemoteStreamStats:info:stats:",
        rtcRoom$onRemoteStreamStats$info$stats);

    registerEvent(r"rtcRoom:onUserJoined:", rtcRoom$onUserJoined);

    registerEvent(r"rtcRoom:onUserLeave:reason:", rtcRoom$onUserLeave$reason);

    registerEvent(r"onTokenWillExpire:", onTokenWillExpire);

    registerEvent(r"onPublishPrivilegeTokenWillExpire:",
        onPublishPrivilegeTokenWillExpire);

    registerEvent(r"onSubscribePrivilegeTokenWillExpire:",
        onSubscribePrivilegeTokenWillExpire);

    registerEvent(r"rtcRoom:onStreamPublishSuccess:isScreen:",
        rtcRoom$onStreamPublishSuccess$isScreen);

    registerEvent(r"rtcRoom:onAVSyncEvent:userId:eventCode:",
        rtcRoom$onAVSyncEvent$userId$eventCode);

    registerEvent(r"rtcRoom:onUserPublishStreamVideo:info:isPublish:",
        rtcRoom$onUserPublishStreamVideo$info$isPublish);

    registerEvent(r"rtcRoom:onUserPublishStreamAudio:info:isPublish:",
        rtcRoom$onUserPublishStreamAudio$info$isPublish);

    registerEvent(r"rtcRoom:onUserPublishScreenVideo:uid:isPublish:",
        rtcRoom$onUserPublishScreenVideo$uid$isPublish);

    registerEvent(r"rtcRoom:onUserPublishScreenAudio:uid:isPublish:",
        rtcRoom$onUserPublishScreenAudio$uid$isPublish);

    registerEvent(r"rtcRoom:onRoomMessageReceived:message:",
        rtcRoom$onRoomMessageReceived$message);

    registerEvent(r"rtcRoom:onRoomBinaryMessageReceived:message:",
        rtcRoom$onRoomBinaryMessageReceived$message);

    registerEvent(r"rtcRoom:onUserMessageReceived:message:",
        rtcRoom$onUserMessageReceived$message);

    registerEvent(r"rtcRoom:onUserBinaryMessageReceived:message:",
        rtcRoom$onUserBinaryMessageReceived$message);

    registerEvent(r"rtcRoom:onUserMessageSendResult:error:",
        rtcRoom$onUserMessageSendResult$error);

    registerEvent(r"rtcRoom:onRoomMessageSendResult:error:",
        rtcRoom$onRoomMessageSendResult$error);

    registerEvent(r"rtcRoom:onSetRoomExtraInfoResult:result:",
        rtcRoom$onSetRoomExtraInfoResult$result);

    registerEvent(
        r"rtcRoom:onRoomExtraInfoUpdate:value:lastUpdateUserId:lastUpdateTimeMs:",
        rtcRoom$onRoomExtraInfoUpdate$value$lastUpdateUserId$lastUpdateTimeMs);

    registerEvent(r"rtcRoom:onUserVisibilityChanged:errorCode:",
        rtcRoom$onUserVisibilityChanged$errorCode);

    registerEvent(r"rtcRoom:onVideoStreamBanned:isBanned:",
        rtcRoom$onVideoStreamBanned$isBanned);

    registerEvent(r"rtcRoom:onAudioStreamBanned:isBanned:",
        rtcRoom$onAudioStreamBanned$isBanned);

    registerEvent(r"rtcRoom:onForwardStreamStateChanged:",
        rtcRoom$onForwardStreamStateChanged);

    registerEvent(
        r"rtcRoom:onForwardStreamEvent:", rtcRoom$onForwardStreamEvent);

    registerEvent(r"rtcRoom:onNetworkQuality:remoteQualities:",
        rtcRoom$onNetworkQuality$remoteQualities);

    registerEvent(r"rtcRoom:onSubtitleStateChanged:errorCode:errorMessage:",
        rtcRoom$onSubtitleStateChanged$errorCode$errorMessage);

    registerEvent(r"rtcRoom:onSubtitleMessageReceived:",
        rtcRoom$onSubtitleMessageReceived);

    registerEvent(r"rtcRoom:onRoomWarning:", rtcRoom$onRoomWarning);

    registerEvent(r"rtcRoom:onStreamAdd:", rtcRoom$onStreamAdd);

    registerEvent(r"rtcRoom:onStreamRemove:stream:reason:",
        rtcRoom$onStreamRemove$stream$reason);
  }

  /// @hidden
  /// @detail callback
  /// @author luomingkang
  /// @brief 游戏房间状态改变回调，加入房间、异常退出房间、发生房间相关的警告或错误时会收到此回调。
  /// @param rtcRoom ByteRTCGameRoom 实例。
  /// @param roomId 房间 ID。
  /// @param uid 用户 ID。
  /// @param state 房间状态码。 <br>
  ///              - 0: 加入房间成功。
  ///              - 1: 加入房间失败、异常退房、发生房间相关的警告或错误。
  ///              - 2: 离开房间。
  /// @param reason 房间状态发生变化的原因。参看 ByteRTCRoomStateChangeReason{@link #ByteRTCRoomStateChangeReason}。
  ///

  FutureOr<void> rtcRoom$onRoomStateChangedWithReason$withUid$state$reason(
      ByteRTCGameRoom rtcRoom,
      NSString roomId,
      NSString uid,
      ByteRTCRoomState state,
      ByteRTCRoomStateChangeReason reason) async {}

  /// @detail callback
  /// @author luomingkang
  /// @brief 游戏房间状态改变回调，加入房间、异常退出房间、发生房间相关的警告或错误时会收到此回调。
  /// @param rtcRoom ByteRTCGameRoom 实例。
  /// @param roomId 房间 ID。
  /// @param uid 用户 ID。
  /// @param state 房间状态码。 <br>
  ///              - 0: 加入房间成功。
  ///              - !0: 加入房间失败、异常退房、发生房间相关的警告或错误。具体原因参看 ByteRTCErrorCode{@link #ByteRTCErrorCode} 及 ByteRTCWarningCode{@link #ByteRTCWarningCode}。
  /// @param extraInfo 额外信息，如 `{"elapsed":1187,"join_type":0}`。 <br>
  ///                  `join_type`表示加入房间的类型，`0`为首次进房，`1`为重连进房。 <br>
  ///                  `elapsed`表示加入房间耗时，即本地用户从调用 joinRoom:userInfo:{@link #ByteRTCGameRoom#joinRoom:userInfo} 到加入房间成功所经历的时间间隔，单位为 ms。
  /// @order 0
  ///

  FutureOr<void> rtcRoom$onRoomStateChanged$withUid$state$extraInfo(
      ByteRTCGameRoom rtcRoom,
      NSString roomId,
      NSString uid,
      NSInteger state,
      NSString extraInfo) async {}

  /// @detail callback
  /// @author luomingkang
  /// @brief 音频流状态改变回调，发生流相关的警告或错误时会收到此回调。
  /// @param rtcRoom ByteRTCGameRoom 实例。
  /// @param roomId 房间 ID。
  /// @param uid 用户 ID。
  /// @param state 流状态码，参看 ByteRTCErrorCode{@link #ByteRTCErrorCode} 及 ByteRTCWarningCode{@link #ByteRTCWarningCode}。
  /// @param extraInfo 附加信息，默认为空。

  FutureOr<void> rtcRoom$onStreamStateChanged$withUid$state$extraInfo(
      ByteRTCGameRoom rtcRoom,
      NSString roomId,
      NSString uid,
      NSInteger state,
      NSString extraInfo) async {}

  /// @detail callback
  /// @author luomingkang
  /// @brief 离开游戏房间成功回调。 <br>
  ///        用户调用 leaveRoom{@link #ByteRTCGameRoom#leaveRoom} 方法后，SDK 会停止所有的发布订阅流，并释放所有通话相关的音视频资源。SDK 完成所有的资源释放后通过此回调通知用户。
  /// @param rtcRoom ByteRTCGameRoom 实例。
  /// @param stats 保留参数，目前为空。
  /// @note
  ///       - 用户调用 leaveRoom{@link #ByteRTCGameRoom#leaveRoom} 方法离开房间后，如果立即调用 destroy{@link #ByteRTCGameRoom#destroy} 销毁房间实例方法销毁 RTC 引擎，则将无法收到此回调事件。
  ///       - 离开游戏房间结束通话后，如果 App 需要使用系统音视频设备，则建议在收到此回调后再初始化音视频设备，否则可能由于 SDK 占用了导致 App 初始化音视频设备失败。
  /// @order 2

  FutureOr<void> rtcRoom$onLeaveRoom(
      ByteRTCGameRoom rtcRoom, ByteRTCRoomStats stats) async {}

  /// @detail callback
  /// @author wangzhanqiang
  /// @brief 发布端调用 setMultiDeviceAVSync:{@link #ByteRTCRoom#setMultiDeviceAVSync} 后音视频同步状态发生改变时，会收到此回调。
  /// @param rtcRoom ByteRTCGameRoom 实例。
  /// @param state 音视频同步状态，参看 ByteRTCAVSyncState{@link #ByteRTCAVSyncState}。

  FutureOr<void> rtcRoom$onAVSyncStateChange(
      ByteRTCGameRoom rtcRoom, ByteRTCAVSyncState state) async {}

  /// @detail callback
  /// @region 房间管理
  /// @author xuyiling.x10
  /// @brief 视频发布状态改变回调。
  /// @param rtcRoom ByteRTCGameRoom 实例。
  /// @param streamId 视频流 ID。
  /// @param info 视频流信息，参看 ByteRTCStreamInfo{@link #ByteRTCStreamInfo}。
  /// @param state 发布状态码，参看 ByteRTCPublishState{@link #ByteRTCPublishState}。
  /// @param reason 本端视频流发布状态改变的具体原因，参看 ByteRTCPublishStateChangeReason{@link #ByteRTCPublishStateChangeReason}。
  /// @order 0

  FutureOr<void> rtcRoom$onVideoPublishStateChanged$info$state$reason(
      ByteRTCGameRoom rtcRoom,
      NSString streamId,
      ByteRTCStreamInfo info,
      ByteRTCPublishState state,
      ByteRTCPublishStateChangeReason reason) async {}

  /// @detail callback
  /// @author xuyiling.x10
  /// @brief 音频发布状态改变回调。
  /// @param rtcRoom ByteRTCGameRoom 实例。
  /// @param streamId 音频流 ID。
  /// @param info 音频流信息，参看 ByteRTCStreamInfo{@link #ByteRTCStreamInfo}。
  /// @param state 发布状态码，参看 ByteRTCPublishState{@link #ByteRTCPublishState}。
  /// @param reason 本端音频流发布状态改变的具体原因，参看 ByteRTCPublishStateChangeReason{@link #ByteRTCPublishStateChangeReason}。
  /// @order 0

  FutureOr<void> rtcRoom$onAudioPublishStateChanged$info$state$reason(
      ByteRTCGameRoom rtcRoom,
      NSString streamId,
      ByteRTCStreamInfo info,
      ByteRTCPublishState state,
      ByteRTCPublishStateChangeReason reason) async {}

  /// @detail callback
  /// @region 房间管理
  /// @author xuyiling.x10
  /// @brief 屏幕流视频发布状态改变回调。
  /// @param rtcRoom ByteRTCGameRoom 实例。
  /// @param roomId 房间 ID。
  /// @param uid 用户 ID。
  /// @param state 发布状态码，参看 ByteRTCPublishState{@link #ByteRTCPublishState}。
  /// @param reason 屏幕流视频发布状态改变的具体原因，参看 ByteRTCPublishStateChangeReason{@link #ByteRTCPublishStateChangeReason}。
  /// @order 0

  FutureOr<void> rtcRoom$onScreenVideoPublishStateChanged$userId$state$reason(
      ByteRTCGameRoom rtcRoom,
      NSString roomId,
      NSString uid,
      ByteRTCPublishState state,
      ByteRTCPublishStateChangeReason reason) async {}

  /// @detail callback
  /// @region 房间管理
  /// @author xuyiling.x10
  /// @brief 屏幕流音频发布状态改变回调。
  /// @param rtcRoom ByteRTCGameRoom 实例。
  /// @param roomId 房间 ID。
  /// @param uid 用户 ID。
  /// @param state 发布状态码，参看 ByteRTCPublishState{@link #ByteRTCPublishState}。
  /// @param reason 屏幕流音频发布状态改变的具体原因，参看 ByteRTCPublishStateChangeReason{@link #ByteRTCPublishStateChangeReason}。
  /// @order 0

  FutureOr<void> rtcRoom$onScreenAudioPublishStateChanged$userId$state$reason(
      ByteRTCGameRoom rtcRoom,
      NSString roomId,
      NSString uid,
      ByteRTCPublishState state,
      ByteRTCPublishStateChangeReason reason) async {}

  /// @detail callback
  /// @author xuyiling.x10
  /// @brief 视频订阅状态发生改变回调。
  /// @param rtcRoom ByteRTCGameRoom 实例。
  /// @param streamId 视频流 ID。
  /// @param info 视频流信息，参看 ByteRTCStreamInfo{@link #ByteRTCStreamInfo}。
  /// @param state 订阅状态码，参看 ByteRTCSubscribeState{@link #ByteRTCSubscribeState}。
  /// @param reason 视频订阅状态改变的具体原因，参看 ByteRTCSubscribeStateChangeReason{@link #ByteRTCSubscribeStateChangeReason}。
  /// @order 0

  FutureOr<void> rtcRoom$onVideoSubscribeStateChanged$info$state$reason(
      ByteRTCGameRoom rtcRoom,
      NSString streamId,
      ByteRTCStreamInfo info,
      ByteRTCSubscribeState state,
      ByteRTCSubscribeStateChangeReason reason) async {}

  /// @detail callback
  /// @author xuyiling.x10
  /// @brief 音频订阅状态发生改变回调。
  /// @param rtcRoom ByteRTCGameRoom 实例。
  /// @param streamId 音频流 ID.
  /// @param info 音频流信息，参看 ByteRTCStreamInfo{@link #ByteRTCStreamInfo}。
  /// @param state 订阅状态码，参看 ByteRTCSubscribeState{@link #ByteRTCSubscribeState}。
  /// @param reason 音频订阅状态改变的具体原因，参看 ByteRTCSubscribeStateChangeReason{@link #ByteRTCSubscribeStateChangeReason}。
  /// @order 0

  FutureOr<void> rtcRoom$onAudioSubscribeStateChanged$info$state$reason(
      ByteRTCGameRoom rtcRoom,
      NSString streamId,
      ByteRTCStreamInfo info,
      ByteRTCSubscribeState state,
      ByteRTCSubscribeStateChangeReason reason) async {}

  /// @detail callback
  /// @author xuyiling.x10
  /// @brief 屏幕流视频订阅状态发生改变回调。
  /// @param rtcRoom ByteRTCGameRoom 实例。
  /// @param roomId 房间 ID。
  /// @param uid 用户 ID。
  /// @param state 订阅状态码，参看 ByteRTCSubscribeState{@link #ByteRTCSubscribeState}。
  /// @param reason 屏幕流视频订阅状态改变的具体原因，参看 ByteRTCSubscribeStateChangeReason{@link #ByteRTCSubscribeStateChangeReason}。
  /// @order 0

  FutureOr<void> rtcRoom$onScreenVideoSubscribeStateChanged$userId$state$reason(
      ByteRTCGameRoom rtcRoom,
      NSString roomId,
      NSString uid,
      ByteRTCSubscribeState state,
      ByteRTCSubscribeStateChangeReason reason) async {}

  /// @detail callback
  /// @author xuyiling.x10
  /// @brief 屏幕流音频订阅状态发生改变回调。
  /// @param rtcRoom ByteRTCGameRoom 实例。
  /// @param roomId 房间 ID。
  /// @param uid 用户 ID。
  /// @param state 订阅状态码，参看 ByteRTCSubscribeState{@link #ByteRTCSubscribeState}。
  /// @param reason 屏幕流音频订阅状态改变的具体原因，参看 ByteRTCSubscribeStateChangeReason{@link #ByteRTCSubscribeStateChangeReason}。
  /// @order 0

  FutureOr<void> rtcRoom$onScreenAudioSubscribeStateChanged$userId$state$reason(
      ByteRTCGameRoom rtcRoom,
      NSString roomId,
      NSString uid,
      ByteRTCSubscribeState state,
      ByteRTCSubscribeStateChangeReason reason) async {}

  /// @detail callback
  /// @region 多房间
  /// @author yejing
  /// @brief 房间内通话统计信息回调。 <br>
  ///        用户进房开始通话后，每 2s 收到一次本回调。
  /// @param rtcRoom ByteRTCGameRoom 实例。
  /// @param stats 当前 ByteRTCGameRoom 统计数据，详见：ByteRTCRoomStats{@link #ByteRTCRoomStats}。

  FutureOr<void> rtcRoom$onRoomStats(
      ByteRTCGameRoom rtcRoom, ByteRTCRoomStats stats) async {}

  /// @hidden 仅内部使用
  /// @detail callback
  /// @region 多房间
  /// @valid since 3.60.
  /// @author taoshasha
  /// @brief 房间事件回调。
  /// @param rtcRoom ByteRTCGameRoom 实例。
  /// @param roomId 房间 ID。
  /// @param uid 用户 ID。
  /// @param state 房间事件状态。详见 ByteRTCRoomEvent{@link #ByteRTCRoomEvent}。
  /// @param info 房间封禁时，包含封禁时间。详见 ByteRTCRoomEventInfo{@link #ByteRTCRoomEventInfo}。
  /// @order 0
  ///

  FutureOr<void> rtcRoom$onRoomEvent$uid$state$info(
      ByteRTCGameRoom rtcRoom,
      NSString roomId,
      NSString uid,
      ByteRTCRoomEvent state,
      ByteRTCRoomEventInfo info) async {}

  /// @detail callback
  /// @author yejing
  /// @brief 本地流数据统计以及网络质量回调。 <br>
  ///        本地用户发布流成功后，SDK 会周期性（2s）的通过此回调事件通知用户发布的流在此次统计周期内的质量统计信息。 <br>
  ///        统计信息通过 ByteRTCLocalStreamStats{@link #ByteRTCLocalStreamStats} 类型的回调参数传递给用户，其中包括发送音视频比特率、发送帧率、编码帧率，网络质量等。
  /// @param rtcRoom ByteRTCGameRoom 实例。
  /// @param streamId 流 ID。
  /// @param info 流信息，参看 ByteRTCStreamInfo{@link #ByteRTCStreamInfo}。
  /// @param stats 当前房间本地流数据统计。详见：ByteRTCLocalStreamStats{@link #ByteRTCLocalStreamStats}

  FutureOr<void> rtcRoom$onLocalStreamStats$info$stats(
      ByteRTCGameRoom rtcRoom,
      NSString streamId,
      ByteRTCStreamInfo info,
      ByteRTCLocalStreamStats stats) async {}

  /// @detail callback
  /// @author yejing
  /// @brief 本地订阅的远端音/视频流数据统计以及网络质量回调。 <br>
  ///        本地用户订阅流成功后，SDK 会周期性（2s）的通过此回调事件通知用户订阅的流在此次统计周期内的质量统计信息，包括：发送音视频比特率、发送帧率、编码帧率，网络质量等。
  /// @param rtcRoom ByteRTCGameRoom 实例。
  /// @param streamId 流 ID。
  /// @param info 流信息，参看 ByteRTCStreamInfo{@link #ByteRTCStreamInfo}。
  /// @param stats 当前房间本地流数据统计。 详见：ByteRTCRemoteStreamStats{@link #ByteRTCRemoteStreamStats}

  FutureOr<void> rtcRoom$onRemoteStreamStats$info$stats(
      ByteRTCGameRoom rtcRoom,
      NSString streamId,
      ByteRTCStreamInfo info,
      ByteRTCRemoteStreamStats stats) async {}

  /// @detail callback
  /// @author luomingkang
  /// @brief 远端可见用户加入房间，或房内不可见用户切换为可见的回调。 <br>
  ///        1. 远端可见用户断网后重新连入房间时，房间内其他用户将收到该事件。 <br>
  ///        2. 新进房用户会收到进房前已在房内的可见用户的进房回调通知。
  /// @param rtcRoom ByteRTCGameRoom 实例。
  /// @param userInfo 用户信息，参看 ByteRTCUserInfo{@link #ByteRTCUserInfo}。

  FutureOr<void> rtcRoom$onUserJoined(
      ByteRTCGameRoom rtcRoom, ByteRTCUserInfo userInfo) async {}

  /// @detail callback
  /// @author luomingkang
  /// @brief 远端用户离开游戏房间，或切至不可见时，本地用户会收到此事件
  /// @param rtcRoom `ByteRTCGameRoom` 实例
  /// @param uid 离开房间，或切至不可见的的远端用户 ID。
  /// @param reason 用户离开房间的原因： <br>
  ///              - 0: 远端用户调用 leaveRoom{@link #ByteRTCGameRoom#leaveRoom} 主动退出房间。
  ///              - 1: 远端用户因 Token 过期或网络原因等掉线。详细信息请参看[连接状态提示](https://www.volcengine.com/docs/6348/95376)
  ///              - 3: 服务端调用 OpenAPI 将远端用户踢出房间。

  FutureOr<void> rtcRoom$onUserLeave$reason(ByteRTCGameRoom rtcRoom,
      NSString uid, ByteRTCUserOfflineReason reason) async {}

  /// @detail callback
  /// @author luomingkang
  /// @brief 当 SDK 检测到 Token 的进房权限将在 30 秒内过期时，触发该回调。
  ///        收到该回调后，你需调用 updateToken:{@link #ByteRTCGameRoom#updateToken} 更新 Token 进房权限。
  /// @param rtcRoom `ByteRTCGameRoom` 实例
  /// @note 若 Token 进房权限过期且未及时更新： <br>
  ///        - 用户此时尝试进房会收到 rtcRoom:onRoomStateChangedWithReason:withUid:state:reason:{@link #ByteRTCGameRoomDelegate#rtcRoom:onRoomStateChangedWithReason:withUid:state:reason}  回调，提示错误码为 `-1000` Token 无效；
  ///        - 用户已在房间内则会被移出房间，本地用户会收到 rtcRoom:onRoomStateChangedWithReason:withUid:state:reason:{@link #ByteRTCGameRoomDelegate#rtcRoom:onRoomStateChangedWithReason:withUid:state:reason}  回调，提示错误码为 `-1009` Token 过期，同时远端用户会收到 rtcRoom:onUserLeave:reason:{@link #ByteRTCGameRoomDelegate#rtcRoom:onUserLeave:reason} 回调，提示原因为 `1` Token 进房权限过期。

  FutureOr<void> onTokenWillExpire(ByteRTCGameRoom rtcRoom) async {}

  /// @detail callback
  /// @author qipengxiang
  /// @brief Token 发布权限过期前 30 秒将触发该回调。 <br>
  ///        收到该回调后，你需调用 updateToken:{@link #ByteRTCGameRoom#updateToken} 更新 Token 发布权限。
  /// @param rtcRoom `ByteRTCGameRoom` 实例
  /// @note Token 发布权限过期后：
  ///        - 已发布流或尝试发布流时，本端会收到 rtcRoom:onVideoPublishStateChanged:info:state:reason:{@link #ByteRTCGameRoomDelegate#rtcRoom:onVideoPublishStateChanged:info:state:reason}、rtcRoom:onAudioPublishStateChanged:info:state:reason:{@link #ByteRTCGameRoomDelegate#rtcRoom:onAudioPublishStateChanged:info:state:reason}、rtcRoom:onScreenVideoPublishStateChanged:userId:state:reason:{@link #ByteRTCGameRoomDelegate#rtcRoom:onScreenVideoPublishStateChanged:userId:state:reason}、rtcRoom:onScreenAudioPublishStateChanged:userId:state:reason:{@link #ByteRTCGameRoomDelegate#rtcRoom:onScreenAudioPublishStateChanged:userId:state:reason} 回调，提示`kPublishStateChangeReasonNoPublishPermission`，没有发布权限。
  ///        - 发布中的流将停止发布。远端用户会收到 rtcRoom:onUserPublishStreamVideo:info:isPublish:{@link #ByteRTCGameRoomDelegate#rtcRoom:onUserPublishStreamVideo:info:isPublish}、rtcRoom:onUserPublishStreamAudio:info:isPublish:{@link #ByteRTCGameRoomDelegate#rtcRoom:onUserPublishStreamAudio:info:isPublish}、rtcRoom:onUserPublishScreenVideo:uid:isPublish:{@link #ByteRTCGameRoomDelegate#rtcRoom:onUserPublishScreenVideo:uid:isPublish} 或 rtcRoom:onUserPublishScreenAudio:uid:isPublish:{@link #ByteRTCGameRoomDelegate#rtcRoom:onUserPublishScreenAudio:uid:isPublish} 回调，提示该流已停止发布。
  /// @order 3

  FutureOr<void> onPublishPrivilegeTokenWillExpire(
      ByteRTCGameRoom rtcRoom) async {}

  /// @detail callback
  /// @author qipengxiang
  /// @brief Token 订阅权限过期前 30 秒将触发该回调。 <br>
  ///        收到该回调后，你需调用 updateToken:{@link #ByteRTCGameRoom#updateToken} 更新 Token 订阅权限有效期。
  /// @param rtcRoom `ByteRTCGameRoom` 实例
  /// @note 若收到该回调后未及时更新 Token，Token 订阅权限过期后，尝试新订阅流会失败，已订阅的流会取消订阅，并且会收到 rtcRoom:onStreamStateChanged:withUid:state:extraInfo:{@link #ByteRTCGameRoomDelegate#rtcRoom:onStreamStateChanged:withUid:state:extraInfo} 回调，提示错误码为 `-1003` 没有订阅权限。

  FutureOr<void> onSubscribePrivilegeTokenWillExpire(
      ByteRTCGameRoom rtcRoom) async {}

  /// @hidden for internal use only
  /// @detail callback
  /// @region 多房间
  /// @author luomingkang
  /// @brief 当发布流成功的时候回调该事件
  /// @param rtcRoom ByteRTCGameRoom 实例。
  /// @param userId 用户 ID
  /// @param isScreen 该流是否是屏幕流 <br>

  FutureOr<void> rtcRoom$onStreamPublishSuccess$isScreen(
      ByteRTCGameRoom rtcRoom, NSString userId, BOOL isScreen) async {}

  /// @detail callback
  /// @valid since 3.60.
  /// @author xuyiling.x10
  /// @brief 发布端调用 setMultiDeviceAVSync:{@link #ByteRTCRoom#setMultiDeviceAVSync} 后音视频同步状态发生错误时，会收到此回调。
  /// @param rtcRoom ByteRTCGameRoom 实例。
  /// @param roomId 房间 ID。
  /// @param userId 用户 ID。
  /// @param eventCode 音视频同步状态错误，参看 ByteRTCAVSyncEvent{@link #ByteRTCAVSyncEvent}。
  /// @order 1
  ///

  FutureOr<void> rtcRoom$onAVSyncEvent$userId$eventCode(ByteRTCGameRoom rtcRoom,
      NSString roomId, NSString userId, ByteRTCAVSyncEvent eventCode) async {}

  /// @detail callback
  /// @region 房间管理
  /// @author xuyiling.x10
  /// @brief 房间内远端摄像头采集的媒体流的回调。
  /// @param rtcRoom ByteRTCGameRoom 实例。
  /// @param streamId 流 ID。
  /// @param info 流信息，参看 ByteRTCStreamInfo{@link #ByteRTCStreamInfo}。
  /// @param isPublish 为true代表流发布，为false代表流移除。
  /// @order 2

  FutureOr<void> rtcRoom$onUserPublishStreamVideo$info$isPublish(
      ByteRTCGameRoom rtcRoom,
      NSString streamId,
      ByteRTCStreamInfo info,
      BOOL isPublish) async {}

  /// @detail callback
  /// @region 房间管理
  /// @author xuyiling.x10
  /// @brief 房间内远端麦克风采集的媒体流的回调。
  /// @param rtcRoom ByteRTCGameRoom 实例。
  /// @param streamId 流 ID。
  /// @param info 流信息，参看 ByteRTCStreamInfo{@link #ByteRTCStreamInfo}。
  /// @param isPublish 为 true 代表流发布，为 false 代表流移除。
  /// @order 2

  FutureOr<void> rtcRoom$onUserPublishStreamAudio$info$isPublish(
      ByteRTCGameRoom rtcRoom,
      NSString streamId,
      ByteRTCStreamInfo info,
      BOOL isPublish) async {}

  /// @detail callback
  /// @region 房间管理
  /// @author xuyiling.x10
  /// @brief 房间内远端屏幕共享的视频流的回调。
  /// @param rtcRoom ByteRTCGameRoom 实例。
  /// @param roomId 房间 ID。
  /// @param uid 远端流发布用户的用户 ID。
  /// @param isPublish 为 true 代表已发布，为 false 代表已取消发布。
  /// @order 2

  FutureOr<void> rtcRoom$onUserPublishScreenVideo$uid$isPublish(
      ByteRTCGameRoom rtcRoom,
      NSString roomId,
      NSString uid,
      BOOL isPublish) async {}

  /// @detail callback
  /// @region 房间管理
  /// @author xuyiling.x10
  /// @brief 房间内远端屏幕共享的音频流的回调。
  /// @param rtcRoom ByteRTCGameRoom 实例。
  /// @param roomId 房间 ID。
  /// @param uid 远端流发布用户的用户 ID。
  /// @param isPublish 为 true 代表已发布，为 false 代表已取消发布。
  /// @order 2

  FutureOr<void> rtcRoom$onUserPublishScreenAudio$uid$isPublish(
      ByteRTCGameRoom rtcRoom,
      NSString roomId,
      NSString uid,
      BOOL isPublish) async {}

  /// @detail callback
  /// @author hanchenchen.c
  /// @brief 广播文本消息回调。
  /// @param rtcRoom ByteRTCGameRoom 实例。
  /// @param uid 消息发送者 ID。
  /// @param message 收到的文本消息内容。
  ///

  FutureOr<void> rtcRoom$onRoomMessageReceived$message(
      ByteRTCGameRoom rtcRoom, NSString uid, NSString message) async {}

  /// @detail callback
  /// @author hanchenchen.c
  /// @brief 收到房间内广播二进制消息的回调。
  /// @param rtcRoom ByteRTCGameRoom 实例。
  /// @param uid 消息发送者 ID 。
  /// @param message 收到的二进制消息内容。
  ///

  FutureOr<void> rtcRoom$onRoomBinaryMessageReceived$message(
      ByteRTCGameRoom rtcRoom, NSString uid, NSData message) async {}

  /// @detail callback
  /// @author hanchenchen.c
  /// @brief 收到来自房间中其他用户发来的点对点文本消息时，会收到此回调。
  /// @param rtcRoom ByteRTCGameRoom 实例。
  /// @param uid 消息发送者的用户 ID。
  /// @param message 收到的文本消息内容。
  ///

  FutureOr<void> rtcRoom$onUserMessageReceived$message(
      ByteRTCGameRoom rtcRoom, NSString uid, NSString message) async {}

  /// @detail callback
  /// @author hanchenchen.c
  /// @brief 收到来自房间中其他用户发来的点对点二进制消息时，会收到此回调。
  /// @param rtcRoom ByteRTCGameRoom 实例。
  /// @param uid 消息发送者的用户 ID。
  /// @param message 收到的二进制消息内容。
  ///

  FutureOr<void> rtcRoom$onUserBinaryMessageReceived$message(
      ByteRTCGameRoom rtcRoom, NSString uid, NSData message) async {}

  /// @detail callback
  /// @author hanchenchen.c
  /// @brief 向房间内单个用户发送文本或二进制消息后（P2P），消息发送方会收到该消息发送结果回调。
  /// @param rtcRoom ByteRTCGameRoom 实例。
  /// @param msgid 本条消息的 ID。
  /// @param error 消息发送结果，详见 ByteRTCUserMessageSendResult{@link #ByteRTCUserMessageSendResult}
  ///

  FutureOr<void> rtcRoom$onUserMessageSendResult$error(ByteRTCGameRoom rtcRoom,
      NSInteger msgid, ByteRTCUserMessageSendResult error) async {}

  /// @detail callback
  /// @author hanchenchen.c
  /// @brief 向房间内群发文本或二进制消息后，消息发送方会收到该消息发送结果回调。
  /// @param rtcRoom ByteRTCGameRoom 实例。
  /// @param msgid 本条消息的 ID。
  /// @param error 消息发送结果，详见 ByteRTCRoomMessageSendResult{@link #ByteRTCRoomMessageSendResult}
  ///

  FutureOr<void> rtcRoom$onRoomMessageSendResult$error(ByteRTCGameRoom rtcRoom,
      NSInteger msgid, ByteRTCRoomMessageSendResult error) async {}

  /// @valid since 3.52.
  /// @detail callback
  /// @author lichangfeng.rtc
  /// @brief 发送附加信息结果回调。
  /// @param rtcRoom ByteRTCGameRoom 实例。
  /// @param taskId 本次调用的任务编号。
  /// @param result 房间附加信息设置结果，详见 ByteRTCSetRoomExtraInfoResult{@link #ByteRTCSetRoomExtraInfoResult}

  FutureOr<void> rtcRoom$onSetRoomExtraInfoResult$result(
      ByteRTCGameRoom rtcRoom,
      NSInteger taskId,
      ByteRTCSetRoomExtraInfoResult result) async {}

  /// @valid since 3.52.
  /// @detail callback
  /// @author lichangfeng.rtc
  /// @brief 接收到房间附加信息的回调。<br>
  ///        另外用户加入房间成功后会收到这个房间全量附加信息
  /// @param rtcRoom ByteRTCGameRoom 实例。
  /// @param key 附加信息的键值
  /// @param value 附加信息的内容
  /// @param lastUpdateUserId 最后更新这条附加信息的用户编号
  /// @param lastUpdateTimeMs 最后更新这条附加信息的 Unix 时间，时间精度是毫秒

  FutureOr<void>
      rtcRoom$onRoomExtraInfoUpdate$value$lastUpdateUserId$lastUpdateTimeMs(
          ByteRTCGameRoom rtcRoom,
          NSString key,
          NSString value,
          NSString lastUpdateUserId,
          NSInteger lastUpdateTimeMs) async {}

  /// @valid since 3.54
  /// @detail callback
  /// @author caocun
  /// @brief 设置用户可见性的回调。
  /// @param rtcRoom `ByteRTCGameRoom` 实例
  /// @param currentUserVisibility 当前用户的可见性。 <br>
  ///        - YES: 可见，用户可以在房间内发布音视频流，房间中的其他用户将收到用户的行为通知，例如进房、开启视频采集和退房。
  ///        - NO: 不可见，用户不可以在房间内发布音视频流，房间中的其他用户不会收到用户的行为通知，例如进房、开启视频采集和退房。
  /// @param errorCode 设置用户可见性错误码，参看 ByteRTCUserVisibilityChangeError{@link #ByteRTCUserVisibilityChangeError}。

  FutureOr<void> rtcRoom$onUserVisibilityChanged$errorCode(
      ByteRTCGameRoom rtcRoom,
      BOOL currentUserVisibility,
      ByteRTCUserVisibilityChangeError errorCode) async {}

  /// @detail callback
  /// @author qipengxiang
  /// @brief 通过调用服务端 BanUserStream/UnbanUserStream 方法禁用/解禁指定房间内指定用户视频流的发送时，触发此回调。
  /// @param rtcRoom `ByteRTCGameRoom` 实例
  /// @param uid 被禁用/解禁的视频流用户 ID
  /// @param banned 视频流发送状态 <br>
  ///        - true: 视频流发送被禁用
  ///        - false: 视频流发送被解禁
  /// @note
  ///        - 房间内指定用户被禁止/解禁视频流发送时，房间内所有用户都会收到该回调。
  ///        - 若被封禁用户断网或退房后再进房，则依然是封禁状态，且房间内所有人会再次收到该回调。
  ///        - 指定用户被封禁后，房间内其他用户退房后再进房，会再次收到该回调。
  ///        - 同一房间解散后再次创建，房间内状态清空。

  FutureOr<void> rtcRoom$onVideoStreamBanned$isBanned(
      ByteRTCGameRoom rtcRoom, NSString uid, BOOL banned) async {}

  /// @detail callback
  /// @author qipengxiang
  /// @brief 通过调用服务端 BanUserStream/UnbanUserStream 方法禁用/解禁指定房间内指定用户音频流的发送时，触发此回调。
  /// @param rtcRoom `ByteRTCGameRoom` 实例
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

  FutureOr<void> rtcRoom$onAudioStreamBanned$isBanned(
      ByteRTCGameRoom rtcRoom, NSString uid, BOOL banned) async {}

  /// @detail callback
  /// @author luomingkang
  /// @brief 跨房间媒体流转发状态和错误回调
  /// @param rtcRoom ByteRTCGameRoom 实例。
  /// @param infos 跨房间媒体流转发目标房间信息数组，详见 ByteRTCForwardStreamStateInfo{@link #ByteRTCForwardStreamStateInfo}
  ///

  FutureOr<void> rtcRoom$onForwardStreamStateChanged(ByteRTCGameRoom rtcRoom,
      NSArray<ByteRTCForwardStreamStateInfo> infos) async {}

  /// @detail callback
  /// @author luomingkang
  /// @brief 跨房间媒体流转发事件回调
  /// @param rtcRoom ByteRTCGameRoom 实例。
  /// @param infos 跨房间媒体流转发目标房间事件数组，详见 ByteRTCForwardStreamEventInfo{@link #ByteRTCForwardStreamEventInfo}
  ///

  FutureOr<void> rtcRoom$onForwardStreamEvent(ByteRTCGameRoom rtcRoom,
      NSArray<ByteRTCForwardStreamEventInfo> infos) async {}

  /// @detail callback
  /// @author chengchao.cc951119
  /// @brief 加入房间并发布或订阅流后， 以每 2 秒一次的频率，报告本地用户和已订阅的远端用户的上下行网络质量信息。
  /// @param rtcRoom ByteRTCGameRoom 实例。
  /// @param localQuality 本端网络质量，详见 ByteRTCNetworkQualityStats{@link #ByteRTCNetworkQualityStats}。
  /// @param remoteQualities 已订阅用户的网络质量，详见 ByteRTCNetworkQualityStats{@link #ByteRTCNetworkQualityStats}。
  /// @note 更多通话中的监测接口，详见[通话中质量监测](https://www.volcengine.com/docs/6348/106866)。

  FutureOr<void> rtcRoom$onNetworkQuality$remoteQualities(
      ByteRTCGameRoom rtcRoom,
      ByteRTCNetworkQualityStats localQuality,
      NSArray<ByteRTCNetworkQualityStats> remoteQualities) async {}

  /// @detail callback
  /// @author qiaoxingwang
  /// @brief 字幕状态发生改变回调。 <br>
  ///         当用户调用 startSubtitle:{@link #ByteRTCRoom#startSubtitle} 和 stopSubtitle{@link #ByteRTCRoom#stopSubtitle} 使字幕状态发生改变或出现错误时，触发该回调。
  /// @param rtcRoom ByteRTCGameRoom 实例。
  /// @param state 字幕状态。参看 ByteRTCSubtitleState{@link #ByteRTCSubtitleState}。
  /// @param errorCode 字幕任务错误码。参看 ByteRTCSubtitleErrorCode{@link #ByteRTCSubtitleErrorCode}。
  /// @param errorMessage 第三方服务出现的错误。当因第三方服务出现错误，导致字幕状态改变时，用户可通过此参数获取具体的错误信息。如果不是因为第三方服务导致字幕状态改变，该字段为空。

  FutureOr<void> rtcRoom$onSubtitleStateChanged$errorCode$errorMessage(
      ByteRTCGameRoom rtcRoom,
      ByteRTCSubtitleState state,
      ByteRTCSubtitleErrorCode errorCode,
      NSString errorMessage) async {}

  /// @detail callback
  /// @author qiaoxingwang
  /// @brief 字幕相关内容回调。
  /// @param rtcRoom ByteRTCGameRoom 实例。
  /// @param subtitles 字幕消息内容。参看 ByteRTCSubtitleMessage{@link #ByteRTCSubtitleMessage}。

  FutureOr<void> rtcRoom$onSubtitleMessageReceived(ByteRTCGameRoom rtcRoom,
      NSArray<ByteRTCSubtitleMessage> subtitles) async {}

  /// @deprecated since 3.41 and will be deleted in later version, use rtcRoom:onRoomStateChangedWithReason:withUid:state:reason:{@link #ByteRTCGameRoomDelegate#rtcRoom:onRoomStateChangedWithReason:withUid:state:reason}  and rtcRoom:onStreamStateChanged:withUid:state:extraInfo:{@link #ByteRTCGameRoomDelegate#rtcRoom:onStreamStateChanged:withUid:state:extraInfo} instead.
  /// @detail callback
  /// @author luomingkang
  /// @brief 发生警告回调。 <br>
  ///        SDK 运行时出现了警告。SDK 通常会自动恢复，警告信息可以忽略。
  /// @param rtcRoom ByteRTCGameRoom 实例。
  /// @param warningCode 警告码，详见枚举类型 ByteRTCWarningCode{@link #ByteRTCWarningCode} 。
  ///

  FutureOr<void> rtcRoom$onRoomWarning(
      ByteRTCGameRoom rtcRoom, ByteRTCWarningCode warningCode) async {}

  /// @deprecated since 3.36 and will be deleted in later version, use rtcRoom:onUserPublishStreamAudio:info:isPublish:{@link #ByteRTCGameRoomDelegate#rtcRoom:onUserPublishStreamAudio:info:isPublish}, rtcRoom:onUserPublishStreamVideo:info:isPublish:{@link #ByteRTCGameRoomDelegate#rtcRoom:onUserPublishStreamVideo:info:isPublish}, rtcRoom:onUserPublishScreenVideo:uid:isPublish:{@link #ByteRTCGameRoomDelegate#rtcRoom:onUserPublishScreenVideo:uid:isPublish}, and rtcRoom:onUserPublishScreenAudio:uid:isPublish:{@link #ByteRTCGameRoomDelegate#rtcRoom:onUserPublishScreenAudio:uid:isPublish} instead.
  /// @detail callback
  /// @author luomingkang
  /// @brief 以下情况会触发此回调： <br>
  ///        - 房间内的用户发布新的音视频流时，房间内的其他用户会收到此回调通知。
  ///        - 房间内的用户原音视频流被移出后，又重新发布音视频流时，房间内的其他用户会收到此回调通知。
  ///        - 用户刚加入房间时，会收到此回调，包含房间中所有已发布的流。
  /// @param rtcRoom ByteRTCGameRoom 实例。
  /// @param stream 流属性，参看 ByteRTCStream{@link #ByteRTCStream} 。
  ///

  FutureOr<void> rtcRoom$onStreamAdd(
      ByteRTCGameRoom rtcRoom, id<ByteRTCStream> stream) async {}

  /// @hidden
  /// @deprecated since 3.36 and will be deleted in later version, use rtcRoom:onUserPublishStreamAudio:info:isPublish:{@link #ByteRTCGameRoomDelegate#rtcRoom:onUserPublishStreamAudio:info:isPublish}, rtcRoom:onUserPublishStreamVideo:info:isPublish:{@link #ByteRTCGameRoomDelegate#rtcRoom:onUserPublishStreamVideo:info:isPublish}, rtcRoom:onUserPublishScreenVideo:uid:isPublish:{@link #ByteRTCGameRoomDelegate#rtcRoom:onUserPublishScreenVideo:uid:isPublish}, and rtcRoom:onUserPublishScreenAudio:uid:isPublish:{@link #ByteRTCGameRoomDelegate#rtcRoom:onUserPublishScreenAudio:uid:isPublish} instead.
  /// @detail callback
  /// @author luomingkang
  /// @brief 房间内的远端用户停止发布音视频流时，本地用户会收到此回调通知。
  /// @param rtcRoom ByteRTCGameRoom 实例。
  /// @param uid 远端流来源的用户 ID 。
  /// @param stream 流的属性，参看 ByteRTCStream{@link #ByteRTCStream}。
  /// @param reason 远端流移除的原因，参看 ByteRTCStreamRemoveReason{@link #ByteRTCStreamRemoveReason} 。
  ///

  FutureOr<void> rtcRoom$onStreamRemove$stream$reason(
      ByteRTCGameRoom rtcRoom,
      NSString uid,
      id<ByteRTCStream> stream,
      ByteRTCStreamRemoveReason reason) async {}
}

class ByteRTCEngineDelegate extends NativeObserverClass {
  static const _$namespace = r'ByteRTCEngineDelegate';

  ByteRTCEngineDelegate([NativeClassOptions? options])
      : super(options == null
            ? const NativeClassOptions([],
                className: _$namespace,
                instanceType: InstanceType.manual,
                bridgeKey: 'com.volcengine.rtc.hybrid_runtime',
                methodMap: {
                  r"rtcEngine$onWarning": r"rtcEngine:onWarning:",
                  r"rtcEngine$onError": r"rtcEngine:onError:",
                  r"rtcEngine$onDeadLockError": r"rtcEngine:onDeadLockError:",
                  r"rtcEngine$onExtensionAccessError$msg":
                      r"rtcEngine:onExtensionAccessError:msg:",
                  r"rtcEngine$onConnectionStateChanged":
                      r"rtcEngine:onConnectionStateChanged:",
                  r"rtcEngine$onNetworkTypeChanged":
                      r"rtcEngine:onNetworkTypeChanged:",
                  r"rtcEngine$onUserStartAudioCapture$info":
                      r"rtcEngine:onUserStartAudioCapture:info:",
                  r"rtcEngine$onUserStopAudioCapture$info":
                      r"rtcEngine:onUserStopAudioCapture:info:",
                  r"rtcEngine$onFirstRemoteAudioFrame$info":
                      r"rtcEngine:onFirstRemoteAudioFrame:info:",
                  r"rtcEngine$onLocalAudioPropertiesReport":
                      r"rtcEngine:onLocalAudioPropertiesReport:",
                  r"rtcEngine$onAudioVADStateUpdate":
                      r"rtcEngine:onAudioVADStateUpdate:",
                  r"rtcEngine$onAudioAEDStateUpdate":
                      r"rtcEngine:onAudioAEDStateUpdate:",
                  r"rtcEngine$onAudioPlaybackDeviceTestVolume":
                      r"rtcEngine:onAudioPlaybackDeviceTestVolume:",
                  r"rtcEngine$onAudioDeviceVolumeChanged$volume$muted":
                      r"rtcEngine:onAudioDeviceVolumeChanged:volume:muted:",
                  r"rtcEngine$onRemoteAudioPropertiesReport$totalRemoteVolume":
                      r"rtcEngine:onRemoteAudioPropertiesReport:totalRemoteVolume:",
                  r"rtcEngine$onActiveSpeaker$uid":
                      r"rtcEngine:onActiveSpeaker:uid:",
                  r"rtcEngine$onRemoteAudioPropertiesReportEx":
                      r"rtcEngine:onRemoteAudioPropertiesReportEx:",
                  r"rtcEngine$onUserStartVideoCapture$info":
                      r"rtcEngine:onUserStartVideoCapture:info:",
                  r"rtcEngine$onUserStopVideoCapture$info":
                      r"rtcEngine:onUserStopVideoCapture:info:",
                  r"rtcEngine$onFirstLocalVideoFrameCaptured$withFrameInfo":
                      r"rtcEngine:onFirstLocalVideoFrameCaptured:withFrameInfo:",
                  r"rtcEngine$onFirstRemoteVideoFrameRendered$info$withFrameInfo":
                      r"rtcEngine:onFirstRemoteVideoFrameRendered:info:withFrameInfo:",
                  r"rtcEngine$onFirstRemoteVideoFrameDecoded$info$withFrameInfo":
                      r"rtcEngine:onFirstRemoteVideoFrameDecoded:info:withFrameInfo:",
                  r"rtcEngine$onRemoteVideoSizeChanged$info$withFrameInfo":
                      r"rtcEngine:onRemoteVideoSizeChanged:info:withFrameInfo:",
                  r"rtcEngine$onLocalVideoSizeChanged$withFrameInfo":
                      r"rtcEngine:onLocalVideoSizeChanged:withFrameInfo:",
                  r"rtcEngine$onAudioDeviceStateChanged$device_type$device_state$device_error":
                      r"rtcEngine:onAudioDeviceStateChanged:device_type:device_state:device_error:",
                  r"rtcEngine$onVideoDeviceStateChanged$device_type$device_state$device_error":
                      r"rtcEngine:onVideoDeviceStateChanged:device_type:device_state:device_error:",
                  r"rtcEngine$onAudioDeviceWarning$deviceType$deviceWarning":
                      r"rtcEngine:onAudioDeviceWarning:deviceType:deviceWarning:",
                  r"rtcEngine$onVideoDeviceWarning$deviceType$deviceWarning":
                      r"rtcEngine:onVideoDeviceWarning:deviceType:deviceWarning:",
                  r"rtcEngine$onAudioFrameSendStateChanged$info$rtcUser$state":
                      r"rtcEngine:onAudioFrameSendStateChanged:info:rtcUser:state:",
                  r"rtcEngine$onVideoFrameSendStateChanged$info$rtcUser$state":
                      r"rtcEngine:onVideoFrameSendStateChanged:info:rtcUser:state:",
                  r"rtcEngine$onAudioFramePlayStateChanged$info$rtcUser$state":
                      r"rtcEngine:onAudioFramePlayStateChanged:info:rtcUser:state:",
                  r"rtcEngine$onVideoFramePlayStateChanged$info$rtcUser$state":
                      r"rtcEngine:onVideoFramePlayStateChanged:info:rtcUser:state:",
                  r"rtcEngine$onFirstLocalAudioFrame":
                      r"rtcEngine:onFirstLocalAudioFrame:",
                  r"rtcEngine$onAudioRouteChanged":
                      r"rtcEngine:onAudioRouteChanged:",
                  r"rtcEngine$onSEIMessageReceived$info$andMessage":
                      r"rtcEngine:onSEIMessageReceived:info:andMessage:",
                  r"rtcEngine$onSEIStreamUpdate$info$eventType":
                      r"rtcEngine:onSEIStreamUpdate:info:eventType:",
                  r"rtcEngine$onStreamSyncInfoReceived$info$streamType$data":
                      r"rtcEngine:onStreamSyncInfoReceived:info:streamType:data:",
                  r"rtcEngine$onSysStats": r"rtcEngine:onSysStats:",
                  r"rtcEngine$onLocalAudioStateChanged$state$error":
                      r"rtcEngine:onLocalAudioStateChanged:state:error:",
                  r"rtcEngine$onRemoteAudioStateChanged$info$state$reason":
                      r"rtcEngine:onRemoteAudioStateChanged:info:state:reason:",
                  r"rtcEngine$onLocalVideoStateChanged$withStreamState$withStreamError":
                      r"rtcEngine:onLocalVideoStateChanged:withStreamState:withStreamError:",
                  r"rtcEngine$onRemoteVideoStateChanged$info$withVideoState$withVideoStateReason":
                      r"rtcEngine:onRemoteVideoStateChanged:info:withVideoState:withVideoStateReason:",
                  r"rtcEngine$onRemoteVideoSuperResolutionModeChanged$info$withMode$withReason":
                      r"rtcEngine:onRemoteVideoSuperResolutionModeChanged:info:withMode:withReason:",
                  r"rtcEngine$onVideoDenoiseModeChanged$withReason":
                      r"rtcEngine:onVideoDenoiseModeChanged:withReason:",
                  r"rtcEngine$onLoginResult$errorCode$elapsed":
                      r"rtcEngine:onLoginResult:errorCode:elapsed:",
                  r"rtcEngine$onLogout": r"rtcEngine:onLogout:",
                  r"rtcEngine$onServerParamsSetResult":
                      r"rtcEngine:onServerParamsSetResult:",
                  r"rtcEngine$onGetPeerOnlineStatus$status":
                      r"rtcEngine:onGetPeerOnlineStatus:status:",
                  r"rtcEngine$onUserMessageReceivedOutsideRoom$message":
                      r"rtcEngine:onUserMessageReceivedOutsideRoom:message:",
                  r"rtcEngine$onUserBinaryMessageReceivedOutsideRoom$message":
                      r"rtcEngine:onUserBinaryMessageReceivedOutsideRoom:message:",
                  r"rtcEngine$onUserMessageReceivedOutsideRoom$uid$message":
                      r"rtcEngine:onUserMessageReceivedOutsideRoom:uid:message:",
                  r"rtcEngine$onUserBinaryMessageReceivedOutsideRoom$uid$message":
                      r"rtcEngine:onUserBinaryMessageReceivedOutsideRoom:uid:message:",
                  r"rtcEngine$onUserMessageSendResultOutsideRoom$error":
                      r"rtcEngine:onUserMessageSendResultOutsideRoom:error:",
                  r"rtcEngine$onServerMessageSendResult$error$message":
                      r"rtcEngine:onServerMessageSendResult:error:message:",
                  r"rtcEngine$onNetworkDetectionResult$quality$rtt$lostRate$bitrate$jitter":
                      r"rtcEngine:onNetworkDetectionResult:quality:rtt:lostRate:bitrate:jitter:",
                  r"rtcEngine$onNetworkDetectionStopped":
                      r"rtcEngine:onNetworkDetectionStopped:",
                  r"rtcEngine$onAudioMixingPlayingProgress$progress":
                      r"rtcEngine:onAudioMixingPlayingProgress:progress:",
                  r"rtcEngine$onPerformanceAlarms$info$mode$reason$sourceWantedData":
                      r"rtcEngine:onPerformanceAlarms:info:mode:reason:sourceWantedData:",
                  r"rtcEngine$onSimulcastSubscribeFallback$info$event":
                      r"rtcEngine:onSimulcastSubscribeFallback:info:event:",
                  r"rtcEngine$onExternalScreenFrameUpdate":
                      r"rtcEngine:onExternalScreenFrameUpdate:",
                  r"rtcEngine$onRecordingStateUpdate$state$error_code$recording_info":
                      r"rtcEngine:onRecordingStateUpdate:state:error_code:recording_info:",
                  r"rtcEngine$onRecordingProgressUpdate$process$recording_info":
                      r"rtcEngine:onRecordingProgressUpdate:process:recording_info:",
                  r"rtcEngine$onAudioRecordingStateUpdate$error_code":
                      r"rtcEngine:onAudioRecordingStateUpdate:error_code:",
                  r"rtcEngine$onCloudProxyConnected":
                      r"rtcEngine:onCloudProxyConnected:",
                  r"rtcEngine$onEchoTestResult": r"rtcEngine:onEchoTestResult:",
                  r"rtcEngine$onAudioDumpStateChanged":
                      r"rtcEngine:onAudioDumpStateChanged:",
                  r"rtcEngineOnNetworkTimeSynchronized":
                      r"rtcEngineOnNetworkTimeSynchronized:",
                  r"rtcEngine$onLicenseWillExpire":
                      r"rtcEngine:onLicenseWillExpire:",
                  r"rtcEngine$onHardwareEchoDetectionResult":
                      r"rtcEngine:onHardwareEchoDetectionResult:",
                  r"rtcEngine$onLocalProxyStateChanged$withProxyState$withProxyError":
                      r"rtcEngine:onLocalProxyStateChanged:withProxyState:withProxyError:",
                  r"rtcEngine$onEffectError$msg":
                      r"rtcEngine:onEffectError:msg:",
                  r"rtcEngine$onMixedStreamEvent$withMixedStreamInfo$withErrorCode":
                      r"rtcEngine:onMixedStreamEvent:withMixedStreamInfo:withErrorCode:",
                  r"rtcEngine$onSingleStreamEvent$withTaskId$withErrorCode":
                      r"rtcEngine:onSingleStreamEvent:withTaskId:withErrorCode:",
                  r"rtcEngine$onRemoteSnapshotTakenToFile$info$filePath$width$height$errorCode$taskId":
                      r"rtcEngine:onRemoteSnapshotTakenToFile:info:filePath:width:height:errorCode:taskId:",
                  r"rtcEngine$onLocalSnapshotTakenToFile$filePath$width$height$errorCode$taskId":
                      r"rtcEngine:onLocalSnapshotTakenToFile:filePath:width:height:errorCode:taskId:",
                  r"rtcEngine$onClientMixedFirstVideoFrame":
                      r"rtcEngine:onClientMixedFirstVideoFrame:",
                  r"rtcEngine$onExperimentalCallback":
                      r"rtcEngine:onExperimentalCallback:",
                  r"rtcEngine$log": r"rtcEngine:log:",
                  r"rtcEngine$onPushPublicStreamResult$publicStreamId$errorCode":
                      r"rtcEngine:onPushPublicStreamResult:publicStreamId:errorCode:",
                  r"rtcEngine$onNetworkTimeSynchronized":
                      r"rtcEngine:onNetworkTimeSynchronized:"
                })
            : NativeClassOptions.fromMap({
                ...options.toMap(),
                'bridgeKey': 'com.volcengine.rtc.hybrid_runtime',
              })) {
    registerEvent(r"rtcEngine:onWarning:", rtcEngine$onWarning);

    registerEvent(r"rtcEngine:onError:", rtcEngine$onError);

    registerEvent(r"rtcEngine:onDeadLockError:", rtcEngine$onDeadLockError);

    registerEvent(r"rtcEngine:onExtensionAccessError:msg:",
        rtcEngine$onExtensionAccessError$msg);

    registerEvent(r"rtcEngine:onConnectionStateChanged:",
        rtcEngine$onConnectionStateChanged);

    registerEvent(
        r"rtcEngine:onNetworkTypeChanged:", rtcEngine$onNetworkTypeChanged);

    registerEvent(r"rtcEngine:onUserStartAudioCapture:info:",
        rtcEngine$onUserStartAudioCapture$info);

    registerEvent(r"rtcEngine:onUserStopAudioCapture:info:",
        rtcEngine$onUserStopAudioCapture$info);

    registerEvent(r"rtcEngine:onFirstRemoteAudioFrame:info:",
        rtcEngine$onFirstRemoteAudioFrame$info);

    registerEvent(r"rtcEngine:onLocalAudioPropertiesReport:",
        rtcEngine$onLocalAudioPropertiesReport);

    registerEvent(
        r"rtcEngine:onAudioVADStateUpdate:", rtcEngine$onAudioVADStateUpdate);

    registerEvent(
        r"rtcEngine:onAudioAEDStateUpdate:", rtcEngine$onAudioAEDStateUpdate);

    registerEvent(r"rtcEngine:onAudioPlaybackDeviceTestVolume:",
        rtcEngine$onAudioPlaybackDeviceTestVolume);

    registerEvent(r"rtcEngine:onAudioDeviceVolumeChanged:volume:muted:",
        rtcEngine$onAudioDeviceVolumeChanged$volume$muted);

    registerEvent(r"rtcEngine:onRemoteAudioPropertiesReport:totalRemoteVolume:",
        rtcEngine$onRemoteAudioPropertiesReport$totalRemoteVolume);

    registerEvent(
        r"rtcEngine:onActiveSpeaker:uid:", rtcEngine$onActiveSpeaker$uid);

    registerEvent(r"rtcEngine:onRemoteAudioPropertiesReportEx:",
        rtcEngine$onRemoteAudioPropertiesReportEx);

    registerEvent(r"rtcEngine:onUserStartVideoCapture:info:",
        rtcEngine$onUserStartVideoCapture$info);

    registerEvent(r"rtcEngine:onUserStopVideoCapture:info:",
        rtcEngine$onUserStopVideoCapture$info);

    registerEvent(r"rtcEngine:onFirstLocalVideoFrameCaptured:withFrameInfo:",
        rtcEngine$onFirstLocalVideoFrameCaptured$withFrameInfo);

    registerEvent(
        r"rtcEngine:onFirstRemoteVideoFrameRendered:info:withFrameInfo:",
        rtcEngine$onFirstRemoteVideoFrameRendered$info$withFrameInfo);

    registerEvent(
        r"rtcEngine:onFirstRemoteVideoFrameDecoded:info:withFrameInfo:",
        rtcEngine$onFirstRemoteVideoFrameDecoded$info$withFrameInfo);

    registerEvent(r"rtcEngine:onRemoteVideoSizeChanged:info:withFrameInfo:",
        rtcEngine$onRemoteVideoSizeChanged$info$withFrameInfo);

    registerEvent(r"rtcEngine:onLocalVideoSizeChanged:withFrameInfo:",
        rtcEngine$onLocalVideoSizeChanged$withFrameInfo);

    registerEvent(
        r"rtcEngine:onAudioDeviceStateChanged:device_type:device_state:device_error:",
        rtcEngine$onAudioDeviceStateChanged$device_type$device_state$device_error);

    registerEvent(
        r"rtcEngine:onVideoDeviceStateChanged:device_type:device_state:device_error:",
        rtcEngine$onVideoDeviceStateChanged$device_type$device_state$device_error);

    registerEvent(r"rtcEngine:onAudioDeviceWarning:deviceType:deviceWarning:",
        rtcEngine$onAudioDeviceWarning$deviceType$deviceWarning);

    registerEvent(r"rtcEngine:onVideoDeviceWarning:deviceType:deviceWarning:",
        rtcEngine$onVideoDeviceWarning$deviceType$deviceWarning);

    registerEvent(r"rtcEngine:onAudioFrameSendStateChanged:info:rtcUser:state:",
        rtcEngine$onAudioFrameSendStateChanged$info$rtcUser$state);

    registerEvent(r"rtcEngine:onVideoFrameSendStateChanged:info:rtcUser:state:",
        rtcEngine$onVideoFrameSendStateChanged$info$rtcUser$state);

    registerEvent(r"rtcEngine:onAudioFramePlayStateChanged:info:rtcUser:state:",
        rtcEngine$onAudioFramePlayStateChanged$info$rtcUser$state);

    registerEvent(r"rtcEngine:onVideoFramePlayStateChanged:info:rtcUser:state:",
        rtcEngine$onVideoFramePlayStateChanged$info$rtcUser$state);

    registerEvent(
        r"rtcEngine:onFirstLocalAudioFrame:", rtcEngine$onFirstLocalAudioFrame);

    registerEvent(
        r"rtcEngine:onAudioRouteChanged:", rtcEngine$onAudioRouteChanged);

    registerEvent(r"rtcEngine:onSEIMessageReceived:info:andMessage:",
        rtcEngine$onSEIMessageReceived$info$andMessage);

    registerEvent(r"rtcEngine:onSEIStreamUpdate:info:eventType:",
        rtcEngine$onSEIStreamUpdate$info$eventType);

    registerEvent(r"rtcEngine:onStreamSyncInfoReceived:info:streamType:data:",
        rtcEngine$onStreamSyncInfoReceived$info$streamType$data);

    registerEvent(r"rtcEngine:onSysStats:", rtcEngine$onSysStats);

    registerEvent(r"rtcEngine:onLocalAudioStateChanged:state:error:",
        rtcEngine$onLocalAudioStateChanged$state$error);

    registerEvent(r"rtcEngine:onRemoteAudioStateChanged:info:state:reason:",
        rtcEngine$onRemoteAudioStateChanged$info$state$reason);

    registerEvent(
        r"rtcEngine:onLocalVideoStateChanged:withStreamState:withStreamError:",
        rtcEngine$onLocalVideoStateChanged$withStreamState$withStreamError);

    registerEvent(
        r"rtcEngine:onRemoteVideoStateChanged:info:withVideoState:withVideoStateReason:",
        rtcEngine$onRemoteVideoStateChanged$info$withVideoState$withVideoStateReason);

    registerEvent(
        r"rtcEngine:onRemoteVideoSuperResolutionModeChanged:info:withMode:withReason:",
        rtcEngine$onRemoteVideoSuperResolutionModeChanged$info$withMode$withReason);

    registerEvent(r"rtcEngine:onVideoDenoiseModeChanged:withReason:",
        rtcEngine$onVideoDenoiseModeChanged$withReason);

    registerEvent(r"rtcEngine:onLoginResult:errorCode:elapsed:",
        rtcEngine$onLoginResult$errorCode$elapsed);

    registerEvent(r"rtcEngine:onLogout:", rtcEngine$onLogout);

    registerEvent(r"rtcEngine:onServerParamsSetResult:",
        rtcEngine$onServerParamsSetResult);

    registerEvent(r"rtcEngine:onGetPeerOnlineStatus:status:",
        rtcEngine$onGetPeerOnlineStatus$status);

    registerEvent(r"rtcEngine:onUserMessageReceivedOutsideRoom:message:",
        rtcEngine$onUserMessageReceivedOutsideRoom$message);

    registerEvent(r"rtcEngine:onUserBinaryMessageReceivedOutsideRoom:message:",
        rtcEngine$onUserBinaryMessageReceivedOutsideRoom$message);

    registerEvent(r"rtcEngine:onUserMessageReceivedOutsideRoom:uid:message:",
        rtcEngine$onUserMessageReceivedOutsideRoom$uid$message);

    registerEvent(
        r"rtcEngine:onUserBinaryMessageReceivedOutsideRoom:uid:message:",
        rtcEngine$onUserBinaryMessageReceivedOutsideRoom$uid$message);

    registerEvent(r"rtcEngine:onUserMessageSendResultOutsideRoom:error:",
        rtcEngine$onUserMessageSendResultOutsideRoom$error);

    registerEvent(r"rtcEngine:onServerMessageSendResult:error:message:",
        rtcEngine$onServerMessageSendResult$error$message);

    registerEvent(
        r"rtcEngine:onNetworkDetectionResult:quality:rtt:lostRate:bitrate:jitter:",
        rtcEngine$onNetworkDetectionResult$quality$rtt$lostRate$bitrate$jitter);

    registerEvent(r"rtcEngine:onNetworkDetectionStopped:",
        rtcEngine$onNetworkDetectionStopped);

    registerEvent(r"rtcEngine:onAudioMixingPlayingProgress:progress:",
        rtcEngine$onAudioMixingPlayingProgress$progress);

    registerEvent(
        r"rtcEngine:onPerformanceAlarms:info:mode:reason:sourceWantedData:",
        rtcEngine$onPerformanceAlarms$info$mode$reason$sourceWantedData);

    registerEvent(r"rtcEngine:onSimulcastSubscribeFallback:info:event:",
        rtcEngine$onSimulcastSubscribeFallback$info$event);

    registerEvent(r"rtcEngine:onExternalScreenFrameUpdate:",
        rtcEngine$onExternalScreenFrameUpdate);

    registerEvent(
        r"rtcEngine:onRecordingStateUpdate:state:error_code:recording_info:",
        rtcEngine$onRecordingStateUpdate$state$error_code$recording_info);

    registerEvent(
        r"rtcEngine:onRecordingProgressUpdate:process:recording_info:",
        rtcEngine$onRecordingProgressUpdate$process$recording_info);

    registerEvent(r"rtcEngine:onAudioRecordingStateUpdate:error_code:",
        rtcEngine$onAudioRecordingStateUpdate$error_code);

    registerEvent(
        r"rtcEngine:onCloudProxyConnected:", rtcEngine$onCloudProxyConnected);

    registerEvent(r"rtcEngine:onEchoTestResult:", rtcEngine$onEchoTestResult);

    registerEvent(r"rtcEngine:onAudioDumpStateChanged:",
        rtcEngine$onAudioDumpStateChanged);

    registerEvent(r"rtcEngineOnNetworkTimeSynchronized:",
        rtcEngineOnNetworkTimeSynchronized);

    registerEvent(
        r"rtcEngine:onLicenseWillExpire:", rtcEngine$onLicenseWillExpire);

    registerEvent(r"rtcEngine:onHardwareEchoDetectionResult:",
        rtcEngine$onHardwareEchoDetectionResult);

    registerEvent(
        r"rtcEngine:onLocalProxyStateChanged:withProxyState:withProxyError:",
        rtcEngine$onLocalProxyStateChanged$withProxyState$withProxyError);

    registerEvent(r"rtcEngine:onEffectError:msg:", rtcEngine$onEffectError$msg);

    registerEvent(
        r"rtcEngine:onMixedStreamEvent:withMixedStreamInfo:withErrorCode:",
        rtcEngine$onMixedStreamEvent$withMixedStreamInfo$withErrorCode);

    registerEvent(r"rtcEngine:onSingleStreamEvent:withTaskId:withErrorCode:",
        rtcEngine$onSingleStreamEvent$withTaskId$withErrorCode);

    registerEvent(
        r"rtcEngine:onRemoteSnapshotTakenToFile:info:filePath:width:height:errorCode:taskId:",
        rtcEngine$onRemoteSnapshotTakenToFile$info$filePath$width$height$errorCode$taskId);

    registerEvent(
        r"rtcEngine:onLocalSnapshotTakenToFile:filePath:width:height:errorCode:taskId:",
        rtcEngine$onLocalSnapshotTakenToFile$filePath$width$height$errorCode$taskId);

    registerEvent(r"rtcEngine:onClientMixedFirstVideoFrame:",
        rtcEngine$onClientMixedFirstVideoFrame);

    registerEvent(
        r"rtcEngine:onExperimentalCallback:", rtcEngine$onExperimentalCallback);

    registerEvent(r"rtcEngine:log:", rtcEngine$log);

    registerEvent(
        r"rtcEngine:onPushPublicStreamResult:publicStreamId:errorCode:",
        rtcEngine$onPushPublicStreamResult$publicStreamId$errorCode);

    registerEvent(r"rtcEngine:onNetworkTimeSynchronized:",
        rtcEngine$onNetworkTimeSynchronized);
  }

  /// @detail callback
  /// @author panjian.fishing
  /// @brief 发生警告回调。 <br>
  ///        SDK 运行时出现了警告。SDK 通常会自动恢复，警告信息可以忽略。
  /// @param engine ByteRTCEngine 对象。
  /// @param code 警告代码，参看 ByteRTCWarningCode{@link #ByteRTCWarningCode}。

  FutureOr<void> rtcEngine$onWarning(
      ByteRTCEngine engine, ByteRTCWarningCode code) async {}

  /// @detail callback
  /// @author panjian.fishing
  /// @brief 发生错误回调。 <br>
  ///        SDK 运行时出现了网络或媒体相关的错误，且无法自动恢复时触发此回调。 <br>
  ///        你可能需要干预.
  /// @param engine ByteRTCEngine 对象。
  /// @param errorCode 错误代码，参看 ByteRTCErrorCode{@link #ByteRTCErrorCode}。

  FutureOr<void> rtcEngine$onError(
      ByteRTCEngine engine, ByteRTCErrorCode errorCode) async {}

  /// @hidden internal use only
  /// @valid since 3.57
  /// @detail callback
  /// @brief 当内部线程发生 block 时，将收到此回调。
  /// @param engine ByteRTCEngine 对象
  /// @param deadlockMsg block 线程的线程名和 block 检测次数。参看 ByteRTCDeadLockMsg{#ByteRTCDeadLockMsg}

  FutureOr<void> rtcEngine$onDeadLockError(
      ByteRTCEngine engine, ByteRTCDeadLockMsg deadlockMsg) async {}

  /// @valid since 3.52
  /// @detail callback
  /// @author zhanyuqiao
  /// @brief 当访问插件失败时，收到此回调。 <br>
  ///        RTC SDK 将一些功能封装成插件。当使用这些功能时，如果插件不存在，功能将无法使用。
  /// @param engine `ByteRTCEngine` 实例
  /// @param extensionName 插件名字
  /// @param msg 失败说明

  FutureOr<void> rtcEngine$onExtensionAccessError$msg(
      ByteRTCEngine engine, NSString extensionName, NSString msg) async {}

  /// @detail callback
  /// @author hanchenchen.c
  /// @brief SDK 与信令服务器连接状态改变回调。连接状态改变时触发。
  /// @param engine ByteRTCEngine 对象
  /// @param state 当前 SDK 与信令服务器的连接状态，详见 ByteRTCConnectionState{@link #ByteRTCConnectionState}。
  /// @note 更多信息参见 [连接状态提示](https://www.volcengine.com/docs/6348/95376)。

  FutureOr<void> rtcEngine$onConnectionStateChanged(
      ByteRTCEngine engine, ByteRTCConnectionState state) async {}

  /// @detail callback
  /// @author hanchenchen.c
  /// @brief SDK 当前网络连接类型改变回调。
  /// @param engine ByteRTCEngine 对象
  /// @param type SDK 当前的网络连接类型，详见 ByteRTCNetworkType{@link #ByteRTCNetworkType}

  FutureOr<void> rtcEngine$onNetworkTypeChanged(
      ByteRTCEngine engine, ByteRTCNetworkType type) async {}

  /// @detail callback
  /// @author dixing
  /// @brief 房间内的可见用户调用 startAudioCapture{@link #ByteRTCEngine#startAudioCapture} 开启音频采集时，房间内其他用户会收到此回调。
  /// @param engine ByteRTCEngine 实例
  /// @param streamId 开启音频采集的远端流 ID
  /// @param info 开启音频采集的远端流信息，参看 ByteRTCStreamInfo{@link #ByteRTCStreamInfo}。

  FutureOr<void> rtcEngine$onUserStartAudioCapture$info(
      ByteRTCEngine engine, NSString streamId, ByteRTCStreamInfo info) async {}

  /// @detail callback
  /// @author dixing
  /// @brief 房间内的可见用户调用 stopAudioCapture{@link #ByteRTCEngine#stopAudioCapture} 关闭音频采集时，房间内其他用户会收到此回调。
  /// @param engine ByteRTCEngine 实例
  /// @param streamId 流 ID。
  /// @param info 关闭音频采集的远端流信息，参看 ByteRTCStreamInfo{@link #ByteRTCStreamInfo}。

  FutureOr<void> rtcEngine$onUserStopAudioCapture$info(
      ByteRTCEngine engine, NSString streamId, ByteRTCStreamInfo info) async {}

  /// @detail callback
  /// @author wangjunzheng
  /// @brief 订阅端接收并解码远端音频流首帧时，收到此回调。包含以下情况： <br>
  ///        1. 发布端发布音频，包含首次发布和取消后再次发布。<br>
  ///        2. 发布端关闭音频采集后，再次打开采集。使用外部源时，停止推流后再次推流。<br>
  ///        3. 发布端发布音频后，订阅端取消订阅音频后，又再次订阅音频。
  /// @param engine ByteRTCEngine 对象
  /// @param streamId 远端流 ID。
  /// @param info 远端流信息，参看 ByteRTCStreamInfo{@link #ByteRTCStreamInfo}。
  /// @note
  ///        - 用户刚收到房间内每一路音频流时，都会收到该回调。
  ///        - 摄像头流、屏幕流，内部采集、外部源、自动订阅和手动订阅的音频流，都符合上述策略。

  FutureOr<void> rtcEngine$onFirstRemoteAudioFrame$info(
      ByteRTCEngine engine, NSString streamId, ByteRTCStreamInfo info) async {}

  /// @detail callback
  /// @author huangshouqin
  /// @brief 调用 enableAudioPropertiesReport:{@link #ByteRTCEngine#enableAudioPropertiesReport} 后，根据设置的 interval 值，你会周期性地收到此回调，了解本地音频的瞬时相关信息。 <br>
  ///        本地音频包括使用 RTC SDK 内部机制采集的麦克风音频，屏幕音频和本地混音音频信息。
  /// @param engine ByteRTCEngine 对象
  /// @param audioPropertiesInfos 本地音频信息，详见 ByteRTCLocalAudioPropertiesInfo{@link #ByteRTCLocalAudioPropertiesInfo} 。在 macOS 端，本地音量可通过 `setAudioCaptureDeviceVolume:` 设置。
  ///

  FutureOr<void> rtcEngine$onLocalAudioPropertiesReport(ByteRTCEngine engine,
      NSArray<ByteRTCLocalAudioPropertiesInfo> audioPropertiesInfos) async {}

  /// @detail callback
  /// @hidden 3.60 for internal use only
  /// @region 音频管理
  /// @author gengjunjie
  /// @brief 人声检测结果回调。 <br>
  ///        调用 enableAudioVADReport 后，根据设置的interval值，本地会周期性地收到此回调 <br>
  /// @param state 人声检测结果，参看 ByteRTCAudioVADType{@link #ByteRTCAudioVADType}。

  FutureOr<void> rtcEngine$onAudioVADStateUpdate(
      ByteRTCEngine engine, ByteRTCAudioVADType state) async {}

  /// @hidden for internal use only
  /// @detail callback
  /// @region 音频管理
  /// @author shiyayun
  /// @brief 音乐场景检测结果回调。 <br>
  ///        调用 enableAudioAEDReport 后，根据设置的interval值，本地会周期性地收到此回调 <br>
  /// @param state 音乐场景检测结果，参看 ByteRTCAudioAEDType{@link #ByteRTCAudioAEDType}。

  FutureOr<void> rtcEngine$onAudioAEDStateUpdate(
      ByteRTCEngine engine, ByteRTCAudioAEDType state) async {}

  /// @hidden(iOS)
  /// @detail callback
  /// @brief 回调音频设备测试时的播放音量
  /// @param engine 参看 ByteRTCEngine{@link #ByteRTCEngine}。
  /// @param volume 音频设备测试播放音量。取值范围：[0,255]
  /// @note 调用 startAudioPlaybackDeviceTest:interval:{@link #ByteRTCAudioDeviceManager#startAudioPlaybackDeviceTest:interval} 或 startAudioDeviceRecordTest:{@link #ByteRTCAudioDeviceManager#startAudioDeviceRecordTest}，开始播放音频文件或录音时，将开启该回调。本回调为周期性回调，回调周期由上述接口的 `interval` 参数指定。
  ///

  FutureOr<void> rtcEngine$onAudioPlaybackDeviceTestVolume(
      ByteRTCEngine engine, int volume) async {}

  /// @hidden(iOS)
  /// @detail callback
  /// @author caocun
  /// @brief 音频设备音量改变回调。当通过系统设置，改变音频设备音量或静音状态时，触发本回调。本回调无需手动开启。
  /// @param engine 参看 ByteRTCEngine{@link #ByteRTCEngine}
  /// @param deviceType 设备类型，包括麦克风和扬声器，参阅 ByteRTCAudioDeviceType{@link #ByteRTCAudioDeviceType}。
  /// @param volume 音量值，[0, 255]。当 volume 变为 0 时，muted 会转为 True。
  /// @param muted 是否禁音状态。扬声器被设置为禁音时，muted 为 True，但 volume 保持不变。
  ///

  FutureOr<void> rtcEngine$onAudioDeviceVolumeChanged$volume$muted(
      ByteRTCEngine engine,
      ByteRTCAudioDeviceType deviceType,
      int volume,
      bool muted) async {}

  /// @detail callback
  /// @author huangshouqin
  /// @brief 远端用户进房后，本地调用 enableAudioPropertiesReport:{@link #ByteRTCEngine#enableAudioPropertiesReport} ，根据设置的 interval 值，本地会周期性地收到此回调，了解订阅的远端用户的瞬时音频信息。 <br>
  ///        远端用户的音频包括使用 RTC SDK 内部机制/自定义机制采集的麦克风音频和屏幕音频。
  /// @param engine ByteRTCEngine 对象
  /// @param audioPropertiesInfos 远端音频信息，其中包含音频流属性、房间 ID、用户 ID ，详见 ByteRTCRemoteAudioPropertiesInfo{@link #ByteRTCRemoteAudioPropertiesInfo}。
  /// @param totalRemoteVolume 所有订阅的远端流混音后的音量，范围是 [0,255]。 <br>
  ///       - [0,25] 接近无声；
  ///       - [25,75] 为低音量；
  ///       - [76,204] 为中音量；
  ///       - [205,255] 为高音量。
  ///

  FutureOr<void> rtcEngine$onRemoteAudioPropertiesReport$totalRemoteVolume(
      ByteRTCEngine engine,
      NSArray<ByteRTCRemoteAudioPropertiesInfo> audioPropertiesInfos,
      NSInteger totalRemoteVolume) async {}

  /// @detail callback
  /// @author zhangcaining
  /// @brief 调用 enableAudioPropertiesReport:{@link #ByteRTCEngine#enableAudioPropertiesReport}  后，根据设置的 `config.interval`，你会周期性地收到此回调，获取房间内的最活跃用户信息。
  /// @param engine `ByteRTCEngine` 实例
  /// @param roomId 房间 ID
  /// @param uid 最活跃用户（ActiveSpeaker）的用户 ID

  FutureOr<void> rtcEngine$onActiveSpeaker$uid(
      ByteRTCEngine engine, NSString roomId, NSString uid) async {}

  /// @hidden for internal use only
  /// @detail callback
  /// @author shenpengliang
  /// @brief 房间内用户暂停/恢复发送视频流时，房间内其他用户会收到此回调。
  /// @param engine ByteRTCEngine 实例
  /// @param roomId 房间 ID
  /// @param uid 暂停/恢复发送视频流的用户 ID。
  /// @param muteState 视频流的发送状态。参看 ByteRTCMuteState{@link #ByteRTCMuteState}。

  FutureOr<void> rtcEngine$onRemoteAudioPropertiesReportEx(ByteRTCEngine engine,
      NSArray<ByteRTCRemoteAudioPropertiesInfo> audioPropertiesInfos) async {}

  /// @detail callback
  /// @author liuyangyang
  /// @brief 房间内的可见用户调用 startVideoCapture{@link #ByteRTCEngine#startVideoCapture} 开启内部视频采集时，房间中其他用户会收到此回调。
  /// @param engine ByteRTCEngine 实例
  /// @param streamId 视频流 ID
  /// @param info 视频流信息，详见 ByteRTCStreamInfo{@link #ByteRTCStreamInfo}。

  FutureOr<void> rtcEngine$onUserStartVideoCapture$info(
      ByteRTCEngine engine, NSString streamId, ByteRTCStreamInfo info) async {}

  /// @detail callback
  /// @author liuyangyang
  /// @brief 房间内的可见用户调用 stopVideoCapture{@link #ByteRTCEngine#stopVideoCapture} 关闭内部视频采集时，房间内其他用户会收到此回调。 <br>
  ///        若发布视频数据前未开启采集，房间内所有可见用户会收到此回调。
  /// @param engine ByteRTCEngine 实例
  /// @param streamId 视频流 ID
  /// @param info 视频流信息，详见 ByteRTCStreamInfo{@link #ByteRTCStreamInfo}。

  FutureOr<void> rtcEngine$onUserStopVideoCapture$info(
      ByteRTCEngine engine, NSString streamId, ByteRTCStreamInfo info) async {}

  /// @detail callback
  /// @author zhangzhenyu.samuel
  /// @brief 第一帧本地采集的视频/屏幕共享画面在本地视图渲染完成时，收到此回调。
  /// @param engine ByteRTCEngine 对象
  /// @param videoSource 视频源对象。参看 ByteRTCVideoSource{@link #ByteRTCVideoSource}。
  /// @param frameInfo 视频帧信息，参看 ByteRTCVideoFrameInfo{@link #ByteRTCVideoFrameInfo}

  FutureOr<void> rtcEngine$onFirstLocalVideoFrameCaptured$withFrameInfo(
      ByteRTCEngine engine,
      ByteRTCVideoSource videoSource,
      ByteRTCVideoFrameInfo frameInfo) async {}

  /// @detail callback
  /// @author zhushufan.ref
  /// @brief SDK 内部渲染成功远端视频流首帧后，收到此回调。包含以下情况： <br>
  ///        1. 发布端首次发布视频 <br>
  ///        2. 在 1 条件下，发布端取消发布视频后，再次发布视频 <br>
  ///        3. 在 1 条件下，发布端关闭视频采集后，再次打开采集（或使用外部源时，停止推流后再次推流） <br>
  ///        4. 在 1 条件下，订阅端取消订阅视频后，再次订阅视频（调用接口包括 subscribeAllStreamsVideo:{@link #ByteRTCRoom-subscribeAllStreamsVideo}pauseAllSubscribedStreamVideo:{@link #ByteRTCRoom-pauseAllSubscribedStreamVideo}/resumeAllSubscribedStreamVideo:{@link #ByteRTCRoom-resumeAllSubscribedStreamVideo}）
  /// @param engine ByteRTCEngine 对象
  /// @param streamId 远端流 ID。
  /// @param info 远端流信息，参看 ByteRTCStreamInfo{@link #ByteRTCStreamInfo}。
  /// @param frameInfo 视频帧信息，参看 ByteRTCVideoFrameInfo{@link #ByteRTCVideoFrameInfo}
  /// @note 仅在采用内部渲染时，符合上述策略。

  FutureOr<void> rtcEngine$onFirstRemoteVideoFrameRendered$info$withFrameInfo(
      ByteRTCEngine engine,
      NSString streamId,
      ByteRTCStreamInfo info,
      ByteRTCVideoFrameInfo frameInfo) async {}

  /// @detail callback
  /// @author zhushufan.ref
  /// @brief 订阅端接收并解码远端视频流首帧后，收到此回调。包含以下情况： <br>
  ///        1. 发布端发布视频，包含首次发布和取消后再次发布。<br>
  ///        2. 发布端关闭视频采集后，再次打开采集。使用外部源时，停止推流后再次推流。<br>
  ///        3. 发布端发布视频后，订阅端取消订阅视频后，又再次订阅视频。
  /// @param engine ByteRTCEngine 对象
  /// @param streamId 远端流 ID。
  /// @param info 远端流信息，参看 ByteRTCStreamInfo{@link #ByteRTCStreamInfo}。
  /// @param frameInfo 视频帧信息，参看 ByteRTCVideoFrameInfo{@link #ByteRTCVideoFrameInfo}
  /// @note
  ///       - 对于主流，进入房间后，仅在发布端第一次发布的时候，订阅端会收到该回调，此后不受重新发布的影响，只要不重新加入房间，就不会再收到该回调。
  ///       - 对于屏幕流，用户每次重新发布屏幕视频流在订阅端都会重新触发一次该回调。

  FutureOr<void> rtcEngine$onFirstRemoteVideoFrameDecoded$info$withFrameInfo(
      ByteRTCEngine engine,
      NSString streamId,
      ByteRTCStreamInfo info,
      ByteRTCVideoFrameInfo frameInfo) async {}

  /// @detail callback
  /// @author zhushufan.ref
  /// @brief 远端视频大小或旋转信息发生改变时，房间内订阅此视频流的用户会收到此回调。
  /// @param engine ByteRTCEngine 对象
  /// @param streamId 远端流 ID。
  /// @param info 远端流信息，参看 ByteRTCStreamInfo{@link #ByteRTCStreamInfo}。
  /// @param frameInfo 视频帧信息，参看 ByteRTCVideoFrameInfo{@link #ByteRTCVideoFrameInfo}

  FutureOr<void> rtcEngine$onRemoteVideoSizeChanged$info$withFrameInfo(
      ByteRTCEngine engine,
      NSString streamId,
      ByteRTCStreamInfo info,
      ByteRTCVideoFrameInfo frameInfo) async {}

  /// @detail callback
  /// @author zhushufan.ref
  /// @brief 本地视频大小或旋转信息发生改变时，收到此回调。
  /// @param engine ByteRTCEngine 对象
  /// @param videoSource 本地视频源对象，仅在多流版本中有效。参看 ByteRTCVideoSource{@link #ByteRTCVideoSource}。
  /// @param frameInfo 视频帧信息，参看 ByteRTCVideoFrameInfo{@link #ByteRTCVideoFrameInfo}

  FutureOr<void> rtcEngine$onLocalVideoSizeChanged$withFrameInfo(
      ByteRTCEngine engine,
      ByteRTCVideoSource videoSource,
      ByteRTCVideoFrameInfo frameInfo) async {}

  /// @detail callback
  /// @author dixing
  /// @brief 音频设备状态回调。提示音频采集、音频播放等设备的状态。
  /// @param engine ByteRTCEngine 实例
  /// @param deviceID 设备 ID
  /// @param deviceType 设备类型，参看 ByteRTCAudioDeviceType{@link #ByteRTCAudioDeviceType}。
  /// @param deviceState 设备状态，参看 ByteRTCMediaDeviceState{@link #ByteRTCMediaDeviceState}。
  /// @param deviceError 设备错误类型，参看 ByteRTCMediaDeviceError{@link #ByteRTCMediaDeviceError}。

  FutureOr<void>
      rtcEngine$onAudioDeviceStateChanged$device_type$device_state$device_error(
          ByteRTCEngine engine,
          NSString deviceID,
          ByteRTCAudioDeviceType deviceType,
          ByteRTCMediaDeviceState deviceState,
          ByteRTCMediaDeviceError deviceError) async {}

  /// @detail callback
  /// @author liuyangyang
  /// @brief 视频设备状态发生改变回调。当设备的视频状态发生改变时，SDK 会触发该回调，提示摄像头视频采集、屏幕视频采集等设备的状态。
  /// @param engine 参看 ByteRTCEngine{@link #ByteRTCEngine}。
  /// @param  deviceID 设备 ID
  /// @param  deviceType 设备类型，参看 ByteRTCVideoDeviceType{@link #ByteRTCVideoDeviceType}。
  /// @param  deviceState 设备状态，参看 ByteRTCMediaDeviceState{@link #ByteRTCMediaDeviceState}。
  /// @param  deviceError 设备错误类型，参看 ByteRTCMediaDeviceError{@link #ByteRTCMediaDeviceError}。

  FutureOr<void>
      rtcEngine$onVideoDeviceStateChanged$device_type$device_state$device_error(
          ByteRTCEngine engine,
          NSString deviceID,
          ByteRTCVideoDeviceType deviceType,
          ByteRTCMediaDeviceState deviceState,
          ByteRTCMediaDeviceError deviceError) async {}

  /// @detail callback
  /// @author dixing
  /// @brief 音频设备警告回调。音频设备包括音频采集设备和音频渲染设备。
  /// @param engine ByteRTCEngine 对象
  /// @param deviceId 设备 ID
  /// @param deviceType 参看 ByteRTCAudioDeviceType{@link #ByteRTCAudioDeviceType}
  /// @param deviceWarning 参看 ByteRTCMediaDeviceWarning{@link #ByteRTCMediaDeviceWarning}

  FutureOr<void> rtcEngine$onAudioDeviceWarning$deviceType$deviceWarning(
      ByteRTCEngine engine,
      NSString deviceId,
      ByteRTCAudioDeviceType deviceType,
      ByteRTCMediaDeviceWarning deviceWarning) async {}

  /// @detail callback
  /// @author liuyangyang
  /// @brief 视频设备警告回调。视频设备包括视频采集设备。
  /// @param engine ByteRTCEngine 对象
  /// @param deviceId 设备 ID
  /// @param deviceType 参看 ByteRTCVideoDeviceType{@link #ByteRTCVideoDeviceType}
  /// @param deviceWarning 参看 ByteRTCMediaDeviceWarning{@link #ByteRTCMediaDeviceWarning}

  FutureOr<void> rtcEngine$onVideoDeviceWarning$deviceType$deviceWarning(
      ByteRTCEngine engine,
      NSString deviceId,
      ByteRTCVideoDeviceType deviceType,
      ByteRTCMediaDeviceWarning deviceWarning) async {}

  /// @detail callback
  /// @author zhangyuanyuan.0101
  /// @brief 本地音频首帧发送状态发生改变时，收到此回调。
  /// @param engine ByteRTCEngine 对象
  /// @param streamId 流 ID
  /// @param info 流信息，参看 ByteRTCStreamInfo{@link #ByteRTCStreamInfo}
  /// @param user 本地用户信息，详见 ByteRTCUser{@link #ByteRTCUser}
  /// @param state 首帧发送状态，详见 ByteRTCFirstFrameSendState{@link #ByteRTCFirstFrameSendState}

  FutureOr<void> rtcEngine$onAudioFrameSendStateChanged$info$rtcUser$state(
      ByteRTCEngine engine,
      NSString streamId,
      ByteRTCStreamInfo info,
      ByteRTCUser user,
      ByteRTCFirstFrameSendState state) async {}

  /// @detail callback
  /// @author shenpengliang
  /// @brief 本地视频首帧发送状态发生改变时，收到此回调。
  /// @param engine ByteRTCEngine 对象
  /// @param streamId 视频流 ID
  /// @param info 视频流信息，参看 ByteRTCStreamInfo{@link #ByteRTCStreamInfo}
  /// @param user 本地用户信息，详见 ByteRTCUser{@link #ByteRTCUser}
  /// @param state 首帧发送状态，详见 ByteRTCFirstFrameSendState{@link #ByteRTCFirstFrameSendState}

  FutureOr<void> rtcEngine$onVideoFrameSendStateChanged$info$rtcUser$state(
      ByteRTCEngine engine,
      NSString streamId,
      ByteRTCStreamInfo info,
      ByteRTCUser user,
      ByteRTCFirstFrameSendState state) async {}

  /// @detail callback
  /// @author zhangyuanyuan.0101
  /// @brief 音频首帧播放状态发生改变时，收到此回调。
  /// @param engine ByteRTCEngine 对象。
  /// @param streamId 音频流 ID
  /// @param info 音频流信息，参看 ByteRTCStreamInfo{@link #ByteRTCStreamInfo}
  /// @param user 音频流来源的用户信息，参看 ByteRTCUser{@link #ByteRTCUser}
  /// @param state 首帧播放状态，参看 ByteRTCFirstFramePlayState{@link #ByteRTCFirstFramePlayState}

  FutureOr<void> rtcEngine$onAudioFramePlayStateChanged$info$rtcUser$state(
      ByteRTCEngine engine,
      NSString streamId,
      ByteRTCStreamInfo info,
      ByteRTCUser user,
      ByteRTCFirstFramePlayState state) async {}

  /// @detail callback
  /// @author shenpengliang
  /// @brief 视频首帧播放状态发生改变时，收到此回调。
  /// @param engine ByteRTCEngine 对象。
  /// @param streamId 视频流 ID
  /// @param info 视频流信息，参看 ByteRTCStreamInfo{@link #ByteRTCStreamInfo}
  /// @param user 视频流来源的用户信息，参看 ByteRTCUser{@link #ByteRTCUser}
  /// @param state 首帧播放状态，参看 ByteRTCFirstFramePlayState{@link #ByteRTCFirstFramePlayState}

  FutureOr<void> rtcEngine$onVideoFramePlayStateChanged$info$rtcUser$state(
      ByteRTCEngine engine,
      NSString streamId,
      ByteRTCStreamInfo info,
      ByteRTCUser user,
      ByteRTCFirstFramePlayState state) async {}

  /// @detail callback
  /// @author zhangyuanyuan.0101
  /// @brief 发布音频流时，采集到第一帧音频帧，收到该回调。
  /// @param engine ByteRTCEngine 对象
  /// @param audioSource 音频源信息，参看 ByteRTCAudioSource{@link #ByteRTCAudioSource}
  /// @note 如果发布音频流时，未开启本地音频采集，SDK 会推送静音帧，也会收到此回调。

  FutureOr<void> rtcEngine$onFirstLocalAudioFrame(
      ByteRTCEngine engine, ByteRTCAudioSource audioSource) async {}

  /// @hidden(macOS)
  /// @detail callback
  /// @author dixing
  /// @brief 音频播放路由变化时，收到该回调。
  /// @param engine `ByteRTCEngine` 实例
  /// @param device 新的音频播放路由，详见 ByteRTCAudioRoute{@link #ByteRTCAudioRoute}
  /// @note 插拔音频外设，或调用 setAudioRoute:{@link #ByteRTCEngine#setAudioRoute} 都可能触发音频路由切换，详见[移动端设置音频路由](https://www.volcengine.com/docs/6348/117836) 。

  FutureOr<void> rtcEngine$onAudioRouteChanged(
      ByteRTCEngine engine, ByteRTCAudioRoute device) async {}

  /// @detail callback
  /// @author wangzhanqiang
  /// @brief 收到通过调用 sendSEIMessage:andRepeatCount:andCountPerFrame:{@link #ByteRTCEngine#sendSEIMessage:andRepeatCount:andCountPerFrame} 发送带有 SEI 消息的视频帧时，收到此回调。
  /// @param engine 当前 ByteRTCEngine 实例。
  /// @param streamId 远端流 ID。
  /// @param info 远端流信息，参看 ByteRTCStreamInfo{@link #ByteRTCStreamInfo}。
  /// @param message 收到的 SEI 消息内容

  FutureOr<void> rtcEngine$onSEIMessageReceived$info$andMessage(
      ByteRTCEngine engine,
      NSString streamId,
      ByteRTCStreamInfo info,
      NSData message) async {}

  /// @detail callback
  /// @author wangzhanqiang
  /// @brief 黑帧视频流发布状态回调。 <br>
  ///        在语音通话场景下，本地用户调用 sendSEIMessage:andRepeatCount:andCountPerFrame:{@link #ByteRTCEngine#sendSEIMessage:andRepeatCount:andCountPerFrame} 通过黑帧视频流发送 SEI 数据时，流的发送状态会通过该回调通知远端用户。 <br>
  ///        你可以通过此回调判断携带 SEI 数据的视频帧为黑帧，从而不对该视频帧进行渲染。
  /// @param engine 当前 ByteRTCEngine 实例。
  /// @param streamId 远端流 ID。
  /// @param info 远端流信息，参看 ByteRTCStreamInfo{@link #ByteRTCStreamInfo}。
  /// @param eventType 黑帧视频流状态，参看 ByteRTCSEIStreamEventType{@link #ByteRTCSEIStreamEventType}

  FutureOr<void> rtcEngine$onSEIStreamUpdate$info$eventType(
      ByteRTCEngine engine,
      NSString streamId,
      ByteRTCStreamInfo info,
      ByteRTCSEIStreamEventType eventType) async {}

  /// @detail callback
  /// @author wangjunzheng
  /// @brief 音频流同步信息回调。可以通过此回调，在远端用户调用 sendStreamSyncInfo:config:{@link #ByteRTCEngine#sendStreamSyncInfo:config} 发送音频流同步消息后，收到远端发送的音频流同步信息。
  /// @param engine 当前 ByteRTCEngine 实例。
  /// @param streamId 远端流 ID。
  /// @param info 远端流信息，参看 ByteRTCStreamInfo{@link #ByteRTCStreamInfo}。
  /// @param streamType 媒体流类型，详见 ByteRTCSyncInfoStreamType{@link #ByteRTCSyncInfoStreamType} 。
  /// @param data 消息内容。

  FutureOr<void> rtcEngine$onStreamSyncInfoReceived$info$streamType$data(
      ByteRTCEngine engine,
      NSString streamId,
      ByteRTCStreamInfo info,
      ByteRTCSyncInfoStreamType streamType,
      NSData data) async {}

  /// @detail callback
  /// @author wangzhanqiang
  /// @brief 报告当前设备 cpu 与 memory 使用率，每 2s 触发一次。
  /// @param engine ByteRTCEngine 对象
  /// @param stats cpu 和 memory 使用率信息，详见 ByteRTCSysStats{@link #ByteRTCSysStats} 数据类型

  FutureOr<void> rtcEngine$onSysStats(
      ByteRTCEngine engine, ByteRTCSysStats stats) async {}

  /// @detail callback
  /// @author zhangyuanyuan.0101
  /// @brief 本地音频流的状态发生改变时，收到此回调。
  /// @param engine ByteRTCEngine 对象
  /// @param audioSource 音频流对象，参看 ByteRTCAudioSource{@link #ByteRTCAudioSource}。
  /// @param state 本地音频设备的当前状态，详见 ByteRTCLocalAudioStreamState{@link #ByteRTCLocalAudioStreamState}
  /// @param error 本地音频流状态改变时的错误码，详见 ByteRTCLocalAudioStreamError{@link #ByteRTCLocalAudioStreamError}

  FutureOr<void> rtcEngine$onLocalAudioStateChanged$state$error(
      ByteRTCEngine engine,
      ByteRTCAudioSource audioSource,
      ByteRTCLocalAudioStreamState state,
      ByteRTCLocalAudioStreamError error) async {}

  /// @detail callback
  /// @author zhangyuanyuan.0101
  /// @brief 订阅的远端音频流状态发生改变时，收到此回调。
  /// @param engine ByteRTCEngine 对象
  /// @param streamId 远端流 ID。
  /// @param info 远端流信息，参看 ByteRTCStreamInfo{@link #ByteRTCStreamInfo}。
  /// @param state 远端音频流的当前状态，详见 ByteRTCRemoteAudioState{@link #ByteRTCRemoteAudioState}
  /// @param reason 远端音频流状态改变的原因，详见 ByteRTCRemoteAudioStateChangeReason{@link #ByteRTCRemoteAudioStateChangeReason}

  FutureOr<void> rtcEngine$onRemoteAudioStateChanged$info$state$reason(
      ByteRTCEngine engine,
      NSString streamId,
      ByteRTCStreamInfo info,
      ByteRTCRemoteAudioState state,
      ByteRTCRemoteAudioStateChangeReason reason) async {}

  /// @detail callback
  /// @author shenpengliang
  /// @brief 本地视频流的状态发生改变时，收到该事件。
  /// @param engine ByteRTCEngine 对象
  /// @param videoSource 视频流对象，参看 ByteRTCVideoSource{@link #ByteRTCVideoSource}。
  /// @param state 本地视频流的当前状态，参看 ByteRTCLocalVideoStreamState{@link #ByteRTCLocalVideoStreamState}
  /// @param error 本地视频状态改变时的错误码，参看 ByteRTCLocalVideoStreamError{@link #ByteRTCLocalVideoStreamError}

  FutureOr<void>
      rtcEngine$onLocalVideoStateChanged$withStreamState$withStreamError(
          ByteRTCEngine engine,
          ByteRTCVideoSource videoSource,
          ByteRTCLocalVideoStreamState state,
          ByteRTCLocalVideoStreamError error) async {}

  /// @detail callback
  /// @author shenpengliang
  /// @brief 远端视频流的状态发生改变时，房间内订阅此流的用户会收到该事件。
  /// @param engine ByteRTCEngine 对象
  /// @param streamId 远端流 ID。
  /// @param info 远端流信息，参看 ByteRTCStreamInfo{@link #ByteRTCStreamInfo}。
  /// @param state 远端视频流的当前状态，参看 ByteRTCRemoteVideoState{@link #ByteRTCRemoteVideoState}
  /// @param reason 远端视频流状态改变的原因，参看 ByteRTCRemoteVideoStateChangeReason{@link #ByteRTCRemoteVideoStateChangeReason}
  /// @note 本回调仅适用于主流，不适用于屏幕流。

  FutureOr<void>
      rtcEngine$onRemoteVideoStateChanged$info$withVideoState$withVideoStateReason(
          ByteRTCEngine engine,
          NSString streamId,
          ByteRTCStreamInfo info,
          ByteRTCRemoteVideoState state,
          ByteRTCRemoteVideoStateChangeReason reason) async {}

  /// @hidden not available on iOS
  /// @valid since 3.54
  /// @detail callback
  /// @author yinkaisheng
  /// @brief 远端视频流的超分状态发生改变时，房间内订阅此流的用户会收到该回调。
  /// @param engine `ByteRTCEngine` 实例
  /// @param streamId 远端流 ID。
  /// @param info 远端流信息，参看 ByteRTCStreamInfo{@link #ByteRTCStreamInfo}。
  /// @param mode 超分模式，参看 ByteRTCVideoSuperResolutionMode{@link #ByteRTCVideoSuperResolutionMode}。
  /// @param reason 超分模式改变原因，参看 ByteRTCVideoSuperResolutionModeChangedReason{@link #ByteRTCVideoSuperResolutionModeChangedReason}。

  FutureOr<void>
      rtcEngine$onRemoteVideoSuperResolutionModeChanged$info$withMode$withReason(
          ByteRTCEngine engine,
          NSString streamId,
          ByteRTCStreamInfo info,
          ByteRTCVideoSuperResolutionMode mode,
          ByteRTCVideoSuperResolutionModeChangedReason reason) async {}

  /// @valid since 3.54
  /// @hidden not available on iOS
  /// @detail callback
  /// @author Yujianli
  /// @brief 降噪模式状态变更回调。当降噪模式的运行状态发生改变，SDK 会触发该回调，提示用户降噪模式改变后的运行状态及状态发生改变的原因。
  /// @param engine `ByteRTCEngine` 实例
  /// @param mode 视频降噪模式，参看 ByteRTCVideoDenoiseMode{@link #ByteRTCVideoDenoiseMode}。
  /// @param reason 视频降噪模式改变的原因，参看 ByteRTCVideoDenoiseModeChangedReason{@link #ByteRTCVideoDenoiseModeChangedReason}。

  FutureOr<void> rtcEngine$onVideoDenoiseModeChanged$withReason(
      ByteRTCEngine engine,
      ByteRTCVideoDenoiseMode mode,
      ByteRTCVideoDenoiseModeChangedReason reason) async {}

  /// @detail callback
  /// @author hanchenchen.c
  /// @brief 登录结果回调
  /// @param engine ByteRTCEngine 对象
  /// @param uid <br>
  ///        登录用户 ID
  /// @param errorCode <br>
  ///        登录结果 <br>
  ///        详见 ByteRTCLoginErrorCode{@link #ByteRTCLoginErrorCode}。
  /// @param elapsed <br>
  ///        从调用 login:uid:{@link #ByteRTCEngine#login:uid} 接口开始到返回结果所用时长。 <br>
  ///        单位为 ms。
  /// @note 调用 login:uid:{@link #ByteRTCEngine#login:uid} 后，会收到此回调。

  FutureOr<void> rtcEngine$onLoginResult$errorCode$elapsed(ByteRTCEngine engine,
      NSString uid, ByteRTCLoginErrorCode errorCode, NSInteger elapsed) async {}

  /// @detail callback
  /// @author hanchenchen.c
  /// @brief 登出结果回调
  /// @param engine ByteRTCEngine 对象
  /// @param reason 用户登出的原因，参看 ByteRTCLogoutReason{@link #ByteRTCLogoutReason}
  /// @note 在以下两种情况下会收到此回调：调用 logout{@link #ByteRTCEngine#logout} 接口主动退出；或其他用户以相同 UserId 进行 `login` 导致本地用户被动登出。

  FutureOr<void> rtcEngine$onLogout(
      ByteRTCEngine engine, ByteRTCLogoutReason reason) async {}

  /// @detail callback
  /// @author hanchenchen.c
  /// @brief 设置应用服务器参数的返回结果
  /// @param engine ByteRTCEngine 对象
  /// @param errorCode <br>
  ///        设置结果 <br>
  ///        - 返回 200，设置成功
  ///        - 返回其他，设置失败，详见 ByteRTCUserMessageSendResult{@link #ByteRTCUserMessageSendResult}
  /// @note 调用 setServerParams:url:{@link #ByteRTCEngine#setServerParams:url} 后，会收到此回调。

  FutureOr<void> rtcEngine$onServerParamsSetResult(
      ByteRTCEngine engine, NSInteger errorCode) async {}

  /// @detail callback
  /// @author hanchenchen.c
  /// @brief 查询对端或本端用户登录状态的返回结果
  /// @param engine ByteRTCEngine 对象
  /// @param peerUserId <br>
  ///        需要查询的用户 ID
  /// @param status <br>
  ///        查询的用户登录状态 <br>
  ///        详见 ByteRTCUserOnlineStatus{@link #ByteRTCUserOnlineStatus}.
  /// @note 必须先调用 getPeerOnlineStatus:{@link #ByteRTCEngin e#getPeerOnlineStatus}，才能收到此回调。

  FutureOr<void> rtcEngine$onGetPeerOnlineStatus$status(ByteRTCEngine engine,
      NSString peerUserId, ByteRTCUserOnlineStatus status) async {}

  /// @detail callback
  /// @author hanchenchen.c
  /// @brief > 该接口将于 3.64 rtcEngine:onUserMessageReceivedOutsideRoom:uid:message:{@link #ByteRTCEngineDelegate#rtcEngine:onUserMessageReceivedOutsideRoom:uid:message} 代替。
  /// @brief 收到房间外用户调用 sendUserMessageOutsideRoom:message:config:{@link #ByteRTCEngine#sendUserMessageOutsideRoom:message:config} 发来的文本消息时，会收到此回调
  /// @param engine ByteRTCEngine 对象。
  /// @param uid 消息发送者 ID。
  /// @param message 收到的文本消息内容。

  FutureOr<void> rtcEngine$onUserMessageReceivedOutsideRoom$message(
      ByteRTCEngine engine, NSString uid, NSString message) async {}

  /// @detail callback
  /// @author hanchenchen.c
  /// @brief > 该接口将于 3.64 rtcEngine:onUserBinaryMessageReceivedOutsideRoom:uid:message:{@link #ByteRTCEngineDelegate#rtcEngine:onUserBinaryMessageReceivedOutsideRoom:uid:message} 代替。
  /// @brief 收到房间外用户调用 sendUserBinaryMessageOutsideRoom:message:config:{@link #ByteRTCEngine#sendUserBinaryMessageOutsideRoom:message:config} 发来的二进制消息时，会收到此回调。
  /// @param engine ByteRTCEngine 对象
  /// @param uid 消息发送者 ID。
  /// @param message 收到的二进制消息内容。

  FutureOr<void> rtcEngine$onUserBinaryMessageReceivedOutsideRoom$message(
      ByteRTCEngine engine, NSString uid, NSData message) async {}

  /// @detail callback
  /// @author hanchenchen.c
  /// @brief 收到房间外用户调用 sendUserMessageOutsideRoom:message:config:{@link #ByteRTCEngine#sendUserMessageOutsideRoom:message:config} 发来的文本消息时，会收到此回调
  /// @param engine ByteRTCEngine 对象。
  /// @param msgid 消息编号。
  /// @param uid 消息发送者 ID。
  /// @param message 收到的文本消息内容。

  FutureOr<void> rtcEngine$onUserMessageReceivedOutsideRoom$uid$message(
      ByteRTCEngine engine,
      NSInteger msgid,
      NSString uid,
      NSString message) async {}

  /// @detail callback
  /// @author hanchenchen.c
  /// @brief 收到房间外用户调用 sendUserBinaryMessageOutsideRoom:message:config:{@link #ByteRTCEngine#sendUserBinaryMessageOutsideRoom:message:config} 发来的二进制消息时，会收到此回调。
  /// @param engine ByteRTCEngine 对象
  /// @param msgid 消息编号。
  /// @param uid 消息发送者 ID。
  /// @param message 收到的二进制消息内容。

  FutureOr<void> rtcEngine$onUserBinaryMessageReceivedOutsideRoom$uid$message(
      ByteRTCEngine engine,
      NSInteger msgid,
      NSString uid,
      NSData message) async {}

  /// @detail callback
  /// @author hanchenchen.c
  /// @brief 给房间外指定的用户发送消息的回调
  /// @param engine ByteRTCEngine 对象
  /// @param msgid <br>
  ///        本条消息的 ID <br>
  ///        所有的 P2P 和 P2Server 消息共用一个 ID 序列。
  /// @param error <br>
  ///        消息发送结果 <br>
  ///        详见 ByteRTCUserMessageSendResult{@link #ByteRTCUserMessageSendResult}。
  /// @note 当调用 sendUserMessageOutsideRoom:message:config:{@link #ByteRTCEngine#sendUserMessageOutsideRoom:message:config} 或 sendUserBinaryMessageOutsideRoom:message:config:{@link #ByteRTCEngine#sendUserBinaryMessageOutsideRoom:message:config} 发送消息后，会收到此回调。

  FutureOr<void> rtcEngine$onUserMessageSendResultOutsideRoom$error(
      ByteRTCEngine engine,
      NSInteger msgid,
      ByteRTCUserMessageSendResult error) async {}

  /// @detail callback
  /// @author hanchenchen.c
  /// @brief 给应用服务器发送消息的回调。
  /// @param engine ByteRTCEngine 对象。
  /// @param msgid 本条消息的 ID。 <br>
  ///              所有的 P2P 和 P2Server 消息共用一个 ID 序列。
  /// @param error 消息发送结果。详见 ByteRTCUserMessageSendResult{@link #ByteRTCUserMessageSendResult}。
  /// @param message 应用服务器收到 HTTP 请求后，在 ACK 中返回的信息。消息不超过 64 KB。
  /// @note 本回调为异步回调。当调用 sendServerMessage:{@link #ByteRTCEngin e#sendServerMessage} 或 sendServerBinaryMessage:{@link #ByteEngin e#sendServerBinaryMessage} 接口发送消息后，会收到此回调。

  FutureOr<void> rtcEngine$onServerMessageSendResult$error$message(
      ByteRTCEngine engine,
      int64_t msgid,
      ByteRTCUserMessageSendResult error,
      NSData message) async {}

  /// @detail callback
  /// @author hanchenchen.c
  /// @brief 通话前网络探测结果的回调。 <br>
  ///        成功调用 startNetworkDetection:uplinkBandwidth:downlink:downlinkBandwidth:{@link #ByteRTCEngine#startNetworkDetection:uplinkBandwidth:downlink:downlinkBandwidth} 接口开始探测后，会在 3s 内首次收到该回调，之后每 2s 收到一次该回调。
  /// @param engine ByteRTCEngine 对象
  /// @param type 探测网络类型为上行/下行
  /// @param quality 探测网络的质量，参看 ByteRTCNetworkQuality{@link #ByteRTCNetworkQuality}。
  /// @param rtt 探测网络的 RTT，单位：ms
  /// @param lostRate 探测网络的丢包率
  /// @param bitrate 探测网络的带宽，单位：kbps
  /// @param jitter 探测网络的抖动,单位：ms

  FutureOr<void>
      rtcEngine$onNetworkDetectionResult$quality$rtt$lostRate$bitrate$jitter(
          ByteRTCEngine engine,
          ByteRTCNetworkDetectionLinkType type,
          ByteRTCNetworkQuality quality,
          int rtt,
          double lostRate,
          int bitrate,
          int jitter) async {}

  /// @detail callback
  /// @author hanchenchen.c
  /// @brief 通话前网络探测结束 <br>
  ///        以下情况将停止探测并收到一次本回调： <br>
  ///        1. 当调用 stopNetworkDetection{@link #ByteRTCEngine#stopNetworkDetection} 接口停止探测后，会收到一次该回调； <br>
  ///        2. 当收到远端/本端音频首帧后，停止探测； <br>
  ///        3. 当探测超过 3 分钟后，停止探测； <br>
  ///        4. 当探测链路断开一定时间之后，停止探测。
  /// @param engine ByteRTCEngine 对象
  /// @param errorCode <br>
  ///        停止探测的原因类型,参考 ByteRTCNetworkDetectionStopReason{@link #ByteRTCNetworkDetectionStopReason}

  FutureOr<void> rtcEngine$onNetworkDetectionStopped(ByteRTCEngine engine,
      ByteRTCNetworkDetectionStopReason errorCode) async {}

  /// @detail callback
  /// @author songxiaomeng.19
  /// @brief 混音音频文件播放进度回调
  /// @param engine 当前 RTC SDK 对象
  /// @param mixId 混音 ID
  /// @param progress 当前混音音频文件播放进度，单位毫秒
  /// @note 调用 setAudioMixingProgressInterval:interval: 将时间间隔设为大于 0 的值后，或调用 startAudioMixing:filePath:config: 将 ByteRTCAudioMixingConfig 中的时间间隔设为大于 0 的值后，SDK 会按照设置的时间间隔回调该事件。

  FutureOr<void> rtcEngine$onAudioMixingPlayingProgress$progress(
      ByteRTCEngine engine, NSInteger mixId, int64_t progress) async {}

  /// @detail callback
  /// @author panjian.fishing
  /// @brief 本地未通过 setPublishFallbackOption:{@link #ByteRTCEngine#setPublishFallbackOption} 开启发布性能回退，检测到设备性能不足时，收到此回调。 <br>
  ///        本地通过 setPublishFallbackOption:{@link #ByteRTCEngine#setPublishFallbackOption} 开启发布性能回退，因设备性能/网络原因，造成发布性能回退/恢复时，收到此回调。
  /// @param engine `ByteRTCEngine` 实例
  /// @param streamId 流 ID
  /// @param info 流信息, 参看 ByteRTCStreamInfo{@link #ByteRTCStreamInfo}。
  /// @param mode 指示本地是否开启发布回退功能。参看 ByteRTCPerformanceAlarmMode{@link #ByteRTCPerformanceAlarmMode} <br>
  ///             - 当发布端未开启发布性能回退时，mode 值为 ByteRTCPerformanceAlarmModeNormal。
  ///             - 当发布端开启发布性能回退时，mode 值为 ByteRTCPerformanceAlarmModeSimulcast。
  /// @param reason 告警原因，参看 ByteRTCPerformanceAlarmReason{@link #ByteRTCPerformanceAlarmReason}
  /// @param data 性能回退相关数据，详见 ByteRTCSourceWantedData{@link #ByteRTCSourceWantedData}。

  FutureOr<void>
      rtcEngine$onPerformanceAlarms$info$mode$reason$sourceWantedData(
          ByteRTCEngine engine,
          NSString streamId,
          ByteRTCStreamInfo info,
          ByteRTCPerformanceAlarmMode mode,
          ByteRTCPerformanceAlarmReason reason,
          ByteRTCSourceWantedData data) async {}

  /// @detail callback
  /// @author panjian.fishing
  /// @brief 音视频流因网络环境变化等原因发生回退，或从回退中恢复时，触发该回调。
  /// @param engine 当前 ByteRTCEngine 实例
  /// @param streamId 流 ID
  /// @param info 流信息, 参看 ByteRTCStreamInfo{@link #ByteRTCStreamInfo}。
  /// @param event 流切换信息。详见 ByteRTCRemoteStreamSwitchEvent{@link #ByteRTCRemoteStreamSwitchEvent}。

  FutureOr<void> rtcEngine$onSimulcastSubscribeFallback$info$event(
      ByteRTCEngine engine,
      NSString streamId,
      ByteRTCStreamInfo info,
      ByteRTCRemoteStreamSwitchEvent event) async {}

  /// @hidden
  /// @detail callback
  /// @author zhoubohui
  /// @brief 外部采集时，调用 setOriginalScreenVideoInfo:withOriginalCaptureHeight: 设置屏幕或窗口大小改变前的分辨率后，若屏幕采集模式为智能模式，你将收到此回调，根据 RTC 智能决策合适的帧率和分辨率积（宽*高）重新采集。
  /// @param engine 参看 ByteRTCEngine{@link #ByteRTCEngine}
  /// @param frameUpdateInfo RTC 智能决策后合适的帧率和分辨率积（宽*高）。参看 ByteRTCFrameUpdateInfo{@link #ByteRTCFrameUpdateInfo}。

  FutureOr<void> rtcEngine$onExternalScreenFrameUpdate(
      ByteRTCEngine engine, ByteRTCFrameUpdateInfo frameUpdateInfo) async {}

  /// @detail callback
  /// @author wangzhanqiang
  /// @brief 获取本地录制状态回调。 <br>
  ///        该回调由 startFileRecording:type:{@link #ByteRTCEngine#startFileRecording:type} 或 stopFileRecording:{@link #ByteRTCEngine#stopFileRecording} 触发。
  /// @param engine ByteRTCEngine 对象
  /// @param videoSource 视频源，参看 ByteRTCVideoSource{@link #ByteRTCVideoSource}
  /// @param state 录制状态，参看 ByteRTCRecordingState{@link #ByteRTCRecordingState}
  /// @param errorCode 录制错误码，参看 ByteRTCRecordingErrorCode{@link #ByteRTCRecordingErrorCode}
  /// @param recordingInfo 录制文件的详细信息，参看 ByteRTCRecordingInfo{@link #ByteRTCRecordingInfo}

  FutureOr<void>
      rtcEngine$onRecordingStateUpdate$state$error_code$recording_info(
          ByteRTCEngine engine,
          ByteRTCVideoSource videoSource,
          ByteRTCRecordingState state,
          ByteRTCRecordingErrorCode errorCode,
          ByteRTCRecordingInfo recordingInfo) async {}

  /// @detail callback
  /// @author wangzhanqiang
  /// @brief 本地录制进度回调。 <br>
  ///        该回调由 startFileRecording:type:{@link #ByteRTCEngine#startFileRecording:type} 触发，录制状态正常时，系统每秒钟都会通过该回调提示录制进度。
  /// @param engine ByteRTCEngine 对象
  /// @param videoSource 视频源，参看 ByteRTCVideoSource{@link #ByteRTCVideoSource}
  /// @param process 录制进度，参看 ByteRTCRecordingProgress{@link #ByteRTCRecordingProgress}
  /// @param recordingInfo 录制文件的详细信息，参看 ByteRTCRecordingInfo{@link #ByteRTCRecordingInfo}

  FutureOr<void> rtcEngine$onRecordingProgressUpdate$process$recording_info(
      ByteRTCEngine engine,
      ByteRTCVideoSource videoSource,
      ByteRTCRecordingProgress process,
      ByteRTCRecordingInfo recordingInfo) async {}

  /// @detail callback
  /// @author huangshouqin
  /// @brief 调用 startAudioRecording:{@link #ByteRTCEngine#startAudioRecording} 或者 stopAudioRecording{@link #ByteRTCEngine#stopAudioRecording} 改变音频文件录制状态时，收到此回调。
  /// @param engine `ByteRTCEngine` 实例
  /// @param state 录制状态，参看 ByteRTCAudioRecordingState{@link #ByteRTCAudioRecordingState}
  /// @param errorCode 录制错误码，参看 ByteRTCAudioRecordingErrorCode{@link #ByteRTCAudioRecordingErrorCode}

  FutureOr<void> rtcEngine$onAudioRecordingStateUpdate$error_code(
      ByteRTCEngine engine,
      ByteRTCAudioRecordingState state,
      ByteRTCAudioRecordingErrorCode errorCode) async {}

  /// @detail callback
  /// @author daining.nemo
  /// @brief 调用 startCloudProxy:{@link #ByteRTCEngine#startCloudProxy} 开启云代理，SDK 首次成功连接云代理服务器时，回调此事件。
  /// @param engine `ByteRTCEngine` 实例
  /// @param interval 从开启云代理到连接成功经过的时间，单位为 ms

  FutureOr<void> rtcEngine$onCloudProxyConnected(
      ByteRTCEngine engine, NSInteger interval) async {}

  /// @detail callback
  /// @author qipengxiang
  /// @brief 关于音视频回路测试结果的回调。
  /// @param engine `ByteRTCEngine` 实例
  /// @param result 测试结果，参看 ByteRTCEchoTestResult{@link #ByteRTCEchoTestResult}。
  /// @note 该回调触发的时机包括： <br>
  ///        - 检测过程中采集设备发生错误时；
  ///        - 检测成功后；
  ///        - 非设备原因导致检测过程中未接收到音/视频回放，停止检测后。

  FutureOr<void> rtcEngine$onEchoTestResult(
      ByteRTCEngine engine, ByteRTCEchoTestResult result) async {}

  /// @hidden for internal use only
  /// @detail callback
  /// @author songxiaomeng.19
  /// @brief 音频 dump 状态回调。
  /// @param engine `ByteRTCEngine` 实例
  /// @param status 音频 dump 状态，参看 ByteRTCAudioDumpStatus{@link #ByteRTCAudioDumpStatus}。
  /// @note 本回调用于内部排查音质相关异常问题，开发者无需关注。

  FutureOr<void> rtcEngine$onAudioDumpStateChanged(
      ByteRTCEngine engine, ByteRTCAudioDumpStatus status) async {}

  /// @detail callback
  /// @author songxiaomeng.19
  /// @brief 首次调用 getNetworkTimeInfo{@link #ByteRTCEngine#getNetworkTimeInfo} 后，SDK 内部启动网络时间同步，同步完成时会触发此回调。
  /// @param engine ByteRTCEngine{@link #ByteRTCEngine} 对象

  FutureOr<void> rtcEngineOnNetworkTimeSynchronized(
      ByteRTCEngine engine) async {}

  /// @hidden internal use only
  /// @detail callback
  /// @author wangyu.1705
  /// @brief license 过期时间提醒
  /// @param engine ByteRTCEngine{@link #ByteRTCEngine} 对象
  /// @param days 过期时间天数

  FutureOr<void> rtcEngine$onLicenseWillExpire(
      ByteRTCEngine engine, NSInteger days) async {}

  /// @detail callback
  /// @author zhangcaining
  /// @brief 通话前回声检测结果回调。
  /// @param engine `ByteRTCEngine` 实例
  /// @param result 参见 ByteRTCHardwareEchoDetectionResult{@link #ByteRTCHardwareEchoDetectionResult}。
  /// @note
  ///        - 通话前调用 startHardwareEchoDetection:{@link #ByteRTCEngine#startHardwareEchoDetection} 后，将触发本回调返回检测结果。
  ///        - 建议在收到检测结果后，调用 stopHardwareEchoDetection{@link #ByteRTCEngine#stopHardwareEchoDetection} 停止检测，释放对音频设备的占用。
  ///        - 如果 SDK 在通话中检测到回声，将通过 rtcEngine:onAudioDeviceWarning:deviceType:deviceWarning:{@link #ByteRTCEngineDelegate#rtcEngine:onAudioDeviceWarning:deviceType:deviceWarning} 回调 `ByteRTCMediaDeviceWarningDetectLeakEcho`。

  FutureOr<void> rtcEngine$onHardwareEchoDetectionResult(
      ByteRTCEngine engine, ByteRTCHardwareEchoDetectionResult result) async {}

  /// @detail callback
  /// @author keshixing.rtc
  /// @brief 本地代理状态发生改变回调。调用 setLocalProxy:{@link #ByteRTCEngin e#setLocalProxy} 设置本地代理后，SDK 会触发此回调，返回代理连接的状态。
  /// @param engine `ByteRTCEngine` 实例
  /// @param type 本地代理类型。参看 ByteRTCLocalProxyType{@link #ByteRTCLocalProxyType} 。
  /// @param state 本地代理状态。参看 ByteRTCLocalProxyState{@link #ByteRTCLocalProxyState}。
  /// @param error 本地代理错误。参看 ByteRTCLocalProxyError{@link #ByteRTCLocalProxyError}。

  FutureOr<void>
      rtcEngine$onLocalProxyStateChanged$withProxyState$withProxyError(
          ByteRTCEngine engine,
          ByteRTCLocalProxyType type,
          ByteRTCLocalProxyState state,
          ByteRTCLocalProxyError error) async {}

  /// @hidden internal use only
  /// @detail callback
  /// @author wangqianqian.1104
  /// @brief 当特效设置失败时，收到此回调。
  /// @param engine `ByteRTCEngine` 实例
  /// @param error 特效错误类型。参看 ByteRTCEffectErrorType{@link #ByteRTCEffectErrorType}。
  /// @param msg 错误信息。

  FutureOr<void> rtcEngine$onEffectError$msg(
      ByteRTCEngine engine, ByteRTCEffectErrorType error, NSString msg) async {}

  /// @valid since 3.60.
  /// @hidden(Linux)
  /// @detail callback
  /// @author lizheng
  /// @brief 合流转推 CDN / WTN 流状态回调
  /// @param engine ByteRTCEngine 对象。
  /// @param event 任务事件，参看 ByteRTCMixedStreamTaskEvent{@link #ByteRTCMixedStreamTaskEvent}。
  /// @param info 任务详情，参看 ByteRTCMixedStreamTaskInfo{@link #ByteRTCMixedStreamTaskInfo}。
  /// @param errorCode 任务错误码，参看 ByteRTCMixedStreamTaskErrorCode{@link #ByteRTCMixedStreamTaskErrorCode}。
  /// @order 0
  ///

  FutureOr<void> rtcEngine$onMixedStreamEvent$withMixedStreamInfo$withErrorCode(
      ByteRTCEngine engine,
      ByteRTCMixedStreamTaskEvent event,
      ByteRTCMixedStreamTaskInfo info,
      ByteRTCMixedStreamTaskErrorCode errorCode) async {}

  /// @valid since 3.60.
  /// @detail callback
  /// @author lizheng
  /// @brief 单流转推 CDN 状态回调。
  /// @param engine ByteRTCEngine 对象。
  /// @param event 任务状态, 参看 ByteRTCSingleStreamTaskEvent{@link #ByteRTCSingleStreamTaskEvent}。
  /// @param taskId 任务 ID。
  /// @param errorCode 错误码，参看 ByteRTCSingleStreamTaskErrorCode{@link #ByteRTCSingleStreamTaskErrorCode}。
  ///

  FutureOr<void> rtcEngine$onSingleStreamEvent$withTaskId$withErrorCode(
      ByteRTCEngine engine,
      ByteRTCSingleStreamTaskEvent event,
      NSString taskId,
      ByteRTCSingleStreamTaskErrorCode errorCode) async {}

  /// @detail callback
  /// @valid since 3.60.
  /// @brief 调用 takeRemoteSnapshotToFile:filePath:{@link #ByteRTCEngine#takeRemoteSnapshotToFile:filePath} 截取视频画面时，会收到此回调报告截图是否成功，以及截取的图片信息。
  /// @param engine ByteRTCEngine 对象。
  /// @param streamId 被截图的视频流 ID。
  /// @param info 视频流信息，参考 ByteRTCStreamInfo{@link #ByteRTCStreamInfo}。
  /// @param filePath 截图文件的保存路径。
  /// @param width 截图图像的宽度。单位：像素。
  /// @param height 截图图像的高度。单位：像素。
  /// @param errorCode 截图错误码。参看 ByteRTCSnapshotErrorCode{@link #ByteRTCSnapshotErrorCode}。
  /// @param taskId 截图任务的编号，和 takeRemoteSnapshotToFile:filePath:{@link #ByteRTCEngine#takeRemoteSnapshotToFile:filePath} 的返回值一致。

  FutureOr<void>
      rtcEngine$onRemoteSnapshotTakenToFile$info$filePath$width$height$errorCode$taskId(
          ByteRTCEngine engine,
          NSString streamId,
          ByteRTCStreamInfo info,
          NSString filePath,
          NSInteger width,
          NSInteger height,
          ByteRTCSnapshotErrorCode errorCode,
          NSInteger taskId) async {}

  /// @detail callback
  /// @valid since 3.60.
  /// @brief 调用 takeLocalSnapshotToFile:{@link #ByteRTCEngine#takeLocalSnapshotToFile} 截取视频画面时，会收到此回调报告截图是否成功，以及截取的图片信息。
  /// @param engine ByteRTCEngine 对象。
  /// @param videoSource 视频源，参看 ByteRTCVideoSource{@link #ByteRTCVideoSource}
  /// @param filePath 截图文件的保存路径。
  /// @param width 截图图像的宽度。单位：像素。
  /// @param height 截图图像的高度。单位：像素。
  /// @param errorCode 截图错误码。参看 ByteRTCSnapshotErrorCode{@link #ByteRTCSnapshotErrorCode}。
  /// @param taskId 截图任务的编号，和 takeLocalSnapshotToFile:{@link #ByteRTCEngine#takeLocalSnapshotToFile} 的返回值一致。

  FutureOr<void>
      rtcEngine$onLocalSnapshotTakenToFile$filePath$width$height$errorCode$taskId(
          ByteRTCEngine engine,
          ByteRTCVideoSource videoSource,
          NSString filePath,
          NSInteger width,
          NSInteger height,
          ByteRTCSnapshotErrorCode errorCode,
          NSInteger taskId) async {}

  /// @hidden for internal use only
  /// @detail callback
  /// @author liujingchao
  /// @brief 客户端合流视频首帧回调
  /// @param taskId 任务 ID

  FutureOr<void> rtcEngine$onClientMixedFirstVideoFrame(
      ByteRTCEngine engine, NSString taskId) async {}

  /// @hidden internal use only
  /// @valid since 3.60.
  /// @detail callback
  /// @author hegangjie
  /// @brief 试验性接口回调
  /// @param param 回调内容(JSON string)

  FutureOr<void> rtcEngine$onExperimentalCallback(
      ByteRTCEngine engine, NSString param) async {}

  /// @deprecated since 3.45 and will be deleted in 3.51.
  /// @detail callback
  /// @author chenweiming.push
  /// @brief SDK 内部日志回调。 <br>
  ///        SDK 内部运行时，会把日志回调给业务方，方便排查问题。
  /// @param engine ByteRTCEngine 对象。
  /// @param dict 日志内容。

  FutureOr<void> rtcEngine$log(ByteRTCEngine engine, NSDictionary dict) async {}

  /// @deprecated since 3.60, use rtcEngine:onMixedStreamEvent:withMixedStreamInfo:withErrorCode:{@link #ByteRTCEngineDelegate#rtcEngine:onMixedStreamEvent:withMixedStreamInfo:withErrorCode} instead.
  /// @detail callback
  /// @author qipengxiang
  /// @brief WTN 流发布结果回调。 <br>
  ///        调用 startPushMixedStream:withPushTargetConfig:withMixedConfig:{@link #ByteRTCEngine#startPushMixedStream:withPushTargetConfig:withMixedConfig} <br>
  ///        接口或直接在服务端启动推 WTN 流功能后，你会通过此回调收到启动结果和推流过程中的错误。
  /// @param engine engine 实例
  /// @param roomId 发布 WTN 流的房间 ID
  /// @param streamId WTN 流 ID。
  /// @param errorCode WTN 流发布结果状态码。 详见 ByteRTCPublicStreamErrorCode{@link #ByteRTCPublicStreamErrorCode}。

  FutureOr<void> rtcEngine$onPushPublicStreamResult$publicStreamId$errorCode(
      ByteRTCEngine engine,
      NSString roomId,
      NSString streamId,
      ByteRTCPublicStreamErrorCode errorCode) async {}

  /// @detail callback
  /// @author songxiaomeng.19
  /// @brief 首次调用 getNetworkTimeInfo{@link #ByteRTCEngine#getNetworkTimeInfo} 后，SDK 内部启动网络时间同步，同步完成时会触发此回调。
  /// @param engine ByteRTCEngine{@link #ByteRTCEngine} 对象

  FutureOr<void> rtcEngine$onNetworkTimeSynchronized() async {}
}

class ByteRTCRTSRoomDelegate extends NativeObserverClass {
  static const _$namespace = r'ByteRTCRTSRoomDelegate';

  ByteRTCRTSRoomDelegate([NativeClassOptions? options])
      : super(options == null
            ? const NativeClassOptions([],
                className: _$namespace,
                instanceType: InstanceType.manual,
                bridgeKey: 'com.volcengine.rtc.hybrid_runtime',
                methodMap: {
                  r"rtsRoom$onRoomStateChanged$withUid$state$extraInfo":
                      r"rtsRoom:onRoomStateChanged:withUid:state:extraInfo:",
                  r"rtsRoom$onLeaveRoom": r"rtsRoom:onLeaveRoom:",
                  r"rtsRoom$onUserJoined": r"rtsRoom:onUserJoined:",
                  r"rtsRoom$onUserLeave$reason": r"rtsRoom:onUserLeave:reason:",
                  r"rtsRoom$onRoomMessageReceived$message":
                      r"rtsRoom:onRoomMessageReceived:message:",
                  r"rtsRoom$onRoomBinaryMessageReceived$message":
                      r"rtsRoom:onRoomBinaryMessageReceived:message:",
                  r"rtsRoom$onRoomMessageSendResult$error":
                      r"rtsRoom:onRoomMessageSendResult:error:",
                  r"rtsRoom$onUserMessageReceived$message":
                      r"rtsRoom:onUserMessageReceived:message:",
                  r"rtsRoom$onUserBinaryMessageReceived$message":
                      r"rtsRoom:onUserBinaryMessageReceived:message:",
                  r"rtsRoom$onRoomMessageReceived$uid$message":
                      r"rtsRoom:onRoomMessageReceived:uid:message:",
                  r"rtsRoom$onRoomBinaryMessageReceived$uid$message":
                      r"rtsRoom:onRoomBinaryMessageReceived:uid:message:",
                  r"rtsRoom$onUserMessageReceived$uid$message":
                      r"rtsRoom:onUserMessageReceived:uid:message:",
                  r"rtsRoom$onUserBinaryMessageReceived$uid$message":
                      r"rtsRoom:onUserBinaryMessageReceived:uid:message:",
                  r"rtsRoom$onUserMessageSendResult$error":
                      r"rtsRoom:onUserMessageSendResult:error:"
                })
            : NativeClassOptions.fromMap({
                ...options.toMap(),
                'bridgeKey': 'com.volcengine.rtc.hybrid_runtime',
              })) {
    registerEvent(r"rtsRoom:onRoomStateChanged:withUid:state:extraInfo:",
        rtsRoom$onRoomStateChanged$withUid$state$extraInfo);

    registerEvent(r"rtsRoom:onLeaveRoom:", rtsRoom$onLeaveRoom);

    registerEvent(r"rtsRoom:onUserJoined:", rtsRoom$onUserJoined);

    registerEvent(r"rtsRoom:onUserLeave:reason:", rtsRoom$onUserLeave$reason);

    registerEvent(r"rtsRoom:onRoomMessageReceived:message:",
        rtsRoom$onRoomMessageReceived$message);

    registerEvent(r"rtsRoom:onRoomBinaryMessageReceived:message:",
        rtsRoom$onRoomBinaryMessageReceived$message);

    registerEvent(r"rtsRoom:onRoomMessageSendResult:error:",
        rtsRoom$onRoomMessageSendResult$error);

    registerEvent(r"rtsRoom:onUserMessageReceived:message:",
        rtsRoom$onUserMessageReceived$message);

    registerEvent(r"rtsRoom:onUserBinaryMessageReceived:message:",
        rtsRoom$onUserBinaryMessageReceived$message);

    registerEvent(r"rtsRoom:onRoomMessageReceived:uid:message:",
        rtsRoom$onRoomMessageReceived$uid$message);

    registerEvent(r"rtsRoom:onRoomBinaryMessageReceived:uid:message:",
        rtsRoom$onRoomBinaryMessageReceived$uid$message);

    registerEvent(r"rtsRoom:onUserMessageReceived:uid:message:",
        rtsRoom$onUserMessageReceived$uid$message);

    registerEvent(r"rtsRoom:onUserBinaryMessageReceived:uid:message:",
        rtsRoom$onUserBinaryMessageReceived$uid$message);

    registerEvent(r"rtsRoom:onUserMessageSendResult:error:",
        rtsRoom$onUserMessageSendResult$error);
  }

  /// @detail callback
  /// @brief RTS 房间状态改变回调，加入房间、异常退出房间、发生房间相关的警告或错误时会收到此回调。
  /// @param rtsRoom RTS 房间实例。
  /// @param roomId 房间 ID。
  /// @param uid 用户 ID。
  /// @param state 房间状态码。  <br>
  ///              + 0: 加入房间成功。  <br>
  ///              + !0: 加入房间失败、异常退房、发生房间相关的警告或错误。
  /// @param extraInfo 额外信息，如 `{"elapsed":1187,"join_type":0}`。<br>
  ///                  - `join_type` 表示加入房间的类型，`0`为首次进房，`1`为重连进房。<br>
  ///                  - `elapsed` 表示加入房间耗时，即本地用户从调用 joinRTSRoom:userInfo:{@link #ByteRTCRTSRoom#joinRTSRoom:userInfo} 到加入 RTS 房间成功所经历的时间间隔，单位为 ms。
  ///

  FutureOr<void> rtsRoom$onRoomStateChanged$withUid$state$extraInfo(
      ByteRTCRTSRoom rtsRoom,
      NSString roomId,
      NSString uid,
      NSInteger state,
      NSString extraInfo) async {}

  /// @detail callback
  /// @brief 离开 RTS 房间成功回调。  <br>
  ///        用户调用 leaveRoom{@link #ByteRTCRTSRoom#leaveRoom} 方法后，SDK 会停止房间内消息的收发，并在释放所有相关资源后，通过此回调通知用户离开房间成功。  <br>
  /// @param rtsRoom RTSRoom 对象。  <br>
  /// @param stats 保留参数，目前为空。
  /// @note
  ///       + 用户调用 leaveRoom{@link #ByteRTCRTSRoom#leaveRoom} 方法离开房间后，如果立即调用 destroy{@link #ByteRTCRTSRoom#destroy} 销毁房间实例，则将无法收到此回调事件。  <br>
  ///       + 离开房间结束通话后，如果 App 需要使用系统音视频设备，则建议在收到此回调后再初始化音视频设备，否则可能由于 SDK 占用了导致 App 初始化音视频设备失败。  <br>

  FutureOr<void> rtsRoom$onLeaveRoom(
      ByteRTCRTSRoom rtsRoom, ByteRTCRoomStats stats) async {}

  /// @detail callback
  /// @brief 远端用户首次进房，或断网后重新连入房间时，房间内其他用户将收到该事件。
  /// @param rtsRoom ByteRTCRTSRoom 对象。  <br>
  /// @param userInfo 用户信息，参看 ByteRTCUserInfo{@link #ByteRTCUserInfo}。

  FutureOr<void> rtsRoom$onUserJoined(
      ByteRTCRTSRoom rtsRoom, ByteRTCUserInfo userInfo) async {}

  /// @detail callback
  /// @brief 远端用户离开 RTS 房间时，本地用户会收到此事件。
  /// @param rtsRoom ByteRTCRTSRoom 对象。  <br>
  /// @param uid 离开房间的远端用户 ID 。  <br>
  /// @param reason 用户离开房间的原因，参看 ByteRTCUserOfflineReason{@link #ByteRTCUserOfflineReason}。

  FutureOr<void> rtsRoom$onUserLeave$reason(ByteRTCRTSRoom rtsRoom,
      NSString uid, ByteRTCUserOfflineReason reason) async {}

  /// @detail callback
  /// @brief 收到房间中调用 sendRoomMessage:{@link #ByteRTCRTSRoom#sendRoomMessage} 发送的广播文本消息时，收到此回调。
  /// @param rtsRoom ByteRTCRTSRoom 对象
  /// @param uid 消息发送者 ID 。
  /// @param message 收到的文本消息内容。
  ///

  FutureOr<void> rtsRoom$onRoomMessageReceived$message(
      ByteRTCRTSRoom rtsRoom, NSString uid, NSString message) async {}

  /// @detail callback
  /// @brief 收到房间中调用 sendRoomBinaryMessage:{@link #ByteRTCRTSRoom#sendRoomBinaryMessage} 发送的广播二进制消息时，收到此回调。
  /// @param rtsRoom ByteRTCRTSRoom 对象
  /// @param uid 消息发送者 ID 。
  /// @param message 收到的二进制消息内容。
  ///

  FutureOr<void> rtsRoom$onRoomBinaryMessageReceived$message(
      ByteRTCRTSRoom rtsRoom, NSString uid, NSData message) async {}

  /// @detail callback
  /// @brief 向房间内所有用户群发文本或二进制消息后，消息发送方会收到该消息发送结果回调。
  /// @param rtsRoom ByteRTCRTSRoom 对象。
  /// @param msgid 本条消息的 ID。
  /// @param error 消息发送结果，详见 ByteRTCRoomMessageSendResult{@link #ByteRTCRoomMessageSendResult}
  /// @note  <br>
  ///        + 你应调用 sendRoomMessage:{@link #ByteRTCRTSRoom#sendRoomMessage} 向房间内群发文本消息 <br>
  ///        + 你应调用 sendRoomBinaryMessage:{@link #ByteRTCRTSRoom#sendRoomBinaryMessage} 向房间内群发二进制消息
  ///

  FutureOr<void> rtsRoom$onRoomMessageSendResult$error(ByteRTCRTSRoom rtsRoom,
      NSInteger msgid, ByteRTCRoomMessageSendResult error) async {}

  /// @detail callback
  /// @brief 收到来自房间中其他用户通过 sendUserMessage:message:config:{@link #ByteRTCRTSRoom#sendUserMessage:message:config} 发来的点对点文本消息时，会收到此回调。
  /// @param rtsRoom ByteRTCRoom 对象
  /// @param uid 消息发送者的用户 ID。
  /// @param message 收到的文本消息内容。
  ///

  FutureOr<void> rtsRoom$onUserMessageReceived$message(
      ByteRTCRTSRoom rtsRoom, NSString uid, NSString message) async {}

  /// @detail callback
  /// @brief 收到来自房间中其他用户通过 sendUserBinaryMessage:message:config:{@link #ByteRTCRTSRoom#sendUserBinaryMessage:message:config} 发来的点对点二进制消息时，会收到此回调。
  /// @param rtsRoom ByteRTCRoom 对象
  /// @param uid 消息发送者的用户 ID。
  /// @param message 收到的二进制消息内容。
  ///

  FutureOr<void> rtsRoom$onUserBinaryMessageReceived$message(
      ByteRTCRTSRoom rtsRoom, NSString uid, NSData message) async {}

  /// @detail callback
  /// @author hanchenchen.c
  /// @brief 收到房间中调用 sendRoomMessage:{@link #ByteRTCRTSRoom#sendRoomMessage} 发送的广播文本消息时，收到此回调。
  /// @param rtsRoom ByteRTCRoom 对象
  /// @param msgid 消息编号。
  /// @param uid 消息发送者 ID 。
  /// @param message 收到的文本消息内容。
  ///

  FutureOr<void> rtsRoom$onRoomMessageReceived$uid$message(
      ByteRTCRTSRoom rtsRoom,
      NSInteger msgid,
      NSString uid,
      NSString message) async {}

  /// @detail callback
  /// @author hanchenchen.c
  /// @brief 收到房间内广播二进制消息的回调。 <br>
  ///        房间内其他用户调用 sendRoomBinaryMessage:{@link #ByteRTCRTSRoom#sendRoomBinaryMessage} 发送的广播二进制消息时，收到此回调。
  /// @param rtsRoom ByteRTCRoom 对象
  /// @param msgid 消息编号。
  /// @param uid 消息发送者 ID 。
  /// @param message 收到的二进制消息内容。
  ///

  FutureOr<void> rtsRoom$onRoomBinaryMessageReceived$uid$message(
      ByteRTCRTSRoom rtsRoom,
      NSInteger msgid,
      NSString uid,
      NSData message) async {}

  /// @detail callback
  /// @author hanchenchen.c
  /// @brief 收到来自房间中其他用户通过 sendUserMessage:message:config:{@link #ByteRTCRTSRoom#sendUserMessage:message:config} 发来的点对点文本消息时，会收到此回调。
  /// @param rtsRoom ByteRTCRoom 对象
  /// @param msgid 消息编号。
  /// @param uid 消息发送者的用户 ID。
  /// @param message 收到的文本消息内容。
  ///

  FutureOr<void> rtsRoom$onUserMessageReceived$uid$message(
      ByteRTCRTSRoom rtsRoom,
      NSInteger msgid,
      NSString uid,
      NSString message) async {}

  /// @detail callback
  /// @author hanchenchen.c
  /// @brief 收到来自房间中其他用户通过 sendUserBinaryMessage:message:config:{@link #ByteRTCRTSRoom#sendUserBinaryMessage:message:config} 发来的点对点二进制消息时，会收到此回调。
  /// @param rtsRoom ByteRTCRoom 对象
  /// @param msgid 消息编号.
  /// @param uid 消息发送者的用户 ID。
  /// @param message 收到的二进制消息内容。
  ///

  FutureOr<void> rtsRoom$onUserBinaryMessageReceived$uid$message(
      ByteRTCRTSRoom rtsRoom,
      NSInteger msgid,
      NSString uid,
      NSData message) async {}

  /// @detail callback
  /// @brief 向房间内单个用户发送文本或二进制消息后（P2P），消息发送方会收到该消息发送结果回调。
  /// @param rtsRoom ByteRTCRoom 对象。
  /// @param msgid 本条消息的 ID。
  /// @param error 消息发送结果，详见 ByteRTCUserMessageSendResult{@link #ByteRTCUserMessageSendResult}
  /// @note
  ///        - 你应调用 sendUserMessage:message:config:{@link #ByteRTCRTSRoom#sendUserMessage:message:config} 向房间内单个用户发送文本消息
  ///        - 你应调用 sendUserBinaryMessage:message:config:{@link #ByteRTCRTSRoom#sendUserBinaryMessage:message:config} 向房间内单个用户发送二进制消息
  ///

  FutureOr<void> rtsRoom$onUserMessageSendResult$error(ByteRTCRTSRoom rtsRoom,
      NSInteger msgid, ByteRTCUserMessageSendResult error) async {}
}

class ByteRTCVideoSinkProtocol extends NativeObserverClass {
  static const _$namespace = r'ByteRTCVideoSinkProtocol';

  ByteRTCVideoSinkProtocol([NativeClassOptions? options])
      : super(options == null
            ? const NativeClassOptions([],
                className: _$namespace,
                instanceType: InstanceType.manual,
                bridgeKey: 'com.volcengine.rtc.hybrid_runtime',
                methodMap: {
                  r"shouldStart": r"shouldStart",
                  r"shouldStop": r"shouldStop",
                  r"shouldDispose": r"shouldDispose",
                  r"renderPixelBuffer$rotation$cameraId$extendedData":
                      r"renderPixelBuffer:rotation:cameraId:extendedData:"
                })
            : NativeClassOptions.fromMap({
                ...options.toMap(),
                'bridgeKey': 'com.volcengine.rtc.hybrid_runtime',
              })) {
    registerEvent(r"shouldStart", shouldStart);

    registerEvent(r"shouldStop", shouldStop);

    registerEvent(r"shouldDispose", shouldDispose);

    registerEvent(r"renderPixelBuffer:rotation:cameraId:extendedData:",
        renderPixelBuffer$rotation$cameraId$extendedData);
  }

  /// @detail callback
  /// @brief 启动渲染器
  /// @note 在开启渲染功能的时候会回调这个方法

  FutureOr<void> shouldStart() async {}

  /// @detail callback
  /// @brief 停止渲染器
  /// @note 在停止渲染功能的时候会回调这个方法

  FutureOr<void> shouldStop() async {}

  /// @detail callback
  /// @brief 释放渲染器
  /// @note 渲染器即将被废弃的时候会回调这个方法

  FutureOr<void> shouldDispose() async {}

  /// @detail callback
  /// @brief 输出视频的 PixelBuffer
  /// @param pixelBuffer 视频的 PixelBuffer
  /// @param rotation 视频旋转角度，{@link #ByteRTCVideoRotation}
  /// @param cameraId 视频的相机 Id, {@link #ByteRTCCameraID}
  /// @param extendedData 视频帧附加的数据,视频解码后获得的附加数据
  /// @note 通过该方法获取视频的 PixelBuffer

  FutureOr<void> renderPixelBuffer$rotation$cameraId$extendedData(
      CVPixelBufferRef pixelBuffer,
      ByteRTCVideoRotation rotation,
      ByteRTCCameraID cameraId,
      NSData extendedData) async {}
}

class ByteRTCClientMixedStreamDelegate extends NativeObserverClass {
  static const _$namespace = r'ByteRTCClientMixedStreamDelegate';

  ByteRTCClientMixedStreamDelegate([NativeClassOptions? options])
      : super(options == null
            ? const NativeClassOptions([],
                className: _$namespace,
                instanceType: InstanceType.manual,
                bridgeKey: 'com.volcengine.rtc.hybrid_runtime',
                methodMap: {
                  r"onClientMixedStreamEvent$withTaskInfo$withMixedType$withErrorCode":
                      r"onClientMixedStreamEvent:withTaskInfo:withMixedType:withErrorCode:",
                  r"onMixedFirstAudioFrame": r"onMixedFirstAudioFrame:",
                  r"onMixedFirstVideoFrame": r"onMixedFirstVideoFrame:",
                  r"onMixedAudioFrame$withTimestamp$withTaskId":
                      r"onMixedAudioFrame:withTimestamp:withTaskId:",
                  r"onMixedVideoFrame$withTaskId":
                      r"onMixedVideoFrame:withTaskId:",
                  r"onMixedDataFrame$withTaskId":
                      r"onMixedDataFrame:withTaskId:"
                })
            : NativeClassOptions.fromMap({
                ...options.toMap(),
                'bridgeKey': 'com.volcengine.rtc.hybrid_runtime',
              })) {
    registerEvent(
        r"onClientMixedStreamEvent:withTaskInfo:withMixedType:withErrorCode:",
        onClientMixedStreamEvent$withTaskInfo$withMixedType$withErrorCode);

    registerEvent(r"onMixedFirstAudioFrame:", onMixedFirstAudioFrame);

    registerEvent(r"onMixedFirstVideoFrame:", onMixedFirstVideoFrame);

    registerEvent(r"onMixedAudioFrame:withTimestamp:withTaskId:",
        onMixedAudioFrame$withTimestamp$withTaskId);

    registerEvent(
        r"onMixedVideoFrame:withTaskId:", onMixedVideoFrame$withTaskId);

    registerEvent(r"onMixedDataFrame:withTaskId:", onMixedDataFrame$withTaskId);
  }

  /// @hidden for internal use only

  FutureOr<void>
      onClientMixedStreamEvent$withTaskInfo$withMixedType$withErrorCode(
          ByteRTCMixedStreamTaskEvent event,
          ByteRTCMixedStreamTaskInfo info,
          ByteRTCMixedStreamType type,
          ByteRTCMixedStreamTaskErrorCode errorCode) async {}

  /// @hidden for internal use only
  /// @detail callback
  /// @author liujingchao
  /// @brief 客户端合流音频首帧回调
  /// @param taskID 任务 ID

  FutureOr<void> onMixedFirstAudioFrame(NSString taskId) async {}

  /// @hidden for internal use only
  /// @detail callback
  /// @author liujingchao
  /// @brief 客户端合流视频首帧回调
  /// @param taskID 任务 ID

  FutureOr<void> onMixedFirstVideoFrame(NSString taskId) async {}

  /// @hidden for internal use only
  /// @detail callback
  /// @author liujingchao
  /// @brief 端云一体合流音频 PCM 回调
  /// @param audioFrame PCM 编码的合流音频数据帧，参看 ByteRTCAudioFrame{@link #ByteRTCAudioFrame}。
  /// @param timeStamp 时间戳，单位毫秒。
  /// @param taskId 转推直播任务 ID。
  /// @note 收到该回调的周期为每 10 毫秒一次，并且每次的音频数据量为 10 毫秒数据量。

  FutureOr<void> onMixedAudioFrame$withTimestamp$withTaskId(
      ByteRTCAudioFrame audioFrame, int64_t timeStamp, NSString taskId) async {}

  /// @hidden for internal use only
  /// @detail callback
  /// @author liujingchao
  /// @brief 端云一体合流视频 YUV 回调
  /// @param videoFrame YUV 合流视频数据帧，参看 ByteRTCVideoFrame{@link #ByteRTCVideoFrame}。
  /// @param taskId 转推直播任务 ID。
  /// @note 收到该回调的周期取决于视频的帧率。

  FutureOr<void> onMixedVideoFrame$withTaskId(
      id<ByteRTCVideoFrame> videoFrame, NSString taskId) async {}

  /// @hidden for internal use only
  /// @detail callback
  /// @author liujingchao
  /// @brief 端云一体合流视频 SEI 数据
  /// @param dataFrame SEI 数据，详见 ByteRTCFrameExtendedData {@link #ByteRTCFrameExtendedData}。
  /// @param taskId 转推直播任务 ID。

  FutureOr<void> onMixedDataFrame$withTaskId(
      ByteRTCFrameExtendedData dataFrame, NSString taskId) async {}
}

class ByteRtcScreenCapturerExtDelegate extends NativeObserverClass {
  static const _$namespace = r'ByteRtcScreenCapturerExtDelegate';

  ByteRtcScreenCapturerExtDelegate([NativeClassOptions? options])
      : super(options == null
            ? const NativeClassOptions([],
                className: _$namespace,
                instanceType: InstanceType.manual,
                bridgeKey: 'com.volcengine.rtc.hybrid_runtime',
                methodMap: {
                  r"onQuitFromApp": r"onQuitFromApp",
                  r"onReceiveMessageFromApp": r"onReceiveMessageFromApp:",
                  r"onSocketDisconnect": r"onSocketDisconnect",
                  r"onSocketConnect": r"onSocketConnect",
                  r"onNotifyAppRunning": r"onNotifyAppRunning"
                })
            : NativeClassOptions.fromMap({
                ...options.toMap(),
                'bridgeKey': 'com.volcengine.rtc.hybrid_runtime',
              })) {
    registerEvent(r"onQuitFromApp", onQuitFromApp);

    registerEvent(r"onReceiveMessageFromApp:", onReceiveMessageFromApp);

    registerEvent(r"onSocketDisconnect", onSocketDisconnect);

    registerEvent(r"onSocketConnect", onSocketConnect);

    registerEvent(r"onNotifyAppRunning", onNotifyAppRunning);
  }

  /// @detail api
  /// @brief 通知 Broadcast Upload Extension 停止采集屏幕并退出。
  /// @note iOS 端调用 stopScreenCapture{@link #ByteRTCEngine#stopScreenCapture}，或 macOS 端调用 stopScreenVideoCapture{@link #ByteRTCEngine#stopScreenVideoCapture}，会触发该方法通知 extension 端的 SDK 停止屏幕采集。

  FutureOr<void> onQuitFromApp() async {}

  /// @detail api
  /// @brief Socket 收到 App 侧发来的信息时，触发该回调
  /// @param message App 侧发送的消息

  FutureOr<void> onReceiveMessageFromApp(NSData message) async {}

  /// @detail api
  /// @brief Socket 连接断开时触发此回调

  FutureOr<void> onSocketDisconnect() async {}

  /// @detail api
  /// @brief Socket 连接成功时触发此回调

  FutureOr<void> onSocketConnect() async {}

  /// @detail api
  /// @brief 检测到 App 正在进行音视频通话时触发此回调。

  FutureOr<void> onNotifyAppRunning() async {}
}

class ByteRTCVideoSnapshotCallbackDelegate extends NativeObserverClass {
  static const _$namespace = r'ByteRTCVideoSnapshotCallbackDelegate';

  ByteRTCVideoSnapshotCallbackDelegate([NativeClassOptions? options])
      : super(options == null
            ? const NativeClassOptions([],
                className: _$namespace,
                instanceType: InstanceType.manual,
                bridgeKey: 'com.volcengine.rtc.hybrid_runtime',
                methodMap: {
                  r"onTakeLocalSnapshotResult$videoSource$image$errorCode":
                      r"onTakeLocalSnapshotResult:videoSource:image:errorCode:",
                  r"onTakeRemoteSnapshotResult$streamId$info$image$errorCode":
                      r"onTakeRemoteSnapshotResult:streamId:info:image:errorCode:"
                })
            : NativeClassOptions.fromMap({
                ...options.toMap(),
                'bridgeKey': 'com.volcengine.rtc.hybrid_runtime',
              })) {
    registerEvent(r"onTakeLocalSnapshotResult:videoSource:image:errorCode:",
        onTakeLocalSnapshotResult$videoSource$image$errorCode);

    registerEvent(r"onTakeRemoteSnapshotResult:streamId:info:image:errorCode:",
        onTakeRemoteSnapshotResult$streamId$info$image$errorCode);
  }

  /// @detail callback
  /// @brief 调用 takeLocalSnapshot:{@link #ByteRTCEngine#takeLocalSnapshot} 截取视频画面时，收到此回调。
  /// @param taskId 本地截图任务的编号。和 takeLocalSnapshot:{@link #ByteRTCEngine#takeLocalSnapshot} 的返回值一致。
  /// @param videoSource 截图的视频流的属性，参看 ByteRTCStreamIndex{@link #ByteRTCStreamIndex}。
  /// @param image 截图。你可以保存为文件，或对其进行二次处理。截图失败时，为空。
  /// @param errorCode 截图错误码： <br>
  ///        - 0: 成功
  ///        - -1: 截图错误。生成图片数据失败或 RGBA 编码失败
  ///        - -2: 截图错误。流无效。
  ///        - -3: 截图错误。截图超时,超时时间 1 秒。

  FutureOr<void> onTakeLocalSnapshotResult$videoSource$image$errorCode(
      NSInteger taskId,
      ByteRTCVideoSource videoSource,
      ByteRTCImage image,
      NSInteger errorCode) async {}

  /// @detail callback
  /// @brief 调用 takeRemoteSnapshot:callback:{@link #ByteRTCEngine#takeRemoteSnapshot:callback} 截取视频画面时，收到此回调。
  /// @param taskId 远端截图任务的编号。和 takeRemoteSnapshot:callback:{@link #ByteRTCEngine#takeRemoteSnapshot:callback} 的返回值一致。
  /// @param streamId 远端视频流的 ID。
  /// @param info 远端视频流的属性，参看 ByteRTCStreamInfo{@link #ByteRTCStreamInfo}。
  /// @param image 截图。你可以保存为文件，或对其进行二次处理。截图失败时，为空。
  /// @param errorCode 截图错误码： <br>
  ///        - 0: 成功
  ///        - -1: 截图错误。生成图片数据失败或 RGBA 编码失败
  ///        - -2: 截图错误。流无效。
  ///        - -3: 截图错误。截图超时,超时时间 1 秒。

  FutureOr<void> onTakeRemoteSnapshotResult$streamId$info$image$errorCode(
      NSInteger taskId,
      NSString streamId,
      ByteRTCStreamInfo info,
      ByteRTCImage image,
      NSInteger errorCode) async {}
}

class ByteRTCLocalEncodedVideoFrameObserver extends NativeObserverClass {
  static const _$namespace = r'ByteRTCLocalEncodedVideoFrameObserver';

  ByteRTCLocalEncodedVideoFrameObserver([NativeClassOptions? options])
      : super(options == null
            ? const NativeClassOptions([],
                className: _$namespace,
                instanceType: InstanceType.manual,
                bridgeKey: 'com.volcengine.rtc.hybrid_runtime',
                methodMap: {
                  r"onLocalEncodedVideoFrame$Frame":
                      r"onLocalEncodedVideoFrame:Frame:"
                })
            : NativeClassOptions.fromMap({
                ...options.toMap(),
                'bridgeKey': 'com.volcengine.rtc.hybrid_runtime',
              })) {
    registerEvent(
        r"onLocalEncodedVideoFrame:Frame:", onLocalEncodedVideoFrame$Frame);
  }

  /// @detail callback
  /// @brief 调用 registerLocalEncodedVideoFrameObserver:{@link #ByteRTCEngine#registerLocalEncodedVideoFrameObserver} 后，SDK 每次使用内部采集，采集到一帧视频帧，或收到一帧外部视频帧时，都会回调该事件。
  /// @param videoSource 本地视频源，参看 ByteRTCVideoSource{@link #ByteRTCVideoSource}
  /// @param frame 本地视频帧信息，参看 ByteRTCEncodedVideoFrame{@link #ByteRTCEncodedVideoFrame}

  FutureOr<void> onLocalEncodedVideoFrame$Frame(
      dynamic videoSource, ByteRTCEncodedVideoFrame frame) async {}
}

class ByteRTCWTNStreamDelegate extends NativeObserverClass {
  static const _$namespace = r'ByteRTCWTNStreamDelegate';

  ByteRTCWTNStreamDelegate([NativeClassOptions? options])
      : super(options == null
            ? const NativeClassOptions([],
                className: _$namespace,
                instanceType: InstanceType.manual,
                bridgeKey: 'com.volcengine.rtc.hybrid_runtime',
                methodMap: {
                  r"onWTNRemoteVideoStats$videoStats":
                      r"onWTNRemoteVideoStats:videoStats:",
                  r"onWTNRemoteAudioStats$audioStats":
                      r"onWTNRemoteAudioStats:audioStats:",
                  r"onWTNVideoSubscribeStateChanged$state$reason":
                      r"onWTNVideoSubscribeStateChanged:state:reason:",
                  r"onWTNAudioSubscribeStateChanged$state$reason":
                      r"onWTNAudioSubscribeStateChanged:state:reason:",
                  r"onWTNFirstRemoteVideoFrameDecoded$withFrameInfo":
                      r"onWTNFirstRemoteVideoFrameDecoded:withFrameInfo:",
                  r"onWTNFirstRemoteAudioFrame": r"onWTNFirstRemoteAudioFrame:",
                  r"onWTNSEIMessageReceived$andChannelId$andMessage":
                      r"onWTNSEIMessageReceived:andChannelId:andMessage:",
                  r"onWTNDataMessageReceived$andMessage$andSourceType":
                      r"onWTNDataMessageReceived:andMessage:andSourceType:"
                })
            : NativeClassOptions.fromMap({
                ...options.toMap(),
                'bridgeKey': 'com.volcengine.rtc.hybrid_runtime',
              })) {
    registerEvent(
        r"onWTNRemoteVideoStats:videoStats:", onWTNRemoteVideoStats$videoStats);

    registerEvent(
        r"onWTNRemoteAudioStats:audioStats:", onWTNRemoteAudioStats$audioStats);

    registerEvent(r"onWTNVideoSubscribeStateChanged:state:reason:",
        onWTNVideoSubscribeStateChanged$state$reason);

    registerEvent(r"onWTNAudioSubscribeStateChanged:state:reason:",
        onWTNAudioSubscribeStateChanged$state$reason);

    registerEvent(r"onWTNFirstRemoteVideoFrameDecoded:withFrameInfo:",
        onWTNFirstRemoteVideoFrameDecoded$withFrameInfo);

    registerEvent(r"onWTNFirstRemoteAudioFrame:", onWTNFirstRemoteAudioFrame);

    registerEvent(r"onWTNSEIMessageReceived:andChannelId:andMessage:",
        onWTNSEIMessageReceived$andChannelId$andMessage);

    registerEvent(r"onWTNDataMessageReceived:andMessage:andSourceType:",
        onWTNDataMessageReceived$andMessage$andSourceType);
  }

  /// @detail callback
  /// @valid since 3.60.
  /// @author hanchenchen
  /// @brief 通话中本地设备接收订阅的远端 WTN 视频流的流 ID 以及远端 WTN 视频流统计信息。
  /// @param streamId WTN 流 ID
  /// @param videoStats 远端 WTN 视频流的统计信息，详见 ByteRTCRemoteVideoStats{@link #ByteRTCRemoteVideoStats}。
  /// @order 0

  FutureOr<void> onWTNRemoteVideoStats$videoStats(
      NSString streamId, ByteRTCRemoteVideoStats videoStats) async {}

  /// @detail callback
  /// @valid since 3.60.
  /// @author hanchenchen
  /// @brief 通话中本地设备接收订阅的远端 WTN 音频流的流 ID 以及远端 WTN 音频流统计信息。
  /// @param streamId WTN 流 ID
  /// @param audioStats 远端 WTN 音频流的统计信息，详见 ByteRTCRemoteAudioStats{@link #ByteRTCRemoteAudioStats}。
  /// @order 1

  FutureOr<void> onWTNRemoteAudioStats$audioStats(
      NSString streamId, ByteRTCRemoteAudioStats audioStats) async {}

  /// @detail callback
  /// @valid since 3.60. 自 3.60 起，该回调替代了 `rtcEngine:onPlayPublicStreamResult:errorCode:` 方法中的 WTN 视频流订阅状态变化通知功能。如果你已升级至 3.60 及以上版本，并且仍在使用该方法，请迁移到此回调。
  /// @author hanchenchen
  /// @brief WTN 视频流订阅状态变化回调 <br>
  ///        通过 subscribeWTNVideoStream:subscribe:{@link #ByteRTCWTNStream#subscribeWTNVideoStream:subscribe} 订阅 WTN 视频流后，可以通过本回调获取订阅结果。
  /// @param streamId WTN 视频流的 ID
  /// @param state 视频流状态码，参看 ByteRTCWTNSubscribeState{@link #ByteRTCWTNSubscribeState}。
  /// @param reason 订阅状态发生变化的原因，参看 ByteRTCWTNSubscribeStateChangeReason{@link #ByteRTCWTNSubscribeStateChangeReason}。
  /// @order 2

  FutureOr<void> onWTNVideoSubscribeStateChanged$state$reason(
      NSString streamId,
      ByteRTCWTNSubscribeState state,
      ByteRTCWTNSubscribeStateChangeReason reason) async {}

  /// @detail callback
  /// @valid since 3.60. 自 3.60 起，该回调替代了 `rtcEngine:onPlayPublicStreamResult:errorCode:` 方法中的 WTN 音频流订阅状态变化通知功能。如果你已升级至 3.60 及以上版本，并且仍在使用该方法，请迁移到此回调。
  /// @author hanchenchen
  /// @brief WTN 音频流订阅状态变化回调 <br>
  ///        通过 subscribeWTNAudioStream:subscribe:{@link #ByteRTCWTNStream#subscribeWTNAudioStream:subscribe} 订阅 WTN 音频流后，可以通过本回调获取订阅结果。
  /// @param streamId WTN 音频流的 ID
  /// @param state 音频流状态码，参看 ByteRTCWTNSubscribeState{@link #ByteRTCWTNSubscribeState}。
  /// @param reason 订阅状态发生变化的原因，参看 ByteRTCWTNSubscribeStateChangeReason{@link #ByteRTCWTNSubscribeStateChangeReason}。
  /// @order 2

  FutureOr<void> onWTNAudioSubscribeStateChanged$state$reason(
      NSString streamId,
      ByteRTCWTNSubscribeState state,
      ByteRTCWTNSubscribeStateChangeReason reason) async {}

  /// @detail callback
  /// @valid since 3.60. 自 3.60 起，该回调替代了 `onWTNFirstRemoteVideoFrameDecoded:withFrameInfo:`。如果你已升级至 3.60 及以上版本，并且仍在使用该方法，请迁移到此回调。
  /// @author hanchenchen
  /// @brief WTN 流的首帧视频解码成功 <br>
  ///        关于 订阅 WTN 流，详见 subscribeWTNVideoStream:subscribe:{@link #ByteRTCWTNStream#subscribeWTNVideoStream:subscribe}。
  /// @param streamId WTN 流 ID
  /// @param frameInfo 视频帧信息，参看 ByteRTCVideoFrameInfo{@link #ByteRTCVideoFrameInfo}
  /// @order 4

  FutureOr<void> onWTNFirstRemoteVideoFrameDecoded$withFrameInfo(
      NSString streamId, ByteRTCVideoFrameInfo frameInfo) async {}

  /// @detail callback
  /// @valid since 3.60. 自 3.60 起，该回调替代了 `rtcEngine:onFirstPublicStreamAudioFrame:`。如果你已升级至 3.60 及以上版本，并且仍在使用该方法，请迁移到此回调。
  /// @author hanchenchen
  /// @brief WTN 流的首帧音频解码成功 <br>
  ///        关于订阅 WTN 音频流，详见 subscribeWTNAudioStream:subscribe:{@link #ByteRTCWTNStream#subscribeWTNAudioStream:subscribe}。
  /// @param streamId WTN 流 ID
  /// @order 3

  FutureOr<void> onWTNFirstRemoteAudioFrame(NSString streamId) async {}

  /// @detail callback
  /// @valid since 3.60. 自 3.60 起，该回调替换了 `rtcEngine:onPublicStreamSEIMessageReceived:andMessage:andSourceType:` 来实现下述功能。如果你已升级至 3.60 及以上版本，并且仍在使用该方法，请迁移到此回调。
  /// @author hanchenchen
  /// @brief 回调 WTN 流中包含的 SEI 信息。 <br>
  ///        subscribeWTNAudioStream:subscribe:{@link #ByteRTCWTNStream#subscribeWTNAudioStream:subscribe}/subscribeWTNVideoStream:subscribe:{@link #ByteRTCWTNStream#subscribeWTNVideoStream:subscribe} 接口启动拉 WTN 音频/视频流功能后，通过此回调收到 WTN 流中的 SEI 消息。
  /// @param streamId WTN 流 ID。
  /// @param channelId SEI 消息通道 ID，取值范围 [0 - 255]。通过此参数，你可以为不同接受方设置不同的 ChannelID，这样不同接收方可以根据回调中的 ChannelID 选择应关注的 SEI 信息。
  /// @param message 收到的 SEI 消息内容。 <br>
  ///                通过调用客户端 `sendPublicStreamSEIMessage` 插入的 SEI 信息。
  ///                当 WTN 流中的多路视频流均包含有 SEI 信息：SEI 不互相冲突时，将通过多次回调分别发送；SEI 在同一帧有冲突时，则只有一条流中的 SEI 信息被透传并融合到 WTN 流中。
  /// @note 通过 Open API 插入的自定义信息，应通过回调 onWTNDataMessageReceived:andMessage:andSourceType:{@link #ByteRTCWTNStreamDelegate#onWTNDataMessageReceived:andMessage:andSourceType} 获取。
  /// @order 5

  FutureOr<void> onWTNSEIMessageReceived$andChannelId$andMessage(
      NSString streamId, int channelId, NSData message) async {}

  /// @detail callback
  /// @valid since 3.60.
  /// @author hanchenchen
  /// @brief 回调 WTN 流中包含的数据信息。 <br>
  ///        通过 subscribeWTNAudioStream:subscribe:{@link #ByteRTCWTNStream#subscribeWTNAudioStream:subscribe}/subscribeWTNVideoStream:subscribe:{@link #ByteRTCWTNStream#subscribeWTNVideoStream:subscribe} 订阅 WTN 流后，可以通过本回调获取 WTN 流中的数据消息，包括调用 Open API 发送的 SEI 消息和音量回调。
  /// @param streamId WTN 流 ID
  /// @param message 收到的数据消息内容，如下： <br>
  /// - 调用 WTN 流 OpenAPI 发送的自定义消息。
  /// - 媒体流音量变化，需要通过 WTN 流 OpenAPI 开启回调。JSON 格式说明如下：
  /// JSON 格式说明如下：<br/>
  /// {<br/>
  /// "Type" : "VolumeIndication", //具体业务类型<br/>
  /// "VolumeInfos" : [ // 业务类型对应信息<br/>
  /// {<br/>
  /// "RoomId":"1000001", // 房间 ID<br/>
  /// "UserId":"1000001", // 用户 ID<br/>
  /// "StreamType":0, // 0:摄像头流；1:屏幕流<br/>
  /// "LinearVolume":1 // 线性音量大小<br/>
  /// }<br/>
  /// @param sourceType 数据消息来源，参看 ByteRTCDataMessageSourceType{@link #ByteRTCDataMessageSourceType}。
  /// @note 通过调用客户端 API 插入的 SEI 信息，应通过回调 onWTNSEIMessageReceived:andChannelId:andMessage:{@link #ByteRTCWTNStreamDelegate#onWTNSEIMessageReceived:andChannelId:andMessage} 获取。

  FutureOr<void> onWTNDataMessageReceived$andMessage$andSourceType(
      NSString streamId,
      NSData message,
      ByteRTCDataMessageSourceType sourceType) async {}
}

class ByteRTCEncryptHandler extends NativeObserverClass {
  static const _$namespace = r'ByteRTCEncryptHandler';

  ByteRTCEncryptHandler([NativeClassOptions? options])
      : super(options == null
            ? const NativeClassOptions([],
                className: _$namespace,
                instanceType: InstanceType.manual,
                bridgeKey: 'com.volcengine.rtc.hybrid_runtime',
                methodMap: {})
            : NativeClassOptions.fromMap({
                ...options.toMap(),
                'bridgeKey': 'com.volcengine.rtc.hybrid_runtime',
              })) {}
}

class ByteRTCAudioEffectPlayerEventHandler extends NativeObserverClass {
  static const _$namespace = r'ByteRTCAudioEffectPlayerEventHandler';

  ByteRTCAudioEffectPlayerEventHandler([NativeClassOptions? options])
      : super(options == null
            ? const NativeClassOptions([],
                className: _$namespace,
                instanceType: InstanceType.manual,
                bridgeKey: 'com.volcengine.rtc.hybrid_runtime',
                methodMap: {
                  r"onAudioEffectPlayerStateChanged$state$error":
                      r"onAudioEffectPlayerStateChanged:state:error:"
                })
            : NativeClassOptions.fromMap({
                ...options.toMap(),
                'bridgeKey': 'com.volcengine.rtc.hybrid_runtime',
              })) {
    registerEvent(r"onAudioEffectPlayerStateChanged:state:error:",
        onAudioEffectPlayerStateChanged$state$error);
  }

  /// @detail callback
  /// @brief 播放状态改变时回调。
  /// @param effectId ByteRTCAudioEffectPlayer{@link #ByteRTCAudioEffectPlayer} 的 ID。通过 getAudioEffectPlayer{@link #ByteRTCEngine#getAudioEffectPlayer} 设置。
  /// @param state 混音状态。参考 ByteRTCPlayerState{@link #ByteRTCPlayerState}。
  /// @param error 错误码。参考 ByteRTCPlayerError{@link #ByteRTCPlayerError}。

  FutureOr<void> onAudioEffectPlayerStateChanged$state$error(
      int effectId, ByteRTCPlayerState state, ByteRTCPlayerError error) async {}
}

class ByteRTCMediaPlayerCustomSourceProvider extends NativeObserverClass {
  static const _$namespace = r'ByteRTCMediaPlayerCustomSourceProvider';

  ByteRTCMediaPlayerCustomSourceProvider([NativeClassOptions? options])
      : super(options == null
            ? const NativeClassOptions([],
                className: _$namespace,
                instanceType: InstanceType.manual,
                bridgeKey: 'com.volcengine.rtc.hybrid_runtime',
                methodMap: {})
            : NativeClassOptions.fromMap({
                ...options.toMap(),
                'bridgeKey': 'com.volcengine.rtc.hybrid_runtime',
              })) {}
}

class ByteRTCRoomDelegate extends NativeObserverClass {
  static const _$namespace = r'ByteRTCRoomDelegate';

  ByteRTCRoomDelegate([NativeClassOptions? options])
      : super(options == null
            ? const NativeClassOptions([],
                className: _$namespace,
                instanceType: InstanceType.manual,
                bridgeKey: 'com.volcengine.rtc.hybrid_runtime',
                methodMap: {
                  r"rtcRoom$onRoomStateChangedWithReason$withUid$state$reason":
                      r"rtcRoom:onRoomStateChangedWithReason:withUid:state:reason:",
                  r"rtcRoom$onRoomStateChanged$withUid$state$extraInfo":
                      r"rtcRoom:onRoomStateChanged:withUid:state:extraInfo:",
                  r"rtcRoom$onLeaveRoom": r"rtcRoom:onLeaveRoom:",
                  r"rtcRoom$onAVSyncStateChange":
                      r"rtcRoom:onAVSyncStateChange:",
                  r"rtcRoom$onVideoPublishStateChanged$info$state$reason":
                      r"rtcRoom:onVideoPublishStateChanged:info:state:reason:",
                  r"rtcRoom$onAudioPublishStateChanged$info$state$reason":
                      r"rtcRoom:onAudioPublishStateChanged:info:state:reason:",
                  r"rtcRoom$onVideoSubscribeStateChanged$info$state$reason":
                      r"rtcRoom:onVideoSubscribeStateChanged:info:state:reason:",
                  r"rtcRoom$onAudioSubscribeStateChanged$info$state$reason":
                      r"rtcRoom:onAudioSubscribeStateChanged:info:state:reason:",
                  r"rtcRoom$onRoomStats": r"rtcRoom:onRoomStats:",
                  r"rtcRoom$onRoomEvent$uid$state$info":
                      r"rtcRoom:onRoomEvent:uid:state:info:",
                  r"rtcRoom$onLocalStreamStats$info$stats":
                      r"rtcRoom:onLocalStreamStats:info:stats:",
                  r"rtcRoom$onRemoteStreamStats$info$stats":
                      r"rtcRoom:onRemoteStreamStats:info:stats:",
                  r"rtcRoom$onUserJoined": r"rtcRoom:onUserJoined:",
                  r"rtcRoom$onUserLeave$reason": r"rtcRoom:onUserLeave:reason:",
                  r"onTokenWillExpire": r"onTokenWillExpire:",
                  r"onPublishPrivilegeTokenWillExpire":
                      r"onPublishPrivilegeTokenWillExpire:",
                  r"onSubscribePrivilegeTokenWillExpire":
                      r"onSubscribePrivilegeTokenWillExpire:",
                  r"rtcRoom$onStreamPublishSuccess$isScreen":
                      r"rtcRoom:onStreamPublishSuccess:isScreen:",
                  r"rtcRoom$onAVSyncEvent$userId$eventCode":
                      r"rtcRoom:onAVSyncEvent:userId:eventCode:",
                  r"rtcRoom$onUserPublishStreamVideo$info$isPublish":
                      r"rtcRoom:onUserPublishStreamVideo:info:isPublish:",
                  r"rtcRoom$onUserPublishStreamAudio$info$isPublish":
                      r"rtcRoom:onUserPublishStreamAudio:info:isPublish:",
                  r"rtcRoom$onRoomMessageReceived$message":
                      r"rtcRoom:onRoomMessageReceived:message:",
                  r"rtcRoom$onRoomBinaryMessageReceived$message":
                      r"rtcRoom:onRoomBinaryMessageReceived:message:",
                  r"rtcRoom$onUserMessageReceived$message":
                      r"rtcRoom:onUserMessageReceived:message:",
                  r"rtcRoom$onUserBinaryMessageReceived$message":
                      r"rtcRoom:onUserBinaryMessageReceived:message:",
                  r"rtcRoom$onRoomMessageReceived$uid$message":
                      r"rtcRoom:onRoomMessageReceived:uid:message:",
                  r"rtcRoom$onRoomBinaryMessageReceived$uid$message":
                      r"rtcRoom:onRoomBinaryMessageReceived:uid:message:",
                  r"rtcRoom$onUserMessageReceived$uid$message":
                      r"rtcRoom:onUserMessageReceived:uid:message:",
                  r"rtcRoom$onUserBinaryMessageReceived$uid$message":
                      r"rtcRoom:onUserBinaryMessageReceived:uid:message:",
                  r"rtcRoom$onUserMessageSendResult$error":
                      r"rtcRoom:onUserMessageSendResult:error:",
                  r"rtcRoom$onRoomMessageSendResult$error":
                      r"rtcRoom:onRoomMessageSendResult:error:",
                  r"rtcRoom$onSetRoomExtraInfoResult$result":
                      r"rtcRoom:onSetRoomExtraInfoResult:result:",
                  r"rtcRoom$onRoomExtraInfoUpdate$value$lastUpdateUserId$lastUpdateTimeMs":
                      r"rtcRoom:onRoomExtraInfoUpdate:value:lastUpdateUserId:lastUpdateTimeMs:",
                  r"rtcRoom$onRoomStreamExtraInfoUpdate$info$extraInfo":
                      r"rtcRoom:onRoomStreamExtraInfoUpdate:info:extraInfo:",
                  r"rtcRoom$onUserVisibilityChanged$errorCode":
                      r"rtcRoom:onUserVisibilityChanged:errorCode:",
                  r"rtcRoom$onVideoStreamBanned$isBanned":
                      r"rtcRoom:onVideoStreamBanned:isBanned:",
                  r"rtcRoom$onAudioStreamBanned$isBanned":
                      r"rtcRoom:onAudioStreamBanned:isBanned:",
                  r"rtcRoom$onForwardStreamStateChanged":
                      r"rtcRoom:onForwardStreamStateChanged:",
                  r"rtcRoom$onForwardStreamEvent":
                      r"rtcRoom:onForwardStreamEvent:",
                  r"rtcRoom$onNetworkQuality$remoteQualities":
                      r"rtcRoom:onNetworkQuality:remoteQualities:",
                  r"rtcRoom$onSubtitleStateChanged$errorCode$errorMessage":
                      r"rtcRoom:onSubtitleStateChanged:errorCode:errorMessage:",
                  r"rtcRoom$onSubtitleMessageReceived":
                      r"rtcRoom:onSubtitleMessageReceived:",
                  r"rtcRoom$onStreamStateChanged$withUid$state$extraInfo":
                      r"rtcRoom:onStreamStateChanged:withUid:state:extraInfo:",
                  r"rtcRoom$onRoomWarning": r"rtcRoom:onRoomWarning:",
                  r"rtcRoom$onStreamAdd": r"rtcRoom:onStreamAdd:",
                  r"rtcRoom$onStreamRemove$stream$reason":
                      r"rtcRoom:onStreamRemove:stream:reason:"
                })
            : NativeClassOptions.fromMap({
                ...options.toMap(),
                'bridgeKey': 'com.volcengine.rtc.hybrid_runtime',
              })) {
    registerEvent(r"rtcRoom:onRoomStateChangedWithReason:withUid:state:reason:",
        rtcRoom$onRoomStateChangedWithReason$withUid$state$reason);

    registerEvent(r"rtcRoom:onRoomStateChanged:withUid:state:extraInfo:",
        rtcRoom$onRoomStateChanged$withUid$state$extraInfo);

    registerEvent(r"rtcRoom:onLeaveRoom:", rtcRoom$onLeaveRoom);

    registerEvent(r"rtcRoom:onAVSyncStateChange:", rtcRoom$onAVSyncStateChange);

    registerEvent(r"rtcRoom:onVideoPublishStateChanged:info:state:reason:",
        rtcRoom$onVideoPublishStateChanged$info$state$reason);

    registerEvent(r"rtcRoom:onAudioPublishStateChanged:info:state:reason:",
        rtcRoom$onAudioPublishStateChanged$info$state$reason);

    registerEvent(r"rtcRoom:onVideoSubscribeStateChanged:info:state:reason:",
        rtcRoom$onVideoSubscribeStateChanged$info$state$reason);

    registerEvent(r"rtcRoom:onAudioSubscribeStateChanged:info:state:reason:",
        rtcRoom$onAudioSubscribeStateChanged$info$state$reason);

    registerEvent(r"rtcRoom:onRoomStats:", rtcRoom$onRoomStats);

    registerEvent(r"rtcRoom:onRoomEvent:uid:state:info:",
        rtcRoom$onRoomEvent$uid$state$info);

    registerEvent(r"rtcRoom:onLocalStreamStats:info:stats:",
        rtcRoom$onLocalStreamStats$info$stats);

    registerEvent(r"rtcRoom:onRemoteStreamStats:info:stats:",
        rtcRoom$onRemoteStreamStats$info$stats);

    registerEvent(r"rtcRoom:onUserJoined:", rtcRoom$onUserJoined);

    registerEvent(r"rtcRoom:onUserLeave:reason:", rtcRoom$onUserLeave$reason);

    registerEvent(r"onTokenWillExpire:", onTokenWillExpire);

    registerEvent(r"onPublishPrivilegeTokenWillExpire:",
        onPublishPrivilegeTokenWillExpire);

    registerEvent(r"onSubscribePrivilegeTokenWillExpire:",
        onSubscribePrivilegeTokenWillExpire);

    registerEvent(r"rtcRoom:onStreamPublishSuccess:isScreen:",
        rtcRoom$onStreamPublishSuccess$isScreen);

    registerEvent(r"rtcRoom:onAVSyncEvent:userId:eventCode:",
        rtcRoom$onAVSyncEvent$userId$eventCode);

    registerEvent(r"rtcRoom:onUserPublishStreamVideo:info:isPublish:",
        rtcRoom$onUserPublishStreamVideo$info$isPublish);

    registerEvent(r"rtcRoom:onUserPublishStreamAudio:info:isPublish:",
        rtcRoom$onUserPublishStreamAudio$info$isPublish);

    registerEvent(r"rtcRoom:onRoomMessageReceived:message:",
        rtcRoom$onRoomMessageReceived$message);

    registerEvent(r"rtcRoom:onRoomBinaryMessageReceived:message:",
        rtcRoom$onRoomBinaryMessageReceived$message);

    registerEvent(r"rtcRoom:onUserMessageReceived:message:",
        rtcRoom$onUserMessageReceived$message);

    registerEvent(r"rtcRoom:onUserBinaryMessageReceived:message:",
        rtcRoom$onUserBinaryMessageReceived$message);

    registerEvent(r"rtcRoom:onRoomMessageReceived:uid:message:",
        rtcRoom$onRoomMessageReceived$uid$message);

    registerEvent(r"rtcRoom:onRoomBinaryMessageReceived:uid:message:",
        rtcRoom$onRoomBinaryMessageReceived$uid$message);

    registerEvent(r"rtcRoom:onUserMessageReceived:uid:message:",
        rtcRoom$onUserMessageReceived$uid$message);

    registerEvent(r"rtcRoom:onUserBinaryMessageReceived:uid:message:",
        rtcRoom$onUserBinaryMessageReceived$uid$message);

    registerEvent(r"rtcRoom:onUserMessageSendResult:error:",
        rtcRoom$onUserMessageSendResult$error);

    registerEvent(r"rtcRoom:onRoomMessageSendResult:error:",
        rtcRoom$onRoomMessageSendResult$error);

    registerEvent(r"rtcRoom:onSetRoomExtraInfoResult:result:",
        rtcRoom$onSetRoomExtraInfoResult$result);

    registerEvent(
        r"rtcRoom:onRoomExtraInfoUpdate:value:lastUpdateUserId:lastUpdateTimeMs:",
        rtcRoom$onRoomExtraInfoUpdate$value$lastUpdateUserId$lastUpdateTimeMs);

    registerEvent(r"rtcRoom:onRoomStreamExtraInfoUpdate:info:extraInfo:",
        rtcRoom$onRoomStreamExtraInfoUpdate$info$extraInfo);

    registerEvent(r"rtcRoom:onUserVisibilityChanged:errorCode:",
        rtcRoom$onUserVisibilityChanged$errorCode);

    registerEvent(r"rtcRoom:onVideoStreamBanned:isBanned:",
        rtcRoom$onVideoStreamBanned$isBanned);

    registerEvent(r"rtcRoom:onAudioStreamBanned:isBanned:",
        rtcRoom$onAudioStreamBanned$isBanned);

    registerEvent(r"rtcRoom:onForwardStreamStateChanged:",
        rtcRoom$onForwardStreamStateChanged);

    registerEvent(
        r"rtcRoom:onForwardStreamEvent:", rtcRoom$onForwardStreamEvent);

    registerEvent(r"rtcRoom:onNetworkQuality:remoteQualities:",
        rtcRoom$onNetworkQuality$remoteQualities);

    registerEvent(r"rtcRoom:onSubtitleStateChanged:errorCode:errorMessage:",
        rtcRoom$onSubtitleStateChanged$errorCode$errorMessage);

    registerEvent(r"rtcRoom:onSubtitleMessageReceived:",
        rtcRoom$onSubtitleMessageReceived);

    registerEvent(r"rtcRoom:onStreamStateChanged:withUid:state:extraInfo:",
        rtcRoom$onStreamStateChanged$withUid$state$extraInfo);

    registerEvent(r"rtcRoom:onRoomWarning:", rtcRoom$onRoomWarning);

    registerEvent(r"rtcRoom:onStreamAdd:", rtcRoom$onStreamAdd);

    registerEvent(r"rtcRoom:onStreamRemove:stream:reason:",
        rtcRoom$onStreamRemove$stream$reason);
  }

  /// @hidden
  /// @detail callback
  /// @author shenpengliang
  /// @brief 房间状态改变回调，加入房间、异常退出房间、发生房间相关的警告或错误时会收到此回调。
  /// @param rtcRoom ByteRTCAudioRoom 实例
  /// @param roomId 房间 ID。
  /// @param uid 用户 ID。
  /// @param state 房间状态码。 <br>
  ///              - 0: 加入房间成功。
  ///              - 1: 加入房间失败、异常退房、发生房间相关的警告或错误。
  ///              - 2: 离开房间。
  /// @param reason 房间状态发生变化的原因。参看 ByteRTCRoomStateChangeReason{@link #ByteRTCRoomStateChangeReason}。
  ///

  FutureOr<void> rtcRoom$onRoomStateChangedWithReason$withUid$state$reason(
      ByteRTCRoom rtcRoom,
      NSString roomId,
      NSString uid,
      ByteRTCRoomState state,
      ByteRTCRoomStateChangeReason reason) async {}

  /// @detail callback
  /// @region 多房间
  /// @author shenpengliang
  /// @brief 房间状态改变回调，加入房间、异常退出房间、发生房间相关的警告或错误时会收到此回调。
  /// @param rtcRoom ByteRTCAudioRoom 实例
  /// @param roomId 房间 ID。
  /// @param uid 用户 ID。
  /// @param state 房间状态码。 <br>
  ///              - 0: 加入房间成功。
  ///              - !0: 加入房间失败、异常退房、发生房间相关的警告或错误。具体原因参看 ByteRTCErrorCode{@link #ByteRTCErrorCode} 及 ByteRTCWarningCode{@link #ByteRTCWarningCode}。
  /// @param extraInfo 额外信息，如 `{"elapsed":1187,"join_type":0}`。 <br>
  ///                  `join_type`表示加入房间的类型，`0`为首次进房，`1`为重连进房。 <br>
  ///                  `elapsed`表示加入房间耗时，即本地用户从调用 joinRoom:userInfo:userVisibility:roomConfig:{@link #ByteRTCRoom#joinRoom:userInfo:userVisibility:roomConfig} 到加入房间成功所经历的时间间隔，单位为 ms。
  /// @order 0
  ///

  FutureOr<void> rtcRoom$onRoomStateChanged$withUid$state$extraInfo(
      ByteRTCRoom rtcRoom,
      NSString roomId,
      NSString uid,
      NSInteger state,
      NSString extraInfo) async {}

  /// @detail callback
  /// @region 多房间
  /// @author shenpengliang
  /// @brief 离开房间成功回调。 <br>
  ///        用户调用 leaveRoom{@link #ByteRTCRoom#leaveRoom} 方法后，SDK 会停止所有的发布订阅流，并释放所有通话相关的音视频资源。SDK 完成所有的资源释放后通过此回调通知用户。
  /// @param rtcRoom  ByteRTCRoom 对象。
  /// @param stats 保留参数，目前为空。
  /// @note
  ///       - 用户调用 leaveRoom{@link #ByteRTCRoom#leaveRoom} 方法离开房间后，如果立即调用 destroy{@link #ByteRTCRoom#destroy} 销毁房间实例或 destroyRTCEngine{@link #ByteRTCEngine#destroyRTCEngine} 方法销毁 RTC 引擎，则将无法收到此回调事件。
  ///       - 离开房间结束通话后，如果 App 需要使用系统音视频设备，则建议在收到此回调后再初始化音视频设备，否则可能由于 SDK 占用了导致 App 初始化音视频设备失败。
  /// @order 2

  FutureOr<void> rtcRoom$onLeaveRoom(
      ByteRTCRoom rtcRoom, ByteRTCRoomStats stats) async {}

  /// @detail callback
  /// @author wangzhanqiang
  /// @brief 发布端调用 setMultiDeviceAVSync:{@link #ByteRTCRoom#setMultiDeviceAVSync} 后音视频同步状态发生改变时，会收到此回调。
  /// @param rtcRoom ByteRTCRoom 实例。
  /// @param state 音视频同步状态，参看 ByteRTCAVSyncState{@link #ByteRTCAVSyncState}。

  FutureOr<void> rtcRoom$onAVSyncStateChange(
      ByteRTCRoom rtcRoom, ByteRTCAVSyncState state) async {}

  /// @detail callback
  /// @region 房间管理
  /// @author xuyiling.x10
  /// @brief 视频发布状态改变回调。
  /// @param rtcRoom ByteRTCRoom 实例。
  /// @param streamId 流 ID。
  /// @param info 流信息。
  /// @param state 发布状态码，参看 ByteRTCPublishState{@link #ByteRTCPublishState}。
  /// @param reason 本端视频流发布状态改变的具体原因，参看 ByteRTCPublishStateChangeReason{@link #ByteRTCPublishStateChangeReason}。
  /// @order 0

  FutureOr<void> rtcRoom$onVideoPublishStateChanged$info$state$reason(
      ByteRTCRoom rtcRoom,
      NSString streamId,
      ByteRTCStreamInfo info,
      ByteRTCPublishState state,
      ByteRTCPublishStateChangeReason reason) async {}

  /// @detail callback
  /// @region 房间管理
  /// @author xuyiling.x10
  /// @brief 音频发布状态改变回调。
  /// @param rtcRoom ByteRTCRoom 实例。
  /// @param streamId 流 ID。
  /// @param info 流信息。
  /// @param state 发布状态码，参看 ByteRTCPublishState{@link #ByteRTCPublishState}。
  /// @param reason 本端音频流发布状态改变的具体原因，参看 ByteRTCPublishStateChangeReason{@link #ByteRTCPublishStateChangeReason}。
  /// @order 0

  FutureOr<void> rtcRoom$onAudioPublishStateChanged$info$state$reason(
      ByteRTCRoom rtcRoom,
      NSString streamId,
      ByteRTCStreamInfo info,
      ByteRTCPublishState state,
      ByteRTCPublishStateChangeReason reason) async {}

  /// @detail callback
  /// @region 房间管理
  /// @author xuyiling.x10
  /// @brief 视频订阅状态发生改变回调。
  /// @param rtcRoom ByteRTCRoom 实例。
  /// @param streamId 流 ID。
  /// @param info 流信息。
  /// @param state 订阅状态码，参看 ByteRTCSubscribeState{@link #ByteRTCSubscribeState}。
  /// @param reason 视频订阅状态改变的具体原因，参看 ByteRTCSubscribeStateChangeReason{@link #ByteRTCSubscribeStateChangeReason}。
  /// @order 0

  FutureOr<void> rtcRoom$onVideoSubscribeStateChanged$info$state$reason(
      ByteRTCRoom rtcRoom,
      NSString streamId,
      ByteRTCStreamInfo info,
      ByteRTCSubscribeState state,
      ByteRTCSubscribeStateChangeReason reason) async {}

  /// @detail callback
  /// @region 房间管理
  /// @author xuyiling.x10
  /// @brief 音频订阅状态发生改变回调。
  /// @param rtcRoom ByteRTCRoom 实例。
  /// @param streamId 流 ID。
  /// @param info 流信息。
  /// @param state 订阅状态码，参看 ByteRTCSubscribeState{@link #ByteRTCSubscribeState}。
  /// @param reason 音频订阅状态改变的具体原因，参看 ByteRTCSubscribeStateChangeReason{@link #ByteRTCSubscribeStateChangeReason}。
  /// @order 0

  FutureOr<void> rtcRoom$onAudioSubscribeStateChanged$info$state$reason(
      ByteRTCRoom rtcRoom,
      NSString streamId,
      ByteRTCStreamInfo info,
      ByteRTCSubscribeState state,
      ByteRTCSubscribeStateChangeReason reason) async {}

  /// @detail callback
  /// @region 多房间
  /// @author yejing
  /// @brief 房间内通话统计信息回调。 <br>
  ///        用户进房开始通话后，每 2s 收到一次本回调。
  /// @param rtcRoom  ByteRTCRoom 对象。
  /// @param stats 当前 ByteRTCRoom 统计数据，详见：ByteRTCRoomStats{@link #ByteRTCRoomStats}

  FutureOr<void> rtcRoom$onRoomStats(
      ByteRTCRoom rtcRoom, ByteRTCRoomStats stats) async {}

  /// @hidden
  /// @detail callback
  /// @region 多房间
  /// @valid since 3.60.
  /// @author taoshasha
  /// @brief 房间事件回调。
  /// @param rtcRoom  ByteRTCRoom 对象。
  /// @param roomId 房间 ID。
  /// @param uid 用户 ID。
  /// @param state 房间事件状态。详见 ByteRTCRoomEvent{@link #ByteRTCRoomEvent}。
  /// @param info 房间封禁时，包含封禁时间。详见 ByteRTCRoomEventInfo{@link #ByteRTCRoomEventInfo}。
  /// @order 0
  ///

  FutureOr<void> rtcRoom$onRoomEvent$uid$state$info(
      ByteRTCRoom rtcRoom,
      NSString roomId,
      NSString uid,
      ByteRTCRoomEvent state,
      ByteRTCRoomEventInfo info) async {}

  /// @detail callback
  /// @author yejing
  /// @brief 本地流数据统计以及网络质量回调。 <br>
  ///        本地用户发布流成功后，SDK 会周期性（2s）的通过此回调事件通知用户发布的流在此次统计周期内的质量统计信息。 <br>
  ///        统计信息通过 ByteRTCLocalStreamStats{@link #ByteRTCLocalStreamStats} 类型的回调参数传递给用户，其中包括发送音视频比特率、发送帧率、编码帧率，网络质量等。
  /// @param rtcRoom  ByteRTCRoom 对象。
  /// @param streamId 流 ID。
  /// @param info 流信息。
  /// @param stats 当前房间本地流数据统计。详见：ByteRTCLocalStreamStats{@link #ByteRTCLocalStreamStats}

  FutureOr<void> rtcRoom$onLocalStreamStats$info$stats(
      ByteRTCRoom rtcRoom,
      NSString streamId,
      ByteRTCStreamInfo info,
      ByteRTCLocalStreamStats stats) async {}

  /// @detail callback
  /// @author yejing
  /// @brief 本地订阅的远端音/视频流数据统计以及网络质量回调。 <br>
  ///        本地用户订阅流成功后，SDK 会周期性（2s）的通过此回调事件通知用户订阅的流在此次统计周期内的质量统计信息，包括：发送音视频比特率、发送帧率、编码帧率，网络质量等。
  /// @param rtcRoom  ByteRTCRoom 对象。
  /// @param streamId 流 ID。
  /// @param info 流信息。
  /// @param stats 当前房间本地流数据统计。 详见：ByteRTCRemoteStreamStats{@link #ByteRTCRemoteStreamStats}

  FutureOr<void> rtcRoom$onRemoteStreamStats$info$stats(
      ByteRTCRoom rtcRoom,
      NSString streamId,
      ByteRTCStreamInfo info,
      ByteRTCRemoteStreamStats stats) async {}

  /// @detail callback
  /// @author shenpengliang
  /// @brief 远端可见用户加入房间，或房内不可见用户切换为可见的回调。 <br>
  ///        1. 远端用户调用 setUserVisibility:{@link #ByteRTCRoom#setUserVisibility} 方法将自身设为可见后加入房间时，房间内其他用户将收到该事件。 <br>
  ///        2. 远端可见用户断网后重新连入房间时，房间内其他用户将收到该事件。 <br>
  ///        3. 房间内隐身远端用户调用 setUserVisibility:{@link #ByteRTCRoom#setUserVisibility} 方法切换至可见时，房间内其他用户将收到该事件。 <br>
  ///        4. 新进房用户会收到进房前已在房内的可见用户的进房回调通知。
  /// @param rtcRoom ByteRTCRoom 对象。
  /// @param userInfo 用户信息，参看 ByteRTCUserInfo{@link #ByteRTCUserInfo}。

  FutureOr<void> rtcRoom$onUserJoined(
      ByteRTCRoom rtcRoom, ByteRTCUserInfo userInfo) async {}

  /// @detail callback
  /// @author shenpengliang
  /// @brief 远端用户离开 RTC 房间，或切至不可见时，本地用户会收到此事件
  /// @param rtcRoom `ByteRTCRoom` 实例
  /// @param uid 离开房间，或切至不可见的的远端用户 ID。
  /// @param reason 用户离开房间的原因： <br>
  ///              - 0: 远端用户调用 leaveRoom{@link #ByteRTCRoom#leaveRoom} 主动退出房间。
  ///              - 1: 远端用户因 Token 过期或网络原因等掉线。详细信息请参看[连接状态提示](https://www.volcengine.com/docs/6348/95376)
  ///              - 2: 远端用户调用 setUserVisibility:{@link #ByteRTCRoom#setUserVisibility} 切换至不可见状态。
  ///              - 3: 服务端调用 OpenAPI 将远端用户踢出房间。

  FutureOr<void> rtcRoom$onUserLeave$reason(ByteRTCRoom rtcRoom, NSString uid,
      ByteRTCUserOfflineReason reason) async {}

  /// @detail callback
  /// @author shenpengliang
  /// @brief 当 SDK 检测到 Token 的进房权限将在 30 秒内过期时，触发该回调。
  ///        收到该回调后，你需调用 updateToken:{@link #ByteRTCRTSRoom#updateToken} 更新 Token 进房权限。
  /// @param rtcRoom `ByteRTCRoom` 实例
  /// @note 若 Token 进房权限过期且未及时更新： <br>
  ///        - 用户此时尝试进房会收到 rtcRoom:onRoomStateChanged:withUid:state:extraInfo:{@link #ByteRTCRoomDelegate#rtcRoom:onRoomStateChanged:withUid:state:extraInfo} 回调，提示错误码为 `-1000` Token 无效；
  ///        - 用户已在房间内则会被移出房间，本地用户会收到 rtcRoom:onRoomStateChanged:withUid:state:extraInfo:{@link #ByteRTCRoomDelegate#rtcRoom:onRoomStateChanged:withUid:state:extraInfo} 回调，提示错误码为 `-1009` Token 过期，同时远端用户会收到 rtcRoom:onUserLeave:reason:{@link #ByteRTCRoomDelegate#rtcRoom:onUserLeave:reason} 回调，提示原因为 `1` Token 进房权限过期。

  FutureOr<void> onTokenWillExpire(ByteRTCRoom rtcRoom) async {}

  /// @detail callback
  /// @author qipengxiang
  /// @brief Token 发布权限过期前 30 秒将触发该回调。 <br>
  ///        收到该回调后，你需调用 updateToken:{@link #ByteRTCRTSRoom#updateToken} 更新 Token 发布权限。
  /// @param rtcRoom `ByteRTCRoom` 实例
  /// @note Token 发布权限过期后：
  ///        - 已发布流或尝试发布流时，本端会收到 rtcRoom:onVideoPublishStateChanged:info:state:reason:{@link #ByteRTCRoomDelegate#rtcRoom:onVideoPublishStateChanged:info:state:reason}、rtcRoom:onAudioPublishStateChanged:info:state:reason:{@link #ByteRTCRoomDelegate#rtcRoom:onAudioPublishStateChanged:info:state:reason}回调，提示`kPublishStateChangeReasonNoPublishPermission`，没有发布权限。
  ///        - 发布中的流将停止发布。远端用户会收到 rtcRoom:onUserPublishStreamVideo:info:isPublish:{@link #ByteRTCRoomDelegate#rtcRoom:onUserPublishStreamVideo:info:isPublish}、rtcRoom:onUserPublishStreamAudio:info:isPublish:{@link #ByteRTCRoomDelegate#rtcRoom:onUserPublishStreamAudio:info:isPublish} 回调，提示该流已停止发布。
  /// @order 3

  FutureOr<void> onPublishPrivilegeTokenWillExpire(ByteRTCRoom rtcRoom) async {}

  /// @detail callback
  /// @author qipengxiang
  /// @brief Token 订阅权限过期前 30 秒将触发该回调。 <br>
  ///        收到该回调后，你需调用 updateToken:{@link #ByteRTCRTSRoom#updateToken} 更新 Token 订阅权限有效期。
  /// @param rtcRoom `ByteRTCRoom` 实例
  /// @note 若收到该回调后未及时更新 Token，Token 订阅权限过期后，尝试新订阅流会失败，已订阅的流会取消订阅，并且会收到 rtcRoom:onVideoSubscribeStateChanged:info:state:reason:{@link #ByteRTCRoomDelegate#rtcRoom:onVideoSubscribeStateChanged:info:state:reason}、rtcRoom:onAudioSubscribeStateChanged:info:state:reason:{@link #ByteRTCRoomDelegate#rtcRoom:onAudioSubscribeStateChanged:info:state:reason} 回调，提示错误码为 `-1003` 没有订阅权限。

  FutureOr<void> onSubscribePrivilegeTokenWillExpire(
      ByteRTCRoom rtcRoom) async {}

  /// @hidden for internal use only
  /// @detail callback
  /// @region 多房间
  /// @author shenpengliang
  /// @brief 当发布流成功的时候回调该事件
  /// @param rtcRoom  ByteRTCRoom 对象。
  /// @param userId 用户 ID
  /// @param isScreen 该流是否是屏幕流 <br>

  FutureOr<void> rtcRoom$onStreamPublishSuccess$isScreen(
      ByteRTCRoom rtcRoom, NSString userId, BOOL isScreen) async {}

  /// @detail callback
  /// @valid since 3.60.
  /// @author xuyiling.x10
  /// @brief 发布端调用 setMultiDeviceAVSync:{@link #ByteRTCRoom#setMultiDeviceAVSync} 后音视频同步状态发生错误时，会收到此回调。
  /// @param rtcRoom ByteRTCRoom 对象
  /// @param roomId 房间 ID。
  /// @param userId 用户 ID。
  /// @param eventCode 音视频同步状态错误，参看 ByteRTCAVSyncEvent{@link #ByteRTCAVSyncEvent}。
  /// @order 1
  ///

  FutureOr<void> rtcRoom$onAVSyncEvent$userId$eventCode(ByteRTCRoom rtcRoom,
      NSString roomId, NSString userId, ByteRTCAVSyncEvent eventCode) async {}

  /// @detail callback
  /// @region 房间管理
  /// @author xuyiling.x10
  /// @brief 房间内远端摄像头采集的媒体流的回调。
  /// @param rtcRoom ByteRTCRoom 对象
  /// @param streamId 流 ID。
  /// @param info 流信息。
  /// @param isPublish 为true代表流发布，为false代表流移除。
  /// @note 当房间内的远端用户调用 publishStreamVideo:{@link #ByteRTCRoom#publishStreamVideo} 成功发布由摄像头采集的媒体流时，本地用户会收到该回调，此时本地用户可以自行选择是否调用 subscribeStreamVideo:subscribe:{@link #ByteRTCRoom#subscribeStreamVideo:subscribe} 订阅此流。
  /// @order 2

  FutureOr<void> rtcRoom$onUserPublishStreamVideo$info$isPublish(
      ByteRTCRoom rtcRoom,
      NSString streamId,
      ByteRTCStreamInfo info,
      BOOL isPublish) async {}

  /// @detail callback
  /// @region 房间管理
  /// @author xuyiling.x10
  /// @brief 房间内远端麦克风采集的媒体流的回调。
  /// @param rtcRoom ByteRTCRoom 对象
  /// @param streamId 流 ID。
  /// @param info 流信息。
  /// @param isPublish 为true代表流发布，为false代表流移除。
  /// @note 当房间内的远端用户调用 publishStreamAudio:{@link #ByteRTCRoom#publishStreamAudio} 成功发布由麦克风采集的媒体流时，本地用户会收到该回调，此时本地用户可以自行选择是否调用 subscribeStreamAudio:subscribe:{@link #ByteRTCRoom#subscribeStreamAudio:subscribe} 订阅此流。
  /// @order 2

  FutureOr<void> rtcRoom$onUserPublishStreamAudio$info$isPublish(
      ByteRTCRoom rtcRoom,
      NSString streamId,
      ByteRTCStreamInfo info,
      BOOL isPublish) async {}

  /// @detail callback
  /// @author hanchenchen.c
  /// @brief > 该接口将于 3.64 onRoomMessageReceived:uid:message:{@link #ByteRTCRoomDelegate#rtcRoom:onRoomMessageReceived:uid:message} 代替。
  /// @brief 收到房间中调用 sendRoomMessage:{@link #ByteRTCRoom#sendRoomMessage} 发送的广播文本消息时，收到此回调。
  /// @param rtcRoom ByteRTCRoom 对象
  /// @param uid 消息发送者 ID 。
  /// @param message 收到的文本消息内容。
  ///

  FutureOr<void> rtcRoom$onRoomMessageReceived$message(
      ByteRTCRoom rtcRoom, NSString uid, NSString message) async {}

  /// @detail callback
  /// @author hanchenchen.c
  /// @brief > 该接口将于 3.64 onRoomBinaryMessageReceived:uid:message:{@link #ByteRTCRoomDelegate#rtcRoom:onRoomBinaryMessageReceived:uid:message} 代替。
  /// @brief 收到房间内广播二进制消息的回调。 <br>
  ///        房间内其他用户调用 sendRoomBinaryMessage:{@link #ByteRTCRoom#sendRoomBinaryMessage} 发送的广播二进制消息时，收到此回调。
  /// @param rtcRoom ByteRTCRoom 对象
  /// @param uid 消息发送者 ID 。
  /// @param message 收到的二进制消息内容。
  ///

  FutureOr<void> rtcRoom$onRoomBinaryMessageReceived$message(
      ByteRTCRoom rtcRoom, NSString uid, NSData message) async {}

  /// @detail callback
  /// @author hanchenchen.c
  /// @brief > 该接口将于 3.64 onUserMessageReceived:uid:message:{@link #ByteRTCRoomDelegate#rtcRoom:onUserMessageReceived:uid:message} 代替。
  /// @brief 收到来自房间中其他用户通过 sendUserMessage:message:config:{@link #ByteRTCRTSRoom#sendUserMessage:message:config} 发来的点对点文本消息时，会收到此回调。
  /// @param rtcRoom ByteRTCRoom 对象
  /// @param uid 消息发送者的用户 ID。
  /// @param message 收到的文本消息内容。
  ///

  FutureOr<void> rtcRoom$onUserMessageReceived$message(
      ByteRTCRoom rtcRoom, NSString uid, NSString message) async {}

  /// @detail callback
  /// @brief > 该接口将于 3.64 onUserBinaryMessageReceived:uid:message:{@link #ByteRTCRoomDelegate#rtcRoom:onUserBinaryMessageReceived:uid:message} 代替。
  /// @author hanchenchen.c
  /// @brief 收到来自房间中其他用户通过 sendUserBinaryMessage:message:config:{@link #ByteRTCRTSRoom#sendUserBinaryMessage:message:config} 发来的点对点二进制消息时，会收到此回调。
  /// @param rtcRoom ByteRTCRoom 对象
  /// @param uid 消息发送者的用户 ID。
  /// @param message 收到的二进制消息内容。
  ///

  FutureOr<void> rtcRoom$onUserBinaryMessageReceived$message(
      ByteRTCRoom rtcRoom, NSString uid, NSData message) async {}

  /// @detail callback
  /// @author hanchenchen.c
  /// @brief 收到房间中调用 sendRoomMessage:{@link #ByteRTCRoom#sendRoomMessage} 发送的广播文本消息时，收到此回调。
  /// @param rtcRoom ByteRTCRoom 对象
  /// @param msgid 消息编号。
  /// @param uid 消息发送者 ID 。
  /// @param message 收到的文本消息内容。
  ///

  FutureOr<void> rtcRoom$onRoomMessageReceived$uid$message(ByteRTCRoom rtcRoom,
      NSInteger msgid, NSString uid, NSString message) async {}

  /// @detail callback
  /// @author hanchenchen.c
  /// @brief 收到房间内广播二进制消息的回调。 <br>
  ///        房间内其他用户调用 sendRoomBinaryMessage:{@link #ByteRTCRoom#sendRoomBinaryMessage} 发送的广播二进制消息时，收到此回调。
  /// @param rtcRoom ByteRTCRoom 对象
  /// @param msgid 消息编号。
  /// @param uid 消息发送者 ID 。
  /// @param message 收到的二进制消息内容。
  ///

  FutureOr<void> rtcRoom$onRoomBinaryMessageReceived$uid$message(
      ByteRTCRoom rtcRoom,
      NSInteger msgid,
      NSString uid,
      NSData message) async {}

  /// @detail callback
  /// @author hanchenchen.c
  /// @brief 收到来自房间中其他用户通过 sendUserMessage:message:config:{@link #ByteRTCRTSRoom#sendUserMessage:message:config} 发来的点对点文本消息时，会收到此回调。
  /// @param rtcRoom ByteRTCRoom 对象
  /// @param msgid 消息编号。
  /// @param uid 消息发送者的用户 ID。
  /// @param message 收到的文本消息内容。
  ///

  FutureOr<void> rtcRoom$onUserMessageReceived$uid$message(ByteRTCRoom rtcRoom,
      NSInteger msgid, NSString uid, NSString message) async {}

  /// @detail callback
  /// @author hanchenchen.c
  /// @brief 收到来自房间中其他用户通过 sendUserBinaryMessage:message:config:{@link #ByteRTCRTSRoom#sendUserBinaryMessage:message:config} 发来的点对点二进制消息时，会收到此回调。
  /// @param rtcRoom ByteRTCRoom 对象
  /// @param msgid 消息编号.
  /// @param uid 消息发送者的用户 ID。
  /// @param message 收到的二进制消息内容。
  ///

  FutureOr<void> rtcRoom$onUserBinaryMessageReceived$uid$message(
      ByteRTCRoom rtcRoom,
      NSInteger msgid,
      NSString uid,
      NSData message) async {}

  /// @detail callback
  /// @author hanchenchen.c
  /// @brief 向房间内单个用户发送文本或二进制消息后（P2P），消息发送方会收到该消息发送结果回调。
  /// @param rtcRoom ByteRTCRoom 对象。
  /// @param msgid 本条消息的 ID。
  /// @param error 消息发送结果，详见 ByteRTCUserMessageSendResult{@link #ByteRTCUserMessageSendResult}
  /// @note
  ///        - 你应调用 sendUserMessage:message:config:{@link #ByteRTCRTSRoom#sendUserMessage:message:config} 向房间内单个用户发送文本消息
  ///        - 你应调用 sendUserBinaryMessage:message:config:{@link #ByteRTCRTSRoom#sendUserBinaryMessage:message:config} 向房间内单个用户发送二进制消息
  ///

  FutureOr<void> rtcRoom$onUserMessageSendResult$error(ByteRTCRoom rtcRoom,
      NSInteger msgid, ByteRTCUserMessageSendResult error) async {}

  /// @detail callback
  /// @author hanchenchen.c
  /// @brief 调用 sendRoomMessage:{@link #ByteRTCRoom#sendRoomMessage} 或 sendRoomBinaryMessage:{@link #ByteRTCRoom#sendRoomBinaryMessage} 向房间内群发文本或二进制消息后，消息发送方会收到该消息发送结果回调。
  /// @param rtcRoom ByteRTCRoom 对象。
  /// @param msgid 本条消息的 ID。
  /// @param error 消息发送结果，详见 ByteRTCRoomMessageSendResult{@link #ByteRTCRoomMessageSendResult}
  ///

  FutureOr<void> rtcRoom$onRoomMessageSendResult$error(ByteRTCRoom rtcRoom,
      NSInteger msgid, ByteRTCRoomMessageSendResult error) async {}

  /// @valid since 3.52.
  /// @detail callback
  /// @author lichangfeng.rtc
  /// @brief 调用 setRoomExtraInfo:value:{@link #ByteRTCRoom#setRoomExtraInfo:value} 结果回调。
  /// @param rtcRoom ByteRTCRoom 对象。
  /// @param taskId 本次调用的任务编号。
  /// @param result 设置房间附加信息结果，详见 ByteRTCSetRoomExtraInfoResult{@link #ByteRTCSetRoomExtraInfoResult}

  FutureOr<void> rtcRoom$onSetRoomExtraInfoResult$result(ByteRTCRoom rtcRoom,
      NSInteger taskId, ByteRTCSetRoomExtraInfoResult result) async {}

  /// @valid since 3.52.
  /// @detail callback
  /// @author lichangfeng.rtc
  /// @brief 接收到房间附加信息的回调。 <br>
  ///        房间内其他用户调用 setRoomExtraInfo:value:{@link #ByteRTCRoom#setRoomExtraInfo:value} 设置房间附加信息，收到此回调。 <br>
  ///        另外用户加入房间成功后会收到这个房间全量附加信息
  /// @param rtcRoom ByteRTCRoom 对象。
  /// @param key 附加信息的键值
  /// @param value 附加信息的内容
  /// @param lastUpdateUserId 最后更新这条附加信息的用户编号
  /// @param lastUpdateTimeMs 最后更新这条附加信息的 Unix 时间，时间精度是毫秒

  FutureOr<void>
      rtcRoom$onRoomExtraInfoUpdate$value$lastUpdateUserId$lastUpdateTimeMs(
          ByteRTCRoom rtcRoom,
          NSString key,
          NSString value,
          NSString lastUpdateUserId,
          NSInteger lastUpdateTimeMs) async {}

  /// @valid since 3.54
  /// @detail callback
  /// @brief 接收同一房间内，其他用户调用 setStreamExtraInfo:{@link #ByteRTCRoom#setStreamExtraInfo} 设置的流附加信息的回调。
  /// @param rtcRoom `ByteRTCRoom` 实例
  /// @param streamId 流附加信息的流 ID
  /// @param streamInfo 流附加信息的流信息
  /// @param extraInfo 流附加信息

  FutureOr<void> rtcRoom$onRoomStreamExtraInfoUpdate$info$extraInfo(
      ByteRTCRoom rtcRoom,
      NSString streamId,
      ByteRTCStreamInfo streamInfo,
      NSString extraInfo) async {}

  /// @valid since 3.54
  /// @detail callback
  /// @author caocun
  /// @brief 用户调用 setUserVisibility:{@link #ByteRTCRoom#setUserVisibility} 设置用户可见性的回调。
  /// @param rtcRoom `ByteRTCRoom` 实例
  /// @param currentUserVisibility 当前用户的可见性。 <br>
  ///        - YES: 可见，用户可以在房间内发布音视频流，房间中的其他用户将收到用户的行为通知，例如进房、开启视频采集和退房。
  ///        - NO: 不可见，用户不可以在房间内发布音视频流，房间中的其他用户不会收到用户的行为通知，例如进房、开启视频采集和退房。
  /// @param errorCode 设置用户可见性错误码，参看 ByteRTCUserVisibilityChangeError{@link #ByteRTCUserVisibilityChangeError}。

  FutureOr<void> rtcRoom$onUserVisibilityChanged$errorCode(
      ByteRTCRoom rtcRoom,
      BOOL currentUserVisibility,
      ByteRTCUserVisibilityChangeError errorCode) async {}

  /// @detail callback
  /// @author qipengxiang
  /// @brief 通过调用服务端 BanUserStream/UnbanUserStream 方法禁用/解禁指定房间内指定用户视频流的发送时，触发此回调。
  /// @param rtcRoom `ByteRTCRoom` 实例
  /// @param uid 被禁用/解禁的视频流用户 ID
  /// @param banned 视频流发送状态 <br>
  ///        - true: 视频流发送被禁用
  ///        - false: 视频流发送被解禁
  /// @note
  ///        - 房间内指定用户被禁止/解禁视频流发送时，房间内所有用户都会收到该回调。
  ///        - 若被封禁用户断网或退房后再进房，则依然是封禁状态，且房间内所有人会再次收到该回调。
  ///        - 指定用户被封禁后，房间内其他用户退房后再进房，会再次收到该回调。
  ///        - 同一房间解散后再次创建，房间内状态清空。

  FutureOr<void> rtcRoom$onVideoStreamBanned$isBanned(
      ByteRTCRoom rtcRoom, NSString uid, BOOL banned) async {}

  /// @detail callback
  /// @author qipengxiang
  /// @brief 通过调用服务端 BanUserStream/UnbanUserStream 方法禁用/解禁指定房间内指定用户音频流的发送时，触发此回调。
  /// @param rtcRoom `ByteRTCRoom` 实例
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

  FutureOr<void> rtcRoom$onAudioStreamBanned$isBanned(
      ByteRTCRoom rtcRoom, NSString uid, BOOL banned) async {}

  /// @detail callback
  /// @author shenpengliang
  /// @brief 跨房间媒体流转发状态和错误回调
  /// @param rtcRoom ByteRTCRoom 对象
  /// @param infos 跨房间媒体流转发目标房间信息数组，详见 ByteRTCForwardStreamStateInfo{@link #ByteRTCForwardStreamStateInfo}
  ///

  FutureOr<void> rtcRoom$onForwardStreamStateChanged(ByteRTCRoom rtcRoom,
      NSArray<ByteRTCForwardStreamStateInfo> infos) async {}

  /// @detail callback
  /// @author shenpengliang
  /// @brief 跨房间媒体流转发事件回调
  /// @param rtcRoom ByteRTCRoom 对象
  /// @param infos 跨房间媒体流转发目标房间事件数组，详见 ByteRTCForwardStreamEventInfo{@link #ByteRTCForwardStreamEventInfo}
  ///

  FutureOr<void> rtcRoom$onForwardStreamEvent(ByteRTCRoom rtcRoom,
      NSArray<ByteRTCForwardStreamEventInfo> infos) async {}

  /// @detail callback
  /// @author chengchao.cc951119
  /// @brief 加入房间并发布或订阅流后， 以每 2 秒一次的频率，报告本地用户和已订阅的远端用户的上下行网络质量信息。
  /// @param rtcRoom ByteRTCRoom 对象
  /// @param localQuality 本端网络质量，详见 ByteRTCNetworkQualityStats{@link #ByteRTCNetworkQualityStats}。
  /// @param remoteQualities 已订阅用户的网络质量，详见 ByteRTCNetworkQualityStats{@link #ByteRTCNetworkQualityStats}。
  /// @note 更多通话中的监测接口，详见[通话中质量监测](https://www.volcengine.com/docs/6348/106866)。

  FutureOr<void> rtcRoom$onNetworkQuality$remoteQualities(
      ByteRTCRoom rtcRoom,
      ByteRTCNetworkQualityStats localQuality,
      NSArray<ByteRTCNetworkQualityStats> remoteQualities) async {}

  /// @detail callback
  /// @author qiaoxingwang
  /// @brief 字幕状态发生改变回调。 <br>
  ///         当用户调用 startSubtitle:{@link #ByteRTCRoom#startSubtitle} 和 stopSubtitle{@link #ByteRTCRoom#stopSubtitle} 使字幕状态发生改变或出现错误时，触发该回调。
  /// @param rtcRoom ByteRTCRoom 实例。
  /// @param state 字幕状态。参看 ByteRTCSubtitleState{@link #ByteRTCSubtitleState}。
  /// @param errorCode 字幕任务错误码。参看 ByteRTCSubtitleErrorCode{@link #ByteRTCSubtitleErrorCode}。
  /// @param errorMessage 第三方服务出现的错误。当因第三方服务出现错误，导致字幕状态改变时，用户可通过此参数获取具体的错误信息。如果不是因为第三方服务导致字幕状态改变，该字段为空。

  FutureOr<void> rtcRoom$onSubtitleStateChanged$errorCode$errorMessage(
      ByteRTCRoom rtcRoom,
      ByteRTCSubtitleState state,
      ByteRTCSubtitleErrorCode errorCode,
      NSString errorMessage) async {}

  /// @detail callback
  /// @author qiaoxingwang
  /// @brief 字幕相关内容回调。 <br>
  ///         当用户调用 startSubtitle:{@link #ByteRTCRoom#startSubtitle} 后会收到此回调，通知字幕的相关信息。
  /// @param rtcRoom ByteRTCRoom 实例。
  /// @param subtitles 字幕消息内容。参看 ByteRTCSubtitleMessage{@link #ByteRTCSubtitleMessage}。

  FutureOr<void> rtcRoom$onSubtitleMessageReceived(
      ByteRTCRoom rtcRoom, NSArray<ByteRTCSubtitleMessage> subtitles) async {}

  /// @deprecated since 3.60, refer to [Upgrade Guide](https://www.volcengine.com/docs/6348/70007) for more.
  /// @detail callback
  /// @author shenpengliang
  /// @brief 流状态改变回调，发生流相关的警告或错误时会收到此回调。
  /// @param rtcRoom `ByteRTCRoom` 实例
  /// @param roomId 房间 ID。
  /// @param uid 用户 ID。
  /// @param state 流状态码，参看 ByteRTCErrorCode{@link #ByteRTCErrorCode} 及 ByteRTCWarningCode{@link #ByteRTCWarningCode}。
  /// @param extraInfo 附加信息，目前为空。

  FutureOr<void> rtcRoom$onStreamStateChanged$withUid$state$extraInfo(
      ByteRTCRoom rtcRoom,
      NSString roomId,
      NSString uid,
      NSInteger state,
      NSString extraInfo) async {}

  /// @hidden
  /// @deprecated since 3.41 and will be deleted in 3.51, use rtcRoom:onRoomStateChanged:withUid:state:extraInfo:{@link #ByteRTCRoomDelegate#rtcRoom:onRoomStateChanged:withUid:state:extraInfo} and rtcRoom:onStreamStateChanged:withUid:state:extraInfo:{@link #ByteRTCRoomDelegate#rtcRoom:onStreamStateChanged:withUid:state:extraInfo} instead.
  /// @detail callback
  /// @author shenpengliang
  /// @brief 发生警告回调。 <br>
  ///        SDK 运行时出现了警告。SDK 通常会自动恢复，警告信息可以忽略。
  /// @param rtcRoom ByteRTCRoom 对象。
  /// @param warningCode 警告码，详见枚举类型 ByteRTCWarningCode{@link #ByteRTCWarningCode} 。
  ///

  FutureOr<void> rtcRoom$onRoomWarning(
      ByteRTCRoom rtcRoom, ByteRTCWarningCode warningCode) async {}

  /// @deprecated since 3.36 and will be deleted in 3.51, use rtcRoom:onUserPublishStreamVideo:info:isPublish:{@link #ByteRTCRoomDelegate#rtcRoom:onUserPublishStreamVideo:info:isPublish}instead.
  /// @detail callback
  /// @author shenpengliang
  /// @brief 以下情况会触发此回调： <br>
  ///        - 房间内的用户发布新的音视频流时，房间内的其他用户会收到此回调通知。
  ///        - 房间内的用户原音视频流被移出后，又重新发布音视频流时，房间内的其他用户会收到此回调通知。
  ///        - 用户刚加入房间时，会收到此回调，包含房间中所有已发布的流。
  /// @param rtcRoom  ByteRTCRoom 对象。
  /// @param stream 流属性，参看 ByteRTCStream{@link #ByteRTCStream} 。
  ///

  FutureOr<void> rtcRoom$onStreamAdd(
      ByteRTCRoom rtcRoom, id<ByteRTCStream> stream) async {}

  /// @hidden
  /// @deprecated since 3.36 and will be deleted in 3.51, use rtcRoom:onUserUnpublishStream:type:reason: instead.
  /// @detail callback
  /// @author shenpengliang
  /// @brief 房间内的远端用户停止发布音视频流时，本地用户会收到此回调通知。
  /// @param rtcRoom ByteRTCRoom 对象
  /// @param uid 远端流来源的用户 ID 。
  /// @param stream 流的属性，参看 ByteRTCStream{@link #ByteRTCStream}。
  /// @param reason 远端流移除的原因，参看 ByteRTCStreamRemoveReason{@link #ByteRTCStreamRemoveReason} 。
  ///

  FutureOr<void> rtcRoom$onStreamRemove$stream$reason(
      ByteRTCRoom rtcRoom,
      NSString uid,
      id<ByteRTCStream> stream,
      ByteRTCStreamRemoveReason reason) async {}
}

class ByteRTCFaceDetectionObserver extends NativeObserverClass {
  static const _$namespace = r'ByteRTCFaceDetectionObserver';

  ByteRTCFaceDetectionObserver([NativeClassOptions? options])
      : super(options == null
            ? const NativeClassOptions([],
                className: _$namespace,
                instanceType: InstanceType.manual,
                bridgeKey: 'com.volcengine.rtc.hybrid_runtime',
                methodMap: {
                  r"onFaceDetectResult": r"onFaceDetectResult:",
                  r"onExpressionDetectResult": r"onExpressionDetectResult:"
                })
            : NativeClassOptions.fromMap({
                ...options.toMap(),
                'bridgeKey': 'com.volcengine.rtc.hybrid_runtime',
              })) {
    registerEvent(r"onFaceDetectResult:", onFaceDetectResult);

    registerEvent(r"onExpressionDetectResult:", onExpressionDetectResult);
  }

  /// @detail callback
  /// @author wangjunlin.3182
  /// @brief 特效 SDK 进行人脸检测结果的回调。 <br>
  ///        调用 enableFaceDetection:withInterval:withModelPath:{@link #ByteRTCVideoEffect#enableFaceDetection:withInterval:withModelPath} 注册了 ByteRTCFaceDetectionObserver{@link #ByteRTCFaceDetectionObserver} ，并使用 RTC SDK 中包含的特效 SDK 进行视频特效处理时，你会收到此回调。
  /// @param result 人脸检测结果, 参看 ByteRTCFaceDetectionResult{@link #ByteRTCFaceDetectionResult}。

  FutureOr<void> onFaceDetectResult(ByteRTCFaceDetectionResult result) async {}

  /// @hidden for intrnal use only
  /// @detail callback
  /// @author zhushufan.ref
  /// @brief 特效 SDK 进行人像属性检测结果的回调。 <br>
  ///        调用 registerFaceDetectionObserver:withInterval: 注册了 ByteRTCFaceDetectionObserver{@link #ByteRTCFaceDetectionObserver}，并调用 setVideoEffectExpressionDetect:{@link #ByteRTCVideoEffect#setVideoEffectExpressionDetect} 开启人像属性检测后，你会收到此回调。
  /// @param result 人像属性检测结果, 参看 ByteRTCExpressionDetectResult{@link #ByteRTCExpressionDetectResult}。

  FutureOr<void> onExpressionDetectResult(
      ByteRTCExpressionDetectResult result) async {}
}

class ByteRTCMediaPlayerAudioFrameObserver extends NativeObserverClass {
  static const _$namespace = r'ByteRTCMediaPlayerAudioFrameObserver';

  ByteRTCMediaPlayerAudioFrameObserver([NativeClassOptions? options])
      : super(options == null
            ? const NativeClassOptions([],
                className: _$namespace,
                instanceType: InstanceType.manual,
                bridgeKey: 'com.volcengine.rtc.hybrid_runtime',
                methodMap: {r"onFrame$audioFrame": r"onFrame:audioFrame:"})
            : NativeClassOptions.fromMap({
                ...options.toMap(),
                'bridgeKey': 'com.volcengine.rtc.hybrid_runtime',
              })) {
    registerEvent(r"onFrame:audioFrame:", onFrame$audioFrame);
  }

  /// @detail callback
  /// @brief 当本地音频文件混音时，回调播放的音频帧。
  /// @param playerId 播放器 ID
  /// @param audioFrame 参看 ByteRTCAudioFrame{@link #ByteRTCAudioFrame}。

  FutureOr<void> onFrame$audioFrame(
      int playerId, ByteRTCAudioFrame audioFrame) async {}
}

class ByteRTCChorusCacheSyncObserver extends NativeObserverClass {
  static const _$namespace = r'ByteRTCChorusCacheSyncObserver';

  ByteRTCChorusCacheSyncObserver([NativeClassOptions? options])
      : super(options == null
            ? const NativeClassOptions([],
                className: _$namespace,
                instanceType: InstanceType.manual,
                bridgeKey: 'com.volcengine.rtc.hybrid_runtime',
                methodMap: {
                  r"onSyncedVideoFrames$withUids":
                      r"onSyncedVideoFrames:withUids:",
                  r"onSyncedUsersChanged": r"onSyncedUsersChanged:",
                  r"onSyncEvent$withError": r"onSyncEvent:withError:"
                })
            : NativeClassOptions.fromMap({
                ...options.toMap(),
                'bridgeKey': 'com.volcengine.rtc.hybrid_runtime',
              })) {
    registerEvent(
        r"onSyncedVideoFrames:withUids:", onSyncedVideoFrames$withUids);

    registerEvent(r"onSyncedUsersChanged:", onSyncedUsersChanged);

    registerEvent(r"onSyncEvent:withError:", onSyncEvent$withError);
  }

  /// @detail callback
  /// @brief 调用 startChorusCacheSync:observer:{@link #ByteRTCEngine#startChorusCacheSync:observer}，并设置为 `consumer` 的用户会通过此回调获取经缓存同步后的视频帧。获取频率通过启动同步时的 `fps` 进行设置。
  /// @param videoFrames 对应 `uids` 的视频帧。参看 ByteRTCVideoFrame{@link #ByteRTCVideoFrame}。
  /// @param uids 参与合唱缓存同步的 `producer` 和 `retransmitter` 的列表，不包括参与但未发送媒体数据的用户。

  FutureOr<void> onSyncedVideoFrames$withUids(
      NSArray<id<ByteRTCVideoFrame>> videoFrames,
      NSArray<NSString> uids) async {}

  /// @detail callback
  /// @brief 参与合唱缓存同步的 `producer` 和 `retransmitter` 发生变化时，收到此回调。
  /// @param uids 当前的参与者列表
  /// @note 有以下情况可能造成参与者发生变化： <br>
  ///        - 用户主动调用 startChorusCacheSync:observer:{@link #ByteRTCEngine#startChorusCacheSync:observer} 或 stopChorusCacheSync{@link #ByteRTCEngine#stopChorusCacheSync};
  ///        - 原本参与缓存同步的用户发生异常退出。

  FutureOr<void> onSyncedUsersChanged(NSArray<NSString> uids) async {}

  /// @detail callback
  /// @brief 缓存同步事件回调
  /// @param event 事件，参看 ByteRTCChorusCacheSyncEvent{@link #ByteRTCChorusCacheSyncEvent}。
  /// @param error 错误码，参看 ByteRTCChorusCacheSyncError{@link #ByteRTCChorusCacheSyncError}。

  FutureOr<void> onSyncEvent$withError(ByteRTCChorusCacheSyncEvent event,
      ByteRTCChorusCacheSyncError error) async {}
}

class ByteRTCKTVManagerDelegate extends NativeObserverClass {
  static const _$namespace = r'ByteRTCKTVManagerDelegate';

  ByteRTCKTVManagerDelegate([NativeClassOptions? options])
      : super(options == null
            ? const NativeClassOptions([],
                className: _$namespace,
                instanceType: InstanceType.manual,
                bridgeKey: 'com.volcengine.rtc.hybrid_runtime',
                methodMap: {
                  r"ktvManager$onMusicListResult$totalSize$errorCode":
                      r"ktvManager:onMusicListResult:totalSize:errorCode:",
                  r"ktvManager$onSearchMusicResult$totalSize$errorCode":
                      r"ktvManager:onSearchMusicResult:totalSize:errorCode:",
                  r"ktvManager$onHotMusicResult$errorCode":
                      r"ktvManager:onHotMusicResult:errorCode:",
                  r"ktvManager$onMusicDetailResult$errorCode":
                      r"ktvManager:onMusicDetailResult:errorCode:",
                  r"ktvManager$onDownloadSuccess$downloadResult":
                      r"ktvManager:onDownloadSuccess:downloadResult:",
                  r"ktvManager$onDownloadFailed$errorCode":
                      r"ktvManager:onDownloadFailed:errorCode:",
                  r"ktvManager$onDownloadMusicProgress$progress":
                      r"ktvManager:onDownloadMusicProgress:progress:",
                  r"ktvManager$onClearCacheResult":
                      r"ktvManager:onClearCacheResult:"
                })
            : NativeClassOptions.fromMap({
                ...options.toMap(),
                'bridgeKey': 'com.volcengine.rtc.hybrid_runtime',
              })) {
    registerEvent(r"ktvManager:onMusicListResult:totalSize:errorCode:",
        ktvManager$onMusicListResult$totalSize$errorCode);

    registerEvent(r"ktvManager:onSearchMusicResult:totalSize:errorCode:",
        ktvManager$onSearchMusicResult$totalSize$errorCode);

    registerEvent(r"ktvManager:onHotMusicResult:errorCode:",
        ktvManager$onHotMusicResult$errorCode);

    registerEvent(r"ktvManager:onMusicDetailResult:errorCode:",
        ktvManager$onMusicDetailResult$errorCode);

    registerEvent(r"ktvManager:onDownloadSuccess:downloadResult:",
        ktvManager$onDownloadSuccess$downloadResult);

    registerEvent(r"ktvManager:onDownloadFailed:errorCode:",
        ktvManager$onDownloadFailed$errorCode);

    registerEvent(r"ktvManager:onDownloadMusicProgress:progress:",
        ktvManager$onDownloadMusicProgress$progress);

    registerEvent(
        r"ktvManager:onClearCacheResult:", ktvManager$onClearCacheResult);
  }

  /// @detail callback
  /// @author lihuan.wuti2ha
  /// @brief 歌曲列表回调。
  /// @param ktvManager 参考 ByteRTCKTVManager{@link #ByteRTCKTVManager}。
  /// @param musics 歌曲数据数组，参看 ByteRTCMusicInfo{@link #ByteRTCMusicInfo}。
  /// @param totalSize 数据条目总数。
  /// @param errorCode 错误码，成功时返回 0，其余值参看 ByteRTCKTVErrorCode{@link #ByteRTCKTVErrorCode}。

  FutureOr<void> ktvManager$onMusicListResult$totalSize$errorCode(
      ByteRTCKTVManager ktvManager,
      NSArray<ByteRTCMusicInfo> musics,
      int totalSize,
      ByteRTCKTVErrorCode errorCode) async {}

  /// @detail callback
  /// @author lihuan.wuti2ha
  /// @brief 搜索歌曲结果回调。
  /// @param ktvManager 参考 ByteRTCKTVManager{@link #ByteRTCKTVManager}。
  /// @param musics 歌曲数据数组，参看 ByteRTCMusicInfo{@link #ByteRTCMusicInfo}。
  /// @param totalSize 数据条目总数。
  /// @param errorCode 错误码，成功时返回 0，其余值参看 ByteRTCKTVErrorCode{@link #ByteRTCKTVErrorCode}。

  FutureOr<void> ktvManager$onSearchMusicResult$totalSize$errorCode(
      ByteRTCKTVManager ktvManager,
      NSArray<ByteRTCMusicInfo> musics,
      int totalSize,
      ByteRTCKTVErrorCode errorCode) async {}

  /// @detail callback
  /// @author lihuan.wuti2ha
  /// @brief 热榜歌曲结果回调。
  /// @param ktvManager 参考 ByteRTCKTVManager{@link #ByteRTCKTVManager}。
  /// @param hotMusicInfos 热榜歌曲数据数组，参看 ByteRTCHotMusicInfo{@link #ByteRTCHotMusicInfo}。
  /// @param errorCode 错误码，成功时返回 0，其余值参看 ByteRTCKTVErrorCode{@link #ByteRTCKTVErrorCode}。

  FutureOr<void> ktvManager$onHotMusicResult$errorCode(
      ByteRTCKTVManager ktvManager,
      NSArray<ByteRTCHotMusicInfo> hotMusicInfos,
      ByteRTCKTVErrorCode errorCode) async {}

  /// @detail callback
  /// @author lihuan.wuti2ha
  /// @brief 歌曲详细信息回调。
  /// @param ktvManager 参考 ByteRTCKTVManager{@link #ByteRTCKTVManager}。
  /// @param music 歌曲数据，参看 ByteRTCMusicInfo{@link #ByteRTCMusicInfo}。
  /// @param errorCode 错误码，成功时返回 0，其余值参看 ByteRTCKTVErrorCode{@link #ByteRTCKTVErrorCode}。

  FutureOr<void> ktvManager$onMusicDetailResult$errorCode(
      ByteRTCKTVManager ktvManager,
      ByteRTCMusicInfo music,
      ByteRTCKTVErrorCode errorCode) async {}

  /// @detail callback
  /// @author lihuan.wuti2ha
  /// @brief 下载成功回调。
  /// @param ktvManager 参考 ByteRTCKTVManager{@link #ByteRTCKTVManager}。
  /// @param downloadId 下载任务 ID。
  /// @param result 下载信息，参看 ByteRTCDownloadResult{@link #ByteRTCDownloadResult}。

  FutureOr<void> ktvManager$onDownloadSuccess$downloadResult(
      ByteRTCKTVManager ktvManager,
      int downloadId,
      ByteRTCDownloadResult result) async {}

  /// @detail callback
  /// @author lihuan.wuti2ha
  /// @brief 下载失败回调。
  /// @param ktvManager 参考 ByteRTCKTVManager{@link #ByteRTCKTVManager}。
  /// @param downloadId 下载任务 ID。
  /// @param errorCode 错误码，参看 ByteRTCKTVErrorCode{@link #ByteRTCKTVErrorCode}。

  FutureOr<void> ktvManager$onDownloadFailed$errorCode(
      ByteRTCKTVManager ktvManager,
      int downloadId,
      ByteRTCKTVErrorCode errorCode) async {}

  /// @detail callback
  /// @author lihuan.wuti2ha
  /// @brief 歌曲文件下载进度回调。
  /// @param ktvManager 参考 ByteRTCKTVManager{@link #ByteRTCKTVManager}。
  /// @param downloadId 下载任务 ID。
  /// @param downloadPercentage 下载进度百分比，取值范围 [0,100]。

  FutureOr<void> ktvManager$onDownloadMusicProgress$progress(
      ByteRTCKTVManager ktvManager,
      int downloadId,
      int downloadPercentage) async {}

  /// @detail callback
  /// @brief 清理文件缓存结果回调。
  /// @param ktvManager 参考 ByteRTCKTVManager{@link #ByteRTCKTVManager}。
  /// @param errorCode 错误码，非 0 为失败，参看 ByteRTCKTVErrorCode{@link #ByteRTCKTVErrorCode}。

  FutureOr<void> ktvManager$onClearCacheResult(
      ByteRTCKTVManager ktvManager, ByteRTCKTVErrorCode errorCode) async {}
}

class ByteRTCMediaPlayerEventHandler extends NativeObserverClass {
  static const _$namespace = r'ByteRTCMediaPlayerEventHandler';

  ByteRTCMediaPlayerEventHandler([NativeClassOptions? options])
      : super(options == null
            ? const NativeClassOptions([],
                className: _$namespace,
                instanceType: InstanceType.manual,
                bridgeKey: 'com.volcengine.rtc.hybrid_runtime',
                methodMap: {
                  r"onMediaPlayerStateChanged$state$error":
                      r"onMediaPlayerStateChanged:state:error:",
                  r"onMediaPlayerPlayingProgress$progress":
                      r"onMediaPlayerPlayingProgress:progress:",
                  r"onMediaPlayerEvent$event$message":
                      r"onMediaPlayerEvent:event:message:"
                })
            : NativeClassOptions.fromMap({
                ...options.toMap(),
                'bridgeKey': 'com.volcengine.rtc.hybrid_runtime',
              })) {
    registerEvent(r"onMediaPlayerStateChanged:state:error:",
        onMediaPlayerStateChanged$state$error);

    registerEvent(r"onMediaPlayerPlayingProgress:progress:",
        onMediaPlayerPlayingProgress$progress);

    registerEvent(
        r"onMediaPlayerEvent:event:message:", onMediaPlayerEvent$event$message);
  }

  /// @detail callback
  /// @brief 播放状态改变时回调。
  /// @param playerId ByteRTCMediaPlayer{@link #ByteRTCMediaPlayer} 的 ID。通过 getMediaPlayer:{@link #ByteRTCEngine#getMediaPlayer} 设置。
  /// @param state 混音状态。参考 ByteRTCPlayerState{@link #ByteRTCPlayerState}。
  /// @param error 错误码。参考 ByteRTCPlayerError{@link #ByteRTCPlayerError}。

  FutureOr<void> onMediaPlayerStateChanged$state$error(
      int playerId, ByteRTCPlayerState state, ByteRTCPlayerError error) async {}

  /// @detail callback
  /// @brief 播放进度周期性回调。回调周期通过 setProgressInterval:{@link #ByteRTCMediaPlayer#setProgressInterval} 设置。
  /// @param playerId ByteRTCMediaPlayer{@link #ByteRTCMediaPlayer} 的 ID。通过 getMediaPlayer:{@link #ByteRTCEngine#getMediaPlayer} 设置。
  /// @param progress 进度。单位 ms。

  FutureOr<void> onMediaPlayerPlayingProgress$progress(
      int playerId, int64_t progress) async {}

  /// @valid since 3.59
  /// @detail callback
  /// @author wangfeng.1004
  /// @brief 播放事件回调。调用 selectAudioTrack:{@link #ByteRTCMediaPlayer#selectAudioTrack} 和 setPosition:{@link #ByteRTCMediaPlayer#setPosition} 后，会触发此回调。
  /// @param playerId ByteRTCMediaPlayer{@link #ByteRTCMediaPlayer} 的 ID。通过 getMediaPlayer:{@link #ByteRTCEngine#getMediaPlayer} 设置。
  /// @param event 播放器事件。参看 ByteRTCPlayerEvent{@link #ByteRTCPlayerEvent}。
  /// @param message 事件描述信息，可能为空。

  FutureOr<void> onMediaPlayerEvent$event$message(
      int playerId, ByteRTCPlayerEvent event, NSString message) async {}
}

class ByteRTCAudioFrameProcessor extends NativeObserverClass {
  static const _$namespace = r'ByteRTCAudioFrameProcessor';

  ByteRTCAudioFrameProcessor([NativeClassOptions? options])
      : super(options == null
            ? const NativeClassOptions([],
                className: _$namespace,
                instanceType: InstanceType.manual,
                bridgeKey: 'com.volcengine.rtc.hybrid_runtime',
                methodMap: {})
            : NativeClassOptions.fromMap({
                ...options.toMap(),
                'bridgeKey': 'com.volcengine.rtc.hybrid_runtime',
              })) {}
}

class ByteRTCAudioFrameObserver extends NativeObserverClass {
  static const _$namespace = r'ByteRTCAudioFrameObserver';

  ByteRTCAudioFrameObserver([NativeClassOptions? options])
      : super(options == null
            ? const NativeClassOptions([],
                className: _$namespace,
                instanceType: InstanceType.manual,
                bridgeKey: 'com.volcengine.rtc.hybrid_runtime',
                methodMap: {
                  r"onRecordAudioFrame": r"onRecordAudioFrame:",
                  r"onPlaybackAudioFrame": r"onPlaybackAudioFrame:",
                  r"onRemoteUserAudioFrame$info$audioFrame":
                      r"onRemoteUserAudioFrame:info:audioFrame:",
                  r"onMixedAudioFrame": r"onMixedAudioFrame:",
                  r"onCaptureMixedAudioFrame": r"onCaptureMixedAudioFrame:"
                })
            : NativeClassOptions.fromMap({
                ...options.toMap(),
                'bridgeKey': 'com.volcengine.rtc.hybrid_runtime',
              })) {
    registerEvent(r"onRecordAudioFrame:", onRecordAudioFrame);

    registerEvent(r"onPlaybackAudioFrame:", onPlaybackAudioFrame);

    registerEvent(r"onRemoteUserAudioFrame:info:audioFrame:",
        onRemoteUserAudioFrame$info$audioFrame);

    registerEvent(r"onMixedAudioFrame:", onMixedAudioFrame);

    registerEvent(r"onCaptureMixedAudioFrame:", onCaptureMixedAudioFrame);
  }

  /// @detail callback
  /// @author wangjunzheng
  /// @brief 返回麦克风录制的音频数据
  /// @param audioFrame 音频数据, 详见： ByteRTCAudioFrame{@link #ByteRTCAudioFrame}

  FutureOr<void> onRecordAudioFrame(ByteRTCAudioFrame audioFrame) async {}

  /// @detail callback
  /// @author wangjunzheng
  /// @brief 返回订阅的所有远端用户混音后的音频数据。
  /// @param audioFrame 音频数据, 详见： ByteRTCAudioFrame{@link #ByteRTCAudioFrame}

  FutureOr<void> onPlaybackAudioFrame(ByteRTCAudioFrame audioFrame) async {}

  /// @detail callback
  /// @author wangjunzheng
  /// @brief 返回远端单个用户的音频数据
  /// @param streamId 远端流对应的唯一标识
  /// @param info 远端流详细信息，参看 ByteRTCStreamInfo{@link #ByteRTCStreamInfo}
  /// @param audioFrame 音频数据，参看 ByteRTCAudioFrame{@link #ByteRTCAudioFrame}
  /// @note 此回调在播放线程调用。不要在此回调中做任何耗时的事情，否则可能会影响整个音频播放链路。

  FutureOr<void> onRemoteUserAudioFrame$info$audioFrame(NSString streamId,
      ByteRTCStreamInfo info, ByteRTCAudioFrame audioFrame) async {}

  /// @detail callback
  /// @author wangjunzheng
  /// @brief 返回本地麦克风录制和订阅的所有远端用户混音后的音频数据
  /// @param audioFrame 音频数据, 详见： ByteRTCAudioFrame{@link #ByteRTCAudioFrame}

  FutureOr<void> onMixedAudioFrame(ByteRTCAudioFrame audioFrame) async {}

  /// @detail callback
  /// @author huanghao
  /// @brief 返回本地麦克风录制的音频数据，本地 `MediaPlayer` / `EffectPlayer` 播放音频文件混音后的音频数据
  /// @param audioFrame 音频数据, 详见： ByteRTCAudioFrame{@link #ByteRTCAudioFrame}

  FutureOr<void> onCaptureMixedAudioFrame(ByteRTCAudioFrame audioFrame) async {}
}

class ByteRTCExternalVideoEncoderEventHandler extends NativeObserverClass {
  static const _$namespace = r'ByteRTCExternalVideoEncoderEventHandler';

  ByteRTCExternalVideoEncoderEventHandler([NativeClassOptions? options])
      : super(options == null
            ? const NativeClassOptions([],
                className: _$namespace,
                instanceType: InstanceType.manual,
                bridgeKey: 'com.volcengine.rtc.hybrid_runtime',
                methodMap: {
                  r"onStart$info": r"onStart:info:",
                  r"onStop$info": r"onStop:info:",
                  r"onRateUpdate$info$withVideoIndex$withFps$withBitRate":
                      r"onRateUpdate:info:withVideoIndex:withFps:withBitRate:",
                  r"onRequestKeyFrame$info$withVideoIndex":
                      r"onRequestKeyFrame:info:withVideoIndex:",
                  r"onActiveVideoLayer$info$withVideoIndex$withActive":
                      r"onActiveVideoLayer:info:withVideoIndex:withActive:"
                })
            : NativeClassOptions.fromMap({
                ...options.toMap(),
                'bridgeKey': 'com.volcengine.rtc.hybrid_runtime',
              })) {
    registerEvent(r"onStart:info:", onStart$info);

    registerEvent(r"onStop:info:", onStop$info);

    registerEvent(r"onRateUpdate:info:withVideoIndex:withFps:withBitRate:",
        onRateUpdate$info$withVideoIndex$withFps$withBitRate);

    registerEvent(r"onRequestKeyFrame:info:withVideoIndex:",
        onRequestKeyFrame$info$withVideoIndex);

    registerEvent(r"onActiveVideoLayer:info:withVideoIndex:withActive:",
        onActiveVideoLayer$info$withVideoIndex$withActive);
  }

  /// @detail callback
  /// @author wangzhanqiang
  /// @brief 提示自定义编码帧可以开始推送的回调。 <br>
  ///        收到该回调后，你即可调用 pushExternalEncodedVideoFrame:withEncodedVideoFrame:{@link #ByteRTCEngine#pushExternalEncodedVideoFrame:withEncodedVideoFrame} 向 SDK 推送自定义编码视频帧
  /// @param streamId 自定义编码流 ID
  /// @param info 自定义编码流信息，参看 ByteRTCStreamInfo{@link #ByteRTCStreamInfo}

  FutureOr<void> onStart$info(
      NSString streamId, ByteRTCStreamInfo info) async {}

  /// @detail callback
  /// @author wangzhanqiang
  /// @brief 当收到该回调时，你需停止向 SDK 推送自定义编码视频帧
  /// @param streamId 自定义编码流 ID
  /// @param info 自定义编码流信息，参看 ByteRTCStreamInfo{@link #ByteRTCStreamInfo}

  FutureOr<void> onStop$info(NSString streamId, ByteRTCStreamInfo info) async {}

  /// @detail callback
  /// @author wangzhanqiang
  /// @brief 当自定义编码流的帧率或码率发生变化时，触发该回调
  /// @param streamId 远端编码流 ID
  /// @param info 远端编码流信息，参看 ByteRTCStreamInfo{@link #ByteRTCStreamInfo}
  /// @param videoIndex 对应编码流的下标
  /// @param fps 变化后的帧率，单位：fps
  /// @param bitRateKps 变化后的码率，单位：kbps

  FutureOr<void> onRateUpdate$info$withVideoIndex$withFps$withBitRate(
      NSString streamId,
      ByteRTCStreamInfo info,
      NSInteger videoIndex,
      NSInteger fps,
      NSInteger bitRateKps) async {}

  /// @detail callback
  /// @author wangzhanqiang
  /// @brief 提示流发布端需重新生成关键帧的回调
  /// @param streamId 远端编码流 ID
  /// @param info 远端编码流信息，参看 ByteRTCStreamInfo{@link #ByteRTCStreamInfo}
  /// @param videoIndex 对应编码流的下标

  FutureOr<void> onRequestKeyFrame$info$withVideoIndex(
      NSString streamId, ByteRTCStreamInfo info, NSInteger videoIndex) async {}

  /// @valid since 3.56
  /// @detail callback
  /// @author wangqianqian.1104
  /// @brief 作为自定义编码视频流的发送端，你会在视频流可发送状态发生变化时，收到此回调。 <br>
  ///        你可以根据此回调的提示，仅对可发送的视频流进行编码，以降低本端视频编码性能消耗。此回调会根据多个因素综合判断触发，包括：本端设备性能和本端网络性能，以及按需订阅场景下，远端用户是否订阅。
  /// @param streamId 自定义编码流 ID
  /// @param info 自定义编码流信息，参看 ByteRTCStreamInfo{@link #ByteRTCStreamInfo}
  /// @param videoIndex 对应编码流的下标
  /// @param active 该路流可发送状态
  /// @note 要收到此回调，必须调用 setVideoSourceType:{@link #ByteRTCEngine#setVideoSourceType} 设置视频源是自定义编码，且通过 setExternalVideoEncoderEventHandler:{@link #ByteRTCEngine#setExternalVideoEncoderEventHandler} 设置了回调句柄。

  FutureOr<void> onActiveVideoLayer$info$withVideoIndex$withActive(
      NSString streamId,
      ByteRTCStreamInfo info,
      NSInteger videoIndex,
      BOOL active) async {}
}

class ByteRTCRemoteEncodedAudioFrameObserver extends NativeObserverClass {
  static const _$namespace = r'ByteRTCRemoteEncodedAudioFrameObserver';

  ByteRTCRemoteEncodedAudioFrameObserver([NativeClassOptions? options])
      : super(options == null
            ? const NativeClassOptions([],
                className: _$namespace,
                instanceType: InstanceType.manual,
                bridgeKey: 'com.volcengine.rtc.hybrid_runtime',
                methodMap: {
                  r"onRemoteEncodedAudioFrame$info$audioFrame":
                      r"onRemoteEncodedAudioFrame:info:audioFrame:"
                })
            : NativeClassOptions.fromMap({
                ...options.toMap(),
                'bridgeKey': 'com.volcengine.rtc.hybrid_runtime',
              })) {
    registerEvent(r"onRemoteEncodedAudioFrame:info:audioFrame:",
        onRemoteEncodedAudioFrame$info$audioFrame);
  }

  /// @detail callback
  /// @hidden for internal use only
  /// @brief 调用 registerRemoteEncodedAudioFrameObserver:{@link #ByteRTCEngine#registerRemoteEncodedAudioFrameObserver} 后，SDK 收到远端音频帧信息时，回调该事件
  /// @param streamId 收到的远端音频流对应的唯一标识
  /// @param info 收到的远端音频流的详细信息，参看 ByteRTCStreamInfo{@link #ByteRTCStreamInfo}
  /// @param audioFrame 远端音频帧信息，参看 ByteRTCEncodedAudioFrameData{@link #ByteRTCEncodedAudioFrameData}

  FutureOr<void> onRemoteEncodedAudioFrame$info$audioFrame(NSString streamId,
      ByteRTCStreamInfo info, ByteRTCEncodedAudioFrameData audioFrame) async {}
}
