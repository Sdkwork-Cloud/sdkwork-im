/// Copyright (c) 2026 Beijing Volcano Engine Technology Ltd.
// SPDX-License-Identifier: MIT

import 'package:flutter/foundation.dart';
import 'package:flutter/services.dart';
import 'package:flutter/widgets.dart';

import '../api/bytertc_render_view.dart';
import '../api/bytertc_video_api.dart';
import '../codegen/pack/keytype.dart' as $p;
import 'helper.dart';

Map<String, String> viewMap = {};

String globalGetViewByViewId(String viewId) => viewMap[viewId]!;

class RTCSurfaceViewState extends State<RTCSurfaceView> {
  RTCSurfaceViewState();

  @override
  Widget build(BuildContext context) {
    if (defaultTargetPlatform == TargetPlatform.android) {
      return GestureDetector(
        behavior: HitTestBehavior.opaque,
        child: AndroidView(
          viewType: 'ByteRTCSurfaceView',
          creationParams: {'type': widget.context.viewType},
          creationParamsCodec: const StandardMessageCodec(),
          onPlatformViewCreated: (int id) {
            viewMap[widget.context.viewId!] = id.toString();
            _setVideoCanvas();
          },
          gestureRecognizers: widget.gestureRecognizers,
        ),
      );
    } else if (defaultTargetPlatform == TargetPlatform.iOS) {
      return GestureDetector(
        behavior: HitTestBehavior.opaque,
        child: UiKitView(
          viewType: 'ByteRTCSurfaceView',
          creationParams: {'type': widget.context.viewType},
          creationParamsCodec: const StandardMessageCodec(),
          onPlatformViewCreated: (int id) {
            viewMap[widget.context.viewId!] = id.toString();
            _setVideoCanvas();
          },
          gestureRecognizers: widget.gestureRecognizers,
        ),
      );
    }
    return Text('$defaultTargetPlatform is not yet supported by the plugin');
  }

  @override
  void didUpdateWidget(RTCSurfaceView oldWidget) {
    super.didUpdateWidget(oldWidget);
    if (oldWidget.context.streamId != widget.context.streamId ||
        oldWidget.context.canvasType != widget.context.canvasType ||
        oldWidget.context.videoRotation != widget.context.videoRotation) {
      _setVideoCanvas();
    } else if (oldWidget.renderMode != widget.renderMode ||
        oldWidget.backgroundColor != widget.backgroundColor) {
      _updateVideoCanvas();
    }

    if (defaultTargetPlatform == TargetPlatform.android) {
      if (oldWidget.zOrderOnTop != widget.zOrderOnTop) {
        _setZOrderOnTop();
      }
      if (oldWidget.zOrderMediaOverlay != widget.zOrderMediaOverlay) {
        _setZOrderMediaOverlay();
      }
    }
  }

  void _updateVideoCanvas() {
    switch (widget.context.canvasType) {
      case VideoCanvasType.local:
        _updateLocalVideo();
        break;
      case VideoCanvasType.remote:
        _updateRemoteVideo();
        break;
      case VideoCanvasType.public_stream:
        _setPublicVideo();
        break;
      case VideoCanvasType.echo_test:
        _setEchoTestVideo();
        break;
    }
  }

  void _setVideoCanvas() {
    switch (widget.context.canvasType) {
      case VideoCanvasType.local:
        _setLocalVideo();
        break;
      case VideoCanvasType.remote:
        _setRemoteVideo();
        break;
      case VideoCanvasType.public_stream:
        _setPublicVideo();
        break;
      case VideoCanvasType.echo_test:
        _setEchoTestVideo();
        break;
    }
  }

  Future<void> _setLocalVideo() async {
    if (!viewMap.containsKey(widget.context.viewId)) {
      return Future.value();
    }
    final String id = viewMap[widget.context.viewId!]!;
    final view = await byteRTCHelper.getView(id);
    $p.VideoCanvas canvas = $p.VideoCanvas(
      backgroundColor: widget.backgroundColor,
      renderMode: widget.renderMode,
      renderRotation: widget.renderRotation,
      view: view,
    );
    await globalEngine?.setLocalVideoCanvas(canvas);
  }

  Future<void> _updateLocalVideo() async {
    if (!viewMap.containsKey(widget.context.viewId)) {
      return Future.value();
    }
    await globalEngine?.updateLocalVideoCanvas(
      renderMode: widget.renderMode,
      backgroundColor: widget.backgroundColor,
    );
  }

  Future<void> _setRemoteVideo() async {
    if (!viewMap.containsKey(widget.context.viewId)) {
      return Future.value();
    }
    if (widget.context.streamId == null || widget.context.streamId!.isEmpty) {
      return Future.value();
    }
    final String id = viewMap[widget.context.viewId!]!;
    final view = await byteRTCHelper.getView(id);
    $p.VideoCanvas canvas = $p.VideoCanvas(
      backgroundColor: widget.backgroundColor,
      renderMode: widget.renderMode,
      renderRotation: widget.renderRotation,
      view: view,
    );
    await globalEngine?.setRemoteVideoCanvas(
      widget.context.streamId!,
      canvas,
    );
  }

  Future<void> _updateRemoteVideo() async {
    if (!viewMap.containsKey(widget.context.viewId)) {
      return Future.value();
    }
    if (widget.context.streamId == null || widget.context.streamId!.isEmpty) {
      return Future.value();
    }
    await globalEngine?.updateRemoteStreamVideoCanvas(
      streamId: widget.context.streamId!,
      renderMode: widget.renderMode,
      backgroundColor: widget.backgroundColor,
    );
  }

  Future<void> _setPublicVideo() async {
    if (!viewMap.containsKey(widget.context.viewId)) {
      return Future.value();
    }
    if (widget.context.streamId == null || widget.context.streamId!.isEmpty) {
      return Future.value();
    }
    final String id = viewMap[widget.context.viewId!]!;
    final view = await byteRTCHelper.getView(id);
    $p.VideoCanvas canvas = $p.VideoCanvas(
      backgroundColor: widget.backgroundColor,
      renderMode: widget.renderMode,
      renderRotation: widget.renderRotation,
      view: view,
    );
    await globalEngine?.wtnStream.setWTNRemoteVideoCanvas(
      widget.context.streamId!,
      canvas,
    );
  }

  Future<void> _updatePublicVideo() async {
    // not used
  }

  Future<void> _setEchoTestVideo() async {
    // Nothing to do, already recorded the video in map.
    return Future.value();
  }

  Future<void> _setZOrderOnTop() async {
    final String id = viewMap[widget.context.viewId!]!;
    byteRTCHelper.setZOrderOnTop(id, widget.zOrderOnTop);
  }

  Future<void> _setZOrderMediaOverlay() async {
    final String id = viewMap[widget.context.viewId!]!;
    byteRTCHelper.setZOrderMediaOverlay(id, widget.zOrderMediaOverlay);
  }

  static Future<dynamic> getAndroidView(String viewId) async {
    if (viewMap.containsKey(viewId)) {
      return await byteRTCHelper.getView(viewMap[viewId]!);
    }
    return Future.value(null);
  }
}
