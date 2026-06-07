/// Copyright (c) 2026 Beijing Volcano Engine Technology Ltd.

import '../codegen/pack/index.dart' as $p;

export '../codegen/pack/keytype.dart' show AudioEffectPlayerConfig;

/// Global audioEffect instance
late dynamic $audio_effect_instance;

/// AudioEffectPlayer
class AudioEffectPlayer extends $p.IAudioEffectPlayer {
  @override
  dynamic $createInstance(List<dynamic> args) {
    print('Do nothing, just return audio effect instance.');
    return $audio_effect_instance;
  }

  /// constructor
  AudioEffectPlayer();
}
