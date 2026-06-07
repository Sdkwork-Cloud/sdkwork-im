import 'dart:math';

const _chars = 'abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789';
final _rnd = Random();

/// Generates a random string of specified length
String _randomString(int length) {
  return String.fromCharCodes(
    Iterable.generate(
      length,
      (_) => _chars.codeUnitAt(_rnd.nextInt(_chars.length)),
    ),
  );
}

/// Generates a UUID with optional prefix
String getUuid([String? prefix]) {
  final timestamp = DateTime.now().millisecondsSinceEpoch;
  final random = _randomString(8);
  final uuid = '${prefix ?? ''}${timestamp}_$random';
  return uuid;
}

/// Generates a short UUID with optional prefix
String getShortUuid([String? prefix]) {
  final random = _randomString(8);
  return '${prefix ?? ''}$random';
}
