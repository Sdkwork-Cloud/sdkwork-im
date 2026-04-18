export interface CompleteUploadRequest {
  bucket: string;
  objectKey: string;
  storageProvider?: string;
  url: string;
  checksum?: string;
  etag?: string;
}
