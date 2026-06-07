/// Interface for property access support
abstract class PropertyAccessible {
  dynamic getProperty(String name);
  void setProperty(String name, dynamic value);
}

/// Access object properties through a path
dynamic keyPathVisitor(dynamic obj, List<String> path) {
  if (obj == null || path.isEmpty) {
    return obj;
  }

  dynamic current = obj;
  for (final key in path) {
    if (current == null) {
      return null;
    }

    // Handle Map
    if (current is Map) {
      current = current[key];
      continue;
    }

    // Handle List
    if (current is List) {
      final index = int.tryParse(key);
      if (index != null && index >= 0 && index < current.length) {
        current = current[index];
        continue;
      }
      return null;
    }

    // Handle property accessible objects
    if (current is PropertyAccessible) {
      current = current.getProperty(key);
      continue;
    }

    // Handle index access objects
    try {
      current = current[key];
      continue;
    } catch (_) {
      // Ignore access errors
    }

    // Handle property access on regular objects
    try {
      final property = current.$getProperty(key);
      if (property != null) {
        current = property;
        continue;
      }
    } catch (_) {
      // Ignore access errors
    }

    return null;
  }

  return current;
}

/// Extension method for dynamic property access
extension DynamicPropertyAccess on dynamic {
  dynamic $getProperty(String name) {
    try {
      return this[name];
    } catch (_) {
      return null;
    }
  }
}
