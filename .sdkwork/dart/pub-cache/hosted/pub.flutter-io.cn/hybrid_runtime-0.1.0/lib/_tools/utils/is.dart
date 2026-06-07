import '../../bridge/idl/spec.dart';

/// Checks if the value is null or undefined
bool isNullOrUndefined(dynamic value) {
  return value == null;
}

/// Checks if the value is an error-like object
bool isErrorLike(dynamic value) {
  if (value == null) return false;
  if (value is Error) return true;
  if (value is Exception) return true;
  if (value is Map) {
    return value.containsKey('message') || value.containsKey('stack');
  }
  return false;
}

/// Checks if the value is an instance
bool isInstance(dynamic value) {
  if (value == null) return false;
  if (value is Instance) return true;
  if (value is Map) {
    return value.containsKey('instanceId') || value.containsKey('serviceName');
  }
  return false;
}

/// Checks if the value is a function
bool isFunction(dynamic value) {
  return value is Function;
}

/// Checks if the value is an object
bool isObject(dynamic value) {
  if (value == null) return false;
  return value is Map;
}

/// Checks if the value is an array
bool isArray(dynamic value) {
  return value is List;
}

/// Checks if the value is a string
bool isString(dynamic value) {
  return value is String;
}

/// Checks if the value is a number
bool isNumber(dynamic value) {
  return value is num;
}

/// Checks if the value is a boolean
bool isBoolean(dynamic value) {
  return value is bool;
}

/// Checks if the value is an instance argument
bool isInstanceArg(dynamic value) {
  return value is Map<String, dynamic> &&
      value['_type'] == ArgType.instance.value;
}

/// Checks if the value is a callback
bool isCallback(dynamic obj) {
  if (obj == null) return false;
  return obj is Map && obj['_type'] == 'callback';
}

/// Checks if the value is a plain object
bool isPlainObject(dynamic obj) {
  if (obj == null) return false;
  return obj is Map<String, dynamic>;
}

/// Checks if the value is a Promise
bool isPromise(dynamic obj) {
  return obj is Future;
}
