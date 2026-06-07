/// Copyright (c) 2026 Beijing Volcano Engine Technology Ltd.

import 'dart:convert';
import 'dart:typed_data';
import '../codegen/pack/index.dart' as $p;

export '../codegen/pack/keytype.dart'
    show
        Position,
        RTCOrientation,
        HumanOrientation,
        MixedStreamControlConfig,
        MixedStreamPushTargetType,
        MixedStreamTaskInfo,
        InterpolationMode,
        StreamLayoutMode,
        MixedStreamTaskEvent;

/// @brief 合流音频配置.
class MixedStreamAudioConfig {
  final $p.MixedStreamAudioCodecType audioCodec;
  final int bitrate;
  final int sampleRate;
  final int channels;
  final $p.MixedStreamAudioProfile audioProfile;

  MixedStreamAudioConfig({
    this.audioCodec = $p.MixedStreamAudioCodecType.aac,
    this.bitrate = 64,
    this.sampleRate = 48000,
    this.channels = 2,
    this.audioProfile = $p.MixedStreamAudioProfile.lc,
  });

  Map<String, dynamic> toMap() {
    return {
      'audioCodec': audioCodec.$value,
      'bitrate': bitrate,
      'sampleRate': sampleRate,
      'channels': channels,
      'audioProfile': audioProfile.$value,
    };
  }

  factory MixedStreamAudioConfig.fromMap(Map<String, dynamic> map) {
    return MixedStreamAudioConfig(
      audioCodec:
          $p.MixedStreamAudioCodecType.values[map['audioCodec'] as int? ?? 0],
      bitrate: map['bitrate'] as int? ?? 64,
      sampleRate: map['sampleRate'] as int? ?? 48000,
      channels: map['channels'] as int? ?? 2,
      audioProfile:
          $p.MixedStreamAudioProfile.values[map['audioProfile'] as int? ?? 0],
    );
  }
}

/// @brief 合流视频配置.
class MixedStreamVideoConfig {
  final int bitrate;
  final int fps;
  final int gop;
  final int width;
  final int height;
  final bool enableBframe;
  final $p.MixedStreamVideoCodecType videoCodec;

  MixedStreamVideoConfig({
    this.bitrate = 500,
    this.fps = 15,
    this.gop = 2,
    this.width = 360,
    this.height = 640,
    this.enableBframe = false,
    this.videoCodec = $p.MixedStreamVideoCodecType.h264,
  });

  Map<String, dynamic> toMap() {
    return {
      'bitrate': bitrate,
      'fps': fps,
      'gop': gop,
      'width': width,
      'height': height,
      'enableBframe': enableBframe,
      'videoCodec': videoCodec.$value,
    };
  }

  factory MixedStreamVideoConfig.fromMap(Map<String, dynamic> map) {
    return MixedStreamVideoConfig(
      bitrate: map['bitrate'] as int? ?? 500,
      fps: map['fps'] as int? ?? 15,
      gop: map['gop'] as int? ?? 2,
      width: map['width'] as int? ?? 360,
      height: map['height'] as int? ?? 640,
      enableBframe: map['enableBframe'] as bool? ?? false,
      videoCodec:
          $p.MixedStreamVideoCodecType.values[map['videoCodec'] as int? ?? 0],
    );
  }
}

/// @brief 合流转推配置参数。
///        如无特别说明，参数可适用于 WTN 流和合流转推任务。
///        如无特别说明，参数可用于启动和更新任务。
class MixedStreamConfig {
  final String roomId;
  final String userId;
  final List<MixedStreamLayoutRegionConfig> regions;
  final MixedStreamVideoConfig? videoConfig;
  final MixedStreamAudioConfig? audioConfig;
  final $p.MixedStreamControlConfig? controlConfig;
  final MixedStreamSpatialAudioConfig? spatialAudioConfig;
  final String backgroundColor;
  final String userConfigExtraInfo;
  final String backgroundImageUrl;
  final dynamic advancedConfig;
  final dynamic authInfo;
  final $p.InterpolationMode interpolationMode;
  final $p.StreamLayoutMode layoutMode;
  $p.MixedStreamPushTargetType pushTargetType;

  MixedStreamConfig({
    required this.roomId,
    required this.userId,
    required this.regions,
    this.videoConfig,
    this.audioConfig,
    this.controlConfig,
    this.spatialAudioConfig,
    this.backgroundColor = '#000000',
    this.userConfigExtraInfo = '',
    this.backgroundImageUrl = '',
    this.advancedConfig,
    this.authInfo,
    this.interpolationMode = $p.InterpolationMode.last_frame_fill,
    this.layoutMode = $p.StreamLayoutMode.auto,
    this.pushTargetType = $p.MixedStreamPushTargetType.push_to_cdn,
  });

  Map<String, dynamic> toMap() {
    return {
      'roomId': roomId,
      'userId': userId,
      'regions': regions.map((e) => e.toMap()).toList(),
      'videoConfig': videoConfig?.toMap(),
      'audioConfig': audioConfig?.toMap(),
      'controlConfig': controlConfig?.toMap(),
      'spatialAudioConfig': spatialAudioConfig?.toMap(),
      'backgroundColor': backgroundColor,
      'userConfigExtraInfo': userConfigExtraInfo,
      'backgroundImageUrl': backgroundImageUrl,
      'advancedConfig': advancedConfig,
      'authInfo': authInfo,
      'interpolationMode': interpolationMode.$value,
      'layoutMode': layoutMode.$value,
      'pushTargetType': pushTargetType.$value,
    };
  }
}

/// @brief 单个图片或视频流在合流中的布局信息。
///        开启转推直播功能后，在多路视频流合流时，你可以设置其中一路视频流在合流中的预设布局信息。
class MixedStreamLayoutRegionConfig {
  /// @brief 用户 ID
  final String userId;

  /// @brief 房间 ID
  final String roomId;

  /// @brief 布局区域左上角 x 坐标
  final int locationX;

  /// @brief 布局区域左上角 y 坐标
  final int locationY;

  /// @brief 布局区域宽度
  final int width;

  /// @brief 布局区域高度
  final int height;

  /// @brief 布局区域层级
  final int zOrder;

  /// @brief 布局区域透明度
  final double alpha;

  /// @brief 布局区域圆角
  final double cornerRadius;

  /// @brief 布局区域媒体类型
  final $p.MixedStreamMediaType mediaType;

  /// @brief 布局区域渲染模式
  final $p.MixedStreamRenderMode renderMode;

  /// @brief 是否为本地用户
  final bool isLocalUser;

  /// @brief 视频流类型
  final $p.MixedStreamVideoType streamType;

  /// @brief 布局区域内容类型
  final $p.MixedStreamLayoutRegionType regionContentType;

  /// @brief 图片水印
  final Uint8List? imageWaterMark;

  /// @brief 图片水印填充模式
  final $p.MixedStreamAlternateImageFillMode alternateImageFillMode;

  /// @brief 视频源裁剪
  final $p.SourceCrop? sourceCrop;

  /// @brief 占位图片 URL
  final String alternateImageUrl;

  /// @brief 空间音频位置
  final $p.Position? spatialPosition;

  /// @brief 图片水印配置
  final $p.MixedStreamLayoutRegionImageWaterMarkConfig? imageWaterMarkConfig;

  /// @brief 应用空间音频
  final bool applySpatialAudio;

  MixedStreamLayoutRegionConfig({
    this.userId = '',
    this.roomId = '',
    this.locationX = 0,
    this.locationY = 0,
    this.width = 360,
    this.height = 640,
    this.zOrder = 0,
    this.alpha = 1.0,
    this.cornerRadius = 0.0,
    this.mediaType = $p.MixedStreamMediaType.audioAndVideo,
    this.renderMode = $p.MixedStreamRenderMode.hidden,
    this.isLocalUser = false,
    this.streamType = $p.MixedStreamVideoType.main,
    this.regionContentType = $p.MixedStreamLayoutRegionType.videoStream,
    this.imageWaterMark,
    this.alternateImageFillMode = $p.MixedStreamAlternateImageFillMode.fit,
    this.sourceCrop,
    this.alternateImageUrl = '',
    this.spatialPosition,
    this.imageWaterMarkConfig,
    this.applySpatialAudio = true,
  });

  Map<String, dynamic> toMap() {
    return {
      'userId': userId,
      'roomId': roomId,
      'locationX': locationX,
      'locationY': locationY,
      'width': width,
      'height': height,
      'zOrder': zOrder,
      'alpha': alpha,
      'cornerRadius': cornerRadius,
      'mediaType': mediaType.$value,
      'renderMode': renderMode.$value,
      'isLocalUser': isLocalUser,
      'streamType': streamType.$value,
      'regionContentType': regionContentType.$value,
      'imageWaterMark': base64Encode(imageWaterMark ?? []),
      'alternateImageFillMode': alternateImageFillMode.$value,
      'sourceCrop': sourceCrop?.toMap(),
      'alternateImageUrl': alternateImageUrl,
      'spatialPosition': spatialPosition?.toMap(),
      'imageWaterMarkConfig': imageWaterMarkConfig?.toMap(),
      'applySpatialAudio': applySpatialAudio,
    };
  }
}

/// @brief 空间音频配置
class MixedStreamSpatialAudioConfig {
  /// @brief 是否启用空间音频
  final bool enableSpatialRender;

  /// @brief 观众空间音频位置
  final $p.Position? audienceSpatialPosition;

  /// @brief 观众空间音频方向
  final $p.HumanOrientation? audienceSpatialOrientation;

  MixedStreamSpatialAudioConfig({
    this.enableSpatialRender = false,
    this.audienceSpatialPosition,
    this.audienceSpatialOrientation,
  });

  MixedStreamSpatialAudioConfig.disabled()
      : this(
            enableSpatialRender: false,
            audienceSpatialPosition: $p.Position(x: 0, y: 0, z: 0),
            audienceSpatialOrientation: $p.HumanOrientation(
              forward: $p.RTCOrientation(x: 1, y: 0, z: 0),
              up: $p.RTCOrientation(x: 0, y: 1, z: 0),
              right: $p.RTCOrientation(x: 0, y: 0, z: 1),
            ));

  Map<String, dynamic> toMap() {
    Map<String, dynamic>? map = audienceSpatialOrientation?.toMap();
    if (map != null) {
      map["forward"] = audienceSpatialOrientation?.forward.toMap();
      map["up"] = audienceSpatialOrientation?.up.toMap();
      map["right"] = audienceSpatialOrientation?.right.toMap();
    }
    return {
      'enableSpatialRender': enableSpatialRender,
      'audienceSpatialPosition': audienceSpatialPosition?.toMap(),
      'audienceSpatialOrientation': map,
    };
  }
}

/// @brief 合流任务配置
class MixedStreamPushTargetConfig extends $p.MixedStreamPushTargetConfig {
  MixedStreamPushTargetConfig({
    super.pushTargetType = $p.MixedStreamPushTargetType.push_to_cdn,
    super.pushCDNUrl = '',
    super.pushWTNStreamId = '',
  });

  MixedStreamPushTargetConfig.toCDN({
    required String pushCDNUrl,
  }) : this(
            pushCDNUrl: pushCDNUrl,
            pushTargetType: $p.MixedStreamPushTargetType.push_to_cdn);

  MixedStreamPushTargetConfig.toWTN({
    required String pushWTNStreamId,
  }) : this(
            pushWTNStreamId: pushWTNStreamId,
            pushTargetType: $p.MixedStreamPushTargetType.push_to_wtn);
}
