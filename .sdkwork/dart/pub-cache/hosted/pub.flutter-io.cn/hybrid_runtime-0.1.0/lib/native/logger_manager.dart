import 'dart:io';

import '../_tools/logger/logger.dart';
import 'package:hybrid_runtime/hybrid_runtime.dart';
import './platform/android/index.dart' as android;
import './platform/ios/index.dart' as ios;

class LoggerManager {
  static final logger = LoggerImpl.getInstance();

  LoggerManager._();

  static void setLogLevel(LogLevel level) {
    logger.setLogLevel(level);
    logger.debug('[logger-manager] setLogLevel: $level');
    if (Platform.isAndroid) {
      android.NativeLoggerManager.setLogLevel(level);
    } else if (Platform.isIOS) {
      ios.NativeLoggerManager.setLogLevel(level);
    }
  }
}
