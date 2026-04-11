library craw_chat_sdk;

export 'package:backend_sdk/backend_sdk.dart';

export 'src/builders.dart';
export 'src/context.dart';
export 'src/conversations_module.dart';
export 'src/device_module.dart';
export 'src/inbox_module.dart';
export 'src/media_module.dart';
export 'src/messages_module.dart';
export 'src/presence_module.dart';
export 'src/realtime_module.dart';
export 'src/rtc_module.dart';
export 'src/session_module.dart';
export 'src/streams_module.dart';
export 'src/types.dart';

import 'package:backend_sdk/backend_sdk.dart';

import 'src/context.dart';
import 'src/conversations_module.dart';
import 'src/device_module.dart';
import 'src/inbox_module.dart';
import 'src/media_module.dart';
import 'src/messages_module.dart';
import 'src/presence_module.dart';
import 'src/realtime_module.dart';
import 'src/rtc_module.dart';
import 'src/session_module.dart';
import 'src/streams_module.dart';
import 'src/types.dart';

class CrawChatClient {
  final CrawChatSdkContext _context;

  final SdkworkBackendClient backendClient;

  late final CrawChatSessionModule session;
  late final CrawChatPresenceModule presence;
  late final CrawChatRealtimeModule realtime;
  late final CrawChatDevicesModule devices;
  late final CrawChatInboxModule inbox;
  late final CrawChatConversationsModule conversations;
  late final CrawChatMessagesModule messages;
  late final CrawChatMediaModule media;
  late final CrawChatStreamsModule streams;
  late final CrawChatRtcModule rtc;

  CrawChatClient(CrawChatClientOptions options)
      : backendClient = options.backendClient,
        _context = CrawChatSdkContext(options.backendClient) {
    session = CrawChatSessionModule(_context);
    presence = CrawChatPresenceModule(_context);
    realtime = CrawChatRealtimeModule(_context);
    devices = CrawChatDevicesModule(_context);
    inbox = CrawChatInboxModule(_context);
    conversations = CrawChatConversationsModule(_context);
    messages = CrawChatMessagesModule(_context);
    media = CrawChatMediaModule(_context);
    streams = CrawChatStreamsModule(_context);
    rtc = CrawChatRtcModule(_context);
  }

  factory CrawChatClient.create({
    SdkworkBackendClient? backendClient,
    SdkworkBackendConfig? backendConfig,
    String? baseUrl,
    String? authToken,
    Map<String, String>? headers,
    int timeout = 30000,
  }) {
    final resolvedConfig = backendConfig ??
        (baseUrl == null
            ? null
            : SdkworkBackendConfig(
                baseUrl: baseUrl,
                timeout: timeout,
                authToken: authToken,
                headers: headers ?? const <String, String>{},
              ));

    if (backendClient == null && resolvedConfig == null) {
      throw ArgumentError(
        'Provide backendClient or baseUrl/backendConfig when creating CrawChatClient.',
      );
    }

    final resolvedBackendClient =
        backendClient ?? SdkworkBackendClient(config: resolvedConfig!);

    return CrawChatClient(
      CrawChatClientOptions(
        backendClient: resolvedBackendClient,
      ),
    );
  }

  void setAuthToken(String token) {
    _context.setAuthToken(token);
  }
}
