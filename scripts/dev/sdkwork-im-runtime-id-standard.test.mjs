import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const repoRoot = path.resolve(__dirname, '..', '..');

function readText(...segments) {
  return fs.readFileSync(path.join(repoRoot, ...segments), 'utf8').replace(/\r\n/gu, '\n');
}

const rootCargo = readText('Cargo.toml');
assert.match(
  rootCargo,
  /sdkwork_id\s*=\s*\{\s*path\s*=\s*"\.\.\/sdkwork-appbase\/crates\/sdkwork-platform-id-service"/u,
  'Rust runtime ID generation must reuse sdkwork-appbase sdkwork-platform-id-service via the sdkwork_id workspace alias.',
);
assert.match(
  rootCargo,
  /"crates\/sdkwork-im-runtime-id"/u,
  'sdkwork-im-runtime-id must be a workspace crate so database insert code can use the standard runtime ID generator.',
);

const runtimeIdSource = readText('crates', 'sdkwork-im-runtime-id', 'src', 'lib.rs');
for (const expectedText of [
  'use sdkwork_id::{SnowflakeIdError, SnowflakeIdGenerator};',
  'pub const SDKWORK_IM_ID_NODE_ID_ENV: &str = "SDKWORK_IM_ID_NODE_ID";',
  'clock_rollback: "reject_and_alert"',
  'node_conflict: "explicit_unique_node_id_required"',
  'failure_handling: "fail_closed_no_random_or_database_fallback"',
]) {
  assert.match(
    runtimeIdSource,
    new RegExp(expectedText.replace(/[.*+?^${}()|[\]\\]/gu, '\\$&'), 'u'),
    `runtime ID source must contain standard fragment: ${expectedText}`,
  );
}
assert.doesNotMatch(
  runtimeIdSource,
  /rand::|random::<|Uuid::|uuid::|bigserial|nextval|max\s*\(\s*id\s*\)/iu,
  'runtime ID generation must not implement random IDs, UUID-derived numeric IDs, database sequences, or MAX(id)+1.',
);

for (const relativePath of [
  'deployments/templates/server.env.example',
  'deployments/templates/quickstart-server-compose.env.example',
]) {
  const template = readText(...relativePath.split('/'));
  assert.match(
    template,
    /^SDKWORK_IM_ID_NODE_ID=\d+$/mu,
    `${relativePath} must declare an explicit Snowflake node id for distributed deployments.`,
  );
}

const releasePlanner = readText('scripts', 'release', 'plan-sdkwork-im-install-packages.mjs');
assert.match(
  releasePlanner,
  /'SDKWORK_IM_ID_NODE_ID'/u,
  'release package database/runtime env override policy must preserve SDKWORK_IM_ID_NODE_ID.',
);

const postgresMigration = readText('deployments', 'database', 'postgres', 'migrations', '001_im_core_schema.sql');
assert.doesNotMatch(
  postgresMigration,
  /\b(?:bigserial|serial)\b|generated\s+.*\s+identity|\bnextval\s*\(/iu,
  'runtime PostgreSQL schema must not allocate runtime business IDs through database auto-increment primitives.',
);
