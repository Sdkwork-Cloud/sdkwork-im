import 'package:flutter/foundation.dart';
import '../utils/stringify.dart';
import 'dart:convert';

/// Log level enumeration
/// Defines different levels of logging with corresponding numeric values
/// Higher values indicate more severe log levels
enum LogLevel {
  /// For detailed debugging information
  debug(0),

  /// For general information about program execution
  info(1),

  /// For warning messages about potential issues
  warn(2),

  /// For error messages about serious problems
  error(3),

  /// Disables all logging
  none(999);

  const LogLevel(this.value);
  final int value;
}

/// Log message structure
/// Represents a single log entry with level, content, timestamp and optional extra data
class LogMessage {
  final LogLevel level; // The severity level of the log message
  final List<dynamic> content; // The actual log message content
  final DateTime timestamp; // When the log was created
  final Map<String, dynamic>? extra; // Additional contextual information

  LogMessage({
    required this.level,
    required this.content,
    DateTime? timestamp,
    this.extra,
  }) : timestamp = timestamp ?? DateTime.now();

  @override
  String toString() {
    final prefix = '[${level.name.toUpperCase()}]';
    final message = stringifyLog(content);
    final extraStr = extra != null ? ' ${stringifyKV(extra!)}' : '';
    return '$prefix $message$extraStr';
  }
}

/// Log consumer function type
/// Defines the signature for functions that can consume log messages
/// Used for custom log handling implementations
typedef LoggerConsumer = Future<void> Function(LogMessage message);

/// Logger initialization options
/// Configuration options for setting up the logger
class LoggerInitOptions {
  final String? sessionId; // Unique identifier for the logging session
  final Map<String, String>? hostInfo; // Information about the host environment
  final LogLevel? logLevel; // Minimum log level to capture

  LoggerInitOptions({
    this.sessionId,
    this.hostInfo,
    this.logLevel,
  });
}

/// Logger interface
/// Defines the contract for logger implementations
abstract class ILogger {
  LogLevel get level;
  set level(LogLevel value);

  /// Log a debug level message
  void debug(String message, [List<dynamic>? args]);

  /// Log an info level message
  void info(String message, [List<dynamic>? args]);

  /// Log a warning level message
  void warn(String message, [List<dynamic>? args]);

  /// Log an error level message
  void error(String message, [List<dynamic>? args]);

  /// Log a message at the specified level
  void log(LogLevel level, String message, [List<dynamic>? args]);

  /// Register a new log consumer
  void registerConsumer(LoggerConsumer consumer);

  /// Initialize the logger with the given options
  void init(LoggerInitOptions options);
}

/// Logger implementation
/// A singleton logger that provides logging functionality with multiple consumers
class LoggerImpl implements ILogger {
  static LoggerImpl? _instance;
  static const int MAX_LOG_LENGTH = 4096; // Maximum length for log messages
  final List<LoggerConsumer> _consumers =
      []; // List of registered log consumers
  LogLevel _level = LogLevel.warn; // Current minimum log level
  LoggerInitOptions? _options; // Logger configuration options

  LoggerImpl._();

  /// Get the singleton instance of the logger
  static LoggerImpl getInstance() {
    _instance ??= LoggerImpl._();
    return _instance!;
  }

  @override
  LogLevel get level => _level;

  @override
  set level(LogLevel value) {
    _level = value;
  }

  /// Set the minimum log level for the logger
  void setLogLevel(LogLevel level) {
    _level = level;
  }

  /// Format the log message content
  /// Handles different types of arguments and applies length limits
  String _formatMessage(List<dynamic> args) {
    try {
      final message = args.map((arg) {
        if (arg is Map || arg is List) {
          return const JsonEncoder.withIndent('  ').convert(arg);
        }
        return arg.toString();
      }).join(' ');

      if (message.length > MAX_LOG_LENGTH) {
        return message.substring(0, MAX_LOG_LENGTH);
      }
      return message;
    } catch (e) {
      return '[Format Error] ${args.join(' ')} - ${e.toString()}';
    }
  }

  /// Format the timestamp for log messages
  String _formatDate(DateTime date) {
    String pad(int n) => n.toString().padLeft(2, '0');
    return '${pad(date.hour)}:${pad(date.minute)}:${pad(date.second)}';
  }

  @override
  void init(LoggerInitOptions options) {
    _options = options;
    if (options.logLevel != null) {
      _level = options.logLevel!;
    }
  }

  @override
  void registerConsumer(LoggerConsumer consumer) {
    _consumers.add(consumer);
  }

  @override
  void debug(String message, [List<dynamic>? args]) {
    log(LogLevel.debug, message, args);
  }

  @override
  void info(String message, [List<dynamic>? args]) {
    log(LogLevel.info, message, args);
  }

  @override
  void warn(String message, [List<dynamic>? args]) {
    log(LogLevel.warn, message, args);
  }

  @override
  void error(String message, [List<dynamic>? args]) {
    log(LogLevel.error, message, args);
  }

  @override
  void log(LogLevel level, String message, [List<dynamic>? args]) {
    if (level.value < _level.value) return;

    final timestamp = _formatDate(DateTime.now());
    final formattedMessage =
        _formatMessage([message, if (args != null) ...args]);

    final logMessage = LogMessage(
      level: level,
      content: ['$timestamp $formattedMessage'],
      extra: _options?.hostInfo,
    );

    debugPrint(logMessage.toString());

    for (final consumer in _consumers) {
      try {
        consumer(logMessage);
      } catch (e) {
        debugPrint('[Logger] Consumer error: $e');
      }
    }
  }
}

/// Global logger instance
final logger = LoggerImpl.getInstance();
