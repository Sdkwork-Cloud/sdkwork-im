export 'is.dart';
export 'map.dart';
export 'stringify.dart';
export 'uuid.dart';

/// Visits a nested map structure using a path of keys
/// Returns the value at the specified path or a default value if not found
T keyPathVisitor<T>(
  Map<String, dynamic> obj,
  List<String> path, [
  T? defaultValue,
]) {
  dynamic result = obj;
  for (final key in path) {
    if (result is! Map) {
      return defaultValue as T;
    }
    result = result[key];
    if (result == null) {
      return defaultValue as T;
    }
  }
  return result as T;
}
