/// Copyright (c) 2026 Beijing Volcano Engine Technology Ltd.

import '../codegen/pack/index.dart' as $p;
import '../core/helper.dart';

export '../codegen/pack/keytype.dart' show MediaPlayerConfig;
export '../codegen/pack/callback.dart' show IMediaPlayerEventHandler;

/// @nodoc
late Map<int, dynamic> $media_player_map = {};

/// @brief 媒体播放器
class MediaPlayer extends $p.IMediaPlayer {
  @override
  dynamic $createInstance(List<dynamic> args) {
    print('Do nothing, just return media player instance [$playerId].');
    return $media_player_map[playerId];
  }

  /// @brief 获取媒体播放器状态
  @override
  Future<$p.PlayerState> getState() async {
    if ($media_player_map[playerId] == null) {
      throw Exception('MediaPlayer not created');
    }
    return await byteRTCHelper.getState($media_player_map[playerId]);
  }

  final int playerId;

  MediaPlayer(this.playerId);
}
