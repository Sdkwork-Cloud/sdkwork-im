import 'dart:convert';
import 'dart:io';
import 'dart:typed_data';

import '../../_tools/logger/logger.dart';
import '../../_tools/utils/is.dart';
import '../../_tools/utils/uuid.dart';
import '../../decorators/class.dart';
import '../idl/spec.dart';
import 'interface.dart';

/// Message protocol implementation for encoding/decoding messages
class MessageProtoImpl implements MessageProto {
  final Map<String, dynamic> _idMapInstance = {};
  final Map<dynamic, String> _instanceMapId = {};
  final Map<String, dynamic> _idMapCallback = {};
  final Map<dynamic, String> _callbackMapId = {};

  /// Collect all native class instances from arguments
  List<NativeClass> collectNativeClass(List<dynamic> args) {
    //IOS传递的参数为map[] 或 dynamic[]格式，若为map需要对map获取value后进行await
    if (Platform.isIOS) {
      args = args.map((arg) {
        // 类型判断：如果是Map格式则提取value，否则直接使用值
        if (arg is Map && arg.containsKey('value')) {
          return arg['value'];
        } else {
          return arg; // 纯值数组直接返回原值
        }
      }).toList();
    }
    final result = <NativeClass>[];
    for (final arg in args) {
      if (arg is NativeClass) {
        result.add(arg);
      }
      if (isArray(arg)) {
        result.addAll(collectNativeClass(arg));
      }
      if (isObject(arg)) {
        result.addAll(collectNativeClass(arg.values.toList()));
      }
    }
    return result;
  }

  @override
  dynamic decodeArg(dynamic arg) {
    if (isNullOrUndefined(arg)) {
      return null;
    }

    if (!isObject(arg)) {
      return arg;
    }

    final map = (arg as Map).cast<String, dynamic>();
    if (!map.containsKey('_type')) {
      return map.map((key, value) => MapEntry(key, decodeArg(value)));
    }

    switch (map['_type']) {
      case 'base64':
        return _decodeBase64(map);
      case 'instance':
        return _decodeInstance(map);
      case 'callback':
        return _decodeCallback(map);
      default:
        return map;
    }
  }

  @override
  List<dynamic> decodeArgs(List<dynamic> args) {
    return args.map((arg) => decodeArg(arg)).toList();
  }

  @override
  dynamic encodeArg(dynamic arg) {
    if (isNullOrUndefined(arg)) {
      return null;
    }

    if (arg is Uint8List) {
      return {'_type': ArgType.base64.value, '_value': base64Encode(arg)};
    }

    if (isInstance(arg)) {
      String? id = _instanceMapId[arg];
      if (id == null) {
        id = getUuid('instance');
        registerProxyInstance(id, arg);
      }

      return {'_type': ArgType.instance.value, '_instanceId': id};
    }

    if (arg is NativeClass) {
      registerProxyInstance(arg.$resource.instanceId, arg);
      return {
        '_type': ArgType.instance.value,
        '_instanceId': arg.$resource.instanceId,
      };
    }

    if (isFunction(arg)) {
      String? id = _callbackMapId[arg];
      if (id == null) {
        id = getUuid('callback');
        _registerCallback(id, arg);
      }

      return {'_type': ArgType.callback.value, '_callbackId': id};
    }

    if (isArray(arg)) {
      return (arg as List).map((item) => encodeArg(item)).toList();
    }

    if (isObject(arg)) {
      return (arg as Map).map((key, value) => MapEntry(key, encodeArg(value)));
    }

    // get enum value
    if (arg is Enum) {
      try {
        final value = (arg as dynamic).$value;
        if (value != null) {
          return value;
        }
        return arg;
      } catch (e) {
        return arg;
      }
    }

    return arg;
  }

  @override
  List<dynamic> encodeArgs(List<dynamic> args) {
    return args.map((arg) => encodeArg(arg)).toList();
  }

  Map<String, dynamic> encodeMembers(Map<String, dynamic> members) {
    return members.map((key, value) => MapEntry(key, encodeArg(value)));
  }

  @override
  dynamic findInstance(String id) {
    return _idMapInstance[id];
  }

  @override
  void registerProxyInstance(String id, dynamic instance) {
    logger.debug('[proto] Register instance:', [id]);
    _idMapInstance[id] = instance;
    _instanceMapId[instance] = id;
  }

  /// Decode base64 data
  Uint8List _decodeBase64(Map<String, dynamic> arg) {
    return base64Decode(arg['data'] ?? arg['_value']);
  }

  /// Decode callback reference
  dynamic _decodeCallback(Map<String, dynamic> arg) {
    final id = arg['_callbackId'];
    return _findCallback(id);
  }

  /// Decode instance reference
  dynamic _decodeInstance(Map<String, dynamic> arg) {
    final id = arg['_instanceId'];
    return _findInstance(id) ?? arg;
  }

  /// Find a callback by ID
  dynamic _findCallback(String? id) {
    return id != null ? _idMapCallback[id] : null;
  }

  /// Find an instance by ID
  dynamic _findInstance(String? id) {
    return id != null ? _idMapInstance[id] : null;
  }

  /// Register a callback function
  void _registerCallback(String id, Function callback) {
    logger.debug('[proto] Register callback:', [id]);
    _idMapCallback[id] = callback;
    _callbackMapId[callback] = id;
  }
}
