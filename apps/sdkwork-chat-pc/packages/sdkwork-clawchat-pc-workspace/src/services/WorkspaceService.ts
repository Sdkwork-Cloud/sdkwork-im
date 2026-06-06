import React from 'react';

export interface AppItem {
  id: string;
  name: string;
  nameKey: string;
  iconName: string; // Using string to represent icon name for serialization
  color: string;
}

export interface DocumentItem {
  id: string;
  name: string;
  nameKey: string;
  timestamp: number;
  type: string;
}

export interface WorkspaceService {
  getApps(): Promise<AppItem[]>;
  getRecentDocuments(): Promise<DocumentItem[]>;
  searchApps(query: string): Promise<AppItem[]>;
  addRecentDocument(doc: DocumentItem): Promise<void>;
  deleteRecentDocument(id: string): Promise<void>;
  addApp(app: AppItem): Promise<void>;
  removeApp(id: string): Promise<void>;
}

const mockApps: AppItem[] = [
  { id: 'notary', name: '公证业务', nameKey: 'apps.notary', iconName: 'ShieldCheck', color: 'bg-indigo-500/20 text-indigo-400' },
  { id: 'calendar', name: '日程安排', nameKey: 'apps.calendar', iconName: 'Calendar', color: 'bg-blue-500/20 text-blue-400' },
  { id: 'approval', name: '审批', nameKey: 'apps.approval', iconName: 'CheckSquare', color: 'bg-green-500/20 text-green-400' },
  { id: 'report', name: '汇报', nameKey: 'apps.report', iconName: 'FileText', color: 'bg-orange-500/20 text-orange-400' },
  { id: 'mail', name: '企业邮箱', nameKey: 'apps.mail', iconName: 'Mail', color: 'bg-purple-500/20 text-purple-400' },
  { id: 'dashboard', name: '数据看板', nameKey: 'apps.dashboard', iconName: 'PieChart', color: 'bg-pink-500/20 text-pink-400' },
  { id: 'attendance', name: '考勤打卡', nameKey: 'apps.attendance', iconName: 'Clock', color: 'bg-yellow-500/20 text-yellow-400' },
  { id: 'drive', name: '云盘', nameKey: 'apps.drive', iconName: 'Cloud', color: 'bg-cyan-500/20 text-cyan-400' },
  { id: 'devices', name: '智能硬件', nameKey: 'apps.devices', iconName: 'Server', color: 'bg-indigo-500/20 text-indigo-400' },
  { id: 'videogen', name: 'AI视频生成', nameKey: 'apps.videogen', iconName: 'Video', color: 'bg-indigo-500/20 text-indigo-400' },
  { id: 'imagegen', name: 'AI图片生成', nameKey: 'apps.imagegen', iconName: 'ImageIcon', color: 'bg-blue-500/20 text-blue-400' },
  { id: 'voicegen', name: 'AI语音合成', nameKey: 'apps.voicegen', iconName: 'Mic', color: 'bg-green-500/20 text-green-400' },
  { id: 'musicgen', name: 'AI音乐生成', nameKey: 'apps.musicgen', iconName: 'Music', color: 'bg-purple-500/20 text-purple-400' },
  { id: 'writing', name: 'AI智能写作', nameKey: 'apps.writing', iconName: 'PenTool', color: 'bg-pink-500/20 text-pink-400' },
];

const mockDocs: DocumentItem[] = [
  { id: 'doc1', name: '2026年Q2产品规划文档.docx', nameKey: 'docs.doc1', timestamp: Date.now() - 1000 * 60 * 30, type: 'word' },
  { id: 'doc2', name: '前端架构升级方案.pdf', nameKey: 'docs.doc2', timestamp: Date.now() - 1000 * 60 * 60 * 2, type: 'pdf' },
  { id: 'doc3', name: 'Q1绩效考核表.xlsx', nameKey: 'docs.doc3', timestamp: Date.now() - 1000 * 60 * 60 * 24, type: 'excel' },
];

class MockWorkspaceService implements WorkspaceService {
  async getApps(): Promise<AppItem[]> {
    return new Promise(resolve => {
      setTimeout(() => resolve(mockApps), 300);
    });
  }

  async getRecentDocuments(): Promise<DocumentItem[]> {
    return new Promise(resolve => {
      setTimeout(() => resolve(mockDocs), 400);
    });
  }

  async searchApps(query: string): Promise<AppItem[]> {
    return new Promise(resolve => {
      setTimeout(() => {
        const lowered = query.toLowerCase();
        resolve(mockApps.filter(app => app.name.includes(query) || app.name.toLowerCase().includes(lowered)));
      }, 300);
    });
  }

  async addRecentDocument(doc: DocumentItem): Promise<void> {
    return new Promise(resolve => {
      setTimeout(() => {
        mockDocs.unshift(doc);
        resolve();
      }, 200);
    });
  }

  async deleteRecentDocument(id: string): Promise<void> {
    return new Promise(resolve => {
      setTimeout(() => {
        const index = mockDocs.findIndex(d => d.id === id);
        if (index > -1) mockDocs.splice(index, 1);
        resolve();
      }, 200);
    });
  }

  async addApp(app: AppItem): Promise<void> {
    return new Promise(resolve => {
      setTimeout(() => {
        mockApps.push(app);
        resolve();
      }, 200);
    });
  }

  async removeApp(id: string): Promise<void> {
    return new Promise(resolve => {
      setTimeout(() => {
        const index = mockApps.findIndex(a => a.id === id);
        if (index > -1) mockApps.splice(index, 1);
        resolve();
      }, 200);
    });
  }
}

export const workspaceService = new MockWorkspaceService();
