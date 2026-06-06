import 'context.dart';
import 'types.dart';

class ImAppIotModule {
  final ImAppSdkContext context;

  ImAppIotModule(this.context);

  Future<ImAppJsonObject?> accessProviderHealth() async {
    return imAppAsJsonObject(
      await context.transportClient.iot.accessProviderHealthRetrieve(),
    );
  }

  Future<ImAppJsonObject?> protocolProviderHealth() async {
    return imAppAsJsonObject(
      await context.transportClient.iot.protocolProviderHealthRetrieve(),
    );
  }

  Future<ImAppJsonObject?> ingestProtocolUplink() async {
    return imAppAsJsonObject(
      await context.transportClient.iot.protocolUplinkCreate(),
    );
  }

  Future<ImAppJsonObject?> ingestProtocolDownlink() async {
    return imAppAsJsonObject(
      await context.transportClient.iot.protocolDownlinkCreate(),
    );
  }
}
