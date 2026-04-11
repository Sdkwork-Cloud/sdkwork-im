import 'dart:convert';

import 'package:backend_sdk/backend_sdk.dart';

import 'types.dart';

class CrawChatBuilders {
  static const String defaultTextFrameEncoding = 'text/plain; charset=utf-8';

  static PostMessageRequest textMessage({
    required String text,
    CrawChatTextMessageOptions options = const CrawChatTextMessageOptions(),
  }) {
    return PostMessageRequest(
      clientMsgId: options.clientMsgId,
      summary: options.summary,
      text: text,
      parts: options.parts,
      renderHints: options.renderHints,
    );
  }

  static EditMessageRequest textEdit({
    required String text,
    CrawChatTextEditOptions options = const CrawChatTextEditOptions(),
  }) {
    return EditMessageRequest(
      summary: options.summary,
      text: text,
      parts: options.parts,
      renderHints: options.renderHints,
    );
  }

  static AppendStreamFrameRequest textFrame(
    CrawChatAppendTextFrameOptions options,
  ) {
    return AppendStreamFrameRequest(
      frameSeq: options.frameSeq,
      frameType: 'text',
      schemaRef: options.schemaRef,
      encoding: options.encoding ?? defaultTextFrameEncoding,
      payload: options.text,
      attributes: options.attributes,
    );
  }

  static PostRtcSignalRequest jsonRtcSignal({
    required String signalType,
    required CrawChatPostJsonSignalOptions options,
  }) {
    final encodedPayload = options.pretty
        ? const JsonEncoder.withIndent('  ').convert(options.payload)
        : jsonEncode(options.payload);
    return PostRtcSignalRequest(
      signalType: signalType,
      schemaRef: options.schemaRef,
      payload: encodedPayload,
      signalingStreamId: options.signalingStreamId,
    );
  }
}
