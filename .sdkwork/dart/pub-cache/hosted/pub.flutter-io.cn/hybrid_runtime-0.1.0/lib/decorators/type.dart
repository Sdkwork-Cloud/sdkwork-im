import '../bridge/global.dart';
import '../bridge/idl/spec.dart';
import '../bridge/message/interface.dart';

/// Unified management of native resources
class NativeResource {
  String instanceId;
  String className;
  MessageClient client;
  NativeResource({
    this.instanceId = '',
    this.className = '',
    MessageClient? client,
  }) : client = client ?? getGlobalMessageClient();
  void update({String? instanceId, String? className, MessageClient? client}) {
    if (instanceId != null) this.instanceId = instanceId;
    if (className != null) this.className = className;
    if (client != null) this.client = client;
  }

  void destroy() {
    client.sendDestroyInstance(instanceId, className);
  }
}

/// Unified management of native resources
class NativeClassOptions {
  final String className;
  final InstanceType instanceType;
  final String? instanceId;
  final List<dynamic>? constructorArgs;
  final Map<String, dynamic>? members;
  final Map<String, String>? methodMap;
  final bool disableInit;
  final String? bridgeKey;

  const NativeClassOptions(
    this.constructorArgs, {
    this.members,
    this.className = '',
    this.instanceType = InstanceType.manual,
    this.instanceId,
    this.methodMap,
    this.disableInit = false,
    this.bridgeKey,
  });

  /**
   * @description 将 NativeClassOptions 实例序列化为 Map。
   * @return {Map<String, dynamic>} 包含对象数据的 Map。
   */
  Map<String, dynamic> toMap() {
    return {
      'className': className,
      'instanceType': instanceType.name,
      'instanceId': instanceId,
      'constructorArgs': constructorArgs,
      'members': members,
      'methodMap': methodMap,
      'disableInit': disableInit,
      'bridgeKey': bridgeKey,
    };
  }

  /**
   * @description 从 Map 反序列化，创建一个 NativeClassOptions 实例。
   * @param {Map<String, dynamic>} map - 包含对象数据的 Map。
   * @return {NativeClassOptions} 一个新的 NativeClassOptions 实例。
   */
  factory NativeClassOptions.fromMap(Map<String, dynamic> map) {
    return NativeClassOptions(
      map['constructorArgs'] != null
          ? List<dynamic>.from(map['constructorArgs'])
          : null,
      members: map['members'] != null
          ? Map<String, dynamic>.from(map['members'])
          : null,
      className: map['className'] as String? ?? '',
      instanceType: (map['instanceType'] as String?) != null
          ? InstanceType.values.firstWhere(
              (e) => e.name == map['instanceType'],
              orElse: () => InstanceType.manual,
            )
          : InstanceType.manual,
      instanceId: map['instanceId'] as String?,
      methodMap: map['methodMap'] != null
          ? Map<String, String>.from(map['methodMap'])
          : null,
      disableInit: map['disableInit'] as bool? ?? false,
      bridgeKey: map['bridgeKey'] as String?,
    );
  }
}
