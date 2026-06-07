import 'dart:async';

import 'package:hybrid_runtime/_tools/logger/logger.dart';

import '../bridge/global.dart';
import '../_tools/utils/uuid.dart';
import 'mixin.dart';
import 'type.dart';

/// Base class for native classes
abstract class NativeClass with NativeClassMixin {
  final NativeResource _resource;
  final List<dynamic> _constructorArgs;
  final Map<String, dynamic>? _members;
  final _ready = Completer<void>();

  /// Whether the instance is initialized
  Future<void> get ready => _ready.future;

  static final _finalizer = Finalizer<NativeResource>((resource) {
    resource.destroy();
  });

  @override
  NativeResource get $resource => _resource;

  void updateResource(NativeResource resource) {
    $resource.update(
      instanceId: resource.instanceId,
      className: resource.className,
      client: resource.client,
    );
  }

  NativeClass(NativeClassOptions options)
    : _resource = NativeResource(
        instanceId: options.instanceId ?? getUuid(options.className),
        className: options.className,
        client: getGlobalMessageClient(options.bridgeKey),
      ),
      _constructorArgs = options.constructorArgs ?? [],
      _members = options.members {
    _finalizer.attach(this, _resource);
    if (!options.disableInit) {
      _init();
    } else {
      _ready.complete();
    }
  }

  Future<void> _init() async {
    try {
      await _resource.client.sendNewInstanceCall(
        _resource.className,
        _constructorArgs,
        _resource.instanceId,
        this,
        _members,
      );
      _ready.complete();
    } catch (e) {
      _ready.completeError(e);
    }
  }

  void destroy() => _resource.destroy();
}

/// Base class for native observer classes
abstract class NativeObserverClass extends NativeClass {
  final Map<String, String>? _methodMap;
  dynamic $instance;
  final Map<String, dynamic>? _eventListeners;

  NativeObserverClass(NativeClassOptions options)
    : _methodMap = options.methodMap ?? const {},
      _eventListeners = {},
      super(
        NativeClassOptions(
          [if (options.methodMap != null) options.methodMap!.values.toList()],
          members: options.members,
          className: options.className,
          instanceId: options.instanceId,
          disableInit: options.disableInit,
          bridgeKey: options.bridgeKey,
        ),
      );

  void registerEvent(String name, dynamic method) {
    _eventListeners![name] = method;
  }

  void updateInstance(dynamic instance) {
    $instance = instance;
  }

  dynamic emit(String name, List<dynamic> args) {
    final methodName = _methodMap?[name] ?? name;
    try {
      final method = _eventListeners?[methodName];
      if (method is Function) {
        return Function.apply(method, args);
      }
    } catch (error) {
      logger.error('Failed to emit event:', [name, error]);
    }
    return null;
  }
}
