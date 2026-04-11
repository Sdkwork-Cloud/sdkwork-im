class ApiPaths {
  static const String apiPrefix = '/api/v1';

  static String backendPath([String path = '']) {
    if (path.isEmpty) return apiPrefix;
    if (path.startsWith('http://') || path.startsWith('https://')) return path;

    final prefixRaw = apiPrefix.trim();
    final normalizedPrefix =
        (prefixRaw.isNotEmpty && prefixRaw != '/') ? '/${prefixRaw.replaceAll(RegExp(r'^/+|/+$'), '')}' : '';
    final normalizedPath = path.startsWith('/') ? path : '/$path';

    if (normalizedPrefix.isEmpty) return normalizedPath;
    if (normalizedPath == normalizedPrefix || normalizedPath.startsWith('$normalizedPrefix/')) {
      return normalizedPath;
    }
    return normalizedPrefix + normalizedPath;
  }
}
