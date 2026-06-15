import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import ts from 'typescript';

const indexText = readFileSync(
  './packages/sdkwork-im-pc-chat/src/index.ts',
  'utf8',
);
const defaultsText = readFileSync(
  './packages/sdkwork-im-pc-chat/src/components/AgentDefaults.ts',
  'utf8',
);

const indexSource = ts.createSourceFile(
  'index.ts',
  indexText,
  ts.ScriptTarget.Latest,
  true,
  ts.ScriptKind.TS,
);

function exportsDefaultAgentConfig(): boolean {
  let found = false;
  const visit = (node: ts.Node): void => {
    if (
      ts.isExportDeclaration(node) &&
      node.exportClause &&
      ts.isNamedExports(node.exportClause) &&
      node.moduleSpecifier &&
      ts.isStringLiteral(node.moduleSpecifier) &&
      node.moduleSpecifier.text === './components/AgentDefaults' &&
      node.exportClause.elements.some((element) => element.name.text === 'DEFAULT_AGENT_CONFIG')
    ) {
      found = true;
      return;
    }
    ts.forEachChild(node, visit);
  };
  visit(indexSource);
  return found;
}

assert.ok(
  defaultsText.includes('DEFAULT_AGENT_CONFIG'),
  'AgentDefaults must own the shared default management profile config',
);
assert.ok(
  exportsDefaultAgentConfig(),
  'chat package public entrypoint must export DEFAULT_AGENT_CONFIG so every agent creation entry uses one default management profile',
);

console.log('sdkwork im pc agent default config export contract passed.');
