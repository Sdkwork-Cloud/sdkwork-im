import 'package:hybrid_runtime/hybrid_runtime.dart';

class NativeLoggerManager extends NativeClass {
  static const _$namespace = r'com.volcengine.VolcApiEngine.logger.VolcLogger';

  static Future<dynamic> setLogLevel(LogLevel level) async {
    return await NativeClassUtils.nativeStaticCall(
      _$namespace,
      'setLogLevel',
      [level.value],
    );
  }

  NativeLoggerManager()
      : super(const NativeClassOptions(
          [],
          className: _$namespace,
        ));
}
