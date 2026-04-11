export interface MediaDownloadUrlResponse {
  mediaAssetId: string;
  storageProvider: string;
  downloadUrl: string;
  expiresInSeconds: number;
}
