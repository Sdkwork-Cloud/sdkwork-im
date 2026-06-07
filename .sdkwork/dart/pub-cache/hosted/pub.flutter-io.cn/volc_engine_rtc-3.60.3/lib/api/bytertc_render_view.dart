/// Copyright (c) 2026 Beijing Volcano Engine Technology Ltd.

import 'package:flutter/foundation.dart';
import 'package:flutter/gestures.dart';
import 'package:flutter/services.dart';
import 'package:flutter/widgets.dart';
import '../codegen/pack/keytype.dart';
import '../core/view.dart';

/// @brief 画布类型
enum VideoCanvasType {
  /// @brief 用于渲染本地视频的画布
  local,

  /// @brief 用于渲染远端视频的画布
  remote,

  /// @brief 用于渲染公共视频流的画布
  public_stream,

  /// @brief 回路测试时渲染本地视频的画布
  echo_test,
}

/// @brief 生成 View ID
String genViewId(String roomId, String uid) {
  return '$roomId-$uid';
}

/// @brief 生成公共流 ID
String genPublicStreamId(String streamId) {
  return 'generic-public-stream-inner-$streamId';
}

String genEchoTestViewId() {
  return genViewId('echoTestContext', 'echoTestContext');
}

/// @brief 用于 [RTCSurfaceView] 初始化
class RTCViewContext {
  /// @brief 画布类型
  final VideoCanvasType canvasType;

  /// @brief 流 ID
  String? streamId;

  /// @brief 公共流 ID
  String? publicStreamId;

  /// @brief 需要被渲染的用户的 ID
  final String userId;

  /// @brief 需要被渲染的用户所在的房间 ID
  final String roomId;

  /// @brief 流类型
  final StreamIndex streamIndex;

  /// @brief 视频旋转角度
  final VideoRotation videoRotation;

  /// @brief View ID
  String? viewId;

  /// @brief View 类型
  String viewType = 'surface';

  /// @brief 是否有视频
  bool hasVideo = false;

  /// @brief 是否有音频
  bool hasAudio = false;

  /// 设置渲染本地视频的画布
  ///
  /// 应用程序通过调用此接口将画布和本地视频流绑定。
  /// 在应用程序开发中，通常在初始化后调用该方法进行本地视频设置，然后再加入房间，退出房间后绑定仍然有效。
  /// 调用 removeLocalVideo 可解除绑定。
  RTCViewContext.localContext({
    required this.userId,
    this.streamIndex = StreamIndex.main,
    this.videoRotation = VideoRotation.rotation0,
  })  : canvasType = VideoCanvasType.local,
        roomId = '',
        viewId = genViewId('', userId);

  /// @brief 设置渲染远端视频的画布
  ///
  /// 应用程序通过调用此接口将画布和远端视频流绑定。退出房间后绑定仍然有效。
  /// 调用 removeRemoteVideo 可解除绑定。
  RTCViewContext.remoteContext({
    required this.roomId,
    required this.userId,
    this.streamIndex = StreamIndex.main,
    this.videoRotation = VideoRotation.rotation0,
    this.streamId = '',
  })  : canvasType = VideoCanvasType.remote,
        viewId = genViewId(roomId, userId);

  /// @brief 设置渲染 WTN 公共流视频的画布
  ///
  /// 应用程序通过调用此接口将画布和 WTN 公共流视频流绑定。退出房间后绑定仍然有效。
  /// 调用 removeRemoteVideo 可解除绑定。
  RTCViewContext.publicStreamContext(String publicStreamId)
      : canvasType = VideoCanvasType.public_stream,
        roomId = '',
        userId = publicStreamId,
        streamId = publicStreamId,
        streamIndex = StreamIndex.main,
        videoRotation = VideoRotation.rotation0,
        viewId = genPublicStreamId(publicStreamId);

  RTCViewContext.echoTestContext()
      : canvasType = VideoCanvasType.echo_test,
        roomId = '',
        userId = '',
        streamIndex = StreamIndex.main,
        videoRotation = VideoRotation.rotation0,
        viewId = genEchoTestViewId();
}

/// @brief 视频渲染设置。
///
/// 若使用 Flutter 3.0.0 及以上版本开发 Android 应用，建议使用 Android 6.0 及以上设备，否则会出现图层显示错误。
///
/// 不同平台对应不同对象：
/// + Android：[TextureView](https://developer.android.com/reference/android/view/TextureView).
/// + iOS：[UIView](https://developer.apple.com/documentation/uikit/uiview).
class RTCSurfaceView extends StatefulWidget {
  /// @brief 传入 context 用于实例初始化
  final RTCViewContext context;

  /// @brief 视频渲染模式
  final VideoRenderMode renderMode;

  /// @brief 视频旋转角度
  final VideoRotation renderRotation;

  /// @brief 用于填充画布空白部分的背景颜色
  ///
  /// 取值范围是 `[0x0000000, 0xFFFFFFFF]`，默认值是 `0x00000000`。
  final int backgroundColor;

  /// @brief 设置 `SurfaceView` 的 Surface 是否放置在本身所在窗口的最顶部。
  ///
  /// 具体参看 [setZOrderOnTop](https://developer.android.com/reference/android/view/SurfaceView#setZOrderOnTop(boolean))。仅适用于 Android。
  final bool zOrderOnTop;

  /// @brief 设置 `SurfaceView` 的 Surface 是否放置在另一个常规 `SurfaceView` 的顶部。
  ///
  /// 具体参看 [setZOrderMediaOverlay](https://developer.android.com/reference/android/view/SurfaceView#setZOrderMediaOverlay(boolean))。仅适用于 Android。
  final bool zOrderMediaOverlay;

  /// @brief `PlatformView` 被创建时，收到此回调
  final PlatformViewCreatedCallback? onPlatformViewCreated;

  /// @brief 应将哪些手势传给 `PlatformView`
  ///
  /// + iOS 参看 [gestureRecognizers property](https://api.flutter.dev/flutter/widgets/UiKitView/gestureRecognizers.html)。
  /// + Android 参看 [gestureRecognizers property](https://api.flutter.dev/flutter/widgets/AndroidView/gestureRecognizers.html)。
  final Set<Factory<OneSequenceGestureRecognizer>>? gestureRecognizers;

  const RTCSurfaceView({
    Key? key,
    required this.context,
    this.renderMode = VideoRenderMode.fit,
    this.renderRotation = VideoRotation.rotation0,
    this.backgroundColor = 0,
    this.zOrderOnTop = false,
    this.zOrderMediaOverlay = false,
    this.onPlatformViewCreated,
    this.gestureRecognizers,
  }) : super(key: key);

  @override
  State<StatefulWidget> createState() => RTCSurfaceViewState();
}
