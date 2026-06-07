/// Assertion utility class
class Assert {
  /// Asserts that the condition is true
  static void isTrue(bool condition, [String? message]) {
    if (!condition) {
      throw AssertionError(message ?? 'Assertion failed');
    }
  }

  /// Asserts that the value is not null
  static void notNull(dynamic value, [String? message]) {
    if (value == null) {
      throw AssertionError(message ?? 'Value must not be null');
    }
  }

  /// Asserts that the string is not empty
  static void notEmpty(String? value, [String? message]) {
    if (value == null || value.isEmpty) {
      throw AssertionError(message ?? 'String must not be empty');
    }
  }

  /// Asserts that the list is not empty
  static void notEmptyList(List? list, [String? message]) {
    if (list == null || list.isEmpty) {
      throw AssertionError(message ?? 'List must not be empty');
    }
  }

  /// Asserts that the value is within the specified range
  static void inRange(num value, num min, num max, [String? message]) {
    if (value < min || value > max) {
      throw AssertionError(
        message ?? 'Value must be between $min and $max',
      );
    }
  }
}
