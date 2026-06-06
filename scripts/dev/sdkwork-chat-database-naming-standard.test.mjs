import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..', '..');

function read(relativePath) {
  return fs.readFileSync(path.join(repoRoot, ...relativePath.split('/')), 'utf8');
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

console.log('sdkwork-chat database naming standard contract passed');
