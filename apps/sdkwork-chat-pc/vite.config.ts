import tailwindcss from '@tailwindcss/vite';
import react from '@vitejs/plugin-react';
import path from 'path';
import {defineConfig, loadEnv, type Plugin} from 'vite';
import { handleSdkworkChatLocalApiRequest } from './local-api';

const generatedImAppSdkEntry = path.resolve(
  __dirname,
  '../../sdks/sdkwork-im-app-sdk/sdkwork-im-app-sdk-typescript/generated/server-openapi/src/index.ts',
);
const generatedImBackendSdkEntry = path.resolve(
  __dirname,
  '../../sdks/sdkwork-im-backend-sdk/sdkwork-im-backend-sdk-typescript/generated/server-openapi/src/index.ts',
);
const generatedAgentAppSdkEntry = path.resolve(
  __dirname,
  '../../../../../../../sdkwork-opensource/sdkwork-kernel/sdks/sdkwork-agent-app-sdk/sdkwork-agent-app-sdk-typescript/generated/server-openapi/src/index.ts',
);
const generatedImSdkEntry = path.resolve(
  __dirname,
  '../../sdks/sdkwork-im-sdk/sdkwork-im-sdk-typescript/src/index.ts',
);
const generatedRtcSdkEntry = path.resolve(
  __dirname,
  '../../sdks/sdkwork-rtc-sdk/sdkwork-rtc-sdk-typescript/src/index.ts',
);
const appbasePcReactEntry = path.resolve(
  __dirname,
  '../../../../apps/sdkwork-appbase/packages/pc-react/foundation/sdkwork-appbase-pc-react/src/index.ts',
);
const authPcReactEntry = path.resolve(
  __dirname,
  '../../../../apps/sdkwork-appbase/packages/pc-react/iam/sdkwork-auth-pc-react/src/index.ts',
);
const authPcReactAuthEntry = path.resolve(
  __dirname,
  '../../../../apps/sdkwork-appbase/packages/pc-react/iam/sdkwork-auth-pc-react/src/auth.ts',
);
const iamContractsEntry = path.resolve(
  __dirname,
  '../../../../apps/sdkwork-appbase/packages/common/iam/sdkwork-iam-contracts/src/index.ts',
);
const iamSdkAdapterEntry = path.resolve(
  __dirname,
  '../../../../apps/sdkwork-appbase/packages/common/iam/sdkwork-iam-sdk-adapter/src/index.ts',
);
const iamSdkPortsEntry = path.resolve(
  __dirname,
  '../../../../apps/sdkwork-appbase/packages/common/iam/sdkwork-iam-sdk-ports/src/index.ts',
);
const i18nPcReactEntry = path.resolve(
  __dirname,
  '../../../../apps/sdkwork-appbase/packages/pc-react/foundation/sdkwork-i18n-pc-react/src/index.ts',
);
const corePcReactEntry = path.resolve(
  __dirname,
  '../../../../apps/sdkwork-core/sdkwork-core-pc-react/src',
);
const uiPcReactSourceRoot = path.resolve(
  __dirname,
  '../../../../apps/sdkwork-ui/sdkwork-ui-pc-react/src',
);
const uiPcReactEntry = path.resolve(
  __dirname,
  '../../../../apps/sdkwork-ui/sdkwork-ui-pc-react/src/index.ts',
);
const uiPcReactStylesEntry = path.resolve(
  __dirname,
  '../../../../apps/sdkwork-ui/sdkwork-ui-pc-react/src/styles/sdkwork-ui.css',
);
const sdkCommonSourceRoot = path.resolve(
  __dirname,
  '../../../../sdk/sdkwork-sdk-commons/sdkwork-sdk-common-typescript/src',
);
const sdkCommonEntry = path.resolve(
  sdkCommonSourceRoot,
  'index.ts',
);

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
    plugins: [sdkworkChatLocalApiPlugin(), react(), tailwindcss()],
    define: {
      'process.env.GEMINI_API_KEY': JSON.stringify(env.GEMINI_API_KEY),
    },
    resolve: {
      alias: [
        { find: '@', replacement: path.resolve(__dirname, '.') },
        { find: '@sdkwork-internal/im-app-api-generated', replacement: generatedImAppSdkEntry },
        { find: '@sdkwork-internal/im-backend-api-generated', replacement: generatedImBackendSdkEntry },
        { find: '@sdkwork/agent-app-sdk', replacement: generatedAgentAppSdkEntry },
        { find: '@sdkwork/im-sdk', replacement: generatedImSdkEntry },
        { find: '@sdkwork/rtc-sdk', replacement: generatedRtcSdkEntry },
        { find: '@sdkwork/appbase-pc-react', replacement: appbasePcReactEntry },
        { find: '@sdkwork/auth-pc-react/auth', replacement: authPcReactAuthEntry },
        { find: '@sdkwork/auth-pc-react', replacement: authPcReactEntry },
        { find: '@sdkwork/iam-contracts', replacement: iamContractsEntry },
        { find: '@sdkwork/iam-sdk-adapter', replacement: iamSdkAdapterEntry },
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
        { find: '@sdkwork/clawchat-pc-types', replacement: path.resolve(__dirname, './packages/sdkwork-clawchat-pc-types/src') },
        { find: '@sdkwork/clawchat-pc-commons', replacement: path.resolve(__dirname, './packages/sdkwork-clawchat-pc-commons/src') },
        { find: '@sdkwork/clawchat-pc-core', replacement: path.resolve(__dirname, './packages/sdkwork-clawchat-pc-core/src') },
        { find: '@sdkwork/clawchat-pc-chat', replacement: path.resolve(__dirname, './packages/sdkwork-clawchat-pc-chat/src') },
        { find: '@sdkwork/clawchat-pc-agent', replacement: path.resolve(__dirname, './packages/sdkwork-clawchat-pc-agent/src') },
        { find: '@sdkwork/clawchat-pc-voice', replacement: path.resolve(__dirname, './packages/sdkwork-clawchat-pc-voice/src') },
        { find: '@sdkwork/clawchat-pc-workspace', replacement: path.resolve(__dirname, './packages/sdkwork-clawchat-pc-workspace/src') },
        { find: '@sdkwork/clawchat-pc-orders', replacement: path.resolve(__dirname, './packages/sdkwork-clawchat-pc-orders/src') },
        { find: '@sdkwork/clawchat-pc-notary', replacement: path.resolve(__dirname, './packages/sdkwork-clawchat-pc-notary/src') },
        { find: '@sdkwork/clawchat-pc-mail', replacement: path.resolve(__dirname, './packages/sdkwork-clawchat-pc-mail/src') },
        { find: '@sdkwork/clawchat-pc-drive', replacement: path.resolve(__dirname, './packages/sdkwork-clawchat-pc-drive/src') },
        { find: '@sdkwork/clawchat-pc-contacts', replacement: path.resolve(__dirname, './packages/sdkwork-clawchat-pc-contacts/src') },
        { find: '@sdkwork/clawchat-pc-calendar', replacement: path.resolve(__dirname, './packages/sdkwork-clawchat-pc-calendar/src') },
        { find: '@sdkwork/clawchat-pc-shop', replacement: path.resolve(__dirname, './packages/sdkwork-clawchat-pc-shop/src') },
        { find: '@sdkwork/clawchat-pc-knowledge', replacement: path.resolve(__dirname, './packages/sdkwork-clawchat-pc-knowledge/src') },
        { find: '@sdkwork/clawchat-pc-devices', replacement: path.resolve(__dirname, './packages/sdkwork-clawchat-pc-devices/src') },
        { find: '@sdkwork/clawchat-pc-community', replacement: path.resolve(__dirname, './packages/sdkwork-clawchat-pc-community/src') },
        { find: '@sdkwork/clawchat-pc-course', replacement: path.resolve(__dirname, './packages/sdkwork-clawchat-pc-course/src') },
        { find: '@sdkwork/clawchat-pc-enterprise', replacement: path.resolve(__dirname, './packages/sdkwork-clawchat-pc-enterprise/src') },
        { find: '@sdkwork/clawchat-console-core', replacement: path.resolve(__dirname, './packages/sdkwork-clawchat-console-core/src') },
        { find: '@sdkwork/clawchat-admin-core', replacement: path.resolve(__dirname, './packages/sdkwork-clawchat-admin-core/src') },
        { find: '@sdkwork/clawchat-pc-approvals', replacement: path.resolve(__dirname, './packages/sdkwork-clawchat-pc-approvals/src') },
        { find: '@sdkwork/clawchat-pc-reports', replacement: path.resolve(__dirname, './packages/sdkwork-clawchat-pc-reports/src') },
        { find: '@sdkwork/clawchat-pc-attendance', replacement: path.resolve(__dirname, './packages/sdkwork-clawchat-pc-attendance/src') },
        { find: '@sdkwork/clawchat-console-users', replacement: path.resolve(__dirname, './packages/sdkwork-clawchat-console-users/src') },
        { find: '@sdkwork/clawchat-console-roles', replacement: path.resolve(__dirname, './packages/sdkwork-clawchat-console-roles/src') },
        { find: '@sdkwork/clawchat-console-communications', replacement: path.resolve(__dirname, './packages/sdkwork-clawchat-console-communications/src') },
        { find: '@sdkwork/clawchat-console-integrations', replacement: path.resolve(__dirname, './packages/sdkwork-clawchat-console-integrations/src') },
        { find: '@sdkwork/clawchat-console-security', replacement: path.resolve(__dirname, './packages/sdkwork-clawchat-console-security/src') },
        { find: '@sdkwork/clawchat-console-settings', replacement: path.resolve(__dirname, './packages/sdkwork-clawchat-console-settings/src') },
        { find: '@sdkwork/clawchat-console-shop', replacement: path.resolve(__dirname, './packages/sdkwork-clawchat-console-shop/src') },
        { find: '@sdkwork/clawchat-console-product', replacement: path.resolve(__dirname, './packages/sdkwork-clawchat-console-product/src') },
        { find: '@sdkwork/clawchat-admin-tenants', replacement: path.resolve(__dirname, './packages/sdkwork-clawchat-admin-tenants/src') },
        { find: '@sdkwork/clawchat-admin-infrastructure', replacement: path.resolve(__dirname, './packages/sdkwork-clawchat-admin-infrastructure/src') },
        { find: '@sdkwork/clawchat-admin-operations', replacement: path.resolve(__dirname, './packages/sdkwork-clawchat-admin-operations/src') },
        { find: '@sdkwork/clawchat-console-dashboard', replacement: path.resolve(__dirname, './packages/sdkwork-clawchat-console-dashboard/src') },
        { find: '@sdkwork/clawchat-admin-dashboard', replacement: path.resolve(__dirname, './packages/sdkwork-clawchat-admin-dashboard/src') },
        { find: '@sdkwork/clawchat-pc-video-gen', replacement: path.resolve(__dirname, './packages/sdkwork-clawchat-pc-video-gen/src') },
        { find: '@sdkwork/clawchat-pc-image-gen', replacement: path.resolve(__dirname, './packages/sdkwork-clawchat-pc-image-gen/src') },
        { find: '@sdkwork/clawchat-pc-voice-gen', replacement: path.resolve(__dirname, './packages/sdkwork-clawchat-pc-voice-gen/src') },
        { find: '@sdkwork/clawchat-pc-music-gen', replacement: path.resolve(__dirname, './packages/sdkwork-clawchat-pc-music-gen/src') },
        { find: '@sdkwork/clawchat-pc-writing', replacement: path.resolve(__dirname, './packages/sdkwork-clawchat-pc-writing/src') },
      ],
      dedupe: ['react', 'react-dom'],
    },
    server: {
      // HMR is disabled in AI Studio via DISABLE_HMR env var.
      // Do not modifyâfile watching is disabled to prevent flickering during agent edits.
      hmr: process.env.DISABLE_HMR !== 'true',
    },
    optimizeDeps: {
      exclude: [
        '@sdkwork-internal/im-app-api-generated',
        '@sdkwork-internal/im-backend-api-generated',
        '@sdkwork/agent-app-sdk',
        '@sdkwork/im-sdk',
        '@sdkwork/rtc-sdk',
        '@sdkwork/appbase-pc-react',
        '@sdkwork/auth-pc-react',
        '@sdkwork/auth-pc-react/auth',
        '@sdkwork/iam-contracts',
        '@sdkwork/iam-sdk-adapter',
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
            if (id.includes('/node_modules/@google/genai')) {
              return 'ai-vendor';
            }
            return undefined;
          },
        },
      },
    },
  };
});
