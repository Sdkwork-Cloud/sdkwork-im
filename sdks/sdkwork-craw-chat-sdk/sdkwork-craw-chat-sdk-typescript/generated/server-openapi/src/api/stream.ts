import { backendApiPath } from './paths';
import type { HttpClient } from '../http/client';
import type { QueryParams } from '../types/common';
import type { AbortStreamRequest, AppendStreamFrameRequest, CheckpointStreamRequest, CompleteStreamRequest, OpenStreamRequest, StreamFrame, StreamFrameWindow, StreamSession } from '../types';


export class StreamApi {
  private client: HttpClient;
  
  constructor(client: HttpClient) { 
    this.client = client; 
  }

/** Open a stream session */
  async open(body: OpenStreamRequest): Promise<StreamSession> {
    return this.client.post<StreamSession>(backendApiPath(`/streams`), body, undefined, undefined, 'application/json');
  }

/** List stream frames */
  async listStreamFrames(streamId: string | number, params?: QueryParams): Promise<StreamFrameWindow> {
    return this.client.get<StreamFrameWindow>(backendApiPath(`/streams/${streamId}/frames`), params);
  }

/** Append a frame to a stream */
  async appendStreamFrame(streamId: string | number, body: AppendStreamFrameRequest): Promise<StreamFrame> {
    return this.client.post<StreamFrame>(backendApiPath(`/streams/${streamId}/frames`), body, undefined, undefined, 'application/json');
  }

/** Checkpoint a stream session */
  async checkpoint(streamId: string | number, body: CheckpointStreamRequest): Promise<StreamSession> {
    return this.client.post<StreamSession>(backendApiPath(`/streams/${streamId}/checkpoint`), body, undefined, undefined, 'application/json');
  }

/** Complete a stream session */
  async complete(streamId: string | number, body: CompleteStreamRequest): Promise<StreamSession> {
    return this.client.post<StreamSession>(backendApiPath(`/streams/${streamId}/complete`), body, undefined, undefined, 'application/json');
  }

/** Abort a stream session */
  async abort(streamId: string | number, body: AbortStreamRequest): Promise<StreamSession> {
    return this.client.post<StreamSession>(backendApiPath(`/streams/${streamId}/abort`), body, undefined, undefined, 'application/json');
  }
}

export function createStreamApi(client: HttpClient): StreamApi {
  return new StreamApi(client);
}
