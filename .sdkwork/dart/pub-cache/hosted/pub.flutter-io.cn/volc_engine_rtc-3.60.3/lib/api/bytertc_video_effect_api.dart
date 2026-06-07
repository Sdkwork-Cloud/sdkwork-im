/// Copyright (c) 2026 Beijing Volcano Engine Technology Ltd.

import '../codegen/pack/index.dart' as $p;

/// @nodoc
late dynamic $video_effect_instance;

/// @brief 视频特效类
class VideoEffect extends $p.IVideoEffect {
  @override
  dynamic $createInstance(List<dynamic> args) {
    print('Do nothing, just return video effect instance.');
    return $video_effect_instance;
  }

  VideoEffect();
}
