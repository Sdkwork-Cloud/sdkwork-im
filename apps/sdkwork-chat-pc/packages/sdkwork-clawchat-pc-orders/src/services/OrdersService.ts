export interface OrderItem {
  id: string;
  productName: string;
  price: number;
  quantity: number;
  imageUrl: string;
}

export interface Order {
  id: string;
  createTime: string;
  customerName: string;
  productInfo: string;
  items?: OrderItem[];
  amount: number;
  status: 'PENDING_PAY' | 'PENDING_SHIP' | 'SHIPPED' | 'COMPLETED' | 'CANCELLED';
}

export interface OrderStats {
  pendingPayAmount: number;
  pendingPayCount: number;
  completedTodayCount: number;
  completedComparedToYesterday: number;
  pendingProcessCount: number;
  pendingTimeoutCount: number;
  monthlyRevenueAmount: number;
  monthlyRevenueCount: number;
}

export interface OrdersService {
  getOrders(): Promise<Order[]>;
  getOrderById(id: string): Promise<Order | null>;
  updateOrderStatus(id: string, status: Order['status']): Promise<Order>;
  getStats(): Promise<OrderStats>;
  createOrder(order: Partial<Order>): Promise<Order>;
  deleteOrder(id: string): Promise<void>;
}

const MOCK_STATS: OrderStats = {
  pendingPayAmount: 5800.00,
  pendingPayCount: 1,
  completedTodayCount: 12,
  completedComparedToYesterday: 3,
  pendingProcessCount: 8,
  pendingTimeoutCount: 2,
  monthlyRevenueAmount: 186498.00,
  monthlyRevenueCount: 154,
};

const MOCK_ORDERS: Order[] = [
  { 
    id: 'ORD-20260417-001', 
    createTime: '2026-04-17 10:23', 
    customerName: '张三网络科技', 
    productInfo: '企业版SaaS服务 x1 年', 
    items: [
      { id: '1', productName: '企业版SaaS服务 (1年)', price: 12999.00, quantity: 1, imageUrl: 'https://images.unsplash.com/photo-1460925895917-afdab827c52f?q=80&w=400&fit=crop' }
    ],
    amount: 12999.00, 
    status: 'COMPLETED' 
  },
  { 
    id: 'ORD-20260417-002', 
    createTime: '2026-04-17 09:45', 
    customerName: '李四跨境电商', 
    productInfo: '高级定制推广套餐', 
    items: [
      { id: '2', productName: '高级定制推广套餐 (海外版)', price: 5800.00, quantity: 1, imageUrl: 'https://images.unsplash.com/photo-1551288049-bebda4e38f71?q=80&w=400&fit=crop' }
    ],
    amount: 5800.00, 
    status: 'PENDING_PAY' 
  },
  { 
    id: 'ORD-20260416-015', 
    createTime: '2026-04-16 16:30', 
    customerName: '王五实业集团', 
    productInfo: '智能客服机器人包', 
    items: [
      { id: '3', productName: '智能客服机器人基础包', price: 15000.00, quantity: 1, imageUrl: 'https://images.unsplash.com/photo-1531746020798-e6953c6e8e04?q=80&w=400&fit=crop' },
      { id: '4', productName: '多语言翻译引擎插件', price: 10000.00, quantity: 1, imageUrl: 'https://images.unsplash.com/photo-1512314889357-e157c22f938d?q=80&w=400&fit=crop' }
    ],
    amount: 25000.00, 
    status: 'PENDING_SHIP' 
  },
  { 
    id: 'ORD-20260416-010', 
    createTime: '2026-04-16 14:12', 
    customerName: '赵六进出口', 
    productInfo: '出海合规咨询服务', 
    items: [
      { id: '5', productName: '欧盟区域数据合规咨询', price: 8000.00, quantity: 1, imageUrl: 'https://images.unsplash.com/photo-1454165804606-c3d57bc86b40?q=80&w=400&fit=crop' }
    ],
    amount: 8000.00, 
    status: 'SHIPPED' 
  },
  { 
    id: 'ORD-20260415-021', 
    createTime: '2026-04-15 11:20', 
    customerName: '孙七教育科技', 
    productInfo: '基础版SaaS服务 x2 年', 
    items: [
      { id: '6', productName: '基础版SaaS服务', price: 5999.00, quantity: 2, imageUrl: 'https://images.unsplash.com/photo-1504384308090-c894fdcc538d?q=80&w=400&fit=crop' }
    ],
    amount: 11998.00, 
    status: 'COMPLETED' 
  },
  { 
    id: 'ORD-20260414-008', 
    createTime: '2026-04-14 09:10', 
    customerName: '周八物流', 
    productInfo: '物流API调用包(100万次)', 
    items: [
      { id: '7', productName: '物流API调用包(100万次)', price: 4500.00, quantity: 1, imageUrl: 'https://images.unsplash.com/photo-1586528116311-ad8ed7c1590f?q=80&w=400&fit=crop' }
    ],
    amount: 4500.00, 
    status: 'CANCELLED' 
  },
  { 
    id: 'ORD-20260413-033', 
    createTime: '2026-04-13 15:40', 
    customerName: '吴九医疗器械', 
    productInfo: '定制化实施部署', 
    items: [
      { id: '8', productName: '本地化服务器部署及调试', price: 35000.00, quantity: 1, imageUrl: 'https://images.unsplash.com/photo-1558494949-ef010cbdcc31?q=80&w=400&fit=crop' }
    ],
    amount: 35000.00, 
    status: 'COMPLETED' 
  },
];

class MockOrdersService implements OrdersService {
  async getOrders(): Promise<Order[]> {
    return new Promise(resolve => {
      setTimeout(() => {
        resolve([...MOCK_ORDERS]);
      }, 300);
    });
  }

  async getOrderById(id: string): Promise<Order | null> {
    return new Promise(resolve => {
      setTimeout(() => {
        resolve(MOCK_ORDERS.find(o => o.id === id) || null);
      }, 200);
    });
  }

  async updateOrderStatus(id: string, status: Order['status']): Promise<Order> {
    return new Promise((resolve, reject) => {
      setTimeout(() => {
        const order = MOCK_ORDERS.find(o => o.id === id);
        if (!order) return reject(new Error('Order not found'));
        order.status = status;
        resolve(order);
      }, 300);
    });
  }

  async getStats(): Promise<OrderStats> {
    return new Promise(resolve => {
      setTimeout(() => {
        resolve(MOCK_STATS);
      }, 300);
    });
  }

  async createOrder(order: Partial<Order>): Promise<Order> {
    return new Promise(resolve => {
      setTimeout(() => {
        const newOrder: Order = {
          id: `ORD-${Date.now()}`,
          createTime: new Date().toISOString().slice(0, 16).replace('T', ' '),
          customerName: order.customerName || '未知客户',
          productInfo: order.productInfo || '未知产品',
          amount: order.amount || 0,
          status: 'PENDING_PAY',
          ...order
        };
        MOCK_ORDERS.unshift(newOrder);
        resolve(newOrder);
      }, 300);
    });
  }

  async deleteOrder(id: string): Promise<void> {
    return new Promise(resolve => {
      setTimeout(() => {
        const index = MOCK_ORDERS.findIndex(o => o.id === id);
        if (index > -1) MOCK_ORDERS.splice(index, 1);
        resolve();
      }, 200);
    });
  }
}

export const ordersService = new MockOrdersService();
