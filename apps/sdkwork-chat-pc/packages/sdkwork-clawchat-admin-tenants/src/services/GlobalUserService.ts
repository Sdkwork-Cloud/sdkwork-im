export interface GlobalUser {
  id: string;
  uin: string;
  name: string;
  email: string;
  tenant: string;
  security: string;
  status: 'active' | 'banned' | 'warning';
}

export interface GetGlobalUsersResponse {
  data: GlobalUser[];
  total: number;
}

class GlobalUserService {
  private mockUsers: GlobalUser[] = [
    { id: '1', uin: "U-9021-848", name: "Alice Walker", email: "alice@acmecorp.com", tenant: "Acme Corp (T-1001)", security: "MFA Enforced", status: "active" },
    { id: '2', uin: "U-9021-849", name: "Bob Smith", email: "bob.smith@nova.io", tenant: "Nova Labs (T-2201)", security: "Password Only", status: "active" },
    { id: '3', uin: "U-8812-421", name: "Unknown User", email: "hacker@malicious.com", tenant: "N/A (Orphaned)", security: "Breached Logins", status: "banned" },
    { id: '4', uin: "U-1024-555", name: "Charlie Davis", email: "charlie.d@global.net", tenant: "Global Tech (T-1045)", security: "MFA Enforced", status: "warning" },
  ];

  async getGlobalUsers(params: { search?: string, status?: string }): Promise<GetGlobalUsersResponse> {
    await new Promise(resolve => setTimeout(resolve, 200));

    let filtered = this.mockUsers;
    if (params.search) {
      const q = params.search.toLowerCase();
      filtered = filtered.filter(u => 
        u.name.toLowerCase().includes(q) || 
        u.email.toLowerCase().includes(q) || 
        u.uin.toLowerCase().includes(q)
      );
    }
    
    if (params.status && params.status !== 'All Global Statuses') {
      const statusMap: Record<string, string> = {
        'Active Accounts': 'active',
        'Banned Globally': 'banned',
        'Pending Verification': 'warning'
      };
      const filterStatus = statusMap[params.status];
      if (filterStatus) {
        filtered = filtered.filter(u => u.status === filterStatus);
      }
    }

    return {
      data: filtered,
      total: 12800000 + this.mockUsers.length - 4
    };
  }

  async updateUserStatus(id: string, status: GlobalUser['status']): Promise<void> {
    await new Promise(resolve => setTimeout(resolve, 200));
    const user = this.mockUsers.find(u => u.id === id);
    if (user) {
      user.status = status;
    }
  }

  async deleteUser(id: string): Promise<void> {
    await new Promise(resolve => setTimeout(resolve, 200));
    this.mockUsers = this.mockUsers.filter(u => u.id !== id);
  }
}

export const globalUserService = new GlobalUserService();
