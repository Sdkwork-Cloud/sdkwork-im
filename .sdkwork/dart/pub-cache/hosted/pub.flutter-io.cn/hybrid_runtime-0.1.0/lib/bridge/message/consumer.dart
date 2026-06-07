import 'package:hybrid_runtime/bridge/message/index.dart';
import 'package:hybrid_runtime/hybrid_runtime.dart';

import '../idl/spec.dart';
import '../../_tools/utils/is.dart';
import '../../_tools/utils/key_path.dart';
import '../../_tools/logger/logger.dart';

/// Message client implementation
/// Handles incoming messages from the native side
class MessageClientImpl extends MessageSenderImpl implements MessageClient {
  @override
  final MessageProtoImpl proto;
  @override
  final dynamic bridge;

  final Map<String, dynamic> _services = {};

  MessageClientImpl(this.proto, this.bridge) : super(proto, bridge);

  @override
  void registerService(String name, Service service) {
    logger.debug('[consumer] Register service:', [name]);
    _services[name] = service;
  }

  @override
  void registerInstance(String id, dynamic instance) {
    logger.debug('[consumer] Register instance:', [id]);
    proto.registerProxyInstance(id, instance);
  }

  @override
  Future<ReturnParams> handleMessage(CallParams params) async {
    try {
      switch (params.callType) {
        case CallType.plainApiCall:
          return _handlePlainApiCall(params);
        case CallType.varGetter:
          return _handleVarGetter(params);
        case CallType.instanceMethodInvoke:
          return _handleInstanceMethodInvoke(params);
        case CallType.instancePropertyGet:
          return _handleInstancePropertyGet(params);
        case CallType.instancePropertySet:
          return _handleInstancePropertySet(params);
        case CallType.instanceEventListenerAdd:
          return _handleInstanceEventAdd(params);
        case CallType.instanceEventListenerRemove:
          return _handleInstanceEventRemove(params);
        case CallType.instanceEventEmit:
          return _handleInstanceEventEmit(params);
        case CallType.callbackEmit:
          return _handleCallbackEmit(params);
        case CallType.destroyInstance:
          return _handleDestroyInstance(params);
        default:
          throw Exception('Unknown call type: ${params.callType}');
      }
    } catch (e) {
      logger.error('[consumer] Handler message failed:', [e]);
      return ReturnParams(
        status: ReturnStatus.failed,
        msg: {'error': e.toString()},
      );
    }
  }

  /// Handle plain API method calls
  ReturnParams _handlePlainApiCall(CallParams params) {
    final service = _findService(params.serviceName);
    final method = _findMethod(service, params.methodName!);
    final args = proto.decodeArgs(params.args ?? []);
    final result = Function.apply(method, args);
    return ReturnParams(status: ReturnStatus.success, msg: result);
  }

  /// Handle variable getter access
  ReturnParams _handleVarGetter(CallParams params) {
    final service = _findService(params.serviceName);
    final args = proto.decodeArgs(params.args ?? []);
    final result = keyPathVisitor(service, args.cast<String>());
    return ReturnParams(status: ReturnStatus.success, msg: result);
  }

  /// Handle instance method invocations
  Future<ReturnParams> _handleInstanceMethodInvoke(CallParams params) async {
    final instance = _findInstance(params.instanceId!);
    final method = _findMethod(instance, params.methodName!);
    final args = proto.decodeArgs(params.args ?? []);
    final result = Function.apply(method, args);

    if (result is Future) {
      final awaitedResult = await result;
      return ReturnParams(status: ReturnStatus.success, msg: awaitedResult);
    }

    return ReturnParams(status: ReturnStatus.success, msg: result);
  }

  /// Handle instance property getter access
  ReturnParams _handleInstancePropertyGet(CallParams params) {
    final instance = _findInstance(params.instanceId!);
    final result = keyPathVisitor(instance, [params.memberName!]);
    return ReturnParams(status: ReturnStatus.success, msg: result);
  }

  /// Handle instance property setter access
  ReturnParams _handleInstancePropertySet(CallParams params) {
    final instance = _findInstance(params.instanceId!);
    final value = proto.decodeArgs(params.args ?? [])[0];
    instance[params.memberName!] = value;
    return const ReturnParams(status: ReturnStatus.success);
  }

  /// Handle instance event listener registration
  ReturnParams _handleInstanceEventAdd(CallParams params) {
    final instance = _findInstance(params.instanceId!);
    if (!isFunction(instance.on)) {
      throw Exception('Instance does not support events');
    }
    instance.on(params.methodName!, () {});
    return const ReturnParams(status: ReturnStatus.success);
  }

  /// Handle instance event listener removal
  ReturnParams _handleInstanceEventRemove(CallParams params) {
    final instance = _findInstance(params.instanceId!);
    if (!isFunction(instance.off)) {
      throw Exception('Instance does not support events');
    }
    instance.off(params.methodName!);
    return const ReturnParams(status: ReturnStatus.success);
  }

  /// Handle instance event emission
  ReturnParams _handleInstanceEventEmit(CallParams params) {
    final instance = _findInstance(params.instanceId!);
    if (instance is! NativeObserverClass) {
      throw Exception('Instance does not support events');
    }

    final args = proto.decodeArgs(params.args ?? []);
    try {
      // Call instance's emit method and get return value
      final result =
          instance.emit(params.methodName ?? params.serviceName, args);

      // Handle return value
      if (result != null && params.waitResult == true) {
        // Encode return value for transmission
        final encodedResult = proto.encodeArg(result);
        sendEventEmitResult(
            params.instanceId!, params.serviceName, encodedResult);
      }
      return const ReturnParams(status: ReturnStatus.success);
    } catch (e) {
      // Handle exceptions
      return ReturnParams(
        status: ReturnStatus.failed,
        msg: {'error': e.toString()},
      );
    }
  }

  /// Handle callback function execution
  Future<ReturnParams> _handleCallbackEmit(CallParams params) async {
    final callback = proto.decodeArg({
      '_type': ArgType.callback.value,
      '_callbackId': params.instanceId,
    });

    if (!isFunction(callback)) {
      throw Exception('Callback not found');
    }

    final args = proto.decodeArgs(params.args ?? []);
    final result = Function.apply(callback, args);

    if (result is Future) {
      final awaitedResult = await result;
      return ReturnParams(status: ReturnStatus.success, msg: awaitedResult);
    }

    return ReturnParams(status: ReturnStatus.success, msg: result);
  }

  /// Handle instance destruction
  ReturnParams _handleDestroyInstance(CallParams params) {
    return const ReturnParams(status: ReturnStatus.success);
  }

  /// Find a registered service by name
  dynamic _findService(String name) {
    final service = _services[name];
    if (service == null) {
      throw Exception('Service not found: $name');
    }
    return service;
  }

  /// Find a registered instance by ID
  dynamic _findInstance(String id) {
    final instance = proto.findInstance(id);
    if (instance == null) {
      throw Exception('Instance not found: $id');
    }
    return instance;
  }

  /// Find a method on a target object
  dynamic _findMethod(dynamic target, String name) {
    final method = target[name];
    if (!isFunction(method)) {
      throw Exception('Method not found: $name');
    }
    return method;
  }
}
