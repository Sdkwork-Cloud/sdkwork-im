import path from 'node:path';
import { fileURLToPath } from 'node:url';

import react from '@vitejs/plugin-react';
import tailwindcss from '@tailwindcss/vite';
import { defineConfig, type Plugin, type UserConfig } from 'vite';
import {
  createAdminSandboxMiddleware,
  isAdminSandboxEnabled,
} from './dev/admin-sandbox.mjs';

const configDir = fileURLToPath(new URL('.', import.meta.url));
const adminBackendNotConfiguredMessage =
  'Admin backend proxy target is not configured. Set SDKWORK_ADMIN_PROXY_TARGET to a compatible /api/admin backend.';
const sdkworkUiSourceRoot = path.join(configDir, 'packages', 'sdkwork-ui-pc-react', 'src');

function resolveProxyTarget(envValue: string | undefined) {
  const trimmedValue = envValue?.trim();
  if (!trimmedValue) {
    return null;
  }

  return /^https?:\/\//i.test(trimmedValue)
    ? trimmedValue
    : `http://${trimmedValue}`;
}

const adminProxyTarget = resolveProxyTarget(
  process.env.SDKWORK_ADMIN_PROXY_TARGET ?? process.env.SDKWORK_ADMIN_BIND,
);
const adminSandboxEnvValue =
  process.env.SDKWORK_ADMIN_SANDBOX ?? process.env.SDKWORK_ADMIN_SANDBOX_MODE;
const adminSandboxEnabled = !adminProxyTarget
  && isAdminSandboxEnabled({
    ...process.env,
    SDKWORK_ADMIN_SANDBOX: adminSandboxEnvValue,
  });

function sendMissingAdminBackendResponse(res: {
  statusCode: number;
  setHeader(name: string, value: string): void;
  end(body: string): void;
}) {
  res.statusCode = 503;
  res.setHeader('content-type', 'application/json; charset=utf-8');
  res.end(
    JSON.stringify({
      error: {
        message: adminBackendNotConfiguredMessage,
      },
      status: 503,
    }),
  );
}

function adminBackendConfigurationGuard(): Plugin {
  return {
    name: 'sdkwork-admin-backend-configuration-guard',
    configureServer(server) {
      server.middlewares.use((req, res, next) => {
        if (!req.url?.startsWith('/api/admin') || adminProxyTarget || adminSandboxEnabled) {
          next();
          return;
        }

        sendMissingAdminBackendResponse(res);
      });
    },
  };
}

function adminSandboxPlugin(): Plugin {
  const middleware = createAdminSandboxMiddleware();

  return {
    name: 'sdkwork-admin-sandbox-middleware',
    configureServer(server) {
      server.middlewares.use(middleware);
    },
  };
}

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

const serverConfig: UserConfig['server'] = adminProxyTarget
  ? {
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
    }
  : {
      host: '0.0.0.0',
      port: 5173,
      strictPort: true,
    };

export default defineConfig({
  base: '/admin/',
  plugins: [
    react(),
    tailwindcss(),
    ...(adminSandboxEnabled ? [adminSandboxPlugin()] : []),
    adminBackendConfigurationGuard(),
  ],
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
        replacement: path.join(sdkworkUiSourceRoot, 'styles', 'sdkwork-ui.css'),
      },
      {
        find: /^@sdkwork\/ui-pc-react\/theme$/,
        replacement: path.join(sdkworkUiSourceRoot, 'theme', 'index.ts'),
      },
      {
        find: /^@sdkwork\/ui-pc-react\/components\/ui$/,
        replacement: path.join(sdkworkUiSourceRoot, 'components', 'ui', 'index.ts'),
      },
      {
        find: /^@sdkwork\/ui-pc-react\/components\/ui\/feedback$/,
        replacement: path.join(sdkworkUiSourceRoot, 'components', 'ui', 'feedback', 'index.ts'),
      },
      {
        find: /^@sdkwork\/ui-pc-react\/components\/patterns\/app-shell$/,
        replacement: path.join(
          sdkworkUiSourceRoot,
          'components',
          'patterns',
          'app-shell',
          'index.ts',
        ),
      },
      {
        find: /^@sdkwork\/ui-pc-react\/components\/patterns\/desktop-shell$/,
        replacement: path.join(
          sdkworkUiSourceRoot,
          'components',
          'patterns',
          'desktop-shell',
          'index.ts',
        ),
      },
      {
        find: /^@sdkwork\/ui-pc-react$/,
        replacement: path.join(sdkworkUiSourceRoot, 'index.ts'),
      },
      ...workspacePackageAliases,
    ],
  },
  server: serverConfig,
  preview: {
    host: '0.0.0.0',
    port: 4173,
    strictPort: true,
  },
});
