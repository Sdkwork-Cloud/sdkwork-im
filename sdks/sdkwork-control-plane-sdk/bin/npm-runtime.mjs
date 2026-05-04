#!/usr/bin/env node
import { existsSync } from 'node:fs';
import path from 'node:path';

function toAbsoluteCandidate(candidate) {
  if (!candidate) {
    return '';
  }
  return path.resolve(candidate);
}

function isJavaScriptCliPath(candidate) {
  return /\.m?js$/i.test(candidate);
}

function resolveJavaScriptCliInvocation(processExecPath, candidate, exists) {
  const absoluteCandidate = toAbsoluteCandidate(candidate);
  if (!absoluteCandidate || !exists(absoluteCandidate)) {
    return null;
  }

  return {
    command: processExecPath,
    argsPrefix: [absoluteCandidate],
  };
}

function resolveExecutableInvocation(candidate, exists) {
  const absoluteCandidate = toAbsoluteCandidate(candidate);
  if (!absoluteCandidate || !exists(absoluteCandidate)) {
    return null;
  }

  return {
    command: absoluteCandidate,
    argsPrefix: [],
  };
}

export function resolveNpmInvocation({
  processExecPath = process.execPath,
  platform = process.platform,
  env = process.env,
  exists = existsSync,
} = {}) {
  const nodeDir = path.dirname(processExecPath);
  const explicitCandidates = [
    env.SDKWORK_NPM_CLI,
    env.NPM_CLI_JS,
    env.npm_execpath,
  ].filter(Boolean);

  for (const candidate of explicitCandidates) {
    if (isJavaScriptCliPath(candidate)) {
      const invocation = resolveJavaScriptCliInvocation(processExecPath, candidate, exists);
      if (invocation) {
        return invocation;
      }
      continue;
    }

    const invocation = resolveExecutableInvocation(candidate, exists);
    if (invocation) {
      return invocation;
    }
  }

  for (const candidate of [
    path.join(nodeDir, 'node_modules', 'npm', 'bin', 'npm-cli.js'),
    path.join(nodeDir, '..', 'lib', 'node_modules', 'npm', 'bin', 'npm-cli.js'),
    path.join(nodeDir, '..', 'node_modules', 'npm', 'bin', 'npm-cli.js'),
  ]) {
    const invocation = resolveJavaScriptCliInvocation(processExecPath, candidate, exists);
    if (invocation) {
      return invocation;
    }
  }

  const executableCandidates = platform === 'win32'
    ? [
      path.join(nodeDir, 'npm.cmd'),
      path.join(nodeDir, 'npm.exe'),
    ]
    : [path.join(nodeDir, 'npm')];

  for (const candidate of executableCandidates) {
    const invocation = resolveExecutableInvocation(candidate, exists);
    if (invocation) {
      return invocation;
    }
  }

  return {
    command: 'npm',
    argsPrefix: [],
  };
}

export function createNpmCommandArgs(args, options = {}) {
  const invocation = resolveNpmInvocation(options);
  return {
    command: invocation.command,
    args: [...invocation.argsPrefix, ...args],
  };
}
