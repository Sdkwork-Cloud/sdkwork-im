import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..', '..');
const workspaceRoot = path.resolve(repoRoot, '..');

function read(relativePath) {
  return fs.readFileSync(path.join(repoRoot, ...relativePath.split('/')), 'utf8');
}

function readWorkspace(relativePath) {
  return fs.readFileSync(path.join(workspaceRoot, ...relativePath.split('/')), 'utf8');
}

function readJson(relativePath) {
  return JSON.parse(read(relativePath));
}

function unique(values) {
  return [...new Set(values)];
}

function extractAll(regex, source, group = 1) {
  return unique([...source.matchAll(regex)].map((match) => match[group].toLowerCase()));
}

function extractRustRawStrings(source) {
  return [...source.matchAll(/r#"(.*?)"#/gsu)].map((match) => match[1]);
}

const prefixRegistry = readJson('specs/database-prefix-registry.json');
const tableRegistry = readJson('specs/database-table-registry.json');
const activeMigrationDir = 'deployments/database/postgres/migrations';
const activeMigrationFiles = fs
  .readdirSync(path.join(repoRoot, ...activeMigrationDir.split('/')))
  .filter((entry) => entry.endsWith('.sql'))
  .sort((left, right) => left.localeCompare(right));
const schema = activeMigrationFiles
  .map((entry) => read(`${activeMigrationDir}/${entry}`))
  .join('\n')
  .toLowerCase();
const databaseSpec = readWorkspace('sdkwork-specs/DATABASE_SPEC.md');
const cargoManifest = read('Cargo.toml');
const runtimeIdCrate = read('crates/sdkwork-im-runtime-id/src/lib.rs');
const sdkworkWorkflow = readJson('sdkwork.workflow.json');
const sharedSdkReleaseSources = readJson('config/shared-sdk-release-sources.json');
const chatPcPnpmWorkspace = read('apps/sdkwork-im-pc/pnpm-workspace.yaml');
const componentSpec = readJson('specs/component.spec.json');
const localSpecReadme = read('specs/README.md');
const namingDoc = read('docs/部署/database-table-naming-standard.md');
const ubuntuWslGuide = read('docs/部署/Ubuntu与WSL-PostgreSQL初始化建库授权手册.md');

assert.equal(prefixRegistry.appCode, 'chat');
assert.equal(prefixRegistry.product, 'sdkwork-chat');
assert.equal(prefixRegistry.authority, '../../../specs/DATABASE_SPEC.md');

const imPrefix = prefixRegistry.prefixes.find((entry) => entry.prefix === 'im');
assert.ok(imPrefix, 'database prefix registry must register im for chat instant messaging tables');
assert.equal(imPrefix.status, 'ACTIVE');
assert.equal(imPrefix.businessDomain, 'instant_messaging');
assert.match(imPrefix.allowedTableNamePattern, /\^im_/u);
assert.ok(imPrefix.forbiddenAliases.includes('chat'));
assert.ok(imPrefix.forbiddenAliases.includes('comms'));

assert.equal(prefixRegistry.nonImPrefixPolicy.mustNotUseImPrefix, true);
assert.equal(prefixRegistry.nonImPrefixPolicy.keepExistingBusinessPrefix, true);
assert.match(
  prefixRegistry.nonImPrefixPolicy.description,
  /outside the instant messaging bounded context/u,
  'non-IM tables must not be swept into the im_ prefix',
);

assert.equal(tableRegistry.appCode, 'chat');
assert.equal(tableRegistry.prefixRegistry, './database-prefix-registry.json');
assert.ok(tableRegistry.databaseProfiles.includes('postgresql'));
assert.ok(tableRegistry.databaseProfiles.includes('sqlite'));

const registeredTables = tableRegistry.tables.map((entry) => entry.tableName);
assert.equal(
  registeredTables.length,
  unique(registeredTables).length,
  'database table registry must not contain duplicate table names',
);

for (const entry of tableRegistry.tables) {
  assert.equal(entry.modulePrefix, 'im', `${entry.tableName} must register modulePrefix=im`);
  const allowedBoundedContexts = new Set([
    'instant_messaging',
    'social',
    'organization',
    'messaging',
    'user',
  ]);
  assert.ok(
    allowedBoundedContexts.has(entry.boundedContext),
    `${entry.tableName} must belong to a registered IM bounded context`,
  );
  assert.match(entry.tableName, /^im_[a-z0-9]+(?:_[a-z0-9]+)*$/u);
  assert.ok(entry.tableProfile, `${entry.tableName} must declare a table profile`);
  assert.ok(entry.writeOwner, `${entry.tableName} must declare a write owner`);
  const migrationPath = entry.migration;
  assert.match(
    migrationPath,
    /^deployments\/database\/postgres\/migrations\/[0-9]{3}_[a-z0-9_]+\.sql$/u,
    `${entry.tableName} must point to a numbered PostgreSQL migration under deployments/database/postgres/migrations`,
  );
  assert.ok(
    fs.existsSync(path.join(repoRoot, ...migrationPath.split('/'))),
    `${entry.tableName} migration file must exist: ${migrationPath}`,
  );
  const migrationSource = read(migrationPath).toLowerCase();
  assert.ok(
    migrationSource.includes(`create table`) && migrationSource.includes(entry.tableName),
    `${entry.tableName} must be created in its registered migration ${migrationPath}`,
  );
}

const migrationTables = extractAll(
  /\bcreate\s+table(?:\s+if\s+not\s+exists)?\s+([a-z_][a-z0-9_]*)\s*\(/giu,
  schema,
);
assert.ok(migrationTables.length > 0, 'core IM schema must define database tables');
for (const table of migrationTables) {
  assert.match(
    table,
    /^im_[a-z0-9]+(?:_[a-z0-9]+)*$/u,
    `instant messaging migration table ${table} must use the im_ prefix`,
  );
  assert.ok(
    registeredTables.includes(table),
    `instant messaging migration table ${table} must be listed in specs/database-table-registry.json`,
  );
  assert.ok(table.length <= 63, `${table} should fit PostgreSQL's default identifier length`);
}

for (const table of registeredTables) {
  assert.ok(
    migrationTables.includes(table),
    `registered IM table ${table} must exist in the canonical migration`,
  );
}
assert.ok(
  !migrationTables.includes('__manual_smoke_check'),
  'manual smoke check tables must not be created by checked-in migrations',
);
assert.ok(
  !registeredTables.includes('__manual_smoke_check'),
  'manual smoke check tables must not be registered as IM business tables',
);

for (const forbiddenPrefix of imPrefix.forbiddenAliases) {
  assert.ok(
    !migrationTables.some((table) => table.startsWith(`${forbiddenPrefix}_`)),
    `IM migration must not use product/project/generic prefix ${forbiddenPrefix}_`,
  );
}

const constraintNames = extractAll(/\bconstraint\s+([a-z_][a-z0-9_]*)\b/giu, schema);
for (const constraintName of constraintNames) {
  assert.match(
    constraintName,
    /^(pk|uk|fk|chk)_im_[a-z0-9]+(?:_[a-z0-9]+)*$/u,
    `IM schema constraint ${constraintName} must be visibly tied to im_`,
  );
  assert.ok(
    constraintName.length <= 63,
    `${constraintName} should fit PostgreSQL's default identifier length`,
  );
}

const indexNames = extractAll(
  /\bcreate\s+(?:unique\s+)?index\s+if\s+not\s+exists\s+([a-z_][a-z0-9_]*)\b/giu,
  schema,
);
for (const indexName of indexNames) {
  assert.match(
    indexName,
    /^(idx|uk)_im_[a-z0-9]+(?:_[a-z0-9]+)*$/u,
    `IM schema index ${indexName} must be visibly tied to im_`,
  );
  assert.ok(indexName.length <= 63, `${indexName} should fit PostgreSQL's default identifier length`);
}

const sqlContractFiles = [
  'crates/im-postgres-realtime-contracts/src/lib.rs',
  'adapters/postgres-realtime/src/lib.rs',
];
const imTableNameHints =
  /(?:conversation|message|realtime|presence|route|rtc|audit|notification|automation|projection|stream|commit|outbox|inbox|idempotency|subscription|checkpoint|fence)/u;
for (const relativePath of sqlContractFiles) {
  const source = read(relativePath).toLowerCase();
  const sqlSource = extractRustRawStrings(source).join('\n').toLowerCase();
  const referencedTables = extractAll(
    /\b(?:from|join|insert\s+into|update|delete\s+from)\s+([a-z_][a-z0-9_]*)\b/giu,
    sqlSource,
  );
  for (const table of referencedTables.filter((name) => imTableNameHints.test(name))) {
    assert.match(
      table,
      /^im_/u,
      `${relativePath} references IM-like table ${table}; IM storage tables must use im_`,
    );
    assert.ok(
      registeredTables.includes(table),
      `${relativePath} references ${table}, which must be registered`,
    );
  }
}

assert.ok(
  componentSpec.canonicalSpecs.some((entry) => entry.file === 'DATABASE_SPEC.md'),
  'component spec must reference the root DATABASE_SPEC authority',
);
assert.ok(
  componentSpec.localExtensionSpecs.some((entry) => entry.file === 'database-prefix-registry.json'),
  'component spec must expose the local database prefix registry',
);
assert.ok(
  componentSpec.localExtensionSpecs.some((entry) => entry.file === 'database-table-registry.json'),
  'component spec must expose the local database table registry',
);

for (const required of [
  'database-prefix-registry.json',
  'database-table-registry.json',
  'database-table-naming-standard.md',
  'im_',
  'non-IM',
]) {
  assert.ok(localSpecReadme.includes(required), `specs/README.md must mention ${required}`);
}

for (const required of [
  'database-prefix-registry.json',
  'database-table-registry.json',
  'im_',
  '~/.sdkwork/chat/data/chat.sqlite',
]) {
  assert.ok(namingDoc.includes(required), `database naming documentation must mention ${required}`);
}
assert.match(namingDoc, /non-im/iu, 'database naming documentation must mention non-IM scope');
assert.ok(
  namingDoc.includes('sdkwork_ai_dev.__manual_smoke_check'),
  'database naming documentation must document the manual smoke check exception',
);
assert.ok(
  ubuntuWslGuide.includes('CREATE TABLE IF NOT EXISTS sdkwork_ai_dev.__manual_smoke_check'),
  'Ubuntu/WSL guide may use a non-IM manual smoke check table for connectivity verification',
);
assert.ok(
  ubuntuWslGuide.includes('DROP TABLE sdkwork_ai_dev.__manual_smoke_check'),
  'manual smoke check table must be dropped in the same guide',
);

assert.match(
  databaseSpec,
  /运行时业务数.{0,4}INSERT.{0,4}`BIGINT id` MUST 由统一 ID Provider 显式生成并绑定写入/u,
  'root DATABASE_SPEC.md must require explicit generated IDs for runtime INSERTs',
);
assert.match(
  databaseSpec,
  /sdkwork_platform_id_service::SnowflakeIdGenerator/u,
  'root DATABASE_SPEC.md must name the shared Rust Snowflake generator authority',
);

assert.match(
  cargoManifest,
  /"crates\/sdkwork-im-runtime-id"/u,
  'sdkwork-im workspace must include the runtime ID capability crate',
);
assert.match(
  cargoManifest,
  /sdkwork_id\s*=\s*\{\s*path\s*=\s*"\.\.\/sdkwork-appbase\/crates\/sdkwork-platform-id-service"/u,
  'sdkwork-im must consume the appbase platform ID service Snowflake generator instead of a local fork',
);
assert.match(
  runtimeIdCrate,
  /use sdkwork_id::\{SnowflakeIdError, SnowflakeIdGenerator\};/u,
  'sdkwork-im runtime ID crate must use the appbase Snowflake generator',
);
assert.match(
  runtimeIdCrate,
  /pub const SDKWORK_IM_ID_NODE_ID_ENV/u,
  'sdkwork-im runtime ID generation must require an explicit node ID env key',
);
assert.match(
  runtimeIdCrate,
  /failure_handling:\s*"fail_closed_no_random_or_database_fallback"/u,
  'sdkwork-im runtime ID strategy must fail closed without random or database fallback IDs',
);

const appbaseReleaseDependency = sdkworkWorkflow.dependencies.find(
  (entry) => entry.id === 'sdkwork-appbase',
);
assert.ok(
  appbaseReleaseDependency,
  'sdkwork-im release workflow must declare the sdkwork-appbase dependency used by runtime IDs and IAM SDKs',
);
assert.equal(appbaseReleaseDependency.repository, 'Sdkwork-Cloud/sdkwork-appbase');
assert.equal(appbaseReleaseDependency.refInput, 'SDKWORK_APPBASE_REF');
assert.equal(appbaseReleaseDependency.tokenSecret, 'SDKWORK_RELEASE_TOKEN');
assert.match(
  appbaseReleaseDependency.ref,
  /^[0-9a-f]{40}$/u,
  'sdkwork-appbase release dependency must be pinned to a reproducible commit ref',
);

const appbaseReleaseSource = sharedSdkReleaseSources.sources['sdkwork-appbase'];
assert.ok(appbaseReleaseSource, 'shared SDK release sources must include sdkwork-appbase');
assert.equal(appbaseReleaseSource.repoUrl, 'https://github.com/Sdkwork-Cloud/sdkwork-appbase.git');
assert.ok(appbaseReleaseSource.ref, 'sdkwork-appbase shared SDK release source must declare a ref');

for (const requiredWorkspacePackage of [
  '../../../sdkwork-appbase/sdks/sdkwork-appbase-app-sdk/sdkwork-appbase-app-sdk-typescript/generated/server-openapi',
  '../../../sdkwork-appbase/sdks/sdkwork-appbase-backend-sdk/sdkwork-appbase-backend-sdk-typescript/generated/server-openapi',
  '../../../sdkwork-appbase/packages/common/iam/sdkwork-iam-contracts',
  '../../../sdkwork-appbase/packages/common/iam/sdkwork-iam-sdk-ports',
  '../../../sdkwork-appbase/packages/common/iam/sdkwork-iam-runtime',
  '../../../sdkwork-appbase/packages/pc-react/iam/sdkwork-auth-pc-react',
  '../../../sdkwork-appbase/packages/pc-react/iam/sdkwork-auth-runtime-pc-react',
]) {
  assert.ok(
    chatPcPnpmWorkspace.includes(`- ${requiredWorkspacePackage}`),
    `sdkwork-im-pc pnpm workspace must declare appbase source package ${requiredWorkspacePackage}`,
  );
}
assert.doesNotMatch(
  chatPcPnpmWorkspace,
  /\blink:/u,
  'sdkwork-im-pc must consume appbase source packages through pnpm workspace declarations, not link: aliases',
);

console.log('sdkwork-chat database naming standard contract passed');
