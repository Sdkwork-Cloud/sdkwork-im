import type {
  CreateDownloadUrlResponse,
  DriveNode,
  DriveSpace,
  DriveUploaderBlobLike,
  DriveUploaderProfile,
  SdkworkDriveAppClient,
  UploaderRetentionRequest,
  UploaderUploadItem,
  UploadSessionMutationResponse,
} from '@sdkwork/drive-app-sdk';
import { getDriveAppSdkClientWithSession } from '@sdkwork/clawchat-pc-core/sdk/driveAppSdkClient';
import {
  readAppSdkSessionTokens,
  resolveAppSdkOrganizationId,
  resolveAppSdkTenantId,
  resolveAppSdkUserId,
} from '@sdkwork/clawchat-pc-core/sdk/session';

export interface FolderItem {
  id: string;
  name: string;
  fileCount: number;
}

export interface DriveFileItem {
  id: string;
  name: string;
  size: string;
  time: string;
  type: 'pdf' | 'word' | 'image' | 'excel' | 'unknown';
}

export interface DriveDownloadGrant {
  downloadUrl: string;
  expiresAtEpochMs: string;
  method: string;
  signedSourceUrl?: string;
}

export interface DriveService {
  getFolders(): Promise<FolderItem[]>;
  createFolder(name: string): Promise<FolderItem>;
  renameFolder(id: string, newName: string): Promise<void>;
  deleteFolder(id: string): Promise<void>;
  getRecentFiles(): Promise<DriveFileItem[]>;
  deleteFile(id: string): Promise<void>;
  renameFile(id: string, newName: string): Promise<void>;
  uploadFile(file: DriveUploaderBlobLike): Promise<DriveFileItem>;
  createDownload(nodeId: string): Promise<DriveDownloadGrant>;
}

interface DriveServiceOptions {
  appId?: string;
  appResourceId?: string;
  client?: SdkworkDriveAppClient;
  organizationId?: string;
  parentNodeId?: string;
  tenantId?: string;
  uploadProfileCode?: DriveUploaderProfile;
  userId?: string;
}

type NodeListResponseLike = { items?: DriveNode[] };
type SpaceListResponseLike = { items?: DriveSpace[] };

const DRIVE_NODE_RESOURCE_ID_PREFIX = 'drive.node.';
const DEFAULT_APP_ID = 'chat';
const DEFAULT_APP_RESOURCE_TYPE = 'drive-file';
const DEFAULT_APP_RESOURCE_ID = 'chat.pc.drive';
const DEFAULT_SCENE = 'drive';
const DEFAULT_SOURCE = 'pc-drive';
const DEFAULT_UPLOAD_PROFILE: DriveUploaderProfile = 'document';
const DEFAULT_RETENTION: UploaderRetentionRequest = { mode: 'long_term' };
const DEFAULT_PAGE_SIZE = '100';
const DEFAULT_DOWNLOAD_TTL_SECONDS = 300;

function asString(value: unknown): string | undefined {
  return typeof value === 'string' && value.trim().length > 0 ? value.trim() : undefined;
}

function normalizeName(value: string, label: string): string {
  const normalized = value.trim();
  if (!normalized) {
    throw new Error(`${label} is required.`);
  }
  if (normalized.length > 255) {
    throw new Error(`${label} must be 255 characters or fewer.`);
  }
  return normalized;
}

function normalizeResourceSegment(value: string, fallback: string): string {
  const segment = value
    .trim()
    .toLowerCase()
    .replace(/[^a-z0-9]+/gu, '-')
    .replace(/^-+|-+$/gu, '')
    .slice(0, 48);
  return segment || fallback;
}

function cryptoSuffix(): string {
  if (typeof crypto !== 'undefined' && typeof crypto.randomUUID === 'function') {
    return crypto.randomUUID().replace(/-/gu, '').slice(0, 20).toLowerCase();
  }
  throw new Error('Drive resource id generation requires crypto.randomUUID.');
}

export function createDriveNodeResourceId(name: string): string {
  return `${DRIVE_NODE_RESOURCE_ID_PREFIX}${normalizeResourceSegment(name, 'node')}.${cryptoSuffix()}`;
}

function extractSpaces(response: SpaceListResponseLike): DriveSpace[] {
  return Array.isArray(response.items) ? response.items : [];
}

function extractNodes(response: NodeListResponseLike): DriveNode[] {
  return Array.isArray(response.items) ? response.items : [];
}

function extensionOf(fileName: string): string {
  const index = fileName.lastIndexOf('.');
  if (index < 0 || index === fileName.length - 1) {
    return '';
  }
  return fileName.slice(index + 1).toLowerCase();
}

function typeFromFileName(fileName: string): DriveFileItem['type'] {
  const extension = extensionOf(fileName);
  if (extension === 'pdf') {
    return 'pdf';
  }
  if (['doc', 'docx'].includes(extension)) {
    return 'word';
  }
  if (['png', 'jpg', 'jpeg', 'gif', 'webp', 'bmp', 'svg'].includes(extension)) {
    return 'image';
  }
  if (['xls', 'xlsx', 'csv'].includes(extension)) {
    return 'excel';
  }
  return 'unknown';
}

function formatSize(bytes?: string): string {
  const value = typeof bytes === 'string' ? Number(bytes) : undefined;
  if (typeof value !== 'number' || !Number.isFinite(value) || value < 0) {
    return 'Unknown';
  }
  if (value < 1024) {
    return `${value} B`;
  }
  if (value < 1024 * 1024) {
    return `${Math.round(value / 1024)} KB`;
  }
  return `${(value / 1024 / 1024).toFixed(1)} MB`;
}

function mapFolder(node: DriveNode): FolderItem {
  return {
    fileCount: 0,
    id: node.id,
    name: node.nodeName,
  };
}

function mapFile(node: DriveNode, sizeBytes?: string): DriveFileItem {
  return {
    id: node.id,
    name: node.nodeName,
    size: formatSize(sizeBytes),
    time: 'Unknown',
    type: typeFromFileName(node.nodeName),
  };
}

function mapUploaderResult(uploadItem: UploaderUploadItem, uploadSession: UploadSessionMutationResponse): DriveFileItem {
  return {
    id: uploadSession.nodeId || uploadItem.nodeId,
    name: uploadItem.originalFileName,
    size: formatSize(uploadItem.contentLength),
    time: 'Unknown',
    type: typeFromFileName(uploadItem.originalFileName),
  };
}

function isActiveNode(node: DriveNode): boolean {
  return node.lifecycleStatus === 'active';
}

function requireValue(value: unknown, label: string): string {
  const normalized = asString(value);
  if (!normalized) {
    throw new Error(`${label} is required.`);
  }
  return normalized;
}

function pickTenantId(explicit?: string): string {
  return requireValue(explicit ?? resolveAppSdkTenantId(readAppSdkSessionTokens()), 'Drive tenant id');
}

function pickUserId(explicit?: string): string {
  return requireValue(explicit ?? resolveAppSdkUserId(readAppSdkSessionTokens()), 'Drive user id');
}

function pickOrganizationId(explicit?: string): string | undefined {
  return explicit ?? resolveAppSdkOrganizationId(readAppSdkSessionTokens());
}

class SdkworkDriveService implements DriveService {
  constructor(private readonly options: DriveServiceOptions = {}) {}

  async getFolders(): Promise<FolderItem[]> {
    const space = await this.resolveSpace();
    const response = await this.client().drive.nodes.list(space.id, {
      pageSize: DEFAULT_PAGE_SIZE,
      parentNodeId: this.options.parentNodeId,
      tenantId: this.tenantId(),
    });
    return extractNodes(response)
      .filter((node) => isActiveNode(node) && node.nodeType === 'folder')
      .map(mapFolder);
  }

  async createFolder(name: string): Promise<FolderItem> {
    const folderName = normalizeName(name, 'Folder name');
    const space = await this.resolveSpace();
    const node = await this.client().drive.nodes.folders.create({
      id: createDriveNodeResourceId(folderName),
      nodeName: folderName,
      operatorId: this.userId(),
      parentNodeId: this.options.parentNodeId,
      spaceId: space.id,
      tenantId: this.tenantId(),
    });
    return mapFolder(node);
  }

  async renameFolder(id: string, newName: string): Promise<void> {
    await this.renameNode(id, newName);
  }

  async deleteFolder(id: string): Promise<void> {
    await this.deleteNode(id);
  }

  async getRecentFiles(): Promise<DriveFileItem[]> {
    const response = await this.client().drive.recent.list({
      pageSize: DEFAULT_PAGE_SIZE,
      tenantId: this.tenantId(),
    });
    return extractNodes(response)
      .filter((node) => isActiveNode(node) && node.nodeType === 'file')
      .map((node) => mapFile(node));
  }

  async deleteFile(id: string): Promise<void> {
    await this.deleteNode(id);
  }

  async renameFile(id: string, newName: string): Promise<void> {
    await this.renameNode(id, newName);
  }

  async uploadFile(file: DriveUploaderBlobLike): Promise<DriveFileItem> {
    const space = await this.resolveSpace();
    const userId = this.userId();
    const result = await this.client().uploader.uploadByProfile(
      this.options.uploadProfileCode ?? DEFAULT_UPLOAD_PROFILE,
      {
        anonymousId: undefined,
        appId: this.appId(),
        appResourceId: this.options.appResourceId ?? DEFAULT_APP_RESOURCE_ID,
        appResourceType: DEFAULT_APP_RESOURCE_TYPE,
        file,
        organizationId: this.organizationId(),
        parentNodeId: this.options.parentNodeId,
        retention: DEFAULT_RETENTION,
        scene: DEFAULT_SCENE,
        source: DEFAULT_SOURCE,
        tenantId: this.tenantId(),
        uploadProfileCode: this.options.uploadProfileCode ?? DEFAULT_UPLOAD_PROFILE,
        userId,
        operatorId: userId,
        spaceId: space.id,
      },
    );
    return mapUploaderResult(result.uploadItem, result.uploadSession);
  }

  async createDownload(nodeId: string): Promise<DriveDownloadGrant> {
    const normalizedNodeId = requireValue(nodeId, 'Drive node id');
    const response = await this.client().drive.nodes.downloadUrls.create(normalizedNodeId, {
      requestedTtlSeconds: DEFAULT_DOWNLOAD_TTL_SECONDS,
      tenantId: this.tenantId(),
    });
    return this.mapDownload(response);
  }

  private async renameNode(id: string, newName: string): Promise<void> {
    await this.client().drive.nodes.update(requireValue(id, 'Drive node id'), {
      nodeName: normalizeName(newName, 'Node name'),
      operatorId: this.userId(),
      tenantId: this.tenantId(),
    });
  }

  private async deleteNode(id: string): Promise<void> {
    await this.client().drive.nodes.delete(requireValue(id, 'Drive node id'), {
      operatorId: this.userId(),
      tenantId: this.tenantId(),
    });
  }

  private async resolveSpace(): Promise<DriveSpace> {
    const response = await this.client().drive.spaces.list({
      ownerSubjectId: this.userId(),
      ownerSubjectType: 'user',
      tenantId: this.tenantId(),
    });
    const space = extractSpaces(response)
      .find((item) => item.lifecycleStatus === 'active' && item.ownerSubjectId === this.userId())
      ?? extractSpaces(response).find((item) => item.lifecycleStatus === 'active');
    if (!space) {
      throw new Error('Drive space is required before listing or uploading files.');
    }
    return space;
  }

  private mapDownload(response: CreateDownloadUrlResponse): DriveDownloadGrant {
    return {
      downloadUrl: requireValue(response.downloadUrl, 'Drive download URL'),
      expiresAtEpochMs: requireValue(response.expiresAtEpochMs, 'Drive download expiry'),
      method: response.method || 'GET',
      ...(response.signedSourceUrl ? { signedSourceUrl: response.signedSourceUrl } : {}),
    };
  }

  private client(): SdkworkDriveAppClient {
    return this.options.client ?? getDriveAppSdkClientWithSession();
  }

  private tenantId(): string {
    return pickTenantId(this.options.tenantId);
  }

  private organizationId(): string | undefined {
    return pickOrganizationId(this.options.organizationId);
  }

  private userId(): string {
    return pickUserId(this.options.userId);
  }

  private appId(): string {
    return requireValue(this.options.appId ?? DEFAULT_APP_ID, 'Drive app id');
  }
}

export function createDriveService(options: DriveServiceOptions = {}): DriveService {
  return new SdkworkDriveService(options);
}

export const driveService = createDriveService();
