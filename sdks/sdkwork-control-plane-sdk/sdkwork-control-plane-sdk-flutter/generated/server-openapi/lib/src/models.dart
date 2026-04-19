import 'package:sdkwork_common_flutter/sdkwork_common_flutter.dart';

typedef JsonObject = Map<String, dynamic>;
typedef QueryParams = Map<String, dynamic>;
typedef Identifier = Object;

const int defaultTimeoutMs = 15000;

class ControlPlaneBackendConfig {
  final String baseUrl;
  final String? authToken;
  final Map<String, String> headers;
  final int timeout;

  const ControlPlaneBackendConfig({
    required this.baseUrl,
    this.authToken,
    this.headers = const <String, String>{},
    this.timeout = defaultTimeoutMs,
  });

  SdkConfig toSdkConfig() {
    return SdkConfig(
      baseUrl: baseUrl,
      timeout: timeout,
      headers: headers,
      authToken: authToken,
    );
  }
}

class AdminApiError implements Exception {
  final int status;
  final Object? payload;
  final String message;

  const AdminApiError({
    required this.status,
    required this.message,
    this.payload,
  });

  @override
  String toString() {
    return 'AdminApiError(status: $status, message: $message, payload: $payload)';
  }
}
