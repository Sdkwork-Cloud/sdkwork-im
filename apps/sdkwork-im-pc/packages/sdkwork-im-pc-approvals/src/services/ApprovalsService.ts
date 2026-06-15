export interface ApprovalItem {
  id: string;
  title: string;
  type: 'leave' | 'reimbursement' | 'purchase' | 'other';
  status: 'pending' | 'approved' | 'rejected' | 'revoked';
  applicant: { id: string; name: string; avatar?: string };
  submitTime: string;
  amount?: number;
  description: string;
  attachments?: { name: string; size: string }[];
}

const mockApprovals: ApprovalItem[] = [
  {
    id: 'APP-20230501-001',
    title: '年假申请-3天',
    type: 'leave',
    status: 'pending',
    applicant: { id: 'u1', name: '张三' },
    submitTime: '今天 09:30',
    description: '因个人事务需要请假3天，期间工作已交接给李四。',
  },
  {
    id: 'APP-20230501-002',
    title: '部门活动经费报销',
    type: 'reimbursement',
    status: 'pending',
    applicant: { id: 'u2', name: '李四' },
    submitTime: '今天 10:15',
    amount: 1250.00,
    description: '4月部门团建餐饮及交通费用报销，附发票10张。',
    attachments: [{ name: '发票扫描件.pdf', size: '2.4MB' }]
  },
  {
    id: 'APP-20230428-005',
    title: '采购新款显示器配置',
    type: 'purchase',
    status: 'approved',
    applicant: { id: 'u3', name: '王五' },
    submitTime: '4月28日 14:20',
    amount: 8500.00,
    description: '设计组需要更新两台色彩显示器，配置清单见附件。',
    attachments: [{ name: '采购清单.xlsx', size: '1.1MB' }]
  }
];

export const ApprovalsService = {
  getPendingApprovals: async () => mockApprovals.filter(a => a.status === 'pending'),
  getHandledApprovals: async () => mockApprovals.filter(a => a.status !== 'pending'),
  getMyRequests: async () => mockApprovals.filter(a => a.applicant.id === 'my-id'),
  approve: async (id: string, comment: string) => { console.log('Approve', id, comment); },
  reject: async (id: string, comment: string) => { console.log('Reject', id, comment); },
  submitApproval: async (item: ApprovalItem) => { console.log('Submit', item); return item; }
};
