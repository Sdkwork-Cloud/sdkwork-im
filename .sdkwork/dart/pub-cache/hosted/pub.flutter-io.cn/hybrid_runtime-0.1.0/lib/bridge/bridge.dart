import 'dart:convert';
import 'package:flutter/services.dart';
import 'package:hybrid_runtime/_tools/logger/logger.dart';
import 'package:hybrid_runtime/bridge/idl/spec.dart';

/// Main Flutter SDK class for handling native communication

class DartBridge {
  static final Map<String, DartBridge> _cacheInstances = <String, DartBridge>{} ;
  factory DartBridge.getInstance(String bridgeKey) {
    if (_cacheInstances.containsKey(bridgeKey)) {
      return _cacheInstances[bridgeKey]!;
    } else {
      _cacheInstances[bridgeKey] = DartBridge._(bridgeKey);
      return _cacheInstances[bridgeKey]!;
    }
  }

  DartBridge._(String bridgeKey) {
    _methodChannel = MethodChannel(bridgeKey);
    _methodChannel.setMethodCallHandler((call) async {
      if (call.method == 'onEvent') {
        try {
          final Map<dynamic, dynamic> args = call.arguments;
          if (args['data'] != null) {
            CallParams callParams;
            if (args['data'] is String) {
              final data = json.decode(args['data'] as String);
              callParams = CallParams.fromJson(data);
            } else {
              callParams = CallParams.fromJson(args['data']);
            }

            for (var handler in _eventHandlers) {
              handler(callParams);
            }
          }
        } catch (e) {
          logger.error('Failed to parse event parameters:', [e]);
        }
      }
      return null;
    });
  }

  late final MethodChannel _methodChannel;
  final _eventHandlers = <Function(CallParams callParams)>[];

  /// Call native method
  Future<ReturnParams> callApi(CallParams params) async {
    try {
      logger.info('callApi:', [params.toJson()]);
      final result = await _methodChannel
          .invokeMethod('callApi', {'params': jsonEncode(params.toJson())});

      // Check and handle return value
      if (result == null) {
        throw Exception('Native return value is empty');
      }

      // Ensure result is string type
      String jsonString;
      if (result is String) {
        jsonString = result;
      } else {
        jsonString = jsonEncode(result);
      }

      try {
        final Map<String, dynamic> json = jsonDecode(jsonString);
        logger.info('call Api result:', [json]);
        return ReturnParams.fromJson(json);
      } catch (e) {
        logger.error('Failed to parse JSON:', [jsonString]);
        throw Exception('Invalid return data format: $e');
      }
    } catch (e) {
      logger.error('Failed to call native method:', [e]);
      rethrow;
    }
  }

  /// Add event listener
  void addEventListener(Function(CallParams callParams) handler) {
    _eventHandlers.add(handler);
  }

  /// Remove event listener
  void removeEventListener(Function(CallParams callParams) handler) {
    _eventHandlers.remove(handler);
  }
}
