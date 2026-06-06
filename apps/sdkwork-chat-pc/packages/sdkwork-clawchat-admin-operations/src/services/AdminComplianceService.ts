import { mockAdminFetch } from '@sdkwork/clawchat-pc-commons';

export interface AuditLog {
  id: string;
  time: string;
  actor: string;
  action: string;
  resource: string;
  ip: string;
}

export interface ComplianceData {
  systemSecure: boolean;
  legalHolds: number;
  uptime: string;
  auditLogs: AuditLog[];
}

class AdminComplianceService {
  async getComplianceData(searchTerm: string): Promise<ComplianceData> {
    const logs = [
      { id: '1', time: "2023-10-24 14:22:10", actor: "root_admin_1", action: "Configure Platform Plan", resource: "Plan: Enterprise Grid", ip: "104.28.19.12" },
      { id: '2', time: "2023-10-24 13:10:05", actor: "sys_automated", action: "Apply Security Patch", resource: "Cluster: EU-West-1", ip: "10.0.0.4" },
      { id: '3', time: "2023-10-24 11:45:00", actor: "root_admin_2", action: "Grant Tenant Admin Access", resource: "Tenant: T-1045", ip: "192.168.1.150" },
      { id: '4', time: "2023-10-23 09:30:12", actor: "support_tier_2", action: "Initiate Legal Hold", resource: "Tenant: T-0982 (DataVault)", ip: "35.190.22.4" },
      { id: '5', time: "2023-10-23 08:15:22", actor: "sys_automated", action: "Daily Backup Completed", resource: "Database: Main_US_East", ip: "10.0.0.8" },
      { id: '6', time: "2023-10-22 16:40:05", actor: "root_admin_1", action: "Modify Rate Limits", resource: "API Gateway: Global", ip: "104.28.19.12" }
    ];

    const mockData = {
      systemSecure: true,
      legalHolds: 14,
      uptime: "99.998%",
      auditLogs: searchTerm ? logs.filter(l => l.action.toLowerCase().includes(searchTerm.toLowerCase()) || l.actor.includes(searchTerm) || l.ip.includes(searchTerm)) : logs
    };

    return mockAdminFetch(`/compliance/dashboard?search=${encodeURIComponent(searchTerm)}`, mockData);
  }
}

export const adminComplianceService = new AdminComplianceService();
