import 'package:hybrid_runtime/bridge/global.dart';
import 'package:hybrid_runtime/hybrid_runtime.dart';

class NativeClassUtils {
  static Future<T> nativeStaticCall<T>(
    String namespace,
    String method, [
    List<dynamic>? args, String? key
  ]) async {
    final messageClient = getGlobalMessageClient(key);
    final result = await messageClient.sendStaticCall(
      namespace,
      method,
      args ?? [],
    );
    final decoded = messageClient.proto.decodeArg(result.msg);
    return decoded;
  }

  /// Get static property
  static Future<T> sendStaticGet<T>(String className, String property, [String? key]) async {
    final messageClient = getGlobalMessageClient(key);
    final result = await messageClient.sendVarGet(
      className,
      property,
    );
    final decoded = messageClient.proto.decodeArg(result.msg);
    return decoded;
  }

  static Future<T> sendVarGet<T>(
    String className,
    String varName,
    T Function(String namespace, String instanceId) createInstance,
    [String? key]
  ) async {
    final messageClient = getGlobalMessageClient(key);
    final result = await messageClient.sendVarGet(
      className,
      varName,
    );
    final decoded = messageClient.proto.decodeArg(result.msg);
    if (decoded is Map) {
      final instance = createInstance(className, decoded['_instanceId']);
      return instance;
    }
    return decoded;
  }
}
