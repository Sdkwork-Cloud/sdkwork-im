import 'dart:convert';

import 'package:im_app_api_generated/im_app_api_generated.dart';

import 'context.dart';
import 'types.dart';

class ImAppDeviceModule {
  final ImAppSdkContext context;

  ImAppDeviceModule(this.context);

  Future<DeviceTwinView?> getTwin(String deviceId) {
    return context.transportClient.device.devicesTwinRetrieve(deviceId);
  }

  Future<DeviceTwinView?> updateTwinDesired(
    String deviceId,
    UpdateDeviceTwinDesiredRequest body,
  ) {
    return context.transportClient.device.devicesTwinDesiredUpdate(
      deviceId,
      body,
    );
  }

  Future<DeviceTwinView?> updateTwinReported(
    String deviceId,
    UpdateDeviceTwinReportedRequest body,
  ) {
    return context.transportClient.device.devicesTwinReportedUpdate(
      deviceId,
      body,
    );
  }

  Future<DeviceTwinView?> updateTwinDesiredJson(
    String deviceId,
    ImAppJsonObject desiredState,
  ) {
    return updateTwinDesired(
      deviceId,
      UpdateDeviceTwinDesiredRequest(
        desiredStateJson: jsonEncode(desiredState),
      ),
    );
  }

  Future<DeviceTwinView?> updateTwinReportedJson(
    String deviceId,
    ImAppJsonObject reportedState,
  ) {
    return updateTwinReported(
      deviceId,
      UpdateDeviceTwinReportedRequest(
        reportedStateJson: jsonEncode(reportedState),
      ),
    );
  }
}
