import '../bridge/message/interface.dart';
import '../bridge/message/index.dart';
import 'type.dart';
import 'package:hybrid_runtime/_tools/logger/logger.dart';

/// Native Class mixin
mixin NativeClassMixin {
  NativeResource get $resource;

  String get _instanceId => $resource.instanceId;
  String get _namespace => $resource.className;
  MessageClient get _messageClient => $resource.client;

  /// Call instance method
  Future<T> nativeCall<T>(
    String method, [
    List<dynamic>? args,
    NativeMethodMeta? meta,
  ]) async {
    final result = await _messageClient.sendInstanceCall(
      _instanceId,
      _namespace,
      method,
      args ?? [],
      meta,
    );
    final decoded = $resource.client.proto.decodeArg(result.msg);
    return decoded;
  }

  /// Get instance property
  Future<T> sendInstanceGet<T>(String property) async {
    final result = await _messageClient.sendInstanceGet(
      _instanceId,
      _namespace,
      property,
    );
    logger.debug(
        'Instance get result:', [result, result.status, property, _instanceId]);
    final decoded = $resource.client.proto.decodeArg(result.msg);
    return decoded;
  }

  /// Set instance property
  Future<void> sendInstanceSet(String property, dynamic value) async {
    await _messageClient.sendInstanceSet(
      _instanceId,
      _namespace,
      property,
      value,
    );
  }

  /// Get instance properties
  Future<Map<String, dynamic>> sendInstancePropertiesGet(dynamic nativeClass) async {
    final result = await _messageClient.sendInstancePropertiesGet(
      _instanceId,
      _namespace,
      nativeClass,
    );
    logger.debug(
        'Instance properties get result:', [result, result.status, _instanceId]);
    final decoded = $resource.client.proto.decodeArg(result.msg);
    return decoded;
  }
}
