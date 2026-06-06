import { NotaryTask, Party, NotaryDocument, TimelineEvent } from '@sdkwork/clawchat-pc-types';

export interface NotaryService {
  getTasks(filters?: { status?: string; searchTerm?: string }): Promise<NotaryTask[]>;
  getTaskById(taskId: string): Promise<NotaryTask | null>;
  createTask(data: Partial<NotaryTask>): Promise<NotaryTask>;
  updateTaskStatus(taskId: string, status: NotaryTask['status']): Promise<NotaryTask>;
  updateTask(taskId: string, updates: Partial<NotaryTask>): Promise<NotaryTask>;
  addParty(taskId: string, party: Omit<Party, 'id'>): Promise<NotaryTask>;
  addDocument(taskId: string, doc: Omit<NotaryDocument, 'status'>): Promise<NotaryTask>;
  deleteTask(taskId: string): Promise<void>;
  removeDocument(taskId: string, documentName: string): Promise<NotaryTask>;
}

// Mock initial data
const mockTasks: NotaryTask[] = [
  { 
    id: 'NT-20260417-101', 
    createTime: '2026-04-17 11:30', 
    processTime: '2026-04-17 13:45:22',
    applicant: '张三网络科技', 
    title: '用户服务协议及隐私政策上链存证',
    notary: '李明',
    remarks: '优先加急处理，合同涉及重要用户数据合规',
    type: '电子合同存证', 
    fee: 500.00, 
    status: 'COMPLETED',
    hash: '0x3f8e...9a12',
    parties: [
      { id: 'p1', name: '张三', role: '申请人', identityId: '11010519900101xxxx', phone: '13800138000', gender: '男' },
      { id: 'p2', name: '李四', role: '被保全人', identityId: '11010519920202xxxx', phone: '13900139000', gender: '女' }
    ],
    documents: [
      { name: '服务协议_签章版.pdf', size: '2.4 MB', status: 'verified', category: 'evidence' },
      { name: '授权委托书.pdf', size: '800 KB', status: 'verified', category: 'evidence' },
      { name: '张三_身份证正反面.png', size: '1.2 MB', status: 'verified', category: 'identity' },
      { name: '李四_身份证正反面.png', size: '1.1 MB', status: 'verified', category: 'identity' },
      { name: '电子存证公证书.pdf', size: '3.5 MB', status: 'verified', category: 'notary' }
    ],
    timeline: [
      { time: '2026-04-17 13:45', event: '业务办结，自动出证', actor: 'System' },
      { time: '2026-04-17 12:10', event: '区块链上链存证成功', actor: 'Node_A' },
      { time: '2026-04-17 11:45', event: '材料审核通过', actor: '审核员 - 李明' },
      { time: '2026-04-17 11:30', event: '提交申请', actor: '客户' }
    ]
  },
  { 
    id: 'NT-20260417-102', 
    createTime: '2026-04-17 10:15', 
    processTime: '2026-04-17 10:15:30',
    applicant: '李四跨境电商', 
    title: '跨境电商独立站外观专利全网固化',
    notary: '张华',
    remarks: '海外诉讼使用',
    type: '知识产权确权公证', 
    fee: 1200.00, 
    status: 'PENDING_REVIEW',
    hash: 'PENDING',
    parties: [
      { id: 'p3', name: '王五', role: '法定代理人', identityId: '31010519850505xxxx', phone: '13700137000', gender: '男' }
    ],
    documents: [
      { name: '核心源代码截图.png', size: '4.1 MB', status: 'pending', category: 'evidence' },
      { name: '外观设计专利图纸.pdf', size: '12 MB', status: 'pending', category: 'evidence' },
      { name: '企业营业执照副本.pdf', size: '2.5 MB', status: 'verified', category: 'identity' }
    ],
    timeline: [
      { time: '2026-04-17 10:15', event: '提交申请，等待资审', actor: '客户' }
    ]
  },
  { 
    id: 'NT-20260416-089', 
    createTime: '2026-04-16 15:45', 
    processTime: '2026-04-17 09:00:15',
    applicant: '王五实业集团', 
    title: '集团核心工艺及保密协议确权',
    notary: '王建国',
    remarks: '内网涉密，请采用隔离取证节点',
    type: '商业秘密确权', 
    fee: 3500.00, 
    status: 'PROCESSING',
    hash: 'PROCESSING_QUEUE',
    parties: [],
    documents: [
      { name: '保密协议(全员).zip', size: '45 MB', status: 'verified', category: 'evidence' },
      { name: '核心工艺流程单.pdf', size: '3.2 MB', status: 'pending', category: 'evidence' }
    ],
    timeline: [
      { time: '2026-04-17 09:00', event: '材料二次复审中...', actor: '高级审核员 - 张华' },
      { time: '2026-04-16 16:30', event: '初审通过，进入复审', actor: '审核员 - 李明' },
      { time: '2026-04-16 15:45', event: '提交申请', actor: '客户' }
    ]
  }
];

class MockNotaryService implements NotaryService {
  private tasks = [...mockTasks];

  private readonly delay = (ms: number) => new Promise(res => setTimeout(res, ms));

  async getTasks(filters?: { status?: string; searchTerm?: string }): Promise<NotaryTask[]> {
    await this.delay(300);
    let result = [...this.tasks];
    
    if (filters?.status && filters.status !== 'ALL') {
      result = result.filter(t => t.status === filters.status);
    }
    
    if (filters?.searchTerm) {
      const term = filters.searchTerm.toLowerCase();
      result = result.filter(t => 
        t.title.toLowerCase().includes(term) ||
        t.applicant.toLowerCase().includes(term) ||
        t.notary.toLowerCase().includes(term)
      );
    }
    
    return result;
  }

  async getTaskById(taskId: string): Promise<NotaryTask | null> {
    await this.delay(200);
    const task = this.tasks.find(t => t.id === taskId);
    return task || null;
  }

  async createTask(data: Partial<NotaryTask>): Promise<NotaryTask> {
    await this.delay(500);
    const newTask: NotaryTask = {
      id: `NT-${new Date().toISOString().slice(0,10).replace(/-/g, '')}-${Math.floor(Math.random() * 900) + 100}`,
      createTime: new Date().toISOString().slice(0, 16).replace('T', ' '),
      applicant: data.applicant || '未知申请人',
      title: data.title || '无标题业务',
      notary: data.notary || '系统分配',
      remarks: data.remarks || '',
      type: data.type || '通用公证',
      status: 'PENDING_REVIEW',
      fee: data.fee || 0,
      hash: 'PENDING',
      parties: data.parties || [],
      documents: data.documents || [],
      timeline: [
        { time: new Date().toISOString().slice(0, 16).replace('T', ' '), event: '提交申请', actor: '客户' }
      ]
    };
    this.tasks = [newTask, ...this.tasks];
    return newTask;
  }

  async updateTaskStatus(taskId: string, status: NotaryTask['status']): Promise<NotaryTask> {
    await this.delay(300);
    const index = this.tasks.findIndex(t => t.id === taskId);
    if (index === -1) throw new Error('Task not found');
    
    const task = { ...this.tasks[index], status };
    task.timeline.unshift({
      time: new Date().toISOString().slice(0, 16).replace('T', ' '),
      event: `状态更新为：${status}`,
      actor: '系统'
    });
    
    this.tasks[index] = task;
    return task;
  }

  async updateTask(taskId: string, updates: Partial<NotaryTask>): Promise<NotaryTask> {
    await this.delay(300);
    const index = this.tasks.findIndex(t => t.id === taskId);
    if (index === -1) throw new Error('Task not found');
    
    const task = { ...this.tasks[index], ...updates };
    this.tasks[index] = task;
    return task;
  }

  async addParty(taskId: string, party: Omit<Party, 'id'>): Promise<NotaryTask> {
    await this.delay(300);
    const index = this.tasks.findIndex(t => t.id === taskId);
    if (index === -1) throw new Error('Task not found');
    
    const newParty: Party = {
      ...party,
      id: `p${Date.now()}`
    };
    
    const task = { ...this.tasks[index] };
    task.parties = [...(task.parties || []), newParty];
    this.tasks[index] = task;
    return task;
  }

  async addDocument(taskId: string, doc: Omit<NotaryDocument, 'status'>): Promise<NotaryTask> {
    await this.delay(300);
    const index = this.tasks.findIndex(t => t.id === taskId);
    if (index === -1) throw new Error('Task not found');
    
    const newDoc: NotaryDocument = {
      ...doc,
      status: 'pending' // Initially pending verification
    };
    
    const task = { ...this.tasks[index] };
    task.documents = [...(task.documents || []), newDoc];
    this.tasks[index] = task;
    return task;
  }

  async deleteTask(taskId: string): Promise<void> {
    await this.delay(300);
    this.tasks = this.tasks.filter(t => t.id !== taskId);
  }

  async removeDocument(taskId: string, documentName: string): Promise<NotaryTask> {
    await this.delay(300);
    const index = this.tasks.findIndex(t => t.id === taskId);
    if (index === -1) throw new Error('Task not found');
    
    const task = { ...this.tasks[index] };
    if (task.documents) {
      task.documents = task.documents.filter(d => d.name !== documentName);
    }
    this.tasks[index] = task;
    return task;
  }
}

export const notaryService: NotaryService = new MockNotaryService();
