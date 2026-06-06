export interface FolderItem {
  id: string;
  name: string;
  fileCount: number;
}

export interface DriveFileItem {
  id: string;
  name: string;
  size: string;
  time: string;
  type: 'pdf' | 'word' | 'image' | 'excel' | 'unknown';
}

export interface DriveService {
  getFolders(): Promise<FolderItem[]>;
  createFolder(name: string): Promise<FolderItem>;
  renameFolder(id: string, newName: string): Promise<void>;
  deleteFolder(id: string): Promise<void>;
  getRecentFiles(): Promise<DriveFileItem[]>;
  deleteFile(id: string): Promise<void>;
  renameFile(id: string, newName: string): Promise<void>;
  uploadFile(file: Partial<DriveFileItem>): Promise<DriveFileItem>;
}

const mockFolders: FolderItem[] = [
  { id: 'f1', name: '项目资料_A', fileCount: 23 },
  { id: 'f2', name: '人力资源', fileCount: 41 },
  { id: 'f3', name: '财务报表_2026', fileCount: 12 },
  { id: 'f4', name: '公证存证案卷', fileCount: 78 },
  { id: 'f5', name: '产品架构设计', fileCount: 15 },
  { id: 'f6', name: '市场调研分析', fileCount: 30 },
];

const mockFiles: DriveFileItem[] = [
  { id: 'd1', name: '前端架构升级方案.pdf', size: '4.2 MB', time: '10分钟前', type: 'pdf' },
  { id: 'd2', name: '2026年Q2产品规划文档.docx', size: '1.1 MB', time: '1小时前', type: 'word' },
  { id: 'd3', name: '公证业务全流程图.png', size: '8.4 MB', time: '昨天 15:30', type: 'image' },
  { id: 'd4', name: 'Q1绩效考核表.xlsx', size: '200 KB', time: '4月16日', type: 'excel' },
  { id: 'd5', name: '系统部署架构_v3.pdf', size: '15.6 MB', time: '4月15日', type: 'pdf' },
];

class MockDriveService implements DriveService {
  async getFolders(): Promise<FolderItem[]> {
    return new Promise(resolve => {
      setTimeout(() => {
        resolve([...mockFolders]);
      }, 300);
    });
  }

  async createFolder(name: string): Promise<FolderItem> {
    return new Promise(resolve => {
      setTimeout(() => {
        const newFolder: FolderItem = {
          id: `f_${Date.now()}`,
          name: name || '新建文件夹',
          fileCount: 0
        };
        mockFolders.push(newFolder);
        resolve(newFolder);
      }, 300);
    });
  }

  async renameFolder(id: string, newName: string): Promise<void> {
    return new Promise((resolve, reject) => {
      setTimeout(() => {
        const folder = mockFolders.find(f => f.id === id);
        if (!folder) return reject(new Error('Folder not found'));
        folder.name = newName;
        resolve();
      }, 200);
    });
  }

  async deleteFolder(id: string): Promise<void> {
    return new Promise(resolve => {
      setTimeout(() => {
        const index = mockFolders.findIndex(f => f.id === id);
        if (index !== -1) mockFolders.splice(index, 1);
        resolve();
      }, 200);
    });
  }

  async getRecentFiles(): Promise<DriveFileItem[]> {
    return new Promise(resolve => {
      setTimeout(() => {
        resolve([...mockFiles]);
      }, 300);
    });
  }

  async deleteFile(id: string): Promise<void> {
    return new Promise(resolve => {
      setTimeout(() => {
        const index = mockFiles.findIndex(f => f.id === id);
        if (index !== -1) mockFiles.splice(index, 1);
        resolve();
      }, 200);
    });
  }

  async renameFile(id: string, newName: string): Promise<void> {
    return new Promise((resolve, reject) => {
      setTimeout(() => {
        const file = mockFiles.find(f => f.id === id);
        if (!file) return reject(new Error('File not found'));
        file.name = newName;
        resolve();
      }, 200);
    });
  }

  async uploadFile(file: Partial<DriveFileItem>): Promise<DriveFileItem> {
    return new Promise(resolve => {
      setTimeout(() => {
        const newFile: DriveFileItem = {
          id: `file_${Date.now()}`,
          name: file.name || 'Untitled File',
          size: file.size || '1 KB',
          time: file.time || '刚刚',
          type: file.type || 'unknown'
        };
        mockFiles.unshift(newFile);
        resolve(newFile);
      }, 500);
    });
  }
}

export const driveService = new MockDriveService();
