export interface OpenStreamRequest {
    streamId: string;
    streamType: string;
    scopeKind: string;
    scopeId: string;
    durabilityClass: 'transient' | 'durableSession' | 'eventLog';
    schemaRef?: string;
}
//# sourceMappingURL=open-stream-request.d.ts.map