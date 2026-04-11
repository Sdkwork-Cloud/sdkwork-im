export interface PostRtcSignalRequest {
  signalType: string;
  schemaRef?: string;
  payload: string;
  signalingStreamId?: string;
}
