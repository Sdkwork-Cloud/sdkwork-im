#!/usr/bin/env node
import { readFileSync } from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const scriptDir = path.dirname(fileURLToPath(import.meta.url));
const workspaceRoot = path.resolve(scriptDir, '..');
const flutterComposedRoot = path.join(
  workspaceRoot,
  'sdkwork-craw-chat-sdk-flutter',
  'composed',
  'lib',
  'src',
);

const typesSource = readFileSync(path.join(flutterComposedRoot, 'types.dart'), 'utf8');
const buildersSource = readFileSync(path.join(flutterComposedRoot, 'builders.dart'), 'utf8');
const mediaModuleSource = readFileSync(
  path.join(flutterComposedRoot, 'media_module.dart'),
  'utf8',
);

const expectations = [
  {
    description: 'text message options expose renderHints',
    pattern:
      /class CrawChatTextMessageOptions[\s\S]*?final Map<String, String>\? renderHints;[\s\S]*?this\.renderHints,/,
    source: typesSource,
  },
  {
    description: 'text edit options expose renderHints',
    pattern:
      /class CrawChatTextEditOptions[\s\S]*?final Map<String, String>\? renderHints;[\s\S]*?this\.renderHints,/,
    source: typesSource,
  },
  {
    description: 'attach text media options expose renderHints',
    pattern:
      /class CrawChatAttachTextMediaOptions[\s\S]*?final Map<String, String>\? renderHints;[\s\S]*?this\.renderHints,/,
    source: typesSource,
  },
  {
    description: 'append text frame options expose attributes',
    pattern:
      /class CrawChatAppendTextFrameOptions[\s\S]*?final Map<String, String>\? attributes;[\s\S]*?this\.attributes,/,
    source: typesSource,
  },
  {
    description: 'text message builder forwards renderHints',
    pattern: /return PostMessageRequest\([\s\S]*?renderHints: options\.renderHints,/,
    source: buildersSource,
  },
  {
    description: 'text edit builder forwards renderHints',
    pattern: /return EditMessageRequest\([\s\S]*?renderHints: options\.renderHints,/,
    source: buildersSource,
  },
  {
    description: 'text frame builder forwards attributes',
    pattern: /return AppendStreamFrameRequest\([\s\S]*?attributes: options\.attributes,/,
    source: buildersSource,
  },
  {
    description: 'attach text media forwards renderHints',
    pattern: /AttachMediaRequest\([\s\S]*?renderHints: options\.renderHints,/,
    source: mediaModuleSource,
  },
];

const failures = expectations
  .filter((expectation) => !expectation.pattern.test(expectation.source))
  .map((expectation) => expectation.description);

if (failures.length > 0) {
  console.error('[sdkwork-craw-chat-sdk] Flutter composed parity verification failed:');
  for (const failure of failures) {
    console.error(`- Missing: ${failure}`);
  }
  process.exit(1);
}

console.log('[sdkwork-craw-chat-sdk] Flutter composed parity verification passed.');
