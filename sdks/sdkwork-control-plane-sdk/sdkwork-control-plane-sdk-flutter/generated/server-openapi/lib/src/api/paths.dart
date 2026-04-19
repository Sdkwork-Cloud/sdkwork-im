class AdminApiPaths {
  static const String controlPrefix = '/api/v1/control';

  static String backendPath([String path = '']) {
    if (path.isEmpty) {
      return '/';
    }
    if (path.startsWith('http://') || path.startsWith('https://')) {
      return path;
    }
    return path.startsWith('/') ? path : '/$path';
  }

  static String control(String path) {
    final normalizedPath = path.startsWith('/') ? path : '/$path';
    return backendPath('$controlPrefix$normalizedPath');
  }
}
