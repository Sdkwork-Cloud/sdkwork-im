import { existsSync } from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

import react from '@vitejs/plugin-react';
import tailwindcss from '@tailwindcss/vite';
import { defineConfig } from 'vite';

const configDir = fileURLToPath(new URL('.', import.meta.url));

function resolveSdkworkUiRoot() {
  const candidates = [
    path.join(configDir, 'node_modules', '@sdkwork', 'ui-pc-react'),
    path.join(configDir, '..', '..', '..', 'sdkwork-ui', 'sdkwork-ui-pc-react'),
    path.join(configDir, '..', '..', '..', '..', '..', 'sdkwork-ui', 'sdkwork-ui-pc-react'),
  ];

  return (
    candidates.find((candidate) => existsSync(path.join(candidate, 'dist'))) ?? candidates[0]
  );
}

const sdkworkUiRoot = resolveSdkworkUiRoot();
const sdkworkUiDistRoot = path.join(sdkworkUiRoot, 'dist');
const defaultAdminProxyTarget = 'http://127.0.0.1:9981';

function resolveUiDist(entryPath: string) {
  return path.join(sdkworkUiDistRoot, entryPath);
}

function resolveProxyTarget(envValue: string | undefined, fallbackTarget: string) {
  const trimmedValue = envValue?.trim();
  if (!trimmedValue) {
    return fallbackTarget;
  }

  return /^https?:\/\//i.test(trimmedValue)
    ? trimmedValue
    : `http://${trimmedValue}`;
}

const adminProxyTarget = resolveProxyTarget(
  process.env.SDKWORK_ADMIN_PROXY_TARGET ?? process.env.SDKWORK_ADMIN_BIND,
  defaultAdminProxyTarget,
);

const workspacePackageAliases = [
  ['sdkwork-craw-chat-admin-admin-api', 'packages/sdkwork-craw-chat-admin-admin-api/src/index.ts'],
  ['sdkwork-craw-chat-admin-auth', 'packages/sdkwork-craw-chat-admin-auth/src/index.tsx'],
  ['sdkwork-craw-chat-admin-core', 'packages/sdkwork-craw-chat-admin-core/src/index.tsx'],
  ['sdkwork-craw-chat-admin-shell', 'packages/sdkwork-craw-chat-admin-shell/src/index.ts'],
  ['sdkwork-craw-chat-admin-types', 'packages/sdkwork-craw-chat-admin-types/src/index.ts'],
  ['sdkwork-craw-chat-admin-overview', 'packages/sdkwork-craw-chat-admin-overview/src/index.tsx'],
  ['sdkwork-craw-chat-admin-tenants', 'packages/sdkwork-craw-chat-admin-tenants/src/index.tsx'],
  ['sdkwork-craw-chat-admin-users', 'packages/sdkwork-craw-chat-admin-users/src/index.tsx'],
  ['sdkwork-craw-chat-admin-conversations', 'packages/sdkwork-craw-chat-admin-conversations/src/index.tsx'],
  ['sdkwork-craw-chat-admin-messages', 'packages/sdkwork-craw-chat-admin-messages/src/index.tsx'],
  ['sdkwork-craw-chat-admin-groups', 'packages/sdkwork-craw-chat-admin-groups/src/index.tsx'],
  ['sdkwork-craw-chat-admin-moderation', 'packages/sdkwork-craw-chat-admin-moderation/src/index.tsx'],
  ['sdkwork-craw-chat-admin-automation', 'packages/sdkwork-craw-chat-admin-automation/src/index.tsx'],
  ['sdkwork-craw-chat-admin-announcements', 'packages/sdkwork-craw-chat-admin-announcements/src/index.tsx'],
  ['sdkwork-craw-chat-admin-realtime', 'packages/sdkwork-craw-chat-admin-realtime/src/index.tsx'],
  ['sdkwork-craw-chat-admin-system', 'packages/sdkwork-craw-chat-admin-system/src/index.tsx'],
  ['sdkwork-craw-chat-admin-settings', 'packages/sdkwork-craw-chat-admin-settings/src/index.tsx'],
].map(([packageName, relativePath]) => ({
  find: new RegExp(`^${packageName}$`),
  replacement: path.join(configDir, relativePath),
}));

function manualChunks(id: string) {
  if (!id.includes('node_modules')) {
    return undefined;
  }

  if (
    id.includes('\\react\\')
    || id.includes('/react/')
    || id.includes('\\react-dom\\')
    || id.includes('/react-dom/')
    || id.includes('\\react-router')
    || id.includes('/react-router')
    || id.includes('\\scheduler\\')
    || id.includes('/scheduler/')
  ) {
    return 'react-vendor';
  }

  if (id.includes('\\lucide-react\\') || id.includes('/lucide-react/')) {
    return 'icon-vendor';
  }

  if (id.includes('\\motion\\') || id.includes('/motion/')) {
    return 'motion-vendor';
  }

  return undefined;
}

export default defineConfig({
  base: '/admin/',
  plugins: [react(), tailwindcss()],
  build: {
    rollupOptions: {
      output: {
        manualChunks,
      },
    },
  },
  resolve: {
    dedupe: ['react', 'react-dom'],
    alias: [
      {
        find: /^motion\/react$/,
        replacement: path.join(configDir, 'src', 'vendor', 'motion-react.tsx'),
      },
      {
        find: /^@sdkwork\/ui-pc-react\/styles\.css$/,
        replacement: resolveUiDist('sdkwork-ui.css'),
      },
      {
        find: /^@sdkwork\/ui-pc-react\/theme$/,
        replacement: resolveUiDist('theme.js'),
      },
      {
        find: /^@sdkwork\/ui-pc-react\/components\/ui$/,
        replacement: resolveUiDist('components-ui.js'),
      },
      {
        find: /^@sdkwork\/ui-pc-react\/components\/ui\/feedback$/,
        replacement: resolveUiDist('ui-feedback.js'),
      },
      {
        find: /^@sdkwork\/ui-pc-react\/components\/patterns\/app-shell$/,
        replacement: resolveUiDist('patterns-app-shell.js'),
      },
      {
        find: /^@sdkwork\/ui-pc-react\/components\/patterns\/desktop-shell$/,
        replacement: resolveUiDist('patterns-desktop-shell.js'),
      },
      {
        find: /^@sdkwork\/ui-pc-react$/,
        replacement: resolveUiDist('index.js'),
      },
      ...workspacePackageAliases,
    ],
  },
  server: {
    host: '0.0.0.0',
    port: 5173,
    strictPort: true,
    proxy: {
      '/api/admin': {
        target: adminProxyTarget,
        changeOrigin: true,
        rewrite: (requestPath: string) => requestPath.replace(/^\/api\/admin/, '/admin'),
      },
    },
  },
  preview: {
    host: '0.0.0.0',
    port: 4173,
    strictPort: true,
  },
});
