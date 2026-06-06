#!/usr/bin/env node

import process from 'node:process';
import { fileURLToPath, pathToFileURL } from 'node:url';

export { resolveCrawChatSharedDatabaseConfig } from './craw-chat-shared-database.mjs';

if (process.argv[1] && import.meta.url === pathToFileURL(process.argv[1]).href) {
  process.stderr.write(
    [
      'start-craw-chat-local-app-api.mjs no longer starts a separate app-api process.',
      'Craw Chat app-api is provided by the Rust unified server.',
      'Use `pnpm server:dev` or `node ./scripts/dev/start-craw-chat-unified-web.mjs`.',
      '',
    ].join('\n'),
  );
  process.exitCode = 1;
}
