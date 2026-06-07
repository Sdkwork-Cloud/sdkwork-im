/// Copyright (c) 2026 Beijing Volcano Engine Technology Ltd.

import 'dart:io';

import 'package:hybrid_runtime/hybrid_runtime.dart';

import '../codegen/pack/index.dart' as $p;
import '../codegen/android/index.dart' as $a;
import '../codegen/ios/index.dart' as $i;
import 'bytertc_range_audio_api.dart';
import 'bytertc_spatial_audio_api.dart';

export '../codegen/pack/api.dart' show RTCRoom;

/// @nodoc
Map<String, dynamic> $room_map = {};

/// @brief 房间类
class RTCRoom extends $p.RTCRoom {
  @override
  dynamic $createInstance(List<dynamic> args) {
    print('Do nothing, just return room instance.');
    return $room_map[roomId];
  }

  RTCRoom(this.roomId,
      {bool autoInitRangeAudio = false, bool autoInitSpatialAudio = false}) {
    if (autoInitRangeAudio) {
      _initRangeAudio();
    }
    if (autoInitSpatialAudio) {
      _initSpatialAudio();
    }
  }

  late RTCRangeAudio _rangeAudioImpl;
  late RTCSpatialAudio _spatialAudioImpl;

  final String roomId;

  Future<void> _initRangeAudio() async {
    dynamic audio_range_instance;
    if (Platform.isAndroid) {
      audio_range_instance = await ($instance as $a.RTCRoom).getRangeAudio();
    } else if (Platform.isIOS) {
      audio_range_instance =
          await ($instance as $i.ByteRTCRoom).getRangeAudio();
    } else {
      throw UnsupportedError('Platform not supported');
    }
    if (audio_range_instance != null) {
      $audio_range_map[roomId] = audio_range_instance;
    }
  }

  Future<void> _initSpatialAudio() async {
    dynamic audio_spatial_instance;
    if (Platform.isAndroid) {
      audio_spatial_instance =
          await ($instance as $a.RTCRoom).getSpatialAudio();
    } else if (Platform.isIOS) {
      audio_spatial_instance =
          await ($instance as $i.ByteRTCRoom).getSpatialAudio();
    } else {
      throw UnsupportedError('Platform not supported');
    }
    if (audio_spatial_instance != null) {
      $audio_spatial_map[roomId] = audio_spatial_instance;
    }
  }

  RTCRangeAudio get rangeAudio {
    _rangeAudioImpl = RTCRangeAudio(roomId);
    return _rangeAudioImpl;
  }

  RTCSpatialAudio get spatialAudio {
    _spatialAudioImpl = RTCSpatialAudio(roomId);
    return _spatialAudioImpl;
  }

  /// @brief 获取空间音频对象
  @override
  Future<RTCSpatialAudio?> getSpatialAudio() async {
    await _initSpatialAudio();
    return spatialAudio;
  }

  /// @brief 获取单流推流对象
  @override
  Future<RTCRangeAudio?> getRangeAudio() async {
    await _initRangeAudio();
    return rangeAudio;
  }

  /// @brief 设置 RTC Room 事件回调
  Future<int> setRTCRoomEventHandler($p.IRTCRoomEventHandler handler) async {
    $android() {
      return ($instance as $a.RTCRoom).setRTCRoomEventHandler(
        packObject(handler, () => $p.android_IRTCRoomEventHandler()),
      );
    }

    $ios() {
      try {
        ($instance as $i.ByteRTCRoom).delegate = packObject(
          handler,
          () => $p.ios_IRTCRoomEventHandler(),
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
}
