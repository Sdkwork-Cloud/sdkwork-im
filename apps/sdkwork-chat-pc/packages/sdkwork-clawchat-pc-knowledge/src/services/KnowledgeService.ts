export interface KnowledgeBase {
  id: string;
  name: string;
  description: string;
  logo: string;
  count: number;
  updatedAt: number;
  type: 'personal' | 'team';
}

export interface KnowledgeDoc {
  id: string;
  baseId: string;
  title: string;
  content: string;
  author: string;
  updatedAt: number;
  tags: string[];
  type?: 'markdown' | 'file' | 'folder';
  parentId?: string;
  fileUrl?: string;
  fileName?: string;
  fileSize?: string;
  fileMimeType?: string;
}

export const knowledgeService = {
  getBases: async (): Promise<KnowledgeBase[]> => {
    return new Promise(resolve => setTimeout(() => resolve([
      { id: 'kb-1', name: '产品需求文档', description: '所有产品的PRD集中地', logo: '🚀', count: 120, updatedAt: Date.now() - 3600000, type: 'team' },
      { id: 'kb-2', name: '技术架构演进', description: '技术选型、架构图、方案设计', logo: '🛠', count: 45, updatedAt: Date.now() - 86400000, type: 'team' },
      { id: 'kb-3', name: '新员工入职指南', description: 'Help yourself!', logo: '🌱', count: 12, updatedAt: Date.now() - 200000000, type: 'team' },
      { id: 'kb-4', name: '个人笔记', description: '随想和学习记录', logo: '📝', count: 88, updatedAt: Date.now(), type: 'personal' },
    ]), 300));
  },
  
  createBase: async (data: Partial<KnowledgeBase>): Promise<KnowledgeBase> => {
    return new Promise(resolve => setTimeout(() => {
      resolve({
        id: `kb-${Date.now()}`,
        name: data.name || '未命名知识库',
        description: data.description || '',
        logo: data.logo || '📁',
        count: 0,
        updatedAt: Date.now(),
        type: data.type || 'personal'
      });
    }, 400));
  },
  
  updateBase: async (id: string, data: Partial<KnowledgeBase>): Promise<KnowledgeBase> => {
    return new Promise(resolve => setTimeout(() => resolve({ id, ...data } as KnowledgeBase), 300));
  },
  
  deleteBase: async (id: string): Promise<boolean> => {
    return new Promise(resolve => setTimeout(() => resolve(true), 400));
  },

  getDocs: async (baseId: string): Promise<KnowledgeDoc[]> => {
    return new Promise(resolve => setTimeout(() => resolve([
      { id: 'folder-1', baseId, title: '需求讨论', content: '', author: '张三', updatedAt: Date.now(), tags: [], type: 'folder' },
      { id: 'folder-2', baseId, title: '产品规范', content: '', author: '张三', updatedAt: Date.now(), tags: [], type: 'folder', parentId: 'folder-1' },
      { id: 'doc-1', baseId, title: 'v2.0 迭代需求说明书', content: '# v2.0 需求\n\n- 新增知识库功能\n- 侧边栏重构\n- 性能优化', author: '张三', updatedAt: Date.now(), tags: ['需求', 'v2.0'], type: 'markdown', parentId: 'folder-1' },
      { id: 'doc-2', baseId, title: '前端架构设计规范', content: '# 前端架构\n\n- React 18\n- Tailwind CSS\n- Vite\n- **最佳实践**：所有服务需要统一通过 service 层代理，保持组件整洁。', author: '李四', updatedAt: Date.now() - 3600000, tags: ['架构', '规范'], type: 'markdown' },
      { id: 'doc-file-1', baseId, title: '用户增长数据报表.pdf', content: '', author: '王五', updatedAt: Date.now() - 86400000, tags: ['数据', '报表'], type: 'file', fileName: '用户增长数据报表.pdf', fileSize: '2.4 MB', fileUrl: 'https://www.w3.org/WAI/ER/tests/xhtml/testfiles/resources/pdf/dummy.pdf', fileMimeType: 'application/pdf', parentId: 'folder-2' },
      { id: 'doc-file-2', baseId, title: '横版风景高清.jpg', content: '', author: '设计部', updatedAt: Date.now() - 50000, tags: ['图片', '素材'], type: 'file', fileName: '横版风景高清.jpg', fileSize: '3.2 MB', fileUrl: 'https://images.unsplash.com/photo-1506744626753-1fa44f14c008?auto=format&fit=crop&w=1920&q=80', fileMimeType: 'image/jpeg' },
      { id: 'doc-file-5', baseId, title: '竖版长图海报.png', content: '', author: '市场部', updatedAt: Date.now() - 70000, tags: ['运营', '海报'], type: 'file', fileName: '竖版长图海报.png', fileSize: '4.5 MB', fileUrl: 'https://images.unsplash.com/photo-1611162617474-5b21e879e113?auto=format&fit=crop&w=800&h=2400&q=80', fileMimeType: 'image/png' },
      { id: 'doc-file-6', baseId, title: '超宽全景大图.jpg', content: '', author: '摄影组', updatedAt: Date.now() - 80000, tags: ['全景'], type: 'file', fileName: '超宽全景大图.jpg', fileSize: '8.1 MB', fileUrl: 'https://images.unsplash.com/photo-1469474968028-56623f02e42e?auto=format&fit=crop&w=3200&h=800&q=80', fileMimeType: 'image/jpeg' },
      { id: 'doc-file-3', baseId, title: '横版演示视频(16:9).mp4', content: '', author: '视频组', updatedAt: Date.now() - 150000, tags: ['视频', '横版'], type: 'file', fileName: '横版演示视频(16:9).mp4', fileSize: '15.6 MB', fileUrl: 'https://www.w3schools.com/html/mov_bbb.mp4', fileMimeType: 'video/mp4' },
      { id: 'doc-file-7', baseId, title: '竖屏短视频(9:16).mp4', content: '', author: '运营组', updatedAt: Date.now() - 180000, tags: ['视频', '短片'], type: 'file', fileName: '竖屏短视频(9:16).mp4', fileSize: '22.4 MB', fileUrl: 'https://storage.googleapis.com/gtv-videos-bucket/sample/ForBiggerBlazes.mp4', fileMimeType: 'video/mp4' },
      { id: 'doc-file-4', baseId, title: '测试背景音乐.mp3', content: '', author: '系统', updatedAt: Date.now() - 250000, tags: ['音频'], type: 'file', fileName: '测试背景音乐.mp3', fileSize: '4.8 MB', fileUrl: 'https://www.w3schools.com/html/horse.ogg', fileMimeType: 'audio/mp3' },
      { id: 'doc-file-8', baseId, title: 'UI设计源文件.fig', content: '', author: 'UI组', updatedAt: Date.now() - 300000, tags: ['设计', 'Figma'], type: 'file', fileName: '主页UI设计稿_v2.fig', fileSize: '128 MB', fileUrl: '#', fileMimeType: 'application/octet-stream' },
      { id: 'doc-4', baseId, title: '设计走查问题汇总', content: '# 设计走查\n\n- 按钮颜色不对\n- 间距过大\n- 字体大小不统一', author: '赵六', updatedAt: Date.now() - 200000000, tags: ['设计', '走查'], type: 'markdown' },
    ]), 400));
  },
  
  createDoc: async (data: Partial<KnowledgeDoc>): Promise<KnowledgeDoc> => {
    return new Promise(resolve => setTimeout(() => {
      resolve({
        id: `doc-${Date.now()}`,
        baseId: data.baseId || '',
        title: data.title || '未命名文档',
        content: data.content || '',
        author: data.author || '当前用户',
        updatedAt: Date.now(),
        tags: data.tags || [],
        type: data.type || 'markdown',
        fileUrl: data.fileUrl,
        fileName: data.fileName,
        fileSize: data.fileSize,
        fileMimeType: data.fileMimeType
      });
    }, 500));
  },
  
  updateDoc: async (id: string, data: Partial<KnowledgeDoc>): Promise<KnowledgeDoc> => {
    return new Promise(resolve => setTimeout(() => resolve({ id, ...data } as KnowledgeDoc), 400));
  },
  
  deleteDoc: async (id: string): Promise<boolean> => {
    return new Promise(resolve => setTimeout(() => resolve(true), 300));
  }
};
