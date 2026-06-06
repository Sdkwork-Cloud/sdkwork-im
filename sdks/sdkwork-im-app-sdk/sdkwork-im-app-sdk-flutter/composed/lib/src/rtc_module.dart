import 'context.dart';
import 'types.dart';

class ImAppRtcModule {
  final ImAppSdkContext context;

  ImAppRtcModule(this.context);

  Future<ImAppJsonObject?> createProviderCallback() async {
    return imAppAsJsonObject(
      await context.transportClient.rtc.providerCallbacksCreate(),
    );
  }

  Future<ImAppJsonObject?> providerHealth() async {
    return imAppAsJsonObject(
      await context.transportClient.rtc.providerHealthRetrieve(),
    );
  }
}
