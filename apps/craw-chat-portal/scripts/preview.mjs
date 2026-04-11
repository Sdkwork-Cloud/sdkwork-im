import { createReadStream } from 'node:fs';
import { access } from 'node:fs/promises';
import http from 'node:http';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const scriptDir = path.dirname(fileURLToPath(import.meta.url));
const appRoot = path.resolve(scriptDir, '..');
const args = process.argv.slice(2);

function readArg(name, fallback) {
  const index = args.indexOf(name);
  return index >= 0 ? args[index + 1] : fallback;
}

const root = path.resolve(appRoot, readArg('--root', '.'));
const port = Number(readArg('--port', '4176'));

const contentTypes = {
  '.css': 'text/css; charset=utf-8',
  '.html': 'text/html; charset=utf-8',
  '.js': 'text/javascript; charset=utf-8',
  '.json': 'application/json; charset=utf-8',
};

function resolveContentType(filePath) {
  return contentTypes[path.extname(filePath)] ?? 'text/plain; charset=utf-8';
}

async function exists(filePath) {
  try {
    await access(filePath);
    return true;
  } catch {
    return false;
  }
}

const server = http.createServer(async (request, response) => {
  const url = new URL(request.url ?? '/', `http://127.0.0.1:${port}`);
  const target = url.pathname === '/' ? '/index.html' : url.pathname;
  const candidate = path.join(root, target);
  const safeCandidate = candidate.startsWith(root) ? candidate : path.join(root, 'index.html');
  const filePath = (await exists(safeCandidate)) ? safeCandidate : path.join(root, 'index.html');

  response.setHeader('Content-Type', resolveContentType(filePath));
  createReadStream(filePath).pipe(response);
});

server.listen(port, '0.0.0.0', () => {
  process.stdout.write(`craw-chat-portal preview listening on ${port}\n`);
});
