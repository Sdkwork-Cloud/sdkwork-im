#!/usr/bin/env node
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import {
  collectExpectationFailures,
  finishFileExpectationVerification,
  readWorkspaceSources,
} from '../../workspace-file-expectation-shared.mjs';

const scriptDir = path.dirname(fileURLToPath(import.meta.url));
const workspaceRoot = path.resolve(scriptDir, '..');
const flutterComposedRoot = path.join(
  workspaceRoot,
  'sdkwork-craw-chat-sdk-flutter',
  'composed',
);

const { mediaModuleSource, readmeSource } = readWorkspaceSources({
  workspaceRoot: flutterComposedRoot,
  files: {
    mediaModuleSource: path.join('lib', 'src', 'media_module.dart'),
    readmeSource: 'README.md',
  },
});

const expectations = [
  {
    description: 'createUpload returns MediaUploadMutationResponse',
    pattern: /Future<MediaUploadMutationResponse\?> createUpload\(CreateUploadRequest body\)/,
    source: mediaModuleSource,
  },
  {
    description: 'completeUpload returns MediaUploadMutationResponse',
    pattern:
      /Future<MediaUploadMutationResponse\?> completeUpload\(\s*String mediaAssetId,\s*CompleteUploadRequest body,\s*\)/,
    source: mediaModuleSource,
  },
  {
    description: 'uploadContent uploads a presigned MediaUploadSession',
    pattern:
      /Future<void> uploadContent\(\s*MediaUploadSession upload,\s*List<int> bytes,\s*\)/,
    source: mediaModuleSource,
  },
  {
    description: 'upload performs create-upload-complete flow',
    pattern:
      /Future<MediaUploadMutationResponse\?> upload\(\s*CreateUploadRequest body,\s*List<int> bytes,\s*\{\s*String\? checksum,\s*\}\s*\)/,
    source: mediaModuleSource,
  },
  {
    description: 'upload builds CompleteUploadRequest from the presigned session',
    pattern:
      /CompleteUploadRequest\(\s*bucket: upload\.bucket,\s*objectKey: upload\.objectKey,\s*storageProvider: upload\.storageProvider,\s*url: upload\.url,\s*checksum: checksum,\s*\)/,
    source: mediaModuleSource,
  },
  {
    description: 'README documents the high-level upload helper',
    pattern: /sdk\.media\.upload\(/,
    source: readmeSource,
  },
  {
    description: 'README explains presigned upload session behavior',
    pattern: /presigned upload session/i,
    source: readmeSource,
  },
];

const failures = collectExpectationFailures(expectations);
finishFileExpectationVerification({
  prefix: 'sdkwork-craw-chat-sdk',
  failures,
  failureHeader: 'Flutter media upload surface verification failed:',
  successMessage: '[sdkwork-craw-chat-sdk] Flutter media upload surface verification passed.',
});
