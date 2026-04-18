import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';

function read(relativePath) {
  return readFileSync(new URL(`../${relativePath}`, import.meta.url), 'utf8');
}

const sharedVerifierSource = read('bin/verify-language-workspace-shared.mjs');
const typescriptVerifierSource = read('bin/verify-typescript-workspace.mjs');
const flutterVerifierSource = read('bin/verify-flutter-workspace.mjs');

assert.match(
  sharedVerifierSource,
  /consumerPackage/,
  'Shared language workspace verifier must support consumerPackage validation.',
);
assert.match(
  sharedVerifierSource,
  /expectedPackageLayers|requiredPackageLayers/,
  'Shared language workspace verifier must support explicit package-layer validation.',
);

assert.match(
  typescriptVerifierSource,
  /verifyLanguageWorkspace/,
  'TypeScript workspace verifier must reuse shared workspace assembly validation.',
);
assert.match(
  typescriptVerifierSource,
  /consumerPackage:[\s\S]*@sdkwork\/craw-chat-sdk/,
  'TypeScript workspace verifier must validate @sdkwork/craw-chat-sdk as the consumer package.',
);
assert.match(
  typescriptVerifierSource,
  /requiredPackageLayers:[\s\S]*root/,
  'TypeScript workspace verifier must validate the assembled root package layer.',
);

assert.match(
  flutterVerifierSource,
  /verifyLanguageWorkspace/,
  'Flutter workspace verifier must reuse shared workspace assembly validation.',
);
assert.match(
  flutterVerifierSource,
  /consumerPackage:[\s\S]*craw_chat_sdk/,
  'Flutter workspace verifier must validate craw_chat_sdk as the consumer package.',
);

console.log('workspace verifier contract test passed');
