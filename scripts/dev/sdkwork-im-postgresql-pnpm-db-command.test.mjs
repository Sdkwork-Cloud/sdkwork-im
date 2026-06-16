import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const repoRoot = path.resolve(__dirname, '..', '..');
const workspaceRoot = path.resolve(repoRoot, '..');

function read(relativePath) {
  return fs.readFileSync(path.join(repoRoot, relativePath), 'utf8');
}

function readJson(relativePath) {
  return JSON.parse(read(relativePath));
}

const packageJson = readJson('package.json');
const envExample = read('.env.postgres.example');
const configIndex = read('docs/部署/postgresql-database-configuration.md');
const ubuntuWslGuide = read('docs/部署/Ubuntu与WSL-PostgreSQL初始化建库授权手册.md');
const devGuide = read('docs/部署/开发环境PostgreSQL数据库配置教程.md');
const productionGuide = read('docs/部署/线上环境PostgreSQL数据库配置教程.md');
const appbaseIamMigrationsDir = path.join(
  workspaceRoot,
  'sdkwork-appbase',
  'packages',
  'native-rust',
  'iam',
  'sdkwork-iam-storage-sqlx-rust',
  'migrations',
);
const appbaseIamMigrationFiles = fs
  .readdirSync(appbaseIamMigrationsDir)
  .filter((entry) => entry.endsWith('.sql'))
  .sort((left, right) => left.localeCompare(right));
const appbaseIamMigrations = appbaseIamMigrationFiles
  .map((entry) => fs.readFileSync(path.join(appbaseIamMigrationsDir, entry), 'utf8'))
  .join('\n');

assert.deepEqual(
  appbaseIamMigrationFiles,
  ['0001_iam_foundation.sql', '0002_drop_legacy_organization_member.sql'],
  'appbase IAM migration catalog must include foundation and legacy cleanup migrations',
);

assert.match(
  appbaseIamMigrations,
  /CREATE TABLE IF NOT EXISTS iam_organization_membership \(/u,
  'appbase IAM migration must create the canonical organization membership table',
);
assert.match(
  appbaseIamMigrations,
  /CREATE TABLE IF NOT EXISTS iam_tenant_member \(/u,
  'appbase IAM migration must create the canonical tenant member table',
);
assert.match(
  appbaseIamMigrations,
  /CREATE TABLE IF NOT EXISTS iam_tenant_signing_key \(/u,
  'appbase IAM migration must create tenant-bound signing key table',
);
for (const requiredSessionColumn of [
  'login_scope',
  'auth_token_kid',
  'access_token_kid',
  'refresh_token_kid',
]) {
  assert.ok(
    appbaseIamMigrations.includes(requiredSessionColumn),
    `appbase IAM session migration must include ${requiredSessionColumn}`,
  );
}
assert.doesNotMatch(
  appbaseIamMigrations,
  /CREATE TABLE IF NOT EXISTS iam_organization_member \(/u,
  'appbase IAM migration must not create the non-canonical iam_organization_member table',
);
assert.match(
  appbaseIamMigrations,
  /DROP TABLE IF EXISTS iam_organization_member/u,
  'appbase IAM migration must explicitly drop the non-canonical iam_organization_member table',
);

assert.equal(
  packageJson.scripts['db:postgres:plan'],
  'node scripts/dev/sdkwork-im-postgres-db.mjs --mode plan --config .env.postgres --dry-run',
  'root pnpm db:postgres:plan must print the audited PostgreSQL initialization and migration plan',
);
assert.equal(
  packageJson.scripts['db:postgres:init'],
  'node scripts/dev/sdkwork-im-postgres-db.mjs --mode init --config .env.postgres',
  'root pnpm db:postgres:init must initialize role/database/schema/grants from .env.postgres',
);
assert.equal(
  packageJson.scripts['db:postgres:migrate'],
  'node scripts/dev/sdkwork-im-postgres-db.mjs --mode migrate --config .env.postgres',
  'root pnpm db:postgres:migrate must run PostgreSQL migrations from .env.postgres',
);

const dbScriptPath = path.join(repoRoot, 'scripts/dev/sdkwork-im-postgres-db.mjs');
assert.ok(fs.existsSync(dbScriptPath), 'PostgreSQL pnpm database script must exist');

const {
  buildPostgresDatabaseUrl,
  createPostgresDbPlan,
  listActivePostgresMigrationBasenames,
  parsePostgresConfig,
  sanitizePostgresDatabaseUrl,
} = await import(pathToFileURL(dbScriptPath).href);

const expectedImMigrationLabels = listActivePostgresMigrationBasenames().map(
  (basename) => `apply PostgreSQL migration ${basename}`,
);

const parsedSplitConfig = parsePostgresConfig({
  configText: [
    'SDKWORK_IM_DATABASE_ENGINE=postgresql',
    'SDKWORK_IM_DATABASE_HOST=127.0.0.1',
    'SDKWORK_IM_DATABASE_PORT=15432',
    'SDKWORK_IM_DATABASE_NAME=sdkwork_ai_dev',
    'SDKWORK_IM_DATABASE_SCHEMA=sdkwork_ai_dev',
    'SDKWORK_IM_DATABASE_USERNAME=sdkwork_ai_dev',
    'SDKWORK_IM_DATABASE_PASSWORD=chat pass',
    'SDKWORK_IM_DATABASE_SSL_MODE=disable',
    'SDKWORK_IM_DATABASE_ADMIN_USERNAME=postgres',
    'SDKWORK_IM_DATABASE_ADMIN_PASSWORD=admin pass',
    'SDKWORK_IM_DATABASE_ADMIN_DATABASE=postgres',
    '',
  ].join('\n'),
  configPath: '.env.postgres',
});
assert.equal(parsedSplitConfig.database.host, '127.0.0.1');
assert.equal(parsedSplitConfig.database.port, '15432');
assert.equal(parsedSplitConfig.database.database, 'sdkwork_ai_dev');
assert.equal(parsedSplitConfig.database.schema, 'sdkwork_ai_dev');
assert.equal(parsedSplitConfig.database.username, 'sdkwork_ai_dev');
assert.equal(parsedSplitConfig.database.password, 'chat pass');
assert.equal(parsedSplitConfig.database.sslmode, 'disable');
assert.equal(parsedSplitConfig.admin.database, 'postgres');
assert.equal(parsedSplitConfig.admin.username, 'postgres');
assert.equal(parsedSplitConfig.admin.password, 'admin pass');

assert.equal(
  buildPostgresDatabaseUrl(parsedSplitConfig.database),
  'postgresql://sdkwork_ai_dev:chat%20pass@127.0.0.1:15432/sdkwork_ai_dev?sslmode=disable',
  'PostgreSQL DB script must assemble a usable app connection URL from split config fields',
);
assert.equal(
  sanitizePostgresDatabaseUrl('postgresql://sdkwork_ai_dev:chat%20pass@127.0.0.1:15432/sdkwork_ai_dev?sslmode=disable'),
  'postgresql://sdkwork_ai_dev:***@127.0.0.1:15432/sdkwork_ai_dev?sslmode=disable',
  'PostgreSQL DB script must redact connection-string passwords in logs and dry-run output',
);

const parsedUrlConfig = parsePostgresConfig({
  configText: [
    'SDKWORK_CLAW_DATABASE_URL=postgresql://url_user:url_pass@db.internal:5432/url_db?sslmode=require',
    'SDKWORK_CLAW_DATABASE_SCHEMA=url_schema',
    'SDKWORK_CLAW_DATABASE_ADMIN_URL=postgresql://postgres:admin_pass@db.internal:5432/postgres?sslmode=require',
    '',
  ].join('\n'),
  configPath: '.env.postgres',
});
assert.equal(parsedUrlConfig.database.host, 'db.internal');
assert.equal(parsedUrlConfig.database.database, 'url_db');
assert.equal(parsedUrlConfig.database.schema, 'url_schema');
assert.equal(parsedUrlConfig.database.username, 'url_user');
assert.equal(parsedUrlConfig.database.password, 'url_pass');
assert.equal(parsedUrlConfig.admin.username, 'postgres');
assert.equal(parsedUrlConfig.admin.password, 'admin_pass');

const parsedWslPsqlConfig = parsePostgresConfig({
  configText: [
    'SDKWORK_IM_DATABASE_ENGINE=postgresql',
    'SDKWORK_IM_DATABASE_HOST=127.0.0.1',
    'SDKWORK_IM_DATABASE_PORT=5432',
    'SDKWORK_IM_DATABASE_NAME=sdkwork_ai_dev',
    'SDKWORK_IM_DATABASE_SCHEMA=sdkwork_ai_dev',
    'SDKWORK_IM_DATABASE_USERNAME=sdkwork_ai_dev',
    'SDKWORK_IM_DATABASE_PASSWORD=sdkworkdev123',
    'SDKWORK_IM_DATABASE_SSL_MODE=disable',
    'SDKWORK_IM_DATABASE_ADMIN_USERNAME=postgres',
    'SDKWORK_IM_DATABASE_ADMIN_PASSWORD=admin pass',
    'SDKWORK_IM_DATABASE_PSQL_COMMAND=wsl.exe',
    'SDKWORK_IM_DATABASE_PSQL_ARGS=-d Ubuntu-22.04 -- psql',
    'SDKWORK_IM_DATABASE_PSQL_PATH_STYLE=wsl',
    '',
  ].join('\n'),
  configPath: '.env.postgres',
});
assert.equal(parsedWslPsqlConfig.psql.command, 'wsl.exe');
assert.deepEqual(parsedWslPsqlConfig.psql.args, ['-d', 'Ubuntu-22.04', '--', 'psql']);
assert.equal(parsedWslPsqlConfig.psql.pathStyle, 'wsl');

const parsedYamlConfig = parsePostgresConfig({
  configText: [
    'provider: postgresql',
    'connection:',
    '  host: 10.0.0.8',
    '  port: 5432',
    '  database: sdkwork_chat_prod',
    '  username: sdkwork_chat_prod',
    '  password: yaml_pass',
    '  sslmode: require',
    'schema:',
    '  name: sdkwork_chat_prod',
    '',
  ].join('\n'),
  configPath: 'postgresql.yaml',
});
assert.equal(parsedYamlConfig.database.host, '10.0.0.8');
assert.equal(parsedYamlConfig.database.database, 'sdkwork_chat_prod');
assert.equal(parsedYamlConfig.database.schema, 'sdkwork_chat_prod');
assert.equal(parsedYamlConfig.database.username, 'sdkwork_chat_prod');
assert.equal(parsedYamlConfig.database.password, 'yaml_pass');
assert.equal(parsedYamlConfig.database.sslmode, 'require');

assert.throws(
  () => parsePostgresConfig({
    configText: [
      'SDKWORK_CLAW_DATABASE_PROVIDER=postgresql',
      'SDKWORK_CLAW_DATABASE_HOST=127.0.0.1',
      'SDKWORK_CLAW_DATABASE_NAME=sdkwork_ai_dev',
      'SDKWORK_CLAW_DATABASE_USERNAME=sdkwork_ai_dev',
      '',
    ].join('\n'),
    configPath: '.env.postgres',
  }),
  /SDKWORK_IM_DATABASE_PASSWORD/u,
  'PostgreSQL DB script must reject incomplete app connection config before invoking psql',
);

const initPlan = createPostgresDbPlan({
  config: parsedSplitConfig,
  mode: 'init',
  repoRoot,
});
assert.deepEqual(
  initPlan.steps.map((step) => step.label),
  ['initialize PostgreSQL role and database', 'initialize PostgreSQL schema and grants'],
  'init mode must provision role/database before target schema/grants',
);
assert.ok(
  initPlan.steps.every((step) => step.command === 'psql'),
  'database init must use psql so the same pnpm command works on Ubuntu, WSL, and Windows with PostgreSQL tools installed',
);
assert.ok(
  initPlan.steps.every((step) => step.env.PGPASSWORD && !step.args.join(' ').includes(step.env.PGPASSWORD)),
  'database init must pass passwords through PGPASSWORD instead of command-line arguments',
);
assert.match(
  initPlan.steps[0].input,
  /CREATE ROLE.*LOGIN PASSWORD|ALTER ROLE.*WITH LOGIN PASSWORD/su,
  'database init must create or update the app login role',
);
assert.match(
  initPlan.steps[0].input,
  /CREATE DATABASE/su,
  'database init must create the target database when it does not exist',
);
assert.match(
  initPlan.steps[1].input,
  /CREATE SCHEMA IF NOT EXISTS.*AUTHORIZATION/su,
  'database init must create the configured schema with app ownership',
);
assert.match(
  initPlan.steps[1].input,
  /ALTER DEFAULT PRIVILEGES/su,
  'database init must configure default privileges for future tables, sequences, and functions',
);
assert.doesNotMatch(
  JSON.stringify(initPlan),
  /chat pass|admin pass/u,
  'audited database plans must not leak raw passwords when serialized',
);
assert.throws(
  () => createPostgresDbPlan({
    config: {
      ...parsedSplitConfig,
      admin: {
        ...parsedSplitConfig.admin,
        password: undefined,
      },
    },
    mode: 'init',
    repoRoot,
  }),
  /SDKWORK_IM_DATABASE_ADMIN_PASSWORD|SDKWORK_IM_DATABASE_ADMIN_URL/u,
  'database init must fail before invoking psql when the admin password is missing',
);

const migratePlan = createPostgresDbPlan({
  config: parsedSplitConfig,
  mode: 'migrate',
  repoRoot,
});
assert.deepEqual(
  migratePlan.steps.map((step) => step.label).filter((label) => label.startsWith('apply PostgreSQL migration')),
  expectedImMigrationLabels,
  'migrate mode must apply all active IM PostgreSQL migrations in lexical order',
);
assert.ok(
  migratePlan.steps[0].args.includes('-f')
    && migratePlan.steps[0].args.some((arg) => arg.endsWith('deployments/database/postgres/migrations/001_im_core_schema.sql')),
  'migrate mode must start with the bootstrap PostgreSQL migration SQL file',
);
const iamMigrationStepOffset = expectedImMigrationLabels.length;
assert.ok(
  migratePlan.steps[iamMigrationStepOffset].args.includes('-f')
    && migratePlan.steps[iamMigrationStepOffset].args.some((arg) => arg.endsWith('sdkwork-appbase/packages/native-rust/iam/sdkwork-iam-storage-sqlx-rust/migrations/0001_iam_foundation.sql')),
  'migrate mode must execute the appbase IAM PostgreSQL migration SQL file so iam_organization_membership exists',
);
assert.ok(
  migratePlan.steps[iamMigrationStepOffset + 1].args.includes('-f')
    && migratePlan.steps[iamMigrationStepOffset + 1].args.some((arg) => arg.endsWith('sdkwork-appbase/packages/native-rust/iam/sdkwork-iam-storage-sqlx-rust/migrations/0002_drop_legacy_organization_member.sql')),
  'migrate mode must execute the appbase IAM cleanup migration SQL file so iam_organization_member is removed',
);
assert.ok(
  migratePlan.steps[0].args.includes('--set')
    && migratePlan.steps[0].args.includes('search_path=sdkwork_ai_dev, public'),
  'migrate mode must set the configured schema search_path before running migration SQL',
);
assert.ok(
  migratePlan.steps[iamMigrationStepOffset].args.includes('--set')
    && migratePlan.steps[iamMigrationStepOffset].args.includes('search_path=sdkwork_ai_dev, public'),
  'appbase IAM migration must use the configured schema search_path before running migration SQL',
);
assert.ok(
  migratePlan.steps[iamMigrationStepOffset + 1].args.includes('--set')
    && migratePlan.steps[iamMigrationStepOffset + 1].args.includes('search_path=sdkwork_ai_dev, public'),
  'appbase IAM cleanup migration must use the configured schema search_path before running migration SQL',
);
assert.equal(migratePlan.steps[0].env.PGPASSWORD, '***', 'serialized migration plan must redact app password');
assert.equal(migratePlan.steps[iamMigrationStepOffset].env.PGPASSWORD, '***', 'serialized appbase IAM migration plan must redact app password');
assert.equal(migratePlan.steps[iamMigrationStepOffset + 1].env.PGPASSWORD, '***', 'serialized appbase IAM cleanup migration plan must redact app password');

const wslMigratePlan = createPostgresDbPlan({
  config: parsedWslPsqlConfig,
  mode: 'migrate',
  repoRoot,
});
assert.equal(wslMigratePlan.steps[0].command, 'wsl.exe');
assert.deepEqual(
  wslMigratePlan.steps[0].args.slice(0, 4),
  ['-d', 'Ubuntu-22.04', '--', 'psql'],
  'Windows developers must be able to run pnpm db:postgres:* through WSL psql when psql is not installed on Windows',
);
assert.ok(
  !wslMigratePlan.steps[0].args.at(-1).includes(':'),
  'WSL psql migration file arguments must not use Windows drive-letter paths',
);
assert.match(
  wslMigratePlan.steps[0].args.at(-1),
  /\/mnt\/[a-z]\//u,
  'WSL psql migration file arguments must use /mnt/<drive>/ paths',
);
assert.match(
  wslMigratePlan.steps[2].args.at(-1),
  /\/mnt\/[a-z]\//u,
  'WSL psql appbase IAM migration file arguments must use /mnt/<drive>/ paths',
);
assert.equal(
  wslMigratePlan.steps[0].env.WSLENV,
  'PGPASSWORD/u:PGSSLMODE/u',
  'WSL psql steps must explicitly bridge PGPASSWORD and PGSSLMODE into the Linux environment',
);
assert.ok(
  !wslMigratePlan.steps[0].args.join(' ').includes('sdkworkdev123'),
  'WSL psql steps must not leak raw passwords into command-line arguments',
);
assert.equal(
  wslMigratePlan.steps[0].shell,
  false,
  'WSL psql steps must bypass Windows shell quoting so psql arguments containing spaces stay intact',
);

const fullPlan = createPostgresDbPlan({
  config: parsedSplitConfig,
  mode: 'plan',
  repoRoot,
});
assert.deepEqual(
  fullPlan.steps.map((step) => step.label),
  [
    'initialize PostgreSQL role and database',
    'initialize PostgreSQL schema and grants',
    ...expectedImMigrationLabels,
    'apply appbase IAM PostgreSQL migration 0001_iam_foundation.sql',
    'apply appbase IAM PostgreSQL migration 0002_drop_legacy_organization_member.sql',
  ],
  'plan mode must show initialization, all active IM migrations, and appbase IAM migration actions',
);

for (const required of [
  'SDKWORK_IM_DATABASE_ENGINE=postgresql',
  'SDKWORK_IM_DATABASE_HOST=127.0.0.1',
  'SDKWORK_IM_DATABASE_NAME=sdkwork_ai_dev',
  'SDKWORK_IM_DATABASE_SCHEMA=sdkwork_ai_dev',
  'SDKWORK_IM_DATABASE_USERNAME=sdkwork_ai_dev',
  'SDKWORK_IM_DATABASE_PASSWORD=sdkworkdev123',
  'SDKWORK_IM_DATABASE_SSL_MODE=disable',
  'SDKWORK_IM_DATABASE_ADMIN_USERNAME=postgres',
  'SDKWORK_IM_DATABASE_ADMIN_PASSWORD=postgres_admin_pass',
  'SDKWORK_IM_DATABASE_ADMIN_DATABASE=postgres',
  'SDKWORK_IM_DATABASE_ADMIN_SSL_MODE=disable',
]) {
  assert.ok(envExample.includes(required), `.env.postgres.example must document ${required}`);
}

for (const doc of [configIndex, ubuntuWslGuide, devGuide]) {
  for (const required of [
    'pnpm db:postgres:plan',
    'pnpm db:postgres:init',
    'pnpm db:postgres:migrate',
    'SDKWORK_IM_DATABASE_ADMIN_PASSWORD',
    'SDKWORK_IM_DATABASE_SCHEMA',
  ]) {
    assert.ok(doc.includes(required), `PostgreSQL docs must include ${required}`);
  }
}

for (const required of [
  'database: sdkwork_chat_prod',
  'schema: sdkwork_chat_prod',
  'username: sdkwork_chat_prod',
  'CREATE ROLE "sdkwork_chat_prod" LOGIN PASSWORD',
  'CREATE DATABASE sdkwork_chat_prod OWNER "sdkwork_chat_prod"',
  'CREATE SCHEMA IF NOT EXISTS sdkwork_chat_prod AUTHORIZATION "sdkwork_chat_prod"',
  'postgresql://sdkwork_chat_prod@postgres.internal.example.com:5432/sdkwork_chat_prod?sslmode=require',
]) {
  assert.ok(productionGuide.includes(required), `production PostgreSQL guide must include ${required}`);
}

console.log('sdkwork-im PostgreSQL pnpm database command contract passed');
