export interface OpenStreamRequest {
  streamId: string;
  streamType: string;
  scopeKind: string;
  scopeId: string;
  durabilityClass: 'transient' | 'durableSession' | 'eventLog';
  schemaRef?: string;
}
