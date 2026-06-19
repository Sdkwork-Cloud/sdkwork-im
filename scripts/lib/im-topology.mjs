import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

export const REPO_ROOT = path.resolve(__dirname, '..', '..');
export const SPEC_PATH = path.join(REPO_ROOT, 'specs/topology.spec.json');

const APP_TOPOLOGY_ENTRY_RELATIVE = path.join('tools', 'topology', 'lib', 'index.mjs');

function resolveAppTopologyEntryPath() {
  const installedEntry = path.join(
    REPO_ROOT,
    'node_modules',
    '@sdkwork',
    'app-topology',
    APP_TOPOLOGY_ENTRY_RELATIVE,
  );
  if (fs.existsSync(installedEntry)) {
    return installedEntry;
  }

  const siblingEntry = path.join(REPO_ROOT, '..', 'sdkwork-app-topology', APP_TOPOLOGY_ENTRY_RELATIVE);
  if (fs.existsSync(siblingEntry)) {
    return siblingEntry;
  }

  throw new Error(
    'Missing @sdkwork/app-topology. Clone ../sdkwork-app-topology next to sdkwork-im or run: pnpm install -w',
  );
}

const {
  buildProfileId,
  createTopologyRuntime,
  isTcpPortReachable,
  loadTopologySpec,
  normalizeText,
  waitForHttpHealthy,
} = await import(pathToFileURL(resolveAppTopologyEntryPath()).href);

const spec = loadTopologySpec(SPEC_PATH);
const runtime = createTopologyRuntime(spec, REPO_ROOT);

export const DEFAULT_DEV_PROFILE_ID = runtime.defaults.developmentProfileId;
export const DEFAULT_BUILD_PROFILE_ID = runtime.defaults.productionProfileId;
export const POSTGRES_REACHABILITY_TIMEOUT_MS = 2000;

export function resolveDevProfileId(hosting, serviceLayout = 'split-services') {
  runtime.assertHosting(hosting);
  runtime.assertServiceLayout(serviceLayout);
  return buildProfileId(hosting, serviceLayout, 'development');
}

export const loadProfile = runtime.loadProfile;
export const applyProfileEnv = runtime.applyProfileEnv;
export const mergeRuntimeEnv = runtime.mergeRuntimeEnv;
export const loadEnvFile = runtime.loadEnvFile;
export const assertHosting = runtime.assertHosting;
export const assertServiceLayout = runtime.assertServiceLayout;
export const resolveSurfaceHttpUrl = runtime.resolveSurfaceHttpUrl.bind(runtime);
export const resolveSurfaceWebsocketOrigin = runtime.resolveSurfaceWebsocketOrigin.bind(runtime);
export const resolveSurfaceBind = runtime.resolveSurfaceBind.bind(runtime);
export const shouldAutostartGateway = runtime.shouldAutostartGateway;
export const resolveGatewayBind = runtime.resolveGatewayBind;
export const resolveGatewayBaseUrl = runtime.resolveGatewayBaseUrl;
export const resolveIamDevEnv = runtime.resolveIamDevEnv;
export const assertPostgresReachableForIam = runtime.assertPostgresReachableForIam;
export const describeIamDatabaseTarget = runtime.describeIamDatabaseTarget;
export const listOrchestrationProcesses = runtime.listOrchestrationProcesses;
export const listHealthSurfaces = runtime.listHealthSurfaces;

export { buildProfileId, normalizeText, isTcpPortReachable, waitForHttpHealthy, spec, runtime };
