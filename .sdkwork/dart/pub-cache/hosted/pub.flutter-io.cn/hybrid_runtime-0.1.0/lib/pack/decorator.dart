import '../bridge/message/interface.dart';

/// Pack class interface
abstract class PackClassInterface {
  dynamic get instance;
  void init(List<dynamic> args);
  void destroy();
}

/// Pack observer interface
abstract class PackObserverInterface {
  dynamic get instance;
  String get instanceId;
  String? get namespace;
  MessageClient? get messageClient;
}

/// Base class for pack classes
abstract class PackClass implements PackClassInterface {
  dynamic _instance;

  @override
  dynamic get instance => _instance;

  @override
  void init(List<dynamic> args) {
    _instance = createInstance(args);
  }

  @override
  void destroy() {
    _instance?.destroy();
    _instance = null;
  }

  /// Factory method for creating instances
  dynamic createInstance(List<dynamic> args);
}

/// Base class for pack observer classes
abstract class PackObserverClass extends PackClass
    implements PackObserverInterface {
  @override
  final String instanceId;

  @override
  final String? namespace;

  @override
  final MessageClient? messageClient;

  PackObserverClass({
    required this.instanceId,
    this.namespace,
    this.messageClient,
  });

  /// Register event handler
  void registerEventHandler(String eventName, Function handler) {
    messageClient?.sendInstanceEventAdd(
      instanceId,
      namespace ?? '',
      eventName,
    );
  }
}
