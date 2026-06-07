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
const schema = read('deployments/database/postgres/migrations/001_im_core_schema.sql').toLowerCase();
const databaseSpec = readWorkspace('sdkwork-specs/DATABASE_SPEC.md');
const cargoManifest = read('Cargo.toml');
const runtimeIdCrate = read('crates/craw-chat-runtime-id/src/lib.rs');
const appbaseUserCenterCargo = readWorkspace(
  'sdkwork-appbase/packages/pc-react/iam/sdkwork-user-center-core-pc-react/native/tauri-rust/Cargo.toml',
);
const appbaseUserCenterLib = readWorkspace(
  'sdkwork-appbase/packages/pc-react/iam/sdkwork-user-center-core-pc-react/native/tauri-rust/src/lib.rs',
);
const appbaseUserCenterAuthority = readWorkspace(
  'sdkwork-appbase/packages/pc-react/iam/sdkwork-user-center-core-pc-react/native/tauri-rust/src/user_center_authority.rs',
);
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
  assert.equal(
    entry.boundedContext,
    'instant_messaging',
    `${entry.tableName} must belong to the instant_messaging bounded context`,
  );
  assert.match(entry.tableName, /^im_[a-z0-9]+(?:_[a-z0-9]+)*$/u);
  assert.ok(entry.tableProfile, `${entry.tableName} must declare a table profile`);
  assert.ok(entry.writeOwner, `${entry.tableName} must declare a write owner`);
  assert.equal(
    entry.migration,
    'deployments/database/postgres/migrations/001_im_core_schema.sql',
    `${entry.tableName} must point to the canonical IM PostgreSQL migration`,
  );
}

const migrationTables = extractAll(
  /\bcreate\s+table\s+if\s+not\s+exists\s+([a-z_][a-z0-9_]*)\s*\(/giu,
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
  /运行时业务数据 INSERT 的 `BIGINT id` MUST 由统一 ID 生成器显式生成并绑定写入/u,
  'root DATABASE_SPEC.md must require explicit generated IDs for runtime INSERTs',
);
assert.match(
  databaseSpec,
  /sdkwork_id::SnowflakeIdGenerator/u,
  'root DATABASE_SPEC.md must name the shared Rust Snowflake generator authority',
);

assert.match(
  cargoManifest,
  /"crates\/craw-chat-runtime-id"/u,
  'craw-chat workspace must include the runtime ID capability crate',
);
assert.match(
  cargoManifest,
  /sdkwork_id\s*=\s*\{\s*path\s*=\s*"\.\.\/sdkwork-appbase\/packages\/native-rust\/foundation\/sdkwork-id-rust"\s*\}/u,
  'craw-chat must consume the appbase sdkwork_id Snowflake generator instead of a local fork',
);
assert.match(
  runtimeIdCrate,
  /use sdkwork_id::\{SnowflakeIdError, SnowflakeIdGenerator\};/u,
  'craw-chat runtime ID crate must use the appbase Snowflake generator',
);
assert.match(
  runtimeIdCrate,
  /pub const SDKWORK_CHAT_ID_NODE_ID_ENV/u,
  'craw-chat runtime ID generation must require an explicit node ID env key',
);
assert.match(
  runtimeIdCrate,
  /failure_handling:\s*"fail_closed_no_random_or_database_fallback"/u,
  'craw-chat runtime ID strategy must fail closed without random or database fallback IDs',
);

assert.match(
  appbaseUserCenterCargo,
  /sdkwork_id\.workspace\s*=\s*true/u,
  'appbase user-center native crate must depend on the shared workspace Snowflake generator',
);
assert.match(
  appbaseUserCenterLib,
  /use sdkwork_id::\{SnowflakeIdError, SnowflakeIdGenerator\};/u,
  'appbase user-center create_identifier must use sdkwork_id::SnowflakeIdGenerator',
);
assert.match(
  appbaseUserCenterLib,
  /static USER_CENTER_ID_GENERATOR:\s*OnceLock<SnowflakeIdGenerator>/u,
  'appbase user-center must keep a process-local Snowflake generator instead of rebuilding per ID',
);
assert.doesNotMatch(
  appbaseUserCenterLib,
  /DECIMAL_IDENTIFIER_BASE|NEXT_DECIMAL_IDENTIFIER|AtomicU64|saturating_mul\(1_000\)/u,
  'appbase user-center must not use the old decimal timestamp ID generator',
);
assert.doesNotMatch(
  appbaseUserCenterAuthority,
  /fn stable_numeric_identifier/u,
  'runtime appbase IAM write IDs must not use stable hash-derived numeric identifiers',
);
assert.match(
  appbaseUserCenterAuthority,
  /fn resolve_existing_external_user/u,
  'appbase user-center must resolve existing users by external subject before allocating a new internal ID',
);
assert.match(
  appbaseUserCenterAuthority,
  /let external_subject = normalize_optional_text\(request\.subject\.as_deref\(\)\)\s*\.or_else\(\|\| normalize_optional_text\(request\.user_id\.as_deref\(\)\)\);/u,
  'appbase session exchange must treat request.user_id as external_subject fallback',
);
assert.doesNotMatch(
  appbaseUserCenterAuthority,
  /let preferred_user_id\s*=\s*normalize_optional_text\(request\.user_id\.as_deref\(\)\)/u,
  'appbase session exchange must not use external request.user_id as the internal iam_user.id',
);

for (const [builderName, ignoredArgument] of [
  ['build_local_user_id', 'email'],
  ['build_local_phone_user_id', 'phone'],
  ['build_external_user_id', 'provider_key'],
  ['build_local_oauth_user_id', 'provider'],
]) {
  const builderMatch = appbaseUserCenterAuthority.match(
    new RegExp(`fn ${builderName}\\([^)]*\\) -> String \\{([\\s\\S]*?)\\n\\}`, 'u'),
  );
  assert.ok(builderMatch, `appbase user-center must define ${builderName}`);
  assert.match(
    builderMatch[1],
    /crate::create_identifier\("user"\)/u,
    `${builderName} must allocate internal IAM user IDs through create_identifier("user")`,
  );
  assert.doesNotMatch(
    builderMatch[1],
    new RegExp(`sanitize_identifier_segment\\(${ignoredArgument}\\)|format!\\(\\s*"user[_-]`, 'u'),
    `${builderName} must not derive internal user IDs from account/email/provider text`,
  );
}

console.log('sdkwork-chat database naming standard contract passed');
