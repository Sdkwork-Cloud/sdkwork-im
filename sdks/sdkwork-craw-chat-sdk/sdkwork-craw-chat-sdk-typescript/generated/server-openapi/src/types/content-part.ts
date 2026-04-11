import type { MediaResource } from './media-resource';

export interface ContentPart {
  kind: 'text' | 'data' | 'media' | 'signal' | 'stream_ref';
  text?: string;
  schemaRef?: string;
  encoding?: string;
  payload?: string;
  mediaAssetId?: string;
  resource?: MediaResource;
  signalType?: string;
  streamId?: string;
  streamType?: string;
  state?: string;
}
