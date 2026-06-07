import '../idl/spec.dart';
import '../idl/event.dart';
import '../../_tools/logger/logger.dart';
import '../global.dart';
import 'interface.dart';

/// Create proxy instance
dynamic createProxyInstance(Instance instance) {
  final client = getGlobalMessageClient();
  final serviceName = instance.serviceName!;
  final instanceId = instance.instanceId!;

  // Create proxy object
  final proxy = _ProxyInstance(
    instanceId: instanceId,
    serviceName: serviceName,
    client: client,
  );

  // Register to message client
  client.registerInstance(instanceId, proxy);

  return proxy;
}

/// Proxy instance implementation
class _ProxyInstance extends EventEmitter {
  final String instanceId;
  final String serviceName;
  final dynamic client;
  final Map<String, dynamic> _cache = {};

  _ProxyInstance({
    required this.instanceId,
    required this.serviceName,
    required this.client,
  }) {
    // Listen to property change event
    on('__propertyChange', _handlePropertyChange);
  }

  // Initialize property cache
  Future<void> init() async {
    // Get initial property list
    final result = await (client as MessageSender).sendInstanceCall(
      instanceId,
      serviceName,
      'getProperties', // Call native getProperties method
      [],
    );

    if (result.msg is Map) {
      _cache.addAll(result.msg as Map<String, dynamic>);
    }
  }

  // Handle property change event
  void _handlePropertyChange(dynamic args) {
    if (args is Map) {
      final String prop = args['property'];
      final dynamic value = args['value'];
      _cache[prop] = value;
      // Trigger additional update notifications
      emit('${prop}Change', value);
    }
  }

  @override
  dynamic noSuchMethod(Invocation invocation) async {
    final memberName = invocation.memberName.toString();
    final positionalArgs = invocation.positionalArguments;
    final namedArgs = invocation.namedArguments;

    // Handle property access
    if (invocation.isGetter) {
      // Return cached value directly
      return _cache[memberName];
    }

    if (invocation.isSetter) {
      return _handleSet(memberName, positionalArgs.first);
    }

    // Handle method call
    return _handleMethodCall(
      memberName,
      [...positionalArgs, ...namedArgs.values],
    );
  }

  dynamic _handleSet(String memberName, dynamic value) {
    logger.debug('[proxy] Set property:', [memberName, value]);
    final result = (client as dynamic).sendInstanceSet(
      instanceId,
      serviceName,
      memberName,
      value,
    );
    return result.msg;
  }

  dynamic _handleMethodCall(String methodName, List<dynamic> args) {
    logger.debug('[proxy] Call method:', [methodName, args]);
    return (client as dynamic).sendInstanceCall(
      instanceId,
      serviceName,
      methodName,
      args,
    );
  }

  @override
  void on(String eventName, EventListener listener) {
    super.on(eventName, listener);
    (client as dynamic).sendInstanceEventAdd(
      instanceId,
      serviceName,
      eventName,
    );
  }

  @override
  void off(String eventName, [EventListener? listener]) {
    super.off(eventName, listener);
    if (!hasListeners(eventName)) {
      (client as dynamic).sendInstanceEventRemove(
        instanceId,
        serviceName,
        eventName,
      );
    }
  }

  /// Get namespace
  String? get namespace => serviceName;
}
