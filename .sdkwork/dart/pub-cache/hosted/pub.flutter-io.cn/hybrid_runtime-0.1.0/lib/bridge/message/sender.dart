import '../idl/spec.dart';
import '../../_tools/logger/logger.dart';
import '../../_tools/trace/tracer.dart';
import 'interface.dart';
import 'proto.dart';
import '../global.dart';

/// Message sender implementation for sending messages to the native side
class MessageSenderImpl implements MessageSender {
  final MessageProtoImpl proto;
  final dynamic bridge;

  MessageSenderImpl(this.proto, this.bridge);

  dynamic get _bridge => bridge ?? getBridge();

  /// Wait for native classes to be ready
  Future<void> _readyNativeClass(List<dynamic> nativeClasses) async {
    for (final nativeClass in nativeClasses) {
      await nativeClass.ready;
    }
  }

  @override
  Future<ReturnParams> sendVarGet(String serviceName, String varName) async {
    final span = tracer.collectApiCall({
      'source': 'dart',
      'target': 'native',
      'serviceName': serviceName,
      'callType': CallType.varGetter.toString(),
    });

    try {
      final params = CallParams(
        callType: CallType.varGetter,
        serviceName: serviceName,
        memberName: varName,
      );

      final result = await _bridge.callApi(params);
      span.success(result);
      return result;
    } catch (e) {
      span.fail(e);
      rethrow;
    }
  }

  @override
  Future<ReturnParams> sendStaticCall(
    String serviceName,
    String methodName,
    List<dynamic> args,
  ) async {
    final span = tracer.collectApiCall({
      'source': 'dart',
      'target': 'native',
      'serviceName': serviceName,
      'methodName': methodName,
      'callType': CallType.plainApiCall.toString(),
      'args': args,
    });

    try {
      final params = CallParams(
        callType: CallType.plainApiCall,
        serviceName: serviceName,
        methodName: methodName,
        args: proto.encodeArgs(args),
      );

      final nativeClasses = proto.collectNativeClass(args);
      await _readyNativeClass(nativeClasses);

      final result = await _bridge.callApi(params);
      span.success(result);
      return result;
    } catch (e) {
      span.fail(e);
      rethrow;
    }
  }

  @override
  Future<ReturnParams> sendStaticCallSync(
    String serviceName,
    String methodName,
    List<dynamic> args,
  ) async {
    try {
      final params = CallParams(
        callType: CallType.plainApiCall,
        serviceName: serviceName,
        methodName: methodName,
        args: proto.encodeArgs(args),
      );

      final nativeClasses = proto.collectNativeClass(args);
      await _readyNativeClass(nativeClasses);

      return _bridge.callApi(params);
    } catch (e) {
      logger.error('[message-sender] Static sync call failed:', [e]);
      rethrow;
    }
  }

  @override
  Future<ReturnParams> sendNewInstanceCall(
    String serviceName,
    List<dynamic> args,
    String instanceId,
    dynamic instance,
    // with member settled
    Map<String, dynamic>? members,
  ) async {
    try {
      final params = CallParams(
        callType: CallType.newInstance,
        serviceName: serviceName,
        args: proto.encodeArgs(args),
        members: proto.encodeMembers(members ?? {}),
        instanceId: instanceId,
        instanceType: InstanceType.manual,
      );

      final nativeClasses = proto.collectNativeClass([
        args,
        ...members?.values.toList() ?? [],
      ]);
      await _readyNativeClass(nativeClasses);

      proto.registerProxyInstance(instanceId, instance);
      return _bridge.callApi(params);
    } catch (e) {
      logger.error('[message-sender] New instance call failed:', [e]);
      rethrow;
    }
  }

  @override
  Future<ReturnParams> sendInstanceCall(
    dynamic instanceOrId,
    String namespace,
    String method,
    List<dynamic> args, [
    NativeMethodMeta? meta,
  ]) async {
    final instanceId = instanceOrId is String
        ? instanceOrId
        : proto.encodeArg(instanceOrId)['_instanceId'];

    final params = CallParams(
      callType: CallType.instanceMethodInvoke,
      serviceName: namespace,
      methodName: method,
      args: proto.encodeArgs(args),
      instanceId: instanceId,
    );

    final nativeClasses = proto.collectNativeClass(args);
    await _readyNativeClass(nativeClasses);

    return _bridge.callApi(params);
  }

  @override
  Future<ReturnParams> sendInstanceSet(
    dynamic instanceOrId,
    String serviceName,
    String property,
    dynamic value,
  ) async {
    try {
      final instanceId = instanceOrId is String
          ? instanceOrId
          : proto.encodeArg(instanceOrId)['_instanceId'];

      final params = CallParams(
        callType: CallType.instancePropertySet,
        serviceName: serviceName,
        memberName: property,
        instanceId: instanceId,
        args: proto.encodeArgs([value]),
      );

      //收集所有nativeClass的参数，并等待其实例化完毕
      final nativeClasses = proto.collectNativeClass([value]);
      await _readyNativeClass(nativeClasses);

      return _bridge.callApi(params);
    } catch (e) {
      logger.error('[message-sender] Instance property set failed:', [e]);
      rethrow;
    }
  }

  @override
  Future<ReturnParams> sendInstanceGet(
    dynamic instanceOrId,
    String serviceName,
    String property,
  ) async {
    try {
      final params = CallParams(
        callType: CallType.instancePropertyGet,
        serviceName: serviceName,
        memberName: property,
        instanceId: instanceOrId is String
            ? instanceOrId
            : proto.encodeArg(instanceOrId)['_instanceId'],
      );

      final nativeClasses = proto.collectNativeClass([instanceOrId]);
      await _readyNativeClass(nativeClasses);

      return _bridge.callApi(params);
    } catch (e) {
      logger.error('[message-sender] Instance get failed:', [e]);
      rethrow;
    }
  }

  @override
  Future<ReturnParams> sendInstanceEventAdd(
    dynamic instanceOrId,
    String serviceName,
    String eventName,
  ) {
    try {
      final instanceId = instanceOrId is String
          ? instanceOrId
          : proto.encodeArg(instanceOrId)['_instanceId'];

      final params = CallParams(
        callType: CallType.instanceEventListenerAdd,
        serviceName: serviceName,
        methodName: eventName,
        instanceId: instanceId,
      );

      return _bridge.callApi(params);
    } catch (e) {
      logger.error('[message-sender] Event add failed:', [e]);
      rethrow;
    }
  }

  @override
  Future<ReturnParams> sendInstanceEventRemove(
    dynamic instanceOrId,
    String serviceName,
    String eventName,
  ) {
    try {
      final instanceId = instanceOrId is String
          ? instanceOrId
          : proto.encodeArg(instanceOrId)['_instanceId'];

      final params = CallParams(
        callType: CallType.instanceEventListenerRemove,
        serviceName: serviceName,
        methodName: eventName,
        instanceId: instanceId,
      );

      return _bridge.callApi(params);
    } catch (e) {
      logger.error('[message-sender] Event remove failed:', [e]);
      rethrow;
    }
  }

  @override
  Future<ReturnParams> sendDestroyInstance(
    dynamic instanceOrId,
    String? serviceName,
  ) {
    try {
      final instanceId = instanceOrId is String
          ? instanceOrId
          : proto.encodeArg(instanceOrId)['_instanceId'];

      final params = CallParams(
        callType: CallType.destroyInstance,
        serviceName: serviceName ?? '',
        instanceId: instanceId,
      );

      return _bridge.callApi(params);
    } catch (e) {
      logger.error('[message-sender] Destroy instance failed:', [e]);
      rethrow;
    }
  }

  @override
  Future<ReturnParams> sendEventEmit(
    String instanceId,
    String serviceName,
    String callbackId,
    List<dynamic> args,
  ) async {
    try {
      final params = CallParams(
        callType: CallType.instanceEventEmit,
        serviceName: serviceName,
        instanceId: instanceId,
        args: proto.encodeArgs(args),
      );

      final nativeClasses = proto.collectNativeClass(args);
      await _readyNativeClass(nativeClasses);

      return _bridge.callApi(params);
    } catch (e) {
      logger.error('[message-sender] Event emit failed:', [e]);
      rethrow;
    }
  }

  /// Send event result back to native side
  Future<ReturnParams> sendEventEmitResult(
    String instanceId,
    String serviceName,
    dynamic result,
  ) async {
    try {
      final params = CallParams(
        callType: CallType.instanceEventResult,
        serviceName: serviceName,
        instanceId: instanceId,
        args: proto.encodeArgs([result]),
      );

      final nativeClasses = proto.collectNativeClass([result]);
      await _readyNativeClass(nativeClasses);

      return await _bridge.callApi(params);
    } catch (e) {
      logger.error('[message-sender] Event emit result failed:', [e]);
      rethrow;
    }
  }

  /**
   * Warning: IOS暂不支持正向调用，sendInstancePropertiesGet和member set之间未做强制同步，只能依赖于消息顺序。
   * 场景1: newInstance 之后不能立即调用 sendInstancePropertiesGet。IOS Member Set 和 sendInstancePropertiesGet在时序上属于异步
   * 场景1例子:
   * $p.UserInfo userInfo = $p.UserInfo(extraInfo: "111", uid: "111");
   * print("[YEMOMO TEST]: ${await (userInfo.$instance as $p_i.ByteRTCUserInfo).sendInstancePropertiesGet(userInfo.$instance)}");
   */
  @override
  Future<ReturnParams> sendInstancePropertiesGet(
    dynamic instanceOrId,
    String serviceName,
    dynamic nativeClass,
  ) async {
    try {
      final params = CallParams(
        callType: CallType.instancePropertiesGet,
        serviceName: serviceName,
        instanceId: instanceOrId is String
            ? instanceOrId
            : proto.encodeArg(instanceOrId)['_instanceId'],
      );

      await nativeClass.ready;

      return await _bridge.callApi(params);
    } catch (e) {
      logger.error('[message-sender] Properties get failed:', [e]);
      rethrow;
    }
  }
}
