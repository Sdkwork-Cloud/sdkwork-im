import assert from 'node:assert/strict';
import {
  buildSdkworkChatAppContextHeaders,
  createSdkworkChatRequestContext,
  createSdkworkChatRequestContextInterceptors,
  resolveAppSdkOrganizationId,
  resolveAppSdkSessionId,
  resolveAppSdkTenantId,
  resolveAppSdkUserId,
  type SdkworkChatSession,
} from '../../apps/sdkwork-chat-pc/packages/sdkwork-clawchat-pc-core/src/sdk/session';

function createJwt(payload: Record<string, unknown>): string {
  const header = Buffer.from(JSON.stringify({ alg: 'none', typ: 'JWT' })).toString('base64url');
  const body = Buffer.from(JSON.stringify(payload)).toString('base64url');
  return `${header}.${body}.`;
}

async function main(): Promise<void> {
  const session: SdkworkChatSession = {
    accessToken: createJwt({
      app_id: 'sdkwork-chat-pc',
      tenant_id: 'tenant-from-access-token',
      organization_id: 'org-from-access-token',
      user_id: 'user-from-access-token',
      sid: 'session-from-access-token',
      environment: 'prod',
      deployment_mode: 'private',
      auth_level: 'mfa',
      data_scope: ['iam.organization.read', 'iam.department.read'],
      permission_scope: ['iam.member.manage'],
      actor_id: 'actor-from-access-token',
      actor_kind: 'user',
      device_id: 'device-from-access-token',
      context_signature: 'signed-context-from-access-token',
    }),
    authToken: createJwt({
      appId: 'sdkwork-chat-pc-auth',
      tenantId: 'tenant-from-auth-token',
      organizationId: 'org-from-auth-token',
      userId: 'user-from-auth-token',
      sessionId: 'session-from-auth-token',
    }),
    context: {
      appId: 'stale-session-context',
      tenantId: 'stale-tenant',
      organizationId: 'stale-org',
      userId: 'stale-user',
      sessionId: 'stale-session',
      environment: 'dev',
      deploymentMode: 'local',
      authLevel: 'password',
      dataScope: [],
      permissionScope: [],
    },
  };

  assert.equal(
    resolveAppSdkTenantId(session),
    'tenant-from-access-token',
    'request context tenant id must come from the JWT claims instead of stale session params',
  );
  assert.equal(
    resolveAppSdkOrganizationId(session),
    'org-from-access-token',
    'request context organization id must come from the JWT claims instead of stale session params',
  );
  assert.equal(
    resolveAppSdkUserId(session),
    'user-from-access-token',
    'request context user id must come from the JWT claims when present',
  );
  assert.equal(
    resolveAppSdkSessionId(session),
    'session-from-access-token',
    'request context session id must come from the JWT claims when present',
  );

  assert.deepEqual(
    createSdkworkChatRequestContext(session),
    {
      appId: 'sdkwork-chat-pc',
      tenantId: 'tenant-from-access-token',
      organizationId: 'org-from-access-token',
      userId: 'user-from-access-token',
      sessionId: 'session-from-access-token',
      environment: 'prod',
      deploymentMode: 'private',
      authLevel: 'mfa',
      dataScope: ['iam.organization.read', 'iam.department.read'],
      permissionScope: ['iam.member.manage'],
      actorId: 'actor-from-access-token',
      actorKind: 'user',
      deviceId: 'device-from-access-token',
      contextSignature: 'signed-context-from-access-token',
    },
    'shared SDK wrapper must encapsulate JWT claims into one request Context instance',
  );

  assert.deepEqual(
    buildSdkworkChatAppContextHeaders(session),
    {
      'X-Sdkwork-App-Id': 'sdkwork-chat-pc',
      'X-Sdkwork-Tenant-Id': 'tenant-from-access-token',
      'X-Sdkwork-Organization-Id': 'org-from-access-token',
      'X-Sdkwork-User-Id': 'user-from-access-token',
      'X-Sdkwork-Session-Id': 'session-from-access-token',
      'X-Sdkwork-Environment': 'prod',
      'X-Sdkwork-Deployment-Mode': 'private',
      'X-Sdkwork-Auth-Level': 'mfa',
      'X-Sdkwork-Actor-Id': 'actor-from-access-token',
      'X-Sdkwork-Actor-Kind': 'user',
      'X-Sdkwork-Device-Id': 'device-from-access-token',
      'X-Sdkwork-Data-Scope': 'iam.organization.read,iam.department.read',
      'X-Sdkwork-Permission-Scope': 'iam.member.manage',
      'X-Sdkwork-Context-Signature': 'signed-context-from-access-token',
    },
    'SDK wrappers must build one request Context from JWT claims and expose it as standard headers',
  );

  const [contextInterceptor] = createSdkworkChatRequestContextInterceptors(session).request;
  assert.deepEqual(
    await contextInterceptor({
      url: '/app/v3/api/iam/departments',
      method: 'GET',
      headers: {
        'X-Sdkwork-Tenant-Id': 'stale-tenant-header',
        'X-Feature-Header': 'keep-me',
      },
    }),
    {
      url: '/app/v3/api/iam/departments',
      method: 'GET',
      headers: {
        'X-Feature-Header': 'keep-me',
        'X-Sdkwork-App-Id': 'sdkwork-chat-pc',
        'X-Sdkwork-Tenant-Id': 'tenant-from-access-token',
        'X-Sdkwork-Organization-Id': 'org-from-access-token',
        'X-Sdkwork-User-Id': 'user-from-access-token',
        'X-Sdkwork-Session-Id': 'session-from-access-token',
        'X-Sdkwork-Environment': 'prod',
        'X-Sdkwork-Deployment-Mode': 'private',
        'X-Sdkwork-Auth-Level': 'mfa',
        'X-Sdkwork-Actor-Id': 'actor-from-access-token',
        'X-Sdkwork-Actor-Kind': 'user',
        'X-Sdkwork-Device-Id': 'device-from-access-token',
        'X-Sdkwork-Data-Scope': 'iam.organization.read,iam.department.read',
        'X-Sdkwork-Permission-Scope': 'iam.member.manage',
        'X-Sdkwork-Context-Signature': 'signed-context-from-access-token',
      },
    },
    'request interceptor must recreate Context headers for every SDK request and override stale request context headers',
  );

  assert.deepEqual(
    buildSdkworkChatAppContextHeaders({
      authToken: createJwt({
        appId: 'sdkwork-chat-pc',
        tenantId: 'tenant-from-auth-token',
        organizationId: 'org-from-auth-token',
        sub: 'user-from-subject',
        sessionId: 'session-from-auth-token',
      }),
    }),
    {
      'X-Sdkwork-App-Id': 'sdkwork-chat-pc',
      'X-Sdkwork-Tenant-Id': 'tenant-from-auth-token',
      'X-Sdkwork-Organization-Id': 'org-from-auth-token',
      'X-Sdkwork-User-Id': 'user-from-subject',
      'X-Sdkwork-Session-Id': 'session-from-auth-token',
    },
    'auth token claims must also be sufficient to build the request Context when access token claims are absent',
  );

  console.log('sdkwork-chat-pc request context contract passed');
}

void main();
