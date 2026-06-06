export interface Community {
  id: string;
  name: string;
  description: string;
  avatar: string;
  cover: string;
  membersCount: number;
  tags: string[];
  tabs?: string[];
}

export interface Post {
  id: string;
  communityId: string;
  author: {
    id: string;
    name: string;
    avatar: string;
  };
  content: string;
  images?: string[];
  likes: number;
  comments: number;
  createdAt: string;
}

export interface ResourceItem {
  id: string;
  name: string;
  size: string;
  type: string;
  uploader: string;
  uploadTime: string;
}

export type PlatformType =
  | "wechat"
  | "qq"
  | "feishu"
  | "dingtalk"
  | "telegram"
  | "discord"
  | "other";

export interface GroupQRCode {
  url: string;
  description: string;
}

export interface ChatGroup {
  id: string;
  name: string;
  platform: PlatformType;
  qrCodes: GroupQRCode[];
  memberCount: number;
  description: string;
}

export interface NewsItem {
  id: string;
  title: string;
  summary: string;
  content: string;
  source: string;
  time: string;
  views: number;
  comments: number;
  cover?: string;
}

export interface DocOutlineNode {
  id: string;
  title: string;
  children?: DocOutlineNode[];
}

export interface RepoItem {
  id: string;
  name: string;
  lang: string;
  desc: string;
  stars: string;
  forks: string;
  updated: string;
  color: string;
}

export interface SoftwareItem {
  id: string;
  name: string;
  desc: string;
  icon: string;
  cat: string;
}

export interface CommunityService {
  getCommunities(): Promise<Community[]>;
  getCommunity(id: string): Promise<Community | undefined>;
  getPosts(communityId: string): Promise<Post[]>;
  getResources(communityId: string): Promise<ResourceItem[]>;
  getGroups(communityId: string): Promise<ChatGroup[]>;
  getNews(communityId: string): Promise<NewsItem[]>;
  getDocsOutline(communityId: string): Promise<DocOutlineNode[]>;
  getRepos(communityId: string): Promise<RepoItem[]>;
  getSoftware(communityId: string): Promise<SoftwareItem[]>;
  updateCommunity(id: string, updates: Partial<Community>): Promise<Community>;
  createPost(communityId: string, content: string, images?: string[]): Promise<Post>;
  toggleLikePost(postId: string): Promise<boolean>;
  createGroup(
    communityId: string,
    group: Omit<ChatGroup, "id">,
  ): Promise<ChatGroup>;
  updateGroup(
    communityId: string,
    groupId: string,
    group: Partial<ChatGroup>,
  ): Promise<void>;
  deleteGroup(communityId: string, groupId: string): Promise<void>;
  deletePost(communityId: string, postId: string): Promise<void>;
  uploadResource(
    communityId: string,
    resource: Omit<ResourceItem, "id" | "uploadTime">,
  ): Promise<ResourceItem>;
  deleteResource(communityId: string, resourceId: string): Promise<void>;
}

const mockCommunities: Community[] = [
  {
    id: "c1",
    name: "AI 探索者俱乐部",
    description: "探讨最新人工智能技术与应用落地，分享前沿资讯与开发经验。",
    avatar:
      "https://images.unsplash.com/photo-1677442136019-21780ecad995?auto=format&fit=crop&q=80&w=150",
    cover:
      "https://images.unsplash.com/photo-1620712943543-bcc4688e7485?auto=format&fit=crop&q=80&w=800",
    membersCount: 1280,
    tags: ["AI", "Tech", "Development"],
    tabs: ["feeds", "resources", "groups", "news", "docs", "repos", "software"],
  },
  {
    id: "c2",
    name: "产品汪与设计喵",
    description: "好产品是如何炼成的？分享交互设计、产品规划、用户体验研究。",
    avatar:
      "https://images.unsplash.com/photo-1561070791-2526d30994b5?auto=format&fit=crop&q=80&w=150",
    cover:
      "https://images.unsplash.com/photo-1558655146-d09347e92766?auto=format&fit=crop&q=80&w=800",
    membersCount: 890,
    tags: ["Product", "Design", "UX"],
    tabs: ["feeds", "resources", "groups", "news", "docs", "repos", "software"],
  },
  {
    id: "c3",
    name: "咖啡与阅读",
    description: "闲暇时光的好去处，分享你最近在读的书和喜欢的咖啡店。",
    avatar:
      "https://images.unsplash.com/photo-1497935586351-b67a49e012bf?auto=format&fit=crop&q=80&w=150",
    cover:
      "https://images.unsplash.com/photo-1509042239860-f550ce710b93?auto=format&fit=crop&q=80&w=800",
    membersCount: 256,
    tags: ["Life", "Reading", "Coffee"],
    tabs: ["feeds", "resources", "groups", "news", "docs", "repos", "software"],
  },
];

const mockPosts: Record<string, Post[]> = {
  c1: [
    {
      id: "p1",
      communityId: "c1",
      author: {
        id: "u1",
        name: "TechGeek",
        avatar: "https://i.pravatar.cc/150?u=tech",
      },
      content:
        "今天测试了最新的大模型 API，响应速度惊人！结合我们的工作流，效率至少提升了 30%。大家可以尝试在本地跑一下测试脚本。",
      likes: 45,
      comments: 12,
      createdAt: "2小时前",
    },
    {
      id: "p2",
      communityId: "c1",
      author: {
        id: "u2",
        name: "AI_Researcher",
        avatar: "https://i.pravatar.cc/150?u=ai",
      },
      content:
        "分享一个关于 RAG (Retrieval-Augmented Generation) 架构优化的最佳实践文档。附件在资源区可以下载，欢迎讨论！",
      images: [
        "https://images.unsplash.com/photo-1677442136019-21780ecad995?auto=format&fit=crop&q=80&w=400",
      ],
      likes: 89,
      comments: 24,
      createdAt: "5小时前",
    },
  ],
};

const mockResources: Record<string, ResourceItem[]> = {
  c1: [
    {
      id: "r1",
      name: "RAG_Architecture_Best_Practices.pdf",
      size: "2.4 MB",
      type: "PDF",
      uploader: "AI_Researcher",
      uploadTime: "2023-11-20",
    },
    {
      id: "r2",
      name: "LLM_Prompt_Engineering_Guide.md",
      size: "128 KB",
      type: "Markdown",
      uploader: "TechGeek",
      uploadTime: "2023-11-18",
    },
    {
      id: "r3",
      name: "Model_Benchmarking_Data.xlsx",
      size: "5.6 MB",
      type: "Excel",
      uploader: "DataScience_Bob",
      uploadTime: "2023-11-15",
    },
  ],
};

const mockGroups: Record<string, ChatGroup[]> = {
  c1: [
    {
      id: "g1",
      name: "AI 探索者官方微信群",
      platform: "wechat",
      qrCodes: [
        {
          url: "https://images.unsplash.com/photo-1611162617474-5b21e879e113?auto=format&fit=crop&q=80&w=200",
          description: "微信扫码加入",
        },
      ],
      memberCount: 450,
      description: "官方主群，讨论 AI 前沿资讯与实战经验",
    },
    {
      id: "g2",
      name: "开发技术讨论钉钉群",
      platform: "dingtalk",
      qrCodes: [
        {
          url: "https://images.unsplash.com/photo-1611162617474-5b21e879e113?auto=format&fit=crop&q=80&w=200",
          description: "钉钉扫码加入",
        },
      ],
      memberCount: 210,
      description: "偏向底层与架构的技术讨论",
    },
    {
      id: "g3",
      name: "Agent 研究飞书群",
      platform: "feishu",
      qrCodes: [
        {
          url: "https://images.unsplash.com/photo-1611162617474-5b21e879e113?auto=format&fit=crop&q=80&w=200",
          description: "飞书扫码加入",
        },
      ],
      memberCount: 156,
      description: "聚焦智能体、大模型能力边界探讨",
    },
  ],
};

class MockCommunityService implements CommunityService {
  async getCommunities(): Promise<Community[]> {
    return new Promise((resolve) =>
      setTimeout(() => resolve([...mockCommunities]), 300),
    );
  }

  async getCommunity(id: string): Promise<Community | undefined> {
    return new Promise((resolve) =>
      setTimeout(() => resolve(mockCommunities.find((c) => c.id === id)), 100),
    );
  }

  async updateCommunity(id: string, updates: Partial<Community>): Promise<Community> {
    return new Promise((resolve, reject) => {
      setTimeout(() => {
        const index = mockCommunities.findIndex((c) => c.id === id);
        if (index > -1) {
          mockCommunities[index] = { ...mockCommunities[index], ...updates };
          resolve(mockCommunities[index]);
        } else {
          reject(new Error("Community not found"));
        }
      }, 300);
    });
  }

  async getPosts(communityId: string): Promise<Post[]> {
    return new Promise((resolve) =>
      setTimeout(() => resolve(mockPosts[communityId] || []), 200),
    );
  }

  async getResources(communityId: string): Promise<ResourceItem[]> {
    return new Promise((resolve) =>
      setTimeout(() => resolve(mockResources[communityId] || []), 200),
    );
  }

  async getGroups(communityId: string): Promise<ChatGroup[]> {
    return new Promise((resolve) =>
      setTimeout(() => resolve(mockGroups[communityId] || []), 200),
    );
  }

  async getNews(communityId: string): Promise<NewsItem[]> {
    return new Promise((resolve) =>
      setTimeout(
        () =>
          resolve([
            {
              id: "n1",
              title: "OpenAI 发布最新研究成果，大语言模型推理能力再突破",
              summary:
                "今日 OpenAI 在官方博客中宣布了其关于提升大模型思维链推理能力的最新进展，新模型在数学和编程基准测试中得分创下历史新高...",
              content:
                "今日 OpenAI 在官方博客中宣布了其关于提升大模型思维链推理能力的最新进展，新模型在数学和编程基准测试中得分创下历史新高...\n\n研究团队表示，这得益于规模化的强化学习训练，模型能够在给出最终答案前，生成数万个中间推理步骤，并通过验证器筛选出最佳路径。\n\n此外，新模型还在代码生成和错误排查上表现出惊人的准确性。这一突破不仅将极大地加速生产力革命，也标志着通用人工智能（AGI）的脚步越来越近。",
              source: "AI前沿资讯",
              time: "3小时前",
              views: 12500,
              comments: 328,
              cover:
                "https://images.unsplash.com/photo-1677442136019-21780ecad995?auto=format&fit=crop&q=80&w=400",
            },
            {
              id: "n2",
              title: "React 19 RC 版本推出，编译器机制带来性能质变",
              summary:
                "React 团队终于在最新博文中确认了 React 19 的新特性，最外界期待的 React Compiler 成为现实，彻底取代 useMemo 和 useCallback...",
              content: "React 19...",
              source: "前端开发周刊",
              time: "昨天",
              views: 8900,
              comments: 156,
            },
            {
              id: "n3",
              title: "2024 年 Q1 全球科技行业投融资报告解读",
              summary:
                "在这个动荡的资本市场环境下，AI 赛道依然火热，而其他领域的早期投资则显现出冷却迹象...",
              content: "行业投融资报告解读...",
              source: "创投圈",
              time: "3天前",
              views: 4500,
              comments: 89,
            },
          ]),
        200,
      ),
    );
  }

  async getDocsOutline(communityId: string): Promise<DocOutlineNode[]> {
    return new Promise((resolve) =>
      setTimeout(
        () =>
          resolve([
            {
              id: "folder_1",
              title: "快速入门",
              children: [
                { id: "doc_1", title: "ClawChat 介绍" },
                { id: "doc_2", title: "应用接入指南" },
                { id: "doc_3", title: "常见问题解答" },
              ],
            },
            {
              id: "folder_2",
              title: "API 参考",
              children: [
                { id: "doc_4", title: "消息发送接口" },
                { id: "doc_5", title: "用户鉴权" },
              ],
            },
            {
              id: "folder_3",
              title: "高级特性",
              children: [
                { id: "doc_6", title: "自定义界面设计" },
              ],
            },
          ]),
        100,
      ),
    );
  }

  async getRepos(communityId: string): Promise<RepoItem[]> {
    return new Promise((resolve) =>
      setTimeout(
        () =>
          resolve([
            {
              id: "repo1",
              name: "clawchat-core",
              lang: "Rust",
              desc: "ClawChat 即时通讯核心服务，包含 WebSocket 网关和消息路由机制。",
              stars: "12.5k",
              forks: "1.2k",
              updated: "2 小时前",
              color: "bg-[#dea584]",
            },
            {
              id: "repo2",
              name: "react-ui-components",
              lang: "TypeScript",
              desc: "企业级高颜值 React 组件库，开箱即用的前端页面构建方案。",
              stars: "8.9k",
              forks: "845",
              updated: "4 小时前",
              color: "bg-[#3178c6]",
            },
            {
              id: "repo3",
              name: "deep-learning-models",
              lang: "Python",
              desc: "收集并整理了各种开源的大语言模型实现代码，方便进行微调和部署。",
              stars: "24.1k",
              forks: "3.4k",
              updated: "昨天",
              color: "bg-[#3572A5]",
            },
            {
              id: "repo4",
              name: "go-microservices-template",
              lang: "Go",
              desc: "基于 Go 语言的微服务脚手架，集成了 gRPC, Consul, 链路追踪等。",
              stars: "4.5k",
              forks: "412",
              updated: "3 天前",
              color: "bg-[#00ADD8]",
            },
          ]),
        200,
      ),
    );
  }

  async getSoftware(communityId: string): Promise<SoftwareItem[]> {
    return new Promise((resolve) =>
      setTimeout(
        () =>
          resolve([
            {
              id: "soft1",
              name: "Cursor",
              desc: "基于 AI 的智能代码编辑器，重塑编程体验。",
              icon: "https://images.unsplash.com/photo-1618401471353-b98afee0b2eb?auto=format&fit=crop&q=80&w=150",
              cat: "开发工具",
            },
            {
              id: "soft2",
              name: "Figma",
              desc: "协作式界面设计工具，将创意变为现实。",
              icon: "https://images.unsplash.com/photo-1611162617213-7d7a39e9b1d7?auto=format&fit=crop&q=80&w=150",
              cat: "设计工具",
            },
            {
              id: "soft3",
              name: "Linear",
              desc: "现代化软件开发项目管理工具，极简高效。",
              icon: "https://images.unsplash.com/photo-1614332287897-cdc485fa562d?auto=format&fit=crop&q=80&w=150",
              cat: "生产力",
            },
            {
              id: "soft4",
              name: "Raycast",
              desc: "极速 Mac 启动器，让你的操作快如闪电。",
              icon: "https://images.unsplash.com/photo-1629654297299-c8506221ca97?auto=format&fit=crop&q=80&w=150",
              cat: "生产力",
            },
            {
              id: "soft5",
              name: "OrbStack",
              desc: "快如闪电的 Docker 桌面环境替代方案。",
              icon: "https://images.unsplash.com/photo-1605379399642-870262d3d051?auto=format&fit=crop&q=80&w=150",
              cat: "开发工具",
            },
            {
              id: "soft6",
              name: "Warp",
              desc: "为现代开发者打造的 Rust 终端，智能补全。",
              icon: "https://images.unsplash.com/photo-1555066931-4365d14bab8c?auto=format&fit=crop&q=80&w=150",
              cat: "开发工具",
            },
          ]),
        200,
      ),
    );
  }

  async createGroup(
    communityId: string,
    group: Omit<ChatGroup, "id">,
  ): Promise<ChatGroup> {
    return new Promise((resolve) => {
      setTimeout(() => {
        const newGroup = { ...group, id: `g${Date.now()}` };
        if (!mockGroups[communityId]) mockGroups[communityId] = [];
        mockGroups[communityId].push(newGroup);
        resolve(newGroup);
      }, 300);
    });
  }

  async updateGroup(
    communityId: string,
    groupId: string,
    group: Partial<ChatGroup>,
  ): Promise<void> {
    return new Promise((resolve) => {
      setTimeout(() => {
        const groups = mockGroups[communityId];
        if (groups) {
          const index = groups.findIndex((g) => g.id === groupId);
          if (index > -1) {
            groups[index] = { ...groups[index], ...group };
          }
        }
        resolve();
      }, 300);
    });
  }

  async deleteGroup(communityId: string, groupId: string): Promise<void> {
    return new Promise((resolve) => {
      setTimeout(() => {
        const groups = mockGroups[communityId];
        if (groups) {
          const index = groups.findIndex((g) => g.id === groupId);
          if (index > -1) {
            groups.splice(index, 1);
          }
        }
        resolve();
      }, 300);
    });
  }

  async createPost(communityId: string, content: string, images?: string[]): Promise<Post> {
    return new Promise((resolve) => {
      setTimeout(() => {
        const newPost: Post = {
          id: `p${Date.now()}`,
          communityId,
          author: {
            id: "me",
            name: "当前用户",
            avatar: "https://i.pravatar.cc/150?u=me",
          },
          content,
          images: images || [],
          likes: 0,
          comments: 0,
          createdAt: "刚刚",
        };
        if (!mockPosts[communityId]) mockPosts[communityId] = [];
        mockPosts[communityId].unshift(newPost);
        resolve(newPost);
      }, 300);
    });
  }

  async toggleLikePost(postId: string): Promise<boolean> {
    return new Promise((resolve) => {
      setTimeout(() => {
        let found = false;
        let isLiked = false;
        for (const communityId in mockPosts) {
          const post = mockPosts[communityId].find((p) => p.id === postId);
          if (post) {
            // Very simple toggle logic without storing user ID for mock purpose
            if (post.likes > 0 && Math.random() < 0.5) { // Just for simulation toggle if it has likes
              post.likes--;
              isLiked = false;
            } else {
              post.likes++;
              isLiked = true;
            }
            found = true;
            break;
           }
        }
        resolve(isLiked);
      }, 100);
    });
  }
  async deletePost(communityId: string, postId: string): Promise<void> {
    return new Promise((resolve) => {
      setTimeout(() => {
        if (mockPosts[communityId]) {
          mockPosts[communityId] = mockPosts[communityId].filter(
            (p) => p.id !== postId,
          );
        }
        resolve();
      }, 200);
    });
  }

  async uploadResource(
    communityId: string,
    resource: Omit<ResourceItem, "id" | "uploadTime">,
  ): Promise<ResourceItem> {
    return new Promise((resolve) => {
      setTimeout(() => {
        const newResource: ResourceItem = {
          ...resource,
          id: `res_${Date.now()}`,
          uploadTime: "刚刚",
        };
        if (!mockResources[communityId]) mockResources[communityId] = [];
        mockResources[communityId].unshift(newResource);
        resolve(newResource);
      }, 300);
    });
  }

  async deleteResource(communityId: string, resourceId: string): Promise<void> {
    return new Promise((resolve) => {
      setTimeout(() => {
        if (mockResources[communityId]) {
          mockResources[communityId] = mockResources[communityId].filter(
            (r) => r.id !== resourceId,
          );
        }
        resolve();
      }, 200);
    });
  }
}

export const communityService = new MockCommunityService();
