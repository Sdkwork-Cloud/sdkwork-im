import '../bridge/idl/spec.dart';

T assertReturn<T>(ReturnParams ret) {
  if (ret.status == ReturnStatus.failed) {
    throw Exception(ret.msg?.toString());
  }

  final value = ret.decoded ?? ret.msg ?? '';
  if (value == null) {
    throw Exception('Return value cannot be null');
  }

  // Convert based on target type
  if (T == String) {
    return value.toString() as T;
  } else if (T == int) {
    return (value is String ? int.parse(value) : value as int) as T;
  } else if (T == double) {
    return (value is String ? double.parse(value) : value as double) as T;
  } else if (T == bool) {
    if (value is String) {
      return (value.toLowerCase() == 'true') as T;
    }
    return value as T;
  }

  return value;
}
