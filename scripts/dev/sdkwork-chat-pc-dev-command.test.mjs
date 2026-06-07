import assert from 'node:assert/strict';
import { EventEmitter } from 'node:events';
import fs from 'node:fs';
import { createRequire } from 'node:module';
import path from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const repoRoot = path.resolve(__dirname, '..', '..');

function readJson(relativePath) {
  return JSON.parse(fs.readFileSync(path.join(repoRoot, relativePath), 'utf8'));
}

const packageJson = readJson('package.json');
const chatPcPackageJson = readJson('apps/sdkwork-chat-pc/package.json');
const desktopWorkspaceSource = fs.readFileSync(
  path.join(repoRoot, 'apps/sdkwork-chat-pc/pnpm-workspace.yaml'),
  'utf8',
);
const desktopNpmrcSource = fs.existsSync(path.join(repoRoot, 'apps/sdkwork-chat-pc/.npmrc'))
  ? fs.readFileSync(path.join(repoRoot, 'apps/sdkwork-chat-pc/.npmrc'), 'utf8')
  : '';
const desktopLockfileSource = fs.readFileSync(
  path.join(repoRoot, 'apps/sdkwork-chat-pc/pnpm-lock.yaml'),
  'utf8',
);
const unifiedWebSource = fs.readFileSync(
  path.join(repoRoot, 'scripts/dev/start-craw-chat-unified-web.mjs'),
  'utf8',
);
const localMinimalNodeCargoSource = fs.readFileSync(
  path.join(repoRoot, 'services/local-minimal-node/Cargo.toml'),
  'utf8',
);
const imDomainCoreCargoSource = fs.readFileSync(
  path.join(repoRoot, 'crates/im-domain-core/Cargo.toml'),
  'utf8',
);
const imPlatformContractsCargoSource = fs.readFileSync(
  path.join(repoRoot, 'crates/im-platform-contracts/Cargo.toml'),
  'utf8',
);
const postgresEnvExampleSource = fs.readFileSync(
  path.join(repoRoot, '.env.postgres.example'),
  'utf8',
);
const postgresDatabaseConfigIndexSource = fs.readFileSync(
  path.join(repoRoot, 'docs/部署/postgresql-database-configuration.md'),
  'utf8',
);
const postgresDevelopmentGuideSource = fs.readFileSync(
  path.join(repoRoot, 'docs/部署/开发环境PostgreSQL数据库配置教程.md'),
  'utf8',
);
const postgresProductionGuideSource = fs.readFileSync(
  path.join(repoRoot, 'docs/部署/线上环境PostgreSQL数据库配置教程.md'),
  'utf8',
);
const localAppApiSource = fs.readFileSync(
  path.join(repoRoot, 'scripts/dev/start-craw-chat-local-app-api.mjs'),
  'utf8',
);
const sharedDatabaseSource = fs.readFileSync(
  path.join(repoRoot, 'scripts/dev/craw-chat-shared-database.mjs'),
  'utf8',
);
const {
  createSdkworkChatPcDevPlan,
  runSdkworkChatPcDev,
} = await import(pathToFileURL(path.join(repoRoot, 'scripts/dev/run-sdkwork-chat-pc-dev.mjs')).href);
const {
  resolveCrawChatSharedDatabaseConfig,
} = await import(pathToFileURL(path.join(repoRoot, 'scripts/dev/craw-chat-shared-database.mjs')).href);

assert.equal(
  packageJson.scripts.dev,
  'node ./scripts/dev/run-sdkwork-chat-pc-dev.mjs --target browser',
  'root pnpm dev must start the browser dev stack with the PostgreSQL env profile by default',
);
assert.equal(
  packageJson.scripts['dev:postgres'],
  'node ./scripts/dev/run-sdkwork-chat-pc-dev.mjs --target browser --database postgres',
  'root pnpm dev:postgres must explicitly start the browser dev stack with the PostgreSQL profile',
);
assert.equal(
  packageJson.scripts['dev:sqlite'],
  'node ./scripts/dev/run-sdkwork-chat-pc-dev.mjs --target browser --database sqlite',
  'root pnpm dev:sqlite must explicitly start the browser dev stack with SQLite',
);
assert.doesNotMatch(
  packageJson.scripts['dev:postgres'],
  / --env-file /u,
  'root pnpm dev:postgres must not use Node 22 reserved --env-file argument',
);
assert.equal(
  packageJson.scripts['tauri:dev'],
  'node ./scripts/dev/run-sdkwork-chat-pc-dev.mjs --target desktop',
  'root pnpm tauri:dev must start the desktop dev stack with the SQLite profile by default',
);
assert.equal(
  packageJson.scripts['tauri:dev:postgres'],
  'node ./scripts/dev/run-sdkwork-chat-pc-dev.mjs --target desktop --database postgres',
  'root pnpm tauri:dev:postgres must explicitly start the desktop dev stack with the PostgreSQL profile',
);
assert.equal(
  packageJson.scripts['tauri:dev:sqlite'],
  'node ./scripts/dev/run-sdkwork-chat-pc-dev.mjs --target desktop --database sqlite',
  'root pnpm tauri:dev:sqlite must explicitly start the desktop dev stack with SQLite',
);
assert.doesNotMatch(
  packageJson.scripts['tauri:dev:postgres'],
  / --env-file /u,
  'root pnpm tauri:dev:postgres must not use Node 22 reserved --env-file argument',
);
assert.equal(
  packageJson.scripts['server:dev'],
  'node ./scripts/dev/start-craw-chat-unified-web.mjs',
  'root server:dev must remain the canonical Craw Chat server startup command',
);
assert.ok(
  unifiedWebSource.includes('ensureDevSiteDist'),
  'root server:dev must create local dev site fallbacks when admin or portal sources are absent',
);
assert.ok(
  unifiedWebSource.includes('cargo')
    && unifiedWebSource.includes('web-gateway')
    && unifiedWebSource.includes('craw-chat-server')
    && unifiedWebSource.includes('CRAW_CHAT_WEB_GATEWAY_RUNTIME_MODE')
    && unifiedWebSource.includes('resolveCrawChatSharedDatabaseConfig'),
  'root server:dev must start the Rust web-gateway in embedded local mode with the shared database config',
);
assert.doesNotMatch(
  unifiedWebSource,
  /\bmvn(?:\.cmd)?\b|mavenCommand|spring-ai-plus-server-app|spring-boot:run|ensureAppbaseAppApi|waitForAppbaseAppApiReady|sdkwork-appbase-app-api|CRAW_CHAT_APPBASE_APP_API_UPSTREAM|SDKWORK_APPBASE_APP_API_BIND_ADDR|SDKWORK_APPBASE_BROWSER_ORIGINS/u,
  'root server:dev must not start or wait for any Java/appbase app-api upstream',
);
assert.ok(
  unifiedWebSource.includes('CRAW_CHAT_ADMIN_SITE_DIR'),
  'root server:dev must pass the resolved admin site dir to the Rust gateway',
);
assert.ok(
  unifiedWebSource.includes('CRAW_CHAT_PORTAL_SITE_DIR'),
  'root server:dev must pass the resolved portal site dir to the Rust gateway',
);
assert.ok(
  unifiedWebSource.includes('terminateStaleCrawChatServerProcesses')
    && unifiedWebSource.includes('target')
    && unifiedWebSource.includes('craw-chat-server.exe'),
  'root server:dev must clean up stale same-workspace Windows craw-chat-server.exe processes before cargo rebuilds the locked binary',
);
assert.ok(
  unifiedWebSource.includes('terminateProcessTree')
    && unifiedWebSource.includes('taskkill')
    && unifiedWebSource.includes('/T')
    && unifiedWebSource.includes('/F'),
  'root server:dev must terminate the Windows cargo/server process tree instead of leaving craw-chat-server.exe behind',
);
assert.ok(
  localAppApiSource.includes('Craw Chat app-api is provided by the Rust unified server')
    && localAppApiSource.includes('resolveCrawChatSharedDatabaseConfig'),
  'legacy local app API wrapper must point developers to the Rust unified server instead of starting a separate app-api host',
);
assert.doesNotMatch(
  localAppApiSource,
  /\bmvn(?:\.cmd)?\b|mavenCommand|spring-ai-plus-server-app|spring-boot:run|SDKWORK_APPBASE_APP_API_BIND_ADDR|SDKWORK_APPBASE_BROWSER_ORIGINS|SDKWORK_APPBASE_APP_API_STARTUP_TIMEOUT_MS/u,
  'legacy local app API wrapper must not retain Java or appbase startup residue',
);
assert.ok(
  !localAppApiSource.includes('run-local-minimal.mjs'),
  'legacy local app API wrapper must not start a second runtime beside the unified Rust server',
);
assert.doesNotMatch(
  localMinimalNodeCargoSource,
  /[A-Za-z]:[\\/]/u,
  'local-minimal-node Rust manifest must not point to an absolute checkout path',
);
assert.ok(
  localMinimalNodeCargoSource.includes(
    'sdkwork-agent-business = { path = "../../../sdkwork-kernel/sdkwork-agent-business", features = ["http-axum"] }',
  ),
  'local-minimal-node must link sdkwork-agent-business from the sibling sdkwork-kernel checkout',
);
for (const cargoSource of [
  localMinimalNodeCargoSource,
  imDomainCoreCargoSource,
  imPlatformContractsCargoSource,
]) {
  assert.doesNotMatch(
    cargoSource,
    /[A-Za-z]:[\\/]([^\\/]+[\\/])*sdkwork-rtc/u,
    'Craw Chat Rust manifests must not point RTC dependencies at an absolute checkout path',
  );
  assert.match(
    cargoSource,
    /\.\.\/\.\.\/\.\.\/sdkwork-rtc\/crates\/sdkwork-rtc-core/u,
    'Craw Chat Rust manifests that depend on sdkwork-rtc-core must link the sibling sdkwork-rtc checkout',
  );
}
assert.ok(
  desktopWorkspaceSource.includes('catalog:'),
  'desktop workspace must define catalog entries used by imported SDKWork packages',
);
assert.doesNotMatch(
  desktopWorkspaceSource,
  /\.\.\/\.\.\/\.\.\/\.\.\/apps\/sdkwork-(?:appbase|core|ui)\//u,
  'desktop workspace must not register sibling sdkwork-appbase/core/ui packages as workspace importers; they stay source-linked dependencies so install never rewrites sibling node_modules',
);
assert.ok(
  fs.existsSync(path.join(repoRoot, 'apps/sdkwork-chat-pc/pnpm-lock.yaml')),
  'desktop app must keep dependency resolution in a pnpm lockfile',
);
assert.ok(
  !fs.existsSync(path.join(repoRoot, 'apps/sdkwork-chat-pc/package-lock.json')),
  'desktop app must not keep an npm package-lock.json alongside pnpm workspace management',
);
assert.ok(
  desktopWorkspaceSource.includes('lucide-react: ^1.7.0'),
  'desktop workspace catalog must resolve lucide-react catalog dependencies',
);
assert.ok(
  desktopWorkspaceSource.includes('vite-plugin-dts: ^4.5.4'),
  'desktop workspace catalog must resolve sdkwork-core build tooling dependencies',
);
assert.match(
  desktopNpmrcSource,
  /^link-workspace-packages=true$/mu,
  'desktop workspace must link workspace packages during local development',
);
assert.match(
  desktopNpmrcSource,
  /^auto-install-peers=false$/mu,
  'desktop workspace must not auto-install peer SDK packages from the registry',
);
assert.match(
  desktopNpmrcSource,
  /^virtual-store-dir=node_modules\/\.pnpm-codex-new$/mu,
  'desktop workspace must use the dedicated local virtual store so stale external workspace links do not block reinstall on Windows',
);

const sharedDependencyNames = [
  '@sdkwork-internal/im-app-api-generated',
  '@sdkwork-internal/im-backend-api-generated',
  '@sdkwork/appbase-app-sdk',
  '@sdkwork/appbase-pc-react',
  '@sdkwork/auth-pc-react',
  '@sdkwork/auth-runtime-pc-react',
  '@sdkwork/core-pc-react',
  '@sdkwork/iam-sdk-ports',
  '@sdkwork/im-sdk',
  '@sdkwork/i18n-pc-react',
  '@sdkwork/ui-pc-react',
];
for (const dependencyName of sharedDependencyNames) {
  const version = chatPcPackageJson.dependencies?.[dependencyName];
  assert.match(
    version,
    /^link:\.\.\//u,
    `local dev dependency ${dependencyName} must use a relative link: specifier`,
  );
  assert.doesNotMatch(
    version,
    /^(?:https?:|git\+|github:|git@)/u,
    `local dev dependency ${dependencyName} must not use a git or registry URL`,
  );
}

const sharedSdkOverrides = chatPcPackageJson.pnpm?.overrides ?? {};
for (const [dependencyName, expectedVersion] of Object.entries({
  '@sdkwork-internal/im-app-api-generated': 'link:../../sdks/sdkwork-im-app-sdk/sdkwork-im-app-sdk-typescript/generated/server-openapi',
  '@sdkwork-internal/im-backend-api-generated': 'link:../../sdks/sdkwork-im-backend-sdk/sdkwork-im-backend-sdk-typescript/generated/server-openapi',
  '@sdkwork/appbase-app-sdk': 'link:../../../sdkwork-appbase/sdks/sdkwork-appbase-app-sdk/sdkwork-appbase-app-sdk-typescript/generated/server-openapi',
  '@sdkwork/im-sdk': 'link:../../sdks/sdkwork-im-sdk/sdkwork-im-sdk-typescript',
  '@sdkwork/rtc-sdk': 'link:../../../sdkwork-rtc/sdks/sdkwork-rtc-sdk/sdkwork-rtc-sdk-typescript',
})) {
  assert.equal(
    sharedSdkOverrides[dependencyName],
    expectedVersion,
    `desktop workspace must override transitive ${dependencyName} to the local source path`,
  );
  assert.doesNotMatch(
    desktopLockfileSource,
    new RegExp(`^\\s{2}'${dependencyName.replaceAll('/', '\\/')}@`, 'mu'),
    `desktop lockfile must not resolve registry package snapshots for ${dependencyName}`,
  );
}

const sharedSdkModeSource = fs.readFileSync(
  path.join(repoRoot, 'scripts/dev/shared-sdk-mode.mjs'),
  'utf8',
);
const sharedSdkGitSource = fs.readFileSync(
  path.join(repoRoot, 'scripts/dev/prepare-shared-sdk-git-sources.mjs'),
  'utf8',
);
const releaseBuildSource = fs.readFileSync(
  path.join(repoRoot, 'scripts/release/run-sdkwork-chat-pc-release-build.mjs'),
  'utf8',
);
const releaseBuildModule = await import(
  pathToFileURL(path.join(repoRoot, 'scripts/release/run-sdkwork-chat-pc-release-build.mjs')).href
);
const sharedSdkReleaseConfig = readJson('config/shared-sdk-release-sources.json');
assert.match(
  packageJson.scripts['prepare:shared-sdk'],
  /scripts\/dev\/prepare-shared-sdk-git-sources\.mjs/u,
  'root prepare:shared-sdk must materialize git-backed shared SDK sources for release builds',
);
assert.match(
  packageJson.scripts['release:build'],
  /scripts\/release\/run-sdkwork-chat-pc-release-build\.mjs/u,
  'root release:build must delegate to the repository-owned release build wrapper',
);
assert.match(
  releaseBuildSource,
  /SDKWORK_SHARED_SDK_MODE.*git/u,
  'release build wrapper must enable git-backed shared SDK mode',
);
assert.match(
  releaseBuildSource,
  /prepare:shared-sdk/u,
  'root release:build must prepare shared SDK sources before building the Vite app',
);
assert.equal(
  typeof releaseBuildModule.createSdkworkChatPcReleaseBuildPlan,
  'function',
  'release build wrapper must expose an auditable command plan',
);
const releaseBuildPlan = releaseBuildModule.createSdkworkChatPcReleaseBuildPlan({
  env: {
    CRAW_CHAT_SERVER_API_BASE_URL: 'https://chat.example.com/',
    CRAW_CHAT_SERVER_WEBSOCKET_BASE_URL: 'wss://realtime.example.com',
    SDKWORK_SHARED_SDK_MODE: 'source',
  },
  repoRoot,
});
assert.deepEqual(
  releaseBuildPlan.steps.map((step) => step.label),
  [
    'prepare shared SDK git sources',
    'install sdkwork-chat-pc workspace',
    'build sdkwork-chat-pc Vite app',
  ],
  'release build must prepare git sources, install once, then build the Vite app from linked source packages',
);
assert.equal(
  releaseBuildPlan.env.SDKWORK_SHARED_SDK_MODE,
  'git',
  'release build plan must force git-backed shared SDK mode',
);
assert.equal(
  releaseBuildPlan.env.SDKWORK_IAM_MODE,
  'private',
  'release build plan must use server-private IAM mode by default',
);
assert.equal(
  releaseBuildPlan.env.VITE_CRAW_CHAT_APP_API_BASE_URL,
  'https://chat.example.com',
  'release build plan must expose the standard server API domain to the Vite app SDK wrapper',
);
assert.equal(
  releaseBuildPlan.env.VITE_CRAW_CHAT_IM_API_BASE_URL,
  'https://chat.example.com',
  'release build plan must expose the standard server API domain to the Vite IM HTTP wrapper',
);
assert.equal(
  releaseBuildPlan.env.VITE_CRAW_CHAT_IM_WEBSOCKET_BASE_URL,
  'wss://realtime.example.com',
  'release build plan must expose the standard server websocket domain to the Vite IM websocket wrapper',
);
assert.deepEqual(
  releaseBuildPlan.steps.at(-1).args,
  ['build'],
  'release build final step must build the sdkwork-chat-pc Vite package from the app workspace cwd',
);
assert.equal(
  path.relative(repoRoot, releaseBuildPlan.steps.at(1).cwd).replaceAll('\\', '/'),
  'apps/sdkwork-chat-pc',
  'release build install step must run from the sdkwork-chat-pc workspace cwd so pnpm uses the app-local .npmrc and virtual store',
);
assert.equal(
  path.relative(repoRoot, releaseBuildPlan.steps.at(-1).cwd).replaceAll('\\', '/'),
  'apps/sdkwork-chat-pc',
  'release build final step must run from the sdkwork-chat-pc workspace cwd',
);
const releaseSpawnCalls = [];
releaseBuildModule.runSdkworkChatPcReleaseBuild({
  env: { SDKWORK_SHARED_SDK_MODE: 'source' },
  repoRoot,
  spawnSyncImpl(command, args, options) {
    releaseSpawnCalls.push({ args, command, options });
    return { status: 0 };
  },
});
assert.deepEqual(
  releaseSpawnCalls.map((call) => call.args),
  releaseBuildPlan.steps.map((step) => step.args),
  'release build runner must execute the audited command plan in order',
);
assert.deepEqual(
  releaseSpawnCalls.map((call) => path.relative(repoRoot, call.options.cwd).replaceAll('\\', '/')),
  releaseBuildPlan.steps.map((step) => path.relative(repoRoot, step.cwd).replaceAll('\\', '/')),
  'release build runner must execute every pnpm step from the audited cwd',
);
assert.ok(
  releaseSpawnCalls.every((call) => call.options.env.SDKWORK_SHARED_SDK_MODE === 'git'),
  'release build runner must force git-backed shared SDK mode for every pnpm command',
);
assert.match(
  sharedSdkModeSource,
  /SDKWORK_SHARED_SDK_MODE/u,
  'shared SDK mode helper must expose SDKWORK_SHARED_SDK_MODE',
);
assert.match(
  sharedSdkGitSource,
  /SDKWORK_SHARED_SDK_GIT_PROTOCOL/u,
  'shared SDK git materializer must allow release jobs to select https or ssh transport',
);
assert.match(
  sharedSdkGitSource,
  /config\/shared-sdk-release-sources\.json/u,
  'shared SDK git materializer must load the release source map from config/shared-sdk-release-sources.json',
);
for (const sourceName of [
  'sdkwork-appbase',
  'sdkwork-core',
  'sdkwork-ui',
  'sdkwork-im-app-sdk',
  'sdkwork-im-backend-sdk',
  'sdkwork-im-sdk',
  'sdkwork-claw-router',
  'sdkwork-birdcoder',
]) {
  const sourceConfig = sharedSdkReleaseConfig.sources?.[sourceName];
  assert.ok(sourceConfig, `release shared SDK config must define ${sourceName}`);
  assert.match(
    sourceConfig.repoUrl,
    /^(?:https:\/\/github\.com\/|git@github\.com:).+\.git$/u,
    `${sourceName} release repoUrl must be a git repository URL`,
  );
  assert.ok(
    typeof sourceConfig.ref === 'string' && sourceConfig.ref.trim().length > 0,
    `${sourceName} release config must pin a non-empty git ref`,
  );
}

const thirdPartyDependencyNames = new Set([
  '@google/genai',
  '@sdkwork/sdk-common',
  '@tailwindcss/typography',
  '@tailwindcss/vite',
  '@tauri-apps/api',
  '@tauri-apps/cli',
  '@tiptap/extension-bubble-menu',
  '@tiptap/extension-floating-menu',
  '@tiptap/extension-image',
  '@tiptap/extension-link',
  '@tiptap/extension-placeholder',
  '@tiptap/pm',
  '@tiptap/react',
  '@tiptap/starter-kit',
  '@types/express',
  '@types/node',
  '@vitejs/plugin-react',
  'autoprefixer',
  'clsx',
  'dotenv',
  'emoji-picker-react',
  'esbuild',
  'express',
  'framer-motion',
  'i18next',
  'lucide-react',
  'motion',
  'qrcode',
  'react',
  'react-dom',
  'react-i18next',
  'react-markdown',
  'react-qr-code',
  'react-router-dom',
  'signature_pad',
  'tailwind-merge',
  'tailwindcss',
  'tiptap-markdown',
  'tsx',
  'typescript',
  'vite',
]);

for (const sectionName of ['dependencies', 'devDependencies']) {
  for (const [name, version] of Object.entries(chatPcPackageJson[sectionName] ?? {})) {
    if (thirdPartyDependencyNames.has(name)) {
      assert.equal(
        version,
        'catalog:',
        `apps/sdkwork-chat-pc ${sectionName}.${name} must use the pnpm catalog`,
      );
    }
  }
}

const packageJsonFiles = fs
  .readdirSync(path.join(repoRoot, 'apps/sdkwork-chat-pc/packages'), { withFileTypes: true })
  .filter((entry) => entry.isDirectory())
  .map((entry) => path.join(repoRoot, 'apps/sdkwork-chat-pc/packages', entry.name, 'package.json'))
  .filter((candidate) => fs.existsSync(candidate));
for (const packageJsonPath of packageJsonFiles) {
  const workspacePackage = JSON.parse(fs.readFileSync(packageJsonPath, 'utf8'));
  for (const sectionName of ['dependencies', 'devDependencies', 'peerDependencies']) {
    for (const [name, version] of Object.entries(workspacePackage[sectionName] ?? {})) {
      if (name.startsWith('@sdkwork/clawchat-')) {
        assert.equal(
          version,
          'workspace:*',
          `${path.relative(repoRoot, packageJsonPath)} ${sectionName}.${name} must use workspace:*`,
        );
        continue;
      }
      if (thirdPartyDependencyNames.has(name)) {
        assert.equal(
          version,
          'catalog:',
          `${path.relative(repoRoot, packageJsonPath)} ${sectionName}.${name} must use catalog:`,
        );
      }
    }
  }
}

const pnpmCommand = process.platform === 'win32' ? 'pnpm.cmd' : 'pnpm';
const pnpmShell = process.platform === 'win32';

const browserPlan = createSdkworkChatPcDevPlan({
  argv: ['--target', 'browser'],
  env: {},
  repoRoot,
});
assert.equal(browserPlan.target, 'browser');
assert.deepEqual(
  browserPlan.processes.map((entry) => entry.label),
  ['craw-chat-server', 'sdkwork-chat-pc-browser'],
  'browser dev must start one unified Craw Chat server process and the browser renderer only',
);
assert.ok(
  !('SDKWORK_APPBASE_APP_API_BIND_ADDR' in browserPlan.processes[0].env)
    && !('CRAW_CHAT_APPBASE_APP_API_UPSTREAM' in browserPlan.processes[0].env)
    && !('SDKWORK_APPBASE_BROWSER_ORIGINS' in browserPlan.processes[0].env),
  'unified server must not configure any Java/appbase app-api upstream',
);
assert.equal(
  browserPlan.processes[0].env.CRAW_CHAT_WEB_GATEWAY_RUNTIME_MODE,
  'embedded',
  'unified server must embed Craw Chat local IM/backend runtime instead of requiring per-service dev ports',
);
assert.equal(
  browserPlan.processes[0].env.SDKWORK_CHAT_DATABASE_URL,
  'postgresql://sdkwork_ai_dev:sdkworkdev123@127.0.0.1:5432/sdkwork_ai_dev?sslmode=disable',
  'unified Rust server must receive the canonical PostgreSQL dev database URL by default',
);
assert.equal(
  browserPlan.processes[0].env.SDKWORK_CLAW_DATABASE_URL,
  browserPlan.processes[0].env.SDKWORK_CHAT_DATABASE_URL,
  'unified Rust server must receive the current compatibility database URL bridge',
);
assert.equal(
  browserPlan.processes[0].env.CRAW_CHAT_BROWSER_ORIGINS,
  'http://127.0.0.1:1620,http://localhost:1620,http://127.0.0.1:4176,http://localhost:4176',
  'unified Rust server must allow the desktop Vite origins at the gateway CORS layer',
);
assert.deepEqual(browserPlan.processes[0], {
  args: ['server:dev'],
  command: pnpmCommand,
  cwd: repoRoot,
  env: browserPlan.processes[0].env,
  label: 'craw-chat-server',
  shell: pnpmShell,
});
assert.deepEqual(browserPlan.processes[1], {
  args: ['--dir', 'apps/sdkwork-chat-pc', 'dev'],
  command: pnpmCommand,
  cwd: repoRoot,
  env: browserPlan.processes[1].env,
  label: 'sdkwork-chat-pc-browser',
  shell: pnpmShell,
});
assert.equal(
  browserPlan.processes[1].env.VITE_SDKWORK_IAM_APP_API_BASE_URL,
  'http://127.0.0.1:18079',
  'browser renderer must point IAM traffic at the unified Craw Chat gateway',
);
assert.equal(
  browserPlan.processes[1].env.VITE_CRAW_CHAT_APP_API_BASE_URL,
  'http://127.0.0.1:18079',
  'browser renderer must point Craw Chat app traffic at the unified Craw Chat gateway',
);
assert.equal(
  browserPlan.processes[1].env.VITE_CRAW_CHAT_IM_API_BASE_URL,
  'http://127.0.0.1:18079',
  'browser renderer must point IM HTTP traffic at the Craw Chat gateway/server',
);
assert.equal(
  browserPlan.processes[1].env.VITE_CRAW_CHAT_IM_WEBSOCKET_BASE_URL,
  'ws://127.0.0.1:18079',
  'browser renderer must point IM websocket traffic at the Craw Chat gateway/server',
);

const postgresDatabaseConfig = resolveCrawChatSharedDatabaseConfig({
  env: {
    SDKWORK_CLAW_DATABASE_URL: 'postgresql://sdkwork_ai_dev:sdkworkdev123@127.0.0.1:15432/sdkwork_ai_dev?sslmode=disable',
  },
  repoRoot,
});
assert.equal(postgresDatabaseConfig.kind, 'postgresql');
assert.equal(
  postgresDatabaseConfig.env.SDKWORK_CLAW_DATABASE_URL,
  'postgresql://sdkwork_ai_dev:sdkworkdev123@127.0.0.1:15432/sdkwork_ai_dev?sslmode=disable',
  'shared DB helper must pass PostgreSQL URLs to the Rust server unchanged',
);
assert.equal(postgresDatabaseConfig.postgres.username, 'sdkwork_ai_dev');
assert.equal(postgresDatabaseConfig.postgres.password, 'sdkworkdev123');
assert.equal(postgresDatabaseConfig.postgres.database, 'sdkwork_ai_dev');

const postgresSplitDatabaseConfig = resolveCrawChatSharedDatabaseConfig({
  env: {
    SDKWORK_CLAW_DATABASE_PROVIDER: 'postgresql',
    SDKWORK_CLAW_DATABASE_HOST: '127.0.0.1',
    SDKWORK_CLAW_DATABASE_PORT: '15432',
    SDKWORK_CLAW_DATABASE_NAME: 'sdkwork_ai_dev',
    SDKWORK_CLAW_DATABASE_USERNAME: 'sdkwork_ai_dev',
    SDKWORK_CLAW_DATABASE_PASSWORD: 'chat pass',
    SDKWORK_CLAW_DATABASE_SSLMODE: 'disable',
    SDKWORK_CLAW_DATABASE_MAX_CONNECTIONS: '12',
  },
  repoRoot,
});
assert.equal(postgresSplitDatabaseConfig.kind, 'postgresql');
assert.equal(
  postgresSplitDatabaseConfig.env.SDKWORK_CLAW_DATABASE_URL,
  'postgresql://sdkwork_ai_dev:chat%20pass@127.0.0.1:15432/sdkwork_ai_dev?sslmode=disable',
  'shared DB helper must assemble PostgreSQL URLs from split database fields',
);
assert.equal(
  postgresSplitDatabaseConfig.env.SDKWORK_CLAW_DATABASE_MAX_CONNECTIONS,
  '12',
  'shared DB helper must pass split-field PostgreSQL max connection settings to the Rust server',
);
assert.equal(postgresSplitDatabaseConfig.postgres.username, 'sdkwork_ai_dev');
assert.equal(postgresSplitDatabaseConfig.postgres.password, 'chat pass');
assert.equal(postgresSplitDatabaseConfig.postgres.database, 'sdkwork_ai_dev');
assert.throws(
  () => resolveCrawChatSharedDatabaseConfig({
    env: {
      SDKWORK_CLAW_DATABASE_PROVIDER: 'postgres',
      SDKWORK_CLAW_DATABASE_HOST: '127.0.0.1',
      SDKWORK_CLAW_DATABASE_NAME: 'sdkwork_ai_dev',
      SDKWORK_CLAW_DATABASE_USERNAME: 'sdkwork_ai_dev',
    },
    repoRoot,
  }),
  /SDKWORK_CHAT_DATABASE_PASSWORD/u,
  'split-field PostgreSQL configuration must require an explicit password',
);
assert.throws(
  () => resolveCrawChatSharedDatabaseConfig({
    env: {
      SDKWORK_CLAW_DATABASE_PROVIDER: 'mysql',
      SDKWORK_CLAW_DATABASE_HOST: '127.0.0.1',
      SDKWORK_CLAW_DATABASE_NAME: 'sdkwork_ai_dev',
      SDKWORK_CLAW_DATABASE_USERNAME: 'sdkwork_ai_dev',
      SDKWORK_CLAW_DATABASE_PASSWORD: 'sdkworkdev123',
    },
    repoRoot,
  }),
  /unsupported SDKWork Chat database engine/u,
  'shared DB helper must reject unsupported split-field database engines instead of silently falling back to SQLite',
);

const postgresUrlPrecedenceConfig = resolveCrawChatSharedDatabaseConfig({
  env: {
    SDKWORK_CLAW_DATABASE_URL: 'postgresql://url_user:url_pass@127.0.0.1:25432/url_db?sslmode=require',
    SDKWORK_CLAW_DATABASE_PROVIDER: 'postgresql',
    SDKWORK_CLAW_DATABASE_HOST: '127.0.0.1',
    SDKWORK_CLAW_DATABASE_PORT: '15432',
    SDKWORK_CLAW_DATABASE_NAME: 'split_db',
    SDKWORK_CLAW_DATABASE_USERNAME: 'split_user',
    SDKWORK_CLAW_DATABASE_PASSWORD: 'split_pass',
    SDKWORK_CLAW_DATABASE_SSLMODE: 'disable',
  },
  repoRoot,
});
assert.equal(
  postgresUrlPrecedenceConfig.env.SDKWORK_CLAW_DATABASE_URL,
  'postgresql://url_user:url_pass@127.0.0.1:25432/url_db?sslmode=require',
  'explicit SDKWORK_CLAW_DATABASE_URL must take precedence over split PostgreSQL fields',
);
assert.doesNotMatch(
  sharedDatabaseSource,
  /SPRING_DATASOURCE|org\.sqlite\.JDBC|org\.postgresql\.Driver/u,
  'shared DB helper must not carry Spring datasource configuration in the Rust server architecture',
);
for (const requiredName of [
  'SDKWORK_CHAT_DATABASE_ENGINE=postgresql',
  'SDKWORK_CHAT_DATABASE_HOST=127.0.0.1',
  'SDKWORK_CHAT_DATABASE_PORT=5432',
  'SDKWORK_CHAT_DATABASE_NAME=sdkwork_ai_dev',
  'SDKWORK_CHAT_DATABASE_USERNAME=sdkwork_ai_dev',
  'SDKWORK_CHAT_DATABASE_PASSWORD=sdkworkdev123',
  'SDKWORK_CHAT_DATABASE_SSL_MODE=disable',
  'SDKWORK_CHAT_DATABASE_MAX_CONNECTIONS=10',
]) {
  assert.ok(
    postgresEnvExampleSource.includes(requiredName),
    `.env.postgres.example must document ${requiredName}`,
  );
}
assert.ok(
  postgresDatabaseConfigIndexSource.includes('./开发环境PostgreSQL数据库配置教程.md')
    && postgresDatabaseConfigIndexSource.includes('./线上环境PostgreSQL数据库配置教程.md')
    && postgresDatabaseConfigIndexSource.includes('pnpm dev')
    && postgresDatabaseConfigIndexSource.includes('pnpm tauri:dev')
    && postgresDatabaseConfigIndexSource.includes('pnpm dev:sqlite')
    && postgresDatabaseConfigIndexSource.includes('/etc/sdkwork/chat/chat.toml')
    && postgresDatabaseConfigIndexSource.includes('/etc/sdkwork/chat/database.secret')
    && postgresDatabaseConfigIndexSource.includes('~/.sdkwork/chat/data/chat.sqlite'),
  'PostgreSQL database configuration index must link the environment-specific development and production guides',
);
assert.ok(
  postgresDevelopmentGuideSource.includes('pnpm dev')
    && postgresDevelopmentGuideSource.includes('pnpm tauri:dev')
    && postgresDevelopmentGuideSource.includes('pnpm dev:postgres')
    && postgresDevelopmentGuideSource.includes('pnpm tauri:dev:postgres')
    && postgresDevelopmentGuideSource.includes('pnpm dev:sqlite')
    && postgresDevelopmentGuideSource.includes('pnpm tauri:dev:sqlite')
    && postgresDevelopmentGuideSource.includes('.env.postgres')
    && postgresDevelopmentGuideSource.includes('SDKWORK_CHAT_DATABASE_ENGINE=postgresql')
    && postgresDevelopmentGuideSource.includes('SDKWORK_CHAT_DATABASE_SSL_MODE=disable')
    && postgresDevelopmentGuideSource.includes('pnpm tauri:dev 默认使用 SQLite')
    && postgresDevelopmentGuideSource.includes('Copy-Item .env.postgres.example .env.postgres'),
  'development PostgreSQL guide must document local env profile setup and both dev startup commands',
);
assert.ok(
  postgresProductionGuideSource.includes('/etc/sdkwork/chat/')
    && postgresProductionGuideSource.includes('chat.toml')
    && postgresProductionGuideSource.includes('server.env')
    && postgresProductionGuideSource.includes('postgresql.yaml')
    && postgresProductionGuideSource.includes('database.secret')
    && postgresProductionGuideSource.includes('Windows Service')
    && postgresProductionGuideSource.includes('PGPASSWORD='),
  'production PostgreSQL guide must document config-root, password-file, and service deployment workflow',
);

const tempEnvDir = path.join(repoRoot, '.runtime', 'dev-command-test');
fs.mkdirSync(tempEnvDir, { recursive: true });
const tempPostgresEnvFile = path.join(tempEnvDir, 'postgres.env');
fs.writeFileSync(
  tempPostgresEnvFile,
  [
    'SDKWORK_CHAT_DATABASE_ENGINE=postgresql',
    'SDKWORK_CHAT_DATABASE_HOST=127.0.0.1',
    'SDKWORK_CHAT_DATABASE_PORT=15433',
    'SDKWORK_CHAT_DATABASE_NAME=env_file_db',
    'SDKWORK_CHAT_DATABASE_USERNAME=env_file_user',
    'SDKWORK_CHAT_DATABASE_PASSWORD=env file pass',
    'SDKWORK_CHAT_DATABASE_SSL_MODE=disable',
    'SDKWORK_CHAT_DATABASE_MAX_CONNECTIONS=15',
    '',
  ].join('\n'),
);
const postgresEnvFilePlan = createSdkworkChatPcDevPlan({
  argv: ['--target', 'browser', '--dev-env-file', tempPostgresEnvFile],
  env: {},
  repoRoot,
});
assert.equal(
  postgresEnvFilePlan.processes[0].env.SDKWORK_CHAT_DATABASE_URL,
  'postgresql://env_file_user:env%20file%20pass@127.0.0.1:15433/env_file_db?sslmode=disable',
  'dev command must load --dev-env-file and pass the canonical assembled PostgreSQL URL to the unified server',
);
assert.equal(
  postgresEnvFilePlan.processes[0].env.SDKWORK_CLAW_DATABASE_URL,
  'postgresql://env_file_user:env%20file%20pass@127.0.0.1:15433/env_file_db?sslmode=disable',
  'dev command must keep the current Rust-compatible PostgreSQL URL bridge',
);
assert.equal(
  postgresEnvFilePlan.processes[0].env.SDKWORK_CHAT_DATABASE_MAX_CONNECTIONS,
  '15',
  'dev command must load canonical PostgreSQL max connections from --dev-env-file',
);

const sqliteBrowserPlan = createSdkworkChatPcDevPlan({
  argv: ['--target', 'browser', '--database', 'sqlite'],
  env: {},
  repoRoot,
});
assert.match(
  sqliteBrowserPlan.processes[0].env.SDKWORK_CHAT_DATABASE_URL,
  /sqlite:\/\/.*[/\\]\.sdkwork[/\\]chat[/\\]data[/\\]chat\.sqlite$/u,
  'pnpm dev:sqlite must explicitly use the SDKWork Chat user-private SQLite database URL',
);

const desktopPlan = createSdkworkChatPcDevPlan({
  argv: ['--target', 'desktop'],
  env: {},
  repoRoot,
});
assert.equal(desktopPlan.target, 'desktop');
assert.deepEqual(
  desktopPlan.processes.map((entry) => entry.label),
  ['craw-chat-server', 'sdkwork-chat-pc-desktop'],
  'desktop dev must start one unified Craw Chat server process and the Tauri desktop process only',
);
assert.deepEqual(desktopPlan.processes[1], {
  args: ['--dir', 'apps/sdkwork-chat-pc/packages/sdkwork-clawchat-pc-desktop', 'desktop:dev:local'],
  command: pnpmCommand,
  cwd: repoRoot,
  env: desktopPlan.processes[1].env,
  label: 'sdkwork-chat-pc-desktop',
  shell: pnpmShell,
});
assert.notDeepEqual(
  desktopPlan.processes[1].args,
  ['--dir', 'apps/sdkwork-chat-pc', 'desktop:dev:local'],
  'desktop dev must run the dedicated desktop package instead of the root web package',
);
assert.equal(
  desktopPlan.processes[1].env.VITE_SDKWORK_IAM_APP_API_BASE_URL,
  'http://127.0.0.1:18079',
  'desktop renderer must point IAM traffic at the unified Craw Chat gateway',
);
assert.equal(
  desktopPlan.processes[1].env.VITE_CRAW_CHAT_IM_API_BASE_URL,
  'http://127.0.0.1:18079',
  'desktop renderer must point IM HTTP traffic at the Craw Chat gateway/server',
);
assert.equal(
  desktopPlan.processes[0].env.SDKWORK_CHAT_DATABASE_ENGINE,
  'sqlite',
  'desktop dev must default to SQLite for local user data',
);
assert.match(
  desktopPlan.processes[0].env.SDKWORK_CHAT_DATABASE_URL,
  /sqlite:\/\/.*[/\\]\.sdkwork[/\\]chat[/\\]data[/\\]chat\.sqlite$/u,
  'desktop dev SQLite data must remain under the SDKWork Chat user-private data directory',
);
assert.equal(
  desktopPlan.processes[0].env.SDKWORK_CLAW_DATABASE_URL,
  desktopPlan.processes[0].env.SDKWORK_CHAT_DATABASE_URL,
  'desktop dev must bridge the canonical SQLite URL to the current Rust-compatible env name',
);

const spawned = [];
function createFakeChild() {
  const child = new EventEmitter();
  child.stdout = new EventEmitter();
  child.stderr = new EventEmitter();
  child.exitCode = null;
  child.signalCode = null;
  child.kill = () => {};
  return child;
}

runSdkworkChatPcDev({
  argv: ['--target', 'desktop'],
  env: {},
  repoRoot,
  spawnImpl(command, args, options) {
    spawned.push({ command, args, options });
    return createFakeChild();
  },
  stdout: { write() {} },
  stderr: { write() {} },
});

assert.equal(spawned.length, 2, 'desktop dev runner must spawn unified server and desktop processes only');
assert.deepEqual(
  spawned.map((entry) => entry.options.shell),
  [pnpmShell, pnpmShell],
  'dev runner must pass the planned shell mode to every child process',
);
const devRunnerSource = fs.readFileSync(
  path.join(repoRoot, 'scripts/dev/run-sdkwork-chat-pc-dev.mjs'),
  'utf8',
);
assert.ok(
  devRunnerSource.includes('terminateProcessTree')
    && devRunnerSource.includes('taskkill')
    && devRunnerSource.includes('/T')
    && devRunnerSource.includes('/F'),
  'dev runner shutdown must terminate Windows process trees so cargo grandchildren cannot keep target/debug/craw-chat-server.exe locked',
);

const chatPcAppRoot = path.join(repoRoot, 'apps/sdkwork-chat-pc');
const appRequire = createRequire(path.join(chatPcAppRoot, 'package.json'));
const { createServer } = await import(pathToFileURL(appRequire.resolve('vite')).href);
const viteServer = await createServer({
  configFile: path.join(chatPcAppRoot, 'vite.config.ts'),
  logLevel: 'silent',
  root: chatPcAppRoot,
  server: {
    hmr: false,
    middlewareMode: true,
  },
});
try {
  const mainEntry = path.join(chatPcAppRoot, 'src/main.tsx');
  const jsxDevRuntime = await viteServer.pluginContainer.resolveId('react/jsx-dev-runtime', mainEntry);
  assert.ok(jsxDevRuntime, 'Vite must resolve the React JSX dev runtime import injected for TSX dev builds');
  const resolvedJsxDevRuntimePath = path.normalize(jsxDevRuntime.id.split('?')[0]);
  assert.doesNotMatch(
    resolvedJsxDevRuntimePath,
    /react[\\/]index\.js[\\/]jsx-dev-runtime$/u,
    'Vite must not append react/jsx-dev-runtime to the bare react/index.js alias',
  );
  assert.ok(
    [
      path.normalize(path.join(chatPcAppRoot, 'node_modules/react/jsx-dev-runtime.js')),
      path.normalize(path.join(chatPcAppRoot, 'node_modules/.vite/deps/react_jsx-dev-runtime.js')),
    ].includes(resolvedJsxDevRuntimePath),
    'Vite must resolve react/jsx-dev-runtime from the chat PC dependency root instead of appending it to react/index.js',
  );
} finally {
  await viteServer.close();
}

assert.throws(
  () => createSdkworkChatPcDevPlan({ argv: ['--target', 'mobile'], env: {}, repoRoot }),
  /Unsupported sdkwork-chat-pc dev target/u,
  'dev command must reject unsupported targets',
);
assert.throws(
  () => createSdkworkChatPcDevPlan({ argv: ['--target', 'browser', '--database', 'mysql'], env: {}, repoRoot }),
  /Unsupported sdkwork-chat-pc dev database/u,
  'dev command must reject unsupported database profiles',
);

console.log('sdkwork-chat-pc root dev command contract passed');
