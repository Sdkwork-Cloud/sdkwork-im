export interface BillingStatItem {
  title: string;
  value: string;
  trend: string;
  isUp: boolean;
}

export interface PlanDistribution {
  name: string;
  percent: number;
  users: number;
}

export interface TransactionInfo {
  id: string;
  tenant: string;
  tenantId: string;
  plan: string;
  amount: string;
  status: 'paid' | 'failed' | 'pending';
  date: string;
}

export interface AdminBillingData {
  stats: Record<string, BillingStatItem>;
  plans: PlanDistribution[];
  transactions: TransactionInfo[];
}

class AdminBillingService {
  async getBillingData(): Promise<AdminBillingData> {
    await new Promise(resolve => setTimeout(resolve, 300));
    return {
      stats: {
        mrr: { title: "Monthly Recurring Revenue", value: "$4.2M", trend: "+5.2%", isUp: true },
        active: { title: "Active Subscriptions", value: "8,102", trend: "+124", isUp: true },
        net: { title: "Net Revenue Retention", value: "112%", trend: "+2.1%", isUp: true },
        churn: { title: "Churn Rate (MRR)", value: "1.2%", trend: "-0.4%", isUp: false },
      },
      plans: [
        { name: "Enterprise Grid", percent: 42, users: 3400 },
        { name: "Business Plus", percent: 35, users: 2800 },
        { name: "Standard", percent: 18, users: 1452 },
        { name: "Free Tier", percent: 5, users: 450 }
      ],
      transactions: [
        { id: '1', tenant: "Acme Corp", tenantId: "T-1001", plan: "Enterprise Grid", amount: "$45,000.00", status: "paid", date: "Today, 09:42 AM" },
        { id: '2', tenant: "Global Tech", tenantId: "T-1045", plan: "Business Plus", amount: "$12,500.00", status: "paid", date: "Today, 08:15 AM" },
        { id: '3', tenant: "Nova Labs", tenantId: "T-2201", plan: "Enterprise Grid", amount: "$38,200.00", status: "failed", date: "Yesterday, 14:30 PM" },
        { id: '4', tenant: "DataSystems Inc", tenantId: "T-0982", plan: "Standard", amount: "$2,400.00", status: "paid", date: "Yesterday, 11:20 AM" },
        { id: '5', tenant: "Stark Industries", tenantId: "T-4451", plan: "Enterprise Grid", amount: "$145,000.00", status: "pending", date: "Oct 12, 2023" },
      ]
    };
  }
}

export const adminBillingService = new AdminBillingService();
