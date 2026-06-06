import { mockConsoleFetch, mockConsolePost } from '@sdkwork/clawchat-pc-commons';

export interface User {
  id: string;
  name: string;
  email: string;
  role: 'admin' | 'member';
  department: string;
  status: 'active' | 'offline' | 'disabled';
  lastLogin: string;
}

export interface GetUsersResponse {
  data: User[];
  total: number;
}

class UserService {
  private mockUsers: User[] = [
    { id: '1', name: '张三', email: 'zhangsan@acme.com', role: 'admin', department: '产品部', status: 'active', lastLogin: '10分钟前' },
    { id: '2', name: '李四', email: 'lisi@acme.com', role: 'member', department: '研发部', status: 'active', lastLogin: '1小时前' },
    { id: '3', name: '王五', email: 'wangwu@acme.com', role: 'member', department: '研发部', status: 'offline', lastLogin: '昨天' },
    { id: '4', name: '赵六', email: 'zhaoliu@acme.com', role: 'member', department: '设计部', status: 'disabled', lastLogin: '1周前' },
    { id: '5', name: '陈七', email: 'chenqi@acme.com', role: 'admin', department: '管理层', status: 'active', lastLogin: '2小时前' },
  ];

  async getUsers(params: { page: number; pageSize: number; search?: string }): Promise<GetUsersResponse> {
    let filtered = this.mockUsers;
    if (params.search) {
      const q = params.search.toLowerCase();
      filtered = filtered.filter(u => 
        u.name.toLowerCase().includes(q) || 
        u.email.toLowerCase().includes(q) ||
        u.department.toLowerCase().includes(q)
      );
    }

    const start = (params.page - 1) * params.pageSize;
    const end = start + params.pageSize;

    const mockData = {
      data: filtered.slice(start, end),
      total: filtered.length
    };
    
    return mockConsoleFetch(`/users/list?page=${params.page}&pageSize=${params.pageSize}${params.search ? `&search=${encodeURIComponent(params.search)}` : ''}`, mockData);
  }

  async deleteUser(id: string): Promise<void> {
    this.mockUsers = this.mockUsers.filter(u => u.id !== id);
    return mockConsolePost(`/users/${id}/delete`, {}, undefined);
  }
}

export const userService = new UserService();
