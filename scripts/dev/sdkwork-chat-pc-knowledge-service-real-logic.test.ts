import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import {
  createKnowledgeService,
  type KnowledgeDoc,
} from '../../apps/sdkwork-chat-pc/packages/sdkwork-clawchat-pc-knowledge/src/services/KnowledgeService';

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const repoRoot = path.resolve(__dirname, '..', '..');

function read(relativePath: string): string {
  return fs.readFileSync(path.join(repoRoot, relativePath), 'utf8');
}

interface SdkCall {
  body?: Record<string, unknown>;
  id?: string;
  operation: string;
  params?: Record<string, unknown>;
}

const sdkCalls: SdkCall[] = [];

function paged<T>(items: T[]): { data: { items: T[]; pageInfo: { page: number; pageSize: number; totalItems: string; totalPages: number } } } {
  return {
    data: {
      items,
      pageInfo: {
        page: 1,
        pageSize: 100,
        totalItems: String(items.length),
        totalPages: items.length > 0 ? 1 : 0,
      },
    },
  };
}

const baseRecord = {
  baseKind: 'wiki',
  capabilityIds: ['knowledge.search', 'knowledge.read'],
  code: 'knowledge-base-engineering-handbook',
  configurationProfileId: 'profile.knowledge.pc.default',
  createdAt: '2026-06-01T09:00:00Z',
  description: 'Engineering team operating handbook.',
  displayName: 'Engineering Handbook',
  id: '910000000000000001',
  knowledgeBaseId: 'knowledge.base.engineering-handbook',
  organizationId: '10',
  ownerUserId: '100',
  providerId: 'provider.knowledge.pc.local',
  retrievalModes: ['wiki', 'keyword'],
  status: 'active',
  tenantId: '1',
  updatedAt: '2026-06-01T09:30:00Z',
  version: '3',
  visibility: 'organization',
};

const documentRecord = {
  categories: ['engineering'],
  chunkCount: 4,
  contentHash: 'sha256-engineering-onboarding',
  contentRef: 'knowledge://pc/documents/engineering-onboarding',
  createdAt: '2026-06-01T09:10:00Z',
  documentKind: 'wiki-page',
  id: '910000000000000101',
  knowledgeBaseId: 'knowledge.base.engineering-handbook',
  knowledgeDocumentId: 'knowledge.document.engineering-onboarding',
  knowledgeSourceId: null,
  metadata: {
    pcContent: '# Onboarding',
    pcParentId: 'knowledge.document.folder-root',
    pcType: 'markdown',
  },
  redactionClassification: 'internal',
  status: 'active',
  summary: 'Onboarding guide',
  tags: ['onboarding'],
  tenantId: '1',
  title: 'Onboarding',
  trustLevel: 4,
  updatedAt: '2026-06-01T09:40:00Z',
  version: '2',
  visibility: 'organization',
};

function cloneRecord<T>(value: T): T {
  return JSON.parse(JSON.stringify(value)) as T;
}

const fakeAgentClient = {
  ai: {
    knowledgeBases: {
      async list(params: Record<string, unknown>) {
        sdkCalls.push({ operation: 'ai.knowledgeBases.list', params });
        return paged([cloneRecord(baseRecord)]);
      },
      async create(body: Record<string, unknown>, params: Record<string, unknown>) {
        sdkCalls.push({ body, operation: 'ai.knowledgeBases.create', params });
        return {
          data: {
            ...cloneRecord(baseRecord),
            code: body.code,
            description: body.description,
            displayName: body.displayName,
            knowledgeBaseId: body.knowledgeBaseId,
            visibility: body.visibility,
          },
        };
      },
      async update(id: string, body: Record<string, unknown>, params: Record<string, unknown>) {
        sdkCalls.push({ body, id, operation: 'ai.knowledgeBases.update', params });
        return {
          data: {
            ...cloneRecord(baseRecord),
            description: body.description,
            displayName: body.displayName,
            knowledgeBaseId: id,
            version: '4',
          },
        };
      },
      async delete(id: string, params: Record<string, unknown>) {
        sdkCalls.push({ id, operation: 'ai.knowledgeBases.delete', params });
        return {
          data: {
            ...cloneRecord(baseRecord),
            knowledgeBaseId: id,
            status: 'deleted',
            version: '5',
          },
        };
      },
    },
    knowledgeList: {
      async list(baseId: string, params: Record<string, unknown>) {
        sdkCalls.push({ id: baseId, operation: 'ai.knowledgeList.list', params });
        return paged([cloneRecord(documentRecord)]);
      },
    },
    knowledgeDocuments: {
      async create(baseId: string, body: Record<string, unknown>, params: Record<string, unknown>) {
        sdkCalls.push({ body, id: baseId, operation: 'ai.knowledgeDocuments.create', params });
        return {
          data: {
            ...cloneRecord(documentRecord),
            contentHash: body.contentHash,
            contentRef: body.contentRef,
            documentKind: body.documentKind,
            knowledgeBaseId: baseId,
            knowledgeDocumentId: body.knowledgeDocumentId,
            metadata: body.metadata,
            summary: body.summary,
            tags: body.tags,
            title: body.title,
          },
        };
      },
      async update(id: string, body: Record<string, unknown>, params: Record<string, unknown>) {
        sdkCalls.push({ body, id, operation: 'ai.knowledgeDocuments.update', params });
        return {
          data: {
            ...cloneRecord(documentRecord),
            contentHash: body.contentHash,
            contentRef: body.contentRef,
            knowledgeDocumentId: id,
            metadata: body.metadata,
            summary: body.summary,
            tags: body.tags,
            title: body.title,
            version: '3',
          },
        };
      },
      async delete(id: string, params: Record<string, unknown>) {
        sdkCalls.push({ id, operation: 'ai.knowledgeDocuments.delete', params });
        return {
          data: {
            ...cloneRecord(documentRecord),
            knowledgeDocumentId: id,
            status: 'deleted',
            version: '4',
          },
        };
      },
    },
  },
};

async function main(): Promise<void> {
  const knowledgeServiceSource = read(
    'apps/sdkwork-chat-pc/packages/sdkwork-clawchat-pc-knowledge/src/services/KnowledgeService.ts',
  );

  assert.match(
    knowledgeServiceSource,
    /getAgentAppSdkClientWithSession/u,
    'knowledge service must use the shared sdkwork-agent-app-sdk client wrapper',
  );
  assert.match(
    knowledgeServiceSource,
    /\.ai\.knowledgeBases\.list\s*\(/u,
    'knowledge base list must use sdkwork-agent-app-sdk ai.knowledgeBases.list',
  );
  assert.match(
    knowledgeServiceSource,
    /\.ai\.knowledgeBases\.create\s*\(/u,
    'knowledge base create must use sdkwork-agent-app-sdk ai.knowledgeBases.create',
  );
  assert.match(
    knowledgeServiceSource,
    /\.ai\.knowledgeList\.list\s*\(/u,
    'knowledge documents list must use sdkwork-agent-app-sdk ai.knowledgeList.list',
  );
  assert.match(
    knowledgeServiceSource,
    /\.ai\.knowledgeDocuments\.create\s*\(/u,
    'knowledge document create must use sdkwork-agent-app-sdk ai.knowledgeDocuments.create',
  );
  assert.doesNotMatch(
    knowledgeServiceSource,
    /setTimeout|mock|Date\.now\s*\(\s*\)|Math\.random\s*\(|performance\.now\s*\(|URL\.createObjectURL|w3\.org|unsplash|w3schools|storage\.googleapis/u,
    'knowledge service must not use local mock data, local object URLs, demo media, or time/random generated resource state',
  );
  assert.doesNotMatch(knowledgeServiceSource, /\bfetch\s*\(/u, 'knowledge service must not use raw fetch');
  assert.doesNotMatch(
    knowledgeServiceSource,
    /\/api\/agent|\/(?:im|app|backend)\/v3/u,
    'knowledge service must not hand-code SDK-owned API paths',
  );
  assert.doesNotMatch(
    knowledgeServiceSource,
    /\b(Authorization|Access-Token|X-API-Key)\b/u,
    'knowledge service must not assemble auth headers manually',
  );

  const service = createKnowledgeService({
    client: fakeAgentClient,
    organizationId: '10',
    ownerUserId: '100',
    tenantId: '1',
  });

  const bases = await service.getBases();
  assert.deepEqual(
    bases,
    [
      {
        count: 0,
        description: 'Engineering team operating handbook.',
        id: 'knowledge.base.engineering-handbook',
        logo: '',
        type: 'team',
        updatedAt: Date.parse('2026-06-01T09:30:00Z'),
        version: '3',
        name: 'Engineering Handbook',
      },
    ],
    'knowledge bases must map SDK records into PC view models without mock entries',
  );

  const docs = await service.getDocs('knowledge.base.engineering-handbook');
  assert.equal(docs[0]?.id, 'knowledge.document.engineering-onboarding');
  assert.equal(docs[0]?.baseId, 'knowledge.base.engineering-handbook');
  assert.equal(docs[0]?.content, '# Onboarding');
  assert.equal(docs[0]?.parentId, 'knowledge.document.folder-root');
  assert.equal(docs[0]?.version, '2');

  const createdBase = await service.createBase({
    description: 'Platform runtime docs.',
    id: 'knowledge.base.platform-runtime',
    logo: 'https://cdn.example.test/kb.png',
    name: 'Platform Runtime',
    type: 'personal',
  });
  assert.equal(createdBase.id, 'knowledge.base.platform-runtime');

  const updatedBase = await service.updateBase('knowledge.base.platform-runtime', {
    description: 'Updated platform runtime docs.',
    name: 'Platform Runtime Updated',
    version: '4',
  });
  assert.equal(updatedBase.version, '4');

  const createdDoc = await service.createDoc({
    baseId: 'knowledge.base.engineering-handbook',
    content: '# Runtime',
    id: 'knowledge.document.runtime',
    parentId: 'knowledge.document.folder-root',
    tags: ['runtime'],
    title: 'Runtime',
    type: 'markdown',
  });
  assert.equal(createdDoc.id, 'knowledge.document.runtime');
  assert.equal(createdDoc.content, '# Runtime');

  const updatedDoc = await service.updateDoc('knowledge.document.runtime', {
    baseId: 'knowledge.base.engineering-handbook',
    content: '# Runtime v2',
    tags: ['runtime', 'v2'],
    title: 'Runtime v2',
    type: 'markdown',
    version: '3',
  });
  assert.equal(updatedDoc.version, '3');

  await service.deleteDoc('knowledge.document.runtime', '3');
  await service.deleteBase('knowledge.base.platform-runtime', '4');

  await assert.rejects(
    () => service.createBase({ name: 'Missing ID' }),
    /Knowledge base id is required/,
    'knowledge base creation must fail closed when no backend-standard knowledge.base. id is provided',
  );
  await assert.rejects(
    () => service.createBase({ id: 'kb-local', name: 'Invalid ID' }),
    /Knowledge base id must start with knowledge\.base\./,
    'knowledge base creation must reject legacy local kb-* ids',
  );
  await assert.rejects(
    () => service.createDoc({
      baseId: 'knowledge.base.engineering-handbook',
      fileName: 'report.pdf',
      fileUrl: 'blob:http://localhost/report',
      title: 'report.pdf',
      type: 'file',
    }),
    /Drive-backed file documents require a stable knowledge document id and Drive reference/,
    'file document creation must fail closed instead of persisting browser object URLs',
  );

  assert.deepEqual(
    sdkCalls.map((call) => call.operation),
    [
      'ai.knowledgeBases.list',
      'ai.knowledgeList.list',
      'ai.knowledgeBases.create',
      'ai.knowledgeBases.update',
      'ai.knowledgeDocuments.create',
      'ai.knowledgeDocuments.update',
      'ai.knowledgeDocuments.delete',
      'ai.knowledgeBases.delete',
    ],
    'knowledge service read/write operations must use sdkwork-agent-app-sdk knowledge resources',
  );
  assert.deepEqual(
    sdkCalls[2]?.body,
    {
      baseKind: 'wiki',
      capabilityIds: ['knowledge.search', 'knowledge.read'],
      code: 'knowledge-base-platform-runtime',
      configurationProfileId: 'profile.knowledge.pc.default',
      description: 'Platform runtime docs.',
      displayName: 'Platform Runtime',
      knowledgeBaseId: 'knowledge.base.platform-runtime',
      organizationId: '10',
      ownerUserId: '100',
      providerId: 'provider.knowledge.pc.local',
      requestedAt: sdkCalls[2]?.body?.requestedAt,
      retrievalModes: ['wiki', 'keyword'],
      visibility: 'private',
    },
    'knowledge base create must map the PC model into the generated agent app SDK DTO',
  );
  assert.deepEqual(
    sdkCalls[4]?.body,
    {
      categories: [],
      contentHash: sdkCalls[4]?.body?.contentHash,
      contentRef: 'knowledge://pc/documents/knowledge.document.runtime',
      documentKind: 'wiki-page',
      knowledgeDocumentId: 'knowledge.document.runtime',
      knowledgeSourceId: null,
      metadata: {
        pcContent: '# Runtime',
        pcParentId: 'knowledge.document.folder-root',
        pcType: 'markdown',
      },
      organizationId: '10',
      redactionClassification: 'internal',
      requestedAt: sdkCalls[4]?.body?.requestedAt,
      summary: '# Runtime',
      tags: ['runtime'],
      title: 'Runtime',
      trustLevel: 3,
    },
    'knowledge document create must persist content through standardized agent app SDK document metadata',
  );

  const nonFileDoc: KnowledgeDoc = {
    baseId: 'knowledge.base.engineering-handbook',
    content: '# OK',
    id: 'knowledge.document.ok',
    title: 'OK',
    updatedAt: 0,
    author: '',
    tags: [],
    type: 'markdown',
  };
  assert.equal(nonFileDoc.type, 'markdown');

  console.log('sdkwork-chat-pc knowledge service real logic contract passed');
}

void main();
