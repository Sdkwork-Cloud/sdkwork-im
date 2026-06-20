import tailwindcss from '@tailwindcss/vite';
import react from '@vitejs/plugin-react';
import path from 'path';
import {defineConfig, loadEnv, type Plugin} from 'vite';
import { handleSdkworkChatLocalApiRequest } from './local-api';

const repoRoot = path.resolve(__dirname, '../..');

function dependencyRoot(dependencyId: string): string {
  return path.resolve(repoRoot, '..', dependencyId);
}

const generatedImAppSdkEntry = path.resolve(
  __dirname,
  '../../sdks/sdkwork-im-app-sdk/sdkwork-im-app-sdk-typescript/generated/server-openapi/src/index.ts',
);
const generatedImBackendSdkEntry = path.resolve(
  __dirname,
  '../../sdks/sdkwork-im-backend-sdk/sdkwork-im-backend-sdk-typescript/generated/server-openapi/src/index.ts',
);
const generatedAgentAppSdkEntry = path.resolve(
  dependencyRoot('sdkwork-kernel'),
  'sdks/sdkwork-agent-app-sdk/sdkwork-agent-app-sdk-typescript/generated/server-openapi/src/index.ts',
);
const generatedAppbaseAppSdkEntry = path.resolve(
  repoRoot,
  '../sdkwork-appbase/sdks/sdkwork-appbase-app-sdk/sdkwork-appbase-app-sdk-typescript/generated/server-openapi/src/index.ts',
);
const generatedAppbaseBackendSdkEntry = path.resolve(
  repoRoot,
  '../sdkwork-appbase/sdks/sdkwork-appbase-backend-sdk/sdkwork-appbase-backend-sdk-typescript/generated/server-openapi/src/index.ts',
);
const generatedAiotAppSdkEntry = path.resolve(
  dependencyRoot('sdkwork-aiot'),
  'sdks/sdkwork-aiot-app-sdk/sdkwork-aiot-app-sdk-typescript/src/index.ts',
);
const generatedAiotBackendSdkEntry = path.resolve(
  dependencyRoot('sdkwork-aiot'),
  'sdks/sdkwork-aiot-backend-sdk/sdkwork-aiot-backend-sdk-typescript/src/index.ts',
);
const generatedDriveAppSdkEntry = path.resolve(
  dependencyRoot('sdkwork-drive'),
  'sdks/sdkwork-drive-app-sdk/sdkwork-drive-app-sdk-typescript/src/index.ts',
);

const generatedNotaryAppSdkEntry = path.resolve(
  dependencyRoot('sdkwork-notary'),
  'sdks/sdkwork-notary-app-sdk/sdkwork-notary-app-sdk-typescript/src/index.ts',
);
const generatedImSdkEntry = path.resolve(
  __dirname,
  '../../sdks/sdkwork-im-sdk/sdkwork-im-sdk-typescript/src/index.ts',
);
const generatedRtcSdkEntry = path.resolve(
  dependencyRoot('sdkwork-rtc'),
  'sdks/sdkwork-rtc-sdk/sdkwork-rtc-sdk-typescript/src/index.ts',
);
const appbasePcReactEntry = path.resolve(
  repoRoot,
  '../sdkwork-appbase/packages/pc-react/foundation/sdkwork-appbase-pc-react/src/index.ts',
);
const authPcReactEntry = path.resolve(
  repoRoot,
  '../sdkwork-appbase/packages/pc-react/iam/sdkwork-auth-pc-react/src/index.ts',
);
const authRuntimePcReactEntry = path.resolve(
  repoRoot,
  '../sdkwork-appbase/packages/pc-react/iam/sdkwork-auth-runtime-pc-react/src/index.ts',
);
const authPcReactAuthEntry = path.resolve(
  repoRoot,
  '../sdkwork-appbase/packages/pc-react/iam/sdkwork-auth-pc-react/src/auth.ts',
);
const iamContractsEntry = path.resolve(
  repoRoot,
  '../sdkwork-appbase/packages/common/iam/sdkwork-iam-contracts/src/index.ts',
);
const iamSdkPortsEntry = path.resolve(
  repoRoot,
  '../sdkwork-appbase/packages/common/iam/sdkwork-iam-sdk-ports/src/index.ts',
);
const i18nPcReactEntry = path.resolve(
  repoRoot,
  '../sdkwork-appbase/packages/pc-react/foundation/sdkwork-i18n-pc-react/src/index.ts',
);
const corePcReactEntry = path.resolve(
  dependencyRoot('sdkwork-core'),
  'sdkwork-core-pc-react/src',
);
const uiPcReactSourceRoot = path.resolve(
  dependencyRoot('sdkwork-ui'),
  'sdkwork-ui-pc-react/src',
);
const uiPcReactEntry = path.resolve(
  repoRoot,
  '../sdkwork-ui/sdkwork-ui-pc-react/src/index.ts',
);
const uiPcReactStylesEntry = path.resolve(
  repoRoot,
  '../sdkwork-ui/sdkwork-ui-pc-react/src/styles/sdkwork-ui.css',
);
const sdkCommonSourceRoot = path.resolve(
  repoRoot,
  '../sdkwork-sdk-commons/sdkwork-sdk-common-typescript/src',
);
const sdkCommonEntry = path.resolve(
  sdkCommonSourceRoot,
  'index.ts',
);
const adminCoreSourceRoot = path.resolve(__dirname, './packages/sdkwork-im-admin-core/src');
const reactEntry = path.resolve(__dirname, 'node_modules/react/index.js');
const reactJsxRuntimeEntry = path.resolve(__dirname, 'node_modules/react/jsx-runtime.js');
const reactJsxDevRuntimeEntry = path.resolve(__dirname, 'node_modules/react/jsx-dev-runtime.js');
const reactDomEntry = path.resolve(__dirname, 'node_modules/react-dom/index.js');
const reactDomClientEntry = path.resolve(__dirname, 'node_modules/react-dom/client.js');
const reactRouterDomEntry = path.resolve(__dirname, 'node_modules/react-router-dom/dist/index.mjs');

function sdkworkChatLocalApiPlugin(): Plugin {
  return {
    name: 'sdkwork-chat-local-api',
    configureServer(server) {
      server.middlewares.use((req, res, next) => {
        handleSdkworkChatLocalApiRequest(req, res)
          .then((handled) => {
            if (!handled) {
              next();
            }
          })
          .catch(next);
      });
    },
  };
}

export default defineConfig(({mode}) => {
  const env = loadEnv(mode, '.', '');
  return {
    define: {
      'process.env.SDKWORK_ACCESS_TOKEN': JSON.stringify(env.SDKWORK_ACCESS_TOKEN ?? ''),
    },
    plugins: [sdkworkChatLocalApiPlugin(), react(), tailwindcss()],
    resolve: {
      alias: [
        { find: '@', replacement: path.resolve(__dirname, '.') },
        { find: 'react/jsx-runtime', replacement: reactJsxRuntimeEntry },
        { find: 'react/jsx-dev-runtime', replacement: reactJsxDevRuntimeEntry },
        { find: 'react-dom/client', replacement: reactDomClientEntry },
        { find: /^react-dom$/, replacement: reactDomEntry },
        { find: /^react-router-dom$/, replacement: reactRouterDomEntry },
        { find: /^react$/, replacement: reactEntry },
        { find: '@sdkwork-internal/im-app-api-generated', replacement: generatedImAppSdkEntry },
        { find: '@sdkwork-internal/im-backend-api-generated', replacement: generatedImBackendSdkEntry },
        { find: '@sdkwork/agent-app-sdk', replacement: generatedAgentAppSdkEntry },
        { find: '@sdkwork/aiot-app-sdk', replacement: generatedAiotAppSdkEntry },
        { find: '@sdkwork/aiot-backend-sdk', replacement: generatedAiotBackendSdkEntry },
        { find: '@sdkwork/appbase-app-sdk', replacement: generatedAppbaseAppSdkEntry },
        { find: '@sdkwork/appbase-backend-sdk', replacement: generatedAppbaseBackendSdkEntry },
        { find: '@sdkwork/drive-app-sdk', replacement: generatedDriveAppSdkEntry },
        { find: '@sdkwork/notary-app-sdk', replacement: generatedNotaryAppSdkEntry },
        { find: '@sdkwork/im-sdk', replacement: generatedImSdkEntry },
        { find: '@sdkwork/rtc-sdk', replacement: generatedRtcSdkEntry },
        { find: '@sdkwork/appbase-pc-react', replacement: appbasePcReactEntry },
        { find: '@sdkwork/auth-pc-react/auth', replacement: authPcReactAuthEntry },
        { find: '@sdkwork/auth-runtime-pc-react', replacement: authRuntimePcReactEntry },
        { find: '@sdkwork/auth-pc-react', replacement: authPcReactEntry },
        { find: '@sdkwork/iam-contracts', replacement: iamContractsEntry },
        { find: '@sdkwork/iam-sdk-ports', replacement: iamSdkPortsEntry },
        { find: '@sdkwork/i18n-pc-react', replacement: i18nPcReactEntry },
        { find: '@sdkwork/core-pc-react', replacement: corePcReactEntry },
        { find: '@sdkwork/ui-pc-react/styles.css', replacement: uiPcReactStylesEntry },
        { find: /^@sdkwork\/ui-pc-react\/(.+)$/, replacement: `${uiPcReactSourceRoot}/$1` },
        { find: '@sdkwork/ui-pc-react', replacement: uiPcReactEntry },
        { find: '@sdkwork/sdk-common/core', replacement: path.resolve(sdkCommonSourceRoot, 'core/index.ts') },
        { find: '@sdkwork/sdk-common/auth', replacement: path.resolve(sdkCommonSourceRoot, 'auth/index.ts') },
        { find: '@sdkwork/sdk-common/http', replacement: path.resolve(sdkCommonSourceRoot, 'http/index.ts') },
        { find: '@sdkwork/sdk-common/errors', replacement: path.resolve(sdkCommonSourceRoot, 'errors/index.ts') },
        { find: '@sdkwork/sdk-common/utils', replacement: path.resolve(sdkCommonSourceRoot, 'utils/index.ts') },
        { find: '@sdkwork/sdk-common', replacement: sdkCommonEntry },
        { find: '@sdkwork/im-pc-types', replacement: path.resolve(__dirname, './packages/sdkwork-im-pc-types/src') },
        { find: '@sdkwork/im-pc-commons', replacement: path.resolve(__dirname, './packages/sdkwork-im-pc-commons/src') },
        { find: '@sdkwork/im-pc-shell', replacement: path.resolve(__dirname, './packages/sdkwork-im-pc-shell/src') },
        { find: '@sdkwork/im-pc-core', replacement: path.resolve(__dirname, './packages/sdkwork-im-pc-core/src') },
        { find: '@sdkwork/im-pc-chat', replacement: path.resolve(__dirname, './packages/sdkwork-im-pc-chat/src') },
        { find: '@sdkwork/im-pc-agent', replacement: path.resolve(__dirname, './packages/sdkwork-im-pc-agent/src') },
        { find: '@sdkwork/im-pc-voice', replacement: path.resolve(__dirname, './packages/sdkwork-im-pc-voice/src') },
        { find: '@sdkwork/im-pc-workspace', replacement: path.resolve(__dirname, './packages/sdkwork-im-pc-workspace/src') },
        { find: '@sdkwork/im-pc-orders', replacement: path.resolve(__dirname, './packages/sdkwork-im-pc-orders/src') },
        { find: '@sdkwork/im-pc-notary', replacement: path.resolve(__dirname, './packages/sdkwork-im-pc-notary/src') },
        { find: '@sdkwork/im-pc-mail', replacement: path.resolve(__dirname, './packages/sdkwork-im-pc-mail/src') },
        { find: '@sdkwork/im-pc-drive', replacement: path.resolve(__dirname, './packages/sdkwork-im-pc-drive/src') },
        { find: '@sdkwork/im-pc-contacts', replacement: path.resolve(__dirname, './packages/sdkwork-im-pc-contacts/src') },
        { find: '@sdkwork/im-pc-calendar', replacement: path.resolve(__dirname, './packages/sdkwork-im-pc-calendar/src') },
        { find: '@sdkwork/im-pc-shop', replacement: path.resolve(__dirname, './packages/sdkwork-im-pc-shop/src') },
        { find: '@sdkwork/im-pc-knowledge', replacement: path.resolve(__dirname, './packages/sdkwork-im-pc-knowledge/src') },
        { find: '@sdkwork/im-pc-devices', replacement: path.resolve(__dirname, './packages/sdkwork-im-pc-devices/src') },
        { find: '@sdkwork/im-pc-community', replacement: path.resolve(__dirname, './packages/sdkwork-im-pc-community/src') },
        { find: '@sdkwork/im-pc-course', replacement: path.resolve(__dirname, './packages/sdkwork-im-pc-course/src') },
        { find: '@sdkwork/im-pc-enterprise', replacement: path.resolve(__dirname, './packages/sdkwork-im-pc-enterprise/src') },
        { find: '@sdkwork/im-console-core', replacement: path.resolve(__dirname, './packages/sdkwork-im-console-core/src') },
        { find: /^@sdkwork\/im-admin-core\/(.+)$/, replacement: `${adminCoreSourceRoot}/$1` },
        { find: '@sdkwork/im-admin-core', replacement: adminCoreSourceRoot },
        { find: '@sdkwork/im-pc-approvals', replacement: path.resolve(__dirname, './packages/sdkwork-im-pc-approvals/src') },
        { find: '@sdkwork/im-pc-reports', replacement: path.resolve(__dirname, './packages/sdkwork-im-pc-reports/src') },
        { find: '@sdkwork/im-pc-attendance', replacement: path.resolve(__dirname, './packages/sdkwork-im-pc-attendance/src') },
        { find: '@sdkwork/im-console-users', replacement: path.resolve(__dirname, './packages/sdkwork-im-console-users/src') },
        { find: '@sdkwork/im-console-roles', replacement: path.resolve(__dirname, './packages/sdkwork-im-console-roles/src') },
        { find: '@sdkwork/im-console-communications', replacement: path.resolve(__dirname, './packages/sdkwork-im-console-communications/src') },
        { find: '@sdkwork/im-console-integrations', replacement: path.resolve(__dirname, './packages/sdkwork-im-console-integrations/src') },
        { find: '@sdkwork/im-console-security', replacement: path.resolve(__dirname, './packages/sdkwork-im-console-security/src') },
        { find: '@sdkwork/im-console-settings', replacement: path.resolve(__dirname, './packages/sdkwork-im-console-settings/src') },
        { find: '@sdkwork/im-console-shop', replacement: path.resolve(__dirname, './packages/sdkwork-im-console-shop/src') },
        { find: '@sdkwork/im-console-product', replacement: path.resolve(__dirname, './packages/sdkwork-im-console-product/src') },
        { find: '@sdkwork/im-admin-tenants', replacement: path.resolve(__dirname, './packages/sdkwork-im-admin-tenants/src') },
        { find: '@sdkwork/im-admin-infrastructure', replacement: path.resolve(__dirname, './packages/sdkwork-im-admin-infrastructure/src') },
        { find: '@sdkwork/im-admin-operations', replacement: path.resolve(__dirname, './packages/sdkwork-im-admin-operations/src') },
        { find: '@sdkwork/im-console-dashboard', replacement: path.resolve(__dirname, './packages/sdkwork-im-console-dashboard/src') },
        { find: '@sdkwork/im-admin-dashboard', replacement: path.resolve(__dirname, './packages/sdkwork-im-admin-dashboard/src') },
        { find: '@sdkwork/im-pc-video-gen', replacement: path.resolve(__dirname, './packages/sdkwork-im-pc-video-gen/src') },
        { find: '@sdkwork/im-pc-image-gen', replacement: path.resolve(__dirname, './packages/sdkwork-im-pc-image-gen/src') },
        { find: '@sdkwork/im-pc-voice-gen', replacement: path.resolve(__dirname, './packages/sdkwork-im-pc-voice-gen/src') },
        { find: '@sdkwork/im-pc-music-gen', replacement: path.resolve(__dirname, './packages/sdkwork-im-pc-music-gen/src') },
        { find: '@sdkwork/im-pc-writing', replacement: path.resolve(__dirname, './packages/sdkwork-im-pc-writing/src') },
      ],
      dedupe: ['react', 'react-dom'],
    },
    server: {
      hmr: process.env.DISABLE_HMR !== 'true',
    },
    optimizeDeps: {
      exclude: [
        '@sdkwork-internal/im-app-api-generated',
        '@sdkwork-internal/im-backend-api-generated',
        '@sdkwork/agent-app-sdk',
        '@sdkwork/aiot-app-sdk',
        '@sdkwork/aiot-backend-sdk',
        '@sdkwork/appbase-app-sdk',
        '@sdkwork/appbase-backend-sdk',
        '@sdkwork/drive-app-sdk',
        '@sdkwork/notary-app-sdk',
        '@sdkwork/im-sdk',
        '@sdkwork/rtc-sdk',
        '@sdkwork/appbase-pc-react',
        '@sdkwork/auth-pc-react',
        '@sdkwork/auth-runtime-pc-react',
        '@sdkwork/auth-pc-react/auth',
        '@sdkwork/iam-contracts',
        '@sdkwork/iam-sdk-ports',
        '@sdkwork/i18n-pc-react',
        '@sdkwork/sdk-common',
        '@sdkwork/core-pc-react',
        '@sdkwork/ui-pc-react',
      ],
    },
    build: {
      rollupOptions: {
        output: {
          manualChunks(id) {
            if (id.includes('/node_modules/react') || id.includes('/node_modules/react-dom')) {
              return 'react-vendor';
            }
            if (id.includes('/node_modules/@tiptap') || id.includes('/node_modules/prosemirror')) {
              return 'editor-vendor';
            }
            return undefined;
          },
        },
      },
      chunkSizeWarningLimit: 2000,
    },
  };
});
