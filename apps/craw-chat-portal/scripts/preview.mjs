import { startPreviewServer } from './lib/preview-server.mjs';

const args = process.argv.slice(2);

function readArg(name, fallback) {
  const index = args.indexOf(name);
  return index >= 0 ? args[index + 1] : fallback;
}

const root = readArg('--root', '.');
const port = Number(readArg('--port', '4176'));

const { port: listeningPort } = await startPreviewServer({ root, port });
process.stdout.write(`craw-chat-portal preview listening on ${listeningPort}\n`);
