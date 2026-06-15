import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const repoRoot = path.resolve(__dirname, '..', '..');

function read(relativePath) {
  return fs.readFileSync(path.join(repoRoot, relativePath), 'utf8');
}

const guidePath = 'docs/部署/Ubuntu与WSL-PostgreSQL初始化建库授权手册.md';
const indexPath = 'docs/部署/postgresql-database-configuration.md';

assert.ok(fs.existsSync(path.join(repoRoot, guidePath)), 'Ubuntu/WSL PostgreSQL initialization guide must exist');

const guide = read(guidePath);
const index = read(indexPath);

for (const required of [
  'sudo apt update',
  'sudo apt install -y postgresql postgresql-contrib',
  'sudo systemctl enable --now postgresql',
  'sudo apt install -y redis-server',
  'sudo service redis-server start',
  'redis-cli ping',
  'PONG',
  'sudo -u postgres psql',
  'CREATE DATABASE sdkwork_ai_dev OWNER sdkwork_ai_dev',
  'CREATE SCHEMA IF NOT EXISTS sdkwork_ai_dev AUTHORIZATION sdkwork_ai_dev',
  'ALTER DEFAULT PRIVILEGES',
  'deployments/database/postgres/migrations/001_im_core_schema.sql',
  'psql -h 127.0.0.1 -p 5432 -U sdkwork_ai_dev -d sdkwork_ai_dev',
  'listen_addresses',
  'pg_hba.conf',
  '192.168.1.0/24',
  'sudo ufw allow from 192.168.1.0/24 to any port 5432 proto tcp',
  'WSL2 NAT',
  'mirrored',
  'netsh interface portproxy',
  'GRANT CONNECT ON DATABASE sdkwork_ai_dev TO sdkwork_ai_dev',
  'GRANT USAGE, CREATE ON SCHEMA sdkwork_ai_dev TO sdkwork_ai_dev',
  'SELECT current_database(), current_user, current_schema()',
  'Windows 跑应用，PostgreSQL 跑在 WSL Ubuntu',
  'Test-NetConnection 127.0.0.1 -Port 5432',
  'wsl hostname -I',
  'SDKWORK_IM_DATABASE_HOST=127.0.0.1',
  'SDKWORK_IM_DATABASE_ENGINE=postgresql',
  'SDKWORK_IM_DATABASE_SSL_MODE=disable',
  'SDKWORK_IM_DATABASE_SCHEMA=sdkwork_ai_dev',
  'SDKWORK_IM_REDIS_HOST=127.0.0.1',
  'SDKWORK_IM_REDIS_PORT=6379',
  'SDKWORK_IM_DATABASE_ADMIN_PASSWORD',
  'pnpm db:postgres:plan',
  'pnpm db:postgres:init',
  'pnpm db:postgres:migrate',
  'pnpm dev',
  'pnpm tauri:dev',
  'pnpm dev:sqlite',
  'pnpm tauri:dev:sqlite',
  'pnpm tauri:dev 默认使用 SQLite',
  'host    sdkwork_ai_dev    sdkwork_ai_dev    127.0.0.1/32',
  'host    sdkwork_ai_dev    postgres          127.0.0.1/32',
  'host    sdkwork_ai_dev    sdkwork_ai_dev    <WINDOWS_HOST_CIDR>',
  'host    sdkwork_ai_dev    postgres          <WINDOWS_HOST_CIDR>',
  'database: sdkwork_chat_prod',
  'username: sdkwork_chat_prod',
  'Windows 应用不应该使用 WSL 内部的 Unix socket',
]) {
  assert.ok(guide.includes(required), `Ubuntu/WSL PostgreSQL guide must include: ${required}`);
}

assert.ok(
  guide.includes('scram-sha-256') || guide.includes('md5'),
  'Ubuntu/WSL PostgreSQL guide must document pg_hba authentication method',
);

assert.ok(
  index.includes('./Ubuntu与WSL-PostgreSQL初始化建库授权手册.md'),
  'PostgreSQL configuration index must link the Ubuntu/WSL initialization guide',
);

console.log('sdkwork-im PostgreSQL Ubuntu/WSL guide contract passed');
