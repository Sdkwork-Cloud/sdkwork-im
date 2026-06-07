/// Copyright (c) 2026 Beijing Volcano Engine Technology Ltd.

import 'dart:convert';
import 'dart:io';
import 'package:async/async.dart';
import 'package:flutter/foundation.dart';
import 'dart:ui';
import 'package:hybrid_runtime/hybrid_runtime.dart';

import 'bytertc_mixed_defines.dart';
import 'bytertc_video_defines.dart';
import 'bytertc_room_api.dart';
import 'bytertc_audio_effect_player_api.dart';
import 'bytertc_media_player_api.dart';
import 'bytertc_video_effect_api.dart';
import 'bytertc_audio_device_manager_api.dart';
import 'bytertc_media_defines.dart';
import 'bytertc_wtn_api.dart';
import '../core/helper.dart';
import '../src/bytertc_take_snapshot_observer_impl.dart';
import '../platforms/android/extends.dart';

import '../codegen/pack/index.dart' as $p;
import '../codegen/android/index.dart' as $a;
import '../codegen/ios/index.dart' as $i;

export '../codegen/pack/api.dart' show RTCEngine;

/// 全局引擎实例
RTCEngine? globalEngine;

late dynamic $engine_instance;

/// 引擎类
class RTCEngine extends $p.RTCEngine {
  @override
  dynamic $createInstance(List<dynamic> args) {
    print('Do nothing, just return engine instance.');
    return $engine_instance;
  }

  RTCEngine();

  late VideoEffect _videoEffectInterface;

  /// 获取视频特效接口
  VideoEffect get videoEffectInterface {
    return _videoEffectInterface;
  }

  late AudioEffectPlayer _audioEffectPlayer;

  /// 获取音频特效播放器接口
  AudioEffectPlayer get audioEffectPlayer {
    return _audioEffectPlayer;
  }

  late WTNStream _wtnStream;

  /// 获取公共流接口
  WTNStream get wtnStream {
    return _wtnStream;
  }

  final TakeSnapshotResultObserver _takeSnapshotResultObserver =
      TakeSnapshotResultObserver();

  /// 获取 SDK 版本号
  static Future<String?> getSDKVersion() async {
    return $p.RTCEngine.getSDKVersion();
  }

  /// 设置日志配置
  static Future<int?> setLogConfig($p.RTCLogConfig logConfig) async {
    return $p.RTCEngine.setLogConfig(logConfig);
  }

  /// 创建引擎实例，返回 engine 实例。
  static Future<RTCEngine> createRTCEngine(RTCVideoContext context) async {
    final map = context.parameters ?? {};
    map['rtc.platform'] = 6;
    final config = $p.EngineConfig(
      appID: context.appId,
      parameters: jsonEncode(map),
      isGameScene: context.isGameScene,
    );
    final handler = context.eventHandler ?? $p.IRTCEngineEventHandler();
    if (Platform.isAndroid) {
      final ctx = await NativeVariables.getApplicationContext();
      config.context = await Future.value(ctx);
      $engine_instance = await $a.RTCEngine.createRTCEngine(
        unpackObject<$a.EngineConfig>(config),
        packObject(handler, () => $p.android_IRTCEngineEventHandler()),
      );
    }

    if (Platform.isIOS) {
      config.parameters = map;
      $engine_instance = await $i.ByteRTCEngine.createRTCEngine(
        unpackObject<$i.ByteRTCEngineConfig>(config),
        packObject(handler, () => $p.ios_IRTCEngineEventHandler()),
      );
    }

    globalEngine = RTCEngine();

    if (context.autoCreateVideoEffectInterface) {
      await globalEngine!.$createRTCVideoEffect();
    }

    if (context.autoCreateAudioEffectPlayer) {
      await globalEngine!.$createAudioEffectPlayer();
    }

    if (context.autoCreateWTNStream) {
      await globalEngine!.$createWTNStream();
    }
    return globalEngine!;
  }

  /// 销毁引擎实例
  void destroy() {
    $p.RTCEngine.destroyRTCEngine();
  }

  /// @brief 创建房间实例
  /// @param roomId 房间 ID
  /// @param autoInitRangeAudio 是否自动创建单流推流对象, 默认不创建
  /// @param autoInitSpatialAudio 是否自动创建空间音频对象, 默认不创建
  /// @return 房间实例
  @override
  Future<RTCRoom?> createRTCRoom(String roomId,
      {bool autoInitRangeAudio = false,
      bool autoInitSpatialAudio = false}) async {
    try {
      dynamic $room_instance;
      if (Platform.isAndroid) {
        $room_instance =
            await ($instance as $a.RTCEngine).createRTCRoom(roomId);
      } else if (Platform.isIOS) {
        $room_instance = await ($instance as $i.ByteRTCEngine).createRTCRoom(
          roomId,
        );
      } else {
        throw UnsupportedError(
          'Not Support Platform ${Platform.operatingSystem}',
        );
      }
      if ($room_instance != null) {
        $room_map[roomId] = $room_instance;
        return RTCRoom(roomId,
            autoInitRangeAudio: autoInitRangeAudio,
            autoInitSpatialAudio: autoInitSpatialAudio);
      }
    } catch (e) {
      return null;
    }
    return null;
  }

  /// 启动屏幕分享
  Future<int> startScreenCapture($p.ScreenMediaType type) async {
    return await byteRTCHelper.startScreenCapture(type);
  }

  /// 设置远端用户优先级
  Future<int> setRemoteUserPriority({
    required String roomId,
    required String uid,
    required $p.RemoteUserPriority priority,
  }) async {
    $android() => ($instance as $a.RTCEngine).setRemoteUserPriority(
          roomId,
          uid,
          $p.t_RemoteUserPriority.code_to_android(priority),
        );
    $ios() => ($instance as $i.ByteRTCEngine).setRemoteUserPriority(
          $p.t_RemoteUserPriority.code_to_ios(priority),
          roomId,
          uid,
        );

    if (Platform.isAndroid) {
      return $android();
    } else if (Platform.isIOS) {
      return $ios();
    } else {
      throw UnsupportedError(
        'Not Support Platform ${Platform.operatingSystem}',
      );
    }
  }

  /// 反馈，用于问题上报。
  Future<int> feedback({
    required List<ProblemFeedbackOption> types,
    ProblemFeedbackInfo? info,
  }) async {
    return byteRTCHelper.feedback(types: types, info: info);
  }

  /// 设置 RTC Engine 事件回调
  Future<int> setRTCEngineEventHandler(
      $p.IRTCEngineEventHandler handler) async {
    $android() {
      return ($instance as $a.RTCEngine).setRtcVideoEventHandler(
        packObject(handler, () => $p.android_IRTCEngineEventHandler()),
      );
    }

    $ios() {
      try {
        ($instance as $i.ByteRTCEngine).delegate = packObject(
          handler,
          () => $p.ios_IRTCEngineEventHandler(),
        );
      } catch (e) {
        return Future.value(-1);
      }
      return Future.value(0);
    }

    if (Platform.isAndroid) {
      return $android();
    } else if (Platform.isIOS) {
      return $ios();
    } else {
      throw UnsupportedError(
        'Not Support Platform ${Platform.operatingSystem}',
      );
    }
  }

  /// Only iOS
  Future<int> sendScreenCaptureExtensionMessage(Uint8List message) {
    if (Platform.isAndroid) {
      return Future<int>.value(-1);
    } else if (Platform.isIOS) {
      return globalEngine
              ?.ios_sendScreenCaptureExtensionMessage(message)
              .then((v) => v ?? -1) ??
          Future.value(-1);
    } else {
      throw UnsupportedError(
        'Not Support Platform ${Platform.operatingSystem}',
      );
    }
  }

  /// 移除公共流画面
  Future<int?> removePublicStreamVideo(String publicStreamId) async {
    $p.VideoCanvas canvas = $p.VideoCanvas(
      backgroundColor: 0x000000,
      renderMode: $p.VideoRenderMode.hidden,
      renderRotation: $p.VideoRotation.rotation0,
      view: null,
    );
    return await globalEngine?.wtnStream.setWTNRemoteVideoCanvas(
      publicStreamId,
      canvas,
    );
  }

  /// 设置传输时使用内置加密的方式
  ///
  /// [key] 是加密密钥，长度限制为 36 位，超出部分将会被截断
  ///
  /// 返回值：
  /// + `0`：调用成功；
  /// + `<0`：调用失败，具体原因参看 ReturnStatus。
  ///
  /// 注意：该方法必须在进房之前调用，可重复调用，以最后调用的参数作为生效参数。
  Future<int?> setEncryptInfo({
    required EncryptType aesType,
    required String key,
  }) {
    if (Platform.isAndroid) {
      return ($instance as $a.RTCEngine).setEncryptInfo(aesType.$value, key)
          as Future<int?>;
    } else {
      return ($instance as $i.ByteRTCEngine).setEncryptInfo(
        // Which should be t_xxx.code_to_ios
        // But it's not exist, besides, the values of ByteRTCEncryptType and EncryptType are same.
        // It's can be written as following.
        ($i.ByteRTCEncryptType.values.firstWhere(
          (e) => e.$value == aesType.$value,
        )),
        key,
      ) as Future<int?>;
    }
  }

  /// @brief 发布端设置全景视频，包括分辨率、高清视野和低清背景分辨率、Tile 大小，以及其他常规编码参数。
  /// @param encoderConfig 期望发布的最大分辨率视频流参数。参看 VideoEncoderConfig{@link #VideoEncoderConfig}。 <br>
  ///                      支持 8K 和 4K 两种分辨率的全景视频。
  /// @return 方法调用结果
  ///        - 0：成功
  ///        - !0：失败
  /// @note
  ///        - 发布全景视频前，绑定自定义采集器，必须调用该方法设置编码参数。支持的视频格式包括 YUV 或者 Texture 纹理。
  ///        - 通过 onFrame{@link #IVideoSink#onFrame} ，接收端获取到视频帧和解码需要的信息，传给自定义渲染器进行渲染。
  ///
  Future<int?> setVideoEncoderConfig(VideoEncoderConfig encoderConfig) async {
    if (Platform.isAndroid) {
      return ($instance as $a.RTCEngine).setVideoEncoderConfig(
          unpackObject<$a.VideoEncoderConfig>(encoderConfig), {});
    } else if (Platform.isIOS) {
      return ($instance as $i.ByteRTCEngine).setVideoEncoderConfig(
          unpackObject<$i.ByteRTCVideoEncoderConfig>(encoderConfig), {});
    } else {
      throw UnsupportedError(
        'Not Support Platform ${Platform.operatingSystem}',
      );
    }
  }

  /// 设置相机曝光位置
  Future<int> setCameraExposurePosition(Offset position) async {
    if (Platform.isAndroid) {
      return ($instance as $a.RTCEngine).setCameraExposurePosition(
        position.dx,
        position.dy,
      );
    } else if (Platform.isIOS) {
      return ($instance as $i.ByteRTCEngine).setCameraExposurePosition({
        'x': position.dx,
        'y': position.dy,
      });
    } else {
      throw UnsupportedError(
        'Not Support Platform ${Platform.operatingSystem}',
      );
    }
  }

  /// 设置相机对焦位置
  Future<int> setCameraFocusPosition(Offset position) async {
    if (Platform.isAndroid) {
      return ($instance as $a.RTCEngine).setCameraFocusPosition(
        position.dx,
        position.dy,
      );
    } else if (Platform.isIOS) {
      return ($instance as $i.ByteRTCEngine).setCameraFocusPosition({
        'x': position.dx,
        'y': position.dy,
      });
    } else {
      throw UnsupportedError(
        'Not Support Platform ${Platform.operatingSystem}',
      );
    }
  }

  /// 移除远端视频
  Future<int> removeRemoteVideo({
    required String streamId,
  }) async {
    return await byteRTCHelper.removeRemoteVideo(
      streamId: streamId,
    );
  }

  /// 移除本地视频
  Future<int> removeLocalVideo() async {
    return await byteRTCHelper.removeLocalVideo();
  }

  /// 截图
  CancelableOperation<LocalSnapshot> takeLocalSnapshot(
    String filePath,
  ) {
    int? _taskId;
    CancelableCompleter<LocalSnapshot> completer = CancelableCompleter(
      onCancel: () {
        if (_taskId != null) {
          _takeSnapshotResultObserver.removeLocal(_taskId.toString());
        }
      },
    );
    void completeHandler(value) {
      if (value != null) {
        if (completer.isCanceled || completer.isCompleted) {
          return;
        }
        _taskId = value;
        _takeSnapshotResultObserver.putLocal(value.toString(), completer);
        print(
            'takeLocalSnapshot taskId: $_taskId, ${_takeSnapshotResultObserver.putLocal}');
      } else {
        completer.completeError(TakeSnapshotResultObserver.errorNoTaskId);
      }
    }

    void errorHandler(error) {
      completer.completeError(TakeSnapshotResultObserver.errorException);
    }

    byteRTCHelper
        .takeLocalSnapshot(filePath, _takeSnapshotResultObserver)
        .then(completeHandler, onError: errorHandler);
    return completer.operation;
  }

  /// 截图
  CancelableOperation<RemoteSnapshot> takeRemoteSnapshot(
    String streamId,
    String filePath,
  ) {
    int? _taskId;
    final CancelableCompleter<RemoteSnapshot> completer = CancelableCompleter(
      onCancel: () {
        if (_taskId != null) {
          _takeSnapshotResultObserver.removeRemote(_taskId.toString());
        }
      },
    );
    void completeHandler(value) {
      if (value != null) {
        if (completer.isCanceled || completer.isCompleted) {
          return;
        }
        _taskId = value;
        _takeSnapshotResultObserver.putRemote(_taskId.toString(), completer);
      } else {
        completer.completeError(TakeSnapshotResultObserver.errorNoTaskId);
      }
    }

    void errorHandler(error) {
      completer.completeError(TakeSnapshotResultObserver.errorException);
    }

    byteRTCHelper
        .takeRemoteSnapshot(streamId, filePath, _takeSnapshotResultObserver)
        .then(completeHandler, onError: errorHandler);
    return completer.operation;
  }

  Future $createAudioEffectPlayer() async {
    if (Platform.isAndroid) {
      $audio_effect_instance =
          await ($instance as $a.RTCEngine).getAudioEffectPlayer();
    } else if (Platform.isIOS) {
      $audio_effect_instance =
          await ($instance as $i.ByteRTCEngine).getAudioEffectPlayer();
    } else {
      throw UnsupportedError(
        'Not Support Platform ${Platform.operatingSystem}',
      );
    }
    _audioEffectPlayer = AudioEffectPlayer();
  }

  Future $createWTNStream() async {
    if (Platform.isAndroid) {
      $wtn_instance = await ($instance as $a.RTCEngine).getWTNStream();
    } else if (Platform.isIOS) {
      $wtn_instance = await ($instance as $i.ByteRTCEngine).getWTNStream();
    } else {
      throw UnsupportedError(
        'Not Support Platform ${Platform.operatingSystem}',
      );
    }
    _wtnStream = WTNStream();
  }

  /// 获取媒体播放器
  Future<MediaPlayer?> getMediaPlayer(int playerId) async {
    if (Platform.isAndroid) {
      $media_player_map[playerId] =
          await ($instance as $a.RTCEngine).getMediaPlayer(playerId);
    } else if (Platform.isIOS) {
      $media_player_map[playerId] =
          await ($instance as $i.ByteRTCEngine).getMediaPlayer(playerId);
    } else {
      throw UnsupportedError(
        'Not Support Platform ${Platform.operatingSystem}',
      );
    }
    return MediaPlayer(playerId);
  }

  /// 获取视频特效管理器，也可设置成创建引擎后自动创建
  @override
  Future<VideoEffect?> getVideoEffectInterface() async {
    await $createRTCVideoEffect();
    return videoEffectInterface;
  }

  /// 获取音频特效管理器，也可设置成创建引擎后自动创建
  Future<AudioEffectPlayer?> getAudioEffectPlayer() async {
    await $createAudioEffectPlayer();
    return audioEffectPlayer;
  }

  /// 获取公共流接口，也可设置成创建引擎后自动创建
  @override
  Future<WTNStream?> getWTNStream() async {
    await $createWTNStream();
    return wtnStream;
  }

  /// 获取音频设备管理器
  @override
  Future<AudioDeviceManager?> getAudioDeviceManager() async {
    if (Platform.isAndroid) {
      $audio_device_manager_instance =
          await ($instance as $a.RTCEngine).getAudioDeviceManager();
    } else if (Platform.isIOS) {
      $audio_device_manager_instance =
          await ($instance as $i.ByteRTCEngine).getAudioDeviceManager();
    } else {
      throw UnsupportedError(
        'Not Support Platform ${Platform.operatingSystem}',
      );
    }
    return AudioDeviceManager();
  }

  /// 设置蓝牙模式, 仅 iOS 可用。
  Future<int?> setBluetoothMode(BluetoothMode mode) async {
    var $modeMap = {
      BluetoothMode.hfp: $i.ByteRTCBluetoothMode.hfp,
      BluetoothMode.a2dp: $i.ByteRTCBluetoothMode.a2dp,
      BluetoothMode.auto: $i.ByteRTCBluetoothMode.auto,
    };
    if (!($modeMap.containsKey(mode))) {
      throw Exception("setBluetoothMode not support:" + mode.toString());
    }
    if (Platform.isIOS) {
      return ($instance as $i.ByteRTCEngine).setBluetoothMode($modeMap[mode]!);
    } else {
      throw UnsupportedError(
        'Not Support Platform ${Platform.operatingSystem}',
      );
    }
  }

  @Deprecated('Not Support Yet.')
  Future<int?> startEchoTest({
    required EchoTestConfig config,
    required int delayTime,
  }) async {
    return 0;
    // final String id = globalGetViewByViewId(genEchoTestViewId());
    // final view = await byteRTCHelper.getView(id);
    // final $p.EchoTestConfig echoTestConfig = $p.EchoTestConfig(
    //   userId: config.userId,
    //   roomId: config.roomId,
    //   token: config.token,
    //   enableAudio: config.enableAudio,
    //   enableVideo: config.enableVideo,
    //   audioReportInterval: config.audioReportInterval,
    //   view: view,
    // );
    // if (view == null) {
    //   debugPrint('EchoTest view is not established yet!');
    //   return Future.value(-1);
    // }
    // if (Platform.isAndroid) {
    //   return ($instance as $a.RTCEngine).startEchoTest(
    //     unpackObject<$a.EchoTestConfig>(echoTestConfig),
    //     delayTime,
    //   );
    // } else if (Platform.isIOS) {
    //   return ($instance as $i.ByteRTCEngine).startEchoTest(
    //     unpackObject<$i.ByteRTCEchoTestConfig>(echoTestConfig),
    //     delayTime,
    //   );
    // } else {
    //   throw UnsupportedError(
    //     'Not Support Platform ${Platform.operatingSystem}',
    //   );
    // }
  }

  $createRTCVideoEffect() async {
    if (Platform.isAndroid) {
      $video_effect_instance =
          await ($instance as $a.RTCEngine).getVideoEffectInterface();
    } else {
      $video_effect_instance =
          await ($instance as $i.ByteRTCEngine).getVideoEffectInterface();
    }
    _videoEffectInterface = VideoEffect();
  }

  /// @valid since 3.60. 自 3.60 起，该接口替代了 `startPushMixedStreamToCDN` 和 `startPushPublicStream` 方法用于实现下述功能。如果你已升级至 3.60 及以上版本，并且仍在使用这两个方法，请迁移到此接口。
  /// @detail api
  /// @author lizheng
  /// @brief 指定房间中的媒体流，合成后一路流发布到 CDN 或发布一路 WTN 流。
  /// @param taskId 转推直播任务 ID，长度不超过 127 字节。 当 MixedStreamConfig{@link #MixedStreamConfig} 中的 `PushTargetType = 0` 时， 用于标识转推直播任务。你可以在同一房间内发起多个转推直播任务，并用不同的 ID 加以区分。当你需要发起多个转推直播任务时，应使用多个 ID；当你仅需发起一个转推直播任务时，建议使用空字符串。 <br>
  /// 当 `PushTargetType = 1` 时，为公共流，此参数设置无效，传空即可。
  /// @param pushTargetConfig 推流目标配置参数，比如设置推流地址、WTN 流 ID。参看 MixedStreamPushTargetConfig{@link #MixedStreamPushTargetConfig}。
  /// @param mixedConfig 合流转推配置参数，比如设置合流的图片、视频视图布局和音频属性。参看 MixedStreamConfig{@link #MixedStreamConfig}。
  /// @return
  ///        - 0: 成功。你可以通过 onMixedStreamEvent{@link #IRTCEngineEventHandler#onMixedStreamEvent} 回调获取启动结果和推流过程中的事件。
  ///        - !0: 调用失败。查看 ReturnStatus{@link #ReturnStatus} 获得更多错误说明
  /// @note
  ///       - 在[控制台](https://console.volcengine.com/rtc/cloudRTC?tab=callback)配置了转推直播和 WTN 流的服务端回调后，调用本接口会收到相应回调。重复调用该接口时，第二次调用会同时触发 [TranscodeStarted](https://www.volcengine.com/docs/6348/75125#transcodestarted) 和 [TranscodeUpdated](https://www.volcengine.com/docs/6348/75125#transcodeupdated)。
  ///       - 调用 stopPushMixedStream{@link #RTCEngine#stopPushMixedStream} 停止转推直播。
  ///       - 调用 updatePushMixedStream{@link #RTCEngine#updatePushMixedStream} 可以更新部分任务参数。
  ///
  Future<int?> startPushMixedStream(
      {required String taskId,
      required MixedStreamPushTargetConfig pushTargetConfig,
      required MixedStreamConfig mixedConfig}) {
    mixedConfig.pushTargetType = pushTargetConfig.pushTargetType;
    return byteRTCHelper.startPushMixedStream(
        taskId: taskId,
        pushTargetConfig: pushTargetConfig,
        mixedConfig: mixedConfig);
  }

  /// 更新推流配置
  Future<int?> updatePushMixedStream(
      {required String taskId,
      required MixedStreamPushTargetConfig pushTargetConfig,
      required MixedStreamConfig mixedConfig}) {
    mixedConfig.pushTargetType = pushTargetConfig.pushTargetType;
    return byteRTCHelper.updatePushMixedStream(
        taskId: taskId,
        pushTargetConfig: pushTargetConfig,
        mixedConfig: mixedConfig);
  }

  /// 获取音频路由
  @override
  Future<AudioRoute?> getAudioRoute() {
    return byteRTCHelper.getAudioRoute();
  }

  /// @note
  /// 仅 iOS 需调用, 使用屏幕共享时调用。
  /// 您也可通过 iOS 的 ByteRTCHelper setExtensionConfig:bundleId: 方法进行设置。
  Future<int?> setExtensionConfig({
    required String groupId,
    required String bundleId,
  }) {
    return byteRTCHelper.setExtensionConfig(
        groupId: groupId, bundleId: bundleId);
  }

  /// 设置视频采集配置
  @override
  Future<int?> setVideoCaptureConfig(VideoCaptureConfig videoCaptureConfig) {
    return byteRTCHelper.setVideoCaptureConfig(videoCaptureConfig);
  }

  /// 更新远端用户视图属性
  Future<int?> updateRemoteStreamVideoCanvas({
    required String streamId,
    VideoRotation rotation = VideoRotation.rotation0,
    VideoRenderMode renderMode = VideoRenderMode.hidden,
    int backgroundColor = 0x00000000,
  }) async {
    if (Platform.isAndroid) {
      return ($instance as $a.RTCEngine).updateRemoteStreamVideoCanvas(
          streamId,
          $p.t_VideoRenderMode.code_to_android(renderMode).$value,
          backgroundColor);
    } else if (Platform.isIOS) {
      return ($instance as $i.ByteRTCEngine).updateRemoteStreamVideoCanvas(
          streamId,
          $p.t_VideoRenderMode.code_to_ios(renderMode),
          backgroundColor);
    } else {
      throw UnsupportedError(
          'Not Support Platform ${Platform.operatingSystem}');
    }
  }
}
