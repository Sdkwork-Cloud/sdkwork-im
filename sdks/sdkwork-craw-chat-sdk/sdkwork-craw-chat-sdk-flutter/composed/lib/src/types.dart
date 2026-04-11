import 'package:backend_sdk/backend_sdk.dart';

typedef CrawChatQueryParams = Map<String, dynamic>;

class CrawChatClientOptions {
  final SdkworkBackendClient backendClient;

  const CrawChatClientOptions({
    required this.backendClient,
  });
}

class CrawChatAppendTextFrameOptions {
  final int frameSeq;
  final String text;
  final String? schemaRef;
  final String? encoding;
  final Map<String, String>? attributes;

  const CrawChatAppendTextFrameOptions({
    required this.frameSeq,
    required this.text,
    this.schemaRef,
    this.encoding,
    this.attributes,
  });
}

class CrawChatPostJsonSignalOptions {
  final String? schemaRef;
  final String? signalingStreamId;
  final Object? payload;
  final bool pretty;

  const CrawChatPostJsonSignalOptions({
    this.schemaRef,
    this.signalingStreamId,
    this.payload,
    this.pretty = false,
  });
}

class CrawChatAttachTextMediaOptions {
  final String? conversationId;
  final String? clientMsgId;
  final String? summary;
  final String text;
  final Map<String, String>? renderHints;

  const CrawChatAttachTextMediaOptions({
    this.conversationId,
    this.clientMsgId,
    this.summary,
    required this.text,
    this.renderHints,
  });
}

class CrawChatTextMessageOptions {
  final String? clientMsgId;
  final String? summary;
  final List<ContentPart>? parts;
  final Map<String, String>? renderHints;

  const CrawChatTextMessageOptions({
    this.clientMsgId,
    this.summary,
    this.parts,
    this.renderHints,
  });
}

class CrawChatTextEditOptions {
  final String? summary;
  final List<ContentPart>? parts;
  final Map<String, String>? renderHints;

  const CrawChatTextEditOptions({
    this.summary,
    this.parts,
    this.renderHints,
  });
}
