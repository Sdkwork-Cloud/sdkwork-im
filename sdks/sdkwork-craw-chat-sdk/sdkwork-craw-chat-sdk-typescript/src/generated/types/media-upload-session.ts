import type { StringMap } from './string-map.js';

export interface MediaUploadSession {
  assetId: string;
  storageProvider: string;
  bucket: string;
  objectKey: string;
  method: string;
  url: string;
  headers: StringMap;
  expiresAt: string;
}
