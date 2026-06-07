/// Copyright (c) 2026 Beijing Volcano Engine Technology Ltd.

// ignore_for_file: camel_case_types, annotate_overrides, null_check_always_fails, unused_import, non_constant_identifier_names
import 'package:hybrid_runtime/hybrid_runtime.dart';
import 'dart:async';
import 'dart:typed_data';
import 'types.dart';
import 'callback.dart';

class RTCWatermarkConfig extends NativeClass {
  static const _$namespace = r'com.ss.bytertc.engine.video.RTCWatermarkConfig';
  static get codegen_$namespace => _$namespace;

  RTCWatermarkConfig([NativeClassOptions? options])
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
  ///
  FutureOr<boolean?> get visibleInPreview async {
    return await sendInstanceGet<boolean?>("visibleInPreview");
  }

  set visibleInPreview(FutureOr<boolean?> value) {
    sendInstanceSet("visibleInPreview", value);
  }

  /// @brief 横屏时的水印位置和大小，参看 ByteWatermark{@link #ByteWatermark}。
  ///
  FutureOr<ByteWatermark?> get positionInLandscapeMode async {
    try {
      final result =
          await sendInstanceGet<ByteWatermark?>("positionInLandscapeMode");
      if (result == null) {
        return null;
      }
      return packObject(result,
          () => ByteWatermark(const NativeClassOptions([], disableInit: true)));
    } catch (e) {
      return null;
    }
  }

  set positionInLandscapeMode(FutureOr<ByteWatermark?> value) {
    sendInstanceSet("positionInLandscapeMode", value);
  }

  /// @brief 竖屏时的水印位置和大小，参看 ByteWatermark{@link #ByteWatermark}。
  ///
  FutureOr<ByteWatermark?> get positionInPortraitMode async {
    try {
      final result =
          await sendInstanceGet<ByteWatermark?>("positionInPortraitMode");
      if (result == null) {
        return null;
      }
      return packObject(result,
          () => ByteWatermark(const NativeClassOptions([], disableInit: true)));
    } catch (e) {
      return null;
    }
  }

  set positionInPortraitMode(FutureOr<ByteWatermark?> value) {
    sendInstanceSet("positionInPortraitMode", value);
  }
}

class LocalAudioPropertiesInfo extends NativeClass {
  static const _$namespace =
      r'com.ss.bytertc.engine.data.LocalAudioPropertiesInfo';
  static get codegen_$namespace => _$namespace;

  LocalAudioPropertiesInfo([NativeClassOptions? options])
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

  /// @brief 音频属性信息，详见 AudioPropertiesInfo{@link #AudioPropertiesInfo}。
  ///
  FutureOr<AudioPropertiesInfo?> get audioPropertiesInfo async {
    try {
      final result =
          await sendInstanceGet<AudioPropertiesInfo?>("audioPropertiesInfo");
      if (result == null) {
        return null;
      }
      return packObject(
          result,
          () => AudioPropertiesInfo(
              const NativeClassOptions([], disableInit: true)));
    } catch (e) {
      return null;
    }
  }

  set audioPropertiesInfo(FutureOr<AudioPropertiesInfo?> value) {
    sendInstanceSet("audioPropertiesInfo", value);
  }
}

enum MediaInputType {
  /// @brief 自定义采集。 <br>
  ///        设置完成后方可直接向 SDK 推送视频帧。
  ///
  MEDIA_INPUT_TYPE_EXTERNAL(0),

  /// @brief 内部 SDK 采集。 <br>
  ///        此设置仅切换至内部采集，你需继续调用 startVideoCapture{@link #RTCEngine#startVideoCapture} 开启内部采集。
  ///
  MEDIA_INPUT_TYPE_INTERNAL(1);

  final dynamic $value;
  const MediaInputType([this.$value]);
}

enum AudioProcessorMethod {
  /// @brief 本地采集的音频。
  ///
  AUDIO_FRAME_PROCESSOR_RECORD(0),

  /// @brief 远端音频流的混音音频。
  ///
  AUDIO_FRAME_PROCESSOR_PLAYBACK(1),

  /// @brief 各个远端音频流。
  ///
  AUDIO_FRAME_PROCESSOR_REMOTE_USER(2),

  /// @brief 软件耳返音频。
  ///
  AUDIO_FRAME_PROCESSOR_EAR_MONITOR(3),

  /// @brief 屏幕共享音频。
  ///
  AUDIO_FRAME_PROCESSOR_SCREEN(4);

  final dynamic $value;
  const AudioProcessorMethod([this.$value]);
}

class FrameUpdateInfo extends NativeClass {
  static const _$namespace = r'com.ss.bytertc.engine.data.FrameUpdateInfo';
  static get codegen_$namespace => _$namespace;

  FrameUpdateInfo([NativeClassOptions? options])
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
  ///
  FutureOr<int?> get pixel async {
    return await sendInstanceGet<int?>("pixel");
  }

  set pixel(FutureOr<int?> value) {
    sendInstanceSet("pixel", value);
  }

  /// @brief 帧率。
  ///
  FutureOr<int?> get frameRate async {
    return await sendInstanceGet<int?>("frameRate");
  }

  set frameRate(FutureOr<int?> value) {
    sendInstanceSet("frameRate", value);
  }
}

enum SEICountPerFrame {
  /// @brief 单发模式。
  /// 在 1 帧间隔内多次发送 SEI 数据时，SEI 数据会按顺序跟随连续的视频帧逐帧发送，单个视频帧最多只有一个 SEI 数据。
  /// 例如，假设在某一帧间隔内需要发送 3 个 SEI 数据，那么第 1 个 SEI 会在当前帧发送，第 2 个 SEI 会在下一帧发送，第 3 个 SEI 会在再下一帧发送。
  ///
  SEI_COUNT_PER_FRAME_SINGLE(0),

  /// @brief 多发模式。
  /// 在 1 帧间隔内多次发送 SEI 数据时，这些 SEI 数据会全部在下一个视频帧上发送，可降低发送延迟，但一个视频帧会携带多个 SEI 数据。
  /// 例如，假设在某一帧间隔内有 3 个 SEI 数据需要发送，这 3 个 SEI 数据将全部在下一个视频帧上发送。
  ///
  SEI_COUNT_PER_FRAME_MULTI(1);

  final dynamic $value;
  const SEICountPerFrame([this.$value]);
}

enum VideoPixelFormat {
  /// @brief 未知格式
  ///
  UNKNOWN(0),

  /// @brief I420 格式
  ///
  I420(1),

  /// @brief NV12 格式
  ///
  NV12(2),

  /// @brief NV21 格式
  ///
  NV21(3),

  /// @brief RGBA 格式
  ///
  RGBA(5),

  /// @brief Texture2D 格式
  ///
  TEXTURE_2D(3553),

  /// @brief TextureOES 格式
  ///
  TEXTURE_OES(36197);

  final dynamic $value;
  const VideoPixelFormat([this.$value]);
}

enum MixedStreamTaskEvent {
  /// @hidden for internal use only
  ///
  BASE(0),

  /// @brief 请求发起转推直播任务
  ///
  START_SUCCESS(1),

  /// @brief 发起转推直播任务失败
  ///
  START_FAILED(2),

  /// @brief 请求更新转推直播任务配置
  ///
  UPDATE_SUCCESS(3),

  /// @brief 更新转推直播任务配置失败
  ///
  UPDATE_FAILED(4),

  /// @brief 结束转推直播任务成功
  ///
  STOP_SUCCESS(5),

  /// @brief 结束转推直播任务失败
  ///
  STOP_FAILED(6),

  /// @brief Warning 事件。
  ///
  WARNING(7);

  final dynamic $value;
  const MixedStreamTaskEvent([this.$value]);
}

enum PublishFallbackOption {
  /// @brief 上行网络不佳或设备性能不足时，不对音视频流作回退处理。默认设置。
  ///
  DISABLE(0),

  /// @brief 上行网络不佳或设备性能不足时，发布的视频流会从大流到小流依次降级，直到与当前网络性能匹配，具体降级规则参看[性能回退](https://www.volcengine.com/docs/6348/70137)文档。
  ///
  SIMULCAST_SMALL_VIDEO_ONLY(1);

  final dynamic $value;
  const PublishFallbackOption([this.$value]);
}

enum AnsMode {
  /// @brief 关闭所有音频降噪能力。
  ///
  ANS_MODE_DISABLE(0),

  /// @brief 适用于微弱降噪。
  ///
  ANS_MODE_LOW(1),

  /// @brief 适用于抑制中度平稳噪声，如空调声和风扇声。
  ///
  ANS_MODE_MEDIUM(2),

  /// @brief 适用于抑制嘈杂非平稳噪声，如键盘声、敲击声、碰撞声、动物叫声。
  ///
  ANS_MODE_HIGH(3),

  /// @brief 启用音频降噪能力。具体的降噪算法由 RTC 智能决策。
  ///
  ANS_MODE_AUTOMATIC(4);

  final dynamic $value;
  const AnsMode([this.$value]);
}

enum PauseResumeControlMediaType {
  /// @brief 只控制音频，不影响视频
  ///
  AUDIO(0),

  /// @brief 只控制视频，不影响音频
  ///
  VIDEO(1),

  /// @brief 同时控制音频和视频
  ///
  AUDIO_AND_VIDEO(2);

  final dynamic $value;
  const PauseResumeControlMediaType([this.$value]);
}

enum NetworkDetectionLinkType {
  /// @brief 上行网络探测
  ///
  UP(0),

  /// @brief 下行网络探测
  ///
  DOWN(1),

  /// @hidden constructor/destructor
  ///
  value(-1);

  final dynamic $value;
  const NetworkDetectionLinkType([this.$value]);
}

class VideoEncoderConfig extends NativeClass {
  static const _$namespace = r'com.ss.bytertc.engine.VideoEncoderConfig';
  static get codegen_$namespace => _$namespace;

  VideoEncoderConfig([NativeClassOptions? options])
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
  ///
  FutureOr<int?> get width async {
    return await sendInstanceGet<int?>("width");
  }

  set width(FutureOr<int?> value) {
    sendInstanceSet("width", value);
  }

  /// @brief 视频高度，单位：px
  ///
  FutureOr<int?> get height async {
    return await sendInstanceGet<int?>("height");
  }

  set height(FutureOr<int?> value) {
    sendInstanceSet("height", value);
  }

  /// @brief 视频帧率，单位：fps
  ///
  FutureOr<int?> get frameRate async {
    return await sendInstanceGet<int?>("frameRate");
  }

  set frameRate(FutureOr<int?> value) {
    sendInstanceSet("frameRate", value);
  }

  /// @brief 最大编码码率，使用 SDK 内部采集时可选设置，自定义采集时必须设置，单位：kbps。 <br>
  ///        设为 `-1` 即适配码率模式，系统将根据输入的分辨率和帧率自动计算适用的码率。 <br>
  ///        设为 `0` 则不对视频流进行编码发送。 <br>
  ///        3.44.1 及以上版本，内部采集时默认值为 `-1`，3.44.1 以前版本无默认值，需手动设置。
  ///
  FutureOr<int?> get maxBitrate async {
    return await sendInstanceGet<int?>("maxBitrate");
  }

  set maxBitrate(FutureOr<int?> value) {
    sendInstanceSet("maxBitrate", value);
  }

  /// @brief 视频最小编码码率, 单位 kbps。编码码率不会低于 `minBitrate`。 <br>
  ///        默认值为 `0`。 <br>
  ///        范围：[0, maxBitrate)，当 `maxBitrate` < `minBitrate` 时，为适配码率模式。 <br>
  ///        以下情况，设置本参数无效： <br>
  ///        - 当 `maxBitrate` 为 `0` 时，不对视频流进行编码发送。
  ///        - 当 `maxBitrate` < `0` 时，适配码率模式。
  ///
  FutureOr<int?> get minBitrate async {
    return await sendInstanceGet<int?>("minBitrate");
  }

  set minBitrate(FutureOr<int?> value) {
    sendInstanceSet("minBitrate", value);
  }

  /// @brief 编码策略偏好，默认为帧率优先。参看 EncoderPreference{@link #EncoderPreference}。
  ///
  FutureOr<EncoderPreference?> get encodePreference async {
    try {
      final result =
          await sendInstanceGet<EncoderPreference?>("encodePreference");
      if (result == null) {
        return null;
      }
      return EncoderPreference.values
          .firstWhere((element) => element == result);
    } catch (e) {
      return null;
    }
  }

  set encodePreference(FutureOr<EncoderPreference?> value) {
    sendInstanceSet("encodePreference", value);
  }

  /// @hidden constructor/destructor
  /// @region 视频管理
  /// @brief 判断当前类数据是否合法
  /// @return 数据合法则返回 true，否则返回 false
  ///

  FutureOr<boolean> isValid() async {
    return await nativeCall('isValid', []);
  }
}

enum VideoDenoiseModeChangedReason {
  /// @brief 未知原因导致视频降噪状态改变。
  ///
  VIDEO_DENOISE_MODE_CHANGED_REASON_NULL(-1),

  /// @brief 通过调用 setVideoDenoiser{@link #RTCEngine#setVideoDenoiser} 成功关闭视频降噪。
  ///
  VIDEO_DENOISE_MODE_CHANGED_REASON_API_OFF(0),

  /// @brief 通过调用 setVideoDenoiser{@link #RTCEngine#setVideoDenoiser} 成功开启视频降噪。
  ///
  VIDEO_DENOISE_MODE_CHANGED_REASON_API_ON(1),

  /// @brief 后台未配置视频降噪，视频降噪开启失败，请联系技术人员解决。
  ///
  VIDEO_DENOISE_MODE_CHANGED_REASON_CONFIG_DISABLED(2),

  /// @brief 后台配置开启了视频降噪。
  ///
  VIDEO_DENOISE_MODE_CHANGED_REASON_CONFIG_ENABLED(3),

  /// @brief 由于内部发生了异常，视频降噪关闭。
  ///
  VIDEO_DENOISE_MODE_CHANGED_REASON_INTERNAL_EXCEPTION(4),

  /// @brief 当前设备性能过载，已动态关闭视频降噪模式。
  ///
  VIDEO_DENOISE_MODE_CHANGED_REASON_DYNAMIC_CLOSE(5),

  /// @brief 当前设备性能裕量充足，已动态开启视频降噪模式。
  ///
  VIDEO_DENOISE_MODE_CHANGED_REASON_DYNAMIC_OPEN(6),

  /// @brief 分辨率导致视频降噪状态发生改变。分辨率过高会导致性能消耗过大，从而导致视频降噪模式关闭。若希望继续使用视频降噪，可选择降低分辨率。
  ///
  VIDEO_DENOISE_MODE_CHANGED_REASON_RESOLUTION(7);

  final dynamic $value;
  const VideoDenoiseModeChangedReason([this.$value]);
}

enum SEIStreamUpdateEvent {
  /// @brief 远端用户发布黑帧视频流。 <br>
  ///        纯语音通话场景下，远端用户调用 sendSEIMessage{@link #RTCEngine#sendSEIMessage} 发送 SEI 数据时，SDK 会自动发布一路黑帧视频流，并触发该回调。
  ///
  STREAM_ADD(0),

  /// @brief 远端黑帧视频流移除。该回调的触发时机包括： <br>
  ///        - 远端用户开启摄像头采集，由语音通话切换至视频通话，黑帧视频流停止发布；
  ///        - 远端用户调用 sendSEIMessage{@link #RTCEngine#sendSEIMessage} 后 1min 内未有 SEI 数据发送，黑帧视频流停止发布；
  ///        - 远端用户调用 setVideoSourceType{@link #RTCEngine#setVideoSourceType} 切换至自定义视频采集时，黑帧视频流停止发布。
  ///
  STREAM_REMOVE(1),

  /// @hidden constructor/destructor
  ///
  value(-1);

  final dynamic $value;
  const SEIStreamUpdateEvent([this.$value]);
}

class LocalStreamStats extends NativeClass {
  static const _$namespace = r'com.ss.bytertc.engine.type.LocalStreamStats';
  static get codegen_$namespace => _$namespace;

  LocalStreamStats([NativeClassOptions? options])
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

  /// @brief 本地设备发送音频流的统计信息，详见 LocalAudioStats{@link #LocalAudioStats} 。
  ///
  FutureOr<LocalAudioStats?> get audioStats async {
    try {
      final result = await sendInstanceGet<LocalAudioStats?>("audioStats");
      if (result == null) {
        return null;
      }
      return packObject(
          result,
          () =>
              LocalAudioStats(const NativeClassOptions([], disableInit: true)));
    } catch (e) {
      return null;
    }
  }

  set audioStats(FutureOr<LocalAudioStats?> value) {
    sendInstanceSet("audioStats", value);
  }

  /// @brief 本地设备发送视频流的统计信息，详见 LocalVideoStats{@link #LocalVideoStats} 。
  ///
  FutureOr<LocalVideoStats?> get videoStats async {
    try {
      final result = await sendInstanceGet<LocalVideoStats?>("videoStats");
      if (result == null) {
        return null;
      }
      return packObject(
          result,
          () =>
              LocalVideoStats(const NativeClassOptions([], disableInit: true)));
    } catch (e) {
      return null;
    }
  }

  set videoStats(FutureOr<LocalVideoStats?> value) {
    sendInstanceSet("videoStats", value);
  }

  /// @brief 所属用户的媒体流是否为屏幕流。你可以知道当前统计数据来自主流还是屏幕流。
  ///
  FutureOr<boolean?> get isScreen async {
    return await sendInstanceGet<boolean?>("isScreen");
  }

  set isScreen(FutureOr<boolean?> value) {
    sendInstanceSet("isScreen", value);
  }

  /// @brief 本地媒体上行网络质量，详见 NetworkQuality{@link #NetworkQuality} 。
  /// @deprecated since 3.36 and will be deleted in 3.51, use onNetworkQuality{@link #IRTCRoomEventHandler#onNetworkQuality} instead.
  ///
  FutureOr<int?> get txQuality async {
    return await sendInstanceGet<int?>("txQuality");
  }

  set txQuality(FutureOr<int?> value) {
    sendInstanceSet("txQuality", value);
  }

  /// @brief 本地媒体下行网络质量，详见 NetworkQuality{@link #NetworkQuality} 。
  /// @deprecated since 3.36 and will be deleted in 3.51, use onNetworkQuality{@link #IRTCRoomEventHandler#onNetworkQuality} instead.
  ///
  FutureOr<int?> get rxQuality async {
    return await sendInstanceGet<int?>("rxQuality");
  }

  set rxQuality(FutureOr<int?> value) {
    sendInstanceSet("rxQuality", value);
  }
}

class AudioPropertiesConfig extends NativeClass {
  static const _$namespace =
      r'com.ss.bytertc.engine.data.AudioPropertiesConfig';
  static get codegen_$namespace => _$namespace;

  AudioPropertiesConfig([NativeClassOptions? options])
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
  ///
  FutureOr<int?> get interval async {
    return await sendInstanceGet<int?>("interval");
  }

  set interval(FutureOr<int?> value) {
    sendInstanceSet("interval", value);
  }

  /// @brief 是否开启音频频谱检测
  ///
  FutureOr<boolean?> get enableSpectrum async {
    return await sendInstanceGet<boolean?>("enableSpectrum");
  }

  set enableSpectrum(FutureOr<boolean?> value) {
    sendInstanceSet("enableSpectrum", value);
  }

  /// @brief 是否开启人声检测 (VAD)
  ///
  FutureOr<boolean?> get enableVad async {
    return await sendInstanceGet<boolean?>("enableVad");
  }

  set enableVad(FutureOr<boolean?> value) {
    sendInstanceSet("enableVad", value);
  }

  /// @brief 音量回调模式。详见 AudioReportMode{@link #AudioReportMode}。
  ///
  FutureOr<AudioReportMode?> get localMainReportMode async {
    try {
      final result =
          await sendInstanceGet<AudioReportMode?>("localMainReportMode");
      if (result == null) {
        return null;
      }
      return AudioReportMode.values.firstWhere((element) => element == result);
    } catch (e) {
      return null;
    }
  }

  set localMainReportMode(FutureOr<AudioReportMode?> value) {
    sendInstanceSet("localMainReportMode", value);
  }

  /// @brief onLocalAudioPropertiesReport{@link #IRTCEngineEventHandler#onLocalAudioPropertiesReport} 中包含音频数据的范围。参看 AudioPropertiesMode{@link #AudioPropertiesMode}。 <br>
  ///        默认仅包含本地麦克风采集的音频数据和本地屏幕音频采集数据，不包含本地混音音频数据。
  ///
  FutureOr<AudioPropertiesMode?> get audioReportMode async {
    try {
      final result =
          await sendInstanceGet<AudioPropertiesMode?>("audioReportMode");
      if (result == null) {
        return null;
      }
      return AudioPropertiesMode.values
          .firstWhere((element) => element == result);
    } catch (e) {
      return null;
    }
  }

  set audioReportMode(FutureOr<AudioPropertiesMode?> value) {
    sendInstanceSet("audioReportMode", value);
  }

  /// @brief 适用于音频属性信息提示的平滑系数。取值范围是 `(0.0, 1.0]`。 <br>
  ///        默认值为 `1.0`，不开启平滑效果；值越小，提示音量平滑效果越明显。如果要开启平滑效果，可以设置为 `0.3`。
  ///
  FutureOr<float?> get smooth async {
    return await sendInstanceGet<float?>("smooth");
  }

  set smooth(FutureOr<float?> value) {
    sendInstanceSet("smooth", value);
  }

  /// @brief 是否回调本地用户的人声基频。
  ///
  FutureOr<boolean?> get enableVoicePitch async {
    return await sendInstanceGet<boolean?>("enableVoicePitch");
  }

  set enableVoicePitch(FutureOr<boolean?> value) {
    sendInstanceSet("enableVoicePitch", value);
  }
}

class PositionInfo extends NativeClass {
  static const _$namespace = r'com.ss.bytertc.engine.data.PositionInfo';
  static get codegen_$namespace => _$namespace;

  PositionInfo([NativeClassOptions? options])
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

  /// @brief 用户在空间音频坐标系里的位置，需自行建立空间直角坐标系。参看 Position{@link #Position}
  ///
  FutureOr<Position?> get position async {
    try {
      final result = await sendInstanceGet<Position?>("position");
      if (result == null) {
        return null;
      }
      return packObject(result,
          () => Position(const NativeClassOptions([], disableInit: true)));
    } catch (e) {
      return null;
    }
  }

  set position(FutureOr<Position?> value) {
    sendInstanceSet("position", value);
  }

  /// @brief 用户在空间音频坐标系里的三维朝向信息。三个向量需要两两垂直。参看 HumanOrientation{@link #HumanOrientation}。
  ///
  FutureOr<HumanOrientation?> get orientation async {
    try {
      final result = await sendInstanceGet<HumanOrientation?>("orientation");
      if (result == null) {
        return null;
      }
      return packObject(
          result,
          () => HumanOrientation(
              const NativeClassOptions([], disableInit: true)));
    } catch (e) {
      return null;
    }
  }

  set orientation(FutureOr<HumanOrientation?> value) {
    sendInstanceSet("orientation", value);
  }

  /// @hidden constructor/destructor
  ///

  FutureOr<float> getFieldPositionX() async {
    return await nativeCall('getFieldPositionX', []);
  }

  /// @hidden constructor/destructor
  ///

  FutureOr<float> getFieldPositionY() async {
    return await nativeCall('getFieldPositionY', []);
  }

  /// @hidden constructor/destructor
  ///

  FutureOr<float> getFieldPositionZ() async {
    return await nativeCall('getFieldPositionZ', []);
  }

  /// @hidden constructor/destructor
  ///

  FutureOr<float> getFieldOrientationForwardX() async {
    return await nativeCall('getFieldOrientationForwardX', []);
  }

  /// @hidden constructor/destructor
  ///

  FutureOr<float> getFieldOrientationForwardY() async {
    return await nativeCall('getFieldOrientationForwardY', []);
  }

  /// @hidden constructor/destructor
  ///

  FutureOr<float> getFieldOrientationForwardZ() async {
    return await nativeCall('getFieldOrientationForwardZ', []);
  }

  /// @hidden constructor/destructor
  ///

  FutureOr<float> getFieldOrientationRightX() async {
    return await nativeCall('getFieldOrientationRightX', []);
  }

  /// @hidden constructor/destructor
  ///

  FutureOr<float> getFieldOrientationRightY() async {
    return await nativeCall('getFieldOrientationRightY', []);
  }

  /// @hidden constructor/destructor
  ///

  FutureOr<float> getFieldOrientationRightZ() async {
    return await nativeCall('getFieldOrientationRightZ', []);
  }

  /// @hidden constructor/destructor
  ///

  FutureOr<float> getFieldOrientationUpX() async {
    return await nativeCall('getFieldOrientationUpX', []);
  }

  /// @hidden constructor/destructor
  ///

  FutureOr<float> getFieldOrientationUpY() async {
    return await nativeCall('getFieldOrientationUpY', []);
  }

  /// @hidden constructor/destructor
  ///

  FutureOr<float> getFieldOrientationUpZ() async {
    return await nativeCall('getFieldOrientationUpZ', []);
  }
}

class MixedStreamLayoutRegionConfig extends NativeClass {
  static const _$namespace =
      r'com.ss.bytertc.engine.live.MixedStreamLayoutRegionConfig';
  static get codegen_$namespace => _$namespace;

  MixedStreamLayoutRegionConfig([NativeClassOptions? options])
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
  /// @brief 设置视频流发布用户的用户 ID。建议设置。
  /// @param userID 用户 ID。
  /// @note
  ///        本参数不支持过程中更新。
  ///
  FutureOr<String?> get userID async {
    return await sendInstanceGet<String?>("userID");
  }

  set userID(FutureOr<String?> value) {
    sendInstanceSet("userID", value);
  }

  /// @detail api
  /// @brief 设置视频流发布用户的房间 ID。建议设置。
  /// @param roomID 房间 ID。必填。
  /// @note
  ///        本参数不支持过程中更新。
  ///
  FutureOr<String?> get roomID async {
    return await sendInstanceGet<String?>("roomID");
  }

  set roomID(FutureOr<String?> value) {
    sendInstanceSet("roomID", value);
  }

  /// @valid since 3.57
  /// @detail api
  /// @brief 设置单个用户画面左上角在整个画布坐标系中的 X 坐标（pixel），即以画布左上角为原点，用户画面左上角相对于原点的横向位移。
  /// @param locationX 用户画面左上角的横坐标。取值范围为 [0, 整体画布宽度)。默认值为 0。
  ///
  FutureOr<int?> get locationX async {
    return await sendInstanceGet<int?>("locationX");
  }

  set locationX(FutureOr<int?> value) {
    sendInstanceSet("locationX", value);
  }

  /// @valid since 3.57
  /// @detail api
  /// @brief 设置单个用户画面左上角在整个画布坐标系中的 Y 坐标（pixel），即以画布左上角为原点，用户画面左上角相对于原点的纵向位移。
  /// @param locationY 用户画面左上角的纵坐标。取值范围为 [0, 整体画布高度)。默认值为 0。
  ///
  FutureOr<int?> get locationY async {
    return await sendInstanceGet<int?>("locationY");
  }

  set locationY(FutureOr<int?> value) {
    sendInstanceSet("locationY", value);
  }

  /// @valid since 3.57
  /// @detail api
  /// @brief 设置单个用户画面的宽度。
  /// @param width 用户画面宽度。取值范围为 [0, 整体画布宽度]，默认值为 360。
  ///
  FutureOr<int?> get width async {
    return await sendInstanceGet<int?>("width");
  }

  set width(FutureOr<int?> value) {
    sendInstanceSet("width", value);
  }

  /// @valid since 3.57
  /// @detail api
  /// @brief 设置单个用户画面的高度。
  /// @param height 用户画面高度。取值范围为 [0, 整体画布高度]，默认值为 640。
  ///
  FutureOr<int?> get height async {
    return await sendInstanceGet<int?>("height");
  }

  set height(FutureOr<int?> value) {
    sendInstanceSet("height", value);
  }

  /// @detail api
  /// @brief 设置用户视频布局在画布中的层级。
  /// @param zOrder 用户视频布局在画布中的层级。取值范围为 [0 - 100]，0 为底层，值越大越上层。默认值为 0。
  ///
  FutureOr<int?> get zOrder async {
    return await sendInstanceGet<int?>("zOrder");
  }

  set zOrder(FutureOr<int?> value) {
    sendInstanceSet("zOrder", value);
  }

  /// @detail api
  /// @brief （仅服务端合流支持设置）设置透明度。
  /// @param alpha 透明度，可选范围为 (0.0, 1.0]，0.0 为全透明。默认值为 1.0。
  ///
  FutureOr<double?> get alpha async {
    return await sendInstanceGet<double?>("alpha");
  }

  set alpha(FutureOr<double?> value) {
    sendInstanceSet("alpha", value);
  }

  /// @detail api
  /// @brief （仅服务端合流支持设置）设置圆角半径。
  /// @param cornerRadius 圆角半径相对画布宽度的比例。默认值为 `0.0`。
  /// @note 做范围判定时，首先根据画布的宽高，将 `width`，`height`，和 `radius` 分别转换为像素值：`width_px`，`height_px`，和 `radius_px`。然后判定是否满足 `radius_px < min(width_px/2, height_px/2)`：若满足，则设置成功；若不满足，则将 `radius_px` 设定为 `min(width_px/2, height_px/2)`，然后将 `radius` 设定为 `radius_px` 相对画布宽度的比例值。
  ///        WTN 流任务不支持设置本参数。
  ///
  FutureOr<double?> get cornerRadius async {
    return await sendInstanceGet<double?>("cornerRadius");
  }

  set cornerRadius(FutureOr<double?> value) {
    sendInstanceSet("cornerRadius", value);
  }

  /// @detail api
  /// @brief （仅服务端合流支持设置）设置合流内容类型。
  /// @param mediaType 合流内容控制。默认值为 `MIXED_STREAM_MEDIA_TYPE_AUDIO_AND_VIDEO(0)`，参看 MixedStreamMediaType{@link #MixedStreamMediaType}。
  ///
  FutureOr<MixedStreamMediaType?> get mediaType async {
    try {
      final result = await sendInstanceGet<MixedStreamMediaType?>("mediaType");
      if (result == null) {
        return null;
      }
      return MixedStreamMediaType.values
          .firstWhere((element) => element == result);
    } catch (e) {
      return null;
    }
  }

  set mediaType(FutureOr<MixedStreamMediaType?> value) {
    sendInstanceSet("mediaType", value);
  }

  /// @detail api
  /// @brief 设置图片或视频流渲染的缩放模式。建议设置。
  /// @param renderMode 图片或视频流的缩放模式，参看 MixedStreamRenderMode{@link #MixedStreamRenderMode}。默认值为 1。
  ///
  FutureOr<MixedStreamRenderMode?> get renderMode async {
    try {
      final result =
          await sendInstanceGet<MixedStreamRenderMode?>("renderMode");
      if (result == null) {
        return null;
      }
      return MixedStreamRenderMode.values
          .firstWhere((element) => element == result);
    } catch (e) {
      return null;
    }
  }

  set renderMode(FutureOr<MixedStreamRenderMode?> value) {
    sendInstanceSet("renderMode", value);
  }

  /// @detail api
  /// @brief 设置是否是本地用户。
  /// @param islocalUser 标识是否是本地用户 <br>
  ///       - true: 是
  ///       - false: 否
  ///
  FutureOr<boolean?> get isLocalUser async {
    return await sendInstanceGet<boolean?>("isLocalUser");
  }

  set isLocalUser(FutureOr<boolean?> value) {
    sendInstanceSet("isLocalUser", value);
  }

  /// @detail api
  /// @brief 设置 region 中的流类型是主流还是屏幕流。仅服务端合流支持合屏幕流。
  /// @param streamType 流类型，参看 MixedStreamVideoType{@link #MixedStreamVideoType}。
  ///
  FutureOr<MixedStreamVideoType?> get streamType async {
    try {
      final result = await sendInstanceGet<MixedStreamVideoType?>("streamType");
      if (result == null) {
        return null;
      }
      return MixedStreamVideoType.values
          .firstWhere((element) => element == result);
    } catch (e) {
      return null;
    }
  }

  set streamType(FutureOr<MixedStreamVideoType?> value) {
    sendInstanceSet("streamType", value);
  }

  /// @detail api
  /// @brief 设置合流布局区域类型。
  /// @param regionContentType 合流布局区域类型，详见 MixedStreamLayoutRegionType{@link #MixedStreamLayoutRegionType}。
  ///
  FutureOr<MixedStreamLayoutRegionType?> get regionContentType async {
    try {
      final result = await sendInstanceGet<MixedStreamLayoutRegionType?>(
          "regionContentType");
      if (result == null) {
        return null;
      }
      return MixedStreamLayoutRegionType.values
          .firstWhere((element) => element == result);
    } catch (e) {
      return null;
    }
  }

  set regionContentType(FutureOr<MixedStreamLayoutRegionType?> value) {
    sendInstanceSet("regionContentType", value);
  }

  /// @detail api
  /// @brief 水印图 RGBA 数据。
  /// @param imageWaterMark 图片合流布局区域类型对应的数据。当 `regionContentType` 为图片类型时需要设置。 <br>
  ///        - `MIXED_STREAM_LAYOUT_REGION_TYPE_IMAGE(1)` 时，传入图片 RGBA 数据。
  ///        - `MIXED_STREAM_LAYOUT_REGION_TYPE_VIDEO_STREAM(0)` 时传入空。
  ///        WTN 流任务不支持设置本参数。
  ///
  FutureOr<ArrayBuffer?> get imageWaterMark async {
    return await sendInstanceGet<ArrayBuffer?>("imageWaterMark");
  }

  set imageWaterMark(FutureOr<ArrayBuffer?> value) {
    sendInstanceSet("imageWaterMark", value);
  }

  /// @detail api
  /// @brief 设置水印图参数。
  /// @param imageWaterMarkConfig 水印图参数。当 `regionContentType` 为图片类型时需要设置。 <br>
  ///        - `MIXED_STREAM_LAYOUT_REGION_TYPE_IMAGE(1)` 时，传入图片参数，参看 MixedStreamLayoutRegionImageWaterMarkConfig{@link #MixedStreamLayoutRegionConfig-MixedStreamLayoutRegionImageWaterMarkConfig}。
  ///        - `MIXED_STREAM_LAYOUT_REGION_TYPE_VIDEO_STREAM(0)` 时传入空。
  ///        WTN 流任务不支持设置本参数。
  ///
  FutureOr<MixedStreamLayoutRegionImageWaterMarkConfig?>
      get imageWaterMarkConfig async {
    try {
      final result =
          await sendInstanceGet<MixedStreamLayoutRegionImageWaterMarkConfig?>(
              "imageWaterMarkConfig");
      if (result == null) {
        return null;
      }
      return packObject(
          result,
          () => MixedStreamLayoutRegionImageWaterMarkConfig(
              const NativeClassOptions([], disableInit: true)));
    } catch (e) {
      return null;
    }
  }

  set imageWaterMarkConfig(
      FutureOr<MixedStreamLayoutRegionImageWaterMarkConfig?> value) {
    sendInstanceSet("imageWaterMarkConfig", value);
  }

  /// @valid since 3.57
  /// @detail api
  /// @brief 设置占位图的填充模式。 <br>
  ///        该方法用来控制当用户停止发布视频流，画面恢复为占位图后，此时占位图的填充模式。
  /// @param alternateImageFillMode 占位图的填充模式。参看 MixedStreamAlternateImageFillMode{@link #MixedStreamAlternateImageFillMode}。
  ///
  FutureOr<MixedStreamAlternateImageFillMode?>
      get alternateImageFillMode async {
    try {
      final result = await sendInstanceGet<MixedStreamAlternateImageFillMode?>(
          "alternateImageFillMode");
      if (result == null) {
        return null;
      }
      return MixedStreamAlternateImageFillMode.values
          .firstWhere((element) => element == result);
    } catch (e) {
      return null;
    }
  }

  set alternateImageFillMode(
      FutureOr<MixedStreamAlternateImageFillMode?> value) {
    sendInstanceSet("alternateImageFillMode", value);
  }

  /// @valid since 3.57
  /// @detail api
  /// @brief 设置占位图
  /// @param alternateImageUrl 占位图的的 URL，长度小于 1024 字符.
  ///
  FutureOr<String?> get alternateImageURL async {
    return await sendInstanceGet<String?>("alternateImageURL");
  }

  set alternateImageURL(FutureOr<String?> value) {
    sendInstanceSet("alternateImageURL", value);
  }

  /// @detail api
  /// @brief 设置当前 region 空间音频位置。
  /// @param spatialPosition 当前 region 的空间音频位置。参加 Position{@link #Position}。
  /// @note
  ///        WTN 流任务不支持设置本参数。
  ///
  FutureOr<Position?> get spatialPosition async {
    try {
      final result = await sendInstanceGet<Position?>("spatialPosition");
      if (result == null) {
        return null;
      }
      return packObject(result,
          () => Position(const NativeClassOptions([], disableInit: true)));
    } catch (e) {
      return null;
    }
  }

  set spatialPosition(FutureOr<Position?> value) {
    sendInstanceSet("spatialPosition", value);
  }

  /// @detail api
  /// @brief 设置某用户是否应用空间音频效果。
  /// @param applySpatialAudio 该用户是否应用空间音频效果： <br>
  ///        - true：启用（默认值）
  ///        - false：禁用
  ///
  FutureOr<boolean?> get applySpatialAudio async {
    return await sendInstanceGet<boolean?>("applySpatialAudio");
  }

  set applySpatialAudio(FutureOr<boolean?> value) {
    sendInstanceSet("applySpatialAudio", value);
  }

  /// @brief 支持对每一路参与WTN 流的视频进行裁剪。
  /// @param sourceCrop 裁剪配置。参见 SourceCrop{@link #SourceCrop}。
  /// @note
  ///        合流转推任务不支持。
  ///
  FutureOr<SourceCrop?> get sourceCrop async {
    try {
      final result = await sendInstanceGet<SourceCrop?>("sourceCrop");
      if (result == null) {
        return null;
      }
      return packObject(result,
          () => SourceCrop(const NativeClassOptions([], disableInit: true)));
    } catch (e) {
      return null;
    }
  }

  set sourceCrop(FutureOr<SourceCrop?> value) {
    sendInstanceSet("sourceCrop", value);
  }
}

enum AVSyncEvent {
  /// @brief 音视频同步失败。<br>
  ///        当前音频源已与其他视频源关联同步关系。 <br>
  ///        单个音频源不支持与多个视频源同时同步。
  ///
  INVALID_UID_REPEATED(0);

  final dynamic $value;
  const AVSyncEvent([this.$value]);
}

class RemoteVideoConfig extends NativeClass {
  static const _$namespace = r'com.ss.bytertc.engine.data.RemoteVideoConfig';
  static get codegen_$namespace => _$namespace;

  RemoteVideoConfig([NativeClassOptions? options])
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
  ///
  FutureOr<int?> get width async {
    return await sendInstanceGet<int?>("width");
  }

  set width(FutureOr<int?> value) {
    sendInstanceSet("width", value);
  }

  /// @brief 视频高度，单位：px
  ///
  FutureOr<int?> get height async {
    return await sendInstanceGet<int?>("height");
  }

  set height(FutureOr<int?> value) {
    sendInstanceSet("height", value);
  }

  /// @brief 期望订阅的最高帧率，单位：fps，默认值为 0 即满帧订阅，设为大于 0 的值时开始生效。 <br>
  ///        如果发布端发布帧率 > 订阅端订阅的帧率，下行媒体服务器 SVC 丢帧，订阅端收到通过此接口设置的帧率；如果发布端发布帧率 < 订阅端订阅的帧率，则订阅端只能收到发布的帧率。<br>
  ///        仅码流支持 SVC 分级编码特性时方可生效。
  ///
  FutureOr<int?> get framerate async {
    return await sendInstanceGet<int?>("framerate");
  }

  set framerate(FutureOr<int?> value) {
    sendInstanceSet("framerate", value);
  }

  /// @hidden constructor/destructor
  ///

  FutureOr<int> getWidth() async {
    return await nativeCall('getWidth', []);
  }

  /// @hidden constructor/destructor
  ///

  FutureOr<int> getHeight() async {
    return await nativeCall('getHeight', []);
  }

  /// @hidden constructor/destructor
  ///

  FutureOr<int> getFrameRate() async {
    return await nativeCall('getFrameRate', []);
  }
}

enum AudioSampleRate {
  /// @brief 默认设置。48000Hz。
  ///
  AUDIO_SAMPLE_RATE_AUTO(-1),

  /// @brief 8000Hz
  ///
  AUDIO_SAMPLE_RATE_8000(8000),

  /// @brief 11025Hz
  ///
  AUDIO_SAMPLE_RATE_11025(11025),

  /// @brief 16000Hz
  ///
  AUDIO_SAMPLE_RATE_16000(16000),

  /// @brief 22050Hz
  ///
  AUDIO_SAMPLE_RATE_22050(22050),

  /// @brief 24000Hz
  ///
  AUDIO_SAMPLE_RATE_24000(24000),

  /// @brief 32000Hz
  ///
  AUDIO_SAMPLE_RATE_32000(32000),

  /// @brief 44100Hz
  ///
  AUDIO_SAMPLE_RATE_44100(44100),

  /// @brief 48000Hz
  ///
  AUDIO_SAMPLE_RATE_48000(48000);

  final dynamic $value;
  const AudioSampleRate([this.$value]);
}

enum SVCLayer {
  /// @brief 不指定分层（默认值）
  ///
  DEFAULT(0),

  /// @brief T0 层
  ///
  BASE(1),

  /// @brief T0+T1 层
  ///
  MAIN(2),

  /// @brief T0+T1+T2 层
  ///
  HIGH(3);

  final dynamic $value;
  const SVCLayer([this.$value]);
}

enum GameSceneType {
  /// @brief 普通场景。<br>
  ///        同一个小队房间的队友，仅支持在同一个世界房间内通话。
  ///
  NORMAL(0),

  /// @brief 主题公园场景。<br>
  ///        同一个小队房间的队友，支持跨世界房间通话。
  ///
  THEMEPARK(1);

  final dynamic $value;
  const GameSceneType([this.$value]);
}

enum VideoDeviceFacing {
  /// @brief 前置摄像头
  ///
  FRONT(0),

  /// @brief 后置摄像头
  ///
  BACK(1),

  /// @brief 未知类型
  ///
  UNKNOWN(2);

  final dynamic $value;
  const VideoDeviceFacing([this.$value]);
}

enum StreamIndex {
  /// @brief
  ///        主流。包括： <br>
  ///        - 由摄像头/麦克风通过内部采集机制，采集到的视频/音频;
  ///        - 通过自定义采集，采集到的视频/音频。
  ///
  STREAM_INDEX_MAIN(0),

  /// @brief 屏幕流。屏幕共享时共享的视频流，或来自声卡的本地播放音频流。
  ///
  STREAM_INDEX_SCREEN(1),

  /// @hidden for internal use only
  ///
  STREAM_INDEX_3RD(2),

  /// @hidden for internal use only
  ///
  STREAM_INDEX_4TH(3),

  /// @hidden for internal use only
  ///
  STREAM_INDEX_5TH(4),

  /// @hidden for internal use only
  ///
  STREAM_INDEX_6TH(5),

  /// @hidden for internal use only
  ///
  STREAM_INDEX_7TH(6),

  /// @hidden for internal use only
  ///
  STREAM_INDEX_MAX(7);

  final dynamic $value;
  const StreamIndex([this.$value]);
}

enum PlayerEvent {
  /// @brief 开始切换音轨 <br>
  ///        开始调用 selectAudioTrack{@link #IMediaPlayer#selectAudioTrack} 时，返回此状态。
  ///
  SELECT_AUDIO_TRACK_BEGIN(0),

  /// @brief 切换音轨成功 <br>
  ///        调用 selectAudioTrack{@link #IMediaPlayer#selectAudioTrack} 成功后，返回此状态。
  ///
  SELECT_AUDIO_TRACK_COMPLETED(1),

  /// @brief 切换音轨失败 <br>
  ///        调用 selectAudioTrack{@link #IMediaPlayer#selectAudioTrack} 失败后，播放器无法切换到指定音轨，继续之前的音轨播放过程，返回此状态。
  ///
  SELECT_AUDIO_TRACK_FAILED(2),

  /// @brief 试图移动播放位置 <br>
  ///        开始调用 setPosition{@link #IMediaPlayer#setPosition} 时，返回此状态。
  ///
  SEEK_BEGIN(3),

  /// @brief 移动播放位置成功 <br>
  ///        调用 setPosition{@link #IMediaPlayer#setPosition} 成功后，返回此状态。
  ///
  SEEK_COMPLETED(4),

  /// @brief 移动播放位置失败 <br>
  ///        调用 setPosition{@link #IMediaPlayer#setPosition} 失败时，返回此状态。
  ///
  SEEK_FAILED(5);

  final dynamic $value;
  const PlayerEvent([this.$value]);
}

class VideoFrameInfo extends NativeClass {
  static const _$namespace = r'com.ss.bytertc.engine.data.VideoFrameInfo';
  static get codegen_$namespace => _$namespace;

  VideoFrameInfo([NativeClassOptions? options])
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

  /// @brief 宽（像素）
  ///
  FutureOr<int?> get width async {
    return await sendInstanceGet<int?>("width");
  }

  set width(FutureOr<int?> value) {
    sendInstanceSet("width", value);
  }

  /// @brief 高（像素）
  ///
  FutureOr<int?> get height async {
    return await sendInstanceGet<int?>("height");
  }

  set height(FutureOr<int?> value) {
    sendInstanceSet("height", value);
  }

  /// @brief 视频帧顺时针旋转角度。参看 VideoRotation{@link #VideoRotation}。
  ///
  FutureOr<VideoRotation?> get rotation async {
    try {
      final result = await sendInstanceGet<VideoRotation?>("rotation");
      if (result == null) {
        return null;
      }
      return VideoRotation.values.firstWhere((element) => element == result);
    } catch (e) {
      return null;
    }
  }

  set rotation(FutureOr<VideoRotation?> value) {
    sendInstanceSet("rotation", value);
  }

  /// @hidden constructor/destructor
  ///

  FutureOr<int> getWidth() async {
    return await nativeCall('getWidth', []);
  }

  /// @hidden constructor/destructor
  ///

  FutureOr<void> setWidth(int width) async {
    return await nativeCall('setWidth', [width]);
  }

  /// @hidden constructor/destructor
  ///

  FutureOr<int> getHeight() async {
    return await nativeCall('getHeight', []);
  }
}

enum AudioAlignmentMode {
  /// @brief 不对齐
  ///
  AUDIO_ALIGNMENT_MODE_OFF(0),

  /// @brief 远端音频流都对齐伴奏进度同步播放
  ///
  AUDIO_ALIGNMENT_MODE_AUDIOMIXING(1);

  final dynamic $value;
  const AudioAlignmentMode([this.$value]);
}

class RemoteStreamStats extends NativeClass {
  static const _$namespace = r'com.ss.bytertc.engine.type.RemoteStreamStats';
  static get codegen_$namespace => _$namespace;

  RemoteStreamStats([NativeClassOptions? options])
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

  /// @brief 用户 ID 。音/视频来源的远端用户 ID 。
  ///
  FutureOr<String?> get uid async {
    return await sendInstanceGet<String?>("uid");
  }

  set uid(FutureOr<String?> value) {
    sendInstanceSet("uid", value);
  }

  /// @brief 远端音频流的统计信息，详见 RemoteAudioStats{@link #RemoteAudioStats}
  ///
  FutureOr<RemoteAudioStats?> get audioStats async {
    try {
      final result = await sendInstanceGet<RemoteAudioStats?>("audioStats");
      if (result == null) {
        return null;
      }
      return packObject(
          result,
          () => RemoteAudioStats(
              const NativeClassOptions([], disableInit: true)));
    } catch (e) {
      return null;
    }
  }

  set audioStats(FutureOr<RemoteAudioStats?> value) {
    sendInstanceSet("audioStats", value);
  }

  /// @brief 远端视频流的统计信息，详见 RemoteVideoStats{@link #RemoteVideoStats}
  ///
  FutureOr<RemoteVideoStats?> get videoStats async {
    try {
      final result = await sendInstanceGet<RemoteVideoStats?>("videoStats");
      if (result == null) {
        return null;
      }
      return packObject(
          result,
          () => RemoteVideoStats(
              const NativeClassOptions([], disableInit: true)));
    } catch (e) {
      return null;
    }
  }

  set videoStats(FutureOr<RemoteVideoStats?> value) {
    sendInstanceSet("videoStats", value);
  }

  /// @brief 所属用户的媒体流是否为屏幕流。你可以知道当前统计数据来自主流还是屏幕流。
  ///
  FutureOr<boolean?> get isScreen async {
    return await sendInstanceGet<boolean?>("isScreen");
  }

  set isScreen(FutureOr<boolean?> value) {
    sendInstanceSet("isScreen", value);
  }

  /// @brief 所属用户的媒体流上行网络质量，详见 NetworkQuality{@link #NetworkQuality} 。
  /// @deprecated since 3.36 and will be deleted in 3.51, use onNetworkQuality{@link #IRTCRoomEventHandler#onNetworkQuality} instead.
  ///
  FutureOr<int?> get txQuality async {
    return await sendInstanceGet<int?>("txQuality");
  }

  set txQuality(FutureOr<int?> value) {
    sendInstanceSet("txQuality", value);
  }

  /// @brief 所属用户的媒体流下行网络质量，详见 NetworkQuality{@link #NetworkQuality} 。
  /// @deprecated since 3.36 and will be deleted in 3.51, use onNetworkQuality{@link #IRTCRoomEventHandler#onNetworkQuality} instead.
  ///
  FutureOr<int?> get rxQuality async {
    return await sendInstanceGet<int?>("rxQuality");
  }

  set rxQuality(FutureOr<int?> value) {
    sendInstanceSet("rxQuality", value);
  }
}

enum VideoDecoderConfig {
  /// @brief 开启 SDK 内部解码，只回调解码后的数据。回调为 onFrame{@link #IVideoSink#onFrame}。
  ///
  VIDEO_DECODER_CONFIG_RAW(0),

  /// @brief 开启自定义解码，只回调解码前数据。回调为 onRemoteEncodedVideoFrame{@link #IRemoteEncodedVideoFrameObserver#onRemoteEncodedVideoFrame}。
  ///
  VIDEO_DECODER_CONFIG_ENCODE(1),

  /// @brief 开启 SDK 内部解码，同时回调解码前和解码后的数据。onFrame{@link #IVideoSink#onFrame} 和 onRemoteEncodedVideoFrame{@link #IRemoteEncodedVideoFrameObserver#onRemoteEncodedVideoFrame}。
  ///
  VIDEO_DECODER_CONFIG_BOTH(2);

  final dynamic $value;
  const VideoDecoderConfig([this.$value]);
}

class SubtitleConfig extends NativeClass {
  static const _$namespace = r'com.ss.bytertc.engine.type.SubtitleConfig';
  static get codegen_$namespace => _$namespace;

  SubtitleConfig([NativeClassOptions? options])
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

  /// @brief 字幕模式。可以根据需要选择识别和翻译两种模式。开启识别模式，会将识别后的用户语音转化成文字；开启翻译模式，会在语音识别后进行翻译。参看 SubtitleMode{@link #SubtitleMode}。
  ///
  FutureOr<SubtitleMode?> get mode async {
    try {
      final result = await sendInstanceGet<SubtitleMode?>("mode");
      if (result == null) {
        return null;
      }
      return SubtitleMode.values.firstWhere((element) => element == result);
    } catch (e) {
      return null;
    }
  }

  set mode(FutureOr<SubtitleMode?> value) {
    sendInstanceSet("mode", value);
  }

  /// @brief 目标翻译语言。可点击 [语言支持](https://www.volcengine.com/docs/4640/35107#\%F0\%9F\%93\%A2\%E5\%AE\%9E\%E6\%97\%B6\%E8\%AF\%AD\%E9\%9F\%B3\%E7\%BF\%BB\%E8\%AF\%91) 查看翻译服务最新支持的语种信息。
  ///
  FutureOr<String?> get targetLanguage async {
    return await sendInstanceGet<String?>("targetLanguage");
  }

  set targetLanguage(FutureOr<String?> value) {
    sendInstanceSet("targetLanguage", value);
  }
}

enum MixedStreamVideoCodecType {
  /// @brief H.264 格式，默认值。
  ///
  MIXED_STREAM_VIDEO_CODEC_TYPE_H264,

  /// @brief ByteVC1 格式。
  ///
  MIXED_STREAM_VIDEO_CODEC_TYPE_BYTEVC1(1);

  final dynamic $value;
  const MixedStreamVideoCodecType([this.$value]);
}

enum StreamLayoutMode {
  /// @brief 自动布局
  ///
  AUTO(0),

  /// @brief 自定义
  ///
  CUSTOM(2);

  final dynamic $value;
  const StreamLayoutMode([this.$value]);
}

class RtcUser extends NativeClass {
  static const _$namespace = r'com.ss.bytertc.engine.type.RtcUser';
  static get codegen_$namespace => _$namespace;

  RtcUser([NativeClassOptions? options])
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

  /// @brief 用户 id
  ///
  FutureOr<String?> get userId async {
    return await sendInstanceGet<String?>("userId");
  }

  set userId(FutureOr<String?> value) {
    sendInstanceSet("userId", value);
  }

  /// @brief 元数据
  ///
  FutureOr<String?> get metaData async {
    return await sendInstanceGet<String?>("metaData");
  }

  set metaData(FutureOr<String?> value) {
    sendInstanceSet("metaData", value);
  }
}

enum ColorSpace {
  /// @brief 色彩空间未知
  ///
  UNKNOWN(0),

  /// @brief BT.601 数字编码标准，色彩空间[16-235]
  ///
  BT601_LIMITED_RANGE(1),

  BT601_FULL_RANGE(2),

  /// @brief BT.7091 数字编码标准，颜色空间[16-235]
  ///
  BT709_LIMITED_RANGE(3),

  /// @brief BT.7091 数字编码标准，颜色空间[0-255]
  ///
  BT709_FULL_RANGE(4);

  final dynamic $value;
  const ColorSpace([this.$value]);
}

class EngineConfig extends NativeClass {
  static const _$namespace = r'com.ss.bytertc.engine.data.EngineConfig';
  static get codegen_$namespace => _$namespace;

  EngineConfig([NativeClassOptions? options])
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

  /// @brief 必填，Android Application Context
  ///
  FutureOr<Context?> get context async {
    return await sendInstanceGet<Context?>("context");
  }

  set context(FutureOr<Context?> value) {
    sendInstanceSet("context", value);
  }

  /// @brief 必填，每个应用的唯一标识符，由 RTC 控制台随机生成的。不同的 AppId 生成的实例在 RTC 中进行音视频通话完全独立，无法互通。
  ///
  FutureOr<String?> get appID async {
    return await sendInstanceGet<String?>("appID");
  }

  set appID(FutureOr<String?> value) {
    sendInstanceSet("appID", value);
  }

  /// @brief 选填，如果需要支持外部纹理硬编码，则需要以 `JObject` 方式传入 `eglContext`
  ///
  FutureOr<Object?> get eglContext async {
    return await sendInstanceGet<Object?>("eglContext");
  }

  set eglContext(FutureOr<Object?> value) {
    sendInstanceSet("eglContext", value);
  }

  /// @hidden for internal use only
  /// @brief 选填，so 文件加载地址，如果需要插件库动态加载，可以通过该字段设置动态库地址
  ///
  FutureOr<String?> get nativeLoadPath async {
    return await sendInstanceGet<String?>("nativeLoadPath");
  }

  set nativeLoadPath(FutureOr<String?> value) {
    sendInstanceSet("nativeLoadPath", value);
  }

  /// @brief 选填，私有参数。如需使用请联系技术支持人员。
  ///
  FutureOr<JSONObject?> get parameters async {
    return await sendInstanceGet<JSONObject?>("parameters");
  }

  set parameters(FutureOr<JSONObject?> value) {
    sendInstanceSet("parameters", value);
  }

  /// @brief 游戏场景类型
  ///
  FutureOr<boolean?> get isGameScene async {
    return await sendInstanceGet<boolean?>("isGameScene");
  }

  set isGameScene(FutureOr<boolean?> value) {
    sendInstanceSet("isGameScene", value);
  }
}

class Rectangle extends NativeClass {
  static const _$namespace = r'com.ss.bytertc.engine.video.Rectangle';
  static get codegen_$namespace => _$namespace;

  Rectangle([NativeClassOptions? options])
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

  /// @brief 矩形区域左上角的 x 坐标
  ///
  FutureOr<int?> get x async {
    return await sendInstanceGet<int?>("x");
  }

  set x(FutureOr<int?> value) {
    sendInstanceSet("x", value);
  }

  /// @brief 矩形区域左上角的 y 坐标
  ///
  FutureOr<int?> get y async {
    return await sendInstanceGet<int?>("y");
  }

  set y(FutureOr<int?> value) {
    sendInstanceSet("y", value);
  }

  /// @brief 矩形宽度(px)
  ///
  FutureOr<int?> get width async {
    return await sendInstanceGet<int?>("width");
  }

  set width(FutureOr<int?> value) {
    sendInstanceSet("width", value);
  }

  /// @brief 矩形高度(px)
  ///
  FutureOr<int?> get height async {
    return await sendInstanceGet<int?>("height");
  }

  set height(FutureOr<int?> value) {
    sendInstanceSet("height", value);
  }
}

class ForwardStreamEventInfo extends NativeClass {
  static const _$namespace =
      r'com.ss.bytertc.engine.data.ForwardStreamEventInfo';
  static get codegen_$namespace => _$namespace;

  ForwardStreamEventInfo([NativeClassOptions? options])
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
  FutureOr<String?> get roomId async {
    return await sendInstanceGet<String?>("roomId");
  }

  set roomId(FutureOr<String?> value) {
    sendInstanceSet("roomId", value);
  }

  /// @brief 跨房间转发媒体流过程中该目标房间发生的事件，参看 ForwardStreamEvent{@link #ForwardStreamEvent}。
  ///
  FutureOr<ForwardStreamEvent?> get event async {
    try {
      final result = await sendInstanceGet<ForwardStreamEvent?>("event");
      if (result == null) {
        return null;
      }
      return ForwardStreamEvent.values
          .firstWhere((element) => element == result);
    } catch (e) {
      return null;
    }
  }

  set event(FutureOr<ForwardStreamEvent?> value) {
    sendInstanceSet("event", value);
  }
}

enum CapturePreference {
  /// @brief （默认）自动设置采集参数。 <br>
  ///        SDK 在开启采集时根据服务端下发的采集配置结合编码参数设置最佳采集参数。
  ///
  AUTO(0),

  /// @brief 手动设置采集参数，包括采集分辨率、帧率。
  ///
  MANUAL(1),

  /// @brief 采集参数与编码参数一致，即在 setVideoEncoderConfig{@link #RTCEngine#setVideoEncoderConfig} 中设置的参数。
  ///
  AUTO_PERFORMANCE(2);

  final dynamic $value;
  const CapturePreference([this.$value]);
}

enum SimulcastStreamType {
  /// @brief 弱流，最小分辨率的流。
  ///
  SIMULCAST_STREAM_TYPE_WEAK(0),

  /// @brief 小流
  ///
  SIMULCAST_STREAM_TYPE_LOW(1),

  /// @brief 中流
  ///
  SIMULCAST_STREAM_TYPE_MID(2),

  /// @brief 大流
  ///
  SIMULCAST_STREAM_TYPE_HIGH(3);

  final dynamic $value;
  const SimulcastStreamType([this.$value]);
}

enum AudioRecordingState {
  /// @brief 录制异常
  ///
  AUDIO_RECORDING_STATE_ERROR(0),

  /// @brief 录制进行中
  ///
  AUDIO_RECORDING_STATE_PROCESSING(1),

  /// @brief 已结束录制，并且录制文件保存成功。
  ///
  AUDIO_RECORDING_STATE_SUCCESS(2);

  final dynamic $value;
  const AudioRecordingState([this.$value]);
}

enum FirstFrameSendState {
  /// @brief 发送中
  ///
  FIRST_FRAME_SEND_STATE_SENDING(0),

  /// @brief 发送成功
  ///
  FIRST_FRAME_SEND_STATE_SENT(1),

  /// @brief 发送失败
  ///
  FIRST_FRAME_SEND_STAT_END(2);

  final dynamic $value;
  const FirstFrameSendState([this.$value]);
}

class PixelFormat extends NativeClass {
  static const _$namespace =
      r'com.ss.bytertc.engine.video.IVideoSink.PixelFormat';
  static get codegen_$namespace => _$namespace;

  /// @brief 原始视频帧格式
  ///
  static Future<int> get Original async {
    return await NativeClassUtils.sendStaticGet<int>(
        _$namespace, "Original", "com.volcengine.rtc.hybrid_runtime");
  }

  /// @brief I420 数据格式
  ///
  static Future<int> get I420 async {
    return await NativeClassUtils.sendStaticGet<int>(
        _$namespace, "I420", "com.volcengine.rtc.hybrid_runtime");
  }

  /// @brief RGBA 格式, 一个像素占据 32 位, 字节序为 A8 B8 G8 R8
  ///
  static Future<int> get RGBA async {
    return await NativeClassUtils.sendStaticGet<int>(
        _$namespace, "RGBA", "com.volcengine.rtc.hybrid_runtime");
  }

  PixelFormat([NativeClassOptions? options])
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
}

enum MessageConfig {
  /// @brief 低延时可靠有序消息
  ///
  RELIABLE_ORDERED(0),

  /// @brief 超低延时有序消息
  ///
  UNRELIABLE_ORDERED(1),

  /// @brief 超低延时无序消息
  ///
  UNRELIABLE_UNORDERED(2),

  /// @hidden constructor/destructor
  ///
  value(-1);

  final dynamic $value;
  const MessageConfig([this.$value]);
}

class RTCRoomStats extends NativeClass {
  static const _$namespace = r'com.ss.bytertc.engine.type.RTCRoomStats';
  static get codegen_$namespace => _$namespace;

  RTCRoomStats([NativeClassOptions? options])
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
  ///
  FutureOr<int?> get totalDuration async {
    return await sendInstanceGet<int?>("totalDuration");
  }

  set totalDuration(FutureOr<int?> value) {
    sendInstanceSet("totalDuration", value);
  }

  /// @brief 本地用户的总发送字节数 (bytes)，累计值
  ///
  FutureOr<long?> get txBytes async {
    return await sendInstanceGet<long?>("txBytes");
  }

  set txBytes(FutureOr<long?> value) {
    sendInstanceSet("txBytes", value);
  }

  /// @brief 本地用户的总接收字节数 (bytes)，累计值
  ///
  FutureOr<long?> get rxBytes async {
    return await sendInstanceGet<long?>("rxBytes");
  }

  set rxBytes(FutureOr<long?> value) {
    sendInstanceSet("rxBytes", value);
  }

  /// @brief 发送码率（kbps），获取该数据时的瞬时值
  ///
  FutureOr<int?> get txKBitRate async {
    return await sendInstanceGet<int?>("txKBitRate");
  }

  set txKBitRate(FutureOr<int?> value) {
    sendInstanceSet("txKBitRate", value);
  }

  /// @brief 接收码率（kbps），获取该数据时的瞬时值
  ///
  FutureOr<int?> get rxKBitRate async {
    return await sendInstanceGet<int?>("rxKBitRate");
  }

  set rxKBitRate(FutureOr<int?> value) {
    sendInstanceSet("rxKBitRate", value);
  }

  /// @brief 音频包的发送码率（kbps），获取该数据时的瞬时值
  ///
  FutureOr<int?> get txAudioKBitRate async {
    return await sendInstanceGet<int?>("txAudioKBitRate");
  }

  set txAudioKBitRate(FutureOr<int?> value) {
    sendInstanceSet("txAudioKBitRate", value);
  }

  /// @brief 音频接收码率（kbps），获取该数据时的瞬时值
  ///
  FutureOr<int?> get rxAudioKBitRate async {
    return await sendInstanceGet<int?>("rxAudioKBitRate");
  }

  set rxAudioKBitRate(FutureOr<int?> value) {
    sendInstanceSet("rxAudioKBitRate", value);
  }

  /// @brief 视频发送码率（kbps），获取该数据时的瞬时值
  ///
  FutureOr<int?> get txVideoKBitRate async {
    return await sendInstanceGet<int?>("txVideoKBitRate");
  }

  set txVideoKBitRate(FutureOr<int?> value) {
    sendInstanceSet("txVideoKBitRate", value);
  }

  /// @brief 视频接收码率（kbps），获取该数据时的瞬时值
  ///
  FutureOr<int?> get rxVideoKBitRate async {
    return await sendInstanceGet<int?>("rxVideoKBitRate");
  }

  set rxVideoKBitRate(FutureOr<int?> value) {
    sendInstanceSet("rxVideoKBitRate", value);
  }

  /// @brief 屏幕发送码率（kbps），获取该数据时的瞬时值
  ///
  FutureOr<int?> get txScreenKBitRate async {
    return await sendInstanceGet<int?>("txScreenKBitRate");
  }

  set txScreenKBitRate(FutureOr<int?> value) {
    sendInstanceSet("txScreenKBitRate", value);
  }

  /// @brief 屏幕接收码率（kbps），获取该数据时的瞬时值
  ///
  FutureOr<int?> get rxScreenKBitRate async {
    return await sendInstanceGet<int?>("rxScreenKBitRate");
  }

  set rxScreenKBitRate(FutureOr<int?> value) {
    sendInstanceSet("rxScreenKBitRate", value);
  }

  /// @brief 当前房间内的可见用户人数。
  ///
  FutureOr<int?> get users async {
    return await sendInstanceGet<int?>("users");
  }

  set users(FutureOr<int?> value) {
    sendInstanceSet("users", value);
  }

  /// @brief 当前系统的 CPU 使用率 (\%)
  ///
  FutureOr<double?> get cpuTotalUsage async {
    return await sendInstanceGet<double?>("cpuTotalUsage");
  }

  set cpuTotalUsage(FutureOr<double?> value) {
    sendInstanceSet("cpuTotalUsage", value);
  }

  /// @brief 当前应用的 CPU 使用率 (\%)
  ///
  FutureOr<double?> get cpuAppUsage async {
    return await sendInstanceGet<double?>("cpuAppUsage");
  }

  set cpuAppUsage(FutureOr<double?> value) {
    sendInstanceSet("cpuAppUsage", value);
  }

  /// @brief 当前应用的上行丢包率，取值范围为 [0, 1]
  ///
  FutureOr<double?> get txLostrate async {
    return await sendInstanceGet<double?>("txLostrate");
  }

  set txLostrate(FutureOr<double?> value) {
    sendInstanceSet("txLostrate", value);
  }

  /// @brief 当前应用的下行丢包率，取值范围为 [0, 1]
  ///
  FutureOr<double?> get rxLostrate async {
    return await sendInstanceGet<double?>("rxLostrate");
  }

  set rxLostrate(FutureOr<double?> value) {
    sendInstanceSet("rxLostrate", value);
  }

  /// @brief 客户端到服务端数据传输的往返时延（单位 ms）
  ///
  FutureOr<int?> get rtt async {
    return await sendInstanceGet<int?>("rtt");
  }

  set rtt(FutureOr<int?> value) {
    sendInstanceSet("rtt", value);
  }

  /// @hidden currently not available
  /// @brief 系统上行网络抖动（ms）
  ///
  FutureOr<int?> get txJitter async {
    return await sendInstanceGet<int?>("txJitter");
  }

  set txJitter(FutureOr<int?> value) {
    sendInstanceSet("txJitter", value);
  }

  /// @hidden currently not available
  /// @brief 系统下行网络抖动（ms）
  ///
  FutureOr<int?> get rxJitter async {
    return await sendInstanceGet<int?>("rxJitter");
  }

  set rxJitter(FutureOr<int?> value) {
    sendInstanceSet("rxJitter", value);
  }

  /// @brief 蜂窝路径发送的码率 (kbps)，为获取该数据时的瞬时值
  ///
  FutureOr<int?> get txCellularKBitrate async {
    return await sendInstanceGet<int?>("txCellularKBitrate");
  }

  set txCellularKBitrate(FutureOr<int?> value) {
    sendInstanceSet("txCellularKBitrate", value);
  }

  /// @brief 蜂窝路径接收码率 (kbps)，为获取该数据时的瞬时值
  ///
  FutureOr<int?> get rxCellularKBitrate async {
    return await sendInstanceGet<int?>("rxCellularKBitrate");
  }

  set rxCellularKBitrate(FutureOr<int?> value) {
    sendInstanceSet("rxCellularKBitrate", value);
  }

  /// @detail api
  /// @brief 重置所有的 RTCRoomStats{@link #RTCRoomStats} 成员变量的值，重新开始统计。
  ///

  FutureOr<void> reset() async {
    return await nativeCall('reset', []);
  }
}

enum AudioMixingState {
  /// @brief 混音已加载
  ///
  AUDIO_MIXING_STATE_PRELOADED(0),

  /// @brief 混音正在播放
  ///
  AUDIO_MIXING_STATE_PLAYING(1),

  /// @brief 混音暂停
  ///
  AUDIO_MIXING_STATE_PAUSED(2),

  /// @brief 混音停止
  ///
  AUDIO_MIXING_STATE_STOPPED(3),

  /// @brief 混音播放失败
  ///
  AUDIO_MIXING_STATE_FAILED(4),

  /// @brief 混音播放结束
  ///
  AUDIO_MIXING_STATE_FINISHED(5),

  /// @brief 准备 PCM 混音
  ///
  AUDIO_MIXING_STATE_PCM_ENABLED(6),

  /// @brief PCM 混音播放结束
  ///
  AUDIO_MIXING_STATE_PCM_DISABLED(7);

  final dynamic $value;
  const AudioMixingState([this.$value]);
}

enum MirrorMode {
  /// @brief 不开启（默认设置）
  ///
  MIRROR_MODE_OFF(0),

  /// @brief 开启
  ///
  MIRROR_MODE_ON(1);

  final dynamic $value;
  const MirrorMode([this.$value]);
}

class ClientMixedStreamConfig extends NativeClass {
  static const _$namespace =
      r'com.ss.bytertc.engine.live.ClientMixedStreamConfig';
  static get codegen_$namespace => _$namespace;

  ClientMixedStreamConfig([NativeClassOptions? options])
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
  /// @detail api
  /// @author liujingchao
  /// @brief 设置客户端合流回调视频格式。设置为系统不支持的格式时，自动回调系统默认格式。
  /// @param videoFormat 客户端合流回调视频格式，参看 MixedStreamClientMixVideoFormat{@link #MixedStreamClientMixVideoFormat}。
  ///
  FutureOr<boolean?> get useAudioMixer async {
    return await sendInstanceGet<boolean?>("useAudioMixer");
  }

  set useAudioMixer(FutureOr<boolean?> value) {
    sendInstanceSet("useAudioMixer", value);
  }

  /// @hidden for internal use only
  /// @detail api
  /// @author liujingchao
  /// @brief 设置客户端合流是否使用混音。
  /// @param useAudioMixer 是否使用混音，默认为 true。
  /// @return 参看 MixedStreamClientMixConfig{@link #MixedStreamClientMixConfig}。
  ///
  FutureOr<MixedStreamClientMixVideoFormat?> get videoFormat async {
    try {
      final result = await sendInstanceGet<MixedStreamClientMixVideoFormat?>(
          "videoFormat");
      if (result == null) {
        return null;
      }
      return MixedStreamClientMixVideoFormat.values
          .firstWhere((element) => element == result);
    } catch (e) {
      return null;
    }
  }

  set videoFormat(FutureOr<MixedStreamClientMixVideoFormat?> value) {
    sendInstanceSet("videoFormat", value);
  }

  /// @hidden constructor/destructor
  ///

  FutureOr<boolean> getClientMixedStreamConfigUseAudioMixer() async {
    return await nativeCall('getClientMixedStreamConfigUseAudioMixer', []);
  }

  /// @hidden constructor/destructor
  ///

  FutureOr<int> getClientMixedStreamConfigVideoFormat() async {
    return await nativeCall('getClientMixedStreamConfigVideoFormat', []);
  }
}

enum UserOnlineStatus {
  /// @brief 对端用户离线 <br>
  ///        对端用户已经调用 `logout`，或者没有调用 `login` 进行登录
  ///
  USER_ONLINE_STATUS_OFFLINE(0),

  /// @brief 对端用户在线 <br>
  ///        对端用户调用 `login` 登录，并且连接状态正常。
  ///
  USER_ONLINE_STATUS_ONLINE(1),

  /// @brief 无法获取对端用户在线状态 <br>
  ///        发生级联错误、对端用户在线状态异常时返回
  ///
  USER_ONLINE_STATUS_UNREACHABLE(2);

  final dynamic $value;
  const UserOnlineStatus([this.$value]);
}

enum ZoomConfigType {
  /// @brief 设置缩放系数
  ///
  ZOOM_FOCUS_OFFSET(0),

  /// @brief 设置移动步长
  ///
  ZOOM_MOVE_OFFSET(1);

  final dynamic $value;
  const ZoomConfigType([this.$value]);
}

enum RecordingType {
  /// @brief 只录制音频
  ///
  RECORD_AUDIO_ONLY(0),

  /// @brief 只录制视频
  ///
  RECORD_VIDEO_ONLY(1),

  /// @brief 同时录制音频和视频
  ///
  RECORD_VIDEO_AND_AUDIO(2);

  final dynamic $value;
  const RecordingType([this.$value]);
}

class MixedStreamTaskInfo extends NativeClass {
  static const _$namespace = r'com.ss.bytertc.engine.live.MixedStreamTaskInfo';
  static get codegen_$namespace => _$namespace;

  MixedStreamTaskInfo([NativeClassOptions? options])
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

  /// @brief 任务类型。
  /// @param targetType 任务类型，合流转推 CDN 还是 WTN 流。
  ///

  FutureOr<MixedStreamTaskInfo> setTargetType(
      MixedStreamPushTargetType targetType) async {
    final result = await nativeCall('setTargetType', [targetType.$value]);
    return packObject(
        result,
        () => MixedStreamTaskInfo(
            const NativeClassOptions([], disableInit: true)));
  }

  /// @brief 任务类型，合流转推 CDN 还是 WTN 流。
  ///

  FutureOr<MixedStreamPushTargetType> getTargetType() async {
    return await nativeCall('getTargetType', []);
  }

  /// @brief 设置任务 ID <br>
  /// 对于 WTN 流任务，该值代表 WTN 流 ID。你可以通过该 ID，指定需要订阅的 WTN 流。
  /// @param taskId 任务 ID
  ///

  FutureOr<MixedStreamTaskInfo> setTaskId(String taskId) async {
    final result = await nativeCall('setTaskId', [taskId]);
    return packObject(
        result,
        () => MixedStreamTaskInfo(
            const NativeClassOptions([], disableInit: true)));
  }

  /// @brief 任务 ID <br>
  /// 对于 WTN 流任务，该值代表 WTN 流 ID。你可以通过该 ID，指定需要订阅的 WTN 流。
  ///

  FutureOr<String> getTaskId() async {
    return await nativeCall('getTaskId', []);
  }
}

enum WTNSubscribeStateChangeReason {
  SUBSCRIBE(0),

  UNSUBSCRIBE(1300),

  REMOTE_UNPUBLISH(1301),

  OVER_CLIENT_SUBSCRIBE_STREAM_LIMIT(1310),

  OVER_STREAM_SUBSCRIBE_USER_LIMIT(1311),

  OVER_STREAM_SUBSCRIBE_REQUEST_LIMIT(1312);

  final dynamic $value;
  const WTNSubscribeStateChangeReason([this.$value]);
}

enum MixedStreamAudioProfile {
  /// @brief AAC-LC 规格，默认值。
  ///
  MIXED_STREAM_AUDIO_PROFILE_LC,

  /// @brief HE-AAC v1 规格。
  ///
  MIXED_STREAM_AUDIO_PROFILE_HEV1(1),

  /// @brief HE-AAC v2 规格。
  ///
  MIXED_STREAM_AUDIO_PROFILE_HEV2(2);

  final dynamic $value;
  const MixedStreamAudioProfile([this.$value]);
}

class VoiceEqualizationConfig extends NativeClass {
  static const _$namespace =
      r'com.ss.bytertc.engine.type.VoiceEqualizationConfig';
  static get codegen_$namespace => _$namespace;

  VoiceEqualizationConfig([NativeClassOptions? options])
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

  /// @brief 频带。参看 VoiceEqualizationBandFrequency{@link #VoiceEqualizationBandFrequency}。
  ///
  FutureOr<VoiceEqualizationBandFrequency?> get frequency async {
    try {
      final result =
          await sendInstanceGet<VoiceEqualizationBandFrequency?>("frequency");
      if (result == null) {
        return null;
      }
      return VoiceEqualizationBandFrequency.values
          .firstWhere((element) => element == result);
    } catch (e) {
      return null;
    }
  }

  set frequency(FutureOr<VoiceEqualizationBandFrequency?> value) {
    sendInstanceSet("frequency", value);
  }

  /// @brief 频带增益（dB）。取值范围是 `[-15, 15]`。
  ///
  FutureOr<int?> get gain async {
    return await sendInstanceGet<int?>("gain");
  }

  set gain(FutureOr<int?> value) {
    sendInstanceSet("gain", value);
  }
}

class LocalVideoStats extends NativeClass {
  static const _$namespace = r'com.ss.bytertc.engine.type.LocalVideoStats';
  static get codegen_$namespace => _$namespace;

  LocalVideoStats([NativeClassOptions? options])
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
  ///
  FutureOr<float?> get sentKBitrate async {
    return await sendInstanceGet<float?>("sentKBitrate");
  }

  set sentKBitrate(FutureOr<float?> value) {
    sendInstanceSet("sentKBitrate", value);
  }

  /// @brief 采集帧率。此次统计周期内的视频采集帧率，单位为 fps 。
  ///
  FutureOr<int?> get inputFrameRate async {
    return await sendInstanceGet<int?>("inputFrameRate");
  }

  set inputFrameRate(FutureOr<int?> value) {
    sendInstanceSet("inputFrameRate", value);
  }

  /// @brief 发送帧率。此次统计周期内实际发送的分辨率最大的视频流的视频发送帧率，单位为 fps 。
  ///
  FutureOr<int?> get sentFrameRate async {
    return await sendInstanceGet<int?>("sentFrameRate");
  }

  set sentFrameRate(FutureOr<int?> value) {
    sendInstanceSet("sentFrameRate", value);
  }

  /// @brief 编码器输出帧率。当前编码器在此次统计周期内实际发送的分辨率最大的视频流的输出帧率，单位为 fps 。
  ///
  FutureOr<int?> get encoderOutputFrameRate async {
    return await sendInstanceGet<int?>("encoderOutputFrameRate");
  }

  set encoderOutputFrameRate(FutureOr<int?> value) {
    sendInstanceSet("encoderOutputFrameRate", value);
  }

  /// @brief 本地渲染帧率。此次统计周期内的本地视频渲染帧率，单位为 fps 。
  ///
  FutureOr<int?> get rendererOutputFrameRate async {
    return await sendInstanceGet<int?>("rendererOutputFrameRate");
  }

  set rendererOutputFrameRate(FutureOr<int?> value) {
    sendInstanceSet("rendererOutputFrameRate", value);
  }

  /// @brief 统计间隔，单位为 ms 。 <br>
  ///        此字段用于设置回调的统计周期，默认设置为 2s 。
  ///
  FutureOr<int?> get statsInterval async {
    return await sendInstanceGet<int?>("statsInterval");
  }

  set statsInterval(FutureOr<int?> value) {
    sendInstanceSet("statsInterval", value);
  }

  /// @brief 视频丢包率。此次统计周期内的视频上行丢包率，取值范围： [0，1] 。
  ///
  FutureOr<float?> get videoLossRate async {
    return await sendInstanceGet<float?>("videoLossRate");
  }

  set videoLossRate(FutureOr<float?> value) {
    sendInstanceSet("videoLossRate", value);
  }

  /// @brief 往返时延，单位为 ms 。
  ///
  FutureOr<int?> get rtt async {
    return await sendInstanceGet<int?>("rtt");
  }

  set rtt(FutureOr<int?> value) {
    sendInstanceSet("rtt", value);
  }

  /// @brief 视频编码码率。此次统计周期内的实际发送的分辨率最大的视频流视频编码码率，单位为 Kbps 。
  ///
  FutureOr<int?> get encodedBitrate async {
    return await sendInstanceGet<int?>("encodedBitrate");
  }

  set encodedBitrate(FutureOr<int?> value) {
    sendInstanceSet("encodedBitrate", value);
  }

  /// @brief 实际发送的分辨率最大的视频流的视频编码宽度，单位为 px 。
  ///
  FutureOr<int?> get encodedFrameWidth async {
    return await sendInstanceGet<int?>("encodedFrameWidth");
  }

  set encodedFrameWidth(FutureOr<int?> value) {
    sendInstanceSet("encodedFrameWidth", value);
  }

  /// @brief 实际发送的分辨率最大的视频流的视频编码高度，单位为 px 。
  ///
  FutureOr<int?> get encodedFrameHeight async {
    return await sendInstanceGet<int?>("encodedFrameHeight");
  }

  set encodedFrameHeight(FutureOr<int?> value) {
    sendInstanceSet("encodedFrameHeight", value);
  }

  /// @brief 此次统计周期内实际发送的分辨率最大的视频流的发送的视频帧总数。
  ///
  FutureOr<int?> get encodedFrameCount async {
    return await sendInstanceGet<int?>("encodedFrameCount");
  }

  set encodedFrameCount(FutureOr<int?> value) {
    sendInstanceSet("encodedFrameCount", value);
  }

  /// @brief 视频的编码类型，具体参考 VideoCodecType{@link #VideoCodecType-2} 。
  ///
  FutureOr<int?> get codecType async {
    return await sendInstanceGet<int?>("codecType");
  }

  set codecType(FutureOr<int?> value) {
    sendInstanceSet("codecType", value);
  }

  /// @brief 所属用户的媒体流是否为屏幕流。你可以知道当前统计数据来自主流还是屏幕流。
  ///
  FutureOr<boolean?> get isScreen async {
    return await sendInstanceGet<boolean?>("isScreen");
  }

  set isScreen(FutureOr<boolean?> value) {
    sendInstanceSet("isScreen", value);
  }

  /// @brief 视频上行网络抖动，单位为 ms 。
  ///
  FutureOr<int?> get jitter async {
    return await sendInstanceGet<int?>("jitter");
  }

  set jitter(FutureOr<int?> value) {
    sendInstanceSet("jitter", value);
  }

  /// @hidden for internal use
  /// @brief 上行视频当前降噪模式所处的状态 (0: 关/1: 开)。
  ///
  FutureOr<int?> get videoDenoiseMode async {
    return await sendInstanceGet<int?>("videoDenoiseMode");
  }

  set videoDenoiseMode(FutureOr<int?> value) {
    sendInstanceSet("videoDenoiseMode", value);
  }
}

enum VideoSourceType {
  /// @brief 自定义采集视频源
  ///
  VIDEO_SOURCE_TYPE_EXTERNAL(0),

  /// @brief 内部采集视频源
  ///
  VIDEO_SOURCE_TYPE_INTERNAL(1),

  VIDEO_SOURCE_TYPE_ENCODED_WITH_SIMULCAST(2),

  VIDEO_SOURCE_TYPE_ENCODED_WITHOUT_SIMULCAST(3);

  final dynamic $value;
  const VideoSourceType([this.$value]);
}

enum MixedStreamPushMode {
  /// @brief 无用户发布媒体流时，发起合流任务无效。默认设置。 <br>
  ///        当有用户发布媒体流时，才能发起合流任务。
  ///
  ON_STREAM(0),

  /// @brief 无用户发布媒体流时，可以使用占位图发起合流任务。 <br>
  ///        占位图设置参看 alternateImageURL{@link #MixedStreamLayoutRegionConfig#alternateImageURL} 和 alternateImageFillMode{@link #MixedStreamLayoutRegionConfig#alternateImageFillMode}。
  ///
  ON_START_REQUEST(1);

  final dynamic $value;
  const MixedStreamPushMode([this.$value]);
}

enum MixedStreamVideoType {
  /// @brief 主流。包括： <br>
  ///        - 由摄像头/麦克风通过内部采集机制，采集到的流
  ///        - 通过自定义采集，采集到的流。
  ///
  MIXED_STREAM_VIDEO_TYPE_MAIN(0),

  /// @brief 屏幕流。
  ///
  MIXED_STREAM_VIDEO_TYPE_SCREEN(1);

  final dynamic $value;
  const MixedStreamVideoType([this.$value]);
}

enum VoiceEqualizationBandFrequency {
  /// @brief 中心频率为 31Hz 的频带。
  ///
  VOICE_EQUALIZATION_BAND_FREQUENCY_31(0),

  /// @brief 中心频率为 62Hz 的频带。
  ///
  VOICE_EQUALIZATION_BAND_FREQUENCY_62(1),

  /// @brief 中心频率为 125Hz 的频带。
  ///
  VOICE_EQUALIZATION_BAND_FREQUENCY_125(2),

  /// @brief 中心频率为 250Hz 的频带。
  ///
  VOICE_EQUALIZATION_BAND_FREQUENCY_250(3),

  /// @brief 中心频率为 500Hz 的频带。
  ///
  VOICE_EQUALIZATION_BAND_FREQUENCY_500(4),

  /// @brief 中心频率为 1kHz 的频带。
  ///
  VOICE_EQUALIZATION_BAND_FREQUENCY_1K(5),

  /// @brief 中心频率为 2kHz 的频带。
  ///
  VOICE_EQUALIZATION_BAND_FREQUENCY_2K(6),

  /// @brief 中心频率为 4kHz 的频带。
  ///
  VOICE_EQUALIZATION_BAND_FREQUENCY_4K(7),

  /// @brief 中心频率为 8kHz 的频带。
  ///
  VOICE_EQUALIZATION_BAND_FREQUENCY_8K(8),

  /// @brief 中心频率为 16kHz 的频带。
  ///
  VOICE_EQUALIZATION_BAND_FREQUENCY_16K(9);

  final dynamic $value;
  const VoiceEqualizationBandFrequency([this.$value]);
}

enum EncoderPreference {
  /// @brief 无偏好。不降低帧率和分辨率。
  ///
  DISABLED(0),

  /// @brief 优先保障帧率。适用于动态画面。
  ///
  MAINTAIN_FRAMERATE(1),

  /// @brief 清晰模式，优先保障分辨率。适用于静态画面。
  ///
  MAINTAIN_QUALITY(2),

  /// @brief 平衡帧率与分辨率。
  /// 对于屏幕流来说是无偏好。不降低帧率和分辨率。
  ///
  AUTO(3);

  final dynamic $value;
  const EncoderPreference([this.$value]);
}

enum RenderError {
  /// @brief 渲染正常
  ///
  RENDER_ERROR_OK(0),

  /// @brief Android 外部直显时使用了内部 surface
  ///
  RENDER_ERROR_USING_INTERNAL_SURFACE(-1),

  /// @brief 设置 Android 外部直显时使用软解
  ///
  RENDER_ERROR_USING_SOFTWARE_DECODER(-2);

  final dynamic $value;
  const RenderError([this.$value]);
}

enum FirstFramePlayState {
  /// @brief 播放中
  ///
  FIRST_FRAME_PLAY_STATE_PLAYING(0),

  /// @brief 播放成功
  ///
  FIRST_FRAME_PLAY_STATE_PLAYED(1),

  /// @brief 播放失败
  ///
  FIRST_FRAME_PLAY_STATE_END(2);

  final dynamic $value;
  const FirstFramePlayState([this.$value]);
}

class ExpressionDetectResult extends NativeClass {
  static const _$namespace =
      r'com.ss.bytertc.engine.video.ExpressionDetectResult';
  static get codegen_$namespace => _$namespace;

  /// @brief 人脸信息存储上限，最多可存储 10 个人脸信息
  ///
  static Future<int> get MAX_COUNT async {
    return await NativeClassUtils.sendStaticGet<int>(
        _$namespace, "MAX_COUNT", "com.volcengine.rtc.hybrid_runtime");
  }

  ExpressionDetectResult([NativeClassOptions? options])
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
  ///
  FutureOr<int?> get detectResult async {
    return await sendInstanceGet<int?>("detectResult");
  }

  set detectResult(FutureOr<int?> value) {
    sendInstanceSet("detectResult", value);
  }

  /// @brief 识别到的人脸数量。
  ///
  FutureOr<int?> get faceCount async {
    return await sendInstanceGet<int?>("faceCount");
  }

  set faceCount(FutureOr<int?> value) {
    sendInstanceSet("faceCount", value);
  }

  /// @brief 特征识别信息。数组的长度和检测到的人脸数量一致。参看 ExpressionDetectInfo{@link #ExpressionDetectInfo}。
  ///
  FutureOr<Array<ExpressionDetectInfo>?> get detectInfo async {
    try {
      final result =
          await sendInstanceGet<Array<ExpressionDetectInfo>?>("detectInfo");
      if (result == null) {
        return null;
      }
      final list = result.map((e) => packObject(
          e,
          () => ExpressionDetectInfo(
              const NativeClassOptions([], disableInit: true))));
      return list.toList();
    } catch (e) {
      return null;
    }
  }

  set detectInfo(FutureOr<Array<ExpressionDetectInfo>?> value) {
    sendInstanceSet("detectInfo", value);
  }
}

enum EchoTestResult {
  /// @brief 接收到采集的音视频的回放，通话回路检测成功
  ///
  ECHO_TEST_SUCCESS(0),

  /// @brief 测试超过 60s 仍未完成，已自动停止
  ///
  ECHO_TEST_TIMEOUT(1),

  /// @brief 上一次测试结束和下一次测试开始之间的时间间隔少于 5s
  ///
  ECHO_TEST_INTERVAL_SHORT(2),

  /// @brief 音频采集异常
  ///
  ECHO_TEST_AUDIO_DEVICE_ERROR(3),

  /// @brief 视频采集异常
  ///
  ECHO_TEST_VIDEO_DEVICE_ERROR(4),

  /// @brief 音频接收异常
  ///
  ECHO_TEST_AUDIO_RECEIVE_ERROR(5),

  /// @brief 视频接收异常
  ///
  ECHO_TEST_VIDEO_RECEIVE_ERROR(6),

  /// @brief 内部错误，不可恢复
  ///
  ECHO_TEST_INTERNAL_ERROR(7);

  final dynamic $value;
  const EchoTestResult([this.$value]);
}

enum EffectBeautyMode {
  /// @brief 美白。
  ///
  WHITE(0),

  /// @brief 磨皮。
  ///
  SMOOTH(1),

  /// @brief 锐化。
  ///
  SHARPEN(2),

  /// @valid since 3.55
  /// @brief 清晰，需集成 v4.4.2+ 版本的特效 SDK。
  ///
  CLEAR(3);

  final dynamic $value;
  const EffectBeautyMode([this.$value]);
}

enum HardwareEchoDetectionResult {
  /// @brief 主动调用 `stopHardwareEchoDetection` 结束流程时，未有回声检测结果。
  ///
  HARDWARE_ECHO_RESULT_CANCELED(0),

  /// @brief 未检测出结果。建议重试，如果仍然失败请联系技术支持协助排查。
  ///
  HARDWARE_ECHO_RESULT_UNKNOWN(1),

  /// @brief 无回声
  ///
  HARDWARE_ECHO_RESULT_NORMAL(2),

  /// @brief 有回声。可通过 UI 建议用户使用耳机设备入会。
  ///
  HARDWARE_ECHO_RESULT_POOR(3);

  final dynamic $value;
  const HardwareEchoDetectionResult([this.$value]);
}

enum MediaDeviceWarning {
  /// @brief 无警告
  ///
  MEDIA_DEVICE_WARNING_OK(0),

  /// @brief 非法设备操作。在使用外部设备时，调用了 SDK 内部设备 API。
  ///
  MEDIA_DEVICE_WARNING_OPERATION_DENIED(1),

  /// @brief 采集到的数据为静音帧。
  ///
  MEDIA_DEVICE_WARNING_CAPTURE_SILENCE(2),

  /// @brief Android 特有的静音，系统层面的静音上报
  ///
  MEDIA_DEVICE_WARNING_ANDROID_SYS_SILENCE(3),

  /// @brief Android 特有的静音消失
  ///
  MEDIA_DEVICE_WARNING_ANDROID_SYS_SILENCE_DISAPPEAR(4),

  /// @hidden for internal use only
  /// @brief 音量过大，超过设备采集范围。建议降低麦克风音量或者降低声源音量。
  ///
  MEDIA_DEVICE_WARNING_DETECT_CLIPPING(10),

  /// @brief 通话中出现回声现象。 <br>
  ///        当 ChannelProfile{@link #ChannelProfile} 为 `CHANNEL_PROFIEL_MEETING` 和 `CHANNEL_PROFILE_MEETING_ROOM`，且 AEC 关闭时，SDK 自动启动回声检测，如果检测到回声问题，将通过 `onAudioDeviceWarning` 返回本枚举值。
  ///
  MEDIA_DEVICE_WARNING_DETECT_LEAK_ECHO(11),

  /// @hidden for internal use only
  /// @brief 低信噪比
  ///
  MEDIA_DEVICE_WARNING_DETECT_LOW_SNR(12),

  /// @hidden for internal use only
  /// @brief 采集插零现象
  ///
  MEDIA_DEVICE_WARNING_DETECT_INSERT_SILENCE(13),

  /// @hidden for internal use only
  /// @brief 设备采集静音
  ///
  MEDIA_DEVICE_WARNING_CAPTURE_DETECT_SILENCE(14),

  /// @hidden for internal use only
  /// @brief 设备采集静音消失
  ///
  MEDIA_DEVICE_WARNING_CAPTURE_DETECT_SILENCE_DISAPPEAR(15),

  /// @brief 啸叫。触发该回调的情况如下： <br>
  ///          - 不支持啸叫抑制的房间模式下，检测到啸叫；
  ///          - 支持啸叫抑制的房间模式下，检测到未被抑制的啸叫。
  ///        仅 CHANNEL_PROFILE_COMMUNICATION(0)、CHANNEL_PROFIEL_MEETING(16)、CHANNEL_PROFILE_MEETING_ROOM(17) 三种房间模式支持啸叫抑制。 <br>
  ///        建议提醒用户检查客户端的距离或将麦克风和扬声器调至静音。
  ///
  MEDIA_DEVICE_WARNING_CAPTURE_DETECT_HOWLING(16),

  /// @brief 当前 AudioScenario 不支持更改音频路由，设置音频路由失败
  ///
  MEDIA_DEVICE_WARNING_SET_AUDIO_ROUTE_INVALID_SCENARIO(20),

  /// @brief 音频设备不存在，设置音频路由失败
  ///
  MEDIA_DEVICE_WARNING_SET_AUDIO_ROUTE_NOT_EXISTS(21),

  /// @brief 音频路由被系统或其他应用占用，设置音频路由失败
  ///
  MEDIA_DEVICE_WARNING_SET_AUDIO_ROUTE_FAILED_BY_PRIORITY(22),

  /// @brief 当前非通话模式 AUDIO_SCENARIO_COMMUNICATION(2)，不支持设置音频路由
  ///
  MEDIA_DEVICE_WARNING_SET_AUDIO_ROUTE_NOT_VOIP_MODE(23),

  /// @brief 音频设备未启动，设置音频路由失败
  ///
  MEDIA_DEVICE_WARNING_SET_AUDIO_ROUTE_DEVICE_NOT_START(24);

  final dynamic $value;
  const MediaDeviceWarning([this.$value]);
}

enum PublishStateChangeReason {
  /// @brief 用户调用发布
  ///
  PUBLISH(0),

  /// @brief 用户取消发布
  ///
  UNPUBLISH(1),

  /// @brief 发布 token 没有权限
  ///
  NO_PUBLISH_PERMISSION(2),

  /// @brief 发布流总数超过上限
  ///
  OVER_STREAM_PUBLISH_LIMIT(3),

  /// @brief 将一路流发布到多个房间时，其中一个房间取消发布失败。
  ///
  MULTIROOM_UNPUBLISH_FAILED(4),

  /// @brief 服务器错误导致发布失败
  ///
  PUBLISH_STREAM_FAILED(5),

  /// @brief 观众尝试发布操作
  ///
  PUBLISH_STREAM_FORBIDEN(6),

  /// @brief 用户已经在其他房间发布过流，或者用户正在发布。
  ///
  USER_IN_PUBLISH(7),

  /// @brief 用户已经在其他房间发布过流，或者用户正在发布。
  ///
  STREAM_PUBLISH_BY_OTHER(8),

  /// @brief 流 ID 无效
  ///
  STREAM_ID_INVALID(9);

  final dynamic $value;
  const PublishStateChangeReason([this.$value]);
}

class SubscribeVideoConfig extends NativeClass {
  static const _$namespace = r'com.ss.bytertc.engine.SubscribeVideoConfig';
  static get codegen_$namespace => _$namespace;

  SubscribeVideoConfig([NativeClassOptions? options])
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

  /// @hidden constructor/destructor
  ///

  FutureOr<int> getVideoIndex() async {
    return await nativeCall('getVideoIndex', []);
  }

  /// @hidden constructor/destructor
  ///

  FutureOr<int> getPriority() async {
    return await nativeCall('getPriority', []);
  }
}

enum SnapshotErrorCode {
  /// @brief 截图成功。
  ///
  OK(0),

  /// @brief 截图错误。生成图片数据失败或 RGBA 编码失败。
  ///
  CREATE_FAIL(-1),

  /// @brief 截图错误。流无效。
  ///
  STREAM_INVALID(-2),

  /// @brief 截图错误。截图超时，超时时间 1 秒。
  ///
  TIMEOUT(-3),

  /// @brief 截图错误。图片保存失败。
  ///
  FILE_SAVE_ERROR(-4);

  final dynamic $value;
  const SnapshotErrorCode([this.$value]);
}

class IVideoFrame extends NativeClass {
  static const _$namespace = r'com.ss.bytertc.engine.video.IVideoFrame';
  static get codegen_$namespace => _$namespace;

  IVideoFrame([NativeClassOptions? options])
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
  /// @brief 获取视频缓冲区类型，参看 VideoBufferType{@link #VideoBufferType}
  ///

  FutureOr<VideoBufferType> bufferType() async {
    return await nativeCall('bufferType', []);
  }

  /// @detail api
  /// @brief 获取视频帧像素格式，参看 VideoPixelFormat{@link #VideoPixelFormat}
  ///

  FutureOr<VideoPixelFormat> pixelFormat() async {
    return await nativeCall('pixelFormat', []);
  }

  /// @detail api
  /// @brief 获取视频内容类型
  /// @return 视频内容类型，参看 VideoContentType{@link #VideoContentType}。
  ///

  FutureOr<VideoContentType> contentType() async {
    return await nativeCall('contentType', []);
  }

  /// @detail api
  /// @brief 获取视频帧时间戳，单位：微秒
  ///

  FutureOr<long> timestampUs() async {
    return await nativeCall('timestampUs', []);
  }

  /// @detail api
  /// @brief 获取视频帧宽度
  ///

  FutureOr<int> width() async {
    return await nativeCall('width', []);
  }

  /// @detail api
  /// @brief 获取视频帧高度
  ///

  FutureOr<int> height() async {
    return await nativeCall('height', []);
  }

  /// @detail api
  /// @brief 获取视频帧旋转角度
  ///

  FutureOr<VideoRotation> rotation() async {
    return await nativeCall('rotation', []);
  }

  /// @detail api
  /// @brief 获取视频帧的摄像头位置信息，参看 CameraId{@link #CameraId}
  ///

  FutureOr<CameraId> cameraId() async {
    return await nativeCall('cameraId', []);
  }

  /// @detail api
  /// @brief 获取视频平面数
  ///

  FutureOr<int> numberOfPlanes() async {
    return await nativeCall('numberOfPlanes', []);
  }

  /// @detail api
  /// @brief 获取视频帧平面数组
  /// @param planeIndex plane 索引
  ///

  FutureOr<ByteBuffer> planeData(int planeIndex) async {
    return await nativeCall('planeData', [planeIndex]);
  }

  /// @detail api
  /// @brief 获取视频帧平面相邻两行图像数据之间的内存长度（单位字节）
  /// @param planeIndex plane 索引
  ///

  FutureOr<int> planeStride(int planeIndex) async {
    return await nativeCall('planeStride', [planeIndex]);
  }

  /// @detail api
  /// @brief 获取 SEI 数据
  ///

  FutureOr<ByteBuffer> seiData() async {
    return await nativeCall('seiData', []);
  }

  /// @detail api
  /// @brief 获取纹理 ID
  ///

  FutureOr<int> textureId() async {
    return await nativeCall('textureId', []);
  }

  /// @detail api
  /// @brief 获取纹理矩阵
  ///

  FutureOr<Array<float>> textureMatrix() async {
    return await nativeCall('textureMatrix', []);
  }

  /// @detail api
  /// @brief 获取EGLContext
  ///

  FutureOr<EGLContext> eglContext() async {
    return await nativeCall('eglContext', []);
  }

  /// @detail api
  /// @brief 视频帧引用计数加一
  /// @note 视频帧消费者希望对视频帧进行异步处理时（例如切换线程进行渲染），需要调用此接口增加引用计数。异步处理结束则需要调用 releaseRef 使引用计数减1
  ///

  FutureOr<void> addRef() async {
    return await nativeCall('addRef', []);
  }

  /// @detail api
  /// @brief 视频帧引用计数减一
  /// @note 视频帧引用计数减为 0 时，视频帧对象会被释放。视频帧对象释放后，不应该继续使用视频帧。
  ///

  FutureOr<long> releaseRef() async {
    return await nativeCall('releaseRef', []);
  }
}

class RecordingConfig extends NativeClass {
  static const _$namespace = r'com.ss.bytertc.engine.data.RecordingConfig';
  static get codegen_$namespace => _$namespace;

  RecordingConfig([NativeClassOptions? options])
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
  ///
  FutureOr<String?> get dirPath async {
    return await sendInstanceGet<String?>("dirPath");
  }

  set dirPath(FutureOr<String?> value) {
    sendInstanceSet("dirPath", value);
  }

  /// @brief 录制存储文件格式，参看 RecordingFileType{@link #RecordingFileType}
  ///
  FutureOr<RecordingFileType?> get recordingFileType async {
    try {
      final result =
          await sendInstanceGet<RecordingFileType?>("recordingFileType");
      if (result == null) {
        return null;
      }
      return RecordingFileType.values
          .firstWhere((element) => element == result);
    } catch (e) {
      return null;
    }
  }

  set recordingFileType(FutureOr<RecordingFileType?> value) {
    sendInstanceSet("recordingFileType", value);
  }
}

enum RecordingFileType {
  /// @brief aac 格式文件
  ///
  AAC(0),

  /// @brief mp4 格式文件
  ///
  MP4(1);

  final dynamic $value;
  const RecordingFileType([this.$value]);
}

class RemoteStreamSwitch extends NativeClass {
  static const _$namespace = r'com.ss.bytertc.engine.type.RemoteStreamSwitch';
  static get codegen_$namespace => _$namespace;

  RemoteStreamSwitch([NativeClassOptions? options])
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

  /// @brief 订阅的音视频流的发布者的用户 ID
  ///
  FutureOr<String?> get uid async {
    return await sendInstanceGet<String?>("uid");
  }

  set uid(FutureOr<String?> value) {
    sendInstanceSet("uid", value);
  }

  /// @brief 是否是屏幕共享流
  ///
  FutureOr<boolean?> get isScreen async {
    return await sendInstanceGet<boolean?>("isScreen");
  }

  set isScreen(FutureOr<boolean?> value) {
    sendInstanceSet("isScreen", value);
  }

  /// @brief 流切换前订阅视频流的分辨率对应的索引
  ///
  FutureOr<int?> get beforeVideoIndex async {
    return await sendInstanceGet<int?>("beforeVideoIndex");
  }

  set beforeVideoIndex(FutureOr<int?> value) {
    sendInstanceSet("beforeVideoIndex", value);
  }

  /// @brief 流切换后订阅视频流的分辨率对应的索引
  ///
  FutureOr<int?> get afterVideoIndex async {
    return await sendInstanceGet<int?>("afterVideoIndex");
  }

  set afterVideoIndex(FutureOr<int?> value) {
    sendInstanceSet("afterVideoIndex", value);
  }

  /// @brief 流切换前是否有视频流
  ///
  FutureOr<boolean?> get beforeEnable async {
    return await sendInstanceGet<boolean?>("beforeEnable");
  }

  set beforeEnable(FutureOr<boolean?> value) {
    sendInstanceSet("beforeEnable", value);
  }

  /// @brief 流切换后是否有视频流
  ///
  FutureOr<boolean?> get afterEnable async {
    return await sendInstanceGet<boolean?>("afterEnable");
  }

  set afterEnable(FutureOr<boolean?> value) {
    sendInstanceSet("afterEnable", value);
  }

  /// @brief 流切换原因，详见类型 FallbackOrRecoverReason{@link #FallbackOrRecoverReason} 。
  ///
  FutureOr<FallbackOrRecoverReason?> get reason async {
    try {
      final result = await sendInstanceGet<FallbackOrRecoverReason?>("reason");
      if (result == null) {
        return null;
      }
      return FallbackOrRecoverReason.values
          .firstWhere((element) => element == result);
    } catch (e) {
      return null;
    }
  }

  set reason(FutureOr<FallbackOrRecoverReason?> value) {
    sendInstanceSet("reason", value);
  }
}

enum PerformanceAlarmReason {
  /// @brief 网络原因差，造成了发送性能回退。仅在开启发送性能回退时，会收到此原因。
  ///
  BANDWIDTH_FALLBACKED(0),

  /// @brief 网络性能恢复，发送性能回退恢复。仅在开启发送性能回退时，会收到此原因。
  ///
  BANDWIDTH_RESUMED(1),

  /// @brief 如果未开启发送性能回退，收到此告警时，意味着性能不足； <br>
  ///        如果开启了发送性能回退，收到此告警时，意味着性能不足，且已发生发送性能回退。
  ///
  PERFORMANCE_FALLBACKED(2),

  /// @brief 如果未开启发送性能回退，收到此告警时，意味着性能不足已恢复； <br>
  ///        如果开启了发送性能回退，收到此告警时，意味着性能不足已恢复，且已发生发送性能回退恢复。
  ///
  PERFORMANCE_RESUMED(3);

  final dynamic $value;
  const PerformanceAlarmReason([this.$value]);
}

enum EarMonitorMode {
  /// @brief 关闭。
  ///
  EAR_MONITOR_MODE_OFF(0),

  /// @brief 开启
  ///
  EAR_MONITOR_MODE_ON(1);

  final dynamic $value;
  const EarMonitorMode([this.$value]);
}

enum VirtualBackgroundSourceType {
  /// @brief 使用纯色背景替换视频原有背景。
  ///
  COLOR(0),

  /// @brief 使用自定义图片背景替换视频原有背景。
  ///
  IMAGE(1);

  final dynamic $value;
  const VirtualBackgroundSourceType([this.$value]);
}

class VideoDimensions extends NativeClass {
  static const _$namespace = r'com.ss.bytertc.engine.video.VideoDimensions';
  static get codegen_$namespace => _$namespace;

  VideoDimensions([NativeClassOptions? options])
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

  /// @brief 宽
  ///
  FutureOr<int?> get width async {
    return await sendInstanceGet<int?>("width");
  }

  set width(FutureOr<int?> value) {
    sendInstanceSet("width", value);
  }

  /// @brief 高
  ///
  FutureOr<int?> get height async {
    return await sendInstanceGet<int?>("height");
  }

  set height(FutureOr<int?> value) {
    sendInstanceSet("height", value);
  }
}

enum AudioPlaybackDevice {
  /// @brief 有线耳机
  ///
  AUDIO_PLAYBACK_DEVICE_HEADSET(1),

  /// @brief 听筒
  ///
  AUDIO_PLAYBACK_DEVICE_EARPIECE(2),

  /// @brief 扬声器
  ///
  AUDIO_PLAYBACK_DEVICE_SPEAKERPHONE(3),

  /// @brief 蓝牙耳机
  ///
  AUDIO_PLAYBACK_DEVICE_HEADSET_BLUETOOTH(4),

  /// @brief USB 设备
  ///
  AUDIO_PLAYBACK_DEVICE_HEADSET_USB(5);

  final dynamic $value;
  const AudioPlaybackDevice([this.$value]);
}

enum VideoOrientation {
  /// @brief （默认）使用相机输出的原始视频帧的角度，不对视频帧进行额外旋转。
  ///
  ADAPTIVE(0),

  /// @brief 固定为竖屏，将相机采集到的视频帧转换为竖屏，在整个 RTC 链路中传递竖屏帧。
  ///
  PORTRAIT(1),

  /// @brief 固定为横屏，将相机采集到的视频帧转换为横屏，在整个 RTC 链路中传递横屏帧。
  ///
  LANDSCAPE(2);

  final dynamic $value;
  const VideoOrientation([this.$value]);
}

class VideoCaptureConfig extends NativeClass {
  static const _$namespace = r'com.ss.bytertc.engine.video.VideoCaptureConfig';
  static get codegen_$namespace => _$namespace;

  VideoCaptureConfig([NativeClassOptions? options])
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

  /// @brief 视频采集模式，参看 CapturePreference{@link #VideoCaptureConfig#CapturePreference}。
  ///
  FutureOr<CapturePreference?> get capturePreference async {
    try {
      final result =
          await sendInstanceGet<CapturePreference?>("capturePreference");
      if (result == null) {
        return null;
      }
      return CapturePreference.values
          .firstWhere((element) => element == result);
    } catch (e) {
      return null;
    }
  }

  set capturePreference(FutureOr<CapturePreference?> value) {
    sendInstanceSet("capturePreference", value);
  }

  /// @brief 视频采集分辨率的宽度，单位：px。
  ///
  FutureOr<int?> get width async {
    return await sendInstanceGet<int?>("width");
  }

  set width(FutureOr<int?> value) {
    sendInstanceSet("width", value);
  }

  /// @brief 视频采集分辨率的高度，单位：px。
  ///
  FutureOr<int?> get height async {
    return await sendInstanceGet<int?>("height");
  }

  set height(FutureOr<int?> value) {
    sendInstanceSet("height", value);
  }

  /// @brief 视频采集帧率，单位：fps。
  ///
  FutureOr<int?> get frameRate async {
    return await sendInstanceGet<int?>("frameRate");
  }

  set frameRate(FutureOr<int?> value) {
    sendInstanceSet("frameRate", value);
  }
}

enum VoiceChangerType {
  /// @brief 原声，不含特效
  ///
  VOICE_CHANGER_ORIGINAL(0),

  /// @brief 巨人
  ///
  VOICE_CHANGER_GIANT(1),

  /// @brief 花栗鼠
  ///
  VOICE_CHANGER_CHIPMUNK(2),

  /// @brief 小黄人
  ///
  VOICE_CHANGER_MINIONST(3),

  /// @brief 颤音
  ///
  VOICE_CHANGER_VIBRATO(4),

  /// @brief 机器人
  ///
  VOICE_CHANGER_ROBOT(5);

  final dynamic $value;
  const VoiceChangerType([this.$value]);
}

class ProblemFeedbackInfo extends NativeClass {
  static const _$namespace = r'com.ss.bytertc.engine.type.ProblemFeedbackInfo';
  static get codegen_$namespace => _$namespace;

  ProblemFeedbackInfo([NativeClassOptions? options])
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

  /// @brief 文字描述
  ///
  FutureOr<String?> get problemDesc async {
    return await sendInstanceGet<String?>("problemDesc");
  }

  set problemDesc(FutureOr<String?> value) {
    sendInstanceSet("problemDesc", value);
  }

  /// @brief 房间信息，参看 ProblemFeedbackRoomInfo{@link #ProblemFeedbackRoomInfo}。
  ///
  FutureOr<List<ProblemFeedbackRoomInfo>?> get roomInfo async {
    try {
      final result =
          await sendInstanceGet<List<ProblemFeedbackRoomInfo>?>("roomInfo");
      if (result == null) {
        return null;
      }
      final list = result.map((e) => packObject(
          e,
          () => ProblemFeedbackRoomInfo(
              const NativeClassOptions([], disableInit: true))));
      return list.toList();
    } catch (e) {
      return null;
    }
  }

  set roomInfo(FutureOr<List<ProblemFeedbackRoomInfo>?> value) {
    sendInstanceSet("roomInfo", value);
  }

  /// @hidden
  ///

  FutureOr<String> getProblemDesc() async {
    return await nativeCall('getProblemDesc', []);
  }

  /// @hidden
  ///

  FutureOr<Array<ProblemFeedbackRoomInfo>> getRoomInfo() async {
    return await nativeCall('getRoomInfo', []);
  }
}

enum WTNSubscribeState {
  /// @brief 订阅 WTN 媒体流
  ///
  SUBSCRIBED(0),

  /// @brief 取消订阅 WTN 媒体流
  ///
  UNSUBSCRIBED(1);

  final dynamic $value;
  const WTNSubscribeState([this.$value]);
}

enum MediaPlayerCustomSourceSeekWhence {
  /// @brief 从音频数据的头开始读取，读取的实际偏移量为参数 offset 的值。
  ///
  SET(0),

  /// @brief 从音频数据的某一位置开始读取，读取的实际偏移量为音频数据当前的读取位置位置加上参数 offset 的值。
  ///
  CUR(1),

  /// @brief 从音频数据的尾开始读取，读取的实际数据偏移量为用户传入的音频数据大小加上参数 offset 的值。
  ///
  END(2),

  /// @brief 返回音频数据的大小。
  ///
  SIZE(3);

  final dynamic $value;
  const MediaPlayerCustomSourceSeekWhence([this.$value]);
}

enum AudioMixingType {
  /// @brief 仅本地播放
  ///
  AUDIO_MIXING_TYPE_PLAYOUT(0),

  /// @brief 仅远端播放
  ///
  AUDIO_MIXING_TYPE_PUBLISH(1),

  /// @brief 本地和远端同时播放
  ///
  AUDIO_MIXING_TYPE_PLAYOUT_AND_PUBLISH(2);

  final dynamic $value;
  const AudioMixingType([this.$value]);
}

enum AudioDumpStatus {
  /// @brief 音频 dump 启动失败
  ///
  AUDIO_DUMP_START_FAILURE(0),

  /// @brief 音频 dump 启动成功
  ///
  AUDIO_DUMP_START_SUCCESS(1),

  /// @brief 音频 dump 停止失败
  ///
  AUDIO_DUMP_STOP_FAILURE(2),

  /// @brief 音频 dump 停止成功
  ///
  AUDIO_DUMP_STOP_SUCCESS(3),

  /// @brief 音频 dump 运行失败
  ///
  AUDIO_DUMP_RUNNING_FAILURE(4),

  /// @brief 音频 dump 运行成功
  ///
  AUDIO_DUMP_RUNNING_SUCCESS(5);

  final dynamic $value;
  const AudioDumpStatus([this.$value]);
}

enum FallbackOrRecoverReason {
  /// @brief 其他原因，非带宽和性能原因引起的回退或恢复。默认值
  ///
  FALLBACK_OR_RECOVER_REASON_UNKNOWN(-1),

  /// @brief 由带宽不足导致的订阅端音视频流回退。
  ///
  FALLBACK_OR_RECOVER_REASON_SUBSCRIBE_FALLBACK_BY_BANDWIDTH(0),

  /// @brief 由性能不足导致的订阅端音视频流回退。
  ///
  FALLBACK_OR_RECOVER_REASON_SUBSCRIBE_FALLBACK_BY_PERFORMANCE(1),

  /// @brief 由带宽恢复导致的订阅端音视频流恢复。
  ///
  FALLBACK_OR_RECOVER_REASON_SUBSCRIBE_RECOVER_BY_BANDWIDTH(2),

  /// @brief 由性能恢复导致的订阅端音视频流恢复。
  ///
  FALLBACK_OR_RECOVER_REASON_SUBSCRIBE_RECOVER_BY_PERFORMANCE(3),

  /// @brief 由带宽不足导致的发布端音视频流回退。
  ///
  FALLBACK_OR_RECOVER_REASON_PUBLISH_FALLBACK_BY_BANDWIDTH(4),

  /// @brief 由性能不足导致的发布端音视频流回退。
  ///
  FALLBACK_OR_RECOVER_REASON_PUBLISH_FALLBACK_BY_PERFORMANCE(5),

  /// @brief 由带宽恢复导致的发布端音视频流恢复。
  ///
  FALLBACK_OR_RECOVER_REASON_PUBLISH_RECOVER_BY_BANDWIDTH(6),

  /// @brief 由性能恢复导致的发布端音视频流恢复。
  ///
  FALLBACK_OR_RECOVER_REASON_PUBLISH_RECOVER_BY_PERFORMANCE(7),

  /// @hidden constructor/destructor
  ///
  value(-1);

  final dynamic $value;
  const FallbackOrRecoverReason([this.$value]);
}

class VideoEncoderConfiguration extends NativeClass {
  static const _$namespace =
      r'com.ss.bytertc.engine.video.VideoEncoderConfiguration';
  static get codegen_$namespace => _$namespace;

  VideoEncoderConfiguration([NativeClassOptions? options])
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

  /// @brief 视频编码像素
  ///
  FutureOr<VideoDimensions?> get dimensions async {
    try {
      final result = await sendInstanceGet<VideoDimensions?>("dimensions");
      if (result == null) {
        return null;
      }
      return packObject(
          result,
          () =>
              VideoDimensions(const NativeClassOptions([], disableInit: true)));
    } catch (e) {
      return null;
    }
  }

  set dimensions(FutureOr<VideoDimensions?> value) {
    sendInstanceSet("dimensions", value);
  }

  /// @brief 视频编码帧率
  ///
  FutureOr<FrameRate?> get frameRate async {
    try {
      final result = await sendInstanceGet<FrameRate?>("frameRate");
      if (result == null) {
        return null;
      }
      return FrameRate.values.firstWhere((element) => element == result);
    } catch (e) {
      return null;
    }
  }

  set frameRate(FutureOr<FrameRate?> value) {
    sendInstanceSet("frameRate", value);
  }

  /// @brief 视频编码码率
  ///
  FutureOr<int?> get kBitrate async {
    return await sendInstanceGet<int?>("kBitrate");
  }

  set kBitrate(FutureOr<int?> value) {
    sendInstanceSet("kBitrate", value);
  }

  FutureOr<int?> get kMinBitrate async {
    return await sendInstanceGet<int?>("kMinBitrate");
  }

  set kMinBitrate(FutureOr<int?> value) {
    sendInstanceSet("kMinBitrate", value);
  }

  /// @brief 视频编码方向模式
  ///
  FutureOr<OrientationMode?> get orientationMode async {
    try {
      final result = await sendInstanceGet<OrientationMode?>("orientationMode");
      if (result == null) {
        return null;
      }
      return OrientationMode.values.firstWhere((element) => element == result);
    } catch (e) {
      return null;
    }
  }

  set orientationMode(FutureOr<OrientationMode?> value) {
    sendInstanceSet("orientationMode", value);
  }
}

enum AudioFrameCallbackMethod {
  /// @brief 本地麦克风录制的音频数据回调
  ///
  AUDIO_FRAME_CALLBACK_RECORD(0),

  /// @brief 订阅的远端所有用户混音后的音频数据回调
  ///
  AUDIO_FRAME_CALLBACK_PLAYBACK(1),

  /// @brief 本地麦克风录制和订阅的远端所有用户混音后的音频数据回调
  ///
  AUDIO_FRAME_CALLBACK_MIXED(2),

  /// @brief 订阅的远端每个用户混音前的音频数据回调
  ///
  AUDIO_FRAME_CALLBACK_REMOTE_USER(3),

  /// @brief 本地麦克风录制和本地 `MediaPlayer`, `EffectPlayer` 播放的音频混音后的音频数据回调
  ///
  AUDIO_FRAME_CALLBACK_CAPTURE_MIXED(5);

  final dynamic $value;
  const AudioFrameCallbackMethod([this.$value]);
}

class RTCRoomConfig extends NativeClass {
  static const _$namespace = r'com.ss.bytertc.engine.RTCRoomConfig';
  static get codegen_$namespace => _$namespace;

  RTCRoomConfig([NativeClassOptions? options])
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

  /// @brief 房间模式，参看 ChannelProfile{@link #ChannelProfile}，默认为 `CHANNEL_PROFILE_COMMUNICATION`，进房后不可更改。
  ///
  FutureOr<ChannelProfile?> get profile async {
    try {
      final result = await sendInstanceGet<ChannelProfile?>("profile");
      if (result == null) {
        return null;
      }
      return ChannelProfile.values.firstWhere((element) => element == result);
    } catch (e) {
      return null;
    }
  }

  set profile(FutureOr<ChannelProfile?> value) {
    sendInstanceSet("profile", value);
  }

  /// @brief 流 ID，进房后不可更改。
  ///
  FutureOr<String?> get streamId async {
    return await sendInstanceGet<String?>("streamId");
  }

  set streamId(FutureOr<String?> value) {
    sendInstanceSet("streamId", value);
  }

  /// @brief 是否自动发布音频流，默认为自动发布。
  ///        + 若调用 setUserVisibility{@link #RTCRoom#setUserVisibility} 将自身可见性设为 false，无论是默认的自动发布流还是手动设置的自动发布流都不会进行发布，你需要将自身可见性设为 true 后方可发布。
  ///        + 多房间场景下，若已在其中一个房间成功设置了自动发布，其他房间的自动发布设置均不会生效。若每个房间均不做设置，则默认在第一个加入的房间内自动发布流。
  ///
  FutureOr<boolean?> get isPublishAudio async {
    return await sendInstanceGet<boolean?>("isPublishAudio");
  }

  set isPublishAudio(FutureOr<boolean?> value) {
    sendInstanceSet("isPublishAudio", value);
  }

  /// @brief 是否自动发布视频流，默认为自动发布。
  ///        + 若调用 setUserVisibility{@link #RTCRoom#setUserVisibility} 将自身可见性设为 false，无论是默认的自动发布流还是手动设置的自动发布流都不会进行发布，你需要将自身可见性设为 true 后方可发布。
  ///        + 多房间场景下，若已在其中一个房间成功设置了自动发布，其他房间的自动发布设置均不会生效。若每个房间均不做设置，则默认在第一个加入的房间内自动发布流。
  ///
  FutureOr<boolean?> get isPublishVideo async {
    return await sendInstanceGet<boolean?>("isPublishVideo");
  }

  set isPublishVideo(FutureOr<boolean?> value) {
    sendInstanceSet("isPublishVideo", value);
  }

  /// @brief 是否自动订阅音频流，默认为自动订阅。 <br>
  ///        包含主流和屏幕流。
  ///
  FutureOr<boolean?> get isAutoSubscribeAudio async {
    return await sendInstanceGet<boolean?>("isAutoSubscribeAudio");
  }

  set isAutoSubscribeAudio(FutureOr<boolean?> value) {
    sendInstanceSet("isAutoSubscribeAudio", value);
  }

  /// @brief 是否自动订阅视频流，默认为自动订阅。 <br>
  ///        包含主流和屏幕流。
  ///
  FutureOr<boolean?> get isAutoSubscribeVideo async {
    return await sendInstanceGet<boolean?>("isAutoSubscribeVideo");
  }

  set isAutoSubscribeVideo(FutureOr<boolean?> value) {
    sendInstanceSet("isAutoSubscribeVideo", value);
  }

  /// @hidden constructor/destructor
  ///

  FutureOr<int> getProfile() async {
    return await nativeCall('getProfile', []);
  }

  /// @hidden constructor/destructor
  ///

  FutureOr<String> getStreamId() async {
    return await nativeCall('getStreamId', []);
  }
}

enum RemoteAudioState {
  /// @brief 远端音频流默认初始状态，在以下时机回调该状态： <br>
  ///       - 本地用户停止接收远端音频流，对应原因是 RemoteAudioStateChangeReason{@link #RemoteAudioStateChangeReason} 中的 `kRemoteAudioReasonLocalMuted`
  ///       - 远端用户停止发送音频流，对应原因是 RemoteAudioStateChangeReason{@link #RemoteAudioStateChangeReason} 中的 `kRemoteAudioReasonRemoteMuted`
  ///       - 远端用户离开房间，对应原因是 RemoteAudioStateChangeReason{@link #RemoteAudioStateChangeReason} 中的 `kRemoteAudioReasonRemoteOffline`
  ///
  REMOTE_AUDIO_STATE_STOPPED(0),

  /// @brief 开始接收远端音频流首包。
  ///
  REMOTE_AUDIO_STATE_STARTING(1),

  /// @brief 远端音频流正在解码，正常播放，在以下时机回调该状态： <br>
  ///       - 成功解码远端音频首帧，对应原因是 RemoteAudioStateChangeReason{@link #RemoteAudioStateChangeReason} 中的 `kRemoteAudioReasonLocalUnmuted`
  ///       - 网络由阻塞恢复正常，对应原因是 RemoteAudioStateChangeReason{@link #RemoteAudioStateChangeReason} 中的 `kRemoteAudioReasonNetworkRecovery`
  ///       - 本地用户恢复接收远端音频流，对应原因是 RemoteAudioStateChangeReason{@link #RemoteAudioStateChangeReason} 中的 `kRemoteAudioReasonLocalUnmuted`
  ///       - 远端用户恢复发送音频流，对应原因是 RemoteAudioStateChangeReason{@link #RemoteAudioStateChangeReason} 中的 `kRemoteAudioReasonRemoteUnmuted`
  ///
  REMOTE_AUDIO_STATE_DECODING(2),

  /// @brief 远端音频流卡顿。 <br>
  ///        网络阻塞导致丢包率大于 40\% 时回调该状态，对应原因是 <br>
  ///        RemoteAudioStateChangeReason{@link #RemoteAudioStateChangeReason} 中的 `kRemoteAudioReasonNetworkCongestion`
  ///
  REMOTE_AUDIO_STATE_FROZEN(3),

  /// @hidden currently not available
  /// @brief 远端音频流播放失败
  /// @note 该错误码暂未使用
  ///
  REMOTE_AUDIO_STATE_FAILED(4);

  final dynamic $value;
  const RemoteAudioState([this.$value]);
}

enum SetRoomExtraInfoResult {
  /// @brief 设置房间附加信息成功
  ///
  SUCCESS(0),

  /// @brief 设置失败，尚未加入房间
  ///
  NOT_JOIN_ROOM(-1),

  /// @brief 设置失败，key 指针为空
  ///
  KEY_IS_NULL(-2),

  /// @brief 设置失败，value 指针为空
  ///
  VALUE_IS_NULL(-3),

  /// @brief 设置失败，未知错误
  ///
  UNKNOW(-99),

  /// @brief 设置失败，key 长度为 0
  ///
  KEY_IS_EMPTY(-400),

  /// @brief 调用 `setRoomExtraInfo` 过于频繁，建议不超过 10 次/秒。
  ///
  TOO_OFTEN(-406),

  /// @brief 设置失败，用户已调用 `setUserVisibility` 将自身设为隐身状态。
  ///
  SILENT_USER(-412),

  /// @brief 设置失败，Key 长度超过 10 字节
  ///
  KEY_TOO_LONG(-413),

  /// @brief 设置失败，value 长度超过 128 字节
  ///
  VALUE_TOO_LONG(-414),

  /// @brief 设置失败，服务器错误
  ///
  SERVER_ERROR(-500);

  final dynamic $value;
  const SetRoomExtraInfoResult([this.$value]);
}

enum VideoSinkMirrorType {
  /// @brief 开启镜像。
  ///
  ON(1),

  /// @brief （默认值）不开启镜像。
  ///
  OFF(2);

  final dynamic $value;
  const VideoSinkMirrorType([this.$value]);
}

enum SubscribeStateChangeReason {
  /// @brief 本端调用订阅
  ///
  SUBSCRIBE(0),

  /// @brief 本端取消订阅
  ///
  UNSUBSCRIBE(1),

  /// @brief 远端发布流
  ///
  REMOTE_PUBLISH(2),

  /// @brief 远端取消发布流
  ///
  REMOTE_UNPUBLISH(3),

  /// @brief 由于服务器错误导致订阅失败。SDK 会自动重试订阅
  ///
  STREAM_FAILED_5XX(4),

  /// @brief 当前房间中找不到订阅的音视频流导致订阅失败。SDK 会自动重试订阅，若仍订阅失败则建议你退出重试。
  ///
  STREAM_FAILED_404(5),

  /// @brief 当用户订阅的音视频流总数已达上限时，继续订阅更多流时会失败，同时用户会收到此错误通知。
  ///
  OVER_STREAM_SUBSCRIBE_LIMIT(6),

  /// @brief 用户订阅所在房间中的音视频流失败，失败原因为用户没有订阅流的权限。
  ///
  NO_SUBSCRIBE_PERMISSION(7);

  final dynamic $value;
  const SubscribeStateChangeReason([this.$value]);
}

enum VideoApplyRotation {
  /// @brief （默认值）不旋转。
  ///
  DEFAULT(-1),

  /// @brief 自动转正视频，即根据视频帧的旋转角信息将视频帧旋转到 0 度。
  ///
  DEGREE_0(0);

  final dynamic $value;
  const VideoApplyRotation([this.$value]);
}

enum MediaStreamType {
  /// @brief 只控制音频
  ///
  RTC_MEDIA_STREAM_TYPE_AUDIO(1),

  /// @brief 只控制视频
  ///
  RTC_MEDIA_STREAM_TYPE_VIDEO(2),

  /// @brief 同时控制音频和视频
  ///
  RTC_MEDIA_STREAM_TYPE_BOTH(3),

  /// @hidden constructor/destructor
  ///
  value(4);

  final dynamic $value;
  const MediaStreamType([this.$value]);
}

class MixedStreamConfig extends NativeClass {
  static const _$namespace = r'com.ss.bytertc.engine.live.MixedStreamConfig';
  static get codegen_$namespace => _$namespace;

  static FutureOr<MixedStreamConfig> defaultMixedStreamConfig() async {
    final result = await NativeClassUtils.nativeStaticCall(
      _$namespace,
      'defaultMixedStreamConfig',
      [],
      'com.volcengine.rtc.hybrid_runtime',
    );
    return packObject(
        result,
        () =>
            MixedStreamConfig(const NativeClassOptions([], disableInit: true)));
  }

  MixedStreamConfig([NativeClassOptions? options])
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
  /// @brief 设置视频转码配置参数。建议设置。
  /// @param videoConfig 视频转码配置参数。详见 MixedStreamVideoConfig{@link #MixedStreamConfig-MixedStreamVideoConfig} 数据类型。
  ///
  FutureOr<MixedStreamVideoConfig?> get videoConfig async {
    try {
      final result =
          await sendInstanceGet<MixedStreamVideoConfig?>("videoConfig");
      if (result == null) {
        return null;
      }
      return packObject(
          result,
          () => MixedStreamVideoConfig(
              const NativeClassOptions([], disableInit: true)));
    } catch (e) {
      return null;
    }
  }

  set videoConfig(FutureOr<MixedStreamVideoConfig?> value) {
    sendInstanceSet("videoConfig", value);
  }

  /// @detail api
  /// @brief 设置音频转码配置参数。建议设置。
  /// @param audioConfig 音频转码配置参数，参看 MixedStreamAudioConfig{@link #MixedStreamConfig-MixedStreamAudioConfig}。
  ///
  FutureOr<MixedStreamAudioConfig?> get audioConfig async {
    try {
      final result =
          await sendInstanceGet<MixedStreamAudioConfig?>("audioConfig");
      if (result == null) {
        return null;
      }
      return packObject(
          result,
          () => MixedStreamAudioConfig(
              const NativeClassOptions([], disableInit: true)));
    } catch (e) {
      return null;
    }
  }

  set audioConfig(FutureOr<MixedStreamAudioConfig?> value) {
    sendInstanceSet("audioConfig", value);
  }

  /// @valid since 3.56
  /// @detail api
  /// @brief 设置服务端合流控制参数。
  /// @param controlConfig 服务端合流控制参数，参看 MixedStreamControlConfig{@link #MixedStreamConfig-MixedStreamControlConfig}。
  ///
  FutureOr<MixedStreamControlConfig?> get controlConfig async {
    try {
      final result =
          await sendInstanceGet<MixedStreamControlConfig?>("controlConfig");
      if (result == null) {
        return null;
      }
      return packObject(
          result,
          () => MixedStreamControlConfig(
              const NativeClassOptions([], disableInit: true)));
    } catch (e) {
      return null;
    }
  }

  set controlConfig(FutureOr<MixedStreamControlConfig?> value) {
    sendInstanceSet("controlConfig", value);
  }

  /// @detail api
  /// @brief 设置推流 CDN 的空间音频参数。
  /// @param spatialConfig 空间音频参数。详见 MixedStreamSpatialAudioConfig{@link #MixedStreamConfig-MixedStreamSpatialAudioConfig} 数据类型。
  ///
  FutureOr<MixedStreamSpatialAudioConfig?> get spatialAudioConfig async {
    try {
      final result = await sendInstanceGet<MixedStreamSpatialAudioConfig?>(
          "spatialAudioConfig");
      if (result == null) {
        return null;
      }
      return packObject(
          result,
          () => MixedStreamSpatialAudioConfig(
              const NativeClassOptions([], disableInit: true)));
    } catch (e) {
      return null;
    }
  }

  set spatialAudioConfig(FutureOr<MixedStreamSpatialAudioConfig?> value) {
    sendInstanceSet("spatialAudioConfig", value);
  }

  /// @detail api
  /// @brief 设置用户布局信息列表。建议设置。
  /// @param regions 用户布局信息列表。为 MixedStreamLayoutRegionConfig{@link #MixedStreamLayoutRegionConfig} 数据类型的数组。每一个该类型对象为一路单独的视频流的布局信息。 <br>
  ///                值不合法或未设置时，自动使用默认值。
  ///
  FutureOr<Array<MixedStreamLayoutRegionConfig>?> get regions async {
    try {
      final result =
          await sendInstanceGet<Array<MixedStreamLayoutRegionConfig>?>(
              "regions");
      if (result == null) {
        return null;
      }
      final list = result.map((e) => packObject(
          e,
          () => MixedStreamLayoutRegionConfig(
              const NativeClassOptions([], disableInit: true))));
      return list.toList();
    } catch (e) {
      return null;
    }
  }

  set regions(FutureOr<Array<MixedStreamLayoutRegionConfig>?> value) {
    sendInstanceSet("regions", value);
  }

  /// @detail api
  /// @brief 设置用户配置的额外信息。
  /// @param userConfigExtraInfo 用户配置的额外信息。
  /// @note
  ///      WTN 流任务不支持设置本参数。
  ///
  FutureOr<String?> get userConfigExtraInfo async {
    return await sendInstanceGet<String?>("userConfigExtraInfo");
  }

  set userConfigExtraInfo(FutureOr<String?> value) {
    sendInstanceSet("userConfigExtraInfo", value);
  }

  /// @detail api
  /// @brief 设置画布的背景颜色。值不合法或未设置时，自动使用默认值。建议设置。
  /// @param backgroundColor 合流背景颜色，用十六进制颜色码（HEX）表示。例如，#FFFFFF 表示纯白，#000000 表示纯黑。默认值为 #000000。
  ///
  FutureOr<String?> get backgroundColor async {
    return await sendInstanceGet<String?>("backgroundColor");
  }

  set backgroundColor(FutureOr<String?> value) {
    sendInstanceSet("backgroundColor", value);
  }

  /// @valid since 3.57
  /// @detail api
  /// @brief 设置合流后整体画布的背景图片 URL。
  /// @param backgroundImageURL 背景图片 URL，长度最大为 1023 bytes。必须是http开头，支持的图片格式包括：JPG, JPEG, PNG。如果背景图片的宽高和整体屏幕的宽高不一致，背景图片会缩放到铺满屏幕。
  ///
  FutureOr<String?> get backgroundImageURL async {
    return await sendInstanceGet<String?>("backgroundImageURL");
  }

  set backgroundImageURL(FutureOr<String?> value) {
    sendInstanceSet("backgroundImageURL", value);
  }

  /// @hidden for internal use only
  ///
  FutureOr<JSONObject?> get advancedConfig async {
    return await sendInstanceGet<JSONObject?>("advancedConfig");
  }

  set advancedConfig(FutureOr<JSONObject?> value) {
    sendInstanceSet("advancedConfig", value);
  }

  /// @detail api
  /// @brief 设置合流房间 ID
  /// @param roomID 发起合流的用户所在的房间 ID
  /// @note
  ///        本参数不支持过程中更新。
  ///
  FutureOr<String?> get roomID async {
    return await sendInstanceGet<String?>("roomID");
  }

  set roomID(FutureOr<String?> value) {
    sendInstanceSet("roomID", value);
  }

  /// @detail api
  /// @brief 设置发起合流任务的用户 ID。`roomID` 和 `userID` 长度相加不得超过 126 字节。建议设置。 <br>
  ///        本参数不支持过程中更新。
  /// @param userID 推流用户 ID。
  ///
  FutureOr<String?> get userID async {
    return await sendInstanceGet<String?>("userID");
  }

  set userID(FutureOr<String?> value) {
    sendInstanceSet("userID", value);
  }

  /// @hidden for internal use only
  ///
  FutureOr<JSONObject?> get authInfo async {
    return await sendInstanceGet<JSONObject?>("authInfo");
  }

  set authInfo(FutureOr<JSONObject?> value) {
    sendInstanceSet("authInfo", value);
  }

  /// @detail api
  /// @brief 设置 WTN 流的补帧模式。参看 InterpolationMode{@link #InterpolationMode}。
  ///
  FutureOr<InterpolationMode?> get interpolationMode async {
    try {
      final result =
          await sendInstanceGet<InterpolationMode?>("interpolationMode");
      if (result == null) {
        return null;
      }
      return InterpolationMode.values
          .firstWhere((element) => element == result);
    } catch (e) {
      return null;
    }
  }

  set interpolationMode(FutureOr<InterpolationMode?> value) {
    sendInstanceSet("interpolationMode", value);
  }

  /// @detail api
  /// @brief 设置 WTN 流的布局模式。参看 StreamLayoutMode{@link #StreamLayoutMode}。
  ///
  FutureOr<StreamLayoutMode?> get layoutMode async {
    try {
      final result = await sendInstanceGet<StreamLayoutMode?>("layoutMode");
      if (result == null) {
        return null;
      }
      return StreamLayoutMode.values.firstWhere((element) => element == result);
    } catch (e) {
      return null;
    }
  }

  set layoutMode(FutureOr<StreamLayoutMode?> value) {
    sendInstanceSet("layoutMode", value);
  }

  /// @hidden constructor/destructor
  ///

  FutureOr<int> getMixedStreamLayoutmode() async {
    return await nativeCall('getMixedStreamLayoutmode', []);
  }

  /// @hidden constructor/destructor
  ///

  FutureOr<String> getMixedStreamRoomID() async {
    return await nativeCall('getMixedStreamRoomID', []);
  }

  /// @hidden constructor/destructor
  ///

  FutureOr<String> getMixedStreamUserID() async {
    return await nativeCall('getMixedStreamUserID', []);
  }

  /// @hidden constructor/destructor
  ///

  FutureOr<String> getMixedStreamAdvancedConfig() async {
    return await nativeCall('getMixedStreamAdvancedConfig', []);
  }

  /// @hidden constructor/destructor
  ///

  FutureOr<String> getMixedStreamAuthInfo() async {
    return await nativeCall('getMixedStreamAuthInfo', []);
  }

  /// @hidden constructor/destructor
  ///

  FutureOr<String> getMixedStreamVideoConfigCodec() async {
    return await nativeCall('getMixedStreamVideoConfigCodec', []);
  }

  /// @hidden constructor/destructor
  ///

  FutureOr<int> getMixedStreamVideoConfigBitrate() async {
    return await nativeCall('getMixedStreamVideoConfigBitrate', []);
  }

  /// @hidden constructor/destructor
  ///

  FutureOr<int> getMixedStreamVideoConfigFps() async {
    return await nativeCall('getMixedStreamVideoConfigFps', []);
  }

  /// @hidden constructor/destructor
  ///

  FutureOr<int> getMixedStreamVideoConfigGop() async {
    return await nativeCall('getMixedStreamVideoConfigGop', []);
  }

  /// @hidden constructor/destructor
  ///

  FutureOr<int> getMixedStreamVideoConfigHeight() async {
    return await nativeCall('getMixedStreamVideoConfigHeight', []);
  }

  /// @hidden constructor/destructor
  ///

  FutureOr<int> getMixedStreamVideoConfigWidth() async {
    return await nativeCall('getMixedStreamVideoConfigWidth', []);
  }

  FutureOr<boolean> getMixedStreamVideoConfigBFrame() async {
    return await nativeCall('getMixedStreamVideoConfigBFrame', []);
  }

  /// @hidden constructor/destructor
  ///

  FutureOr<String> getMixedStreamAudioConfigAudioProfile() async {
    return await nativeCall('getMixedStreamAudioConfigAudioProfile', []);
  }

  /// @hidden constructor/destructor
  ///

  FutureOr<String> getMixedStreamAudioConfigCodec() async {
    return await nativeCall('getMixedStreamAudioConfigCodec', []);
  }

  /// @hidden constructor/destructor
  ///

  FutureOr<int> getMixedStreamaudioConfigBitrate() async {
    return await nativeCall('getMixedStreamaudioConfigBitrate', []);
  }

  /// @hidden constructor/destructor
  ///

  FutureOr<int> getMixedStreamAudioConfigSampleRate() async {
    return await nativeCall('getMixedStreamAudioConfigSampleRate', []);
  }

  /// @hidden constructor/destructor
  ///

  FutureOr<int> getMixedStreamAudioConfigChannels() async {
    return await nativeCall('getMixedStreamAudioConfigChannels', []);
  }

  /// @hidden constructor/destructor
  ///

  FutureOr<boolean>
      getMixedStreamServerControlConfigEnableVolumeIndication() async {
    return await nativeCall(
        'getMixedStreamServerControlConfigEnableVolumeIndication', []);
  }

  /// @hidden constructor/destructor
  ///

  FutureOr<float>
      getMixedStreamServerControlConfigVolumeIndicationInterval() async {
    return await nativeCall(
        'getMixedStreamServerControlConfigVolumeIndicationInterval', []);
  }

  /// @hidden constructor/destructor
  ///

  FutureOr<int> getMixedStreamServerControlConfigTalkVolume() async {
    return await nativeCall('getMixedStreamServerControlConfigTalkVolume', []);
  }

  /// @hidden constructor/destructor
  ///

  FutureOr<boolean> getMixedStreamServerControlConfigIsAddVolumeValue() async {
    return await nativeCall(
        'getMixedStreamServerControlConfigIsAddVolumeValue', []);
  }

  /// @hidden constructor/destructor
  ///

  FutureOr<int> getMixedStreamServerControlConfigSeiContentMode() async {
    return await nativeCall(
        'getMixedStreamServerControlConfigSeiContentMode', []);
  }

  /// @hidden constructor/destructor
  ///

  FutureOr<int> getMixedStreamServerControlConfigSeiPayloadType() async {
    return await nativeCall(
        'getMixedStreamServerControlConfigSeiPayloadType', []);
  }

  /// @hidden constructor/destructor
  ///

  FutureOr<String> getMixedStreamServerControlConfigSeiPayloadUuid() async {
    return await nativeCall(
        'getMixedStreamServerControlConfigSeiPayloadUuid', []);
  }

  /// @hidden constructor/destructor
  ///

  FutureOr<int> getMixedStreamServerControlConfigMediaType() async {
    return await nativeCall('getMixedStreamServerControlConfigMediaType', []);
  }

  /// @hidden constructor/destructor
  ///

  FutureOr<int> getMixedStreamServerControlConfigPushStreamMode() async {
    return await nativeCall(
        'getMixedStreamServerControlConfigPushStreamMode', []);
  }

  /// @hidden constructor/destructor
  ///

  FutureOr<int> getMixedStreamSyncControlConfigSyncStrategy() async {
    return await nativeCall('getMixedStreamSyncControlConfigSyncStrategy', []);
  }

  /// @hidden constructor/destructor
  ///

  FutureOr<int> getMixedStreamSyncControlConfigQueueLength() async {
    return await nativeCall('getMixedStreamSyncControlConfigQueueLength', []);
  }

  /// @hidden constructor/destructor
  ///

  FutureOr<boolean> getMixedStreamSyncControlConfigVideoNeedMix() async {
    return await nativeCall('getMixedStreamSyncControlConfigVideoNeedMix', []);
  }

  /// @hidden constructor/destructor
  ///

  FutureOr<String> getMixedStreamSyncControlConfigBaseUser() async {
    return await nativeCall('getMixedStreamSyncControlConfigBaseUser', []);
  }

  /// @hidden constructor/destructor
  ///

  FutureOr<boolean> getMixedStreamSpatialConfigEnableSpatialRender() async {
    return await nativeCall(
        'getMixedStreamSpatialConfigEnableSpatialRender', []);
  }

  /// @hidden constructor/destructor
  ///

  FutureOr<float> getMixedStreamSpatialConfigAudienceSpatialPositionX() async {
    return await nativeCall(
        'getMixedStreamSpatialConfigAudienceSpatialPositionX', []);
  }

  /// @hidden constructor/destructor
  ///

  FutureOr<float> getMixedStreamSpatialConfigAudienceSpatialPositionY() async {
    return await nativeCall(
        'getMixedStreamSpatialConfigAudienceSpatialPositionY', []);
  }

  /// @hidden constructor/destructor
  ///

  FutureOr<float> getMixedStreamSpatialConfigAudienceSpatialPositionZ() async {
    return await nativeCall(
        'getMixedStreamSpatialConfigAudienceSpatialPositionZ', []);
  }

  /// @hidden constructor/destructor
  ///

  FutureOr<float>
      getMixedStreamSpatialConfigAudienceSpatialOrientationForwardX() async {
    return await nativeCall(
        'getMixedStreamSpatialConfigAudienceSpatialOrientationForwardX', []);
  }

  /// @hidden constructor/destructor
  ///

  FutureOr<float>
      getMixedStreamSpatialConfigAudienceSpatialOrientationForwardY() async {
    return await nativeCall(
        'getMixedStreamSpatialConfigAudienceSpatialOrientationForwardY', []);
  }

  /// @hidden constructor/destructor
  ///

  FutureOr<float>
      getMixedStreamSpatialConfigAudienceSpatialOrientationForwardZ() async {
    return await nativeCall(
        'getMixedStreamSpatialConfigAudienceSpatialOrientationForwardZ', []);
  }

  /// @hidden constructor/destructor
  ///

  FutureOr<float>
      getMixedStreamSpatialConfigAudienceSpatialOrientationRightX() async {
    return await nativeCall(
        'getMixedStreamSpatialConfigAudienceSpatialOrientationRightX', []);
  }

  /// @hidden constructor/destructor
  ///

  FutureOr<float>
      getMixedStreamSpatialConfigAudienceSpatialOrientationRightY() async {
    return await nativeCall(
        'getMixedStreamSpatialConfigAudienceSpatialOrientationRightY', []);
  }

  /// @hidden constructor/destructor
  ///

  FutureOr<float>
      getMixedStreamSpatialConfigAudienceSpatialOrientationRightZ() async {
    return await nativeCall(
        'getMixedStreamSpatialConfigAudienceSpatialOrientationRightZ', []);
  }

  /// @hidden constructor/destructor
  ///

  FutureOr<float>
      getMixedStreamSpatialConfigAudienceSpatialOrientationUpX() async {
    return await nativeCall(
        'getMixedStreamSpatialConfigAudienceSpatialOrientationUpX', []);
  }

  /// @hidden constructor/destructor
  ///

  FutureOr<float>
      getMixedStreamSpatialConfigAudienceSpatialOrientationUpY() async {
    return await nativeCall(
        'getMixedStreamSpatialConfigAudienceSpatialOrientationUpY', []);
  }

  /// @hidden constructor/destructor
  ///

  FutureOr<float>
      getMixedStreamSpatialConfigAudienceSpatialOrientationUpZ() async {
    return await nativeCall(
        'getMixedStreamSpatialConfigAudienceSpatialOrientationUpZ', []);
  }

  /// @hidden constructor/destructor
  ///

  FutureOr<Array<MixedStreamLayoutRegionConfig>>
      getMixedStreamLayoutRegionConfigs() async {
    return await nativeCall('getMixedStreamLayoutRegionConfigs', []);
  }

  /// @hidden constructor/destructor
  ///

  FutureOr<String> getMixedStreamUserConfigExtraInfo() async {
    return await nativeCall('getMixedStreamUserConfigExtraInfo', []);
  }

  /// @hidden constructor/destructor
  ///

  FutureOr<String> getMixedStreamBackgroundColor() async {
    return await nativeCall('getMixedStreamBackgroundColor', []);
  }

  /// @hidden constructor/destructor
  ///

  FutureOr<String> getMixedStreamBackgroundImageURL() async {
    return await nativeCall('getMixedStreamBackgroundImageURL', []);
  }

  /// @hidden constructor/destructor
  ///

  FutureOr<double> getMixedStreamLayoutSourceCropX(
      MixedStreamLayoutRegionConfig region) async {
    return await nativeCall('getMixedStreamLayoutSourceCropX', [region]);
  }

  /// @hidden constructor/destructor
  ///

  FutureOr<double> getMixedStreamLayoutSourceCropY(
      MixedStreamLayoutRegionConfig region) async {
    return await nativeCall('getMixedStreamLayoutSourceCropY', [region]);
  }

  /// @hidden constructor/destructor
  ///

  FutureOr<double> getMixedStreamLayoutSourceCropW(
      MixedStreamLayoutRegionConfig region) async {
    return await nativeCall('getMixedStreamLayoutSourceCropW', [region]);
  }

  /// @hidden constructor/destructor
  ///

  FutureOr<double> getMixedStreamLayoutSourceCropH(
      MixedStreamLayoutRegionConfig region) async {
    return await nativeCall('getMixedStreamLayoutSourceCropH', [region]);
  }

  /// @hidden constructor/destructor
  ///

  FutureOr<String> getMixedStreamLayoutRegionUserID(
      MixedStreamLayoutRegionConfig region) async {
    return await nativeCall('getMixedStreamLayoutRegionUserID', [region]);
  }

  /// @hidden constructor/destructor
  ///

  FutureOr<String> getMixedStreamLayoutRegionRoomID(
      MixedStreamLayoutRegionConfig region) async {
    return await nativeCall('getMixedStreamLayoutRegionRoomID', [region]);
  }

  /// @hidden constructor/destructor
  ///

  FutureOr<int> getMixedStreamLayoutRegionX(
      MixedStreamLayoutRegionConfig region) async {
    return await nativeCall('getMixedStreamLayoutRegionX', [region]);
  }

  /// @hidden constructor/destructor
  ///

  FutureOr<int> getMixedStreamLayoutRegionY(
      MixedStreamLayoutRegionConfig region) async {
    return await nativeCall('getMixedStreamLayoutRegionY', [region]);
  }

  /// @hidden constructor/destructor
  ///

  FutureOr<int> getMixedStreamLayoutRegionW(
      MixedStreamLayoutRegionConfig region) async {
    return await nativeCall('getMixedStreamLayoutRegionW', [region]);
  }

  /// @hidden constructor/destructor
  ///

  FutureOr<int> getMixedStreamLayoutRegionH(
      MixedStreamLayoutRegionConfig region) async {
    return await nativeCall('getMixedStreamLayoutRegionH', [region]);
  }

  /// @hidden constructor/destructor
  ///

  FutureOr<int> getMixedStreamLayoutRegionZOrder(
      MixedStreamLayoutRegionConfig region) async {
    return await nativeCall('getMixedStreamLayoutRegionZOrder', [region]);
  }

  /// @hidden constructor/destructor
  ///

  FutureOr<double> getMixedStreamLayoutRegionAlpha(
      MixedStreamLayoutRegionConfig region) async {
    return await nativeCall('getMixedStreamLayoutRegionAlpha', [region]);
  }

  /// @hidden constructor/destructor
  ///

  FutureOr<double> getMixedStreamLayoutRegionCornerRadius(
      MixedStreamLayoutRegionConfig region) async {
    return await nativeCall('getMixedStreamLayoutRegionCornerRadius', [region]);
  }

  /// @hidden constructor/destructor
  ///

  FutureOr<int> getMixedStreamLayoutRegionMediaType(
      MixedStreamLayoutRegionConfig region) async {
    return await nativeCall('getMixedStreamLayoutRegionMediaType', [region]);
  }

  /// @hidden constructor/destructor
  ///

  FutureOr<int> getMixedStreamLayoutRegionRenderMode(
      MixedStreamLayoutRegionConfig region) async {
    return await nativeCall('getMixedStreamLayoutRegionRenderMode', [region]);
  }

  /// @hidden constructor/destructor
  ///

  FutureOr<boolean> getMixedStreamLayoutRegionLocalUser(
      MixedStreamLayoutRegionConfig region) async {
    return await nativeCall('getMixedStreamLayoutRegionLocalUser', [region]);
  }

  /// @hidden constructor/destructor
  ///

  FutureOr<int> getMixedStreamLayoutRegionScreenStream(
      MixedStreamLayoutRegionConfig region) async {
    return await nativeCall('getMixedStreamLayoutRegionScreenStream', [region]);
  }

  /// @hidden constructor/destructor
  ///

  FutureOr<int> getMixedStreamLayoutRegionContentType(
      MixedStreamLayoutRegionConfig region) async {
    return await nativeCall('getMixedStreamLayoutRegionContentType', [region]);
  }

  /// @hidden constructor/destructor
  ///

  FutureOr<ArrayBuffer> getMixedStreamLayoutRegionData(
      MixedStreamLayoutRegionConfig region) async {
    return await nativeCall('getMixedStreamLayoutRegionData', [region]);
  }

  /// @hidden constructor/destructor
  ///

  FutureOr<int> getMixedStreamLayoutRegionDataParamImageWidth(
      MixedStreamLayoutRegionConfig region) async {
    return await nativeCall(
        'getMixedStreamLayoutRegionDataParamImageWidth', [region]);
  }

  /// @hidden constructor/destructor
  ///

  FutureOr<int> getMixedStreamLayoutRegionDataParamImageHeight(
      MixedStreamLayoutRegionConfig region) async {
    return await nativeCall(
        'getMixedStreamLayoutRegionDataParamImageHeight', [region]);
  }

  /// @hidden constructor/destructor
  ///

  FutureOr<int> getMixedStreamLayoutRegionAlternateImageFillMode(
      MixedStreamLayoutRegionConfig region) async {
    return await nativeCall(
        'getMixedStreamLayoutRegionAlternateImageFillMode', [region]);
  }

  /// @hidden constructor/destructor
  ///

  FutureOr<String> getMixedStreamLayoutRegionAlternateImageURL(
      MixedStreamLayoutRegionConfig region) async {
    return await nativeCall(
        'getMixedStreamLayoutRegionAlternateImageURL', [region]);
  }

  /// @hidden constructor/destructor
  ///

  FutureOr<float> getMixedStreamLayoutRegionSpatialPositionX(
      MixedStreamLayoutRegionConfig region) async {
    return await nativeCall(
        'getMixedStreamLayoutRegionSpatialPositionX', [region]);
  }

  /// @hidden constructor/destructor
  ///

  FutureOr<float> getMixedStreamLayoutRegionSpatialPositionY(
      MixedStreamLayoutRegionConfig region) async {
    return await nativeCall(
        'getMixedStreamLayoutRegionSpatialPositionY', [region]);
  }

  /// @hidden constructor/destructor
  ///

  FutureOr<float> getMixedStreamLayoutRegionSpatialPositionZ(
      MixedStreamLayoutRegionConfig region) async {
    return await nativeCall(
        'getMixedStreamLayoutRegionSpatialPositionZ', [region]);
  }

  /// @hidden constructor/destructor
  ///

  FutureOr<boolean> getMixedStreamLayoutRegionApplySpatialAudio(
      MixedStreamLayoutRegionConfig region) async {
    return await nativeCall(
        'getMixedStreamLayoutRegionApplySpatialAudio', [region]);
  }

  /// @hidden constructor/destructor
  ///

  FutureOr<int> getMixedStreamInterpolationMode() async {
    return await nativeCall('getMixedStreamInterpolationMode', []);
  }

  /// @hidden constructor/destructor
  ///

  FutureOr<int> getMixedStreamLayoutMode() async {
    return await nativeCall('getMixedStreamLayoutMode', []);
  }

  /// @hidden constructor/destructor
  ///

  FutureOr<JSONObject> getTranscodeMessage() async {
    return await nativeCall('getTranscodeMessage', []);
  }
}

enum AudioProfileType {
  /// @brief 默认音质 <br>
  ///        服务器下发或客户端已设置的 ChannelProfile{@link #ChannelProfile} 的音质配置
  ///
  AUDIO_PROFILE_DEFAULT(0),

  /// @brief 流畅。 <br>
  ///        单声道，采样率为 16 kHz，编码码率为 32 Kbps。 <br>
  ///        流畅优先、低功耗、低流量消耗，适用于大部分游戏场景，如小队语音、组队语音、国战语音等。
  ///
  AUDIO_PROFILE_FLUENT(1),

  /// @brief 单声道标准音质。 <br>
  ///        采样率为 24 kHz，编码码率为 48 Kbps。 <br>
  ///        适用于对音质有一定要求的场景，同时延时、功耗和流量消耗相对适中，适合教育场景和狼人杀等游戏。
  ///
  AUDIO_PROFILE_STANDARD(2),

  /// @brief 双声道音乐音质 <br>
  ///        采样率为 48 kHz，编码码率为 128 kbps。 <br>
  ///        超高音质，同时延时、功耗和流量消耗相对较大，适用于连麦 PK 等音乐场景。 <br>
  ///        游戏场景不建议使用。
  ///
  AUDIO_PROFILE_HD(3),

  /// @brief 双声道标准音质。采样率为 48 KHz，编码码率最大值为 80 Kbps
  ///
  AUDIO_PROFILE_STANDARD_STEREO(4),

  /// @brief 单声道音乐音质。采样率为 48 kHz，编码码率最大值为 64 Kbps
  ///
  AUDIO_PROFILE_HD_MONO(5);

  final dynamic $value;
  const AudioProfileType([this.$value]);
}

class MixedStreamVideoConfig extends NativeClass {
  static const _$namespace =
      r'com.ss.bytertc.engine.live.MixedStreamVideoConfig';
  static get codegen_$namespace => _$namespace;

  MixedStreamVideoConfig([NativeClassOptions? options])
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

  /// @hidden constructor/destructor
  ///
  FutureOr<MixedStreamVideoCodecType?> get videoCodec async {
    try {
      final result =
          await sendInstanceGet<MixedStreamVideoCodecType?>("videoCodec");
      if (result == null) {
        return null;
      }
      return MixedStreamVideoCodecType.values
          .firstWhere((element) => element == result);
    } catch (e) {
      return null;
    }
  }

  set videoCodec(FutureOr<MixedStreamVideoCodecType?> value) {
    sendInstanceSet("videoCodec", value);
  }

  /// @hidden constructor/destructor
  ///
  FutureOr<int?> get fps async {
    return await sendInstanceGet<int?>("fps");
  }

  set fps(FutureOr<int?> value) {
    sendInstanceSet("fps", value);
  }

  /// @hidden constructor/destructor
  ///
  FutureOr<int?> get gop async {
    return await sendInstanceGet<int?>("gop");
  }

  set gop(FutureOr<int?> value) {
    sendInstanceSet("gop", value);
  }

  /// @hidden constructor/destructor
  ///
  FutureOr<int?> get bitrate async {
    return await sendInstanceGet<int?>("bitrate");
  }

  set bitrate(FutureOr<int?> value) {
    sendInstanceSet("bitrate", value);
  }

  /// @hidden constructor/destructor
  ///
  FutureOr<int?> get width async {
    return await sendInstanceGet<int?>("width");
  }

  set width(FutureOr<int?> value) {
    sendInstanceSet("width", value);
  }

  /// @hidden constructor/destructor
  ///
  FutureOr<int?> get height async {
    return await sendInstanceGet<int?>("height");
  }

  set height(FutureOr<int?> value) {
    sendInstanceSet("height", value);
  }

  /// @hidden constructor/destructor
  ///
  FutureOr<boolean?> get enableBframe async {
    return await sendInstanceGet<boolean?>("enableBframe");
  }

  set enableBframe(FutureOr<boolean?> value) {
    sendInstanceSet("enableBframe", value);
  }
}

class ScreenSharingParameters extends NativeClass {
  static const _$namespace =
      r'com.ss.bytertc.engine.video.ScreenSharingParameters';
  static get codegen_$namespace => _$namespace;

  ScreenSharingParameters([NativeClassOptions? options])
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

  /// @brief 屏幕采集编码最大宽度,
  ///
  FutureOr<int?> get maxWidth async {
    return await sendInstanceGet<int?>("maxWidth");
  }

  set maxWidth(FutureOr<int?> value) {
    sendInstanceSet("maxWidth", value);
  }

  /// @brief 屏幕采集编码最大高度
  ///
  FutureOr<int?> get maxHeight async {
    return await sendInstanceGet<int?>("maxHeight");
  }

  set maxHeight(FutureOr<int?> value) {
    sendInstanceSet("maxHeight", value);
  }

  /// @brief 屏幕采集编码帧率, 单位 fps
  ///
  FutureOr<int?> get frameRate async {
    return await sendInstanceGet<int?>("frameRate");
  }

  set frameRate(FutureOr<int?> value) {
    sendInstanceSet("frameRate", value);
  }

  /// @brief 屏幕采集编码码率, `-1` 为自动码率, SDK 会根据宽高信息选择合适的码率，单位 kbps
  ///
  FutureOr<int?> get bitrate async {
    return await sendInstanceGet<int?>("bitrate");
  }

  set bitrate(FutureOr<int?> value) {
    sendInstanceSet("bitrate", value);
  }

  /// @brief 视频最小编码码率, 单位 kbps。编码码率不会低于 `minBitrate`。 <br>
  ///        默认值为 `0`。 <br>
  ///        范围：[0, bitrate)，当 `bitrate` < `minBitrate` 时，为适配码率模式。 <br>
  ///        以下情况，设置本参数无效： <br>
  ///        - 当 `bitrate` 为 `0` 时，不对视频流进行编码发送。
  ///        - 当 `bitrate` < `0` 时，适配码率模式。
  ///
  FutureOr<int?> get minBitrate async {
    return await sendInstanceGet<int?>("minBitrate");
  }

  set minBitrate(FutureOr<int?> value) {
    sendInstanceSet("minBitrate", value);
  }
}

enum AudioDeviceType {
  /// @brief 未知设备
  ///
  AUDIO_DEVICE_TYPE_UNKNOWN(-1),

  /// @brief 音频渲染设备
  ///
  AUDIO_DEVICE_TYPE_RENDER_DEVICE(0),

  /// @brief 音频采集设备类型
  ///
  AUDIO_DEVICE_TYPE_CAPTURE_DEVICE(1),

  /// @brief 屏幕流音频设备
  ///
  AUDIO_DEVICE_TYPE_SCREEN_CAPTURE_DEVICE(2);

  final dynamic $value;
  const AudioDeviceType([this.$value]);
}

enum VideoRotationMode {
  /// @brief App 方向
  ///
  FOLLOW_APP(0),

  /// @brief 重力方向
  ///
  FOLLOW_GSENSOR(1);

  final dynamic $value;
  const VideoRotationMode([this.$value]);
}

enum VideoRotation {
  /// @brief 顺时针旋转 0 度
  ///
  VIDEO_ROTATION_0(0),

  /// @brief 顺时针旋转 90 度
  ///
  VIDEO_ROTATION_90(90),

  /// @brief 顺时针旋转 180 度
  ///
  VIDEO_ROTATION_180(180),

  /// @brief 顺时针旋转 270 度
  ///
  VIDEO_ROTATION_270(270);

  final dynamic $value;
  const VideoRotation([this.$value]);
}

class StreamInfo extends NativeClass {
  static const _$namespace = r'com.ss.bytertc.engine.data.StreamInfo';
  static get codegen_$namespace => _$namespace;

  StreamInfo([NativeClassOptions? options])
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

  /// @brief 流 ID。
  ///
  FutureOr<String?> get streamId async {
    return await sendInstanceGet<String?>("streamId");
  }

  set streamId(FutureOr<String?> value) {
    sendInstanceSet("streamId", value);
  }

  /// @brief 房间 ID。
  ///
  FutureOr<String?> get roomId async {
    return await sendInstanceGet<String?>("roomId");
  }

  set roomId(FutureOr<String?> value) {
    sendInstanceSet("roomId", value);
  }

  /// @brief 用户 ID。
  ///
  FutureOr<String?> get userId async {
    return await sendInstanceGet<String?>("userId");
  }

  set userId(FutureOr<String?> value) {
    sendInstanceSet("userId", value);
  }

  /// @brief 流属性，包括主流、屏幕流。参看 [StreamIndex](70083#StreamIndex-2)
  ///
  FutureOr<int?> get streamIndex async {
    return await sendInstanceGet<int?>("streamIndex");
  }

  set streamIndex(FutureOr<int?> value) {
    sendInstanceSet("streamIndex", value);
  }

  /// @brief 是否是屏幕流。
  ///
  FutureOr<boolean?> get isScreen async {
    return await sendInstanceGet<boolean?>("isScreen");
  }

  set isScreen(FutureOr<boolean?> value) {
    sendInstanceSet("isScreen", value);
  }
}

enum MixedStreamPushTargetType {
  /// @brief 推到 CDN
  ///
  PUSH_TO_CDN(0),

  /// @brief WTN 流
  ///
  PUSH_TO_WTN(1);

  final dynamic $value;
  const MixedStreamPushTargetType([this.$value]);
}

enum ReturnStatus {
  /// @brief 成功。
  ///
  RETURN_STATUS_SUCCESS(0),

  /// @brief 失败。
  ///
  RETURN_STATUS_FAILURE(-1),

  /// @brief 参数错误。
  ///
  RETURN_STATUS_PARAMETER_ERR(-2),

  /// @brief 接口状态错误。
  ///
  RETURN_STATUS_WRONG_STATE(-3),

  /// @brief 失败，用户已在房间内。
  ///
  RETURN_STATUS_HAS_IN_ROOM(-4),

  /// @brief 失败，用户已登录。
  ///
  RETURN_STATUS_HAS_IN_LOGIN(-5),

  /// @brief 失败，用户已经在进行通话回路测试中。
  ///
  RETURN_STATUS_HAS_IN_ECHO_TEST(-6),

  /// @brief 失败，音视频均未采集。
  ///
  RETURN_STATUS_NEITHER_VIDEO_NOR_AUDIO(-7),

  /// @brief 失败，该 roomId 已被使用。
  ///
  RETURN_STATUS_ROOMID_IN_USE(-8),

  /// @brief 失败，屏幕流不支持。
  ///
  RETURN_STATUS_SCREEN_NOT_SUPPORT(-9),

  /// @brief 失败，不支持该操作。
  ///
  RETURN_STATUS_NOT_SUPPORT(-10),

  /// @brief 失败，资源已占用。
  ///
  RETURN_STATUS_RESOURCE_OVERFLOW(-11),

  /// @brief 失败，不支持视频接口调用。
  ///
  RETURN_STATUS_VIDEO_NOT_SUPPORT(-12),

  /// @brief 失败，没有音频帧。
  ///
  RETURN_STATUS_AUDIO_NO_FRAME(-101),

  /// @brief 失败，未实现。
  ///
  RETURN_STATUS_AUDIO_NOT_IMPLEMENTED(-102),

  /// @brief 失败，采集设备无麦克风权限，尝试初始化设备失败。
  ///
  RETURN_STATUS_AUDIO_NO_PERMISSION(-103),

  /// @brief 失败，设备不存在。当前没有设备或设备被移除时返回该值。
  ///
  RETURN_STATUS_AUDIO_DEVICE_NOT_EXISTS(-104),

  /// @brief 失败，设备音频格式不支持。
  ///
  RETURN_STATUS_AUDIO_DEVICE_FORMAT_NOT_SUPPORT(-105),

  /// @brief 失败，系统无可用设备。
  ///
  RETURN_STATUS_AUDIO_DEVICE_NO_DEVICE(-106),

  /// @brief 失败，当前设备不可用，需更换设备。
  ///
  RETURN_STATUS_AUDIO_DEVICE_CAN_NOT_USE(-107),

  /// @brief 系统错误，设备初始化失败。
  ///
  RETURN_STATUS_AUDIO_DEVICE_INIT_FAILED(-108),

  /// @brief 系统错误，设备开启失败。
  ///
  RETURN_STATUS_AUDIO_DEVICE_START_FAILED(-109),

  /// @brief 共享的进程不存在，共享错误。
  ///
  RETURN_STATUS_AUDIO_DEVICE_PROCESS_NOT_EXIST(-110),

  /// @brief 失败。底层未初始化，engine 无效。
  ///
  RETURN_STATUS_NATIVE_IN_VALID(-201),

  /// @brief 警告。推送视频帧到 RTC SDK 时，相邻视频帧的时间戳差异应当和推帧操作的间隔相同。如果不同，会收到此警告。
  ///
  RETURN_STATUS_VIDEO_TIMESTAMP_WARNING(-202);

  final dynamic $value;
  const ReturnStatus([this.$value]);
}

enum AudioFrameType {
  /// @brief PCM 16bit
  ///
  FRAME_TYPE_PCM16(0);

  final dynamic $value;
  const AudioFrameType([this.$value]);
}

enum ForwardStreamState {
  /// @brief 空闲状态 <br>
  ///        - 成功调用 `stopForwardStreamToRooms` 后，所有目标房间为空闲状态。
  ///        - 成功调用 `updateForwardStreamToRooms` 减少目标房间后，本次减少的目标房间为空闲状态。
  ///
  FORWARD_STREAM_STATE_IDLE(0),

  /// @brief 开始转发 <br>
  ///        - 调用 `startForwardStreamToRooms` 成功向所有房间开始转发媒体流后，返回此状态。
  ///        - 调用 `updateForwardStreamToRooms` 后，成功向新增目标房间开始转发媒体流后，返回此状态。
  ///
  FORWARD_STREAM_STATE_SUCCESS(1),

  /// @brief 转发失败，失败详情参考 ForwardStreamError{@link #forwardStreamError} <br>
  ///        调用 `startForwardStreamToRooms` 或 `updateForwardStreamToRooms` 后，如遇转发失败，返回此状态。
  ///
  FORWARD_STREAM_STATE_FAILURE(2);

  final dynamic $value;
  const ForwardStreamState([this.$value]);
}

class ReceiveRange extends NativeClass {
  static const _$namespace = r'com.ss.bytertc.engine.data.ReceiveRange';
  static get codegen_$namespace => _$namespace;

  ReceiveRange([NativeClassOptions? options])
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
  ///
  FutureOr<int?> get min async {
    return await sendInstanceGet<int?>("min");
  }

  set min(FutureOr<int?> value) {
    sendInstanceSet("min", value);
  }

  /// @brief 能够收听语音的最大距离值，该值须 > 0 且 ≥ min。 <br>
  ///        当收听者和声源距离处于 [min, max) 之间时，收听到的音量根据距离呈衰减效果。 <br>
  ///        超出该值范围的音频将无法收听到。
  ///
  FutureOr<int?> get max async {
    return await sendInstanceGet<int?>("max");
  }

  set max(FutureOr<int?> value) {
    sendInstanceSet("max", value);
  }
}

enum MixedStreamMediaType {
  /// @brief 包含音频和视频
  ///
  MIXED_STREAM_MEDIA_TYPE_AUDIO_AND_VIDEO(0),

  /// @brief 只包含音频
  ///
  MIXED_STREAM_MEDIA_TYPE_AUDIO_ONLY(1),

  /// @hidden currently not available
  /// @brief 只包含视频
  ///
  MIXED_STREAM_MEDIA_TYPE_VIDEO_ONLY(2);

  final dynamic $value;
  const MixedStreamMediaType([this.$value]);
}

enum OrientationMode {
  /// @brief 视频输出方向与采集方向一致。
  ///
  ORIENTATION_MODE_ADAPTIVE(0),

  /// @brief 固定横屏输出视频。若采集到的视频是竖屏模式，则视频编码器会对其进行裁剪。
  ///
  ORIENTATION_MODE_FIXED_LANDSCAPE(1),

  /// @brief 固定竖屏输出视频。若采集到的视频是横屏模式，则视频编码器会对其进行裁剪。
  ///
  ORIENTATION_MODE_FIXED_PORTRAIT(2);

  final dynamic $value;
  const OrientationMode([this.$value]);
}

class StandardPitchInfo extends NativeClass {
  static const _$namespace = r'com.ss.bytertc.engine.data.StandardPitchInfo';
  static get codegen_$namespace => _$namespace;

  StandardPitchInfo([NativeClassOptions? options])
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
  FutureOr<int?> get startTime async {
    return await sendInstanceGet<int?>("startTime");
  }

  set startTime(FutureOr<int?> value) {
    sendInstanceSet("startTime", value);
  }

  /// @brief 持续时间，单位 ms。
  FutureOr<int?> get duration async {
    return await sendInstanceGet<int?>("duration");
  }

  set duration(FutureOr<int?> value) {
    sendInstanceSet("duration", value);
  }

  /// @brief 音高。
  FutureOr<int?> get pitch async {
    return await sendInstanceGet<int?>("pitch");
  }

  set pitch(FutureOr<int?> value) {
    sendInstanceSet("pitch", value);
  }
}

class RemoteAudioPropertiesInfo extends NativeClass {
  static const _$namespace =
      r'com.ss.bytertc.engine.data.RemoteAudioPropertiesInfo';
  static get codegen_$namespace => _$namespace;

  RemoteAudioPropertiesInfo([NativeClassOptions? options])
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
  /// @brief 远端流 ID。
  ///
  FutureOr<String?> get streamId async {
    return await sendInstanceGet<String?>("streamId");
  }

  set streamId(FutureOr<String?> value) {
    sendInstanceSet("streamId", value);
  }

  /// @detail keytype
  /// @brief 远端流信息，详见 StreamInfo{@link #StreamInfo}
  ///
  FutureOr<StreamInfo?> get streamInfo async {
    try {
      final result = await sendInstanceGet<StreamInfo?>("streamInfo");
      if (result == null) {
        return null;
      }
      return packObject(result,
          () => StreamInfo(const NativeClassOptions([], disableInit: true)));
    } catch (e) {
      return null;
    }
  }

  set streamInfo(FutureOr<StreamInfo?> value) {
    sendInstanceSet("streamInfo", value);
  }

  /// @detail keytype
  /// @brief 音频属性信息，详见 AudioPropertiesInfo{@link #AudioPropertiesInfo}
  ///
  FutureOr<AudioPropertiesInfo?> get audioPropertiesInfo async {
    try {
      final result =
          await sendInstanceGet<AudioPropertiesInfo?>("audioPropertiesInfo");
      if (result == null) {
        return null;
      }
      return packObject(
          result,
          () => AudioPropertiesInfo(
              const NativeClassOptions([], disableInit: true)));
    } catch (e) {
      return null;
    }
  }

  set audioPropertiesInfo(FutureOr<AudioPropertiesInfo?> value) {
    sendInstanceSet("audioPropertiesInfo", value);
  }
}

enum MixedStreamClientMixVideoFormat {
  /// @brief YUV I420。Android、Windows 默认回调格式。支持系统：Android、Windows。
  ///
  MIXED_STREAM_CLIENT_MIX_VIDEO_FORMAT_YUV_I420(0),

  /// @brief OpenGL GL_TEXTURE_2D 格式纹理。支持系统：安卓。
  ///
  MIXED_STREAM_CLIENT_MIX_VIDEO_FORMAT_TEXTURE_2D(1),

  /// @brief CVPixelBuffer BGRA。iOS 默认回调格式。支持系统: iOS。
  ///
  MIXED_STREAM_CLIENT_MIX_VIDEO_FORMAT_CVPIXEL_BUFFER_BGRA(2),

  /// @brief YUV NV12。macOS 默认回调格式。支持系统: macOS。
  ///
  MIXED_STREAM_CLIENT_MIX_VIDEO_FORMAT_YUV_NV12(3);

  final dynamic $value;
  const MixedStreamClientMixVideoFormat([this.$value]);
}

class RemoteAudioStats extends NativeClass {
  static const _$namespace = r'com.ss.bytertc.engine.type.RemoteAudioStats';
  static get codegen_$namespace => _$namespace;

  RemoteAudioStats([NativeClassOptions? options])
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
  ///
  FutureOr<float?> get audioLossRate async {
    return await sendInstanceGet<float?>("audioLossRate");
  }

  set audioLossRate(FutureOr<float?> value) {
    sendInstanceSet("audioLossRate", value);
  }

  /// @brief 接收码率。统计周期内的音频接收码率，单位为 kbps 。
  ///
  FutureOr<float?> get receivedKBitrate async {
    return await sendInstanceGet<float?>("receivedKBitrate");
  }

  set receivedKBitrate(FutureOr<float?> value) {
    sendInstanceSet("receivedKBitrate", value);
  }

  /// @brief 音频卡顿次数。统计周期内的卡顿次数。
  ///
  FutureOr<int?> get stallCount async {
    return await sendInstanceGet<int?>("stallCount");
  }

  set stallCount(FutureOr<int?> value) {
    sendInstanceSet("stallCount", value);
  }

  /// @brief 音频卡顿时长。统计周期内的卡顿时长，单位为 ms 。
  ///
  FutureOr<int?> get stallDuration async {
    return await sendInstanceGet<int?>("stallDuration");
  }

  set stallDuration(FutureOr<int?> value) {
    sendInstanceSet("stallDuration", value);
  }

  /// @brief 用户体验级别的端到端延时。从发送端采集完成编码开始到接收端解码完成渲染开始的延时，单位为 ms 。
  ///
  FutureOr<long?> get e2eDelay async {
    return await sendInstanceGet<long?>("e2eDelay");
  }

  set e2eDelay(FutureOr<long?> value) {
    sendInstanceSet("e2eDelay", value);
  }

  /// @brief 播放采样率。统计周期内的音频播放采样率信息，单位为 Hz 。
  ///
  FutureOr<int?> get playoutSampleRate async {
    return await sendInstanceGet<int?>("playoutSampleRate");
  }

  set playoutSampleRate(FutureOr<int?> value) {
    sendInstanceSet("playoutSampleRate", value);
  }

  /// @brief 统计间隔。此次统计周期的间隔，单位为 ms 。
  ///
  FutureOr<int?> get statsInterval async {
    return await sendInstanceGet<int?>("statsInterval");
  }

  set statsInterval(FutureOr<int?> value) {
    sendInstanceSet("statsInterval", value);
  }

  /// @brief 客户端到服务端数据传输的往返时延，单位为 ms 。
  ///
  FutureOr<int?> get rtt async {
    return await sendInstanceGet<int?>("rtt");
  }

  set rtt(FutureOr<int?> value) {
    sendInstanceSet("rtt", value);
  }

  /// @brief 发送端——服务端——接收端全链路数据传输往返时延。单位为 ms 。
  ///
  FutureOr<int?> get totalRtt async {
    return await sendInstanceGet<int?>("totalRtt");
  }

  set totalRtt(FutureOr<int?> value) {
    sendInstanceSet("totalRtt", value);
  }

  /// @brief 远端用户发送的音频流质量。值含义参考 NetworkQuality{@link #NetworkQuality} 。
  ///
  FutureOr<int?> get quality async {
    return await sendInstanceGet<int?>("quality");
  }

  set quality(FutureOr<int?> value) {
    sendInstanceSet("quality", value);
  }

  /// @brief 因引入 jitter buffer 机制导致的延时。单位为 ms 。
  ///
  FutureOr<int?> get jitterBufferDelay async {
    return await sendInstanceGet<int?>("jitterBufferDelay");
  }

  set jitterBufferDelay(FutureOr<int?> value) {
    sendInstanceSet("jitterBufferDelay", value);
  }

  /// @brief 音频声道数。
  ///
  FutureOr<int?> get numChannels async {
    return await sendInstanceGet<int?>("numChannels");
  }

  set numChannels(FutureOr<int?> value) {
    sendInstanceSet("numChannels", value);
  }

  /// @brief 音频接收采样率。统计周期内接收到的远端音频采样率信息，单位为 Hz 。
  ///
  FutureOr<int?> get receivedSampleRate async {
    return await sendInstanceGet<int?>("receivedSampleRate");
  }

  set receivedSampleRate(FutureOr<int?> value) {
    sendInstanceSet("receivedSampleRate", value);
  }

  /// @brief 远端用户在加入房间后发生音频卡顿的累计时长占音频总有效时长的百分比。音频有效时长是指远端用户进房发布音频流后，除停止发送音频流和禁用音频模块之外的音频时长。
  ///
  FutureOr<int?> get frozenRate async {
    return await sendInstanceGet<int?>("frozenRate");
  }

  set frozenRate(FutureOr<int?> value) {
    sendInstanceSet("frozenRate", value);
  }

  /// @brief 音频丢包补偿(PLC) 样点总个数。
  ///
  FutureOr<int?> get concealedSamples async {
    return await sendInstanceGet<int?>("concealedSamples");
  }

  set concealedSamples(FutureOr<int?> value) {
    sendInstanceSet("concealedSamples", value);
  }

  /// @brief 音频丢包补偿(PLC) 累计次数。
  ///
  FutureOr<int?> get concealmentEvent async {
    return await sendInstanceGet<int?>("concealmentEvent");
  }

  set concealmentEvent(FutureOr<int?> value) {
    sendInstanceSet("concealmentEvent", value);
  }

  /// @brief 音频解码采样率。统计周期内的音频解码采样率信息，单位为 Hz 。
  ///
  FutureOr<int?> get decSampleRate async {
    return await sendInstanceGet<int?>("decSampleRate");
  }

  set decSampleRate(FutureOr<int?> value) {
    sendInstanceSet("decSampleRate", value);
  }

  /// @brief 此次订阅中，对远端音频流进行解码的累计耗时。单位为 s。
  ///
  FutureOr<int?> get decDuration async {
    return await sendInstanceGet<int?>("decDuration");
  }

  set decDuration(FutureOr<int?> value) {
    sendInstanceSet("decDuration", value);
  }

  /// @brief 音频下行网络抖动，单位为 ms 。
  ///
  FutureOr<int?> get jitter async {
    return await sendInstanceGet<int?>("jitter");
  }

  set jitter(FutureOr<int?> value) {
    sendInstanceSet("jitter", value);
  }

  /// @brief 音频解码帧率。
  ///
  FutureOr<double?> get decodeFrameRate async {
    return await sendInstanceGet<double?>("decodeFrameRate");
  }

  set decodeFrameRate(FutureOr<double?> value) {
    sendInstanceSet("decodeFrameRate", value);
  }
}

class ProblemFeedbackRoomInfo extends NativeClass {
  static const _$namespace =
      r'com.ss.bytertc.engine.type.ProblemFeedbackRoomInfo';
  static get codegen_$namespace => _$namespace;

  ProblemFeedbackRoomInfo([NativeClassOptions? options])
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

  /// @brief 房间 ID
  ///
  FutureOr<String?> get roomId async {
    return await sendInstanceGet<String?>("roomId");
  }

  set roomId(FutureOr<String?> value) {
    sendInstanceSet("roomId", value);
  }

  /// @brief 用户 ID
  ///
  FutureOr<String?> get userId async {
    return await sendInstanceGet<String?>("userId");
  }

  set userId(FutureOr<String?> value) {
    sendInstanceSet("userId", value);
  }

  /// @hidden
  ///

  FutureOr<String> getRoomId() async {
    return await nativeCall('getRoomId', []);
  }

  /// @hidden
  ///

  FutureOr<String> getUserId() async {
    return await nativeCall('getUserId', []);
  }
}

enum AudioRenderType {
  /// @brief 自定义渲染
  ///
  AUDIO_RENDER_TYPE_EXTERNAL(0),

  /// @brief 内部渲染
  ///
  AUDIO_RENDER_TYPE_INTERNAL(1);

  final dynamic $value;
  const AudioRenderType([this.$value]);
}

class SourceCrop extends NativeClass {
  static const _$namespace = r'com.ss.bytertc.engine.live.SourceCrop';
  static get codegen_$namespace => _$namespace;

  SourceCrop([NativeClassOptions? options])
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

  /// @brief 裁剪后得到的视频帧左上角横坐标相对于裁剪前整体画面的归一化比例，取值范围[0.0, 1.0)
  ///
  FutureOr<double?> get locationX async {
    return await sendInstanceGet<double?>("locationX");
  }

  set locationX(FutureOr<double?> value) {
    sendInstanceSet("locationX", value);
  }

  /// @brief 裁剪后得到的视频帧左上角纵坐标相对于裁剪前整体画面的归一化比例，取值范围[0.0, 1.0)
  ///
  FutureOr<double?> get locationY async {
    return await sendInstanceGet<double?>("locationY");
  }

  set locationY(FutureOr<double?> value) {
    sendInstanceSet("locationY", value);
  }

  /// @brief 裁剪后得到的视频帧宽度相对于裁剪前整体画面的归一化比例，取值范围(0.0, 1.0]
  ///
  FutureOr<double?> get widthProportion async {
    return await sendInstanceGet<double?>("widthProportion");
  }

  set widthProportion(FutureOr<double?> value) {
    sendInstanceSet("widthProportion", value);
  }

  /// @brief 裁剪后得到的视频帧高度相对于裁剪前整体画面的归一化比例，取值范围(0.0, 1.0]
  ///
  FutureOr<double?> get heightProportion async {
    return await sendInstanceGet<double?>("heightProportion");
  }

  set heightProportion(FutureOr<double?> value) {
    sendInstanceSet("heightProportion", value);
  }
}

enum RemoteAudioStateChangeReason {
  /// @brief 内部原因
  ///
  REMOTE_AUDIO_STATE_CHANGE_REASON_INTERNAL(0),

  /// @brief 网络阻塞
  ///
  REMOTE_AUDIO_STATE_CHANGE_REASON_NETWORK_CONGESTION(1),

  /// @brief 网络恢复正常
  ///
  REMOTE_AUDIO_STATE_CHANGE_REASON_NETWORK_RECOVERY(2),

  /// @brief 本地用户停止接收远端音频流
  ///
  REMOTE_AUDIO_STATE_CHANGE_REASON_LOCAL_MUTED(3),

  /// @brief 本地用户恢复接收远端音频流
  ///
  REMOTE_AUDIO_STATE_CHANGE_REASON_LOCAL_UNMUTED(4),

  /// @brief 远端用户停止发送音频流
  ///
  REMOTE_AUDIO_STATE_CHANGE_REASON_REMOTE_MUTED(5),

  /// @brief 远端用户恢复发送音频流
  ///
  REMOTE_AUDIO_STATE_CHANGE_REASON_REMOTE_UNMUTED(6),

  /// @brief 远端用户离开房间
  ///
  REMOTE_AUDIO_STATE_CHANGE_REASON_REMOTE_OFFLINE(7);

  final dynamic $value;
  const RemoteAudioStateChangeReason([this.$value]);
}

enum VideoPictureType {
  /// @brief 未知类型
  ///
  VIDEO_PICTURE_TYPE_UNKNOWN(0),

  /// @brief I 帧，关键帧，编解码不需要参考其他视频帧
  ///
  VIDEO_PICTURE_TYPE_I(1),

  /// @brief P 帧，向前参考帧，编解码需要参考前一帧视频帧
  ///
  VIDEO_PICTURE_TYPE_P(2),

  /// @brief B 帧，前后参考帧，编解码需要参考前后两帧视频帧
  ///
  VIDEO_PICTURE_TYPE_B(3),

  /// @hidden constructor/destructor
  ///
  value(-1);

  final dynamic $value;
  const VideoPictureType([this.$value]);
}

class VoiceReverbConfig extends NativeClass {
  static const _$namespace = r'com.ss.bytertc.engine.type.VoiceReverbConfig';
  static get codegen_$namespace => _$namespace;

  VoiceReverbConfig([NativeClassOptions? options])
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
  ///
  FutureOr<float?> get roomSize async {
    return await sendInstanceGet<float?>("roomSize");
  }

  set roomSize(FutureOr<float?> value) {
    sendInstanceSet("roomSize", value);
  }

  /// @brief 混响的拖尾长度，取值范围 `[0.0, 100.0]`。默认值为 `50.0f`。
  ///
  FutureOr<float?> get decayTime async {
    return await sendInstanceGet<float?>("decayTime");
  }

  set decayTime(FutureOr<float?> value) {
    sendInstanceSet("decayTime", value);
  }

  /// @brief 混响的衰减阻尼大小，取值范围 `[0.0, 100.0]`。默认值为 `50.0f`。
  ///
  FutureOr<float?> get damping async {
    return await sendInstanceGet<float?>("damping");
  }

  set damping(FutureOr<float?> value) {
    sendInstanceSet("damping", value);
  }

  /// @brief 早期反射信号强度。取值范围 `[-20.0, 10.0]`，单位为 dB。默认值为 `0.0f`。
  ///
  FutureOr<float?> get wetGain async {
    return await sendInstanceGet<float?>("wetGain");
  }

  set wetGain(FutureOr<float?> value) {
    sendInstanceSet("wetGain", value);
  }

  /// @brief 原始信号强度。取值范围 `[-20.0, 10.0]`，单位为 dB。默认值为 `0.0f`。
  ///
  FutureOr<float?> get dryGain async {
    return await sendInstanceGet<float?>("dryGain");
  }

  set dryGain(FutureOr<float?> value) {
    sendInstanceSet("dryGain", value);
  }

  /// @brief 早期反射信号的延迟。取值范围 `[0.0, 200.0]`，单位为 ms。默认值为 `0.0f`。
  ///
  FutureOr<float?> get preDelay async {
    return await sendInstanceGet<float?>("preDelay");
  }

  set preDelay(FutureOr<float?> value) {
    sendInstanceSet("preDelay", value);
  }
}

enum CameraId {
  /// @brief 前置摄像头
  ///
  CAMERA_ID_FRONT(0),

  /// @brief 后置摄像头（默认设置）
  ///
  CAMERA_ID_BACK(1),

  /// @hidden currently not available
  /// @brief 外接摄像头
  ///
  CAMERA_ID_EXTERNAL(2),

  /// @brief 无效值
  ///
  CAMERA_ID_INVALID(3);

  final dynamic $value;
  const CameraId([this.$value]);
}

class MediaTypeEnhancementConfig extends NativeClass {
  static const _$namespace =
      r'com.ss.bytertc.engine.type.MediaTypeEnhancementConfig';
  static get codegen_$namespace => _$namespace;

  MediaTypeEnhancementConfig([NativeClassOptions? options])
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
  ///
  FutureOr<boolean?> get enhanceSignaling async {
    return await sendInstanceGet<boolean?>("enhanceSignaling");
  }

  set enhanceSignaling(FutureOr<boolean?> value) {
    sendInstanceSet("enhanceSignaling", value);
  }

  /// @brief 对屏幕共享以外的其他音频，是否启用蜂窝网络辅助增强。默认不启用。
  ///
  FutureOr<boolean?> get enhanceAudio async {
    return await sendInstanceGet<boolean?>("enhanceAudio");
  }

  set enhanceAudio(FutureOr<boolean?> value) {
    sendInstanceSet("enhanceAudio", value);
  }

  /// @brief 对屏幕共享视频以外的其他视频，是否启用蜂窝网络辅助增强。默认不启用。
  ///
  FutureOr<boolean?> get enhanceVideo async {
    return await sendInstanceGet<boolean?>("enhanceVideo");
  }

  set enhanceVideo(FutureOr<boolean?> value) {
    sendInstanceSet("enhanceVideo", value);
  }

  /// @brief 对屏幕共享音频，是否启用蜂窝网络辅助增强。默认不启用。
  ///
  FutureOr<boolean?> get enhanceScreenAudio async {
    return await sendInstanceGet<boolean?>("enhanceScreenAudio");
  }

  set enhanceScreenAudio(FutureOr<boolean?> value) {
    sendInstanceSet("enhanceScreenAudio", value);
  }

  /// @brief 对屏幕共享视频，是否启用蜂窝网络辅助增强。默认不启用。
  ///
  FutureOr<boolean?> get enhanceScreenVideo async {
    return await sendInstanceGet<boolean?>("enhanceScreenVideo");
  }

  set enhanceScreenVideo(FutureOr<boolean?> value) {
    sendInstanceSet("enhanceScreenVideo", value);
  }
}

enum AVSyncState {
  /// @brief 音视频开始同步
  ///
  AV_SYNC_STATE_STREAM_SYNC_BEGIN(0),

  /// @brief 音视频同步过程中音频移除，但不影响当前的同步关系
  ///
  AV_SYNC_STATE_AUDIO_STREAM_REMOVE(1),

  /// @brief 音视频同步过程中视频移除，但不影响当前的同步关系
  ///
  AV_SYNC_STATE_VIDEO_STREAM_REMOVE(2),

  /// @hidden for internal use only
  /// @brief 订阅端设置同步
  ///
  AV_SYNC_STATE_SET_AV_SYNC_STRESM_ID(3);

  final dynamic $value;
  const AVSyncState([this.$value]);
}

enum VideoBufferType {
  /// @brief 原始内存数据
  ///
  RAW_MEMORY(0),

  /// @brief GL Texture
  ///
  GL_TEXTURE(2);

  final dynamic $value;
  const VideoBufferType([this.$value]);
}

enum MixedStreamLayoutRegionType {
  /// @brief 视频。
  ///
  MIXED_STREAM_LAYOUT_REGION_TYPE_VIDEO_STREAM(0),

  /// @brief 水印图片。
  ///
  MIXED_STREAM_LAYOUT_REGION_TYPE_IMAGE(1);

  final dynamic $value;
  const MixedStreamLayoutRegionType([this.$value]);
}

class VideoPreprocessorConfig extends NativeClass {
  static const _$namespace =
      r'com.ss.bytertc.engine.video.VideoPreprocessorConfig';
  static get codegen_$namespace => _$namespace;

  VideoPreprocessorConfig([NativeClassOptions? options])
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

  /// @brief 设置请求的像素格式，参看 VideoPixelFormat{@link #VideoPixelFormat}。 <br>
  ///        当前仅支持 `I420`、`TEXTURE_2D` 和 `UNKNOWN` 格式。
  ///
  FutureOr<VideoPixelFormat?> get requiredPixelFormat async {
    try {
      final result =
          await sendInstanceGet<VideoPixelFormat?>("requiredPixelFormat");
      if (result == null) {
        return null;
      }
      return VideoPixelFormat.values.firstWhere((element) => element == result);
    } catch (e) {
      return null;
    }
  }

  set requiredPixelFormat(FutureOr<VideoPixelFormat?> value) {
    sendInstanceSet("requiredPixelFormat", value);
  }
}

enum CodecMode {
  /// @brief 自动模式
  ///
  CODEC_MODE_AUTO(0),

  /// @brief 硬编码模式
  ///
  CODEC_MODE_HARDWARE(1),

  /// @brief 软编码模式
  ///
  CODEC_MODE_SOFTWARE(2);

  final dynamic $value;
  const CodecMode([this.$value]);
}

class FaceDetectionResult extends NativeClass {
  static const _$namespace = r'com.ss.bytertc.engine.video.FaceDetectionResult';
  static get codegen_$namespace => _$namespace;

  FaceDetectionResult([NativeClassOptions? options])
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
  ///        - !0：检测失败。详见[CV 错误码](https://www.volcengine.com/docs/6705/102042)。
  ///
  FutureOr<int?> get detectResult async {
    return await sendInstanceGet<int?>("detectResult");
  }

  set detectResult(FutureOr<int?> value) {
    sendInstanceSet("detectResult", value);
  }

  /// @brief 原始图片宽度(px)
  ///
  FutureOr<int?> get imageWidth async {
    return await sendInstanceGet<int?>("imageWidth");
  }

  set imageWidth(FutureOr<int?> value) {
    sendInstanceSet("imageWidth", value);
  }

  /// @brief 原始图片高度(px)
  ///
  FutureOr<int?> get imageHeight async {
    return await sendInstanceGet<int?>("imageHeight");
  }

  set imageHeight(FutureOr<int?> value) {
    sendInstanceSet("imageHeight", value);
  }

  /// @brief 识别到人脸的矩形框。数组的长度和检测到的人脸数量一致。参看 Rectangle{@link #Rectangle}。
  ///
  FutureOr<Array<Rectangle>?> get faces async {
    try {
      final result = await sendInstanceGet<Array<Rectangle>?>("faces");
      if (result == null) {
        return null;
      }
      final list = result.map((e) => packObject(
          e, () => Rectangle(const NativeClassOptions([], disableInit: true))));
      return list.toList();
    } catch (e) {
      return null;
    }
  }

  set faces(FutureOr<Array<Rectangle>?> value) {
    sendInstanceSet("faces", value);
  }

  /// @brief 进行人脸识别的视频帧的时间戳。
  ///
  FutureOr<long?> get frameTimestampUs async {
    return await sendInstanceGet<long?>("frameTimestampUs");
  }

  set frameTimestampUs(FutureOr<long?> value) {
    sendInstanceSet("frameTimestampUs", value);
  }
}

enum EarMonitorAudioFilter {
  /// @brief 无音频处理。
  ///
  NONE(1),

  /// @brief 经本地音频处理后的音频，包括 3A、变声、混响等 SDK 提供处理能力以及自定义音频处理。
  ///
  REUSE_AUDIO_PROCESSING(32768);

  final dynamic $value;
  const EarMonitorAudioFilter([this.$value]);
}

class MixedStreamSpatialAudioConfig extends NativeClass {
  static const _$namespace =
      r'com.ss.bytertc.engine.live.MixedStreamSpatialAudioConfig';
  static get codegen_$namespace => _$namespace;

  MixedStreamSpatialAudioConfig([NativeClassOptions? options])
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

  /// @brief 是否开启推流 CDN 时的空间音频效果。
  /// @note 当你启用此效果时，你需要设定推流中各个 MixedStreamLayoutRegionConfig{@link #MixedStreamLayoutRegionConfig} 的 `spatial_position` 值，实现空间音频效果。
  ///
  FutureOr<boolean?> get enableSpatialRender async {
    return await sendInstanceGet<boolean?>("enableSpatialRender");
  }

  set enableSpatialRender(FutureOr<boolean?> value) {
    sendInstanceSet("enableSpatialRender", value);
  }

  /// @brief 听众的空间位置。参看 Position{@link #Position}。 <br>
  ///        听众指收听来自 CDN 的音频流的用户。
  ///        WTN 流任务不支持设置本参数。
  ///
  FutureOr<Position?> get audienceSpatialPosition async {
    try {
      final result =
          await sendInstanceGet<Position?>("audienceSpatialPosition");
      if (result == null) {
        return null;
      }
      return packObject(result,
          () => Position(const NativeClassOptions([], disableInit: true)));
    } catch (e) {
      return null;
    }
  }

  set audienceSpatialPosition(FutureOr<Position?> value) {
    sendInstanceSet("audienceSpatialPosition", value);
  }

  /// @brief 听众的空间朝向。参看 HumanOrientation{@link #HumanOrientation}。 <br>
  ///        听众指收听来自 CDN 的音频流的用户。
  ///
  FutureOr<HumanOrientation?> get audienceSpatialOrientation async {
    try {
      final result = await sendInstanceGet<HumanOrientation?>(
          "audienceSpatialOrientation");
      if (result == null) {
        return null;
      }
      return packObject(
          result,
          () => HumanOrientation(
              const NativeClassOptions([], disableInit: true)));
    } catch (e) {
      return null;
    }
  }

  set audienceSpatialOrientation(FutureOr<HumanOrientation?> value) {
    sendInstanceSet("audienceSpatialOrientation", value);
  }

  /// @hidden for internal use
  /// @brief 设置听众的空间位置。听众指收听来自 CDN 的音频流的用户。
  /// @param x 在空间直角坐标系中的 x 坐标。
  /// @param y 在空间直角坐标系中的 y 坐标。
  /// @param z 在空间直角坐标系中的 z 坐标。
  ///

  FutureOr<MixedStreamSpatialAudioConfig> setAudienceSpatialPosition(
      float x, float y, float z) async {
    final result = await nativeCall('setAudienceSpatialPosition', [x, y, z]);
    return packObject(
        result,
        () => MixedStreamSpatialAudioConfig(
            const NativeClassOptions([], disableInit: true)));
  }
}

enum ScreenMediaType {
  /// @brief 仅采集视频
  ///
  SCREEN_MEDIA_TYPE_VIDEO_ONLY(0),

  /// @brief 仅采集音频
  ///
  SCREEN_MEDIA_TYPE_AUDIO_ONLY(1),

  /// @brief 采集音频和视频
  ///
  SCREEN_MEDIA_TYPE_VIDEO_AND_AUDIO(2);

  final dynamic $value;
  const ScreenMediaType([this.$value]);
}

enum RemoteVideoStateChangeReason {
  /// @brief 内部原因
  ///
  REMOTE_VIDEO_STATE_CHANGE_REASON_INTERNAL(0),

  /// @brief 网络阻塞
  ///
  REMOTE_VIDEO_STATE_CHANGE_REASON_NETWORK_CONGESTION(1),

  /// @brief 网络恢复正常
  ///
  REMOTE_VIDEO_STATE_CHANGE_REASON_NETWORK_RECOVERY(2),

  /// @brief 本地用户停止接收远端视频流或本地用户禁用视频模块
  ///
  REMOTE_VIDEO_STATE_CHANGE_REASON_LOCAL_MUTED(3),

  /// @brief 本地用户恢复接收远端视频流或本地用户启用视频模块
  ///
  REMOTE_VIDEO_STATE_CHANGE_REASON_LOCAL_UNMUTED(4),

  /// @brief 远端用户停止发送视频流或远端用户禁用视频模块
  ///
  REMOTE_VIDEO_STATE_CHANGE_REASON_REMOTE_MUTED(5),

  /// @brief 远端用户恢复发送视频流或远端用户启用视频模块
  ///
  REMOTE_VIDEO_STATE_CHANGE_REASON_REMOTE_UNMUTED(6),

  /// @brief 远端用户离开房间。
  ///
  REMOTE_VIDEO_STATE_CHANGE_REASON_REMOTE_OFFLINE(7);

  final dynamic $value;
  const RemoteVideoStateChangeReason([this.$value]);
}

class GameRoomConfig extends NativeClass {
  static const _$namespace = r'com.ss.bytertc.engine.GameRoomConfig';
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

  /// @brief 房间模式，参看 GameRoomType{@link #GameRoomType}，默认为 `TEAM`，进房后不可更改。
  ///
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

  /// @brief 游戏场景，参看 GameSceneType{@link #GameSceneType}。
  ///
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

  FutureOr<int> getGameSceneType() async {
    return await nativeCall('getGameSceneType', []);
  }

  FutureOr<int> getGameRoomType() async {
    return await nativeCall('getGameRoomType', []);
  }
}

enum VideoSuperResolutionMode {
  /// @brief 关闭超分。
  ///
  VIDEO_SUPER_RESOLUTION_MODE_OFF(0),

  /// @brief 开启超分。
  ///
  VIDEO_SUPER_RESOLUTION_MODE_ON(1);

  final dynamic $value;
  const VideoSuperResolutionMode([this.$value]);
}

enum ProblemFeedbackOption {
  /// @brief 没有问题
  ///
  NONE(0),

  /// @brief 其他问题
  ///
  OTHER_MESSAGE(1),

  /// @brief 连接失败
  ///
  DISCONNECTED(2),

  /// @brief 耳返延迟大
  ///
  EAR_BACK_DELAY(4),

  /// @brief 本端有杂音
  ///
  LOCAL_NOISE(1024),

  /// @brief 本端声音卡顿
  ///
  LOCAL_AUDIO_LAGGING(2048),

  /// @brief 本端无声音
  ///
  LOCAL_NO_AUDIO(4096),

  /// @brief 本端声音大/小
  ///
  LOCAL_AUDIO_STRENGTH(8192),

  /// @brief 本端有回声
  ///
  LOCAL_ECHO(16384),

  /// @brief 本端视频模糊
  ///
  LOCAL_VIDEO_FUZZY(16777216),

  /// @brief 本端音视频不同步
  ///
  LOCAL_NOT_SYNC(33554432),

  /// @brief 本端视频卡顿
  ///
  LOCAL_VIDEO_LAGGING(67108864),

  /// @brief 本端无画面
  ///
  LOCAL_NO_VIDEO(134217728),

  /// @brief 远端有杂音
  ///
  REMOTE_NOISE(32),

  /// @brief 远端声音卡顿
  ///
  REMOTE_AUDIO_LAGGING(64),

  /// @brief 远端无声音
  ///
  REMOTE_NO_AUDIO(128),

  /// @brief 远端声音大/小
  ///
  REMOTE_AUDIO_STRENGTH(256),

  /// @brief 远端有回声
  ///
  REMOTE_ECHO(512),

  /// @brief 远端视频模糊
  ///
  REMOTE_VIDEO_FUZZY(524288),

  /// @brief 远端音视频不同步
  ///
  REMOTE_NOT_SYNC(1048576),

  /// @brief 远端视频卡顿
  ///
  REMOTE_VIDEO_LAGGING(2097152),

  /// @brief 远端无画面
  ///
  REMOTE_NO_VIDEO(4194304),

  /// @hidden constructor/destructor
  ///
  value;

  final dynamic $value;
  const ProblemFeedbackOption([this.$value]);
}

class MediaPlayerConfig extends NativeClass {
  static const _$namespace = r'com.ss.bytertc.engine.data.MediaPlayerConfig';
  static get codegen_$namespace => _$namespace;

  MediaPlayerConfig([NativeClassOptions? options])
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

  /// @brief 混音播放类型，详见 AudioMixingType{@link #AudioMixingType}
  ///
  FutureOr<AudioMixingType?> get type async {
    try {
      final result = await sendInstanceGet<AudioMixingType?>("type");
      if (result == null) {
        return null;
      }
      return AudioMixingType.values.firstWhere((element) => element == result);
    } catch (e) {
      return null;
    }
  }

  set type(FutureOr<AudioMixingType?> value) {
    sendInstanceSet("type", value);
  }

  /// @brief 混音播放次数 <br>
  ///       - play_count <= 0: 无限循环
  ///       - play_count == 1: 播放一次（默认）
  ///       - play_count > 1: 播放 play_count 次
  ///
  FutureOr<int?> get playCount async {
    return await sendInstanceGet<int?>("playCount");
  }

  set playCount(FutureOr<int?> value) {
    sendInstanceSet("playCount", value);
  }

  /// @brief 混音起始位置。默认值为 0，单位为毫秒。
  ///
  FutureOr<int?> get startPos async {
    return await sendInstanceGet<int?>("startPos");
  }

  set startPos(FutureOr<int?> value) {
    sendInstanceSet("startPos", value);
  }

  /// @brief 设置音频文件混音时，收到 onMediaPlayerPlayingProgress{@link #IMediaPlayerEventHandler#onMediaPlayerPlayingProgress} 的间隔。单位毫秒。 <br>
  ///       - interval > 0 时，触发回调。实际间隔为 10 的倍数。如果输入数值不能被 10 整除，将自动向上取整。例如传入 `52`，实际间隔为 60 ms。
  ///       - interval <= 0 时，不会触发回调。
  ///
  FutureOr<long?> get callbackOnProgressInterval async {
    return await sendInstanceGet<long?>("callbackOnProgressInterval");
  }

  set callbackOnProgressInterval(FutureOr<long?> value) {
    sendInstanceSet("callbackOnProgressInterval", value);
  }

  /// @brief 在采集音频数据时，附带本地混音文件播放进度的时间戳。启用此功能会提升远端人声和音频文件混音播放时的同步效果。 <br>
  ///        - 仅在单个音频文件混音时使用有效。
  ///        - `true` 时开启此功能，`false` 时关闭此功能，默认为关闭。
  ///
  FutureOr<boolean?> get syncProgressToRecordFrame async {
    return await sendInstanceGet<boolean?>("syncProgressToRecordFrame");
  }

  set syncProgressToRecordFrame(FutureOr<boolean?> value) {
    sendInstanceSet("syncProgressToRecordFrame", value);
  }

  /// @brief 是否自动播放。如果不自动播放，调用 start{@link #IMediaPlayer#start} 播放音乐文件。
  ///
  FutureOr<boolean?> get autoPlay async {
    return await sendInstanceGet<boolean?>("autoPlay");
  }

  set autoPlay(FutureOr<boolean?> value) {
    sendInstanceSet("autoPlay", value);
  }
}

enum RemoteUserPriority {
  /// @brief 用户优先级为低，默认值。
  ///
  REMOTE_USER_PRIORITY_LOW(0),

  /// @brief 用户优先级为正常。
  ///
  REMOTE_USER_PRIORITY_MEDIUM(100),

  /// @brief 用户优先级为高。
  ///
  REMOTE_USER_PRIORITY_HIGH(200);

  final dynamic $value;
  const RemoteUserPriority([this.$value]);
}

enum LocalProxyState {
  /// @brief TCP 代理服务器连接成功。
  ///
  INITED(0),

  /// @brief 本地代理连接成功。
  ///
  CONNECTED(1),

  /// @brief 本地代理连接出现错误。
  ///
  ERROR(2);

  final dynamic $value;
  const LocalProxyState([this.$value]);
}

class RemoteVideoRenderConfig extends NativeClass {
  static const _$namespace = r'com.ss.bytertc.engine.RemoteVideoRenderConfig';
  static get codegen_$namespace => _$namespace;

  RemoteVideoRenderConfig([NativeClassOptions? options])
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

  /// @brief 渲染模式。 <br>
  ///        - 1（`RENDER_MODE_HIDDEN`）：视窗填满优先，默认值。视频帧等比缩放，直至视窗被视频填满。如果视频帧长宽比例与视窗不同，视频帧的多出部分将无法显示。缩放完成后，视频帧的一边长和视窗的对应边长一致，另一边长大于等于视窗对应边长。
  ///        - 2（`RENDER_MODE_FIT`）：视频帧内容全部显示优先。视频帧等比缩放，直至视频帧能够在视窗上全部显示。如果视频帧长宽比例与视窗不同，视窗上未被视频帧填满区域将填充 `backgroundColor`。缩放完成后，视频帧的一边长和视窗的对应边长一致，另一边长小于等于视窗对应边长。
  ///        - 3（`RENDER_MODE_FILL`）：视频帧自适应画布。视频帧非等比缩放，直至画布被填满。在此过程中，视频帧的长宽比例可能会发生变化。
  ///
  FutureOr<int?> get renderMode async {
    return await sendInstanceGet<int?>("renderMode");
  }

  set renderMode(FutureOr<int?> value) {
    sendInstanceSet("renderMode", value);
  }

  /// @brief 用于填充画布空白部分的背景颜色。取值范围是 `[0x00000000, 0xFFFFFFFF]`。默认值是 `0x00000000`。其中，透明度设置无效。
  ///
  FutureOr<int?> get backgroundColor async {
    return await sendInstanceGet<int?>("backgroundColor");
  }

  set backgroundColor(FutureOr<int?> value) {
    sendInstanceSet("backgroundColor", value);
  }

  /// @brief 视频帧旋转角度。参看 VideoRotation{@link #VideoRotation}。默认为 0 度，即不做旋转处理。
  ///
  FutureOr<VideoRotation?> get renderRotation async {
    try {
      final result = await sendInstanceGet<VideoRotation?>("renderRotation");
      if (result == null) {
        return null;
      }
      return VideoRotation.values.firstWhere((element) => element == result);
    } catch (e) {
      return null;
    }
  }

  set renderRotation(FutureOr<VideoRotation?> value) {
    sendInstanceSet("renderRotation", value);
  }
}

enum MixedStreamType {
  /// @brief 服务端合流。
  ///
  MIXED_STREAM_TYPE_BY_SERVER(0),

  /// @brief 端云一体合流。SDK 智能决策在客户端或服务端完成合流。
  ///
  MIXED_STREAM_TYPE_BY_CLIENT(1);

  final dynamic $value;
  const MixedStreamType([this.$value]);
}

class Position extends NativeClass {
  static const _$namespace = r'com.ss.bytertc.engine.data.Position';
  static get codegen_$namespace => _$namespace;

  Position([NativeClassOptions? options])
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
  ///
  FutureOr<float?> get x async {
    return await sendInstanceGet<float?>("x");
  }

  set x(FutureOr<float?> value) {
    sendInstanceSet("x", value);
  }

  /// @brief y 坐标
  ///
  FutureOr<float?> get y async {
    return await sendInstanceGet<float?>("y");
  }

  set y(FutureOr<float?> value) {
    sendInstanceSet("y", value);
  }

  /// @brief z 坐标
  ///
  FutureOr<float?> get z async {
    return await sendInstanceGet<float?>("z");
  }

  set z(FutureOr<float?> value) {
    sendInstanceSet("z", value);
  }
}

enum FrameRate {
  FRAME_RATE_FPS_1(1),

  FRAME_RATE_FPS_7(7),

  FRAME_RATE_FPS_10(10),

  FRAME_RATE_FPS_15(15),

  FRAME_RATE_FPS_24(24),

  FRAME_RATE_FPS_30(30),

  FRAME_RATE_FPS_60(60);

  final dynamic $value;
  const FrameRate([this.$value]);
}

enum RecordingErrorCode {
  /// @brief 录制正常
  ///
  RECORDING_ERROR_CODE_OK(0),

  /// @brief 没有文件写权限
  ///
  RECORDING_ERROR_CODE_NO_PERMISSION(-1),

  /// @brief 当前版本 SDK 不支持本地录制功能，请联系技术支持人员
  ///
  RECORDING_ERROR_CODE_NOT_SUPPORT(-2),

  /// @brief 其他异常
  ///
  RECORDING_ERROR_CODE_NO_OTHER(-3);

  final dynamic $value;
  const RecordingErrorCode([this.$value]);
}

class RemoteVideoStats extends NativeClass {
  static const _$namespace = r'com.ss.bytertc.engine.type.RemoteVideoStats';
  static get codegen_$namespace => _$namespace;

  RemoteVideoStats([NativeClassOptions? options])
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

  /// @brief 远端视频流宽度
  ///
  FutureOr<int?> get width async {
    return await sendInstanceGet<int?>("width");
  }

  set width(FutureOr<int?> value) {
    sendInstanceSet("width", value);
  }

  /// @brief 远端视频流高度
  ///
  FutureOr<int?> get height async {
    return await sendInstanceGet<int?>("height");
  }

  set height(FutureOr<int?> value) {
    sendInstanceSet("height", value);
  }

  /// @brief 视频丢包率。统计周期内的视频下行丢包率，取值范围为 [0，1] 。
  ///
  FutureOr<float?> get videoLossRate async {
    return await sendInstanceGet<float?>("videoLossRate");
  }

  set videoLossRate(FutureOr<float?> value) {
    sendInstanceSet("videoLossRate", value);
  }

  /// @brief 接收码率。统计周期内的视频接收码率，单位为 kbps 。
  ///
  FutureOr<float?> get receivedKBitrate async {
    return await sendInstanceGet<float?>("receivedKBitrate");
  }

  set receivedKBitrate(FutureOr<float?> value) {
    sendInstanceSet("receivedKBitrate", value);
  }

  /// @brief 解码器输出帧率。统计周期内的视频解码器输出帧率，单位 fps 。
  ///
  FutureOr<int?> get decoderOutputFrameRate async {
    return await sendInstanceGet<int?>("decoderOutputFrameRate");
  }

  set decoderOutputFrameRate(FutureOr<int?> value) {
    sendInstanceSet("decoderOutputFrameRate", value);
  }

  /// @brief 渲染帧率。统计周期内的视频渲染帧率，单位 fps 。
  ///
  FutureOr<int?> get rendererOutputFrameRate async {
    return await sendInstanceGet<int?>("rendererOutputFrameRate");
  }

  set rendererOutputFrameRate(FutureOr<int?> value) {
    sendInstanceSet("rendererOutputFrameRate", value);
  }

  /// @brief 卡顿次数。统计周期内的卡顿次数。
  ///
  FutureOr<int?> get stallCount async {
    return await sendInstanceGet<int?>("stallCount");
  }

  set stallCount(FutureOr<int?> value) {
    sendInstanceSet("stallCount", value);
  }

  /// @brief 卡顿时长。统计周期内的视频卡顿总时长。单位 ms 。
  ///
  FutureOr<int?> get stallDuration async {
    return await sendInstanceGet<int?>("stallDuration");
  }

  set stallDuration(FutureOr<int?> value) {
    sendInstanceSet("stallDuration", value);
  }

  /// @brief 用户体验级别的端到端延时，从发送端采集完成编码开始到接收端解码完成渲染开始的延时，单位为毫秒
  ///
  FutureOr<long?> get e2eDelay async {
    return await sendInstanceGet<long?>("e2eDelay");
  }

  set e2eDelay(FutureOr<long?> value) {
    sendInstanceSet("e2eDelay", value);
  }

  /// @brief 所属用户的媒体流是否为屏幕流。你可以知道当前统计数据来自主流还是屏幕流。
  ///
  FutureOr<boolean?> get isScreen async {
    return await sendInstanceGet<boolean?>("isScreen");
  }

  set isScreen(FutureOr<boolean?> value) {
    sendInstanceSet("isScreen", value);
  }

  /// @brief 统计间隔，此次统计周期的间隔，单位为 ms 。 <br>
  ///        此字段用于设置回调的统计周期，目前设置为 2s 。
  ///
  FutureOr<int?> get statsInterval async {
    return await sendInstanceGet<int?>("statsInterval");
  }

  set statsInterval(FutureOr<int?> value) {
    sendInstanceSet("statsInterval", value);
  }

  /// @brief 往返时延，单位为 ms 。
  ///
  FutureOr<int?> get rtt async {
    return await sendInstanceGet<int?>("rtt");
  }

  set rtt(FutureOr<int?> value) {
    sendInstanceSet("rtt", value);
  }

  /// @brief 远端用户在进房后发生视频卡顿的累计时长占视频总有效时长的百分比（\%）。视频有效时长是指远端用户进房发布视频流后，除停止发送视频流和禁用视频模块之外的视频时长。
  ///
  FutureOr<int?> get frozenRate async {
    return await sendInstanceGet<int?>("frozenRate");
  }

  set frozenRate(FutureOr<int?> value) {
    sendInstanceSet("frozenRate", value);
  }

  /// @brief 视频的编码类型，具体参考 VideoCodecType{@link #VideoCodecType-2} 。
  ///
  FutureOr<int?> get codecType async {
    return await sendInstanceGet<int?>("codecType");
  }

  set codecType(FutureOr<int?> value) {
    sendInstanceSet("codecType", value);
  }

  /// @brief 对应多种分辨率的流的下标。
  ///
  FutureOr<int?> get videoIndex async {
    return await sendInstanceGet<int?>("videoIndex");
  }

  set videoIndex(FutureOr<int?> value) {
    sendInstanceSet("videoIndex", value);
  }

  /// @brief 视频下行网络抖动，单位为 ms。
  ///
  FutureOr<int?> get jitter async {
    return await sendInstanceGet<int?>("jitter");
  }

  set jitter(FutureOr<int?> value) {
    sendInstanceSet("jitter", value);
  }

  /// @hidden for internal use only
  /// @brief 远端视频超分模式，参看 VideoSuperResolutionMode{@link #VideoSuperResolutionMode}。
  ///
  FutureOr<int?> get superResolutionMode async {
    return await sendInstanceGet<int?>("superResolutionMode");
  }

  set superResolutionMode(FutureOr<int?> value) {
    sendInstanceSet("superResolutionMode", value);
  }

  /// @brief 用户体验级别的端到端延时。从发送端开始采集到接收端渲染完成的延时，单位为 ms 。
  ///
  FutureOr<int?> get capToRenderDelay async {
    return await sendInstanceGet<int?>("capToRenderDelay");
  }

  set capToRenderDelay(FutureOr<int?> value) {
    sendInstanceSet("capToRenderDelay", value);
  }

  /// @brief 音画同步差异，单位为 ms 。
  ///
  FutureOr<int?> get avSyncDiffMs async {
    return await sendInstanceGet<int?>("avSyncDiffMs");
  }

  set avSyncDiffMs(FutureOr<int?> value) {
    sendInstanceSet("avSyncDiffMs", value);
  }

  /// @brief 视频解码平均耗时，单位 ms。
  ///
  FutureOr<int?> get codecElapsePerFrame async {
    return await sendInstanceGet<int?>("codecElapsePerFrame");
  }

  set codecElapsePerFrame(FutureOr<int?> value) {
    sendInstanceSet("codecElapsePerFrame", value);
  }
}

enum SyncInfoStreamType {
  /// @brief 音频流
  ///
  SYNC_INFO_STREAM_TYPE_AUDIO(0);

  final dynamic $value;
  const SyncInfoStreamType([this.$value]);
}

class ForwardStreamInfo extends NativeClass {
  static const _$namespace = r'com.ss.bytertc.engine.data.ForwardStreamInfo';
  static get codegen_$namespace => _$namespace;

  ForwardStreamInfo([NativeClassOptions? options])
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
  FutureOr<String?> get roomId async {
    return await sendInstanceGet<String?>("roomId");
  }

  set roomId(FutureOr<String?> value) {
    sendInstanceSet("roomId", value);
  }

  /// @brief 使用转发目标房间 RoomID 和 UserID 生成的 Token。 <br>
  ///        测试时可使用控制台生成临时 Token，正式上线需要使用密钥 SDK 在你的服务端生成并下发 Token。 <br>
  ///        如果 Token 无效，转发失败。
  ///
  FutureOr<String?> get token async {
    return await sendInstanceGet<String?>("token");
  }

  set token(FutureOr<String?> value) {
    sendInstanceSet("token", value);
  }
}

enum MixedStreamAudioCodecType {
  /// @brief AAC 格式。
  ///
  MIXED_STREAM_AUDIO_CODEC_TYPE_AAC;

  final dynamic $value;
  const MixedStreamAudioCodecType([this.$value]);
}

enum AudioSelectionPriority {
  /// @brief 正常，参加音频选路
  ///
  AUDIO_SELECTION_PRIORITY_NORMAL(0),

  /// @brief 高优先级，跳过音频选路
  ///
  AUDIO_SELECTION_PRIORITY_HIGIH(1);

  final dynamic $value;
  const AudioSelectionPriority([this.$value]);
}

class AudioPropertiesInfo extends NativeClass {
  static const _$namespace = r'com.ss.bytertc.engine.data.AudioPropertiesInfo';
  static get codegen_$namespace => _$namespace;

  AudioPropertiesInfo([NativeClassOptions? options])
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
  ///        - [0, 25]: 近似无声
  ///        - [26, 75]: 低音量
  ///        - [76, 204]: 中音量
  ///        - [205, 255]: 高音量
  ///
  FutureOr<int?> get linearVolume async {
    return await sendInstanceGet<int?>("linearVolume");
  }

  set linearVolume(FutureOr<int?> value) {
    sendInstanceSet("linearVolume", value);
  }

  /// @brief 非线性音量。由原始音量的对数值转化而来，因此在中低音量时更灵敏，可以用作 Active Speaker（房间内最活跃用户）的识别。取值范围是：[-127，0]，单位 dB。 <br>
  ///        - [-127, -60]: 近似无声
  ///        - [-59, -40]: 低音量
  ///        - [-39, -20]: 中音量
  ///        - [-19, 0]: 高音量
  ///
  FutureOr<int?> get nonlinearVolume async {
    return await sendInstanceGet<int?>("nonlinearVolume");
  }

  set nonlinearVolume(FutureOr<int?> value) {
    sendInstanceSet("nonlinearVolume", value);
  }

  /// @brief 频谱数组
  ///
  FutureOr<Array<float>?> get spectrum async {
    return await sendInstanceGet<Array<float>?>("spectrum");
  }

  set spectrum(FutureOr<Array<float>?> value) {
    sendInstanceSet("spectrum", value);
  }

  /// @brief 人声检测（VAD）结果 <br>
  ///        - 1: 检测到人声。
  ///        - 0: 未检测到人声。
  ///        - -1: 未开启 VAD。
  ///
  FutureOr<int?> get vad async {
    return await sendInstanceGet<int?>("vad");
  }

  set vad(FutureOr<int?> value) {
    sendInstanceSet("vad", value);
  }

  /// @brief 本地用户的人声基频，单位为赫兹。 <br>
  ///        同时满足以下两个条件时，返回的值为本地用户的人声基频： <br>
  ///      - 调用 enableAudioPropertiesReport{@link #RTCEngine#enableAudioPropertiesReport}，并设置参数 enableVoicePitch 的值为 `true`。
  ///      - 本地采集的音频中包含本地用户的人声。
  ///        其他情况下返回 `0`。
  ///
  FutureOr<double?> get voicePitch async {
    return await sendInstanceGet<double?>("voicePitch");
  }

  set voicePitch(FutureOr<double?> value) {
    sendInstanceSet("voicePitch", value);
  }
}

enum VideoSimulcastMode {
  /// @brief 单流模式。始终只有 1 路分辨率的流。
  ///
  VIDEO_SIMULCAST_MODE_ONLY_ONE(0),

  /// @brief 按需订阅模式。发送端会根据订阅端的状态，按需发布。无订阅偏好设置默认发送 2 路。
  ///
  VIDEO_SIMULCAST_MODE_ON_DEMAND(1),

  /// @brief 订阅弱流。发送端始终按照设置的参数发布所有大小流。默认发送 2 路。
  ///
  VIDEO_SIMULCAST_MODE_ALWAYS_SIMULCAST(2);

  final dynamic $value;
  const VideoSimulcastMode([this.$value]);
}

class UserInfo extends NativeClass {
  static const _$namespace = r'com.ss.bytertc.engine.UserInfo';
  static get codegen_$namespace => _$namespace;

  UserInfo([NativeClassOptions? options])
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

  /// @brief 用户 ID。该字符串符合正则表达式：`[a-zA-Z0-9_\@\\-\\.]{1,128}`。 <br>
  ///        你需要自行设置或管理 uid，并保证同一房间内每个 uid 的唯一性。
  ///
  FutureOr<String?> get uid async {
    return await sendInstanceGet<String?>("uid");
  }

  set uid(FutureOr<String?> value) {
    sendInstanceSet("uid", value);
  }

  /// @brief 用户的额外信息，最大长度为 200 字节。会在 `onUserJoined` 中回调给远端用户。
  ///
  FutureOr<String?> get extraInfo async {
    return await sendInstanceGet<String?>("extraInfo");
  }

  set extraInfo(FutureOr<String?> value) {
    sendInstanceSet("extraInfo", value);
  }

  /// @hidden constructor/destructor
  ///

  FutureOr<String> getUid() async {
    return await nativeCall('getUid', []);
  }

  /// @hidden constructor/destructor
  ///

  FutureOr<String> getExtraInfo() async {
    return await nativeCall('getExtraInfo', []);
  }
}

enum MediaPlayerCustomSourceStreamType {
  /// @brief 当播放来自本地的 PCM 数据时，使用此选项。
  ///
  RAW(0),

  /// @brief 当播放来自内存的音频数据时，使用此选项。
  ///
  ENCODED(1);

  final dynamic $value;
  const MediaPlayerCustomSourceStreamType([this.$value]);
}

enum MediaDeviceError {
  /// @brief 媒体设备正常
  ///
  MEDIA_DEVICE_ERROR_OK(0),

  /// @brief 没有权限启动媒体设备
  ///
  MEDIA_DEVICE_ERROR_NOPERMISSION(1),

  /// @brief 媒体设备已经在使用中
  ///
  MEDIA_DEVICE_ERROR_DEVICEBUSY(2),

  /// @brief 媒体设备错误
  ///
  MEDIA_DEVICE_ERROR_DEVICEFAILURE(3),

  /// @brief 未找到指定的媒体设备
  ///
  MEDIA_DEVICE_ERROR_DEVICENOTFOUND(4),

  /// @brief 媒体设备被移除
  ///
  MEDIA_DEVICE_ERROR_DEVICEDISCONNECTED(5),

  /// @brief 无采集数据。当媒体设备的预期行为是正常采集，但没有收到采集数据时，将收到该错误。
  ///
  MEDIA_DEVICE_ERROR_DEVICENOCALLBACK(6),

  /// @brief 设备采样率不支持
  ///
  MEDIA_DEVICE_ERROR_UNSUPPORTFORMAT(7);

  final dynamic $value;
  const MediaDeviceError([this.$value]);
}

class StreamSyncInfoConfig extends NativeClass {
  static const _$namespace = r'com.ss.bytertc.engine.data.StreamSyncInfoConfig';
  static get codegen_$namespace => _$namespace;

  StreamSyncInfoConfig([NativeClassOptions? options])
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

  /// @brief 信息发送的重复次数。取值范围是 [0,25]，建议设置为 [3,5]
  ///
  FutureOr<int?> get repeatCount async {
    return await sendInstanceGet<int?>("repeatCount");
  }

  set repeatCount(FutureOr<int?> value) {
    sendInstanceSet("repeatCount", value);
  }

  /// @brief 媒体流信息同步的流类型，见 SyncInfoStreamType{@link #SyncInfoStreamType}
  ///
  FutureOr<SyncInfoStreamType?> get streamType async {
    try {
      final result = await sendInstanceGet<SyncInfoStreamType?>("streamType");
      if (result == null) {
        return null;
      }
      return SyncInfoStreamType.values
          .firstWhere((element) => element == result);
    } catch (e) {
      return null;
    }
  }

  set streamType(FutureOr<SyncInfoStreamType?> value) {
    sendInstanceSet("streamType", value);
  }
}

enum ZoomDirectionType {
  /// @brief 相机向左移动
  ///
  CAMERA_MOVE_LEFT(0),

  /// @brief 相机向右移动
  ///
  CAMERA_MOVE_RIGHT(1),

  /// @brief 相机向上移动
  ///
  CAMERA_MOVE_UP(2),

  /// @brief 相机向下移动
  ///
  CAMERA_MOVE_DOWN(3),

  /// @brief 相机缩小焦距
  ///
  CAMERA_ZOOM_OUT(4),

  /// @brief 相机放大焦距
  ///
  CAMERA_ZOOM_IN(5),

  /// @brief 恢复到原始大小
  ///
  CAMERA_RESET(6);

  final dynamic $value;
  const ZoomDirectionType([this.$value]);
}

enum AudioQuality {
  /// @brief 低音质
  ///
  AUDIO_QUALITY_LOW(0),

  /// @brief 中音质
  ///
  AUDIO_QUALITY_MEDIUM(1),

  /// @brief 高音质
  ///
  AUDIO_QUALITY_HIGH(2),

  /// @brief 超高音质
  ///
  AUDIO_QUALITY_ULTRA_HIGH(3);

  final dynamic $value;
  const AudioQuality([this.$value]);
}

class VideoCanvas extends NativeClass {
  static const _$namespace = r'com.ss.bytertc.engine.VideoCanvas';
  static get codegen_$namespace => _$namespace;

  /// @brief 视窗填满优先，默认值。 <br>
  ///        视频帧等比缩放，直至视窗被视频填满。如果视频帧长宽比例与视窗不同，视频帧的多出部分将无法显示。 <br>
  ///        缩放完成后，视频帧的一边长和视窗的对应边长一致，另一边长大于等于视窗对应边长。
  ///
  static Future<int> get RENDER_MODE_HIDDEN async {
    return await NativeClassUtils.sendStaticGet<int>(
        _$namespace, "RENDER_MODE_HIDDEN", "com.volcengine.rtc.hybrid_runtime");
  }

  /// @brief 视频帧内容全部显示优先。 <br>
  ///        视频帧等比缩放，直至视频帧能够在视窗上全部显示。如果视频帧长宽比例与视窗不同，视窗上未被视频帧填满区域将填充 `background_color`。 <br>
  ///        缩放完成后，视频帧的一边长和视窗的对应边长一致，另一边长小于等于视窗对应边长。
  ///
  static Future<int> get RENDER_MODE_FIT async {
    return await NativeClassUtils.sendStaticGet<int>(
        _$namespace, "RENDER_MODE_FIT", "com.volcengine.rtc.hybrid_runtime");
  }

  /// @brief 视频帧自适应画布。 <br>
  ///        视频帧非等比缩放，直至画布被填满。在此过程中，视频帧的长宽比例可能会发生变化。
  ///
  static Future<int> get RENDER_MODE_FILL async {
    return await NativeClassUtils.sendStaticGet<int>(
        _$namespace, "RENDER_MODE_FILL", "com.volcengine.rtc.hybrid_runtime");
  }

  VideoCanvas([NativeClassOptions? options])
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

  /// @brief 本地视图句柄。 <br>
  ///        渲染 View 对象时，使用此字段，并将 `renderSurface` 设置为 `null`。
  ///
  FutureOr<View?> get renderView async {
    return await sendInstanceGet<View?>("renderView");
  }

  set renderView(FutureOr<View?> value) {
    sendInstanceSet("renderView", value);
  }

  /// @brief 本地 surface 句柄。 <br>
  ///        渲染 Surface 对象时，使用此字段，并将 `renderView` 设置为 `null`。
  ///
  FutureOr<Surface?> get renderSurface async {
    return await sendInstanceGet<Surface?>("renderSurface");
  }

  set renderSurface(FutureOr<Surface?> value) {
    sendInstanceSet("renderSurface", value);
  }

  /// @brief 渲染模式，可选 `RENDER_MODE_HIDDEN(1)`, `RENDER_MODE_FIT(2)` 和 `RENDER_MODE_FILL(3)`。默认值为 `RENDER_MODE_HIDDEN(1)`。
  ///
  FutureOr<int?> get renderMode async {
    return await sendInstanceGet<int?>("renderMode");
  }

  set renderMode(FutureOr<int?> value) {
    sendInstanceSet("renderMode", value);
  }

  /// @brief 用于填充画布空白部分的背景颜色。取值范围是 `[0x00000000, 0xFFFFFFFF]`。默认值是 `0x00000000`。其中，透明度设置无效。
  ///
  FutureOr<int?> get backgroundColor async {
    return await sendInstanceGet<int?>("backgroundColor");
  }

  set backgroundColor(FutureOr<int?> value) {
    sendInstanceSet("backgroundColor", value);
  }

  /// @brief 视频帧旋转角度。参看 VideoRotation{@link #VideoRotation}。默认为 0 度，即不做旋转处理。 <br>
  ///        该设置仅对远端视频有效，对本地视频设置不生效。
  ///
  FutureOr<VideoRotation?> get renderRotation async {
    try {
      final result = await sendInstanceGet<VideoRotation?>("renderRotation");
      if (result == null) {
        return null;
      }
      return VideoRotation.values.firstWhere((element) => element == result);
    } catch (e) {
      return null;
    }
  }

  set renderRotation(FutureOr<VideoRotation?> value) {
    sendInstanceSet("renderRotation", value);
  }
}

class MixedStreamAudioConfig extends NativeClass {
  static const _$namespace =
      r'com.ss.bytertc.engine.live.MixedStreamAudioConfig';
  static get codegen_$namespace => _$namespace;

  MixedStreamAudioConfig([NativeClassOptions? options])
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

  FutureOr<MixedStreamAudioCodecType?> get audioCodec async {
    try {
      final result =
          await sendInstanceGet<MixedStreamAudioCodecType?>("audioCodec");
      if (result == null) {
        return null;
      }
      return MixedStreamAudioCodecType.values
          .firstWhere((element) => element == result);
    } catch (e) {
      return null;
    }
  }

  set audioCodec(FutureOr<MixedStreamAudioCodecType?> value) {
    sendInstanceSet("audioCodec", value);
  }

  /// @detail api
  /// @brief 设置音频码率。建议设置。
  /// @param bitrate 音频码率，单位 Kbps。可取范围 [32, 192]，默认值为 64 Kbps。
  ///
  FutureOr<int?> get bitrate async {
    return await sendInstanceGet<int?>("bitrate");
  }

  set bitrate(FutureOr<int?> value) {
    sendInstanceSet("bitrate", value);
  }

  /// @detail api
  /// @brief 设置音频采样率。建议设置。
  /// @param sampleRate 音频采样率，单位 Hz。可取 32000 Hz、44100 Hz、48000 Hz，默认值为 48000 Hz。
  ///
  FutureOr<int?> get sampleRate async {
    return await sendInstanceGet<int?>("sampleRate");
  }

  set sampleRate(FutureOr<int?> value) {
    sendInstanceSet("sampleRate", value);
  }

  /// @detail api
  /// @brief 设置声道数。建议设置。
  /// @param channels 音频声道数。可取 1（单声道）、2（双声道），默认值为 2。
  ///
  FutureOr<int?> get channels async {
    return await sendInstanceGet<int?>("channels");
  }

  set channels(FutureOr<int?> value) {
    sendInstanceSet("channels", value);
  }

  /// @detail api
  /// @brief 设置 AAC 编码规格。建议设置。
  /// @param audioProfile AAC 规格，参看 MixedStreamAudioProfile{@link #MixedStreamAudioProfile}。默认值为 `MIXED_STREAM_AUDIO_PROFILE_LC("LC")`。
  /// @note
  ///        WTN 流任务不支持设置本参数。
  ///
  FutureOr<MixedStreamAudioProfile?> get audioProfile async {
    try {
      final result =
          await sendInstanceGet<MixedStreamAudioProfile?>("audioProfile");
      if (result == null) {
        return null;
      }
      return MixedStreamAudioProfile.values
          .firstWhere((element) => element == result);
    } catch (e) {
      return null;
    }
  }

  set audioProfile(FutureOr<MixedStreamAudioProfile?> value) {
    sendInstanceSet("audioProfile", value);
  }
}

enum VideoDenoiseMode {
  /// @brief 视频降噪关闭。
  ///
  VIDEO_DENOISE_MODE_OFF(0),

  /// @brief 视频降噪开启，由 ByteRTC 后台配置视频降噪算法。
  ///
  VIDEO_DENOISE_MODE_AUTO(1);

  final dynamic $value;
  const VideoDenoiseMode([this.$value]);
}

enum DataMessageSourceType {
  /// @brief 通过客户端或服务端的插入的自定义消息。
  ///
  DATA_MESSAGE_SOURCE_TYPE_DEFAULT(0),

  /// @brief 系统数据，包含音量指示信息。
  ///
  DATA_MESSAGE_SOURCE_TYPE_SYSTEM(1);

  final dynamic $value;
  const DataMessageSourceType([this.$value]);
}

class Orientation extends NativeClass {
  static const _$namespace = r'com.ss.bytertc.engine.data.Orientation';
  static get codegen_$namespace => _$namespace;

  Orientation([NativeClassOptions? options])
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
  ///
  FutureOr<float?> get x async {
    return await sendInstanceGet<float?>("x");
  }

  set x(FutureOr<float?> value) {
    sendInstanceSet("x", value);
  }

  /// @brief y 坐标
  ///
  FutureOr<float?> get y async {
    return await sendInstanceGet<float?>("y");
  }

  set y(FutureOr<float?> value) {
    sendInstanceSet("y", value);
  }

  /// @brief z 坐标
  ///
  FutureOr<float?> get z async {
    return await sendInstanceGet<float?>("z");
  }

  set z(FutureOr<float?> value) {
    sendInstanceSet("z", value);
  }
}

enum NetworkDetectionStopReason {
  /// @brief 用户主动停止
  ///
  USER(0),

  /// @brief 探测超过三分钟
  ///
  TIMEOUT(1),

  /// @brief 探测网络连接断开。 <br>
  ///        当超过 12s 没有收到回复，SDK 将断开网络连接，并且不再尝试重连。
  ///
  CONNECTION_LOST(2),

  /// @brief 本地开始推拉流，停止探测
  ///
  STREAMING(3),

  /// @brief 网络探测失败，内部异常
  ///
  INNER_ERR(4),

  /// @hidden constructor/destructor
  ///
  value(-1);

  final dynamic $value;
  const NetworkDetectionStopReason([this.$value]);
}

class RTCNativeLibraryLoader extends NativeClass {
  static const _$namespace =
      r'com.ss.bytertc.engine.loader.RTCNativeLibraryLoader';
  static get codegen_$namespace => _$namespace;

  RTCNativeLibraryLoader([NativeClassOptions? options])
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

  /// @detail callback
  /// @region 房间管理
  /// @brief SDK 在需要加载动态库时通过该方法回调
  /// @param libraryName 要加载的动态库名称
  ///

  FutureOr<boolean> load(String libraryName) async {
    return await nativeCall('load', [libraryName]);
  }
}

enum RoomStateChangeReason {
  /// @brief 首次进房成功。
  ///
  JOIN_ROOM(0),

  /// @brief 重新进房，比如断网重连。
  ///
  RECONNECT(1),

  /// @brief 离开房间。
  ///
  LEAVE_ROOM(2),

  /// @brief 进房失败。 <br>
  ///        初次进房或者由于网络状况不佳断网重连时，由于服务器错误导致进房失败。SDK 会自动重试进房。通过 onRoomStateChanged{@link #IRTCRoomEventHandler#onRoomStateChanged} 回调。
  ///
  JOIN_ROOM_FAILED(-2001),

  /// @brief Token 无效。 <br>
  ///        进房时使用的 Token 无效或过期失效。需要用户重新获取 Token，并调用 `updateToken` 方法更新 Token。通过 onRoomStateChanged{@link #IRTCRoomEventHandler#onRoomStateChanged} 回调。
  ///
  INVALID_TOKEN(-1000),

  /// @brief Token 过期。调用 `joinRoom` 使用新的 Token 重新加入房间。通过 onRoomStateChanged{@link #IRTCRoomEventHandler#onRoomStateChanged} 回调。
  ///
  TOKEN_EXPIRED(-1009),

  /// @brief 调用 `updateToken` 传入的 Token 无效。通过 onRoomStateChanged{@link #IRTCRoomEventHandler#onRoomStateChanged} 回调。
  ///
  UPDATE_TOKEN_WITH_INVALID_TOKEN(-1010),

  /// @brief 房间被封禁。通过 onRoomStateChanged{@link #IRTCRoomEventHandler#onRoomStateChanged} 回调。
  ///
  ROOM_FORBIDDEN(-1025),

  /// @brief 用户被封禁。通过 onRoomStateChanged{@link #IRTCRoomEventHandler#onRoomStateChanged} 回调。
  ///
  USER_FORBIDDEN(-1026),

  /// @brief 服务端调用 OpenAPI 将当前用户踢出房间。通过 onRoomStateChanged{@link #IRTCRoomEventHandler#onRoomStateChanged} 回调。
  ///
  KICKED_OUT(-1006),

  /// @brief 服务端调用 OpenAPI 解散房间，所有用户被移出房间。通过 onRoomStateChanged{@link #IRTCRoomEventHandler#onRoomStateChanged} 回调。
  ///
  ROOM_DISMISS(-1011),

  /// @brief 相同用户 ID 的用户加入本房间，当前用户被踢出房间。通过 onRoomStateChanged{@link #IRTCRoomEventHandler#onRoomStateChanged} 回调。
  ///
  DUPLICATE_LOGIN(-1004),

  /// @hidden internal use only
  /// @brief 加入房间错误。 <br>
  ///        调用 `joinRoom` 方法时, LICENSE 计费账号未使用 LICENSE_AUTHENTICATE SDK，加入房间错误。通过 onRoomStateChanged{@link #IRTCRoomEventHandler#onRoomStateChanged} 回调。
  ///
  WITHOUT_LICENSE_AUTHENTICATE_SDK(-1012),

  /// @hidden internal use only
  /// @brief 服务端 license 过期，拒绝进房。通过 onRoomStateChanged{@link #IRTCRoomEventHandler#onRoomStateChanged} 回调。
  ///
  SERVER_LICENSE_EXPIRED(-1017),

  /// @hidden internal use only
  /// @brief 超过服务端 license 许可的并发量上限，拒绝进房。通过 onRoomStateChanged{@link #IRTCRoomEventHandler#onRoomStateChanged} 回调。
  ///
  EXCEEDS_THE_UPPER_LIMIT(-1018),

  /// @hidden internal use only
  /// @brief license 参数错误，拒绝进房。通过 onRoomStateChanged{@link #IRTCRoomEventHandler#onRoomStateChanged} 回调。
  ///
  LICENSE_PARAMETER_ERROR(-1019),

  /// @hidden internal use only
  /// @brief license 证书路径错误。通过 onRoomStateChanged{@link #IRTCRoomEventHandler#onRoomStateChanged} 回调。
  ///
  LICENSE_FILE_PATH_ERROR(-1020),

  /// @hidden internal use only
  /// @brief license 证书不合法。通过 onRoomStateChanged{@link #IRTCRoomEventHandler#onRoomStateChanged} 回调。
  ///
  LICENSE_ILLEGAL(-1021),

  /// @hidden internal use only
  /// @brief license 证书已经过期，拒绝进房。通过 onRoomStateChanged{@link #IRTCRoomEventHandler#onRoomStateChanged} 回调。
  ///
  LICENSE_EXPIRED(-1022),

  /// @hidden internal use only
  /// @brief license 证书内容不匹配。通过 onRoomStateChanged{@link #IRTCRoomEventHandler#onRoomStateChanged} 回调。
  ///
  LICENSE_INFORMATION_NOT_MATCH(-1023),

  /// @hidden internal use only
  /// @brief license 当前证书与缓存证书不匹配。通过 onRoomStateChanged{@link #IRTCRoomEventHandler#onRoomStateChanged} 回调。
  ///
  LICENSE_NOT_MATCH_WITH_CACHE(-1024),

  /// @hidden internal use only
  /// @brief license 计费方法没有加载成功。可能是因为 license 相关插件未正确集成。通过 onRoomStateChanged{@link #IRTCRoomEventHandler#onRoomStateChanged} 回调。
  ///
  LICENSE_FUNCTION_NOT_FOUND(-1027),

  /// @brief 服务端异常状态导致退出房间。 <br>
  ///        SDK 与信令服务器断开，并不再自动重连，可联系技术支持。通过 onRoomStateChanged{@link #IRTCRoomEventHandler#onRoomStateChanged} 回调。
  ///
  STATE_ABNORMAL_SERVER_STATUS(-1084),

  /// @brief 加入房间错误。 <br>
  /// 进房时发生未知错误导致加入房间失败。需要用户重新加入房间。通过 onRoomStateChanged{@link #IRTCRoomEventHandler#onRoomStateChanged} 回调。
  ///
  UNKNOWN(-1001);

  final dynamic $value;
  const RoomStateChangeReason([this.$value]);
}

enum MixedStreamRenderMode {
  /// @brief 视窗填满优先，默认值。 <br>
  ///        视频尺寸等比缩放，直至视窗被填满。当视频尺寸与显示窗口尺寸不一致时，多出的视频将被截掉。
  ///
  MIXED_STREAM_RENDER_MODE_HIDDEN(1),

  /// @brief 视频帧内容全部显示优先。 <br>
  ///        视频尺寸等比缩放，优先保证视频内容全部显示。当视频尺寸与显示窗口尺寸不一致时，会把窗口未被填满的区域填充成背景颜色。
  ///
  MIXED_STREAM_RENDER_MODE_FIT(2),

  /// @brief 视频帧自适应画布。 <br>
  ///        视频尺寸非等比例缩放，把窗口充满。在此过程中，视频帧的长宽比例可能会发生变化。
  ///
  MIXED_STREAM_RENDER_MODE_ADAPTIVE(3);

  final dynamic $value;
  const MixedStreamRenderMode([this.$value]);
}

class AudioFormat extends NativeClass {
  static const _$namespace = r'com.ss.bytertc.engine.data.AudioFormat';
  static get codegen_$namespace => _$namespace;

  AudioFormat([NativeClassOptions? options])
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

  /// @brief 音频采样率，参看 AudioSampleRate{@link #AudioSampleRate}。
  ///
  FutureOr<AudioSampleRate?> get sampleRate async {
    try {
      final result = await sendInstanceGet<AudioSampleRate?>("sampleRate");
      if (result == null) {
        return null;
      }
      return AudioSampleRate.values.firstWhere((element) => element == result);
    } catch (e) {
      return null;
    }
  }

  set sampleRate(FutureOr<AudioSampleRate?> value) {
    sendInstanceSet("sampleRate", value);
  }

  /// @brief 音频声道，参看 AudioChannel{@link #AudioChannel}。
  ///
  FutureOr<AudioChannel?> get channel async {
    try {
      final result = await sendInstanceGet<AudioChannel?>("channel");
      if (result == null) {
        return null;
      }
      return AudioChannel.values.firstWhere((element) => element == result);
    } catch (e) {
      return null;
    }
  }

  set channel(FutureOr<AudioChannel?> value) {
    sendInstanceSet("channel", value);
  }

  /// @brief 单次回调的音频帧中包含的采样点数。默认值为 `0`，此时，采样点数取最小值。 <br>
  ///        最小值为回调间隔是 0.01s 时的值，即 `sampleRate * channel * 0.01s`。 <br>
  ///        最大值是 `2048`。超出取值范围时，采样点数取默认值。 <br>
  ///        该参数仅在设置读写回调时生效，调用 enableAudioFrameCallback{@link #RTCEngine#enableAudioFrameCallback} 开启只读模式回调时设置该参数不生效。
  ///
  FutureOr<int?> get samplesPerCall async {
    return await sendInstanceGet<int?>("samplesPerCall");
  }

  set samplesPerCall(FutureOr<int?> value) {
    sendInstanceSet("samplesPerCall", value);
  }
}

enum LocalLogLevel {
  /// @brief 信息级别。
  ///
  INFO(0),

  /// @brief （默认值）警告级别。
  ///
  WARNING(1),

  /// @brief 错误级别。
  ///
  ERROR(2),

  /// @brief 关闭日志。
  ///
  NONE(3);

  final dynamic $value;
  const LocalLogLevel([this.$value]);
}

class AudioRecordingConfig extends NativeClass {
  static const _$namespace = r'com.ss.bytertc.engine.data.AudioRecordingConfig';
  static get codegen_$namespace => _$namespace;

  AudioRecordingConfig([NativeClassOptions? options])
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

  /// @brief 录制文件路径。一个有读写权限的绝对路径，包含文件名和文件后缀。 <br>
  /// 录制文件的格式仅支持 .aac 和 .wav。
  ///
  FutureOr<String?> get absoluteFileName async {
    return await sendInstanceGet<String?>("absoluteFileName");
  }

  set absoluteFileName(FutureOr<String?> value) {
    sendInstanceSet("absoluteFileName", value);
  }

  /// @brief 录音采样率。参看 AudioSampleRate{@link #AudioSampleRate}。
  ///
  FutureOr<AudioSampleRate?> get sampleRate async {
    try {
      final result = await sendInstanceGet<AudioSampleRate?>("sampleRate");
      if (result == null) {
        return null;
      }
      return AudioSampleRate.values.firstWhere((element) => element == result);
    } catch (e) {
      return null;
    }
  }

  set sampleRate(FutureOr<AudioSampleRate?> value) {
    sendInstanceSet("sampleRate", value);
  }

  /// @brief 录音音频声道。参看 AudioChannel{@link #AudioChannel}。 <br>
  ///       如果录制时设置的的音频声道数与采集时的音频声道数不同： <br>
  ///        - 如果采集的声道数为 1，录制的声道数为 2，那么，录制的音频为经过单声道数据拷贝后的双声道数据，而不是立体声。
  ///        - 如果采集的声道数为 2，录制的声道数为 1，那么，录制的音频为经过双声道数据混合后的单声道数据。
  ///
  FutureOr<AudioChannel?> get channel async {
    try {
      final result = await sendInstanceGet<AudioChannel?>("channel");
      if (result == null) {
        return null;
      }
      return AudioChannel.values.firstWhere((element) => element == result);
    } catch (e) {
      return null;
    }
  }

  set channel(FutureOr<AudioChannel?> value) {
    sendInstanceSet("channel", value);
  }

  /// @brief 录音内容来源，参看 AudioFrameSource{@link #AudioFrameSource}。 <br>
  /// 默认为 AUDIO_FRAME_SOURCE_MIXED(2)。
  ///
  FutureOr<AudioFrameSource?> get frameSource async {
    try {
      final result = await sendInstanceGet<AudioFrameSource?>("frameSource");
      if (result == null) {
        return null;
      }
      return AudioFrameSource.values.firstWhere((element) => element == result);
    } catch (e) {
      return null;
    }
  }

  set frameSource(FutureOr<AudioFrameSource?> value) {
    sendInstanceSet("frameSource", value);
  }

  /// @brief 录音音质。仅在录制文件格式为 .aac 时可以设置。参看 AudioQuality{@link #AudioQuality}。 <br>
  /// 采样率为 32kHz 时，不同音质录制文件（时长为 10min）的大小分别是： <br>
  ///        - 低音质：1.2MB；
  ///        - 【默认】中音质：2MB；
  ///        - 高音质：3.75MB；
  ///        - 超高音质：7.5MB。
  ///
  FutureOr<AudioQuality?> get quality async {
    try {
      final result = await sendInstanceGet<AudioQuality?>("quality");
      if (result == null) {
        return null;
      }
      return AudioQuality.values.firstWhere((element) => element == result);
    } catch (e) {
      return null;
    }
  }

  set quality(FutureOr<AudioQuality?> value) {
    sendInstanceSet("quality", value);
  }
}

enum RemoteMirrorType {
  /// @brief （默认值）远端视频渲染无镜像效果。
  ///
  NONE(0),

  /// @brief 远端视频渲染有镜像效果。
  ///
  RENDER(1);

  final dynamic $value;
  const RemoteMirrorType([this.$value]);
}

class VideoFrameData extends NativeClass {
  static const _$namespace = r'com.ss.bytertc.engine.data.VideoFrameData';
  static get codegen_$namespace => _$namespace;

  VideoFrameData([NativeClassOptions? options])
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

  /// @brief 视频帧缓冲区类型，参看 VideoBufferType{@link #VideoBufferType}。必填。
  ///
  FutureOr<VideoBufferType?> get bufferType async {
    try {
      final result = await sendInstanceGet<VideoBufferType?>("bufferType");
      if (result == null) {
        return null;
      }
      return VideoBufferType.values.firstWhere((element) => element == result);
    } catch (e) {
      return null;
    }
  }

  set bufferType(FutureOr<VideoBufferType?> value) {
    sendInstanceSet("bufferType", value);
  }

  /// @brief 视频帧像素格式，参看 VideoPixelFormat{@link #VideoPixelFormat}。当 `bufferType` 为 `GL_TEXTURE` 时，必填。
  ///
  FutureOr<VideoPixelFormat?> get pixelFormat async {
    try {
      final result = await sendInstanceGet<VideoPixelFormat?>("pixelFormat");
      if (result == null) {
        return null;
      }
      return VideoPixelFormat.values.firstWhere((element) => element == result);
    } catch (e) {
      return null;
    }
  }

  set pixelFormat(FutureOr<VideoPixelFormat?> value) {
    sendInstanceSet("pixelFormat", value);
  }

  /// @brief 视频帧内容类型，参看 VideoContentType{@link #VideoContentType}。
  ///
  FutureOr<VideoContentType?> get contentType async {
    try {
      final result = await sendInstanceGet<VideoContentType?>("contentType");
      if (result == null) {
        return null;
      }
      return VideoContentType.values.firstWhere((element) => element == result);
    } catch (e) {
      return null;
    }
  }

  set contentType(FutureOr<VideoContentType?> value) {
    sendInstanceSet("contentType", value);
  }

  /// @brief 视频帧时间戳，单位：微秒。必填。
  ///
  FutureOr<long?> get timestampUs async {
    return await sendInstanceGet<long?>("timestampUs");
  }

  set timestampUs(FutureOr<long?> value) {
    sendInstanceSet("timestampUs", value);
  }

  /// @brief 视频帧宽度。必填。
  ///
  FutureOr<int?> get width async {
    return await sendInstanceGet<int?>("width");
  }

  set width(FutureOr<int?> value) {
    sendInstanceSet("width", value);
  }

  /// @brief 视频帧高度。必填。
  ///
  FutureOr<int?> get height async {
    return await sendInstanceGet<int?>("height");
  }

  set height(FutureOr<int?> value) {
    sendInstanceSet("height", value);
  }

  /// @brief 视频帧旋转角度
  ///
  FutureOr<VideoRotation?> get rotation async {
    try {
      final result = await sendInstanceGet<VideoRotation?>("rotation");
      if (result == null) {
        return null;
      }
      return VideoRotation.values.firstWhere((element) => element == result);
    } catch (e) {
      return null;
    }
  }

  set rotation(FutureOr<VideoRotation?> value) {
    sendInstanceSet("rotation", value);
  }

  /// @brief 视频帧平面数量。当 `bufferType` 为 `RAW_MEMORY` 时，必填。
  ///
  FutureOr<int?> get numberOfPlanes async {
    return await sendInstanceGet<int?>("numberOfPlanes");
  }

  set numberOfPlanes(FutureOr<int?> value) {
    sendInstanceSet("numberOfPlanes", value);
  }

  /// @brief 视频帧平面数组。当 `bufferType` 为 `RAW_MEMORY` 时，必填。
  ///
  FutureOr<Array<ByteBuffer>?> get planeData async {
    return await sendInstanceGet<Array<ByteBuffer>?>("planeData");
  }

  set planeData(FutureOr<Array<ByteBuffer>?> value) {
    sendInstanceSet("planeData", value);
  }

  /// @brief stride 数组。stride 指视频帧平面相邻两行图像数据之间的内存长度（单位字节）。当 `bufferType` 为 `RAW_MEMORY` 时，必填。
  ///
  FutureOr<Array<int>?> get planeStride async {
    return await sendInstanceGet<Array<int>?>("planeStride");
  }

  set planeStride(FutureOr<Array<int>?> value) {
    sendInstanceSet("planeStride", value);
  }

  /// @brief SEI 数据
  ///
  FutureOr<ByteArray?> get seiData async {
    return await sendInstanceGet<ByteArray?>("seiData");
  }

  set seiData(FutureOr<ByteArray?> value) {
    sendInstanceSet("seiData", value);
  }

  /// @brief 视频帧感兴趣区域数据
  ///
  FutureOr<ByteArray?> get roiData async {
    return await sendInstanceGet<ByteArray?>("roiData");
  }

  set roiData(FutureOr<ByteArray?> value) {
    sendInstanceSet("roiData", value);
  }

  /// @brief 纹理 ID。当 `bufferType` 为 `RAW_MEMORY` 或 `GL_TEXTURE` 时，必填。
  ///
  FutureOr<int?> get textureId async {
    return await sendInstanceGet<int?>("textureId");
  }

  set textureId(FutureOr<int?> value) {
    sendInstanceSet("textureId", value);
  }

  /// @brief 纹理矩阵
  ///
  FutureOr<Array<float>?> get textureMatrix async {
    return await sendInstanceGet<Array<float>?>("textureMatrix");
  }

  set textureMatrix(FutureOr<Array<float>?> value) {
    sendInstanceSet("textureMatrix", value);
  }

  /// @brief EGLContext
  ///
  FutureOr<EGLContext?> get eglContext async {
    return await sendInstanceGet<EGLContext?>("eglContext");
  }

  set eglContext(FutureOr<EGLContext?> value) {
    sendInstanceSet("eglContext", value);
  }
}

enum ConnectionState {
  /// @brief 连接断开超过 12s，此时 SDK 会尝试自动重连。
  ///
  CONNECTION_STATE_DISCONNECTED(1),

  /// @brief 首次请求建立连接，正在连接中。
  ///
  CONNECTION_STATE_CONNECTING(2),

  /// @brief 首次连接成功。
  ///
  CONNECTION_STATE_CONNECTED(3),

  /// @brief 涵盖了以下情况： <br>
  ///        - 首次连接时，10 秒内未连接成功;
  ///        - 连接成功后，断连 10 秒。自动重连中。
  ///
  CONNECTION_STATE_RECONNECTING(4),

  /// @brief 连接断开后，重连成功。
  ///
  CONNECTION_STATE_RECONNECTED(5),

  /// @brief 处于 `CONNECTION_STATE_DISCONNECTED` 状态超过 10 秒，且期间重连未成功。SDK 仍将继续尝试重连。
  ///
  CONNECTION_STATE_LOST(6),

  /// @brief 连接失败，服务端状态异常。SDK 不会自动重连，请重新进房，或联系技术支持。
  ///
  CONNECTION_STATE_FAILED(7);

  final dynamic $value;
  const ConnectionState([this.$value]);
}

enum MediaPlayerCustomSourceMode {
  /// @brief 当播放来自本地的 PCM 数据时，使用此选项。
  ///
  PUSH(0),

  /// @brief 当播放来自内存的音频数据时，使用此选项。
  ///
  PULL(1);

  final dynamic $value;
  const MediaPlayerCustomSourceMode([this.$value]);
}

class SubscribeConfig extends NativeClass {
  static const _$namespace = r'com.ss.bytertc.engine.SubscribeConfig';
  static get codegen_$namespace => _$namespace;

  SubscribeConfig([NativeClassOptions? options])
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

  /// @brief 是否是屏幕流
  ///
  FutureOr<boolean?> get isScreen async {
    return await sendInstanceGet<boolean?>("isScreen");
  }

  set isScreen(FutureOr<boolean?> value) {
    sendInstanceSet("isScreen", value);
  }

  /// @brief 是否订阅视频
  ///
  FutureOr<boolean?> get subVideo async {
    return await sendInstanceGet<boolean?>("subVideo");
  }

  set subVideo(FutureOr<boolean?> value) {
    sendInstanceSet("subVideo", value);
  }

  /// @brief 是否订阅音频
  ///
  FutureOr<boolean?> get subAudio async {
    return await sendInstanceGet<boolean?>("subAudio");
  }

  set subAudio(FutureOr<boolean?> value) {
    sendInstanceSet("subAudio", value);
  }

  /// @brief 订阅的视频流分辨率下标。 <br>
  ///        用户可以在一路流中发布多个不同分辨率的视频。因此订阅流时，需要指定订阅的具体分辨率。此参数即用于指定需订阅的分辨率的下标，默认值为 0 。
  ///
  FutureOr<int?> get videoIndex async {
    return await sendInstanceGet<int?>("videoIndex");
  }

  set videoIndex(FutureOr<int?> value) {
    sendInstanceSet("videoIndex", value);
  }

  /// @brief 视频宽度，单位：px
  ///
  FutureOr<int?> get subWidth async {
    return await sendInstanceGet<int?>("subWidth");
  }

  set subWidth(FutureOr<int?> value) {
    sendInstanceSet("subWidth", value);
  }

  /// @brief 视频高度，单位：px
  ///
  FutureOr<int?> get subHeight async {
    return await sendInstanceGet<int?>("subHeight");
  }

  set subHeight(FutureOr<int?> value) {
    sendInstanceSet("subHeight", value);
  }

  /// @hidden for internal use only
  ///
  FutureOr<int?> get subVideoIndex async {
    return await sendInstanceGet<int?>("subVideoIndex");
  }

  set subVideoIndex(FutureOr<int?> value) {
    sendInstanceSet("subVideoIndex", value);
  }

  /// @brief 期望订阅的最高帧率，单位：fps，默认值为 0，设为大于 0 的值时开始生效。 <br>
  ///        如果发布端发布帧率 > 订阅端订阅的帧率，下行媒体服务器 SVC 丢帧，订阅端收到通过此接口设置的帧率；如果发布端发布帧率 < 订阅端订阅的帧率，则订阅端只能收到发布的帧率。<br>
  ///        仅码流支持 SVC 分级编码特性时方可生效。
  ///
  FutureOr<int?> get framerate async {
    return await sendInstanceGet<int?>("framerate");
  }

  set framerate(FutureOr<int?> value) {
    sendInstanceSet("framerate", value);
  }

  /// @hidden constructor/destructor
  ///

  FutureOr<boolean> equals(Object o) async {
    return await nativeCall('equals', [o]);
  }
}

class RemoteStreamKey extends NativeClass {
  static const _$namespace = r'com.ss.bytertc.engine.data.RemoteStreamKey';
  static get codegen_$namespace => _$namespace;

  RemoteStreamKey([NativeClassOptions? options])
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

  /// @brief 房间 ID。
  ///
  FutureOr<String?> get roomId async {
    return await sendInstanceGet<String?>("roomId");
  }

  set roomId(FutureOr<String?> value) {
    sendInstanceSet("roomId", value);
  }

  /// @brief 用户 ID。
  ///
  FutureOr<String?> get userId async {
    return await sendInstanceGet<String?>("userId");
  }

  set userId(FutureOr<String?> value) {
    sendInstanceSet("userId", value);
  }

  /// @brief 流属性，包括主流、屏幕流。参看 [StreamIndex](70083#StreamIndex-2)
  ///
  FutureOr<StreamIndex?> get streamIndex async {
    try {
      final result = await sendInstanceGet<StreamIndex?>("streamIndex");
      if (result == null) {
        return null;
      }
      return StreamIndex.values.firstWhere((element) => element == result);
    } catch (e) {
      return null;
    }
  }

  set streamIndex(FutureOr<StreamIndex?> value) {
    sendInstanceSet("streamIndex", value);
  }

  /// @brief 获取房间 ID
  ///

  FutureOr<String> getRoomId() async {
    return await nativeCall('getRoomId', []);
  }

  /// @brief 获取用户 ID
  ///

  FutureOr<String> getUserId() async {
    return await nativeCall('getUserId', []);
  }

  /// @brief 获取流属性，包括主流、屏幕流。参看 [StreamIndex](#streamindex-2)
  ///

  FutureOr<StreamIndex> getStreamIndex() async {
    return await nativeCall('getStreamIndex', []);
  }

  /// @brief 检查当前类中是否有为空的字段
  ///

  FutureOr<boolean> hasNullProperty() async {
    return await nativeCall('hasNullProperty', []);
  }
}

enum RemoteVideoSinkPosition {
  /// @hidden not available
  /// @brief 解码后。
  ///
  AFTER_DECODER(0),

  /// @brief （默认值）后处理后。
  ///
  AFTER_POST_PROCESS(1);

  final dynamic $value;
  const RemoteVideoSinkPosition([this.$value]);
}

enum StreamSubscribeState {
  /// @brief 订阅/取消订阅流成功
  ///
  SUCCESS(0),

  /// @brief 订阅/取消订阅流失败，本地用户未在房间中
  ///
  FAILED_NOT_IN_ROOM(1),

  /// @brief 订阅/取消订阅流失败，房间内未找到指定的音视频流
  ///
  FAILED_STREAM_NOT_FOUND(2),

  FAILED_AUTO_MODE(3),

  /// @brief 订阅/取消订阅流失败，信令错误，请重试
  ///
  FAILED_SIGNAL(4);

  final dynamic $value;
  const StreamSubscribeState([this.$value]);
}

class ByteWatermark extends NativeClass {
  static const _$namespace = r'com.ss.bytertc.engine.video.ByteWatermark';
  static get codegen_$namespace => _$namespace;

  ByteWatermark([NativeClassOptions? options])
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
  ///
  FutureOr<float?> get x async {
    return await sendInstanceGet<float?>("x");
  }

  set x(FutureOr<float?> value) {
    sendInstanceSet("x", value);
  }

  /// @brief 水印图片相对视频流左上角的纵向偏移与视频流高度的比值，取值范围为 [0,1)。
  ///
  FutureOr<float?> get y async {
    return await sendInstanceGet<float?>("y");
  }

  set y(FutureOr<float?> value) {
    sendInstanceSet("y", value);
  }

  /// @brief 水印图片宽度与视频流宽度的比值，取值范围 [0,1)。
  ///
  FutureOr<float?> get width async {
    return await sendInstanceGet<float?>("width");
  }

  set width(FutureOr<float?> value) {
    sendInstanceSet("width", value);
  }

  /// @brief 水印图片高度与视频流高度的比值，取值范围为 [0,1)。
  ///
  FutureOr<float?> get height async {
    return await sendInstanceGet<float?>("height");
  }

  set height(FutureOr<float?> value) {
    sendInstanceSet("height", value);
  }
}

class CloudProxyInfo extends NativeClass {
  static const _$namespace = r'com.ss.bytertc.engine.data.CloudProxyInfo';
  static get codegen_$namespace => _$namespace;

  CloudProxyInfo([NativeClassOptions? options])
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
  ///
  FutureOr<String?> get cloudProxyIp async {
    return await sendInstanceGet<String?>("cloudProxyIp");
  }

  set cloudProxyIp(FutureOr<String?> value) {
    sendInstanceSet("cloudProxyIp", value);
  }

  /// @detail keytype
  /// @brief 云代理服务器端口
  ///
  FutureOr<int?> get cloudProxyPort async {
    return await sendInstanceGet<int?>("cloudProxyPort");
  }

  set cloudProxyPort(FutureOr<int?> value) {
    sendInstanceSet("cloudProxyPort", value);
  }
}

enum VideoContentType {
  /// @brief 普通视频
  ///
  NORMAL_FRAME(0),

  /// @brief 黑帧视频
  ///
  BLACK_FRAME(1);

  final dynamic $value;
  const VideoContentType([this.$value]);
}

enum MixedStreamSEIContentMode {
  /// @brief 视频流中包含全部的 SEI 信息。默认设置。
  ///
  MIXED_STREAM_SEI_CONTENT_MODE_DEFAULT(0),

  /// @brief 随非关键帧传输的 SEI 数据中仅包含音量信息。 <br>
  ///        当设置 enableVolumeIndication{@link #MixedStreamControlConfig#enableVolumeIndication} 为 true 时，此参数设置生效。
  ///
  MIXED_STREAM_SEI_CONTENT_MODE_ENABLE_VOLUME_INDICATION(1);

  final dynamic $value;
  const MixedStreamSEIContentMode([this.$value]);
}

class RecordingProgress extends NativeClass {
  static const _$namespace = r'com.ss.bytertc.engine.data.RecordingProgress';
  static get codegen_$namespace => _$namespace;

  RecordingProgress([NativeClassOptions? options])
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
  ///
  FutureOr<long?> get duration async {
    return await sendInstanceGet<long?>("duration");
  }

  set duration(FutureOr<long?> value) {
    sendInstanceSet("duration", value);
  }

  /// @brief 当前录制文件的大小，单位：byte
  ///
  FutureOr<long?> get fileSize async {
    return await sendInstanceGet<long?>("fileSize");
  }

  set fileSize(FutureOr<long?> value) {
    sendInstanceSet("fileSize", value);
  }
}

enum PlayerState {
  /// @brief 播放未启动
  ///
  IDLE(0),

  /// @brief 已加载
  ///
  PRELOADED(1),

  /// @brief 已打开
  ///
  OPENED(2),

  /// @brief 正在播放
  ///
  PLAYING(3),

  /// @brief 播放已暂停
  ///
  PAUSED(4),

  /// @brief 播放已被主动停止
  ///
  STOPPED(5),

  /// @brief 播放失败
  ///
  FAILED(6),

  /// @brief 播放自然结束
  ///
  FINISHED(7),

  /// @brief 循环播放已结束
  ///
  LOOP_FINISHED(8);

  final dynamic $value;
  const PlayerState([this.$value]);
}

class LocalProxyConfiguration extends NativeClass {
  static const _$namespace =
      r'com.ss.bytertc.engine.type.LocalProxyConfiguration';
  static get codegen_$namespace => _$namespace;

  LocalProxyConfiguration([NativeClassOptions? options])
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

  /// @brief 本地代理类型，参看 [LocalProxyType](70083#localproxytype-2)。
  ///
  FutureOr<LocalProxyType?> get localProxyType async {
    try {
      final result = await sendInstanceGet<LocalProxyType?>("localProxyType");
      if (result == null) {
        return null;
      }
      return LocalProxyType.values.firstWhere((element) => element == result);
    } catch (e) {
      return null;
    }
  }

  set localProxyType(FutureOr<LocalProxyType?> value) {
    sendInstanceSet("localProxyType", value);
  }

  /// @brief 本地代理服务器 IP。
  ///
  FutureOr<String?> get localProxyIp async {
    return await sendInstanceGet<String?>("localProxyIp");
  }

  set localProxyIp(FutureOr<String?> value) {
    sendInstanceSet("localProxyIp", value);
  }

  /// @brief 本地代理服务器端口。
  ///
  FutureOr<int?> get localProxyPort async {
    return await sendInstanceGet<int?>("localProxyPort");
  }

  set localProxyPort(FutureOr<int?> value) {
    sendInstanceSet("localProxyPort", value);
  }

  /// @brief 本地代理用户名。
  ///
  FutureOr<String?> get localProxyUsername async {
    return await sendInstanceGet<String?>("localProxyUsername");
  }

  set localProxyUsername(FutureOr<String?> value) {
    sendInstanceSet("localProxyUsername", value);
  }

  /// @brief 本地代理密码。
  ///
  FutureOr<String?> get localProxyPassword async {
    return await sendInstanceGet<String?>("localProxyPassword");
  }

  set localProxyPassword(FutureOr<String?> value) {
    sendInstanceSet("localProxyPassword", value);
  }
}

enum GameRoomType {
  /// @brief 小队房间
  ///
  TEAM(0),

  /// @brief 世界房间
  ///
  WORLD(1);

  final dynamic $value;
  const GameRoomType([this.$value]);
}

class NetworkQualityStats extends NativeClass {
  static const _$namespace = r'com.ss.bytertc.engine.type.NetworkQualityStats';
  static get codegen_$namespace => _$namespace;

  NetworkQualityStats([NativeClassOptions? options])
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

  /// @brief 用户 id
  ///
  FutureOr<String?> get uid async {
    return await sendInstanceGet<String?>("uid");
  }

  set uid(FutureOr<String?> value) {
    sendInstanceSet("uid", value);
  }

  /// @brief 本端的上行/下行的丢包率，范围 [0.0,1.0] <br>
  ///        当 `uid` 为本地用户时，代表发布流的上行丢包率。 <br>
  ///        当 `uid` 为远端用户时，代表接收所有订阅流的下行丢包率。
  ///
  FutureOr<double?> get fractionLost async {
    return await sendInstanceGet<double?>("fractionLost");
  }

  set fractionLost(FutureOr<double?> value) {
    sendInstanceSet("fractionLost", value);
  }

  /// @brief 当 `uid` 为本地用户时有效，客户端到服务端的往返延时（RTT），单位：ms
  ///
  FutureOr<int?> get rtt async {
    return await sendInstanceGet<int?>("rtt");
  }

  set rtt(FutureOr<int?> value) {
    sendInstanceSet("rtt", value);
  }

  /// @brief 本端的音视频 RTP 包 2 秒内的平均传输速率，单位：bps <br>
  ///        当 `uid` 为本地用户时，代表发送速率。 <br>
  ///        当 `uid` 为远端用户时，代表所有订阅流的接收速率。
  ///
  FutureOr<int?> get totalBandwidth async {
    return await sendInstanceGet<int?>("totalBandwidth");
  }

  set totalBandwidth(FutureOr<int?> value) {
    sendInstanceSet("totalBandwidth", value);
  }

  /// @brief 上行网络质量分。分数越高网络质量越差，详见 NetworkQuality{@link #NetworkQuality}。
  ///
  FutureOr<int?> get txQuality async {
    return await sendInstanceGet<int?>("txQuality");
  }

  set txQuality(FutureOr<int?> value) {
    sendInstanceSet("txQuality", value);
  }

  /// @brief 下行网络质量分。分数越高网络质量越差，详见 NetworkQuality{@link #NetworkQuality}。
  ///
  FutureOr<int?> get rxQuality async {
    return await sendInstanceGet<int?>("rxQuality");
  }

  set rxQuality(FutureOr<int?> value) {
    sendInstanceSet("rxQuality", value);
  }
}

class AudioEffectPlayerConfig extends NativeClass {
  static const _$namespace =
      r'com.ss.bytertc.engine.data.AudioEffectPlayerConfig';
  static get codegen_$namespace => _$namespace;

  AudioEffectPlayerConfig([NativeClassOptions? options])
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

  /// @brief 混音播放类型，详见 AudioMixingType{@link #AudioMixingType}
  ///
  FutureOr<AudioMixingType?> get type async {
    try {
      final result = await sendInstanceGet<AudioMixingType?>("type");
      if (result == null) {
        return null;
      }
      return AudioMixingType.values.firstWhere((element) => element == result);
    } catch (e) {
      return null;
    }
  }

  set type(FutureOr<AudioMixingType?> value) {
    sendInstanceSet("type", value);
  }

  /// @brief 混音播放次数 <br>
  ///       - play_count <= 0: 无限循环
  ///       - play_count == 1: 播放一次（默认）
  ///       - play_count > 1: 播放 play_count 次
  ///
  FutureOr<int?> get playCount async {
    return await sendInstanceGet<int?>("playCount");
  }

  set playCount(FutureOr<int?> value) {
    sendInstanceSet("playCount", value);
  }

  /// @brief 混音起始位置。默认值为 0，单位为毫秒。
  ///
  FutureOr<int?> get startPos async {
    return await sendInstanceGet<int?>("startPos");
  }

  set startPos(FutureOr<int?> value) {
    sendInstanceSet("startPos", value);
  }

  /// @brief 与音乐文件原始音调相比的升高/降低值，取值范围为 `[-12，12]`，默认值为 0。每相邻两个值的音高距离相差半音，正值表示升调，负值表示降调。
  ///
  FutureOr<int?> get pitch async {
    return await sendInstanceGet<int?>("pitch");
  }

  set pitch(FutureOr<int?> value) {
    sendInstanceSet("pitch", value);
  }
}

enum MediaDeviceState {
  /// @brief 设备开启采集
  ///
  MEDIA_DEVICE_STATE_STARTED(1),

  /// @brief 设备停止采集
  ///
  MEDIA_DEVICE_STATE_STOPPED(2),

  /// @brief 设备运行时错误 <br>
  ///        例如，当媒体设备的预期行为是正常采集，但没有收到采集数据时，将回调该状态。
  ///
  MEDIA_DEVICE_STATE_RUNTIMEERROR(3),

  /// @brief 设备已插入
  ///
  MEDIA_DEVICE_STATE_ADDED(10),

  /// @brief 设备被移除
  ///
  MEDIA_DEVICE_STATE_REMOVED(11),

  /// @brief 系统通话打断了音视频通话。将在通话结束后自动恢复。
  ///
  MEDIA_DEVICE_STATE_INTERRUPTION_BEGAN(12),

  /// @brief 音视频通话已从系统电话中恢复
  ///
  MEDIA_DEVICE_STATE_INTERRUPTION_ENDED(13);

  final dynamic $value;
  const MediaDeviceState([this.$value]);
}

enum VideoCodecType {
  /// @brief 其他
  ///
  CODEC_TYPE_AUTO(0),

  /// @brief H264
  ///
  CODEC_TYPE_H264(1),

  /// @brief ByteVC1
  ///
  CODEC_TYPE_BYTEVC1(2);

  final dynamic $value;
  const VideoCodecType([this.$value]);
}

class LocalAudioStats extends NativeClass {
  static const _$namespace = r'com.ss.bytertc.engine.type.LocalAudioStats';
  static get codegen_$namespace => _$namespace;

  LocalAudioStats([NativeClassOptions? options])
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
  ///
  FutureOr<float?> get audioLossRate async {
    return await sendInstanceGet<float?>("audioLossRate");
  }

  set audioLossRate(FutureOr<float?> value) {
    sendInstanceSet("audioLossRate", value);
  }

  /// @brief 发送码率。此次统计周期内的音频发送码率，单位为 kbps 。
  ///
  FutureOr<float?> get sendKBitrate async {
    return await sendInstanceGet<float?>("sendKBitrate");
  }

  set sendKBitrate(FutureOr<float?> value) {
    sendInstanceSet("sendKBitrate", value);
  }

  /// @brief 采集采样率。此次统计周期内的音频采集采样率信息，单位为 Hz 。
  ///
  FutureOr<int?> get recordSampleRate async {
    return await sendInstanceGet<int?>("recordSampleRate");
  }

  set recordSampleRate(FutureOr<int?> value) {
    sendInstanceSet("recordSampleRate", value);
  }

  /// @brief 统计间隔。此次统计周期的间隔，单位为 ms 。 <br>
  ///        此字段用于设置回调的统计周期，默认设置为 2s 。
  ///
  FutureOr<int?> get statsInterval async {
    return await sendInstanceGet<int?>("statsInterval");
  }

  set statsInterval(FutureOr<int?> value) {
    sendInstanceSet("statsInterval", value);
  }

  /// @brief 往返时延。单位为 ms 。
  ///
  FutureOr<int?> get rtt async {
    return await sendInstanceGet<int?>("rtt");
  }

  set rtt(FutureOr<int?> value) {
    sendInstanceSet("rtt", value);
  }

  /// @brief 音频声道数。
  ///
  FutureOr<int?> get numChannels async {
    return await sendInstanceGet<int?>("numChannels");
  }

  set numChannels(FutureOr<int?> value) {
    sendInstanceSet("numChannels", value);
  }

  /// @brief 音频发送采样率。此次统计周期内的音频发送采样率信息，单位为 Hz 。
  ///
  FutureOr<int?> get sentSampleRate async {
    return await sendInstanceGet<int?>("sentSampleRate");
  }

  set sentSampleRate(FutureOr<int?> value) {
    sendInstanceSet("sentSampleRate", value);
  }

  /// @brief 音频上行网络抖动，单位为 ms 。
  ///
  FutureOr<int?> get jitter async {
    return await sendInstanceGet<int?>("jitter");
  }

  set jitter(FutureOr<int?> value) {
    sendInstanceSet("jitter", value);
  }

  /// @brief 音频采集播放延时，单位为 ms 。
  ///
  FutureOr<int?> get audioDeviceLoopDelay async {
    return await sendInstanceGet<int?>("audioDeviceLoopDelay");
  }

  set audioDeviceLoopDelay(FutureOr<int?> value) {
    sendInstanceSet("audioDeviceLoopDelay", value);
  }

  ///
  /// @brief 音频编码帧率。
  ///
  FutureOr<double?> get encodeFrameRate async {
    return await sendInstanceGet<double?>("encodeFrameRate");
  }

  set encodeFrameRate(FutureOr<double?> value) {
    sendInstanceSet("encodeFrameRate", value);
  }
}

enum ForwardStreamError {
  /// @brief 正常
  ///
  FORWARD_STREAM_ERROR_OK(0),

  /// @brief 参数异常
  ///
  FORWARD_STREAM_ERROR_INVALID_ARGUMENT(1201),

  /// @brief token 错误
  ///
  FORWARD_STREAM_ERROR_INVALID_TOKEN(1202),

  /// @brief 服务端异常
  ///
  FORWARD_STREAM_ERROR_RESPONSE(1203),

  /// @brief 目标房间有相同 user id 的用户加入，转发中断
  ///
  FORWARD_STREAM_ERROR_REMOTE_KICKED(1204),

  /// @brief 服务端不支持转发功能
  ///
  FORWARD_STREAM_ERROR_NOT_SUPPORT(1205);

  final dynamic $value;
  const ForwardStreamError([this.$value]);
}

enum LocalProxyType {
  /// @brief Socks5 代理。选用此代理服务器，需满足 Socks5 协议标准文档的要求。
  ///
  SOCKS5(1),

  /// @brief Http 隧道代理。
  ///
  HTTP_TUNNEL(2);

  final dynamic $value;
  const LocalProxyType([this.$value]);
}

class VideoDeviceInfo extends NativeClass {
  static const _$namespace = r'com.ss.bytertc.engine.video.VideoDeviceInfo';
  static get codegen_$namespace => _$namespace;

  VideoDeviceInfo([NativeClassOptions? options])
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

  /// @brief 设备 ID
  ///
  FutureOr<String?> get deviceId async {
    return await sendInstanceGet<String?>("deviceId");
  }

  set deviceId(FutureOr<String?> value) {
    sendInstanceSet("deviceId", value);
  }

  /// @brief 设备名称
  ///
  FutureOr<String?> get deviceName async {
    return await sendInstanceGet<String?>("deviceName");
  }

  set deviceName(FutureOr<String?> value) {
    sendInstanceSet("deviceName", value);
  }

  /// @detail keytype
  /// @brief 视频设备朝向信息，参看 VideoDeviceFacing{@link #VideoDeviceFacing}。
  ///
  FutureOr<VideoDeviceFacing?> get deviceFacing async {
    try {
      final result = await sendInstanceGet<VideoDeviceFacing?>("deviceFacing");
      if (result == null) {
        return null;
      }
      return VideoDeviceFacing.values
          .firstWhere((element) => element == result);
    } catch (e) {
      return null;
    }
  }

  set deviceFacing(FutureOr<VideoDeviceFacing?> value) {
    sendInstanceSet("deviceFacing", value);
  }
}

enum LocalAudioStreamState {
  /// @brief 本地音频默认初始状态。 <br>
  ///        麦克风停止工作时回调该状态，对应错误码 LocalAudioStreamError{@link #LocalAudioStreamError} 中的 kLocalAudioStreamErrorOk 。
  ///
  LOCAL_AUDIO_STREAM_STATE_STOPPED(0),

  /// @brief 本地音频录制设备启动成功。 <br>
  ///        采集到音频首帧时回调该状态，对应错误码 LocalAudioStreamError{@link #LocalAudioStreamError} 中的 kLocalAudioStreamErrorOk 。
  ///
  LOCAL_AUDIO_STREAM_STATE_RECORDING(1),

  /// @brief 本地音频首帧编码成功。 <br>
  ///        音频首帧编码成功时回调该状态，对应错误码 LocalAudioStreamError{@link #LocalAudioStreamError} 中的 kLocalAudioStreamErrorOk 。
  ///
  LOCAL_AUDIO_STREAM_STATE_ENCODING(2),

  /// @brief 本地音频启动失败，在以下时机回调该状态： <br>
  ///       - 本地录音设备启动失败，对应错误码 LocalAudioStreamError{@link #LocalAudioStreamError} 中的 kLocalAudioStreamErrorRecordFailure 。
  ///       - 检测到没有录音设备权限，对应错误码 LocalAudioStreamError{@link #LocalAudioStreamError} 中的 kLocalAudioStreamErrorDeviceNoPermission。
  ///       - 音频编码失败，对应错误码 LocalAudioStreamError{@link #LocalAudioStreamError} 中的 kLocalAudioStreamErrorEncodeFailure。
  ///
  LOCAL_AUDIO_STREAM_STATE_FAILED(3);

  final dynamic $value;
  const LocalAudioStreamState([this.$value]);
}

class SourceWantedData extends NativeClass {
  static const _$namespace = r'com.ss.bytertc.engine.type.SourceWantedData';
  static get codegen_$namespace => _$namespace;

  SourceWantedData([NativeClassOptions? options])
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

  /// @brief 如果未开启发送性能回退，此值表示推荐的视频输入宽度； <br>
  ///        如果开启了发送性能回退，此值表示当前推流的最大宽度。
  ///
  FutureOr<int?> get width async {
    return await sendInstanceGet<int?>("width");
  }

  set width(FutureOr<int?> value) {
    sendInstanceSet("width", value);
  }

  /// @brief 如果未开启发送性能回退，此值表示推荐的视频输入高度； <br>
  ///        如果开启了发送性能回退，此值表示当前推流的最大高度。
  ///
  FutureOr<int?> get height async {
    return await sendInstanceGet<int?>("height");
  }

  set height(FutureOr<int?> value) {
    sendInstanceSet("height", value);
  }

  /// @brief 如果未开启发送性能回退，此值表示推荐的视频输入帧率，单位 fps； <br>
  ///        如果开启了发送性能回退，此值表示当前推流的最大帧率，单位 fps。
  ///
  FutureOr<int?> get frameRate async {
    return await sendInstanceGet<int?>("frameRate");
  }

  set frameRate(FutureOr<int?> value) {
    sendInstanceSet("frameRate", value);
  }
}

enum SubscribeMediaType {
  /// @brief 既不订阅音频，也不订阅视频
  ///
  NONE(0),

  /// @brief 只订阅音频，不订阅视频
  ///
  AUDIO_ONLY(1),

  /// @brief 只订阅视频，不订阅音频
  ///
  VIDEO_ONLY(2),

  /// @brief 同时订阅音频和视频
  ///
  AUDIO_AND_VIDEO(3);

  final dynamic $value;
  const SubscribeMediaType([this.$value]);
}

class HumanOrientation extends NativeClass {
  static const _$namespace = r'com.ss.bytertc.engine.data.HumanOrientation';
  static get codegen_$namespace => _$namespace;

  HumanOrientation([NativeClassOptions? options])
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

  /// @brief 正前方朝向，默认值为 `{1,0,0}`，即正前方朝向 x 轴正方向
  ///
  FutureOr<Orientation?> get forward async {
    try {
      final result = await sendInstanceGet<Orientation?>("forward");
      if (result == null) {
        return null;
      }
      return packObject(result,
          () => Orientation(const NativeClassOptions([], disableInit: true)));
    } catch (e) {
      return null;
    }
  }

  set forward(FutureOr<Orientation?> value) {
    sendInstanceSet("forward", value);
  }

  /// @brief 正右方朝向，默认值为 `{0,1,0}`，即右手朝向 y 轴正方向
  ///
  FutureOr<Orientation?> get right async {
    try {
      final result = await sendInstanceGet<Orientation?>("right");
      if (result == null) {
        return null;
      }
      return packObject(result,
          () => Orientation(const NativeClassOptions([], disableInit: true)));
    } catch (e) {
      return null;
    }
  }

  set right(FutureOr<Orientation?> value) {
    sendInstanceSet("right", value);
  }

  /// @brief 正上方朝向，默认值为 `{0,0,1}`，即头顶朝向 z 轴正方向
  ///
  FutureOr<Orientation?> get up async {
    try {
      final result = await sendInstanceGet<Orientation?>("up");
      if (result == null) {
        return null;
      }
      return packObject(result,
          () => Orientation(const NativeClassOptions([], disableInit: true)));
    } catch (e) {
      return null;
    }
  }

  set up(FutureOr<Orientation?> value) {
    sendInstanceSet("up", value);
  }
}

class MediaPlayerCustomSource extends NativeClass {
  static const _$namespace =
      r'com.ss.bytertc.engine.data.MediaPlayerCustomSource';
  static get codegen_$namespace => _$namespace;

  MediaPlayerCustomSource([NativeClassOptions? options])
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

  /// @hidden reserved for later use
  ///
  FutureOr<IMediaPlayerCustomSourceProvider?> get provider async {
    try {
      final result =
          await sendInstanceGet<IMediaPlayerCustomSourceProvider?>("provider");
      if (result == null) {
        return null;
      }
      return packObject(
          result,
          () => IMediaPlayerCustomSourceProvider(
              const NativeClassOptions([], disableInit: true)));
    } catch (e) {
      return null;
    }
  }

  set provider(FutureOr<IMediaPlayerCustomSourceProvider?> value) {
    sendInstanceSet("provider", value);
  }

  /// @detail keytype
  /// @brief 数据源模式，详见 MediaPlayerCustomSourceMode{@link #MediaPlayerCustomSourceMode}。
  ///
  FutureOr<MediaPlayerCustomSourceMode?> get mode async {
    try {
      final result =
          await sendInstanceGet<MediaPlayerCustomSourceMode?>("mode");
      if (result == null) {
        return null;
      }
      return MediaPlayerCustomSourceMode.values
          .firstWhere((element) => element == result);
    } catch (e) {
      return null;
    }
  }

  set mode(FutureOr<MediaPlayerCustomSourceMode?> value) {
    sendInstanceSet("mode", value);
  }

  /// @detail keytype
  /// @brief 数据源类型，详见 MediaPlayerCustomSourceStreamType{@link #MediaPlayerCustomSourceStreamType}
  ///
  FutureOr<MediaPlayerCustomSourceStreamType?> get type async {
    try {
      final result =
          await sendInstanceGet<MediaPlayerCustomSourceStreamType?>("type");
      if (result == null) {
        return null;
      }
      return MediaPlayerCustomSourceStreamType.values
          .firstWhere((element) => element == result);
    } catch (e) {
      return null;
    }
  }

  set type(FutureOr<MediaPlayerCustomSourceStreamType?> value) {
    sendInstanceSet("type", value);
  }
}

enum SubscribeState {
  /// @brief 订阅成功
  ///
  SUBSCRIBED(0),

  /// @brief 订阅失败
  ///
  UNSUBSCRIBED(1);

  final dynamic $value;
  const SubscribeState([this.$value]);
}

class SysStats extends NativeClass {
  static const _$namespace = r'com.ss.bytertc.engine.SysStats';
  static get codegen_$namespace => _$namespace;

  SysStats([NativeClassOptions? options])
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

  /// @brief 设备的 CPU 核数
  ///
  FutureOr<int?> get cpuCores async {
    return await sendInstanceGet<int?>("cpuCores");
  }

  set cpuCores(FutureOr<int?> value) {
    sendInstanceSet("cpuCores", value);
  }

  /// @brief 应用的 CPU 使用率，取值范围为 [0, 1]。
  ///
  FutureOr<double?> get cpuAppUsage async {
    return await sendInstanceGet<double?>("cpuAppUsage");
  }

  set cpuAppUsage(FutureOr<double?> value) {
    sendInstanceSet("cpuAppUsage", value);
  }

  /// @brief 系统的 CPU 使用率，取值范围为 [0, 1]。
  ///
  FutureOr<double?> get cpuTotalUsage async {
    return await sendInstanceGet<double?>("cpuTotalUsage");
  }

  set cpuTotalUsage(FutureOr<double?> value) {
    sendInstanceSet("cpuTotalUsage", value);
  }

  /// @brief 应用的内存占用大小（单位 MB）
  ///
  FutureOr<double?> get memoryUsage async {
    return await sendInstanceGet<double?>("memoryUsage");
  }

  set memoryUsage(FutureOr<double?> value) {
    sendInstanceSet("memoryUsage", value);
  }

  /// @brief 设备的内存大小 单位：MB
  ///
  FutureOr<long?> get fullMemory async {
    return await sendInstanceGet<long?>("fullMemory");
  }

  set fullMemory(FutureOr<long?> value) {
    sendInstanceSet("fullMemory", value);
  }

  /// @brief 系统已使用内存 MB
  ///
  FutureOr<long?> get totalMemoryUsage async {
    return await sendInstanceGet<long?>("totalMemoryUsage");
  }

  set totalMemoryUsage(FutureOr<long?> value) {
    sendInstanceSet("totalMemoryUsage", value);
  }

  /// @brief 系统当前空闲内存（MB）
  ///
  FutureOr<long?> get freeMemory async {
    return await sendInstanceGet<long?>("freeMemory");
  }

  set freeMemory(FutureOr<long?> value) {
    sendInstanceSet("freeMemory", value);
  }

  /// @brief 当前应用的内存使用率（单位 \%）
  ///
  FutureOr<double?> get memoryRatio async {
    return await sendInstanceGet<double?>("memoryRatio");
  }

  set memoryRatio(FutureOr<double?> value) {
    sendInstanceSet("memoryRatio", value);
  }

  /// @brief 系统内存使用率（单位 \%）
  ///
  FutureOr<double?> get totalMemoryRatio async {
    return await sendInstanceGet<double?>("totalMemoryRatio");
  }

  set totalMemoryRatio(FutureOr<double?> value) {
    sendInstanceSet("totalMemoryRatio", value);
  }
}

enum ForwardStreamEvent {
  /// @brief 本端与服务器网络连接断开，暂停转发。
  ///
  FORWARD_STREAM_EVENT_DISCONNECTED(0),

  /// @brief 本端与服务器网络连接恢复，转发服务连接成功。
  ///
  FORWARD_STREAM_EVENT_CONNECTED(1),

  /// @brief 转发中断。转发过程中，如果相同 user_id 的用户进入目标房间，转发中断。
  ///
  FORWARD_STREAM_EVENT_INTERRUPT(2),

  /// @brief 目标房间已更新，由 `updateForwardStreamToRooms` 触发。
  ///
  FORWARD_STREAM_EVENT_DST_ROOM_UPDATED(3),

  /// @brief API 调用时序错误。例如，在调用 `startForwardStreamToRooms` 之前调用 `updateForwardStreamToRooms`。
  ///
  FORWARD_STREAM_EVENT_UN_EXPECT_API_CALL(4);

  final dynamic $value;
  const ForwardStreamEvent([this.$value]);
}

enum AttenuationType {
  /// @brief 不随距离衰减
  ///
  ATTENUATION_TYPE_NONE(0),

  /// @brief 线性衰减，音量随距离增大而线性减小
  ///
  ATTENUATION_TYPE_LINEAR(1),

  /// @brief 指数型衰减，音量随距离增大进行指数衰减
  ///
  ATTENUATION_TYPE_EXPONENTIAL(2);

  final dynamic $value;
  const AttenuationType([this.$value]);
}

class MixedStreamControlConfig extends NativeClass {
  static const _$namespace =
      r'com.ss.bytertc.engine.live.MixedStreamControlConfig';
  static get codegen_$namespace => _$namespace;

  MixedStreamControlConfig([NativeClassOptions? options])
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
  /// @detail api
  /// @brief 设置是否开启单独发送声音提示 SEI 的功能。
  /// @param enableVolumeIndication 是否开启单独发送声音提示 SEI： <br>
  ///        - true：开启；
  ///        - false：关闭。（默认值）
  /// @note 开启后，你可以通过 seiContentMode{@link #MixedStreamControlConfig#seiContentMode} 控制 SEI 的内容是否只携带声音信息。
  ///
  FutureOr<boolean?> get enableVolumeIndication async {
    return await sendInstanceGet<boolean?>("enableVolumeIndication");
  }

  set enableVolumeIndication(FutureOr<boolean?> value) {
    sendInstanceSet("enableVolumeIndication", value);
  }

  /// @valid since 3.56
  /// @detail api
  /// @brief 设置声音信息提示间隔。
  /// @param volumeIndicationInterval 提示间隔，单位为秒，取值范围为 [0.3,+∞)，默认值为 2。 <br>
  ///        此值仅取整百毫秒。若传入两位及以上小数，则四舍五入取第一位小数的值。例如，若传入 0.36，则取 0.4。
  ///
  FutureOr<float?> get volumeIndicationInterval async {
    return await sendInstanceGet<float?>("volumeIndicationInterval");
  }

  set volumeIndicationInterval(FutureOr<float?> value) {
    sendInstanceSet("volumeIndicationInterval", value);
  }

  /// @valid since 3.56
  /// @detail api
  /// @brief 设置有效音量大小。
  /// @param talkVolume 有效音量大小，取值范围为 [0, 255]，默认值为 0。 <br>
  ///        超出取值范围则自动调整为默认值，即 0。
  ///
  FutureOr<int?> get talkVolume async {
    return await sendInstanceGet<int?>("talkVolume");
  }

  set talkVolume(FutureOr<int?> value) {
    sendInstanceSet("talkVolume", value);
  }

  /// @valid since 3.56
  /// @detail api
  /// @brief 设置声音信息 SEI 是否包含音量值。
  /// @param addVolumeValue 是否包含音量值： <br>
  ///        - true：包含；
  ///        - false：不包含。默认值。
  ///
  FutureOr<boolean?> get isAddVolumeValue async {
    return await sendInstanceGet<boolean?>("isAddVolumeValue");
  }

  set isAddVolumeValue(FutureOr<boolean?> value) {
    sendInstanceSet("isAddVolumeValue", value);
  }

  /// @valid since 3.56
  /// @detail api
  /// @brief 设置 SEI 内容。
  /// @param seiContentMode SEI 内容，参看 MixedStreamSEIContentMode{@link #MixedStreamSEIContentMode}。
  ///
  FutureOr<MixedStreamSEIContentMode?> get seiContentMode async {
    try {
      final result =
          await sendInstanceGet<MixedStreamSEIContentMode?>("seiContentMode");
      if (result == null) {
        return null;
      }
      return MixedStreamSEIContentMode.values
          .firstWhere((element) => element == result);
    } catch (e) {
      return null;
    }
  }

  set seiContentMode(FutureOr<MixedStreamSEIContentMode?> value) {
    sendInstanceSet("seiContentMode", value);
  }

  /// @valid since 3.56
  /// @detail api
  /// @author liujingchao
  /// @brief 设置合流转推 SEI 信息的 payload type。
  /// @param seiPayloadType 默认值为 `100`，只支持设置 `5` 和 `100`。
  /// @return MixedStreamControlConfig，参看 MixedStreamControlConfig{@link# MixedStreamControlConfig}。
  /// @note 在转推直播的过程中，该参数不支持变更。
  ///
  FutureOr<int?> get seiPayloadType async {
    return await sendInstanceGet<int?>("seiPayloadType");
  }

  set seiPayloadType(FutureOr<int?> value) {
    sendInstanceSet("seiPayloadType", value);
  }

  /// @valid since 3.56
  /// @detail api
  /// @author liujingchao
  /// @brief 设置合流转推 SEI 信息的 Payload UUID。
  /// @param seiPayloadUuid 该参数长度需为 32 位，否则会收到错误码为 1091 的回调。该参数每个字符的范围需为 [0, 9] [a, f] [A, F]。 <br>
  ///                       该参数不应带有`-`字符，如系统自动生成的 UUID 中带有`-`，则应删去。
  /// @return MixedStreamControlConfig，参看 MixedStreamControlConfig{@link# MixedStreamControlConfig}。
  /// @note PayloadType 为 `5` 时，必须填写 PayloadUUID，否则会收到错误回调，错误码为 1091。 <br>
  ///        PayloadType 不是 `5` 时，不需要填写 PayloadUUID，如果填写会被后端忽略。 <br>
  ///        在转推直播的过程中，该参数不支持变更。
  ///
  FutureOr<String?> get seiPayloadUuid async {
    return await sendInstanceGet<String?>("seiPayloadUuid");
  }

  set seiPayloadUuid(FutureOr<String?> value) {
    sendInstanceSet("seiPayloadUuid", value);
  }

  /// @valid since 3.57
  /// @detail api
  /// @brief 设置合流推到 CDN 时输出的媒体流类型。
  /// @param mediaType 输出的媒体流类型。参看 MixedStreamMediaType{@link #MixedStreamMediaType}。 <br>
  ///        默认输出音视频流。支持输出纯音频流，但暂不支持输出纯视频流。
  ///
  FutureOr<MixedStreamMediaType?> get mediaType async {
    try {
      final result = await sendInstanceGet<MixedStreamMediaType?>("mediaType");
      if (result == null) {
        return null;
      }
      return MixedStreamMediaType.values
          .firstWhere((element) => element == result);
    } catch (e) {
      return null;
    }
  }

  set mediaType(FutureOr<MixedStreamMediaType?> value) {
    sendInstanceSet("mediaType", value);
  }

  /// @valid since 3.57
  /// @detail api
  /// @brief 设置是否在没有用户发布流的情况下发起转推直播。
  /// @param pushStreamMode 合流任务发起模式。具体参看 MixedStreamPushMode{@link #MixedStreamPushMode}。 <br>
  ///        该参数在发起合流任务后的转推直播过程中不支持动态变更。
  ///
  FutureOr<MixedStreamPushMode?> get pushStreamMode async {
    try {
      final result =
          await sendInstanceGet<MixedStreamPushMode?>("pushStreamMode");
      if (result == null) {
        return null;
      }
      return MixedStreamPushMode.values
          .firstWhere((element) => element == result);
    } catch (e) {
      return null;
    }
  }

  set pushStreamMode(FutureOr<MixedStreamPushMode?> value) {
    sendInstanceSet("pushStreamMode", value);
  }
}

enum LocalProxyError {
  /// @brief 本地代理服务器无错误。
  ///
  OK(0),

  /// @brief 代理服务器回复的版本号不符合 Socks5 协议标准文档的规定，导致 Socks5 代理连接失败。请检查代理服务器是否存在异常。
  ///
  SOCKS5_VERSION_ERROR(1),

  /// @brief 代理服务器回复的格式错误不符合 Socks5 协议标准文档的规定，导致 Socks5 代理连接失败。请检查代理服务器是否存在异常。
  ///
  SOCKS5_FORMAT_ERROR(2),

  /// @brief 代理服务器回复的字段值不符合 Socks5 协议标准文档的规定，导致 Socks5 代理连接失败。请检查代理服务器是否存在异常。
  ///
  SOCKS5_INVALID_VALUE(3),

  /// @brief 未提供代理服务器的用户名及密码，导致 Socks5 代理连接失败。请重新调用 `setLocalProxy`，在设置本地代理时填入用户名和密码。
  ///
  SOCKS5_USER_PASS_NOT_GIVEN(4),

  /// @brief TCP 关闭，导致 Socks5 代理连接失败。请检查网络或者代理服务器是否存在异常。
  ///
  SOCKS5_TCP_CLOSED(5),

  /// @brief Http 隧道代理错误。请检查 Http 隧道代理服务器或者网络是否存在异常。
  ///
  HTTP_TUNNEL_FAILED(6);

  final dynamic $value;
  const LocalProxyError([this.$value]);
}

enum VideoSuperResolutionModeChangedReason {
  /// @brief 调用 setRemoteVideoSuperResolution{@link #RTCEngine#setRemoteVideoSuperResolution} 成功关闭超分。
  ///
  VIDEO_SUPER_RESOLUTION_MODE_CHANGED_REASON_API_OFF(0),

  /// @brief 调用 setRemoteVideoSuperResolution{@link #RTCEngine#setRemoteVideoSuperResolution} 成功开启超分。
  ///
  VIDEO_SUPER_RESOLUTION_MODE_CHANGED_REASON_API_ON(1),

  /// @brief 开启超分失败，远端视频流的原始视频分辨率超过 640 × 360 px。
  ///
  VIDEO_SUPER_RESOLUTION_MODE_CHANGED_REASON_RESOLUTION_EXCEED(2),

  /// @brief 开启超分失败，已对一路远端流开启超分。
  ///
  VIDEO_SUPER_RESOLUTION_MODE_CHANGED_REASON_OVER_USE(3),

  /// @brief 设备不支持使用超分辨率。
  ///
  VIDEO_SUPER_RESOLUTION_MODE_CHANGED_REASON_DEVICE_NOT_SUPPORT(4),

  /// @brief 当前设备性能存在风险，已动态关闭超分。
  ///
  VIDEO_SUPER_RESOLUTION_MODE_CHANGED_REASON_DYNAMIC_CLOSE(5),

  /// @brief 超分因其他原因关闭。
  ///
  VIDEO_SUPER_RESOLUTION_MODE_CHANGED_REASON_OTHER_SETTING_DISABLED(6),

  /// @brief 超分因其他原因开启。
  ///
  VIDEO_SUPER_RESOLUTION_MODE_CHANGED_REASON_OTHER_SETTING_ENABLED(7),

  /// @brief SDK 没有编译超分组件。
  ///
  VIDEO_SUPER_RESOLUTION_MODE_CHANGED_REASON_NO_COMPONENT(8),

  /// @brief 远端流不存在。房间 ID 或用户 ID 无效，或对方没有发布流。
  ///
  VIDEO_SUPER_RESOLUTION_MODE_CHANGED_REASON_STREAM_NOT_EXIST(9);

  final dynamic $value;
  const VideoSuperResolutionModeChangedReason([this.$value]);
}

class NetworkTimeInfo extends NativeClass {
  static const _$namespace = r'com.ss.bytertc.engine.NetworkTimeInfo';
  static get codegen_$namespace => _$namespace;

  NetworkTimeInfo([NativeClassOptions? options])
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
  ///
  FutureOr<long?> get timestamp async {
    return await sendInstanceGet<long?>("timestamp");
  }

  set timestamp(FutureOr<long?> value) {
    sendInstanceSet("timestamp", value);
  }

  /// @hidden constructor/destructor
  ///

  FutureOr<void> nativeSetTimestamp(long timestamp) async {
    return await nativeCall('nativeSetTimestamp', [timestamp]);
  }
}

class MixedStreamPushTargetConfig extends NativeClass {
  static const _$namespace =
      r'com.ss.bytertc.engine.live.MixedStreamPushTargetConfig';
  static get codegen_$namespace => _$namespace;

  MixedStreamPushTargetConfig([NativeClassOptions? options])
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
  /// @brief 设置推流目标。
  ///
  FutureOr<MixedStreamPushTargetType?> get pushTargetType async {
    try {
      final result =
          await sendInstanceGet<MixedStreamPushTargetType?>("pushTargetType");
      if (result == null) {
        return null;
      }
      return MixedStreamPushTargetType.values
          .firstWhere((element) => element == result);
    } catch (e) {
      return null;
    }
  }

  set pushTargetType(FutureOr<MixedStreamPushTargetType?> value) {
    sendInstanceSet("pushTargetType", value);
  }

  /// @detail api
  /// @brief 设置推流 CDN 地址。仅支持 RTMP 协议，Url 必须满足正则 `/^rtmps?:\\/\\//`。建议设置。 <br>
  ///        本参数不支持过程中更新。
  ///
  FutureOr<String?> get pushCDNURL async {
    return await sendInstanceGet<String?>("pushCDNURL");
  }

  set pushCDNURL(FutureOr<String?> value) {
    sendInstanceSet("pushCDNURL", value);
  }

  /// @detail keytype
  /// @brief WTN 流 ID。合流任务不支持设置本参数。
  ///
  FutureOr<String?> get pushWTNStreamID async {
    return await sendInstanceGet<String?>("pushWTNStreamID");
  }

  set pushWTNStreamID(FutureOr<String?> value) {
    sendInstanceSet("pushWTNStreamID", value);
  }

  FutureOr<int> getMixedStreamPushTargetTypePushTargetType() async {
    return await nativeCall('getMixedStreamPushTargetTypePushTargetType', []);
  }

  FutureOr<String> getMixedStreamPushTargetTypePushCDNURL() async {
    return await nativeCall('getMixedStreamPushTargetTypePushCDNURL', []);
  }

  FutureOr<String> getMixedStreamPushTargetTypePushWTNStreamID() async {
    return await nativeCall('getMixedStreamPushTargetTypePushWTNStreamID', []);
  }
}

class ExpressionDetectInfo extends NativeClass {
  static const _$namespace =
      r'com.ss.bytertc.engine.video.ExpressionDetectInfo';
  static get codegen_$namespace => _$namespace;

  ExpressionDetectInfo([NativeClassOptions? options])
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
  ///
  FutureOr<float?> get age async {
    return await sendInstanceGet<float?>("age");
  }

  set age(FutureOr<float?> value) {
    sendInstanceSet("age", value);
  }

  /// @brief 预测为男性的概率，取值范围 (0.0, 1.0)。
  ///
  FutureOr<float?> get boyProb async {
    return await sendInstanceGet<float?>("boyProb");
  }

  set boyProb(FutureOr<float?> value) {
    sendInstanceSet("boyProb", value);
  }

  /// @brief 预测的吸引力分数，取值范围 (0, 100)。
  ///
  FutureOr<float?> get attractive async {
    return await sendInstanceGet<float?>("attractive");
  }

  set attractive(FutureOr<float?> value) {
    sendInstanceSet("attractive", value);
  }

  /// @brief 预测的微笑程度，取值范围 (0, 100)。
  ///
  FutureOr<float?> get happyScore async {
    return await sendInstanceGet<float?>("happyScore");
  }

  set happyScore(FutureOr<float?> value) {
    sendInstanceSet("happyScore", value);
  }

  /// @brief 预测的伤心程度，取值范围 (0, 100)。
  ///
  FutureOr<float?> get sadScore async {
    return await sendInstanceGet<float?>("sadScore");
  }

  set sadScore(FutureOr<float?> value) {
    sendInstanceSet("sadScore", value);
  }

  /// @brief 预测的生气程度，取值范围 (0, 100)。
  ///
  FutureOr<float?> get angryScore async {
    return await sendInstanceGet<float?>("angryScore");
  }

  set angryScore(FutureOr<float?> value) {
    sendInstanceSet("angryScore", value);
  }

  /// @brief 预测的吃惊程度，取值范围 (0, 100)。
  ///
  FutureOr<float?> get surpriseScore async {
    return await sendInstanceGet<float?>("surpriseScore");
  }

  set surpriseScore(FutureOr<float?> value) {
    sendInstanceSet("surpriseScore", value);
  }

  /// @brief 预测的情绪激动程度，取值范围 (0, 100)。
  ///
  FutureOr<float?> get arousal async {
    return await sendInstanceGet<float?>("arousal");
  }

  set arousal(FutureOr<float?> value) {
    sendInstanceSet("arousal", value);
  }

  /// @brief 预测的情绪正负程度，取值范围 (-100, 100)。
  ///
  FutureOr<float?> get valence async {
    return await sendInstanceGet<float?>("valence");
  }

  set valence(FutureOr<float?> value) {
    sendInstanceSet("valence", value);
  }
}

enum MixedStreamAlternateImageFillMode {
  /// @brief 占位图跟随用户原始视频帧相同的比例缩放。默认设置。
  ///
  FIT(0),

  /// @brief 占位图不跟随用户原始视频帧相同的比例缩放，保持图片原有比例。
  ///
  FILL(1);

  final dynamic $value;
  const MixedStreamAlternateImageFillMode([this.$value]);
}

class DestInfo extends NativeClass {
  static const _$namespace = r'com.ss.bytertc.engine.data.DestInfo';
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

  /// @brief 目标房间 ID，指定要将媒体流推送到哪个房间。
  ///
  FutureOr<String?> get roomId async {
    return await sendInstanceGet<String?>("roomId");
  }

  set roomId(FutureOr<String?> value) {
    sendInstanceSet("roomId", value);
  }

  /// @brief 转推的流在目标房间中的用户 ID，目标房间的用户将看到该 ID 作为流的发布者，也可用于订阅该流。
  ///
  FutureOr<String?> get userId async {
    return await sendInstanceGet<String?>("userId");
  }

  set userId(FutureOr<String?> value) {
    sendInstanceSet("userId", value);
  }
}

enum AlphaLayout {
  /// @brief Alpha 数据置于 RGB 数据上方。
  ///
  TOP(0),

  /// @hidden currently not available
  /// @brief Alpha 数据置于 RGB 数据下方。
  ///
  BOTTOM(1),

  /// @hidden currently not available
  /// @brief Alpha 数据置于 RGB 数据左方。
  ///
  LEFT(2),

  /// @hidden currently not available
  /// @brief Alpha 数据置于 RGB 数据右方。
  ///
  RIGHT(3);

  final dynamic $value;
  const AlphaLayout([this.$value]);
}

class AudioFrame extends NativeClass {
  static const _$namespace = r'com.ss.bytertc.engine.utils.AudioFrame';
  static get codegen_$namespace => _$namespace;

  AudioFrame([NativeClassOptions? options])
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
  ///
  FutureOr<ArrayBuffer?> get buffer async {
    return await sendInstanceGet<ArrayBuffer?>("buffer");
  }

  set buffer(FutureOr<ArrayBuffer?> value) {
    sendInstanceSet("buffer", value);
  }

  /// @brief 采样点个数
  ///
  FutureOr<int?> get samples async {
    return await sendInstanceGet<int?>("samples");
  }

  set samples(FutureOr<int?> value) {
    sendInstanceSet("samples", value);
  }

  /// @brief 采样率，参看 AudioSampleRate{@link #AudioSampleRate}。
  ///
  FutureOr<AudioSampleRate?> get sampleRate async {
    try {
      final result = await sendInstanceGet<AudioSampleRate?>("sampleRate");
      if (result == null) {
        return null;
      }
      return AudioSampleRate.values.firstWhere((element) => element == result);
    } catch (e) {
      return null;
    }
  }

  set sampleRate(FutureOr<AudioSampleRate?> value) {
    sendInstanceSet("sampleRate", value);
  }

  /// @brief 音频声道，参看 AudioChannel{@link #AudioChannel}。
  ///
  FutureOr<AudioChannel?> get channel async {
    try {
      final result = await sendInstanceGet<AudioChannel?>("channel");
      if (result == null) {
        return null;
      }
      return AudioChannel.values.firstWhere((element) => element == result);
    } catch (e) {
      return null;
    }
  }

  set channel(FutureOr<AudioChannel?> value) {
    sendInstanceSet("channel", value);
  }
}

enum LocalVideoStreamState {
  /// @brief 本地视频采集停止状态（默认初始状态） <br>
  ///        本地视频采集关闭时回调该状态，对应错误码 LocalVideoStreamError{@link #LocalVideoStreamError} 中的 `LOCAL_VIDEO_STREAM_ERROR_OK`
  ///
  LOCAL_VIDEO_STREAM_STATE_STOPPED(0),

  /// @brief 本地视频采集设备启动成功 <br>
  ///        本地视频采集开启时回调该状态，对应错误码 LocalVideoStreamError{@link #LocalVideoStreamError} 中的 `LOCAL_VIDEO_STREAM_ERROR_OK`
  ///
  LOCAL_VIDEO_STREAM_STATE_RECORDING(1),

  /// @brief 本地视频采集后，首帧编码成功 <br>
  ///        本地视频首帧编码成功时回调该状态，对应错误码 LocalVideoStreamError{@link #LocalVideoStreamError} 中的 `LOCAL_VIDEO_STREAM_ERROR_OK`
  ///
  LOCAL_VIDEO_STREAM_STATE_ENCODING(2),

  /// @brief 本地视频启动失败 <br>
  ///        - 本地视频采集设备启动失败，对应错误码 LocalVideoStreamError{@link #LocalVideoStreamError} 中的 `LOCAL_VIDEO_STREAM_ERROR_FAILURE`
  ///        - 检测到没有视频采集设备权限，对应错误码 LocalVideoStreamError{@link #LocalVideoStreamError} 中的 `LOCAL_VIDEO_STREAM_ERROR_DEVICE_NO_PERMISSION`
  ///        - 视频编码失败，对应错误码 LocalVideoStreamError{@link #LocalVideoStreamError} 中的 `LOCAL_VIDEO_STREAM_ERROR_ENCODE_FAILURE`
  ///
  LOCAL_VIDEO_STREAM_STATE_FAILED(3);

  final dynamic $value;
  const LocalVideoStreamState([this.$value]);
}

enum UserVisibilityChangeError {
  /// @brief 成功。
  ///
  OK(0),

  /// @brief 未知错误。
  ///
  UNKNOWN(1),

  /// @brief 房间内可见用户达到上限。
  ///
  TOO_MANY_VISIBLE_USER(2);

  final dynamic $value;
  const UserVisibilityChangeError([this.$value]);
}

enum RoomState {
  /// @brief 加入房间成功
  ///
  JOIN_SUCCESS(0),

  /// @brief 加入房间失败
  ///
  JOIN_FAILED(1),

  /// @brief 离开房间
  ///
  LEFT(2);

  final dynamic $value;
  const RoomState([this.$value]);
}

enum LocalVideoStreamError {
  /// @brief 状态正常（本地视频状态改变正常时默认返回值）
  ///
  LOCAL_VIDEO_STREAM_ERROR_OK(0),

  /// @brief 本地视频流发布失败
  ///
  LOCAL_VIDEO_STREAM_ERROR_FAILURE(1),

  /// @brief 没有权限启动本地视频采集设备
  ///
  LOCAL_VIDEO_STREAM_ERROR_DEVICE_NO_PERMISSION(2),

  /// @brief 本地视频采集设备已被占用
  ///
  LOCAL_VIDEO_STREAM_ERROR_DEVICE_BUSY(3),

  /// @brief 本地视频采集设备不存在或已移除
  ///
  LOCAL_VIDEO_STREAM_ERROR_DEVICE_NOT_FOUND(4),

  /// @brief 本地视频采集失败，建议检查采集设备是否正常工作
  ///
  LOCAL_VIDEO_STREAM_ERROR_CAPTURE_FAILURE(5),

  /// @brief 本地视频编码失败
  ///
  LOCAL_VIDEO_STREAM_ERROR_ENCODE_FAILURE(6),

  /// @brief 通话过程中本地视频采集设备被其他程序抢占，导致设备连接中断
  ///
  LOCAL_VIDEO_STREAM_ERROR_DEVICE_DISCONNECTED(7);

  final dynamic $value;
  const LocalVideoStreamError([this.$value]);
}

enum LogoutReason {
  /// @brief 用户主动退出 <br>
  ///        用户调用 `logout` 接口登出，或者销毁引擎登出。
  ///
  LOGOUT_REASON_LOGOUT(0),

  /// @brief 用户被动退出 <br>
  ///        另一个用户以相同 UserId 进行了 `login`，导致本端用户被踢出。
  ///
  LOGOUT_REASON_DUPLICATE_LOGIN(1);

  final dynamic $value;
  const LogoutReason([this.$value]);
}

enum LocalVideoSinkPosition {
  /// @brief 采集后。
  ///
  AFTER_CAPTURE(0),

  /// @brief （默认值）前处理后。
  ///
  AFTER_PREPROCESS(1);

  final dynamic $value;
  const LocalVideoSinkPosition([this.$value]);
}

class RTCLogConfig extends NativeClass {
  static const _$namespace = r'com.ss.bytertc.engine.data.RTCLogConfig';
  static get codegen_$namespace => _$namespace;

  RTCLogConfig([NativeClassOptions? options])
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
  ///
  FutureOr<String?> get logPath async {
    return await sendInstanceGet<String?>("logPath");
  }

  set logPath(FutureOr<String?> value) {
    sendInstanceSet("logPath", value);
  }

  /// @brief 日志文件最大占用的总空间，单位为 MB，选填。取值范围为 1～100 MB，默认值为 10 MB。 <br>
  ///        若 `logFileSize` < 1，取 1 MB。若 `logFileSize` > 100，取 100 MB。 <br>
  ///        其中，单个日志文件最大为 2 MB： <br>
  ///        \\</ul>\<li> 若 1 ≤ <code>logFileSize</code> ≤ 2，则会生成一个日志文件。\</li>\<li>若 <code>logFileSize</code> > 2，假设 <code>logFileSize/2</code> 的整数部分为 N，则前 N 个文件，每个文件会写满 2 MB，第 N+1 个文件大小不超过 <code>logFileSize mod 2</code>，否则会删除最老的文件，以此类推。\</li></ul>
  ///
  FutureOr<int?> get logFileSize async {
    return await sendInstanceGet<int?>("logFileSize");
  }

  set logFileSize(FutureOr<int?> value) {
    sendInstanceSet("logFileSize", value);
  }

  /// @brief 日志等级，参看 LocalLogLevel{@link #LocalLogLevel}，默认为警告级别，选填。
  ///
  FutureOr<LocalLogLevel?> get logLevel async {
    try {
      final result = await sendInstanceGet<LocalLogLevel?>("logLevel");
      if (result == null) {
        return null;
      }
      return LocalLogLevel.values.firstWhere((element) => element == result);
    } catch (e) {
      return null;
    }
  }

  set logLevel(FutureOr<LocalLogLevel?> value) {
    sendInstanceSet("logLevel", value);
  }

  /// @brief 日志文件名前缀，选填。该字符串必须符合正则表达式：[a-zA-Z0-9_\@\\-\\.]{1,128}。 <br>
  ///        最终的日志文件名为`前缀 + "_" + 文件创建时间 + "_rtclog".log`，如 `logPrefix_2023-05-25_172324_rtclog.log`。
  ///
  FutureOr<String?> get logFilenamePrefix async {
    return await sendInstanceGet<String?>("logFilenamePrefix");
  }

  set logFilenamePrefix(FutureOr<String?> value) {
    sendInstanceSet("logFilenamePrefix", value);
  }
}

enum RemoteVideoState {
  /// @brief 远端视频流默认初始状态 <br>
  ///        在以下时机回调该状态： <br>
  ///        - 本地用户停止接收远端视频流，对应错误码 RemoteVideoStateChangeReason{@link #RemoteVideoStateChangeReason} 中的 REMOTE_VIDEO_STATE_CHANGE_REASON_LOCAL_MUTED。
  ///        - 远端用户停止发送视频流，对应错误码 RemoteVideoStateChangeReason{@link #RemoteVideoStateChangeReason} 中的 REMOTE_VIDEO_STATE_CHANGE_REASON_REMOTE_MUTED。
  ///        - 远端用户离开房间，对应错误码 RemoteVideoStateChangeReason{@link #RemoteVideoStateChangeReason} 中的 REMOTE_VIDEO_STATE_CHANGE_REASON_REMOTE_OFFLINE。
  ///
  REMOTE_VIDEO_STATE_STOPPED(0),

  /// @brief 本地用户已接收远端视频首包 <br>
  ///        收到远端视频首包时回调该状态，对应错误码 RemoteVideoStateChangeReason{@link #RemoteVideoStateChangeReason} 中的 REMOTE_VIDEO_STATE_CHANGE_REASON_LOCAL_UNMUTED。
  ///
  REMOTE_VIDEO_STATE_STARTING(1),

  /// @brief 远端视频流正在解码，正常播放 <br>
  ///        在以下时机回调该状态： <br>
  ///        - 成功解码远端视频首帧，对应错误码 RemoteVideoStateChangeReason{@link #RemoteVideoStateChangeReason} 中的 REMOTE_VIDEO_STATE_CHANGE_REASON_LOCAL_UNMUTED。
  ///        - 网络由阻塞恢复正常，对应错误码 RemoteVideoStateChangeReason{@link #RemoteVideoStateChangeReason} 中的 REMOTE_VIDEO_STATE_CHANGE_REASON_NETWORK_RECOVERY。
  ///        - 本地用户恢复接收远端视频流，对应错误码 RemoteVideoStateChangeReason{@link #RemoteVideoStateChangeReason} 中的 REMOTE_VIDEO_STATE_CHANGE_REASON_LOCAL_UNMUTED。
  ///        - 远端用户恢复发送视频流，对应错误码 RemoteVideoStateChangeReason{@link #RemoteVideoStateChangeReason} 中的 REMOTE_VIDEO_STATE_CHANGE_REASON_NETWORK_CONGESTION。
  ///
  REMOTE_VIDEO_STATE_DECODING(2),

  /// @brief 远端视频流卡顿 <br>
  ///        网络阻塞、丢包率大于 40\%时回调该状态，对应错误码 RemoteVideoStateChangeReason{@link #RemoteVideoStateChangeReason} 中的 REMOTE_VIDEO_REASON_NETWORK_CONGESTION 。
  ///
  REMOTE_VIDEO_STATE_FROZEN(3),

  /// @hidden currently not available
  /// @brief 远端视频流播放失败
  /// @note 如果内部处理远端视频流失败，则会回调该方法，对应错误码 RemoteVideoStateChangeReason{@link #RemoteVideoStateChangeReason} 中的 REMOTE_VIDEO_REASON_INTERNAL 。
  ///
  REMOTE_VIDEO_STATE_FAILED(4);

  final dynamic $value;
  const RemoteVideoState([this.$value]);
}

enum AudioChannel {
  /// @brief 默认设置。双声道。
  ///
  AUDIO_CHANNEL_AUTO(-1),

  /// @brief 单声道
  ///
  AUDIO_CHANNEL_MONO(1),

  /// @brief 双声道
  ///
  AUDIO_CHANNEL_STEREO(2);

  final dynamic $value;
  const AudioChannel([this.$value]);
}

enum AudioMixingError {
  /// @brief 正常
  ///
  AUDIO_MIXING_ERROR_OK(0),

  /// @brief 预加载失败。找不到混音文件或者文件长度超出 20s
  ///
  AUDIO_MIXING_ERROR_PRELOAD_FAILED(1),

  /// @brief 混音开启失败。找不到混音文件或者混音文件打开失败
  ///
  AUDIO_MIXING_ERROR_START_FAILED(2),

  /// @brief 混音 ID 异常
  ///
  AUDIO_MIXING_ERROR_ID_NOT_FOUND(3),

  /// @brief 设置混音文件的播放位置出错
  ///
  AUDIO_MIXING_ERROR_SET_POSITION_FAILED(4),

  /// @brief 音量参数不合法，仅支持设置的音量值为[0, 400]
  ///
  AUDIO_MIXING_ERROR_INVALID_VOLUME(5),

  /// @brief 播放的文件与预加载的文件不一致。请先使用 unloadAudioMixing 卸载此前的文件。
  ///
  AUDIO_MIXING_ERROR_LOAD_CONFLICT(6),

  /// @brief 不支持此混音类型。
  ///
  AUDIO_MIXING_ERROR_ID_TYPE_NOT_MATCH(7),

  /// @brief 设置混音文件的音调不合法
  ///
  AUDIO_MIXING_ERROR_ID_TYPE_INVALID_PITCH(8),

  /// @brief 设置混音文件的音轨不合法
  ///
  AUDIO_MIXING_ERROR_INVALID_AUDIO_TRACK(9),

  /// @brief 混音文件正在启动中
  ///
  AUDIO_MIXING_ERROR_IS_STARTING(10),

  /// @brief 设置混音文件的播放速度不合法
  ///
  AUDIO_MIXING_ERROR_INVALID_PLAYBACK_SPEED(11);

  final dynamic $value;
  const AudioMixingError([this.$value]);
}

class IVideoSink extends NativeClass {
  static const _$namespace = r'com.ss.bytertc.engine.video.IVideoSink';
  static get codegen_$namespace => _$namespace;

  IVideoSink([NativeClassOptions? options])
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

  /// @detail callback
  /// @brief 视频帧回调
  /// @param frame 视频帧结构类，参看 IVideoFrame{@link #IVideoFrame}
  ///

  FutureOr<void> onFrame(IVideoFrame frame) async {
    return await nativeCall('onFrame', [frame]);
  }

  /// @detail api
  /// @brief 获取外部渲染耗时。
  /// @return 外部渲染平均耗时，单位：毫秒
  ///

  FutureOr<int> getRenderElapse() async {
    return await nativeCall('getRenderElapse', []);
  }
}

enum RoomEvent {
  /// @brief 当房间内人数超过 500 人时，停止向房间内已有用户发送 `onUserJoined` 和 `onUserLeave` 回调，并通过广播提示房间内所有用户。通过 onRoomStateChanged{@link #IRTCRoomEventHandler#onRoomStateChanged} 回调。
  ///
  USER_NOTIFY_STOP(-2013),

  /// @brief 房间/用户被封禁，通过房间事件通知封禁时间。
  ///
  FORBIDDEN(-2012);

  final dynamic $value;
  const RoomEvent([this.$value]);
}

class RemoteVideoSinkConfig extends NativeClass {
  static const _$namespace =
      r'com.ss.bytertc.engine.video.RemoteVideoSinkConfig';
  static get codegen_$namespace => _$namespace;

  RemoteVideoSinkConfig([NativeClassOptions? options])
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

  /// @brief 远端视频帧回调位置，参看 RemoteVideoSinkPosition{@link #RemoteVideoSinkPosition}，默认回调后处理后的视频帧。
  ///
  FutureOr<RemoteVideoSinkPosition?> get position async {
    try {
      final result =
          await sendInstanceGet<RemoteVideoSinkPosition?>("position");
      if (result == null) {
        return null;
      }
      return RemoteVideoSinkPosition.values
          .firstWhere((element) => element == result);
    } catch (e) {
      return null;
    }
  }

  set position(FutureOr<RemoteVideoSinkPosition?> value) {
    sendInstanceSet("position", value);
  }

  /// @brief 远端视频帧回调格式，参看 VideoPixelFormat{@link #VideoPixelFormat}，默认值为 0。
  ///
  FutureOr<VideoPixelFormat?> get pixelFormat async {
    try {
      final result = await sendInstanceGet<VideoPixelFormat?>("pixelFormat");
      if (result == null) {
        return null;
      }
      return VideoPixelFormat.values.firstWhere((element) => element == result);
    } catch (e) {
      return null;
    }
  }

  set pixelFormat(FutureOr<VideoPixelFormat?> value) {
    sendInstanceSet("pixelFormat", value);
  }

  /// @brief 是否将视频帧自动转正，参看 VideoApplyRotation{@link #VideoApplyRotation}，默认为不旋转。
  ///
  FutureOr<VideoApplyRotation?> get applyRotation async {
    try {
      final result =
          await sendInstanceGet<VideoApplyRotation?>("applyRotation");
      if (result == null) {
        return null;
      }
      return VideoApplyRotation.values
          .firstWhere((element) => element == result);
    } catch (e) {
      return null;
    }
  }

  set applyRotation(FutureOr<VideoApplyRotation?> value) {
    sendInstanceSet("applyRotation", value);
  }

  /// @brief 是否将视频帧镜像。参看 VideoSinkMirrorType{@link #VideoSinkMirrorType}，默认为不镜像。 <br>
  ///        本设置与 setRemoteVideoMirrorType{@link #RTCEngine#setRemoteVideoMirrorType} （适用于内部渲染）相互独立。
  ///
  FutureOr<VideoSinkMirrorType?> get mirrorType async {
    try {
      final result = await sendInstanceGet<VideoSinkMirrorType?>("mirrorType");
      if (result == null) {
        return null;
      }
      return VideoSinkMirrorType.values
          .firstWhere((element) => element == result);
    } catch (e) {
      return null;
    }
  }

  set mirrorType(FutureOr<VideoSinkMirrorType?> value) {
    sendInstanceSet("mirrorType", value);
  }
}

enum InterpolationMode {
  /// @detail keytype
  /// @brief 补最后一帧
  ///
  LAST_FRAME_FILL(0),

  /// @detail keytype
  /// @brief 补背景图片
  ///
  BACKGROUND_IMAGE_FILL(1);

  final dynamic $value;
  const InterpolationMode([this.$value]);
}

enum AudioReportMode {
  /// @brief 默认始终开启音量回调。
  ///
  AUDIO_REPORT_MODE_NORMAL(0),

  /// @brief 可见用户进房并停止推流后，关闭音量回调。
  ///
  AUDIO_REPORT_MODE_DISCONNECT(1),

  /// @brief 可见用户进房并停止推流后，开启音量回调，回调值重置为 0。
  ///
  AUDIO_REPORT_MODE_RESET(2);

  final dynamic $value;
  const AudioReportMode([this.$value]);
}

enum AudioFrameSource {
  /// @brief 本地麦克风采集的音频数据。
  ///
  AUDIO_FRAME_SOURCE_MIC(0),

  /// @brief 远端所有用户混音后的数据
  ///
  AUDIO_FRAME_SOURCE_PLAYBACK(1),

  /// @brief 本地麦克风和所有远端用户音频流的混音后的数据
  ///
  AUDIO_FRAME_SOURCE_MIXED(2);

  final dynamic $value;
  const AudioFrameSource([this.$value]);
}

enum AudioSourceType {
  /// @brief 自定义采集音频源
  ///
  AUDIO_SOURCE_TYPE_EXTERNAL(0),

  /// @brief RTC SDK 内部采集音频源
  ///
  AUDIO_SOURCE_TYPE_INTERNAL(1);

  final dynamic $value;
  const AudioSourceType([this.$value]);
}

class IAudioFrame extends NativeClass {
  static const _$namespace = r'com.ss.bytertc.engine.utils.IAudioFrame';
  static get codegen_$namespace => _$namespace;

  IAudioFrame([NativeClassOptions? options])
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
  /// @author majun.lvhiei
  /// @brief 获取音频帧时间戳。
  /// @return 音频帧时间戳，单位：微秒。
  ///

  FutureOr<long> timestamp_us() async {
    return await nativeCall('timestamp_us', []);
  }

  /// @detail api
  /// @author majun.lvhiei
  /// @brief 获取音频采样率，参看 AudioSampleRate{@link #AudioSampleRate}。
  /// @return 音频采样率
  ///

  FutureOr<AudioSampleRate> sample_rate() async {
    return await nativeCall('sample_rate', []);
  }

  /// @detail api
  /// @author majun.lvhiei
  /// @brief 获取音频声道，参看 AudioChannel{@link #AudioChannel}。
  /// @return 音频声道
  /// @note 双声道的情况下，左右声道的音频帧数据以 LRLRLR 形式排布。
  ///

  FutureOr<AudioChannel> channel() async {
    return await nativeCall('channel', []);
  }

  /// @detail api
  /// @author majun.lvhiei
  /// @brief 获取音频帧内存块地址
  /// @return 音频帧的 ByteBuffer
  ///

  FutureOr<ByteBuffer> getDataBuffer() async {
    return await nativeCall('getDataBuffer', []);
  }

  /// @detail api
  /// @author majun.lvhiei
  /// @brief 获取音频帧数据大小。
  /// @return 音频帧数据大小，单位：字节。
  ///

  FutureOr<int> data_size() async {
    return await nativeCall('data_size', []);
  }

  /// @detail api
  /// @brief 获取音频帧类型，目前只支持 PCM，参看 AudioFrameType{@link #AudioFrameType}。
  /// @return 音频帧类型
  ///

  FutureOr<AudioFrameType> frame_type() async {
    return await nativeCall('frame_type', []);
  }

  /// @detail api
  /// @hidden for internal use only
  /// @brief 获取音频帧额外信息内存块地址
  /// @return 音频帧额外信息内存块地址
  ///

  FutureOr<ByteBuffer> getExtraInfo() async {
    return await nativeCall('getExtraInfo', []);
  }

  /// @detail api
  /// @hidden for internal use only
  /// @brief 获取音频帧额外信息大小
  /// @return 音频帧数据额外信息大小，单位：字节。
  ///

  FutureOr<int> extraInfoSize() async {
    return await nativeCall('extraInfoSize', []);
  }

  /// @detail api
  /// @brief 释放音频帧。
  ///

  FutureOr<void> release() async {
    return await nativeCall('release', []);
  }
}

enum MirrorType {
  /// @brief 本地预览和编码传输时均无镜像效果
  ///
  MIRROR_TYPE_NONE(0),

  /// @brief 本地预览时有镜像效果，编码传输时无镜像效果
  ///
  MIRROR_TYPE_RENDER(1),

  /// @brief 本地预览时无镜像效果，仅编码传输时有镜像效果
  ///
  MIRROR_TYPE_ENCODER(2),

  /// @brief 本地预览和编码传输时均有镜像效果
  ///
  MIRROR_TYPE_RENDER_AND_ENCODER(3);

  final dynamic $value;
  const MirrorType([this.$value]);
}

enum TorchState {
  /// @brief 关闭
  ///
  TORCH_STATE_OFF(0),

  /// @brief 打开
  ///
  TORCH_STATE_ON(1);

  final dynamic $value;
  const TorchState([this.$value]);
}

class EchoTestConfig extends NativeClass {
  static const _$namespace = r'com.ss.bytertc.engine.data.EchoTestConfig';
  static get codegen_$namespace => _$namespace;

  EchoTestConfig([NativeClassOptions? options])
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
  ///
  FutureOr<View?> get view async {
    return await sendInstanceGet<View?>("view");
  }

  set view(FutureOr<View?> value) {
    sendInstanceSet("view", value);
  }

  /// @brief 进行音视频通话回路测试的用户 ID
  ///
  FutureOr<String?> get uid async {
    return await sendInstanceGet<String?>("uid");
  }

  set uid(FutureOr<String?> value) {
    sendInstanceSet("uid", value);
  }

  /// @brief 测试用户加入的房间 ID
  ///
  FutureOr<String?> get roomId async {
    return await sendInstanceGet<String?>("roomId");
  }

  set roomId(FutureOr<String?> value) {
    sendInstanceSet("roomId", value);
  }

  /// @brief 对用户进房时进行鉴权验证的动态密钥，用于保证音视频通话回路测试的安全性。
  ///
  FutureOr<String?> get token async {
    return await sendInstanceGet<String?>("token");
  }

  set token(FutureOr<String?> value) {
    sendInstanceSet("token", value);
  }

  /// @brief 是否检测音频。检测设备为系统默认音频设备。 <br>
  ///        - true：是
  ///            - 若使用 SDK 内部采集，此时设备麦克风会自动开启，并在 audioReportInterval 值大于 0 时触发 `onLocalAudioPropertiesReport` 回调，你可以根据该回调判断麦克风的工作状态
  ///            - 若使用自定义采集，此时你需调用 pushExternalAudioFrame{@link #RTCEngine#pushExternalAudioFrame} 将采集到的音频推送给 SDK
  ///        - false：否
  ///
  FutureOr<boolean?> get enableAudio async {
    return await sendInstanceGet<boolean?>("enableAudio");
  }

  set enableAudio(FutureOr<boolean?> value) {
    sendInstanceSet("enableAudio", value);
  }

  /// @brief 是否检测视频。PC 端默认检测列表中第一个视频设备。 <br>
  ///        - true：是
  ///            - 若使用 SDK 内部采集，此时设备摄像头会自动开启
  ///            - 若使用自定义采集，此时你需调用 pushExternalVideoFrame{@link #RTCEngine#pushExternalVideoFrame} 将采集到的视频推送给 SDK
  ///        - false：否
  /// @note 视频的发布参数固定为：分辨率 640px × 360px，帧率 15fps。
  ///
  FutureOr<boolean?> get enableVideo async {
    return await sendInstanceGet<boolean?>("enableVideo");
  }

  set enableVideo(FutureOr<boolean?> value) {
    sendInstanceSet("enableVideo", value);
  }

  /// @brief 音量信息提示间隔，单位：ms，默认为 100ms <br>
  ///       - `<= 0`: 关闭信息提示
  ///       - `(0,100]`: 开启信息提示，不合法的 interval 值，SDK 自动设置为 100ms
  ///       - `> 100`: 开启信息提示，并将信息提示间隔设置为此值
  ///
  FutureOr<int?> get audioReportInterval async {
    return await sendInstanceGet<int?>("audioReportInterval");
  }

  set audioReportInterval(FutureOr<int?> value) {
    sendInstanceSet("audioReportInterval", value);
  }

  FutureOr<View> getEchoRenderView() async {
    return await nativeCall('getEchoRenderView', []);
  }

  FutureOr<String> getEchoUid() async {
    return await nativeCall('getEchoUid', []);
  }

  FutureOr<String> getEchoRoomId() async {
    return await nativeCall('getEchoRoomId', []);
  }

  FutureOr<String> getEchoToken() async {
    return await nativeCall('getEchoToken', []);
  }

  FutureOr<boolean> getEchoEnabledAudio() async {
    return await nativeCall('getEchoEnabledAudio', []);
  }

  FutureOr<boolean> getEchoEnabledVideo() async {
    return await nativeCall('getEchoEnabledVideo', []);
  }

  FutureOr<int> getAudioReportInterval() async {
    return await nativeCall('getAudioReportInterval', []);
  }
}

enum VoiceReverbType {
  /// @brief 原声，不含特效
  ///
  VOICE_REVERB_ORIGINAL(0),

  /// @brief 回声
  ///
  VOICE_REVERB_ECHO(1),

  /// @brief 演唱会
  ///
  VOICE_REVERB_CONCERT(2),

  /// @brief 空灵
  ///
  VOICE_REVERB_ETHEREAL(3),

  /// @brief KTV
  ///
  VOICE_REVERB_KTV(4),

  /// @brief 录音棚
  ///
  VOICE_REVERB_STUDIO(5),

  /// @brief 虚拟立体声
  ///
  VOICE_REVERB_VIRTUAL_STEREO(6),

  /// @brief 空旷
  ///
  VOICE_REVERB_SPACIOUS(7),

  /// @brief 3D 人声
  ///
  VOICE_REVERB_3D(8),

  /// @hidden internal use
  /// @brief 流行
  ///
  VOICE_REVERB_POP(9),

  /// @hidden internal use
  /// @brief 蹦迪
  ///
  VOICE_REVERB_DISCO(10),

  /// @hidden internal use
  /// @brief 老唱片
  ///
  VOICE_REVERB_OLDRECORD(11),

  /// @hidden internal use
  /// @brief 和声
  ///
  VOICE_REVERB_HARMONY(12),

  /// @hidden internal use
  /// @brief 摇滚
  ///
  VOICE_REVERB_ROCK(13),

  /// @hidden internal use
  /// @brief 蓝调
  ///
  VOICE_REVERB_BLUES(14),

  /// @hidden internal use
  /// @brief 爵士
  ///
  VOICE_REVERB_JAZZ(15),

  /// @hidden internal use
  /// @brief 电子
  ///
  VOICE_REVERB_ELECTRONIC(16),

  /// @hidden internal use
  /// @brief 黑胶
  ///
  VOICE_REVERB_VINYL(17),

  /// @hidden internal use
  /// @brief 密室
  ///
  VOICE_REVERB_CHAMBER(18),

  /// @hidden for internal use only
  /// @brief 增强原声
  ///
  VOICE_REVERB_ENHANCE_ORIGINAL(19),

  /// @hidden for internal use only
  /// @brief 浴室
  ///
  VOICE_REVERB_BATHROOM(20),

  /// @hidden for internal use only
  /// @brief 自然
  ///
  VOICE_REVERB_NATURAL(21),

  /// @hidden for internal use only
  /// @brief 楼道
  ///
  VOICE_REVERB_HALLWAY(22);

  final dynamic $value;
  const VoiceReverbType([this.$value]);
}

enum SingleStreamPushType {
  /// @brief 转推到 CDN
  ///
  SINGLE_STREAM_PUSH_TYPE_TO_CDN(1),

  /// @brief 转推到 RTC 房间
  ///
  SINGLE_STREAM_PUSH_TYPE_TO_RTC(2);

  final dynamic $value;
  const SingleStreamPushType([this.$value]);
}

enum PlayerError {
  /// @brief 正常
  ///
  OK(0),

  /// @brief 不支持此类型
  ///
  FORMAT_NOT_SUPPORT(1),

  /// @brief 无效的播放路径
  ///
  INVALID_PATH(2),

  /// @brief 未满足前序接口调用的要求。请查看具体接口文档。
  ///
  INVALID_STATE(3),

  /// @brief 设置播放位置出错。
  ///
  INVALID_POSITION(4),

  /// @brief 音量参数不合法。
  ///
  INVALID_VOLUME(5),

  /// @brief 音调参数设置不合法。
  ///
  INVALID_PITCH(6),

  /// @brief 音轨参数设置不合法。
  ///
  INVALID_AUDIO_TRACK_INDEX(7),

  /// @brief 播放速度参数设置不合法
  ///
  INVALID_PLAYBACK_SPEED(8),

  /// @brief 音效 ID 异常。还未加载或播放文件，就调用其他 API。
  ///
  INVALID_EFFECT_ID(9),

  /// @brief 资源正在播放中
  ///
  ERROR_CURRENTLY_PLAYING(10);

  final dynamic $value;
  const PlayerError([this.$value]);
}

enum PerformanceAlarmMode {
  /// @brief 未开启发布性能回退
  ///
  NORMAL(0),

  /// @brief 已开启发布性能回退
  ///
  SIMULCAST(1);

  final dynamic $value;
  const PerformanceAlarmMode([this.$value]);
}

enum StreamRemoveReason {
  /// @brief 远端用户停止发布流。
  ///
  STREAM_REMOVE_REASON_UNPUBLISH(0),

  /// @brief 远端用户发布流失败。
  ///
  STREAM_REMOVE_REASON_PUBLISH_FAILED(1),

  /// @brief 媒体服务器 10s 没收到客户端的媒体数据。
  ///
  STREAM_REMOVE_REASON_KEEP_LIVE_FAILED(2),

  /// @brief 远端用户断网。
  ///
  STREAM_REMOVE_REASON_CLIENT_DISCONNECTED(3),

  /// @brief 远端用户重新发布流。
  ///
  STREAM_REMOVE_REASON_REPUBLISH(4),

  /// @brief 其他原因。
  ///
  STREAM_REMOVE_REASON_OTHER(5),

  /// @brief 远端用户 Token 发布权限过期。
  ///
  STREAM_REMOVE_REASON_PUBLISH_PRIVILEGE_TOKEN_EXPIRED(6);

  final dynamic $value;
  const StreamRemoveReason([this.$value]);
}

enum ChannelProfile {
  /// @brief 普通音频通话，默认模式 <br>
  ///        与 `CHANNEL_PROFIEL_MEETING(16)` 配置相同。 <br>
  ///        你可以联系技术支持更换默认配置参数。
  ///
  CHANNEL_PROFILE_COMMUNICATION(0),

  /// @brief 游戏语音模式，低功耗、低流量消耗。 <br>
  ///        低端机在此模式下运行时，进行了额外的性能优化： <br>
  ///            - 部分低端机型配置编码帧长 40/60
  ///            - 部分低端机型关闭软件 3A 音频处理
  ///        增强对 iOS 其他屏幕录制进行的兼容性，避免音频录制被 RTC 打断。
  ///
  CHANNEL_PROFILE_GAME(2),

  /// @brief 云游戏模式。 <br>
  ///        如果你的游戏场景需要低延迟的配置，使用此设置。 <br>
  ///        此设置下，弱网抗性较差。
  ///
  CHANNEL_PROFILE_CLOUD_GAME(3),

  /// @brief 云渲染模式。超低延时配置。 <br>
  ///        如果你的应用并非游戏但又对延时要求较高时，选用此模式 <br>
  ///        该模式下，音视频通话延时会明显降低，但同时弱网抗性、通话音质等均会受到一定影响。
  ///
  CHANNEL_PROFILE_LOW_LATENCY(4),

  /// @brief 适用于 3 人及以上纯语音通话。 <br>
  ///        通话中，闭麦时为是媒体模式，上麦后切换为通话模式。
  ///
  CHANNEL_PROFILE_CHAT_ROOM(6),

  /// @brief 适用于单主播和观众进行音视频互动的直播。通话模式，上下麦不会有模式切换，避免音量突变现象
  ///
  CHANNEL_PROFILE_INTERACTIVE_PODCAST(10),

  /// @brief 适合在线实时合唱场景，高音质，超低延迟。使用本配置前请联系技术支持进行协助完成其他配置。
  ///
  CHANNEL_PROFILE_CHORUS(12),

  /// @brief 适用于 1 vs 1 游戏串流，支持公网或局域网。
  ///
  CHANNEL_PROFILE_GAME_STREAMING(14),

  /// @brief 适用于云端会议中的个人设备。
  ///
  CHANNEL_PROFIEL_MEETING(16),

  /// @brief 适用于云端会议中的会议室终端设备，例如 Rooms，投屏盒子等。
  ///
  CHANNEL_PROFILE_MEETING_ROOM(17),

  /// @brief 适用于课堂互动，房间内所有成员都可以进行音视频互动 <br>
  ///        当你的场景中需要同时互动的成员超过 10 人时使用此模式
  ///
  CHANNEL_PROFILE_CLASSROOM(18),

  /// @brief 注重流畅性，缺省码率相对低。适用于通话。
  ///
  CHANNEL_PROFILE_CALL(19),

  /// @brief 更注重画质，视频缺省码率相对高。适用于直播互动。
  ///
  CHANNEL_PROFILE_LIVE(20);

  final dynamic $value;
  const ChannelProfile([this.$value]);
}

class VirtualBackgroundSource extends NativeClass {
  static const _$namespace =
      r'com.ss.bytertc.engine.data.VirtualBackgroundSource';
  static get codegen_$namespace => _$namespace;

  VirtualBackgroundSource([NativeClassOptions? options])
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

  /// @brief 虚拟背景类型，详见 VirtualBackgroundSourceType{@link #VirtualBackgroundSourceType} 。
  ///
  FutureOr<VirtualBackgroundSourceType?> get sourceType async {
    try {
      final result =
          await sendInstanceGet<VirtualBackgroundSourceType?>("sourceType");
      if (result == null) {
        return null;
      }
      return VirtualBackgroundSourceType.values
          .firstWhere((element) => element == result);
    } catch (e) {
      return null;
    }
  }

  set sourceType(FutureOr<VirtualBackgroundSourceType?> value) {
    sendInstanceSet("sourceType", value);
  }

  /// @brief 纯色背景使用的颜色。 <br>
  ///        格式为 0xAARRGGBB 。
  ///
  FutureOr<int?> get sourceColor async {
    return await sendInstanceGet<int?>("sourceColor");
  }

  set sourceColor(FutureOr<int?> value) {
    sendInstanceSet("sourceColor", value);
  }

  /// @brief 自定义背景图片的绝对路径。 <br>
  ///       - 支持本地文件绝对路径 (file://xxx) 和 Asset 资源路径 (asset://xxx)。
  ///       - 支持的格式为 jpg、jpeg、png。
  ///       - 图片分辨率超过 1080P 时，图片会被等比缩放至和视频一致。
  ///       - 图片和视频宽高比一致时，图片会被直接缩放至和视频一致。
  ///       - 图片和视频长宽比不一致时，为保证图片内容不变形，图片按短边缩放至与视频帧一致，使图片填满视频帧，对多出的高或宽进行剪裁。
  ///       - 自定义图片带有局部透明效果时，透明部分由黑色代替。
  ///
  FutureOr<String?> get sourcePath async {
    return await sendInstanceGet<String?>("sourcePath");
  }

  set sourcePath(FutureOr<String?> value) {
    sendInstanceSet("sourcePath", value);
  }
}

enum KTVPlayerErrorCode {
  /// @brief 成功。
  ///
  OK(0),

  /// @brief 播放错误，请下载后播放。
  ///
  FILE_NOT_EXIST(-3020),

  /// @brief 播放错误，请确认文件播放格式。
  ///
  FILE_ERROR(-3021),

  /// @brief 播放错误，未进入房间。
  ///
  NOT_JOIN_ROOM(-3022),

  /// @brief 参数错误。
  ///
  PARAM(-3023),

  /// @brief 播放失败，找不到文件或文件打开失败。
  ///
  START_ERROR(-3024),

  /// @brief 混音 ID 异常。
  ///
  MIX_ID_ERROR(-3025),

  /// @brief 设置播放位置出错。
  ///
  POSITION_ERROR(-3026),

  /// @brief 音量参数不合法，可设置的取值范围为 [0,400]。
  ///
  AUDIO_VOLUME_ERROR(-3027),

  /// @brief 不支持此混音类型。
  ///
  TYPE_ERROR(-3028),

  /// @brief 音调文件不合法。
  ///
  PITCH_ERROR(-3029),

  /// @brief 音轨不合法。
  ///
  AUDIO_TRACK_ERROR(-3030),

  /// @brief 混音启动中。
  ///
  STARTING_ERROR(-3031);

  final dynamic $value;
  const KTVPlayerErrorCode([this.$value]);
}

enum AudioMixingDualMonoMode {
  /// @brief 和音频文件一致
  ///
  AUDIO_MIXING_DUAL_MONO_MODE_AUTO(0),

  /// @brief 只能听到音频文件中左声道的音频
  ///
  AUDIO_MIXING_DUAL_MONO_MODE_L(1),

  /// @brief 只能听到音频文件中右声道的音频
  ///
  AUDIO_MIXING_DUAL_MONO_MODE_R(2),

  /// @brief 能同时听到音频文件中左右声道的音频
  ///
  AUDIO_MIXING_DUAL_MONO_MODE_MIX(3);

  final dynamic $value;
  const AudioMixingDualMonoMode([this.$value]);
}

class VideoEncoderConfig$VideoDimensions extends NativeClass {
  static const _$namespace =
      r'com.ss.bytertc.engine.VideoEncoderConfig.VideoDimensions';
  static get codegen_$namespace => _$namespace;

  VideoEncoderConfig$VideoDimensions([NativeClassOptions? options])
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

  FutureOr<int?> get width async {
    return await sendInstanceGet<int?>("width");
  }

  set width(FutureOr<int?> value) {
    sendInstanceSet("width", value);
  }

  FutureOr<int?> get height async {
    return await sendInstanceGet<int?>("height");
  }

  set height(FutureOr<int?> value) {
    sendInstanceSet("height", value);
  }
}

class RecordingInfo extends NativeClass {
  static const _$namespace = r'com.ss.bytertc.engine.data.RecordingInfo';
  static get codegen_$namespace => _$namespace;

  RecordingInfo([NativeClassOptions? options])
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
  ///
  FutureOr<String?> get filePath async {
    return await sendInstanceGet<String?>("filePath");
  }

  set filePath(FutureOr<String?> value) {
    sendInstanceSet("filePath", value);
  }

  /// @brief 录制文件的视频编码类型，参看 VideoCodecType{@link #RecordingInfo-VideoCodecType}。
  ///
  FutureOr<VideoCodecType?> get videoCodecType async {
    try {
      final result = await sendInstanceGet<VideoCodecType?>("videoCodecType");
      if (result == null) {
        return null;
      }
      return VideoCodecType.values.firstWhere((element) => element == result);
    } catch (e) {
      return null;
    }
  }

  set videoCodecType(FutureOr<VideoCodecType?> value) {
    sendInstanceSet("videoCodecType", value);
  }

  /// @brief 录制视频的宽，单位：像素。纯音频录制请忽略该字段
  ///
  FutureOr<int?> get width async {
    return await sendInstanceGet<int?>("width");
  }

  set width(FutureOr<int?> value) {
    sendInstanceSet("width", value);
  }

  /// @brief 录制视频的高，单位：像素。纯音频录制请忽略该字段
  ///
  FutureOr<int?> get height async {
    return await sendInstanceGet<int?>("height");
  }

  set height(FutureOr<int?> value) {
    sendInstanceSet("height", value);
  }
}

enum AudioScenarioType {
  /// @brief 默认场景，适用大部分场景。
  ///
  DEFAULT(0),

  /// @brief 聊天室场景。通话清晰度较高，适用于会议，聊天室场景。
  ///
  CHATROOM(1),

  /// @brief 游戏语音场景。
  ///
  GAMESTREAMING(2),

  /// @brief 合唱场景。延迟较低。
  ///
  CHORUS(3),

  /// @brief 教育场景。适用于以人声教学内容为主的高音质场景。
  ///
  EDUCATION(4),

  /// @brief AI 对话场景。适用于真人与 AI 智能体互动的场景。
  ///
  AICLIENT(5);

  final dynamic $value;
  const AudioScenarioType([this.$value]);
}

enum SubtitleState {
  /// @brief 开启字幕。
  ///
  SUBTITLE_STATE_STARTED(0),

  /// @brief 关闭字幕。
  ///
  SUBTITLE_STATE_STOPED(1),

  /// @brief 字幕任务出现错误。
  ///
  SUBTITLE_STATE_ERROR(2);

  final dynamic $value;
  const SubtitleState([this.$value]);
}

enum EffectErrorType {
  /// @brief 特效无错误。
  ///
  OK(0),

  /// @hidden 仅用于会议
  /// @brief 虚拟背景设置错误。
  ///
  EFFECT_ERROR_VIRTUAL_BACKFROUND_FAILURE(1),

  /// @hidden 仅用于会议
  /// @brief 特效独立进程崩溃。
  ///
  EFFECT_ERROR_CHILD_PROC_TERMINATE(2);

  final dynamic $value;
  const EffectErrorType([this.$value]);
}

enum SubtitleMode {
  /// @brief 识别模式。在此模式下，房间内用户语音会被转为文字。
  ///
  SUBTITLE_MODE_RECOGINTE(0),

  /// @brief 翻译模式。在此模式下，房间内用户语音会先被转为文字，再被翻译为目标语言。
  ///
  SUBTITLE_MODE_TRANSLATION(1);

  final dynamic $value;
  const SubtitleMode([this.$value]);
}

enum AudioPropertiesMode {
  /// @brief 仅包含本地麦克风采集的音频数据和本地屏幕音频采集数据。
  ///
  AUDIO_PROPERTIES_MODE_MICROPHONE(0),

  /// @brief 包含以下信息： <br>
  ///        - 本地麦克风采集的音频数据和本地屏幕音频采集数据；
  ///        - 本地混音的音频数据。
  ///
  AUDIO_PROPERTIES_MODE_AUDIOMIXING(1);

  final dynamic $value;
  const AudioPropertiesMode([this.$value]);
}

class LocalVideoSinkConfig extends NativeClass {
  static const _$namespace =
      r'com.ss.bytertc.engine.video.LocalVideoSinkConfig';
  static get codegen_$namespace => _$namespace;

  LocalVideoSinkConfig([NativeClassOptions? options])
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

  /// @brief 本地视频帧回调位置，参看 LocalVideoSinkPosition{@link #LocalVideoSinkPosition}，默认回调前处理后的视频帧。
  ///
  FutureOr<LocalVideoSinkPosition?> get position async {
    try {
      final result = await sendInstanceGet<LocalVideoSinkPosition?>("position");
      if (result == null) {
        return null;
      }
      return LocalVideoSinkPosition.values
          .firstWhere((element) => element == result);
    } catch (e) {
      return null;
    }
  }

  set position(FutureOr<LocalVideoSinkPosition?> value) {
    sendInstanceSet("position", value);
  }

  /// @brief 本地视频帧回调格式，参看 VideoPixelFormat{@link #VideoPixelFormat}，默认值为 0。
  ///
  FutureOr<VideoPixelFormat?> get pixelFormat async {
    try {
      final result = await sendInstanceGet<VideoPixelFormat?>("pixelFormat");
      if (result == null) {
        return null;
      }
      return VideoPixelFormat.values.firstWhere((element) => element == result);
    } catch (e) {
      return null;
    }
  }

  set pixelFormat(FutureOr<VideoPixelFormat?> value) {
    sendInstanceSet("pixelFormat", value);
  }
}

class ForwardStreamStateInfo extends NativeClass {
  static const _$namespace =
      r'com.ss.bytertc.engine.data.ForwardStreamStateInfo';
  static get codegen_$namespace => _$namespace;

  ForwardStreamStateInfo([NativeClassOptions? options])
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
  FutureOr<String?> get roomId async {
    return await sendInstanceGet<String?>("roomId");
  }

  set roomId(FutureOr<String?> value) {
    sendInstanceSet("roomId", value);
  }

  /// @brief 跨房间转发媒体流过程中该目标房间的状态，参看 ForwardStreamState{@link #ForwardStreamState}
  ///
  FutureOr<ForwardStreamState?> get state async {
    try {
      final result = await sendInstanceGet<ForwardStreamState?>("state");
      if (result == null) {
        return null;
      }
      return ForwardStreamState.values
          .firstWhere((element) => element == result);
    } catch (e) {
      return null;
    }
  }

  set state(FutureOr<ForwardStreamState?> value) {
    sendInstanceSet("state", value);
  }

  /// @brief 跨房间转发媒体流过程中该目标房间抛出的错误码，参看 ForwardStreamError{@link #ForwardStreamError}
  ///
  FutureOr<ForwardStreamError?> get error async {
    try {
      final result = await sendInstanceGet<ForwardStreamError?>("error");
      if (result == null) {
        return null;
      }
      return ForwardStreamError.values
          .firstWhere((element) => element == result);
    } catch (e) {
      return null;
    }
  }

  set error(FutureOr<ForwardStreamError?> value) {
    sendInstanceSet("error", value);
  }
}

class RoomEventInfo extends NativeClass {
  static const _$namespace = r'com.ss.bytertc.engine.type.RoomEventInfo';
  static get codegen_$namespace => _$namespace;

  RoomEventInfo([NativeClassOptions? options])
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
  FutureOr<long?> get forbiddenTime async {
    return await sendInstanceGet<long?>("forbiddenTime");
  }

  set forbiddenTime(FutureOr<long?> value) {
    sendInstanceSet("forbiddenTime", value);
  }
}

class PushSingleStreamParam extends NativeClass {
  static const _$namespace =
      r'com.ss.bytertc.engine.live.PushSingleStreamParam';
  static get codegen_$namespace => _$namespace;

  PushSingleStreamParam([NativeClassOptions? options])
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

  /// @brief 媒体流所在的房间 ID
  ///
  FutureOr<String?> get roomId async {
    return await sendInstanceGet<String?>("roomId");
  }

  set roomId(FutureOr<String?> value) {
    sendInstanceSet("roomId", value);
  }

  /// @brief 媒体流所属的用户 ID
  ///
  FutureOr<String?> get userId async {
    return await sendInstanceGet<String?>("userId");
  }

  set userId(FutureOr<String?> value) {
    sendInstanceSet("userId", value);
  }

  /// @brief 推流 CDN 地址。仅支持 RTMP 协议，Url 必须满足正则 `/^rtmps?:\\/\\//`。 <br>
  ///        本参数不支持过程中更新。
  ///
  FutureOr<String?> get url async {
    return await sendInstanceGet<String?>("url");
  }

  set url(FutureOr<String?> value) {
    sendInstanceSet("url", value);
  }

  /// @brief 媒体流是否为屏幕流。
  ///
  FutureOr<boolean?> get isScreen async {
    return await sendInstanceGet<boolean?>("isScreen");
  }

  set isScreen(FutureOr<boolean?> value) {
    sendInstanceSet("isScreen", value);
  }

  /// @brief 跨房间转发的目标房间信息数组，默认值为 null。<br>
  ///        当你需要将当前房间的媒体流转发到其他房间时，可以通过这个列表指定多个目标房间的信息。<br>
  ///        每个 DestInfo 包含目标房间的 roomId 和 userId。详见 DestInfo{@link DestInfo}。
  ///
  FutureOr<List<DestInfo>?> get destInfos async {
    try {
      final result = await sendInstanceGet<List<DestInfo>?>("destInfos");
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

  set destInfos(FutureOr<List<DestInfo>?> value) {
    sendInstanceSet("destInfos", value);
  }

  /// @brief 单流转推类型，指定将媒体流转推到 CDN 还是 RTC 房间，默认值为转推到 CDN。参见 SingleStreamPushType{@link SingleStreamPushType}。
  ///
  FutureOr<SingleStreamPushType?> get pushType async {
    try {
      final result = await sendInstanceGet<SingleStreamPushType?>("pushType");
      if (result == null) {
        return null;
      }
      return SingleStreamPushType.values
          .firstWhere((element) => element == result);
    } catch (e) {
      return null;
    }
  }

  set pushType(FutureOr<SingleStreamPushType?> value) {
    sendInstanceSet("pushType", value);
  }
}

enum SubscribeFallbackOptions {
  /// @brief 下行网络不佳或设备性能不足时，不对音视频流作回退处理。默认设置。
  ///
  SUBSCRIBE_FALLBACK_OPTIONS_DISABLED(0),

  /// @brief 下行网络不佳或设备性能不足时，对视频流做降级处理，具体降级规则参看[音视频流回退](#70137)。 <br>
  ///        该设置仅对发布端调用 setLocalSimulcastMode{@link #RTCEngine#setlocalsimulcastmode-2} 开启发送多路流功能的情况生效。
  ///
  SUBSCRIBE_FALLBACK_OPTIONS_STREAM_LOW(1),

  /// @brief 下行网络不佳或设备性能不足时，先对视频流做回退处理。当网络状况不满足接收弱流时，则自动取消接收视频，仅接收音频。 <br>
  ///        该设置仅对发布端调用 setLocalSimulcastMode{@link #RTCEngine#setlocalsimulcastmode-2} 开启发送多路流功能的情况生效。
  ///
  SUBSCRIBE_FALLBACK_OPTIONS_AUDIO_ONLY(2);

  final dynamic $value;
  const SubscribeFallbackOptions([this.$value]);
}

class MixedStreamLayoutRegionImageWaterMarkConfig extends NativeClass {
  static const _$namespace =
      r'com.ss.bytertc.engine.live.MixedStreamLayoutRegionImageWaterMarkConfig';
  static get codegen_$namespace => _$namespace;

  MixedStreamLayoutRegionImageWaterMarkConfig([NativeClassOptions? options])
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

  /// @brief 设置原始图片的宽度。
  /// @param imageWidth 原始图片宽度，单位为 px。
  ///
  FutureOr<int?> get imageWidth async {
    return await sendInstanceGet<int?>("imageWidth");
  }

  set imageWidth(FutureOr<int?> value) {
    sendInstanceSet("imageWidth", value);
  }

  /// @brief 设置原始图片的高度。
  /// @param imageHeight 原始图片高度，单位为 px。
  ///
  FutureOr<int?> get imageHeight async {
    return await sendInstanceGet<int?>("imageHeight");
  }

  set imageHeight(FutureOr<int?> value) {
    sendInstanceSet("imageHeight", value);
  }
}

enum NetworkQuality {
  /// @detail keytype
  /// @brief 媒体流网络质量未知。
  ///
  NETWORK_QUALITY_UNKNOWN(0),

  /// @detail keytype
  /// @brief 媒体流网络质量极好。
  ///
  NETWORK_QUALITY_EXCELLENT(1),

  /// @detail keytype
  /// @brief 媒体流网络质量好。
  ///
  NETWORK_QUALITY_GOOD(2),

  /// @detail keytype
  /// @brief 媒体流网络质量较差但不影响沟通。
  ///
  NETWORK_QUALITY_POOR(3),

  /// @detail keytype
  /// @brief 媒体流网络质量差沟通不顺畅。
  ///
  NETWORK_QUALITY_BAD(4),

  /// @detail keytype
  /// @brief 媒体流网络质量非常差。
  ///
  NETWORK_QUALITY_VERY_BAD(5),

  /// @detail keytype
  /// @brief 网络连接断开，无法通话。网络可能由于 12s 内无应答、开启飞行模式、拔掉网线等原因断开。 <br>
  ///        更多网络状态信息参见 [连接状态提示](https://www.volcengine.com/docs/6348/95376)。
  ///
  NETWORK_QUALITY_DOWN(6);

  final dynamic $value;
  const NetworkQuality([this.$value]);
}

enum RecordingState {
  /// @brief 录制异常
  ///
  RECORDING_STATE_ERROE(0),

  /// @brief 录制进行中
  ///
  RECORDING_STATE_PROCESSING(1),

  /// @brief 录制文件保存成功，调用 `stopFileRecording` 结束录制之后才会收到该状态码。
  ///
  RECORDING_STATE_SUCCESS(2);

  final dynamic $value;
  const RecordingState([this.$value]);
}

enum AudioRoute {
  /// @brief 默认设备。通过 `setDefaultAudioRoute` 设置的音频路由。
  ///
  AUDIO_ROUTE_DEFAULT(-1),

  /// @brief 有线耳机
  ///
  AUDIO_ROUTE_HEADSET(1),

  /// @brief 听筒。设备自带的，一般用于通话的播放硬件。
  ///
  AUDIO_ROUTE_EARPIECE(2),

  /// @brief 扬声器。设备自带的，一般用于免提播放的硬件。
  ///
  AUDIO_ROUTE_SPEAKERPHONE(3),

  /// @brief 蓝牙耳机
  ///
  AUDIO_ROUTE_HEADSET_BLUETOOTH(4),

  /// @brief USB 设备
  ///
  AUDIO_ROUTE_HEADSET_USB(5);

  final dynamic $value;
  const AudioRoute([this.$value]);
}

enum LocalAudioStreamError {
  /// @brief 本地音频状态正常
  ///
  LOCAL_AUDIO_STREAM_ERROR_OK(0),

  /// @brief 本地音频出错原因未知
  ///
  LOCAL_AUDIO_STREAM_ERROR_FAILURE(1),

  /// @brief 没有权限启动本地音频录制设备
  ///
  LOCAL_AUDIO_STREAM_ERROR_DEVICE_NO_PERMISSION(2),

  /// @hidden currently not available
  /// @brief 本地音频录制设备已经在使用中
  /// @note 该错误码暂未使用
  ///
  LOCAL_AUDIO_STREAM_ERROR_DEVICE_BUSY(3),

  /// @brief 本地音频录制失败，建议你检查录制设备是否正常工作
  ///
  LOCAL_AUDIO_STREAM_ERROR_RECORD_FAILURE(4),

  /// @brief 本地音频编码失败
  ///
  LOCAL_AUDIO_STREAM_ERROR_ENCODE_FAILURE(5),

  /// @brief 没有可用的音频录制设备
  ///
  LOCAL_AUDIO_STREAM_ERROR_NO_RECORDING_DEVICE(6);

  final dynamic $value;
  const LocalAudioStreamError([this.$value]);
}

class SubtitleMessage extends NativeClass {
  static const _$namespace = r'com.ss.bytertc.engine.type.SubtitleMessage';
  static get codegen_$namespace => _$namespace;

  static FutureOr<SubtitleMessage> create(String userId, String text,
      String language, int mode, int sequence, boolean definite) async {
    final result = await NativeClassUtils.nativeStaticCall(
      _$namespace,
      'create',
      [userId, text, language, mode, sequence, definite],
      'com.volcengine.rtc.hybrid_runtime',
    );
    return packObject(result,
        () => SubtitleMessage(const NativeClassOptions([], disableInit: true)));
  }

  SubtitleMessage([NativeClassOptions? options])
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
  ///
  FutureOr<String?> get userId async {
    return await sendInstanceGet<String?>("userId");
  }

  set userId(FutureOr<String?> value) {
    sendInstanceSet("userId", value);
  }

  /// @brief 字幕文本, 采用 UTF-8 编码。
  ///
  FutureOr<String?> get text async {
    return await sendInstanceGet<String?>("text");
  }

  set text(FutureOr<String?> value) {
    sendInstanceSet("text", value);
  }

  /// @brief 字幕语种，根据字幕模式为原文或译文对应的语种。
  ///
  FutureOr<String?> get language async {
    return await sendInstanceGet<String?>("language");
  }

  set language(FutureOr<String?> value) {
    sendInstanceSet("language", value);
  }

  /// @brief 字幕模式，参看 SubtitleMode{@link #SubtitleMode}。
  ///
  FutureOr<SubtitleMode?> get mode async {
    try {
      final result = await sendInstanceGet<SubtitleMode?>("mode");
      if (result == null) {
        return null;
      }
      return SubtitleMode.values.firstWhere((element) => element == result);
    } catch (e) {
      return null;
    }
  }

  set mode(FutureOr<SubtitleMode?> value) {
    sendInstanceSet("mode", value);
  }

  /// @brief 字幕文本序列号，同一发言人的完整发言和不完整发言会按递增顺序单独分别编号。
  ///
  FutureOr<int?> get sequence async {
    return await sendInstanceGet<int?>("sequence");
  }

  set sequence(FutureOr<int?> value) {
    sendInstanceSet("sequence", value);
  }

  /// @brief 语音识别出的文本是否为一段完整的一句话。 True 代表是, False 代表不是。
  ///
  FutureOr<boolean?> get definite async {
    return await sendInstanceGet<boolean?>("definite");
  }

  set definite(FutureOr<boolean?> value) {
    sendInstanceSet("definite", value);
  }
}

enum MuteState {
  /// @brief 发送
  ///
  MUTE_STATE_OFF(0),

  /// @brief 停止发送
  ///
  MUTE_STATE_ON(1);

  final dynamic $value;
  const MuteState([this.$value]);
}

enum VideoDeviceType {
  /// @brief 未知设备类型
  ///
  VIDEO_DEVICE_TYPE_UNKNOWN(-1),

  /// @brief 视频渲染设备类型
  ///
  VIDEO_DEVICE_TYPE_RENDER_DEVICE(0),

  /// @brief 视频采集设备类型
  ///
  VIDEO_DEVICE_TYPE_CAPTURE_DEVICE(1),

  /// @brief 屏幕流视频设备
  ///
  VIDEO_DEVICE_TYPE_SCREEN_CAPTURE_DEVICE(2);

  final dynamic $value;
  const VideoDeviceType([this.$value]);
}

enum PublishState {
  /// @brief 发布成功
  ///
  PUBLISHED(0),

  /// @brief 发布失败
  ///
  UNPUBLISHED(1);

  final dynamic $value;
  const PublishState([this.$value]);
}

class RTCEncodedVideoFrame extends NativeClass {
  static const _$namespace =
      r'com.ss.bytertc.engine.mediaio.RTCEncodedVideoFrame';
  static get codegen_$namespace => _$namespace;

  RTCEncodedVideoFrame([NativeClassOptions? options])
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

  /// @brief 视频帧数据指针地址 <br>
  ///        buffer 为 Direct 类型且 size 需和 capacity 相等
  ///
  FutureOr<ByteBuffer?> get buffer async {
    return await sendInstanceGet<ByteBuffer?>("buffer");
  }

  set buffer(FutureOr<ByteBuffer?> value) {
    sendInstanceSet("buffer", value);
  }

  /// @brief 视频采集时间戳，单位：微秒
  ///
  FutureOr<long?> get timestampUs async {
    return await sendInstanceGet<long?>("timestampUs");
  }

  set timestampUs(FutureOr<long?> value) {
    sendInstanceSet("timestampUs", value);
  }

  /// @brief 视频编码时间戳，单位：微秒
  ///
  FutureOr<long?> get timestampDtsUs async {
    return await sendInstanceGet<long?>("timestampDtsUs");
  }

  set timestampDtsUs(FutureOr<long?> value) {
    sendInstanceSet("timestampDtsUs", value);
  }

  /// @brief 视频分辨率的宽度，单位：px
  ///
  FutureOr<int?> get width async {
    return await sendInstanceGet<int?>("width");
  }

  set width(FutureOr<int?> value) {
    sendInstanceSet("width", value);
  }

  /// @brief 视频分辨率的高度，单位：px
  ///
  FutureOr<int?> get height async {
    return await sendInstanceGet<int?>("height");
  }

  set height(FutureOr<int?> value) {
    sendInstanceSet("height", value);
  }

  /// @brief 视频编码类型。参看 VideoCodecType{@link #VideoCodecType-2}
  ///
  FutureOr<VideoCodecType?> get videoCodecType async {
    try {
      final result = await sendInstanceGet<VideoCodecType?>("videoCodecType");
      if (result == null) {
        return null;
      }
      return VideoCodecType.values.firstWhere((element) => element == result);
    } catch (e) {
      return null;
    }
  }

  set videoCodecType(FutureOr<VideoCodecType?> value) {
    sendInstanceSet("videoCodecType", value);
  }

  /// @brief 视频帧类型。参看 VideoPictureType{@link #VideoPictureType}
  ///
  FutureOr<VideoPictureType?> get videoPictureType async {
    try {
      final result =
          await sendInstanceGet<VideoPictureType?>("videoPictureType");
      if (result == null) {
        return null;
      }
      return VideoPictureType.values.firstWhere((element) => element == result);
    } catch (e) {
      return null;
    }
  }

  set videoPictureType(FutureOr<VideoPictureType?> value) {
    sendInstanceSet("videoPictureType", value);
  }

  /// @brief 视频帧旋转角度，默认旋转 0 度。参看 VideoRotation{@link #VideoRotation}
  ///
  FutureOr<VideoRotation?> get videoRotation async {
    try {
      final result = await sendInstanceGet<VideoRotation?>("videoRotation");
      if (result == null) {
        return null;
      }
      return VideoRotation.values.firstWhere((element) => element == result);
    } catch (e) {
      return null;
    }
  }

  set videoRotation(FutureOr<VideoRotation?> value) {
    sendInstanceSet("videoRotation", value);
  }
}

enum RenderMode {
  ByteRTCRenderModeHidden(1),

  ByteRTCRenderModeFit(2),

  ByteRTCRenderModeFill(3);

  final dynamic $value;
  const RenderMode([this.$value]);
}

enum NetworkType {
  UNKNOWN(-1),

  NONE(0),

  LAN(1),

  WIFI(2),

  MOBILE_2G(3),

  MOBILE_3G(4),

  MOBILE_4G(5),

  MOBILE_5G(6);

  final dynamic $value;
  const NetworkType([this.$value]);
}

enum UserOfflineReason {
  ByteRTCUserOfflineReasonQuit(0),

  ByteRTCUserOfflineReasonDropped(1),

  ByteRTCUserOfflineReasonSwitchToInvisible(2),

  ByteRTCUserOfflineReasonKickedByAdmin(3);

  final dynamic $value;
  const UserOfflineReason([this.$value]);
}
