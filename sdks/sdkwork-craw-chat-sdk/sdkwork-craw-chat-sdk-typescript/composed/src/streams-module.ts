import { buildTextFrameRequest } from './builders.js';
import type {
  AbortStreamRequest,
  AppendStreamFrameRequest,
  AppendTextFrameOptions,
  CheckpointStreamRequest,
  CompleteStreamRequest,
  OpenStreamRequest,
  QueryParams,
  StreamFrame,
  StreamFrameWindow,
  StreamSession,
} from './types.js';
import type { CrawChatSdkContext } from './sdk-context.js';

export class CrawChatStreamsModule {
  constructor(private readonly context: CrawChatSdkContext) {}

  open(body: OpenStreamRequest): Promise<StreamSession> {
    return this.context.backendClient.stream.open(body);
  }

  listFrames(
    streamId: string | number,
    params?: QueryParams,
  ): Promise<StreamFrameWindow> {
    return this.context.backendClient.stream.listStreamFrames(streamId, params);
  }

  appendFrame(
    streamId: string | number,
    body: AppendStreamFrameRequest,
  ): Promise<StreamFrame> {
    return this.context.backendClient.stream.appendStreamFrame(streamId, body);
  }

  appendTextFrame(
    streamId: string | number,
    options: AppendTextFrameOptions,
  ): Promise<StreamFrame> {
    return this.appendFrame(streamId, buildTextFrameRequest(options));
  }

  checkpoint(
    streamId: string | number,
    body: CheckpointStreamRequest,
  ): Promise<StreamSession> {
    return this.context.backendClient.stream.checkpoint(streamId, body);
  }

  complete(
    streamId: string | number,
    body: CompleteStreamRequest,
  ): Promise<StreamSession> {
    return this.context.backendClient.stream.complete(streamId, body);
  }

  abort(
    streamId: string | number,
    body: AbortStreamRequest,
  ): Promise<StreamSession> {
    return this.context.backendClient.stream.abort(streamId, body);
  }
}
