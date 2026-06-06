export interface Tenant {
  id: string;
  name: string;
  plan: 'Enterprise' | 'Business' | 'Pro';
  users: string;
  status: 'active' | 'warning';
  revenue: string;
  region: string;
}

export interface GetTenantsResponse {
  data: Tenant[];
  total: number;
}

class TenantService {
  private mockTenants: Tenant[] = [
    { id: 'T-8491', name: 'Acme Corporation', plan: 'Enterprise', users: '12.4K', status: 'active', revenue: '$4,200', region: 'US-East' },
    { id: 'T-8492', name: 'Global Tech Inc.', plan: 'Business', users: '2.8K', status: 'active', revenue: '$850', region: 'EU-West' },
    { id: 'T-8493', name: 'Startup Hub', plan: 'Pro', users: '84', status: 'active', revenue: '$99', region: 'AP-South' },
    { id: 'T-8494', name: 'Design Co.', plan: 'Business', users: '420', status: 'warning', revenue: '$250', region: 'US-West' },
    { id: 'T-8495', name: 'Stark Industries', plan: 'Enterprise', users: '45.2K', status: 'active', revenue: '$18,500', region: 'Global' },
  ];

  async getTenants(params: { search?: string }): Promise<GetTenantsResponse> {
    await new Promise(resolve => setTimeout(resolve, 200));

    let filtered = this.mockTenants;
    if (params.search) {
      const q = params.search.toLowerCase();
      filtered = filtered.filter(t => t.name.toLowerCase().includes(q) || t.id.toLowerCase().includes(q) || t.region.toLowerCase().includes(q));
    }

    return {
      data: filtered,
      total: 8240
    };
  }
}

export const tenantService = new TenantService();
