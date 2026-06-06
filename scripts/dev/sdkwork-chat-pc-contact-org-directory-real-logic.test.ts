import assert from 'node:assert/strict';
import type { ImSdkClient } from '@sdkwork/im-sdk';
import { createSdkworkContactService } from '../../apps/sdkwork-chat-pc/packages/sdkwork-clawchat-pc-chat/src/services/ContactService';
import {
  createSdkworkOrganizationDirectoryService,
  type OrganizationDirectoryClient,
} from '../../apps/sdkwork-chat-pc/packages/sdkwork-clawchat-pc-chat/src/services/OrganizationDirectoryService';

const fakeImClient = {
  chat: {
    contacts: {
      async list() {
        return { items: [], hasMore: false };
      },
    },
  },
} as unknown as ImSdkClient;

function assertNoTenantParam(value: Record<string, unknown> | undefined, message: string): void {
  assert.equal(value?.tenantId, undefined, message);
}

function assertNoOrganizationParam(value: Record<string, unknown> | undefined, message: string): void {
  assert.equal(value?.organizationId, undefined, message);
}

async function main(): Promise<void> {
  const organizationCalls: string[] = [];
  const organizationClient = {
    iam: {
      organizations: {
        async list(params) {
          assertNoTenantParam(params, 'current tenant context must not be sent as iam.organizations.list params');
          organizationCalls.push(`iam.organizations.list:${params?.tenantId ?? ''}`);
          return [
            {
              organizationId: 'org-group',
              tenantId: 'tenant-1',
              name: 'SDKWork Group',
              parentOrganizationId: null,
              organizationKind: 'group',
              tenantBoundaryKind: 'root_tenant',
              verificationStatus: 'verified',
              appBoundaryEnabled: true,
              dataBoundaryKind: 'tenant_shared',
              order: 0,
            },
            {
              organizationId: 'org-company',
              tenantId: 'tenant-1',
              name: 'SDKWork Cloud Company',
              parentOrganizationId: 'org-group',
              organizationKind: 'company',
              tenantBoundaryKind: 'sub_tenant',
              verificationStatus: 'verified',
              appBoundaryEnabled: true,
              dataBoundaryKind: 'organization_isolated',
              order: 10,
            },
          ];
        },
      },
      departments: {
        async list(params) {
          assertNoTenantParam(params, 'current tenant context must not be sent as iam.departments.list params');
          assertNoOrganizationParam(params, 'current organization context must not be sent as iam.departments.list params');
          organizationCalls.push(`iam.departments.list:${params?.organizationId ?? ''}`);
          return [
            {
              departmentId: 'dept-root',
              organizationId: 'org-company',
              name: 'Company Headquarters',
              parentDepartmentId: null,
              departmentKind: 'business_unit',
              order: 0,
            },
            {
              departmentId: 'dept-rd',
              organizationId: 'org-company',
              name: 'Research',
              parentDepartmentId: 'dept-root',
              departmentKind: 'department',
              order: 20,
            },
          ];
        },
      },
      departmentAssignments: {
        async list(params) {
          assertNoTenantParam(params, 'current tenant context must not be sent as iam.departmentAssignments.list params');
          assertNoOrganizationParam(params, 'current organization context must not be sent as iam.departmentAssignments.list params');
          organizationCalls.push(`iam.departmentAssignments.list:${params?.departmentId ?? ''}`);
          return params?.departmentId === 'dept-rd'
            ? [
                {
                  assignmentId: 'assign-alice-rd',
                  membershipId: 'membership-alice-company',
                  organizationId: 'org-company',
                  departmentId: 'dept-rd',
                  userId: 'u_alice',
                  displayName: 'Alice',
                  avatarUrl: 'https://example.com/alice.png',
                  email: 'alice@example.com',
                  phone: '13800000001',
                  positionName: 'Engineer',
                  roleCodes: ['org.member', 'department.engineer'],
                  assignmentType: 'primary',
                  status: 'active',
                },
              ]
            : [];
        },
      },
      organizationMemberships: {
        async list(params) {
          assertNoTenantParam(params, 'current tenant context must not be sent as iam.organizationMemberships.list params');
          organizationCalls.push(`iam.organizationMemberships.list:${params?.organizationId ?? ''}`);
          return [];
        },
      },
      positions: {
        async list(params) {
          assertNoTenantParam(params, 'current tenant context must not be sent as iam.positions.list params');
          organizationCalls.push(`iam.positions.list:${params?.organizationId ?? ''}`);
          return [];
        },
      },
      positionAssignments: {
        async list(params) {
          assertNoTenantParam(params, 'current tenant context must not be sent as iam.positionAssignments.list params');
          organizationCalls.push(`iam.positionAssignments.list:${params?.departmentAssignmentId ?? ''}`);
          return [];
        },
      },
      roleBindings: {
        async list(params) {
          assertNoTenantParam(params, 'current tenant context must not be sent as iam.roleBindings.list params');
          organizationCalls.push(`iam.roleBindings.list:${params?.scopeId ?? ''}`);
          return [];
        },
      },
    },
    async listOrganizations(params) {
      assertNoTenantParam(params, 'current tenant context must not be sent through compat listOrganizations params');
      organizationCalls.push(`compat.listOrganizations:${params?.tenantId ?? ''}`);
      return [
        {
          organizationId: 'org-company',
          tenantId: 'tenant-1',
          name: 'SDKWork Cloud Company',
          parentOrganizationId: 'org-group',
        },
      ];
    },
    async listDepartments(organizationId) {
      organizationCalls.push(`compat.listDepartments:${organizationId}`);
      return [];
    },
    async listDepartmentAssignments(departmentId) {
      organizationCalls.push(`compat.listDepartmentAssignments:${departmentId}`);
      return [];
    },
  } satisfies OrganizationDirectoryClient;

  const forbiddenPortalAppClient = {
    portal: {
      home: {
        async retrieve() {
          throw new Error('portal.home.retrieve must not back contact organization directory');
        },
      },
    },
  };

  const organizationDirectoryService = createSdkworkOrganizationDirectoryService(() => organizationClient);
  const directoryBackedService = createSdkworkContactService(
    () => fakeImClient,
    () => forbiddenPortalAppClient,
    () => organizationDirectoryService,
  );

  assert.deepEqual(
    await directoryBackedService.getDepartments(),
    [
      { id: 'dept-root', name: 'Company Headquarters', organizationId: 'org-company', parentId: null, order: 0 },
      { id: 'dept-rd', name: 'Research', organizationId: 'org-company', parentId: 'dept-root', order: 20 },
    ],
    'contact org directory must map departments from the independent Organization/Department directory client',
  );
  assert.deepEqual(
    await directoryBackedService.getUsersByDepartment('dept-rd'),
    [
      {
        assignmentType: 'primary',
        avatar: 'https://example.com/alice.png',
        departmentAssignmentId: 'assign-alice-rd',
        departmentId: 'dept-rd',
        email: 'alice@example.com',
        id: 'u_alice',
        name: 'Alice',
        organizationId: 'org-company',
        organizationMembershipId: 'membership-alice-company',
        phone: '13800000001',
        position: 'Engineer',
        py: 'alice',
        roleCodes: ['department.engineer', 'org.member'],
        status: 'online',
      },
    ],
    'contact org directory users must come from department assignments instead of IM friendship contacts',
  );
  assert.deepEqual(
    organizationCalls,
    [
      'iam.departments.list:',
      'iam.departmentAssignments.list:dept-rd',
      'iam.positionAssignments.list:assign-alice-rd',
      'iam.roleBindings.list:assign-alice-rd',
    ],
    'contact org directory must read through the independent organization directory client',
  );

  const productDirectoryCalls: string[] = [];
  const productDirectoryService = createSdkworkOrganizationDirectoryService(() => ({
    iam: {
      organizations: {
        async list() {
          productDirectoryCalls.push('iam.organizations.list');
          return [];
        },
        tree: {
          async retrieve(params) {
            assertNoTenantParam(params, 'current tenant context must not be sent as iam.organizations.tree.retrieve params');
            productDirectoryCalls.push(`iam.organizations.tree.retrieve:${params?.tenantId ?? ''}`);
            return {
              items: [
                {
                  organizationId: 'org-group',
                  tenantId: 'tenant-1',
                  name: 'SDKWork Group',
                  parentOrganizationId: null,
                  organizationKind: 'group',
                  tenantBoundaryKind: 'root_tenant',
                  verificationStatus: 'verified',
                  status: 'active',
                  order: 0,
                  children: [
                    {
                      organizationId: 'org-company',
                      tenantId: 'tenant-1',
                      name: 'SDKWork Cloud Company',
                      parentOrganizationId: 'org-group',
                      organizationKind: 'company',
                      tenantBoundaryKind: 'operating_subject',
                      verificationStatus: 'verified',
                      dataBoundaryKind: 'organization_isolated',
                      appBoundaryEnabled: true,
                      status: 'active',
                      order: 10,
                      children: [],
                    },
                  ],
                },
              ],
            };
          },
        },
      },
      departments: {
        async list() {
          productDirectoryCalls.push('iam.departments.list');
          return [];
        },
        tree: {
          async retrieve(params) {
            assertNoTenantParam(params, 'current tenant context must not be sent as iam.departments.tree.retrieve params');
            productDirectoryCalls.push(`iam.departments.tree.retrieve:${params?.organizationId ?? ''}`);
            return {
              items: [
                {
                  departmentId: 'dept-root',
                  tenantId: 'tenant-1',
                  organizationId: 'org-company',
                  name: 'Company Headquarters',
                  parentDepartmentId: null,
                  departmentKind: 'headquarters',
                  status: 'active',
                  order: 0,
                  children: [
                    {
                      departmentId: 'dept-rd',
                      tenantId: 'tenant-1',
                      organizationId: 'org-company',
                      name: 'Research',
                      parentDepartmentId: 'dept-root',
                      departmentKind: 'department',
                      status: 'active',
                      order: 20,
                      children: [],
                    },
                  ],
                },
              ],
            };
          },
        },
      },
      departmentAssignments: {
        async list(params) {
          assertNoTenantParam(params, 'current tenant context must not be sent as iam.departmentAssignments.list params');
          productDirectoryCalls.push(`iam.departmentAssignments.list:${params?.organizationId ?? ''}:${params?.departmentId ?? ''}`);
          return [
            {
              assignmentId: 'assign-alice-rd',
              membershipId: 'membership-alice-company',
              organizationId: 'org-company',
              departmentId: 'dept-rd',
              userId: 'u_alice',
              displayName: 'Alice',
              email: 'alice@example.com',
              phone: '13800000001',
              avatarUrl: 'https://example.com/alice.png',
              assignmentType: 'primary',
              status: 'active',
              positionId: 'pos-engineer',
              positionName: 'Engineer',
              roleCodes: ['org.member'],
            },
          ];
        },
      },
      positionAssignments: {
        async list(params) {
          assertNoTenantParam(params, 'current tenant context must not be sent as iam.positionAssignments.list params');
          productDirectoryCalls.push(`iam.positionAssignments.list:${params?.departmentAssignmentId ?? ''}`);
          return [
            {
              positionAssignmentId: 'pos-assign-alice-principal',
              departmentAssignmentId: 'assign-alice-rd',
              tenantId: 'tenant-1',
              organizationId: 'org-company',
              departmentId: 'dept-rd',
              userId: 'u_alice',
              positionId: 'pos-engineer',
              positionName: 'Principal Engineer',
              status: 'active',
            },
          ];
        },
      },
      roleBindings: {
        async list(params) {
          assertNoTenantParam(params, 'current tenant context must not be sent as iam.roleBindings.list params');
          productDirectoryCalls.push(`iam.roleBindings.list:${params?.scopeKind ?? ''}:${params?.scopeId ?? ''}:${params?.principalId ?? ''}`);
          return [
            {
              roleBindingId: 'rb-alice-rd-engineer',
              tenantId: 'tenant-1',
              roleCode: 'department.engineer',
              principalKind: 'department_assignment',
              principalId: 'assign-alice-rd',
              scopeKind: 'department_assignment',
              scopeId: 'assign-alice-rd',
              status: 'active',
            },
          ];
        },
      },
    },
  }) satisfies OrganizationDirectoryClient);

  assert.deepEqual(
    await productDirectoryService.getOrganizationTree(),
    [
      {
        appBoundaryEnabled: undefined,
        children: [
          {
            appBoundaryEnabled: true,
            children: [],
            dataBoundaryKind: 'organization_isolated',
            id: 'org-company',
            name: 'SDKWork Cloud Company',
            order: 10,
            organizationId: 'org-company',
            organizationKind: 'company',
            parentOrganizationId: 'org-group',
            status: 'active',
            tenantBoundaryKind: 'operating_subject',
            tenantId: 'tenant-1',
            verificationStatus: 'verified',
          },
        ],
        dataBoundaryKind: undefined,
        id: 'org-group',
        name: 'SDKWork Group',
        order: 0,
        organizationId: 'org-group',
        organizationKind: 'group',
        parentOrganizationId: null,
        status: 'active',
        tenantBoundaryKind: 'root_tenant',
        tenantId: 'tenant-1',
        verificationStatus: 'verified',
      },
    ],
    'contact org directory must expose the organization hierarchy as organizations, not departments folded into iam_organizations',
  );
  assert.deepEqual(
    await productDirectoryService.getDepartmentTree('org-company'),
    [
      {
        children: [
          {
            children: [],
            id: 'dept-rd',
            name: 'Research',
            order: 20,
            organizationId: 'org-company',
            parentId: 'dept-root',
          },
        ],
        id: 'dept-root',
        name: 'Company Headquarters',
        order: 0,
        organizationId: 'org-company',
        parentId: null,
      },
    ],
    'contact org directory must expose departments through /departments hierarchy independent of organization hierarchy',
  );
  assert.deepEqual(
    await productDirectoryService.getUsersByDepartment('dept-rd'),
    [
      {
        assignmentType: 'primary',
        avatar: 'https://example.com/alice.png',
        departmentAssignmentId: 'assign-alice-rd',
        departmentId: 'dept-rd',
        email: 'alice@example.com',
        id: 'u_alice',
        name: 'Alice',
        organizationId: 'org-company',
        organizationMembershipId: 'membership-alice-company',
        phone: '13800000001',
        position: 'Principal Engineer',
        positionAssignments: [
          {
            positionAssignmentId: 'pos-assign-alice-principal',
            positionId: 'pos-engineer',
            positionName: 'Principal Engineer',
            status: 'active',
          },
        ],
        py: 'alice',
        roleBindings: [
          {
            roleBindingId: 'rb-alice-rd-engineer',
            roleCode: 'department.engineer',
            scopeId: 'assign-alice-rd',
            scopeKind: 'department_assignment',
            status: 'active',
          },
        ],
        roleCodes: ['department.engineer', 'org.member'],
        status: 'online',
      },
    ],
    'contact org directory members must carry organization membership, department assignment, position assignment, and scoped role binding context',
  );
  assert.deepEqual(
    productDirectoryCalls,
    [
      'iam.organizations.tree.retrieve:',
      'iam.departments.tree.retrieve:org-company',
      'iam.departmentAssignments.list:org-company:dept-rd',
      'iam.positionAssignments.list:assign-alice-rd',
      'iam.roleBindings.list:department_assignment:assign-alice-rd:',
    ],
    'contact org directory product view must use organization tree, department tree, position assignment, and role binding SDK APIs',
  );

  const memberManagementCalls: string[] = [];
  const memberManagementAdminCalls: string[] = [];
  const memberManagementClient = {
    iam: {
      users: {
        current: {
          async retrieve() {
            memberManagementCalls.push('iam.users.current.retrieve');
            return {
              userId: 'u_admin',
              displayName: 'Organization Admin',
              email: 'admin@example.com',
              avatarUrl: 'https://example.com/admin.png',
              status: 'active',
            };
          },
        },
      },
      organizationMemberships: {
        async list(params) {
          assertNoTenantParam(params, 'current tenant context must not be sent as iam.organizationMemberships.list params');
          memberManagementCalls.push(`iam.organizationMemberships.list:${params?.organizationId ?? ''}:${params?.userId ?? ''}`);
          if (params?.userId === 'u_admin') {
            return [
              {
                membershipId: 'membership-admin-company',
                tenantId: 'tenant-1',
                organizationId: 'org-company',
                userId: 'u_admin',
                primary: true,
                status: 'active',
              },
            ];
          }
          return [];
        },
      },
      roleBindings: {
        async list(params) {
          assertNoTenantParam(params, 'current tenant context must not be sent as iam.roleBindings.list params');
          memberManagementCalls.push(`iam.roleBindings.list:${params?.scopeKind ?? ''}:${params?.scopeId ?? ''}:${params?.principalId ?? ''}`);
          if (params?.principalId === 'membership-admin-company') {
            return [
              {
                roleBindingId: 'rb-admin-company',
                tenantId: 'tenant-1',
                roleCode: 'org.admin',
                principalKind: 'organization_membership',
                principalId: 'membership-admin-company',
                scopeKind: 'organization',
                scopeId: 'org-company',
                status: 'active',
              },
            ];
          }
          return [];
        },
      },
    },
  } satisfies OrganizationDirectoryClient;
  const memberManagementAdmin = {
    users: {
      async create(body) {
        assertNoTenantParam(body, 'current tenant context must not be sent in admin.users.create body');
        memberManagementAdminCalls.push(`admin.users.create:${body.email ?? body.phone ?? ''}`);
        return {
          userId: 'u_invited',
          displayName: body.displayName,
          email: body.email,
          status: 'invited',
        };
      },
    },
    organizationMemberships: {
      async create(body) {
        assertNoTenantParam(body, 'current tenant context must not be sent in admin.organizationMemberships.create body');
        memberManagementAdminCalls.push(`admin.organizationMemberships.create:${body.organizationId}:${body.userId}`);
        return {
          membershipId: body.userId === 'u_invited' ? 'membership-invited-company' : 'membership-charlie-company',
          ...body,
        };
      },
    },
    departmentAssignments: {
      async create(body) {
        assertNoTenantParam(body, 'current tenant context must not be sent in admin.departmentAssignments.create body');
        memberManagementAdminCalls.push(`admin.departmentAssignments.create:${body.departmentId}:${body.organizationMembershipId}`);
        return {
          assignmentId: body.userId === 'u_invited' ? 'assign-invited-rd' : 'assign-charlie-rd',
          ...body,
        };
      },
    },
    positionAssignments: {
      async create(body) {
        assertNoTenantParam(body, 'current tenant context must not be sent in admin.positionAssignments.create body');
        memberManagementAdminCalls.push(`admin.positionAssignments.create:${body.departmentAssignmentId}:${body.positionId ?? ''}`);
        return {
          positionAssignmentId: 'pos-assign-charlie',
          ...body,
        };
      },
    },
    roleBindings: {
      async create(body) {
        assertNoTenantParam(body, 'current tenant context must not be sent in admin.roleBindings.create body');
        memberManagementAdminCalls.push(`admin.roleBindings.create:${body.scopeKind}:${body.scopeId}:${body.roleCode}`);
        return {
          roleBindingId: body.roleCode === 'org.member' ? 'rb-org-member' : 'rb-dept-engineer',
          ...body,
        };
      },
    },
  };
  const memberManagementDirectoryService = createSdkworkOrganizationDirectoryService(() => memberManagementClient, {
    admin: memberManagementAdmin,
  });

  assert.deepEqual(
    await memberManagementDirectoryService.getCurrentUser(),
    {
      avatar: 'https://example.com/admin.png',
      email: 'admin@example.com',
      id: 'u_admin',
      name: 'Organization Admin',
      py: 'organizationadmin',
      status: 'online',
    },
    'organization contacts view must read the logged-in IAM user through iam.users.current.retrieve',
  );
  assert.deepEqual(
    await memberManagementDirectoryService.getOrganizationPermissions('org-company'),
    {
      adminCapabilityAvailable: true,
      canAssignRoles: true,
      canInviteMembers: true,
      canManageMembers: true,
      currentUserId: 'u_admin',
      organizationId: 'org-company',
      organizationMembershipIds: ['membership-admin-company'],
      reason: 'role_allowed',
      roleCodes: ['org.admin'],
    },
    'organization contacts view must derive admin member-management permissions from scoped organization role bindings',
  );
  assert.deepEqual(
    await memberManagementDirectoryService.addOrganizationMember({
      assignmentType: 'secondary',
      departmentId: 'dept-rd',
      membershipType: 'employee',
      organizationId: 'org-company',
      positionId: 'pos-engineer',
      roleCodes: ['org.member', 'department.engineer'],
      userId: 'u_charlie',
    }),
    {
      departmentAssignmentId: 'assign-charlie-rd',
      organizationId: 'org-company',
      organizationMembershipId: 'membership-charlie-company',
      positionAssignmentIds: ['pos-assign-charlie'],
      roleBindingIds: ['rb-org-member', 'rb-dept-engineer'],
      userId: 'u_charlie',
    },
    'organization contacts view must add a member through membership, department assignment, position assignment, and scoped role binding APIs',
  );
  assert.deepEqual(
    await memberManagementDirectoryService.inviteOrganizationMember({
      assignmentType: 'primary',
      departmentId: 'dept-rd',
      displayName: 'Invited User',
      email: 'invite@example.com',
      membershipType: 'employee',
      organizationId: 'org-company',
      roleCodes: ['org.member'],
    }),
    {
      departmentAssignmentId: 'assign-invited-rd',
      invitedUserId: 'u_invited',
      organizationId: 'org-company',
      organizationMembershipId: 'membership-invited-company',
      positionAssignmentIds: [],
      roleBindingIds: ['rb-org-member'],
      userId: 'u_invited',
    },
    'organization contacts view must invite unknown people through the IAM user capability before attaching organization membership',
  );
  assert.deepEqual(
    memberManagementAdminCalls,
    [
      'admin.organizationMemberships.create:org-company:u_charlie',
      'admin.departmentAssignments.create:dept-rd:membership-charlie-company',
      'admin.positionAssignments.create:assign-charlie-rd:pos-engineer',
      'admin.roleBindings.create:organization:org-company:org.member',
      'admin.roleBindings.create:department_assignment:assign-charlie-rd:department.engineer',
      'admin.users.create:invite@example.com',
      'admin.organizationMemberships.create:org-company:u_invited',
      'admin.departmentAssignments.create:dept-rd:membership-invited-company',
      'admin.roleBindings.create:organization:org-company:org.member',
    ],
    'organization member management must use injected IAM admin capabilities instead of handwritten backend HTTP',
  );

  const deniedAdminCalls: string[] = [];
  const deniedDirectoryService = createSdkworkOrganizationDirectoryService(() => ({
    iam: {
      users: {
        current: {
          async retrieve() {
            return { userId: 'u_member', displayName: 'Member User', status: 'active' };
          },
        },
      },
      organizationMemberships: {
        async list() {
          return [
            {
              membershipId: 'membership-member-company',
              organizationId: 'org-company',
              userId: 'u_member',
              status: 'active',
            },
          ];
        },
      },
      roleBindings: {
        async list() {
          return [
            {
              roleBindingId: 'rb-member-company',
              roleCode: 'org.member',
              scopeKind: 'organization',
              scopeId: 'org-company',
              status: 'active',
            },
          ];
        },
      },
    },
  }) satisfies OrganizationDirectoryClient, {
    admin: {
      organizationMemberships: {
        async create(body) {
          assertNoTenantParam(body, 'current tenant context must not be sent in denied admin body');
          deniedAdminCalls.push(`admin.organizationMemberships.create:${body.userId}`);
          return body;
        },
      },
    },
  });
  await assert.rejects(
    () => deniedDirectoryService.addOrganizationMember({
      organizationId: 'org-company',
      userId: 'u_blocked',
    }),
    /not allowed to manage organization members/u,
    'organization member management must be rejected before mutation when the logged-in user has no admin role',
  );
  assert.deepEqual(deniedAdminCalls, [], 'permission-denied organization member management must not call admin mutation capabilities');

  const emptyOrganizationDirectoryService = createSdkworkOrganizationDirectoryService(() => ({
    iam: {
      departments: {
        async list() {
          return [];
        },
      },
      departmentAssignments: {
        async list() {
          return [];
        },
      },
    },
    async listDepartments() {
      return [];
    },
    async listDepartmentAssignments() {
      return [];
    },
  }));
  const emptyDirectoryService = createSdkworkContactService(
    () => fakeImClient,
    () => forbiddenPortalAppClient,
    () => emptyOrganizationDirectoryService,
  );

  assert.deepEqual(
    await emptyDirectoryService.getDepartments(),
    [],
    'contact org directory must not synthesize departments when the organization directory has no records',
  );
  assert.deepEqual(
    await emptyDirectoryService.getUsersByDepartment('org-root'),
    [],
    'contact org directory users must not fall back to IM contacts for a synthetic org-root department',
  );

  const multiOrganizationCalls: string[] = [];
  const multiOrganizationDirectoryService = createSdkworkOrganizationDirectoryService(() => ({
    iam: {
      organizations: {
        async list(params) {
          assertNoTenantParam(params, 'current tenant context must not be sent as iam.organizations.list params');
          multiOrganizationCalls.push(`iam.organizations.list:${params?.tenantId ?? ''}`);
          return [
            {
              organizationId: 'org-group',
              tenantId: 'tenant-1',
              name: 'SDKWork Group',
              parentOrganizationId: null,
              organizationKind: 'group',
              tenantBoundaryKind: 'root_tenant',
              verificationStatus: 'verified',
              status: 'active',
              order: 0,
            },
            {
              organizationId: 'org-company',
              tenantId: 'tenant-1',
              name: 'SDKWork Cloud Company',
              parentOrganizationId: 'org-group',
              organizationKind: 'company',
              tenantBoundaryKind: 'operating_subject',
              verificationStatus: 'verified',
              status: 'active',
              order: 10,
            },
          ];
        },
      },
      organizationMemberships: {
        async list(params) {
          assertNoTenantParam(params, 'current tenant context must not be sent as iam.organizationMemberships.list params');
          multiOrganizationCalls.push(`iam.organizationMemberships.list:${params?.userId ?? ''}`);
          return [
            {
              membershipId: 'membership-bob-group',
              tenantId: 'tenant-1',
              organizationId: 'org-group',
              userId: 'u_bob',
              membershipType: 'employee',
              status: 'active',
              primary: false,
            },
            {
              membershipId: 'membership-bob-company',
              tenantId: 'tenant-1',
              organizationId: 'org-company',
              userId: 'u_bob',
              membershipType: 'employee',
              status: 'active',
              primary: true,
            },
          ];
        },
      },
      departments: {
        async list(params) {
          assertNoTenantParam(params, 'current tenant context must not be sent as iam.departments.list params');
          assertNoOrganizationParam(params, 'current organization context must not be sent as iam.departments.list params');
          multiOrganizationCalls.push(`iam.departments.list:${params?.organizationId ?? ''}`);
          return [
            {
              departmentId: 'dept-company-root',
              tenantId: 'tenant-1',
              organizationId: 'org-company',
              name: 'Company Headquarters',
              parentDepartmentId: null,
              departmentKind: 'headquarters',
              status: 'active',
              order: 0,
            },
          ];
        },
      },
      departmentAssignments: {
        async list(params) {
          assertNoTenantParam(params, 'current tenant context must not be sent as iam.departmentAssignments.list params');
          multiOrganizationCalls.push(`iam.departmentAssignments.list:${params?.organizationId ?? ''}:${params?.departmentId ?? ''}`);
          return [];
        },
      },
    },
  }));

  assert.deepEqual(
    await multiOrganizationDirectoryService.getDepartments(),
    [
      {
        id: 'dept-company-root',
        name: 'Company Headquarters',
        organizationId: 'org-company',
        parentId: null,
        order: 0,
      },
    ],
    'contact org directory must resolve the active organization membership before listing departments in a multi-organization tenant',
  );
  assert.deepEqual(
    multiOrganizationCalls,
    [
      'iam.departments.list:',
    ],
    'contact org directory must rely on request Context for current organization instead of resolving and passing it as params',
  );

  console.log('sdkwork-chat-pc contact org directory real-logic contract passed');
}

void main();
