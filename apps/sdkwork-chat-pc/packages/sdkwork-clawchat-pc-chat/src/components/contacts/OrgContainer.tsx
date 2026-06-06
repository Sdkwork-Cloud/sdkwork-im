import React, { useEffect, useMemo, useState } from 'react';
import { Building2, Briefcase, FolderTree, GitBranch, MessageSquare, Search, UserPlus, Users, X } from 'lucide-react';
import { Avatar, cn } from '@sdkwork/clawchat-pc-commons';
import { toast } from '../Toast';
import { organizationDirectoryService } from '../../services/OrganizationDirectoryService';
import type { User as UserType } from '@sdkwork/clawchat-pc-types';
import type {
  OrgDepartment,
  OrgDepartmentNode,
  OrgOrganization,
  OrgOrganizationNode,
  OrganizationDirectoryPermission,
} from '../../services/OrganizationDirectoryService';

type OrgContainerProps = {
  onUserSelect: (user: UserType, deptName: string) => void;
  selectedUserId: string | null;
  onSendMessage?: (user: UserType) => void;
  searchQuery?: string;
};

type MemberManagementMode = 'add' | 'invite';

type MemberManagementForm = {
  assignmentType: string;
  departmentId: string;
  displayName: string;
  email: string;
  memberTarget: string;
  membershipType: string;
  phone: string;
  positionId: string;
  positionName: string;
  roleCodes: string;
};

const EMPTY_MEMBER_MANAGEMENT_FORM: MemberManagementForm = {
  assignmentType: 'primary',
  departmentId: '',
  displayName: '',
  email: '',
  memberTarget: '',
  membershipType: 'employee',
  phone: '',
  positionId: '',
  positionName: '',
  roleCodes: 'org.member',
};

function normalizeSearch(value: string): string {
  return value.trim().toLowerCase();
}

function matchesQuery(query: string, ...values: Array<string | null | undefined>): boolean {
  if (!query) {
    return true;
  }
  return values.some((value) => value?.toLowerCase().includes(query));
}

function flattenOrganizationTree(nodes: OrgOrganizationNode[]): OrgOrganization[] {
  return nodes.flatMap((node) => [node, ...flattenOrganizationTree(node.children)]);
}

function flattenDepartmentTree(nodes: OrgDepartmentNode[]): OrgDepartment[] {
  return nodes.flatMap((node) => [node, ...flattenDepartmentTree(node.children)]);
}

function sortDepartmentNodes(nodes: OrgDepartmentNode[]): OrgDepartmentNode[] {
  return [...nodes].sort((left, right) => left.order - right.order || left.name.localeCompare(right.name));
}

function buildDepartmentTreeForView(departments: OrgDepartment[]): OrgDepartmentNode[] {
  const byId = new Map<string, OrgDepartmentNode>();
  for (const department of departments) {
    byId.set(department.id, {
      ...department,
      children: [],
    });
  }

  const roots: OrgDepartmentNode[] = [];
  for (const department of byId.values()) {
    const parent = department.parentId ? byId.get(department.parentId) : undefined;
    if (parent) {
      parent.children.push(department);
    } else {
      roots.push(department);
    }
  }

  const sortRecursively = (nodes: OrgDepartmentNode[]): OrgDepartmentNode[] => sortDepartmentNodes(nodes).map((node) => ({
    ...node,
    children: sortRecursively(node.children),
  }));
  return sortRecursively(roots);
}

function filterOrganizationTree(nodes: OrgOrganizationNode[], query: string): OrgOrganizationNode[] {
  if (!query) {
    return nodes;
  }

  return nodes
    .map((node) => {
      const children = filterOrganizationTree(node.children, query);
      if (
        children.length > 0
        || matchesQuery(
          query,
          node.name,
          node.organizationKind,
          node.tenantBoundaryKind,
          node.dataBoundaryKind,
          node.verificationStatus,
          node.status,
        )
      ) {
        return {
          ...node,
          children,
        };
      }
      return null;
    })
    .filter(Boolean) as OrgOrganizationNode[];
}

function filterDepartmentTree(nodes: OrgDepartmentNode[], query: string): OrgDepartmentNode[] {
  if (!query) {
    return nodes;
  }

  return nodes
    .map((node) => {
      const children = filterDepartmentTree(node.children, query);
      if (children.length > 0 || matchesQuery(query, node.name, node.organizationId)) {
        return {
          ...node,
          children,
        };
      }
      return null;
    })
    .filter(Boolean) as OrgDepartmentNode[];
}

function selectDefaultOrganization(organizations: OrgOrganization[], tree: OrgOrganizationNode[]): OrgOrganization | null {
  const candidates = organizations.length > 0 ? organizations : flattenOrganizationTree(tree);
  const active = candidates.filter((organization) => !organization.status || organization.status.toLowerCase() === 'active');
  const selectable = active.length > 0 ? active : candidates;
  return selectable.find((organization) => (
    organization.tenantBoundaryKind === 'operating_subject'
    || organization.dataBoundaryKind === 'organization_isolated'
    || organization.dataBoundaryKind === 'regulated_isolated'
    || organization.appBoundaryEnabled === true
    || ['company', 'subsidiary', 'branch', 'division'].includes(organization.organizationKind ?? '')
  )) ?? selectable[0] ?? null;
}

function firstDepartment(nodes: OrgDepartmentNode[]): OrgDepartment | null {
  for (const node of nodes) {
    return node;
  }
  return null;
}

function boundaryText(organization: OrgOrganization): string | null {
  if (organization.tenantBoundaryKind === 'operating_subject') {
    return '独立主体';
  }
  if (organization.dataBoundaryKind === 'organization_isolated') {
    return '组织数据域';
  }
  if (organization.dataBoundaryKind === 'regulated_isolated') {
    return '监管数据域';
  }
  if (organization.appBoundaryEnabled) {
    return '应用边界';
  }
  return null;
}

function memberMatchesQuery(user: UserType, query: string): boolean {
  return matchesQuery(
    query,
    user.name,
    user.email,
    user.phone,
    user.position,
    user.departmentAssignmentId,
    user.organizationMembershipId,
    ...(user.roleCodes ?? []),
    ...(user.roleBindings ?? []).map((binding) => binding.roleCode),
    ...(user.positionAssignments ?? []).map((assignment) => assignment.positionName),
  );
}

function parseRoleCodes(value: string): string[] {
  return value
    .split(/[,\s，、]+/u)
    .map((item) => item.trim())
    .filter(Boolean);
}

function errorMessage(error: unknown, fallback: string): string {
  return error instanceof Error && error.message.trim().length > 0 ? error.message : fallback;
}

export const OrgContainer: React.FC<OrgContainerProps> = ({
  onUserSelect,
  selectedUserId,
  onSendMessage,
  searchQuery = '',
}) => {
  const [organizations, setOrganizations] = useState<OrgOrganization[]>([]);
  const [organizationTree, setOrganizationTree] = useState<OrgOrganizationNode[]>([]);
  const [currentOrganization, setCurrentOrganization] = useState<OrgOrganization | null>(null);
  const [departmentTree, setDepartmentTree] = useState<OrgDepartmentNode[]>([]);
  const [allDepartments, setAllDepartments] = useState<OrgDepartment[]>([]);
  const [currentDepartment, setCurrentDepartment] = useState<OrgDepartment | null>(null);
  const [users, setUsers] = useState<UserType[]>([]);
  const [selectedUser, setSelectedUser] = useState<UserType | null>(null);
  const [organizationPermission, setOrganizationPermission] = useState<OrganizationDirectoryPermission | null>(null);
  const [loading, setLoading] = useState(true);
  const [membersLoading, setMembersLoading] = useState(false);
  const [localSearch, setLocalSearch] = useState('');
  const [memberManagementMode, setMemberManagementMode] = useState<MemberManagementMode>('add');
  const [memberManagementOpen, setMemberManagementOpen] = useState(false);
  const [memberManagementForm, setMemberManagementForm] = useState<MemberManagementForm>(EMPTY_MEMBER_MANAGEMENT_FORM);
  const [memberSaving, setMemberSaving] = useState(false);

  const activeSearchQuery = normalizeSearch(localSearch || searchQuery);
  const visibleOrganizationTree = useMemo(
    () => filterOrganizationTree(organizationTree, activeSearchQuery),
    [activeSearchQuery, organizationTree],
  );
  const visibleDepartmentTree = useMemo(
    () => filterDepartmentTree(departmentTree, activeSearchQuery),
    [activeSearchQuery, departmentTree],
  );
  const visibleUsers = useMemo(
    () => users.filter((user) => memberMatchesQuery(user, activeSearchQuery)),
    [activeSearchQuery, users],
  );
  const canManageMembers = organizationPermission?.canManageMembers === true;
  const canInviteMembers = organizationPermission?.canInviteMembers === true;
  const memberManagementUnavailable = organizationPermission?.reason === 'missing_admin_capability';

  useEffect(() => {
    void loadRoot();
  }, []);

  const loadRoot = async () => {
    setLoading(true);
    try {
      const orgs = await organizationDirectoryService.getOrganizations();
      const orgTree = await organizationDirectoryService.getOrganizationTree();
      await organizationDirectoryService.getCurrentUser().catch(() => null);
      setOrganizations(orgs);
      setOrganizationTree(orgTree);
      setCurrentOrganization(null);
      setOrganizationPermission(null);
      setDepartmentTree([]);
      setAllDepartments([]);
      setCurrentDepartment(null);
      setUsers([]);
      setSelectedUser(null);
      setMemberManagementOpen(false);

      const defaultOrganization = selectDefaultOrganization(orgs, orgTree);
      if (defaultOrganization) {
        await loadOrganizationData(defaultOrganization, false);
      }
    } catch {
      setOrganizations([]);
      setOrganizationTree([]);
      setDepartmentTree([]);
      setAllDepartments([]);
      setCurrentDepartment(null);
      setUsers([]);
      setOrganizationPermission(null);
      setMemberManagementOpen(false);
      toast('加载部门数据失败', 'error');
    } finally {
      setLoading(false);
    }
  };

  const loadOrganizationData = async (organization: OrgOrganization, manageLoading = true) => {
    if (manageLoading) {
      setLoading(true);
    }
    setSelectedUser(null);
    setCurrentOrganization(organization);
    setOrganizationPermission(null);
    setMemberManagementOpen(false);

    try {
      const permission = await organizationDirectoryService.getOrganizationPermissions(organization.organizationId).catch(() => null);
      setOrganizationPermission(permission);

      let tree = await organizationDirectoryService.getDepartmentTree(organization.organizationId);
      let flatDepartments = flattenDepartmentTree(tree);
      if (flatDepartments.length === 0) {
        flatDepartments = await organizationDirectoryService.getDepartments(organization.organizationId);
        tree = buildDepartmentTreeForView(flatDepartments);
      }

      setDepartmentTree(tree);
      setAllDepartments(flatDepartments);

      const rootDepartment = firstDepartment(tree) ?? flatDepartments[0] ?? null;
      setMemberManagementForm({
        ...EMPTY_MEMBER_MANAGEMENT_FORM,
        departmentId: rootDepartment?.id ?? '',
      });
      if (rootDepartment) {
        await loadDepartmentData(rootDepartment.id, flatDepartments, organization);
      } else {
        setCurrentDepartment(null);
        setUsers([]);
      }
    } catch {
      setDepartmentTree([]);
      setAllDepartments([]);
      setCurrentDepartment(null);
      setUsers([]);
      setOrganizationPermission(null);
      setMemberManagementOpen(false);
      toast('加载部门数据失败', 'error');
    } finally {
      if (manageLoading) {
        setLoading(false);
      }
    }
  };

  const loadDepartmentData = async (
    departmentId: string,
    knownDepartments = allDepartments,
    organization = currentOrganization,
  ) => {
    setMembersLoading(true);
    setSelectedUser(null);
    try {
      const department = knownDepartments.find((item) => item.id === departmentId) ?? null;
      setCurrentDepartment(department);
      const deptUsers = await organizationDirectoryService.getUsersByDepartment(departmentId);
      setUsers(deptUsers);
    } catch {
      setUsers([]);
      toast('加载部门成员失败', 'error');
    } finally {
      setMembersLoading(false);
    }
  };

  const handleOrganizationNavigate = (organization: OrgOrganization) => {
    void loadOrganizationData(organization);
  };

  const handleNavigate = (dept: OrgDepartment) => {
    setMemberManagementForm((form) => ({ ...form, departmentId: dept.id }));
    void loadDepartmentData(dept.id);
  };

  const openMemberManagement = (mode: MemberManagementMode) => {
    if (!currentOrganization) {
      toast('请选择组织', 'error');
      return;
    }
    if (!canManageMembers) {
      toast(memberManagementUnavailable ? '组织成员管理能力未接入' : '没有组织成员管理权限', 'error');
      return;
    }
    if (mode === 'invite' && !canInviteMembers) {
      toast('组织成员邀请能力未接入', 'error');
      return;
    }

    const departmentId = currentDepartment?.id ?? allDepartments[0]?.id ?? '';
    setMemberManagementMode(mode);
    setMemberManagementForm({
      ...EMPTY_MEMBER_MANAGEMENT_FORM,
      departmentId,
    });
    setMemberManagementOpen(true);
  };

  const closeMemberManagement = () => {
    setMemberManagementOpen(false);
    setMemberSaving(false);
  };

  const updateMemberManagementForm = (field: keyof MemberManagementForm, value: string) => {
    setMemberManagementForm((form) => ({ ...form, [field]: value }));
  };

  const handleMemberManagementSubmit = async (event: React.FormEvent<HTMLFormElement>) => {
    event.preventDefault();
    if (!currentOrganization) {
      toast('请选择组织', 'error');
      return;
    }
    if (!canManageMembers) {
      toast('没有组织成员管理权限', 'error');
      return;
    }

    const departmentId = memberManagementForm.departmentId.trim();
    if (!departmentId) {
      toast('请选择部门', 'error');
      return;
    }

    const organizationId = currentOrganization.organizationId;
    setMemberSaving(true);
    try {
      const sharedInput = {
        assignmentType: memberManagementForm.assignmentType.trim() || undefined,
        departmentId,
        membershipType: memberManagementForm.membershipType.trim() || undefined,
        organizationId,
        positionId: memberManagementForm.positionId.trim() || undefined,
        positionName: memberManagementForm.positionName.trim() || undefined,
        roleCodes: parseRoleCodes(memberManagementForm.roleCodes),
      };

      if (memberManagementMode === 'invite') {
        await organizationDirectoryService.inviteOrganizationMember({
          ...sharedInput,
          displayName: memberManagementForm.displayName.trim() || undefined,
          email: memberManagementForm.email.trim() || undefined,
          phone: memberManagementForm.phone.trim() || undefined,
        });
      } else {
        const userId = memberManagementForm.memberTarget.trim();
        if (!userId) {
          toast('请输入用户ID', 'error');
          setMemberSaving(false);
          return;
        }
        await organizationDirectoryService.addOrganizationMember({
          ...sharedInput,
          userId,
        });
      }

      setMemberManagementOpen(false);
      setMemberManagementForm({
        ...EMPTY_MEMBER_MANAGEMENT_FORM,
        departmentId,
      });
      await loadDepartmentData(departmentId, allDepartments, currentOrganization);
      toast('成员管理已提交', 'success');
    } catch (error) {
      toast(errorMessage(error, '成员管理提交失败'), 'error');
    } finally {
      setMemberSaving(false);
    }
  };

  const handleUserSelect = (user: UserType) => {
    setSelectedUser(user);
    onUserSelect(user, currentDepartmentName());
  };

  const handleSendMessage = (user: UserType) => {
    handleUserSelect(user);
    if (onSendMessage) {
      onSendMessage(user);
    } else {
      toast('聊天能力未接入，无法发起会话', 'error');
    }
  };

  const currentDepartmentName = () => {
    return currentDepartment?.name
      || currentOrganization?.name
      || '未知组织';
  };

  const renderOrganizationNode = (organization: OrgOrganizationNode, depth = 0): React.ReactNode => {
    const selected = currentOrganization?.organizationId === organization.organizationId;
    const boundary = boundaryText(organization);
    return (
      <div key={organization.organizationId}>
        <button
          type="button"
          className={cn(
            'group flex w-full items-center gap-2 px-3 py-2 text-left transition-colors hover:bg-white/5',
            selected && 'bg-white/10 hover:bg-white/10',
          )}
          style={{ paddingLeft: `${12 + depth * 14}px` }}
          onClick={() => handleOrganizationNavigate(organization)}
        >
          {organization.children.length > 0 ? (
            <GitBranch size={14} className="shrink-0 text-gray-500" />
          ) : (
            <Building2 size={14} className="shrink-0 text-gray-500" />
          )}
          <span className={cn('min-w-0 flex-1 truncate text-[13px]', selected ? 'text-gray-100' : 'text-gray-300')}>
            {organization.name}
          </span>
          {boundary && (
            <span className="shrink-0 rounded border border-cyan-400/20 px-1.5 py-0.5 text-[10px] text-cyan-300">
              {boundary}
            </span>
          )}
        </button>
        {organization.children.map((child) => renderOrganizationNode(child, depth + 1))}
      </div>
    );
  };

  const renderDepartmentNode = (department: OrgDepartmentNode, depth = 0): React.ReactNode => {
    const selected = currentDepartment?.id === department.id;
    return (
      <div key={department.id}>
        <button
          type="button"
          className={cn(
            'group flex w-full items-center gap-2 px-3 py-2 text-left transition-colors hover:bg-white/5',
            selected && 'bg-white/10 hover:bg-white/10',
          )}
          style={{ paddingLeft: `${12 + depth * 14}px` }}
          onClick={() => handleNavigate(department)}
        >
          <FolderTree size={14} className="shrink-0 text-gray-500" />
          <span className={cn('min-w-0 flex-1 truncate text-[13px]', selected ? 'text-gray-100' : 'text-gray-300')}>
            {department.name}
          </span>
          {department.children.length > 0 && (
            <span className="shrink-0 rounded bg-white/5 px-1.5 py-0.5 text-[10px] text-gray-500">
              {department.children.length}
            </span>
          )}
        </button>
        {department.children.map((child) => renderDepartmentNode(child, depth + 1))}
      </div>
    );
  };

  return (
    <div className="flex flex-1 min-w-0 bg-[#1e1e1e]">
      <div className="flex w-[260px] shrink-0 flex-col border-r border-white/5 bg-[#202020]">
        <div className="border-b border-white/5 px-4 py-3">
          <div className="mb-3 flex items-center justify-between">
            <div className="flex items-center gap-2 text-sm font-medium text-gray-200">
              <Building2 size={16} className="text-gray-500" />
              组织
            </div>
            <span className="text-xs text-gray-500">{organizations.length}</span>
          </div>
          <div className="relative">
            <Search size={14} className="absolute left-2.5 top-1/2 -translate-y-1/2 text-gray-500" />
            <input
              value={localSearch}
              onChange={(event) => setLocalSearch(event.target.value)}
              placeholder={searchQuery ? searchQuery : '搜索组织、部门、成员'}
              className="h-8 w-full rounded-md border border-white/5 bg-[#181818] pl-8 pr-2 text-[13px] text-gray-200 outline-none transition-colors placeholder:text-gray-600 focus:border-indigo-500/40"
            />
          </div>
        </div>

        <div className="min-h-0 flex-1 overflow-y-auto py-2 custom-scrollbar">
          {loading ? (
            <div className="px-4 py-3 text-sm text-gray-500">加载中...</div>
          ) : visibleOrganizationTree.length > 0 ? (
            visibleOrganizationTree.map((organization) => renderOrganizationNode(organization))
          ) : (
            <div className="px-4 py-6 text-center text-sm text-gray-500">暂无组织数据</div>
          )}
        </div>
      </div>

      <div className="flex min-w-0 flex-1 flex-col border-r border-white/5">
        <div className="flex min-h-0 flex-1">
          <div className="flex w-[280px] shrink-0 flex-col border-r border-white/5">
            <div className="flex h-10 shrink-0 items-center justify-between border-b border-white/5 px-4">
              <div className="flex items-center gap-2 text-sm font-medium text-gray-300">
                <FolderTree size={15} className="text-gray-500" />
                部门
              </div>
              <span className="text-xs text-gray-500">{allDepartments.length}</span>
            </div>
            <div className="min-h-0 flex-1 overflow-y-auto py-2 custom-scrollbar">
              {loading ? (
                <div className="px-4 py-3 text-sm text-gray-500">加载中...</div>
              ) : visibleDepartmentTree.length > 0 ? (
                visibleDepartmentTree.map((department) => renderDepartmentNode(department))
              ) : (
                <div className="px-4 py-6 text-center text-sm text-gray-500">暂无部门数据</div>
              )}
            </div>
          </div>

          <div className="flex min-w-0 flex-1 flex-col">
            <div className="flex h-10 shrink-0 items-center justify-between border-b border-white/5 px-4">
              <div className="flex min-w-0 items-center gap-2 text-sm font-medium text-gray-300">
                <Users size={15} className="shrink-0 text-gray-500" />
                <span className="truncate">{currentDepartment?.name ?? '成员'}</span>
              </div>
              <div className="flex shrink-0 items-center gap-2">
                <span className="text-xs text-gray-500">{visibleUsers.length}</span>
                {currentOrganization && canManageMembers && (
                  <button
                    type="button"
                    className="flex h-7 w-7 items-center justify-center rounded-md text-gray-400 transition-colors hover:bg-indigo-500/10 hover:text-indigo-300"
                    title="添加成员"
                    onClick={() => openMemberManagement('add')}
                  >
                    <UserPlus size={15} />
                  </button>
                )}
              </div>
            </div>

            {memberManagementOpen && currentOrganization && (
              <form
                className="shrink-0 border-b border-white/5 bg-[#1b1b1b] px-4 py-3"
                onSubmit={handleMemberManagementSubmit}
              >
                <div className="mb-3 flex items-center justify-between gap-3">
                  <div className="flex min-w-0 items-center gap-2">
                    <UserPlus size={15} className="shrink-0 text-indigo-300" />
                    <span className="truncate text-sm font-medium text-gray-200">
                      {memberManagementMode === 'invite' ? '邀请成员' : '添加成员'}
                    </span>
                  </div>
                  <button
                    type="button"
                    className="flex h-7 w-7 items-center justify-center rounded-md text-gray-500 transition-colors hover:bg-white/5 hover:text-gray-200"
                    title="关闭"
                    onClick={closeMemberManagement}
                  >
                    <X size={15} />
                  </button>
                </div>

                <div className="mb-3 flex rounded-md border border-white/5 bg-[#151515] p-0.5">
                  <button
                    type="button"
                    className={cn(
                      'h-7 flex-1 rounded px-2 text-xs transition-colors',
                      memberManagementMode === 'add' ? 'bg-white/10 text-gray-100' : 'text-gray-500 hover:text-gray-300',
                    )}
                    onClick={() => setMemberManagementMode('add')}
                  >
                    添加
                  </button>
                  <button
                    type="button"
                    className={cn(
                      'h-7 flex-1 rounded px-2 text-xs transition-colors',
                      memberManagementMode === 'invite' ? 'bg-white/10 text-gray-100' : 'text-gray-500 hover:text-gray-300',
                    )}
                    disabled={!canInviteMembers}
                    onClick={() => openMemberManagement('invite')}
                  >
                    邀请
                  </button>
                </div>

                <div className="grid grid-cols-2 gap-2">
                  <select
                    value={memberManagementForm.departmentId}
                    onChange={(event) => updateMemberManagementForm('departmentId', event.target.value)}
                    className="h-8 min-w-0 rounded-md border border-white/5 bg-[#181818] px-2 text-[12px] text-gray-200 outline-none focus:border-indigo-500/40"
                  >
                    {allDepartments.map((department) => (
                      <option key={department.id} value={department.id}>
                        {department.name}
                      </option>
                    ))}
                  </select>
                  <select
                    value={memberManagementForm.assignmentType}
                    onChange={(event) => updateMemberManagementForm('assignmentType', event.target.value)}
                    className="h-8 min-w-0 rounded-md border border-white/5 bg-[#181818] px-2 text-[12px] text-gray-200 outline-none focus:border-indigo-500/40"
                  >
                    <option value="primary">主岗</option>
                    <option value="secondary">兼岗</option>
                    <option value="acting">代理</option>
                  </select>
                  {memberManagementMode === 'add' ? (
                    <input
                      value={memberManagementForm.memberTarget}
                      onChange={(event) => updateMemberManagementForm('memberTarget', event.target.value)}
                      placeholder="用户ID"
                      className="col-span-2 h-8 min-w-0 rounded-md border border-white/5 bg-[#181818] px-2 text-[12px] text-gray-200 outline-none placeholder:text-gray-600 focus:border-indigo-500/40"
                    />
                  ) : (
                    <>
                      <input
                        value={memberManagementForm.displayName}
                        onChange={(event) => updateMemberManagementForm('displayName', event.target.value)}
                        placeholder="姓名"
                        className="h-8 min-w-0 rounded-md border border-white/5 bg-[#181818] px-2 text-[12px] text-gray-200 outline-none placeholder:text-gray-600 focus:border-indigo-500/40"
                      />
                      <input
                        value={memberManagementForm.email}
                        onChange={(event) => updateMemberManagementForm('email', event.target.value)}
                        placeholder="邮箱"
                        className="h-8 min-w-0 rounded-md border border-white/5 bg-[#181818] px-2 text-[12px] text-gray-200 outline-none placeholder:text-gray-600 focus:border-indigo-500/40"
                      />
                    </>
                  )}
                  <input
                    value={memberManagementForm.positionId}
                    onChange={(event) => updateMemberManagementForm('positionId', event.target.value)}
                    placeholder="岗位ID"
                    className="h-8 min-w-0 rounded-md border border-white/5 bg-[#181818] px-2 text-[12px] text-gray-200 outline-none placeholder:text-gray-600 focus:border-indigo-500/40"
                  />
                  <input
                    value={memberManagementForm.roleCodes}
                    onChange={(event) => updateMemberManagementForm('roleCodes', event.target.value)}
                    placeholder="角色编码"
                    className="h-8 min-w-0 rounded-md border border-white/5 bg-[#181818] px-2 text-[12px] text-gray-200 outline-none placeholder:text-gray-600 focus:border-indigo-500/40"
                  />
                </div>

                <div className="mt-3 flex items-center justify-between gap-3">
                  <div className="min-w-0 truncate text-[11px] text-gray-500">
                    {organizationPermission?.roleCodes.join(', ') || currentOrganization.name}
                  </div>
                  <button
                    type="submit"
                    disabled={memberSaving || allDepartments.length === 0}
                    className="h-8 shrink-0 rounded-md bg-indigo-500 px-3 text-xs font-medium text-white transition-colors hover:bg-indigo-400 disabled:cursor-not-allowed disabled:bg-gray-700 disabled:text-gray-400"
                  >
                    {memberSaving ? '提交中' : '提交'}
                  </button>
                </div>
              </form>
            )}

            {!canManageMembers && memberManagementUnavailable && (
              <div className="shrink-0 border-b border-white/5 px-4 py-2 text-xs text-amber-300">
                组织成员管理能力未接入
              </div>
            )}

            <div className="min-h-0 flex-1 overflow-y-auto p-2 custom-scrollbar">
              {membersLoading ? (
                <div className="p-6 text-center text-sm text-gray-500">加载成员中...</div>
              ) : visibleUsers.length > 0 ? (
                <div className="flex flex-col gap-1">
                  {visibleUsers.map((user) => (
                    <div
                      key={user.departmentAssignmentId ?? user.id}
                      className={cn(
                        'group flex cursor-pointer items-center gap-3 rounded-md px-3 py-2.5 transition-colors hover:bg-white/5',
                        (selectedUserId === user.id || selectedUser?.departmentAssignmentId === user.departmentAssignmentId) && 'bg-white/10 hover:bg-white/10',
                      )}
                      onClick={() => handleUserSelect(user)}
                    >
                      <Avatar src={user.avatar} alt={user.name} className="h-9 w-9 shrink-0 rounded bg-[#2b2b2d]" />
                      <div className="min-w-0 flex-1">
                        <div className="flex min-w-0 items-center gap-2">
                          <span className="truncate text-[14px] font-medium text-gray-200">{user.name}</span>
                          {user.assignmentType && (
                            <span className="shrink-0 rounded bg-white/5 px-1.5 py-0.5 text-[10px] text-gray-500">
                              {user.assignmentType}
                            </span>
                          )}
                        </div>
                        <div className="mt-0.5 flex min-w-0 items-center gap-2 text-[12px] text-gray-500">
                          {user.position && (
                            <span className="inline-flex min-w-0 items-center gap-1 truncate">
                              <Briefcase size={12} className="shrink-0" />
                              <span className="truncate">{user.position}</span>
                            </span>
                          )}
                          {user.email && <span className="truncate">{user.email}</span>}
                        </div>
                        {(user.roleCodes?.length || user.roleBindings?.length || user.positionAssignments?.length) ? (
                          <div className="mt-1 flex flex-wrap gap-1">
                            {(user.roleCodes ?? []).slice(0, 3).map((roleCode) => (
                              <span key={roleCode} className="rounded border border-indigo-400/15 px-1.5 py-0.5 text-[10px] text-indigo-300">
                                {roleCode}
                              </span>
                            ))}
                            {user.roleBindings && user.roleBindings.length > 3 && (
                              <span className="rounded border border-white/10 px-1.5 py-0.5 text-[10px] text-gray-500">
                                +{user.roleBindings.length - 3}
                              </span>
                            )}
                            {user.positionAssignments && user.positionAssignments.length > 1 && (
                              <span className="rounded border border-white/10 px-1.5 py-0.5 text-[10px] text-gray-500">
                                {user.positionAssignments.length}岗位
                              </span>
                            )}
                          </div>
                        ) : null}
                      </div>
                      <button
                        type="button"
                        className="flex h-8 w-8 shrink-0 items-center justify-center rounded-md text-gray-500 opacity-0 transition-colors hover:bg-indigo-500/10 hover:text-indigo-300 group-hover:opacity-100"
                        title="发消息"
                        onClick={(event) => {
                          event.stopPropagation();
                          handleSendMessage(user);
                        }}
                      >
                        <MessageSquare size={15} />
                      </button>
                    </div>
                  ))}
                </div>
              ) : (
                <div className="p-8 text-center text-sm text-gray-500">暂无成员数据</div>
              )}
            </div>
          </div>
        </div>
      </div>
    </div>
  );
};
