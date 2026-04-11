import 'package:backend_sdk/backend_sdk.dart';

import 'context.dart';
import 'types.dart';

class CrawChatRealtimeModule {
  final CrawChatSdkContext context;

  CrawChatRealtimeModule(this.context);

  Future<RealtimeSubscriptionSnapshot?> replaceSubscriptions(
    SyncRealtimeSubscriptionsRequest body,
  ) {
    return context.backendClient.realtime.syncRealtimeSubscriptions(body);
  }

  Future<RealtimeEventWindow?> pullEvents([CrawChatQueryParams? params]) {
    return context.backendClient.realtime.listRealtimeEvents(params);
  }

  Future<RealtimeAckState?> ackEvents(AckRealtimeEventsRequest body) {
    return context.backendClient.realtime.ackRealtimeEvents(body);
  }
}
