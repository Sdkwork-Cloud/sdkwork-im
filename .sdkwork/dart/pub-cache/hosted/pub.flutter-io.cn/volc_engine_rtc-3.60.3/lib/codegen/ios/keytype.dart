/// Copyright (c) 2026 Beijing Volcano Engine Technology Ltd.

// ignore_for_file: camel_case_types, annotate_overrides, null_check_always_fails, unused_import, non_constant_identifier_names
import 'package:hybrid_runtime/hybrid_runtime.dart';
import 'dart:async';
import 'dart:typed_data';
import 'types.dart';
import 'api.dart';
import 'external.dart';
import 'callback.dart';

enum ByteRTCVideoApplyRotation {
  /// @brief （默认值）不旋转。
  ///
  ByteRTCVideoApplyRotationDefault(-1),

  /// @brief 自动转正视频，即根据视频帧的旋转角信息将视频帧旋转到 0 度。
  ///
  ByteRTCVideoApplyRotation0(0);

  final dynamic $value;
  const ByteRTCVideoApplyRotation([this.$value]);
}

enum ByteRTCEnv {
  /// @brief 线上环境。
  ///
  ByteRTCEnvProduct(0),

  /// @brief BOE 环境。
  ///
  ByteRTCEnvBOE(1),

  /// @brief 测试环境。
  ///
  ByteRTCEnvTest(2);

  final dynamic $value;
  const ByteRTCEnv([this.$value]);
}

enum ByteRTCDataMessageSourceType {
  /// @brief 通过客户端或服务端的插入的自定义消息。
  ///
  ByteRTCDataMessageSourceTypeDefault(0),

  /// @brief 系统数据，包含音量指示信息。
  ///
  ByteRTCDataMessageSourceTypeSystem(1);

  final dynamic $value;
  const ByteRTCDataMessageSourceType([this.$value]);
}

enum ByteRTCVideoPixelFormat {
  /// @brief 未知格式
  ///
  ByteRTCVideoPixelFormatUnknown(0),

  /// @brief YUV I420 格式
  ///
  ByteRTCVideoPixelFormatI420(1),

  /// @brief YUV NV12 格式
  ///
  ByteRTCVideoPixelFormatNV12(2),

  /// @brief YUV NV21 格式
  ///
  ByteRTCVideoPixelFormatNV21(3),

  /// @brief RGB 24bit 格式，
  ///
  ByteRTCVideoPixelFormatRGB24(4),

  /// @brief RGBA 编码格式
  ///
  ByteRTCVideoPixelFormatRGBA(5),

  /// @brief ARGB 编码格式
  ///
  ByteRTCVideoPixelFormatARGB(6),

  /// @brief BGRA 编码格式
  ///
  ByteRTCVideoPixelFormatBGRA(7),

  /// @brief 像素格式结束标志。新加的格式数值应该小于ByteRTCVideoPixelFormatEndMark。
  ///

  ByteRTCVideoPixelFormatEndMark('0xFF'),

  /// @brief Texture2D 格式
  ///
  ByteRTCVideoPixelFormatTexture2D('0x0DE1'),

  /// @brief TextureOES 格式
  ///
  ByteRTCVideoPixelFormatTextureOES('0x8D65');

  final dynamic $value;
  const ByteRTCVideoPixelFormat([this.$value]);
}

enum ByteRTCMediaDeviceState {
  /// @brief 设备已开启
  ///
  ByteRTCMediaDeviceStateStarted(1),

  /// @brief 设备已停止
  ///
  ByteRTCMediaDeviceStateStopped(2),

  /// @brief 设备运行时错误 <br>
  ///       例如，当媒体设备的预期行为是正常采集，但没有收到采集数据时，将回调该状态。
  ///
  ByteRTCMediaDeviceStateRuntimeError(3),

  /// @brief 设备已插入 <br>
  /// 你可以调用获取设备接口更新设备列表。
  ///
  ByteRTCMediaDeviceStateAdded(10),

  /// @brief 设备被移除 <br>
  /// 你可以调用获取设备接口更新设备列表。
  ///
  ByteRTCMediaDeviceStateRemoved(11),

  /// @brief 系统通话，锁屏或第三方应用打断了音视频通话。将在通话结束或第三方应用结束占用后自动恢复。
  ///
  ByteRTCMediaDeviceStateInterruptionBegan(12),

  /// @brief 音视频通话已从系统电话或第三方应用打断中恢复
  ///
  ByteRTCMediaDeviceStateInterruptionEnded(13),

  /// @hidden(iOS)
  /// @brief 获取设备列表超时后，收到设备列表通知。 <br>
  /// 再次调用获取设备接口更新设备列表。
  ///
  ByteRTCMediaDeviceListUpdated(16);

  final dynamic $value;
  const ByteRTCMediaDeviceState([this.$value]);
}

enum GameSceneType {
  /// @brief 普通场景。<br>
  ///        同一个小队房间的队友，仅支持在同一个世界房间内通话。
  ///
  GameSceneTypeNormal(0),

  /// @brief 主题公园场景。<br>
  ///        同一个小队房间的队友，支持跨世界房间通话。
  ///
  GameSceneTypeThemePark(1);

  final dynamic $value;
  const GameSceneType([this.$value]);
}

enum ByteRTCRecordingFileType {
  /// @brief aac 格式文件
  ///
  ByteRTCRecordingFileTypeAAC(0),

  /// @brief mp4 格式文件
  ///
  ByteRTCRecordingFileTypeMP4(1);

  final dynamic $value;
  const ByteRTCRecordingFileType([this.$value]);
}

enum ByteRTCMixedStreamTaskEvent {
  /// @hidden for internal use only
  ///
  ByteRTCMixedStreamTaskEventBase(0),

  /// @brief 任务发起成功。
  ///
  ByteRTCMixedStreamTaskEventStartSuccess(1),

  /// @brief 任务发起失败。
  ///
  ByteRTCMixedStreamTaskEventStartFailed(2),

  /// @brief 任务更新成功。
  ///
  ByteRTCMixedStreamTaskEventUpdateSuccess(3),

  /// @brief 任务更新失败。
  ///
  ByteRTCMixedStreamTaskEventUpdateFailed(4),

  /// @brief 任务停止。
  ///
  ByteRTCMixedStreamTaskEventStopSuccess(5),

  /// @brief 结束任务失败。
  ///
  ByteRTCMixedStreamTaskEventStopFailed(6),

  /// @brief Warning 事件
  ///
  ByteRTCMixedStreamTaskEventWarning(7);

  final dynamic $value;
  const ByteRTCMixedStreamTaskEvent([this.$value]);
}

enum ByteRTCSubscribeMediaType {
  /// @brief 既不订阅音频，也不订阅视频
  ///
  ByteRTCSubscribeMediaTypeNone(0),

  /// @brief 只订阅音频，不订阅视频
  ///
  ByteRTCSubscribeMediaTypeAudioOnly(1),

  /// @brief 只订阅视频，不订阅音频
  ///
  ByteRTCSubscribeMediaTypeVideoOnly(2),

  /// @brief 同时订阅音频和视频
  ///
  ByteRTCSubscribeMediaTypeAudioAndVideo(3);

  final dynamic $value;
  const ByteRTCSubscribeMediaType([this.$value]);
}

enum ByteRTCMuteState {
  /// @brief 发送
  ///
  ByteRTCMuteStateOff(0),

  /// @brief 停止发送
  ///
  ByteRTCMuteStateOn(1);

  final dynamic $value;
  const ByteRTCMuteState([this.$value]);
}

class ByteRTCRemoteVideoConfig extends NativeClass {
  static const _$namespace = r'ByteRTCRemoteVideoConfig';
  static get codegen_$namespace => _$namespace;

  ByteRTCRemoteVideoConfig([NativeClassOptions? options])
      : super(options == null
            ? const NativeClassOptions(
                [],
                className: _$namespace,
                instanceType: InstanceType.manual,
                bridgeKey: 'com.volcengine.rtc.hybrid_runtime',
              )
            : NativeClassOptions.fromMap({
                ...options.toMap(),
                'bridgeKey': 'com.volcengine.rtc.hybrid_runtime',
              }));

  /// @brief 期望订阅的最高帧率，单位：fps，默认值为 0 即满帧订阅，设为大于 0 的值时开始生效。 <br>
  ///        如果发布端发布帧率 > 订阅端订阅的帧率，下行媒体服务器 SVC 丢帧，订阅端收到通过此接口设置的帧率；如果发布端发布帧率 < 订阅端订阅的帧率，则订阅端只能收到发布的帧率。<br>
  ///        仅码流支持 SVC 分级编码特性时方可生效。
  FutureOr<int?> get framerate async {
    return await sendInstanceGet<int?>("framerate");
  }

  set framerate(FutureOr<int?> value) {
    sendInstanceSet("framerate", value);
  }

  /// @brief 视频宽度，单位：px
  FutureOr<int?> get width async {
    return await sendInstanceGet<int?>("width");
  }

  set width(FutureOr<int?> value) {
    sendInstanceSet("width", value);
  }

  /// @brief 视频高度，单位：px
  FutureOr<int?> get height async {
    return await sendInstanceGet<int?>("height");
  }

  set height(FutureOr<int?> value) {
    sendInstanceSet("height", value);
  }
}

enum ByteRTCEffectErrorType {
  /// @hidden 仅用于会议
  /// @brief 虚拟背景设置错误。
  ///
  ByteRTCEffectErrorVirtualBackgroundFailure(1),

  /// @hidden 仅用于会议
  /// @brief 特效独立进程崩溃。
  ///
  ByteRTCEffectErrorChildProcTerminate(2);

  final dynamic $value;
  const ByteRTCEffectErrorType([this.$value]);
}

class ByteRTCMixedStreamTaskInfo extends NativeClass {
  static const _$namespace = r'ByteRTCMixedStreamTaskInfo';
  static get codegen_$namespace => _$namespace;

  ByteRTCMixedStreamTaskInfo([NativeClassOptions? options])
      : super(options == null
            ? const NativeClassOptions(
                [],
                className: _$namespace,
                instanceType: InstanceType.manual,
                bridgeKey: 'com.volcengine.rtc.hybrid_runtime',
              )
            : NativeClassOptions.fromMap({
                ...options.toMap(),
                'bridgeKey': 'com.volcengine.rtc.hybrid_runtime',
              }));

  /// @detail keytype
  /// @brief 任务 ID
  /// 对于 WTN 流任务，该值代表 WTN 流 ID。你可以通过该 ID，指定需要订阅的 WTN 流。
  FutureOr<NSString?> get taskId async {
    return await sendInstanceGet<NSString?>("taskId");
  }

  set taskId(FutureOr<NSString?> value) {
    sendInstanceSet("taskId", value);
  }

  /// @detail keytype
  /// @brief 任务类型，合流转推 CDN 还是 WTN 流。
  FutureOr<ByteRTCMixedStreamPushTargetType?> get pushTargetType async {
    try {
      final result = await sendInstanceGet<ByteRTCMixedStreamPushTargetType?>(
          "pushTargetType");
      if (result == null) {
        return null;
      }
      return ByteRTCMixedStreamPushTargetType.values
          .firstWhere((element) => element == result);
    } catch (e) {
      return null;
    }
  }

  set pushTargetType(FutureOr<ByteRTCMixedStreamPushTargetType?> value) {
    sendInstanceSet("pushTargetType", value);
  }
}

enum ByteRTCPlayState {
  /// @brief 播放中。
  ///
  ByteRTCPlayStatePlaying(1),

  /// @brief 暂停中。
  ///
  ByteRTCPlayStatePaused(2),

  /// @brief 已停止。
  ///
  ByteRTCPlayStateStoped(3),

  /// @brief 播放失败。
  ///
  ByteRTCPlayStateFailed(4),

  /// @brief 播放结束。
  ///
  ByteRTCPlayStateFinished(5);

  final dynamic $value;
  const ByteRTCPlayState([this.$value]);
}

enum ByteRTCScreenCaptureSourceType {
  /// @brief 类型未知
  ///
  ByteRTCScreenCaptureSourceTypeUnknown(0),

  /// @brief 应用程序的窗口
  ///
  ByteRTCScreenCaptureSourceTypeWindow(1),

  /// @brief 桌面
  ///
  ByteRTCScreenCaptureSourceTypeScreen(2);

  final dynamic $value;
  const ByteRTCScreenCaptureSourceType([this.$value]);
}

class ByteRTCLogConfig extends NativeClass {
  static const _$namespace = r'ByteRTCLogConfig';
  static get codegen_$namespace => _$namespace;

  ByteRTCLogConfig([NativeClassOptions? options])
      : super(options == null
            ? const NativeClassOptions(
                [],
                className: _$namespace,
                instanceType: InstanceType.manual,
                bridgeKey: 'com.volcengine.rtc.hybrid_runtime',
              )
            : NativeClassOptions.fromMap({
                ...options.toMap(),
                'bridgeKey': 'com.volcengine.rtc.hybrid_runtime',
              }));

  /// @brief 日志存储路径，必填。
  FutureOr<NSString?> get logPath async {
    return await sendInstanceGet<NSString?>("logPath");
  }

  set logPath(FutureOr<NSString?> value) {
    sendInstanceSet("logPath", value);
  }

  /// @brief 日志等级，参看 ByteRTCLocalLogLevel{@link #ByteRTCLocalLogLevel}，默认为警告级别，选填。
  FutureOr<ByteRTCLocalLogLevel?> get logLevel async {
    try {
      final result = await sendInstanceGet<ByteRTCLocalLogLevel?>("logLevel");
      if (result == null) {
        return null;
      }
      return ByteRTCLocalLogLevel.values
          .firstWhere((element) => element == result);
    } catch (e) {
      return null;
    }
  }

  set logLevel(FutureOr<ByteRTCLocalLogLevel?> value) {
    sendInstanceSet("logLevel", value);
  }

  /// @brief 日志文件最大占用的总空间，单位为 MB，选填。取值范围为 1～100 MB，默认值为 10 MB。 <br>
  ///        若 `logFileSize` < 1，取 1 MB。若 `logFileSize` > 100，取 100 MB。 <br>
  ///        其中，单个日志文件最大为 2 MB： <br>
  ///        \\</ul>\<li> 若 1 ≤ <code>logFileSize</code> ≤ 2，则会生成一个日志文件。\</li>\<li>若 <code>logFileSize</code> > 2，假设 <code>logFileSize/2</code> 的整数部分为 N，则前 N 个文件，每个文件会写满 2 MB，第 N+1 个文件大小不超过 <code>logFileSize mod 2</code>，否则会删除最老的文件，以此类推。\</li></ul>
  FutureOr<int?> get logFileSize async {
    return await sendInstanceGet<int?>("logFileSize");
  }

  set logFileSize(FutureOr<int?> value) {
    sendInstanceSet("logFileSize", value);
  }

  /// @brief 日志文件名前缀，选填。该字符串必须符合正则表达式：[a-zA-Z0-9_\@\\-\\.]{1,128}。 <br>
  ///        最终的日志文件名为`前缀 + "_" + 文件创建时间 + "_rtclog".log`，如 `logPrefix_2023-05-25_172324_rtclog.log`。
  FutureOr<NSString?> get logFilenamePrefix async {
    return await sendInstanceGet<NSString?>("logFilenamePrefix");
  }

  set logFilenamePrefix(FutureOr<NSString?> value) {
    sendInstanceSet("logFilenamePrefix", value);
  }
}

class GameRoomConfig extends NativeClass {
  static const _$namespace = r'GameRoomConfig';
  static get codegen_$namespace => _$namespace;

  GameRoomConfig([NativeClassOptions? options])
      : super(options == null
            ? const NativeClassOptions(
                [],
                className: _$namespace,
                instanceType: InstanceType.manual,
                bridgeKey: 'com.volcengine.rtc.hybrid_runtime',
              )
            : NativeClassOptions.fromMap({
                ...options.toMap(),
                'bridgeKey': 'com.volcengine.rtc.hybrid_runtime',
              }));

  /// @detail keytype
  /// @brief Game room type. Game room type. See GameRoomType{@link #GameRoomType} for details. The default value is `GAME_RTC_ROOM_TYPE_TEAM` (Team room), and it cannot be changed after joining the room.

  FutureOr<GameRoomType?> get gameRoomType async {
    try {
      final result = await sendInstanceGet<GameRoomType?>("gameRoomType");
      if (result == null) {
        return null;
      }
      return GameRoomType.values.firstWhere((element) => element == result);
    } catch (e) {
      return null;
    }
  }

  set gameRoomType(FutureOr<GameRoomType?> value) {
    sendInstanceSet("gameRoomType", value);
  }

  /// @detail keytype
  /// @brief Game scene type. See GameSceneType{@link #GameSceneType} for details. The default value is `GameSceneTypeNormal` (Normal scene), and it cannot be changed after joining the room.

  FutureOr<GameSceneType?> get gameSceneType async {
    try {
      final result = await sendInstanceGet<GameSceneType?>("gameSceneType");
      if (result == null) {
        return null;
      }
      return GameSceneType.values.firstWhere((element) => element == result);
    } catch (e) {
      return null;
    }
  }

  set gameSceneType(FutureOr<GameSceneType?> value) {
    sendInstanceSet("gameSceneType", value);
  }
}

class ByteRTCVoiceReverbConfig extends NativeClass {
  static const _$namespace = r'ByteRTCVoiceReverbConfig';
  static get codegen_$namespace => _$namespace;

  ByteRTCVoiceReverbConfig([NativeClassOptions? options])
      : super(options == null
            ? const NativeClassOptions(
                [],
                className: _$namespace,
                instanceType: InstanceType.manual,
                bridgeKey: 'com.volcengine.rtc.hybrid_runtime',
              )
            : NativeClassOptions.fromMap({
                ...options.toMap(),
                'bridgeKey': 'com.volcengine.rtc.hybrid_runtime',
              }));

  /// @brief 混响模拟的房间大小，取值范围 `[0.0, 100.0]`。默认值为 `50.0f`。房间越大，混响越强。
  FutureOr<float?> get roomSize async {
    return await sendInstanceGet<float?>("roomSize");
  }

  set roomSize(FutureOr<float?> value) {
    sendInstanceSet("roomSize", value);
  }

  /// @brief 混响的拖尾长度，取值范围 `[0.0, 100.0]`。默认值为 `50.0f`。
  FutureOr<float?> get decayTime async {
    return await sendInstanceGet<float?>("decayTime");
  }

  set decayTime(FutureOr<float?> value) {
    sendInstanceSet("decayTime", value);
  }

  /// @brief 混响的衰减阻尼大小，取值范围 `[0.0, 100.0]`。默认值为 `50.0f`。
  FutureOr<float?> get damping async {
    return await sendInstanceGet<float?>("damping");
  }

  set damping(FutureOr<float?> value) {
    sendInstanceSet("damping", value);
  }

  /// @brief 早期反射信号强度。取值范围 `[-20.0, 10.0]`，单位为 dB。默认值为 `0.0f`。
  FutureOr<float?> get wetGain async {
    return await sendInstanceGet<float?>("wetGain");
  }

  set wetGain(FutureOr<float?> value) {
    sendInstanceSet("wetGain", value);
  }

  /// @brief 原始信号强度。取值范围 `[-20.0, 10.0]`，单位为 dB。默认值为 `0.0f`。
  FutureOr<float?> get dryGain async {
    return await sendInstanceGet<float?>("dryGain");
  }

  set dryGain(FutureOr<float?> value) {
    sendInstanceSet("dryGain", value);
  }

  /// @brief 早期反射信号的延迟。取值范围 `[0.0, 200.0]`，单位为 ms。默认值为 `0.0f`。
  FutureOr<float?> get preDelay async {
    return await sendInstanceGet<float?>("preDelay");
  }

  set preDelay(FutureOr<float?> value) {
    sendInstanceSet("preDelay", value);
  }
}

enum ByteRTCSEICountPerFrame {
  /// @brief 单发模式。即在 1 帧间隔内多次发送 SEI 数据时，多个 SEI 按队列逐帧发送。
  ///
  ByteRTCSEICountPerFrameSingle(0),

  /// @brief 多发模式。即在 1 帧间隔内多次发送 SEI 数据时，多个 SEI 随下个视频帧同时发送。
  ///
  ByteRTCSEICountPerFrameMulti(1);

  final dynamic $value;
  const ByteRTCSEICountPerFrame([this.$value]);
}

enum ByteRTCAudioSelectionPriority {
  /// @brief 正常，参加音频选路
  ///
  ByteRTCAudioSelectionPriorityNormal(0),

  /// @brief 高优先级，跳过音频选路
  ///
  ByteRTCAudioSelectionPriorityHigh(1);

  final dynamic $value;
  const ByteRTCAudioSelectionPriority([this.$value]);
}

class ByteRTCSubtitleConfig extends NativeClass {
  static const _$namespace = r'ByteRTCSubtitleConfig';
  static get codegen_$namespace => _$namespace;

  ByteRTCSubtitleConfig([NativeClassOptions? options])
      : super(options == null
            ? const NativeClassOptions(
                [],
                className: _$namespace,
                instanceType: InstanceType.manual,
                bridgeKey: 'com.volcengine.rtc.hybrid_runtime',
              )
            : NativeClassOptions.fromMap({
                ...options.toMap(),
                'bridgeKey': 'com.volcengine.rtc.hybrid_runtime',
              }));

  /// @brief 字幕模式。可以根据需要选择识别和翻译两种模式。开启识别模式，会将识别后的用户语音转化成文字；开启翻译模式，会在语音识别后进行翻译。参看 ByteRTCSubtitleMode{@link #ByteRTCSubtitleMode}。
  FutureOr<ByteRTCSubtitleMode?> get mode async {
    try {
      final result = await sendInstanceGet<ByteRTCSubtitleMode?>("mode");
      if (result == null) {
        return null;
      }
      return ByteRTCSubtitleMode.values
          .firstWhere((element) => element == result);
    } catch (e) {
      return null;
    }
  }

  set mode(FutureOr<ByteRTCSubtitleMode?> value) {
    sendInstanceSet("mode", value);
  }

  /// @brief 目标翻译语言。可点击 [语言支持](https://www.volcengine.com/docs/4640/35107#\%F0\%9F\%93\%A2\%E5\%AE\%9E\%E6\%97\%B6\%E8\%AF\%AD\%E9\%9F\%B3\%E7\%BF\%BB\%E8\%AF\%91) 查看翻译服务最新支持的语种信息。
  FutureOr<NSString?> get targetLanguage async {
    return await sendInstanceGet<NSString?>("targetLanguage");
  }

  set targetLanguage(FutureOr<NSString?> value) {
    sendInstanceSet("targetLanguage", value);
  }
}

class ByteRTCSingScoringRealtimeInfo extends NativeClass {
  static const _$namespace = r'ByteRTCSingScoringRealtimeInfo';
  static get codegen_$namespace => _$namespace;

  ByteRTCSingScoringRealtimeInfo([NativeClassOptions? options])
      : super(options == null
            ? const NativeClassOptions(
                [],
                className: _$namespace,
                instanceType: InstanceType.manual,
                bridgeKey: 'com.volcengine.rtc.hybrid_runtime',
              )
            : NativeClassOptions.fromMap({
                ...options.toMap(),
                'bridgeKey': 'com.volcengine.rtc.hybrid_runtime',
              }));

  /// @brief 当前的播放进度。
  ///
  FutureOr<int?> get currentPosition async {
    return await sendInstanceGet<int?>("currentPosition");
  }

  set currentPosition(FutureOr<int?> value) {
    sendInstanceSet("currentPosition", value);
  }

  /// @brief 演唱者的音高。
  ///
  FutureOr<int?> get userPitch async {
    return await sendInstanceGet<int?>("userPitch");
  }

  set userPitch(FutureOr<int?> value) {
    sendInstanceSet("userPitch", value);
  }

  /// @brief 标准音高。
  ///
  FutureOr<int?> get standardPitch async {
    return await sendInstanceGet<int?>("standardPitch");
  }

  set standardPitch(FutureOr<int?> value) {
    sendInstanceSet("standardPitch", value);
  }

  /// @brief 歌词分句索引。
  ///
  FutureOr<int?> get sentenceIndex async {
    return await sendInstanceGet<int?>("sentenceIndex");
  }

  set sentenceIndex(FutureOr<int?> value) {
    sendInstanceSet("sentenceIndex", value);
  }

  /// @brief 上一句歌词的评分。
  ///
  FutureOr<int?> get sentenceScore async {
    return await sendInstanceGet<int?>("sentenceScore");
  }

  set sentenceScore(FutureOr<int?> value) {
    sendInstanceSet("sentenceScore", value);
  }

  /// @brief 当前演唱的累计分数。
  ///
  FutureOr<int?> get totalScore async {
    return await sendInstanceGet<int?>("totalScore");
  }

  set totalScore(FutureOr<int?> value) {
    sendInstanceSet("totalScore", value);
  }

  /// @brief 当前演唱的平均分数。
  ///
  FutureOr<int?> get averageScore async {
    return await sendInstanceGet<int?>("averageScore");
  }

  set averageScore(FutureOr<int?> value) {
    sendInstanceSet("averageScore", value);
  }
}

enum ByteRTCVideoRotation {
  /// @brief 不旋转
  ///
  ByteRTCVideoRotation0(0),

  /// @brief 顺时针旋转 90 度
  ///
  ByteRTCVideoRotation90(90),

  /// @brief 顺时针旋转 180 度
  ///
  ByteRTCVideoRotation180(180),

  /// @brief 顺时针旋转 270 度
  ///
  ByteRTCVideoRotation270(270);

  final dynamic $value;
  const ByteRTCVideoRotation([this.$value]);
}

enum ByteRTCMixedStreamSyncStrategy {
  /// @brief 不使用同步策略
  ///
  ByteRTCMixedStreamSyncStrategyNoSync(0),

  /// @brief 使用音频精准同步策略
  ///
  ByteRTCMixedStreamSyncStrategyAudioPreciseSync(1),

  /// @brief 使用单通模式同步策略
  ///
  ByteRTCMixedStreamSyncStrategySimplexModeSync(2);

  final dynamic $value;
  const ByteRTCMixedStreamSyncStrategy([this.$value]);
}

class ByteRTCMusicInfo extends NativeClass {
  static const _$namespace = r'ByteRTCMusicInfo';
  static get codegen_$namespace => _$namespace;

  ByteRTCMusicInfo([NativeClassOptions? options])
      : super(options == null
            ? const NativeClassOptions(
                [],
                className: _$namespace,
                instanceType: InstanceType.manual,
                bridgeKey: 'com.volcengine.rtc.hybrid_runtime',
              )
            : NativeClassOptions.fromMap({
                ...options.toMap(),
                'bridgeKey': 'com.volcengine.rtc.hybrid_runtime',
              }));

  /// @brief 音乐 ID。
  FutureOr<NSString?> get musicId async {
    return await sendInstanceGet<NSString?>("musicId");
  }

  set musicId(FutureOr<NSString?> value) {
    sendInstanceSet("musicId", value);
  }

  /// @brief 音乐名称。
  FutureOr<NSString?> get musicName async {
    return await sendInstanceGet<NSString?>("musicName");
  }

  set musicName(FutureOr<NSString?> value) {
    sendInstanceSet("musicName", value);
  }

  /// @brief 歌手。
  FutureOr<NSString?> get singer async {
    return await sendInstanceGet<NSString?>("singer");
  }

  set singer(FutureOr<NSString?> value) {
    sendInstanceSet("singer", value);
  }

  /// @brief 版权商 ID。
  FutureOr<NSString?> get vendorId async {
    return await sendInstanceGet<NSString?>("vendorId");
  }

  set vendorId(FutureOr<NSString?> value) {
    sendInstanceSet("vendorId", value);
  }

  /// @brief 版权商名称。
  FutureOr<NSString?> get vendorName async {
    return await sendInstanceGet<NSString?>("vendorName");
  }

  set vendorName(FutureOr<NSString?> value) {
    sendInstanceSet("vendorName", value);
  }

  /// @brief 最新更新时间戳，单位为毫秒。
  FutureOr<int64_t?> get updateTimestamp async {
    return await sendInstanceGet<int64_t?>("updateTimestamp");
  }

  set updateTimestamp(FutureOr<int64_t?> value) {
    sendInstanceSet("updateTimestamp", value);
  }

  /// @brief 封面地址。
  FutureOr<NSString?> get posterUrl async {
    return await sendInstanceGet<NSString?>("posterUrl");
  }

  set posterUrl(FutureOr<NSString?> value) {
    sendInstanceSet("posterUrl", value);
  }

  /// @brief 歌词格式类型，参看 ByteRTCLyricStatus{@link #ByteRTCLyricStatus}。
  FutureOr<ByteRTCLyricStatus?> get lyricStatus async {
    try {
      final result = await sendInstanceGet<ByteRTCLyricStatus?>("lyricStatus");
      if (result == null) {
        return null;
      }
      return ByteRTCLyricStatus.values
          .firstWhere((element) => element == result);
    } catch (e) {
      return null;
    }
  }

  set lyricStatus(FutureOr<ByteRTCLyricStatus?> value) {
    sendInstanceSet("lyricStatus", value);
  }

  /// @brief 歌曲长度，单位为毫秒。
  FutureOr<int?> get duration async {
    return await sendInstanceGet<int?>("duration");
  }

  set duration(FutureOr<int?> value) {
    sendInstanceSet("duration", value);
  }

  /// @brief 歌曲是否支持打分。
  FutureOr<BOOL?> get enableScore async {
    return await sendInstanceGet<BOOL?>("enableScore");
  }

  set enableScore(FutureOr<BOOL?> value) {
    sendInstanceSet("enableScore", value);
  }

  /// @brief 歌曲高潮片段开始时间，单位为毫秒。
  FutureOr<int?> get climaxStartTime async {
    return await sendInstanceGet<int?>("climaxStartTime");
  }

  set climaxStartTime(FutureOr<int?> value) {
    sendInstanceSet("climaxStartTime", value);
  }

  /// @brief 歌曲高潮片段停止时间，单位为毫秒。
  FutureOr<int?> get climaxEndTime async {
    return await sendInstanceGet<int?>("climaxEndTime");
  }

  set climaxEndTime(FutureOr<int?> value) {
    sendInstanceSet("climaxEndTime", value);
  }
}

class ByteRTCEncodedVideoFrame extends NativeClass {
  static const _$namespace = r'ByteRTCEncodedVideoFrame';
  static get codegen_$namespace => _$namespace;

  ByteRTCEncodedVideoFrame([NativeClassOptions? options])
      : super(options == null
            ? const NativeClassOptions(
                [],
                className: _$namespace,
                instanceType: InstanceType.manual,
                bridgeKey: 'com.volcengine.rtc.hybrid_runtime',
              )
            : NativeClassOptions.fromMap({
                ...options.toMap(),
                'bridgeKey': 'com.volcengine.rtc.hybrid_runtime',
              }));

  /// @brief 视频编码类型。参看 ByteRTCVideoCodecType{@link #ByteRTCVideoCodecType}
  FutureOr<ByteRTCVideoCodecType?> get codecType async {
    try {
      final result = await sendInstanceGet<ByteRTCVideoCodecType?>("codecType");
      if (result == null) {
        return null;
      }
      return ByteRTCVideoCodecType.values
          .firstWhere((element) => element == result);
    } catch (e) {
      return null;
    }
  }

  set codecType(FutureOr<ByteRTCVideoCodecType?> value) {
    sendInstanceSet("codecType", value);
  }

  /// @brief 视频帧编码类型。参看 ByteRTCVideoPictureType{@link #ByteRTCVideoPictureType}
  FutureOr<ByteRTCVideoPictureType?> get pictureType async {
    try {
      final result =
          await sendInstanceGet<ByteRTCVideoPictureType?>("pictureType");
      if (result == null) {
        return null;
      }
      return ByteRTCVideoPictureType.values
          .firstWhere((element) => element == result);
    } catch (e) {
      return null;
    }
  }

  set pictureType(FutureOr<ByteRTCVideoPictureType?> value) {
    sendInstanceSet("pictureType", value);
  }

  /// @brief 视频采集时间戳，单位：微秒
  FutureOr<SInt64?> get timestampUs async {
    return await sendInstanceGet<SInt64?>("timestampUs");
  }

  set timestampUs(FutureOr<SInt64?> value) {
    sendInstanceSet("timestampUs", value);
  }

  /// @brief 视频编码时间戳，单位：微秒
  FutureOr<SInt64?> get timestampDtsUs async {
    return await sendInstanceGet<SInt64?>("timestampDtsUs");
  }

  set timestampDtsUs(FutureOr<SInt64?> value) {
    sendInstanceSet("timestampDtsUs", value);
  }

  /// @brief 视频帧宽，单位：px
  FutureOr<int?> get width async {
    return await sendInstanceGet<int?>("width");
  }

  set width(FutureOr<int?> value) {
    sendInstanceSet("width", value);
  }

  /// @brief 视频帧高，单位：px
  FutureOr<int?> get height async {
    return await sendInstanceGet<int?>("height");
  }

  set height(FutureOr<int?> value) {
    sendInstanceSet("height", value);
  }

  /// @brief 视频帧旋转角度。参看 ByteRTCVideoRotation{@link #ByteRTCVideoRotation}
  FutureOr<ByteRTCVideoRotation?> get rotation async {
    try {
      final result = await sendInstanceGet<ByteRTCVideoRotation?>("rotation");
      if (result == null) {
        return null;
      }
      return ByteRTCVideoRotation.values
          .firstWhere((element) => element == result);
    } catch (e) {
      return null;
    }
  }

  set rotation(FutureOr<ByteRTCVideoRotation?> value) {
    sendInstanceSet("rotation", value);
  }

  /// @brief 视频帧数据指针地址
  FutureOr<NSData?> get data async {
    return await sendInstanceGet<NSData?>("data");
  }

  set data(FutureOr<NSData?> value) {
    sendInstanceSet("data", value);
  }
}

enum ByteRTCNetworkDetectionStopReason {
  /// @brief 用户主动停止。
  ///
  ByteRTCNetworkDetectionStopReasonUser(0),

  /// @brief 探测超过三分钟。
  ///
  ByteRTCNetworkDetectionStopReasonTimeout(1),

  /// @brief 探测网络连接断开。 <br>
  ///        当超过 12s 没有收到回复，SDK 将断开网络连接，并且不再尝试重连。
  ///
  ByteRTCNetworkDetectionStopReasonConnectionLost(2),

  /// @brief 本地开始推拉流，停止探测。
  ///
  ByteRTCNetworkDetectionStopReasonStreaming(3),

  /// @brief 网络探测失败，内部异常
  ///
  ByteRTCNetworkDetectionStopReasonInnerErr(4);

  final dynamic $value;
  const ByteRTCNetworkDetectionStopReason([this.$value]);
}

enum ByteRTCDownloadFileType {
  /// @brief 音频文件。
  ///
  ByteRTCDownloadFileTypeMusic(1),

  /// @brief KRC 歌词文件。
  ///
  ByteRTCDownloadFileTypeKRC(2),

  /// @brief LRC 歌词文件。
  ///
  ByteRTCDownloadFileTypeLRC(3),

  /// @brief MIDI 文件。
  ///
  ByteRTCDownloadFileTypeMIDI(4);

  final dynamic $value;
  const ByteRTCDownloadFileType([this.$value]);
}

enum ByteRTCReturnStatus {
  /// @brief 成功。
  ///
  ByteRTCReturnStatusSuccess(0),

  /// @brief 失败。
  ///
  ByteRTCReturnStatusFailure(-1),

  /// @brief 参数错误。
  ///
  ByteRTCReturnStatusParameterErr(-2),

  /// @brief 接口状态错误。
  ///
  ByteRTCReturnStatusWrongState(-3),

  /// @brief 失败，用户已在房间内。
  ///
  ByteRTCReturnStatusHasInRoom(-4),

  /// @brief 失败，用户已登录。
  ///
  ByteRTCReturnStatusHasInLogin(-5),

  /// @brief 失败，用户已经在进行通话回路测试中。
  ///
  ByteRTCReturnStatusHasInEchoTest(-6),

  /// @brief 失败，音视频均未采集。
  ///
  ByteRTCReturnStatusNeitherVideoNorAudio(-7),

  /// @brief 失败，该 roomId 已被使用。
  ///
  ByteRTCReturnStatusRoomIdInUse(-8),

  /// @brief 失败，屏幕流不支持。
  ///
  ByteRTCReturnStatusScreenNotSupport(-9),

  /// @brief 失败，不支持该操作。
  ///
  ByteRTCReturnStatusNotSupport(-10),

  /// @brief 失败，资源已占用。
  ///
  ByteRTCReturnStatusResourceOverflow(-11),

  /// @brief 失败，没有音频帧。
  ///
  ByteRTCReturnStatusAudioNoFrame(-101),

  /// @brief 失败，未实现。
  ///
  ByteRTCReturnStatusAudioNotImplemented(-102),

  /// @brief 失败，采集设备无麦克风权限，尝试初始化设备失败。
  ///
  ByteRTCReturnStatusAudioNoPermission(-103),

  /// @brief 失败，设备不存在。当前没有设备或设备被移除时返回该值。
  ///
  ByteRTCReturnStatusAudioDeviceNotExists(-104),

  /// @brief 失败，设备音频格式不支持。
  ///
  ByteRTCReturnStatusAudioDeviceFormatNotSupport(-105),

  /// @brief 失败，系统无可用设备。
  ///
  ByteRTCReturnStatusAudioDeviceNoDevice(-106),

  /// @brief 失败，当前设备不可用，需更换设备。
  ///
  ByteRTCReturnStatusAudioDeviceCannotUse(-107),

  /// @brief 系统错误，设备初始化失败。
  ///
  ByteRTCReturnStatusAudioDeviceInitFailed(-108),

  /// @brief 系统错误，设备开启失败。
  ///
  ByteRTCReturnStatusAudioDeviceStartFailed(-109),

  /// @hidden(iOS)
  /// @brief 暂不支持指定音频进程采集，采集失败。
  ///
  ByteRTCReturnStatusAudioDeviceProcessNotExist(-110),

  /// @brief 失败。底层未初始化，engine 无效。
  ///
  ByteRTCReturnStatusNativeInValid(-201);

  final dynamic $value;
  const ByteRTCReturnStatus([this.$value]);
}

class ByteRTCForwardStreamEventInfo extends NativeClass {
  static const _$namespace = r'ByteRTCForwardStreamEventInfo';
  static get codegen_$namespace => _$namespace;

  ByteRTCForwardStreamEventInfo([NativeClassOptions? options])
      : super(options == null
            ? const NativeClassOptions(
                [],
                className: _$namespace,
                instanceType: InstanceType.manual,
                bridgeKey: 'com.volcengine.rtc.hybrid_runtime',
              )
            : NativeClassOptions.fromMap({
                ...options.toMap(),
                'bridgeKey': 'com.volcengine.rtc.hybrid_runtime',
              }));

  /// @brief 跨房间转发媒体流过程中的发生该事件的目标房间 ID <br>
  ///        空字符串代表所有目标房间
  ///
  FutureOr<NSString?> get roomId async {
    return await sendInstanceGet<NSString?>("roomId");
  }

  set roomId(FutureOr<NSString?> value) {
    sendInstanceSet("roomId", value);
  }

  /// @brief 跨房间转发媒体流过程中该目标房间发生的事件，参看 ByteRTCForwardStreamEvent{@link #ByteRTCForwardStreamEvent}
  ///
  FutureOr<ByteRTCForwardStreamEvent?> get event async {
    try {
      final result = await sendInstanceGet<ByteRTCForwardStreamEvent?>("event");
      if (result == null) {
        return null;
      }
      return ByteRTCForwardStreamEvent.values
          .firstWhere((element) => element == result);
    } catch (e) {
      return null;
    }
  }

  set event(FutureOr<ByteRTCForwardStreamEvent?> value) {
    sendInstanceSet("event", value);
  }
}

class ByteRTCRectangle extends NativeClass {
  static const _$namespace = r'ByteRTCRectangle';
  static get codegen_$namespace => _$namespace;

  ByteRTCRectangle([NativeClassOptions? options])
      : super(options == null
            ? const NativeClassOptions(
                [],
                className: _$namespace,
                instanceType: InstanceType.manual,
                bridgeKey: 'com.volcengine.rtc.hybrid_runtime',
              )
            : NativeClassOptions.fromMap({
                ...options.toMap(),
                'bridgeKey': 'com.volcengine.rtc.hybrid_runtime',
              }));

  /// @brief 矩形区域左上角的 x 坐标。
  FutureOr<int?> get x async {
    return await sendInstanceGet<int?>("x");
  }

  set x(FutureOr<int?> value) {
    sendInstanceSet("x", value);
  }

  /// @brief 矩形区域左上角的 y 坐标。
  FutureOr<int?> get y async {
    return await sendInstanceGet<int?>("y");
  }

  set y(FutureOr<int?> value) {
    sendInstanceSet("y", value);
  }

  /// @brief 矩形宽度，单位：px。
  FutureOr<int?> get width async {
    return await sendInstanceGet<int?>("width");
  }

  set width(FutureOr<int?> value) {
    sendInstanceSet("width", value);
  }

  /// @brief 矩形高度，单位：px。
  FutureOr<int?> get height async {
    return await sendInstanceGet<int?>("height");
  }

  set height(FutureOr<int?> value) {
    sendInstanceSet("height", value);
  }
}

enum ByteRTCZoomConfigType {
  /// @brief 设置缩放系数
  ///
  ByteRTCZoomConfigTypeFocusOffset(0),

  /// @brief 设置移动步长
  ///
  ByteRTCZoomConfigTypeMoveOffset(1);

  final dynamic $value;
  const ByteRTCZoomConfigType([this.$value]);
}

enum ByteRTCAudioRenderType {
  /// @brief 自定义渲染音频
  ///
  ByteRTCAudioRenderTypeExternal(0),

  /// @brief RTC SDK 内部渲染音频
  ///
  ByteRTCAudioRenderTypeInternal(1);

  final dynamic $value;
  const ByteRTCAudioRenderType([this.$value]);
}

class ByteRTCEncodedAudioFrameData extends NativeClass {
  static const _$namespace = r'ByteRTCEncodedAudioFrameData';
  static get codegen_$namespace => _$namespace;

  ByteRTCEncodedAudioFrameData([NativeClassOptions? options])
      : super(options == null
            ? const NativeClassOptions(
                [],
                className: _$namespace,
                instanceType: InstanceType.manual,
                bridgeKey: 'com.volcengine.rtc.hybrid_runtime',
              )
            : NativeClassOptions.fromMap({
                ...options.toMap(),
                'bridgeKey': 'com.volcengine.rtc.hybrid_runtime',
              }));

  /// @hidden for internal use only
  /// @brief 音频编码类型，参看 ByteRTCAudioCodecType{@link #ByteRTCAudioCodecType}。
  FutureOr<ByteRTCAudioCodecType?> get codecType async {
    try {
      final result = await sendInstanceGet<ByteRTCAudioCodecType?>("codecType");
      if (result == null) {
        return null;
      }
      return ByteRTCAudioCodecType.values
          .firstWhere((element) => element == result);
    } catch (e) {
      return null;
    }
  }

  set codecType(FutureOr<ByteRTCAudioCodecType?> value) {
    sendInstanceSet("codecType", value);
  }

  /// @hidden for internal use only
  /// @brief 数据
  FutureOr<NSData?> get buffer async {
    return await sendInstanceGet<NSData?>("buffer");
  }

  set buffer(FutureOr<NSData?> value) {
    sendInstanceSet("buffer", value);
  }

  /// @hidden for internal use only
  /// @brief 音频声道，参看 ByteRTCAudioChannel{@link #ByteRTCAudioChannel}。 <br>
  ///        双声道的情况下，左右声道的音频帧数据以 LRLRLR 形式排布。
  FutureOr<ByteRTCAudioChannel?> get channel async {
    try {
      final result = await sendInstanceGet<ByteRTCAudioChannel?>("channel");
      if (result == null) {
        return null;
      }
      return ByteRTCAudioChannel.values
          .firstWhere((element) => element == result);
    } catch (e) {
      return null;
    }
  }

  set channel(FutureOr<ByteRTCAudioChannel?> value) {
    sendInstanceSet("channel", value);
  }

  /// @hidden for internal use only
  /// @brief 采样率，参看 ByteRTCAudioSampleRate{@link #ByteRTCAudioSampleRate}。
  FutureOr<ByteRTCAudioSampleRate?> get sampleRate async {
    try {
      final result =
          await sendInstanceGet<ByteRTCAudioSampleRate?>("sampleRate");
      if (result == null) {
        return null;
      }
      return ByteRTCAudioSampleRate.values
          .firstWhere((element) => element == result);
    } catch (e) {
      return null;
    }
  }

  set sampleRate(FutureOr<ByteRTCAudioSampleRate?> value) {
    sendInstanceSet("sampleRate", value);
  }

  /// @hidden for internal use only
  /// @brief 数据大小
  FutureOr<int?> get size async {
    return await sendInstanceGet<int?>("size");
  }

  set size(FutureOr<int?> value) {
    sendInstanceSet("size", value);
  }

  /// @hidden for internal use only
  /// @brief 时间戳，单位为微秒。
  FutureOr<long?> get timestampUs async {
    return await sendInstanceGet<long?>("timestampUs");
  }

  set timestampUs(FutureOr<long?> value) {
    sendInstanceSet("timestampUs", value);
  }

  /// @hidden for internal use only
  /// @brief 音频帧时长，单位为毫秒。
  FutureOr<int?> get frameSizeMs async {
    return await sendInstanceGet<int?>("frameSizeMs");
  }

  set frameSizeMs(FutureOr<int?> value) {
    sendInstanceSet("frameSizeMs", value);
  }

  /// @hidden for internal use only
  /// @brief 额外信息数据
  FutureOr<NSData?> get extraInfo async {
    return await sendInstanceGet<NSData?>("extraInfo");
  }

  set extraInfo(FutureOr<NSData?> value) {
    sendInstanceSet("extraInfo", value);
  }
}

enum ByteRTCVideoEncoderPreference {
  /// @brief 无偏好。不降低帧率和分辨率。
  ///
  ByteRTCVideoEncoderPreferenceDisabled(0),

  /// @brief 优先保障帧率。适用于动态画面。
  ///
  ByteRTCVideoEncoderPreferenceMaintainFramerate(1),

  /// @brief 清晰模式，优先保障分辨率。适用于静态画面。
  ///
  ByteRTCVideoEncoderPreferenceMaintainQuality(2),

  /// @brief 平衡帧率与分辨率。
  /// 对于屏幕流来说是无偏好。不降低帧率和分辨率。
  ///
  ByteRTCVideoEncoderPreferenceAuto(3);

  final dynamic $value;
  const ByteRTCVideoEncoderPreference([this.$value]);
}

enum ByteRTCMediaPlayerCustomSourceMode {
  /// @brief 当播放来自本地的 PCM 数据时，使用此选项。
  ///
  ByteRTCMediaPlayerCustomSourceModePush(0),

  /// @brief 当播放来自内存的音频数据时，使用此选项。
  ///
  ByteRTCMediaPlayerCustomSourceModePull(1);

  final dynamic $value;
  const ByteRTCMediaPlayerCustomSourceMode([this.$value]);
}

class ByteRTCSubtitleMessage extends NativeClass {
  static const _$namespace = r'ByteRTCSubtitleMessage';
  static get codegen_$namespace => _$namespace;

  ByteRTCSubtitleMessage([NativeClassOptions? options])
      : super(options == null
            ? const NativeClassOptions(
                [],
                className: _$namespace,
                instanceType: InstanceType.manual,
                bridgeKey: 'com.volcengine.rtc.hybrid_runtime',
              )
            : NativeClassOptions.fromMap({
                ...options.toMap(),
                'bridgeKey': 'com.volcengine.rtc.hybrid_runtime',
              }));

  /// @brief 说话者的用户 ID。
  FutureOr<NSString?> get userId async {
    return await sendInstanceGet<NSString?>("userId");
  }

  set userId(FutureOr<NSString?> value) {
    sendInstanceSet("userId", value);
  }

  /// @brief 语音识别或翻译后的文本, 采用 UTF-8 编码。
  FutureOr<NSString?> get text async {
    return await sendInstanceGet<NSString?>("text");
  }

  set text(FutureOr<NSString?> value) {
    sendInstanceSet("text", value);
  }

  /// @brief 字幕语种，根据字幕模式为原文或译文对应的语种。
  FutureOr<NSString?> get language async {
    return await sendInstanceGet<NSString?>("language");
  }

  set language(FutureOr<NSString?> value) {
    sendInstanceSet("language", value);
  }

  /// @brief 字幕模式，参看 ByteRTCSubtitleMode{@link #ByteRTCSubtitleMode}。
  FutureOr<ByteRTCSubtitleMode?> get mode async {
    try {
      final result = await sendInstanceGet<ByteRTCSubtitleMode?>("mode");
      if (result == null) {
        return null;
      }
      return ByteRTCSubtitleMode.values
          .firstWhere((element) => element == result);
    } catch (e) {
      return null;
    }
  }

  set mode(FutureOr<ByteRTCSubtitleMode?> value) {
    sendInstanceSet("mode", value);
  }

  /// @brief 语音识别或翻译后形成的文本的序列号，同一发言人的完整发言和不完整发言会按递增顺序单独分别编号。
  FutureOr<NSInteger?> get sequence async {
    return await sendInstanceGet<NSInteger?>("sequence");
  }

  set sequence(FutureOr<NSInteger?> value) {
    sendInstanceSet("sequence", value);
  }

  /// @brief 语音识别出的文本是否为一段完整的一句话。 True 代表是, False 代表不是。
  FutureOr<BOOL?> get definite async {
    return await sendInstanceGet<BOOL?>("definite");
  }

  set definite(FutureOr<BOOL?> value) {
    sendInstanceSet("definite", value);
  }
}

enum ByteRTCDownloadLyricType {
  /// @brief KRC 歌词文件。
  ///
  ByteRTCDownloadLyricTypeKRC(0),

  /// @brief LRC 歌词文件。
  ///
  ByteRTCDownloadLyricTypeLRC(1);

  final dynamic $value;
  const ByteRTCDownloadLyricType([this.$value]);
}

class ByteRTCMixedStreamControlConfig extends NativeClass {
  static const _$namespace = r'ByteRTCMixedStreamControlConfig';
  static get codegen_$namespace => _$namespace;

  ByteRTCMixedStreamControlConfig([NativeClassOptions? options])
      : super(options == null
            ? const NativeClassOptions(
                [],
                className: _$namespace,
                instanceType: InstanceType.manual,
                bridgeKey: 'com.volcengine.rtc.hybrid_runtime',
              )
            : NativeClassOptions.fromMap({
                ...options.toMap(),
                'bridgeKey': 'com.volcengine.rtc.hybrid_runtime',
              }));

  /// @valid since 3.56
  /// @brief 是否开启单独发送声音提示 SEI 的功能： <br>
  ///        - True：开启；
  ///        - False：关闭。（默认值）
  ///        开启后，你可以通过 `ByteRTCMixedStreamControlConfig.seiContentMode` 控制 SEI 的内容是否只携带声音信息。
  FutureOr<BOOL?> get enableVolumeIndication async {
    return await sendInstanceGet<BOOL?>("enableVolumeIndication");
  }

  set enableVolumeIndication(FutureOr<BOOL?> value) {
    sendInstanceSet("enableVolumeIndication", value);
  }

  /// @valid since 3.56
  /// @brief 声音提示间隔，单位为秒，取值范围为 [0.3,+∞)，默认值为 2。 <br>
  ///        此值仅取整百毫秒。若传入两位及以上小数，则四舍五入取第一位小数的值。例如，若传入 0.36，则取 0.4。
  FutureOr<CGFloat?> get volumeIndicationInterval async {
    return await sendInstanceGet<CGFloat?>("volumeIndicationInterval");
  }

  set volumeIndicationInterval(FutureOr<CGFloat?> value) {
    sendInstanceSet("volumeIndicationInterval", value);
  }

  /// @valid since 3.56
  /// @brief 有效音量大小，取值范围为 [0, 255]，默认值为 0。 <br>
  ///        超出取值范围则自动调整为默认值，即 0。
  FutureOr<NSInteger?> get talkVolume async {
    return await sendInstanceGet<NSInteger?>("talkVolume");
  }

  set talkVolume(FutureOr<NSInteger?> value) {
    sendInstanceSet("talkVolume", value);
  }

  /// @valid since 3.56
  /// @brief 声音信息 SEI 是否包含音量值： <br>
  ///        - True：是；
  ///        - False：否，默认值。
  FutureOr<BOOL?> get isAddVolumeValue async {
    return await sendInstanceGet<BOOL?>("isAddVolumeValue");
  }

  set isAddVolumeValue(FutureOr<BOOL?> value) {
    sendInstanceSet("isAddVolumeValue", value);
  }

  /// @valid since 3.56
  /// @brief 设置 SEI 内容。参看 ByteRTCMixedStreamSEIContentMode{@link #ByteRTCMixedStreamSEIContentMode}。
  FutureOr<ByteRTCMixedStreamSEIContentMode?> get seiContentMode async {
    try {
      final result = await sendInstanceGet<ByteRTCMixedStreamSEIContentMode?>(
          "seiContentMode");
      if (result == null) {
        return null;
      }
      return ByteRTCMixedStreamSEIContentMode.values
          .firstWhere((element) => element == result);
    } catch (e) {
      return null;
    }
  }

  set seiContentMode(FutureOr<ByteRTCMixedStreamSEIContentMode?> value) {
    sendInstanceSet("seiContentMode", value);
  }

  /// @valid since 3.56
  /// @brief SEI 信息的 payload type。 <br>
  ///        默认值为 `100`，只支持设置 `5` 和 `100`。 <br>
  ///        在转推直播的过程中，该参数不支持变更。
  FutureOr<NSInteger?> get seiPayloadType async {
    return await sendInstanceGet<NSInteger?>("seiPayloadType");
  }

  set seiPayloadType(FutureOr<NSInteger?> value) {
    sendInstanceSet("seiPayloadType", value);
  }

  /// @valid since 3.56
  /// @brief SEI 信息的 payload UUID。
  /// @note PayloadType 为 `5` 时，必须填写 PayloadUUID，否则会收到错误回调，错误码为 1091。 <br>
  ///         PayloadType 不是 `5` 时，不需要填写 PayloadUUID，如果填写会被后端忽略。 <br>
  ///         该参数长度需为 32 位，否则会收到错误码为 1091 的回调。 <br>
  ///         该参数每个字符的范围需为 [0, 9] [a, f] [A, F] <br>
  ///         该参数不应带有`-`字符，如系统自动生成的 UUID 中带有`-`，则应删去。 <br>
  ///         在转推直播的过程中，该参数不支持变更。
  FutureOr<NSString?> get seiPayloadUUID async {
    return await sendInstanceGet<NSString?>("seiPayloadUUID");
  }

  set seiPayloadUUID(FutureOr<NSString?> value) {
    sendInstanceSet("seiPayloadUUID", value);
  }

  /// @valid since 3.57
  /// @brief 设置合流推到 CDN 时输出的媒体流类型。参看 ByteRTCMixedStreamMediaType{@link #ByteRTCMixedStreamMediaType}。 <br>
  ///        默认输出音视频流。支持输出纯音频流，但暂不支持输出纯视频流。
  FutureOr<ByteRTCMixedStreamMediaType?> get mediaType async {
    try {
      final result =
          await sendInstanceGet<ByteRTCMixedStreamMediaType?>("mediaType");
      if (result == null) {
        return null;
      }
      return ByteRTCMixedStreamMediaType.values
          .firstWhere((element) => element == result);
    } catch (e) {
      return null;
    }
  }

  set mediaType(FutureOr<ByteRTCMixedStreamMediaType?> value) {
    sendInstanceSet("mediaType", value);
  }

  /// @valid since 3.57
  /// @brief 设置是否在没有用户发布流的情况下发起转推直播。具体参看 ByteRTCMixedStreamPushMode{@link #ByteRTCMixedStreamPushMode}。 <br>
  ///        该参数在发起合流任务后的转推直播过程中不支持动态变更。
  FutureOr<ByteRTCMixedStreamPushMode?> get pushStreamMode async {
    try {
      final result =
          await sendInstanceGet<ByteRTCMixedStreamPushMode?>("pushStreamMode");
      if (result == null) {
        return null;
      }
      return ByteRTCMixedStreamPushMode.values
          .firstWhere((element) => element == result);
    } catch (e) {
      return null;
    }
  }

  set pushStreamMode(FutureOr<ByteRTCMixedStreamPushMode?> value) {
    sendInstanceSet("pushStreamMode", value);
  }
}

class ByteRTCMixedStreamConfig extends NativeClass {
  static const _$namespace = r'ByteRTCMixedStreamConfig';
  static get codegen_$namespace => _$namespace;

  /// @brief 获取默认转推直播配置参数。
  /// @return 转推直播配置参数，参看 ByteRTCMixedStreamConfig{@link #ByteRTCMixedStreamConfig}。

  static FutureOr<ByteRTCMixedStreamConfig> defaultMixedStreamConfig() async {
    try {
      final result = await NativeClassUtils.nativeStaticCall(
        _$namespace,
        'defaultMixedStreamConfig',
        [],
        'com.volcengine.rtc.hybrid_runtime',
      );
      return packObject(
          result,
          () => ByteRTCMixedStreamConfig(
              const NativeClassOptions([], disableInit: true)));
    } catch (e) {
      rethrow;
    }
  }

  ByteRTCMixedStreamConfig([NativeClassOptions? options])
      : super(options == null
            ? const NativeClassOptions(
                [],
                className: _$namespace,
                instanceType: InstanceType.manual,
                bridgeKey: 'com.volcengine.rtc.hybrid_runtime',
              )
            : NativeClassOptions.fromMap({
                ...options.toMap(),
                'bridgeKey': 'com.volcengine.rtc.hybrid_runtime',
              }));

  /// @brief 合流背景颜色，用十六进制颜色码（HEX）表示。例如，#FFFFFF 表示纯白，#000000 表示纯黑。默认值为 #000000。建议设置。 <br>
  ///        值不合法或未设置时，自动使用默认值。
  FutureOr<NSString?> get backgroundColor async {
    return await sendInstanceGet<NSString?>("backgroundColor");
  }

  set backgroundColor(FutureOr<NSString?> value) {
    sendInstanceSet("backgroundColor", value);
  }

  /// @brief 用户布局信息列表。每条流的具体布局参看 ByteRTCMixedStreamLayoutRegionConfig{@link #ByteRTCMixedStreamLayoutRegionConfig}。建议设置。 <br>
  ///        值不合法或未设置时，自动使用默认值。
  FutureOr<NSArray<ByteRTCMixedStreamLayoutRegionConfig>?> get regions async {
    try {
      final result =
          await sendInstanceGet<NSArray<ByteRTCMixedStreamLayoutRegionConfig>?>(
              "regions");
      if (result == null) {
        return null;
      }
      final list = result.map((e) => packObject(
          e,
          () => ByteRTCMixedStreamLayoutRegionConfig(
              const NativeClassOptions([], disableInit: true))));
      return list.toList();
    } catch (e) {
      return null;
    }
  }

  set regions(FutureOr<NSArray<ByteRTCMixedStreamLayoutRegionConfig>?> value) {
    sendInstanceSet("regions", value);
  }

  /// @brief 视频转码参数。详见 ByteRTCMixedStreamVideoConfig{@link #ByteRTCMixedStreamVideoConfig} 数据类型。建议设置。
  FutureOr<ByteRTCMixedStreamVideoConfig?> get videoConfig async {
    try {
      final result =
          await sendInstanceGet<ByteRTCMixedStreamVideoConfig?>("videoConfig");
      if (result == null) {
        return null;
      }
      return packObject(
          result,
          () => ByteRTCMixedStreamVideoConfig(
              const NativeClassOptions([], disableInit: true)));
    } catch (e) {
      return null;
    }
  }

  set videoConfig(FutureOr<ByteRTCMixedStreamVideoConfig?> value) {
    sendInstanceSet("videoConfig", value);
  }

  /// @brief 音频合流参数，参看 ByteRTCMixedStreamAudioConfig{@link #ByteRTCMixedStreamAudioConfig}。建议设置。 <br>
  ///      - 本参数不支持过程中更新。
  ///      - WTN 流任务不支持设置本参数。
  FutureOr<ByteRTCMixedStreamAudioConfig?> get audioConfig async {
    try {
      final result =
          await sendInstanceGet<ByteRTCMixedStreamAudioConfig?>("audioConfig");
      if (result == null) {
        return null;
      }
      return packObject(
          result,
          () => ByteRTCMixedStreamAudioConfig(
              const NativeClassOptions([], disableInit: true)));
    } catch (e) {
      return null;
    }
  }

  set audioConfig(FutureOr<ByteRTCMixedStreamAudioConfig?> value) {
    sendInstanceSet("audioConfig", value);
  }

  /// @hidden(macOS)
  /// @brief 转推 CDN 空间音频配置。详见 ByteRTCMixedStreamSpatialAudioConfig{@link #ByteRTCMixedStreamSpatialAudioConfig} 。
  FutureOr<ByteRTCMixedStreamSpatialAudioConfig?> get spatialAudioConfig async {
    try {
      final result =
          await sendInstanceGet<ByteRTCMixedStreamSpatialAudioConfig?>(
              "spatialAudioConfig");
      if (result == null) {
        return null;
      }
      return packObject(
          result,
          () => ByteRTCMixedStreamSpatialAudioConfig(
              const NativeClassOptions([], disableInit: true)));
    } catch (e) {
      return null;
    }
  }

  set spatialAudioConfig(
      FutureOr<ByteRTCMixedStreamSpatialAudioConfig?> value) {
    sendInstanceSet("spatialAudioConfig", value);
  }

  /// @brief 服务端合流控制参数。详见 ByteRTCMixedStreamControlConfig{@link #ByteRTCMixedStreamControlConfig} 。
  FutureOr<ByteRTCMixedStreamControlConfig?> get controlConfig async {
    try {
      final result = await sendInstanceGet<ByteRTCMixedStreamControlConfig?>(
          "controlConfig");
      if (result == null) {
        return null;
      }
      return packObject(
          result,
          () => ByteRTCMixedStreamControlConfig(
              const NativeClassOptions([], disableInit: true)));
    } catch (e) {
      return null;
    }
  }

  set controlConfig(FutureOr<ByteRTCMixedStreamControlConfig?> value) {
    sendInstanceSet("controlConfig", value);
  }

  /// @hidden for internal use only
  /// @brief 动态扩展自定义参数。
  FutureOr<NSMutableDictionary?> get advancedConfig async {
    return await sendInstanceGet<NSMutableDictionary?>("advancedConfig");
  }

  set advancedConfig(FutureOr<NSMutableDictionary?> value) {
    sendInstanceSet("advancedConfig", value);
  }

  /// @hidden for internal use only
  /// @brief 业务透传鉴权信息
  FutureOr<NSMutableDictionary?> get authInfo async {
    return await sendInstanceGet<NSMutableDictionary?>("authInfo");
  }

  set authInfo(FutureOr<NSMutableDictionary?> value) {
    sendInstanceSet("authInfo", value);
  }

  /// @hidden for internal use only
  /// @brief 推流 CDN 地址。仅支持 RTMP 协议，Url 必须满足正则 `/^rtmps?:\\/\\//`。建议设置。 <br>
  ///      - 本参数不支持过程中更新。
  ///      - WTN 流任务不支持设置本参数。
  FutureOr<NSString?> get pushURL async {
    return await sendInstanceGet<NSString?>("pushURL");
  }

  set pushURL(FutureOr<NSString?> value) {
    sendInstanceSet("pushURL", value);
  }

  /// @brief 推流房间 ID。`roomID` 和 `userID` 长度相加不得超过 126 字节。建议设置。 <br>
  ///        本参数不支持过程中更新。
  FutureOr<NSString?> get roomID async {
    return await sendInstanceGet<NSString?>("roomID");
  }

  set roomID(FutureOr<NSString?> value) {
    sendInstanceSet("roomID", value);
  }

  /// @brief 推流用户 ID。`roomID` 和 `userID` 长度相加不得超过 126 字节。建议设置。 <br>
  ///        本参数不支持过程中更新。
  FutureOr<NSString?> get userID async {
    return await sendInstanceGet<NSString?>("userID");
  }

  set userID(FutureOr<NSString?> value) {
    sendInstanceSet("userID", value);
  }

  /// @brief 用户配置的额外数据。 <br>
  /// WTN 流任务不支持设置本参数。
  FutureOr<NSString?> get userConfigExtraInfo async {
    return await sendInstanceGet<NSString?>("userConfigExtraInfo");
  }

  set userConfigExtraInfo(FutureOr<NSString?> value) {
    sendInstanceSet("userConfigExtraInfo", value);
  }

  /// @valid since 3.57
  /// @brief 设置合流后整体画布的背景图片 URL，长度最大为 1024 bytes。 <br>
  ///        支持的图片格式包括：JPG, JPEG, PNG。如果背景图片的宽高和整体屏幕的宽高不一致，背景图片会缩放到铺满屏幕。
  FutureOr<NSString?> get backgroundImageURL async {
    return await sendInstanceGet<NSString?>("backgroundImageURL");
  }

  set backgroundImageURL(FutureOr<NSString?> value) {
    sendInstanceSet("backgroundImageURL", value);
  }

  /// @brief WTN 流流布局模式。参看 ByteRTCStreamLayoutMode{@link #ByteRTCStreamLayoutMode}。可选： <br>
  ///        - `auto`: 自动布局。默认值
  ///        - `custom`: 自定义布局。
  FutureOr<ByteRTCStreamLayoutMode?> get layoutMode async {
    try {
      final result =
          await sendInstanceGet<ByteRTCStreamLayoutMode?>("layoutMode");
      if (result == null) {
        return null;
      }
      return ByteRTCStreamLayoutMode.values
          .firstWhere((element) => element == result);
    } catch (e) {
      return null;
    }
  }

  set layoutMode(FutureOr<ByteRTCStreamLayoutMode?> value) {
    sendInstanceSet("layoutMode", value);
  }

  /// @brief WTN 流的补帧模式。参看 ByteRTCInterpolationMode{@link #ByteRTCInterpolationMode}。可选：
  FutureOr<ByteRTCInterpolationMode?> get interpolationMode async {
    try {
      final result =
          await sendInstanceGet<ByteRTCInterpolationMode?>("interpolationMode");
      if (result == null) {
        return null;
      }
      return ByteRTCInterpolationMode.values
          .firstWhere((element) => element == result);
    } catch (e) {
      return null;
    }
  }

  set interpolationMode(FutureOr<ByteRTCInterpolationMode?> value) {
    sendInstanceSet("interpolationMode", value);
  }

  /// @brief 任务类型。参看 ByteRTCMixedStreamPushTargetType{@link #ByteRTCMixedStreamPushTargetType}。可选：
  FutureOr<ByteRTCMixedStreamPushTargetType?> get pushTargetType async {
    try {
      final result = await sendInstanceGet<ByteRTCMixedStreamPushTargetType?>(
          "pushTargetType");
      if (result == null) {
        return null;
      }
      return ByteRTCMixedStreamPushTargetType.values
          .firstWhere((element) => element == result);
    } catch (e) {
      return null;
    }
  }

  set pushTargetType(FutureOr<ByteRTCMixedStreamPushTargetType?> value) {
    sendInstanceSet("pushTargetType", value);
  }
}

class ByteRTCMixedStreamSyncControlConfig extends NativeClass {
  static const _$namespace = r'ByteRTCMixedStreamSyncControlConfig';
  static get codegen_$namespace => _$namespace;

  ByteRTCMixedStreamSyncControlConfig([NativeClassOptions? options])
      : super(options == null
            ? const NativeClassOptions(
                [],
                className: _$namespace,
                instanceType: InstanceType.manual,
                bridgeKey: 'com.volcengine.rtc.hybrid_runtime',
              )
            : NativeClassOptions.fromMap({
                ...options.toMap(),
                'bridgeKey': 'com.volcengine.rtc.hybrid_runtime',
              }));

  /// @brief 同步策略
  FutureOr<ByteRTCMixedStreamSyncStrategy?> get syncStrategy async {
    try {
      final result = await sendInstanceGet<ByteRTCMixedStreamSyncStrategy?>(
          "syncStrategy");
      if (result == null) {
        return null;
      }
      return ByteRTCMixedStreamSyncStrategy.values
          .firstWhere((element) => element == result);
    } catch (e) {
      return null;
    }
  }

  set syncStrategy(FutureOr<ByteRTCMixedStreamSyncStrategy?> value) {
    sendInstanceSet("syncStrategy", value);
  }

  /// @brief 在进行同步处理时，基准流所属用户的 ID。默认为空。
  FutureOr<NSString?> get baseUserID async {
    return await sendInstanceGet<NSString?>("baseUserID");
  }

  set baseUserID(FutureOr<NSString?> value) {
    sendInstanceSet("baseUserID", value);
  }

  /// @brief 在进行同步处理时，缓存音视频流的最大长度。单位为毫秒。默认值为 2000。 <br>
  ///        参与转推直播的这些媒体流延迟越高，应该将此值设置的越大。但此值越大，因缓存媒体流造成的内存占用也会更大。推荐值为 `2000`。
  FutureOr<NSInteger?> get maxCacheTimeMs async {
    return await sendInstanceGet<NSInteger?>("maxCacheTimeMs");
  }

  set maxCacheTimeMs(FutureOr<NSInteger?> value) {
    sendInstanceSet("maxCacheTimeMs", value);
  }

  /// @brief 是否通过 RTC SDK 进行转推直播。默认为 True。 <br>
  ///        如果选择 `False`，你会通过 onCacheSyncVideo:withDataFrame:withUids:taskId:{@link #ByteRTCClientMixedStreamDelegate#onCacheSyncVideo:withDataFrame:withUids:taskId} 收到同步的帧，你可以使用此视频帧，自行实现合流转推。
  FutureOr<BOOL?> get videoNeedSdkMix async {
    return await sendInstanceGet<BOOL?>("videoNeedSdkMix");
  }

  set videoNeedSdkMix(FutureOr<BOOL?> value) {
    sendInstanceSet("videoNeedSdkMix", value);
  }
}

enum ByteRTCMixedStreamRenderMode {
  /// @brief 视窗填满优先，默认值。 <br>
  ///        视频尺寸等比缩放，直至视窗被填满。当视频尺寸与显示窗口尺寸不一致时，多出的视频将被截掉。
  ///
  ByteRTCMixedStreamRenderModeHidden(1),

  /// @brief 视频帧内容全部显示优先。 <br>
  ///        视频尺寸等比缩放，优先保证视频内容全部显示。当视频尺寸与显示窗口尺寸不一致时，会把窗口未被填满的区域填充成背景颜色。
  ///
  ByteRTCMixedStreamRenderModeFit(2),

  /// @brief 视频帧自适应画布。 <br>
  ///        视频尺寸非等比例缩放，把窗口充满。在此过程中，视频帧的长宽比例可能会发生变化。
  ///
  ByteRTCMixedStreamRenderModeAdaptive(3);

  final dynamic $value;
  const ByteRTCMixedStreamRenderMode([this.$value]);
}

class ByteRTCProblemFeedbackRoomInfo extends NativeClass {
  static const _$namespace = r'ByteRTCProblemFeedbackRoomInfo';
  static get codegen_$namespace => _$namespace;

  ByteRTCProblemFeedbackRoomInfo([NativeClassOptions? options])
      : super(options == null
            ? const NativeClassOptions(
                [],
                className: _$namespace,
                instanceType: InstanceType.manual,
                bridgeKey: 'com.volcengine.rtc.hybrid_runtime',
              )
            : NativeClassOptions.fromMap({
                ...options.toMap(),
                'bridgeKey': 'com.volcengine.rtc.hybrid_runtime',
              }));

  /// @brief 通话质量反馈的房间 ID
  ///
  FutureOr<NSString?> get roomId async {
    return await sendInstanceGet<NSString?>("roomId");
  }

  set roomId(FutureOr<NSString?> value) {
    sendInstanceSet("roomId", value);
  }

  /// @brief 通话质量反馈的用户 ID
  ///
  FutureOr<NSString?> get userId async {
    return await sendInstanceGet<NSString?>("userId");
  }

  set userId(FutureOr<NSString?> value) {
    sendInstanceSet("userId", value);
  }
}

enum ByteRTCEffectBeautyMode {
  /// @brief 美白。
  ///
  ByteRTCEffectBeautyModeWhite(0),

  /// @brief 磨皮。
  ///
  ByteRTCEffectBeautyModeSmooth(1),

  /// @brief 锐化。
  ///
  ByteRTCEffectBeautyModeSharpen(2),

  /// @valid since 3.55
  /// @brief 清晰，需集成 v4.4.2+ 版本的特效 SDK。
  ///
  ByteRTCEffectBeautyModeClear(3);

  final dynamic $value;
  const ByteRTCEffectBeautyMode([this.$value]);
}

enum ByteRTCNetworkType {
  /// @brief 网络连接类型未知。
  ///
  ByteRTCNetworkTypeUnknown(-1),

  /// @brief 网络连接已断开。
  ///
  ByteRTCNetworkTypeDisconnected(0),

  /// @brief 网络连接类型为 LAN 。
  ///
  ByteRTCNetworkTypeLAN(1),

  /// @brief 网络连接类型为 Wi-Fi（包含热点）。
  ///
  ByteRTCNetworkTypeWIFI(2),

  /// @brief 网络连接类型为 2G 移动网络。
  ///
  ByteRTCNetworkTypeMobile2G(3),

  /// @brief 网络连接类型为 3G 移动网络。
  ///
  ByteRTCNetworkTypeMobile3G(4),

  /// @brief 网络连接类型为 4G 移动网络。
  ///
  ByteRTCNetworkTypeMobile4G(5),

  /// @brief 网络连接类型为 5G 移动网络。
  ///
  ByteRTCNetworkTypeMobile5G(6);

  final dynamic $value;
  const ByteRTCNetworkType([this.$value]);
}

enum ByteRTCForwardStreamError {
  /// @brief 正常
  ///
  ByteRTCForwardStreamErrorOK(0),

  /// @brief 参数异常
  ///
  ByteRTCForwardStreamErrorInvalidArgument(1201),

  /// @brief Token 错误
  ///
  ByteRTCForwardStreamErrorInvalidToken(1202),

  /// @brief 服务端异常
  ///
  ByteRTCForwardStreamErrorResponse(1203),

  /// @brief 目标房间有相同 user id 的用户加入，转发中断
  ///
  ByteRTCForwardStreamErrorRemoteKicked(1204),

  /// @brief 服务端不支持转发功能
  ///
  ByteRTCForwardStreamErrorNotSupport(1205);

  final dynamic $value;
  const ByteRTCForwardStreamError([this.$value]);
}

enum ByteRTCSubscribeState {
  /// @brief 订阅成功
  ///
  ByteRTCSubscribeStateSubscribe(0),

  /// @brief 订阅失败
  ///
  ByteRTCSubscribeStateUnsubscribe(1);

  final dynamic $value;
  const ByteRTCSubscribeState([this.$value]);
}

enum ByteRTCMixedStreamClientMixVideoFormat {
  /// @brief YUV I420。Android、Windows 默认回调格式。支持系统：Android、Windows。
  ///
  ByteRTCMixedStreamClientMixVideoFormatI420(0),

  /// @brief OpenGL GL_TEXTURE_2D 格式纹理。支持系统：安卓。
  ///
  ByteRTCMixedStreamClientMixVideoFormatTexture2D(1),

  /// @brief CVPixelBuffer BGRA。iOS 默认回调格式。支持系统: iOS。
  ///
  ByteRTCMixedStreamClientMixVideoFormatCVPixelBufferBGRA(2),

  /// @brief YUV NV12。macOS 默认回调格式。支持系统: macOS。
  ///
  ByteRTCMixedStreamClientMixVideoFormatNV12(3);

  final dynamic $value;
  const ByteRTCMixedStreamClientMixVideoFormat([this.$value]);
}

enum ByteRTCAudioFrameMethod {
  /// @brief 本地采集的音频。
  ///
  ByteRTCAudioFrameProcessorRecord(0),

  /// @brief 远端音频流的混音音频。
  ///
  ByteRTCAudioFrameProcessorPlayback(1),

  /// @brief 各个远端音频流。
  ///
  ByteRTCAudioFrameProcessorRemoteUser(2),

  /// @hidden(macOS)
  /// @brief 软件耳返音频。
  ///
  ByteRTCAudioFrameProcessorEarMonitor(3),

  /// @brief 屏幕共享音频。
  ///
  ByteRTCAudioFrameProcessorScreen(4);

  final dynamic $value;
  const ByteRTCAudioFrameMethod([this.$value]);
}

class ByteRTCVideoByteWatermark extends NativeClass {
  static const _$namespace = r'ByteRTCVideoByteWatermark';
  static get codegen_$namespace => _$namespace;

  ByteRTCVideoByteWatermark([NativeClassOptions? options])
      : super(options == null
            ? const NativeClassOptions(
                [],
                className: _$namespace,
                instanceType: InstanceType.manual,
                bridgeKey: 'com.volcengine.rtc.hybrid_runtime',
              )
            : NativeClassOptions.fromMap({
                ...options.toMap(),
                'bridgeKey': 'com.volcengine.rtc.hybrid_runtime',
              }));

  /// @brief 水印图片相对视频流左上角的横向偏移与视频流宽度的比值，取值范围为 [0,1)。
  FutureOr<float?> get x async {
    return await sendInstanceGet<float?>("x");
  }

  set x(FutureOr<float?> value) {
    sendInstanceSet("x", value);
  }

  /// @brief 水印图片相对视频流左上角的纵向偏移与视频流高度的比值，取值范围为 [0,1)。
  FutureOr<float?> get y async {
    return await sendInstanceGet<float?>("y");
  }

  set y(FutureOr<float?> value) {
    sendInstanceSet("y", value);
  }

  /// @brief 水印图片宽度与视频流宽度的比值，取值范围 [0,1)。
  FutureOr<float?> get width async {
    return await sendInstanceGet<float?>("width");
  }

  set width(FutureOr<float?> value) {
    sendInstanceSet("width", value);
  }

  /// @brief 水印图片高度与视频流高度的比值，取值范围为 [0,1)。
  FutureOr<float?> get height async {
    return await sendInstanceGet<float?>("height");
  }

  set height(FutureOr<float?> value) {
    sendInstanceSet("height", value);
  }
}

enum ByteRTCLocalVideoStreamError {
  /// @brief 本地视频状态正常（本地视频状态改变正常时默认返回值）
  ///
  ByteRTCLocalVideoStreamErrorOk(0),

  /// @brief 本地视频流发布失败
  ///
  ByteRTCLocalVideoStreamErrorFailure(1),

  /// @brief 没有权限启动本地视频采集设备
  ///
  ByteRTCLocalVideoStreamErrorDeviceNoPermission(2),

  /// @brief 本地视频采集设备已被占用
  ///
  ByteRTCLocalVideoStreamErrorDeviceBusy(3),

  /// @brief 本地视频采集设备不存在或已移除
  ///
  ByteRTCLocalVideoStreamErrorDeviceNotFound(4),

  /// @brief 本地视频采集失败，建议检查采集设备是否正常工作
  ///
  ByteRTCLocalVideoStreamErrorCaptureFailure(5),

  /// @brief 本地视频编码失败
  ///
  ByteRTCLocalVideoStreamErrorEncodeFailure(6),

  /// @brief 通话过程中本地视频采集设备被其他程序抢占，导致设备连接中断
  ///
  ByteRTCLocalVideoStreamErrorDeviceDisconnected(7);

  final dynamic $value;
  const ByteRTCLocalVideoStreamError([this.$value]);
}

enum ByteRTCVideoOutputOrientationMode {
  /// @brief 自适应布局
  ///
  ByteRTCVideoOutputOrientationModeAdaptative(0),

  /// @brief 横屏布局
  ///
  ByteRTCVideoOutputOrientationModeFixedLandscape(1),

  /// @brief 竖屏布局
  ///
  ByteRTCVideoOutputOrientationModeFixedPortrait(2);

  final dynamic $value;
  const ByteRTCVideoOutputOrientationMode([this.$value]);
}

class ByteRTCDeadLockMsg extends NativeClass {
  static const _$namespace = r'ByteRTCDeadLockMsg';
  static get codegen_$namespace => _$namespace;

  ByteRTCDeadLockMsg([NativeClassOptions? options])
      : super(options == null
            ? const NativeClassOptions(
                [],
                className: _$namespace,
                instanceType: InstanceType.manual,
                bridgeKey: 'com.volcengine.rtc.hybrid_runtime',
              )
            : NativeClassOptions.fromMap({
                ...options.toMap(),
                'bridgeKey': 'com.volcengine.rtc.hybrid_runtime',
              }));

  /// @brief session_id
  FutureOr<NSString?> get blockSessionId async {
    return await sendInstanceGet<NSString?>("blockSessionId");
  }

  set blockSessionId(FutureOr<NSString?> value) {
    sendInstanceSet("blockSessionId", value);
  }

  /// @brief 死锁线程等待链

  FutureOr<NSString?> get blockingPaths async {
    return await sendInstanceGet<NSString?>("blockingPaths");
  }

  set blockingPaths(FutureOr<NSString?> value) {
    sendInstanceSet("blockingPaths", value);
  }

  /// @brief For how long the thread is blocked.

  FutureOr<BOOL?> get isCritical async {
    return await sendInstanceGet<BOOL?>("isCritical");
  }

  set isCritical(FutureOr<BOOL?> value) {
    sendInstanceSet("isCritical", value);
  }
}

class ByteRTCAudioPropertiesConfig extends NativeClass {
  static const _$namespace = r'ByteRTCAudioPropertiesConfig';
  static get codegen_$namespace => _$namespace;

  ByteRTCAudioPropertiesConfig([NativeClassOptions? options])
      : super(options == null
            ? const NativeClassOptions(
                [],
                className: _$namespace,
                instanceType: InstanceType.manual,
                bridgeKey: 'com.volcengine.rtc.hybrid_runtime',
              )
            : NativeClassOptions.fromMap({
                ...options.toMap(),
                'bridgeKey': 'com.volcengine.rtc.hybrid_runtime',
              }));

  /// @brief 信息提示间隔，单位：ms <br>
  ///       - `<= 0`: 关闭信息提示
  ///       - `(0,100]`: 开启信息提示，不合法的 interval 值，SDK 自动设置为 100ms
  ///       - `> 100`: 开启信息提示，并将信息提示间隔设置为此值
  FutureOr<NSInteger?> get interval async {
    return await sendInstanceGet<NSInteger?>("interval");
  }

  set interval(FutureOr<NSInteger?> value) {
    sendInstanceSet("interval", value);
  }

  /// @brief 是否开启音频频谱检测。
  FutureOr<BOOL?> get enableSpectrum async {
    return await sendInstanceGet<BOOL?>("enableSpectrum");
  }

  set enableSpectrum(FutureOr<BOOL?> value) {
    sendInstanceSet("enableSpectrum", value);
  }

  /// @brief 是否开启人声检测 (VAD)。
  FutureOr<BOOL?> get enableVad async {
    return await sendInstanceGet<BOOL?>("enableVad");
  }

  set enableVad(FutureOr<BOOL?> value) {
    sendInstanceSet("enableVad", value);
  }

  /// @brief 音量回调模式。详见 ByteRTCAudioReportMode{@link #ByteRTCAudioReportMode}。
  FutureOr<ByteRTCAudioReportMode?> get localMainReportMode async {
    try {
      final result =
          await sendInstanceGet<ByteRTCAudioReportMode?>("localMainReportMode");
      if (result == null) {
        return null;
      }
      return ByteRTCAudioReportMode.values
          .firstWhere((element) => element == result);
    } catch (e) {
      return null;
    }
  }

  set localMainReportMode(FutureOr<ByteRTCAudioReportMode?> value) {
    sendInstanceSet("localMainReportMode", value);
  }

  /// @brief 适用于音频属性信息提示的平滑系数。取值范围是 `(0.0, 1.0]`。 <br>
  ///        默认值为 `1.0`，不开启平滑效果；值越小，提示音量平滑效果越明显。如果要开启平滑效果，可以设置为 `0.3`。
  FutureOr<float?> get smooth async {
    return await sendInstanceGet<float?>("smooth");
  }

  set smooth(FutureOr<float?> value) {
    sendInstanceSet("smooth", value);
  }

  /// @brief rtcEngine:onLocalAudioPropertiesReport:{@link #ByteRTCEngineDelegate#rtcEngine:onLocalAudioPropertiesReport} 中包含音频数据的范围。参看 ByteRTCAudioPropertiesMode{@link #ByteRTCAudioPropertiesMode}。 <br>
  ///        默认仅包含本地麦克风采集的音频数据和本地屏幕音频采集数据。
  FutureOr<ByteRTCAudioPropertiesMode?> get audioReportMode async {
    try {
      final result =
          await sendInstanceGet<ByteRTCAudioPropertiesMode?>("audioReportMode");
      if (result == null) {
        return null;
      }
      return ByteRTCAudioPropertiesMode.values
          .firstWhere((element) => element == result);
    } catch (e) {
      return null;
    }
  }

  set audioReportMode(FutureOr<ByteRTCAudioPropertiesMode?> value) {
    sendInstanceSet("audioReportMode", value);
  }

  /// @brief 是否回调本地用户的人声基频。
  FutureOr<BOOL?> get enableVoicePitch async {
    return await sendInstanceGet<BOOL?>("enableVoicePitch");
  }

  set enableVoicePitch(FutureOr<BOOL?> value) {
    sendInstanceSet("enableVoicePitch", value);
  }
}

class ByteRTCRemoteStreamStats extends NativeClass {
  static const _$namespace = r'ByteRTCRemoteStreamStats';
  static get codegen_$namespace => _$namespace;

  ByteRTCRemoteStreamStats([NativeClassOptions? options])
      : super(options == null
            ? const NativeClassOptions(
                [],
                className: _$namespace,
                instanceType: InstanceType.manual,
                bridgeKey: 'com.volcengine.rtc.hybrid_runtime',
              )
            : NativeClassOptions.fromMap({
                ...options.toMap(),
                'bridgeKey': 'com.volcengine.rtc.hybrid_runtime',
              }));

  /// @brief 用户 ID 。音频来源的用户 ID 。
  FutureOr<NSString?> get uid async {
    return await sendInstanceGet<NSString?>("uid");
  }

  set uid(FutureOr<NSString?> value) {
    sendInstanceSet("uid", value);
  }

  /// @brief 远端音频流的统计信息，详见 ByteRTCRemoteAudioStats{@link #ByteRTCRemoteAudioStats}
  FutureOr<ByteRTCRemoteAudioStats?> get audioStats async {
    try {
      final result =
          await sendInstanceGet<ByteRTCRemoteAudioStats?>("audioStats");
      if (result == null) {
        return null;
      }
      return packObject(
          result,
          () => ByteRTCRemoteAudioStats(
              const NativeClassOptions([], disableInit: true)));
    } catch (e) {
      return null;
    }
  }

  set audioStats(FutureOr<ByteRTCRemoteAudioStats?> value) {
    sendInstanceSet("audioStats", value);
  }

  /// @brief 远端视频流的统计信息，详见 ByteRTCRemoteVideoStats{@link #ByteRTCRemoteVideoStats}
  FutureOr<ByteRTCRemoteVideoStats?> get videoStats async {
    try {
      final result =
          await sendInstanceGet<ByteRTCRemoteVideoStats?>("videoStats");
      if (result == null) {
        return null;
      }
      return packObject(
          result,
          () => ByteRTCRemoteVideoStats(
              const NativeClassOptions([], disableInit: true)));
    } catch (e) {
      return null;
    }
  }

  set videoStats(FutureOr<ByteRTCRemoteVideoStats?> value) {
    sendInstanceSet("videoStats", value);
  }

  /// @brief 所属用户的媒体流是否为屏幕流。你可以知道当前统计数据来自主流还是屏幕流。
  FutureOr<BOOL?> get isScreen async {
    return await sendInstanceGet<BOOL?>("isScreen");
  }

  set isScreen(FutureOr<BOOL?> value) {
    sendInstanceSet("isScreen", value);
  }

  /// @brief 所属用户的媒体流上行网络质量，详见 ByteRTCNetworkQuality{@link #ByteRTCNetworkQuality}
  /// @deprecated since 3.36 and will be deleted in 3.51, use rtcRoom:onNetworkQuality:remoteQualities:{@link #ByteRTCRoomDelegate#rtcRoom:onNetworkQuality:remoteQualities} instead
  FutureOr<ByteRTCNetworkQuality?> get txQuality async {
    try {
      final result = await sendInstanceGet<ByteRTCNetworkQuality?>("txQuality");
      if (result == null) {
        return null;
      }
      return ByteRTCNetworkQuality.values
          .firstWhere((element) => element == result);
    } catch (e) {
      return null;
    }
  }

  set txQuality(FutureOr<ByteRTCNetworkQuality?> value) {
    sendInstanceSet("txQuality", value);
  }

  /// @brief 所属用户的媒体流下行网络质量，详见 ByteRTCNetworkQuality{@link #ByteRTCNetworkQuality}
  /// @deprecated since 3.36 and will be deleted in 3.51, use rtcRoom:onNetworkQuality:remoteQualities:{@link #ByteRTCRoomDelegate#rtcRoom:onNetworkQuality:remoteQualities} instead
  FutureOr<ByteRTCNetworkQuality?> get rxQuality async {
    try {
      final result = await sendInstanceGet<ByteRTCNetworkQuality?>("rxQuality");
      if (result == null) {
        return null;
      }
      return ByteRTCNetworkQuality.values
          .firstWhere((element) => element == result);
    } catch (e) {
      return null;
    }
  }

  set rxQuality(FutureOr<ByteRTCNetworkQuality?> value) {
    sendInstanceSet("rxQuality", value);
  }
}

enum ByteRTCVideoDenoiseMode {
  /// @brief 视频降噪关闭。
  ///
  ByteRTCVideoDenoiseModeOff(0),

  /// @brief 视频降噪开启，由 ByteRTC 后台配置视频降噪算法。
  ///
  ByteRTCVideoDenoiseModeAuto(1);

  final dynamic $value;
  const ByteRTCVideoDenoiseMode([this.$value]);
}

class ByteRTCStreamSyncInfoConfig extends NativeClass {
  static const _$namespace = r'ByteRTCStreamSyncInfoConfig';
  static get codegen_$namespace => _$namespace;

  ByteRTCStreamSyncInfoConfig([NativeClassOptions? options])
      : super(options == null
            ? const NativeClassOptions(
                [],
                className: _$namespace,
                instanceType: InstanceType.manual,
                bridgeKey: 'com.volcengine.rtc.hybrid_runtime',
              )
            : NativeClassOptions.fromMap({
                ...options.toMap(),
                'bridgeKey': 'com.volcengine.rtc.hybrid_runtime',
              }));

  /// @brief 消息发送的重复次数，取值范围是 [0,25]，建议设置为 [3,5]。
  FutureOr<int?> get repeatCount async {
    return await sendInstanceGet<int?>("repeatCount");
  }

  set repeatCount(FutureOr<int?> value) {
    sendInstanceSet("repeatCount", value);
  }

  /// @brief 媒体流信息同步的流类型，见 ByteRTCSyncInfoStreamType{@link #ByteRTCSyncInfoStreamType} 。
  FutureOr<ByteRTCSyncInfoStreamType?> get streamType async {
    try {
      final result =
          await sendInstanceGet<ByteRTCSyncInfoStreamType?>("streamType");
      if (result == null) {
        return null;
      }
      return ByteRTCSyncInfoStreamType.values
          .firstWhere((element) => element == result);
    } catch (e) {
      return null;
    }
  }

  set streamType(FutureOr<ByteRTCSyncInfoStreamType?> value) {
    sendInstanceSet("streamType", value);
  }
}

class ByteRTCExpressionDetectResult extends NativeClass {
  static const _$namespace = r'ByteRTCExpressionDetectResult';
  static get codegen_$namespace => _$namespace;

  ByteRTCExpressionDetectResult([NativeClassOptions? options])
      : super(options == null
            ? const NativeClassOptions(
                [],
                className: _$namespace,
                instanceType: InstanceType.manual,
                bridgeKey: 'com.volcengine.rtc.hybrid_runtime',
              )
            : NativeClassOptions.fromMap({
                ...options.toMap(),
                'bridgeKey': 'com.volcengine.rtc.hybrid_runtime',
              }));

  /// @brief 特征识别结果 <br>
  ///        - 0：识别成功
  ///        - !0：识别失败
  FutureOr<int?> get detectResult async {
    return await sendInstanceGet<int?>("detectResult");
  }

  set detectResult(FutureOr<int?> value) {
    sendInstanceSet("detectResult", value);
  }

  /// @brief 识别到的人脸数量。
  FutureOr<int?> get faceCount async {
    return await sendInstanceGet<int?>("faceCount");
  }

  set faceCount(FutureOr<int?> value) {
    sendInstanceSet("faceCount", value);
  }

  /// @brief 特征识别信息。数组的长度和检测到的人脸数量一致。参看 ByteRTCExpressionDetectInfo{@link #ByteRTCExpressionDetectInfo}。
  FutureOr<NSArray<ByteRTCExpressionDetectInfo>?> get detectInfo async {
    try {
      final result =
          await sendInstanceGet<NSArray<ByteRTCExpressionDetectInfo>?>(
              "detectInfo");
      if (result == null) {
        return null;
      }
      final list = result.map((e) => packObject(
          e,
          () => ByteRTCExpressionDetectInfo(
              const NativeClassOptions([], disableInit: true))));
      return list.toList();
    } catch (e) {
      return null;
    }
  }

  set detectInfo(FutureOr<NSArray<ByteRTCExpressionDetectInfo>?> value) {
    sendInstanceSet("detectInfo", value);
  }
}

enum ByteRTCMixedStreamAudioProfile {
  /// @brief AAC-LC 规格，默认值。
  ///
  ByteRTCMixedStreamAudioProfileLC(0),

  /// @brief HE-AAC v1 规格。
  ///
  ByteRTCMixedStreamAudioProfileHEv1(1),

  /// @brief HE-AAC v2 规格。
  ///
  ByteRTCMixedStreamAudioProfileHEv2(2);

  final dynamic $value;
  const ByteRTCMixedStreamAudioProfile([this.$value]);
}

class ByteRTCExpressionDetectInfo extends NativeClass {
  static const _$namespace = r'ByteRTCExpressionDetectInfo';
  static get codegen_$namespace => _$namespace;

  ByteRTCExpressionDetectInfo([NativeClassOptions? options])
      : super(options == null
            ? const NativeClassOptions(
                [],
                className: _$namespace,
                instanceType: InstanceType.manual,
                bridgeKey: 'com.volcengine.rtc.hybrid_runtime',
              )
            : NativeClassOptions.fromMap({
                ...options.toMap(),
                'bridgeKey': 'com.volcengine.rtc.hybrid_runtime',
              }));

  /// @brief 预测年龄，取值范围 (0, 100)。
  FutureOr<float?> get age async {
    return await sendInstanceGet<float?>("age");
  }

  set age(FutureOr<float?> value) {
    sendInstanceSet("age", value);
  }

  /// @brief 预测为男性的概率，取值范围 (0.0, 1.0)。
  FutureOr<float?> get boyProb async {
    return await sendInstanceGet<float?>("boyProb");
  }

  set boyProb(FutureOr<float?> value) {
    sendInstanceSet("boyProb", value);
  }

  /// @brief 预测的吸引力分数，取值范围 (0, 100)。
  FutureOr<float?> get attractive async {
    return await sendInstanceGet<float?>("attractive");
  }

  set attractive(FutureOr<float?> value) {
    sendInstanceSet("attractive", value);
  }

  /// @brief 预测的微笑程度，取值范围 (0, 100)。
  FutureOr<float?> get happyScore async {
    return await sendInstanceGet<float?>("happyScore");
  }

  set happyScore(FutureOr<float?> value) {
    sendInstanceSet("happyScore", value);
  }

  /// @brief 预测的伤心程度，取值范围 (0, 100)。
  FutureOr<float?> get sadScore async {
    return await sendInstanceGet<float?>("sadScore");
  }

  set sadScore(FutureOr<float?> value) {
    sendInstanceSet("sadScore", value);
  }

  /// @brief 预测的生气程度，取值范围 (0, 100)。
  FutureOr<float?> get angryScore async {
    return await sendInstanceGet<float?>("angryScore");
  }

  set angryScore(FutureOr<float?> value) {
    sendInstanceSet("angryScore", value);
  }

  /// @brief 预测的吃惊程度，取值范围 (0, 100)。
  FutureOr<float?> get surpriseScore async {
    return await sendInstanceGet<float?>("surpriseScore");
  }

  set surpriseScore(FutureOr<float?> value) {
    sendInstanceSet("surpriseScore", value);
  }

  /// @brief 预测的情绪激动程度，取值范围 (0, 100)。
  FutureOr<float?> get arousal async {
    return await sendInstanceGet<float?>("arousal");
  }

  set arousal(FutureOr<float?> value) {
    sendInstanceSet("arousal", value);
  }

  /// @brief 预测的情绪正负程度，取值范围 (-100, 100)。
  FutureOr<float?> get valence async {
    return await sendInstanceGet<float?>("valence");
  }

  set valence(FutureOr<float?> value) {
    sendInstanceSet("valence", value);
  }
}

enum ByteRTCVideoDeviceType {
  /// @brief 未知视频设备
  ///
  ByteRTCVideoDeviceTypeUnknown(-1),

  /// @brief 视频渲染设备类型
  ///
  ByteRTCVideoDeviceTypeRenderDevice(0),

  /// @brief 视频采集设备类型
  ///
  ByteRTCVideoDeviceTypeCaptureDevice(1),

  /// @brief 屏幕流视频设备
  ///
  ByteRTCVideoDeviceTypeScreenCaptureDevice(2);

  final dynamic $value;
  const ByteRTCVideoDeviceType([this.$value]);
}

enum ByteRTCVideoSinkPixelFormat {
  /// @brief 原始视频帧格式
  ///
  ByteRTCVideoSinkPixelFormatOriginal(0),

  /// @brief YUV I420 格式
  ///
  ByteRTCVideoSinkPixelFormatI420(1),

  /// @brief YUV NV12 格式
  ///
  ByteRTCVideoSinkPixelFormatNV12(2),

  /// @brief RGBA 格式, 字节序为 R8 G8 B8 A8
  ///
  ByteRTCVideoSinkPixelFormatRGBA(5),

  /// @brief BGRA 格式
  ///
  ByteRTCVideoSinkPixelFormatBGRA(7);

  final dynamic $value;
  const ByteRTCVideoSinkPixelFormat([this.$value]);
}

enum ByteRTCMirrorType {
  /// @brief 本地预览和编码传输时均无镜像效果
  ///
  ByteRTCMirrorTypeNone(0),

  /// @brief 本地预览时有镜像效果，编码传输时无镜像效果
  ///
  ByteRTCMirrorTypeRender(1),

  /// @brief 本地预览时无镜像效果，仅编码传输时有镜像效果
  ///
  ByteRTCMirrorTypeEncoder(2),

  /// @brief 本地预览和编码传输时均有镜像效果
  ///
  ByteRTCMirrorTypeRenderAndEncoder(3);

  final dynamic $value;
  const ByteRTCMirrorType([this.$value]);
}

class ByteRTCSourceCropInfo extends NativeClass {
  static const _$namespace = r'ByteRTCSourceCropInfo';
  static get codegen_$namespace => _$namespace;

  ByteRTCSourceCropInfo([NativeClassOptions? options])
      : super(options == null
            ? const NativeClassOptions(
                [],
                className: _$namespace,
                instanceType: InstanceType.manual,
                bridgeKey: 'com.volcengine.rtc.hybrid_runtime',
              )
            : NativeClassOptions.fromMap({
                ...options.toMap(),
                'bridgeKey': 'com.volcengine.rtc.hybrid_runtime',
              }));

  /// @brief 裁剪后得到的视频帧左上角横坐标相对于裁剪前整体画面的比例，取值范围[0.0, 1.0)
  FutureOr<CGFloat?> get locationX async {
    return await sendInstanceGet<CGFloat?>("locationX");
  }

  set locationX(FutureOr<CGFloat?> value) {
    sendInstanceSet("locationX", value);
  }

  /// @brief 裁剪后得到的视频帧左上角纵坐标相对于裁剪前整体画面的比例，取值范围[0.0, 1.0)
  FutureOr<CGFloat?> get locationY async {
    return await sendInstanceGet<CGFloat?>("locationY");
  }

  set locationY(FutureOr<CGFloat?> value) {
    sendInstanceSet("locationY", value);
  }

  /// @brief 裁剪后得到的视频帧宽度相对于裁剪前整体画面的比例，取值范围(0.0, 1.0]
  FutureOr<CGFloat?> get widthProportion async {
    return await sendInstanceGet<CGFloat?>("widthProportion");
  }

  set widthProportion(FutureOr<CGFloat?> value) {
    sendInstanceSet("widthProportion", value);
  }

  /// @brief 裁剪后得到的视频帧高度相对于裁剪前整体画面的比例，取值范围(0.0, 1.0]
  FutureOr<CGFloat?> get heightProportion async {
    return await sendInstanceGet<CGFloat?>("heightProportion");
  }

  set heightProportion(FutureOr<CGFloat?> value) {
    sendInstanceSet("heightProportion", value);
  }
}

class ByteRTCUser extends NativeClass {
  static const _$namespace = r'ByteRTCUser';
  static get codegen_$namespace => _$namespace;

  ByteRTCUser([NativeClassOptions? options])
      : super(options == null
            ? const NativeClassOptions(
                [],
                className: _$namespace,
                instanceType: InstanceType.manual,
                bridgeKey: 'com.volcengine.rtc.hybrid_runtime',
              )
            : NativeClassOptions.fromMap({
                ...options.toMap(),
                'bridgeKey': 'com.volcengine.rtc.hybrid_runtime',
              }));

  /// @brief 用户 ID
  FutureOr<NSString?> get userId async {
    return await sendInstanceGet<NSString?>("userId");
  }

  set userId(FutureOr<NSString?> value) {
    sendInstanceSet("userId", value);
  }

  /// @brief 元信息
  FutureOr<NSString?> get metaData async {
    return await sendInstanceGet<NSString?>("metaData");
  }

  set metaData(FutureOr<NSString?> value) {
    sendInstanceSet("metaData", value);
  }
}

class ByteRTCMixedStreamAudioConfig extends NativeClass {
  static const _$namespace = r'ByteRTCMixedStreamAudioConfig';
  static get codegen_$namespace => _$namespace;

  ByteRTCMixedStreamAudioConfig([NativeClassOptions? options])
      : super(options == null
            ? const NativeClassOptions(
                [],
                className: _$namespace,
                instanceType: InstanceType.manual,
                bridgeKey: 'com.volcengine.rtc.hybrid_runtime',
              )
            : NativeClassOptions.fromMap({
                ...options.toMap(),
                'bridgeKey': 'com.volcengine.rtc.hybrid_runtime',
              }));

  /// @brief 音频编码格式。建议设置。
  /// @param codec 音频编码格式，参看 ByteRTCMixedStreamAudioCodecType{@link #ByteRTCMixedStreamAudioCodecType}。默认值为 `0`。建议设置。
  FutureOr<ByteRTCMixedStreamAudioCodecType?> get audioCodec async {
    try {
      final result = await sendInstanceGet<ByteRTCMixedStreamAudioCodecType?>(
          "audioCodec");
      if (result == null) {
        return null;
      }
      return ByteRTCMixedStreamAudioCodecType.values
          .firstWhere((element) => element == result);
    } catch (e) {
      return null;
    }
  }

  set audioCodec(FutureOr<ByteRTCMixedStreamAudioCodecType?> value) {
    sendInstanceSet("audioCodec", value);
  }

  /// @brief 音频采样率，单位 Hz。可取 32000 Hz、44100 Hz、48000 Hz，默认值为 48000 Hz。建议设置。
  FutureOr<NSInteger?> get sampleRate async {
    return await sendInstanceGet<NSInteger?>("sampleRate");
  }

  set sampleRate(FutureOr<NSInteger?> value) {
    sendInstanceSet("sampleRate", value);
  }

  /// @brief 音频声道数。可取 1（单声道）、2（双声道），默认值为 2。建议设置。
  FutureOr<NSInteger?> get channels async {
    return await sendInstanceGet<NSInteger?>("channels");
  }

  set channels(FutureOr<NSInteger?> value) {
    sendInstanceSet("channels", value);
  }

  /// @brief 音频码率，单位 Kbps。可取范围 [32, 192]，默认值为 64 Kbps。建议设置。
  FutureOr<NSInteger?> get bitrate async {
    return await sendInstanceGet<NSInteger?>("bitrate");
  }

  set bitrate(FutureOr<NSInteger?> value) {
    sendInstanceSet("bitrate", value);
  }

  /// @brief AAC 编码规格，参看 ByteRTCMixedStreamAudioProfile{@link #ByteRTCMixedStreamAudioProfile}。默认值为 `0`。建议设置。
  FutureOr<ByteRTCMixedStreamAudioProfile?> get audioProfile async {
    try {
      final result = await sendInstanceGet<ByteRTCMixedStreamAudioProfile?>(
          "audioProfile");
      if (result == null) {
        return null;
      }
      return ByteRTCMixedStreamAudioProfile.values
          .firstWhere((element) => element == result);
    } catch (e) {
      return null;
    }
  }

  set audioProfile(FutureOr<ByteRTCMixedStreamAudioProfile?> value) {
    sendInstanceSet("audioProfile", value);
  }
}

enum ByteRTCVideoFrameType {
  /// @brief 视频帧类型：内存数据
  ///
  ByteRTCVideoFrameTypeRawMemory(0),

  /// @brief 视频帧类型：CVPixelBuffer
  ///
  ByteRTCVideoFrameTypePixelBuffer(1);

  final dynamic $value;
  const ByteRTCVideoFrameType([this.$value]);
}

class ByteRTCMixedStreamLayoutRegionConfig extends NativeClass {
  static const _$namespace = r'ByteRTCMixedStreamLayoutRegionConfig';
  static get codegen_$namespace => _$namespace;

  ByteRTCMixedStreamLayoutRegionConfig([NativeClassOptions? options])
      : super(options == null
            ? const NativeClassOptions(
                [],
                className: _$namespace,
                instanceType: InstanceType.manual,
                bridgeKey: 'com.volcengine.rtc.hybrid_runtime',
              )
            : NativeClassOptions.fromMap({
                ...options.toMap(),
                'bridgeKey': 'com.volcengine.rtc.hybrid_runtime',
              }));

  /// @brief 视频流发布用户的用户 ID 。建议设置。
  FutureOr<NSString?> get userID async {
    return await sendInstanceGet<NSString?>("userID");
  }

  set userID(FutureOr<NSString?> value) {
    sendInstanceSet("userID", value);
  }

  /// @brief 图片或视频流所在房间的房间 ID。建议设置。 <br>
  ///        如果此图片或视频流是通过 startForwardStreamToRooms:{@link #ByteRTCRoom#startForwardStreamToRooms} 转发到你所在房间的媒体流时，你应将房间 ID 设置为你所在的房间 ID。
  FutureOr<NSString?> get roomID async {
    return await sendInstanceGet<NSString?>("roomID");
  }

  set roomID(FutureOr<NSString?> value) {
    sendInstanceSet("roomID", value);
  }

  /// @brief 单个用户画面左上角在整个画布坐标系中的 X 坐标（pixel），即以画布左上角为原点，用户画面左上角相对于原点的横向位移。 <br>
  ///        取值范围为 [0, 整体画布宽度)。默认值为 0。
  FutureOr<NSInteger?> get locationX async {
    return await sendInstanceGet<NSInteger?>("locationX");
  }

  set locationX(FutureOr<NSInteger?> value) {
    sendInstanceSet("locationX", value);
  }

  /// @brief 单个用户画面左上角在整个画布坐标系中的 Y 坐标（pixel），即以画布左上角为原点，用户画面左上角相对于原点的纵向位移。 <br>
  ///        取值范围为 [0, 整体画布高度)。默认值为 0。
  FutureOr<NSInteger?> get locationY async {
    return await sendInstanceGet<NSInteger?>("locationY");
  }

  set locationY(FutureOr<NSInteger?> value) {
    sendInstanceSet("locationY", value);
  }

  /// @brief 单个用户画面的宽度。取值范围为 [0, 整体画布宽度]，默认值为 360。
  FutureOr<NSInteger?> get width async {
    return await sendInstanceGet<NSInteger?>("width");
  }

  set width(FutureOr<NSInteger?> value) {
    sendInstanceSet("width", value);
  }

  /// @brief 单个用户画面的高度。取值范围为 [0, 整体画布高度]，默认值为 640。
  FutureOr<NSInteger?> get height async {
    return await sendInstanceGet<NSInteger?>("height");
  }

  set height(FutureOr<NSInteger?> value) {
    sendInstanceSet("height", value);
  }

  /// @brief 用户视频布局在画布中的层级。取值范围为 [0 - 100]，0 为底层，值越大越上层。默认值为 0。建议设置。
  FutureOr<NSInteger?> get zOrder async {
    return await sendInstanceGet<NSInteger?>("zOrder");
  }

  set zOrder(FutureOr<NSInteger?> value) {
    sendInstanceSet("zOrder", value);
  }

  FutureOr<BOOL?> get isLocalUser async {
    return await sendInstanceGet<BOOL?>("isLocalUser");
  }

  set isLocalUser(FutureOr<BOOL?> value) {
    sendInstanceSet("isLocalUser", value);
  }

  FutureOr<ByteRTCMixedStreamVideoType?> get streamType async {
    try {
      final result =
          await sendInstanceGet<ByteRTCMixedStreamVideoType?>("streamType");
      if (result == null) {
        return null;
      }
      return ByteRTCMixedStreamVideoType.values
          .firstWhere((element) => element == result);
    } catch (e) {
      return null;
    }
  }

  set streamType(FutureOr<ByteRTCMixedStreamVideoType?> value) {
    sendInstanceSet("streamType", value);
  }

  /// @brief 透明度，可选范围为 (0.0, 1.0]，0.0 为全透明。默认值为 1.0。
  FutureOr<CGFloat?> get alpha async {
    return await sendInstanceGet<CGFloat?>("alpha");
  }

  set alpha(FutureOr<CGFloat?> value) {
    sendInstanceSet("alpha", value);
  }

  /// @brief 圆角半径相对画布宽度的比例。默认值为 `0.0`。 <br>
  ///        做范围判定时，首先根据画布的宽高，将 `width`，`height`，和 `cornerRadius` 分别转换为像素值：`width_px`，`height_px`，和 `cornerRadius_px`。然后判定是否满足 `cornerRadius_px < min(width_px/2, height_px/2)`：若满足，则设置成功；若不满足，则将 `cornerRadius_px` 设定为 `min(width_px/2, height_px/2)`，然后将 `cornerRadius` 设定为 `cornerRadius_px` 相对画布宽度的比例值。
  ///        WTN 流任务不支持设置本参数。
  FutureOr<CGFloat?> get cornerRadius async {
    return await sendInstanceGet<CGFloat?>("cornerRadius");
  }

  set cornerRadius(FutureOr<CGFloat?> value) {
    sendInstanceSet("cornerRadius", value);
  }

  /// @brief 合流内容控制。默认值为 `ByteRTCTranscoderContentControlTypeHasAudioAndVideo`，参看 ByteRTCMixedStreamMediaType{@link #ByteRTCMixedStreamMediaType} 。
  FutureOr<ByteRTCMixedStreamMediaType?> get mediaType async {
    try {
      final result =
          await sendInstanceGet<ByteRTCMixedStreamMediaType?>("mediaType");
      if (result == null) {
        return null;
      }
      return ByteRTCMixedStreamMediaType.values
          .firstWhere((element) => element == result);
    } catch (e) {
      return null;
    }
  }

  set mediaType(FutureOr<ByteRTCMixedStreamMediaType?> value) {
    sendInstanceSet("mediaType", value);
  }

  /// @brief 图片或视频流的缩放模式，参看 ByteRTCMixedStreamRenderMode{@link #ByteRTCMixedStreamRenderMode}。默认值为 1。建议设置。
  FutureOr<ByteRTCMixedStreamRenderMode?> get renderMode async {
    try {
      final result =
          await sendInstanceGet<ByteRTCMixedStreamRenderMode?>("renderMode");
      if (result == null) {
        return null;
      }
      return ByteRTCMixedStreamRenderMode.values
          .firstWhere((element) => element == result);
    } catch (e) {
      return null;
    }
  }

  set renderMode(FutureOr<ByteRTCMixedStreamRenderMode?> value) {
    sendInstanceSet("renderMode", value);
  }

  /// @brief 合流布局区域类型。参看 ByteRTCMixedStreamLayoutRegionType{@link #ByteRTCMixedStreamLayoutRegionType}。建议设置。
  FutureOr<ByteRTCMixedStreamLayoutRegionType?> get regionContentType async {
    try {
      final result = await sendInstanceGet<ByteRTCMixedStreamLayoutRegionType?>(
          "regionContentType");
      if (result == null) {
        return null;
      }
      return ByteRTCMixedStreamLayoutRegionType.values
          .firstWhere((element) => element == result);
    } catch (e) {
      return null;
    }
  }

  set regionContentType(FutureOr<ByteRTCMixedStreamLayoutRegionType?> value) {
    sendInstanceSet("regionContentType", value);
  }

  /// @brief 水印图 RGBA 数据。当 `regionContentType` 为图片类型时需要设置。 <br>
  ///        - `ByteRTCMixedStreamLayoutRegionTypeImage = 1` 时，传入图片 RGBA 数据。
  ///        - `ByteRTCMixedStreamLayoutRegionTypeVideoStream = 0` 时传入空。
  ///        WTN 流任务不支持设置本参数。
  FutureOr<NSData?> get imageWaterMark async {
    return await sendInstanceGet<NSData?>("imageWaterMark");
  }

  set imageWaterMark(FutureOr<NSData?> value) {
    sendInstanceSet("imageWaterMark", value);
  }

  /// @brief 水印图参数。当 `regionContentType` 为图片类型时需要设置。 <br>
  ///        - `ByteRTCMixedStreamLayoutRegionTypeImage = 1` 时，传入图片参数，参看 ByteRTCMixedStreamLayoutRegionImageWaterMarkConfig{@link #ByteRTCMixedStreamLayoutRegionImageWaterMarkConfig}。
  ///        - `ByteRTCMixedStreamLayoutRegionTypeVideoStream = 0` 时传入空。
  ///        WTN 流任务不支持设置本参数。
  FutureOr<ByteRTCMixedStreamLayoutRegionImageWaterMarkConfig?>
      get imageWaterMarkConfig async {
    try {
      final result = await sendInstanceGet<
              ByteRTCMixedStreamLayoutRegionImageWaterMarkConfig?>(
          "imageWaterMarkConfig");
      if (result == null) {
        return null;
      }
      return packObject(
          result,
          () => ByteRTCMixedStreamLayoutRegionImageWaterMarkConfig(
              const NativeClassOptions([], disableInit: true)));
    } catch (e) {
      return null;
    }
  }

  set imageWaterMarkConfig(
      FutureOr<ByteRTCMixedStreamLayoutRegionImageWaterMarkConfig?> value) {
    sendInstanceSet("imageWaterMarkConfig", value);
  }

  /// @brief 空间位置。参看 ByteRTCPosition{@link #ByteRTCPosition}。
  FutureOr<ByteRTCPosition?> get spatialPosition async {
    try {
      final result = await sendInstanceGet<ByteRTCPosition?>("spatialPosition");
      if (result == null) {
        return null;
      }
      return packObject(
          result,
          () =>
              ByteRTCPosition(const NativeClassOptions([], disableInit: true)));
    } catch (e) {
      return null;
    }
  }

  set spatialPosition(FutureOr<ByteRTCPosition?> value) {
    sendInstanceSet("spatialPosition", value);
  }

  /// @brief 设置某用户是否应用空间音频效果： <br>
  ///        - Yes：启用（默认值）
  ///        - No：禁用
  FutureOr<BOOL?> get applySpatialAudio async {
    return await sendInstanceGet<BOOL?>("applySpatialAudio");
  }

  set applySpatialAudio(FutureOr<BOOL?> value) {
    sendInstanceSet("applySpatialAudio", value);
  }

  /// @valid since 3.57
  /// @brief 设置占位图的填充模式。 <br>
  ///        该参数用来控制当用户停止发布视频流，画面恢复为占位图后，此时占位图的填充模式。参看 ByteRTCMixedStreamAlternateImageFillMode{@link #ByteRTCMixedStreamAlternateImageFillMode}。
  FutureOr<ByteRTCMixedStreamAlternateImageFillMode?>
      get alternateImageFillMode async {
    try {
      final result =
          await sendInstanceGet<ByteRTCMixedStreamAlternateImageFillMode?>(
              "alternateImageFillMode");
      if (result == null) {
        return null;
      }
      return ByteRTCMixedStreamAlternateImageFillMode.values
          .firstWhere((element) => element == result);
    } catch (e) {
      return null;
    }
  }

  set alternateImageFillMode(
      FutureOr<ByteRTCMixedStreamAlternateImageFillMode?> value) {
    sendInstanceSet("alternateImageFillMode", value);
  }

  /// @valid since 3.57
  /// @brief 设置占位图的 URL，长度小于 1024 字符.
  FutureOr<NSString?> get alternateImageUrl async {
    return await sendInstanceGet<NSString?>("alternateImageUrl");
  }

  set alternateImageUrl(FutureOr<NSString?> value) {
    sendInstanceSet("alternateImageUrl", value);
  }

  /// @valid since 3.57
  /// @brief WTN 流裁剪区域。参看 ByteRTCSourceCropInfo{@link #ByteRTCSourceCropInfo}。
  FutureOr<ByteRTCSourceCropInfo?> get sourceCrop async {
    try {
      final result =
          await sendInstanceGet<ByteRTCSourceCropInfo?>("sourceCrop");
      if (result == null) {
        return null;
      }
      return packObject(
          result,
          () => ByteRTCSourceCropInfo(
              const NativeClassOptions([], disableInit: true)));
    } catch (e) {
      return null;
    }
  }

  set sourceCrop(FutureOr<ByteRTCSourceCropInfo?> value) {
    sendInstanceSet("sourceCrop", value);
  }
}

enum ByteRTCLocalAudioStreamError {
  /// @brief 本地音频状态正常
  ///
  ByteRTCLocalAudioStreamErrorOk(0),

  /// @brief 本地音频出错原因未知
  ///
  ByteRTCLocalAudioStreamErrorFailure(1),

  /// @brief 没有权限启动本地音频录制设备
  ///
  ByteRTCLocalAudioStreamErrorDeviceNoPermission(2),

  /// @hidden currently not available
  /// @brief 本地音频录制设备已经在使用中
  /// @note 该错误码暂未使用
  ///
  ByteRTCLocalAudioStreamErrorDeviceBusy(3),

  /// @brief 本地音频录制失败，建议你检查录制设备是否正常工作
  ///
  ByteRTCLocalAudioStreamErrorRecordFailure(4),

  /// @brief 本地音频编码失败
  ///
  ByteRTCLocalAudioStreamErrorEncodeFailure(5),

  /// @brief 没有可用的音频录制设备
  ///
  ByteRTCLocalAudioStreamErrorNoRecordingDevice(6);

  final dynamic $value;
  const ByteRTCLocalAudioStreamError([this.$value]);
}

enum ByteRTCChorusCacheSyncMode {
  /// @brief 合唱场景下，主唱应采用此模式，以发送带时间戳信息的媒体数据。
  ///
  ByteRTCChorusCacheSyncModeProducer(0),

  /// @brief 合唱场景下，副唱应采用此模式。 <br>
  ///        此模式下，副唱收到来自主唱的带时间戳的媒体数据。副唱发送的媒体数据中带有来自主唱的时间戳。
  ///
  ByteRTCChorusCacheSyncModeRetransmitter(1),

  /// @brief 合唱场景下，听众应采用此模式。 <br>
  ///        此模式下，听众收到来自主唱的时间戳，并据此对齐来自主唱和副唱的媒体数据，以获得良好的合唱播放效果。
  ///
  ByteRTCChorusCacheSyncModeConsumer(2);

  final dynamic $value;
  const ByteRTCChorusCacheSyncMode([this.$value]);
}

enum ByteRTCConnectionState {
  /// @brief 连接断开超过 12s，此时 SDK 会尝试自动重连。
  ///
  ByteRTCConnectionStateDisconnected(1),

  /// @brief 首次请求建立连接，正在连接中。
  ///
  ByteRTCConnectionStateConnecting(2),

  /// @brief 首次连接成功。
  ///
  ByteRTCConnectionStateConnected(3),

  /// @brief 涵盖了以下情况： <br>
  ///        - 首次连接时，10 秒连接不成功;
  ///        - 连接成功后，断连 10 秒。自动重连中。
  ///
  ByteRTCConnectionStateReconnecting(4),

  /// @brief 连接断开后，重连成功。
  ///
  ByteRTCConnectionStateReconnected(5),

  /// @brief 处于 `ByteRTCConnectionStateDisconnected` 状态超过 10 秒，且期间重连未成功。SDK 仍将继续尝试重连。
  ///
  ByteRTCConnectionStateLost(6),

  /// @brief 连接失败，服务端状态异常。SDK 不会自动重连，请重新进房，或联系技术支持。
  ///
  ByteRTCConnectionStateFailed(7);

  final dynamic $value;
  const ByteRTCConnectionState([this.$value]);
}

class ByteRTCLocalAudioStats extends NativeClass {
  static const _$namespace = r'ByteRTCLocalAudioStats';
  static get codegen_$namespace => _$namespace;

  ByteRTCLocalAudioStats([NativeClassOptions? options])
      : super(options == null
            ? const NativeClassOptions(
                [],
                className: _$namespace,
                instanceType: InstanceType.manual,
                bridgeKey: 'com.volcengine.rtc.hybrid_runtime',
              )
            : NativeClassOptions.fromMap({
                ...options.toMap(),
                'bridgeKey': 'com.volcengine.rtc.hybrid_runtime',
              }));

  /// @brief 音频丢包率。此次统计周期内的音频上行丢包率，取值范围为 [0, 1] 。
  FutureOr<float?> get audioLossRate async {
    return await sendInstanceGet<float?>("audioLossRate");
  }

  set audioLossRate(FutureOr<float?> value) {
    sendInstanceSet("audioLossRate", value);
  }

  /// @brief 发送的码率。此次统计周期内的音频发送码率，单位为 kbps 。
  FutureOr<float?> get sentKBitrate async {
    return await sendInstanceGet<float?>("sentKBitrate");
  }

  set sentKBitrate(FutureOr<float?> value) {
    sendInstanceSet("sentKBitrate", value);
  }

  /// @brief 采集采样率。此次统计周期内的音频采集采样率信息，单位为 Hz 。
  FutureOr<NSInteger?> get recordSampleRate async {
    return await sendInstanceGet<NSInteger?>("recordSampleRate");
  }

  set recordSampleRate(FutureOr<NSInteger?> value) {
    sendInstanceSet("recordSampleRate", value);
  }

  /// @brief 统计间隔。此次统计周期的间隔，单位为 ms 。 <br>
  ///        此字段用于设置回调的统计周期，默认设置为 2s 。
  FutureOr<NSInteger?> get statsInterval async {
    return await sendInstanceGet<NSInteger?>("statsInterval");
  }

  set statsInterval(FutureOr<NSInteger?> value) {
    sendInstanceSet("statsInterval", value);
  }

  /// @brief 往返时延。单位为 ms 。
  FutureOr<NSInteger?> get rtt async {
    return await sendInstanceGet<NSInteger?>("rtt");
  }

  set rtt(FutureOr<NSInteger?> value) {
    sendInstanceSet("rtt", value);
  }

  /// @brief 音频声道数。
  FutureOr<NSInteger?> get numChannels async {
    return await sendInstanceGet<NSInteger?>("numChannels");
  }

  set numChannels(FutureOr<NSInteger?> value) {
    sendInstanceSet("numChannels", value);
  }

  /// @brief 音频发送采样率。此次统计周期内的音频发送采样率信息，单位为 Hz 。
  FutureOr<NSInteger?> get sentSampleRate async {
    return await sendInstanceGet<NSInteger?>("sentSampleRate");
  }

  set sentSampleRate(FutureOr<NSInteger?> value) {
    sendInstanceSet("sentSampleRate", value);
  }

  /// @brief 音频上行网络抖动，单位为 ms。
  FutureOr<NSInteger?> get jitter async {
    return await sendInstanceGet<NSInteger?>("jitter");
  }

  set jitter(FutureOr<NSInteger?> value) {
    sendInstanceSet("jitter", value);
  }

  /// @brief 音频设备的采集和播放延时，单位为 ms。
  FutureOr<NSInteger?> get audioDeviceLoopDelay async {
    return await sendInstanceGet<NSInteger?>("audioDeviceLoopDelay");
  }

  set audioDeviceLoopDelay(FutureOr<NSInteger?> value) {
    sendInstanceSet("audioDeviceLoopDelay", value);
  }

  ///
  /// @brief 音频编码帧率。
  FutureOr<double?> get encodeFrameRate async {
    return await sendInstanceGet<double?>("encodeFrameRate");
  }

  set encodeFrameRate(FutureOr<double?> value) {
    sendInstanceSet("encodeFrameRate", value);
  }
}

enum ByteRTCAudioPlaybackDevice {
  /// @brief 有线耳机
  ///
  ByteRTCAudioPlaybackDeviceHeadset(1),

  /// @brief 听筒
  ///
  ByteRTCAudioPlaybackDeviceEarpiece(2),

  /// @brief 扬声器
  ///
  ByteRTCAudioPlaybackDeviceSpeakerphone(3),

  /// @brief 蓝牙耳机
  ///
  ByteRTCAudioPlaybackDeviceHeadsetBluetooth(4),

  /// @brief USB 设备
  ///
  ByteRTCAudioPlaybackDeviceHeadsetUSB(5);

  final dynamic $value;
  const ByteRTCAudioPlaybackDevice([this.$value]);
}

class ByteRTCVideoWatermarkConfig extends NativeClass {
  static const _$namespace = r'ByteRTCVideoWatermarkConfig';
  static get codegen_$namespace => _$namespace;

  ByteRTCVideoWatermarkConfig([NativeClassOptions? options])
      : super(options == null
            ? const NativeClassOptions(
                [],
                className: _$namespace,
                instanceType: InstanceType.manual,
                bridgeKey: 'com.volcengine.rtc.hybrid_runtime',
              )
            : NativeClassOptions.fromMap({
                ...options.toMap(),
                'bridgeKey': 'com.volcengine.rtc.hybrid_runtime',
              }));

  /// @brief 水印是否在视频预览中可见，默认可见。
  FutureOr<BOOL?> get visibleInPreview async {
    return await sendInstanceGet<BOOL?>("visibleInPreview");
  }

  set visibleInPreview(FutureOr<BOOL?> value) {
    sendInstanceSet("visibleInPreview", value);
  }

  /// @brief 横屏时的水印位置和大小，参看 ByteRTCVideoByteWatermark{@link #ByteRTCVideoByteWatermark}。
  FutureOr<ByteRTCVideoByteWatermark?> get positionInLandscapeMode async {
    try {
      final result = await sendInstanceGet<ByteRTCVideoByteWatermark?>(
          "positionInLandscapeMode");
      if (result == null) {
        return null;
      }
      return packObject(
          result,
          () => ByteRTCVideoByteWatermark(
              const NativeClassOptions([], disableInit: true)));
    } catch (e) {
      return null;
    }
  }

  set positionInLandscapeMode(FutureOr<ByteRTCVideoByteWatermark?> value) {
    sendInstanceSet("positionInLandscapeMode", value);
  }

  /// @brief 竖屏时的水印位置和大小，参看 ByteRTCVideoByteWatermark{@link #ByteRTCVideoByteWatermark}。
  FutureOr<ByteRTCVideoByteWatermark?> get positionInPortraitMode async {
    try {
      final result = await sendInstanceGet<ByteRTCVideoByteWatermark?>(
          "positionInPortraitMode");
      if (result == null) {
        return null;
      }
      return packObject(
          result,
          () => ByteRTCVideoByteWatermark(
              const NativeClassOptions([], disableInit: true)));
    } catch (e) {
      return null;
    }
  }

  set positionInPortraitMode(FutureOr<ByteRTCVideoByteWatermark?> value) {
    sendInstanceSet("positionInPortraitMode", value);
  }
}

enum ByteRTCVideoStreamScaleMode {
  /// @brief 自动缩放模式，默认设置为 ByteRTCVideoStreamScaleModeFitWithCropping。
  ///
  ByteRTCVideoStreamScaleModeAuto(0),

  /// @brief 对视频帧进行缩放，直至充满和视窗分辨率一致为止。这一过程不保证等比缩放。
  ///
  ByteRTCVideoStreamScaleModeStretch(1),

  /// @brief 视窗填满优先。 <br>
  ///        视频帧等比缩放，直至视窗被视频填满。如果视频帧长宽比例与视窗不同，视频帧的多出部分将无法显示。 <br>
  ///        缩放完成后，视频帧的一边长和视窗的对应边长一致，另一边长大于等于视窗对应边长。
  ///
  ByteRTCVideoStreamScaleModeFitWithCropping(2),

  /// @brief 视频帧内容全部显示优先。 <br>
  ///        视频帧等比缩放，直至视频帧能够在视窗上全部显示。如果视频帧长宽比例与视窗不同，视窗上未被视频帧填满区域将被涂黑。 <br>
  ///        缩放完成后，视频帧的一边长和视窗的对应边长一致，另一边长小于等于视窗对应边长。保持纵横比来缩放图像，填充短边
  ///
  ByteRTCVideoStreamScaleModeFitWithFilling(3);

  final dynamic $value;
  const ByteRTCVideoStreamScaleMode([this.$value]);
}

class ByteRTCEchoTestConfig extends NativeClass {
  static const _$namespace = r'ByteRTCEchoTestConfig';
  static get codegen_$namespace => _$namespace;

  ByteRTCEchoTestConfig([NativeClassOptions? options])
      : super(options == null
            ? const NativeClassOptions(
                [],
                className: _$namespace,
                instanceType: InstanceType.manual,
                bridgeKey: 'com.volcengine.rtc.hybrid_runtime',
              )
            : NativeClassOptions.fromMap({
                ...options.toMap(),
                'bridgeKey': 'com.volcengine.rtc.hybrid_runtime',
              }));

  /// @brief 用于渲染接收到的视频的视图
  FutureOr<ByteRTCView?> get view async {
    return await sendInstanceGet<ByteRTCView?>("view");
  }

  set view(FutureOr<ByteRTCView?> value) {
    sendInstanceSet("view", value);
  }

  /// @brief 测试用户加入的房间 ID。
  FutureOr<NSString?> get roomId async {
    return await sendInstanceGet<NSString?>("roomId");
  }

  set roomId(FutureOr<NSString?> value) {
    sendInstanceSet("roomId", value);
  }

  /// @brief 进行音视频通话回路测试的用户 ID
  FutureOr<NSString?> get userId async {
    return await sendInstanceGet<NSString?>("userId");
  }

  set userId(FutureOr<NSString?> value) {
    sendInstanceSet("userId", value);
  }

  /// @brief 对用户进房时进行鉴权验证的动态密钥，用于保证音视频通话回路测试的安全性。
  FutureOr<NSString?> get token async {
    return await sendInstanceGet<NSString?>("token");
  }

  set token(FutureOr<NSString?> value) {
    sendInstanceSet("token", value);
  }

  /// @brief 是否检测音频。检测设备为系统默认音频设备。 <br>
  ///        - true：是
  ///            - 若使用 SDK 内部采集，此时设备麦克风会自动开启，并在 audioReportInterval 值大于 0 时触发 `onLocalAudioPropertiesReport` 回调，你可以根据该回调判断麦克风的工作状态
  ///            - 若使用自定义采集，此时你需调用 pushExternalAudioFrame:{@link #ByteRTCEngine#pushExternalAudioFrame} 将采集到的音频推送给 SDK
  ///        - false：否
  FutureOr<BOOL?> get enableAudio async {
    return await sendInstanceGet<BOOL?>("enableAudio");
  }

  set enableAudio(FutureOr<BOOL?> value) {
    sendInstanceSet("enableAudio", value);
  }

  /// @brief 是否检测视频。PC 端默认检测列表中第一个视频设备。 <br>
  ///        - true：是
  ///            - 若使用 SDK 内部采集，此时设备摄像头会自动开启
  ///            - 若使用自定义采集，此时你需调用 pushExternalVideoFrame:{@link #ByteRTCEngine#pushExternalVideoFrame} 将采集到的视频推送给 SDK
  ///        - false：否
  ///        视频的发布参数固定为：分辨率 640px × 360px，帧率 15fps。
  FutureOr<BOOL?> get enableVideo async {
    return await sendInstanceGet<BOOL?>("enableVideo");
  }

  set enableVideo(FutureOr<BOOL?> value) {
    sendInstanceSet("enableVideo", value);
  }

  /// @brief 音量信息提示间隔，单位：ms，默认为 100ms <br>
  ///       - `<= 0`: 关闭信息提示
  ///       - `(0,100]`: 开启信息提示，不合法的 interval 值，SDK 自动设置为 100ms
  ///       - `> 100`: 开启信息提示，并将信息提示间隔设置为此值
  FutureOr<NSInteger?> get audioReportInterval async {
    return await sendInstanceGet<NSInteger?>("audioReportInterval");
  }

  set audioReportInterval(FutureOr<NSInteger?> value) {
    sendInstanceSet("audioReportInterval", value);
  }
}

enum ByteRTCScreenMediaType {
  /// @brief 只采集视频数据
  ///
  ByteRTCScreenMediaTypeVideoOnly(0),

  /// @brief 只采集音频数据
  ///
  ByteRTCScreenMediaTypeAudioOnly(1),

  /// @brief 音视频数据都采集
  ///
  ByteRTCScreenMediaTypeVideoAndAudio(2);

  final dynamic $value;
  const ByteRTCScreenMediaType([this.$value]);
}

enum ByteRTCFirstFrameSendState {
  /// @brief 发送中
  ///
  ByteRTCFirstFrameSendStateSending(0),

  /// @brief 发送成功
  ///
  ByteRTCFirstFrameSendStateSent(1),

  /// @brief 发送失败
  ///
  ByteRTCFirstFrameSendStateEnd(2);

  final dynamic $value;
  const ByteRTCFirstFrameSendState([this.$value]);
}

enum ByteRTCVideoSuperResolutionModeChangedReason {
  /// @brief 调用 setRemoteVideoSuperResolution:withMode:{@link #ByteRTCEngine#setRemoteVideoSuperResolution:withMode} 成功关闭超分。
  ///
  ByteRTCVideoSuperResolutionModeChangedReasonAPIOff(0),

  /// @brief 调用 setRemoteVideoSuperResolution:withMode:{@link #ByteRTCEngine#setRemoteVideoSuperResolution:withMode} 成功开启超分。
  ///
  ByteRTCVideoSuperResolutionModeChangedReasonAPIOn(1),

  /// @brief 开启超分失败，远端视频流的原始视频分辨率超过 640 × 360 px。
  ///
  ByteRTCVideoSuperResolutionModeChangedReasonResolutionExceed(2),

  /// @brief 开启超分失败，已对一路远端流开启超分。
  ///
  ByteRTCVideoSuperResolutionModeChangedReasonOverUse(3),

  /// @brief 设备不支持使用超分辨率。
  ///
  ByteRTCVideoSuperResolutionModeChangedReasonDeviceNotSupport(4),

  /// @brief 当前设备性能存在风险，已动态关闭
  ///
  ByteRTCVideoSuperResolutionModeChangedReasonDynamicClose(5),

  /// @brief 超分因其他原因关闭。
  ///
  ByteRTCVideoSuperResolutionModeChangedReasonOtherSettingDisabled(6),

  /// @brief 超分因其他原因开启。
  ///
  ByteRTCVideoSuperResolutionModeChangedReasonOtherSettingEnabled(7),

  /// @brief SDK 没有编译超分组件。
  ///
  ByteRTCVideoSuperResolutionModeChangedReasonNoComponent(8),

  /// @brief 远端流不存在。房间 ID 或用户 ID 无效，或对方没有发布流。
  ///
  ByteRTCVideoSuperResolutionModeChangedReasonStreamNotExist(9);

  final dynamic $value;
  const ByteRTCVideoSuperResolutionModeChangedReason([this.$value]);
}

class ByteRTCFaceDetectionResult extends NativeClass {
  static const _$namespace = r'ByteRTCFaceDetectionResult';
  static get codegen_$namespace => _$namespace;

  ByteRTCFaceDetectionResult([NativeClassOptions? options])
      : super(options == null
            ? const NativeClassOptions(
                [],
                className: _$namespace,
                instanceType: InstanceType.manual,
                bridgeKey: 'com.volcengine.rtc.hybrid_runtime',
              )
            : NativeClassOptions.fromMap({
                ...options.toMap(),
                'bridgeKey': 'com.volcengine.rtc.hybrid_runtime',
              }));

  /// @brief 人脸检测结果 <br>
  ///        - 0：检测成功
  ///        - !0：检测失败。详见[错误码](https://www.volcengine.com/docs/6705/102042)。
  FutureOr<int?> get detectResult async {
    return await sendInstanceGet<int?>("detectResult");
  }

  set detectResult(FutureOr<int?> value) {
    sendInstanceSet("detectResult", value);
  }

  /// @brief 原始图片宽度(px)
  FutureOr<int?> get imageWidth async {
    return await sendInstanceGet<int?>("imageWidth");
  }

  set imageWidth(FutureOr<int?> value) {
    sendInstanceSet("imageWidth", value);
  }

  /// @brief 原始图片高度(px)
  FutureOr<int?> get imageHeight async {
    return await sendInstanceGet<int?>("imageHeight");
  }

  set imageHeight(FutureOr<int?> value) {
    sendInstanceSet("imageHeight", value);
  }

  /// @brief 识别到人脸的矩形框。数组的长度和检测到的人脸数量一致。参看 ByteRTCRectangle{@link #ByteRTCRectangle}。
  FutureOr<NSArray<ByteRTCRectangle>?> get faces async {
    try {
      final result = await sendInstanceGet<NSArray<ByteRTCRectangle>?>("faces");
      if (result == null) {
        return null;
      }
      final list = result.map((e) => packObject(
          e,
          () => ByteRTCRectangle(
              const NativeClassOptions([], disableInit: true))));
      return list.toList();
    } catch (e) {
      return null;
    }
  }

  set faces(FutureOr<NSArray<ByteRTCRectangle>?> value) {
    sendInstanceSet("faces", value);
  }

  /// @brief 进行人脸识别的视频帧的时间戳。
  FutureOr<int?> get frameTimestamp async {
    return await sendInstanceGet<int?>("frameTimestamp");
  }

  set frameTimestamp(FutureOr<int?> value) {
    sendInstanceSet("frameTimestamp", value);
  }
}

enum ByteRTCStreamLayoutMode {
  /// @brief 自动布局
  ///
  ByteRTCStreamLayoutModeAuto(0),

  /// @brief 自定义
  ///
  ByteRTCStreamLayoutModeCustom(2);

  final dynamic $value;
  const ByteRTCStreamLayoutMode([this.$value]);
}

enum ByteRTCForwardStreamState {
  /// @brief 空闲状态 <br>
  ///        - 成功调用 `stopForwardStreamToRooms` 后，所有目标房间为空闲状态。
  ///        - 成功调用 `updateForwardStreamToRooms` 减少目标房间后，本次减少的目标房间为空闲状态。
  ///
  ByteRTCForwardStreamStateIdle(0),

  /// @brief 开始转发 <br>
  ///        - 调用 `startForwardStreamToRooms` 成功向所有房间开始转发媒体流后，返回此状态。
  ///        - 调用 `updateForwardStreamToRooms` 后，成功向新增目标房间开始转发媒体流后，返回此状态。
  ///
  ByteRTCForwardStreamStateSuccess(1),

  /// @brief 转发失败，失败详情参考 ByteRTCForwardStreamError{@link #ByteRTCForwardStreamError} <br>
  ///        调用 `startForwardStreamToRooms` 或 `updateForwardStreamToRooms` 后，如遇转发失败，返回此状态。
  ///
  ByteRTCForwardStreamStateFailure(2);

  final dynamic $value;
  const ByteRTCForwardStreamState([this.$value]);
}

enum ByteRTCMixedStreamType {
  /// @brief 通过服务端合流
  ///
  ByteRTCMixedStreamByServer(0),

  /// @brief 端云一体合流。SDK 智能决策在客户端或服务端完成合流。 <br>
  ///        使用前，请联系技术支持同学开通，否则不生效。
  ///
  ByteRTCMixedStreamByClient(1);

  final dynamic $value;
  const ByteRTCMixedStreamType([this.$value]);
}

enum ByteRTCRemoteUserPriority {
  /// @brief 用户优先级为低，默认值
  ///
  ByteRTCRemoteUserPriorityLow(0),

  /// @brief 用户优先级为正常
  ///
  ByteRTCRemoteUserPriorityMedium(100),

  /// @brief 用户优先级为高
  ///
  ByteRTCRemoteUserPriorityHigh(200);

  final dynamic $value;
  const ByteRTCRemoteUserPriority([this.$value]);
}

enum ByteRTCRenderMode {
  /// @brief 视窗填满优先，默认值。 <br>
  ///        视频尺寸等比缩放，直至视窗被填满。当视频尺寸与显示窗口尺寸不一致时，多出的视频将被截掉。
  ///
  ByteRTCRenderModeHidden(1),

  /// @brief 视频帧内容全部显示优先。 <br>
  ///        视频尺寸等比缩放，优先保证视频内容全部显示。当视频尺寸与显示窗口尺寸不一致时，会把窗口未被填满的区域填充成背景颜色。
  ///
  ByteRTCRenderModeFit(2),

  /// @brief 视频帧自适应画布。 <br>
  ///        视频尺寸非等比例缩放，把窗口充满。在此过程中，视频帧的长宽比例可能会发生变化。
  ///
  ByteRTCRenderModeFill(3);

  final dynamic $value;
  const ByteRTCRenderMode([this.$value]);
}

enum ByteRTCVideoStreamType {
  /// @brief 高分辨率视频流
  ///
  ByteRTCVideoStreamTypeHigh(0),

  /// @brief 低分辨率视频
  ///
  ByteRTCVideoStreamTypeLow(1);

  final dynamic $value;
  const ByteRTCVideoStreamType([this.$value]);
}

enum ByteRTCCameraID {
  /// @brief 前置摄像头
  ///
  ByteRTCCameraIDFront(0),

  /// @brief 后置摄像头
  ///
  ByteRTCCameraIDBack(1),

  /// @hidden currently not available
  /// @brief 外接摄像头
  ///
  ByteRTCCameraIDExternal(2),

  /// @brief 无效值
  ///
  ByteRTCCameraIDInvalid(3);

  final dynamic $value;
  const ByteRTCCameraID([this.$value]);
}

class ByteRTCRecordingInfo extends NativeClass {
  static const _$namespace = r'ByteRTCRecordingInfo';
  static get codegen_$namespace => _$namespace;

  ByteRTCRecordingInfo([NativeClassOptions? options])
      : super(options == null
            ? const NativeClassOptions(
                [],
                className: _$namespace,
                instanceType: InstanceType.manual,
                bridgeKey: 'com.volcengine.rtc.hybrid_runtime',
              )
            : NativeClassOptions.fromMap({
                ...options.toMap(),
                'bridgeKey': 'com.volcengine.rtc.hybrid_runtime',
              }));

  /// @brief 录制文件的绝对路径，包含文件名和文件后缀
  FutureOr<NSString?> get filePath async {
    return await sendInstanceGet<NSString?>("filePath");
  }

  set filePath(FutureOr<NSString?> value) {
    sendInstanceSet("filePath", value);
  }

  /// @brief 录制文件的视频编码类型，参看 ByteRTCVideoCodecType{@link #ByteRTCVideoCodecType}
  FutureOr<ByteRTCVideoCodecType?> get codecType async {
    try {
      final result = await sendInstanceGet<ByteRTCVideoCodecType?>("codecType");
      if (result == null) {
        return null;
      }
      return ByteRTCVideoCodecType.values
          .firstWhere((element) => element == result);
    } catch (e) {
      return null;
    }
  }

  set codecType(FutureOr<ByteRTCVideoCodecType?> value) {
    sendInstanceSet("codecType", value);
  }

  /// @brief 录制视频的宽，单位：像素。纯音频录制请忽略该字段
  FutureOr<NSInteger?> get width async {
    return await sendInstanceGet<NSInteger?>("width");
  }

  set width(FutureOr<NSInteger?> value) {
    sendInstanceSet("width", value);
  }

  /// @brief 录制视频的高，单位：像素。纯音频录制请忽略该字段
  FutureOr<NSInteger?> get height async {
    return await sendInstanceGet<NSInteger?>("height");
  }

  set height(FutureOr<NSInteger?> value) {
    sendInstanceSet("height", value);
  }
}

enum ByteRTCRecordingType {
  /// @brief 只录制音频
  ///
  ByteRTCRecordingTypeAudioOnly(0),

  /// @brief 只录制视频
  ///
  ByteRTCRecordingTypeVideoOnly(1),

  /// @brief 同时录制音频和视频
  ///
  ByteRTCRecordingTypeVideoAndAudio(2);

  final dynamic $value;
  const ByteRTCRecordingType([this.$value]);
}

class ByteRTCVideoCanvas extends NativeClass {
  static const _$namespace = r'ByteRTCVideoCanvas';
  static get codegen_$namespace => _$namespace;

  ByteRTCVideoCanvas([NativeClassOptions? options])
      : super(options == null
            ? const NativeClassOptions(
                [],
                className: _$namespace,
                instanceType: InstanceType.manual,
                bridgeKey: 'com.volcengine.rtc.hybrid_runtime',
              )
            : NativeClassOptions.fromMap({
                ...options.toMap(),
                'bridgeKey': 'com.volcengine.rtc.hybrid_runtime',
              }));

  /// @brief 本地视图句柄
  FutureOr<ByteRTCView?> get view async {
    return await sendInstanceGet<ByteRTCView?>("view");
  }

  set view(FutureOr<ByteRTCView?> value) {
    sendInstanceSet("view", value);
  }

  /// @brief 渲染模式，参看 ByteRTCRenderMode{@link #ByteRTCRenderMode}
  FutureOr<ByteRTCRenderMode?> get renderMode async {
    try {
      final result = await sendInstanceGet<ByteRTCRenderMode?>("renderMode");
      if (result == null) {
        return null;
      }
      return ByteRTCRenderMode.values
          .firstWhere((element) => element == result);
    } catch (e) {
      return null;
    }
  }

  set renderMode(FutureOr<ByteRTCRenderMode?> value) {
    sendInstanceSet("renderMode", value);
  }

  /// @brief 用于填充画布空白部分的背景颜色。取值范围是 `[0x00000000, 0xFFFFFFFF]`,格式为 BGR。默认值是 `0x00000000`。其中，透明度设置无效。
  FutureOr<NSInteger?> get backgroundColor async {
    return await sendInstanceGet<NSInteger?>("backgroundColor");
  }

  set backgroundColor(FutureOr<NSInteger?> value) {
    sendInstanceSet("backgroundColor", value);
  }

  /// @brief 视频帧旋转角度。参看 ByteRTCVideoRotation{@link #ByteRTCVideoRotation}。默认为 0 度，即不做旋转处理。 <br>
  ///        该设置仅对远端视频有效，对本地视频设置不生效。
  FutureOr<ByteRTCVideoRotation?> get renderRotation async {
    try {
      final result =
          await sendInstanceGet<ByteRTCVideoRotation?>("renderRotation");
      if (result == null) {
        return null;
      }
      return ByteRTCVideoRotation.values
          .firstWhere((element) => element == result);
    } catch (e) {
      return null;
    }
  }

  set renderRotation(FutureOr<ByteRTCVideoRotation?> value) {
    sendInstanceSet("renderRotation", value);
  }
}

class ByteRTCMediaPlayerConfig extends NativeClass {
  static const _$namespace = r'ByteRTCMediaPlayerConfig';
  static get codegen_$namespace => _$namespace;

  ByteRTCMediaPlayerConfig([NativeClassOptions? options])
      : super(options == null
            ? const NativeClassOptions(
                [],
                className: _$namespace,
                instanceType: InstanceType.manual,
                bridgeKey: 'com.volcengine.rtc.hybrid_runtime',
              )
            : NativeClassOptions.fromMap({
                ...options.toMap(),
                'bridgeKey': 'com.volcengine.rtc.hybrid_runtime',
              }));

  /// @brief 混音播放类型，详见 ByteRTCAudioMixingType{@link #ByteRTCAudioMixingType}
  FutureOr<ByteRTCAudioMixingType?> get type async {
    try {
      final result = await sendInstanceGet<ByteRTCAudioMixingType?>("type");
      if (result == null) {
        return null;
      }
      return ByteRTCAudioMixingType.values
          .firstWhere((element) => element == result);
    } catch (e) {
      return null;
    }
  }

  set type(FutureOr<ByteRTCAudioMixingType?> value) {
    sendInstanceSet("type", value);
  }

  /// @brief 混音播放次数 <br>
  ///       - play_count <= 0: 无限循环
  ///       - play_count == 1: 播放一次（默认）
  ///       - play_count > 1: 播放 play_count 次
  FutureOr<NSInteger?> get playCount async {
    return await sendInstanceGet<NSInteger?>("playCount");
  }

  set playCount(FutureOr<NSInteger?> value) {
    sendInstanceSet("playCount", value);
  }

  /// @brief 混音起始位置。默认值为 0，单位为毫秒。
  FutureOr<NSInteger?> get startPos async {
    return await sendInstanceGet<NSInteger?>("startPos");
  }

  set startPos(FutureOr<NSInteger?> value) {
    sendInstanceSet("startPos", value);
  }

  /// @brief 设置音频文件混音时，收到 onMediaPlayerPlayingProgress:progress:{@link #ByteRTCMediaPlayerEventHandler#onMediaPlayerPlayingProgress:progress} 的间隔。单位毫秒。 <br>
  ///       - interval > 0 时，触发回调。实际间隔为 10 的倍数。如果输入数值不能被 10 整除，将自动向上取整。例如传入 `52`，实际间隔为 60 ms。
  ///       - interval <= 0 时，不会触发回调。
  FutureOr<NSInteger?> get callbackOnProgressInterval async {
    return await sendInstanceGet<NSInteger?>("callbackOnProgressInterval");
  }

  set callbackOnProgressInterval(FutureOr<NSInteger?> value) {
    sendInstanceSet("callbackOnProgressInterval", value);
  }

  /// @brief 在采集音频数据时，附带本地混音文件播放进度的时间戳。启用此功能会提升远端人声和音频文件混音播放时的同步效果。 <br>
  ///        - 仅在单个音频文件混音时使用有效。
  ///        - `true` 时开启此功能，`false` 时关闭此功能，默认为关闭。
  FutureOr<BOOL?> get syncProgressToRecordFrame async {
    return await sendInstanceGet<BOOL?>("syncProgressToRecordFrame");
  }

  set syncProgressToRecordFrame(FutureOr<BOOL?> value) {
    sendInstanceSet("syncProgressToRecordFrame", value);
  }

  /// @brief 是否自动播放。如果不自动播放，调用 start{@link #ByteRTCMediaPlayer#start} 播放音乐文件。默认为 True。
  FutureOr<BOOL?> get autoPlay async {
    return await sendInstanceGet<BOOL?>("autoPlay");
  }

  set autoPlay(FutureOr<BOOL?> value) {
    sendInstanceSet("autoPlay", value);
  }
}

enum ByteRTCSyncInfoStreamType {
  /// @brief 音频流
  ///
  ByteRTCSyncInfoStreamTypeAudio(0);

  final dynamic $value;
  const ByteRTCSyncInfoStreamType([this.$value]);
}

enum ByteRTCLocalAudioStreamState {
  /// @brief 本地音频默认初始状态。 <br>
  ///        麦克风停止工作时回调该状态，对应错误码 ByteRTCLocalAudioStreamError{@link #ByteRTCLocalAudioStreamError} 中的 ByteRTCLocalAudioStreamErrorOk
  ///
  ByteRTCLocalAudioStreamStateStopped(0),

  /// @brief 本地音频录制设备启动成功。 <br>
  ///        采集到音频首帧时回调该状态，对应错误码 ByteRTCLocalAudioStreamError{@link #ByteRTCLocalAudioStreamError} 中的 ByteRTCLocalAudioStreamErrorOk
  ///
  ByteRTCLocalAudioStreamStateRecording(1),

  /// @brief 本地音频首帧编码成功。 <br>
  ///        音频首帧编码成功时回调该状态，对应错误码 ByteRTCLocalAudioStreamError{@link #ByteRTCLocalAudioStreamError} 中的 ByteRTCLocalAudioStreamErrorOk
  ///
  ByteRTCLocalAudioStreamStateEncoding(2),

  /// @brief 本地音频启动失败，在以下时机回调该状态： <br>
  ///       - 本地录音设备启动失败，对应错误码 ByteRTCLocalAudioStreamError{@link #ByteRTCLocalAudioStreamError} 中的 ByteRTCLocalAudioStreamErrorRecordFailure
  ///       - 检测到没有录音设备权限，对应错误码 ByteRTCLocalAudioStreamError{@link #ByteRTCLocalAudioStreamError} 中的 ByteRTCLocalAudioStreamErrorDeviceNoPermission
  ///       - 音频编码失败，对应错误码 ByteRTCLocalAudioStreamError{@link #ByteRTCLocalAudioStreamError} 中的 ByteRTCLocalAudioStreamError
  ///
  ByteRTCLocalAudioStreamStateFailed(3);

  final dynamic $value;
  const ByteRTCLocalAudioStreamState([this.$value]);
}

class ByteRTCRoomConfig extends NativeClass {
  static const _$namespace = r'ByteRTCRoomConfig';
  static get codegen_$namespace => _$namespace;

  ByteRTCRoomConfig([NativeClassOptions? options])
      : super(options == null
            ? const NativeClassOptions(
                [],
                className: _$namespace,
                instanceType: InstanceType.manual,
                bridgeKey: 'com.volcengine.rtc.hybrid_runtime',
              )
            : NativeClassOptions.fromMap({
                ...options.toMap(),
                'bridgeKey': 'com.volcengine.rtc.hybrid_runtime',
              }));

  /// @brief 房间模式，参看 ByteRTCRoomProfile{@link #ByteRTCRoomProfile}，默认为普通音视频通话模式，进房后不可更改。
  FutureOr<ByteRTCRoomProfile?> get profile async {
    try {
      final result = await sendInstanceGet<ByteRTCRoomProfile?>("profile");
      if (result == null) {
        return null;
      }
      return ByteRTCRoomProfile.values
          .firstWhere((element) => element == result);
    } catch (e) {
      return null;
    }
  }

  set profile(FutureOr<ByteRTCRoomProfile?> value) {
    sendInstanceSet("profile", value);
  }

  /// @brief 流 ID，默认为空。
  FutureOr<NSString?> get streamId async {
    return await sendInstanceGet<NSString?>("streamId");
  }

  set streamId(FutureOr<NSString?> value) {
    sendInstanceSet("streamId", value);
  }

  /// @brief 是否自动订阅音频流，默认为自动订阅。
  ///        + 若调用 `setUserVisibility` 将自身可见性设为 false，无论是默认的自动发布流还是手动设置的自动发布流都不会进行发布，你需要将自身可见性设为 true 后方可发布。
  ///        + 多房间场景下，若已在其中一个房间成功设置了自动发布，其他房间的自动发布设置均不会生效。若每个房间均不做设置，则默认在第一个加入的房间内自动发布流。
  ///
  FutureOr<BOOL?> get isPublishAudio async {
    return await sendInstanceGet<BOOL?>("isPublishAudio");
  }

  set isPublishAudio(FutureOr<BOOL?> value) {
    sendInstanceSet("isPublishAudio", value);
  }

  /// @brief 是否自动发布视频流，默认为自动发布。 <br>
  ///        + 若调用 setUserVisibility:{@link #ByteRTCRoom#setUserVisibility} 将自身可见性设为 false，无论是默认的自动发布流还是手动设置的自动发布流都不会进行发布，你需要将自身可见性设为 true 后方可发布。
  ///        + 多房间场景下，若已在其中一个房间成功设置了自动发布，其他房间的自动发布设置均不会生效。若每个房间均不做设置，则默认在第一个加入的房间内自动发布流。
  ///
  FutureOr<BOOL?> get isPublishVideo async {
    return await sendInstanceGet<BOOL?>("isPublishVideo");
  }

  set isPublishVideo(FutureOr<BOOL?> value) {
    sendInstanceSet("isPublishVideo", value);
  }

  /// @brief 是否自动订阅音频流，默认为自动订阅。 <br>
  ///        包含主流和屏幕流。
  FutureOr<BOOL?> get isAutoSubscribeAudio async {
    return await sendInstanceGet<BOOL?>("isAutoSubscribeAudio");
  }

  set isAutoSubscribeAudio(FutureOr<BOOL?> value) {
    sendInstanceSet("isAutoSubscribeAudio", value);
  }

  /// @brief 是否自动订阅主视频流，默认为自动订阅。 <br>
  ///        包含主流和屏幕流。
  FutureOr<BOOL?> get isAutoSubscribeVideo async {
    return await sendInstanceGet<BOOL?>("isAutoSubscribeVideo");
  }

  set isAutoSubscribeVideo(FutureOr<BOOL?> value) {
    sendInstanceSet("isAutoSubscribeVideo", value);
  }
}

class ByteRTCRemoteStreamKey extends NativeClass {
  static const _$namespace = r'ByteRTCRemoteStreamKey';
  static get codegen_$namespace => _$namespace;

  ByteRTCRemoteStreamKey([NativeClassOptions? options])
      : super(options == null
            ? const NativeClassOptions(
                [],
                className: _$namespace,
                instanceType: InstanceType.manual,
                bridgeKey: 'com.volcengine.rtc.hybrid_runtime',
              )
            : NativeClassOptions.fromMap({
                ...options.toMap(),
                'bridgeKey': 'com.volcengine.rtc.hybrid_runtime',
              }));

  /// @brief 用户 ID
  FutureOr<NSString?> get userId async {
    return await sendInstanceGet<NSString?>("userId");
  }

  set userId(FutureOr<NSString?> value) {
    sendInstanceSet("userId", value);
  }

  /// @brief 房间 ID
  FutureOr<NSString?> get roomId async {
    return await sendInstanceGet<NSString?>("roomId");
  }

  set roomId(FutureOr<NSString?> value) {
    sendInstanceSet("roomId", value);
  }

  /// @brief 流属性，包括主流、屏幕流。参看 ByteRTCStreamIndex{@link #ByteRTCStreamIndex}
  FutureOr<ByteRTCStreamIndex?> get streamIndex async {
    try {
      final result = await sendInstanceGet<ByteRTCStreamIndex?>("streamIndex");
      if (result == null) {
        return null;
      }
      return ByteRTCStreamIndex.values
          .firstWhere((element) => element == result);
    } catch (e) {
      return null;
    }
  }

  set streamIndex(FutureOr<ByteRTCStreamIndex?> value) {
    sendInstanceSet("streamIndex", value);
  }
}

enum ByteRTCAudioQuality {
  /// @brief 低音质
  ///
  ByteRTCAudioQualityLow(0),

  /// @brief 中音质
  ///
  ByteRTCAudioQualityMedium(1),

  /// @brief 高音质
  ///
  ByteRTCAudioQualityHigh(2),

  /// @brief 超高音质
  ///
  ByteRTCAudioQualityUltraHigh(3);

  final dynamic $value;
  const ByteRTCAudioQuality([this.$value]);
}

enum ByteRTCLyricStatus {
  /// @brief 无歌词。
  ///
  ByteRTCLyricStatusNone(0),

  /// @brief KRC 歌词。
  ///
  ByteRTCLyricStatusKRC(1),

  /// @brief LRC 歌词。
  ///
  ByteRTCLyricStatusLRC(2),

  /// @brief KRC 歌词和 LRC 歌词均有。
  ///
  ByteRTCLyricStatusKRCAndLRC(3);

  final dynamic $value;
  const ByteRTCLyricStatus([this.$value]);
}

enum ByteRTCHardwareEchoDetectionResult {
  /// @brief 主动调用 `stopHardwareEchoDetection` 结束流程时，未有回声检测结果。
  ///
  ByteRTCHardwareEchoDetectionCanceled(0),

  /// @brief 未检测出结果。建议重试，如果仍然失败请联系技术支持协助排查。
  ///
  ByteRTCHardwareEchoDetectionUnknown(1),

  /// @brief 无回声
  ///
  ByteRTCHardwareEchoDetectionNormal(2),

  /// @brief 有回声。可通过 UI 建议用户使用耳机设备入会。
  ///
  ByteRTCHardwareEchoDetectionPoor(3);

  final dynamic $value;
  const ByteRTCHardwareEchoDetectionResult([this.$value]);
}

class ByteRTCPushSingleStreamParam extends NativeClass {
  static const _$namespace = r'ByteRTCPushSingleStreamParam';
  static get codegen_$namespace => _$namespace;

  ByteRTCPushSingleStreamParam([NativeClassOptions? options])
      : super(options == null
            ? const NativeClassOptions(
                [],
                className: _$namespace,
                instanceType: InstanceType.manual,
                bridgeKey: 'com.volcengine.rtc.hybrid_runtime',
              )
            : NativeClassOptions.fromMap({
                ...options.toMap(),
                'bridgeKey': 'com.volcengine.rtc.hybrid_runtime',
              }));

  /// @brief 推流 CDN 地址。仅支持 RTMP 协议，Url 必须满足正则 `/^rtmps?:\\/\\//`。
  FutureOr<NSString?> get url async {
    return await sendInstanceGet<NSString?>("url");
  }

  set url(FutureOr<NSString?> value) {
    sendInstanceSet("url", value);
  }

  /// @brief 媒体流所在的房间 ID
  FutureOr<NSString?> get roomId async {
    return await sendInstanceGet<NSString?>("roomId");
  }

  set roomId(FutureOr<NSString?> value) {
    sendInstanceSet("roomId", value);
  }

  /// @brief 媒体流所属的用户 ID
  FutureOr<NSString?> get userId async {
    return await sendInstanceGet<NSString?>("userId");
  }

  set userId(FutureOr<NSString?> value) {
    sendInstanceSet("userId", value);
  }

  /// @brief 媒体流是否为屏幕流。
  FutureOr<BOOL?> get isScreen async {
    return await sendInstanceGet<BOOL?>("isScreen");
  }

  set isScreen(FutureOr<BOOL?> value) {
    sendInstanceSet("isScreen", value);
  }

  /// @brief 转推目标房间数组，默认值为nullptr
  FutureOr<NSArray<DestInfo>?> get destInfo async {
    try {
      final result = await sendInstanceGet<NSArray<DestInfo>?>("destInfo");
      if (result == null) {
        return null;
      }
      final list = result.map((e) => packObject(
          e, () => DestInfo(const NativeClassOptions([], disableInit: true))));
      return list.toList();
    } catch (e) {
      return null;
    }
  }

  set destInfo(FutureOr<NSArray<DestInfo>?> value) {
    sendInstanceSet("destInfo", value);
  }

  /// @brief 转推类型，默认值转推CDN
  FutureOr<ByteRTCSingleStreamPushType?> get pushType async {
    try {
      final result =
          await sendInstanceGet<ByteRTCSingleStreamPushType?>("pushType");
      if (result == null) {
        return null;
      }
      return ByteRTCSingleStreamPushType.values
          .firstWhere((element) => element == result);
    } catch (e) {
      return null;
    }
  }

  set pushType(FutureOr<ByteRTCSingleStreamPushType?> value) {
    sendInstanceSet("pushType", value);
  }
}

class ByteRTCFrameUpdateInfo extends NativeClass {
  static const _$namespace = r'ByteRTCFrameUpdateInfo';
  static get codegen_$namespace => _$namespace;

  ByteRTCFrameUpdateInfo([NativeClassOptions? options])
      : super(options == null
            ? const NativeClassOptions(
                [],
                className: _$namespace,
                instanceType: InstanceType.manual,
                bridgeKey: 'com.volcengine.rtc.hybrid_runtime',
              )
            : NativeClassOptions.fromMap({
                ...options.toMap(),
                'bridgeKey': 'com.volcengine.rtc.hybrid_runtime',
              }));

  /// @brief 分辨率积（宽*高）。
  FutureOr<int?> get pixel async {
    return await sendInstanceGet<int?>("pixel");
  }

  set pixel(FutureOr<int?> value) {
    sendInstanceSet("pixel", value);
  }

  /// @brief 帧率。
  FutureOr<int?> get framerate async {
    return await sendInstanceGet<int?>("framerate");
  }

  set framerate(FutureOr<int?> value) {
    sendInstanceSet("framerate", value);
  }
}

class ByteRTCStreamInfo extends NativeClass {
  static const _$namespace = r'ByteRTCStreamInfo';
  static get codegen_$namespace => _$namespace;

  ByteRTCStreamInfo([NativeClassOptions? options])
      : super(options == null
            ? const NativeClassOptions(
                [],
                className: _$namespace,
                instanceType: InstanceType.manual,
                bridgeKey: 'com.volcengine.rtc.hybrid_runtime',
              )
            : NativeClassOptions.fromMap({
                ...options.toMap(),
                'bridgeKey': 'com.volcengine.rtc.hybrid_runtime',
              }));

  /// @brief 流 ID
  FutureOr<NSString?> get streamId async {
    return await sendInstanceGet<NSString?>("streamId");
  }

  set streamId(FutureOr<NSString?> value) {
    sendInstanceSet("streamId", value);
  }

  /// @brief 用户 ID
  FutureOr<NSString?> get userId async {
    return await sendInstanceGet<NSString?>("userId");
  }

  set userId(FutureOr<NSString?> value) {
    sendInstanceSet("userId", value);
  }

  /// @brief 房间 ID
  FutureOr<NSString?> get roomId async {
    return await sendInstanceGet<NSString?>("roomId");
  }

  set roomId(FutureOr<NSString?> value) {
    sendInstanceSet("roomId", value);
  }

  /// @brief 流索引
  FutureOr<NSInteger?> get streamIndex async {
    return await sendInstanceGet<NSInteger?>("streamIndex");
  }

  set streamIndex(FutureOr<NSInteger?> value) {
    sendInstanceSet("streamIndex", value);
  }

  /// @brief 流属性，是否为屏幕流
  FutureOr<BOOL?> get isScreen async {
    return await sendInstanceGet<BOOL?>("isScreen");
  }

  set isScreen(FutureOr<BOOL?> value) {
    sendInstanceSet("isScreen", value);
  }
}

enum ByteRTCAudioChannel {
  /// @brief 默认设置。默认值为 `2`。
  ///
  ByteRTCAudioChannelAuto(-1),

  /// @brief 单声道
  ///
  ByteRTCAudioChannelMono(1),

  /// @brief 双声道
  ///
  ByteRTCAudioChannelStereo(2);

  final dynamic $value;
  const ByteRTCAudioChannel([this.$value]);
}

enum ByteRTCSubtitleState {
  /// @brief 开启字幕。
  ///
  ByteRTCSubtitleStateStarted(0),

  /// @brief 关闭字幕。
  ///
  ByteRTCSubtitleStateStoped(1),

  /// @brief 字幕任务出现错误。
  ///
  ByteRTCSubtitleStateError(2);

  final dynamic $value;
  const ByteRTCSubtitleState([this.$value]);
}

class ByteRTCVideoFrame extends NativeObserverClass {
  static const _$namespace = r'ByteRTCVideoFrame';

  ByteRTCVideoFrame([NativeClassOptions? options])
      : super(options == null
            ? const NativeClassOptions([],
                className: _$namespace,
                instanceType: InstanceType.manual,
                bridgeKey: 'com.volcengine.rtc.hybrid_runtime',
                methodMap: {r"addRef": r"addRef"})
            : NativeClassOptions.fromMap({
                ...options.toMap(),
                'bridgeKey': 'com.volcengine.rtc.hybrid_runtime',
              })) {
    registerEvent(r"addRef", addRef);
  }

  /// @brief 视频帧引用计数加一
  /// @note 视频帧消费者希望对视频帧进行异步处理时（例如切换线程进行渲染），需要调用此接口增加引用计数。异步处理结束则需要调用 `releaseRef` 使引用计数减1

  FutureOr<void> addRef() async {}
}

class ByteRTCLocalAudioPropertiesInfo extends NativeClass {
  static const _$namespace = r'ByteRTCLocalAudioPropertiesInfo';
  static get codegen_$namespace => _$namespace;

  ByteRTCLocalAudioPropertiesInfo([NativeClassOptions? options])
      : super(options == null
            ? const NativeClassOptions(
                [],
                className: _$namespace,
                instanceType: InstanceType.manual,
                bridgeKey: 'com.volcengine.rtc.hybrid_runtime',
              )
            : NativeClassOptions.fromMap({
                ...options.toMap(),
                'bridgeKey': 'com.volcengine.rtc.hybrid_runtime',
              }));

  /// @detail keytype
  /// @brief 音频属性信息，详见 ByteRTCAudioPropertiesInfo{@link #ByteRTCAudioPropertiesInfo}
  FutureOr<ByteRTCAudioPropertiesInfo?> get audioPropertiesInfo async {
    try {
      final result = await sendInstanceGet<ByteRTCAudioPropertiesInfo?>(
          "audioPropertiesInfo");
      if (result == null) {
        return null;
      }
      return packObject(
          result,
          () => ByteRTCAudioPropertiesInfo(
              const NativeClassOptions([], disableInit: true)));
    } catch (e) {
      return null;
    }
  }

  set audioPropertiesInfo(FutureOr<ByteRTCAudioPropertiesInfo?> value) {
    sendInstanceSet("audioPropertiesInfo", value);
  }
}

enum ByteRTCAudioFrameSource {
  /// @brief 本地麦克风采集的音频数据。
  ///
  ByteRTCAudioFrameSourceTypeMic(0),

  /// @brief 远端所有用户混音后的数据
  ///
  ByteRTCAudioFrameSourceTypePlayback(1),

  /// @brief 本地麦克风和所有远端用户音频流的混音后的数据
  ///
  ByteRTCAudioFrameSourceTypeMixed(2);

  final dynamic $value;
  const ByteRTCAudioFrameSource([this.$value]);
}

class ByteRTCMixedStreamLayoutRegionImageWaterMarkConfig extends NativeClass {
  static const _$namespace =
      r'ByteRTCMixedStreamLayoutRegionImageWaterMarkConfig';
  static get codegen_$namespace => _$namespace;

  ByteRTCMixedStreamLayoutRegionImageWaterMarkConfig(
      [NativeClassOptions? options])
      : super(options == null
            ? const NativeClassOptions(
                [],
                className: _$namespace,
                instanceType: InstanceType.manual,
                bridgeKey: 'com.volcengine.rtc.hybrid_runtime',
              )
            : NativeClassOptions.fromMap({
                ...options.toMap(),
                'bridgeKey': 'com.volcengine.rtc.hybrid_runtime',
              }));

  /// @brief 原始图片的宽度，单位为 px。
  FutureOr<NSInteger?> get imageWidth async {
    return await sendInstanceGet<NSInteger?>("imageWidth");
  }

  set imageWidth(FutureOr<NSInteger?> value) {
    sendInstanceSet("imageWidth", value);
  }

  /// @brief 原始图片的高度，单位为 px。
  FutureOr<NSInteger?> get imageHeight async {
    return await sendInstanceGet<NSInteger?>("imageHeight");
  }

  set imageHeight(FutureOr<NSInteger?> value) {
    sendInstanceSet("imageHeight", value);
  }
}

enum ByteRTCMouseCursorCaptureState {
  /// @brief 采集鼠标信息。
  ///
  ByteRTCMouseCursorCaptureStateOn(0),

  /// @brief 不采集鼠标信息。
  ///
  ByteRTCMouseCursorCaptureStateOff(1);

  final dynamic $value;
  const ByteRTCMouseCursorCaptureState([this.$value]);
}

enum ByteRTCInterpolationMode {
  /// @detail keytype
  /// @brief 补最后一帧
  ///
  ByteRTCInterpolationModeLastFrameFill(0),

  /// @detail keytype
  /// @brief 补背景图片
  ///
  ByteRTCInterpolationModeBackgroundImageFill(1);

  final dynamic $value;
  const ByteRTCInterpolationMode([this.$value]);
}

enum ByteRTCRecordingState {
  /// @brief 录制异常
  ///
  ByteRTCRecordingStateError(0),

  /// @brief 录制进行中
  ///
  ByteRTCRecordingStateProcessing(1),

  /// @brief 录制文件保存成功，调用 `stopFileRecording:` 结束录制之后才会收到该状态码。
  ///
  ByteRTCRecordingStateSuccess(2);

  final dynamic $value;
  const ByteRTCRecordingState([this.$value]);
}

enum ByteRTCLocalProxyError {
  /// @brief 本地代理服务器无错误。
  ///
  ByteRTCLocalProxyErrorOK(0),

  /// @brief 代理服务器回复的版本号不符合 Socks5 协议标准文档的规定，导致 Socks5 代理连接失败。请检查代理服务器是否存在异常。
  ///
  ByteRTCLocalProxyErrorSocks5VersionError(1),

  /// @brief 代理服务器回复的格式错误不符合 Socks5 协议标准文档的规定，导致 Socks5 代理连接失败。请检查代理服务器是否存在异常。
  ///
  ByteRTCLocalProxyErrorSocks5FormatError(2),

  /// @brief 代理服务器回复的字段值不符合 Socks5 协议标准文档的规定，导致 Socks5 代理连接失败。请检查代理服务器是否存在异常。
  ///
  ByteRTCLocalProxyErrorSocks5InvalidValue(3),

  /// @brief 未提供代理服务器的用户名及密码，导致 Socks5 代理连接失败。请重新调用 `setLocalProxy`，在设置本地代理时填入用户名和密码。
  ///
  ByteRTCLocalProxyErrorSocks5UserPassNotGiven(4),

  /// @brief TCP 关闭，导致 Socks5 代理连接失败。请检查网络或者代理服务器是否存在异常。
  ///
  ByteRTCLocalProxyErrorSocks5TcpClosed(5),

  /// @brief Http 隧道代理错误。请检查 Http 隧道代理服务器或者网络是否存在异常。
  ///
  ByteRTCLocalProxyErrorHttpTunnelFailed(6);

  final dynamic $value;
  const ByteRTCLocalProxyError([this.$value]);
}

class ByteRTCRemoteVideoSinkConfig extends NativeClass {
  static const _$namespace = r'ByteRTCRemoteVideoSinkConfig';
  static get codegen_$namespace => _$namespace;

  ByteRTCRemoteVideoSinkConfig([NativeClassOptions? options])
      : super(options == null
            ? const NativeClassOptions(
                [],
                className: _$namespace,
                instanceType: InstanceType.manual,
                bridgeKey: 'com.volcengine.rtc.hybrid_runtime',
              )
            : NativeClassOptions.fromMap({
                ...options.toMap(),
                'bridgeKey': 'com.volcengine.rtc.hybrid_runtime',
              }));

  /// @brief 远端视频帧回调位置，参看 ByteRTCLocalVideoSinkPosition{@link #ByteRTCLocalVideoSinkPosition}，默认回调后处理后的视频帧。
  FutureOr<ByteRTCRemoteVideoSinkPosition?> get position async {
    try {
      final result =
          await sendInstanceGet<ByteRTCRemoteVideoSinkPosition?>("position");
      if (result == null) {
        return null;
      }
      return ByteRTCRemoteVideoSinkPosition.values
          .firstWhere((element) => element == result);
    } catch (e) {
      return null;
    }
  }

  set position(FutureOr<ByteRTCRemoteVideoSinkPosition?> value) {
    sendInstanceSet("position", value);
  }

  /// @brief 远端视频帧回调格式，参看 ByteRTCVideoSinkPixelFormat{@link #ByteRTCVideoSinkPixelFormat}，默认值为 0。
  FutureOr<ByteRTCVideoSinkPixelFormat?> get requiredPixelFormat async {
    try {
      final result = await sendInstanceGet<ByteRTCVideoSinkPixelFormat?>(
          "requiredPixelFormat");
      if (result == null) {
        return null;
      }
      return ByteRTCVideoSinkPixelFormat.values
          .firstWhere((element) => element == result);
    } catch (e) {
      return null;
    }
  }

  set requiredPixelFormat(FutureOr<ByteRTCVideoSinkPixelFormat?> value) {
    sendInstanceSet("requiredPixelFormat", value);
  }

  /// @brief 是否将视频帧自动转正，参看 ByteRTCVideoApplyRotation{@link #ByteRTCVideoApplyRotation}，默认为不旋转。
  FutureOr<ByteRTCVideoApplyRotation?> get applyRotation async {
    try {
      final result =
          await sendInstanceGet<ByteRTCVideoApplyRotation?>("applyRotation");
      if (result == null) {
        return null;
      }
      return ByteRTCVideoApplyRotation.values
          .firstWhere((element) => element == result);
    } catch (e) {
      return null;
    }
  }

  set applyRotation(FutureOr<ByteRTCVideoApplyRotation?> value) {
    sendInstanceSet("applyRotation", value);
  }

  /// @brief 是否将视频帧镜像。参看 ByteRTCVideoSinkMirrorType{@link #ByteRTCVideoSinkMirrorType}，默认为不镜像。 <br>
  ///        本设置与 setRemoteVideoMirrorType:withMirrorType:{@link #ByteRTCEngine#setRemoteVideoMirrorType:withMirrorType} （适用于内部渲染）相互独立。
  FutureOr<ByteRTCVideoSinkMirrorType?> get mirrorType async {
    try {
      final result =
          await sendInstanceGet<ByteRTCVideoSinkMirrorType?>("mirrorType");
      if (result == null) {
        return null;
      }
      return ByteRTCVideoSinkMirrorType.values
          .firstWhere((element) => element == result);
    } catch (e) {
      return null;
    }
  }

  set mirrorType(FutureOr<ByteRTCVideoSinkMirrorType?> value) {
    sendInstanceSet("mirrorType", value);
  }
}

enum ByteRTCPlayerState {
  /// @brief 播放未启动
  ///
  ByteRTCPlayerStateIdle(0),

  /// @brief 已加载
  ///
  ByteRTCPlayerStatePreloaded(1),

  /// @brief 已打开
  ///
  ByteRTCPlayerStateOpened(2),

  /// @brief 正在播放
  ///
  ByteRTCPlayerStatePlaying(3),

  /// @brief 播放已暂停
  ///
  ByteRTCPlayerStatePaused(4),

  /// @brief 播放已被主动停止
  ///
  ByteRTCPlayerStateStopped(5),

  /// @brief 播放失败
  ///
  ByteRTCPlayerStateFailed(6),

  /// @brief 播放自然结束
  ///
  ByteRTCPlayerStateFinished(7),

  /// @brief 循环播放已结束
  ///

  ByteRTCPlayerStateLoopFinished(8);

  final dynamic $value;
  const ByteRTCPlayerState([this.$value]);
}

enum ByteRTCRemoteAudioState {
  /// @brief 远端音频流默认初始状态，在以下时机回调该状态： <br>
  ///       - 本地用户停止接收远端音频流，对应原因是 ByteRTCRemoteAudioStateChangeReason{@link #ByteRTCRemoteAudioStateChangeReason} 中的 `ByteRTCRemoteAudioStateChangeReasonLocalMuted`
  ///       - 远端用户停止发送音频流，对应原因是 ByteRTCRemoteAudioStateChangeReason{@link #ByteRTCRemoteAudioStateChangeReason} 中的 `ByteRTCRemoteAudioStateChangeReasonRemoteMuted`
  ///       - 远端用户离开房间，对应原因是 ByteRTCRemoteAudioStateChangeReason{@link #ByteRTCRemoteAudioStateChangeReason} 中的 `ByteRTCRemoteAudioStateChangeReasonRemoteOffline`
  ///
  ByteRTCRemoteAudioStateStopped(0),

  /// @brief 开始接收远端音频流首包。
  ///
  ByteRTCRemoteAudioStateStarting(1),

  /// @brief 远端音频流正在解码，正常播放，在以下时机回调该状态： <br>
  ///       - 成功解码远端音频首帧，对应原因是 ByteRTCRemoteAudioStateChangeReason{@link #ByteRTCRemoteAudioStateChangeReason} 中的 `ByteRTCRemoteAudioStateChangeReasonLocalUnmuted`
  ///       - 网络由阻塞恢复正常，对应原因是 ByteRTCRemoteAudioStateChangeReason{@link #ByteRTCRemoteAudioStateChangeReason} 中的 `ByteRTCRemoteAudioStateChangeReasonNetworkRecovery`
  ///       - 本地用户恢复接收远端音频流，对应原因是 ByteRTCRemoteAudioStateChangeReason{@link #ByteRTCRemoteAudioStateChangeReason} 中的 `ByteRTCRemoteAudioStateChangeReasonLocalUnmuted`
  ///       - 远端用户恢复发送音频流，对应原因是 ByteRTCRemoteAudioStateChangeReason{@link #ByteRTCRemoteAudioStateChangeReason} 中的 `ByteRTCRemoteAudioStateChangeReasonRemoteUnmuted`
  ///
  ByteRTCRemoteAudioStateDecoding(2),

  /// @brief 远端音频流卡顿。
  ///
  ByteRTCRemoteAudioStateFrozen(3),

  /// @hidden currently not available
  /// @brief 远端音频流播放失败
  /// @note 该错误码暂未使用
  ///
  ByteRTCRemoteAudioStateFailed(4);

  final dynamic $value;
  const ByteRTCRemoteAudioState([this.$value]);
}

enum GameRoomType {
  /// @brief 小队房间
  ///
  GameRoomTypeTeam(0),

  /// @brief 世界房间
  ///
  GameRoomTypeWorld(1);

  final dynamic $value;
  const GameRoomType([this.$value]);
}

class ByteRTCPosition extends NativeClass {
  static const _$namespace = r'ByteRTCPosition';
  static get codegen_$namespace => _$namespace;

  ByteRTCPosition([NativeClassOptions? options])
      : super(options == null
            ? const NativeClassOptions(
                [],
                className: _$namespace,
                instanceType: InstanceType.manual,
                bridgeKey: 'com.volcengine.rtc.hybrid_runtime',
              )
            : NativeClassOptions.fromMap({
                ...options.toMap(),
                'bridgeKey': 'com.volcengine.rtc.hybrid_runtime',
              }));

  /// @brief x 坐标
  FutureOr<float?> get x async {
    return await sendInstanceGet<float?>("x");
  }

  set x(FutureOr<float?> value) {
    sendInstanceSet("x", value);
  }

  /// @brief y 坐标
  FutureOr<float?> get y async {
    return await sendInstanceGet<float?>("y");
  }

  set y(FutureOr<float?> value) {
    sendInstanceSet("y", value);
  }

  /// @brief z 坐标
  FutureOr<float?> get z async {
    return await sendInstanceGet<float?>("z");
  }

  set z(FutureOr<float?> value) {
    sendInstanceSet("z", value);
  }
}

enum ByteRTCAVSyncEvent {
  /// @brief 音视频同步失败。<br>
  ///        当前音频源已与其他视频源关联同步关系。 <br>
  ///        单个音频源不支持与多个视频源同时同步。
  ///
  ByteRTCAVSyncEventInvalidUidRepeated(0);

  final dynamic $value;
  const ByteRTCAVSyncEvent([this.$value]);
}

enum ByteRTCMixedStreamVideoType {
  /// @brief 主流。包括： <br>
  ///        - 由摄像头/麦克风通过内部采集机制，采集到的流
  ///        - 通过自定义采集，采集到的流。
  ///
  ByteRTCMixedStreamVideoTypeMain(0),

  /// @brief 屏幕流。
  ///
  ByteRTCMixedStreamVideoTypeScreen(1);

  final dynamic $value;
  const ByteRTCMixedStreamVideoType([this.$value]);
}

enum ByteRTCSEIStreamEventType {
  /// @brief 远端用户发布黑帧视频流。 <br>
  ///        纯语音通话场景下，远端用户调用 sendSEIMessage:andRepeatCount:andCountPerFrame:{@link #ByteRTCEngine#sendSEIMessage:andRepeatCount:andCountPerFrame} 发送 SEI 数据时，SDK 会自动发布一路黑帧视频流，并触发该回调。
  ///
  ByteRTCSEIStreamEventTypeStreamAdd(0),

  /// @brief 远端黑帧视频流移除。该回调的触发时机包括： <br>
  ///        - 远端用户开启摄像头采集，由语音通话切换至视频通话，黑帧视频流停止发布；
  ///        - 远端用户调用 sendSEIMessage:andRepeatCount:andCountPerFrame:{@link #ByteRTCEngine#sendSEIMessage:andRepeatCount:andCountPerFrame} 后 1min 内未有 SEI 数据发送，黑帧视频流停止发布；
  ///        - 远端用户调用 setVideoSourceType:{@link #ByteRTCEngine#setVideoSourceType} 切换至自定义视频采集时，黑帧视频流停止发布。
  ///
  ByteRTCSEIStreamEventTypeStreamRemove(1);

  final dynamic $value;
  const ByteRTCSEIStreamEventType([this.$value]);
}

enum ByteRTCSubtitleMode {
  /// @brief 识别模式。在此模式下，房间内用户语音会被转为文字。
  ///
  ByteRTCSubtitleModeRecognition(0),

  /// @brief 翻译模式。在此模式下，房间内用户语音会先被转为文字，再被翻译为目标语言。
  ///
  ByteRTCSubtitleModeTranslation(1);

  final dynamic $value;
  const ByteRTCSubtitleMode([this.$value]);
}

class ByteRTCVideoSolution extends NativeClass {
  static const _$namespace = r'ByteRTCVideoSolution';
  static get codegen_$namespace => _$namespace;

  ByteRTCVideoSolution([NativeClassOptions? options])
      : super(options == null
            ? const NativeClassOptions(
                [],
                className: _$namespace,
                instanceType: InstanceType.manual,
                bridgeKey: 'com.volcengine.rtc.hybrid_runtime',
              )
            : NativeClassOptions.fromMap({
                ...options.toMap(),
                'bridgeKey': 'com.volcengine.rtc.hybrid_runtime',
              }));

  /// @brief 视频分辨率
  FutureOr<CGSize?> get videoSize async {
    return await sendInstanceGet<CGSize?>("videoSize");
  }

  set videoSize(FutureOr<CGSize?> value) {
    sendInstanceSet("videoSize", value);
  }

  /// @brief 视频预设帧率
  FutureOr<NSInteger?> get frameRate async {
    return await sendInstanceGet<NSInteger?>("frameRate");
  }

  set frameRate(FutureOr<NSInteger?> value) {
    sendInstanceSet("frameRate", value);
  }

  /// @brief 最高编码码率（千比特每秒）。建议使用 `-1`，SDK 会自动根据分辨率和帧率适配合适的码率。
  FutureOr<NSInteger?> get maxKbps async {
    return await sendInstanceGet<NSInteger?>("maxKbps");
  }

  set maxKbps(FutureOr<NSInteger?> value) {
    sendInstanceSet("maxKbps", value);
  }

  /// @brief 最低编码码率（千比特每秒）
  FutureOr<NSInteger?> get minKbps async {
    return await sendInstanceGet<NSInteger?>("minKbps");
  }

  set minKbps(FutureOr<NSInteger?> value) {
    sendInstanceSet("minKbps", value);
  }

  /// @brief 视频编码质量策略，参见 ByteRTCVideoEncoderPreference{@link #ByteRTCVideoEncoderPreference}
  FutureOr<ByteRTCVideoEncoderPreference?> get encoderPreference async {
    try {
      final result = await sendInstanceGet<ByteRTCVideoEncoderPreference?>(
          "encoderPreference");
      if (result == null) {
        return null;
      }
      return ByteRTCVideoEncoderPreference.values
          .firstWhere((element) => element == result);
    } catch (e) {
      return null;
    }
  }

  set encoderPreference(FutureOr<ByteRTCVideoEncoderPreference?> value) {
    sendInstanceSet("encoderPreference", value);
  }
}

enum ByteRTCColorSpace {
  /// @brief 未知的颜色空间，默认使用 kColorSpaceYCbCrBT601LimitedRange 颜色空间
  ///
  ByteRTCColorSpaceUnknown(0),

  /// @brief BT.601 数字编码标准，颜色空间[16-235]
  ///
  ByteRTCColorSpaceYCbCrBT601LimitedRange(1),

  ByteRTCColorSpaceYCbCrBT601FullRange(2),

  /// @brief BT.7091 数字编码标准，颜色空间[16-235]
  ///
  ByteRTCColorSpaceYCbCrBT709LimitedRange(3),

  /// @brief BT.7091 数字编码标准，颜色空间[0-255]
  ///
  ByteRTCColorSpaceYCbCrBT709FullRange(4);

  final dynamic $value;
  const ByteRTCColorSpace([this.$value]);
}

enum ByteRTCLocalProxyState {
  /// @brief TCP 代理服务器连接成功。
  ///
  ByteRTCLocalProxyStateInited(0),

  /// @brief 本地代理连接成功。
  ///
  ByteRTCLocalProxyStateConnected(1),

  /// @brief 本地代理连接出现错误。
  ///
  ByteRTCLocalProxyStateError(2);

  final dynamic $value;
  const ByteRTCLocalProxyState([this.$value]);
}

enum ByteRTCAudioSourceType {
  /// @brief 自定义采集音频源
  ///
  ByteRTCAudioSourceTypeExternal(0),

  /// @brief RTC SDK 内部采集音频源
  ///
  ByteRTCAudioSourceTypeInternal(1);

  final dynamic $value;
  const ByteRTCAudioSourceType([this.$value]);
}

class ByteRTCRemoteAudioStats extends NativeClass {
  static const _$namespace = r'ByteRTCRemoteAudioStats';
  static get codegen_$namespace => _$namespace;

  ByteRTCRemoteAudioStats([NativeClassOptions? options])
      : super(options == null
            ? const NativeClassOptions(
                [],
                className: _$namespace,
                instanceType: InstanceType.manual,
                bridgeKey: 'com.volcengine.rtc.hybrid_runtime',
              )
            : NativeClassOptions.fromMap({
                ...options.toMap(),
                'bridgeKey': 'com.volcengine.rtc.hybrid_runtime',
              }));

  /// @brief 音频丢包率。统计周期内的音频下行丢包率，取值范围为 [0, 1] 。
  FutureOr<float?> get audioLossRate async {
    return await sendInstanceGet<float?>("audioLossRate");
  }

  set audioLossRate(FutureOr<float?> value) {
    sendInstanceSet("audioLossRate", value);
  }

  /// @brief 接收码率。统计周期内的音频接收码率，单位为 kbps 。
  FutureOr<float?> get receivedKBitrate async {
    return await sendInstanceGet<float?>("receivedKBitrate");
  }

  set receivedKBitrate(FutureOr<float?> value) {
    sendInstanceSet("receivedKBitrate", value);
  }

  /// @brief 音频卡顿次数。统计周期内的卡顿次数。
  FutureOr<NSInteger?> get stallCount async {
    return await sendInstanceGet<NSInteger?>("stallCount");
  }

  set stallCount(FutureOr<NSInteger?> value) {
    sendInstanceSet("stallCount", value);
  }

  /// @brief 音频卡顿时长。统计周期内的卡顿时长，单位为 ms 。
  FutureOr<NSInteger?> get stallDuration async {
    return await sendInstanceGet<NSInteger?>("stallDuration");
  }

  set stallDuration(FutureOr<NSInteger?> value) {
    sendInstanceSet("stallDuration", value);
  }

  /// @brief 播放采样率。统计周期内的音频播放采样率信息，单位为 Hz 。
  FutureOr<NSInteger?> get playoutSampleRate async {
    return await sendInstanceGet<NSInteger?>("playoutSampleRate");
  }

  set playoutSampleRate(FutureOr<NSInteger?> value) {
    sendInstanceSet("playoutSampleRate", value);
  }

  /// @brief 用户体验级别的端到端延时。从发送端采集完成编码开始到接收端解码完成渲染开始的延时，单位为 ms 。
  FutureOr<NSInteger?> get e2eDelay async {
    return await sendInstanceGet<NSInteger?>("e2eDelay");
  }

  set e2eDelay(FutureOr<NSInteger?> value) {
    sendInstanceSet("e2eDelay", value);
  }

  /// @brief 统计间隔。此次统计周期的间隔，单位为 ms 。
  FutureOr<NSInteger?> get statsInterval async {
    return await sendInstanceGet<NSInteger?>("statsInterval");
  }

  set statsInterval(FutureOr<NSInteger?> value) {
    sendInstanceSet("statsInterval", value);
  }

  /// @brief 客户端到服务端数据传输的往返时延，单位为 ms。
  FutureOr<NSInteger?> get rtt async {
    return await sendInstanceGet<NSInteger?>("rtt");
  }

  set rtt(FutureOr<NSInteger?> value) {
    sendInstanceSet("rtt", value);
  }

  /// @brief 发送端——服务端——接收端全链路数据传输往返时延。单位为 ms 。
  FutureOr<NSInteger?> get totalRtt async {
    return await sendInstanceGet<NSInteger?>("totalRtt");
  }

  set totalRtt(FutureOr<NSInteger?> value) {
    sendInstanceSet("totalRtt", value);
  }

  /// @brief 远端用户发送的音频流质量。值含义参考 ByteRTCNetworkQuality{@link #ByteRTCNetworkQuality} 。
  FutureOr<NSInteger?> get quality async {
    return await sendInstanceGet<NSInteger?>("quality");
  }

  set quality(FutureOr<NSInteger?> value) {
    sendInstanceSet("quality", value);
  }

  /// @brief 因引入 jitter buffer 机制导致的延时。单位为 ms 。
  FutureOr<NSInteger?> get jitterBufferDelay async {
    return await sendInstanceGet<NSInteger?>("jitterBufferDelay");
  }

  set jitterBufferDelay(FutureOr<NSInteger?> value) {
    sendInstanceSet("jitterBufferDelay", value);
  }

  /// @brief 音频声道数。
  FutureOr<NSInteger?> get numChannels async {
    return await sendInstanceGet<NSInteger?>("numChannels");
  }

  set numChannels(FutureOr<NSInteger?> value) {
    sendInstanceSet("numChannels", value);
  }

  /// @brief 音频接收采样率。统计周期内接收到的远端音频采样率信息，单位为 Hz 。
  FutureOr<NSInteger?> get receivedSampleRate async {
    return await sendInstanceGet<NSInteger?>("receivedSampleRate");
  }

  set receivedSampleRate(FutureOr<NSInteger?> value) {
    sendInstanceSet("receivedSampleRate", value);
  }

  /// @brief 远端用户在加入房间后发生音频卡顿的累计时长占音频总有效时长的百分比。音频有效时长是指远端用户进房发布音频流后，除停止发送音频流和禁用音频模块之外的音频时长。
  FutureOr<NSInteger?> get frozenRate async {
    return await sendInstanceGet<NSInteger?>("frozenRate");
  }

  set frozenRate(FutureOr<NSInteger?> value) {
    sendInstanceSet("frozenRate", value);
  }

  /// @brief 音频 PLC 样点总个数。
  FutureOr<NSInteger?> get concealedSamples async {
    return await sendInstanceGet<NSInteger?>("concealedSamples");
  }

  set concealedSamples(FutureOr<NSInteger?> value) {
    sendInstanceSet("concealedSamples", value);
  }

  /// @brief 音频丢包补偿(PLC) 累计次数。
  FutureOr<NSInteger?> get concealmentEvent async {
    return await sendInstanceGet<NSInteger?>("concealmentEvent");
  }

  set concealmentEvent(FutureOr<NSInteger?> value) {
    sendInstanceSet("concealmentEvent", value);
  }

  /// @brief 音频解码采样率。统计周期内的音频解码采样率信息，单位为 Hz 。
  FutureOr<NSInteger?> get decSampleRate async {
    return await sendInstanceGet<NSInteger?>("decSampleRate");
  }

  set decSampleRate(FutureOr<NSInteger?> value) {
    sendInstanceSet("decSampleRate", value);
  }

  /// @brief 此次订阅中，对远端音频流进行解码的累计耗时。单位为 s。
  FutureOr<NSInteger?> get decDuration async {
    return await sendInstanceGet<NSInteger?>("decDuration");
  }

  set decDuration(FutureOr<NSInteger?> value) {
    sendInstanceSet("decDuration", value);
  }

  /// @brief 视频下行网络抖动，单位为 ms。
  FutureOr<NSInteger?> get jitter async {
    return await sendInstanceGet<NSInteger?>("jitter");
  }

  set jitter(FutureOr<NSInteger?> value) {
    sendInstanceSet("jitter", value);
  }

  /// @brief 音频解码帧率。
  FutureOr<double?> get decodeFrameRate async {
    return await sendInstanceGet<double?>("decodeFrameRate");
  }

  set decodeFrameRate(FutureOr<double?> value) {
    sendInstanceSet("decodeFrameRate", value);
  }
}

class ByteRTCRecordingProgress extends NativeClass {
  static const _$namespace = r'ByteRTCRecordingProgress';
  static get codegen_$namespace => _$namespace;

  ByteRTCRecordingProgress([NativeClassOptions? options])
      : super(options == null
            ? const NativeClassOptions(
                [],
                className: _$namespace,
                instanceType: InstanceType.manual,
                bridgeKey: 'com.volcengine.rtc.hybrid_runtime',
              )
            : NativeClassOptions.fromMap({
                ...options.toMap(),
                'bridgeKey': 'com.volcengine.rtc.hybrid_runtime',
              }));

  /// @brief 当前文件的累计录制时长，单位：毫秒
  FutureOr<longlong?> get duration async {
    return await sendInstanceGet<longlong?>("duration");
  }

  set duration(FutureOr<longlong?> value) {
    sendInstanceSet("duration", value);
  }

  /// @brief 当前录制文件的大小，单位：byte
  FutureOr<longlong?> get fileSize async {
    return await sendInstanceGet<longlong?>("fileSize");
  }

  set fileSize(FutureOr<longlong?> value) {
    sendInstanceSet("fileSize", value);
  }
}

class ByteRTCVideoCaptureConfig extends NativeClass {
  static const _$namespace = r'ByteRTCVideoCaptureConfig';
  static get codegen_$namespace => _$namespace;

  ByteRTCVideoCaptureConfig([NativeClassOptions? options])
      : super(options == null
            ? const NativeClassOptions(
                [],
                className: _$namespace,
                instanceType: InstanceType.manual,
                bridgeKey: 'com.volcengine.rtc.hybrid_runtime',
              )
            : NativeClassOptions.fromMap({
                ...options.toMap(),
                'bridgeKey': 'com.volcengine.rtc.hybrid_runtime',
              }));

  /// @brief 视频采集模式，参看 ByteRTCVideoCapturePreference{@link #ByteRTCVideoCapturePreference}
  FutureOr<ByteRTCVideoCapturePreference?> get preference async {
    try {
      final result =
          await sendInstanceGet<ByteRTCVideoCapturePreference?>("preference");
      if (result == null) {
        return null;
      }
      return ByteRTCVideoCapturePreference.values
          .firstWhere((element) => element == result);
    } catch (e) {
      return null;
    }
  }

  set preference(FutureOr<ByteRTCVideoCapturePreference?> value) {
    sendInstanceSet("preference", value);
  }

  /// @brief 视频采集帧率，单位：fps。
  FutureOr<NSInteger?> get frameRate async {
    return await sendInstanceGet<NSInteger?>("frameRate");
  }

  set frameRate(FutureOr<NSInteger?> value) {
    sendInstanceSet("frameRate", value);
  }
}

enum ByteRTSReturnStatus {
  /// @hidden currently not available
  ///
  ByteRTSReturnStatusSuccess(0),

  /// @hidden currently not available
  ///
  ByteRTSReturnStatusFailure(-1),

  /// @hidden currently not available
  ///
  ByteRTSReturnStatusParameterErr(-2),

  /// @hidden currently not available
  ///
  ByteRTSReturnStatusWrongState(-3),

  /// @hidden currently not available
  ///
  ByteRTSReturnStatusHasInRoom(-4),

  /// @hidden currently not available
  ///
  ByteRTSReturnStatusHasInLogin(-5),

  /// @hidden currently not available
  ///
  ByteRTSReturnStatusRoomIdInUse(-8);

  final dynamic $value;
  const ByteRTSReturnStatus([this.$value]);
}

enum ByteRTCFallbackOrRecoverReason {
  /// @brief 其他原因，非带宽和性能原因引起的回退或恢复。默认值
  ///
  ByteRTCFallbackOrRecoverReasonUnknown(-1),

  /// @brief 由带宽不足导致的订阅端音视频流回退。
  ///
  ByteRTCFallbackOrRecoverReasonSubscribeFallbackByBandwidth(0),

  /// @brief 由性能不足导致的订阅端音视频流回退。
  ///
  ByteRTCFallbackOrRecoverReasonSubscribeFallbackByPerformance(1),

  /// @brief 由带宽恢复导致的订阅端音视频流恢复。
  ///
  ByteRTCFallbackOrRecoverReasonSubscribeRecoverByBandwidth(2),

  /// @brief 由性能恢复导致的订阅端音视频流恢复。
  ///
  ByteRTCFallbackOrRecoverReasonSubscribeRecoverByPerformance(3),

  /// @brief 由带宽不足导致的发布端音视频流回退。
  ///
  ByteRTCFallbackOrRecoverReasonPublishFallbackByBandwidth(4),

  /// @brief 由性能不足导致的发布端音视频流回退。
  ///
  ByteRTCFallbackOrRecoverReasonPublishFallbackByPerformance(5),

  /// @brief 由带宽恢复导致的发布端音视频流恢复。
  ///
  ByteRTCFallbackOrRecoverReasonPublishRecoverByBandwidth(6),

  /// @brief 由性能恢复导致的发布端音视频流恢复。
  ///
  ByteRTCFallbackOrRecoverReasonPublishRecoverByPerformance(7);

  final dynamic $value;
  const ByteRTCFallbackOrRecoverReason([this.$value]);
}

enum ByteRTCRemoteVideoState {
  /// @brief 远端视频流默认初始状态, 在以下时机回调该状态： <br>
  ///       - 本地用户停止接收远端视频流，对应错误码 ByteRTCRemoteVideoStateChangeReason{@link #ByteRTCRemoteVideoStateChangeReason} 中的 ByteRTCRemoteVideoStateChangeReasonLocalMuted
  ///       - 远端用户停止发送视频流，对应错误码 ByteRTCRemoteVideoStateChangeReason{@link #ByteRTCRemoteVideoStateChangeReason} 中的 ByteRTCRemoteVideoStateChangeReasonRemoteMuted
  ///       - 远端用户离开房间，对应错误码 ByteRTCRemoteVideoStateChangeReason{@link #ByteRTCRemoteVideoStateChangeReason} 中的 ByteRTCRemoteVideoStateChangeReasonRemoteOffline
  ///
  ByteRTCRemoteVideoStateStopped(0),

  /// @brief 本地用户已接收远端视频首包 <br>
  ///        收到远端视频首包时回调该状态，对应错误码 ByteRTCRemoteVideoStateChangeReason{@link #ByteRTCRemoteVideoStateChangeReason} 中的 ByteRTCRemoteVideoStateChangeReasonLocalUnmuted
  ///
  ByteRTCRemoteVideoStateStarting(1),

  /// @brief 远端视频流正在解码，正常播放, 在以下时机回调该状态： <br>
  ///       - 成功解码远端视频首帧，对应错误码 ByteRTCRemoteVideoStateChangeReason{@link #ByteRTCRemoteVideoStateChangeReason} 中的 ByteRTCRemoteVideoStateChangeReasonLocalUnmuted
  ///       - 网络由阻塞恢复正常，对应错误码 ByteRTCRemoteVideoStateChangeReason{@link #ByteRTCRemoteVideoStateChangeReason} 中的 ByteRTCRemoteVideoStateChangeReasonNetworkRecovery
  ///       - 本地用户恢复接收远端视频流，对应错误码 ByteRTCRemoteVideoStateChangeReason{@link #ByteRTCRemoteVideoStateChangeReason} 中的 ByteRTCRemoteVideoStateChangeReasonLocalUnmuted
  ///       - 远端用户恢复发送视频流，对应错误码 ByteRTCRemoteVideoStateChangeReason{@link #ByteRTCRemoteVideoStateChangeReason} 中的 ByteRTCRemoteVideoStateChangeReasonRemoteUnmuted
  ///
  ByteRTCRemoteVideoStateDecoding(2),

  /// @brief 远端视频流卡顿 <br>
  ///        网络阻塞、丢包率等原因造成视频卡顿流时会回报该状态，对应错误码 ByteRTCRemoteVideoStateChangeReason{@link #ByteRTCRemoteVideoStateChangeReason} 中的 ByteRTCRemoteVideoStateChangeReasonNetworkCongestion
  ///
  ByteRTCRemoteVideoStateFrozen(3),

  /// @brief 远端视频流播放失败 <br>
  ///        如果内部处理远端视频流失败，则会回调该方法， 对应错误码 ByteRTCRemoteVideoStateChangeReason{@link #ByteRTCRemoteVideoStateChangeReason} 中的 ByteRTCRemoteVideoStateChangeReasonInternal
  ///
  ByteRTCRemoteVideoStateFailed(4);

  final dynamic $value;
  const ByteRTCRemoteVideoState([this.$value]);
}

class ByteRTCEngineConfig extends NativeClass {
  static const _$namespace = r'ByteRTCEngineConfig';
  static get codegen_$namespace => _$namespace;

  ByteRTCEngineConfig([NativeClassOptions? options])
      : super(options == null
            ? const NativeClassOptions(
                [],
                className: _$namespace,
                instanceType: InstanceType.manual,
                bridgeKey: 'com.volcengine.rtc.hybrid_runtime',
              )
            : NativeClassOptions.fromMap({
                ...options.toMap(),
                'bridgeKey': 'com.volcengine.rtc.hybrid_runtime',
              }));

  /// @brief 每个应用的唯一标识符。只有使用相同的 app_id 生成的实例，才能够进行音视频通信。
  FutureOr<NSString?> get appID async {
    return await sendInstanceGet<NSString?>("appID");
  }

  set appID(FutureOr<NSString?> value) {
    sendInstanceSet("appID", value);
  }

  /// @brief 是否是游戏场景。
  FutureOr<BOOL?> get isGameScene async {
    return await sendInstanceGet<BOOL?>("isGameScene");
  }

  set isGameScene(FutureOr<BOOL?> value) {
    sendInstanceSet("isGameScene", value);
  }

  /// @brief 私有参数。如需使用请联系技术支持人员。
  FutureOr<NSDictionary?> get parameters async {
    return await sendInstanceGet<NSDictionary?>("parameters");
  }

  set parameters(FutureOr<NSDictionary?> value) {
    sendInstanceSet("parameters", value);
  }
}

enum ByteRTCVideoBufferType {
  /// @brief 原始内存数据
  ///
  ByteRTCVideoBufferTypeRawMemory(0),

  /// @brief CVPixelBufferRef 类型
  ///
  ByteRTCVideoBufferTypeCVPixelBuffer(1),

  /// @brief OpenGL 纹理数据类型
  ///
  ByteRTCVideoBufferTypeGLTexture(2),

  /// @brief cuda 数据类型
  ///
  ByteRTCVideoBufferTypeCuda(3),

  /// @brief direct3d11 纹理
  ///
  ByteRTCVideoBufferTypeD3D11(4),

  /// @brief vaapi 数据格式
  ///
  ByteRTCVideoBufferTypeVAAPI(5),

  /// @hidden(Windows)
  /// @brief nvidia jetson dma 数据格式
  ///
  ByteRTCVideoBufferTypeNvidiaJetsonDma(6);

  final dynamic $value;
  const ByteRTCVideoBufferType([this.$value]);
}

class ByteRTCAudioEnhancementConfig extends NativeClass {
  static const _$namespace = r'ByteRTCAudioEnhancementConfig';
  static get codegen_$namespace => _$namespace;

  ByteRTCAudioEnhancementConfig([NativeClassOptions? options])
      : super(options == null
            ? const NativeClassOptions(
                [],
                className: _$namespace,
                instanceType: InstanceType.manual,
                bridgeKey: 'com.volcengine.rtc.hybrid_runtime',
              )
            : NativeClassOptions.fromMap({
                ...options.toMap(),
                'bridgeKey': 'com.volcengine.rtc.hybrid_runtime',
              }));

  /// @brief 对信令消息，是否启用蜂窝网络辅助增强。默认不启用。
  FutureOr<BOOL?> get enhanceSignaling async {
    return await sendInstanceGet<BOOL?>("enhanceSignaling");
  }

  set enhanceSignaling(FutureOr<BOOL?> value) {
    sendInstanceSet("enhanceSignaling", value);
  }

  /// @brief 对音频，是否启用蜂窝网络辅助增强。默认不启用。
  FutureOr<BOOL?> get enhanceAudio async {
    return await sendInstanceGet<BOOL?>("enhanceAudio");
  }

  set enhanceAudio(FutureOr<BOOL?> value) {
    sendInstanceSet("enhanceAudio", value);
  }
}

enum ByteRTCAudioMixingDualMonoMode {
  /// @brief 和音频文件一致
  ///
  ByteRTCAudioMixingDualMonoModeAuto(0),

  /// @brief 只能听到音频文件中左声道的音频
  ///
  ByteRTCAudioMixingDualMonoModeL(1),

  /// @brief 只能听到音频文件中右声道的音频
  ///
  ByteRTCAudioMixingDualMonoModeR(2),

  /// @brief 能同时听到音频文件中左右声道的音频
  ///
  ByteRTCAudioMixingDualMonoModeMix(3);

  final dynamic $value;
  const ByteRTCAudioMixingDualMonoMode([this.$value]);
}

enum ByteRTCMediaPlayerCustomSourceStreamType {
  /// @brief 当播放来自本地的 PCM 数据时，使用此选项。
  ///
  ByteRTCMediaPlayerCustomSourceStreamTypeRaw(0),

  /// @brief 当播放来自内存的音频数据时，使用此选项。
  ///
  ByteRTCMediaPlayerCustomSourceStreamTypeEncoded(1);

  final dynamic $value;
  const ByteRTCMediaPlayerCustomSourceStreamType([this.$value]);
}

class ByteRTCMixedStreamVideoConfig extends NativeClass {
  static const _$namespace = r'ByteRTCMixedStreamVideoConfig';
  static get codegen_$namespace => _$namespace;

  ByteRTCMixedStreamVideoConfig([NativeClassOptions? options])
      : super(options == null
            ? const NativeClassOptions(
                [],
                className: _$namespace,
                instanceType: InstanceType.manual,
                bridgeKey: 'com.volcengine.rtc.hybrid_runtime',
              )
            : NativeClassOptions.fromMap({
                ...options.toMap(),
                'bridgeKey': 'com.volcengine.rtc.hybrid_runtime',
              }));

  /// @brief 视频编码格式，参看 ByteRTCMixedStreamVideoCodecType{@link #ByteRTCMixedStreamVideoCodecType}。默认值为 `0`。建议设置。 <br>
  ///        本参数不支持过程中更新。
  FutureOr<ByteRTCMixedStreamVideoCodecType?> get videoCodec async {
    try {
      final result = await sendInstanceGet<ByteRTCMixedStreamVideoCodecType?>(
          "videoCodec");
      if (result == null) {
        return null;
      }
      return ByteRTCMixedStreamVideoCodecType.values
          .firstWhere((element) => element == result);
    } catch (e) {
      return null;
    }
  }

  set videoCodec(FutureOr<ByteRTCMixedStreamVideoCodecType?> value) {
    sendInstanceSet("videoCodec", value);
  }

  /// @brief 合流视频宽度。单位为 px，范围为 [2, 1920]，必须是偶数。默认值为 640 px。建议设置。 <br>
  ///        设置值为非偶数时，自动向上取偶数。
  FutureOr<NSInteger?> get width async {
    return await sendInstanceGet<NSInteger?>("width");
  }

  set width(FutureOr<NSInteger?> value) {
    sendInstanceSet("width", value);
  }

  /// @brief 合流视频高度。单位为 px，范围为 [2, 1920]，必须是偶数。默认值为 360 px。建议设置。 <br>
  ///        设置值为非偶数时，自动向上取偶数。
  FutureOr<NSInteger?> get height async {
    return await sendInstanceGet<NSInteger?>("height");
  }

  set height(FutureOr<NSInteger?> value) {
    sendInstanceSet("height", value);
  }

  /// @brief 合流视频帧率。单位为 FPS，取值范围为 [1,60]，默认值为 15 FPS。建议设置。
  FutureOr<NSInteger?> get fps async {
    return await sendInstanceGet<NSInteger?>("fps");
  }

  set fps(FutureOr<NSInteger?> value) {
    sendInstanceSet("fps", value);
  }

  /// @brief 视频 I 帧时间间隔。单位为秒，取值范围为 [1, 5]，默认值为 2 秒。建议设置。 <br>
  ///        本参数不支持过程中更新。
  FutureOr<NSInteger?> get gop async {
    return await sendInstanceGet<NSInteger?>("gop");
  }

  set gop(FutureOr<NSInteger?> value) {
    sendInstanceSet("gop", value);
  }

  /// @brief 合流视频码率。单位为 Kbps，取值范围为 [1,10000]，默认值为自适应模式。建议设置。
  FutureOr<NSInteger?> get bitrate async {
    return await sendInstanceGet<NSInteger?>("bitrate");
  }

  set bitrate(FutureOr<NSInteger?> value) {
    sendInstanceSet("bitrate", value);
  }

  /// @brief 是否在合流中开启 B 帧，仅服务端合流支持.
  FutureOr<BOOL?> get enableBFrame async {
    return await sendInstanceGet<BOOL?>("enableBFrame");
  }

  set enableBFrame(FutureOr<BOOL?> value) {
    sendInstanceSet("enableBFrame", value);
  }
}

enum ByteRTCVoiceReverbType {
  /// @brief 原声，不含特效
  ///
  ByteRTCVoiceReverbOriginal(0),

  /// @brief 回声
  ///
  ByteRTCVoiceReverbEcho(1),

  /// @brief 演唱会
  ///
  ByteRTCVoiceReverbConcert(2),

  /// @brief 空灵
  ///
  ByteRTCVoiceReverbEthereal(3),

  /// @brief KTV
  ///
  ByteRTCVoiceReverbKTV(4),

  /// @brief 录音棚
  ///
  ByteRTCVoiceReverbStudio(5),

  /// @brief 虚拟立体声
  ///
  ByteRTCVoiceReverbVirtualStereo(6),

  /// @brief 空旷
  ///
  ByteRTCVoiceReverbSpacious(7),

  /// @brief 3D 人声
  ///
  ByteRTCVoiceReverb3D(8),

  /// @hidden internal use
  /// @brief 流行
  ///
  ByteRTCVoiceReverbPop(9),

  /// @hidden internal use
  /// @brief 蹦迪
  ///
  ByteRTCVoiceReverbDisco(10),

  /// @hidden internal use
  /// @brief 老唱片
  ///
  ByteRTCVoiceReverbOldRecord(11),

  /// @hidden internal use
  /// @brief 和声
  ///
  ByteRTCVoiceReverbHarmony(12),

  /// @hidden internal use
  /// @brief 摇滚
  ///
  ByteRTCVoiceReverbRock(13),

  /// @hidden internal use
  /// @brief 蓝调
  ///
  ByteRTCVoiceReverbBlues(14),

  /// @hidden internal use
  /// @brief 爵士
  ///
  ByteRTCVoiceReverbJazz(15),

  /// @hidden internal use
  /// @brief 电子
  ///
  ByteRTCVoiceReverbElectronic(16),

  /// @hidden internal use
  /// @brief 黑胶
  ///
  ByteRTCVoiceReverbVinyl(17),

  /// @hidden internal use
  /// @brief 密室
  ///
  ByteRTCVoiceReverbChamber(18),

  /// @hidden internal use
  /// @brief 增强原声
  ///
  ByteRTCVoiceReverbEnhanceOriginal(19),

  /// @hidden internal use
  /// @brief 浴室
  ///
  ByteRTCVoiceReverbBathroom(20),

  /// @hidden internal use
  /// @brief 自然
  ///
  ByteRTCVoiceReverbNatural(21),

  /// @hidden internal use
  /// @brief 楼道
  ///
  ByteRTCVoiceReverbHallway(22);

  final dynamic $value;
  const ByteRTCVoiceReverbType([this.$value]);
}

enum ByteRTCVideoSourceType {
  /// @brief 自定义采集视频源
  ///
  ByteRTCVideoSourceTypeExternal(0),

  /// @brief 内部采集视频源
  ///
  ByteRTCVideoSourceTypeInternal(1),

  /// @brief 自定义编码视频源。 <br>
  ///        你仅需推送分辨率最大的一路编码后视频流，SDK 将自动转码生成多路小流
  ///
  ByteRTCVideoSourceTypeEncodedAutoSimulcast(2),

  /// @brief 自定义编码视频源。 <br>
  ///        SDK 不会自动生成多路流，你需要自行生成并推送多路流
  ///
  ByteRTCVideoSourceTypeEncodedManualSimulcast(3);

  final dynamic $value;
  const ByteRTCVideoSourceType([this.$value]);
}

enum ByteRTCMixedStreamPushTargetType {
  /// @brief 推到 CDN
  ///
  ByteRTCMixedStreamPushTargetTypeToCDN(0),

  /// @brief WTN 流
  ///
  ByteRTCMixedStreamPushTargetTypeToWTN(1);

  final dynamic $value;
  const ByteRTCMixedStreamPushTargetType([this.$value]);
}

enum ByteRTCStreamIndex {
  /// @brief 主流。包括： <br>
  ///        - 由摄像头/麦克风通过内部采集机制，采集到的视频/音频;
  ///        - 通过自定义采集，采集到的视频/音频。
  ///
  ByteRTCStreamIndexMain(0),

  /// @brief 屏幕流。屏幕共享时共享的视频流，或来自声卡的本地播放音频流。
  ///
  ByteRTCStreamIndexScreen(1),

  /// @hidden for internal use only
  ///
  ByteRTCStreamIndex3rd(2),

  /// @hidden for internal use only
  ///
  ByteRTCStreamIndex4th(3),

  /// @hidden for internal use only
  ///
  ByteRTCStreamIndex5th(4),

  /// @hidden for internal use only
  ///
  ByteRTCStreamIndex6th(5),

  /// @hidden for internal use only
  ///
  ByteRTCStreamIndex7th(6),

  /// @hidden for internal use only
  ///
  ByteRTCStreamIndexMax(7);

  final dynamic $value;
  const ByteRTCStreamIndex([this.$value]);
}

class ByteRTCRoomEventInfo extends NativeClass {
  static const _$namespace = r'ByteRTCRoomEventInfo';
  static get codegen_$namespace => _$namespace;

  ByteRTCRoomEventInfo([NativeClassOptions? options])
      : super(options == null
            ? const NativeClassOptions(
                [],
                className: _$namespace,
                instanceType: InstanceType.manual,
                bridgeKey: 'com.volcengine.rtc.hybrid_runtime',
              )
            : NativeClassOptions.fromMap({
                ...options.toMap(),
                'bridgeKey': 'com.volcengine.rtc.hybrid_runtime',
              }));

  /// @brief 房间/用户被封禁，通过房间事件通知封禁时间。
  ///
  FutureOr<NSInteger?> get forbiddenTime async {
    return await sendInstanceGet<NSInteger?>("forbiddenTime");
  }

  set forbiddenTime(FutureOr<NSInteger?> value) {
    sendInstanceSet("forbiddenTime", value);
  }
}

class ByteRTCAudioFrame extends NativeClass {
  static const _$namespace = r'ByteRTCAudioFrame';
  static get codegen_$namespace => _$namespace;

  ByteRTCAudioFrame([NativeClassOptions? options])
      : super(options == null
            ? const NativeClassOptions(
                [],
                className: _$namespace,
                instanceType: InstanceType.manual,
                bridgeKey: 'com.volcengine.rtc.hybrid_runtime',
              )
            : NativeClassOptions.fromMap({
                ...options.toMap(),
                'bridgeKey': 'com.volcengine.rtc.hybrid_runtime',
              }));

  /// @brief PCM 数据
  FutureOr<NSData?> get buffer async {
    return await sendInstanceGet<NSData?>("buffer");
  }

  set buffer(FutureOr<NSData?> value) {
    sendInstanceSet("buffer", value);
  }

  /// @brief 采样点个数
  FutureOr<int?> get samples async {
    return await sendInstanceGet<int?>("samples");
  }

  set samples(FutureOr<int?> value) {
    sendInstanceSet("samples", value);
  }

  /// @brief 音频声道，参看 ByteRTCAudioChannel{@link #ByteRTCAudioChannel}。 <br>
  ///        双声道的情况下，左右声道的音频帧数据以 LRLRLR 形式排布。
  FutureOr<ByteRTCAudioChannel?> get channel async {
    try {
      final result = await sendInstanceGet<ByteRTCAudioChannel?>("channel");
      if (result == null) {
        return null;
      }
      return ByteRTCAudioChannel.values
          .firstWhere((element) => element == result);
    } catch (e) {
      return null;
    }
  }

  set channel(FutureOr<ByteRTCAudioChannel?> value) {
    sendInstanceSet("channel", value);
  }

  /// @brief 采样率，参看 ByteRTCAudioSampleRate{@link #ByteRTCAudioSampleRate}。
  FutureOr<ByteRTCAudioSampleRate?> get sampleRate async {
    try {
      final result =
          await sendInstanceGet<ByteRTCAudioSampleRate?>("sampleRate");
      if (result == null) {
        return null;
      }
      return ByteRTCAudioSampleRate.values
          .firstWhere((element) => element == result);
    } catch (e) {
      return null;
    }
  }

  set sampleRate(FutureOr<ByteRTCAudioSampleRate?> value) {
    sendInstanceSet("sampleRate", value);
  }
}

enum ByteRTCVideoContentType {
  /// @brief 普通视频
  ///
  ByteRTCVideoContentTypeNormalFrame(0),

  /// @brief 黑帧视频
  ///
  ByteRTCVideoContentTypeBlackFrame(1);

  final dynamic $value;
  const ByteRTCVideoContentType([this.$value]);
}

enum ByteRTCNetworkDetectionLinkType {
  /// @brief 上行网络探测。
  ///
  ByteRTCNetworkDetectionLinkTypeUp(0),

  /// @brief 下行网络探测。
  ///
  ByteRTCNetworkDetectionLinkTypeDown(1);

  final dynamic $value;
  const ByteRTCNetworkDetectionLinkType([this.$value]);
}

class ByteRTCProblemFeedbackInfo extends NativeClass {
  static const _$namespace = r'ByteRTCProblemFeedbackInfo';
  static get codegen_$namespace => _$namespace;

  ByteRTCProblemFeedbackInfo([NativeClassOptions? options])
      : super(options == null
            ? const NativeClassOptions(
                [],
                className: _$namespace,
                instanceType: InstanceType.manual,
                bridgeKey: 'com.volcengine.rtc.hybrid_runtime',
              )
            : NativeClassOptions.fromMap({
                ...options.toMap(),
                'bridgeKey': 'com.volcengine.rtc.hybrid_runtime',
              }));

  /// @brief 通话质量反馈的文字描述
  ///
  FutureOr<NSString?> get problemDesc async {
    return await sendInstanceGet<NSString?>("problemDesc");
  }

  set problemDesc(FutureOr<NSString?> value) {
    sendInstanceSet("problemDesc", value);
  }

  /// @brief 通话质量反馈的房间信息。参看 ByteRTCProblemFeedbackRoomInfo{@link #ByteRTCProblemFeedbackRoomInfo}。
  ///
  FutureOr<NSArray<ByteRTCProblemFeedbackRoomInfo>?> get roomInfo async {
    try {
      final result =
          await sendInstanceGet<NSArray<ByteRTCProblemFeedbackRoomInfo>?>(
              "roomInfo");
      if (result == null) {
        return null;
      }
      final list = result.map((e) => packObject(
          e,
          () => ByteRTCProblemFeedbackRoomInfo(
              const NativeClassOptions([], disableInit: true))));
      return list.toList();
    } catch (e) {
      return null;
    }
  }

  set roomInfo(FutureOr<NSArray<ByteRTCProblemFeedbackRoomInfo>?> value) {
    sendInstanceSet("roomInfo", value);
  }
}

class ByteRTCRemoteStreamSwitchEvent extends NativeClass {
  static const _$namespace = r'ByteRTCRemoteStreamSwitchEvent';
  static get codegen_$namespace => _$namespace;

  ByteRTCRemoteStreamSwitchEvent([NativeClassOptions? options])
      : super(options == null
            ? const NativeClassOptions(
                [],
                className: _$namespace,
                instanceType: InstanceType.manual,
                bridgeKey: 'com.volcengine.rtc.hybrid_runtime',
              )
            : NativeClassOptions.fromMap({
                ...options.toMap(),
                'bridgeKey': 'com.volcengine.rtc.hybrid_runtime',
              }));

  /// @brief 订阅的音视频流的发布者的用户 ID。
  FutureOr<NSString?> get uid async {
    return await sendInstanceGet<NSString?>("uid");
  }

  set uid(FutureOr<NSString?> value) {
    sendInstanceSet("uid", value);
  }

  /// @brief 流是否是屏幕流
  FutureOr<BOOL?> get isScreen async {
    return await sendInstanceGet<BOOL?>("isScreen");
  }

  set isScreen(FutureOr<BOOL?> value) {
    sendInstanceSet("isScreen", value);
  }

  /// @brief 流切换前订阅视频流的分辨率对应的索引
  FutureOr<NSInteger?> get beforeVideoIndex async {
    return await sendInstanceGet<NSInteger?>("beforeVideoIndex");
  }

  set beforeVideoIndex(FutureOr<NSInteger?> value) {
    sendInstanceSet("beforeVideoIndex", value);
  }

  /// @brief 流切换后订阅视频流的分辨率对应的索引
  FutureOr<NSInteger?> get afterVideoIndex async {
    return await sendInstanceGet<NSInteger?>("afterVideoIndex");
  }

  set afterVideoIndex(FutureOr<NSInteger?> value) {
    sendInstanceSet("afterVideoIndex", value);
  }

  /// @brief 流切换前是否有视频流
  FutureOr<BOOL?> get beforeVideoEnabled async {
    return await sendInstanceGet<BOOL?>("beforeVideoEnabled");
  }

  set beforeVideoEnabled(FutureOr<BOOL?> value) {
    sendInstanceSet("beforeVideoEnabled", value);
  }

  /// @brief 流切换后是否有视频流
  FutureOr<BOOL?> get afterVideoEnabled async {
    return await sendInstanceGet<BOOL?>("afterVideoEnabled");
  }

  set afterVideoEnabled(FutureOr<BOOL?> value) {
    sendInstanceSet("afterVideoEnabled", value);
  }

  /// @brief 流切换原因，详见 ByteRTCFallbackOrRecoverReason{@link #ByteRTCFallbackOrRecoverReason}。
  FutureOr<ByteRTCFallbackOrRecoverReason?> get reason async {
    try {
      final result =
          await sendInstanceGet<ByteRTCFallbackOrRecoverReason?>("reason");
      if (result == null) {
        return null;
      }
      return ByteRTCFallbackOrRecoverReason.values
          .firstWhere((element) => element == result);
    } catch (e) {
      return null;
    }
  }

  set reason(FutureOr<ByteRTCFallbackOrRecoverReason?> value) {
    sendInstanceSet("reason", value);
  }
}

enum ByteRTCMediaDeviceWarning {
  /// @brief 无警告
  ///
  ByteRTCMediaDeviceWarningOK(0),

  /// @brief 非法设备操作。在使用外部设备时，调用了 SDK 内部设备 API。
  ///
  ByteRTCMediaDeviceWarningOperationDenied(1),

  /// @brief 采集到的数据为静音帧。
  ///
  ByteRTCMediaDeviceWarningCaptureSilence(2),

  /// @hidden for internal use only
  /// @brief 音量过大，超过设备采集范围。建议降低麦克风音量或者降低声源音量。
  ///
  ByteRTCMediaDeviceWarningDetectClipping(10),

  /// @brief 通话中出现回声现象。 <br>
  ///        当 ByteRTCRoomProfile{@link #ByteRTCRoomProfile} 为 `ByteRTCRoomProfileMeeting` 和 `ByteRTCRoomProfileMeetingRoom` ，且 AEC 关闭时，SDK 自动启动回声检测，如果检测到回声问题，将通过 `rtcEngine:onAudioDeviceWarning:deviceType:deviceWarning:` 返回本枚举值。
  ///
  ByteRTCMediaDeviceWarningDetectLeakEcho(11),

  /// @hidden for internal use only
  /// @brief 低信噪比
  ///
  ByteRTCMediaDeviceWarningDetectLowSNR(12),

  /// @hidden for internal use only
  /// @brief 采集插零现象
  ///
  ByteRTCMediaDeviceWarningDetectInsertSilence(13),

  /// @hidden for internal use only
  /// @brief 设备采集静音
  ///
  ByteRTCMediaDeviceWarningCaptureDetectSilence(14),

  /// @hidden for internal use only
  /// @brief 设备采集静音消失
  ///
  ByteRTCMediaDeviceWarningCaptureDetectSilenceDisappear(15),

  /// @brief 啸叫。触发该回调的情况如下：1）不支持啸叫抑制的房间模式下，检测到啸叫；2）支持啸叫抑制的房间模式下，检测到未被抑制的啸叫。 <br>
  ///        仅 ByteRTCRoomProfileCommunication、ByteRTCRoomProfileMeeting、ByteRTCRoomProfileMeetingRoom 三种房间模式支持啸叫抑制。 <br>
  ///        建议提醒用户检查客户端的距离或将麦克风和扬声器调至静音。
  ///
  ByteRTCMediaDeviceWarningCaptureDetectHowling(16),

  /// @hidden(macOS)
  /// @brief 当前 AudioScenario 不支持更改音频路由，设置音频路由失败
  ///
  ByteRTCMediaDeviceWarningSetAudioRouteInvalidScenario(20),

  /// @hidden(macOS)
  /// @brief 音频设备不存在，设置音频路由失败
  ///
  ByteRTCMediaDeviceWarningSetAudioRouteNotExists(21),

  /// @hidden(macOS)
  /// @brief 音频路由被系统或其他应用占用，设置音频路由失败
  ///
  ByteRTCMediaDeviceWarningSetAudioRouteFailedByPriority(22),

  /// @hidden(macOS)
  /// @brief 当前非通话模式 ByteRTCAudioScenarioCommunication，不支持设置音频路由
  ///
  ByteRTCMediaDeviceWarningSetAudioRouteNotVoipMode(23),

  /// @hidden(macOS)
  /// @brief 音频设备未启动，设置音频路由失败
  ///
  ByteRTCMediaDeviceWarningSetAudioRouteDeviceNotStart(24),

  /// @hidden(macOS)
  /// @brief 非纯媒体音频场景，此时不支持切换蓝牙传输协议。待切换至纯媒体音频场景后生效。
  ///
  ByteRTCMediaDeviceWarningSetBluetoothModeScenarioUnsupport(25),

  /// @hidden(macOS)
  /// @brief 当前不支持设置 HFP。
  ///
  ByteRTCMediaDeviceWarningSetBluetoothModeUnsupport(26);

  final dynamic $value;
  const ByteRTCMediaDeviceWarning([this.$value]);
}

enum ByteRTCPlayerEvent {
  /// @brief 开始切换音轨 <br>
  ///        开始调用 selectAudioTrack:{@link #ByteRTCMediaPlayer#selectAudioTrack} 时，返回此状态。
  ///

  ByteRTCPlayerEventSelectAudioTrackBegin(0),

  /// @brief 切换音轨成功 <br>
  ///        调用 selectAudioTrack:{@link #ByteRTCMediaPlayer#selectAudioTrack} 成功后，播放器切换到指定音轨播放，返回此状态。
  ///

  ByteRTCPlayerEventSelectAudioTrackCompleted(1),

  /// @brief 切换音轨失败 <br>
  ///        调用 selectAudioTrack:{@link #ByteRTCMediaPlayer#selectAudioTrack} 失败后，播放器无法切换到指定音轨，继续之前的音轨播放过程，返回此状态。
  ///

  ByteRTCPlayerEventSelectAudioTrackFailed(2),

  /// @brief 试图移动播放位置 <br>
  ///        开始调用 setPosition:{@link #ByteRTCMediaPlayer#setPosition} 时，返回此状态。
  ///

  ByteRTCPlayerEventSeekBegin(3),

  /// @brief 移动播放位置成功 <br>
  ///        调用 setPosition:{@link #ByteRTCMediaPlayer#setPosition} 成功后，返回此状态。
  ///

  ByteRTCPlayerEventSeekCompleted(4),

  /// @brief 移动播放位置失败 <br>
  ///        调用 setPosition:{@link #ByteRTCMediaPlayer#setPosition} 失败时，返回此状态。
  ///

  ByteRTCPlayerEventSeekFailed(5);

  final dynamic $value;
  const ByteRTCPlayerEvent([this.$value]);
}

enum ByteRTCMediaInputType {
  /// @brief 自定义采集。 <br>
  ///        设置完成后方可直接向 SDK 推送视频帧。
  ///
  ByteRTCMediaInputTypeExternal(0),

  /// @brief 内部 SDK 采集。 <br>
  ///        此设置仅切换至内部采集，你需继续调用 startVideoCapture{@link #ByteRTCEngine#startVideoCapture} 开启内部采集。
  ///
  ByteRTCMediaInputTypeInternal(1);

  final dynamic $value;
  const ByteRTCMediaInputType([this.$value]);
}

class ByteRTCRecordingConfig extends NativeClass {
  static const _$namespace = r'ByteRTCRecordingConfig';
  static get codegen_$namespace => _$namespace;

  ByteRTCRecordingConfig([NativeClassOptions? options])
      : super(options == null
            ? const NativeClassOptions(
                [],
                className: _$namespace,
                instanceType: InstanceType.manual,
                bridgeKey: 'com.volcengine.rtc.hybrid_runtime',
              )
            : NativeClassOptions.fromMap({
                ...options.toMap(),
                'bridgeKey': 'com.volcengine.rtc.hybrid_runtime',
              }));

  /// @brief 录制文件保存的绝对路径，需精确到文件夹，文件名由 RTC 自动生成。你需要确保对该路径具有读写权限。
  FutureOr<NSString?> get dirPath async {
    return await sendInstanceGet<NSString?>("dirPath");
  }

  set dirPath(FutureOr<NSString?> value) {
    sendInstanceSet("dirPath", value);
  }

  /// @brief 录制存储文件格式，参看 ByteRTCRecordingFileType{@link #ByteRTCRecordingFileType}
  FutureOr<ByteRTCRecordingFileType?> get recordingFileType async {
    try {
      final result =
          await sendInstanceGet<ByteRTCRecordingFileType?>("recordingFileType");
      if (result == null) {
        return null;
      }
      return ByteRTCRecordingFileType.values
          .firstWhere((element) => element == result);
    } catch (e) {
      return null;
    }
  }

  set recordingFileType(FutureOr<ByteRTCRecordingFileType?> value) {
    sendInstanceSet("recordingFileType", value);
  }
}

class ByteRTCHumanOrientation extends NativeClass {
  static const _$namespace = r'ByteRTCHumanOrientation';
  static get codegen_$namespace => _$namespace;

  ByteRTCHumanOrientation([NativeClassOptions? options])
      : super(options == null
            ? const NativeClassOptions(
                [],
                className: _$namespace,
                instanceType: InstanceType.manual,
                bridgeKey: 'com.volcengine.rtc.hybrid_runtime',
              )
            : NativeClassOptions.fromMap({
                ...options.toMap(),
                'bridgeKey': 'com.volcengine.rtc.hybrid_runtime',
              }));

  /// @brief 正前方朝向，默认值为 {1,0,0}，即正前方朝向 x 轴正方向
  FutureOr<ByteRTCOrientation?> get forward async {
    try {
      final result = await sendInstanceGet<ByteRTCOrientation?>("forward");
      if (result == null) {
        return null;
      }
      return packObject(
          result,
          () => ByteRTCOrientation(
              const NativeClassOptions([], disableInit: true)));
    } catch (e) {
      return null;
    }
  }

  set forward(FutureOr<ByteRTCOrientation?> value) {
    sendInstanceSet("forward", value);
  }

  /// @brief 正右方朝向，默认值为 {0,1,0}，即右手朝向 y 轴正方向
  FutureOr<ByteRTCOrientation?> get right async {
    try {
      final result = await sendInstanceGet<ByteRTCOrientation?>("right");
      if (result == null) {
        return null;
      }
      return packObject(
          result,
          () => ByteRTCOrientation(
              const NativeClassOptions([], disableInit: true)));
    } catch (e) {
      return null;
    }
  }

  set right(FutureOr<ByteRTCOrientation?> value) {
    sendInstanceSet("right", value);
  }

  /// @brief 正上方朝向，默认值为 {0,0,1}，即头顶朝向 z 轴正方向
  FutureOr<ByteRTCOrientation?> get up async {
    try {
      final result = await sendInstanceGet<ByteRTCOrientation?>("up");
      if (result == null) {
        return null;
      }
      return packObject(
          result,
          () => ByteRTCOrientation(
              const NativeClassOptions([], disableInit: true)));
    } catch (e) {
      return null;
    }
  }

  set up(FutureOr<ByteRTCOrientation?> value) {
    sendInstanceSet("up", value);
  }
}

enum ByteRTCWTNSubscribeStateChangeReason {
  /// @brief 订阅 WTN 媒体流成功
  ///
  ByteRTCWTNSubscribeStateChangeReasonSubscribe(0),

  /// @brief 其他原因订阅失败
  ///
  ByteRTCWTNSubscribeStateChangeReasonUnsubscribe(1300),

  /// @brief 订阅失败，拉流时远端未发布
  ///
  ByteRTCWTNSubscribeStateChangeReasonRemotePublish(1301),

  /// @brief 订阅失败，超出单端订阅上限。一个引擎实例最多拉 5 路流。
  ///
  ByteRTCWTNSubscribeStateChangeReasonOverClientSubscribeStreamLimit(1310),

  /// @brief 订阅失败。超出单流订阅人数上限。该限制由 RTC 服务端决定。
  ///
  ByteRTCWTNSubscribeStateChangeReasonOverStreamSubscribeUserLimit(1311),

  /// @brief 订阅失败。超出单流订阅请求 QPS 上限。该限制由 RTC 服务端决定。
  ///
  ByteRTCWTNSubscribeStateChangeReasonOverStreamSubscribeRequestLimit(1312);

  final dynamic $value;
  const ByteRTCWTNSubscribeStateChangeReason([this.$value]);
}

enum ByteRTCTorchState {
  /// @brief 相机补光灯关闭
  ///
  ByteRTCTorchStateOff(0),

  /// @brief 相机补光灯打开
  ///
  ByteRTCTorchStateOn(1);

  final dynamic $value;
  const ByteRTCTorchState([this.$value]);
}

class ByteRTCOrientation extends NativeClass {
  static const _$namespace = r'ByteRTCOrientation';
  static get codegen_$namespace => _$namespace;

  ByteRTCOrientation([NativeClassOptions? options])
      : super(options == null
            ? const NativeClassOptions(
                [],
                className: _$namespace,
                instanceType: InstanceType.manual,
                bridgeKey: 'com.volcengine.rtc.hybrid_runtime',
              )
            : NativeClassOptions.fromMap({
                ...options.toMap(),
                'bridgeKey': 'com.volcengine.rtc.hybrid_runtime',
              }));

  /// @brief x 坐标
  FutureOr<float?> get x async {
    return await sendInstanceGet<float?>("x");
  }

  set x(FutureOr<float?> value) {
    sendInstanceSet("x", value);
  }

  /// @brief y 坐标
  FutureOr<float?> get y async {
    return await sendInstanceGet<float?>("y");
  }

  set y(FutureOr<float?> value) {
    sendInstanceSet("y", value);
  }

  /// @brief z 坐标
  FutureOr<float?> get z async {
    return await sendInstanceGet<float?>("z");
  }

  set z(FutureOr<float?> value) {
    sendInstanceSet("z", value);
  }
}

enum ByteRTCMixedStreamMediaType {
  /// @brief 包含音频和视频
  ///
  ByteRTCMixedStreamMediaTypeAudioAndVideo(0),

  /// @brief 只包含音频
  ///
  ByteRTCMixedStreamMediaTypeAudioOnly(1),

  /// @hidden currently not available
  /// @brief 只包含视频
  ///
  ByteRTCMixedStreamMediaTypeVideoOnly(2);

  final dynamic $value;
  const ByteRTCMixedStreamMediaType([this.$value]);
}

enum ByteRTCVideoEnhancementMode {
  /// @brief 关闭弱光适应
  ///
  ByteRTCVideoEnhancementModeDisabled(0),

  /// @brief 开启弱光适应
  ///
  ByteRTCVideoEnhancementModeAuto(1);

  final dynamic $value;
  const ByteRTCVideoEnhancementMode([this.$value]);
}

class ByteRTCMixedStreamSpatialAudioConfig extends NativeClass {
  static const _$namespace = r'ByteRTCMixedStreamSpatialAudioConfig';
  static get codegen_$namespace => _$namespace;

  ByteRTCMixedStreamSpatialAudioConfig([NativeClassOptions? options])
      : super(options == null
            ? const NativeClassOptions(
                [],
                className: _$namespace,
                instanceType: InstanceType.manual,
                bridgeKey: 'com.volcengine.rtc.hybrid_runtime',
              )
            : NativeClassOptions.fromMap({
                ...options.toMap(),
                'bridgeKey': 'com.volcengine.rtc.hybrid_runtime',
              }));

  /// @brief 是否开启推流 CDN 时的空间音频效果。 <br>
  ///        当你启用此效果时，你需要设定推流中各个 ByteRTCMixedStreamLayoutRegionConfig{@link #ByteRTCMixedStreamLayoutRegionConfig} 的 `spatialPosition` 值，实现空间音频效果。
  FutureOr<BOOL?> get enableSpatialRender async {
    return await sendInstanceGet<BOOL?>("enableSpatialRender");
  }

  set enableSpatialRender(FutureOr<BOOL?> value) {
    sendInstanceSet("enableSpatialRender", value);
  }

  /// @brief 听众的空间位置。参看 ByteRTCPosition{@link #ByteRTCPosition}。 <br>
  ///        听众指收听来自 CDN 的音频流的用户。 <br>
  ///        WTN 流任务不支持设置本参数。
  FutureOr<ByteRTCPosition?> get audienceSpatialPosition async {
    try {
      final result =
          await sendInstanceGet<ByteRTCPosition?>("audienceSpatialPosition");
      if (result == null) {
        return null;
      }
      return packObject(
          result,
          () =>
              ByteRTCPosition(const NativeClassOptions([], disableInit: true)));
    } catch (e) {
      return null;
    }
  }

  set audienceSpatialPosition(FutureOr<ByteRTCPosition?> value) {
    sendInstanceSet("audienceSpatialPosition", value);
  }

  /// @brief 听众的空间朝向。参看 ByteRTCHumanOrientation{@link #ByteRTCHumanOrientation}。 <br>
  ///        听众指收听来自 CDN 的音频流的用户。
  FutureOr<ByteRTCHumanOrientation?> get audienceSpatialOrientation async {
    try {
      final result = await sendInstanceGet<ByteRTCHumanOrientation?>(
          "audienceSpatialOrientation");
      if (result == null) {
        return null;
      }
      return packObject(
          result,
          () => ByteRTCHumanOrientation(
              const NativeClassOptions([], disableInit: true)));
    } catch (e) {
      return null;
    }
  }

  set audienceSpatialOrientation(FutureOr<ByteRTCHumanOrientation?> value) {
    sendInstanceSet("audienceSpatialOrientation", value);
  }
}

enum ByteRTCMixedStreamAlternateImageFillMode {
  /// @brief 占位图跟随用户原始视频帧相同的比例缩放。默认设置。
  ///
  ByteRTCMixedStreamAlternateImageFillModeFit(0),

  /// @brief 占位图不跟随用户原始视频帧相同的比例缩放，保持图片原有比例。
  ///
  ByteRTCMixedStreamAlternateImageFillModeFill(1);

  final dynamic $value;
  const ByteRTCMixedStreamAlternateImageFillMode([this.$value]);
}

class ByteRTCLocalVideoSinkConfig extends NativeClass {
  static const _$namespace = r'ByteRTCLocalVideoSinkConfig';
  static get codegen_$namespace => _$namespace;

  ByteRTCLocalVideoSinkConfig([NativeClassOptions? options])
      : super(options == null
            ? const NativeClassOptions(
                [],
                className: _$namespace,
                instanceType: InstanceType.manual,
                bridgeKey: 'com.volcengine.rtc.hybrid_runtime',
              )
            : NativeClassOptions.fromMap({
                ...options.toMap(),
                'bridgeKey': 'com.volcengine.rtc.hybrid_runtime',
              }));

  /// @brief 本地视频帧回调位置，参看 ByteRTCLocalVideoSinkPosition{@link #ByteRTCLocalVideoSinkPosition}，默认回调前处理后的视频帧。
  FutureOr<ByteRTCLocalVideoSinkPosition?> get position async {
    try {
      final result =
          await sendInstanceGet<ByteRTCLocalVideoSinkPosition?>("position");
      if (result == null) {
        return null;
      }
      return ByteRTCLocalVideoSinkPosition.values
          .firstWhere((element) => element == result);
    } catch (e) {
      return null;
    }
  }

  set position(FutureOr<ByteRTCLocalVideoSinkPosition?> value) {
    sendInstanceSet("position", value);
  }

  /// @brief 本地视频帧回调格式，参看 ByteRTCVideoSinkPixelFormat{@link #ByteRTCVideoSinkPixelFormat}，默认值为 0。
  FutureOr<ByteRTCVideoSinkPixelFormat?> get requiredPixelFormat async {
    try {
      final result = await sendInstanceGet<ByteRTCVideoSinkPixelFormat?>(
          "requiredPixelFormat");
      if (result == null) {
        return null;
      }
      return ByteRTCVideoSinkPixelFormat.values
          .firstWhere((element) => element == result);
    } catch (e) {
      return null;
    }
  }

  set requiredPixelFormat(FutureOr<ByteRTCVideoSinkPixelFormat?> value) {
    sendInstanceSet("requiredPixelFormat", value);
  }
}

enum ByteRTCVideoDecoderConfig {
  /// @brief 开启 SDK 内部解码，只回调解码后的数据。回调为 renderPixelBuffer:rotation:contentType:extendedData:{@link #ByteRTCVideoSinkDelegate#renderPixelBuffer:rotation:contentType:extendedData}
  ///
  ByteRTCVideoDecoderConfigRaw(0),

  /// @brief 开启自定义解码，只回调解码前数据。回调为 onRemoteEncodedVideoFrame:info:withEncodedVideoFrame:{@link #ByteRTCRemoteEncodedVideoFrameObserver#onRemoteEncodedVideoFrame:info:withEncodedVideoFrame}。
  ///
  ByteRTCVideoDecoderConfigEncode(1),

  /// @brief 开启 SDK 内部解码，同时回调解码前和解码后的数据
  ///
  ByteRTCVideoDecoderConfigBoth(2);

  final dynamic $value;
  const ByteRTCVideoDecoderConfig([this.$value]);
}

enum ByteRTCAudioAlignmentMode {
  /// @brief 不对齐
  ///
  ByteRTCAudioAlignmentModeOff(0),

  /// @brief 远端音频流都对齐伴奏进度同步播放
  ///
  ByteRTCAudioAlignmentModeAudioMixing(1);

  final dynamic $value;
  const ByteRTCAudioAlignmentMode([this.$value]);
}

enum ByteRTCFrameRateRatio {
  /// @brief 100\%
  ///
  ByteRTCFrameRateRatioOrigin(0),

  /// @brief 50\%
  ///
  ByteRTCFrameRateRatioHalf(1),

  /// @brief 25\%
  ///
  ByteRTCFrameRateRatioQuater(2);

  final dynamic $value;
  const ByteRTCFrameRateRatio([this.$value]);
}

class ByteRTCStream extends NativeClass {
  static const _$namespace = r'ByteRTCStream';
  static get codegen_$namespace => _$namespace;

  ByteRTCStream([NativeClassOptions? options])
      : super(options == null
            ? const NativeClassOptions(
                [],
                className: _$namespace,
                instanceType: InstanceType.manual,
                bridgeKey: 'com.volcengine.rtc.hybrid_runtime',
              )
            : NativeClassOptions.fromMap({
                ...options.toMap(),
                'bridgeKey': 'com.volcengine.rtc.hybrid_runtime',
              }));

  /// @brief 发布此流的用户 ID。
  FutureOr<NSString?> get userId async {
    return await sendInstanceGet<NSString?>("userId");
  }

  set userId(FutureOr<NSString?> value) {
    sendInstanceSet("userId", value);
  }

  /// @brief 此流是否为共享屏幕流。
  FutureOr<BOOL?> get isScreen async {
    return await sendInstanceGet<BOOL?>("isScreen");
  }

  set isScreen(FutureOr<BOOL?> value) {
    sendInstanceSet("isScreen", value);
  }

  /// @brief 此流是否包括视频流。
  FutureOr<BOOL?> get hasVideo async {
    return await sendInstanceGet<BOOL?>("hasVideo");
  }

  set hasVideo(FutureOr<BOOL?> value) {
    sendInstanceSet("hasVideo", value);
  }

  /// @brief 流是否包括音频流。
  FutureOr<BOOL?> get hasAudio async {
    return await sendInstanceGet<BOOL?>("hasAudio");
  }

  set hasAudio(FutureOr<BOOL?> value) {
    sendInstanceSet("hasAudio", value);
  }

  /// @brief 视频流的分辨率信息。 <br>
  ///         当远端用户调用 setVideoEncoderConfig:{@link #ByteRTCEngine#setVideoEncoderConfig} 方法发布多个配置的视频流时，此处会包含该用户发布的所有视频流的属性信息。 <br>
  ///         参看 ByteRTCVideoSolution{@link #ByteRTCVideoSolution}。
  FutureOr<NSArray<ByteRTCVideoSolution>?> get videoStreamDescriptions async {
    try {
      final result = await sendInstanceGet<NSArray<ByteRTCVideoSolution>?>(
          "videoStreamDescriptions");
      if (result == null) {
        return null;
      }
      final list = result.map((e) => packObject(
          e,
          () => ByteRTCVideoSolution(
              const NativeClassOptions([], disableInit: true))));
      return list.toList();
    } catch (e) {
      return null;
    }
  }

  set videoStreamDescriptions(FutureOr<NSArray<ByteRTCVideoSolution>?> value) {
    sendInstanceSet("videoStreamDescriptions", value);
  }

  /// @brief 视频流最大分辨率，在开启多分辨率发布订阅时，回调发布端能支持的最大发布分辨率。
  FutureOr<ByteRTCVideoSolution?> get maxVideoStreamDescription async {
    try {
      final result = await sendInstanceGet<ByteRTCVideoSolution?>(
          "maxVideoStreamDescription");
      if (result == null) {
        return null;
      }
      return packObject(
          result,
          () => ByteRTCVideoSolution(
              const NativeClassOptions([], disableInit: true)));
    } catch (e) {
      return null;
    }
  }

  set maxVideoStreamDescription(FutureOr<ByteRTCVideoSolution?> value) {
    sendInstanceSet("maxVideoStreamDescription", value);
  }

  FutureOr<ByteRTCStreamIndex?> get streamIndex async {
    try {
      final result = await sendInstanceGet<ByteRTCStreamIndex?>("streamIndex");
      if (result == null) {
        return null;
      }
      return ByteRTCStreamIndex.values
          .firstWhere((element) => element == result);
    } catch (e) {
      return null;
    }
  }

  set streamIndex(FutureOr<ByteRTCStreamIndex?> value) {
    sendInstanceSet("streamIndex", value);
  }
}

class ByteRTCLocalProxyInfo extends NativeClass {
  static const _$namespace = r'ByteRTCLocalProxyInfo';
  static get codegen_$namespace => _$namespace;

  ByteRTCLocalProxyInfo([NativeClassOptions? options])
      : super(options == null
            ? const NativeClassOptions(
                [],
                className: _$namespace,
                instanceType: InstanceType.manual,
                bridgeKey: 'com.volcengine.rtc.hybrid_runtime',
              )
            : NativeClassOptions.fromMap({
                ...options.toMap(),
                'bridgeKey': 'com.volcengine.rtc.hybrid_runtime',
              }));

  /// @brief 本地代理的类型，参看 ByteRTCLocalProxyType{@link #ByteRTCLocalProxyType}。
  FutureOr<ByteRTCLocalProxyType?> get localProxyType async {
    try {
      final result =
          await sendInstanceGet<ByteRTCLocalProxyType?>("localProxyType");
      if (result == null) {
        return null;
      }
      return ByteRTCLocalProxyType.values
          .firstWhere((element) => element == result);
    } catch (e) {
      return null;
    }
  }

  set localProxyType(FutureOr<ByteRTCLocalProxyType?> value) {
    sendInstanceSet("localProxyType", value);
  }

  /// @detail keytype
  /// @brief 本地代理服务器 IP。
  FutureOr<NSString?> get localProxyIp async {
    return await sendInstanceGet<NSString?>("localProxyIp");
  }

  set localProxyIp(FutureOr<NSString?> value) {
    sendInstanceSet("localProxyIp", value);
  }

  /// @detail keytype
  /// @brief 本地代理服务器端口。
  FutureOr<int?> get localProxyPort async {
    return await sendInstanceGet<int?>("localProxyPort");
  }

  set localProxyPort(FutureOr<int?> value) {
    sendInstanceSet("localProxyPort", value);
  }

  /// @detail keytype
  /// @brief 本地代理用户名。
  ///
  FutureOr<NSString?> get localProxyUsername async {
    return await sendInstanceGet<NSString?>("localProxyUsername");
  }

  set localProxyUsername(FutureOr<NSString?> value) {
    sendInstanceSet("localProxyUsername", value);
  }

  /// @brief 本地代理密码。
  ///
  FutureOr<NSString?> get localProxyPassword async {
    return await sendInstanceGet<NSString?>("localProxyPassword");
  }

  set localProxyPassword(FutureOr<NSString?> value) {
    sendInstanceSet("localProxyPassword", value);
  }
}

class ByteRTCAudioFormat extends NativeClass {
  static const _$namespace = r'ByteRTCAudioFormat';
  static get codegen_$namespace => _$namespace;

  ByteRTCAudioFormat([NativeClassOptions? options])
      : super(options == null
            ? const NativeClassOptions(
                [],
                className: _$namespace,
                instanceType: InstanceType.manual,
                bridgeKey: 'com.volcengine.rtc.hybrid_runtime',
              )
            : NativeClassOptions.fromMap({
                ...options.toMap(),
                'bridgeKey': 'com.volcengine.rtc.hybrid_runtime',
              }));

  /// @brief 音频采样率，详见 ByteRTCAudioSampleRate{@link #ByteRTCAudioSampleRate}
  FutureOr<ByteRTCAudioSampleRate?> get sampleRate async {
    try {
      final result =
          await sendInstanceGet<ByteRTCAudioSampleRate?>("sampleRate");
      if (result == null) {
        return null;
      }
      return ByteRTCAudioSampleRate.values
          .firstWhere((element) => element == result);
    } catch (e) {
      return null;
    }
  }

  set sampleRate(FutureOr<ByteRTCAudioSampleRate?> value) {
    sendInstanceSet("sampleRate", value);
  }

  /// @brief 音频声道，详见 ByteRTCAudioChannel{@link #ByteRTCAudioChannel}
  FutureOr<ByteRTCAudioChannel?> get channel async {
    try {
      final result = await sendInstanceGet<ByteRTCAudioChannel?>("channel");
      if (result == null) {
        return null;
      }
      return ByteRTCAudioChannel.values
          .firstWhere((element) => element == result);
    } catch (e) {
      return null;
    }
  }

  set channel(FutureOr<ByteRTCAudioChannel?> value) {
    sendInstanceSet("channel", value);
  }

  /// @brief 单次回调的音频帧中包含的采样点数。默认值为 `0`，此时，采样点数取最小值。 <br>
  ///        最小值为回调间隔是 0.01s 时的值，即 `sampleRate * channel * 0.01s`。 <br>
  ///        最大值是 `2048`。超出取值范围时，采样点数取默认值。 <br>
  ///        该参数仅在设置读写回调时生效，调用 enableAudioFrameCallback:format:{@link #ByteRTCEngine#enableAudioFrameCallback:format} 开启只读模式回调时设置该参数不生效。
  FutureOr<int?> get samplesPerCall async {
    return await sendInstanceGet<int?>("samplesPerCall");
  }

  set samplesPerCall(FutureOr<int?> value) {
    sendInstanceSet("samplesPerCall", value);
  }
}

class ByteRTCMixedStreamPushTargetConfig extends NativeClass {
  static const _$namespace = r'ByteRTCMixedStreamPushTargetConfig';
  static get codegen_$namespace => _$namespace;

  ByteRTCMixedStreamPushTargetConfig([NativeClassOptions? options])
      : super(options == null
            ? const NativeClassOptions(
                [],
                className: _$namespace,
                instanceType: InstanceType.manual,
                bridgeKey: 'com.volcengine.rtc.hybrid_runtime',
              )
            : NativeClassOptions.fromMap({
                ...options.toMap(),
                'bridgeKey': 'com.volcengine.rtc.hybrid_runtime',
              }));

  /// @detail api
  /// @brief 推流 CDN 地址。仅支持 RTMP 协议，Url 必须满足正则 `/^rtmps?:\\/\\//`。建议设置。 <br>
  ///        本参数不支持过程中更新。
  ///        WTN 流任务不支持设置本参数。
  FutureOr<NSString?> get pushCDNURL async {
    return await sendInstanceGet<NSString?>("pushCDNURL");
  }

  set pushCDNURL(FutureOr<NSString?> value) {
    sendInstanceSet("pushCDNURL", value);
  }

  /// @detail keytype
  /// @brief WTN 流 ID。
  ///        合流任务不支持设置本参数。
  FutureOr<NSString?> get pushWTNStreamID async {
    return await sendInstanceGet<NSString?>("pushWTNStreamID");
  }

  set pushWTNStreamID(FutureOr<NSString?> value) {
    sendInstanceSet("pushWTNStreamID", value);
  }

  /// @detail keytype
  /// @brief 推流任务类型。
  FutureOr<ByteRTCMixedStreamPushTargetType?> get pushTargetType async {
    try {
      final result = await sendInstanceGet<ByteRTCMixedStreamPushTargetType?>(
          "pushTargetType");
      if (result == null) {
        return null;
      }
      return ByteRTCMixedStreamPushTargetType.values
          .firstWhere((element) => element == result);
    } catch (e) {
      return null;
    }
  }

  set pushTargetType(FutureOr<ByteRTCMixedStreamPushTargetType?> value) {
    sendInstanceSet("pushTargetType", value);
  }
}

class ByteRTCMediaTypeEnhancementConfig extends NativeClass {
  static const _$namespace = r'ByteRTCMediaTypeEnhancementConfig';
  static get codegen_$namespace => _$namespace;

  ByteRTCMediaTypeEnhancementConfig([NativeClassOptions? options])
      : super(options == null
            ? const NativeClassOptions(
                [],
                className: _$namespace,
                instanceType: InstanceType.manual,
                bridgeKey: 'com.volcengine.rtc.hybrid_runtime',
              )
            : NativeClassOptions.fromMap({
                ...options.toMap(),
                'bridgeKey': 'com.volcengine.rtc.hybrid_runtime',
              }));

  /// @brief 对信令消息，是否启用蜂窝网络辅助增强。默认不启用。
  FutureOr<BOOL?> get enhanceSignaling async {
    return await sendInstanceGet<BOOL?>("enhanceSignaling");
  }

  set enhanceSignaling(FutureOr<BOOL?> value) {
    sendInstanceSet("enhanceSignaling", value);
  }

  /// @brief 对屏幕共享以外的其他音频，是否启用蜂窝网络辅助增强。默认不启用。
  FutureOr<BOOL?> get enhanceAudio async {
    return await sendInstanceGet<BOOL?>("enhanceAudio");
  }

  set enhanceAudio(FutureOr<BOOL?> value) {
    sendInstanceSet("enhanceAudio", value);
  }

  /// @brief 对屏幕共享视频以外的其他视频，是否启用蜂窝网络辅助增强。默认不启用。
  FutureOr<BOOL?> get enhanceVideo async {
    return await sendInstanceGet<BOOL?>("enhanceVideo");
  }

  set enhanceVideo(FutureOr<BOOL?> value) {
    sendInstanceSet("enhanceVideo", value);
  }

  /// @brief 对屏幕共享音频，是否启用蜂窝网络辅助增强。默认不启用。
  FutureOr<BOOL?> get enhanceScreenAudio async {
    return await sendInstanceGet<BOOL?>("enhanceScreenAudio");
  }

  set enhanceScreenAudio(FutureOr<BOOL?> value) {
    sendInstanceSet("enhanceScreenAudio", value);
  }

  /// @brief 对屏幕共享视频，是否启用蜂窝网络辅助增强。默认不启用。
  FutureOr<BOOL?> get enhanceScreenVideo async {
    return await sendInstanceGet<BOOL?>("enhanceScreenVideo");
  }

  set enhanceScreenVideo(FutureOr<BOOL?> value) {
    sendInstanceSet("enhanceScreenVideo", value);
  }
}

enum ByteRTCRangeAudioMode {
  /// @brief 默认模式
  ///
  ByteRTCRangeAudioModeUndefined(0),

  /// @brief 小队模式
  ///
  ByteRTCRangeAudioModeTeam(1),

  /// @brief 世界模式
  ///
  ByteRTCRangeAudioModeWorld(2);

  final dynamic $value;
  const ByteRTCRangeAudioMode([this.$value]);
}

class ByteRTCMediaPlayerCustomSource extends NativeClass {
  static const _$namespace = r'ByteRTCMediaPlayerCustomSource';
  static get codegen_$namespace => _$namespace;

  ByteRTCMediaPlayerCustomSource([NativeClassOptions? options])
      : super(options == null
            ? const NativeClassOptions(
                [],
                className: _$namespace,
                instanceType: InstanceType.manual,
                bridgeKey: 'com.volcengine.rtc.hybrid_runtime',
              )
            : NativeClassOptions.fromMap({
                ...options.toMap(),
                'bridgeKey': 'com.volcengine.rtc.hybrid_runtime',
              }));

  /// @detail keytype
  /// @brief 仅使用内存播放时，传入对应的 ByteRTCMediaPlayerCustomSourceProvider{@link #ByteRTCMediaPlayerCustomSourceProvider} 实例。
  FutureOr<ByteRTCMediaPlayerCustomSourceProvider?> get provider async {
    try {
      final result =
          await sendInstanceGet<ByteRTCMediaPlayerCustomSourceProvider?>(
              "provider");
      if (result == null) {
        return null;
      }
      return packObject(
          result,
          () => ByteRTCMediaPlayerCustomSourceProvider(
              const NativeClassOptions([], disableInit: true)));
    } catch (e) {
      return null;
    }
  }

  set provider(FutureOr<ByteRTCMediaPlayerCustomSourceProvider?> value) {
    sendInstanceSet("provider", value);
  }

  /// @detail keytype
  /// @brief 数据源模式，详见 ByteRTCMediaPlayerCustomSourceMode{@link #ByteRTCMediaPlayerCustomSourceMode}。默认为 `push`。
  FutureOr<ByteRTCMediaPlayerCustomSourceMode?> get mode async {
    try {
      final result =
          await sendInstanceGet<ByteRTCMediaPlayerCustomSourceMode?>("mode");
      if (result == null) {
        return null;
      }
      return ByteRTCMediaPlayerCustomSourceMode.values
          .firstWhere((element) => element == result);
    } catch (e) {
      return null;
    }
  }

  set mode(FutureOr<ByteRTCMediaPlayerCustomSourceMode?> value) {
    sendInstanceSet("mode", value);
  }

  /// @detail keytype
  /// @brief 数据源类型，详见 ByteRTCMediaPlayerCustomSourceStreamType{@link #ByteRTCMediaPlayerCustomSourceStreamType}。默认为 `raw`。
  FutureOr<ByteRTCMediaPlayerCustomSourceStreamType?> get type async {
    try {
      final result =
          await sendInstanceGet<ByteRTCMediaPlayerCustomSourceStreamType?>(
              "type");
      if (result == null) {
        return null;
      }
      return ByteRTCMediaPlayerCustomSourceStreamType.values
          .firstWhere((element) => element == result);
    } catch (e) {
      return null;
    }
  }

  set type(FutureOr<ByteRTCMediaPlayerCustomSourceStreamType?> value) {
    sendInstanceSet("type", value);
  }
}

enum ByteRTCVideoSimulcastMode {
  /// @brief 单流模式。始终只有 1 路分辨率的流。
  ///
  ByteRTCVideoSimulcastModeOnlyOne(0),

  /// @brief 按需订阅模式。发送端会根据订阅端的状态，按需发布。无订阅偏好设置默认发送 2 路。
  ///
  ByteRTCVideoSimulcastModeOnDemand(1),

  /// @brief 订阅弱流。发送端始终按照设置的参数发布所有大小流。默认发送 2 路。
  ///
  ByteRTCVideoSimulcastModeAlwaysSimulcast(2);

  final dynamic $value;
  const ByteRTCVideoSimulcastMode([this.$value]);
}

enum ByteRTCUserRoleType {
  /// @brief 主播角色。该角色用户可在房间内发布和订阅音视频流，房间中的其他用户可以感知到该用户的存在。
  ///
  ByteRTCUserRoleTypeBroadcaster(1),

  /// @brief 隐身用户角色。此角色用户只可在房间内订阅音视频流，房间中的其他用户无法感知到该用户的存在。
  ///
  ByteRTCUserRoleTypeSilentAudience(2);

  final dynamic $value;
  const ByteRTCUserRoleType([this.$value]);
}

enum ByteRTCSubscribeStateChangeReason {
  /// @brief 本端调用订阅
  ///
  ByteRTCSubscribeStateChangeReasonSubscribe(0),

  /// @brief 本端取消订阅
  ///
  ByteRTCSubscribeStateChangeReasonUnsubscribe(1),

  /// @brief 远端发布流
  ///
  ByteRTCSubscribeStateChangeReasonRemotePublish(2),

  /// @brief 远端取消发布流
  ///
  ByteRTCSubscribeStateChangeReasonRemoteUnpublish(3),

  /// @brief 由于服务器错误导致订阅失败。SDK 会自动重试订阅
  ///
  ByteRTCSubscribeStateChangeReasonStreamFailed5xx(4),

  /// @brief 当前房间中找不到订阅的音视频流导致订阅失败。SDK 会自动重试订阅，若仍订阅失败则建议你退出重试。
  ///
  ByteRTCSubscribeStateChangeReasonStreamFailed404(5),

  /// @brief 当用户订阅的音视频流总数已达上限时，继续订阅更多流时会失败，同时用户会收到此错误通知。
  ///
  ByteRTCSubscribeStateChangeReasonOverStreamSubscribeLimit(6),

  /// @brief 用户订阅所在房间中的音视频流失败，失败原因为用户没有订阅流的权限。
  ///
  ByteRTCSubscribeStateChangeReasonNoSubscribePermission(7);

  final dynamic $value;
  const ByteRTCSubscribeStateChangeReason([this.$value]);
}

class ByteRTCChorusCacheSyncConfig extends NativeClass {
  static const _$namespace = r'ByteRTCChorusCacheSyncConfig';
  static get codegen_$namespace => _$namespace;

  ByteRTCChorusCacheSyncConfig([NativeClassOptions? options])
      : super(options == null
            ? const NativeClassOptions(
                [],
                className: _$namespace,
                instanceType: InstanceType.manual,
                bridgeKey: 'com.volcengine.rtc.hybrid_runtime',
              )
            : NativeClassOptions.fromMap({
                ...options.toMap(),
                'bridgeKey': 'com.volcengine.rtc.hybrid_runtime',
              }));

  /// @brief 最大媒体缓存时长（ms）。 <br>
  ///        取值范围是 `[500, 2500]`，默认值是 `2000`。 <br>
  ///        值越大，同步效果越好，但会造成占用内存较大。如果参与缓存同步的各路媒体流之间的时间差超过此值，会造成丢帧。
  FutureOr<int?> get maxCacheTimeMs async {
    return await sendInstanceGet<int?>("maxCacheTimeMs");
  }

  set maxCacheTimeMs(FutureOr<int?> value) {
    sendInstanceSet("maxCacheTimeMs", value);
  }

  /// @brief 模式。参看 ByteRTCChorusCacheSyncMode{@link #ByteRTCChorusCacheSyncMode}. 默认值是 `retransmitter`。
  FutureOr<ByteRTCChorusCacheSyncMode?> get mode async {
    try {
      final result = await sendInstanceGet<ByteRTCChorusCacheSyncMode?>("mode");
      if (result == null) {
        return null;
      }
      return ByteRTCChorusCacheSyncMode.values
          .firstWhere((element) => element == result);
    } catch (e) {
      return null;
    }
  }

  set mode(FutureOr<ByteRTCChorusCacheSyncMode?> value) {
    sendInstanceSet("mode", value);
  }

  /// @brief 收到 onSyncedVideoFrames:withUids:{@link #ByteRTCChorusCacheSyncObserver#onSyncedVideoFrames:withUids} 的频率。 <br>
  ///        默认值是 `15`。此值通常应小于等于原始视频帧率；如果大于原始视频帧率，可能会收到重复帧。
  FutureOr<int?> get videoFps async {
    return await sendInstanceGet<int?>("videoFps");
  }

  set videoFps(FutureOr<int?> value) {
    sendInstanceSet("videoFps", value);
  }
}

enum ByteRTCMixedStreamAudioCodecType {
  /// @detail keytype
  /// @brief AAC 格式。
  ///
  ByteRTCMixedStreamAudioCodecTypeAAC(0);

  final dynamic $value;
  const ByteRTCMixedStreamAudioCodecType([this.$value]);
}

enum ByteRTCRemoteVideoSinkPosition {
  /// @hidden not available
  /// @brief 解码后。
  ///
  ByteRTCRemoteVideoSinkPositionAfterDecoder(0),

  /// @brief （默认值）后处理后。
  ///
  ByteRTCRemoteVideoSinkPositionAfterPostprocess(1);

  final dynamic $value;
  const ByteRTCRemoteVideoSinkPosition([this.$value]);
}

class ByteRTCSubscribeVideoConfig extends NativeClass {
  static const _$namespace = r'ByteRTCSubscribeVideoConfig';
  static get codegen_$namespace => _$namespace;

  ByteRTCSubscribeVideoConfig([NativeClassOptions? options])
      : super(options == null
            ? const NativeClassOptions(
                [],
                className: _$namespace,
                instanceType: InstanceType.manual,
                bridgeKey: 'com.volcengine.rtc.hybrid_runtime',
              )
            : NativeClassOptions.fromMap({
                ...options.toMap(),
                'bridgeKey': 'com.volcengine.rtc.hybrid_runtime',
              }));

  /// @brief 订阅的视频流分辨率下标。 <br>
  ///        当远端用户通过调用 setLocalSimulcastMode:{@link #ByteRTCEngine#setlocalsimulcastmode} 方法启动发布多路不同分辨率的视频流时，本地用户需通过此参数指定希望订阅的流。 <br>
  ///        默认值为 0，即订阅第一路流。 <br>
  ///        如果不想更改之前的设置，可以输入 -1。
  FutureOr<NSInteger?> get videoIndex async {
    return await sendInstanceGet<NSInteger?>("videoIndex");
  }

  set videoIndex(FutureOr<NSInteger?> value) {
    sendInstanceSet("videoIndex", value);
  }

  /// @brief 远端用户优先级，参看 ByteRTCRemoteUserPriority{@link #ByteRTCRemoteUserPriority}，默认值为 0。
  FutureOr<NSInteger?> get priority async {
    return await sendInstanceGet<NSInteger?>("priority");
  }

  set priority(FutureOr<NSInteger?> value) {
    sendInstanceSet("priority", value);
  }
}

class ByteRTCVoiceEqualizationConfig extends NativeClass {
  static const _$namespace = r'ByteRTCVoiceEqualizationConfig';
  static get codegen_$namespace => _$namespace;

  ByteRTCVoiceEqualizationConfig([NativeClassOptions? options])
      : super(options == null
            ? const NativeClassOptions(
                [],
                className: _$namespace,
                instanceType: InstanceType.manual,
                bridgeKey: 'com.volcengine.rtc.hybrid_runtime',
              )
            : NativeClassOptions.fromMap({
                ...options.toMap(),
                'bridgeKey': 'com.volcengine.rtc.hybrid_runtime',
              }));

  /// @brief 频带。参看 ByteRTCBandFrequency{@link #ByteRTCBandFrequency}。
  FutureOr<ByteRTCBandFrequency?> get frequency async {
    try {
      final result = await sendInstanceGet<ByteRTCBandFrequency?>("frequency");
      if (result == null) {
        return null;
      }
      return ByteRTCBandFrequency.values
          .firstWhere((element) => element == result);
    } catch (e) {
      return null;
    }
  }

  set frequency(FutureOr<ByteRTCBandFrequency?> value) {
    sendInstanceSet("frequency", value);
  }

  /// @brief 频带增益（dB）。取值范围是 `[-15, 15]`。
  FutureOr<int?> get gain async {
    return await sendInstanceGet<int?>("gain");
  }

  set gain(FutureOr<int?> value) {
    sendInstanceSet("gain", value);
  }
}

enum ByteRTCWTNSubscribeState {
  /// @brief 订阅 WTN 媒体流
  ///
  ByteRTCWTNSubscribeStateSubscribe(0),

  /// @brief 取消订阅 WTN 媒体流
  ///
  ByteRTCWTNSubscribeStateUnsubscribe(1);

  final dynamic $value;
  const ByteRTCWTNSubscribeState([this.$value]);
}

class ByteRTCDownloadResult extends NativeClass {
  static const _$namespace = r'ByteRTCDownloadResult';
  static get codegen_$namespace => _$namespace;

  ByteRTCDownloadResult([NativeClassOptions? options])
      : super(options == null
            ? const NativeClassOptions(
                [],
                className: _$namespace,
                instanceType: InstanceType.manual,
                bridgeKey: 'com.volcengine.rtc.hybrid_runtime',
              )
            : NativeClassOptions.fromMap({
                ...options.toMap(),
                'bridgeKey': 'com.volcengine.rtc.hybrid_runtime',
              }));

  /// @brief 音乐 ID。
  FutureOr<NSString?> get musicId async {
    return await sendInstanceGet<NSString?>("musicId");
  }

  set musicId(FutureOr<NSString?> value) {
    sendInstanceSet("musicId", value);
  }

  /// @brief 下载文件类型，参看 ByteRTCDownloadFileType{@link #ByteRTCDownloadFileType}。
  FutureOr<ByteRTCDownloadFileType?> get fileType async {
    try {
      final result =
          await sendInstanceGet<ByteRTCDownloadFileType?>("fileType");
      if (result == null) {
        return null;
      }
      return ByteRTCDownloadFileType.values
          .firstWhere((element) => element == result);
    } catch (e) {
      return null;
    }
  }

  set fileType(FutureOr<ByteRTCDownloadFileType?> value) {
    sendInstanceSet("fileType", value);
  }

  /// @brief 文件存放路径。
  FutureOr<NSString?> get filePath async {
    return await sendInstanceGet<NSString?>("filePath");
  }

  set filePath(FutureOr<NSString?> value) {
    sendInstanceSet("filePath", value);
  }
}

enum ByteRTCRoomEvent {
  /// @brief 当房间内人数超过 500 人时，停止向房间内已有用户发送 `rtcRoom:onUserJoined:` 和 `rtcEngine:onUserLeave:reason:` 回调，并通过广播提示房间内所有用户。通过 rtcRoom:onRoomStateChanged:withUid:state:extraInfo:{@link #ByteRTCRoomDelegate#rtcRoom:onRoomStateChanged:withUid:state:extraInfo} 回调。
  ///
  ByteRTCRoomEventUserNotifyStop(-2013),

  /// @brief 房间/用户被封禁，通过房间事件通知封禁时间。
  ///
  ByteRTCRoomEventForbidden(-2012);

  final dynamic $value;
  const ByteRTCRoomEvent([this.$value]);
}

enum ByteRTCSingleStreamPushType {
  /// @brief 单流转推到CDN
  ///
  ByteRTCSingleStreamPushToCDN(1),

  /// @brief 单流转推到RTC房间
  ///
  ByteRTCSingleStreamPushToRTC(2);

  final dynamic $value;
  const ByteRTCSingleStreamPushType([this.$value]);
}

enum ByteRTCMixedStreamLayoutRegionType {
  /// @brief 视频。
  ///
  ByteRTCMixedStreamLayoutRegionTypeVideoStream(0),

  /// @brief 水印图片。
  ///
  ByteRTCMixedStreamLayoutRegionTypeImage(1);

  final dynamic $value;
  const ByteRTCMixedStreamLayoutRegionType([this.$value]);
}

enum ByteRTCAudioProfileType {
  /// @brief 默认音质 <br>
  ///        服务器下发或客户端已设置的 ByteRTCRoomProfile{@link #ByteRTCRoomProfile} 的音质配置
  ///
  ByteRTCAudioProfileDefault(0),

  /// @brief 流畅 <br>
  ///        单声道，采样率为 16 kHz，编码码率为 32 Kbps。 <br>
  ///        流畅优先、低功耗、低流量消耗，适用于大部分游戏场景，如小队语音、组队语音、国战语音等。
  ///
  ByteRTCAudioProfileFluent(1),

  /// @brief 单声道标准音质。 <br>
  ///        采样率为 24 kHz，编码码率为 48 Kbps。 <br>
  ///        适用于对音质有一定要求的场景，同时延时、功耗和流量消耗相对适中，适合教育场景和狼人杀等游戏。
  ///
  ByteRTCAudioProfileStandard(2),

  /// @brief 双声道音乐音质 <br>
  ///        采样率为 48 kHz，编码码率为 128 kbps。 <br>
  ///        超高音质，同时延时、功耗和流量消耗相对较大，适用于连麦 PK 等音乐场景。 <br>
  ///        游戏场景不建议使用。
  ///
  ByteRTCAudioProfileHD(3),

  /// @brief 双声道标准音质。采样率为 48 KHz，编码码率最大值为 80 Kbps
  ///
  ByteRTCAudioProfileStandardStereo(4),

  /// @brief 单声道音乐音质。采样率为 48 kHz，编码码率最大值为 64 Kbps
  ///
  ByteRTCAudioProfileHDMono(5);

  final dynamic $value;
  const ByteRTCAudioProfileType([this.$value]);
}

enum ByteRTCPublishStateChangeReason {
  /// @brief 用户调用发布
  ///
  ByteRTCPublishStateChangeReasonPublish(0),

  /// @brief 用户取消发布
  ///
  ByteRTCPublishStateChangeReasonUnpublish(1),

  /// @brief 发布 token 没有权限
  ///
  ByteRTCPublishStateChangeReasonNoPublishPermission(2),

  /// @brief 发布流总数超过上限
  ///
  ByteRTCPublishStateChangeReasonOverStreamPublishLimit(3),

  /// @brief 在一路流推多房间的场景下，在至少有两个房间在发布同一路流时，其中一个房间取消发布失败
  ///
  ByteRTCPublishStateChangeReasonMultiRoomUnpublishFailed(4),

  /// @brief 服务器错误导致发布失败
  ///
  ByteRTCPublishStateChangeReasonPublishStreamFailed(5),

  /// @brief 观众尝试发布操作
  ///
  ByteRTCPublishStateChangeReasonPublishStreamForbidden(6),

  /// @brief 用户已经在其他房间发布过流，或者用户正在发布。
  ///
  ByteRTCPublishStateChangeReasonUserInPublish(7),

  /// @brief 其他用户发布了相同streamid的流，导致本端发布的流被踢掉
  ///
  ByteRTCPublishStateChangeReasonStreamPublishByOther(8),

  /// @brief 流 ID 无效
  ///
  ByteRTCPublishStateChangeReasonStreamIdInvalid(9);

  final dynamic $value;
  const ByteRTCPublishStateChangeReason([this.$value]);
}

class ByteRTCVideoSinkDelegate extends NativeObserverClass {
  static const _$namespace = r'ByteRTCVideoSinkDelegate';

  ByteRTCVideoSinkDelegate([NativeClassOptions? options])
      : super(options == null
            ? const NativeClassOptions([],
                className: _$namespace,
                instanceType: InstanceType.manual,
                bridgeKey: 'com.volcengine.rtc.hybrid_runtime',
                methodMap: {
                  r"onFrame": r"onFrame:",
                  r"renderPixelBuffer$rotation$contentType$extendedData":
                      r"renderPixelBuffer:rotation:contentType:extendedData:"
                })
            : NativeClassOptions.fromMap({
                ...options.toMap(),
                'bridgeKey': 'com.volcengine.rtc.hybrid_runtime',
              })) {
    registerEvent(r"onFrame:", onFrame);

    registerEvent(r"renderPixelBuffer:rotation:contentType:extendedData:",
        renderPixelBuffer$rotation$contentType$extendedData);
  }

  /// @detail api
  /// @brief 输出视频的 PixelBuffer
  /// @param videoFrame 视频帧

  FutureOr<void> onFrame(id<ByteRTCVideoFrame> videoFrame) async {}

  /// @deprecated since 3.54, use onFrame: instead
  /// @detail api
  /// @brief 输出视频的 PixelBuffer
  /// @param pixelBuffer 视频的 PixelBuffer
  /// @param rotation 视频旋转角度，参看 ByteRTCVideoRotation{@link #ByteRTCVideoRotation}
  /// @param contentType 视频内部类型 参看 ByteRTCVideoContentType{@link #ByteRTCVideoContentType}
  /// @param extendedData 视频解码后获得的附加数据

  FutureOr<void> renderPixelBuffer$rotation$contentType$extendedData(
      CVPixelBufferRef pixelBuffer,
      ByteRTCVideoRotation rotation,
      ByteRTCVideoContentType contentType,
      NSData extendedData) async {}
}

class ByteRTCVirtualBackgroundSource extends NativeClass {
  static const _$namespace = r'ByteRTCVirtualBackgroundSource';
  static get codegen_$namespace => _$namespace;

  ByteRTCVirtualBackgroundSource([NativeClassOptions? options])
      : super(options == null
            ? const NativeClassOptions(
                [],
                className: _$namespace,
                instanceType: InstanceType.manual,
                bridgeKey: 'com.volcengine.rtc.hybrid_runtime',
              )
            : NativeClassOptions.fromMap({
                ...options.toMap(),
                'bridgeKey': 'com.volcengine.rtc.hybrid_runtime',
              }));

  /// @brief 虚拟背景类型，详见 ByteRTCVirtualBackgroundSourceType{@link #ByteRTCVirtualBackgroundSourceType} 。
  FutureOr<ByteRTCVirtualBackgroundSourceType?> get sourceType async {
    try {
      final result = await sendInstanceGet<ByteRTCVirtualBackgroundSourceType?>(
          "sourceType");
      if (result == null) {
        return null;
      }
      return ByteRTCVirtualBackgroundSourceType.values
          .firstWhere((element) => element == result);
    } catch (e) {
      return null;
    }
  }

  set sourceType(FutureOr<ByteRTCVirtualBackgroundSourceType?> value) {
    sendInstanceSet("sourceType", value);
  }

  /// @brief 纯色背景使用的颜色。 <br>
  ///        格式为 0xAARRGGBB 。
  FutureOr<int?> get sourceColor async {
    return await sendInstanceGet<int?>("sourceColor");
  }

  set sourceColor(FutureOr<int?> value) {
    sendInstanceSet("sourceColor", value);
  }

  /// @brief 自定义背景图片的绝对路径。 <br>
  ///       - 支持的格式为 jpg、jpeg、png。
  ///       - 图片分辨率超过 1080P 时，图片会被等比缩放至和视频一致。
  ///       - 图片和视频宽高比一致时，图片会被直接缩放至和视频一致。
  ///       - 图片和视频长宽比不一致时，为保证图片内容不变形，图片按短边缩放至与视频帧一致，使图片填满视频帧，对多出的高或宽进行剪裁。
  ///       - 自定义图片带有局部透明效果时，透明部分由黑色代替。
  FutureOr<NSString?> get sourcePath async {
    return await sendInstanceGet<NSString?>("sourcePath");
  }

  set sourcePath(FutureOr<NSString?> value) {
    sendInstanceSet("sourcePath", value);
  }
}

enum ByteRTCZoomDirectionType {
  /// @brief 相机向左移动
  ///
  ByteRTCZoomDirectionTypeMoveLeft(0),

  /// @brief 相机向右移动
  ///
  ByteRTCZoomDirectionTypeMoveRight(1),

  /// @brief 相机向上移动
  ///
  ByteRTCZoomDirectionTypeMoveUp(2),

  /// @brief 相机向下移动
  ///
  ByteRTCZoomDirectionTypeMoveDown(3),

  /// @brief 相机缩小焦距
  ///
  ByteRTCZoomDirectionTypeZoomOut(4),

  /// @brief 相机放大焦距
  ///
  ByteRTCZoomDirectionTypeZoomIn(5),

  /// @brief 恢复到原始画面
  ///
  ByteRTCZoomDirectionTypeReset(6);

  final dynamic $value;
  const ByteRTCZoomDirectionType([this.$value]);
}

enum ByteRTCAudioVADType {
  /// @brief 未检测到人声
  ///
  ByteRTCAudioVADTypeNoSpeech(0),

  /// @brief 检测到人声。
  ///
  ByteRTCAudioVADTypeSpeech(1);

  final dynamic $value;
  const ByteRTCAudioVADType([this.$value]);
}

enum ByteRTCPerformanceAlarmMode {
  /// @brief 未开启发布性能回退
  ///
  ByteRTCPerformanceAlarmModeNormal(0),

  /// @brief 已开启发布性能回退
  ///
  ByteRTCPerformanceAlarmModeSimulcast(1);

  final dynamic $value;
  const ByteRTCPerformanceAlarmMode([this.$value]);
}

class ByteRTCPositionInfo extends NativeClass {
  static const _$namespace = r'ByteRTCPositionInfo';
  static get codegen_$namespace => _$namespace;

  ByteRTCPositionInfo([NativeClassOptions? options])
      : super(options == null
            ? const NativeClassOptions(
                [],
                className: _$namespace,
                instanceType: InstanceType.manual,
                bridgeKey: 'com.volcengine.rtc.hybrid_runtime',
              )
            : NativeClassOptions.fromMap({
                ...options.toMap(),
                'bridgeKey': 'com.volcengine.rtc.hybrid_runtime',
              }));

  /// @brief 用户在空间音频坐标系里的位置，需自行建立空间直角坐标系。参看 ByteRTCPosition{@link #ByteRTCPosition}。
  FutureOr<ByteRTCPosition?> get position async {
    try {
      final result = await sendInstanceGet<ByteRTCPosition?>("position");
      if (result == null) {
        return null;
      }
      return packObject(
          result,
          () =>
              ByteRTCPosition(const NativeClassOptions([], disableInit: true)));
    } catch (e) {
      return null;
    }
  }

  set position(FutureOr<ByteRTCPosition?> value) {
    sendInstanceSet("position", value);
  }

  /// @brief 用户在空间音频坐标系里的三维朝向信息。三个向量需要两两垂直。参看 ByteRTCHumanOrientation{@link #ByteRTCHumanOrientation}。
  FutureOr<ByteRTCHumanOrientation?> get orientation async {
    try {
      final result =
          await sendInstanceGet<ByteRTCHumanOrientation?>("orientation");
      if (result == null) {
        return null;
      }
      return packObject(
          result,
          () => ByteRTCHumanOrientation(
              const NativeClassOptions([], disableInit: true)));
    } catch (e) {
      return null;
    }
  }

  set orientation(FutureOr<ByteRTCHumanOrientation?> value) {
    sendInstanceSet("orientation", value);
  }
}

enum ByteRTCDataFrameType {
  /// @brief SEI 数据
  ///
  ByteRTCDataFrameTypeSei(0),

  /// @brief 人脸识别数据
  ///
  ByteRTCDataFrameTypeRoi(1),

  /// @brief 其他数据帧类型
  ///
  ByteRTCDataFrameTypeOther(2);

  final dynamic $value;
  const ByteRTCDataFrameType([this.$value]);
}

enum ByteRTCRemoteVideoStateChangeReason {
  /// @brief 内部原因
  ///
  ByteRTCRemoteVideoStateChangeReasonInternal(0),

  /// @brief 网络阻塞
  ///
  ByteRTCRemoteVideoStateChangeReasonNetworkCongestion(1),

  /// @brief 网络恢复正常
  ///
  ByteRTCRemoteVideoStateChangeReasonNetworkRecovery(2),

  /// @brief 本地用户停止接收远端视频流或本地用户禁用视频模块
  ///
  ByteRTCRemoteVideoStateChangeReasonLocalMuted(3),

  /// @brief 本地用户恢复接收远端视频流或本地用户启用视频模块
  ///
  ByteRTCRemoteVideoStateChangeReasonLocalUnmuted(4),

  /// @brief 远端用户停止发送视频流或远端用户禁用视频模块
  ///
  ByteRTCRemoteVideoStateChangeReasonRemoteMuted(5),

  /// @brief 远端用户恢复发送视频流或远端用户启用视频模块
  ///
  ByteRTCRemoteVideoStateChangeReasonRemoteUnmuted(6),

  /// @brief 远端用户离开房间
  ///
  ByteRTCRemoteVideoStateChangeReasonRemoteOffline(7);

  final dynamic $value;
  const ByteRTCRemoteVideoStateChangeReason([this.$value]);
}

enum ByteRTCSnapshotErrorCode {
  /// @brief 截图成功。
  ///
  ByteRTCSnapshotErrorCodeOk(0),

  /// @brief 截图错误。生成图片数据失败或 RGBA 编码失败。
  ///
  ByteRTCSnapshotErrorCodeCreateFail(-1),

  /// @brief 截图错误。流无效。
  ///
  ByteRTCSnapshotErrorCodeStreamInvalid(-2),

  /// @brief 截图错误。截图超时，超时时间 1 秒。
  ///
  ByteRTCSnapshotErrorCodeTimeout(-3),

  /// @brief 截图错误。图片保存失败。
  ///
  ByteRTCSnapshotErrorCodeFileSaveError(-4);

  final dynamic $value;
  const ByteRTCSnapshotErrorCode([this.$value]);
}

enum ByteRTCRemoteMirrorType {
  /// @brief （默认值）远端视频渲染无镜像效果。
  ///
  ByteRTCRemoteMirrorTypeNone(0),

  /// @brief 远端视频渲染有镜像效果。
  ///
  ByteRTCRemoteMirrorTypeRender(1);

  final dynamic $value;
  const ByteRTCRemoteMirrorType([this.$value]);
}

class ByteRTCSysStats extends NativeClass {
  static const _$namespace = r'ByteRTCSysStats';
  static get codegen_$namespace => _$namespace;

  ByteRTCSysStats([NativeClassOptions? options])
      : super(options == null
            ? const NativeClassOptions(
                [],
                className: _$namespace,
                instanceType: InstanceType.manual,
                bridgeKey: 'com.volcengine.rtc.hybrid_runtime',
              )
            : NativeClassOptions.fromMap({
                ...options.toMap(),
                'bridgeKey': 'com.volcengine.rtc.hybrid_runtime',
              }));

  /// @brief 当前系统 cpu 核数
  FutureOr<int?> get cpuCores async {
    return await sendInstanceGet<int?>("cpuCores");
  }

  set cpuCores(FutureOr<int?> value) {
    sendInstanceSet("cpuCores", value);
  }

  /// @brief 当前应用的 CPU 使用率，取值范围为 [0, 1]。
  FutureOr<double?> get cpuAppUsage async {
    return await sendInstanceGet<double?>("cpuAppUsage");
  }

  set cpuAppUsage(FutureOr<double?> value) {
    sendInstanceSet("cpuAppUsage", value);
  }

  /// @hidden currently not available
  /// @brief 当前系统的 CPU 使用率，取值范围为 [0, 1]。
  FutureOr<double?> get cpuTotalUsage async {
    return await sendInstanceGet<double?>("cpuTotalUsage");
  }

  set cpuTotalUsage(FutureOr<double?> value) {
    sendInstanceSet("cpuTotalUsage", value);
  }

  /// @brief 当前 App 的内存使用（单位 MB）
  FutureOr<double?> get memoryUsage async {
    return await sendInstanceGet<double?>("memoryUsage");
  }

  set memoryUsage(FutureOr<double?> value) {
    sendInstanceSet("memoryUsage", value);
  }

  /// @brief 全量内存（单位 MB）
  FutureOr<longlong?> get fullMemory async {
    return await sendInstanceGet<longlong?>("fullMemory");
  }

  set fullMemory(FutureOr<longlong?> value) {
    sendInstanceSet("fullMemory", value);
  }

  /// @brief 系统已使用内存（单位 MB）
  FutureOr<longlong?> get totalMemoryUsage async {
    return await sendInstanceGet<longlong?>("totalMemoryUsage");
  }

  set totalMemoryUsage(FutureOr<longlong?> value) {
    sendInstanceSet("totalMemoryUsage", value);
  }

  /// @brief 空闲可分配内存（单位 MB）
  FutureOr<longlong?> get freeMemory async {
    return await sendInstanceGet<longlong?>("freeMemory");
  }

  set freeMemory(FutureOr<longlong?> value) {
    sendInstanceSet("freeMemory", value);
  }

  /// @brief 当前应用的内存使用率（单位 \%）
  FutureOr<double?> get memoryRatio async {
    return await sendInstanceGet<double?>("memoryRatio");
  }

  set memoryRatio(FutureOr<double?> value) {
    sendInstanceSet("memoryRatio", value);
  }

  /// @brief 系统内存使用率（单位 \%）
  FutureOr<double?> get totalMemoryRatio async {
    return await sendInstanceGet<double?>("totalMemoryRatio");
  }

  set totalMemoryRatio(FutureOr<double?> value) {
    sendInstanceSet("totalMemoryRatio", value);
  }
}

class ByteRTCFrameExtendedData extends NativeClass {
  static const _$namespace = r'ByteRTCFrameExtendedData';
  static get codegen_$namespace => _$namespace;

  ByteRTCFrameExtendedData([NativeClassOptions? options])
      : super(options == null
            ? const NativeClassOptions(
                [],
                className: _$namespace,
                instanceType: InstanceType.manual,
                bridgeKey: 'com.volcengine.rtc.hybrid_runtime',
              )
            : NativeClassOptions.fromMap({
                ...options.toMap(),
                'bridgeKey': 'com.volcengine.rtc.hybrid_runtime',
              }));

  /// @brief 数据类型，详见 ByteRTCDataFrameType{@link #ByteRTCDataFrameType}。
  FutureOr<ByteRTCDataFrameType?> get frameType async {
    try {
      final result = await sendInstanceGet<ByteRTCDataFrameType?>("frameType");
      if (result == null) {
        return null;
      }
      return ByteRTCDataFrameType.values
          .firstWhere((element) => element == result);
    } catch (e) {
      return null;
    }
  }

  set frameType(FutureOr<ByteRTCDataFrameType?> value) {
    sendInstanceSet("frameType", value);
  }

  /// @brief 附加数据
  FutureOr<NSData?> get extendedData async {
    return await sendInstanceGet<NSData?>("extendedData");
  }

  set extendedData(FutureOr<NSData?> value) {
    sendInstanceSet("extendedData", value);
  }

  /// @brief 附加数据长度
  FutureOr<NSInteger?> get extendedDataLen async {
    return await sendInstanceGet<NSInteger?>("extendedDataLen");
  }

  set extendedDataLen(FutureOr<NSInteger?> value) {
    sendInstanceSet("extendedDataLen", value);
  }
}

class ByteRTCSingScoringConfig extends NativeClass {
  static const _$namespace = r'ByteRTCSingScoringConfig';
  static get codegen_$namespace => _$namespace;

  ByteRTCSingScoringConfig([NativeClassOptions? options])
      : super(options == null
            ? const NativeClassOptions(
                [],
                className: _$namespace,
                instanceType: InstanceType.manual,
                bridgeKey: 'com.volcengine.rtc.hybrid_runtime',
              )
            : NativeClassOptions.fromMap({
                ...options.toMap(),
                'bridgeKey': 'com.volcengine.rtc.hybrid_runtime',
              }));

  /// @brief 打分维度，详见 ByteRTCMulDimSingScoringMode{@link #ByteRTCMulDimSingScoringMode}。
  ///
  FutureOr<ByteRTCMulDimSingScoringMode?> get mode async {
    try {
      final result =
          await sendInstanceGet<ByteRTCMulDimSingScoringMode?>("mode");
      if (result == null) {
        return null;
      }
      return ByteRTCMulDimSingScoringMode.values
          .firstWhere((element) => element == result);
    } catch (e) {
      return null;
    }
  }

  set mode(FutureOr<ByteRTCMulDimSingScoringMode?> value) {
    sendInstanceSet("mode", value);
  }

  /// @brief 音频采样率。仅支持 44100 Hz、48000 Hz。
  ///
  FutureOr<ByteRTCAudioSampleRate?> get sampleRate async {
    try {
      final result =
          await sendInstanceGet<ByteRTCAudioSampleRate?>("sampleRate");
      if (result == null) {
        return null;
      }
      return ByteRTCAudioSampleRate.values
          .firstWhere((element) => element == result);
    } catch (e) {
      return null;
    }
  }

  set sampleRate(FutureOr<ByteRTCAudioSampleRate?> value) {
    sendInstanceSet("sampleRate", value);
  }

  /// @brief 歌词文件路径。打分功能仅支持 KRC 歌词文件。
  ///
  FutureOr<NSString?> get lyricsFilepath async {
    return await sendInstanceGet<NSString?>("lyricsFilepath");
  }

  set lyricsFilepath(FutureOr<NSString?> value) {
    sendInstanceSet("lyricsFilepath", value);
  }

  /// @brief 歌曲 midi 文件路径。
  ///
  FutureOr<NSString?> get midiFilepath async {
    return await sendInstanceGet<NSString?>("midiFilepath");
  }

  set midiFilepath(FutureOr<NSString?> value) {
    sendInstanceSet("midiFilepath", value);
  }
}

enum ByteRTCAudioAEDType {
  /// @brief 未检测到音乐场景
  ///
  ByteRTCAudioAEDTypeNoMusic(0),

  /// @brief 检测到音乐场景。
  ///
  ByteRTCAudioAEDTypeMusic(1);

  final dynamic $value;
  const ByteRTCAudioAEDType([this.$value]);
}

enum ByteRTCEarMonitorMode {
  /// @brief 关闭。
  ///
  ByteRTCEarMonitorModeOff(0),

  /// @brief 开启。
  ///
  ByteRTCEarMonitorModeOn(1);

  final dynamic $value;
  const ByteRTCEarMonitorMode([this.$value]);
}

enum ByteRTCLogLevel {
  /// @brief 打印 trace 级别及以上级别信息。
  ///
  ByteRTCLogLevelTrace(0),

  /// @brief 打印 debug 级别及以上级别信息。
  ///
  ByteRTCLogLevelDebug(1),

  /// @brief 打印 info 级别及以上级别信息。
  ///
  ByteRTCLogLevelInfo(2),

  /// @brief 打印 warning 级别及以上级别信息。
  ///
  ByteRTCLogLevelWarning(3),

  /// @brief 打印 error 级别信息。
  ///
  ByteRTCLogLevelError(4);

  final dynamic $value;
  const ByteRTCLogLevel([this.$value]);
}

enum ByteRTCMirrorMode {
  ByteRTCMirrorModeOff(0),

  ByteRTCMirrorModeOn(1);

  final dynamic $value;
  const ByteRTCMirrorMode([this.$value]);
}

class ByteRTCScreenCaptureParam extends NativeClass {
  static const _$namespace = r'ByteRTCScreenCaptureParam';
  static get codegen_$namespace => _$namespace;

  ByteRTCScreenCaptureParam([NativeClassOptions? options])
      : super(options == null
            ? const NativeClassOptions(
                [],
                className: _$namespace,
                instanceType: InstanceType.manual,
                bridgeKey: 'com.volcengine.rtc.hybrid_runtime',
              )
            : NativeClassOptions.fromMap({
                ...options.toMap(),
                'bridgeKey': 'com.volcengine.rtc.hybrid_runtime',
              }));

  /// @brief 视频最大宽度，单位：像素。
  FutureOr<NSInteger?> get width async {
    return await sendInstanceGet<NSInteger?>("width");
  }

  set width(FutureOr<NSInteger?> value) {
    sendInstanceSet("width", value);
  }

  /// @brief 视频最大高度，单位：像素。
  FutureOr<NSInteger?> get height async {
    return await sendInstanceGet<NSInteger?>("height");
  }

  set height(FutureOr<NSInteger?> value) {
    sendInstanceSet("height", value);
  }

  /// @brief 屏幕采集帧率，单位：fps
  FutureOr<NSInteger?> get frameRate async {
    return await sendInstanceGet<NSInteger?>("frameRate");
  }

  set frameRate(FutureOr<NSInteger?> value) {
    sendInstanceSet("frameRate", value);
  }

  /// @brief 发送屏幕采集码率，单位 kbps
  FutureOr<NSInteger?> get bitrate async {
    return await sendInstanceGet<NSInteger?>("bitrate");
  }

  set bitrate(FutureOr<NSInteger?> value) {
    sendInstanceSet("bitrate", value);
  }

  /// @brief 视频最小编码码率, 单位 kbps。编码码率不会低于 `minBitrate`。 <br>
  ///        默认值为 `0`。 <br>
  ///        范围：[0, bitrate)，当 `bitrate` < `minBitrate` 时，为适配码率模式。 <br>
  ///        以下情况，设置本参数无效： <br>
  ///        - 当 `bitrate` 为 `0` 时，不对视频流进行编码发送。
  ///        - 当 `bitrate` < `0` 时，适配码率模式。
  FutureOr<NSInteger?> get minBitrate async {
    return await sendInstanceGet<NSInteger?>("minBitrate");
  }

  set minBitrate(FutureOr<NSInteger?> value) {
    sendInstanceSet("minBitrate", value);
  }

  /// @brief 采集区域
  FutureOr<CGRect?> get regionRect async {
    return await sendInstanceGet<CGRect?>("regionRect");
  }

  set regionRect(FutureOr<CGRect?> value) {
    sendInstanceSet("regionRect", value);
  }

  /// @brief 是否采集鼠标
  FutureOr<ByteRTCMouseCursorCaptureState?> get mouseCursorCaptureState async {
    try {
      final result = await sendInstanceGet<ByteRTCMouseCursorCaptureState?>(
          "mouseCursorCaptureState");
      if (result == null) {
        return null;
      }
      return ByteRTCMouseCursorCaptureState.values
          .firstWhere((element) => element == result);
    } catch (e) {
      return null;
    }
  }

  set mouseCursorCaptureState(FutureOr<ByteRTCMouseCursorCaptureState?> value) {
    sendInstanceSet("mouseCursorCaptureState", value);
  }

  /// @brief 屏幕过滤设置
  FutureOr<NSArray<NSNumber>?> get excludedWindowList async {
    return await sendInstanceGet<NSArray<NSNumber>?>("excludedWindowList");
  }

  set excludedWindowList(FutureOr<NSArray<NSNumber>?> value) {
    sendInstanceSet("excludedWindowList", value);
  }

  /// @brief 采集区域的边框高亮设置，参看 ByteRTCHighlightConfig{@link #ByteRTCHighlightConfig}。
  FutureOr<ByteRTCHighlightConfig?> get highlightConfig async {
    try {
      final result =
          await sendInstanceGet<ByteRTCHighlightConfig?>("highlightConfig");
      if (result == null) {
        return null;
      }
      return packObject(
          result,
          () => ByteRTCHighlightConfig(
              const NativeClassOptions([], disableInit: true)));
    } catch (e) {
      return null;
    }
  }

  set highlightConfig(FutureOr<ByteRTCHighlightConfig?> value) {
    sendInstanceSet("highlightConfig", value);
  }
}

enum ByteRTCVideoRotationMode {
  /// @brief App 方向
  ///
  ByteRTCVideoRotationModeFollowApp(0),

  /// @brief 重力方向
  ///
  ByteRTCVideoRotationModeFollowGSensor(1);

  final dynamic $value;
  const ByteRTCVideoRotationMode([this.$value]);
}

enum ByteRTCNetworkQuality {
  /// @brief 网络质量未知。
  ///
  ByteRTCNetworkQualityUnknown(0),

  /// @brief 网络质量极好。
  ///
  ByteRTCNetworkQualityExcellent(1),

  /// @brief 主观感觉和 kNetworkQualityExcellent 差不多，但码率可能略低。
  ///
  ByteRTCNetworkQualityGood(2),

  /// @brief 主观感受有瑕疵但不影响沟通。
  ///
  ByteRTCNetworkQualityPoor(3),

  /// @brief 勉强能沟通但不顺畅。
  ///
  ByteRTCNetworkQualityBad(4),

  /// @brief 网络质量非常差，基本不能沟通。
  ///
  ByteRTCNetworkQualityVeryBad(5),

  /// @brief 网络连接断开，无法通话。网络可能由于 12s 内无应答、开启飞行模式、拔掉网线等原因断开。 <br>
  ///        更多网络状态信息参见 [连接状态提示](https://www.volcengine.com/docs/6348/95376)。
  ///
  ByteRTCNetworkQualityDown(6);

  final dynamic $value;
  const ByteRTCNetworkQuality([this.$value]);
}

enum ByteRTCAlphaLayout {
  /// @brief Alpha 数据置于 RGB 数据上方。
  ///
  ByteRTCAlphaLayoutTop(0),

  /// @hidden currently not available
  /// @brief Alpha 数据置于 RGB 数据下方。
  ///
  ByteRTCAlphaLayoutBottom(1),

  /// @hidden currently not available
  /// @brief Alpha 数据置于 RGB 数据左方。
  ///
  ByteRTCAlphaLayoutLeft(2),

  /// @hidden currently not available
  /// @brief The Alpha data is placed to the right of the RGB data.
  ///

  ByteRTCAlphaLayoutRight(3);

  final dynamic $value;
  const ByteRTCAlphaLayout([this.$value]);
}

class ByteRTCRemoteVideoRenderConfig extends NativeClass {
  static const _$namespace = r'ByteRTCRemoteVideoRenderConfig';
  static get codegen_$namespace => _$namespace;

  ByteRTCRemoteVideoRenderConfig([NativeClassOptions? options])
      : super(options == null
            ? const NativeClassOptions(
                [],
                className: _$namespace,
                instanceType: InstanceType.manual,
                bridgeKey: 'com.volcengine.rtc.hybrid_runtime',
              )
            : NativeClassOptions.fromMap({
                ...options.toMap(),
                'bridgeKey': 'com.volcengine.rtc.hybrid_runtime',
              }));

  /// @brief 渲染模式，参看 ByteRTCRenderMode{@link #ByteRTCRenderMode}
  FutureOr<ByteRTCRenderMode?> get renderMode async {
    try {
      final result = await sendInstanceGet<ByteRTCRenderMode?>("renderMode");
      if (result == null) {
        return null;
      }
      return ByteRTCRenderMode.values
          .firstWhere((element) => element == result);
    } catch (e) {
      return null;
    }
  }

  set renderMode(FutureOr<ByteRTCRenderMode?> value) {
    sendInstanceSet("renderMode", value);
  }

  /// @brief 用于填充画布空白部分的背景颜色。取值范围是 `[0x00000000, 0xFFFFFFFF]`,格式为 BGR。默认值是 `0x00000000`。其中，透明度设置无效。
  FutureOr<NSInteger?> get backgroundColor async {
    return await sendInstanceGet<NSInteger?>("backgroundColor");
  }

  set backgroundColor(FutureOr<NSInteger?> value) {
    sendInstanceSet("backgroundColor", value);
  }

  /// @brief 视频帧旋转角度。参看 ByteRTCVideoRotation{@link #ByteRTCVideoRotation}。默认为 0 度，即不做旋转处理。
  FutureOr<ByteRTCVideoRotation?> get renderRotation async {
    try {
      final result =
          await sendInstanceGet<ByteRTCVideoRotation?>("renderRotation");
      if (result == null) {
        return null;
      }
      return ByteRTCVideoRotation.values
          .firstWhere((element) => element == result);
    } catch (e) {
      return null;
    }
  }

  set renderRotation(FutureOr<ByteRTCVideoRotation?> value) {
    sendInstanceSet("renderRotation", value);
  }
}

enum ByteRTCVideoCodecType {
  /// @brief 未知类型
  ///
  ByteRTCVideoCodecTypeUnknown(0),

  /// @brief 标准 H264 编码格式
  ///
  ByteRTCVideoCodecTypeH264(1),

  /// @brief ByteVC1 编码器
  ///
  ByteRTCVideoCodecTypeByteVC1(2);

  final dynamic $value;
  const ByteRTCVideoCodecType([this.$value]);
}

enum ByteRTCVideoPictureType {
  /// @brief 未知类型
  ///
  ByteRTCVideoPictureTypeUnknown(0),

  /// @brief I 帧，关键帧，编解码不需要参考其他视频帧
  ///
  ByteRTCVideoPictureTypeI(1),

  /// @brief P 帧，向前参考帧，编解码需要参考前一帧视频帧
  ///
  ByteRTCVideoPictureTypeP(2),

  /// @brief B 帧，前后参考帧，编解码需要参考前后两帧视频帧
  ///
  ByteRTCVideoPictureTypeB(3);

  final dynamic $value;
  const ByteRTCVideoPictureType([this.$value]);
}

enum ByteRTCKTVPlayerErrorCode {
  /// @brief 成功。
  ///
  ByteRTCKTVPlayerErrorCodeOK(0),

  /// @brief 播放错误，请下载后播放。
  ///
  ByteRTCKTVPlayerErrorCodeFileNotExist(-3020),

  /// @brief 播放错误，请确认文件播放格式。
  ///
  ByteRTCKTVPlayerErrorCodeFileError(-3021),

  /// @brief 播放错误，未进入房间。
  ///
  ByteRTCKTVPlayerErrorCodeNotJoinRoom(-3022),

  /// @brief 参数错误。
  ///
  ByteRTCKTVPlayerErrorCodeParam(-3023),

  /// @brief 播放失败，找不到文件或文件打开失败。
  ///
  ByteRTCKTVPlayerErrorCodeStartError(-3024),

  /// @brief 混音 ID 异常。
  ///
  ByteRTCKTVPlayerErrorCodeMixIdError(-3025),

  /// @brief 设置播放位置出错。
  ///
  ByteRTCKTVPlayerErrorCodePositionError(-3026),

  /// @brief 音量参数不合法，可设置的取值范围为 [0,400]。
  ///
  ByteRTCKTVPlayerErrorCodeAudioVolumeError(-3027),

  /// @brief 不支持此混音类型。
  ///
  ByteRTCKTVPlayerErrorCodeTypeError(-3028),

  /// @brief 音调文件不合法。
  ///
  ByteRTCKTVPlayerErrorCodePitchError(-3029),

  /// @brief 音轨不合法。
  ///
  ByteRTCKTVPlayerErrorCodeAudioTrackError(-3030),

  /// @brief 混音启动中。
  ///
  ByteRTCKTVPlayerErrorCodeStartingError(-3031);

  final dynamic $value;
  const ByteRTCKTVPlayerErrorCode([this.$value]);
}

enum ByteRTCLocalVideoSinkPosition {
  /// @brief 采集后。
  ///
  ByteRTCLocalVideoSinkPositionAfterCapture(0),

  /// @brief （默认值）前处理后。
  ///
  ByteRTCLocalVideoSinkPositionAfterPreprocess(1);

  final dynamic $value;
  const ByteRTCLocalVideoSinkPosition([this.$value]);
}

enum ByteRTCRecordingErrorCode {
  /// @brief 录制正常
  ///
  ByteRTCRecordingErrorCodeOk(0),

  /// @brief 没有文件写权限
  ///
  ByteRTCRecordingErrorCodeNoPermission(-1),

  /// @brief 当前版本 SDK 不支持本地录制功能，请联系技术支持人员
  ///
  ByteRTCRecordingErrorCodeNotSupport(-2),

  /// @brief 其他异常
  ///
  ByteRTCRecordingErrorCodeOther(-3);

  final dynamic $value;
  const ByteRTCRecordingErrorCode([this.$value]);
}

enum ByteRTCAVSyncState {
  /// @brief 音视频开始同步
  ///
  ByteRTCAVSyncStateAVStreamSyncBegin(0),

  /// @brief 音视频同步过程中音频移除，但不影响当前的同步关系
  ///
  ByteRTCAVSyncStateAudioStreamRemove(1),

  /// @brief 音视频同步过程中视频移除，但不影响当前的同步关系
  ///
  ByteRTCAVSyncStateVideoStreamRemove(2),

  /// @hidden for internal use only
  /// @brief 订阅端设置同步
  ///
  ByteRTCAVSyncStateSetAVSyncStreamId(3);

  final dynamic $value;
  const ByteRTCAVSyncState([this.$value]);
}

enum ByteRTCChorusCacheSyncEvent {
  /// @brief 成功。
  ///
  ByteRTCChorusCacheSyncEventStartSuccess(0),

  /// @brief 失败。
  ///
  ByteRTCChorusCacheSyncEventStartFailed(1);

  final dynamic $value;
  const ByteRTCChorusCacheSyncEvent([this.$value]);
}

enum ByteRTCEncryptType {
  /// @brief 不使用内置加密。默认值。
  ///
  ByteRTCEncryptTypeCustomize(0),

  /// @brief AES-128-CBC 加密算法
  ///
  ByteRTCEncryptTypeAES128CBC(1),

  /// @brief AES-256-CBC 加密算法
  ///
  ByteRTCEncryptTypeAES256CBC(2),

  /// @brief AES-128-ECB 加密算法
  ///
  ByteRTCEncryptTypeAES128ECB(3),

  /// @brief AES-256-ECB 加密算法
  ///
  ByteRTCEncryptTypeAES256ECB(4);

  final dynamic $value;
  const ByteRTCEncryptType([this.$value]);
}

enum ByteRTCPauseResumControlMediaType {
  /// @brief 只控制音频，不影响视频
  ///
  ByteRTCControlMediaTypeAudio(0),

  /// @brief 只控制视频，不影响音频
  ///
  ByteRtcControlMediaTypeVideo(1),

  /// @brief 同时控制音频和视频
  ///
  ByteRtcControlMediaTypeAudioAndVideo(2);

  final dynamic $value;
  const ByteRTCPauseResumControlMediaType([this.$value]);
}

class ByteRTCReceiveRange extends NativeClass {
  static const _$namespace = r'ByteRTCReceiveRange';
  static get codegen_$namespace => _$namespace;

  ByteRTCReceiveRange([NativeClassOptions? options])
      : super(options == null
            ? const NativeClassOptions(
                [],
                className: _$namespace,
                instanceType: InstanceType.manual,
                bridgeKey: 'com.volcengine.rtc.hybrid_runtime',
              )
            : NativeClassOptions.fromMap({
                ...options.toMap(),
                'bridgeKey': 'com.volcengine.rtc.hybrid_runtime',
              }));

  /// @brief 能够接收语音、并且具有衰减效果的最小距离值，该值须 ≥ 0，但 ≤ max。 <br>
  ///        小于该值的范围内没有范围语音效果，即收听到的音频音量相同。
  FutureOr<int?> get min async {
    return await sendInstanceGet<int?>("min");
  }

  set min(FutureOr<int?> value) {
    sendInstanceSet("min", value);
  }

  /// @brief 能够收听语音的最大距离值，该值须 > 0 且 ≥ min。 <br>
  ///        当收听者和声源距离处于 [min, max) 之间时，收听到的音量根据距离呈衰减效果。 <br>
  ///        超出该值范围的音频将无法收听到。
  FutureOr<int?> get max async {
    return await sendInstanceGet<int?>("max");
  }

  set max(FutureOr<int?> value) {
    sendInstanceSet("max", value);
  }
}

enum ByteRTCMediaDeviceType {
  /// @brief 未知音频设备
  ///
  ByteRTCMediaDeviceTypeAudioUnknown(-1),

  /// @brief 音频渲染设备类型
  ///
  ByteRTCMediaDeviceTypeAudioRenderDevice(0),

  /// @brief 音频采集设备类型
  ///
  ByteRTCMediaDeviceTypeAudioCaptureDevice(1),

  ByteRTCMediaDeviceTypeVideoRenderDevice(2),

  /// @brief 视频采集设备类型
  ///
  ByteRTCMediaDeviceTypeVideoCaptureDevice(3),

  /// @brief 屏幕流视频设备
  ///
  ByteRTCMediaDeviceTypeScreenVideoCaptureDevice(4),

  /// @brief 屏幕流音频设备
  ///
  ByteRTCMediaDeviceTypeScreenAudioCaptureDevice(5);

  final dynamic $value;
  const ByteRTCMediaDeviceType([this.$value]);
}

enum ByteRTCMusicFilterType {
  /// @brief 不过滤。
  ///
  ByteRTCMusicFilterTypeNone(0),

  /// @brief 过滤没有歌词的歌曲。
  ///
  ByteRTCMusicFilterTypeWithoutLyric(1),

  /// @brief 过滤不支持打分的歌曲。
  ///
  ByteRTCMusicFilterTypeUnsupportedScore(2),

  /// @brief 过滤不支持伴唱切换的歌曲。
  ///
  ByteRTCMusicFilterTypeUnsupportedAccopmay(4),

  /// @brief 过滤没有高潮片段的歌曲。
  ///
  ByteRTCMusicFilterTypeUnsupportedClimx(8);

  final dynamic $value;
  const ByteRTCMusicFilterType([this.$value]);
}

enum ByteRTCStreamRemoveReason {
  /// @brief 远端用户停止发布流。
  ///
  ByteRTCStreamRemoveReasonUnpublish(0),

  /// @brief 远端用户发布流失败。
  ///
  ByteRTCStreamRemoveReasonPublishFailed(1),

  /// @brief 媒体服务器 10s 没收到客户端的媒体数据。
  ///
  ByteRTCStreamRemoveReasonKeepLiveFailed(2),

  /// @brief 远端用户断网。
  ///
  ByteRTCStreamRemoveReasonClientDisconnected(3),

  /// @brief 远端用户重新发布流。
  ///
  ByteRTCStreamRemoveReasonRepublish(4),

  /// @brief 其他原因。
  ///
  ByteRTCStreamRemoveReasonOther(5),

  /// @brief 远端用户 Token 发布权限过期。
  ///
  ByteRTCStreamRemoveReasonPublishPrivilegeExpired(6);

  final dynamic $value;
  const ByteRTCStreamRemoveReason([this.$value]);
}

enum ByteRTCRemoteAudioStateChangeReason {
  /// @brief 内部原因
  ///
  ByteRTCRemoteAudioStateChangeReasonInternal(0),

  /// @brief 网络阻塞
  ///
  ByteRTCRemoteAudioStateChangeReasonNetworkCongestion(1),

  /// @brief 网络恢复正常
  ///
  ByteRTCRemoteAudioStateChangeReasonNetworkRecovery(2),

  /// @brief 本地用户停止接收远端音频流
  ///
  ByteRTCRemoteAudioStateChangeReasonLocalMuted(3),

  /// @brief 本地用户恢复接收远端音频流
  ///
  ByteRTCRemoteAudioStateChangeReasonLocalUnmuted(4),

  /// @brief 远端用户停止发送音频流
  ///
  ByteRTCRemoteAudioStateChangeReasonRemoteMuted(5),

  /// @brief 远端用户恢复发送音频流
  ///
  ByteRTCRemoteAudioStateChangeReasonRemoteUnmuted(6),

  /// @brief 远端用户离开房间
  ///
  ByteRTCRemoteAudioStateChangeReasonRemoteOffline(7);

  final dynamic $value;
  const ByteRTCRemoteAudioStateChangeReason([this.$value]);
}

class ByteRTCAudioVolumeInfo extends NativeClass {
  static const _$namespace = r'ByteRTCAudioVolumeInfo';
  static get codegen_$namespace => _$namespace;

  ByteRTCAudioVolumeInfo([NativeClassOptions? options])
      : super(options == null
            ? const NativeClassOptions(
                [],
                className: _$namespace,
                instanceType: InstanceType.manual,
                bridgeKey: 'com.volcengine.rtc.hybrid_runtime',
              )
            : NativeClassOptions.fromMap({
                ...options.toMap(),
                'bridgeKey': 'com.volcengine.rtc.hybrid_runtime',
              }));

  /// @brief 音频流来源的用户 ID
  FutureOr<NSString?> get uid async {
    return await sendInstanceGet<NSString?>("uid");
  }

  set uid(FutureOr<NSString?> value) {
    sendInstanceSet("uid", value);
  }

  /// @brief 线性音量，取值范围为：[0,255]
  FutureOr<NSUInteger?> get linearVolume async {
    return await sendInstanceGet<NSUInteger?>("linearVolume");
  }

  set linearVolume(FutureOr<NSUInteger?> value) {
    sendInstanceSet("linearVolume", value);
  }

  /// @brief 非线性音量，取值范围为：[-127,0]
  FutureOr<NSUInteger?> get nonlinearVolume async {
    return await sendInstanceGet<NSUInteger?>("nonlinearVolume");
  }

  set nonlinearVolume(FutureOr<NSUInteger?> value) {
    sendInstanceSet("nonlinearVolume", value);
  }
}

class DestInfo extends NativeClass {
  static const _$namespace = r'DestInfo';
  static get codegen_$namespace => _$namespace;

  DestInfo([NativeClassOptions? options])
      : super(options == null
            ? const NativeClassOptions(
                [],
                className: _$namespace,
                instanceType: InstanceType.manual,
                bridgeKey: 'com.volcengine.rtc.hybrid_runtime',
              )
            : NativeClassOptions.fromMap({
                ...options.toMap(),
                'bridgeKey': 'com.volcengine.rtc.hybrid_runtime',
              }));

  /// @brief 目标房间ID
  ///
  FutureOr<NSString?> get roomId async {
    return await sendInstanceGet<NSString?>("roomId");
  }

  set roomId(FutureOr<NSString?> value) {
    sendInstanceSet("roomId", value);
  }

  /// @brief 目标房间中的用户ID
  ///
  FutureOr<NSString?> get userId async {
    return await sendInstanceGet<NSString?>("userId");
  }

  set userId(FutureOr<NSString?> value) {
    sendInstanceSet("userId", value);
  }
}

class ByteRTCAudioRecordingConfig extends NativeClass {
  static const _$namespace = r'ByteRTCAudioRecordingConfig';
  static get codegen_$namespace => _$namespace;

  ByteRTCAudioRecordingConfig([NativeClassOptions? options])
      : super(options == null
            ? const NativeClassOptions(
                [],
                className: _$namespace,
                instanceType: InstanceType.manual,
                bridgeKey: 'com.volcengine.rtc.hybrid_runtime',
              )
            : NativeClassOptions.fromMap({
                ...options.toMap(),
                'bridgeKey': 'com.volcengine.rtc.hybrid_runtime',
              }));

  /// @brief 录制文件路径。一个有读写权限的绝对路径，包含文件名和文件后缀。
  /// @note 录制文件的格式仅支持 .aac 和 .wav。
  FutureOr<NSString?> get absoluteFileName async {
    return await sendInstanceGet<NSString?>("absoluteFileName");
  }

  set absoluteFileName(FutureOr<NSString?> value) {
    sendInstanceSet("absoluteFileName", value);
  }

  /// @brief 录音内容来源，参看 ByteRTCAudioFrameSource{@link #ByteRTCAudioFrameSource}。 <br>
  /// 默认为 ByteRTCAudioFrameSourceTypeMixed = 2。
  FutureOr<ByteRTCAudioFrameSource?> get frameSource async {
    try {
      final result =
          await sendInstanceGet<ByteRTCAudioFrameSource?>("frameSource");
      if (result == null) {
        return null;
      }
      return ByteRTCAudioFrameSource.values
          .firstWhere((element) => element == result);
    } catch (e) {
      return null;
    }
  }

  set frameSource(FutureOr<ByteRTCAudioFrameSource?> value) {
    sendInstanceSet("frameSource", value);
  }

  /// @brief 录音采样率。参看 ByteRTCAudioSampleRate{@link #ByteRTCAudioSampleRate}。
  FutureOr<ByteRTCAudioSampleRate?> get sampleRate async {
    try {
      final result =
          await sendInstanceGet<ByteRTCAudioSampleRate?>("sampleRate");
      if (result == null) {
        return null;
      }
      return ByteRTCAudioSampleRate.values
          .firstWhere((element) => element == result);
    } catch (e) {
      return null;
    }
  }

  set sampleRate(FutureOr<ByteRTCAudioSampleRate?> value) {
    sendInstanceSet("sampleRate", value);
  }

  /// @brief 录音音频声道。参看 ByteRTCAudioChannel{@link #ByteRTCAudioChannel}。 <br>
  ///       如果录制时设置的的音频声道数与采集时的音频声道数不同： <br>
  ///        - 如果采集的声道数为 1，录制的声道数为 2，那么，录制的音频为经过单声道数据拷贝后的双声道数据，而不是立体声。
  ///        - 如果采集的声道数为 2，录制的声道数为 1，那么，录制的音频为经过双声道数据混合后的单声道数据。
  FutureOr<ByteRTCAudioChannel?> get channel async {
    try {
      final result = await sendInstanceGet<ByteRTCAudioChannel?>("channel");
      if (result == null) {
        return null;
      }
      return ByteRTCAudioChannel.values
          .firstWhere((element) => element == result);
    } catch (e) {
      return null;
    }
  }

  set channel(FutureOr<ByteRTCAudioChannel?> value) {
    sendInstanceSet("channel", value);
  }

  /// @brief 录音音质。仅在录制文件格式为 .aac 时可以设置。参看 ByteRTCAudioQuality{@link #ByteRTCAudioQuality}。 <br>
  /// 采样率为 32kHz 时，不同音质录制文件（时长为 10min）的大小分别是： <br>
  ///        - 低音质：1.2MB；
  ///        - 【默认】中音质：2MB；
  ///        - 高音质：3.75MB；
  ///        - 超高音质：7.5MB。
  FutureOr<ByteRTCAudioQuality?> get quality async {
    try {
      final result = await sendInstanceGet<ByteRTCAudioQuality?>("quality");
      if (result == null) {
        return null;
      }
      return ByteRTCAudioQuality.values
          .firstWhere((element) => element == result);
    } catch (e) {
      return null;
    }
  }

  set quality(FutureOr<ByteRTCAudioQuality?> value) {
    sendInstanceSet("quality", value);
  }
}

class ByteRTCVideoFrameInfo extends NativeClass {
  static const _$namespace = r'ByteRTCVideoFrameInfo';
  static get codegen_$namespace => _$namespace;

  ByteRTCVideoFrameInfo([NativeClassOptions? options])
      : super(options == null
            ? const NativeClassOptions(
                [],
                className: _$namespace,
                instanceType: InstanceType.manual,
                bridgeKey: 'com.volcengine.rtc.hybrid_runtime',
              )
            : NativeClassOptions.fromMap({
                ...options.toMap(),
                'bridgeKey': 'com.volcengine.rtc.hybrid_runtime',
              }));

  /// @brief 视频帧的宽度（像素）
  FutureOr<NSInteger?> get width async {
    return await sendInstanceGet<NSInteger?>("width");
  }

  set width(FutureOr<NSInteger?> value) {
    sendInstanceSet("width", value);
  }

  /// @brief 视频帧的高度（像素）
  FutureOr<NSInteger?> get height async {
    return await sendInstanceGet<NSInteger?>("height");
  }

  set height(FutureOr<NSInteger?> value) {
    sendInstanceSet("height", value);
  }

  /// @brief 视频帧顺时针旋转角度。参看 ByteRTCVideoRotation{@link #ByteRTCVideoRotation}。
  FutureOr<ByteRTCVideoRotation?> get rotation async {
    try {
      final result = await sendInstanceGet<ByteRTCVideoRotation?>("rotation");
      if (result == null) {
        return null;
      }
      return ByteRTCVideoRotation.values
          .firstWhere((element) => element == result);
    } catch (e) {
      return null;
    }
  }

  set rotation(FutureOr<ByteRTCVideoRotation?> value) {
    sendInstanceSet("rotation", value);
  }
}

enum ByteRTCAudioDeviceType {
  /// @brief 未知音频设备
  ///
  ByteRTCAudioDeviceTypeUnknown(-1),

  /// @brief 音频渲染设备
  ///
  ByteRTCAudioDeviceTypeRenderDevice(0),

  /// @brief 音频采集设备
  ///
  ByteRTCAudioDeviceTypeCaptureDevice(1),

  /// @brief 屏幕流音频设备
  ///
  ByteRTCAudioDeviceTypeScreenCaptureDevice(2);

  final dynamic $value;
  const ByteRTCAudioDeviceType([this.$value]);
}

enum ByteRTCAudioRecordingState {
  /// @brief 录制异常
  ///
  ByteRTCAudioRecordingStateError(0),

  /// @brief 录制进行中
  ///
  ByteRTCAudioRecordingStateProcessing(1),

  /// @brief 已结束录制，并且录制文件保存成功。
  ///
  ByteRTCAudioRecordingStateSuccess(2);

  final dynamic $value;
  const ByteRTCAudioRecordingState([this.$value]);
}

class ByteRTCScreenCaptureSourceInfo extends NativeClass {
  static const _$namespace = r'ByteRTCScreenCaptureSourceInfo';
  static get codegen_$namespace => _$namespace;

  ByteRTCScreenCaptureSourceInfo([NativeClassOptions? options])
      : super(options == null
            ? const NativeClassOptions(
                [],
                className: _$namespace,
                instanceType: InstanceType.manual,
                bridgeKey: 'com.volcengine.rtc.hybrid_runtime',
              )
            : NativeClassOptions.fromMap({
                ...options.toMap(),
                'bridgeKey': 'com.volcengine.rtc.hybrid_runtime',
              }));

  /// @brief 屏幕分享时，共享对象的类型，参看 ByteRTCScreenCaptureSourceType{@link #ByteRTCScreenCaptureSourceType}
  FutureOr<ByteRTCScreenCaptureSourceType?> get sourceType async {
    try {
      final result =
          await sendInstanceGet<ByteRTCScreenCaptureSourceType?>("sourceType");
      if (result == null) {
        return null;
      }
      return ByteRTCScreenCaptureSourceType.values
          .firstWhere((element) => element == result);
    } catch (e) {
      return null;
    }
  }

  set sourceType(FutureOr<ByteRTCScreenCaptureSourceType?> value) {
    sendInstanceSet("sourceType", value);
  }

  /// @brief 屏幕分享时，共享对象的 ID。
  FutureOr<intptr_t?> get sourceId async {
    return await sendInstanceGet<intptr_t?>("sourceId");
  }

  set sourceId(FutureOr<intptr_t?> value) {
    sendInstanceSet("sourceId", value);
  }

  /// @brief 屏幕分享时共享对象的名称。
  FutureOr<NSString?> get sourceName async {
    return await sendInstanceGet<NSString?>("sourceName");
  }

  set sourceName(FutureOr<NSString?> value) {
    sendInstanceSet("sourceName", value);
  }

  /// @brief 共享的应用窗体所属应用的名称 <br>
  ///        当共享对象为应用窗体时有效
  FutureOr<NSString?> get application async {
    return await sendInstanceGet<NSString?>("application");
  }

  set application(FutureOr<NSString?> value) {
    sendInstanceSet("application", value);
  }

  /// @brief 共享的应用窗体所属应用进程的 pid <br>
  ///        当共享对象为应用窗体时有效
  FutureOr<int?> get pid async {
    return await sendInstanceGet<int?>("pid");
  }

  set pid(FutureOr<int?> value) {
    sendInstanceSet("pid", value);
  }

  /// @brief 共享的屏幕是否为主屏。 <br>
  ///        当共享对象为屏幕时有效
  FutureOr<BOOL?> get primaryMonitor async {
    return await sendInstanceGet<BOOL?>("primaryMonitor");
  }

  set primaryMonitor(FutureOr<BOOL?> value) {
    sendInstanceSet("primaryMonitor", value);
  }

  /// @brief 屏幕共享对象的坐标。多显示器的场景下，屏幕坐标系统以主屏左上角为原点 (0, 0)，向右向下扩展。
  FutureOr<CGRect?> get regionRect async {
    return await sendInstanceGet<CGRect?>("regionRect");
  }

  set regionRect(FutureOr<CGRect?> value) {
    sendInstanceSet("regionRect", value);
  }
}

enum ByteRTCVideoCodecMode {
  /// @brief 自动选择
  ///
  ByteRTCVideoCodecModeAuto(0),

  /// @brief 硬编码
  ///
  ByteRTCVideoCodecModeHardware(1),

  /// @brief 软编码
  ///
  ByteRTCVideoCodecModeSoftware(2);

  final dynamic $value;
  const ByteRTCVideoCodecMode([this.$value]);
}

class ByteRTCAudioPropertiesInfo extends NativeClass {
  static const _$namespace = r'ByteRTCAudioPropertiesInfo';
  static get codegen_$namespace => _$namespace;

  ByteRTCAudioPropertiesInfo([NativeClassOptions? options])
      : super(options == null
            ? const NativeClassOptions(
                [],
                className: _$namespace,
                instanceType: InstanceType.manual,
                bridgeKey: 'com.volcengine.rtc.hybrid_runtime',
              )
            : NativeClassOptions.fromMap({
                ...options.toMap(),
                'bridgeKey': 'com.volcengine.rtc.hybrid_runtime',
              }));

  /// @brief 线性音量，与原始音量呈线性关系，数值越大，音量越大。取值范围是：[0,255]。 <br>
  ///        - [0, 25]: 无声
  ///        - [26, 75]: 低音量
  ///        - [76, 204]: 中音量
  ///        - [205, 255]: 高音量
  FutureOr<NSInteger?> get linearVolume async {
    return await sendInstanceGet<NSInteger?>("linearVolume");
  }

  set linearVolume(FutureOr<NSInteger?> value) {
    sendInstanceSet("linearVolume", value);
  }

  /// @brief 非线性音量。由原始音量的对数值转化而来，因此在中低音量时更灵敏，可以用作 Active Speaker（房间内最活跃用户）的识别。取值范围是：[-127，0]，单位 dB。 <br>
  ///        - [-127, -60]: 无声
  ///        - [-59, -40]: 低音量
  ///        - [-39, -20]: 中音量
  ///        - [-19, 0]: 高音量
  FutureOr<NSInteger?> get nonlinearVolume async {
    return await sendInstanceGet<NSInteger?>("nonlinearVolume");
  }

  set nonlinearVolume(FutureOr<NSInteger?> value) {
    sendInstanceSet("nonlinearVolume", value);
  }

  /// @brief 人声检测（VAD）结果 <br>
  ///        - 1: 检测到人声。
  ///        - 0: 未检测到人声。
  ///        - -1: 未开启 VAD。
  FutureOr<NSInteger?> get vad async {
    return await sendInstanceGet<NSInteger?>("vad");
  }

  set vad(FutureOr<NSInteger?> value) {
    sendInstanceSet("vad", value);
  }

  /// @brief 频谱数组。默认长度为 257。
  FutureOr<NSArray<NSNumber>?> get spectrum async {
    return await sendInstanceGet<NSArray<NSNumber>?>("spectrum");
  }

  set spectrum(FutureOr<NSArray<NSNumber>?> value) {
    sendInstanceSet("spectrum", value);
  }

  /// @brief 本地用户的人声基频，单位为赫兹。 <br>
  ///        同时满足以下两个条件时，返回的值为本地用户的人声基频： <br>
  ///      - 调用 enableAudioPropertiesReport:{@link #ByteRTCEngine#enableAudioPropertiesReport}，并设置参数 enableVoicePitch 的值为 `true`。
  ///      - 本地采集的音频中包含本地用户的人声。
  ///        其他情况下返回 `0`。
  FutureOr<NSInteger?> get voicePitch async {
    return await sendInstanceGet<NSInteger?>("voicePitch");
  }

  set voicePitch(FutureOr<NSInteger?> value) {
    sendInstanceSet("voicePitch", value);
  }
}

class ByteRTCHighlightConfig extends NativeClass {
  static const _$namespace = r'ByteRTCHighlightConfig';
  static get codegen_$namespace => _$namespace;

  ByteRTCHighlightConfig([NativeClassOptions? options])
      : super(options == null
            ? const NativeClassOptions(
                [],
                className: _$namespace,
                instanceType: InstanceType.manual,
                bridgeKey: 'com.volcengine.rtc.hybrid_runtime',
              )
            : NativeClassOptions.fromMap({
                ...options.toMap(),
                'bridgeKey': 'com.volcengine.rtc.hybrid_runtime',
              }));

  /// @brief 是否显示高亮边框，默认显示。
  FutureOr<BOOL?> get enableHighlight async {
    return await sendInstanceGet<BOOL?>("enableHighlight");
  }

  set enableHighlight(FutureOr<BOOL?> value) {
    sendInstanceSet("enableHighlight", value);
  }

  /// @brief 边框的颜色, 颜色格式为十六进制 ARGB: 0xAARRGGB。
  FutureOr<uint32_t?> get borderColor async {
    return await sendInstanceGet<uint32_t?>("borderColor");
  }

  set borderColor(FutureOr<uint32_t?> value) {
    sendInstanceSet("borderColor", value);
  }

  /// @brief 边框的宽度，单位：像素。
  FutureOr<int?> get borderWidth async {
    return await sendInstanceGet<int?>("borderWidth");
  }

  set borderWidth(FutureOr<int?> value) {
    sendInstanceSet("borderWidth", value);
  }
}

enum ByteRTCAudioDumpStatus {
  /// @brief 音频 dump 启动失败
  ///
  ByteRTCAudioDumpStartFailure(0),

  /// @brief 音频 dump 启动成功
  ///
  ByteRTCAudioDumpStartSuccess(1),

  /// @brief 音频 dump 停止失败
  ///
  ByteRTCAudioDumpStopFailure(2),

  /// @brief 音频 dump 停止成功
  ///
  ByteRTCAudioDumpStopSuccess(3),

  /// @brief 音频 dump 运行失败
  ///
  ByteRTCAudioDumpRunningFailure(4),

  /// @brief 音频 dump 运行成功
  ///
  ByteRTCAudioDumpRunningSuccess(5);

  final dynamic $value;
  const ByteRTCAudioDumpStatus([this.$value]);
}

enum ByteRTCLogoutReason {
  /// @brief 用户主动退出 <br>
  ///        用户调用 `logout` 接口登出，或者销毁引擎登出。
  ///
  ByteRTCLogoutReasonLogout(0),

  /// @brief 用户被动退出 <br>
  ///        另一个用户以相同 UserId 进行了 `login`，导致本端用户被踢出。
  ///
  ByteRTCLogoutReasonDuplicateLogin(1);

  final dynamic $value;
  const ByteRTCLogoutReason([this.$value]);
}

class ByteRTCExpressionDetectConfig extends NativeClass {
  static const _$namespace = r'ByteRTCExpressionDetectConfig';
  static get codegen_$namespace => _$namespace;

  ByteRTCExpressionDetectConfig([NativeClassOptions? options])
      : super(options == null
            ? const NativeClassOptions(
                [],
                className: _$namespace,
                instanceType: InstanceType.manual,
                bridgeKey: 'com.volcengine.rtc.hybrid_runtime',
              )
            : NativeClassOptions.fromMap({
                ...options.toMap(),
                'bridgeKey': 'com.volcengine.rtc.hybrid_runtime',
              }));

  /// @brief 是否开启年龄检测。
  FutureOr<BOOL?> get enableAgeDetect async {
    return await sendInstanceGet<BOOL?>("enableAgeDetect");
  }

  set enableAgeDetect(FutureOr<BOOL?> value) {
    sendInstanceSet("enableAgeDetect", value);
  }

  /// @brief 是否开启性别检测。
  FutureOr<BOOL?> get enableGenderDetect async {
    return await sendInstanceGet<BOOL?>("enableGenderDetect");
  }

  set enableGenderDetect(FutureOr<BOOL?> value) {
    sendInstanceSet("enableGenderDetect", value);
  }

  /// @brief 是否开启表情检测。
  FutureOr<BOOL?> get enableEmotionDetect async {
    return await sendInstanceGet<BOOL?>("enableEmotionDetect");
  }

  set enableEmotionDetect(FutureOr<BOOL?> value) {
    sendInstanceSet("enableEmotionDetect", value);
  }

  /// @brief 是否开启吸引力检测。
  FutureOr<BOOL?> get enableAttractivenessDetect async {
    return await sendInstanceGet<BOOL?>("enableAttractivenessDetect");
  }

  set enableAttractivenessDetect(FutureOr<BOOL?> value) {
    sendInstanceSet("enableAttractivenessDetect", value);
  }

  /// @brief 是否开启开心程度检测。
  FutureOr<BOOL?> get enableHappinessDetect async {
    return await sendInstanceGet<BOOL?>("enableHappinessDetect");
  }

  set enableHappinessDetect(FutureOr<BOOL?> value) {
    sendInstanceSet("enableHappinessDetect", value);
  }
}

enum ByteRTCAnsMode {
  /// @brief 关闭所有音频降噪能力。
  ///
  ByteRTCAnsModeDisable(0),

  /// @brief 适用于微弱降噪。
  ///
  ByteRTCAnsModeLow(1),

  /// @brief 适用于抑制中度平稳噪声，如空调声和风扇声。
  ///
  ByteRTCAnsModeMedium(2),

  /// @brief 适用于抑制嘈杂非平稳噪声，如键盘声、敲击声、碰撞声、动物叫声。
  ///
  ByteRTCAnsModeHigh(3),

  /// @brief 启用音频降噪能力。具体的降噪算法由 RTC 智能决策。
  ///
  ByteRTCAnsModeAutomatic(4);

  final dynamic $value;
  const ByteRTCAnsMode([this.$value]);
}

class ByteRTCSourceWantedData extends NativeClass {
  static const _$namespace = r'ByteRTCSourceWantedData';
  static get codegen_$namespace => _$namespace;

  ByteRTCSourceWantedData([NativeClassOptions? options])
      : super(options == null
            ? const NativeClassOptions(
                [],
                className: _$namespace,
                instanceType: InstanceType.manual,
                bridgeKey: 'com.volcengine.rtc.hybrid_runtime',
              )
            : NativeClassOptions.fromMap({
                ...options.toMap(),
                'bridgeKey': 'com.volcengine.rtc.hybrid_runtime',
              }));

  /// @brief 推荐视频输入宽
  FutureOr<NSInteger?> get width async {
    return await sendInstanceGet<NSInteger?>("width");
  }

  set width(FutureOr<NSInteger?> value) {
    sendInstanceSet("width", value);
  }

  /// @brief 推荐视频输入高
  FutureOr<NSInteger?> get height async {
    return await sendInstanceGet<NSInteger?>("height");
  }

  set height(FutureOr<NSInteger?> value) {
    sendInstanceSet("height", value);
  }

  /// @brief 推荐视频输入帧率
  FutureOr<NSInteger?> get frameRate async {
    return await sendInstanceGet<NSInteger?>("frameRate");
  }

  set frameRate(FutureOr<NSInteger?> value) {
    sendInstanceSet("frameRate", value);
  }
}

class ByteRTCForwardStreamStateInfo extends NativeClass {
  static const _$namespace = r'ByteRTCForwardStreamStateInfo';
  static get codegen_$namespace => _$namespace;

  ByteRTCForwardStreamStateInfo([NativeClassOptions? options])
      : super(options == null
            ? const NativeClassOptions(
                [],
                className: _$namespace,
                instanceType: InstanceType.manual,
                bridgeKey: 'com.volcengine.rtc.hybrid_runtime',
              )
            : NativeClassOptions.fromMap({
                ...options.toMap(),
                'bridgeKey': 'com.volcengine.rtc.hybrid_runtime',
              }));

  /// @brief 跨房间转发媒体流过程中目标房间 ID <br>
  ///        空字符串代表所有目标房间
  ///
  FutureOr<NSString?> get roomId async {
    return await sendInstanceGet<NSString?>("roomId");
  }

  set roomId(FutureOr<NSString?> value) {
    sendInstanceSet("roomId", value);
  }

  /// @brief 跨房间转发媒体流过程中该目标房间的状态，参看 ByteRTCForwardStreamState{@link #ByteRTCForwardStreamState}
  ///
  FutureOr<ByteRTCForwardStreamState?> get state async {
    try {
      final result = await sendInstanceGet<ByteRTCForwardStreamState?>("state");
      if (result == null) {
        return null;
      }
      return ByteRTCForwardStreamState.values
          .firstWhere((element) => element == result);
    } catch (e) {
      return null;
    }
  }

  set state(FutureOr<ByteRTCForwardStreamState?> value) {
    sendInstanceSet("state", value);
  }

  /// @brief 跨房间转发媒体流过程中该目标房间抛出的错误码，参看 ByteRTCForwardStreamError{@link #ByteRTCForwardStreamError}
  ///
  FutureOr<ByteRTCForwardStreamError?> get error async {
    try {
      final result = await sendInstanceGet<ByteRTCForwardStreamError?>("error");
      if (result == null) {
        return null;
      }
      return ByteRTCForwardStreamError.values
          .firstWhere((element) => element == result);
    } catch (e) {
      return null;
    }
  }

  set error(FutureOr<ByteRTCForwardStreamError?> value) {
    sendInstanceSet("error", value);
  }
}

enum ByteRTCVideoStreamState {
  /// @brief 设置本地视频属性成功
  ///
  ByteRTCVideoStreamStateSuccess(0),

  /// @brief 本地视频属性参数不合法
  ///
  ByteRTCVideoStreamStateInvalid(-2);

  final dynamic $value;
  const ByteRTCVideoStreamState([this.$value]);
}

enum ByteRTCAudioScenarioType {
  /// @brief 默认场景，适用大部分场景。
  ///
  ByteRTCAudioScenarioTypeDefault(0),

  /// @brief 聊天室场景。通话清晰度较高，适用于会议，聊天室场景。
  ///
  ByteRTCAudioScenarioTypeChatRoom(1),

  /// @brief 游戏语音场景。
  ///
  ByteRTCAudioScenarioTypeGameStreaming(2),

  /// @brief 合唱场景。延迟较低。
  ///
  ByteRTCAudioScenarioTypeChorus(3),

  /// @brief 教育场景。适用于以人声教学内容为主的高音质场景。
  ///
  ByteRTCAudioScenarioTypeEducation(4),

  /// @brief AI 对话场景。适用于真人与 AI 智能体互动的场景。
  ///
  ByteRTCAudioScenarioTypeAiClient(5);

  final dynamic $value;
  const ByteRTCAudioScenarioType([this.$value]);
}

enum ByteRTCForwardStreamEvent {
  /// @brief 本端与服务器网络连接断开，暂停转发。
  ///
  ByteRTCForwardStreamEventDisconnected(0),

  /// @brief 本端与服务器网络连接恢复，转发服务连接成功。
  ///
  ByteRTCForwardStreamEventConnected(1),

  /// @brief 转发中断。转发过程中，如果相同 user_id 的用户进入目标房间，转发中断。
  ///
  ByteRTCForwardStreamEventInterrupt(2),

  /// @brief 目标房间已更新，由 `updateForwardStreamToRooms` 触发。
  ///
  ByteRTCForwardStreamEventDstRoomUpdated(3),

  /// @brief API 调用时序错误。例如，在调用 `startForwardStreamToRooms` 之前调用 `updateForwardStreamToRooms` 。
  ///
  ByteRTCForwardStreamEventUnExpectAPICall(4);

  final dynamic $value;
  const ByteRTCForwardStreamEvent([this.$value]);
}

class ByteRTCCloudProxyInfo extends NativeClass {
  static const _$namespace = r'ByteRTCCloudProxyInfo';
  static get codegen_$namespace => _$namespace;

  ByteRTCCloudProxyInfo([NativeClassOptions? options])
      : super(options == null
            ? const NativeClassOptions(
                [],
                className: _$namespace,
                instanceType: InstanceType.manual,
                bridgeKey: 'com.volcengine.rtc.hybrid_runtime',
              )
            : NativeClassOptions.fromMap({
                ...options.toMap(),
                'bridgeKey': 'com.volcengine.rtc.hybrid_runtime',
              }));

  /// @detail keytype
  /// @brief 云代理服务器 IP
  FutureOr<NSString?> get cloudProxyIp async {
    return await sendInstanceGet<NSString?>("cloudProxyIp");
  }

  set cloudProxyIp(FutureOr<NSString?> value) {
    sendInstanceSet("cloudProxyIp", value);
  }

  /// @detail keytype
  /// @brief 云代理服务器端口
  FutureOr<int?> get cloudProxyPort async {
    return await sendInstanceGet<int?>("cloudProxyPort");
  }

  set cloudProxyPort(FutureOr<int?> value) {
    sendInstanceSet("cloudProxyPort", value);
  }
}

class ByteRtcScreenCapturerExt extends NativeClass {
  static const _$namespace = r'ByteRtcScreenCapturerExt';
  static get codegen_$namespace => _$namespace;

  ByteRtcScreenCapturerExt([NativeClassOptions? options])
      : super(options == null
            ? const NativeClassOptions(
                [],
                className: _$namespace,
                instanceType: InstanceType.manual,
                bridgeKey: 'com.volcengine.rtc.hybrid_runtime',
              )
            : NativeClassOptions.fromMap({
                ...options.toMap(),
                'bridgeKey': 'com.volcengine.rtc.hybrid_runtime',
              }));

  /// @detail keytype
  /// @brief 只读变量，用于获取 ByteRtcScreenCapturerExt 实例。

  FutureOr<ByteRtcScreenCapturerExt?> get shared async {
    try {
      final result = await sendInstanceGet<ByteRtcScreenCapturerExt?>("shared");
      if (result == null) {
        return null;
      }
      return packObject(
          result,
          () => ByteRtcScreenCapturerExt(
              const NativeClassOptions([], disableInit: true)));
    } catch (e) {
      return null;
    }
  }

  set shared(FutureOr<ByteRtcScreenCapturerExt?> value) {
    sendInstanceSet("shared", value);
  }

  /// @detail keytype
  /// @brief ByteRtcScreenCapturerExt 实例的回调代理。

  FutureOr<NSObject<ByteRtcScreenCapturerExtDelegate>?> get delegate async {
    try {
      final result =
          await sendInstanceGet<NSObject<ByteRtcScreenCapturerExtDelegate>?>(
              "delegate");
      if (result == null) {
        return null;
      }
      return packObject(
          result,
          () => ByteRtcScreenCapturerExtDelegate(
              const NativeClassOptions([], disableInit: true)));
    } catch (e) {
      return null;
    }
  }

  set delegate(FutureOr<NSObject<ByteRtcScreenCapturerExtDelegate>?> value) {
    sendInstanceSet("delegate", value);
  }

  /// @detail api
  /// @brief 开始屏幕采集 <br>
  ///        Extension 启动后，系统将自动调用该方法开启屏幕采集。
  /// @param delegate 回调代理，参看 ByteRtcScreenCapturerExtDelegate{@link #ByteRtcScreenCapturerExtDelegate}
  /// @param groupId App groups 中配置的 group ID

  FutureOr<void> startWithDelegate(
      NSObject<ByteRtcScreenCapturerExtDelegate> delegate,
      NSString groupId) async {
    return await nativeCall('startWithDelegate:groupId:', [delegate, groupId]);
  }

  /// @detail api
  /// @brief 结束屏幕采集 <br>
  ///        Extension 关闭后，系统将自动调用该方法停止屏幕采集。

  FutureOr<void> stop() async {
    return await nativeCall('stop', []);
  }

  /// @detail api
  /// @brief 推送 Extension 采集的数据
  /// @param sampleBuffer 采集到的数据
  /// @param sampleBufferType 数据类型

  FutureOr<void> processSampleBuffer(CMSampleBufferRef sampleBuffer,
      RPSampleBufferType sampleBufferType) async {
    return await nativeCall(
        'processSampleBuffer:withType:', [sampleBuffer, sampleBufferType]);
  }
}

enum ByteRTCVideoOrientation {
  /// @brief （默认）使用相机输出的原始视频帧的角度，不对视频帧进行额外旋转。
  ///
  ByteRTCVideoOrientationAdaptive(0),

  /// @brief 固定为竖屏，将相机采集到的视频帧转换为竖屏，在整个 RTC 链路中传递竖屏帧。
  ///
  ByteRTCVideoOrientationPortrait(1),

  /// @brief 固定为横屏，将相机采集到的视频帧转换为横屏，在整个 RTC 链路中传递横屏帧。
  ///
  ByteRTCVideoOrientationLandscape(2);

  final dynamic $value;
  const ByteRTCVideoOrientation([this.$value]);
}

enum ByteRTCBluetoothMode {
  /// @brief 默认采用 auto 模式，具体如下： <br>
  /// <table>
  /// <tr>
  ///   <th>场景</th>
  ///   <th> HFP </th>
  ///   <th> A2DP </th>
  /// </tr>
  /// <tr>
  ///   <th>纯通话场景</th>
  ///   <th> 蓝牙设备支持 HFP </th>
  ///   <th> 蓝牙设备不支持 HFP </th>
  /// </tr>
  /// <tr>
  ///   <th>纯媒体场景</th>
  ///   <th> 使用蓝牙设备采集播放音频 </th>
  ///   <th> 使用 iOS 设备采集音频，蓝牙设备播放音频 </th>
  /// </tr>
  /// </table>
  ///
  auto(0),

  /// @brief 高级音频分配配置文件（A2DP）。立体声、高音质。采用 iOS 设备进行音频采集，蓝牙设备进行播放。
  ///
  a2dp(1),

  /// @brief 免提配置文件（HFP）。单声道、普通音质。音频采集和播放设备都使用蓝牙设备。
  ///
  hfp(2);

  final dynamic $value;
  const ByteRTCBluetoothMode([this.$value]);
}

class ByteRTCNetworkTimeInfo extends NativeClass {
  static const _$namespace = r'ByteRTCNetworkTimeInfo';
  static get codegen_$namespace => _$namespace;

  ByteRTCNetworkTimeInfo([NativeClassOptions? options])
      : super(options == null
            ? const NativeClassOptions(
                [],
                className: _$namespace,
                instanceType: InstanceType.manual,
                bridgeKey: 'com.volcengine.rtc.hybrid_runtime',
              )
            : NativeClassOptions.fromMap({
                ...options.toMap(),
                'bridgeKey': 'com.volcengine.rtc.hybrid_runtime',
              }));

  /// @brief 网络时间，单位：ms
  FutureOr<int64_t?> get timestamp async {
    return await sendInstanceGet<int64_t?>("timestamp");
  }

  set timestamp(FutureOr<int64_t?> value) {
    sendInstanceSet("timestamp", value);
  }
}

class ByteRTCScreenParam extends NativeClass {
  static const _$namespace = r'ByteRTCScreenParam';
  static get codegen_$namespace => _$namespace;

  ByteRTCScreenParam([NativeClassOptions? options])
      : super(options == null
            ? const NativeClassOptions(
                [],
                className: _$namespace,
                instanceType: InstanceType.manual,
                bridgeKey: 'com.volcengine.rtc.hybrid_runtime',
              )
            : NativeClassOptions.fromMap({
                ...options.toMap(),
                'bridgeKey': 'com.volcengine.rtc.hybrid_runtime',
              }));

  /// @brief 编码帧率,单位为 fps
  FutureOr<NSInteger?> get frameRate async {
    return await sendInstanceGet<NSInteger?>("frameRate");
  }

  set frameRate(FutureOr<NSInteger?> value) {
    sendInstanceSet("frameRate", value);
  }

  /// @brief 编码码率，小于 0 时 SDK 会根据高宽自适应码率, 单位 kbps
  FutureOr<NSInteger?> get bitrate async {
    return await sendInstanceGet<NSInteger?>("bitrate");
  }

  set bitrate(FutureOr<NSInteger?> value) {
    sendInstanceSet("bitrate", value);
  }
}

class ByteRTCUserInfo extends NativeClass {
  static const _$namespace = r'ByteRTCUserInfo';
  static get codegen_$namespace => _$namespace;

  ByteRTCUserInfo([NativeClassOptions? options])
      : super(options == null
            ? const NativeClassOptions(
                [],
                className: _$namespace,
                instanceType: InstanceType.manual,
                bridgeKey: 'com.volcengine.rtc.hybrid_runtime',
              )
            : NativeClassOptions.fromMap({
                ...options.toMap(),
                'bridgeKey': 'com.volcengine.rtc.hybrid_runtime',
              }));

  /// @brief 用户 ID。该字符串符合正则表达式：`[a-zA-Z0-9_\@\\-\\.]{1,128}`。
  FutureOr<NSString?> get userId async {
    return await sendInstanceGet<NSString?>("userId");
  }

  set userId(FutureOr<NSString?> value) {
    sendInstanceSet("userId", value);
  }

  /// @brief 用户需要透传的额外的信息，字符长度限制为 200 字节。会在 `rtcRoom:onUserJoined:` 中回调给远端用户。
  FutureOr<NSString?> get extraInfo async {
    return await sendInstanceGet<NSString?>("extraInfo");
  }

  set extraInfo(FutureOr<NSString?> value) {
    sendInstanceSet("extraInfo", value);
  }
}

enum ByteRTCRoomProfile {
  /// @brief 默认场景，通用模式 <br>
  ///        与 `ByteRTCRoomProfileMeeting = 16` 配置相同。 <br>
  ///        你可以联系技术支持更换默认配置参数。
  ///
  ByteRTCRoomProfileCommunication(0),

  /// @brief 游戏语音模式，低功耗、低流量消耗。 <br>
  ///        低端机在此模式下运行时，进行了额外的性能优化： <br>
  ///            - 部分低端机型配置编码帧长 40/60
  ///            - 部分低端机型关闭软件 3A 音频处理
  ///        增强对 iOS 其他屏幕录制进行的兼容性，避免音频录制被 RTC 打断。
  ///
  ByteRTCRoomProfileGame(2),

  /// @brief 云游戏模式。 <br>
  ///        如果你的游戏场景需要低延迟的配置，使用此设置。 <br>
  ///        此设置下，弱网抗性较差。
  ///
  ByteRTCRoomProfileCloudGame(3),

  /// @brief 云渲染模式。超低延时配置。 <br>
  ///        如果你的应用并非游戏但又对延时要求较高时，选用此模式 <br>
  ///        该模式下，音视频通话延时会明显降低，但同时弱网抗性、通话音质等均会受到一定影响。
  ///
  ByteRTCRoomProfileLowLatency(4),

  /// @brief 适用于 3 人及以上纯语音通话。 <br>
  ///        通话中，闭麦时为是媒体模式，上麦后切换为通话模式。
  ///
  ByteRTCRoomProfileChatRoom(6),

  /// @brief 适用于单主播和观众进行音视频互动的直播。通话模式，上下麦不会有模式切换，避免音量突变现象
  ///
  ByteRTCRoomProfileInteractivePodcast(10),

  /// @brief 适合在线实时合唱场景，高音质，超低延迟。使用本配置前请联系技术支持进行协助完成其他配置。
  ///
  ByteRTCRoomProfileChorus(12),

  /// @brief 适用于 1 vs 1 游戏串流，支持公网或局域网。
  ///
  ByteRTCRoomProfileGameStreaming(14),

  /// @brief 适用于云端会议中的个人设备
  ///
  ByteRTCRoomProfileMeeting(16),

  /// @brief 适用于云端会议中的会议室终端设备，例如 Rooms，投屏盒子等。
  ///
  ByteRTCRoomProfileMeetingRoom(17),

  /// @brief 适用于课堂互动，房间内所有成员都可以进行音视频互动 <br>
  ///        当你的场景中需要同时互动的成员超过 10 人时使用此模式
  ///
  ByteRTCRoomProfileClassroom(18),

  /// @brief 注重流畅性，缺省码率相对低。适用于通话。
  ///
  ByteRTCRoomProfileCall(19),

  /// @brief 更注重画质，视频缺省码率相对高。。适用于直播互动。
  ///
  ByteRTCRoomProfileLive(20);

  final dynamic $value;
  const ByteRTCRoomProfile([this.$value]);
}

enum ByteRTCMediaStreamType {
  /// @brief 只控制音频
  ///
  ByteRTCMediaStreamTypeAudio(1),

  /// @brief 只控制视频
  ///
  ByteRTCMediaStreamTypeVideo(2),

  /// @brief 同时控制音频和视频
  ///
  ByteRTCMediaStreamTypeBoth(3);

  final dynamic $value;
  const ByteRTCMediaStreamType([this.$value]);
}

class ByteRTCRemoteAudioPropertiesInfo extends NativeClass {
  static const _$namespace = r'ByteRTCRemoteAudioPropertiesInfo';
  static get codegen_$namespace => _$namespace;

  ByteRTCRemoteAudioPropertiesInfo([NativeClassOptions? options])
      : super(options == null
            ? const NativeClassOptions(
                [],
                className: _$namespace,
                instanceType: InstanceType.manual,
                bridgeKey: 'com.volcengine.rtc.hybrid_runtime',
              )
            : NativeClassOptions.fromMap({
                ...options.toMap(),
                'bridgeKey': 'com.volcengine.rtc.hybrid_runtime',
              }));

  /// @detail keytype
  /// @brief 远端流的唯一标识
  FutureOr<NSString?> get streamId async {
    return await sendInstanceGet<NSString?>("streamId");
  }

  set streamId(FutureOr<NSString?> value) {
    sendInstanceSet("streamId", value);
  }

  /// @detail keytype
  /// @brief 远端流详细信息，详见 ByteRTCStreamInfo{@link #ByteRTCStreamInfo}
  FutureOr<ByteRTCStreamInfo?> get streamInfo async {
    try {
      final result = await sendInstanceGet<ByteRTCStreamInfo?>("streamInfo");
      if (result == null) {
        return null;
      }
      return packObject(
          result,
          () => ByteRTCStreamInfo(
              const NativeClassOptions([], disableInit: true)));
    } catch (e) {
      return null;
    }
  }

  set streamInfo(FutureOr<ByteRTCStreamInfo?> value) {
    sendInstanceSet("streamInfo", value);
  }

  /// @detail keytype
  /// @brief 音频属性信息，详见 ByteRTCAudioPropertiesInfo{@link #ByteRTCAudioPropertiesInfo}
  FutureOr<ByteRTCAudioPropertiesInfo?> get audioPropertiesInfo async {
    try {
      final result = await sendInstanceGet<ByteRTCAudioPropertiesInfo?>(
          "audioPropertiesInfo");
      if (result == null) {
        return null;
      }
      return packObject(
          result,
          () => ByteRTCAudioPropertiesInfo(
              const NativeClassOptions([], disableInit: true)));
    } catch (e) {
      return null;
    }
  }

  set audioPropertiesInfo(FutureOr<ByteRTCAudioPropertiesInfo?> value) {
    sendInstanceSet("audioPropertiesInfo", value);
  }
}

enum ByteRTCAudioMixingError {
  /// @brief 正常
  ///
  ByteRTCAudioMixingErrorOk(0),

  /// @brief 预加载失败，找不到混音文件或者文件长度超出 20s
  ///
  ByteRTCAudioMixingErrorPreloadFailed(1),

  /// @brief 混音开启失败，找不到混音文件或者混音文件打开失败
  ///
  ByteRTCAudioMixingErrorStartFailed(2),

  /// @brief 混音 ID 异常
  ///
  ByteRTCAudioMixingErrorIdNotFound(3),

  /// @brief 设置混音文件的播放位置出错
  ///
  ByteRTCAudioMixingErrorSetPositionFailed(4),

  /// @brief 音量参数不合法，仅支持设置的音量值为[0, 400]
  ///
  ByteRTCAudioMixingErrorInValidVolume(5),

  /// @brief 播放的文件与预加载的文件不一致。请先使用 unloadAudioMixing: 卸载此前的文件。
  ///
  ByteRTCAudioMixingErrorLoadConflict(6),

  /// @brief 不支持此混音类型。
  ///
  ByteRTCAudioMixingErrorIdTypeNotMatch(7),

  /// @brief 设置混音文件的音调不合法
  ///
  ByteRTCAudioMixingErrorInValidPitch(8),

  /// @brief 设置混音文件的音轨不合法
  ///
  ByteRTCAudioMixingErrorInValidAudioTrack(9),

  /// @brief 混音文件正在启动中
  ///
  ByteRTCAudioMixingErrorIsStarting(10),

  /// @brief 设置混音文件的播放速度不合法
  ///
  ByteRTCAudioMixingErrorInValidPlaybackSpeed(11);

  final dynamic $value;
  const ByteRTCAudioMixingError([this.$value]);
}

enum ByteRTCSubscribeMode {
  /// @brief 自动订阅模式。SDK 会自动为你订阅房间中的每一路流。
  ///
  ByteRTCSubscribeModeAuto(0),

  /// @brief 手动订阅模式。SDK 不自动订阅房间内的音视频流。你应根据需要调用 `subscribeStream` 方法手动订阅其他用户发布的音视频流。
  ///
  ByteRTCSubscribeModeManual(1);

  final dynamic $value;
  const ByteRTCSubscribeMode([this.$value]);
}

class ByteRTCHotMusicInfo extends NativeClass {
  static const _$namespace = r'ByteRTCHotMusicInfo';
  static get codegen_$namespace => _$namespace;

  ByteRTCHotMusicInfo([NativeClassOptions? options])
      : super(options == null
            ? const NativeClassOptions(
                [],
                className: _$namespace,
                instanceType: InstanceType.manual,
                bridgeKey: 'com.volcengine.rtc.hybrid_runtime',
              )
            : NativeClassOptions.fromMap({
                ...options.toMap(),
                'bridgeKey': 'com.volcengine.rtc.hybrid_runtime',
              }));

  /// @brief 热榜名称。
  FutureOr<NSString?> get hotName async {
    return await sendInstanceGet<NSString?>("hotName");
  }

  set hotName(FutureOr<NSString?> value) {
    sendInstanceSet("hotName", value);
  }

  /// @brief 歌曲数据，参看 ByteRTCMusicInfo{@link #ByteRTCMusicInfo}。
  FutureOr<NSArray<ByteRTCMusicInfo>?> get musics async {
    try {
      final result =
          await sendInstanceGet<NSArray<ByteRTCMusicInfo>?>("musics");
      if (result == null) {
        return null;
      }
      final list = result.map((e) => packObject(
          e,
          () => ByteRTCMusicInfo(
              const NativeClassOptions([], disableInit: true))));
      return list.toList();
    } catch (e) {
      return null;
    }
  }

  set musics(FutureOr<NSArray<ByteRTCMusicInfo>?> value) {
    sendInstanceSet("musics", value);
  }
}

enum ByteRTCUserVisibilityChangeError {
  /// @brief 成功。
  ///
  ByteRTCUserVisibilityChangeErrorOk(0),

  /// @brief 未知错误。
  ///
  ByteRTCUserVisibilityChangeErrorUnknown(1),

  /// @brief 房间内可见用户达到上限。
  ///
  ByteRTCUserVisibilityChangeErrorTooManyVisibleUser(2);

  final dynamic $value;
  const ByteRTCUserVisibilityChangeError([this.$value]);
}

enum ByteRTCVirtualBackgroundSourceType {
  /// @brief 使用纯色背景替换视频原有背景。
  ///
  ByteRTCVirtualBackgroundSourceTypeColor(0),

  /// @brief 使用自定义图片替换视频原有背景。
  ///
  ByteRTCVirtualBackgroundSourceTypeImage(1);

  final dynamic $value;
  const ByteRTCVirtualBackgroundSourceType([this.$value]);
}

enum ByteRTCAudioSampleRate {
  /// @brief 默认设置。48000Hz。
  ///
  ByteRTCAudioSampleRateAuto(-1),

  /// @brief 8000Hz
  ///
  ByteRTCAudioSampleRate8000(8000),

  /// @brief 11025Hz
  ///
  ByteRTCAudioSampleRate11025(11025),

  /// @brief 16000Hz
  ///
  ByteRTCAudioSampleRate16000(16000),

  /// @brief 22050Hz
  ///
  ByteRTCAudioSampleRate22050(22050),

  /// @brief 24000Hz
  ///
  ByteRTCAudioSampleRate24000(24000),

  /// @brief 32000Hz
  ///
  ByteRTCAudioSampleRate32000(32000),

  /// @brief 44100Hz
  ///
  ByteRTCAudioSampleRate44100(44100),

  /// @brief 48000Hz
  ///
  ByteRTCAudioSampleRate48000(48000);

  final dynamic $value;
  const ByteRTCAudioSampleRate([this.$value]);
}

enum ByteRTCVideoCapturePreference {
  /// @brief （默认）自动设置采集参数。 <br>
  ///        SDK 在开启采集时根据服务端下发的采集配置结合编码参数设置最佳采集参数。
  ///
  ByteRTCVideoCapturePreferenceAuto(0),

  /// @brief 手动设置采集参数，包括采集分辨率、帧率。
  ///
  ByteRTCVideoCapturePreferenceMannal(1),

  /// @brief 采集参数与编码参数一致
  ///
  ByteRTCVideoCapturePreferenceAutoPerformance(2);

  final dynamic $value;
  const ByteRTCVideoCapturePreference([this.$value]);
}

enum ByteRTCPublishFallbackOption {
  /// @brief 上行网络不佳或设备性能不足时，不对音视频流作回退处理。默认设置。
  ///
  ByteRTCPublishFallbackOptionDisabled(0),

  /// @brief 上行网络不佳或设备性能不足时，发布的视频流会从大流到小流依次降级，直到与当前网络性能匹配，具体降级规则参看[性能回退](https://www.volcengine.com/docs/6348/70137)文档。
  ///
  ByteRTCPublishFallbackOptionSimulcast(1);

  final dynamic $value;
  const ByteRTCPublishFallbackOption([this.$value]);
}

enum ByteRTCEchoTestResult {
  /// @brief 接收到采集的音视频的回放，通话回路检测成功
  ///
  ByteRTCEchoTestResultSuccess(0),

  /// @brief 测试超过 60s 仍未完成，已自动停止
  ///
  ByteRTCEchoTestResultTimeout(1),

  /// @brief 上一次测试结束和下一次测试开始之间的时间间隔少于 5s
  ///
  ByteRTCEchoTestResultIntervalShort(2),

  /// @brief 音频采集异常
  ///
  ByteRTCEchoTestResultAudioDeviceError(3),

  /// @brief 视频采集异常
  ///
  ByteRTCEchoTestResultVideoDeviceError(4),

  /// @brief 音频接收异常
  ///
  ByteRTCEchoTestResultAudioReceiveError(5),

  /// @brief 视频接收异常
  ///
  ByteRTCEchoTestResultVideoReceiveError(6),

  /// @brief 内部错误，不可恢复
  ///
  ByteRTCEchoTestResultInternalError(7);

  final dynamic $value;
  const ByteRTCEchoTestResult([this.$value]);
}

enum ByteRTCAudioRoute {
  /// @brief 通过 `setDefaultAudioRoute:` 设置的音频路由
  ///
  ByteRTCAudioRouteDefault(-1),

  /// @brief 有线耳机
  ///
  ByteRTCAudioRouteHeadset(1),

  /// @brief 听筒。设备自带的，一般用于通话的播放硬件。
  ///
  ByteRTCAudioRouteEarpiece(2),

  /// @brief 扬声器。设备自带的，一般用于免提播放的硬件。
  ///
  ByteRTCAudioRouteSpeakerphone(3),

  /// @brief 蓝牙耳机
  ///
  ByteRTCAudioRouteHeadsetBluetooth(4),

  ByteRTCAudioRouteHeadsetUSB(5);

  final dynamic $value;
  const ByteRTCAudioRoute([this.$value]);
}

enum ByteRTCPlayerError {
  /// @brief 正常
  ///
  ByteRTCPlayerErrorOK(0),

  /// @brief 不支持此类型
  ///
  ByteRTCPlayerErrorFormatNotSupport(1),

  /// @brief 无效的播放路径
  ///
  ByteRTCPlayerErrorInvalidPath(2),

  /// @brief 未满足前序接口调用的要求。请查看具体接口文档。
  ///
  ByteRTCPlayerErrorInvalidState(3),

  /// @brief 设置播放位置出错。
  ///
  ByteRTCPlayerErrorInvalidPosition(4),

  /// @brief 音量参数不合法。
  ///
  ByteRTCPlayerErrorInvalidVolume(5),

  /// @brief 音调参数设置不合法。
  ///
  ByteRTCPlayerErrorInvalidPitch(6),

  /// @brief 音轨参数设置不合法。
  ///
  ByteRTCPlayerErrorInvalidAudioTrackIndex(7),

  /// @brief 播放速度参数设置不合法
  ///
  ByteRTCPlayerErrorInvalidPlaybackSpeed(8),

  /// @brief 音效 ID 异常。还未加载或播放文件，就调用其他 API。
  ///
  ByteRTCPlayerErrorInvalidEffectId(9),

  /// @brief 资源正在播放中
  ///
  ByteRTCPlayerErrorCurrentlyPlaying(10);

  final dynamic $value;
  const ByteRTCPlayerError([this.$value]);
}

enum ByteRTCAudioFrameCallbackMethod {
  /// @brief 本地麦克风录制的音频数据回调
  ///
  ByteRTCAudioFrameCallbackRecord(0),

  /// @brief 订阅的远端所有用户混音后的音频数据回调
  ///
  ByteRTCAudioFrameCallbackPlayback(1),

  /// @brief 本地麦克风录制和订阅的远端所有用户混音后的音频数据回调
  ///
  ByteRTCAudioFrameCallbackMixed(2),

  /// @brief 订阅的远端每个用户混音前的音频数据回调
  ///
  ByteRTCAudioFrameCallbackRemoteUser(3),

  /// @brief 本地麦克风录制和本地 `MediaPlayer`, `EffectPlayer` 播放的音频混音后的音频数据回调
  ///
  ByteRTCAudioFrameCallbackCaptureMixed(5);

  final dynamic $value;
  const ByteRTCAudioFrameCallbackMethod([this.$value]);
}

enum ByteRTCAttenuationType {
  /// @brief 不随距离衰减
  ///
  ByteRTCAttenuationTypeNone(0),

  /// @brief 线性衰减，音量随距离增大而线性减小
  ///
  ByteRTCAttenuationTypeLinear(1),

  /// @brief 指数型衰减，音量随距离增大进行指数衰减
  ///
  ByteRTCAttenuationTypeExponential(2);

  final dynamic $value;
  const ByteRTCAttenuationType([this.$value]);
}

class ByteRTCVideoPreprocessorConfig extends NativeClass {
  static const _$namespace = r'ByteRTCVideoPreprocessorConfig';
  static get codegen_$namespace => _$namespace;

  ByteRTCVideoPreprocessorConfig([NativeClassOptions? options])
      : super(options == null
            ? const NativeClassOptions(
                [],
                className: _$namespace,
                instanceType: InstanceType.manual,
                bridgeKey: 'com.volcengine.rtc.hybrid_runtime',
              )
            : NativeClassOptions.fromMap({
                ...options.toMap(),
                'bridgeKey': 'com.volcengine.rtc.hybrid_runtime',
              }));

  /// @brief 视频帧的像素格式，参看 ByteRTCVideoPixelFormat{@link #ByteRTCVideoPixelFormat}。 <br>
  ///        当前仅支持 `ByteRTCVideoPixelFormatI420` 和 `ByteRTCVideoPixelFormatUnknown` 格式。
  FutureOr<ByteRTCVideoPixelFormat?> get requiredPixelFormat async {
    try {
      final result = await sendInstanceGet<ByteRTCVideoPixelFormat?>(
          "requiredPixelFormat");
      if (result == null) {
        return null;
      }
      return ByteRTCVideoPixelFormat.values
          .firstWhere((element) => element == result);
    } catch (e) {
      return null;
    }
  }

  set requiredPixelFormat(FutureOr<ByteRTCVideoPixelFormat?> value) {
    sendInstanceSet("requiredPixelFormat", value);
  }
}

class ByteRTCRoomStats extends NativeClass {
  static const _$namespace = r'ByteRTCRoomStats';
  static get codegen_$namespace => _$namespace;

  ByteRTCRoomStats([NativeClassOptions? options])
      : super(options == null
            ? const NativeClassOptions(
                [],
                className: _$namespace,
                instanceType: InstanceType.manual,
                bridgeKey: 'com.volcengine.rtc.hybrid_runtime',
              )
            : NativeClassOptions.fromMap({
                ...options.toMap(),
                'bridgeKey': 'com.volcengine.rtc.hybrid_runtime',
              }));

  /// @brief 本地用户在本次通话中的参与时长，单位为 s
  FutureOr<NSInteger?> get duration async {
    return await sendInstanceGet<NSInteger?>("duration");
  }

  set duration(FutureOr<NSInteger?> value) {
    sendInstanceSet("duration", value);
  }

  /// @brief 本地用户的总发送字节数 (bytes)，累计值
  FutureOr<NSInteger?> get txBytes async {
    return await sendInstanceGet<NSInteger?>("txBytes");
  }

  set txBytes(FutureOr<NSInteger?> value) {
    sendInstanceSet("txBytes", value);
  }

  /// @brief 本地用户的总接收字节数 (bytes)，累计值
  FutureOr<NSInteger?> get rxBytes async {
    return await sendInstanceGet<NSInteger?>("rxBytes");
  }

  set rxBytes(FutureOr<NSInteger?> value) {
    sendInstanceSet("rxBytes", value);
  }

  /// @brief 发送码率（kbps），获取该数据时的瞬时值
  FutureOr<NSInteger?> get txKbitrate async {
    return await sendInstanceGet<NSInteger?>("txKbitrate");
  }

  set txKbitrate(FutureOr<NSInteger?> value) {
    sendInstanceSet("txKbitrate", value);
  }

  /// @brief 接收码率（kbps），获取该数据时的瞬时值
  FutureOr<NSInteger?> get rxKbitrate async {
    return await sendInstanceGet<NSInteger?>("rxKbitrate");
  }

  set rxKbitrate(FutureOr<NSInteger?> value) {
    sendInstanceSet("rxKbitrate", value);
  }

  /// @brief 本地用户的音频发送码率 (kbps)，瞬时值
  FutureOr<NSInteger?> get txAudioKBitrate async {
    return await sendInstanceGet<NSInteger?>("txAudioKBitrate");
  }

  set txAudioKBitrate(FutureOr<NSInteger?> value) {
    sendInstanceSet("txAudioKBitrate", value);
  }

  /// @brief 本地用户的音频接收码率 (kbps)，瞬时值
  FutureOr<NSInteger?> get rxAudioKBitrate async {
    return await sendInstanceGet<NSInteger?>("rxAudioKBitrate");
  }

  set rxAudioKBitrate(FutureOr<NSInteger?> value) {
    sendInstanceSet("rxAudioKBitrate", value);
  }

  /// @brief 本地用户的视频发送码率 (kbps)，瞬时值
  FutureOr<NSInteger?> get txVideoKBitrate async {
    return await sendInstanceGet<NSInteger?>("txVideoKBitrate");
  }

  set txVideoKBitrate(FutureOr<NSInteger?> value) {
    sendInstanceSet("txVideoKBitrate", value);
  }

  /// @brief 本地用户的视频接收码率 (kbps)，瞬时值
  FutureOr<NSInteger?> get rxVideoKBitrate async {
    return await sendInstanceGet<NSInteger?>("rxVideoKBitrate");
  }

  set rxVideoKBitrate(FutureOr<NSInteger?> value) {
    sendInstanceSet("rxVideoKBitrate", value);
  }

  /// @brief 屏幕接收码率，获取该数据时的瞬时值，单位为 Kbps
  FutureOr<NSInteger?> get txScreenKBitrate async {
    return await sendInstanceGet<NSInteger?>("txScreenKBitrate");
  }

  set txScreenKBitrate(FutureOr<NSInteger?> value) {
    sendInstanceSet("txScreenKBitrate", value);
  }

  /// @brief 屏幕发送码率，获取该数据时的瞬时值，单位为 Kbps
  FutureOr<NSInteger?> get rxScreenKBitrate async {
    return await sendInstanceGet<NSInteger?>("rxScreenKBitrate");
  }

  set rxScreenKBitrate(FutureOr<NSInteger?> value) {
    sendInstanceSet("rxScreenKBitrate", value);
  }

  /// @brief 当前房间内的可见用户人数，包括本地用户自身
  FutureOr<NSInteger?> get userCount async {
    return await sendInstanceGet<NSInteger?>("userCount");
  }

  set userCount(FutureOr<NSInteger?> value) {
    sendInstanceSet("userCount", value);
  }

  /// @brief App 现在的下行丢包率
  FutureOr<float?> get rxLostrate async {
    return await sendInstanceGet<float?>("rxLostrate");
  }

  set rxLostrate(FutureOr<float?> value) {
    sendInstanceSet("rxLostrate", value);
  }

  /// @brief App 现在的上行丢包率
  FutureOr<float?> get txLostrate async {
    return await sendInstanceGet<float?>("txLostrate");
  }

  set txLostrate(FutureOr<float?> value) {
    sendInstanceSet("txLostrate", value);
  }

  /// @brief 客户端到服务端的往返时延
  FutureOr<NSInteger?> get rtt async {
    return await sendInstanceGet<NSInteger?>("rtt");
  }

  set rtt(FutureOr<NSInteger?> value) {
    sendInstanceSet("rtt", value);
  }

  /// @hidden currently not available
  /// @brief 系统上行网络抖动（ms）
  FutureOr<NSInteger?> get txJitter async {
    return await sendInstanceGet<NSInteger?>("txJitter");
  }

  set txJitter(FutureOr<NSInteger?> value) {
    sendInstanceSet("txJitter", value);
  }

  /// @hidden currently not available
  /// @brief 系统下行网络抖动（ms）
  FutureOr<NSInteger?> get rxJitter async {
    return await sendInstanceGet<NSInteger?>("rxJitter");
  }

  set rxJitter(FutureOr<NSInteger?> value) {
    sendInstanceSet("rxJitter", value);
  }

  /// @brief 蜂窝路径发送的码率 (kbps)，为获取该数据时的瞬时值
  FutureOr<NSInteger?> get txCellularKBitrate async {
    return await sendInstanceGet<NSInteger?>("txCellularKBitrate");
  }

  set txCellularKBitrate(FutureOr<NSInteger?> value) {
    sendInstanceSet("txCellularKBitrate", value);
  }

  /// @brief 蜂窝路径接收码率 (kbps)，为获取该数据时的瞬时值
  FutureOr<NSInteger?> get rxCellularKBitrate async {
    return await sendInstanceGet<NSInteger?>("rxCellularKBitrate");
  }

  set rxCellularKBitrate(FutureOr<NSInteger?> value) {
    sendInstanceSet("rxCellularKBitrate", value);
  }
}

enum ByteRTCMediaDeviceError {
  /// @brief 媒体设备正常
  ///
  ByteRTCMediaDeviceErrorOK(0),

  /// @brief 没有权限启动媒体设备
  ///
  ByteRTCMediaDeviceErrorDeviceNoPermission(1),

  /// @brief 媒体设备已经在使用中
  ///
  ByteRTCMediaDeviceErrorDeviceBusy(2),

  /// @brief 媒体设备错误
  ///
  ByteRTCMediaDeviceErrorDeviceFailure(3),

  /// @brief 未找到指定的媒体设备
  ///
  ByteRTCMediaDeviceErrorDeviceNotFound(4),

  /// @brief 媒体设备被移除
  ///
  ByteRTCMediaDeviceErrorDeviceDisconnected(5),

  /// @brief 无采集数据。当媒体设备的预期行为是正常采集，但没有收到采集数据时，将收到该错误。
  ///
  ByteRTCMediaDeviceErrorDeviceNoCallback(6),

  /// @brief 设备采样率不支持
  ///
  ByteRTCMediaDeviceErrorUNSupportFormat(7),

  /// @hidden(macOS)
  /// @brief iOS 屏幕采集没有 group Id 参数
  ///
  ByteRTCMediaDeviceErrorNotFindGroupId(8),

  /// @hidden(macOS)
  /// @brief 视频采集中断：因用户使用系统相机，应用切换到后台运行，导致采集中断。
  ///
  ByteRTCMediaDeviceErrorNotAvailableInBackground(9),

  /// @hidden(macOS)
  /// @brief 视频采集中断：可能由于其他应用占用系统相机，导致视频设备暂时不可用，从而造成采集中断。
  ///
  ByteRTCMediaDeviceErrorVideoInUseByAnotherClient(10),

  /// @hidden(macOS)
  /// @brief 视频采集中断：当前应用处于侧拉、分屏或者画中画模式时，导致采集中断。
  ///
  ByteRTCMediaDeviceErrorNotAvailableWithMultipleForegroundApps(11),

  /// @hidden(macOS)
  /// @brief 视频采集中断：由于系统性能不足导致中断，比如设备过热。
  ///
  ByteRTCMediaDeviceErrorNotAvailableDueToSystemPressure(12),

  /// @hidden(iOS)
  /// @brief 系统服务错误或长时间不响应，建议重启音频服务进程或重启设备。
  ///
  ByteRTCMediaDeviceErrorAudioServerNeedRestart(13);

  final dynamic $value;
  const ByteRTCMediaDeviceError([this.$value]);
}

enum ByteRTCMode {
  /// @brief 通用模式，该模式下只能进行普通会议模式的语音通话，开启自动发布订阅。
  ///
  ByteRTCModeGeneral(0),

  /// @brief 游戏语音模式，该模式下关闭自动发布订阅，会按照游戏业务的策略进行主动发布订阅语音。
  ///
  ByteRTCModeLocalAudio(1);

  final dynamic $value;
  const ByteRTCMode([this.$value]);
}

class ByteRTCVideoFrameData extends NativeClass {
  static const _$namespace = r'ByteRTCVideoFrameData';
  static get codegen_$namespace => _$namespace;

  ByteRTCVideoFrameData([NativeClassOptions? options])
      : super(options == null
            ? const NativeClassOptions(
                [],
                className: _$namespace,
                instanceType: InstanceType.manual,
                bridgeKey: 'com.volcengine.rtc.hybrid_runtime',
              )
            : NativeClassOptions.fromMap({
                ...options.toMap(),
                'bridgeKey': 'com.volcengine.rtc.hybrid_runtime',
              }));

  /// @brief 视频帧缓冲区类型，参考 ByteRTCVideoBufferType {@link #ByteRTCVideoBufferType}。必填。
  FutureOr<ByteRTCVideoBufferType?> get bufferType async {
    try {
      final result =
          await sendInstanceGet<ByteRTCVideoBufferType?>("bufferType");
      if (result == null) {
        return null;
      }
      return ByteRTCVideoBufferType.values
          .firstWhere((element) => element == result);
    } catch (e) {
      return null;
    }
  }

  set bufferType(FutureOr<ByteRTCVideoBufferType?> value) {
    sendInstanceSet("bufferType", value);
  }

  /// @brief 视频帧像素格式，参考 ByteRTCVideoPixelFormat {@link #ByteRTCVideoPixelFormat}。当 `bufferType` 为 `ByteRTCVideoBufferTypeGLTexture` 时必填。
  FutureOr<ByteRTCVideoPixelFormat?> get pixelFormat async {
    try {
      final result =
          await sendInstanceGet<ByteRTCVideoPixelFormat?>("pixelFormat");
      if (result == null) {
        return null;
      }
      return ByteRTCVideoPixelFormat.values
          .firstWhere((element) => element == result);
    } catch (e) {
      return null;
    }
  }

  set pixelFormat(FutureOr<ByteRTCVideoPixelFormat?> value) {
    sendInstanceSet("pixelFormat", value);
  }

  /// @brief 视频内容类型，参看 ByteRTCVideoContentType{@link #ByteRTCVideoContentType}
  FutureOr<ByteRTCVideoContentType?> get contentType async {
    try {
      final result =
          await sendInstanceGet<ByteRTCVideoContentType?>("contentType");
      if (result == null) {
        return null;
      }
      return ByteRTCVideoContentType.values
          .firstWhere((element) => element == result);
    } catch (e) {
      return null;
    }
  }

  set contentType(FutureOr<ByteRTCVideoContentType?> value) {
    sendInstanceSet("contentType", value);
  }

  /// @brief 当前帧的时间戳。必填。
  FutureOr<int?> get timestamp async {
    return await sendInstanceGet<int?>("timestamp");
  }

  set timestamp(FutureOr<int?> value) {
    sendInstanceSet("timestamp", value);
  }

  /// @brief 视频帧宽度。必填。
  FutureOr<int?> get width async {
    return await sendInstanceGet<int?>("width");
  }

  set width(FutureOr<int?> value) {
    sendInstanceSet("width", value);
  }

  /// @brief 视频帧高度。必填。
  FutureOr<int?> get height async {
    return await sendInstanceGet<int?>("height");
  }

  set height(FutureOr<int?> value) {
    sendInstanceSet("height", value);
  }

  /// @brief `CVPixelBufferRef` 类型的数据，当 `format` 为 `kPixelFormatCVPixelBuffer` 时，必填。
  FutureOr<CVPixelBufferRef?> get cvpixelbuffer async {
    return await sendInstanceGet<CVPixelBufferRef?>("cvpixelbuffer");
  }

  set cvpixelbuffer(FutureOr<CVPixelBufferRef?> value) {
    sendInstanceSet("cvpixelbuffer", value);
  }

  /// @brief 视频帧旋转角度，参看 ByteRTCVideoRotation{@link #ByteRTCVideoRotation}。
  FutureOr<ByteRTCVideoRotation?> get rotation async {
    try {
      final result = await sendInstanceGet<ByteRTCVideoRotation?>("rotation");
      if (result == null) {
        return null;
      }
      return ByteRTCVideoRotation.values
          .firstWhere((element) => element == result);
    } catch (e) {
      return null;
    }
  }

  set rotation(FutureOr<ByteRTCVideoRotation?> value) {
    sendInstanceSet("rotation", value);
  }

  /// @brief 视频帧的摄像头位置信息，参考 ByteRTCCameraID{@link #ByteRTCCameraID}
  FutureOr<ByteRTCCameraID?> get cameraId async {
    try {
      final result = await sendInstanceGet<ByteRTCCameraID?>("cameraId");
      if (result == null) {
        return null;
      }
      return ByteRTCCameraID.values.firstWhere((element) => element == result);
    } catch (e) {
      return null;
    }
  }

  set cameraId(FutureOr<ByteRTCCameraID?> value) {
    sendInstanceSet("cameraId", value);
  }

  /// @brief SEI 数据
  FutureOr<NSData?> get seiData async {
    return await sendInstanceGet<NSData?>("seiData");
  }

  set seiData(FutureOr<NSData?> value) {
    sendInstanceSet("seiData", value);
  }

  /// @brief 视频帧感兴趣区域数据
  FutureOr<NSData?> get roiData async {
    return await sendInstanceGet<NSData?>("roiData");
  }

  set roiData(FutureOr<NSData?> value) {
    sendInstanceSet("roiData", value);
  }

  /// @brief 视频帧平面数。当 `bufferType` 为 `ByteRTCVideoBufferTypeRawMemory` 时必填。
  FutureOr<int?> get numberOfPlanes async {
    return await sendInstanceGet<int?>("numberOfPlanes");
  }

  set numberOfPlanes(FutureOr<int?> value) {
    sendInstanceSet("numberOfPlanes", value);
  }

  /// @brief 视频帧平面数组。当 `bufferType` 为 `ByteRTCVideoBufferTypeRawMemory` 时必填。
  FutureOr<Unknown?> get planeDataArray async {
    return await sendInstanceGet<Unknown?>("planeDataArray");
  }

  set planeDataArray(FutureOr<Unknown?> value) {
    sendInstanceSet("planeDataArray", value);
  }

  /// @brief stride 数组。stride 指视频帧平面相邻两行图像数据之间的内存长度（单位字节）。当 `bufferType` 为 `ByteRTCVideoBufferTypeRawMemory` 时必填。
  FutureOr<int?> get planeStrideArray async {
    return await sendInstanceGet<int?>("planeStrideArray");
  }

  set planeStrideArray(FutureOr<int?> value) {
    sendInstanceSet("planeStrideArray", value);
  }
}

enum ByteRTCMulDimSingScoringMode {
  /// @brief 按照音高进行评分。
  ///
  ByteRTCMulDimSingScoringModeNote(0);

  final dynamic $value;
  const ByteRTCMulDimSingScoringMode([this.$value]);
}

enum ByteRTCAudioReportMode {
  /// @brief 默认始终开启音量回调。
  ///
  ByteRTCAudioReportModeNormal(0),

  /// @brief 可见用户进房并停止推流后，关闭音量回调。
  ///
  ByteRTCAudioReportModeDisconnect(1),

  /// @brief 可见用户进房并停止推流后，开启音量回调，回调值重置为 0。
  ///
  ByteRTCAudioReportModeReset(2);

  final dynamic $value;
  const ByteRTCAudioReportMode([this.$value]);
}

enum ByteRTCEarMonitorAudioFilter {
  /// @brief 无音频处理。
  ///
  ByteRTCEarMonitorAudioFilterNone('0x0001'),

  /// @brief 经本地音频处理后的音频，包括 3A、变声、混响等 SDK 提供处理能力以及自定义音频处理。
  ///
  ByteRTCEarMonitorAudioFilterReuseAudioProcessing('0x8000');

  final dynamic $value;
  const ByteRTCEarMonitorAudioFilter([this.$value]);
}

enum ByteRTCLocalVideoStreamState {
  /// @brief 本地视频默认初始状态 <br>
  ///        摄像头停止工作时回调该状态，对应错误码 ByteRTCLocalVideoStreamError{@link #ByteRTCLocalVideoStreamError} 中的 ByteRTCLocalVideoStreamErrorOk
  ///
  ByteRTCLocalVideoStreamStateStopped(0),

  /// @brief 本地视频录制设备启动成功 <br>
  ///        采集到视频首帧时回调该状态，对应错误码 ByteRTCLocalVideoStreamError{@link #ByteRTCLocalVideoStreamError} 中的 ByteRTCLocalVideoStreamErrorOk
  ///
  ByteRTCLocalVideoStreamStateRecording(1),

  /// @brief 本地视频首帧编码成功 <br>
  ///        本地视频首帧编码成功时回调该状态，对应错误码 ByteRTCLocalVideoStreamError{@link #ByteRTCLocalVideoStreamError} 中的 ByteRTCLocalVideoStreamErrorOk
  ///
  ByteRTCLocalVideoStreamStateEncoding(2),

  /// @brief 本地视频启动失败, 在以下时机回调该状态： <br>
  ///       - 本地采集设备启动失败，对应错误码 ByteRTCLocalVideoStreamError{@link #ByteRTCLocalVideoStreamError} 中的 ByteRTCLocalVideoStreamErrorCaptureFailure
  ///       - 检测到没有摄像头权限，对应错误码 ByteRTCLocalVideoStreamError{@link #ByteRTCLocalVideoStreamError} 中的 ByteRTCLocalVideoStreamErrorDeviceNoPermission
  ///       - 视频编码失败，对应错误码 ByteRTCLocalVideoStreamError{@link #ByteRTCLocalVideoStreamError} 中的 ByteRTCLocalVideoStreamErrorEncodeFailure
  ///
  ByteRTCLocalVideoStreamStateFailed(3);

  final dynamic $value;
  const ByteRTCLocalVideoStreamState([this.$value]);
}

enum ByteRTCAACProfile {
  /// @brief AAC-LC 规格，默认值。
  ///
  ByteRTCAACProfileLC(0),

  /// @brief HE-AAC v1 规格。
  ///
  ByteRTCAACProfileHEv1(1),

  /// @brief HE-AAC v2 规格。
  ///
  ByteRTCAACProfileHEv2(2);

  final dynamic $value;
  const ByteRTCAACProfile([this.$value]);
}

enum ByteRTCVideoDenoiseModeChangedReason {
  /// @brief 未知原因导致视频降噪状态改变。
  ///
  ByteRTCVideoDenoiseModeChangedReasonNull(-1),

  /// @brief 通过调用 setVideoDenoiser:{@link #ByteRTCEngine#setVideoDenoiser} 成功关闭视频降噪。
  ///
  ByteRTCVideoDenoiseModeChangedReasonApiOff(0),

  /// @brief 通过调用 setVideoDenoiser:{@link #ByteRTCEngine#setVideoDenoiser} 成功开启视频降噪。
  ///
  ByteRTCVideoDenoiseModeChangedReasonApiOn(1),

  /// @brief 后台未配置视频降噪，视频降噪开启失败，请联系技术人员解决。
  ///
  ByteRTCVideoDenoiseModeChangedReasonConfigDisabled(2),

  /// @brief 后台配置开启了视频降噪。
  ///
  ByteRTCVideoDenoiseModeChangedReasonConfigEnabled(3),

  /// @brief 由于内部发生了异常，视频降噪关闭。
  ///
  ByteRTCVideoDenoiseModeChangedReasonInternalException(4),

  /// @brief 当前设备性能过载，已动态关闭视频降噪。
  ///
  ByteRTCVideoDenoiseModeChangedReasonDynamicClose(5),

  /// @brief 当前设备性能裕量充足，已动态开启视频降噪。
  ///
  ByteRTCVideoDenoiseModeChangedReasonDynamicOpen(6),

  /// @brief 分辨率导致视频降噪状态发生改变。分辨率过高会导致性能消耗过大，从而导致视频降噪关闭。如若希望继续使用视频降噪，可选择降低分辨率。
  ///
  ByteRTCVideoDenoiseModeChangedReasonResolution(7);

  final dynamic $value;
  const ByteRTCVideoDenoiseModeChangedReason([this.$value]);
}

class ByteRTCLocalStreamStats extends NativeClass {
  static const _$namespace = r'ByteRTCLocalStreamStats';
  static get codegen_$namespace => _$namespace;

  ByteRTCLocalStreamStats([NativeClassOptions? options])
      : super(options == null
            ? const NativeClassOptions(
                [],
                className: _$namespace,
                instanceType: InstanceType.manual,
                bridgeKey: 'com.volcengine.rtc.hybrid_runtime',
              )
            : NativeClassOptions.fromMap({
                ...options.toMap(),
                'bridgeKey': 'com.volcengine.rtc.hybrid_runtime',
              }));

  /// @brief 本地设备发送音频流的统计信息，详见 ByteRTCLocalAudioStats{@link #ByteRTCLocalAudioStats}
  FutureOr<ByteRTCLocalAudioStats?> get audioStats async {
    try {
      final result =
          await sendInstanceGet<ByteRTCLocalAudioStats?>("audioStats");
      if (result == null) {
        return null;
      }
      return packObject(
          result,
          () => ByteRTCLocalAudioStats(
              const NativeClassOptions([], disableInit: true)));
    } catch (e) {
      return null;
    }
  }

  set audioStats(FutureOr<ByteRTCLocalAudioStats?> value) {
    sendInstanceSet("audioStats", value);
  }

  /// @brief 本地设备发送视频流的统计信息，详见 ByteRTCLocalVideoStats{@link #ByteRTCLocalVideoStats}
  FutureOr<ByteRTCLocalVideoStats?> get videoStats async {
    try {
      final result =
          await sendInstanceGet<ByteRTCLocalVideoStats?>("videoStats");
      if (result == null) {
        return null;
      }
      return packObject(
          result,
          () => ByteRTCLocalVideoStats(
              const NativeClassOptions([], disableInit: true)));
    } catch (e) {
      return null;
    }
  }

  set videoStats(FutureOr<ByteRTCLocalVideoStats?> value) {
    sendInstanceSet("videoStats", value);
  }

  /// @brief 所属用户的媒体流是否为屏幕流。你可以知道当前统计数据来自主流还是屏幕流。
  FutureOr<BOOL?> get isScreen async {
    return await sendInstanceGet<BOOL?>("isScreen");
  }

  set isScreen(FutureOr<BOOL?> value) {
    sendInstanceSet("isScreen", value);
  }

  /// @brief 所属用户的媒体流上行网络质量，详见 ByteRTCNetworkQuality{@link #ByteRTCNetworkQuality}
  /// @deprecated since 3.36 and will be deleted in 3.51, use rtcRoom:onNetworkQuality:remoteQualities:{@link #ByteRTCRoomDelegate#rtcRoom:onNetworkQuality:remoteQualities} instead
  FutureOr<ByteRTCNetworkQuality?> get txQuality async {
    try {
      final result = await sendInstanceGet<ByteRTCNetworkQuality?>("txQuality");
      if (result == null) {
        return null;
      }
      return ByteRTCNetworkQuality.values
          .firstWhere((element) => element == result);
    } catch (e) {
      return null;
    }
  }

  set txQuality(FutureOr<ByteRTCNetworkQuality?> value) {
    sendInstanceSet("txQuality", value);
  }

  /// @brief 所属用户的媒体流下行网络质量，详见 ByteRTCNetworkQuality{@link #ByteRTCNetworkQuality}
  /// @deprecated since 3.36 and will be deleted in 3.51, use rtcRoom:onNetworkQuality:remoteQualities:{@link #ByteRTCRoomDelegate#rtcRoom:onNetworkQuality:remoteQualities} instead
  FutureOr<ByteRTCNetworkQuality?> get rxQuality async {
    try {
      final result = await sendInstanceGet<ByteRTCNetworkQuality?>("rxQuality");
      if (result == null) {
        return null;
      }
      return ByteRTCNetworkQuality.values
          .firstWhere((element) => element == result);
    } catch (e) {
      return null;
    }
  }

  set rxQuality(FutureOr<ByteRTCNetworkQuality?> value) {
    sendInstanceSet("rxQuality", value);
  }
}

enum ByteRTCMixedStreamSEIContentMode {
  /// @brief 视频流中包含全部的 SEI 信息。默认设置。
  ///
  ByteRTCMixedStreamSEIContentModeDefault(0),

  /// @brief 随非关键帧传输的 SEI 数据中仅包含音量信息。 <br>
  ///        当设置 `ByteRTCMixedStreamControlConfig.enableVolumeIndication` 为 True 时，此参数设置生效。
  ///
  ByteRTCMixedStreamSEIContentModeEnableVolumeIndication(1);

  final dynamic $value;
  const ByteRTCMixedStreamSEIContentMode([this.$value]);
}

enum ByteRTCMediaPlayerCustomSourceSeekWhence {
  /// @brief 从音频数据的头开始读取，读取后的位置为参数 offset 的值。
  ///
  ByteRTCMediaPlayerCustomSourceSeekWhenceSet(0),

  /// @brief 从音频数据的某一位置开始读取，读取后的位置为音频数据当前的读取位置加上参数 offset 的值。
  ///
  ByteRTCMediaPlayerCustomSourceSeekWhenceCur(1),

  /// @brief 从音频数据的尾开始读取，读取后的位置为用户传入的音频数据大小加上参数 offset 的值。
  ///
  ByteRTCMediaPlayerCustomSourceSeekWhenceEnd(2),

  /// @brief 返回音频数据的大小。
  ///
  ByteRTCMediaPlayerCustomSourceSeekWhenceSize(3);

  final dynamic $value;
  const ByteRTCMediaPlayerCustomSourceSeekWhence([this.$value]);
}

enum ByteRTCRoomStateChangeReason {
  /// @brief 首次进房成功。
  ///
  ByteRTCRoomStateChangeReasonJoinRoom(0),

  /// @brief 重新进房，比如断网重连。
  ///
  ByteRTCRoomStateChangeReasonReconnect(1),

  /// @brief 离开房间。
  ///
  ByteRTCRoomStateChangeReasonLeaveRoom(2),

  /// @brief 进房失败。通过 rtcRoom:onRoomStateChanged:withUid:state:extraInfo:{@link #ByteRTCRoomDelegate#rtcRoom:onRoomStateChanged:withUid:state:extraInfo} 回调。 <br>
  ///        初次进房或者由于网络状况不佳断网重连时，由于服务器错误导致进房失败。SDK 会自动重试进房。
  ///
  ByteRTCRoomStateChangeReasonJoinRoomFailed(-2001),

  /// @brief Token 无效。通过 rtcRoom:onRoomStateChanged:withUid:state:extraInfo:{@link #ByteRTCRoomDelegate#rtcRoom:onRoomStateChanged:withUid:state:extraInfo} 回调。 <br>
  ///        调用 joinRoom:userInfo:userVisibility:roomConfig:{@link #ByteRTCRoom#joinRoom:userInfo:userVisibility:roomConfig} 进房时使用的 Token 参数有误或过期失效。需要重新获取 Token，并调用 updateToken:{@link #ByteRTCRTSRoom#updateToken} 方法更新 Token。
  ///
  ByteRTCRoomStateChangeReasonInvalidToken(-1000),

  /// @brief Token 过期。加入房间后 Token 过期时，返回此错误码。需使用新的 Token 重新加入房间。通过 rtcRoom:onRoomStateChanged:withUid:state:extraInfo:{@link #ByteRTCRoomDelegate#rtcRoom:onRoomStateChanged:withUid:state:extraInfo} 回调。
  ///
  ByteRTCRoomStateChangeReasonTokenExpired(-1009),

  /// @brief 调用 `updateToken:` 传入的 Token 无效。通过 rtcRoom:onRoomStateChanged:withUid:state:extraInfo:{@link #ByteRTCRoomDelegate#rtcRoom:onRoomStateChanged:withUid:state:extraInfo} 回调。
  ///
  ByteRTCRoomStateChangeReasonUpdateTokenWithInvalidToken(-1010),

  /// @brief 房间被封禁。通过 rtcRoom:onRoomStateChanged:withUid:state:extraInfo:{@link #ByteRTCRoomDelegate#rtcRoom:onRoomStateChanged:withUid:state:extraInfo} 回调。
  ///
  ByteRTCRoomStateChangeReasonRoomForbidden(-1025),

  /// @brief 用户被封禁。通过 rtcRoom:onRoomStateChanged:withUid:state:extraInfo:{@link #ByteRTCRoomDelegate#rtcRoom:onRoomStateChanged:withUid:state:extraInfo} 回调。
  ///
  ByteRTCRoomStateChangeReasonUserForbidden(-1026),

  /// @brief 服务端调用 OpenAPI 将当前用户踢出房间。通过 rtcRoom:onRoomStateChanged:withUid:state:extraInfo:{@link #ByteRTCRoomDelegate#rtcRoom:onRoomStateChanged:withUid:state:extraInfo} 回调。
  ///
  ByteRTCRoomStateChangeReasonKickedOut(-1006),

  /// @brief 服务端调用 OpenAPI 解散房间，所有用户被移出房间。通过 rtcRoom:onRoomStateChanged:withUid:state:extraInfo:{@link #ByteRTCRoomDelegate#rtcRoom:onRoomStateChanged:withUid:state:extraInfo} 回调。
  ///
  ByteRTCRoomStateChangeReasonRoomDismiss(-1011),

  /// @brief 相同用户 ID 的用户加入本房间，当前用户被踢出房间。通过 rtcRoom:onRoomStateChanged:withUid:state:extraInfo:{@link #ByteRTCRoomDelegate#rtcRoom:onRoomStateChanged:withUid:state:extraInfo} 回调。
  ///
  ByteRTCRoomStateChangeReasonDuplicateLogin(-1004),

  /// @hidden internal use only
  /// @brief 加入房间错误。通过 rtcRoom:onRoomStateChanged:withUid:state:extraInfo:{@link #ByteRTCRoomDelegate#rtcRoom:onRoomStateChanged:withUid:state:extraInfo} 回调。 <br>
  ///        调用 `joinRoom:userInfo:roomConfig:` 方法时, LICENSE 计费账号未使用 LICENSE_AUTHENTICATE SDK，加入房间错误。
  ///
  ByteRTCRoomStateChangeReasonWithoutLicenseAuthenticateSDK(-1012),

  /// @hidden internal use only
  /// @brief 服务端 license 过期，拒绝进房。通过 rtcRoom:onRoomStateChanged:withUid:state:extraInfo:{@link #ByteRTCRoomDelegate#rtcRoom:onRoomStateChanged:withUid:state:extraInfo} 回调。
  ///
  ByteRTCRoomStateChangeReasonServerLicenseExpired(-1017),

  /// @hidden internal use only
  /// @brief 超过服务端 license 许可的并发量上限，拒绝进房。通过 rtcRoom:onRoomStateChanged:withUid:state:extraInfo:{@link #ByteRTCRoomDelegate#rtcRoom:onRoomStateChanged:withUid:state:extraInfo} 回调。
  ///
  ByteRTCRoomStateChangeReasonExceedsTheUpperLimit(-1018),

  /// @hidden internal use only
  /// @brief license 参数错误，拒绝进房。通过 rtcRoom:onRoomStateChanged:withUid:state:extraInfo:{@link #ByteRTCRoomDelegate#rtcRoom:onRoomStateChanged:withUid:state:extraInfo} 回调。
  ///
  ByteRTCRoomStateChangeReasonLicenseParameterError(-1019),

  /// @hidden internal use only
  /// @brief license 证书路径错误。通过 rtcRoom:onRoomStateChanged:withUid:state:extraInfo:{@link #ByteRTCRoomDelegate#rtcRoom:onRoomStateChanged:withUid:state:extraInfo} 回调。
  ///
  ByteRTCRoomStateChangeReasonLicenseFilePathError(-1020),

  /// @hidden internal use only
  /// @brief license 证书不合法。通过 rtcRoom:onRoomStateChanged:withUid:state:extraInfo:{@link #ByteRTCRoomDelegate#rtcRoom:onRoomStateChanged:withUid:state:extraInfo} 回调。
  ///
  ByteRTCRoomStateChangeReasonLicenseIllegal(-1021),

  /// @hidden internal use only
  /// @brief license 证书已经过期，拒绝进房。通过 rtcRoom:onRoomStateChanged:withUid:state:extraInfo:{@link #ByteRTCRoomDelegate#rtcRoom:onRoomStateChanged:withUid:state:extraInfo} 回调。
  ///
  ByteRTCRoomStateChangeReasonLicenseExpired(-1022),

  /// @hidden internal use only
  /// @brief license 证书内容不匹配。通过 rtcRoom:onRoomStateChanged:withUid:state:extraInfo:{@link #ByteRTCRoomDelegate#rtcRoom:onRoomStateChanged:withUid:state:extraInfo} 回调。
  ///
  ByteRTCRoomStateChangeReasonLicenseInformationNotMatch(-1023),

  /// @hidden internal use only
  /// @brief license 当前证书与缓存证书不匹配。通过 rtcRoom:onRoomStateChanged:withUid:state:extraInfo:{@link #ByteRTCRoomDelegate#rtcRoom:onRoomStateChanged:withUid:state:extraInfo} 回调。
  ///
  ByteRTCRoomStateChangeReasonLicenseNotMatchWithCache(-1024),

  /// @hidden internal use only
  /// @brief license 计费方法没有加载成功。可能是因为 license 相关插件未正确集成。通过 rtcRoom:onRoomStateChanged:withUid:state:extraInfo:{@link #ByteRTCRoomDelegate#rtcRoom:onRoomStateChanged:withUid:state:extraInfo}  回调。
  ///
  ByteRTCRoomStateChangeReasonLicenseFunctionNotFound(-1027),

  /// @brief 服务端异常状态导致退出房间。通过 rtcRoom:onRoomStateChanged:withUid:state:extraInfo:{@link #ByteRTCRoomDelegate#rtcRoom:onRoomStateChanged:withUid:state:extraInfo} 回调。 <br>
  ///        SDK 与信令服务器断开，并不再自动重连，可联系技术支持。
  ///
  ByteRTCRoomStateChangeReasonStateAbnormalServerStatus(-1084),

  /// @brief 加入房间错误。通过 rtcRoom:onRoomStateChanged:withUid:state:extraInfo:{@link #ByteRTCRoomDelegate#rtcRoom:onRoomStateChanged:withUid:state:extraInfo} 回调。 <br>
  ///        进房时发生未知错误导致加入房间失败。需要用户重新加入房间。
  ///
  ByteRTCRoomStateChangeReasonUnknown(-1001);

  final dynamic $value;
  const ByteRTCRoomStateChangeReason([this.$value]);
}

enum ByteRTCMixedStreamVideoCodecType {
  /// @brief H.264 格式，默认值。
  ///
  ByteRTCMixedStreamVideoCodecTypeH264(0),

  /// @brief ByteVC1 格式。
  ///
  ByteRTCMixedStreamVideoCodecTypeByteVC1(1);

  final dynamic $value;
  const ByteRTCMixedStreamVideoCodecType([this.$value]);
}

class ByteRTCSubscribeConfig extends NativeClass {
  static const _$namespace = r'ByteRTCSubscribeConfig';
  static get codegen_$namespace => _$namespace;

  ByteRTCSubscribeConfig([NativeClassOptions? options])
      : super(options == null
            ? const NativeClassOptions(
                [],
                className: _$namespace,
                instanceType: InstanceType.manual,
                bridgeKey: 'com.volcengine.rtc.hybrid_runtime',
              )
            : NativeClassOptions.fromMap({
                ...options.toMap(),
                'bridgeKey': 'com.volcengine.rtc.hybrid_runtime',
              }));

  /// @brief 是否是屏幕流。 <br>
  ///         用户通过设置此参数，订阅该远端用户发布的屏幕共享流或非屏幕共享流。 YES 为订阅屏幕共享流，NO 为订阅非屏幕共享流，默认值为 YES。
  FutureOr<BOOL?> get isScreen async {
    return await sendInstanceGet<BOOL?>("isScreen");
  }

  set isScreen(FutureOr<BOOL?> value) {
    sendInstanceSet("isScreen", value);
  }

  /// @brief 是否订阅视频。 <br>
  ///         用户通过设置此参数，选择是否订阅远端流中的视频。 YES 为订阅视频，NO 为不订阅视频，默认值为 YES 。
  FutureOr<BOOL?> get subscribeVideo async {
    return await sendInstanceGet<BOOL?>("subscribeVideo");
  }

  set subscribeVideo(FutureOr<BOOL?> value) {
    sendInstanceSet("subscribeVideo", value);
  }

  /// @brief 是否订阅音频。 <br>
  ///         用户通过设置此参数，选择是否订阅远端流中的音频。YES 为订阅音频，NO 为不订阅音频，默认值为 YES 。
  FutureOr<BOOL?> get subscribeAudio async {
    return await sendInstanceGet<BOOL?>("subscribeAudio");
  }

  set subscribeAudio(FutureOr<BOOL?> value) {
    sendInstanceSet("subscribeAudio", value);
  }

  /// @brief 订阅的视频流分辨率下标。 <br>
  ///         用户可以通过调用 setVideoEncoderConfig:{@link #ByteRTCEngine#setVideoEncoderConfig} 方法发布多个不同分辨率的视频。因此订阅流时，需要指定订阅的具体分辨率。此参数即用于指定需订阅的分辨率的下标，默认值为 0 。
  FutureOr<NSInteger?> get videoIndex async {
    return await sendInstanceGet<NSInteger?>("videoIndex");
  }

  set videoIndex(FutureOr<NSInteger?> value) {
    sendInstanceSet("videoIndex", value);
  }

  /// @brief 订阅的视频流时域分层，默认值为 0。
  FutureOr<NSInteger?> get svcLayer async {
    return await sendInstanceGet<NSInteger?>("svcLayer");
  }

  set svcLayer(FutureOr<NSInteger?> value) {
    sendInstanceSet("svcLayer", value);
  }

  /// @brief 订阅的宽度信息，单位：px，默认值为 0。
  FutureOr<NSInteger?> get width async {
    return await sendInstanceGet<NSInteger?>("width");
  }

  set width(FutureOr<NSInteger?> value) {
    sendInstanceSet("width", value);
  }

  /// @brief 订阅的高度信息，单位：px， 默认值为 0。
  FutureOr<NSInteger?> get height async {
    return await sendInstanceGet<NSInteger?>("height");
  }

  set height(FutureOr<NSInteger?> value) {
    sendInstanceSet("height", value);
  }

  /// @hidden for internal use only
  FutureOr<NSInteger?> get subVideoIndex async {
    return await sendInstanceGet<NSInteger?>("subVideoIndex");
  }

  set subVideoIndex(FutureOr<NSInteger?> value) {
    sendInstanceSet("subVideoIndex", value);
  }

  /// @brief 期望订阅的最高帧率，单位：fps，默认值为 0，设为大于 0 的值时开始生效。 <br>
  ///        如果发布端发布帧率 > 订阅端订阅的帧率，下行媒体服务器 SVC 丢帧，订阅端收到通过此接口设置的帧率；如果发布端发布帧率 < 订阅端订阅的帧率，则订阅端只能收到发布的帧率。<br>
  ///        仅码流支持 SVC 分级编码特性时方可生效。
  FutureOr<NSInteger?> get framerate async {
    return await sendInstanceGet<NSInteger?>("framerate");
  }

  set framerate(FutureOr<NSInteger?> value) {
    sendInstanceSet("framerate", value);
  }
}

enum ByteRTCSetRoomExtraInfoResult {
  /// @brief 设置房间附加信息成功
  ///
  ByteRTCSetRoomExtraInfoResultSuccess(0),

  /// @brief 设置失败，尚未加入房间
  ///
  ByteRTCSetRoomExtraInfoResultNotJoinRoom(-1),

  /// @brief 设置失败，key 指针为空
  ///
  ByteRTCSetRoomExtraInfoResultKeyIsNull(-2),

  /// @brief 设置失败，value 指针为空
  ///
  ByteRTCSetRoomExtraInfoResultValueIsNull(-3),

  /// @brief 设置失败，未知错误
  ///
  ByteRTCSetRoomExtraInfoResultUnknow(-99),

  /// @brief 设置失败，key 长度为 0
  ///
  ByteRTCSetRoomExtraInfoResultKeyIsEmpty(-400),

  /// @brief 调用 `setRoomExtraInfo` 过于频繁，建议不超过 10 次/秒。
  ///
  ByteRTCSetRoomExtraInfoResultTooOften(-406),

  /// @brief 设置失败，用户已调用 `setUserVisibility` 将自身设为隐身状态。
  ///
  ByteRTCSetRoomExtraInfoResultSilentUser(-412),

  /// @brief 设置失败，Key 长度超过 10 字节
  ///
  ByteRTCSetRoomExtraInfoResultKeyTooLong(-413),

  /// @brief 设置失败，value 长度超过 128 字节
  ///
  ByteRTCSetRoomExtraInfoResultValueTooLong(-414),

  /// @brief 设置失败，服务器错误
  ///
  ByteRTCSetRoomExtraInfoResultServerError(-500);

  final dynamic $value;
  const ByteRTCSetRoomExtraInfoResult([this.$value]);
}

class ByteRTCStandardPitchInfo extends NativeClass {
  static const _$namespace = r'ByteRTCStandardPitchInfo';
  static get codegen_$namespace => _$namespace;

  ByteRTCStandardPitchInfo([NativeClassOptions? options])
      : super(options == null
            ? const NativeClassOptions(
                [],
                className: _$namespace,
                instanceType: InstanceType.manual,
                bridgeKey: 'com.volcengine.rtc.hybrid_runtime',
              )
            : NativeClassOptions.fromMap({
                ...options.toMap(),
                'bridgeKey': 'com.volcengine.rtc.hybrid_runtime',
              }));

  /// @brief 开始时间，单位 ms。
  ///
  FutureOr<int?> get startTime async {
    return await sendInstanceGet<int?>("startTime");
  }

  set startTime(FutureOr<int?> value) {
    sendInstanceSet("startTime", value);
  }

  /// @brief 持续时间，单位 ms。
  ///
  FutureOr<int?> get duration async {
    return await sendInstanceGet<int?>("duration");
  }

  set duration(FutureOr<int?> value) {
    sendInstanceSet("duration", value);
  }

  /// @brief 音高。
  ///
  FutureOr<int?> get pitch async {
    return await sendInstanceGet<int?>("pitch");
  }

  set pitch(FutureOr<int?> value) {
    sendInstanceSet("pitch", value);
  }
}

class ByteRTCRemoteVideoStats extends NativeClass {
  static const _$namespace = r'ByteRTCRemoteVideoStats';
  static get codegen_$namespace => _$namespace;

  ByteRTCRemoteVideoStats([NativeClassOptions? options])
      : super(options == null
            ? const NativeClassOptions(
                [],
                className: _$namespace,
                instanceType: InstanceType.manual,
                bridgeKey: 'com.volcengine.rtc.hybrid_runtime',
              )
            : NativeClassOptions.fromMap({
                ...options.toMap(),
                'bridgeKey': 'com.volcengine.rtc.hybrid_runtime',
              }));

  /// @brief 远端视频宽度。
  FutureOr<NSInteger?> get width async {
    return await sendInstanceGet<NSInteger?>("width");
  }

  set width(FutureOr<NSInteger?> value) {
    sendInstanceSet("width", value);
  }

  /// @brief 远端视频高度。
  FutureOr<NSInteger?> get height async {
    return await sendInstanceGet<NSInteger?>("height");
  }

  set height(FutureOr<NSInteger?> value) {
    sendInstanceSet("height", value);
  }

  /// @brief 远端视频丢包率。统计周期内的视频下行丢包率，取值范围为 [0, 1] 。
  FutureOr<float?> get videoLossRate async {
    return await sendInstanceGet<float?>("videoLossRate");
  }

  set videoLossRate(FutureOr<float?> value) {
    sendInstanceSet("videoLossRate", value);
  }

  /// @brief 接收码率。统计周期内的视频接收码率，单位为 kbps 。
  FutureOr<float?> get receivedKBitrate async {
    return await sendInstanceGet<float?>("receivedKBitrate");
  }

  set receivedKBitrate(FutureOr<float?> value) {
    sendInstanceSet("receivedKBitrate", value);
  }

  /// @brief 远端视频接收帧率。
  FutureOr<NSInteger?> get receivedFrameRate async {
    return await sendInstanceGet<NSInteger?>("receivedFrameRate");
  }

  set receivedFrameRate(FutureOr<NSInteger?> value) {
    sendInstanceSet("receivedFrameRate", value);
  }

  /// @brief 远端视频解码输出帧率。
  FutureOr<NSInteger?> get decoderOutputFrameRate async {
    return await sendInstanceGet<NSInteger?>("decoderOutputFrameRate");
  }

  set decoderOutputFrameRate(FutureOr<NSInteger?> value) {
    sendInstanceSet("decoderOutputFrameRate", value);
  }

  /// @brief 远端视频渲染输出帧率。
  FutureOr<NSInteger?> get renderOutputFrameRate async {
    return await sendInstanceGet<NSInteger?>("renderOutputFrameRate");
  }

  set renderOutputFrameRate(FutureOr<NSInteger?> value) {
    sendInstanceSet("renderOutputFrameRate", value);
  }

  /// @brief 远端视频卡顿次数。
  FutureOr<NSInteger?> get stallCount async {
    return await sendInstanceGet<NSInteger?>("stallCount");
  }

  set stallCount(FutureOr<NSInteger?> value) {
    sendInstanceSet("stallCount", value);
  }

  /// @brief 远端视频卡顿时长，单位为 ms 。
  FutureOr<NSInteger?> get stallDuration async {
    return await sendInstanceGet<NSInteger?>("stallDuration");
  }

  set stallDuration(FutureOr<NSInteger?> value) {
    sendInstanceSet("stallDuration", value);
  }

  /// @brief 用户体验级别的端到端延时。从发送端采集完成编码开始到接收端解码完成渲染开始的延时，单位为 ms 。
  FutureOr<NSInteger?> get e2eDelay async {
    return await sendInstanceGet<NSInteger?>("e2eDelay");
  }

  set e2eDelay(FutureOr<NSInteger?> value) {
    sendInstanceSet("e2eDelay", value);
  }

  /// @brief 远端视频流是否是屏幕共享流。你可以知道当前数据来自主流还是屏幕流。
  FutureOr<BOOL?> get isScreen async {
    return await sendInstanceGet<BOOL?>("isScreen");
  }

  set isScreen(FutureOr<BOOL?> value) {
    sendInstanceSet("isScreen", value);
  }

  /// @brief 统计间隔，此次统计周期的间隔，单位为 ms 。 <br>
  ///        此字段用于设置回调的统计周期，目前设置为 2s 。
  FutureOr<NSInteger?> get statsInterval async {
    return await sendInstanceGet<NSInteger?>("statsInterval");
  }

  set statsInterval(FutureOr<NSInteger?> value) {
    sendInstanceSet("statsInterval", value);
  }

  /// @brief 往返时延。单位为 ms 。
  FutureOr<NSInteger?> get rtt async {
    return await sendInstanceGet<NSInteger?>("rtt");
  }

  set rtt(FutureOr<NSInteger?> value) {
    sendInstanceSet("rtt", value);
  }

  /// @brief 远端用户在进房后发生视频卡顿的累计时长占视频总有效时长的百分比（\%）。视频有效时长是指远端用户进房发布视频流后，除停止发送视频流和禁用视频模块之外的视频时长。
  FutureOr<NSInteger?> get frozenRate async {
    return await sendInstanceGet<NSInteger?>("frozenRate");
  }

  set frozenRate(FutureOr<NSInteger?> value) {
    sendInstanceSet("frozenRate", value);
  }

  /// @brief 编码类型。参看 ByteRTCVideoCodecType{@link #ByteRTCVideoCodecType} 类型。
  FutureOr<ByteRTCVideoCodecType?> get codecType async {
    try {
      final result = await sendInstanceGet<ByteRTCVideoCodecType?>("codecType");
      if (result == null) {
        return null;
      }
      return ByteRTCVideoCodecType.values
          .firstWhere((element) => element == result);
    } catch (e) {
      return null;
    }
  }

  set codecType(FutureOr<ByteRTCVideoCodecType?> value) {
    sendInstanceSet("codecType", value);
  }

  /// @brief SDK 订阅的远端视频流的分辨率下标。
  FutureOr<NSInteger?> get videoIndex async {
    return await sendInstanceGet<NSInteger?>("videoIndex");
  }

  set videoIndex(FutureOr<NSInteger?> value) {
    sendInstanceSet("videoIndex", value);
  }

  /// @brief 视频下行网络抖动，单位为 ms。
  FutureOr<NSInteger?> get jitter async {
    return await sendInstanceGet<NSInteger?>("jitter");
  }

  set jitter(FutureOr<NSInteger?> value) {
    sendInstanceSet("jitter", value);
  }

  /// @hidden for internal use only
  /// @brief 远端视频超分模式，参看 ByteRTCVideoSuperResolutionMode{@link #ByteRTCVideoSuperResolutionMode}。
  FutureOr<ByteRTCVideoSuperResolutionMode?> get superResolutionMode async {
    try {
      final result = await sendInstanceGet<ByteRTCVideoSuperResolutionMode?>(
          "superResolutionMode");
      if (result == null) {
        return null;
      }
      return ByteRTCVideoSuperResolutionMode.values
          .firstWhere((element) => element == result);
    } catch (e) {
      return null;
    }
  }

  set superResolutionMode(FutureOr<ByteRTCVideoSuperResolutionMode?> value) {
    sendInstanceSet("superResolutionMode", value);
  }

  /// @brief 用户体验级别的端到端延时。从发送端开始采集到接收端渲染完成的延时，单位为 ms 。
  FutureOr<NSInteger?> get capToRenderDelay async {
    return await sendInstanceGet<NSInteger?>("capToRenderDelay");
  }

  set capToRenderDelay(FutureOr<NSInteger?> value) {
    sendInstanceSet("capToRenderDelay", value);
  }

  /// @brief 音画同步差异，单位为 ms 。
  FutureOr<NSInteger?> get avSyncDiffMs async {
    return await sendInstanceGet<NSInteger?>("avSyncDiffMs");
  }

  set avSyncDiffMs(FutureOr<NSInteger?> value) {
    sendInstanceSet("avSyncDiffMs", value);
  }

  /// @brief 视频解码平均耗时，单位 ms。
  FutureOr<NSInteger?> get codecElapsePerFrame async {
    return await sendInstanceGet<NSInteger?>("codecElapsePerFrame");
  }

  set codecElapsePerFrame(FutureOr<NSInteger?> value) {
    sendInstanceSet("codecElapsePerFrame", value);
  }
}

enum ByteRTCBackgroundMode {
  /// @brief 无
  ///
  ByteRTCBackgroundModeNone(0),

  /// @brief 虚化
  ///
  ByteRTCBackgroundModeBlur(1),

  /// @brief 背景 1
  ///
  ByteRTCBackgroundModeA(2),

  /// @brief 背景 2
  ///
  ByteRTCBackgroundModeB(3);

  final dynamic $value;
  const ByteRTCBackgroundMode([this.$value]);
}

class ByteRTCAudioEffectPlayerConfig extends NativeClass {
  static const _$namespace = r'ByteRTCAudioEffectPlayerConfig';
  static get codegen_$namespace => _$namespace;

  ByteRTCAudioEffectPlayerConfig([NativeClassOptions? options])
      : super(options == null
            ? const NativeClassOptions(
                [],
                className: _$namespace,
                instanceType: InstanceType.manual,
                bridgeKey: 'com.volcengine.rtc.hybrid_runtime',
              )
            : NativeClassOptions.fromMap({
                ...options.toMap(),
                'bridgeKey': 'com.volcengine.rtc.hybrid_runtime',
              }));

  /// @detail keytype
  /// @brief 混音播放类型，详见 ByteRTCAudioMixingType{@link #ByteRTCAudioMixingType}
  FutureOr<ByteRTCAudioMixingType?> get type async {
    try {
      final result = await sendInstanceGet<ByteRTCAudioMixingType?>("type");
      if (result == null) {
        return null;
      }
      return ByteRTCAudioMixingType.values
          .firstWhere((element) => element == result);
    } catch (e) {
      return null;
    }
  }

  set type(FutureOr<ByteRTCAudioMixingType?> value) {
    sendInstanceSet("type", value);
  }

  /// @brief 与音乐文件原始音调相比的升高/降低值，取值范围为 `[-12，12]`，默认值为 0。每相邻两个值的音高距离相差半音，正值表示升调，负值表示降调。
  FutureOr<NSInteger?> get pitch async {
    return await sendInstanceGet<NSInteger?>("pitch");
  }

  set pitch(FutureOr<NSInteger?> value) {
    sendInstanceSet("pitch", value);
  }

  /// @brief 混音播放次数 <br>
  ///       - play_count <= 0: 无限循环
  ///       - play_count == 1: 播放一次（默认）
  ///       - play_count > 1: 播放 play_count 次
  FutureOr<NSInteger?> get playCount async {
    return await sendInstanceGet<NSInteger?>("playCount");
  }

  set playCount(FutureOr<NSInteger?> value) {
    sendInstanceSet("playCount", value);
  }

  /// @brief 混音起始位置。默认值为 0，单位为毫秒。
  FutureOr<NSInteger?> get startPos async {
    return await sendInstanceGet<NSInteger?>("startPos");
  }

  set startPos(FutureOr<NSInteger?> value) {
    sendInstanceSet("startPos", value);
  }
}

enum ByteRTCAudioMixingState {
  /// @brief 混音已加载
  ///
  ByteRTCAudioMixingStatePreloaded(0),

  /// @brief 混音正在播放
  ///
  ByteRTCAudioMixingStatePlaying(1),

  /// @brief 混音暂停
  ///
  ByteRTCAudioMixingStatePaused(2),

  /// @brief 混音停止
  ///
  ByteRTCAudioMixingStateStopped(3),

  /// @brief 混音播放失败
  ///
  ByteRTCAudioMixingStateFailed(4),

  /// @brief 混音播放结束
  ///
  ByteRTCAudioMixingStateFinished(5),

  /// @brief 准备 PCM 混音
  ///
  ByteRTCAudioMixingStatePCMEnabled(6),

  /// @brief PCM 混音播放结束
  ///
  ByteRTCAudioMixingStatePCMDisabled(7);

  final dynamic $value;
  const ByteRTCAudioMixingState([this.$value]);
}

enum ByteRTCAudioTrackType {
  /// @brief 播放原唱。
  ///
  ByteRTCAudioTrackTypeOriginal(1),

  /// @brief 播放伴唱。
  ///
  ByteRTCAudioTrackTypeAccompy(2);

  final dynamic $value;
  const ByteRTCAudioTrackType([this.$value]);
}

enum ByteRTCLocalProxyType {
  /// @brief Socks5 代理。选用此代理服务器，需满足 Socks5 协议标准文档的要求。
  ///
  ByteRTCLocalProxyTypeSocks5(1),

  /// @brief Http 隧道代理。
  ///
  ByteRTCLocalProxyTypeHttpTunnel(2);

  final dynamic $value;
  const ByteRTCLocalProxyType([this.$value]);
}

class ByteRTCVideoEncoderConfig extends NativeClass {
  static const _$namespace = r'ByteRTCVideoEncoderConfig';
  static get codegen_$namespace => _$namespace;

  ByteRTCVideoEncoderConfig([NativeClassOptions? options])
      : super(options == null
            ? const NativeClassOptions(
                [],
                className: _$namespace,
                instanceType: InstanceType.manual,
                bridgeKey: 'com.volcengine.rtc.hybrid_runtime',
              )
            : NativeClassOptions.fromMap({
                ...options.toMap(),
                'bridgeKey': 'com.volcengine.rtc.hybrid_runtime',
              }));

  /// @brief 视频宽度，单位：px
  FutureOr<NSInteger?> get width async {
    return await sendInstanceGet<NSInteger?>("width");
  }

  set width(FutureOr<NSInteger?> value) {
    sendInstanceSet("width", value);
  }

  /// @brief 视频高度，单位：px
  FutureOr<NSInteger?> get height async {
    return await sendInstanceGet<NSInteger?>("height");
  }

  set height(FutureOr<NSInteger?> value) {
    sendInstanceSet("height", value);
  }

  /// @brief 视频帧率，单位：fps
  FutureOr<NSInteger?> get frameRate async {
    return await sendInstanceGet<NSInteger?>("frameRate");
  }

  set frameRate(FutureOr<NSInteger?> value) {
    sendInstanceSet("frameRate", value);
  }

  /// @brief 最大编码码率，使用 SDK 内部采集时可选设置，自定义采集时必须设置，单位：kbps。 <br>
  ///        设为 -1 即适配码率模式，系统将根据输入的分辨率和帧率自动计算适用的码率。 <br>
  ///        设为 0 则不对视频流进行编码发送。 <br>
  ///        344 及以上版本，内部采集时默认值为 -1，344 以前版本无默认值，需手动设置。
  FutureOr<NSInteger?> get maxBitrate async {
    return await sendInstanceGet<NSInteger?>("maxBitrate");
  }

  set maxBitrate(FutureOr<NSInteger?> value) {
    sendInstanceSet("maxBitrate", value);
  }

  /// @brief 视频最小编码码率, 单位 kbps。编码码率不会低于 `minBitrate`。 <br>
  ///        默认值为 `0`。 <br>
  ///        范围：[0, maxBitrate)，当 `maxBitrate` < `minBitrate` 时，为适配码率模式。 <br>
  ///        以下情况，设置本参数无效： <br>
  ///        - 当 `maxBitrate` 为 `0` 时，不对视频流进行编码发送。
  ///        - 当 `maxBitrate` < `0` 时，适配码率模式。
  FutureOr<NSInteger?> get minBitrate async {
    return await sendInstanceGet<NSInteger?>("minBitrate");
  }

  set minBitrate(FutureOr<NSInteger?> value) {
    sendInstanceSet("minBitrate", value);
  }

  /// @brief 编码策略偏好，默认为帧率优先。参看 ByteRTCVideoEncoderPreference{@link #ByteRTCVideoEncoderPreference}。
  FutureOr<ByteRTCVideoEncoderPreference?> get encoderPreference async {
    try {
      final result = await sendInstanceGet<ByteRTCVideoEncoderPreference?>(
          "encoderPreference");
      if (result == null) {
        return null;
      }
      return ByteRTCVideoEncoderPreference.values
          .firstWhere((element) => element == result);
    } catch (e) {
      return null;
    }
  }

  set encoderPreference(FutureOr<ByteRTCVideoEncoderPreference?> value) {
    sendInstanceSet("encoderPreference", value);
  }
}

enum ByteRTCMessageConfig {
  /// @brief 低延时可靠有序消息
  ///
  ByteRTCMessageConfigReliableOrdered(0),

  /// @brief 超低延时有序消息
  ///
  ByteRTCMessageConfigUnreliableOrdered(1),

  /// @brief 超低延时无序消息
  ///
  ByteRTCMessageConfigUnreliableUnordered(2);

  final dynamic $value;
  const ByteRTCMessageConfig([this.$value]);
}

class ByteRTCNetworkQualityStats extends NativeClass {
  static const _$namespace = r'ByteRTCNetworkQualityStats';
  static get codegen_$namespace => _$namespace;

  ByteRTCNetworkQualityStats([NativeClassOptions? options])
      : super(options == null
            ? const NativeClassOptions(
                [],
                className: _$namespace,
                instanceType: InstanceType.manual,
                bridgeKey: 'com.volcengine.rtc.hybrid_runtime',
              )
            : NativeClassOptions.fromMap({
                ...options.toMap(),
                'bridgeKey': 'com.volcengine.rtc.hybrid_runtime',
              }));

  /// @brief 用户 ID
  FutureOr<NSString?> get uid async {
    return await sendInstanceGet<NSString?>("uid");
  }

  set uid(FutureOr<NSString?> value) {
    sendInstanceSet("uid", value);
  }

  /// @brief 本端的上行/下行的丢包率，范围 [0.0,1.0] <br>
  ///        当 `uid` 为本地用户时，代表发布流的上行丢包率。 <br>
  ///        当 `uid` 为远端用户时，代表接收所有订阅流的下行丢包率。
  FutureOr<double?> get lossRatio async {
    return await sendInstanceGet<double?>("lossRatio");
  }

  set lossRatio(FutureOr<double?> value) {
    sendInstanceSet("lossRatio", value);
  }

  /// @brief 当 `uid` 为本地用户时有效，客户端到服务端的往返延时（RTT），单位：ms
  FutureOr<int?> get rtt async {
    return await sendInstanceGet<int?>("rtt");
  }

  set rtt(FutureOr<int?> value) {
    sendInstanceSet("rtt", value);
  }

  /// @brief 本端的音视频 RTP 包 2 秒内的平均传输速率，单位：bps <br>
  ///        当 `uid` 为本地用户时，代表发送速率。 <br>
  ///        当 `uid` 为远端用户时，代表所有订阅流的接收速率。
  FutureOr<int?> get totalBandwidth async {
    return await sendInstanceGet<int?>("totalBandwidth");
  }

  set totalBandwidth(FutureOr<int?> value) {
    sendInstanceSet("totalBandwidth", value);
  }

  /// @brief 上行网络质量评分，详见 ByteRTCNetworkQuality{@link #ByteRTCNetworkQuality}。
  FutureOr<ByteRTCNetworkQuality?> get txQuality async {
    try {
      final result = await sendInstanceGet<ByteRTCNetworkQuality?>("txQuality");
      if (result == null) {
        return null;
      }
      return ByteRTCNetworkQuality.values
          .firstWhere((element) => element == result);
    } catch (e) {
      return null;
    }
  }

  set txQuality(FutureOr<ByteRTCNetworkQuality?> value) {
    sendInstanceSet("txQuality", value);
  }

  /// @brief 下行网络质量评分，详见 ByteRTCNetworkQuality{@link #ByteRTCNetworkQuality}。
  FutureOr<ByteRTCNetworkQuality?> get rxQuality async {
    try {
      final result = await sendInstanceGet<ByteRTCNetworkQuality?>("rxQuality");
      if (result == null) {
        return null;
      }
      return ByteRTCNetworkQuality.values
          .firstWhere((element) => element == result);
    } catch (e) {
      return null;
    }
  }

  set rxQuality(FutureOr<ByteRTCNetworkQuality?> value) {
    sendInstanceSet("rxQuality", value);
  }
}

enum ByteRTCRoomState {
  /// @brief 加入房间成功
  ///
  ByteRTCRoomStateJoinSuccess(0),

  /// @brief 加入房间失败
  ///
  ByteRTCRoomStateJoinFailed(1),

  /// @brief 离开房间
  ///
  ByteRTCRoomStateLeft(2);

  final dynamic $value;
  const ByteRTCRoomState([this.$value]);
}

enum ByteRTCVideoSuperResolutionMode {
  /// @brief 关闭超分。
  ///
  ByteRTCVideoSuperResolutionModeOff(0),

  /// @brief 开启超分。
  ///
  ByteRTCVideoSuperResolutionModeOn(1);

  final dynamic $value;
  const ByteRTCVideoSuperResolutionMode([this.$value]);
}

enum ByteRTCVideoSimulcastStreamType {
  /// @brief 弱流，最小分辨率的流。
  ///
  ByteRTCVideoSimulcastStreamTypeWeak(0),

  /// @brief 小流
  ///
  ByteRTCVideoSimulcastStreamTypeLow(1),

  /// @brief 中流
  ///
  ByteRTCVideoSimulcastStreamTypeMid(2),

  /// @brief 大流
  ///
  ByteRTCVideoSimulcastStreamTypeHigh(3);

  final dynamic $value;
  const ByteRTCVideoSimulcastStreamType([this.$value]);
}

enum ByteRTCUserOnlineStatus {
  /// @brief 对端用户离线 <br>
  ///        对端用户已经调用 `logout`，或者没有调用 login:uid:{@link #ByteRTCEngine#login:uid} 进行登录
  ///
  ByteRTCUserOnlineStatusOffline(0),

  /// @brief 对端用户在线 <br>
  ///        对端用户调用 login:uid:{@link #ByteRTCEngine#login:uid} 登录，并且连接状态正常
  ///
  ByteRTCUserOnlineStatusOnline(1),

  /// @brief 无法获取对端用户在线状态 <br>
  ///        发生级联错误、对端用户在线状态异常时返回
  ///
  ByteRTCUserOnlineStatusUnreachable(2);

  final dynamic $value;
  const ByteRTCUserOnlineStatus([this.$value]);
}

enum ByteRTCVoiceChangerType {
  /// @brief 原声，不含特效
  ///
  ByteRTCVoiceChangerOriginal(0),

  /// @brief 巨人
  ///
  ByteRTCVoiceChangerGiant(1),

  /// @brief 花栗鼠
  ///
  ByteRTCVoiceChangerChipmunk(2),

  /// @brief 小黄人
  ///
  ByteRTCVoiceChangerMinionst(3),

  /// @brief 颤音
  ///
  ByteRTCVoiceChangerVibrato(4),

  /// @brief 机器人
  ///
  ByteRTCVoiceChangerRobot(5);

  final dynamic $value;
  const ByteRTCVoiceChangerType([this.$value]);
}

enum ByteRTCDivideModel {
  /// @brief 自研
  ///
  ByteRTCDivideModelDefault(0),

  /// @brief effect 分割模型
  ///
  ByteRTCDivideModelEffect(1);

  final dynamic $value;
  const ByteRTCDivideModel([this.$value]);
}

enum ByteRTCBandFrequency {
  /// @brief 中心频率为 31Hz 的频带。
  ///
  ByteRTCBandFrequency31(0),

  /// @brief 中心频率为 62Hz 的频带。
  ///
  ByteRTCBandFrequency62(1),

  /// @brief 中心频率为 125Hz 的频带。
  ///
  ByteRTCBandFrequency125(2),

  /// @brief 中心频率为 250Hz 的频带。
  ///
  ByteRTCBandFrequency250(3),

  /// @brief 中心频率为 500Hz 的频带。
  ///
  ByteRTCBandFrequency500(4),

  /// @brief 中心频率为 1kHz 的频带。
  ///
  ByteRTCBandFrequency1k(5),

  /// @brief 中心频率为 2kHz 的频带。
  ///
  ByteRTCBandFrequency2k(6),

  /// @brief 中心频率为 4kHz 的频带。
  ///
  ByteRTCBandFrequency4k(7),

  /// @brief 中心频率为 8kHz 的频带。
  ///
  ByteRTCBandFrequency8k(8),

  /// @brief 中心频率为 16kHz 的频带。
  ///
  ByteRTCBandFrequency16k(9);

  final dynamic $value;
  const ByteRTCBandFrequency([this.$value]);
}

enum ByteRTCPublishState {
  /// @brief 发布成功
  ///
  ByteRTCPublishStatePublish(0),

  /// @brief 发布失败
  ///
  ByteRTCPublishStateUnpublish(1);

  final dynamic $value;
  const ByteRTCPublishState([this.$value]);
}

enum ByteRTCAudioPlayType {
  /// @brief 仅本地播放。
  ///
  ByteRTCAudioPlayTypeLocal(0),

  /// @brief 仅远端播放。
  ///
  ByteRTCAudioPlayTypeRemote(1),

  /// @brief 本地、远端同时播放。
  ///
  ByteRTCAudioPlayTypeLocalAndRemote(2);

  final dynamic $value;
  const ByteRTCAudioPlayType([this.$value]);
}

enum ByteRTCLocalLogLevel {
  /// @brief 信息级别。
  ///
  ByteRTCLocalLogLevelInfo(0),

  /// @brief （默认值）警告级别。
  ///
  ByteRTCLocalLogLevelWarning(1),

  /// @brief 错误级别。
  ///
  ByteRTCLocalLogLevelError(2),

  /// @brief 关闭日志。
  ///
  ByteRTCLocalLogLevelNone(3);

  final dynamic $value;
  const ByteRTCLocalLogLevel([this.$value]);
}

enum ByteRTCAudioMixingType {
  /// @brief 仅本地播放
  ///
  ByteRTCAudioMixingTypePlayout(0),

  /// @brief 仅远端播放
  ///
  ByteRTCAudioMixingTypePublish(1),

  /// @brief 本地和远端同时播放
  ///
  ByteRTCAudioMixingTypePlayoutAndPublish(2);

  final dynamic $value;
  const ByteRTCAudioMixingType([this.$value]);
}

enum ByteRTCTranscodingAudioCodec {
  /// @detail keytype
  /// @brief AAC 格式。
  ///
  ByteRTCTranscodingAudioCodecAAC(0);

  final dynamic $value;
  const ByteRTCTranscodingAudioCodec([this.$value]);
}

enum ByteRTCFirstFramePlayState {
  /// @brief 播放中
  ///
  ByteRTCFirstFramePlayStatePlaying(0),

  /// @brief 播放成功
  ///
  ByteRTCFirstFramePlayStatePlay(1),

  /// @brief 播放失败
  ///
  ByteRTCFirstFramePlayStateEnd(2);

  final dynamic $value;
  const ByteRTCFirstFramePlayState([this.$value]);
}

enum ByteRTCVideoSinkMirrorType {
  /// @brief 开启镜像。
  ///
  ByteRTCVideoSinkMirrorTypeOn(1),

  /// @brief （默认值）不开启镜像。
  ///
  ByteRTCVideoSinkMirrorTypeOff(2);

  final dynamic $value;
  const ByteRTCVideoSinkMirrorType([this.$value]);
}

enum ByteRTCAudioPropertiesMode {
  /// @brief 仅包含本地麦克风采集的音频数据和本地屏幕音频采集数据。
  ///
  ByteRTCAudioPropertiesModeMicrohone(0),

  /// @brief 包含以下信息： <br>
  ///        - 本地麦克风采集的音频数据和本地屏幕音频采集数据；
  ///        - 本地混音的音频数据。
  ///
  ByteRTCAudioPropertiesModeAudioMixing(1);

  final dynamic $value;
  const ByteRTCAudioPropertiesMode([this.$value]);
}

class ByteRTCLocalVideoStats extends NativeClass {
  static const _$namespace = r'ByteRTCLocalVideoStats';
  static get codegen_$namespace => _$namespace;

  ByteRTCLocalVideoStats([NativeClassOptions? options])
      : super(options == null
            ? const NativeClassOptions(
                [],
                className: _$namespace,
                instanceType: InstanceType.manual,
                bridgeKey: 'com.volcengine.rtc.hybrid_runtime',
              )
            : NativeClassOptions.fromMap({
                ...options.toMap(),
                'bridgeKey': 'com.volcengine.rtc.hybrid_runtime',
              }));

  /// @brief 发送码率。此次统计周期内实际发送的分辨率最大的视频流的发送码率，单位为 Kbps
  FutureOr<float?> get sentKBitrate async {
    return await sendInstanceGet<float?>("sentKBitrate");
  }

  set sentKBitrate(FutureOr<float?> value) {
    sendInstanceSet("sentKBitrate", value);
  }

  /// @brief 采集帧率。此次统计周期内的视频采集帧率，单位为 fps 。
  FutureOr<NSInteger?> get inputFrameRate async {
    return await sendInstanceGet<NSInteger?>("inputFrameRate");
  }

  set inputFrameRate(FutureOr<NSInteger?> value) {
    sendInstanceSet("inputFrameRate", value);
  }

  /// @brief 发送帧率。此次统计周期内实际发送的分辨率最大的视频流的视频发送帧率，单位为 fps 。
  FutureOr<NSInteger?> get sentFrameRate async {
    return await sendInstanceGet<NSInteger?>("sentFrameRate");
  }

  set sentFrameRate(FutureOr<NSInteger?> value) {
    sendInstanceSet("sentFrameRate", value);
  }

  /// @brief 编码器输出帧率。当前编码器在此次统计周期内实际发送的分辨率最大的视频流的输出帧率，单位为 fps 。
  FutureOr<NSInteger?> get encoderOutputFrameRate async {
    return await sendInstanceGet<NSInteger?>("encoderOutputFrameRate");
  }

  set encoderOutputFrameRate(FutureOr<NSInteger?> value) {
    sendInstanceSet("encoderOutputFrameRate", value);
  }

  /// @brief 本地渲染帧率。此次统计周期内的本地视频渲染帧率，单位为 fps 。
  FutureOr<NSInteger?> get rendererOutputFrameRate async {
    return await sendInstanceGet<NSInteger?>("rendererOutputFrameRate");
  }

  set rendererOutputFrameRate(FutureOr<NSInteger?> value) {
    sendInstanceSet("rendererOutputFrameRate", value);
  }

  /// @brief 统计间隔，单位为 ms 。 <br>
  ///        此字段用于设置回调的统计周期，默认设置为 2s 。
  FutureOr<NSInteger?> get statsInterval async {
    return await sendInstanceGet<NSInteger?>("statsInterval");
  }

  set statsInterval(FutureOr<NSInteger?> value) {
    sendInstanceSet("statsInterval", value);
  }

  /// @brief 视频丢包率。统计周期内的视频上行丢包率，取值范围为 [0, 1] 。
  FutureOr<float?> get videoLossRate async {
    return await sendInstanceGet<float?>("videoLossRate");
  }

  set videoLossRate(FutureOr<float?> value) {
    sendInstanceSet("videoLossRate", value);
  }

  /// @brief 往返时延。单位为 ms 。
  FutureOr<NSInteger?> get rtt async {
    return await sendInstanceGet<NSInteger?>("rtt");
  }

  set rtt(FutureOr<NSInteger?> value) {
    sendInstanceSet("rtt", value);
  }

  /// @brief 视频编码码率。此次统计周期内的实际发送的分辨率最大的视频流视频编码码率，单位为 Kbps 。
  FutureOr<NSInteger?> get encodedBitrate async {
    return await sendInstanceGet<NSInteger?>("encodedBitrate");
  }

  set encodedBitrate(FutureOr<NSInteger?> value) {
    sendInstanceSet("encodedBitrate", value);
  }

  /// @brief 实际发送的分辨率最大的视频流的视频编码宽度，单位为 px 。
  FutureOr<NSInteger?> get encodedFrameWidth async {
    return await sendInstanceGet<NSInteger?>("encodedFrameWidth");
  }

  set encodedFrameWidth(FutureOr<NSInteger?> value) {
    sendInstanceSet("encodedFrameWidth", value);
  }

  /// @brief 实际发送的分辨率最大的视频流的视频编码高度，单位为 px 。
  FutureOr<NSInteger?> get encodedFrameHeight async {
    return await sendInstanceGet<NSInteger?>("encodedFrameHeight");
  }

  set encodedFrameHeight(FutureOr<NSInteger?> value) {
    sendInstanceSet("encodedFrameHeight", value);
  }

  /// @brief 此次统计周期内实际发送的分辨率最大的视频流的发送的视频帧总数。
  FutureOr<NSInteger?> get encodedFrameCount async {
    return await sendInstanceGet<NSInteger?>("encodedFrameCount");
  }

  set encodedFrameCount(FutureOr<NSInteger?> value) {
    sendInstanceSet("encodedFrameCount", value);
  }

  /// @brief 编码类型。参看 ByteRTCVideoCodecType{@link #ByteRTCVideoCodecType} 类型。
  FutureOr<ByteRTCVideoCodecType?> get codecType async {
    try {
      final result = await sendInstanceGet<ByteRTCVideoCodecType?>("codecType");
      if (result == null) {
        return null;
      }
      return ByteRTCVideoCodecType.values
          .firstWhere((element) => element == result);
    } catch (e) {
      return null;
    }
  }

  set codecType(FutureOr<ByteRTCVideoCodecType?> value) {
    sendInstanceSet("codecType", value);
  }

  /// @brief 所属用户的媒体流是否为屏幕流。你可以知道当前统计数据来自主流还是屏幕流。
  FutureOr<BOOL?> get isScreen async {
    return await sendInstanceGet<BOOL?>("isScreen");
  }

  set isScreen(FutureOr<BOOL?> value) {
    sendInstanceSet("isScreen", value);
  }

  /// @brief 视频上行网络抖动，单位为 ms。
  FutureOr<NSInteger?> get jitter async {
    return await sendInstanceGet<NSInteger?>("jitter");
  }

  set jitter(FutureOr<NSInteger?> value) {
    sendInstanceSet("jitter", value);
  }

  /// @hidden(macOS)
  /// @brief 视频降噪模式。具体参看 ByteRTCVideoDenoiseMode{@link #ByteRTCVideoDenoiseMode} 。
  FutureOr<ByteRTCVideoDenoiseMode?> get videoDenoiseMode async {
    try {
      final result =
          await sendInstanceGet<ByteRTCVideoDenoiseMode?>("videoDenoiseMode");
      if (result == null) {
        return null;
      }
      return ByteRTCVideoDenoiseMode.values
          .firstWhere((element) => element == result);
    } catch (e) {
      return null;
    }
  }

  set videoDenoiseMode(FutureOr<ByteRTCVideoDenoiseMode?> value) {
    sendInstanceSet("videoDenoiseMode", value);
  }
}

enum ByteRTCSubscribeFallbackOption {
  /// @brief 下行网络不佳或设备性能不足时，不对音视频流作回退处理。默认设置。
  ///
  ByteRTCSubscribeFallbackOptionDisabled(0),

  /// @brief 下行网络不佳或设备性能不足时，对视频流做降级处理，具体降级规则参看[音视频流回退](#70137)。 <br>
  ///        该设置仅对发布端调用 `enableSimulcastMode:` 开启发送多路流功能的情况生效。
  ///
  ByteRTCSubscribeFallbackOptionVideoStreamLow(1),

  /// @brief 下行网络不佳或设备性能不足时，先对视频流做回退处理。当网络状况不满足接收弱流时，则自动取消接收视频，仅接收音频。 <br>
  ///        该设置仅对发布端调用 `enableSimulcastMode:` 开启发送多路流功能的情况生效。
  ///
  ByteRTCSubscribeFallbackOptionAudioOnly(2);

  final dynamic $value;
  const ByteRTCSubscribeFallbackOption([this.$value]);
}

class ByteRTCForwardStreamConfiguration extends NativeClass {
  static const _$namespace = r'ByteRTCForwardStreamConfiguration';
  static get codegen_$namespace => _$namespace;

  ByteRTCForwardStreamConfiguration([NativeClassOptions? options])
      : super(options == null
            ? const NativeClassOptions(
                [],
                className: _$namespace,
                instanceType: InstanceType.manual,
                bridgeKey: 'com.volcengine.rtc.hybrid_runtime',
              )
            : NativeClassOptions.fromMap({
                ...options.toMap(),
                'bridgeKey': 'com.volcengine.rtc.hybrid_runtime',
              }));

  /// @brief 跨房间转发媒体流过程中目标房间 ID
  ///
  FutureOr<NSString?> get roomId async {
    return await sendInstanceGet<NSString?>("roomId");
  }

  set roomId(FutureOr<NSString?> value) {
    sendInstanceSet("roomId", value);
  }

  /// @brief 使用转发目标房间 RoomID 和 UserID 生成 Token。 <br>
  ///        测试时可使用控制台生成临时 Token，正式上线需要使用密钥 SDK 在你的服务端生成并下发 Token。 <br>
  ///        如果 Token 无效，转发失败。
  ///
  FutureOr<NSString?> get token async {
    return await sendInstanceGet<NSString?>("token");
  }

  set token(FutureOr<NSString?> value) {
    sendInstanceSet("token", value);
  }
}

enum ByteRTCMixedStreamPushMode {
  /// @brief 无用户发布媒体流时，发起合流任务无效。默认设置。
  ///        当有用户发布媒体流时，才能发起合流任务。
  ///
  ByteRTCMixedStreamPushModeOnStream(0),

  /// @brief 无用户发布媒体流时，可以使用占位图发起合流任务。 <br>
  ///        占位图设置参看 alternateImageUrl{@link #ByteRTCMixedStreamLayoutRegionConfig#alternateImageUrl} 和 alternateImageFillMode{@link #ByteRTCMixedStreamLayoutRegionConfig#alternateImageFillMode}
  ///
  ByteRTCMixedStreamPushModeOnStartRequest(1);

  final dynamic $value;
  const ByteRTCMixedStreamPushMode([this.$value]);
}

enum ByteRTCPerformanceAlarmReason {
  /// @brief 网络原因差，造成了发送性能回退。仅在开启发送性能回退时，会收到此原因。
  ///
  ByteRTCPerformanceAlarmReasonBandwidthFallback(0),

  /// @brief 网络性能恢复，发送性能回退恢复。仅在开启发送性能回退时，会收到此原因。
  ///
  ByteRTCPerformanceAlarmReasonBandwidthResumed(1),

  /// @brief 如果未开启发送性能回退，收到此告警时，意味着性能不足； <br>
  ///        如果开启了发送性能回退，收到此告警时，意味着性能不足，且已发生发送性能回退。
  ///
  ByteRTCPerformanceAlarmReasonFallback(2),

  /// @brief 如果未开启发送性能回退，收到此告警时，意味着性能不足已恢复； <br>
  ///        如果开启了发送性能回退，收到此告警时，意味着性能不足已恢复，且已发生发送性能回退恢复。
  ///
  ByteRTCPerformanceAlarmReasonResumed(3);

  final dynamic $value;
  const ByteRTCPerformanceAlarmReason([this.$value]);
}
