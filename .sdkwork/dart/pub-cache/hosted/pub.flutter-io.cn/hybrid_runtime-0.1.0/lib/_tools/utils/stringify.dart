import 'dart:convert';
import 'is.dart';

/// Safely converts a value to a string
String safeStringify(dynamic value) {
  try {
    if (isNullOrUndefined(value)) {
      return 'null';
    }

    if (isString(value)) {
      return value;
    }

    if (isFunction(value)) {
      return 'function';
    }

    if (isErrorLike(value)) {
      if (value is Error) {
        return value.toString();
      }
      if (value is Exception) {
        return value.toString();
      }
      return jsonEncode(value);
    }

    return jsonEncode(value);
  } catch (e) {
    return value.toString();
  }
}

/// Formats log content
String stringifyLog(List<dynamic> content) {
  return content.map((item) => safeStringify(item)).join(' ');
}

/// Formats key-value pairs
String stringifyKV(Map<String, dynamic> kv) {
  final pairs = kv.entries.map((entry) {
    final key = entry.key;
    final value = safeStringify(entry.value);
    return '$key=$value';
  });
  return pairs.join(' ');
}
