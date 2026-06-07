/// Copyright (c) 2026 Beijing Volcano Engine Technology Ltd.

import '../codegen/pack/callback.dart';
import '../codegen/pack/index.dart' as $p;
import '../codegen/pack/types.dart';

export '../codegen/pack/keytype.dart'
    show
        SubscribeFallbackOptions,
        VideoEncoderConfig,
        MixedStreamVideoCodecType,
        VoiceEqualizationConfig,
        CapturePreference,
        AudioMixingType,
        PlayerState,
        AudioSampleRate,
        AudioRecordingState,
        PlayerError,
        PositionInfo,
        ReceiveRange,
        AudioChannel,
        AttenuationType,
        AudioQuality,
        AudioPropertiesMode,
        AudioFrameSource,
        VoiceReverbConfig,
        AudioRoute,
        AudioAlignmentMode,
        AudioRecordingConfig,
        AudioReportMode,
        VideoSimulcastMode,
        EarMonitorMode,
        AudioProfileType,
        VoiceChangerType,
        VoiceReverbType,
        AnsMode,
        AudioScenarioType,
        AudioPropertiesConfig,
        RemoteAudioPropertiesInfo,
        VideoSuperResolutionMode,
        VoiceEqualizationBandFrequency,
        VideoSuperResolutionModeChangedReason,
        AudioPropertiesInfo,
        LocalAudioPropertiesInfo,
        MediaTypeEnhancementConfig,
        CameraId,
        StreamInfo,
        EffectBeautyMode,
        FaceDetectionResult,
        MediaStreamType,
        AudioDeviceType,
        MirrorType,
        VideoEncoderPreference,
        MixedStreamAlternateImageFillMode,
        MixedStreamAudioCodecType,
        MixedStreamAudioProfile,
        MixedStreamLayoutRegionImageWaterMarkConfig,
        MixedStreamLayoutRegionType,
        MixedStreamMediaType,
        MixedStreamPushMode,
        MixedStreamRenderMode,
        MixedStreamSEIContentMode,
        MixedStreamVideoType,
        PauseResumeControlMediaType,
        RecordingType,
        Rectangle,
        RemoteMirrorType,
        RemoteStreamSwitch,
        RemoteVideoConfig,
        ScreenMediaType,
        SourceCrop,
        SourceWantedData,
        SubscribeConfig,
        TorchState,
        VideoCaptureConfig,
        VideoDenoiseMode,
        VideoDeviceType,
        VideoFrameInfo,
        VideoOrientation,
        VideoRenderMode,
        VideoRotation,
        VideoRotationMode,
        // VirtualBackgroundSource,
        VirtualBackgroundSourceType,
        ZoomConfigType,
        EarMonitorAudioFilter,
        ZoomDirectionType;

export '../codegen/pack/errorcode.dart' show AudioRecordingErrorCode;

/// @brief 引擎初始化参数
class RTCVideoContext {
  /// @brief 每个应用的唯一标识符，由 RTC 控制台随机生成的。
  ///
  /// 不同的 AppId 生成的实例在 RTC 中进行音视频通话完全独立，无法互通。
  String appId;

  /// @brief 私有参数。如需使用请联系技术支持人员。
  Map<String, dynamic>? parameters;

  /// @brief 引擎相关回调事件
  IRTCEngineEventHandler? eventHandler;

  /// @brief 是否为游戏场景类型
  bool isGameScene;

  /// @brief 是否自动创建视频特效管理器
  bool autoCreateVideoEffectInterface;

  /// @brief 是否自动创建音频特效管理器
  bool autoCreateAudioEffectPlayer;

  /// @brief 是否自动创建公共流接口
  bool autoCreateWTNStream;

  RTCVideoContext(
      {required this.appId,
      this.parameters,
      this.eventHandler,
      this.autoCreateAudioEffectPlayer = false,
      this.autoCreateVideoEffectInterface = false,
      this.autoCreateWTNStream = false,
      this.isGameScene = false});

  Map<String, dynamic> toMap() {
    HashMap<String, dynamic> dic = HashMap();
    dic['appId'] = appId;
    dic['parameters'] = parameters;
    dic['eventHandler'] = eventHandler;
    dic['autoCreateAudioEffectPlayer'] = autoCreateAudioEffectPlayer;
    dic['autoCreateVideoEffectInterface'] = autoCreateVideoEffectInterface;
    dic['autoCreateWTNStream'] = autoCreateWTNStream;
    dic['isGameScene'] = isGameScene;
    return dic;
  }
}

/// @brief AAC 编码类型
enum AACProfile {
  /// @brief 编码等级 AAC-LC
  lc,

  /// @brief 编码等级 HE-AAC v1
  hev1,

  /// @brief 编码等级 HE-AAC v2
  hev2,
}

/// @brief 本地截图
class LocalSnapshot {
  /// @brief 截图任务 ID
  int taskId;

  /// @brief 截图结果文件路径
  final String filePath;

  /// @brief 图片宽度
  int width;

  /// @brief 图片高度
  int height;

  LocalSnapshot({
    this.taskId = 0,
    required this.filePath,
    this.width = 0,
    this.height = 0,
  });
}

/// @brief 远端截图
class RemoteSnapshot {
  /// @brief 截图任务 ID
  int taskId;

  /// @brief 远端流 ID
  final String streamId;

  /// @brief 流信息
  final $p.StreamInfo streamInfo;

  /// @brief 截图结果文件路径
  final String filePath;

  /// @brief 图片宽度
  int width;

  /// @brief 图片高度
  int height;

  RemoteSnapshot({
    this.taskId = 0,
    required this.streamId,
    required this.streamInfo,
    required this.filePath,
    this.width = 0,
    this.height = 0,
  });
}

/// @brief 水印图片相对视频流的位置和大小。
class Watermark extends $p.Watermark {
  Watermark({
    required super.x,
    required super.y,
    required super.width,
    required super.height,
  });

  Watermark.none() : super(x: 0, y: 0, width: 0, height: 0);
}

/// @brief 水印参数
class WatermarkConfig extends $p.WatermarkConfig {
  WatermarkConfig({
    bool visibleInPreview = true,
    Watermark? positionInLandscapeMode,
    Watermark? positionInPortraitMode,
  }) : super(
          visibleInPreview: visibleInPreview,
          positionInLandscapeMode: positionInLandscapeMode ?? Watermark.none(),
          positionInPortraitMode: positionInPortraitMode ?? Watermark.none(),
        );
}

/// @brief 背景贴纸对象。
class VirtualBackgroundSource extends $p.VirtualBackgroundSource {
  VirtualBackgroundSource({
    $p.VirtualBackgroundSourceType sourceType =
        $p.VirtualBackgroundSourceType.color,
    int sourceColor = 0x000000,
    String sourcePath = '',
  }) : super(
          sourceType: sourceType,
          sourceColor: sourceColor,
          sourcePath: sourcePath,
        );

  VirtualBackgroundSource.color({
    required super.sourceColor,
  }) : super(
          sourceType: $p.VirtualBackgroundSourceType.color,
          sourcePath: '',
        );

  VirtualBackgroundSource.image({
    required super.sourcePath,
  }) : super(
          sourceType: $p.VirtualBackgroundSourceType.image,
          sourceColor: 0x000000,
        );
}

/// @brief 媒体流信息同步的相关配置
class StreamSyncInfoConfig extends $p.StreamSyncInfoConfig {
  StreamSyncInfoConfig({
    int repeatCount = 0,
    $p.SyncInfoStreamType streamType = $p.SyncInfoStreamType.audio,
  }) : super(
          repeatCount: repeatCount,
          streamType: streamType,
        );
}
