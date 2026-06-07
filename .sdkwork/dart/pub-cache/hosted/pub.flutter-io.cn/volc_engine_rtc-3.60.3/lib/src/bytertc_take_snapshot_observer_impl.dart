/// Copyright (c) 2026 Beijing Volcano Engine Technology Ltd.

import 'dart:io';

import 'package:async/async.dart';
import 'package:flutter/foundation.dart';
import 'package:hybrid_runtime/hybrid_runtime.dart';
import '../codegen/pack/index.dart' as $p;

import '../api/bytertc_video_defines.dart';

/// @brief 视频截图结果回调
class TakeSnapshotResultObserver extends NativeObserverClass {
  static const _$namespace_a =
      r'com.ss.bytertc.engine.video.ISnapshotResultCallback';

  static const _$namespace_i = r'ByteRTCHelperDelegate';

  static const iOSRTCMixedStreamObserverMethodMap = {
    r"onTakeLocalSnapshotResult$width$height$filePath$errorCode":
        r"onTakeLocalSnapshotResult:width:height:filePath:errorCode:",
    r"onTakeRemoteSnapshotResult$streamId$streamInfo$width$height$filePath$errorCode":
        r"onTakeRemoteSnapshotResult:streamId:streamInfo:width:height:filePath:errorCode:",
  };

  static const androidRTCMixedStreamObserverMethodMap = {
    r"onTakeLocalSnapshotResult": r"onTakeLocalSnapshotResult",
    r"onTakeRemoteSnapshotResult": r"onTakeRemoteSnapshotResult",
  };

  static get codegen_$namespace {
    if (Platform.isAndroid) {
      return _$namespace_a;
    }
    return _$namespace_i;
  }

  TakeSnapshotResultObserver()
      : super(
          NativeClassOptions([],
              className: codegen_$namespace,
              instanceType: InstanceType.manual,
              methodMap: Platform.isAndroid
                  ? androidRTCMixedStreamObserverMethodMap
                  : iOSRTCMixedStreamObserverMethodMap,
              bridgeKey: 'com.volcengine.rtc.hybrid_runtime'),
        ) {
    if (Platform.isAndroid) {
      registerEvent(r"onTakeLocalSnapshotResult", onTakeLocalSnapshotResult);
      registerEvent(r"onTakeRemoteSnapshotResult", onTakeRemoteSnapshotResult);
    }
    if (Platform.isIOS) {
      registerEvent(
        r"onTakeLocalSnapshotResult:width:height:filePath:errorCode:",
        onTakeLocalSnapshotResult,
      );
      registerEvent(
        r"onTakeRemoteSnapshotResult:streamId:streamInfo:width:height:filePath:errorCode:",
        onTakeRemoteSnapshotResult,
      );
    }
  }

  /// @brief 调用异常
  static const int errorException = -100;

  /// @brief 未返回 TASK ID
  static const int errorNoTaskId = -101;

  /// @brief 文件写入失败
  static const int errorWriteFileFailed = -102;

  /// @brief 图片格式错误
  static const int errorImageFormat = -103;

  final Map<String, CancelableCompleter<LocalSnapshot>> _localCompleters = {};

  final Map<String, CancelableCompleter<RemoteSnapshot>> _remoteCompleters = {};

  void removeLocal(String taskId) => _localCompleters.remove(taskId);

  void putLocal(String taskId, CancelableCompleter<LocalSnapshot> completer) {
    _localCompleters[taskId] = completer;
  }

  void removeRemote(String taskId) => _remoteCompleters.remove(taskId);

  void putRemote(
    String taskId,
    CancelableCompleter<RemoteSnapshot> completer,
  ) {
    _remoteCompleters[taskId] = completer;
  }

  void onTakeLocalSnapshotResult(
    dynamic taskId,
    int width,
    int height,
    String filePath,
    int error,
  ) {
    var completer = _localCompleters.remove(taskId.toString());
    debugPrint(_localCompleters.toString());
    if (completer == null) {
      debugPrint('Completer<LocalSnapshot> not found!');
      return;
    }
    if (completer.isCompleted || completer.isCanceled) {
      debugPrint('Completer<LocalSnapshot> is consumed!');
      return;
    }
    if (error == 0) {
      completer.complete(
        LocalSnapshot(
          taskId: int.tryParse(taskId.toString()) ?? 0,
          filePath: filePath,
          width: width,
          height: height,
        ),
      );
    } else {
      completer.completeError(error);
    }
  }

  void onTakeRemoteSnapshotResult(
    dynamic taskId,
    String streamId,
    Map<String, dynamic> streamInfo,
    int width,
    int height,
    String filePath,
    int error,
  ) {
    var completer = _remoteCompleters.remove(taskId.toString());
    if (completer == null) {
      debugPrint('Completer<RemoteSnapshot> not found!');
      return;
    }
    if (completer.isCompleted || completer.isCanceled) {
      debugPrint('Completer<RemoteSnapshot> is consumed!');
      return;
    }
    if (error == 0) {
      $p.StreamInfo _streamInfo = $p.StreamInfo(
        streamId: streamInfo['streamId'],
        userId: streamInfo['userId'],
        roomId: streamInfo['roomId'],
        streamIndex: streamInfo['streamIndex'],
        isScreen: streamInfo['isScreen'],
      );
      completer.complete(
        RemoteSnapshot(
          taskId: int.tryParse(taskId.toString()) ?? 0,
          streamId: streamId,
          streamInfo: _streamInfo,
          filePath: filePath,
          width: width,
          height: height,
        ),
      );
    } else {
      completer.completeError(error);
    }
  }
}
