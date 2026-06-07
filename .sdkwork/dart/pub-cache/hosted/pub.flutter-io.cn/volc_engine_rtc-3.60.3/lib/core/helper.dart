/// Copyright (c) 2026 Beijing Volcano Engine Technology Ltd.

import 'dart:io';
import 'dart:typed_data';
import 'package:hybrid_runtime/hybrid_runtime.dart';

import '../api/bytertc_video_api.dart';
import '../api/bytertc_media_defines.dart' as $m;
import '../codegen/pack/index.dart' as $p;
import '../codegen/android/index.dart' as $a;
import '../codegen/ios/index.dart' as $i;
import '../api/bytertc_mixed_defines.dart';
import '../src/bytertc_take_snapshot_observer_impl.dart';

/// Inner patch helper
class BasicHelper extends NativeClass {
  static String namespace = Platform.isAndroid
      ? r'com.ss.bytertc.engine.flutter.ByteRTCHelper'
      : r'ByteRTCHelper';
  BasicHelper()
      : super(NativeClassOptions(
          [],
          className: BasicHelper.namespace,
          instanceType: InstanceType.manual,
          bridgeKey: 'com.volcengine.rtc.hybrid_runtime',
        ));

  Future<T> invoke<T>({
    required String method,
    required List<dynamic> args,
    bool addGlobalEngine = true,
    bool isStatic = false,
  }) async {
    if (addGlobalEngine) {
      args.insert(0, $engine_instance);
    }
    return isStatic
        ? await NativeClassUtils.nativeStaticCall(
            namespace, method, args, 'com.volcengine.rtc.hybrid_runtime')
        : await nativeCall(method, args);
  }
}

class ByteRTCHelper extends PackClass {
  ByteRTCHelper() : super();

  @override
  dynamic $createInstance(List<dynamic> args) {
    if (Platform.isAndroid || Platform.isIOS) {
      return BasicHelper();
    } else {
      throw UnsupportedError('Unsupported platform');
    }
  }

  Future<dynamic> getView(String id) async {
    if (Platform.isAndroid) {
      return await ($instance as BasicHelper).invoke<dynamic>(
        method: 'getView',
        args: [id],
        addGlobalEngine: false,
      );
    } else if (Platform.isIOS) {
      return await ($instance as BasicHelper).invoke<dynamic>(
        method: 'getView:',
        args: [id],
        addGlobalEngine: false,
      );
    } else {
      throw UnsupportedError('Unsupported platform');
    }
  }

  /// Only for android surface view, set z order on top
  Future<int> setZOrderOnTop(String id, bool onTop) async {
    if (Platform.isAndroid) {
      return await ($instance as BasicHelper).invoke<int>(
        method: 'setZOrderOnTop',
        args: [id, onTop],
      );
    } else {
      throw UnsupportedError('Unsupported platform');
    }
  }

  /// Only for android surface view, set z order media overlay
  Future<int> setZOrderMediaOverlay(String id, bool isMediaOverlay) async {
    if (Platform.isAndroid) {
      return await ($instance as BasicHelper).invoke<int>(
        method: 'setZOrderMediaOverlay:',
        args: [id, isMediaOverlay],
      );
    } else {
      throw UnsupportedError('Unsupported platform');
    }
  }

  /// Screen Capture for android
  Future<int> startScreenCapture($p.ScreenMediaType type) async {
    if (Platform.isAndroid) {
      return await ($instance as BasicHelper).invoke<int>(
        method: 'startScreenCapture',
        args: [$p.t_ScreenMediaType.code_to_android(type).$value],
      );
    } else if (Platform.isIOS) {
      return await ($instance as BasicHelper).invoke<int>(
        method: 'startScreenCapture:type:',
        args: [$p.t_ScreenMediaType.code_to_ios(type).$value],
      );
    } else {
      throw UnsupportedError('Unsupported platform');
    }
  }

  Future<int> feedback({
    required List<$m.ProblemFeedbackOption> types,
    $m.ProblemFeedbackInfo? info,
  }) async {
    if (Platform.isAndroid) {
      return await ($instance as BasicHelper).invoke<int>(
        method: 'feedback',
        args: [types.map((v) => v.index).toList(), unpackObject(info)],
      );
    } else if (Platform.isIOS) {
      return await ($instance as BasicHelper).invoke<int>(
        method: 'feedback:types:info:',
        args: [types.map((v) => v.index).toList(), unpackObject(info)],
      );
    } else {
      throw UnsupportedError('Unsupported platform');
    }
  }

  Future<int?> takeLocalSnapshot(
    String filePath,
    TakeSnapshotResultObserver observer,
  ) async {
    await byteRTCHelper.addSnapshotEventHandler(observer);
    if (Platform.isAndroid) {
      return await ($instance as BasicHelper).invoke<int>(
        method: 'takeLocalSnapshot',
        args: [filePath],
      );
    } else if (Platform.isIOS) {
      return await ($instance as BasicHelper).invoke<int>(
        method: 'takeLocalSnapshot:filePath:',
        args: [filePath],
      );
    } else {
      throw UnsupportedError('Unsupported platform');
    }
  }

  Future<int?> takeRemoteSnapshot(
    String streamId,
    String filePath,
    TakeSnapshotResultObserver observer,
  ) async {
    await byteRTCHelper.addSnapshotEventHandler(observer);
    if (Platform.isAndroid) {
      return await ($instance as BasicHelper).invoke<int>(
        method: 'takeRemoteSnapshot',
        args: [streamId, filePath],
      );
    } else if (Platform.isIOS) {
      return await ($instance as BasicHelper).invoke<int>(
        method: 'takeRemoteSnapshot:streamId:filePath:',
        args: [streamId, filePath],
      );
    } else {
      throw UnsupportedError('Unsupported platform');
    }
  }

  Future<int> sendPublicStreamSEIMessage({
    $p.StreamIndex streamIndex = $p.StreamIndex.main,
    required int channelId,
    required Uint8List message,
    required int repeatCount,
    $p.SEICountPerFrame mode = $p.SEICountPerFrame.single,
  }) async {
    if (Platform.isAndroid) {
      return await ($instance as BasicHelper).invoke<int>(
        method: 'sendPublicStreamSEIMessage',
        args: [
          $p.t_StreamIndex.code_to_android(streamIndex).$value,
          channelId,
          message,
          repeatCount,
          $p.t_SEICountPerFrame.code_to_android(mode).$value,
        ],
      );
    } else if (Platform.isIOS) {
      return await ($instance as BasicHelper).invoke<int>(
        method:
            'sendPublicStreamSEIMessage:streamIndex:channelId:message:repeatCount:countPerFrame:',
        args: [
          $p.t_StreamIndex.code_to_ios(streamIndex).$value,
          channelId,
          message,
          repeatCount,
          $p.t_SEICountPerFrame.code_to_ios(mode).$value,
        ],
      );
    } else {
      throw UnsupportedError('Unsupported platform');
    }
  }

  Future<int> removeLocalVideo() async {
    if (Platform.isAndroid) {
      return await ($instance as BasicHelper).invoke<int>(
        method: 'removeLocalVideo',
        args: [],
      );
    } else if (Platform.isIOS) {
      return await ($instance as BasicHelper).invoke<int>(
        method: 'removeLocalVideo:',
        args: [],
      );
    } else {
      throw UnsupportedError('Unsupported platform');
    }
  }

  Future<int> updateRemoteStreamVideoCanvas({
    required $p.RemoteStreamKey remoteStreamKey,
    required $p.VideoRenderMode renderMode,
    int backgroundColor = 0x000000,
  }) async {
    if (Platform.isAndroid) {
      return await ($instance as BasicHelper).invoke<int>(
        method: 'updateRemoteStreamVideoCanvas',
        args: [
          unpackObject(remoteStreamKey),
          $p.t_VideoRenderMode.code_to_android(renderMode).$value,
          backgroundColor,
        ],
      );
    } else if (Platform.isIOS) {
      return await ($instance as BasicHelper).invoke<int>(
        method:
            'updateRemoteStreamVideoCanvas:remoteStreamKey:renderMode:backgroundColor:',
        args: [
          unpackObject(remoteStreamKey),
          $p.t_VideoRenderMode.code_to_ios(renderMode).$value,
          backgroundColor,
        ],
      );
    } else {
      throw UnsupportedError('Unsupported platform');
    }
  }

  Future<int> removeRemoteVideo({
    required String streamId,
  }) async {
    if (Platform.isAndroid) {
      return await ($instance as BasicHelper).invoke<int>(
        method: 'removeRemoteVideo',
        args: [streamId],
      );
    } else if (Platform.isIOS) {
      return await ($instance as BasicHelper).invoke<int>(
        method: 'removeRemoteVideo:streamId:',
        args: [streamId],
      );
    } else {
      throw UnsupportedError('Unsupported platform');
    }
  }

  /// 添加截图事件回调
  Future<int> addSnapshotEventHandler(
    TakeSnapshotResultObserver oberserver,
  ) async {
    if (Platform.isAndroid) {
      return Future.value(0);
    } else if (Platform.isIOS) {
      return await ($instance as BasicHelper).invoke<int>(
        method: 'addSnapshotEventHandler:',
        args: [oberserver],
        addGlobalEngine: false,
      );
    } else {
      throw UnsupportedError('Unsupported platform');
    }
  }

  Future<int> setExtensionConfig({
    required String groupId,
    required String bundleId,
  }) async {
    if (Platform.isIOS) {
      return await ($instance as BasicHelper).invoke<int>(
        method: 'setExtensionConfig:groupId:bundleId:',
        args: [groupId, bundleId],
      );
    }
    return Future.value(0);
  }

  Future<int> startPushMixedStream(
      {required String taskId,
      required MixedStreamPushTargetConfig pushTargetConfig,
      required MixedStreamConfig mixedConfig}) async {
    if (Platform.isAndroid) {
      return await ($instance as BasicHelper).invoke<int>(
        method: 'startPushMixedStream',
        args: [
          taskId,
          unpackObject<$a.MixedStreamPushTargetConfig>(pushTargetConfig),
          mixedConfig.toMap(),
        ],
      );
    } else if (Platform.isIOS) {
      return await ($instance as BasicHelper).invoke<int>(
        method:
            'startPushMixedStream:taskId:withPushTargetConfig:withMixedConfig:',
        args: [
          taskId,
          unpackObject<$i.ByteRTCMixedStreamPushTargetConfig>(pushTargetConfig),
          mixedConfig.toMap(),
        ],
      );
    } else {
      throw UnsupportedError('Unsupported platform');
    }
  }

  Future<int> updatePushMixedStream(
      {required String taskId,
      required MixedStreamPushTargetConfig pushTargetConfig,
      required MixedStreamConfig mixedConfig}) async {
    if (Platform.isAndroid) {
      return await ($instance as BasicHelper).invoke<int>(
        method: 'updatePushMixedStream',
        args: [
          taskId,
          unpackObject<$a.MixedStreamPushTargetConfig>(pushTargetConfig),
          mixedConfig.toMap(),
        ],
      );
    } else if (Platform.isIOS) {
      return await ($instance as BasicHelper).invoke<int>(
        method:
            'updatePushMixedStream:taskId:withPushTargetConfig:withMixedConfig:',
        args: [
          taskId,
          unpackObject<$i.ByteRTCMixedStreamPushTargetConfig>(pushTargetConfig),
          mixedConfig.toMap(),
        ],
      );
    } else {
      throw UnsupportedError('Unsupported platform');
    }
  }

  Future<$p.AudioRoute> getAudioRoute() async {
    if (Platform.isAndroid) {
      int res = await ($instance as BasicHelper).invoke<int>(
        method: 'getAudioRoute',
        args: [],
      );
      final route = $a.AudioRoute.values.firstWhere((e) => e.$value == res);
      return $p.t_AudioRoute.android_to_code(route);
    } else if (Platform.isIOS) {
      int res = await ($instance as BasicHelper).invoke<int>(
        method: 'getAudioRoute:',
        args: [],
      );
      final route =
          $i.ByteRTCAudioRoute.values.firstWhere((e) => e.$value == res);
      return $p.t_AudioRoute.ios_to_code(route);
    } else {
      throw UnsupportedError('Unsupported platform');
    }
  }

  Future<$p.PlayerState> getState(dynamic instance) async {
    if (Platform.isAndroid) {
      int res = await ($instance as BasicHelper).invoke<dynamic>(
        method: 'getState',
        args: [instance],
        addGlobalEngine: false,
      );
      final state = $a.PlayerState.values.firstWhere((e) => e.$value == res);
      return $p.t_PlayerState.android_to_code(state);
    } else if (Platform.isIOS) {
      int res = await ($instance as BasicHelper).invoke<dynamic>(
        method: 'getState:',
        args: [instance],
        addGlobalEngine: false,
      );
      final state =
          $i.ByteRTCPlayerState.values.firstWhere((e) => e.$value == res);
      return $p.t_PlayerState.ios_to_code(state);
    } else {
      throw UnsupportedError('Unsupported platform');
    }
  }

  Future<int> setVideoCaptureConfig(
      $p.VideoCaptureConfig videoCaptureConfig) async {
    if (Platform.isAndroid) {
      return await ($instance as BasicHelper).invoke<int>(
        method: 'setVideoCaptureConfig',
        args: [
          unpackObject<$a.VideoCaptureConfig>(videoCaptureConfig),
        ],
      );
    } else if (Platform.isIOS) {
      return await ($instance as BasicHelper).invoke<int>(
        method: 'setVideoCaptureConfig:videoCaptureConfig:',
        args: [
          videoCaptureConfig.toMap(),
        ],
      );
    } else {
      throw UnsupportedError('Unsupported platform');
    }
  }
}

final ByteRTCHelper byteRTCHelper = new ByteRTCHelper();
