export interface ServerNode {
  region: string;
  status: 'healthy' | 'warning' | 'error';
  cpu: number;
  mem: number;
  connections: string;
}

export interface MetricItem {
  title: string;
  value: string;
  usage: number;
}

export interface InfraStatusData {
  metrics: {
    connectionPool: MetricItem;
    dbIops: MetricItem;
    redisHitRate: MetricItem;
  };
  nodes: ServerNode[];
}

class InfraStatusService {
  async getStatusData(): Promise<InfraStatusData> {
    await new Promise(resolve => setTimeout(resolve, 300));
    return {
      metrics: {
        connectionPool: { title: "Global Connection Pool", value: "842.5K", usage: 65 },
        dbIops: { title: "Database IOPS (Avg)", value: "42,050", usage: 45 },
        redisHitRate: { title: "Redis Cache Hit Rate", value: "98.2%", usage: 98 },
      },
      nodes: [
        { region: 'us-east-1', status: 'healthy', cpu: 42, mem: 60, connections: '124K' },
        { region: 'us-west-2', status: 'healthy', cpu: 38, mem: 55, connections: '98K' },
        { region: 'eu-central-1', status: 'warning', cpu: 85, mem: 92, connections: '180K' },
        { region: 'ap-southeast-1', status: 'healthy', cpu: 20, mem: 40, connections: '45K' },
      ]
    };
  }
}

export const infraStatusService = new InfraStatusService();
