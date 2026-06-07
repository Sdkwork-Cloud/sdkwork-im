import '../bridge/message/interface.dart';

typedef EventHandler<T> = void Function(T event);

abstract class NativeObserver<T> {
  void on(String eventName, EventHandler<T> handler);
  void emit(String eventName, T event);
}

class NativeEventEmitter<T> implements NativeObserver<T> {
  final Map<String, List<EventHandler<T>>> _events = {};
  final MessageClient _messageClient;
  final String _instanceId;
  final String? _namespace;

  NativeEventEmitter(this._messageClient, String className)
      : _instanceId = _generateId(className),
        _namespace = className;

  static String _generateId(String prefix) {
    return '${prefix}_${DateTime.now().millisecondsSinceEpoch}';
  }

  @override
  void on(String eventName, EventHandler<T> handler) {
    var handlers = _events[eventName];

    if (handlers == null) {
      handlers = [];
      _events[eventName] = handlers;
    }

    if (handlers.isEmpty) {
      _messageClient.sendInstanceEventAdd(
        _instanceId,
        _namespace ?? '',
        eventName,
      );
    }

    handlers.add(handler);
  }

  @override
  void emit(String eventName, T event) {
    final handlers = _events[eventName];
    handlers?.forEach((handler) => handler(event));
  }
}
