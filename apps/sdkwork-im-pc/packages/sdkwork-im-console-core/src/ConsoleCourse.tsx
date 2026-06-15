import React, { useState } from 'react';
import { 
  Plus, Search, Edit, Trash2, Video, Airplay, 
  Users, BarChart2, MoreVertical, PlayCircle
} from 'lucide-react';
import { cn } from '@sdkwork/im-pc-commons';

interface Course {
  id: string;
  title: string;
  type: 'video' | 'live';
  status: 'draft' | 'published' | 'ended';
  instructor: string;
  enrollment: number;
  duration: string;
  price: number;
  updatedAt: string;
}

const MOCK_COURSES: Course[] = [
  { id: '1', title: '前端高阶架构与工程化实战', type: 'video', status: 'published', instructor: '张老师', enrollment: 1250, duration: '12h 30m', price: 999, updatedAt: '2024-05-20' },
  { id: '2', title: 'React 19 核心原理解析', type: 'video', status: 'published', instructor: '李老师', enrollment: 820, duration: '8h 15m', price: 499, updatedAt: '2024-05-18' },
  { id: '3', title: '企业级大模型应用落地实战研讨会', type: 'live', status: 'published', instructor: 'AI专家组', enrollment: 300, duration: '2h 00m', price: 199, updatedAt: '2024-05-25' },
  { id: '4', title: 'Flutter 跨端开发跨系统实战', type: 'video', status: 'draft', instructor: '王老师', enrollment: 0, duration: '0h 0m', price: 599, updatedAt: '2024-05-27' },
];

export const ConsoleCourse: React.FC = () => {
  const [courses, setCourses] = useState<Course[]>(MOCK_COURSES);
  const [activeTab, setActiveTab] = useState<'all' | 'video' | 'live'>('all');
  const [searchQuery, setSearchQuery] = useState('');

  const filteredCourses = courses.filter(c => {
    if (activeTab !== 'all' && c.type !== activeTab) return false;
    if (searchQuery && !c.title.toLowerCase().includes(searchQuery.toLowerCase())) return false;
    return true;
  });

  const getStatusBadge = (status: string) => {
    switch (status) {
      case 'published': return <span className="px-2 py-0.5 rounded text-[11px] font-medium bg-green-500/10 text-green-500 border border-green-500/20">已发布</span>;
      case 'draft': return <span className="px-2 py-0.5 rounded text-[11px] font-medium bg-gray-500/10 text-gray-500 border border-gray-500/20">草稿</span>;
      case 'ended': return <span className="px-2 py-0.5 rounded text-[11px] font-medium bg-red-500/10 text-red-500 border border-red-500/20">已结束</span>;
      default: return null;
    }
  };

  return (
    <div className="flex flex-col h-full bg-console-bg-panel rounded-xl border border-console-border overflow-hidden">
      {/* Header */}
      <div className="p-6 border-b border-console-border flex flex-wrap gap-4 items-center justify-between bg-console-bg-panel/50">
        <div>
          <h2 className="text-lg font-semibold text-console-text-main mb-1">课程管理</h2>
          <p className="text-sm text-console-text-muted">管理企业的内部培训与对外售卖的在线课程、直播培训</p>
        </div>
        <button className="px-4 py-2 bg-blue-600 hover:bg-blue-700 text-white rounded-lg text-sm font-medium transition-colors flex items-center gap-2 shadow-sm">
          <Plus size={16} /> 创建课程
        </button>
      </div>

      {/* Filters and Actions */}
      <div className="px-6 py-4 border-b border-console-border flex flex-wrap gap-4 items-center justify-between">
        <div className="flex bg-console-input-bg p-1 rounded-lg border border-console-border">
          <button 
            onClick={() => setActiveTab('all')}
            className={cn("px-4 py-1.5 rounded-md text-sm transition-colors", activeTab === 'all' ? "bg-console-active-bg text-console-active-text font-medium shadow-sm" : "text-console-text-muted hover:text-console-text-main")}
          >
            全部课程
          </button>
          <button 
            onClick={() => setActiveTab('video')}
            className={cn("px-4 py-1.5 rounded-md text-sm transition-colors flex items-center gap-1.5", activeTab === 'video' ? "bg-console-active-bg text-console-active-text font-medium shadow-sm" : "text-console-text-muted hover:text-console-text-main")}
          >
            <Video size={14} /> 视频课
          </button>
          <button 
            onClick={() => setActiveTab('live')}
            className={cn("px-4 py-1.5 rounded-md text-sm transition-colors flex items-center gap-1.5", activeTab === 'live' ? "bg-console-active-bg text-console-active-text font-medium shadow-sm" : "text-console-text-muted hover:text-console-text-main")}
          >
            <Airplay size={14} /> 直播课
          </button>
        </div>

        <div className="flex items-center gap-3">
          <div className="relative">
            <Search size={16} className="absolute left-3 top-1/2 -translate-y-1/2 text-console-text-muted" />
            <input 
              type="text" 
              placeholder="搜索课程名称..." 
              value={searchQuery}
              onChange={(e) => setSearchQuery(e.target.value)}
              className="w-64 bg-console-input-bg border-console-border border rounded-lg py-1.5 pl-9 pr-4 text-sm text-console-text-main focus:border-blue-500 focus:ring-1 focus:ring-blue-500 outline-none transition-all placeholder:text-console-text-muted"
            />
          </div>
        </div>
      </div>

      {/* Table grid */}
      <div className="flex-1 overflow-auto custom-scrollbar">
        <table className="w-full text-left border-collapse">
          <thead>
            <tr className="border-b border-console-border bg-console-bg-panel/50 sticky top-0 z-10 backdrop-blur-sm">
              <th className="px-6 py-4 text-xs font-semibold text-console-text-muted uppercase tracking-wider">课程信息</th>
              <th className="px-6 py-4 text-xs font-semibold text-console-text-muted uppercase tracking-wider">讲师</th>
              <th className="px-6 py-4 text-xs font-semibold text-console-text-muted uppercase tracking-wider">学习人数</th>
              <th className="px-6 py-4 text-xs font-semibold text-console-text-muted uppercase tracking-wider">状态</th>
              <th className="px-6 py-4 text-xs font-semibold text-console-text-muted uppercase tracking-wider">价格</th>
              <th className="px-6 py-4 text-right text-xs font-semibold text-console-text-muted uppercase tracking-wider">操作</th>
            </tr>
          </thead>
          <tbody className="divide-y divide-console-border">
            {filteredCourses.length > 0 ? filteredCourses.map(course => (
              <tr key={course.id} className="hover:bg-console-bg-hover transition-colors group">
                <td className="px-6 py-4">
                  <div className="flex items-start gap-4">
                    <div className="w-24 h-16 rounded-lg bg-console-input-bg border border-console-border flex flex-col items-center justify-center text-console-text-muted shrink-0 relative overflow-hidden">
                      {course.type === 'video' ? <PlayCircle size={24} className="opacity-50" /> : <Airplay size={24} className="opacity-50" />}
                      <div className="absolute bottom-1 right-1 text-[10px] bg-black/60 text-white px-1 rounded backdrop-blur-sm">{course.duration}</div>
                    </div>
                    <div>
                      <h4 className="text-sm font-medium text-console-text-main group-hover:text-blue-500 transition-colors cursor-pointer line-clamp-1 mb-1">{course.title}</h4>
                      <p className="text-[12px] text-console-text-muted flex items-center gap-2">
                        {course.type === 'video' ? <span className="flex items-center gap-1 text-blue-500"><Video size={12}/> 录播</span> : <span className="flex items-center gap-1 text-purple-500"><Airplay size={12}/> 直播</span>}
                        <span>•</span>
                        <span>更新于 {course.updatedAt}</span>
                      </p>
                    </div>
                  </div>
                </td>
                <td className="px-6 py-4">
                  <div className="flex items-center gap-2">
                    <div className="w-6 h-6 rounded-full bg-console-border flex items-center justify-center text-[10px] text-console-text-main font-bold">
                      {course.instructor[0]}
                    </div>
                    <span className="text-sm text-console-text-main">{course.instructor}</span>
                  </div>
                </td>
                <td className="px-6 py-4">
                  <div className="flex items-center gap-2 text-sm text-console-text-main">
                    <Users size={16} className="text-console-text-muted" />
                    {course.enrollment}
                  </div>
                </td>
                <td className="px-6 py-4">
                  {getStatusBadge(course.status)}
                </td>
                <td className="px-6 py-4">
                  <span className="text-sm font-medium text-console-text-main">
                    {course.price === 0 ? '免费' : `¥${course.price}`}
                  </span>
                </td>
                <td className="px-6 py-4 text-right">
                  <div className="flex flex-row justify-end items-center gap-2 opacity-0 group-hover:opacity-100 transition-opacity">
                    <button className="p-1.5 text-console-text-muted hover:text-blue-500 hover:bg-blue-500/10 rounded-md transition-colors" title="编辑">
                      <Edit size={16} />
                    </button>
                    <button className="p-1.5 text-console-text-muted hover:text-blue-500 hover:bg-blue-500/10 rounded-md transition-colors" title="数据分析">
                      <BarChart2 size={16} />
                    </button>
                    <button className="p-1.5 text-console-text-muted hover:text-red-500 hover:bg-red-500/10 rounded-md transition-colors" title="删除">
                      <Trash2 size={16} />
                    </button>
                  </div>
                </td>
              </tr>
            )) : (
              <tr>
                <td colSpan={6} className="px-6 py-16 text-center text-console-text-muted">
                  <div className="flex flex-col items-center justify-center">
                    <Video size={32} className="opacity-20 mb-3" />
                    <p>没有匹配的课程数据</p>
                  </div>
                </td>
              </tr>
            )}
          </tbody>
        </table>
      </div>

    </div>
  );
};
