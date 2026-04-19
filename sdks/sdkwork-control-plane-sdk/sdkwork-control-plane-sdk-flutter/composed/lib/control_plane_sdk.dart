library control_plane_sdk;

export 'package:control_plane_backend_sdk/control_plane_backend_sdk.dart';

export 'src/context.dart';
export 'src/meta_module.dart';
export 'src/nodes_module.dart';
export 'src/protocol_module.dart';
export 'src/providers_module.dart';
export 'src/social_module.dart';
export 'src/social_runtime_module.dart';
export 'src/types.dart';

import 'package:control_plane_backend_sdk/control_plane_backend_sdk.dart';

import 'src/context.dart';
import 'src/meta_module.dart';
import 'src/nodes_module.dart';
import 'src/protocol_module.dart';
import 'src/providers_module.dart';
import 'src/social_module.dart';
import 'src/social_runtime_module.dart';
import 'src/types.dart';

class ControlPlaneSdkClient {
  final ControlPlaneSdkContext _context;

  final ControlPlaneBackendClient backendClient;

  late final ControlPlaneMetaModule meta;
  late final ControlPlaneProtocolModule protocol;
  late final ControlPlaneProvidersModule providers;
  late final ControlPlaneSocialModule social;
  late final ControlPlaneSocialRuntimeModule socialRuntime;
  late final ControlPlaneNodesModule nodes;

  ControlPlaneSdkClient(ControlPlaneSdkClientOptions options)
      : backendClient = options.backendClient,
        _context = ControlPlaneSdkContext(options.backendClient) {
    meta = ControlPlaneMetaModule(_context);
    protocol = ControlPlaneProtocolModule(_context);
    providers = ControlPlaneProvidersModule(_context);
    social = ControlPlaneSocialModule(_context);
    socialRuntime = ControlPlaneSocialRuntimeModule(_context);
    nodes = ControlPlaneNodesModule(_context);
  }

  factory ControlPlaneSdkClient.create({
    ControlPlaneBackendClient? backendClient,
    String? baseUrl,
    String? authToken,
    Map<String, String>? headers,
    int timeout = defaultTimeoutMs,
  }) {
    final resolvedConfig = baseUrl == null
        ? null
        : ControlPlaneBackendConfig(
            baseUrl: baseUrl,
            authToken: authToken,
            headers: headers ?? const <String, String>{},
            timeout: timeout,
          );

    if (backendClient == null && resolvedConfig == null) {
      throw ArgumentError(
        'Provide backendClient or baseUrl when creating ControlPlaneSdkClient.',
      );
    }

    final resolvedBackendClient =
        backendClient ?? ControlPlaneBackendClient(config: resolvedConfig!);

    return ControlPlaneSdkClient(
      ControlPlaneSdkClientOptions(
        backendClient: resolvedBackendClient,
      ),
    );
  }

  void setAuthToken(String token) {
    _context.setAuthToken(token);
  }
}
