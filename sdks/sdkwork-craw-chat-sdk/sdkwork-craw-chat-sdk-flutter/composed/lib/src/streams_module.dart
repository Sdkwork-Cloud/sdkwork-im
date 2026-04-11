import 'package:backend_sdk/backend_sdk.dart';

import 'builders.dart';
import 'context.dart';
import 'types.dart';

class CrawChatStreamsModule {
  final CrawChatSdkContext context;

  CrawChatStreamsModule(this.context);

  Future<StreamSession?> open(OpenStreamRequest body) {
    return context.backendClient.stream.open(body);
  }

  Future<StreamFrameWindow?> listFrames(
    String streamId, [
    CrawChatQueryParams? params,
  ]) {
    return context.backendClient.stream.listStreamFrames(streamId, params);
  }

  Future<StreamFrame?> appendFrame(
    String streamId,
    AppendStreamFrameRequest body,
  ) {
    return context.backendClient.stream.appendStreamFrame(streamId, body);
  }

  Future<StreamFrame?> appendTextFrame(
    String streamId,
    CrawChatAppendTextFrameOptions options,
  ) {
    return appendFrame(
      streamId,
      CrawChatBuilders.textFrame(options),
    );
  }

  Future<StreamSession?> checkpoint(
    String streamId,
    CheckpointStreamRequest body,
  ) {
    return context.backendClient.stream.checkpoint(streamId, body);
  }

  Future<StreamSession?> complete(
    String streamId,
    CompleteStreamRequest body,
  ) {
    return context.backendClient.stream.complete(streamId, body);
  }

  Future<StreamSession?> abort(
    String streamId,
    AbortStreamRequest body,
  ) {
    return context.backendClient.stream.abort(streamId, body);
  }
}
