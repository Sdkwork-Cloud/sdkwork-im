/// Copyright (c) 2026 Beijing Volcano Engine Technology Ltd.

import '../codegen/pack/index.dart' as $p;

/// @nodoc
Map<String, dynamic> $audio_range_map = {};

/// @brief 远端音频管理类
class RTCRangeAudio extends $p.IRangeAudio {
  @override
  dynamic $createInstance(List<dynamic> args) {
    print('Do nothing, just return range audio instance.');
    return $audio_range_map[roomId];
  }

  /// @brief 房间 ID
  final String roomId;

  RTCRangeAudio(this.roomId);
}
