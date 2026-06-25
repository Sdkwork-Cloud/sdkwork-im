#!/usr/bin/env node

import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';

const appRoot = path.resolve(import.meta.dirname, '..');
const repoRoot = path.resolve(appRoot, '..', '..');

function readText(...segments) {
  return fs.readFileSync(path.join(appRoot, ...segments), 'utf8');
}

function readJson(...segments) {
  return JSON.parse(readText(...segments));
}

function readRepoText(...segments) {
  return fs.readFileSync(path.join(repoRoot, ...segments), 'utf8');
}

function readRepoJson(...segments) {
  return JSON.parse(readRepoText(...segments));
}

const packageJson = readJson('package.json');
const releaseSources = readRepoJson('config', 'shared-sdk-release-sources.json');
const workflow = readRepoJson('sdkwork.workflow.json');
const gatewayConfigSource = readRepoText('crates', 'sdkwork-im-cloud-gateway-config', 'src', 'lib.rs');
const gatewaySource = readRepoText('services', 'sdkwork-im-cloud-gateway', 'src', 'lib.rs');
const sharedSdkGitSource = readRepoText('scripts', 'dev', 'prepare-shared-sdk-git-sources.mjs');
const releaseBuildSource = readRepoText('scripts', 'release', 'run-sdkwork-im-pc-release-build.mjs');
const devRunnerSource = readRepoText('scripts', 'lib', 'im-pc-dev.mjs');
const componentSpec = readRepoJson('specs', 'component.spec.json');
const moduleRegistrySource = readText('packages', 'sdkwork-im-pc-shell', 'src', 'moduleRegistry.ts');
const courseServiceSource = readText('packages', 'sdkwork-im-pc-course', 'src', 'services', 'CourseService.ts');
const courseClientSource = readText('packages', 'sdkwork-im-pc-core', 'src', 'sdk', 'courseAppSdkClient.ts');
const viteConfigSource = readText('vite.config.ts');
const tsconfig = readJson('tsconfig.json');

assert.equal(
  packageJson.scripts?.['test:course-app-sdk-integration'],
  'node scripts/course-app-sdk-integration-contract.test.mjs',
  'Chat PC must expose a dedicated course app SDK integration contract script.',
);

assert.equal(
  readJson('packages', 'sdkwork-im-pc-core', 'package.json').dependencies?.['@sdkwork/course-app-sdk'],
  'workspace:*',
  'Chat PC must consume sdkwork-course through the workspace app SDK package.',
);

assert.match(
  releaseSources.sources?.['sdkwork-course']?.repoUrl ?? '',
  /^(?:https:\/\/github\.com\/|git@github\.com:)Sdkwork-Cloud\/sdkwork-course\.git$/u,
  'Shared SDK release config must materialize sdkwork-course from the canonical git repository.',
);

assert.equal(
  releaseSources.sources?.['sdkwork-course']?.ref,
  workflow.dependencies?.find((dependency) => dependency.id === 'sdkwork-course')?.ref,
  'Shared SDK release config must use the same pinned sdkwork-course ref as sdkwork.workflow.json.',
);

assert.match(
  sharedSdkGitSource,
  /id:\s*['"]sdkwork-course['"][\s\S]*sdkwork-course-app-sdk[\\/]sdkwork-course-app-sdk-typescript[\\/]generated[\\/]server-openapi[\\/]package\.json/u,
  'Shared SDK git materializer must know how to prepare the sdkwork-course app SDK source.',
);

assert.match(
  releaseBuildSource,
  /SDKWORK_SHARED_COURSE_GIT_REF[\s\S]*SDKWORK_COURSE_REF/u,
  'Release build plan must bridge SDKWORK_COURSE_REF into the shared SDK materializer ref for the course app SDK.',
);

assert.match(
  devRunnerSource,
  /SDKWORK_IM_PLATFORM_API_GATEWAY_HTTP_URL[\s\S]*explicitCourseAppApiUpstream[\s\S]*SDKWORK_IM_COURSE_APP_API_UPSTREAM/u,
  'PC dev runner must default sdkwork-course traffic through the shared gateway root while preserving explicit Course split upstream overrides.',
);

assert.match(
  gatewayConfigSource,
  /sdkwork-course-app-api[\s\S]*SDKWORK_IM_COURSE_APP_API_UPSTREAM[\s\S]*SDKWORK_COURSE_APP_API_UPSTREAM[\s\S]*SDKWORK_COURSE_APP_API_BASE_URL/u,
  'Gateway config must expose deterministic sdkwork-course app-api upstream environment keys.',
);

assert.match(
  gatewaySource,
  /COURSE_APP_API_SEGMENTS[\s\S]*"courses"[\s\S]*"course_applications"/u,
  'Web gateway must declare sdkwork-course app-api route segments.',
);

assert.match(
  gatewaySource,
  /"sdkwork-course-app-api"[\s\S]*SdkworkCourseAppSdk/u,
  'Web gateway must route sdkwork-course app-api paths to the Course app SDK upstream.',
);

const dependencySurface = componentSpec.contracts?.dependencyApiSurfaces?.find(
  (surface) => surface.apiAuthority === 'sdkwork-course-app-api',
);
assert.ok(dependencySurface, 'component.spec.json must declare sdkwork-course-app-api dependency surface.');
assert.equal(dependencySurface.sdkFamily, 'sdkwork-course-app-sdk');

const splitOverrideEnvKeys =
  componentSpec.integration?.foundationApiGateway?.splitOverrideEnvKeys?.['sdkwork-course-app-api'];
assert.deepEqual(splitOverrideEnvKeys, [
  'SDKWORK_IM_COURSE_APP_API_UPSTREAM',
  'SDKWORK_COURSE_APP_API_UPSTREAM',
  'SDKWORK_COURSE_APP_API_BASE_URL',
]);

assert.match(
  moduleRegistrySource,
  /COMMERCIAL_RUNTIME_MODULES[\s\S]*"course"/u,
  'Course must be enabled in commercial runtime modules after SDK wiring.',
);

assert.match(
  courseServiceSource,
  /getCourseAppSdkClientWithSession/u,
  'CourseService must consume the generated course app SDK client instead of local mock data.',
);
assert.doesNotMatch(
  courseServiceSource,
  /unsplash|COURSES\s*=\s*\[/u,
  'CourseService must not keep local mock catalog data.',
);

assert.match(courseClientSource, /@sdkwork\/course-app-sdk/u);
assert.match(viteConfigSource, /@sdkwork\/course-app-sdk/u);
assert.ok(tsconfig.compilerOptions?.paths?.['@sdkwork/course-app-sdk']);

console.log('course app SDK integration contract checks passed');
