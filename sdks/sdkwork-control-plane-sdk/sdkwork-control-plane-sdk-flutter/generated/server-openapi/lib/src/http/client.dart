import 'package:sdkwork_common_flutter/sdkwork_common_flutter.dart';

import '../models.dart';

class AdminHttpClient extends BaseHttpClient {
  AdminHttpClient({
    required ControlPlaneBackendConfig config,
  }) : super(config.toSdkConfig());
}
