/// Copyright (c) 2026 Beijing Volcano Engine Technology Ltd.

import '../codegen/pack/index.dart' as $p;

/// @nodoc
late dynamic $audio_device_manager_instance;

/// @nodoc
class AudioDeviceManager extends $p.AudioDeviceManager {
  @override
  dynamic $createInstance(List<dynamic> args) {
    print('Do nothing, just return audio device manager instance.');
    return $audio_device_manager_instance;
  }

  /// @nodoc
  AudioDeviceManager();
}
