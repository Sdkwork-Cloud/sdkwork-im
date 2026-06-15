export type CourseCategory = "all" | "live" | "design" | "frontend" | "backend" | "ai";

export interface CourseMessage {
  id: number;
  user: string;
  text: string;
  time: string;
  isPro: boolean;
}

export interface CourseComment {
  id: number;
  user: string;
  avatar: string;
  text: string;
  time: string;
  likes: number;
  isLiked: boolean;
}

export interface CourseLesson {
  id: string;
  title: string;
  duration: string;
  status: "completed" | "playing" | "locked";
}

export interface CourseChapter {
  title: string;
  lessons: CourseLesson[];
}

export interface Course {
  id: string;
  title: string;
  instructor: string;
  type: "video" | "live";
  level: string;
  duration: string;
  students: number;
  viewers: number;
  updatedAt?: string;
  cover: string;
  category: CourseCategory;
  rating: number;
  progress: number;
  tags?: string[];
  chapters?: CourseChapter[];
  messages?: CourseMessage[];
  comments?: CourseComment[];
}

const COURSES: Course[] = [
  {
    id: "l1",
    title: "【直播】2025 AI 应用架构最新趋势与落地方案",
    instructor: "Andrew Ng · 首席 AI 科学家",
    type: "live",
    duration: "Live",
    students: 12500,
    viewers: 12500,
    rating: 4.9,
    cover: "https://images.unsplash.com/photo-1485827404703-89b55fcc595e?auto=format&fit=crop&q=80&w=1200",
    tags: ["大模型", "架构设计", "LIVE"],
    level: "高阶",
    category: "live",
    progress: 0,
    messages: [
      { id: 1, user: "张三", text: "老师声音很清楚～", time: "10:01", isPro: false },
      { id: 2, user: "李四", text: "期待这节课的内容，前面讲的很深。", time: "10:02", isPro: true },
      { id: 3, user: "王五", text: "刚进来，开始了没有呀？", time: "10:05", isPro: false },
      { id: 4, user: "Alex_Dev", text: "这个架构真的可以直接用在生产环境吗？", time: "10:08", isPro: true },
    ]
  },
  {
    id: "c1",
    title: "进阶 React 19 性能优化与并发渲染实战",
    instructor: "张三 · 资深前端架构师",
    type: "video",
    level: "进阶",
    duration: "12课时",
    students: 8900,
    viewers: 0,
    updatedAt: "12小时前",
    cover: "https://images.unsplash.com/photo-1633356122544-f134324a6cee?auto=format&fit=crop&q=80&w=600",
    category: "frontend",
    rating: 4.9,
    progress: 35,
    tags: ["框架源码", "性能调优"],
    chapters: [
      {
        title: "第一章：基础架构准备",
        lessons: [
          { id: "l1", title: "1.1 课程介绍与学习指南", duration: "05:22", status: "completed" },
          { id: "l2", title: "1.2 开发环境配置与工具链", duration: "12:45", status: "playing" },
          { id: "l3", title: "1.3 核心框架原理解析与应用", duration: "25:30", status: "locked" }
        ]
      },
      {
        title: "第二章：核心模块开发与调优",
        lessons: [
          { id: "l4", title: "2.1 状态管理选型与最佳实践", duration: "18:10", status: "locked" },
          { id: "l5", title: "2.2 路由机制与动态代码分割", duration: "22:15", status: "locked" },
          { id: "l6", title: "2.3 网络请求层封装与缓存策略", duration: "19:40", status: "locked" }
        ]
      }
    ],
    comments: [
      { id: 1, user: "云上的日子", avatar: "https://images.unsplash.com/photo-1534528741775-53994a69daeb?auto=format&fit=crop&q=80&w=100", text: "干货满满，这个架构图解答了我多年的疑惑！直接三连了。", time: "2小时前", likes: 1240, isLiked: false },
      { id: 2, user: "CodeRunner", avatar: "https://images.unsplash.com/photo-1506794778202-cad84cf45f1d?auto=format&fit=crop&q=80&w=100", text: "老师讲得非常通透，特别是虚拟DOM更新机制那一段，比官方文档还要好懂。", time: "5小时前", likes: 856, isLiked: false },
      { id: 3, user: "前端小菜鸟", avatar: "https://images.unsplash.com/photo-1494790108377-be9c29b29330?auto=format&fit=crop&q=80&w=100", text: "有没有大佬总结一下课代表笔记？太长了记不住😂", time: "1天前", likes: 342, isLiked: false },
      { id: 4, user: "TechGeek", avatar: "https://images.unsplash.com/photo-1527980965255-d3b416303d12?auto=format&fit=crop&q=80&w=100", text: "这套方案可以在生产环境中直接落地吗？有没有性能损耗的数据参考？", time: "2天前", likes: 128, isLiked: false },
    ]
  },
  {
    id: "c2",
    title: "Rust 编程实战，构建高性能服务端",
    instructor: "王五 · 后端技术专家",
    type: "video",
    level: "高阶",
    duration: "24课时",
    students: 3400,
    viewers: 0,
    cover: "https://images.unsplash.com/photo-1555066931-4365d14bab8c?auto=format&fit=crop&q=80&w=600",
    category: "backend",
    rating: 4.9,
    progress: 0,
    tags: ["服务端编程", "内存安全"]
  },
  {
    id: "c3",
    title: "AI 应用开发：提示词工程与模型微调实战指南",
    instructor: "AI团队 · 特邀专家组",
    type: "video",
    level: "进阶",
    duration: "16课时",
    students: 5600,
    viewers: 0,
    cover: "https://images.unsplash.com/photo-1677442136019-21780ecad995?auto=format&fit=crop&q=80&w=600",
    category: "ai",
    rating: 4.8,
    progress: 0,
    tags: ["Prompt", "Fine-tuning"]
  },
  {
    id: "c4",
    title: "产品级动效：高级 UI 交互设计与代码还原",
    instructor: "李四 · 全栈设计师",
    type: "video",
    level: "初级",
    duration: "10课时",
    students: 12000,
    viewers: 0,
    cover: "https://images.unsplash.com/photo-1611162617213-7d7a39e9b1d7?auto=format&fit=crop&q=80&w=600",
    category: "design",
    rating: 4.9,
    progress: 100,
    tags: ["Figma", "Framer Motion"]
  },
];

export const courseService = {
  getCourses: async (): Promise<Course[]> => {
    return [...COURSES];
  },
  
  getFeaturedCourse: async (): Promise<Course | null> => {
    return COURSES.find(c => c.id === 'l1') || null;
  }
};
