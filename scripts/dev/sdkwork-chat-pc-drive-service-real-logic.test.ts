import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

import {
  createDriveService,
  type DriveFileItem,
} from '../../apps/sdkwork-chat-pc/packages/sdkwork-clawchat-pc-drive/src/services/DriveService';

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

function cloneRecord<T>(value: T): T {
  return JSON.parse(JSON.stringify(value)) as T;
}

const spaceRecord = {
  displayName: 'My Drive',
  id: 'drive.space.personal.1',
  lifecycleStatus: 'active',
  ownerSubjectId: '100',
  ownerSubjectType: 'user',
  spaceType: 'personal',
  tenantId: '1',
  version: '2',
};

const folderRecord = {
  id: 'drive.node.folder.docs',
  lifecycleStatus: 'active',
  nodeName: 'Docs',
  nodeType: 'folder',
  parentNodeId: undefined,
  spaceId: 'drive.space.personal.1',
  tenantId: '1',
  version: '3',
};

const fileRecord = {
  id: 'drive.node.file.report',
  lifecycleStatus: 'active',
  nodeName: 'report.pdf',
  nodeType: 'file',
  parentNodeId: undefined,
  spaceId: 'drive.space.personal.1',
  tenantId: '1',
  version: '4',
};

const fakeClient = {
  drive: {
    spaces: {
      async list(params: Record<string, unknown>) {
        sdkCalls.push({ operation: 'drive.spaces.list', params });
        return { items: [cloneRecord(spaceRecord)] };
      },
    },
    nodes: {
      folders: {
        async create(body: Record<string, unknown>) {
          sdkCalls.push({ body, operation: 'drive.nodes.folders.create' });
          return {
            ...cloneRecord(folderRecord),
            id: body.id,
            nodeName: body.nodeName,
            parentNodeId: body.parentNodeId,
            spaceId: body.spaceId,
            tenantId: body.tenantId,
          };
        },
      },
      async list(spaceId: string, params: Record<string, unknown>) {
        sdkCalls.push({ id: spaceId, operation: 'drive.nodes.list', params });
        return { items: [cloneRecord(folderRecord), cloneRecord(fileRecord)] };
      },
      async update(nodeId: string, body: Record<string, unknown>) {
        sdkCalls.push({ body, id: nodeId, operation: 'drive.nodes.update' });
        return {
          ...cloneRecord(nodeId.includes('folder') ? folderRecord : fileRecord),
          id: nodeId,
          nodeName: body.nodeName ?? (nodeId.includes('folder') ? folderRecord.nodeName : fileRecord.nodeName),
        };
      },
      async delete(nodeId: string, params: Record<string, unknown>) {
        sdkCalls.push({ id: nodeId, operation: 'drive.nodes.delete', params });
        return { deleted: true, id: nodeId };
      },
      downloadUrls: {
        async create(nodeId: string, params: Record<string, unknown>) {
          sdkCalls.push({ id: nodeId, operation: 'drive.nodes.downloadUrls.create', params });
          return {
            downloadUrl: `https://download.example.test/${nodeId}`,
            expiresAtEpochMs: '1790000000000',
            method: 'GET',
            signedSourceUrl: `https://provider.example.test/${nodeId}`,
          };
        },
      },
    },
    recent: {
      async list(params: Record<string, unknown>) {
        sdkCalls.push({ operation: 'drive.recent.list', params });
        return { items: [cloneRecord(fileRecord)] };
      },
    },
    downloadUrls: {
      async create(body: Record<string, unknown>) {
        sdkCalls.push({ body, operation: 'drive.downloadUrls.create' });
        return {
          downloadUrl: `https://download.example.test/${body.nodeId}`,
          expiresAtEpochMs: '1790000000000',
          method: 'GET',
          signedSourceUrl: `https://provider.example.test/${body.nodeId}`,
        };
      },
    },
  },
  uploader: {
    async uploadByProfile(profile: string, body: Record<string, unknown>) {
      sdkCalls.push({ body: { ...body, file: undefined }, operation: 'uploader.uploadByProfile', params: { profile } });
      return {
        parts: [],
        uploadItem: {
          appId: body.appId,
          appResourceId: body.appResourceId,
          appResourceType: body.appResourceType,
          actorId: body.userId,
          actorType: 'user',
          checksumSha256Hex: 'sha256-uploaded',
          cleanupStatus: 'active',
          contentLength: String((body.file as { size: number }).size),
          contentType: (body.file as { type?: string }).type || 'application/octet-stream',
          contentTypeGroup: 'document',
          fileFingerprint: 'fingerprint',
          id: 'drive.upload.item.1',
          nodeId: 'drive.node.file.uploaded',
          originalFileName: (body.file as { name?: string }).name || 'uploaded.bin',
          postProcessStatus: 'completed',
          retentionMode: 'long_term',
          spaceId: 'drive.space.personal.1',
          status: 'completed',
          taskId: 'drive.upload.task.1',
          tenantId: body.tenantId,
          uploadProfileCode: profile,
          uploadedBytes: String((body.file as { size: number }).size),
          uploadedPartsCount: '1',
          uploadSessionId: 'drive.upload.session.1',
          totalParts: '1',
          chunkSizeBytes: '5242880',
        },
        uploadSession: {
          bucket: '',
          expiresAtEpochMs: '1790000000000',
          id: 'drive.upload.session.1',
          nodeId: 'drive.node.file.uploaded',
          objectKey: '',
          spaceId: 'drive.space.personal.1',
          state: 'completed',
          storageProviderId: 'drive.provider.local',
          storageUploadId: 'drive.provider.upload.1',
          tenantId: body.tenantId,
          version: '1',
        },
      };
    },
  },
};

const testFile = {
  name: 'uploaded.pdf',
  size: 4096,
  type: 'application/pdf',
  async arrayBuffer() {
    return new ArrayBuffer(0);
  },
  slice() {
    return this;
  },
};

async function main(): Promise<void> {
  const serviceSource = read('apps/sdkwork-chat-pc/packages/sdkwork-clawchat-pc-drive/src/services/DriveService.ts');
  const viewSource = read('apps/sdkwork-chat-pc/packages/sdkwork-clawchat-pc-drive/src/index.tsx');
  const detailPanelSource = read(
    'apps/sdkwork-chat-pc/packages/sdkwork-clawchat-pc-drive/src/components/DriveDetailPanel.tsx',
  );

  assert.match(
    serviceSource,
    /getDriveAppSdkClientWithSession/u,
    'drive service must use the shared generated Drive app SDK client wrapper',
  );
  assert.match(serviceSource, /\.drive\.spaces\.list\s*\(/u, 'drive service must list spaces through Drive app SDK');
  assert.match(serviceSource, /\.drive\.nodes\.list\s*\(/u, 'drive service must list nodes through Drive app SDK');
  assert.match(
    serviceSource,
    /\.drive\.nodes\.folders\.create\s*\(/u,
    'drive service must create folders through Drive app SDK',
  );
  assert.match(serviceSource, /\.drive\.nodes\.update\s*\(/u, 'drive service must rename nodes through Drive app SDK');
  assert.match(serviceSource, /\.drive\.nodes\.delete\s*\(/u, 'drive service must delete nodes through Drive app SDK');
  assert.match(serviceSource, /\.drive\.recent\.list\s*\(/u, 'drive service must load recent files through Drive app SDK');
  assert.match(
    serviceSource,
    /\.uploader\.uploadByProfile\s*\(/u,
    'drive service must upload files through Drive app SDK uploader facade',
  );
  assert.doesNotMatch(
    serviceSource,
    /mock|setTimeout|Date\.now\s*\(\s*\)|Math\.random\s*\(|URL\.createObjectURL|dummy file|drive\.sdkwork\.com\/share/u,
    'drive service must not keep local mock data, fake timers, local IDs, or synthetic links',
  );
  assert.doesNotMatch(serviceSource, /\bfetch\s*\(/u, 'drive service must not use raw fetch');
  assert.doesNotMatch(
    serviceSource,
    /\/(?:im|app|backend)\/v3/u,
    'drive service must not hand-code SDK-owned API paths',
  );
  assert.doesNotMatch(
    serviceSource,
    /\b(Authorization|Access-Token|X-API-Key)\b/u,
    'drive service must not assemble auth headers manually',
  );
  assert.doesNotMatch(
    `${viewSource}\n${detailPanelSource}`,
    /URL\.createObjectURL|dummy file|drive\.sdkwork\.com\/share|new Blob\s*\(/u,
    'drive UI must not synthesize downloads or share links locally',
  );

  const service = createDriveService({
    appId: 'chat',
    appResourceId: 'chat.pc.drive',
    client: fakeClient,
    organizationId: '10',
    tenantId: '1',
    userId: '100',
  });

  const folders = await service.getFolders();
  assert.deepEqual(
    folders,
    [{ fileCount: 0, id: 'drive.node.folder.docs', name: 'Docs' }],
    'folder list must be mapped from Drive node records only',
  );

  const recentFiles = await service.getRecentFiles();
  assert.deepEqual(
    recentFiles,
    [
      {
        id: 'drive.node.file.report',
        name: 'report.pdf',
        size: 'Unknown',
        time: 'Unknown',
        type: 'pdf',
      },
    ],
    'recent files must be mapped from Drive node records without invented size or time values',
  );

  const createdFolder = await service.createFolder('Roadmap');
  assert.equal(createdFolder.name, 'Roadmap');
  assert.ok(createdFolder.id.startsWith('drive.node.'), 'created folder ids must use Drive node resource ids');

  await service.renameFolder('drive.node.folder.docs', 'Renamed Docs');
  await service.renameFile('drive.node.file.report', 'renamed-report.pdf');
  await service.deleteFolder('drive.node.folder.docs');
  await service.deleteFile('drive.node.file.report');

  const uploaded = await service.uploadFile(testFile);
  assert.deepEqual(
    uploaded,
    {
      id: 'drive.node.file.uploaded',
      name: 'uploaded.pdf',
      size: '4 KB',
      time: 'Unknown',
      type: 'pdf',
    } satisfies DriveFileItem,
    'uploadFile must return the stable Drive node id from uploader completion',
  );

  const download = await service.createDownload('drive.node.file.report');
  assert.equal(download.downloadUrl, 'https://download.example.test/drive.node.file.report');

  await assert.rejects(
    () => createDriveService({
      appId: 'chat',
      client: fakeClient,
      tenantId: '1',
      userId: '',
    }).uploadFile(testFile),
    /Drive user id is required/,
    'Drive upload must fail closed when actor attribution is missing',
  );

  assert.deepEqual(
    sdkCalls.map((call) => call.operation),
    [
      'drive.spaces.list',
      'drive.nodes.list',
      'drive.recent.list',
      'drive.spaces.list',
      'drive.nodes.folders.create',
      'drive.nodes.update',
      'drive.nodes.update',
      'drive.nodes.delete',
      'drive.nodes.delete',
      'drive.spaces.list',
      'uploader.uploadByProfile',
      'drive.nodes.downloadUrls.create',
    ],
    'drive operations must use generated Drive app SDK resources and uploader facade',
  );

  assert.deepEqual(
    sdkCalls.find((call) => call.operation === 'uploader.uploadByProfile')?.body,
    {
      anonymousId: undefined,
      appId: 'chat',
      appResourceId: 'chat.pc.drive',
      appResourceType: 'drive-file',
      file: undefined,
      organizationId: '10',
      parentNodeId: undefined,
      retention: { mode: 'long_term' },
      scene: 'drive',
      source: 'pc-drive',
      tenantId: '1',
      uploadProfileCode: 'document',
      userId: '100',
      operatorId: '100',
      spaceId: 'drive.space.personal.1',
    },
    'upload attribution must include tenant, organization, user, app, resource, scene, source, profile, and retention',
  );

  console.log('sdkwork-chat-pc drive service real logic contract passed');
}

void main();
