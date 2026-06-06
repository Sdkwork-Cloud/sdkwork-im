import { mockConsoleFetch, mockConsolePost } from '@sdkwork/clawchat-pc-commons';

export interface Role {
  id: string;
  name: string;
  desc: string;
  count: number;
  system: boolean;
}

export interface GetRolesResponse {
  data: Role[];
  total: number;
}

class RoleService {
  private mockRoles: Role[] = [
    { id: '1', name: '超级管理员', desc: '拥有企业所有模块的完全控制权。', count: 2, system: true },
    { id: '2', name: '安全合规管理员', desc: '管理安全策略、审计日志和数据防泄漏。', count: 3, system: true },
    { id: '3', name: '部门管理员', desc: '管理本部门的人员和基础通信设置。', count: 15, system: false },
    { id: '4', name: '开发集成者', desc: '管理自建应用、第三方集成及 Webhook。', count: 8, system: false },
    { id: '5', name: '普通员工', desc: '默认角色，允许基础的聊天及应用使用。', count: 1215, system: true }
  ];

  async getRoles(): Promise<GetRolesResponse> {
    const mockData = {
      data: this.mockRoles,
      total: this.mockRoles.length
    };
    return mockConsoleFetch('/roles/list', mockData);
  }

  async updateRole(id: string, updates: Partial<Role>): Promise<Role> {
    const role = this.mockRoles.find(r => r.id === id);
    if (!role) throw new Error('Role not found');
    Object.assign(role, updates);
    return mockConsolePost(`/roles/${id}/update`, updates, role);
  }
}

export const roleService = new RoleService();
