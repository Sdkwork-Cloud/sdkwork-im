import 'package:hybrid_runtime/hybrid_runtime.dart';

/// Wrap native object into specified type instance
/// T is the target type
/// receiver is the native object
/// factory is the constructor of target type
T packObject<T>(dynamic receiver, T Function() factory) {
  if (isPrimitiveValue(receiver)) {
    return receiver as T;
  }

  final instance = factory(); // Create instance through factory method
  if (instance is PackClass) {
    (instance as PackClass).updateInstance(receiver);
  } else if (instance is NativeObserverClass) {
    (instance as NativeObserverClass).updateInstance(receiver);
  } else if (instance is NativeClass) {
    final map = Map<String, dynamic>.from(receiver);
    (instance as NativeClass).updateResource(NativeResource(
      instanceId: map['_instanceId'],
      client: instance.$resource.client,
    ));
  }
  return instance;
}

/// Check if value is primitive type
bool isPrimitiveValue(dynamic value) {
  return value == null || value is num || value is String || value is bool;
}

T unpackObject<T>(dynamic instance) {
  if (instance is! PackClass) {
    return instance as T;
  }
  return instance.$instance as T;
}
