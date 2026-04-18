import { isAdminSandboxEnabled } from './admin-sandbox.mjs';

const FORBIDDEN_ADMIN_RELEASE_MARKERS = Object.freeze([
  {
    description: 'default sandbox admin password',
    pattern: /ChangeMe123/g,
  },
  {
    description: 'sandbox admin email',
    pattern: /admin@sdkwork\.local/g,
  },
  {
    description: 'sandbox admin session token seed',
    pattern: /sandbox-admin-session/g,
  },
  {
    description: 'sandbox seed marker',
    pattern: /admin-sandbox-seed/g,
  },
]);

function bundleEntryToText(entry) {
  if (!entry) {
    return '';
  }

  if (entry.type === 'chunk') {
    return typeof entry.code === 'string' ? entry.code : '';
  }

  if (typeof entry.source === 'string') {
    return entry.source;
  }

  if (entry.source instanceof Uint8Array) {
    return Buffer.from(entry.source).toString('utf8');
  }

  return '';
}

export function assertAdminReleaseSafety({ command, env = process.env } = {}) {
  if (command !== 'build') {
    return;
  }

  if (!isAdminSandboxEnabled(env)) {
    return;
  }

  throw new Error(
    'SDKWORK_ADMIN_SANDBOX must be disabled for production builds. Release artifacts cannot be built from the in-memory sandbox backend.',
  );
}

export function collectAdminBundleContentViolations(bundle = {}) {
  const violations = [];

  for (const [fileName, bundleEntry] of Object.entries(bundle)) {
    const source = bundleEntryToText(bundleEntry);

    if (!source) {
      continue;
    }

    for (const marker of FORBIDDEN_ADMIN_RELEASE_MARKERS) {
      const matches = source.match(marker.pattern);

      if (!matches) {
        continue;
      }

      violations.push({
        description: marker.description,
        fileName,
        match: matches[0],
      });
    }
  }

  return violations;
}

export function assertAdminBundleContentSafe(bundle = {}) {
  const violations = collectAdminBundleContentViolations(bundle);

  if (violations.length === 0) {
    return;
  }

  const formattedViolations = violations
    .map(({ fileName, description, match }) => `${fileName}: ${description} (${match})`)
    .join('; ');

  throw new Error(
    `Admin release bundle contains forbidden sandbox/demo content. Remove the leak before shipping: ${formattedViolations}`,
  );
}

export function createAdminReleaseSafetyPlugin({ command, env = process.env } = {}) {
  return {
    name: 'sdkwork-admin-release-safety',
    buildStart() {
      assertAdminReleaseSafety({ command, env });
    },
    generateBundle(_outputOptions, bundle) {
      assertAdminBundleContentSafe(bundle);
    },
  };
}
