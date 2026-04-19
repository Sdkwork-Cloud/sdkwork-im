const String backendApiPrefix = '';

String backendApiPath(String path) {
  if (path.isEmpty) {
    return backendApiPrefix;
  }
  if (path.startsWith('http://') || path.startsWith('https://')) {
    return path;
  }
  final normalizedPrefixRaw = backendApiPrefix.trim();
  final normalizedPrefix = normalizedPrefixRaw.isEmpty
      ? ''
      : '/${normalizedPrefixRaw.replaceAll(RegExp(r'^/+|/+$'), '')}';
  final normalizedPath = path.startsWith('/') ? path : '/$path';
  if (normalizedPrefix.isEmpty || normalizedPrefix == '/') {
    return normalizedPath;
  }
  if (normalizedPath == normalizedPrefix ||
      normalizedPath.startsWith('$normalizedPrefix/')) {
    return normalizedPath;
  }
  return '$normalizedPrefix$normalizedPath';
}
