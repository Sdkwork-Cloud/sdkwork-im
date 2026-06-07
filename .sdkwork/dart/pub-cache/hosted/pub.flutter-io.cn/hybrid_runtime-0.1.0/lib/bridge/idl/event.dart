import '../../_tools/logger/logger.dart';

/// Event listener function type
typedef EventListener = dynamic Function(dynamic args);

/// Base class for event emitter functionality
/// Provides methods for event subscription, unsubscription, and emission
class EventEmitter {
  final Map<String, Set<EventListener>> _events = {};

  /// Add an event listener for a specific event
  void on(String eventName, EventListener listener) {
    logger.debug('[event] Add listener for event:', [eventName]);
    _events.putIfAbsent(eventName, () => {}).add(listener);
  }

  /// Remove an event listener or all listeners for an event
  void off(String eventName, [EventListener? listener]) {
    if (listener == null) {
      logger.debug('[event] Remove all listeners for event:', [eventName]);
      _events.remove(eventName);
      return;
    }

    logger.debug('[event] Remove listener for event:', [eventName]);
    final listeners = _events[eventName];
    if (listeners != null) {
      listeners.remove(listener);
      if (listeners.isEmpty) {
        _events.remove(eventName);
      }
    }
  }

  /// Emit an event with optional arguments
  /// Returns the result of the last listener execution
  dynamic emit(String eventName, [dynamic args]) {
    logger.debug('[event] Emit event:', [eventName, args]);
    final listeners = _events[eventName];
    if (listeners == null || listeners.isEmpty) {
      return null;
    }

    var result;
    for (final listener in listeners) {
      try {
        result = listener(args);
      } catch (e) {
        logger.error('[event] Event listener execution failed:', [e]);
      }
    }
    return result;
  }

  /// Check if there are any listeners for a specific event
  bool hasListeners(String eventName) {
    final listeners = _events[eventName];
    return listeners != null && listeners.isNotEmpty;
  }

  /// Get the number of listeners for a specific event
  int listenerCount(String eventName) {
    return _events[eventName]?.length ?? 0;
  }

  /// Get all registered event names
  List<String> get eventNames {
    return _events.keys.toList();
  }

  /// Get all listeners for a specific event
  Set<EventListener> listeners(String eventName) {
    return _events[eventName] ?? {};
  }

  /// Remove all event listeners, optionally for a specific event
  void removeAllListeners([String? eventName]) {
    if (eventName != null) {
      _events.remove(eventName);
    } else {
      _events.clear();
    }
  }

  /// Add a one-time event listener that will be removed after first execution
  void once(String eventName, EventListener listener) {
    void onceWrapper(args) {
      off(eventName, onceWrapper);
      listener(args);
    }

    on(eventName, onceWrapper);
  }
}
