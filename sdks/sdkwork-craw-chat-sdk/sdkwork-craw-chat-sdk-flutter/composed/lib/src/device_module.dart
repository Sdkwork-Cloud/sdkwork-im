import 'package:backend_sdk/backend_sdk.dart';

import 'context.dart';
import 'types.dart';

class CrawChatDevicesModule {
  final CrawChatSdkContext context;

  CrawChatDevicesModule(this.context);

  Future<RegisteredDeviceView?> register(RegisterDeviceRequest body) {
    return context.backendClient.device.register(body);
  }

  Future<DeviceSyncFeedResponse?> getSyncFeed(
    String deviceId, [
    CrawChatQueryParams? params,
  ]) {
    return context.backendClient.device.getDeviceSyncFeed(deviceId, params);
  }
}
