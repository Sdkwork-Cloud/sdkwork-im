import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const repoRoot = path.resolve(__dirname, '..', '..');
const appRoot = path.join(repoRoot, 'apps/sdkwork-chat-pc');
const desktopPackageRoot = path.join(appRoot, 'packages/sdkwork-clawchat-pc-desktop');

function read(relativePath) {
  return fs.readFileSync(path.join(repoRoot, relativePath), 'utf8');
}

function readJson(relativePath) {
  return JSON.parse(read(relativePath));
}

function extractYamlSchemaBlock(source, schemaName) {
  const marker = `    ${schemaName}:\n`;
  const start = source.indexOf(marker);
  assert.notEqual(start, -1, `IM OpenAPI must define ${schemaName}`);

  const rest = source.slice(start + marker.length);
  const nextSchema = /\n    [A-Za-z0-9][A-Za-z0-9_]*:\n/u.exec(rest);
  const end = nextSchema ? start + marker.length + nextSchema.index : source.length;
  return source.slice(start, end);
}

function assertFile(relativePath, message) {
  assert.ok(fs.existsSync(path.join(repoRoot, relativePath)), message ?? `${relativePath} must exist`);
}

function listFiles(rootRelativePath, predicate) {
  const root = path.join(repoRoot, rootRelativePath);
  const result = [];
  const walk = (dir) => {
    for (const entry of fs.readdirSync(dir, { withFileTypes: true })) {
      const fullPath = path.join(dir, entry.name);
      if (entry.isDirectory()) {
        walk(fullPath);
        continue;
      }
      if (!predicate || predicate(fullPath)) {
        result.push(fullPath);
      }
    }
  };
  walk(root);
  return result;
}

const appPackageJson = readJson('apps/sdkwork-chat-pc/package.json');
const retiredGenericAppSdkPackage = `@sdkwork/${'app'}-sdk`;
const retiredGenericBackendSdkPackage = `@sdkwork/${'backend'}-sdk`;

assert.equal(appPackageJson.name, '@sdkwork/chat-pc', 'desktop app package must use a standard SDKWork package name');
assert.equal(appPackageJson.scripts.dev, 'vite --host 127.0.0.1 --port 1620 --strictPort');
assert.equal(appPackageJson.scripts['dev:tauri'], 'vite --host 127.0.0.1 --port 1620 --strictPort');
assert.equal(appPackageJson.scripts['desktop:dev:local'], 'pnpm --filter @sdkwork/clawchat-pc-desktop desktop:dev:local');
assert.equal(appPackageJson.scripts['desktop:build:local'], 'pnpm --filter @sdkwork/clawchat-pc-desktop desktop:build:local');
assert.ok(
  appPackageJson.dependencies['@sdkwork-internal/im-app-api-generated'],
  'PC app must depend on generated sdkwork-im-app-sdk TypeScript package',
);
assert.ok(
  appPackageJson.dependencies['@sdkwork-internal/im-backend-api-generated'],
  'PC app must depend on generated sdkwork-im-backend-sdk TypeScript package so backend SDK ownership is explicit',
);
assert.ok(
  !appPackageJson.dependencies[retiredGenericAppSdkPackage],
  'PC app must not depend on the generic spring-ai-plus app SDK package',
);
assert.ok(
  !appPackageJson.dependencies[retiredGenericBackendSdkPackage],
  'PC app must not depend on the generic spring-ai-plus backend SDK package',
);
assert.ok(appPackageJson.dependencies['@sdkwork/im-sdk'], 'PC app must depend on generated @sdkwork/im-sdk');
assert.ok(appPackageJson.dependencies['@sdkwork/rtc-sdk'], 'PC app must depend on standard @sdkwork/rtc-sdk for call capability');
assert.ok(appPackageJson.dependencies['@sdkwork/appbase-pc-react'], 'PC app must depend on sdkwork-appbase PC wrapper');
assert.ok(appPackageJson.dependencies['@sdkwork/auth-runtime-pc-react'], 'PC app must depend on the appbase high-level auth runtime');
assert.equal(
  appPackageJson.dependencies['@sdkwork/iam-sdk-adapter'],
  undefined,
  'PC app must not depend on the lower-level appbase IAM SDK adapter after appbase auth runtime migration',
);
assert.ok(appPackageJson.dependencies['@sdkwork/iam-sdk-ports'], 'PC app must depend on the appbase IAM SDK ports');
assert.ok(!appPackageJson.dependencies['@tauri-apps/api'], 'Tauri renderer API must live in the desktop workspace package');
assert.ok(!appPackageJson.devDependencies['@tauri-apps/cli'], 'Tauri CLI must live in the desktop workspace package');
assert.ok(!fs.existsSync(path.join(appRoot, 'src-tauri')), 'root sdkwork-chat-pc must not own the Tauri shell directly');
assert.ok(fs.existsSync(path.join(desktopPackageRoot, 'node_modules/@tauri-apps/cli')), 'desktop package must have its own Tauri CLI dependency installed');

const desktopPackageJson = readJson('apps/sdkwork-chat-pc/packages/sdkwork-clawchat-pc-desktop/package.json');
assert.equal(desktopPackageJson.name, '@sdkwork/clawchat-pc-desktop');
assert.equal(desktopPackageJson.scripts['desktop:dev:local'], 'node ../../../../scripts/run-tauri-cli.mjs dev');
assert.equal(desktopPackageJson.scripts['desktop:build:local'], 'node ../../../../scripts/run-tauri-cli.mjs build');
assert.equal(desktopPackageJson.scripts['dev:tauri'], 'pnpm --dir ../.. dev:tauri');
assert.equal(desktopPackageJson.scripts.build, 'pnpm --dir ../.. build');
assert.equal(desktopPackageJson.devDependencies['@tauri-apps/cli'], 'catalog:');
assert.equal(desktopPackageJson.dependencies['@tauri-apps/api'], 'catalog:');

assertFile('apps/sdkwork-chat-pc/packages/sdkwork-clawchat-pc-desktop/src-tauri/tauri.conf.json');
assertFile('apps/sdkwork-chat-pc/packages/sdkwork-clawchat-pc-desktop/src-tauri/Cargo.toml');
assertFile('apps/sdkwork-chat-pc/packages/sdkwork-clawchat-pc-desktop/src-tauri/src/main.rs');
assertFile('apps/sdkwork-chat-pc/packages/sdkwork-clawchat-pc-desktop/src-tauri/build.rs');
assertFile('apps/sdkwork-chat-pc/packages/sdkwork-clawchat-pc-desktop/src-tauri/icons/icon.ico');
assertFile('apps/sdkwork-chat-pc/packages/sdkwork-clawchat-pc-desktop/src-tauri/icons/32x32.png');
assertFile('apps/sdkwork-chat-pc/packages/sdkwork-clawchat-pc-desktop/src-tauri/icons/128x128.png');
assertFile('apps/sdkwork-chat-pc/packages/sdkwork-clawchat-pc-desktop/src-tauri/icons/128x128@2x.png');

const tauriCargoToml = read('apps/sdkwork-chat-pc/packages/sdkwork-clawchat-pc-desktop/src-tauri/Cargo.toml');
assert.match(tauriCargoToml, /tauri\s*=\s*\{[^}]*version\s*=\s*"~2\.11\.0"/u);
assert.match(tauriCargoToml, /tauri-plugin-shell\s*=\s*"2"/u);
assert.match(tauriCargoToml, /serde_json\s*=\s*"1"/u);
assert.match(tauriCargoToml, /\n\[workspace\]\s*$/u, 'Tauri crate must be an independent workspace inside the Craw Chat Rust workspace');

const tauriConfig = readJson('apps/sdkwork-chat-pc/packages/sdkwork-clawchat-pc-desktop/src-tauri/tauri.conf.json');
assert.equal(tauriConfig.productName, 'SDKWork Chat PC');
assert.equal(tauriConfig.identifier, 'com.sdkwork.chatpc');
assert.equal(tauriConfig.build.devUrl, 'http://127.0.0.1:1620');
assert.equal(tauriConfig.build.frontendDist, '../../../dist');
assert.equal(tauriConfig.app.windows[0].decorations, false, 'Tauri shell must preserve the current custom titlebar');
assert.equal(tauriConfig.app.windows[0].minWidth, 1200);
assert.equal(tauriConfig.app.windows[0].minHeight, 760);
assert.deepEqual(tauriConfig.bundle.icon, [
  'icons/32x32.png',
  'icons/128x128.png',
  'icons/128x128@2x.png',
  'icons/icon.ico',
]);

const viteConfig = read('apps/sdkwork-chat-pc/vite.config.ts');
const tsconfig = read('apps/sdkwork-chat-pc/tsconfig.json');
assert.match(viteConfig, /@sdkwork-internal\/im-app-api-generated/u, 'Vite must alias generated IM app SDK source');
assert.match(
  viteConfig,
  /sdks[\\\/]sdkwork-im-app-sdk[\\\/]sdkwork-im-app-sdk-typescript[\\\/]generated[\\\/]server-openapi[\\\/]src[\\\/]index\.ts/u,
);
assert.match(
  viteConfig,
  /@sdkwork-internal\/im-backend-api-generated/u,
  'Vite must alias generated IM backend SDK source',
);
assert.match(
  viteConfig,
  /sdks[\\\/]sdkwork-im-backend-sdk[\\\/]sdkwork-im-backend-sdk-typescript[\\\/]generated[\\\/]server-openapi[\\\/]src[\\\/]index\.ts/u,
);
assert.match(
  tsconfig,
  /sdks[\\\/]sdkwork-im-app-sdk[\\\/]sdkwork-im-app-sdk-typescript[\\\/]generated[\\\/]server-openapi[\\\/]src[\\\/]index\.ts/u,
  'TypeScript must resolve generated IM app SDK from source for live development',
);
assert.match(
  tsconfig,
  /sdks[\\\/]sdkwork-im-backend-sdk[\\\/]sdkwork-im-backend-sdk-typescript[\\\/]generated[\\\/]server-openapi[\\\/]src[\\\/]index\.ts/u,
  'TypeScript must resolve generated IM backend SDK from source for live development',
);
assert.doesNotMatch(
  viteConfig,
  /spring-ai-plus-(?:app|backend)-api|@sdkwork\/(?:app|backend)-sdk/u,
  'Vite must not route the PC app through generic spring-ai-plus app/backend SDKs',
);
assert.doesNotMatch(
  tsconfig,
  /spring-ai-plus-(?:app|backend)-api|@sdkwork\/(?:app|backend)-sdk/u,
  'TypeScript must not route the PC app through generic spring-ai-plus app/backend SDKs',
);
assert.match(viteConfig, /@sdkwork\/im-sdk/u, 'Vite must alias generated IM SDK source');
assert.match(viteConfig, /sdks[\\\/]sdkwork-im-sdk[\\\/]sdkwork-im-sdk-typescript[\\\/]src[\\\/]index\.ts/u);
assert.match(
  tsconfig,
  /sdks[\\\/]sdkwork-im-sdk[\\\/]sdkwork-im-sdk-typescript[\\\/]src[\\\/]index\.ts/u,
  'TypeScript must resolve generated IM SDK from source for live development',
);
assert.match(viteConfig, /@sdkwork\/rtc-sdk/u, 'Vite must alias generated RTC SDK source');
assert.match(viteConfig, /sdkwork-rtc[\\\/]sdks[\\\/]sdkwork-rtc-sdk[\\\/]sdkwork-rtc-sdk-typescript[\\\/]src[\\\/]index\.ts/u);
assert.match(
  tsconfig,
  /sdkwork-rtc[\\\/]sdks[\\\/]sdkwork-rtc-sdk[\\\/]sdkwork-rtc-sdk-typescript[\\\/]src[\\\/]index\.ts/u,
  'TypeScript must resolve generated RTC SDK from source for live development',
);
assert.match(viteConfig, /@sdkwork\/appbase-pc-react/u, 'Vite must alias appbase PC package source');
assert.match(viteConfig, /@sdkwork\/core-pc-react/u, 'Vite must alias SDKWork core PC React package');
assert.match(
  viteConfig,
  /sdkwork-core[\\\/]sdkwork-core-pc-react[\\\/]src/u,
  'Vite must alias SDKWork core PC React to source for live development',
);
assert.doesNotMatch(
  viteConfig,
  /sdkwork-core[\\\/]sdkwork-core-pc-react[\\\/]dist[\\\/]index\.js/u,
  'Vite must not consume the prebuilt SDKWork core PC React dist artifact during local development',
);
assert.doesNotMatch(
  tsconfig,
  /sdkwork-core[\\\/]sdkwork-core-pc-react[\\\/]dist[\\\/]index\.d\.ts/u,
  'TypeScript must not consume the prebuilt SDKWork core PC React declaration artifact during local development',
);
assert.match(viteConfig, /@sdkwork\/ui-pc-react/u, 'Vite must alias SDKWork UI PC React package');
assert.match(
  viteConfig,
  /sdkwork-ui[\\\/]sdkwork-ui-pc-react[\\\/]src/u,
  'Vite must alias SDKWork UI PC React to source for live development',
);
assert.doesNotMatch(
  viteConfig,
  /sdkwork-ui[\\\/]sdkwork-ui-pc-react[\\\/]dist[\\\/]index\.js/u,
  'Vite must not consume the prebuilt SDKWork UI PC React dist artifact during local development',
);
assert.doesNotMatch(
  tsconfig,
  /sdkwork-ui[\\\/]sdkwork-ui-pc-react[\\\/]dist[\\\/]index\.d\.ts/u,
  'TypeScript must not consume the prebuilt SDKWork UI PC React declaration artifact during local development',
);
for (const localReactPackageName of ['@sdkwork/core-pc-react', '@sdkwork/ui-pc-react']) {
  assert.match(
    viteConfig,
    new RegExp(`optimizeDeps:[\\s\\S]*exclude:[\\s\\S]*['"]${localReactPackageName.replaceAll('/', '\\/')}['"]`, 'u'),
    `Vite must exclude ${localReactPackageName} from dependency pre-bundling so source edits stay live`,
  );
}
for (const localSdkPackageName of [
  '@sdkwork-internal/im-app-api-generated',
  '@sdkwork-internal/im-backend-api-generated',
  '@sdkwork/appbase-app-sdk',
  '@sdkwork/im-sdk',
  '@sdkwork/rtc-sdk',
  '@sdkwork/appbase-pc-react',
  '@sdkwork/auth-pc-react',
  '@sdkwork/auth-pc-react/auth',
  '@sdkwork/auth-runtime-pc-react',
  '@sdkwork/iam-contracts',
  '@sdkwork/iam-sdk-ports',
  '@sdkwork/i18n-pc-react',
]) {
  assert.match(
    viteConfig,
    new RegExp(`optimizeDeps:[\\s\\S]*exclude:[\\s\\S]*['"]${localSdkPackageName.replaceAll('/', '\\/')}['"]`, 'u'),
    `Vite must exclude ${localSdkPackageName} from dependency pre-bundling so source edits stay live`,
  );
}
assert.match(
  viteConfig,
  /manualChunks/u,
  'Vite release build must define manual chunks for stable dependency packaging',
);
for (const chunkName of ['react-vendor', 'editor-vendor', 'ai-vendor']) {
  assert.match(
    viteConfig,
    new RegExp(chunkName, 'u'),
    `Vite release build must expose a ${chunkName} chunk`,
  );
}
assert.doesNotMatch(
  viteConfig,
  /return ['"]sdkwork-shared['"]/u,
  'Vite must not split linked SDKWork source packages into a circular shared chunk',
);

const iamEnvModule = await import(pathToFileURL(path.join(appRoot, 'scripts/sdkwork-chat-iam-env.mjs')).href);
const resolvedEnv = iamEnvModule.resolveSdkworkChatIamCommandEnv({
  env: {},
  iamMode: 'desktop-local',
  target: 'desktop-dev',
});
assert.deepEqual(resolvedEnv.errors, []);
assert.equal(resolvedEnv.env.SDKWORK_IAM_MODE, 'local');
assert.equal(resolvedEnv.env.VITE_SDKWORK_DEPLOYMENT_MODE, 'local');
assert.equal(resolvedEnv.env.VITE_SDKWORK_IAM_APP_API_BASE_URL, 'http://127.0.0.1:18079');
assert.equal(resolvedEnv.env.VITE_CRAW_CHAT_APP_API_BASE_URL, 'http://127.0.0.1:18079');
assert.equal(resolvedEnv.env.VITE_CRAW_CHAT_IM_API_BASE_URL, 'http://127.0.0.1:18079');
assert.equal(resolvedEnv.env.VITE_CRAW_CHAT_IM_WEBSOCKET_BASE_URL, 'ws://127.0.0.1:18079');
assert.equal(resolvedEnv.env.SDKWORK_IAM_LOCAL_VERIFY_CODE_FIXED, '123456');

const webBuildSameOriginEnv = iamEnvModule.resolveSdkworkChatIamCommandEnv({
  env: {},
  target: 'web-build',
});
assert.deepEqual(webBuildSameOriginEnv.errors, []);
assert.equal(webBuildSameOriginEnv.env.SDKWORK_IAM_MODE, 'private');
assert.equal(webBuildSameOriginEnv.env.VITE_SDKWORK_DEPLOYMENT_MODE, 'private');
assert.equal(
  webBuildSameOriginEnv.env.VITE_CRAW_CHAT_APP_API_BASE_URL,
  undefined,
  'web release builds without explicit domain binding must not bake localhost app API URLs into the bundle',
);
assert.equal(
  webBuildSameOriginEnv.env.VITE_CRAW_CHAT_IM_API_BASE_URL,
  undefined,
  'web release builds without explicit domain binding must let the runtime resolve IM HTTP from window.location.origin',
);
assert.equal(
  webBuildSameOriginEnv.env.VITE_CRAW_CHAT_IM_WEBSOCKET_BASE_URL,
  undefined,
  'web release builds without explicit domain binding must let the runtime derive IM websocket from the public origin',
);

const releaseDomainEnv = iamEnvModule.resolveSdkworkChatIamCommandEnv({
  env: {
    CRAW_CHAT_APP_API_BASE_URL: 'https://chat.example.com/',
    CRAW_CHAT_IM_API_BASE_URL: 'https://chat.example.com',
    CRAW_CHAT_IM_WEBSOCKET_BASE_URL: 'wss://chat.example.com',
  },
  target: 'server-build',
});
assert.deepEqual(releaseDomainEnv.errors, []);
assert.equal(releaseDomainEnv.env.VITE_CRAW_CHAT_APP_API_BASE_URL, 'https://chat.example.com');
assert.equal(releaseDomainEnv.env.VITE_CRAW_CHAT_IM_API_BASE_URL, 'https://chat.example.com');
assert.equal(releaseDomainEnv.env.VITE_CRAW_CHAT_IM_WEBSOCKET_BASE_URL, 'wss://chat.example.com');

const releaseServerDomainEnv = iamEnvModule.resolveSdkworkChatIamCommandEnv({
  env: {
    CRAW_CHAT_SERVER_API_BASE_URL: 'https://chat.example.com/',
    CRAW_CHAT_SERVER_WEBSOCKET_BASE_URL: 'wss://chat.example.com/im',
  },
  target: 'server-build',
});
assert.deepEqual(releaseServerDomainEnv.errors, []);
assert.equal(
  releaseServerDomainEnv.env.VITE_CRAW_CHAT_APP_API_BASE_URL,
  'https://chat.example.com',
  'release builds must bind app API traffic from the standard server api domain env',
);
assert.equal(
  releaseServerDomainEnv.env.VITE_CRAW_CHAT_IM_API_BASE_URL,
  'https://chat.example.com',
  'release builds must bind IM HTTP traffic from the standard server api domain env',
);
assert.equal(
  releaseServerDomainEnv.env.VITE_CRAW_CHAT_IM_WEBSOCKET_BASE_URL,
  'wss://chat.example.com/im',
  'release builds must bind IM websocket traffic from the standard server websocket domain env',
);

const releaseCanonicalServerDomainEnv = iamEnvModule.resolveSdkworkChatIamCommandEnv({
  env: {
    SDKWORK_CHAT_SERVER_API_BASE_URL: 'https://chat.example.com/sdkwork/chat/',
    SDKWORK_CHAT_SERVER_WEBSOCKET_BASE_URL: 'wss://chat.example.com/sdkwork/chat',
  },
  target: 'server-build',
});
assert.deepEqual(releaseCanonicalServerDomainEnv.errors, []);
assert.equal(
  releaseCanonicalServerDomainEnv.env.VITE_CRAW_CHAT_APP_API_BASE_URL,
  'https://chat.example.com/sdkwork/chat',
  'release builds must bind app API traffic from the canonical SDKWORK_CHAT server API env',
);
assert.equal(
  releaseCanonicalServerDomainEnv.env.VITE_CRAW_CHAT_IM_API_BASE_URL,
  'https://chat.example.com/sdkwork/chat',
  'release builds must bind IM HTTP traffic from the canonical SDKWORK_CHAT server API env',
);
assert.equal(
  releaseCanonicalServerDomainEnv.env.VITE_CRAW_CHAT_IM_WEBSOCKET_BASE_URL,
  'wss://chat.example.com/sdkwork/chat',
  'release builds must bind IM websocket traffic from the canonical SDKWORK_CHAT server websocket env',
);

const releaseServerApiOnlyDomainEnv = iamEnvModule.resolveSdkworkChatIamCommandEnv({
  env: {
    CRAW_CHAT_SERVER_API_BASE_URL: 'https://api.chat.example.com/',
  },
  target: 'server-build',
});
assert.deepEqual(releaseServerApiOnlyDomainEnv.errors, []);
assert.equal(
  releaseServerApiOnlyDomainEnv.env.VITE_CRAW_CHAT_APP_API_BASE_URL,
  'https://api.chat.example.com',
  'release builds must bind app API traffic when only the standard server API domain is configured',
);
assert.equal(
  releaseServerApiOnlyDomainEnv.env.VITE_CRAW_CHAT_IM_API_BASE_URL,
  'https://api.chat.example.com',
  'release builds must bind IM HTTP traffic when only the standard server API domain is configured',
);
assert.equal(
  releaseServerApiOnlyDomainEnv.env.VITE_CRAW_CHAT_IM_WEBSOCKET_BASE_URL,
  'wss://api.chat.example.com',
  'release builds must derive IM websocket base URL from the server API domain when no websocket domain is configured',
);

const releaseFullContractPathEnv = iamEnvModule.resolveSdkworkChatIamCommandEnv({
  env: {
    CRAW_CHAT_APP_API_BASE_URL: 'https://chat.example.com/app/v3/api/',
    CRAW_CHAT_IM_API_BASE_URL: 'https://chat.example.com/im/v3/api/',
    CRAW_CHAT_IM_WEBSOCKET_BASE_URL: 'wss://chat.example.com/im/v3/api/realtime/ws',
  },
  target: 'server-build',
});
assert.deepEqual(releaseFullContractPathEnv.errors, []);
assert.equal(
  releaseFullContractPathEnv.env.VITE_CRAW_CHAT_APP_API_BASE_URL,
  'https://chat.example.com',
  'release app API base URL must strip the SDK-owned /app/v3/api contract prefix',
);
assert.equal(
  releaseFullContractPathEnv.env.VITE_CRAW_CHAT_IM_API_BASE_URL,
  'https://chat.example.com',
  'release IM API base URL must strip the SDK-owned /im/v3/api contract prefix',
);
assert.equal(
  releaseFullContractPathEnv.env.VITE_CRAW_CHAT_IM_WEBSOCKET_BASE_URL,
  'wss://chat.example.com',
  'release IM websocket base URL must strip the SDK-owned realtime websocket path',
);

const invalidReleaseDomainEnv = iamEnvModule.resolveSdkworkChatIamCommandEnv({
  env: {
    CRAW_CHAT_SERVER_API_BASE_URL: 'chat.example.com',
    CRAW_CHAT_SERVER_WEBSOCKET_BASE_URL: 'https://chat.example.com',
  },
  target: 'server-build',
});
assert.match(
  invalidReleaseDomainEnv.errors.join('\n'),
  /CRAW_CHAT_SERVER_API_BASE_URL/u,
  'release builds must fail when an explicit server API domain is not an absolute http(s) URL',
);
assert.match(
  invalidReleaseDomainEnv.errors.join('\n'),
  /CRAW_CHAT_SERVER_WEBSOCKET_BASE_URL/u,
  'release builds must fail when an explicit server websocket domain is not a ws(s) URL',
);

const serverEnvTemplate = read('deployments/templates/server.env.example');
const quickstartServerEnvTemplate = read('deployments/templates/quickstart-server-compose.env.example');
assert.doesNotMatch(
  serverEnvTemplate,
  /CRAW_CHAT_SERVER_WEBSOCKET_BASE_URL=.*\/im\/v3\/api\/realtime\/ws/u,
  'server env template must document websocket base URL, not the SDK-owned realtime websocket endpoint',
);
assert.match(
  serverEnvTemplate,
  /SDKWORK_CHAT_SERVER_API_BASE_URL=https:\/\/chat\.example\.com\/sdkwork\/chat/u,
  'server env template must document the canonical /sdkwork/chat mount root',
);
assert.match(
  serverEnvTemplate,
  /SDKWORK_CHAT_SERVER_WEBSOCKET_BASE_URL=wss:\/\/realtime\.example\.com\/sdkwork\/chat/u,
  'server env template must document the canonical websocket mount root',
);
assert.doesNotMatch(
  quickstartServerEnvTemplate,
  /CRAW_CHAT_SERVER_WEBSOCKET_BASE_URL=.*\/im\/v3\/api\/realtime\/ws/u,
  'quickstart env template must document websocket base URL, not the SDK-owned realtime websocket endpoint',
);

assertFile('apps/sdkwork-chat-pc/packages/sdkwork-clawchat-pc-core/src/sdk/appSdkClient.ts');
assertFile('apps/sdkwork-chat-pc/packages/sdkwork-clawchat-pc-core/src/sdk/agentAppSdkClient.ts');
assertFile('apps/sdkwork-chat-pc/packages/sdkwork-clawchat-pc-core/src/sdk/appAuthService.ts');
assertFile('apps/sdkwork-chat-pc/packages/sdkwork-clawchat-pc-core/src/sdk/imSdkClient.ts');
assertFile('apps/sdkwork-chat-pc/packages/sdkwork-clawchat-pc-core/src/sdk/session.ts');

const coreIndex = read('apps/sdkwork-chat-pc/packages/sdkwork-clawchat-pc-core/src/index.ts');
assert.match(coreIndex, /export \* from '\.\/sdk\/appSdkClient'/u);
assert.match(coreIndex, /export \* from '\.\/sdk\/appAuthService'/u);
assert.match(coreIndex, /export \* from '\.\/sdk\/imSdkClient'/u);
assert.match(coreIndex, /export \* from '\.\/sdk\/session'/u);

const appSdkClientSource = read('apps/sdkwork-chat-pc/packages/sdkwork-clawchat-pc-core/src/sdk/appSdkClient.ts');
const agentAppSdkClientSource = read('apps/sdkwork-chat-pc/packages/sdkwork-clawchat-pc-core/src/sdk/agentAppSdkClient.ts');
const sessionSource = read('apps/sdkwork-chat-pc/packages/sdkwork-clawchat-pc-core/src/sdk/session.ts');
assert.match(
  appSdkClientSource,
  /from ['"]@sdkwork-internal\/im-app-api-generated['"]/u,
  'app SDK wrapper must use generated sdkwork-im-app-sdk client',
);
assert.match(appSdkClientSource, /createClient/u);
assert.match(appSdkClientSource, /createAppSdkClientConfig/u);
assert.match(appSdkClientSource, /getAppSdkClientWithSession/u);
assert.match(appSdkClientSource, /SdkworkImAppClient/u, 'app SDK wrapper must expose product-scoped SdkworkImAppClient naming');
assert.match(appSdkClientSource, /window\.location\.origin/u, 'app SDK wrapper must support release same-origin domain binding');
assert.match(appSdkClientSource, /if\s*\(\s*!import\.meta\.env\.DEV\s*\)/u, 'app SDK wrapper must keep localhost defaults in a Vite-prunable dev-only branch');
assert.match(
  sessionSource,
  /export function resolveAppSdkUserId/u,
  'session helper must expose the SDKWork AppContext user id from the IAM login session',
);
assert.match(
  sessionSource,
  /export function buildSdkworkChatAppContextHeaders/u,
  'session helper must centralize SDKWork AppContext headers for app and IM SDK clients',
);
assert.match(
  sessionSource,
  /export function createSdkworkChatRequestContext/u,
  'session helper must encapsulate JWT claims and persisted session context into a request Context instance',
);
assert.match(
  sessionSource,
  /export function createSdkworkChatRequestContextInterceptors/u,
  'session helper must expose request interceptors that attach a fresh SDKWork AppContext to every SDK request',
);
assert.match(
  sessionSource,
  /X-Sdkwork-User-Id/u,
  'session helper must include X-Sdkwork-User-Id so IM APIs receive a complete AppContext',
);
assert.match(
  appSdkClientSource,
  /createSdkworkChatRequestContextInterceptors/u,
  'app SDK wrapper must install the shared request Context interceptor so every generated SDK request gets fresh AppContext headers',
);
assert.doesNotMatch(
  appSdkClientSource,
  /headers:\s*buildSdkworkChatAppContextHeaders/u,
  'app SDK wrapper must not keep stale static AppContext headers after token refresh; request Context belongs in the interceptor',
);
assert.doesNotMatch(
  appSdkClientSource,
  /tenantId:\s*resolveAppSdkTenantId|organizationId:\s*resolveAppSdkOrganizationId/u,
  'app SDK wrapper must not pass current tenantId/organizationId as static config; current scope belongs in the request Context interceptor',
);
assert.doesNotMatch(
  appSdkClientSource,
  /const\s+LOCAL_APP_API_BASE_URL\s*=\s*['"]http:\/\/127\.0\.0\.1:18079['"]/u,
  'app SDK wrapper must not keep localhost fallback as a production-retained top-level constant',
);
assert.doesNotMatch(appSdkClientSource, /\bfetch\s*\(/u, 'app SDK wrapper must not use raw fetch');

assert.match(
  agentAppSdkClientSource,
  /from ['"]@sdkwork\/agent-app-sdk['"]/u,
  'agent app SDK wrapper must use the generated agent app SDK client',
);
assert.match(
  agentAppSdkClientSource,
  /createSdkworkChatRequestContextInterceptors/u,
  'agent app SDK wrapper must install the shared request Context interceptor so every generated SDK request gets fresh AppContext headers',
);
assert.doesNotMatch(
  agentAppSdkClientSource,
  /headers:\s*buildSdkworkChatAppContextHeaders/u,
  'agent app SDK wrapper must not keep stale static AppContext headers after token refresh; request Context belongs in the interceptor',
);
assert.doesNotMatch(
  agentAppSdkClientSource,
  /tenantId:\s*resolveAppSdkTenantId|organizationId:\s*resolveAppSdkOrganizationId/u,
  'agent app SDK wrapper must not pass current tenantId/organizationId as static config; current scope belongs in the request Context interceptor',
);
assert.doesNotMatch(agentAppSdkClientSource, /\bfetch\s*\(/u, 'agent app SDK wrapper must not use raw fetch');

const appAuthSource = read('apps/sdkwork-chat-pc/packages/sdkwork-clawchat-pc-core/src/sdk/appAuthService.ts');
const appAuthRuntimeSource = read('apps/sdkwork-chat-pc/packages/sdkwork-clawchat-pc-core/src/sdk/appAuthRuntime.ts');
const appbaseAppSdkClientSource = read('apps/sdkwork-chat-pc/packages/sdkwork-clawchat-pc-core/src/sdk/appbaseAppSdkClient.ts');
assert.match(
  appbaseAppSdkClientSource,
  /from ['"]@sdkwork\/appbase-app-sdk['"]/u,
  'appbase app SDK wrapper must use the generated sdkwork-appbase-app-sdk client for appbase-owned app surfaces',
);
assert.match(
  appAuthSource,
  /import\s+\{[\s\S]*createSdkworkAuthAppbaseIntegration[\s\S]*\}\s+from\s+['"]@sdkwork\/auth-pc-react['"]/u,
  'auth service must integrate the appbase PC wrapper through the high-level auth appbase integration factory',
);
assert.doesNotMatch(
  appAuthSource,
  /@sdkwork\/appbase-pc-react|@sdkwork\/auth-pc-react\/auth|createSdkworkAppCapabilityPresetManifest|createAuthRouteCatalog|appbasePackageMeta|authPackageMeta/u,
  'auth service must not hand-build appbase/auth metadata or route catalogs after the high-level auth appbase integration migration',
);
assert.match(
  appAuthRuntimeSource,
  /@sdkwork\/auth-runtime-pc-react/u,
  'auth runtime must integrate the appbase high-level PC auth runtime package',
);
assert.match(
  appAuthRuntimeSource,
  /createSdkworkAppbasePcAuthRuntime/u,
  'auth runtime must create the appbase auth runtime through the standard high-level factory',
);
for (const method of ['logout', 'getCurrentSession']) {
  assert.match(appAuthSource, new RegExp(`\\b${method}\\s*\\(`, 'u'), `auth service must expose ${method}`);
}
for (const localAuthMethod of ['login', 'register', 'refreshToken', 'sendVerifyCode', 'verifyCode']) {
  assert.doesNotMatch(
    appAuthSource,
    new RegExp(`\\b${localAuthMethod}\\s*\\(`, 'u'),
    `auth service must not expose product-local ${localAuthMethod}; appbase runtime owns auth flows`,
  );
}
for (const generatedMethod of [
  'auth.sessions.create',
  'auth.registrations.create',
  'auth.sessions.refresh',
  'openPlatform.qrAuth.sessions.create',
  'openPlatform.qrAuth.sessions.retrieve',
  'openPlatform.qrAuth.sessions.scans.create',
  'openPlatform.qrAuth.sessions.passwords.create',
]) {
  assert.doesNotMatch(
    `${appAuthSource}\n${appAuthRuntimeSource}`,
    new RegExp(generatedMethod.replace(/[.*+?^${}()|[\]\\]/g, '\\$&'), 'u'),
    `PC product code must not remap ${generatedMethod}; appbase auth runtime owns auth validation and session creation`,
  );
}
assert.match(
  appAuthSource,
  /service\.auth\.sessions\.current\.retrieve[\s\S]*service\.auth\.sessions\.current\.delete/u,
  'auth service may only bridge current-session bootstrap and logout through the appbase runtime service',
);
assert.doesNotMatch(
  `${appAuthSource}\n${appAuthRuntimeSource}`,
  /@sdkwork\/iam-sdk-adapter|createIamAppSdkAdapter|unwrapIamSdkResponse|getIam\(\)/u,
  'auth integration must not import or call the lower-level IAM SDK adapter after appbase auth runtime migration',
);
assert.doesNotMatch(appAuthSource, /\bfetch\s*\(/u, 'auth service must not use raw fetch');
assert.doesNotMatch(appAuthSource, /PlusApiResult|LoginVO|assertSuccess/u, 'auth service must not keep generic spring app SDK envelope shims');

const pcImSdkClientSource = read('apps/sdkwork-chat-pc/packages/sdkwork-clawchat-pc-core/src/sdk/imSdkClient.ts');
assert.match(pcImSdkClientSource, /from ['"]@sdkwork\/im-sdk['"]/u, 'IM wrapper must use generated/composed @sdkwork/im-sdk');
assert.match(pcImSdkClientSource, /new ImSdkClient/u);
assert.match(pcImSdkClientSource, /getImSdkClientWithSession/u);
assert.match(pcImSdkClientSource, /tokenProvider:\s*tokenManager/u, 'IM wrapper must use the same dynamic token manager as IAM login');
assert.match(pcImSdkClientSource, /accessToken:\s*resolveAppSdkAccessToken/u, 'IM wrapper must pass accessToken from IAM login session');
assert.match(
  pcImSdkClientSource,
  /headerProvider:\s*\(\)\s*=>\s*buildImSdkContextHeaders/u,
  'IM wrapper must attach tenant/organization/user AppContext through a fresh per-request headerProvider',
);
assert.doesNotMatch(
  pcImSdkClientSource,
  /tenantId:\s*resolveAppSdkTenantId|organizationId:\s*resolveAppSdkOrganizationId/u,
  'IM wrapper must not pass current tenantId/organizationId as static options; current scope belongs in the request headerProvider',
);
assert.match(
  pcImSdkClientSource,
  /buildSdkworkChatAppContextHeaders/u,
  'IM wrapper must use the shared AppContext header builder so IM APIs receive x-sdkwork-user-id',
);
assert.match(pcImSdkClientSource, /http:\/\/127\.0\.0\.1:18079/u, 'IM wrapper local dev fallback HTTP base URL must point at Craw Chat IM API');
assert.match(pcImSdkClientSource, /ws:\/\/127\.0\.0\.1:18079/u, 'IM wrapper local dev fallback websocket base URL must point at Craw Chat IM API');
assert.match(pcImSdkClientSource, /window\.location\.origin/u, 'IM wrapper must support release same-origin HTTP domain binding');
assert.match(pcImSdkClientSource, /protocol\s*===\s*['"]https:['"]/u, 'IM wrapper must derive wss websocket URLs from https release origins');
assert.match(pcImSdkClientSource, /if\s*\(\s*!import\.meta\.env\.DEV\s*\)/u, 'IM wrapper must keep localhost defaults in Vite-prunable dev-only branches');
assert.doesNotMatch(
  pcImSdkClientSource,
  /const\s+LOCAL_IM_(?:API|WEBSOCKET)_BASE_URL\s*=\s*['"].*127\.0\.0\.1:18079['"]/u,
  'IM wrapper must not keep localhost fallbacks as production-retained top-level constants',
);
assert.doesNotMatch(pcImSdkClientSource, /VITE_CRAW_CHAT_APP_API_BASE_URL/u, 'IM wrapper must not fall back to the appbase App API URL');
assert.doesNotMatch(pcImSdkClientSource, /\bfetch\s*\(/u, 'IM wrapper must not use raw fetch');

const chatServiceSource = read('apps/sdkwork-chat-pc/packages/sdkwork-clawchat-pc-chat/src/services/ChatService.ts');
const imOpenApiSource = read('sdks/sdkwork-im-sdk/openapi/craw-chat-im.openapi.yaml');
const imSdkSource = read('sdks/sdkwork-im-sdk/sdkwork-im-sdk-typescript/src/sdk.ts');
const imConversationsModuleSource = read('sdks/sdkwork-im-sdk/sdkwork-im-sdk-typescript/src/conversations-module.ts');
const imMessagesModuleSource = read('sdks/sdkwork-im-sdk/sdkwork-im-sdk-typescript/src/messages-module.ts');
const imRtcModuleSource = read('sdks/sdkwork-im-sdk/sdkwork-im-sdk-typescript/src/rtc-module.ts');
const imTransportClientLikeSource = read('sdks/sdkwork-im-sdk/sdkwork-im-sdk-typescript/src/transport-client-like.ts');
assert.match(chatServiceSource, /@sdkwork\/im-sdk/u, 'chat service must route IM behavior through @sdkwork/im-sdk types');
assert.match(chatServiceSource, /getImSdkClientWithSession/u, 'chat service must use the shared IM SDK client wrapper');
assert.match(
  chatServiceSource,
  /subscribeMessages|subscribeConversationMessages/u,
  'chat service must expose SDK-backed realtime message subscription',
);
assert.match(chatServiceSource, /\.connect\s*\(/u, 'chat service must open IM realtime through @sdkwork/im-sdk');
assert.match(
  chatServiceSource,
  /messages\.onConversation/u,
  'chat service must subscribe to conversation messages through IM live SDK',
);
assert.match(
  chatServiceSource,
  /startDirectChat/u,
  'chat service must expose a semantic SDK-backed direct chat starter for contact flows',
);
assert.match(
  chatServiceSource,
  /\.conversations\.bindDirectChat\s*\(/u,
  'chat service startDirectChat must bind direct chats through the generated IM SDK',
);
assert.match(
  chatServiceSource,
  /\.conversations\.updateProfile\s*\(\s*boundConversationId/u,
  'chat service startDirectChat must sync the direct chat display profile through the IM SDK',
);
assert.match(
  chatServiceSource,
  /\.conversations\.updatePreferences\s*\(\s*boundConversationId\s*,\s*\{\s*isHidden:\s*false\s*\}/u,
  'chat service startDirectChat must unhide the real direct chat conversation through the IM SDK',
);
assert.match(
  chatServiceSource,
  /mapReplyReferenceToMessageReply/u,
  'chat service must map standard MessageReplyReference values into the existing PC message reply shape',
);
assert.match(
  chatServiceSource,
  /buildReplyReference/u,
  'chat service must map PC reply state into the standard MessageReplyReference request shape',
);
assert.match(
  chatServiceSource,
  /entry\.body\?\.replyTo/u,
  'chat service must restore reply references from timeline MessageBody.replyTo',
);
assert.match(
  chatServiceSource,
  /decodedMessage\.replyTo/u,
  'chat service must restore reply references from realtime decoded messages',
);
assert.match(
  chatServiceSource,
  /replyTo:\s*replyReference/u,
  'chat service must send reply references through the generated IM SDK message body',
);
assert.match(
  chatServiceSource,
  /function\s+buildMediaMessageParts\s*\(/u,
  'chat service rich send path must build standard IM media content parts for PC attachments',
);
assert.match(
  chatServiceSource,
  /kind:\s*['"]media['"]/u,
  'chat service rich send path must persist attachments as kind=media ContentPart values',
);
assert.match(
  chatServiceSource,
  /const\s+driveUri\s*=\s*`drive:\/\/spaces\/\$\{spaceId\}\/nodes\/\$\{nodeId\}`/u,
  'chat service media ContentPart must include canonical Drive references for backend sync',
);
assert.match(
  chatServiceSource,
  /parts:\s*buildMediaMessageParts\s*\(\s*chatId\s*,\s*content\s*,\s*type\s*,\s*extraInfo\s*\)/u,
  'chat service rich send path must send standard media parts through the IM SDK postMessage request',
);
assert.doesNotMatch(
  chatServiceSource,
  /postMessage\s*\(\s*chatId\s*,\s*\{[\s\S]*?\n\s*text:\s*content\s*,[\s\S]*?renderHints:/u,
  'chat service media send path must not persist local object URLs as plain text bodies',
);
assert.match(
  chatServiceSource,
  /lifecycle\.onStateChange/u,
  'chat service must observe IM live lifecycle so dropped realtime connections can be resubscribed',
);
assert.match(
  chatServiceSource,
  /setTimeout\s*\(\s*\(\s*\)\s*=>\s*this\.restartLiveSubscription/u,
  'chat service must schedule a realtime resubscribe after dropped IM live connections',
);
const messageInputSource = read('apps/sdkwork-chat-pc/packages/sdkwork-clawchat-pc-chat/src/components/MessageInput.tsx');
assert.doesNotMatch(
  messageInputSource,
  /mock to work|Sending message:|console\.log\s*\(/u,
  'message input must not keep mock-send branches or console-only fake delivery paths',
);
const voiceRecorderFailureStart = messageInputSource.indexOf('} catch (err)');
assert.notEqual(voiceRecorderFailureStart, -1, 'message input voice recorder failure path must remain auditable');
const voiceRecorderFailureSource = messageInputSource.slice(voiceRecorderFailureStart, messageInputSource.indexOf('  return (', voiceRecorderFailureStart));
assert.doesNotMatch(
  voiceRecorderFailureSource,
  /模拟语音|mock voice|startTimer\s*\(\s*\)/u,
  'message input must fail closed when microphone access is unavailable instead of starting a mock voice recording',
);
const settingsServiceSource = read('apps/sdkwork-chat-pc/packages/sdkwork-clawchat-pc-chat/src/services/SettingsService.ts');
assert.match(
  settingsServiceSource,
  /getAppSdkClientWithSession/u,
  'settings service remote configuration must use the generated im-app-sdk wrapper',
);
assert.match(
  settingsServiceSource,
  /\.portal\.home\.retrieve\s*\(\s*\)/u,
  'settings service module configuration must read the app portal snapshot through the generated SDK',
);
assert.match(
  settingsServiceSource,
  /\.device\.twin\.retrieve\s*\(/u,
  'settings service device list must read the generated app SDK device twin',
);
assert.match(
  settingsServiceSource,
  /\.device\.twin\.desired\.update\s*\(/u,
  'settings service device removal must write desired device state through the generated app SDK',
);
assert.doesNotMatch(settingsServiceSource, /class\s+MockSettingsService/u, 'settings service must not be mock-backed');
assert.doesNotMatch(settingsServiceSource, /\bfetch\s*\(/u, 'settings service must not use raw fetch');
assert.doesNotMatch(settingsServiceSource, /\/api\/config\/modules/u, 'settings service must not hand-code module config paths');
assertFile('apps/sdkwork-chat-pc/packages/sdkwork-clawchat-pc-chat/src/services/DeviceSyncFeedService.ts');
const deviceSyncFeedServiceSource = read('apps/sdkwork-chat-pc/packages/sdkwork-clawchat-pc-chat/src/services/DeviceSyncFeedService.ts');
assert.match(
  deviceSyncFeedServiceSource,
  /DEVICE_SYNC_PAGE_LIMIT\s*=\s*100/u,
  'device sync feed helper must use the standard bounded page size',
);
assert.match(
  deviceSyncFeedServiceSource,
  /client\.device\.syncFeed\.retrieve\s*\(/u,
  'device sync feed helper must retrieve windows through the generated IM SDK',
);
assert.match(
  deviceSyncFeedServiceSource,
  /nextAfterSeq/u,
  'device sync feed helper must persist nextAfterSeq for incremental sync',
);
assert.doesNotMatch(deviceSyncFeedServiceSource, /\bfetch\s*\(/u, 'device sync feed helper must not use raw fetch');
assert.doesNotMatch(deviceSyncFeedServiceSource, /\/im\/v3/u, 'device sync feed helper must not hand-code IM HTTP paths');
assert.match(
  chatServiceSource,
  /DeviceSyncFeedService/u,
  'chat service must share the standard device sync feed helper instead of keeping local polling logic',
);
assert.match(chatServiceSource, /syncDeviceFeed/u, 'chat service must expose device sync feed consumption');
assert.match(chatServiceSource, /syncOfflineMessages/u, 'chat service must expose offline message window sync');
assert.match(
  chatServiceSource,
  /originEventType\s*===\s*['"]message\.posted['"]/u,
  'chat service device sync must consume offline message.posted feed entries',
);
assert.match(
  chatServiceSource,
  /function\s+mapDeviceSyncEntryToMessage[\s\S]*parseDeviceSyncPayload[\s\S]*firstBodyPart[\s\S]*resolvePartMessageType/u,
  'chat service device sync must restore message type from standard message.posted body parts',
);
assert.match(
  chatServiceSource,
  /mapRecordReplyReferenceToMessageReply\s*\(\s*toRecord\s*\(\s*body\.replyTo\s*\)\s*\)/u,
  'chat service device sync must restore reply references from standard message.posted payload',
);
assert.match(
  chatServiceSource,
  /resolvePayloadMessageContent\s*\(\s*body\s*,\s*payload\s*,\s*entry\s*,\s*messageType\s*\)/u,
  'chat service device sync must restore media/file content from standard message.posted payload',
);
assert.match(
  chatServiceSource,
  /originEventType\s*===\s*['"]conversation\.read_cursor_updated['"]/u,
  'chat service device sync must consume read cursor feed entries',
);
assert.doesNotMatch(chatServiceSource, /class MockChatService/u, 'chat service must not be mock-backed');
assert.doesNotMatch(chatServiceSource, /mockChats|mockMessages/u, 'chat service must not keep mock branches');
for (const requiredOperation of [
  'messages.reactions.create',
  'messages.reactions.delete',
  'messages.pin.create',
  'messages.pin.delete',
  'messages.visibility.delete',
  'messages.favorites.list',
  'messages.favorites.create',
  'messages.favorites.delete',
  'conversations.messages.interactionSummary.retrieve',
  'conversations.pins.list',
  'conversations.profile.retrieve',
  'conversations.profile.update',
  'conversations.preferences.retrieve',
  'conversations.preferences.update',
  'social.contacts.preferences.retrieve',
  'social.contacts.preferences.update',
  'social.contacts.tags.list',
  'social.contacts.tags.create',
  'social.contacts.tags.update',
  'social.contacts.tags.delete',
  'social.contacts.recommendations.create',
  'rtc.sessions.retrieve',
]) {
  assert.match(
    imOpenApiSource,
    new RegExp(requiredOperation.replace(/[.*+?^${}()|[\]\\]/g, '\\$&'), 'u'),
    `IM OpenAPI must expose SDKWork v3 operationId ${requiredOperation}`,
  );
}
assert.match(
  imRtcModuleSource,
  /retrieve\s*\(\s*rtcSessionId:\s*string\s*\|\s*number\s*\)\s*:\s*Promise<RtcSession>/u,
  'IM SDK RTC module must expose a semantic retrieve method for RTC session state backfill',
);
assert.match(
  imRtcModuleSource,
  /transportClient\.rtc\.sessions\.retrieve\s*\(\s*rtcSessionId\s*\)/u,
  'IM SDK rtc.retrieve must delegate to the generated rtc.sessions.retrieve transport method',
);
assert.match(
  imTransportClientLikeSource,
  /retrieve\s*\(\s*rtcSessionId:\s*string\s*\|\s*number\s*\)\s*:\s*Promise<RtcSession>/u,
  'IM SDK transport client contract must include generated rtc.sessions.retrieve',
);
for (const requiredSchema of [
  'MessageReactionRequest',
  'MessageReactionMutationResult',
  'MessagePinMutationResult',
  'MessageVisibilityMutationResult',
  'MessageFavoriteType',
  'FavoriteMessageRequest',
  'MessageFavoriteView',
  'FavoriteMessagesResponse',
  'DeleteMessageFavoriteResponse',
  'MessageInteractionSummaryView',
  'MessageReactionCountView',
  'MessagePinView',
  'PinnedMessagesResponse',
  'MessageBody',
  'MessageReplyReference',
  'MessageType',
  'TimelineViewEntry',
  'ConversationProfileView',
  'UpdateConversationProfileRequest',
  'ConversationPreferencesView',
  'UpdateConversationPreferencesRequest',
  'ContactPreferencesView',
  'UpdateContactPreferencesRequest',
  'ContactRecommendationView',
  'ContactTagView',
  'ContactTagsResponse',
  'CreateContactRecommendationRequest',
  'CreateContactTagRequest',
  'DeleteContactTagResponse',
  'UpdateContactTagRequest',
]) {
  assert.match(
    imOpenApiSource,
    new RegExp(`\\b${requiredSchema}\\b`, 'u'),
    `IM OpenAPI must define ${requiredSchema} for generated SDK typing`,
  );
}
const timelineViewEntrySchema = extractYamlSchemaBlock(imOpenApiSource, 'TimelineViewEntry');
for (const requiredField of [
  'sender',
  'body',
  'messageType',
  'deliveryMode',
  'occurredAt',
]) {
  assert.match(
    timelineViewEntrySchema,
    new RegExp(`\\b${requiredField}:`, 'u'),
    `TimelineViewEntry must expose ${requiredField} so PC history does not fall back to local mock fields`,
  );
  assert.match(
    timelineViewEntrySchema,
    new RegExp(`- ${requiredField}\\b`, 'u'),
    `TimelineViewEntry must require ${requiredField} in the IM OpenAPI contract`,
  );
}
assert.match(
  timelineViewEntrySchema,
  /sender:\s*\n\s+\$ref:\s*'#\/components\/schemas\/Sender'/u,
  'TimelineViewEntry.sender must reuse the standard Sender schema',
);
assert.match(
  timelineViewEntrySchema,
  /body:\s*\n\s+\$ref:\s*'#\/components\/schemas\/MessageBody'/u,
  'TimelineViewEntry.body must reuse the standard MessageBody schema',
);
const messageBodySchema = extractYamlSchemaBlock(imOpenApiSource, 'MessageBody');
assert.match(
  messageBodySchema,
  /replyTo:\s*\n\s+\$ref:\s*'#\/components\/schemas\/MessageReplyReference'/u,
  'MessageBody.replyTo must use the standard reply reference schema so PC reply previews survive sync',
);
const postMessageRequestSchema = extractYamlSchemaBlock(imOpenApiSource, 'PostMessageRequest');
assert.match(
  postMessageRequestSchema,
  /replyTo:\s*\n\s+\$ref:\s*'#\/components\/schemas\/MessageReplyReference'/u,
  'PostMessageRequest.replyTo must use the standard reply reference schema so PC sendMessage persists replies',
);
const editMessageRequestSchema = extractYamlSchemaBlock(imOpenApiSource, 'EditMessageRequest');
assert.match(
  editMessageRequestSchema,
  /replyTo:\s*\n\s+\$ref:\s*'#\/components\/schemas\/MessageReplyReference'/u,
  'EditMessageRequest.replyTo must use the standard reply reference schema for future reply-aware edits',
);
const messageReplyReferenceSchema = extractYamlSchemaBlock(imOpenApiSource, 'MessageReplyReference');
for (const requiredField of ['messageId', 'senderDisplayName', 'contentPreview']) {
  assert.match(
    messageReplyReferenceSchema,
    new RegExp(`\\b${requiredField}:`, 'u'),
    `MessageReplyReference must expose ${requiredField} for PC reply rendering`,
  );
  assert.match(
    messageReplyReferenceSchema,
    new RegExp(`- ${requiredField}\\b`, 'u'),
    `MessageReplyReference must require ${requiredField} for deterministic reply sync`,
  );
}
assert.match(
  timelineViewEntrySchema,
  /messageType:\s*\n\s+\$ref:\s*'#\/components\/schemas\/MessageType'/u,
  'TimelineViewEntry.messageType must reuse the standard MessageType schema',
);
assert.match(
  extractYamlSchemaBlock(imOpenApiSource, 'MessageType'),
  /enum:\s*\n\s+- standard\s*\n\s+- system\s*\n\s+- signal/u,
  'MessageType must standardize standard/system/signal values for generated SDK typing',
);
const conversationProfileViewSchema = extractYamlSchemaBlock(imOpenApiSource, 'ConversationProfileView');
for (const requiredField of [
  'tenantId',
  'conversationId',
  'displayName',
  'avatarUrl',
  'notice',
  'updatedAt',
]) {
  assert.match(
    conversationProfileViewSchema,
    new RegExp(`\\b${requiredField}:`, 'u'),
    `ConversationProfileView must expose ${requiredField} so PC group profile sync is typed`,
  );
  assert.match(
    conversationProfileViewSchema,
    new RegExp(`- ${requiredField}\\b`, 'u'),
    `ConversationProfileView must require ${requiredField} for deterministic group profile sync`,
  );
}
const updateConversationProfileRequestSchema = extractYamlSchemaBlock(
  imOpenApiSource,
  'UpdateConversationProfileRequest',
);
for (const optionalField of ['displayName', 'avatarUrl', 'notice']) {
  assert.match(
    updateConversationProfileRequestSchema,
    new RegExp(`\\b${optionalField}:`, 'u'),
    `UpdateConversationProfileRequest must expose partial ${optionalField} updates`,
  );
}
assert.doesNotMatch(
  updateConversationProfileRequestSchema,
  /\n\s+required:/u,
  'UpdateConversationProfileRequest must remain a partial update request',
);
const conversationPreferencesViewSchema = extractYamlSchemaBlock(imOpenApiSource, 'ConversationPreferencesView');
for (const requiredField of [
  'tenantId',
  'conversationId',
  'principalKind',
  'principalId',
  'isPinned',
  'isMuted',
  'isMarkedUnread',
  'isHidden',
  'updatedAt',
]) {
  assert.match(
    conversationPreferencesViewSchema,
    new RegExp(`\\b${requiredField}:`, 'u'),
    `ConversationPreferencesView must expose ${requiredField} so PC chat preference sync is typed`,
  );
  assert.match(
    conversationPreferencesViewSchema,
    new RegExp(`- ${requiredField}\\b`, 'u'),
    `ConversationPreferencesView must require ${requiredField} for deterministic per-user chat preference sync`,
  );
}
const updateConversationPreferencesRequestSchema = extractYamlSchemaBlock(
  imOpenApiSource,
  'UpdateConversationPreferencesRequest',
);
for (const optionalField of ['isPinned', 'isMuted', 'isMarkedUnread', 'isHidden']) {
  assert.match(
    updateConversationPreferencesRequestSchema,
    new RegExp(`\\b${optionalField}:`, 'u'),
    `UpdateConversationPreferencesRequest must expose partial ${optionalField} updates`,
  );
}
assert.doesNotMatch(
  updateConversationPreferencesRequestSchema,
  /\n\s+required:/u,
  'UpdateConversationPreferencesRequest must remain a partial update request',
);
const contactPreferencesViewSchema = extractYamlSchemaBlock(imOpenApiSource, 'ContactPreferencesView');
for (const requiredField of [
  'tenantId',
  'ownerUserId',
  'targetUserId',
  'isStarred',
  'remark',
  'isBlocked',
  'updatedAt',
]) {
  assert.match(
    contactPreferencesViewSchema,
    new RegExp(`\\b${requiredField}:`, 'u'),
    `ContactPreferencesView must expose ${requiredField} so PC contact preference sync is typed`,
  );
  assert.match(
    contactPreferencesViewSchema,
    new RegExp(`- ${requiredField}\\b`, 'u'),
    `ContactPreferencesView must require ${requiredField} for deterministic per-user contact preference sync`,
  );
}
const updateContactPreferencesRequestSchema = extractYamlSchemaBlock(
  imOpenApiSource,
  'UpdateContactPreferencesRequest',
);
for (const optionalField of ['isStarred', 'remark', 'isBlocked']) {
  assert.match(
    updateContactPreferencesRequestSchema,
    new RegExp(`\\b${optionalField}:`, 'u'),
    `UpdateContactPreferencesRequest must expose partial ${optionalField} updates`,
  );
}
assert.match(
  updateContactPreferencesRequestSchema,
  /remark:\s*\n\s+maxLength:\s*256/u,
  'UpdateContactPreferencesRequest.remark must be length bounded for user-controlled contact display text',
);
assert.doesNotMatch(
  updateContactPreferencesRequestSchema,
  /\n\s+required:/u,
  'UpdateContactPreferencesRequest must remain a partial update request',
);
const contactTagViewSchema = extractYamlSchemaBlock(imOpenApiSource, 'ContactTagView');
for (const requiredField of [
  'tenantId',
  'ownerUserId',
  'tagId',
  'name',
  'color',
  'count',
  'bg',
  'border',
  'createdAt',
  'updatedAt',
]) {
  assert.match(
    contactTagViewSchema,
    new RegExp(`\\b${requiredField}:`, 'u'),
    `ContactTagView must expose ${requiredField} so PC contact tag sync is typed`,
  );
  assert.match(
    contactTagViewSchema,
    new RegExp(`- ${requiredField}\\b`, 'u'),
    `ContactTagView must require ${requiredField} for deterministic per-user contact tag sync`,
  );
}
const contactTagsResponseSchema = extractYamlSchemaBlock(imOpenApiSource, 'ContactTagsResponse');
for (const requiredField of ['items', 'hasMore']) {
  assert.match(
    contactTagsResponseSchema,
    new RegExp(`\\b${requiredField}:`, 'u'),
    `ContactTagsResponse must expose ${requiredField} for cursor-bounded contact tag sync`,
  );
  assert.match(
    contactTagsResponseSchema,
    new RegExp(`- ${requiredField}\\b`, 'u'),
    `ContactTagsResponse must require ${requiredField} for deterministic cursor-bounded contact tag sync`,
  );
}
assert.match(
  contactTagsResponseSchema,
  /nextCursor:/u,
  'ContactTagsResponse must expose nextCursor so PC contact tags can page through SDK windows',
);
const createContactTagRequestSchema = extractYamlSchemaBlock(imOpenApiSource, 'CreateContactTagRequest');
for (const requiredField of ['name', 'color']) {
  assert.match(
    createContactTagRequestSchema,
    new RegExp(`\\b${requiredField}:`, 'u'),
    `CreateContactTagRequest must expose ${requiredField} for PC contact tag creation`,
  );
  assert.match(
    createContactTagRequestSchema,
    new RegExp(`- ${requiredField}\\b`, 'u'),
    `CreateContactTagRequest must require ${requiredField} for PC contact tag creation`,
  );
}
for (const optionalField of ['count', 'bg', 'border']) {
  assert.match(
    createContactTagRequestSchema,
    new RegExp(`\\b${optionalField}:`, 'u'),
    `CreateContactTagRequest must expose optional ${optionalField} for PC tag styling parity`,
  );
}
const updateContactTagRequestSchema = extractYamlSchemaBlock(imOpenApiSource, 'UpdateContactTagRequest');
for (const optionalField of ['name', 'color', 'count', 'bg', 'border']) {
  assert.match(
    updateContactTagRequestSchema,
    new RegExp(`\\b${optionalField}:`, 'u'),
    `UpdateContactTagRequest must expose partial ${optionalField} updates for PC contact tags`,
  );
}
assert.doesNotMatch(
  updateContactTagRequestSchema,
  /\n\s+required:/u,
  'UpdateContactTagRequest must remain a partial update request',
);
const contactRecommendationViewSchema = extractYamlSchemaBlock(imOpenApiSource, 'ContactRecommendationView');
for (const requiredField of [
  'tenantId',
  'ownerUserId',
  'targetUserId',
  'recommendationId',
  'createdAt',
]) {
  assert.match(
    contactRecommendationViewSchema,
    new RegExp(`\\b${requiredField}:`, 'u'),
    `ContactRecommendationView must expose ${requiredField} so PC contact recommendation sync is typed`,
  );
  assert.match(
    contactRecommendationViewSchema,
    new RegExp(`- ${requiredField}\\b`, 'u'),
    `ContactRecommendationView must require ${requiredField} for deterministic contact recommendation sync`,
  );
}
const messageVisibilityMutationResultSchema = extractYamlSchemaBlock(imOpenApiSource, 'MessageVisibilityMutationResult');
for (const requiredField of [
  'tenantId',
  'conversationId',
  'messageId',
  'messageSeq',
  'principalKind',
  'principalId',
  'isDeleted',
  'updatedAt',
]) {
  assert.match(
    messageVisibilityMutationResultSchema,
    new RegExp(`\\b${requiredField}:`, 'u'),
    `MessageVisibilityMutationResult must expose ${requiredField} so PC message delete can sync current-principal visibility`,
  );
  assert.match(
    messageVisibilityMutationResultSchema,
    new RegExp(`- ${requiredField}\\b`, 'u'),
    `MessageVisibilityMutationResult must require ${requiredField} for deterministic current-principal message delete sync`,
  );
}
const createContactRecommendationRequestSchema = extractYamlSchemaBlock(
  imOpenApiSource,
  'CreateContactRecommendationRequest',
);
assert.match(
  createContactRecommendationRequestSchema,
  /targetConversationId:/u,
  'CreateContactRecommendationRequest must expose targetConversationId for conversation-scoped contact recommendations',
);
const favoriteMessageRequestSchema = extractYamlSchemaBlock(imOpenApiSource, 'FavoriteMessageRequest');
for (const requiredField of ['conversationId', 'favoriteType', 'title', 'contentPreview', 'sourceDisplayName']) {
  assert.match(
    favoriteMessageRequestSchema,
    new RegExp(`\\b${requiredField}:`, 'u'),
    `FavoriteMessageRequest must expose ${requiredField} for PC message favorite sync`,
  );
  assert.match(
    favoriteMessageRequestSchema,
    new RegExp(`- ${requiredField}\\b`, 'u'),
    `FavoriteMessageRequest must require ${requiredField} for deterministic PC message favorite sync`,
  );
}
assert.match(
  extractYamlSchemaBlock(imOpenApiSource, 'MessageFavoriteType'),
  /enum:\s*\n\s+- link\s*\n\s+- image\s*\n\s+- file\s*\n\s+- chat/u,
  'MessageFavoriteType must standardize the PC favorite filter taxonomy',
);
const messageFavoriteViewSchema = extractYamlSchemaBlock(imOpenApiSource, 'MessageFavoriteView');
for (const requiredField of [
  'tenantId',
  'principalKind',
  'principalId',
  'favoriteId',
  'favoriteType',
  'conversationId',
  'messageId',
  'messageSeq',
  'title',
  'contentPreview',
  'sourceDisplayName',
  'favoritedAt',
]) {
  assert.match(
    messageFavoriteViewSchema,
    new RegExp(`\\b${requiredField}:`, 'u'),
    `MessageFavoriteView must expose ${requiredField} so PC favorites are principal-scoped and renderable`,
  );
  assert.match(
    messageFavoriteViewSchema,
    new RegExp(`- ${requiredField}\\b`, 'u'),
    `MessageFavoriteView must require ${requiredField} for deterministic PC favorites sync`,
  );
}
const favoriteMessagesResponseSchema = extractYamlSchemaBlock(imOpenApiSource, 'FavoriteMessagesResponse');
for (const requiredField of ['items', 'hasMore']) {
  assert.match(
    favoriteMessagesResponseSchema,
    new RegExp(`\\b${requiredField}:`, 'u'),
    `FavoriteMessagesResponse must expose ${requiredField} for bounded PC favorites sync`,
  );
  assert.match(
    favoriteMessagesResponseSchema,
    new RegExp(`- ${requiredField}\\b`, 'u'),
    `FavoriteMessagesResponse must require ${requiredField} for deterministic PC favorites sync`,
  );
}
assert.match(
  favoriteMessagesResponseSchema,
  /nextCursor:/u,
  'FavoriteMessagesResponse must expose nextCursor so PC favorites can page through SDK windows',
);
for (const sdkMethod of ['addReaction', 'removeReaction', 'pinMessage', 'unpinMessage']) {
  assert.match(
    imSdkSource,
    new RegExp(`\\b${sdkMethod}\\s*\\(`, 'u'),
    `IM SDK client must expose semantic ${sdkMethod} method`,
  );
  assert.match(
    imMessagesModuleSource,
    new RegExp(`\\b${sdkMethod}\\s*\\(`, 'u'),
    `IM messages module must implement semantic ${sdkMethod} method`,
  );
}
assert.match(
  imSdkSource,
  /\bdeleteMessageForMe\s*\(/u,
  'IM SDK client must expose semantic deleteMessageForMe method for current-principal message visibility delete',
);
assert.match(
  imMessagesModuleSource,
  /\bdeleteForMe\s*\(/u,
  'IM messages module must implement semantic deleteForMe method for current-principal message visibility delete',
);
assert.match(
  imMessagesModuleSource,
  /chat\.messages\.visibility\.delete\s*\(/u,
  'IM messages module deleteForMe must route through the generated chat.messages.visibility.delete transport resource',
);
for (const sdkMethod of ['listFavorites', 'favoriteMessage', 'deleteFavorite']) {
  assert.match(
    imMessagesModuleSource,
    new RegExp(`\\b${sdkMethod}\\s*\\(`, 'u'),
    `IM messages module must implement semantic ${sdkMethod} method for message favorites`,
  );
}
assert.match(
  imMessagesModuleSource,
  /readonly\s+favorites\s*=/u,
  'IM messages module must expose messages.favorites as the app-facing composed SDK resource',
);
assert.match(
  imMessagesModuleSource,
  /chat\.messages\.favorites\.list\s*\(/u,
  'IM messages module listFavorites must route through generated chat.messages.favorites.list',
);
assert.match(
  imMessagesModuleSource,
  /chat\.messages\.favorites\.create\s*\(/u,
  'IM messages module favoriteMessage must route through generated chat.messages.favorites.create',
);
assert.match(
  imMessagesModuleSource,
  /chat\.messages\.favorites\.delete\s*\(/u,
  'IM messages module deleteFavorite must route through generated chat.messages.favorites.delete',
);
for (const sdkMethod of ['listMessageFavorites', 'favoriteMessage', 'deleteMessageFavorite']) {
  assert.match(
    imSdkSource,
    new RegExp(`\\b${sdkMethod}\\s*\\(`, 'u'),
    `IM SDK client must expose semantic ${sdkMethod} method for message favorites`,
  );
}
for (const sdkMethod of ['getMessageInteractionSummary', 'listPinnedMessages']) {
  assert.match(
    imConversationsModuleSource,
    new RegExp(`\\b${sdkMethod}\\s*\\(`, 'u'),
    `IM conversations module must implement semantic ${sdkMethod} method`,
  );
}
for (const sdkMethod of ['getPreferences', 'updatePreferences']) {
  assert.match(
    imConversationsModuleSource,
    new RegExp(`\\b${sdkMethod}\\s*\\(`, 'u'),
    `IM conversations module must implement semantic ${sdkMethod} method for per-user chat preferences`,
  );
}
for (const sdkMethod of ['getProfile', 'updateProfile']) {
  assert.match(
    imConversationsModuleSource,
    new RegExp(`\\b${sdkMethod}\\s*\\(`, 'u'),
    `IM conversations module must implement semantic ${sdkMethod} method for shared group profile sync`,
  );
}
assert.match(
  imTransportClientLikeSource,
  /reactions:\s*\{/u,
  'IM transport client type must expose generated chat.messages.reactions resource',
);
assert.match(
  imTransportClientLikeSource,
  /pin:\s*\{/u,
  'IM transport client type must expose generated chat.messages.pin resource',
);
assert.match(
  imTransportClientLikeSource,
  /visibility:\s*\{[\s\S]*delete\(messageId: string \| number\): Promise<MessageVisibilityMutationResult>;/u,
  'IM transport client type must expose generated chat.messages.visibility delete resource',
);
assert.match(
  imTransportClientLikeSource,
  /favorites:\s*\{[\s\S]*list\(params\?: QueryParams & \{ favoriteType\?: MessageFavoriteType \}\): Promise<FavoriteMessagesResponse>;/u,
  'IM transport client type must expose generated chat.messages.favorites list resource',
);
assert.match(
  imTransportClientLikeSource,
  /create\(messageId: string \| number, body: FavoriteMessageRequest\): Promise<MessageFavoriteView>;/u,
  'IM transport client type must expose generated chat.messages.favorites create resource',
);
assert.match(
  imTransportClientLikeSource,
  /delete\(favoriteId: string \| number\): Promise<DeleteMessageFavoriteResponse>;/u,
  'IM transport client type must expose generated chat.messages.favorites delete resource',
);
assert.match(
  imTransportClientLikeSource,
  /interactionSummary:\s*\{/u,
  'IM transport client type must expose generated chat.conversations.messages.interactionSummary resource',
);
assert.match(
  imTransportClientLikeSource,
  /pins:\s*\{/u,
  'IM transport client type must expose generated chat.conversations.pins resource',
);
assert.match(
  imTransportClientLikeSource,
  /preferences:\s*\{/u,
  'IM transport client type must expose generated chat.conversations.preferences resource',
);
assert.match(
  imTransportClientLikeSource,
  /profile:\s*\{/u,
  'IM transport client type must expose generated chat.conversations.profile resource',
);
assert.match(
  imTransportClientLikeSource,
  /tags:\s*\{[\s\S]*list\(params\?: QueryParams\): Promise<ContactTagsResponse>;/u,
  'IM transport client type must expose generated social.contacts.tags list resource',
);
assert.match(
  imTransportClientLikeSource,
  /create\(body: CreateContactTagRequest\): Promise<ContactTagView>;/u,
  'IM transport client type must expose generated social.contacts.tags create resource',
);
assert.match(
  imTransportClientLikeSource,
  /update\(\s*tagId: string \| number,\s*body: UpdateContactTagRequest,?\s*\): Promise<ContactTagView>;/u,
  'IM transport client type must expose generated social.contacts.tags update resource',
);
assert.match(
  imTransportClientLikeSource,
  /delete\(tagId: string \| number\): Promise<DeleteContactTagResponse>;/u,
  'IM transport client type must expose generated social.contacts.tags delete resource',
);
assert.match(
  imTransportClientLikeSource,
  /recommendations:\s*\{[\s\S]*create\(\s*targetUserId: string \| number,\s*body: CreateContactRecommendationRequest,?\s*\): Promise<ContactRecommendationView>;/u,
  'IM transport client type must expose generated social.contacts.recommendations create resource',
);
assert.match(
  imTransportClientLikeSource,
  /users:\s*\{[\s\S]*list\(params\?:\s*\{[\s\S]*q\?: string;[\s\S]*limit\?: number;[\s\S]*cursor\?: string;[\s\S]*\}\): Promise<SocialUserSearchResponse>;/u,
  'IM transport client type must expose generated social.users.list for add-friend search',
);
assert.match(
  chatServiceSource,
  /this\.client\(\)\.addReaction\s*\(/u,
  'chat service addReaction must persist through the IM SDK before applying local projection state',
);
assert.match(
  chatServiceSource,
  /this\.client\(\)\.removeReaction\s*\(/u,
  'chat service removeReaction must persist through the IM SDK before applying local projection state',
);
assert.match(
  chatServiceSource,
  /async\s+deleteMessage\s*\([^)]*\)\s*:\s*Promise<void>\s*\{\s*await\s+this\.client\(\)\.messages\.deleteForMe\s*\(\s*messageId\s*\)/u,
  'chat service deleteMessage must hide messages for the current user through the IM SDK visibility API',
);
assert.doesNotMatch(
  chatServiceSource,
  /async\s+deleteMessage\s*\([^)]*\)\s*:\s*Promise<void>\s*\{[\s\S]*?recallMessage/u,
  'chat service deleteMessage must not recall messages when deleting only the current user copy',
);
assert.match(
  chatServiceSource,
  /MESSAGE_PAGE_LIMIT\s*=\s*100/u,
  'chat service message sync must use a bounded page size',
);
assert.match(
  chatServiceSource,
  /listAllTimelineEntries/u,
  'chat service getMessages must load all timeline pages from the IM SDK',
);
assert.match(
  chatServiceSource,
  /nextAfterSeq/u,
  'chat service timeline sync must continue through SDK afterSeq cursor windows',
);
assert.match(
  chatServiceSource,
  /resolveTimelineMessageType/u,
  'chat service must derive PC message type from the complete timeline body/render hints',
);
assert.match(
  chatServiceSource,
  /resolveTimelineMessageContent/u,
  'chat service must derive PC message content from the complete timeline body/resource projection',
);
assert.match(
  chatServiceSource,
  /entry\.sender\?\.id/u,
  'chat service historical messages must use TimelineViewEntry.sender instead of system fallback',
);
assert.match(
  chatServiceSource,
  /entry\.body/u,
  'chat service historical messages must use TimelineViewEntry.body instead of summary-only fallback',
);
assert.match(
  chatServiceSource,
  /entry\.committedAt\s*\?\?\s*entry\.occurredAt/u,
  'chat service historical messages must use backend timeline timestamps',
);
assert.match(
  chatServiceSource,
  /conversations\.getMessageInteractionSummary\s*\(/u,
  'chat service getMessages must restore historical reaction state through the IM SDK',
);
assert.match(
  chatServiceSource,
  /conversations\.getPreferences\s*\(/u,
  'chat service getChats must restore per-user pin/mute preferences through the IM SDK',
);
assert.match(
  chatServiceSource,
  /conversations\.getProfile\s*\(/u,
  'chat service getChats must restore shared group profile fields through the IM SDK',
);
assert.match(
  chatServiceSource,
  /conversations\.updateProfile\s*\(/u,
  'chat service updateChat/createChat must persist shared group profile fields through the IM SDK',
);
for (const profileField of ['displayName', 'avatarUrl', 'notice']) {
  assert.match(
    chatServiceSource,
    new RegExp(`\\b${profileField}\\b`, 'u'),
    `chat service must map PC chat profile state through ${profileField}`,
  );
}
assert.match(
  chatServiceSource,
  /conversations\.updatePreferences\s*\(\s*chatId,\s*\{\s*isPinned/u,
  'chat service pinChat must persist pin state through the IM SDK conversation preferences API',
);
assert.match(
  chatServiceSource,
  /conversations\.updatePreferences\s*\(\s*chatId,\s*\{\s*isMuted/u,
  'chat service muteChat must persist mute state through the IM SDK conversation preferences API',
);
assert.match(
  chatServiceSource,
  /markAsUnread\s*\(/u,
  'chat service must expose a semantic markAsUnread method for PC conversation unread preference sync',
);
assert.match(
  chatServiceSource,
  /conversations\.updatePreferences\s*\(\s*chatId,\s*\{\s*isMarkedUnread/u,
  'chat service markAsUnread/markAsRead must persist manual unread state through the IM SDK conversation preferences API',
);
assert.match(
  chatServiceSource,
  /isMarkedUnread/u,
  'chat service must map PC manual unread state through ConversationPreferencesView.isMarkedUnread',
);
assert.match(
  chatServiceSource,
  /preferences\.isHidden/u,
  'chat service must map hidden chat-list state through ConversationPreferencesView.isHidden',
);
assert.match(
  chatServiceSource,
  /if\s*\(\s*viewState\?\.isHidden\s*\)\s*\{\s*return undefined;\s*\}/u,
  'chat service getChats must filter hidden conversations using SDK-backed preferences',
);
assert.match(
  chatServiceSource,
  /conversations\.updatePreferences\s*\(\s*chatId,\s*\{\s*isHidden:\s*true\s*\}\s*\)/u,
  'chat service deleteChat must hide the chat list item through the IM SDK conversation preferences API',
);
assert.match(
  chatServiceSource,
  /conversations\.updatePreferences\s*\(\s*chat\.id,\s*\{\s*isHidden:\s*false\s*\}\s*\)/u,
  'chat service createChat must reopen hidden conversations through the IM SDK conversation preferences API',
);
assert.doesNotMatch(
  chatServiceSource,
  /hiddenConversationIds/u,
  'chat service must not keep hidden conversation state in local memory',
);
assert.doesNotMatch(
  chatServiceSource,
  /conversations\.leave\s*\(/u,
  'chat service deleteChat must not leave a conversation when deleting the current user chat-list item',
);

const favoriteServiceSource = read('apps/sdkwork-chat-pc/packages/sdkwork-clawchat-pc-chat/src/services/FavoriteService.ts');
assert.match(favoriteServiceSource, /@sdkwork\/im-sdk/u, 'favorite service must route message favorites through @sdkwork/im-sdk types');
assert.match(favoriteServiceSource, /getImSdkClientWithSession/u, 'favorite service must use the shared IM SDK client wrapper');
assert.match(
  favoriteServiceSource,
  /class\s+SdkworkFavoriteService/u,
  'favorite service must expose an SDK-backed service implementation',
);
assert.match(
  favoriteServiceSource,
  /FAVORITES_PAGE_LIMIT\s*=\s*100/u,
  'favorite service message favorite sync must use a bounded SDK page size',
);
assert.match(
  favoriteServiceSource,
  /this\.client\(\)\.messages\.favorites\.list\s*\(/u,
  'favorite service getFavorites must list message favorites through the composed IM SDK',
);
assert.match(
  favoriteServiceSource,
  /this\.client\(\)\.messages\.favorites\.create\s*\(/u,
  'favorite service addFavorite must create message favorites through the composed IM SDK',
);
assert.match(
  favoriteServiceSource,
  /this\.client\(\)\.messages\.favorites\.delete\s*\(/u,
  'favorite service removeFavorite must delete message favorites through the composed IM SDK',
);
assert.match(
  favoriteServiceSource,
  /mapFilterToFavoriteType/u,
  'favorite service must map PC favorite filters into the standard MessageFavoriteType taxonomy',
);
assert.match(
  favoriteServiceSource,
  /messageId.*is required|Favorite messageId is required/u,
  'favorite service must reject favorite creation without a backend message id',
);
assert.match(
  favoriteServiceSource,
  /conversationId.*is required|Favorite conversationId is required/u,
  'favorite service must reject favorite creation without a backend conversation id',
);
assert.doesNotMatch(favoriteServiceSource, /class\s+MockFavoriteService/u, 'favorite service must not be mock-backed');
assert.doesNotMatch(favoriteServiceSource, /mockFavorites|setTimeout|console\.log/u, 'favorite service must not keep mock favorite branches');
assert.doesNotMatch(favoriteServiceSource, /\bfetch\s*\(/u, 'favorite service must not use raw fetch');
assert.doesNotMatch(favoriteServiceSource, /\/im\/v3/u, 'favorite service must not hand-code IM HTTP paths');
assert.doesNotMatch(favoriteServiceSource, /\b(Authorization|Access-Token|X-API-Key)\b/u, 'favorite service must not assemble auth headers manually');

const contactServiceSource = read('apps/sdkwork-chat-pc/packages/sdkwork-clawchat-pc-chat/src/services/ContactService.ts');
const organizationDirectoryServiceSource = read('apps/sdkwork-chat-pc/packages/sdkwork-clawchat-pc-chat/src/services/OrganizationDirectoryService.ts');
assert.match(contactServiceSource, /@sdkwork\/im-sdk/u, 'contact service must route contacts and friendship behavior through @sdkwork/im-sdk types');
assert.match(contactServiceSource, /getImSdkClientWithSession/u, 'contact service must use the shared IM SDK client wrapper');
assert.match(
  organizationDirectoryServiceSource,
  /getAppbaseAppSdkClientWithSession/u,
  'organization directory service must use the generated appbase app SDK wrapper for IAM organization directory resources',
);
assert.match(
  contactServiceSource,
  /class\s+SdkworkContactService/u,
  'contact service must expose an SDK-backed service implementation',
);
assert.match(
  contactServiceSource,
  /\.chat\.contacts\.list\s*\(/u,
  'contact service getContacts must list contacts through the generated IM SDK',
);
assert.match(
  contactServiceSource,
  /\.social\.friendRequests\.create\s*\(/u,
  'contact service addFriend must submit friend requests through the generated IM SDK',
);
assert.match(
  contactServiceSource,
  /addFriendBySearchQuery\s*\([\s\S]*?this\.searchContacts\s*\(\s*normalizedQuery\s*\)[\s\S]*?this\.addFriend\s*\(\s*targetUser\.id\s*\)/u,
  'contact service direct add-by-input must resolve a real searched user before submitting a friend request',
);
assert.match(
  contactServiceSource,
  /\.social\.users\.list\s*\(/u,
  'contact service searchContacts must query users through the generated IM SDK social user search endpoint',
);
assert.match(
  contactServiceSource,
  /loadContactPeerProfiles\s*\([\s\S]*?this\.loadUserProfile\s*\(\s*userId\s*\)/u,
  'contact service contact list hydration must resolve backend contact ids through real social user search profiles',
);
assert.doesNotMatch(
  contactServiceSource,
  /createUserFromId\(normalizedQuery\)/u,
  'contact service searchContacts must not synthesize a user from the search input when backend search has no match',
);
assert.doesNotMatch(
  contactServiceSource,
  /getUserById[\s\S]*createUserFromId\(normalizedId\)/u,
  'contact service getUserById must not synthesize a user from arbitrary input when backend search has no match',
);
assert.match(
  contactServiceSource,
  /\.social\.friendRequests\.list\s*\(/u,
  'contact service getFriendRequests must list friend requests through the generated IM SDK',
);
assert.match(
  contactServiceSource,
  /FRIEND_REQUESTS_PAGE_LIMIT/u,
  'contact service friend request sync must use a bounded SDK page size',
);
assert.match(
  contactServiceSource,
  /listAllFriendRequests/u,
  'contact service getFriendRequests must page through incoming and outgoing friend requests',
);
assert.match(
  contactServiceSource,
  /nextCursor/u,
  'contact service friend request sync must continue through SDK cursor windows',
);
assert.match(
  contactServiceSource,
  /\.social\.friendRequests\.accept\s*\(/u,
  'contact service handleFriendRequest must accept friend requests through the generated IM SDK',
);
assert.match(
  contactServiceSource,
  /\.social\.friendRequests\.decline\s*\(/u,
  'contact service handleFriendRequest must decline friend requests through the generated IM SDK',
);
assert.match(
  contactServiceSource,
  /\.social\.friendships\.remove\s*\(/u,
  'contact service deleteContact must remove friendships through the generated IM SDK',
);
assert.match(
  contactServiceSource,
  /\.social\.contacts\.preferences\.retrieve\s*\(/u,
  'contact service getContacts/getStarredContacts must restore contact preferences through the generated IM SDK',
);
assert.match(
  contactServiceSource,
  /\.social\.contacts\.preferences\.update\s*\(/u,
  'contact service contact preference mutations must persist through the generated IM SDK',
);
assert.match(
  contactServiceSource,
  /CONTACT_TAGS_PAGE_LIMIT/u,
  'contact service contact tag sync must use a bounded SDK page size',
);
assert.match(
  contactServiceSource,
  /listAllContactTags/u,
  'contact service getTags must page through contact tags from the generated IM SDK',
);
assert.match(
  contactServiceSource,
  /\.social\.contacts\.tags\.list\s*\(/u,
  'contact service getTags must list contact tags through the generated IM SDK',
);
assert.match(
  contactServiceSource,
  /\.social\.contacts\.tags\.create\s*\(/u,
  'contact service addTag must create contact tags through the generated IM SDK',
);
assert.match(
  contactServiceSource,
  /\.social\.contacts\.tags\.update\s*\(/u,
  'contact service updateTag must update contact tags through the generated IM SDK',
);
assert.match(
  contactServiceSource,
  /\.social\.contacts\.tags\.delete\s*\(/u,
  'contact service removeTag must delete contact tags through the generated IM SDK',
);
assert.match(
  contactServiceSource,
  /\.social\.contacts\.recommendations\.create\s*\(/u,
  'contact service recommendToFriend must create contact recommendations through the generated IM SDK',
);
assert.match(
  contactServiceSource,
  /organizationDirectoryService/u,
  'contact service organization directory compatibility methods must delegate to the independent OrganizationDirectoryService',
);
assert.match(
  organizationDirectoryServiceSource,
  /\.iam\.departments\.list\s*\(/u,
  'organization directory service must list departments through the generated app SDK iam.departments resource',
);
assert.match(
  organizationDirectoryServiceSource,
  /\.iam\.departmentAssignments\.list\s*\(/u,
  'organization directory service must list department assignments through the generated app SDK iam.departmentAssignments resource',
);
assert.match(
  organizationDirectoryServiceSource,
  /\.iam\.organizations\.list\s*\(/u,
  'organization directory service must expose organization subjects through the generated app SDK iam.organizations resource',
);
assert.doesNotMatch(
  contactServiceSource,
  /\.portal\.home\.retrieve\s*\(\s*\)|mapPortalSnapshotToDepartments/u,
  'contact service must not back the organization directory with portal home snapshots',
);
assert.doesNotMatch(
  organizationDirectoryServiceSource,
  /\.portal\.home\.retrieve\s*\(\s*\)|mapPortalSnapshotToDepartments|iam_accounts|iamAccounts|department_members|departmentMembers/u,
  'organization directory service must not use portal home, iam_accounts, or direct department member modeling',
);
assert.match(
  organizationDirectoryServiceSource,
  /\.iam\.users\.current\.retrieve\s*\(\s*\)/u,
  'organization directory service must resolve the logged-in user through iam.users.current.retrieve',
);
assert.match(
  organizationDirectoryServiceSource,
  /\.iam\.roleBindings\.list\s*\(\s*\{[\s\S]*?principalId:[\s\S]*?scopeKind:\s*['"]organization['"][\s\S]*?scopeId:\s*resolvedOrganizationId/u,
  'organization directory service must derive organization permissions from scoped role bindings rather than direct user roles',
);
assert.match(
  organizationDirectoryServiceSource,
  /OrganizationDirectoryAdminCapability/u,
  'organization directory service must model member writes as an injected IAM admin capability boundary',
);
assert.match(
  organizationDirectoryServiceSource,
  /organizationMemberships\.create[\s\S]*departmentAssignments\.create/u,
  'organization directory service member management must create organization membership before department assignment',
);
assert.match(
  organizationDirectoryServiceSource,
  /roleBindings\.create/u,
  'organization directory service member management must use scoped role binding creation for admin-assigned roles',
);
assert.doesNotMatch(
  organizationDirectoryServiceSource,
  /\bfetch\s*\(|\/backend\/v3\/api|\/app\/v3\/api|\/iam\/(?:organizations|departments|organization_memberships|department_assignments)|\b(Authorization|Access-Token|X-API-Key)\b/u,
  'organization directory service must not hand-code IAM HTTP paths or manual auth headers',
);
assert.match(
  contactServiceSource,
  /isStarred|isBlocked|remark/u,
  'contact service must map PC star, blacklist, and remark state through ContactPreferencesView',
);
assert.match(
  contactServiceSource,
  /DeviceSyncFeedService/u,
  'contact service must share the standard device sync feed helper for contact incremental sync',
);
assert.match(
  contactServiceSource,
  /syncContactsFromDeviceFeed/u,
  'contact service must expose contact sync from the standard device feed',
);
assert.match(
  contactServiceSource,
  /originEventType\s*===\s*['"]friendship\.activated['"]/u,
  'contact service device sync must consume friendship.activated feed entries',
);
assert.match(
  contactServiceSource,
  /originEventType\s*===\s*['"]friendship\.removed['"]/u,
  'contact service device sync must consume friendship.removed feed entries',
);
assert.doesNotMatch(contactServiceSource, /class\s+MockContactService/u, 'contact service must not be mock-backed');
assert.doesNotMatch(contactServiceSource, /mockUsers|mockFriendRequests/u, 'contact service must not keep mock contacts or friend requests');
assert.doesNotMatch(
  contactServiceSource,
  /starredContactIds|contactRemarks|blacklistedContactIds|recommendedContactIds|INITIAL_TAGS|\bprivate\s+tags\b|\bthis\.tags\b/u,
  'contact service must not persist star, remark, blacklist, tag, or recommendation contact state in local memory',
);
assert.doesNotMatch(
  contactServiceSource,
  /LOCAL_DEPARTMENTS|id:\s*['"]org-root['"]|name:\s*['"]SDKWork['"][\s\S]*name:\s*['"]Contacts['"]/u,
  'contact service must not expose a fixed mock organization tree when backend directory data is unavailable',
);
assert.doesNotMatch(contactServiceSource, /\bfetch\s*\(/u, 'contact service must not use raw fetch');
assert.doesNotMatch(contactServiceSource, /\/im\/v3/u, 'contact service must not hand-code IM HTTP paths');
assert.doesNotMatch(contactServiceSource, /\b(Authorization|Access-Token|X-API-Key)\b/u, 'contact service must not assemble auth headers manually');

const addFriendModalSource = read('apps/sdkwork-chat-pc/packages/sdkwork-clawchat-pc-chat/src/components/AddFriendModal.tsx');
assert.match(
  addFriendModalSource,
  /const\s+normalizedQuery\s*=\s*searchQuery\.trim\s*\(\s*\)[\s\S]*contactService\.searchContacts\s*\(\s*normalizedQuery\s*\)/u,
  'add friend modal must search contacts through the SDK-backed contact service with a normalized target id',
);
assert.match(
  addFriendModalSource,
  /contactService\.addFriend\s*\(\s*result\.id\s*\)/u,
  'add friend modal must create friend requests through ContactService',
);
assert.match(
  addFriendModalSource,
  /buildSearchResultDescription\s*\(\s*results\[0\]\s*\)/u,
  'add friend modal must render backend profile fields for search results instead of fixed mock descriptions',
);
assert.doesNotMatch(
  addFriendModalSource,
  /id:\s*normalizedQuery/u,
  'add friend modal must not synthesize a user id from the search input when backend search has no match',
);
assert.doesNotMatch(
  addFriendModalSource,
  /api\.dicebear\.com\/7\.x\/avataaars\/svg\?seed=\$\{normalizedQuery\}/u,
  'add friend modal must not synthesize a mock avatar from the search input when backend search has no match',
);
assert.doesNotMatch(
  addFriendModalSource,
  /['"]new-friend['"]/u,
  'add friend modal must not synthesize a fake friend request target id',
);
assert.doesNotMatch(
  addFriendModalSource,
  /desc:\s*['"]ClawChat\s+用户['"]/u,
  'add friend modal must not hard-code a mock-looking user description for SDK search results',
);
assert.doesNotMatch(
  addFriendModalSource,
  /ClawChat\s+用户/u,
  'add friend modal search result fallback text must be derived from backend user data instead of a fixed mock-looking label',
);

const allContactsContainerSource = read('apps/sdkwork-chat-pc/packages/sdkwork-clawchat-pc-chat/src/components/contacts/AllContactsContainer.tsx');
assert.match(
  allContactsContainerSource,
  /contactService\.addFriendBySearchQuery\s*\(\s*qs\s*\)/u,
  'all contacts direct add entry must search real users before creating a friend request',
);
assert.doesNotMatch(
  allContactsContainerSource,
  /contactService\.addFriend\s*\(\s*qs\s*\)/u,
  'all contacts direct add entry must not submit raw input as the friend request target user id',
);
const orgContainerSource = read('apps/sdkwork-chat-pc/packages/sdkwork-clawchat-pc-chat/src/components/contacts/OrgContainer.tsx');
const contactsPageSource = read('apps/sdkwork-chat-pc/packages/sdkwork-clawchat-pc-chat/src/pages/ContactsView.tsx');
assert.match(
  orgContainerSource,
  /organizationDirectoryService\.getOrganizations\s*\(\s*\)/u,
  'organization contacts view must load organizations through the independent SDK-backed organization directory service before departments',
);
assert.match(
  orgContainerSource,
  /organizationDirectoryService\.getOrganizationTree\s*\(\s*\)/u,
  'organization contacts view must render the organization hierarchy instead of a flat organization picker',
);
assert.match(
  orgContainerSource,
  /organizationDirectoryService\.getDepartments\s*\(\s*organization\.organizationId\s*\)/u,
  'organization contacts view must load departments for the selected organization instead of tenant-wide departments',
);
assert.match(
  orgContainerSource,
  /organizationDirectoryService\.getDepartmentTree\s*\(\s*organization\.organizationId\s*\)/u,
  'organization contacts view must render the independent department hierarchy through /departments/tree',
);
assert.match(
  orgContainerSource,
  /organizationDirectoryService\.getUsersByDepartment\s*\(/u,
  'organization contacts view must read users from department assignments through the organization directory service',
);
assert.match(
  orgContainerSource,
  /organizationDirectoryService\.getCurrentUser\s*\(\s*\)/u,
  'organization contacts view must load the logged-in IAM user before evaluating organization admin actions',
);
assert.match(
  orgContainerSource,
  /organizationDirectoryService\.getOrganizationPermissions\s*\(\s*organization\.organizationId\s*\)/u,
  'organization contacts view must evaluate scoped organization permissions for the selected organization',
);
assert.doesNotMatch(
  orgContainerSource,
  /currentOrganization\?\.verificationStatus|grid\s+shrink-0\s+grid-cols-3|currentOrganization\.tenantBoundaryKind|currentOrganization\.dataBoundaryKind/u,
  'organization contacts view must not render a complex right-content top header; keep the right content focused on departments and members',
);
assert.match(
  orgContainerSource,
  /canManageMembers[\s\S]*UserPlus/u,
  'organization contacts view must expose member management controls only from the permission-aware admin state',
);
assert.match(
  orgContainerSource,
  /organizationDirectoryService\.(?:addOrganizationMember|inviteOrganizationMember)\s*\(/u,
  'organization contacts view admin flow must submit member add or invite through OrganizationDirectoryService',
);
assert.doesNotMatch(
  orgContainerSource,
  /toast\s*\(\s*['"][^'"]*(?:邀请|invite|添加|add)[^'"]*['"]\s*,\s*['"]success['"]\s*\)[\s\S]*?setUsers\s*\(/u,
  'organization contacts view must not fake member add or invite success by locally mutating the member list',
);
assert.match(
  orgContainerSource,
  /positionAssignments|roleBindings|roleCodes|departmentAssignmentId|organizationMembershipId/u,
  'organization contacts view must surface member assignment, position, and scoped role binding context',
);
assert.match(
  orgContainerSource,
  /searchQuery/u,
  'organization contacts view must support global contact search across organizations, departments, and assignment-backed members',
);
assert.doesNotMatch(
  orgContainerSource,
  /contactService\.getDepartments|contactService\.getUsersByDepartment/u,
  'organization contacts view must not route Organization/Department directory reads through ContactService',
);
assert.doesNotMatch(
  orgContainerSource,
  /toast\s*\(\s*`[^`]*\$\{selectedUser\.name\}[^`]*会话[^`]*`\s*,\s*['"]success['"]\s*\)/u,
  'organization contacts view must not show a fake successful chat navigation when no real send-message handler is connected',
);
assert.match(
  contactsPageSource,
  /<OrgContainer[\s\S]*searchQuery=\{searchQuery\}/u,
  'contacts page must pass the global search query into the organization directory view',
);
const contactDetailPaneSource = read('apps/sdkwork-chat-pc/packages/sdkwork-clawchat-pc-chat/src/components/contacts/ContactDetailPane.tsx');
assert.doesNotMatch(
  contactDetailPaneSource,
  /else\s+toast\s*\(\s*`[^`]*\$\{user\.name\}[^`]*会话[^`]*`\s*,\s*['"]success['"]\s*\)/u,
  'contact detail send-message button must not fake successful chat navigation without a real SDK-backed handler',
);
assert.doesNotMatch(
  contactDetailPaneSource,
  /else\s+toast\s*\(\s*['"][^'"]*发起(?:语音|视频)?通话[^'"]*['"]\s*,\s*['"]success['"]\s*\)/u,
  'contact detail call buttons must not fake successful RTC start without a real SDK-backed handler',
);

const createGroupModalSource = read('apps/sdkwork-chat-pc/packages/sdkwork-clawchat-pc-chat/src/components/CreateGroupModal.tsx');
assert.match(
  createGroupModalSource,
  /onCreated\?:\s*\(group:\s*Chat\)\s*=>\s*void/u,
  'create group modal must expose the backend-created group chat to the parent flow',
);
assert.match(
  createGroupModalSource,
  /const\s+group\s*=\s*await\s+groupService\.createGroup\s*\(\s*['"][^'"]*['"]\s*,\s*Array\.from\s*\(\s*selected\s*\)\s*\)/u,
  'create group modal must create group conversations through the SDK-backed group service',
);
assert.match(
  createGroupModalSource,
  /onCreated\?\.\(\s*group\s*\)/u,
  'create group modal must return the real backend-created group chat after creation',
);
assert.doesNotMatch(
  createGroupModalSource,
  /chatService\.createChat|id:\s*['"]group-/u,
  'create group modal must not locally synthesize group chat records',
);

const groupsContainerSource = read('apps/sdkwork-chat-pc/packages/sdkwork-clawchat-pc-chat/src/components/contacts/GroupsContainer.tsx');
assert.match(
  groupsContainerSource,
  /onOpenGroup\?:\s*\(group:\s*Chat\)\s*=>\s*void/u,
  'groups contacts container must expose selected backend group chats to the parent flow',
);
assert.match(
  groupsContainerSource,
  /onOpenGroup\?\.\(\s*group\s*\)/u,
  'groups contacts container must open the real group chat selected from the SDK-backed group list',
);
assert.match(
  groupsContainerSource,
  /onOpenGroup\?\.\(\s*newGroup\s*\)/u,
  'groups contacts container must open the real backend-created group chat after prompt creation',
);
assert.doesNotMatch(
  groupsContainerSource,
  /chatService\.createChat|id:\s*['"]group-/u,
  'groups contacts container must not locally synthesize group chat records',
);

const groupServiceSource = read('apps/sdkwork-chat-pc/packages/sdkwork-clawchat-pc-chat/src/services/GroupService.ts');
const createGroupSource = groupServiceSource.slice(
  groupServiceSource.indexOf('  async createGroup('),
  groupServiceSource.indexOf('  async getGroups('),
);
assert.match(groupServiceSource, /@sdkwork\/im-sdk/u, 'group service must route group conversations through @sdkwork/im-sdk types');
assert.match(groupServiceSource, /getImSdkClientWithSession/u, 'group service must use the shared IM SDK client wrapper');
assert.match(
  groupServiceSource,
  /class\s+SdkworkGroupService/u,
  'group service must expose an SDK-backed service implementation',
);
assert.match(
  groupServiceSource,
  /\.conversations\.create\s*\(/u,
  'group service createGroup must create group conversations through the IM SDK',
);
assert.match(
  groupServiceSource,
  /crypto\.randomUUID/u,
  'group service createGroup must use standard UUID client request ids instead of Date.now/Math.random mock ids',
);
assert.match(
  groupServiceSource,
  /\.conversations\.listMembers\s*\(/u,
  'group service must list conversation members through the IM SDK',
);
assert.match(
  groupServiceSource,
  /\.conversations\.addMember\s*\(/u,
  'group service addMembers must add conversation members through the IM SDK',
);
assert.match(
  groupServiceSource,
  /addMembersBySearchQuery\s*\([\s\S]*?\.social\.users\.list\s*\([\s\S]*?this\.addMembers\s*\(\s*groupId\s*,\s*resolvedMemberIds\s*\)/u,
  'group service add-by-input must resolve real users through social user search before inviting group members',
);
assert.match(
  groupServiceSource,
  /\.conversations\.updateProfile\s*\(/u,
  'group service createGroup must persist group display profile through the IM SDK',
);
assert.match(
  groupServiceSource,
  /\.conversations\.updatePreferences\s*\([\s\S]*\{\s*isHidden:\s*false\s*\}/u,
  'group service createGroup must unhide the backend-created group conversation through the IM SDK',
);
assert.match(
  groupServiceSource,
  /\.conversations\.removeMember\s*\(/u,
  'group service removeMember must remove conversation members through the IM SDK',
);
assert.match(
  groupServiceSource,
  /\.conversations\.leave\s*\(/u,
  'group service deleteGroup must leave group conversations through the IM SDK',
);
assert.match(
  groupServiceSource,
  /DeviceSyncFeedService/u,
  'group service must share the standard device sync feed helper for group member incremental sync',
);
assert.match(
  groupServiceSource,
  /syncGroupMembersFromDeviceFeed/u,
  'group service must expose group member sync from the standard device feed',
);
assert.match(
  groupServiceSource,
  /originEventType\.startsWith\s*\(\s*['"]conversation\.member_['"]\s*\)/u,
  'group service device sync must consume conversation.member_* feed entries',
);
assert.doesNotMatch(groupServiceSource, /class\s+MockGroupService/u, 'group service must not be mock-backed');
assert.doesNotMatch(groupServiceSource, /mockGroups|setTimeout|console\.log/u, 'group service must not keep mock group branches');
assert.doesNotMatch(groupServiceSource, /group-\$\{Date\.now\(\)\}-\$\{Math\.random/u, 'group service must not generate mock group ids with Date.now and Math.random');
assert.doesNotMatch(createGroupSource, /chatService\.updateChat\s*\(/u, 'group service createGroup must not mask backend group creation with local updateChat fallbacks');
assert.doesNotMatch(groupServiceSource, /\bfetch\s*\(/u, 'group service must not use raw fetch');
assert.doesNotMatch(groupServiceSource, /\/im\/v3/u, 'group service must not hand-code IM HTTP paths');
assert.doesNotMatch(groupServiceSource, /\b(Authorization|Access-Token|X-API-Key)\b/u, 'group service must not assemble auth headers manually');

const newFriendsContainerSource = read('apps/sdkwork-chat-pc/packages/sdkwork-clawchat-pc-chat/src/components/contacts/NewFriendsContainer.tsx');
assert.match(
  newFriendsContainerSource,
  /Avatar\s+src=\{req\.avatar\}/u,
  'new friends list must render avatars resolved from real friend request peer profiles',
);
assert.doesNotMatch(
  newFriendsContainerSource,
  /picsum\.photos\/seed\/\$\{req\.name\}/u,
  'new friends list must not synthesize friend request avatars from picsum',
);

const agentServiceSource = read('apps/sdkwork-chat-pc/packages/sdkwork-clawchat-pc-chat/src/services/AgentService.ts');
assert.match(
  agentServiceSource,
  /getAgentAppSdkClientWithSession/u,
  'agent service catalog, lifecycle, and runtime operations must use the shared sdkwork-agent-app-sdk client wrapper',
);
assert.match(
  agentServiceSource,
  /@sdkwork\/agent-app-sdk/u,
  'agent service must type its standardized agent DTO mapping from sdkwork-agent-app-sdk',
);
assert.match(
  agentServiceSource,
  /class\s+SdkworkAgentService/u,
  'agent service must expose an SDK-backed implementation instead of a mock implementation',
);
assert.match(
  agentServiceSource,
  /\.ai\.agents\.list\s*\(/u,
  'agent service discovery lists must read backend agent records through sdkwork-agent-app-sdk',
);
assert.match(
  agentServiceSource,
  /\.ai\.agents\.create\s*\(/u,
  'agent create flow must persist through sdkwork-agent-app-sdk',
);
assert.match(
  agentServiceSource,
  /\.ai\.agents\.update\s*\(/u,
  'agent update flow must persist through sdkwork-agent-app-sdk',
);
assert.match(
  agentServiceSource,
  /\.ai\.agents\.providerBindings\.create\s*\(/u,
  'agent publish flow must prepare a provider binding through sdkwork-agent-app-sdk before deployment',
);
assert.match(
  agentServiceSource,
  /\.ai\.agents\.deployments\.create\s*\(/u,
  'agent publish flow must create a deployment snapshot through sdkwork-agent-app-sdk',
);
assert.match(
  agentServiceSource,
  /\.ai\.agents\.delete\s*\(/u,
  'agent delete flow must persist through sdkwork-agent-app-sdk',
);
assert.match(
  agentServiceSource,
  /\.ai\.agents\.previewResponses\.create\s*\(/u,
  'agent preview flow must execute through sdkwork-agent-app-sdk runtime preview responses',
);
assert.match(
  agentServiceSource,
  /\.ai\.agents\.promptOptimizations\.create\s*\(/u,
  'agent prompt optimizer must execute through sdkwork-agent-app-sdk runtime prompt optimizations',
);
assert.doesNotMatch(agentServiceSource, /class\s+MockAgentService/u, 'agent service must not be mock-backed');
assert.doesNotMatch(agentServiceSource, /mockAgents|mockMarketAgents/u, 'agent service must not keep mock agent catalogs');
assert.doesNotMatch(
  agentServiceSource,
  /getAppSdkClientWithSession|SdkworkImAppClient|\.automation\.executions\.create|pc\.agent\.(?:preview_response|prompt_optimize)/u,
  'agent service must not bypass sdkwork-agent-app-sdk runtime operations through im-app-sdk automation execution',
);
assert.doesNotMatch(agentServiceSource, /setTimeout|console\.log|Date\.now/u, 'agent service must not simulate async agent catalog behavior locally');
assert.doesNotMatch(agentServiceSource, /\bfetch\s*\(/u, 'agent service must not use raw fetch');
assert.doesNotMatch(agentServiceSource, /\/(?:im|app|backend)\/v3/u, 'agent service must not hand-code SDK-owned API paths');
assert.doesNotMatch(agentServiceSource, /\b(Authorization|Access-Token|X-API-Key)\b/u, 'agent service must not assemble auth headers manually');

const createAgentViewSource = read('apps/sdkwork-chat-pc/packages/sdkwork-clawchat-pc-chat/src/pages/CreateAgentView.tsx');
assert.match(
  createAgentViewSource,
  /agentService\.requestPreviewResponse\s*\(/u,
  'create agent preview chat must request real backend agent runtime execution through AgentService',
);
assert.match(
  createAgentViewSource,
  /ensurePersistedAgentForRuntime\s*\(/u,
  'create agent runtime preview and prompt optimizer must persist a standard backend agent before execution',
);
assert.match(
  createAgentViewSource,
  /config:\s*await\s+ensurePersistedAgentForRuntime\s*\(/u,
  'create agent runtime requests must not send local draft ids to backend agent runtime endpoints',
);
assert.match(
  createAgentViewSource,
  /agentService\.optimizePrompt\s*\(/u,
  'create agent prompt optimizer must request real backend agent runtime execution through AgentService',
);
assert.match(
  createAgentViewSource,
  /knowledgeBaseIds:\s*selectedKnowledgeIds/u,
  'create agent save and publish flows must persist selected knowledge base bindings through AgentService',
);
assert.match(
  createAgentViewSource,
  /welcomeMessage/u,
  'create agent save and publish flows must persist the configured welcome message through AgentService',
);
assert.doesNotMatch(
  createAgentViewSource,
  /id:\s*draftId\s*\?\?\s*initialAgentId\s*\?\?\s*createDraftAgentId\s*\(/u,
  'create agent runtime configs must not synthesize non-standard draft-* ids for backend execution',
);
assert.doesNotMatch(
  createAgentViewSource,
  /Mock response|杩欐槸鍩轰簬褰撳墠閰嶇疆鐨勬祴璇曞洖澶嶃€俓n|setTimeout\s*\(\s*\(\s*\)\s*=>\s*\{[\s\S]*setTestMessages|setTimeout\s*\(\s*\(\s*\)\s*=>\s*\{[\s\S]*setPrompt/u,
  'create agent view must not synthesize local preview replies or prompt optimization results',
);

assertFile('apps/sdkwork-chat-pc/packages/sdkwork-clawchat-pc-chat/src/services/ImSyncCoordinatorService.ts');
const imSyncCoordinatorServiceSource = read('apps/sdkwork-chat-pc/packages/sdkwork-clawchat-pc-chat/src/services/ImSyncCoordinatorService.ts');
assert.match(
  imSyncCoordinatorServiceSource,
  /class\s+SdkworkImSyncCoordinatorService/u,
  'IM sync coordinator must expose a real SDK-backed startup synchronization service',
);
assert.match(
  imSyncCoordinatorServiceSource,
  /syncOfflineMessages\s*\(\s*deviceId\s*\)/u,
  'IM sync coordinator startup sync must run offline message window synchronization',
);
assert.match(
  imSyncCoordinatorServiceSource,
  /syncContactsFromDeviceFeed\s*\(\s*deviceId\s*\)/u,
  'IM sync coordinator startup sync must run friend/contact incremental synchronization',
);
assert.match(
  imSyncCoordinatorServiceSource,
  /syncGroupMembersFromDeviceFeed\s*\(\s*deviceId\s*\)/u,
  'IM sync coordinator startup sync must run group member incremental synchronization',
);
assert.match(
  imSyncCoordinatorServiceSource,
  /retrieveDeviceSyncFeedWindow\s*\(/u,
  'IM sync coordinator must inspect the standard device sync feed for RTC backfill hints',
);
assert.match(
  imSyncCoordinatorServiceSource,
  /recoverRtcSession\s*\(\s*hint\.rtcSessionId/u,
  'IM sync coordinator must recover RTC sessions from offline device sync signal entries',
);
assert.doesNotMatch(imSyncCoordinatorServiceSource, /\bfetch\s*\(/u, 'IM sync coordinator must not use raw fetch');
assert.doesNotMatch(imSyncCoordinatorServiceSource, /\/im\/v3/u, 'IM sync coordinator must not hand-code IM HTTP paths');
assert.doesNotMatch(imSyncCoordinatorServiceSource, /\b(Authorization|Access-Token|X-API-Key)\b/u, 'IM sync coordinator must not assemble auth headers manually');

const chatPackageIndexSource = read('apps/sdkwork-chat-pc/packages/sdkwork-clawchat-pc-chat/src/index.ts');
assert.match(
  chatPackageIndexSource,
  /export\s+\{\s*imSyncCoordinatorService\s*\}\s+from\s+["']\.\/services\/ImSyncCoordinatorService["']/u,
  'chat package must export the SDK-backed IM sync coordinator as a first-class service',
);
assert.match(
  chatPackageIndexSource,
  /export\s+type\s+\{[^}]*ImStartupSyncResult[^}]*ImSyncCoordinatorService[^}]*\}\s+from\s+["']\.\/services\/ImSyncCoordinatorService["']/u,
  'chat package must export IM startup sync coordinator types for package consumers',
);

const chatRightPanelSource = read('apps/sdkwork-chat-pc/packages/sdkwork-clawchat-pc-chat/src/components/ChatRightPanel.tsx');
const chatLayoutSource = read('apps/sdkwork-chat-pc/packages/sdkwork-clawchat-pc-chat/src/pages/ChatLayout.tsx');
const addMemberModalStart = chatLayoutSource.indexOf('{activeModal === "addMember" && (');
assert.notEqual(addMemberModalStart, -1, 'chat layout must keep the add-member modal behavior auditable');
const addMemberModalEnd = chatLayoutSource.indexOf('{activeModal === "editName"', addMemberModalStart);
assert.notEqual(addMemberModalEnd, -1, 'chat layout add-member modal block must end before edit-name modal');
const addMemberModalSource = chatLayoutSource.slice(addMemberModalStart, addMemberModalEnd);
const editNameModalStart = chatLayoutSource.indexOf('{activeModal === "editName" && (');
assert.notEqual(editNameModalStart, -1, 'chat layout must keep edit-name modal behavior auditable');
const editNameModalEnd = chatLayoutSource.indexOf('{activeModal === "editNotice"', editNameModalStart);
assert.notEqual(editNameModalEnd, -1, 'chat layout edit-name modal block must end before edit-notice modal');
const editNameModalSource = chatLayoutSource.slice(editNameModalStart, editNameModalEnd);
const editNoticeModalStart = chatLayoutSource.indexOf('{activeModal === "editNotice" && (');
assert.notEqual(editNoticeModalStart, -1, 'chat layout must keep edit-notice modal behavior auditable');
const editNoticeModalEnd = chatLayoutSource.indexOf('</motion.div>', editNoticeModalStart);
assert.notEqual(editNoticeModalEnd, -1, 'chat layout edit-notice modal block must end before inline modal closes');
const editNoticeModalSource = chatLayoutSource.slice(editNoticeModalStart, editNoticeModalEnd);
const contactsViewStart = chatLayoutSource.indexOf('case "contacts":');
assert.notEqual(contactsViewStart, -1, 'chat layout must keep the contacts view behavior auditable');
const contactsViewEnd = chatLayoutSource.indexOf('case "favorites":', contactsViewStart);
assert.notEqual(contactsViewEnd, -1, 'chat layout contacts branch must end before favorites branch');
const contactsViewSource = chatLayoutSource.slice(contactsViewStart, contactsViewEnd);
const contactsSendMessageStart = contactsViewSource.indexOf('onSendMessage={async');
assert.notEqual(contactsSendMessageStart, -1, 'chat layout contacts branch must keep send-message behavior auditable');
const contactsSendMessageEnd = contactsViewSource.indexOf('onStartCall=', contactsSendMessageStart);
assert.notEqual(contactsSendMessageEnd, -1, 'chat layout contacts send-message block must end before start-call block');
const contactsSendMessageSource = contactsViewSource.slice(contactsSendMessageStart, contactsSendMessageEnd);
const agentStartHandlerStart = chatLayoutSource.indexOf('const handleStartAgentChat = async');
assert.notEqual(agentStartHandlerStart, -1, 'chat layout must keep the agent start-chat handler auditable');
const agentStartHandlerEnd = chatLayoutSource.indexOf('  const renderHeaderContent', agentStartHandlerStart);
assert.notEqual(agentStartHandlerEnd, -1, 'chat layout agent start-chat handler must end before renderHeaderContent');
const agentStartHandlerSource = chatLayoutSource.slice(agentStartHandlerStart, agentStartHandlerEnd);
const enterpriseViewStart = chatLayoutSource.indexOf('case "enterprise":');
assert.notEqual(enterpriseViewStart, -1, 'chat layout must keep the enterprise view behavior auditable');
const enterpriseViewEnd = chatLayoutSource.indexOf('case "devices":', enterpriseViewStart);
assert.notEqual(enterpriseViewEnd, -1, 'chat layout enterprise branch must end before devices branch');
const enterpriseViewSource = chatLayoutSource.slice(enterpriseViewStart, enterpriseViewEnd);
assert.match(
  chatLayoutSource,
  /import\s+\{\s*groupService\s*\}\s+from\s+["']\.\.\/services\/GroupService["']/u,
  'chat layout group member invite flow must import the SDK-backed group service',
);
assert.match(
  chatLayoutSource,
  /import\s+\{\s*imSyncCoordinatorService\s*\}\s+from\s+["']\.\.\/services\/ImSyncCoordinatorService["']/u,
  'chat layout startup lifecycle must import the SDK-backed IM sync coordinator',
);
assert.match(
  chatLayoutSource,
  /imSyncCoordinatorService\.syncStartup\s*\(\s*\)/u,
  'chat layout startup lifecycle must run offline messages, contacts, groups, and RTC recovery before initial chat hydration',
);
assert.match(
  chatLayoutSource,
  /<CreateGroupModal[\s\S]*?onCreated=\{async\s*\(group\)\s*=>\s*\{[\s\S]*?chatService\.getChats\s*\(\s*\)[\s\S]*?setActiveChat\s*\([\s\S]*?group[\s\S]*?setActiveTab\s*\(\s*["']chat["']\s*\)/u,
  'chat layout create-group flow must hydrate backend conversations and open the real group chat after creation',
);
assert.match(
  contactsSendMessageSource,
  /chatService\.startDirectChat\s*\(\s*user\s*\)/u,
  'chat layout contacts send-message flow must start a real SDK-backed direct chat',
);
assert.match(
  contactsSendMessageSource,
  /chatService\.getChats\s*\(\s*\)/u,
  'chat layout contacts send-message flow must hydrate from backend conversations after direct chat binding',
);
assert.match(
  contactsViewSource,
  /onOpenGroup=\{async\s*\(group\)\s*=>\s*\{[\s\S]*?chatService\.getChats\s*\(\s*\)[\s\S]*?setActiveChat\s*\([\s\S]*?group[\s\S]*?setActiveTab\s*\(\s*["']chat["']\s*\)/u,
  'chat layout contacts group flow must hydrate backend conversations and open the selected real group chat',
);
assert.doesNotMatch(
  contactsSendMessageSource,
  /id:\s*user\.id/u,
  'chat layout contacts send-message flow must not synthesize a local chat id from the contact user id',
);
assert.doesNotMatch(
  contactsSendMessageSource,
  /lastMessage:\s*\{/u,
  'chat layout contacts send-message flow must not synthesize a local empty lastMessage for a direct chat',
);
assert.match(
  chatServiceSource,
  /startAgentChat\s*\([\s\S]*?\.conversations\.createAgentDialog\s*\(/u,
  'chat service must start agent chats through the SDK-backed agent dialog API',
);
assert.match(
  agentStartHandlerSource,
  /chatService\.startAgentChat\s*\(\s*agent\s*\)/u,
  'chat layout agent start flow must create a real SDK-backed agent dialog',
);
assert.doesNotMatch(
  agentStartHandlerSource,
  /const\s+chatId\s*=\s*`agent-\$\{agent\.id\}`/u,
  'chat layout agent start flow must not synthesize a local agent chat id',
);
assert.doesNotMatch(
  agentStartHandlerSource,
  /senderId:\s*["']bot["'][\s\S]*type:\s*["']text["']/u,
  'chat layout agent start flow must not synthesize a local bot welcome message',
);
assert.match(
  chatServiceSource,
  /startEnterpriseChat\s*\([\s\S]*?\.conversations\.bindDirectChat\s*\(/u,
  'chat service must start enterprise chats through the SDK-backed direct chat binding API',
);
assert.match(
  chatServiceSource,
  /rightActorKind:\s*['"]enterprise['"]/u,
  'chat service enterprise chat binding must model the target as an enterprise principal',
);
assert.match(
  enterpriseViewSource,
  /chatService\.startEnterpriseChat\s*\(\s*\{\s*id:\s*enterpriseId,\s*name:\s*enterpriseName,\s*\}\s*\)/u,
  'chat layout enterprise start flow must create a real SDK-backed enterprise conversation',
);
assert.match(
  enterpriseViewSource,
  /chatService\.getChats\s*\(\s*\)/u,
  'chat layout enterprise start flow must hydrate from backend conversations after enterprise binding',
);
assert.doesNotMatch(
  enterpriseViewSource,
  /const\s+chatId\s*=\s*`ent-\$\{enterpriseId\}`/u,
  'chat layout enterprise start flow must not synthesize a local enterprise chat id',
);
assert.doesNotMatch(
  enterpriseViewSource,
  /lastMessage:\s*\{/u,
  'chat layout enterprise start flow must not synthesize a local enterprise welcome message',
);
assert.doesNotMatch(
  enterpriseViewSource,
  /senderId:\s*["']bot["']/u,
  'chat layout enterprise start flow must not synthesize bot-authored enterprise messages',
);
assert.doesNotMatch(
  enterpriseViewSource,
  /members:\s*\[\s*["']currentUser["']\s*,\s*["']ent["']\s*\]/u,
  'chat layout enterprise start flow must not synthesize local enterprise members',
);
const enterpriseServiceSource = read('apps/sdkwork-chat-pc/packages/sdkwork-clawchat-pc-enterprise/src/services/EnterpriseService.ts');
const enterpriseListSource = read('apps/sdkwork-chat-pc/packages/sdkwork-clawchat-pc-enterprise/src/components/EnterpriseList.tsx');
assert.match(
  enterpriseServiceSource,
  /getAppSdkClientWithSession/u,
  'enterprise service must use the shared generated im-app-sdk client wrapper',
);
assert.match(
  enterpriseServiceSource,
  /class\s+SdkworkEnterpriseService/u,
  'enterprise service must expose an SDK-backed enterprise catalog implementation',
);
assert.match(
  enterpriseServiceSource,
  /\.portal\.home\.retrieve\s*\(\s*\)/u,
  'enterprise service must load the backend portal home snapshot through the generated im-app-sdk',
);
assert.match(
  enterpriseServiceSource,
  /\.portal\.workspace\.retrieve\s*\(\s*\)/u,
  'enterprise service must fall back to the real workspace snapshot instead of hard-coded enterprises',
);
assert.match(
  enterpriseListSource,
  /enterpriseService\.getEnterprises\s*\(\s*\)/u,
  'enterprise list must render enterprise principals from the SDK-backed enterprise service',
);
assert.doesNotMatch(enterpriseListSource, /mockEnterprises/u, 'enterprise list must not render a local mock enterprise catalog');
assert.doesNotMatch(enterpriseServiceSource, /mockEnterprises|dicebear/u, 'enterprise service must not keep mock enterprise catalog data');
assert.doesNotMatch(enterpriseServiceSource, /\bfetch\s*\(/u, 'enterprise service must not use raw fetch');
assert.doesNotMatch(enterpriseServiceSource, /\/(?:im|app|backend)\/v3/u, 'enterprise service must not hand-code SDK-owned API paths');
assert.doesNotMatch(enterpriseServiceSource, /\b(Authorization|Access-Token|X-API-Key)\b/u, 'enterprise service must not assemble auth headers manually');
assert.match(
  addMemberModalSource,
  /groupService\.addMembersBySearchQuery\s*\(\s*activeChat\.id\s*,\s*addedIds\s*\)/u,
  'chat layout add-member modal must resolve input through GroupService user search before inviting members',
);
assert.match(
  addMemberModalSource,
  /groupService\.getGroups\s*\(\s*\)/u,
  'chat layout add-member modal must refresh member projection from the SDK-backed group service after invites',
);
assert.match(
  editNameModalSource,
  /onClick=\{async\s*\(\)\s*=>\s*\{[\s\S]*?await\s+chatService\.updateChat\s*\(\s*activeChat\.id,\s*\{[\s\S]*?name:\s*modalInput[\s\S]*?\}/u,
  'chat layout edit-name flow must await the SDK-backed chat update before mutating local UI state',
);
assert.match(
  editNoticeModalSource,
  /onClick=\{async\s*\(\)\s*=>\s*\{[\s\S]*?await\s+chatService\.updateChat\s*\(\s*activeChat\.id,\s*\{[\s\S]*?notice:\s*modalInput[\s\S]*?\}/u,
  'chat layout edit-notice flow must await the SDK-backed chat update before mutating local UI state',
);
assert.doesNotMatch(
  addMemberModalSource,
  /groupService\.addMembers\s*\(\s*activeChat\.id\s*,\s*addedIds\s*\)/u,
  'chat layout add-member modal must not submit raw input as group member ids',
);
assert.doesNotMatch(
  addMemberModalSource,
  /new\s+Set\s*\(\s*\[\s*\.\.\.\(\s*activeChat\.members\s*\|\|\s*\[\]\s*\)\s*,\s*\.\.\.addedIds\s*\]\s*\)/u,
  'chat layout add-member modal must not locally synthesize group members after SDK invites',
);
assert.doesNotMatch(
  addMemberModalSource,
  /memberCount\s*:\s*updatedMembers\.length/u,
  'chat layout add-member modal must not locally synthesize group member counts after SDK invites',
);
assert.doesNotMatch(
  addMemberModalSource,
  /chatService\.updateChat\s*\(\s*activeChat\.id\s*,\s*\{[\s\S]*?\bmembers\s*:/u,
  'chat layout add-member modal must not mutate group members through local chat view-state updates',
);
assert.doesNotMatch(
  chatRightPanelSource,
  /chatService\.(muteChat|pinChat|deleteChat)\s*\(/u,
  'chat right panel must delegate chat preference mutations to the parent so SDK success controls local state',
);
assert.match(
  chatRightPanelSource,
  /onToggleMute:\s*\(\)\s*=>\s*Promise<void>/u,
  'chat right panel must expose an async mute command controlled by the parent flow',
);
assert.match(
  chatRightPanelSource,
  /onTogglePin:\s*\(\)\s*=>\s*Promise<void>/u,
  'chat right panel must expose an async pin command controlled by the parent flow',
);
assert.match(
  chatRightPanelSource,
  /onDeleteChat:\s*\(\)\s*=>\s*Promise<void>/u,
  'chat right panel must expose an async delete command controlled by the parent flow',
);
assert.match(
  chatLayoutSource,
  /onToggleMute=\{async\s*\(\)\s*=>\s*\{[\s\S]*?await\s+chatService\.muteChat\s*\(/u,
  'chat layout right-panel mute flow must await the SDK mutation before local state changes',
);
assert.match(
  chatLayoutSource,
  /onTogglePin=\{async\s*\(\)\s*=>\s*\{[\s\S]*?await\s+chatService\.pinChat\s*\(/u,
  'chat layout right-panel pin flow must await the SDK mutation before local state changes',
);
assert.match(
  chatLayoutSource,
  /onDeleteChat=\{async\s*\(\)\s*=>\s*\{[\s\S]*?await\s+chatService\.deleteChat\s*\(/u,
  'chat layout right-panel delete flow must await the SDK mutation before local state changes',
);

const messageListSource = read('apps/sdkwork-chat-pc/packages/sdkwork-clawchat-pc-chat/src/components/MessageList.tsx');
assert.match(
  messageListSource,
  /chatService\.subscribeMessages|chatService\.subscribeConversationMessages/u,
  'message list must subscribe to SDK realtime updates',
);
assert.match(
  messageListSource,
  /chatService\.deleteMessage\s*\(\s*chatId\s*,\s*messageId\s*\)/u,
  'message list delete action must persist current-user delete through chatService',
);
assert.match(
  messageListSource,
  /favoriteService\.addFavorite\s*\(\s*\{[\s\S]*conversationId:\s*contextMenu\.msg\.chatId\s*\?\?\s*chatId/u,
  'message list favorite action must pass the backend conversation id into FavoriteService',
);
assert.match(
  messageListSource,
  /favoriteService\.addFavorite\s*\(\s*\{[\s\S]*messageId:\s*contextMenu\.msg\.id/u,
  'message list favorite action must pass the backend message id into FavoriteService',
);

const chatListSource = read('apps/sdkwork-chat-pc/packages/sdkwork-clawchat-pc-chat/src/components/ChatList.tsx');
assert.match(
  chatListSource,
  /chat\.isMarkedUnread/u,
  'chat list must render manual unread state from the SDK-backed conversation preferences view',
);
assert.match(
  chatListSource,
  /chatService\.markAsUnread\s*\(\s*chat\.id\s*\)/u,
  'chat list mark-unread action must persist through the chat service semantic SDK-backed method',
);
assert.doesNotMatch(
  chatListSource,
  /updateChat\s*\(\s*chat\.id\s*,\s*\{\s*unreadCount\s*:/u,
  'chat list must not simulate manual unread state through local unreadCount updates',
);
assert.doesNotMatch(
  chatListSource,
  /_forceUnread/u,
  'chat list must not keep private _forceUnread state once manual unread is SDK-backed',
);

const chatWindowSource = read('apps/sdkwork-chat-pc/packages/sdkwork-clawchat-pc-chat/src/components/ChatWindow.tsx');
assert.doesNotMatch(
  chatWindowSource,
  /fakeAgentResponses|Mock a streaming response feel|setTimeout\s*\(\s*async\s*\(\s*\)\s*=>/u,
  'chat window must not simulate received messages locally',
);

assertFile('apps/sdkwork-chat-pc/packages/sdkwork-clawchat-pc-chat/src/services/CallService.ts');
const callServiceSource = read('apps/sdkwork-chat-pc/packages/sdkwork-clawchat-pc-chat/src/services/CallService.ts');
assert.match(callServiceSource, /@sdkwork\/rtc-sdk/u, 'call service must use the standard RTC SDK call stack');
assert.match(
  callServiceSource,
  /createStandardRtcCallControllerStack/u,
  'call service must compose calls through the standard RTC call controller stack',
);
assert.match(
  callServiceSource,
  /getImSdkClientWithSession/u,
  'call service must reuse the shared IM SDK client created from the IAM login session',
);
assert.match(callServiceSource, /startOutgoingCall/u, 'call service must expose outgoing call orchestration');
assert.match(callServiceSource, /recoverRtcSession/u, 'call service must expose RTC session recovery for backend state backfill');
assert.match(
  callServiceSource,
  /\.rtc\.retrieve\s*\(\s*rtcSessionId\s*\)/u,
  'call service recoverRtcSession must read RTC session state through the composed IM SDK',
);
assert.match(
  callServiceSource,
  /toRecoveredServiceState/u,
  'call service must map backend RTC session state into PC call snapshot state',
);
assert.match(
  callServiceSource,
  /providerSessionId\s*\?\?\s*rtcSession\.rtcSessionId/u,
  'call service must recover provider room id from backend RTC session metadata',
);
assert.match(
  callServiceSource,
  /watchConversationIds:\s*\[rtcSession\.conversationId\]/u,
  'call service must watch the recovered RTC conversation for subsequent RTC state updates',
);
assert.match(callServiceSource, /setAudioMuted/u, 'call service must expose audio mute through the RTC media client');
assert.match(callServiceSource, /setVideoMuted/u, 'call service must expose video mute through the RTC media client');
assert.match(callServiceSource, /endCall/u, 'call service must expose SDK-backed call termination');
assert.doesNotMatch(callServiceSource, /\bfetch\s*\(/u, 'call service must not use raw fetch');
assert.doesNotMatch(callServiceSource, /\/im\/v3/u, 'call service must not hand-code IM HTTP paths');
assert.doesNotMatch(callServiceSource, /\b(Authorization|Access-Token|X-API-Key)\b/u, 'call service must not assemble auth headers manually');

const callOverlaySource = read('apps/sdkwork-chat-pc/packages/sdkwork-clawchat-pc-chat/src/components/CallOverlay.tsx');
assert.match(callOverlaySource, /callService/u, 'call overlay must delegate RTC behavior to the call service');
assert.match(callOverlaySource, /startOutgoingCall/u, 'call overlay must start calls through the SDK-backed call service');
assert.match(callOverlaySource, /setAudioMuted/u, 'call overlay must mute audio through the SDK-backed call service');
assert.match(callOverlaySource, /setVideoMuted/u, 'call overlay must mute video through the SDK-backed call service');
assert.match(callOverlaySource, /endCall/u, 'call overlay must end calls through the SDK-backed call service');
assert.doesNotMatch(
  callOverlaySource,
  /setTimeout\s*\(\s*\(\s*\)\s*=>\s*\{?\s*setCallState\s*\(\s*['"]connected['"]/u,
  'call overlay must not simulate successful RTC connection with a timeout',
);
assert.match(
  chatLayoutSource,
  /conversationId=\{callTarget\.id\}/u,
  'chat layout must pass the active conversation id into CallOverlay for RTC session creation',
);

const serviceFiles = listFiles('apps/sdkwork-chat-pc/packages', (candidate) => {
  const normalized = candidate.replaceAll('\\', '/');
  return normalized.includes('/src/services/') && /\.(ts|tsx)$/.test(normalized);
});
for (const filePath of serviceFiles) {
  const source = fs.readFileSync(filePath, 'utf8');
  assert.doesNotMatch(source, /\/im\/v3/u, `${path.relative(repoRoot, filePath)} must not hand-code IM HTTP paths`);
  assert.doesNotMatch(source, /\b(Authorization|Access-Token|X-API-Key)\b/u, `${path.relative(repoRoot, filePath)} must not assemble auth headers manually`);
}

console.log('sdkwork-chat-pc SDK integration contract passed');
