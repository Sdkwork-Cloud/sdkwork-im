export interface AdminMetrics {
  systemLoad: { value: string; trend: string; isUp: boolean };
  activeTenants: { value: string; trend: string; isUp: boolean };
  activeConnections: { value: string; trend: string; isUp: boolean };
  globalNodes: { value: string; trend: string; isUp: boolean };
}

export interface NetworkThroughput {
  egress: number;
  ingress: number;
}

export interface SystemAnomaly {
  id: string;
  type: 'critical' | 'warning' | 'info';
  tenant: string;
  message: string;
  time: string;
}

export interface AdminDashboardData {
  metrics: AdminMetrics;
  throughput: NetworkThroughput[];
  anomalies: SystemAnomaly[];
}

class AdminDashboardService {
  async getDashboardData(): Promise<AdminDashboardData> {
    await new Promise(resolve => setTimeout(resolve, 200));

    return {
      metrics: {
        systemLoad: { value: "28%", trend: "-2%", isUp: false },
        activeTenants: { value: "8,240", trend: "+12", isUp: true },
        activeConnections: { value: "1.2M", trend: "+45k", isUp: true },
        globalNodes: { value: "12", trend: "0", isUp: true },
      },
      throughput: [
        { egress: 30, ingress: 18 },
        { egress: 45, ingress: 27 },
        { egress: 25, ingress: 15 },
        { egress: 60, ingress: 36 },
        { egress: 85, ingress: 51 },
        { egress: 40, ingress: 24 },
        { egress: 70, ingress: 42 },
        { egress: 90, ingress: 54 },
        { egress: 50, ingress: 30 },
        { egress: 65, ingress: 39 },
        { egress: 35, ingress: 21 },
        { egress: 80, ingress: 48 },
      ],
      anomalies: [
        { id: '1', type: "critical", tenant: "T-4829", message: "Database connection pool exhausted", time: "2m ago" },
        { id: '2', type: "warning", tenant: "T-9921", message: "Spike in auth failures (120 req/s)", time: "15m ago" },
        { id: '3', type: "info", tenant: "System", message: "Routine backup completed successfully", time: "1h ago" },
        { id: '4', type: "warning", tenant: "T-1021", message: "Payment gateway latency > 2s", time: "2.5h ago" },
      ]
    };
  }
}

export const adminDashboardService = new AdminDashboardService();
