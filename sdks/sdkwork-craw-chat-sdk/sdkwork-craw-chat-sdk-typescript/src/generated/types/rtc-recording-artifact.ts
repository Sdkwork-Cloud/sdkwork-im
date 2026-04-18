export interface RtcRecordingArtifact {
  tenantId: string;
  rtcSessionId: string;
  bucket: string;
  objectKey: string;
  storageProvider?: string;
  playbackUrl?: string;
}
