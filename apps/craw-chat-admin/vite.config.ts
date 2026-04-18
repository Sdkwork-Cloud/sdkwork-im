import { existsSync } from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

import react from '@vitejs/plugin-react';
import tailwindcss from '@tailwindcss/vite';
import { defineConfig, type Plugin, type UserConfig } from 'vite';
import { createWebSecurityHeaders } from '../../scripts/dev/web-security-headers.mjs';

const configDir = fileURLToPath(new URL('.', import.meta.url));
const adminBackendNotConfiguredMessage =
  'Admin backend proxy target is not configured. Set SDKWORK_ADMIN_PROXY_TARGET to a compatible /api/admin backend.';

type AdminSandboxMiddleware = (
  req: AsyncIterable<Uint8Array> & {
    url?: string;
    method?: string;
    headers?: unknown;
  },
  res: {
    statusCode: number;
    setHeader(name: string, value: string): void;
    end(body?: string): void;
  },
  next: () => void,
) => void | Promise<void>;

type AdminSandboxModule = {
  createAdminSandboxMiddleware(options?: {
    state?: unknown;
    onSandboxCredentialsResolved?: (credentials: {
      email: string;
      password: string;
      source: string;
    }) => void;
  }): AdminSandboxMiddleware;
  isAdminSandboxEnabled(env?: Record<string, string | undefined>): boolean;
};

type ReleaseSafetyModule = {
  createAdminReleaseSafetyPlugin(options?: {
    command?: string;
    env?: Record<string, string | undefined>;
  }): Plugin;
};

const managementSdkComposedDistRoot = path.join(
  configDir,
  '..',
  '..',
  'sdks',
  'sdkwork-craw-chat-sdk-management',
  'sdkwork-craw-chat-sdk-management-typescript',
  'composed',
  'dist',
);
const managementSdkGeneratedDistRoot = path.join(
  configDir,
  '..',
  '..',
  'sdks',
  'sdkwork-craw-chat-sdk-management',
  'sdkwork-craw-chat-sdk-management-typescript',
  'generated',
  'server-openapi',
  'dist',
);
const sdkCommonDistRoot = path.join(
  configDir,
  '..',
  '..',
  'sdk',
  'sdkwork-sdk-commons',
  'sdkwork-sdk-common-typescript',
  'dist',
);

const sdkworkUiPackageRoots = [
  path.join(configDir, '..', '..', '..', 'sdkwork-ui', 'sdkwork-ui-pc-react'),
  path.join(configDir, '..', '..', '..', '..', '..', 'sdkwork-ui', 'sdkwork-ui-pc-react'),
  path.join(configDir, 'node_modules', '@sdkwork', 'ui-pc-react'),
];

function resolveSdkworkUiDistEntry(entryPath: string) {
  const distCandidates = sdkworkUiPackageRoots.map((packageRoot) => (
    path.join(packageRoot, 'dist', entryPath)
  ));
  const resolvedCandidate = distCandidates.find((candidate) => (
    existsSync(candidate)
  ));

  return resolvedCandidate ?? distCandidates[0];
}

function resolveProxyTarget(envValue: string | undefined) {
  const trimmedValue = envValue?.trim();
  if (!trimmedValue) {
    return null;
  }

  return /^https?:\/\//i.test(trimmedValue)
    ? trimmedValue
    : `http://${trimmedValue}`;
}

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

function adminBackendConfigurationGuard({
  adminProxyTarget,
  adminSandboxEnabled,
}: {
  adminProxyTarget: string | null;
  adminSandboxEnabled: boolean;
}): Plugin {
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

function adminSandboxPlugin(
  createAdminSandboxMiddleware: AdminSandboxModule['createAdminSandboxMiddleware'],
): Plugin {
  let loggedCredentials = false;
  const middleware = createAdminSandboxMiddleware({
    onSandboxCredentialsResolved(credentials) {
      if (loggedCredentials) {
        return;
      }

      loggedCredentials = true;
      console.warn(
        `[sdkwork-admin-sandbox] enabled with login ${credentials.email} / ${credentials.password} (${credentials.source}). ` +
          'Override with SDKWORK_ADMIN_SANDBOX_EMAIL and SDKWORK_ADMIN_SANDBOX_PASSWORD.',
      );
    },
  });

  return {
    name: 'sdkwork-admin-sandbox-middleware',
    configureServer(server) {
      server.middlewares.use(middleware);
    },
  };
}

const workspacePackageAliases = [
  ['@sdkwork/craw-chat-admin-sdk', '../../sdks/sdkwork-craw-chat-sdk-admin/sdkwork-craw-chat-sdk-admin-typescript/composed/src/index.ts'],
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
  ['sdkwork-craw-chat-admin-storage', 'packages/sdkwork-craw-chat-admin-storage/src/index.tsx'],
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

export default defineConfig(async ({ command }) => {
  const adminSandboxModule = await import(
    new URL('./dev/admin-sandbox.mjs', import.meta.url).href
  ) as unknown as AdminSandboxModule;
  const releaseSafetyModule = await import(
    new URL('./dev/release-safety.mjs', import.meta.url).href
  ) as unknown as ReleaseSafetyModule;
  const adminProxyTarget = resolveProxyTarget(
    process.env.SDKWORK_ADMIN_PROXY_TARGET ?? process.env.SDKWORK_ADMIN_BIND,
  );
  const adminSandboxEnvValue =
    process.env.SDKWORK_ADMIN_SANDBOX ?? process.env.SDKWORK_ADMIN_SANDBOX_MODE;
  const adminSandboxEnabled = !adminProxyTarget
    && adminSandboxModule.isAdminSandboxEnabled({
      ...process.env,
      SDKWORK_ADMIN_SANDBOX: adminSandboxEnvValue,
    });
  const serverConfig: UserConfig['server'] = adminProxyTarget
    ? {
        host: '0.0.0.0',
        port: 5173,
        strictPort: true,
        headers: createWebSecurityHeaders({ allowInlineScripts: true }),
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
        headers: createWebSecurityHeaders({ allowInlineScripts: true }),
      };

  return {
    base: '/admin/',
    plugins: [
      react(),
      tailwindcss(),
      releaseSafetyModule.createAdminReleaseSafetyPlugin({ command }),
      ...(adminSandboxEnabled
        ? [adminSandboxPlugin(adminSandboxModule.createAdminSandboxMiddleware)]
        : []),
      adminBackendConfigurationGuard({
        adminProxyTarget,
        adminSandboxEnabled,
      }),
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
          replacement: resolveSdkworkUiDistEntry('sdkwork-ui.css'),
        },
        {
          find: /^@sdkwork\/ui-pc-react\/theme$/,
          replacement: resolveSdkworkUiDistEntry('theme.js'),
        },
        {
          find: /^@sdkwork\/ui-pc-react\/components\/ui$/,
          replacement: resolveSdkworkUiDistEntry('components-ui.js'),
        },
        {
          find: /^@sdkwork\/ui-pc-react\/components\/ui\/feedback$/,
          replacement: resolveSdkworkUiDistEntry('ui-feedback.js'),
        },
        {
          find: /^@sdkwork\/ui-pc-react\/components\/patterns\/app-shell$/,
          replacement: resolveSdkworkUiDistEntry('patterns-app-shell.js'),
        },
        {
          find: /^@sdkwork\/ui-pc-react\/components\/patterns\/desktop-shell$/,
          replacement: resolveSdkworkUiDistEntry('patterns-desktop-shell.js'),
        },
        {
          find: /^@sdkwork\/ui-pc-react$/,
          replacement: resolveSdkworkUiDistEntry('index.js'),
        },
        {
          find: /^@sdkwork\/craw-chat-sdk-management$/,
          replacement: path.join(managementSdkComposedDistRoot, 'index.js'),
        },
        {
          find: /^@sdkwork\/craw-chat-management-backend-sdk$/,
          replacement: path.join(managementSdkGeneratedDistRoot, 'index.js'),
        },
        {
          find: /^@sdkwork\/sdk-common$/,
          replacement: path.join(sdkCommonDistRoot, 'index.js'),
        },
        ...workspacePackageAliases,
      ],
    },
    server: serverConfig,
    preview: {
      host: '0.0.0.0',
      port: 4173,
      strictPort: true,
      headers: createWebSecurityHeaders(),
    },
  };
});
