export interface Group {
  id: string;
  name: string;
  type: 'public' | 'private';
  members: number;
  owner: string;
  status: 'active' | 'archived';
  messagesToDay: number;
  created: string;
}

export interface GetGroupsResponse {
  data: Group[];
  total: number;
}

class GroupService {
  private mockGroups: Group[] = [
    { id: 'G-1001', name: '全员交流群', type: 'public', members: 1240, owner: 'Admin', status: 'active', messagesToDay: 4210, created: '2023-01-15' },
    { id: 'G-1002', name: '产品前线', type: 'private', members: 45, owner: '张三', status: 'active', messagesToDay: 852, created: '2023-03-22' },
    { id: 'G-1003', name: 'Q3 项目作战室', type: 'private', members: 12, owner: '李四', status: 'active', messagesToDay: 124, created: '2023-06-10' },
    { id: 'G-1004', name: '技术支持中心', type: 'public', members: 320, owner: 'System', status: 'active', messagesToDay: 532, created: '2023-02-05' },
    { id: 'G-1005', name: '已归档-旧项目', type: 'private', members: 8, owner: '王五', status: 'archived', messagesToDay: 0, created: '2022-11-10' },
  ];

  async getGroups(params: { page: number; pageSize: number; search?: string }): Promise<GetGroupsResponse> {
    await new Promise(resolve => setTimeout(resolve, 300));
    let filtered = this.mockGroups;
    if (params.search) {
      const q = params.search.toLowerCase();
      filtered = filtered.filter(g => 
        g.name.toLowerCase().includes(q) || 
        g.owner.toLowerCase().includes(q) ||
        g.id.toLowerCase().includes(q)
      );
    }
    const start = (params.page - 1) * params.pageSize;
    const end = start + params.pageSize;

    return {
      data: filtered.slice(start, end),
      total: filtered.length
    };
  }
}

export const groupService = new GroupService();
