import assert from 'node:assert/strict';
import { pathToFileURL } from 'node:url';
import type { SdkworkAppClient } from '@sdkwork/agent-app-sdk';
import type * as KnowledgeServiceModule from '../packages/sdkwork-im-pc-knowledge/src/services/KnowledgeService.ts';

type KnowledgeServiceExports = typeof KnowledgeServiceModule;

async function loadKnowledgeServiceModule(): Promise<KnowledgeServiceExports> {
  const moduleUrl = pathToFileURL(
    './packages/sdkwork-im-pc-knowledge/src/services/KnowledgeService.ts',
  ).href;
  const loaded = (await import(moduleUrl)) as Partial<KnowledgeServiceExports> & {
    default?: Partial<KnowledgeServiceExports>;
  };
  const createKnowledgeService =
    loaded.createKnowledgeService ?? loaded.default?.createKnowledgeService;
  assert.equal(typeof createKnowledgeService, 'function');
  return {
    ...loaded.default,
    ...loaded,
    createKnowledgeService,
  } as KnowledgeServiceExports;
}

const fakeClient = {
  ai: {
    knowledgeBases: {
      async list(params: { page?: number; pageSize?: number }) {
        assert.deepEqual(params, {
          page: 1,
          pageSize: 100,
        });
        return {
          data: {
            items: [
              {
                id: '9001',
                tenantId: '1',
                organizationId: '10',
                ownerUserId: '100',
                knowledgeBaseId: 'knowledge.base.pc.contract.counted',
                code: 'knowledge-base-pc-contract-counted',
                displayName: 'PC Contract Counted',
                description: 'Counted through SDK projection',
                providerId: 'provider.knowledge.pc.local',
                baseKind: 'wiki',
                retrievalModes: ['wiki', 'keyword'],
                capabilityIds: ['knowledge.search', 'knowledge.read'],
                configurationProfileId: 'profile.knowledge.pc.default',
                documentCount: 7,
                status: 'active',
                visibility: 'private',
                version: '3',
                createdAt: '2026-06-01T00:00:00Z',
                updatedAt: '2026-06-01T00:10:00Z',
                deletedAt: null,
              },
            ],
            pageInfo: {
              page: 1,
              pageSize: 100,
              totalItems: '1',
              totalPages: 1,
            },
          },
        };
      },
    },
  },
} as unknown as SdkworkAppClient;

const { createKnowledgeService } = await loadKnowledgeServiceModule();
const knowledgeService = createKnowledgeService({
  client: fakeClient,
});

const bases = await knowledgeService.getBases();

assert.equal(bases.length, 1);
assert.equal(bases[0]?.id, 'knowledge.base.pc.contract.counted');
assert.equal(bases[0]?.count, 7);
assert.equal(bases[0]?.type, 'personal');

console.log('sdkwork im pc knowledge service document count contract passed.');
