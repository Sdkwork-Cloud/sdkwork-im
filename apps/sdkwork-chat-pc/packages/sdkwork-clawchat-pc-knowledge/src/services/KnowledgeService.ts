import type {
  CreateKnowledgeBaseRequest,
  CreateKnowledgeDocumentRequest,
  KnowledgeBaseRecord,
  KnowledgeDocumentRecord,
  SdkworkAppClient as SdkworkAgentAppClient,
  UpdateKnowledgeBaseRequest,
  UpdateKnowledgeDocumentRequest,
} from '@sdkwork/agent-app-sdk';
import { getAgentAppSdkClientWithSession } from '@sdkwork/clawchat-pc-core/sdk/agentAppSdkClient';

export interface KnowledgeBase {
  id: string;
  name: string;
  description: string;
  logo: string;
  count: number;
  updatedAt: number;
  type: 'personal' | 'team';
  version?: string;
}

export interface KnowledgeDoc {
  id: string;
  baseId: string;
  title: string;
  content: string;
  author: string;
  updatedAt: number;
  tags: string[];
  type?: 'markdown' | 'file' | 'folder';
  parentId?: string;
  fileUrl?: string;
  fileName?: string;
  fileSize?: string;
  fileMimeType?: string;
  version?: string;
}

export interface KnowledgeService {
  getBases(): Promise<KnowledgeBase[]>;
  createBase(data: Partial<KnowledgeBase>): Promise<KnowledgeBase>;
  updateBase(id: string, data: Partial<KnowledgeBase>): Promise<KnowledgeBase>;
  deleteBase(id: string, expectedVersion?: string): Promise<boolean>;
  getDocs(baseId: string): Promise<KnowledgeDoc[]>;
  createDoc(data: Partial<KnowledgeDoc>): Promise<KnowledgeDoc>;
  updateDoc(id: string, data: Partial<KnowledgeDoc>): Promise<KnowledgeDoc>;
  deleteDoc(id: string, expectedVersion?: string): Promise<boolean>;
}

interface KnowledgeServiceOptions {
  client?: SdkworkAgentAppClient;
}

type RecordLike = Record<string, unknown>;

const DEFAULT_PROVIDER_ID = 'provider.knowledge.pc.local';
const DEFAULT_CONFIGURATION_PROFILE_ID = 'profile.knowledge.pc.default';
const DEFAULT_RETRIEVAL_MODES = ['wiki', 'keyword'] as const;
const DEFAULT_CAPABILITY_IDS = ['knowledge.search', 'knowledge.read'] as const;
const DEFAULT_DOCUMENT_TRUST_LEVEL = 3;
const KNOWLEDGE_BASE_ID_PREFIX = 'knowledge.base.';
const KNOWLEDGE_DOCUMENT_ID_PREFIX = 'knowledge.document.';

function isRecord(value: unknown): value is RecordLike {
  return typeof value === 'object' && value !== null && !Array.isArray(value);
}

function asString(value: unknown): string | undefined {
  return typeof value === 'string' && value.trim().length > 0 ? value.trim() : undefined;
}

function asNumber(value: unknown): number | undefined {
  return typeof value === 'number' && Number.isFinite(value) ? value : undefined;
}

function asStringArray(value: unknown): string[] {
  if (!Array.isArray(value)) {
    return [];
  }
  return value
    .map((item) => asString(item))
    .filter((item): item is string => Boolean(item));
}

function extractArray(value: unknown): unknown[] {
  if (Array.isArray(value)) {
    return value;
  }
  if (!isRecord(value)) {
    return [];
  }
  for (const key of ['items', 'data', 'records', 'list', 'knowledgeBases', 'knowledgeDocuments']) {
    const nested = value[key];
    if (Array.isArray(nested)) {
      return nested;
    }
    if (isRecord(nested)) {
      const items = extractArray(nested);
      if (items.length > 0) {
        return items;
      }
    }
  }
  return [];
}

function parseTimestamp(value: unknown): number {
  const numeric = asNumber(value);
  if (typeof numeric === 'number') {
    return numeric;
  }
  const stringValue = asString(value);
  if (!stringValue) {
    return 0;
  }
  const parsed = Date.parse(stringValue);
  return Number.isFinite(parsed) ? parsed : 0;
}

function normalizeSlug(value: string, fallback: string): string {
  const slug = value
    .trim()
    .toLowerCase()
    .replace(/[^a-z0-9]+/gu, '-')
    .replace(/^-+|-+$/gu, '')
    .slice(0, 56);
  return slug || fallback;
}

function compactSuffix(): string {
  if (typeof crypto !== 'undefined' && typeof crypto.randomUUID === 'function') {
    return crypto.randomUUID().replace(/-/gu, '').slice(0, 12).toLowerCase();
  }
  throw new Error('Knowledge resource id generation requires crypto.randomUUID.');
}

export function createKnowledgeBaseResourceId(name: string): string {
  return `${KNOWLEDGE_BASE_ID_PREFIX}${normalizeSlug(name, 'untitled')}.${compactSuffix()}`;
}

export function createKnowledgeDocumentResourceId(title: string): string {
  return `${KNOWLEDGE_DOCUMENT_ID_PREFIX}${normalizeSlug(title, 'untitled')}.${compactSuffix()}`;
}

function requireStandardId(value: unknown, fieldName: string, prefix: string): string {
  const id = asString(value);
  if (!id) {
    throw new Error(`${fieldName} is required.`);
  }
  if (!id.startsWith(prefix)) {
    throw new Error(`${fieldName} must start with ${prefix}.`);
  }
  if (id.length > 128 || !/^[a-z0-9._-]+$/u.test(id) || id.split('.').some((segment) => segment.length === 0)) {
    throw new Error(`${fieldName} must use lowercase standard id characters.`);
  }
  return id;
}

function requireKnowledgeBaseId(value: unknown): string {
  return requireStandardId(value, 'Knowledge base id', KNOWLEDGE_BASE_ID_PREFIX);
}

function requireKnowledgeDocumentId(value: unknown): string {
  return requireStandardId(value, 'Knowledge document id', KNOWLEDGE_DOCUMENT_ID_PREFIX);
}

function requestTimestamp(): string {
  return new Date().toISOString();
}

function codeFromKnowledgeBaseId(knowledgeBaseId: string): string {
  return knowledgeBaseId.replace(/\./gu, '-').slice(0, 128);
}

function metadataOf(value: unknown): RecordLike {
  return isRecord(value) ? value : {};
}

function mapKnowledgeBase(record: KnowledgeBaseRecord): KnowledgeBase {
  return {
    count: record.documentCount,
    description: record.description ?? '',
    id: record.knowledgeBaseId,
    logo: '',
    name: record.displayName,
    type: record.visibility === 'private' ? 'personal' : 'team',
    updatedAt: parseTimestamp(record.updatedAt),
    version: record.version,
  };
}

function mapKnowledgeDocument(record: KnowledgeDocumentRecord): KnowledgeDoc {
  const metadata = metadataOf(record.metadata);
  const pcType = asString(metadata.pcType);
  const driveUri = asString(metadata.driveUri);
  const content = asString(metadata.pcContent) ?? record.summary ?? '';
  const type: KnowledgeDoc['type'] =
    pcType === 'folder' || pcType === 'file' || pcType === 'markdown'
      ? pcType
      : record.documentKind === 'wiki-section'
        ? 'folder'
        : 'markdown';

  return {
    author: asString(metadata.pcAuthor) ?? '',
    baseId: record.knowledgeBaseId,
    content,
    fileMimeType: asString(metadata.mimeType),
    fileName: asString(metadata.fileName),
    fileSize: asString(metadata.fileSize),
    fileUrl: driveUri,
    id: record.knowledgeDocumentId,
    parentId: asString(metadata.pcParentId),
    tags: Array.isArray(record.tags) ? record.tags : [],
    title: record.title,
    type,
    updatedAt: parseTimestamp(record.updatedAt),
    version: record.version,
  };
}

function extractKnowledgeBaseRecords(response: unknown): KnowledgeBaseRecord[] {
  const payload = isRecord(response) ? response.data : response;
  return extractArray(payload)
    .filter((item): item is KnowledgeBaseRecord => isRecord(item) && Boolean(asString(item.knowledgeBaseId)));
}

function extractKnowledgeDocumentRecords(response: unknown): KnowledgeDocumentRecord[] {
  const payload = isRecord(response) ? response.data : response;
  return extractArray(payload)
    .filter((item): item is KnowledgeDocumentRecord => isRecord(item) && Boolean(asString(item.knowledgeDocumentId)));
}

function summarizeContent(content: string): string {
  return content.trim().replace(/\s+/gu, ' ').slice(0, 160);
}

function stableContentHash(content: string, title: string): string {
  let hash = 0x811c9dc5;
  const input = `${title}\n${content}`;
  for (let index = 0; index < input.length; index += 1) {
    hash ^= input.charCodeAt(index);
    hash = Math.imul(hash, 0x01000193);
  }
  return `sha256-pc-${(hash >>> 0).toString(16).padStart(8, '0')}`;
}

function buildDocumentMetadata(data: Partial<KnowledgeDoc>): Record<string, unknown> {
  const type = data.type ?? 'markdown';
  return {
    pcContent: data.content ?? '',
    ...(data.parentId ? { pcParentId: data.parentId } : {}),
    pcType: type,
    ...(data.fileName ? { fileName: data.fileName } : {}),
    ...(data.fileSize ? { fileSize: data.fileSize } : {}),
    ...(data.fileMimeType ? { mimeType: data.fileMimeType } : {}),
    ...(data.fileUrl?.startsWith('drive://') ? { driveUri: data.fileUrl } : {}),
  };
}

function assertFileDocumentHasDriveReference(data: Partial<KnowledgeDoc>): void {
  if (data.type !== 'file') {
    return;
  }
  if (!asString(data.id) || !data.fileUrl?.startsWith('drive://')) {
    throw new Error('Drive-backed file documents require a stable knowledge document id and Drive reference.');
  }
}

function buildCreateBaseRequest(data: Partial<KnowledgeBase>): CreateKnowledgeBaseRequest {
  const knowledgeBaseId = requireKnowledgeBaseId(data.id);
  const name = asString(data.name);
  if (!name) {
    throw new Error('Knowledge base name is required.');
  }
  return {
    baseKind: 'wiki',
    capabilityIds: [...DEFAULT_CAPABILITY_IDS],
    code: codeFromKnowledgeBaseId(knowledgeBaseId),
    configurationProfileId: DEFAULT_CONFIGURATION_PROFILE_ID,
    description: data.description ?? '',
    displayName: name,
    knowledgeBaseId,
    providerId: DEFAULT_PROVIDER_ID,
    requestedAt: requestTimestamp(),
    retrievalModes: [...DEFAULT_RETRIEVAL_MODES],
    visibility: data.type === 'personal' ? 'private' : 'organization',
  };
}

function buildUpdateBaseRequest(data: Partial<KnowledgeBase>): UpdateKnowledgeBaseRequest {
  return {
    ...(data.description !== undefined ? { description: data.description } : {}),
    ...(data.name !== undefined ? { displayName: data.name } : {}),
    ...(data.type ? { visibility: data.type === 'personal' ? 'private' : 'organization' } : {}),
    ...(data.version ? { expectedVersion: data.version } : {}),
    requestedAt: requestTimestamp(),
  };
}

function buildCreateDocumentRequest(data: Partial<KnowledgeDoc>): CreateKnowledgeDocumentRequest {
  assertFileDocumentHasDriveReference(data);
  const knowledgeDocumentId = requireKnowledgeDocumentId(data.id);
  const title = asString(data.title);
  if (!title) {
    throw new Error('Knowledge document title is required.');
  }
  const content = data.content ?? '';
  return {
    categories: [],
    contentHash: stableContentHash(content, title),
    contentRef: `knowledge://pc/documents/${knowledgeDocumentId}`,
    documentKind: data.type === 'folder' ? 'wiki-section' : 'wiki-page',
    knowledgeDocumentId,
    knowledgeSourceId: null,
    metadata: buildDocumentMetadata(data),
    redactionClassification: 'internal',
    requestedAt: requestTimestamp(),
    summary: summarizeContent(content),
    tags: data.tags ?? [],
    title,
    trustLevel: DEFAULT_DOCUMENT_TRUST_LEVEL,
  };
}

function buildUpdateDocumentRequest(data: Partial<KnowledgeDoc>): UpdateKnowledgeDocumentRequest {
  assertFileDocumentHasDriveReference(data);
  const title = asString(data.title);
  const content = data.content ?? '';
  return {
    categories: [],
    ...(content || title ? { contentHash: stableContentHash(content, title ?? '') } : {}),
    ...(data.id ? { contentRef: `knowledge://pc/documents/${data.id}` } : {}),
    ...(data.type ? { documentKind: data.type === 'folder' ? 'wiki-section' : 'wiki-page' } : {}),
    ...(data.version ? { expectedVersion: data.version } : {}),
    metadata: buildDocumentMetadata(data),
    redactionClassification: 'internal',
    requestedAt: requestTimestamp(),
    ...(content ? { summary: summarizeContent(content) } : {}),
    tags: data.tags ?? [],
    title,
    trustLevel: DEFAULT_DOCUMENT_TRUST_LEVEL,
  };
}

class SdkworkKnowledgeService implements KnowledgeService {
  constructor(private readonly options: KnowledgeServiceOptions = {}) {}

  async getBases(): Promise<KnowledgeBase[]> {
    const response = await this.client().ai.knowledgeBases.list({
      page: 1,
      pageSize: 100,
    });
    return extractKnowledgeBaseRecords(response).map(mapKnowledgeBase);
  }

  async createBase(data: Partial<KnowledgeBase>): Promise<KnowledgeBase> {
    const response = await this.client().ai.knowledgeBases.create(
      buildCreateBaseRequest(data),
    );
    return mapKnowledgeBase(response.data);
  }

  async updateBase(id: string, data: Partial<KnowledgeBase>): Promise<KnowledgeBase> {
    const response = await this.client().ai.knowledgeBases.update(
      requireKnowledgeBaseId(id),
      buildUpdateBaseRequest(data),
    );
    return mapKnowledgeBase(response.data);
  }

  async deleteBase(id: string, expectedVersion?: string): Promise<boolean> {
    await this.client().ai.knowledgeBases.delete(requireKnowledgeBaseId(id), {
      expectedVersion,
      requestedAt: requestTimestamp(),
    });
    return true;
  }

  async getDocs(baseId: string): Promise<KnowledgeDoc[]> {
    const response = await this.client().ai.knowledgeList.list(requireKnowledgeBaseId(baseId), {
      page: 1,
      pageSize: 100,
    });
    return extractKnowledgeDocumentRecords(response).map(mapKnowledgeDocument);
  }

  async createDoc(data: Partial<KnowledgeDoc>): Promise<KnowledgeDoc> {
    const baseId = requireKnowledgeBaseId(data.baseId);
    const response = await this.client().ai.knowledgeDocuments.create(
      baseId,
      buildCreateDocumentRequest(data),
    );
    return mapKnowledgeDocument(response.data);
  }

  async updateDoc(id: string, data: Partial<KnowledgeDoc>): Promise<KnowledgeDoc> {
    const documentId = requireKnowledgeDocumentId(id);
    const response = await this.client().ai.knowledgeDocuments.update(
      documentId,
      buildUpdateDocumentRequest({ ...data, id: documentId }),
    );
    return mapKnowledgeDocument(response.data);
  }

  async deleteDoc(id: string, expectedVersion?: string): Promise<boolean> {
    await this.client().ai.knowledgeDocuments.delete(requireKnowledgeDocumentId(id), {
      expectedVersion,
      requestedAt: requestTimestamp(),
    });
    return true;
  }

  private client(): SdkworkAgentAppClient {
    return this.options.client ?? getAgentAppSdkClientWithSession();
  }
}

export function createKnowledgeService(options: KnowledgeServiceOptions = {}): KnowledgeService {
  return new SdkworkKnowledgeService(options);
}

export const knowledgeService = createKnowledgeService();
