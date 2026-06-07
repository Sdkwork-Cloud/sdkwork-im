/// Copyright (c) 2026 Beijing Volcano Engine Technology Ltd.

import '../codegen/pack/index.dart' as $p;

/// @nodoc
Map<String, dynamic> $audio_spatial_map = {};

/// @brief 空间音频类
class RTCSpatialAudio extends $p.ISpatialAudio {
  @override
  dynamic $createInstance(List<dynamic> args) {
    print('Do nothing, just return spatial audio instance.');
    return $audio_spatial_map[roomId];
  }

  final String roomId;

  RTCSpatialAudio(this.roomId);
}
