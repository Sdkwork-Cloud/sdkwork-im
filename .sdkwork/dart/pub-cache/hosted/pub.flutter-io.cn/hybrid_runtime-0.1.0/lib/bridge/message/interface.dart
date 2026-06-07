import '../idl/spec.dart';

/// Message client interface that implements both sender and consumer interfaces
abstract class MessageClient implements MessageSender, MessageConsumer {
  @override
  MessageProto get proto;
}

/// Message protocol interface
abstract class MessageProto {
  /// Encode argument list
  List<dynamic> encodeArgs(List<dynamic> args);

  /// Decode argument list
  List<dynamic> decodeArgs(List<dynamic> args);

  /// Register proxy instance
  void registerProxyInstance(String id, dynamic instance);

  /// Find proxy instance
  dynamic findInstance(String id);

  /// Encode a single argument
  dynamic encodeArg(dynamic arg);

  /// Decode a single argument
  dynamic decodeArg(dynamic arg);
}

/// Native method metadata
class NativeMethodMeta {
  final bool? sync;
  final bool? constructor;
  final String? instanceId;

  NativeMethodMeta({this.sync, this.constructor, this.instanceId});
}

/// Message sender interface
abstract class MessageSender {
  /// Send variable get request
  Future<ReturnParams> sendVarGet(String serviceName, String varName);

  /// Send static call request
  Future<ReturnParams> sendStaticCall(
    String serviceName,
    String methodName,
    List<dynamic> args,
  );

  /// Send static call request
  Future<ReturnParams> sendStaticCallSync(
    String serviceName,
    String methodName,
    List<dynamic> args,
  );

  /// Send instance creation request
  Future<ReturnParams> sendNewInstanceCall(
    String serviceName,
    List<dynamic> args,
    String instanceId,
    dynamic instance,
    // with member settled
    Map<String, dynamic>? members,
  );

  /// Send instance method call request
  Future<ReturnParams> sendInstanceCall(
    dynamic instanceOrId,
    String serviceName,
    String methodName,
    List<dynamic> args, [
    NativeMethodMeta? meta,
  ]);

  /// Send instance property get request
  Future<ReturnParams> sendInstanceGet(
    dynamic instanceOrId,
    String serviceName,
    String property,
  );

  /// Send instance event listener request
  Future<ReturnParams> sendInstanceEventAdd(
    dynamic instanceOrId,
    String serviceName,
    String eventName,
  );

  /// Send instance event remove request
  Future<ReturnParams> sendInstanceEventRemove(
    dynamic instanceOrId,
    String serviceName,
    String eventName,
  );

  /// Send instance destroy request
  Future<ReturnParams> sendDestroyInstance(
    dynamic instanceOrId,
    String? serviceName,
  );

  /// Send event trigger request
  Future<ReturnParams> sendEventEmit(
    String instanceId,
    String serviceName,
    String callbackId,
    List<dynamic> args,
  );

  /// Send instance property set request
  Future<ReturnParams> sendInstanceSet(
    dynamic instanceOrId,
    String serviceName,
    String property,
    dynamic value,
  );

  Future<ReturnParams> sendInstancePropertiesGet(
    dynamic instanceOrId,
    String serviceName,
    dynamic nativeClass,
  );
}

/// Message consumer interface
abstract class MessageConsumer {
  MessageProto get proto;
  void registerService(String name, Service service);
  void registerInstance(String id, dynamic instance);
  Future<ReturnParams> handleMessage(CallParams params);
}
