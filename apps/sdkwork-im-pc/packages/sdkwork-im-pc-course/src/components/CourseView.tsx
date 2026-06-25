import React, { useState, useEffect } from "react";
import { PlayCircle, BookOpen, Star, Clock, Users, Award, Play, CheckCircle2, Tv, Sparkles, TrendingUp } from "lucide-react";
import { motion, AnimatePresence } from "motion/react";
import { VideoPlayerView } from "./VideoPlayerView";
import { LiveRoomView } from "./LiveRoomView";
import { courseService, Course, CourseCategory } from "../services/CourseService";

export const CourseView: React.FC = () => {
  const [activeCategory, setActiveCategory] = useState<string>("all");
  const [selectedCourse, setSelectedCourse] = useState<Course | null>(null);
  const [courses, setCourses] = useState<Course[]>([]);
  const [featuredCourse, setFeaturedCourse] = useState<Course | null>(null);

  useEffect(() => {
    const fetchData = async () => {
      const fetchedCourses = await courseService.getCourses();
      setCourses(fetchedCourses);
      const featured = await courseService.getFeaturedCourse();
      setFeaturedCourse(featured);
    };
    fetchData();
  }, []);

  const categories = [
    { id: "all", name: "综合推荐" },
    { id: "live", name: "公开课/直播" },
    { id: "design", name: "UI/UX 设计" },
    { id: "frontend", name: "前端架构" },
    { id: "backend", name: "后端与云原生" },
    { id: "ai", name: "AI 与大模型" },
  ];

  const filteredCourses = activeCategory === "all" ? courses : courses.filter(c => c.category === activeCategory);

  return (
    <AnimatePresence mode="wait">
      {selectedCourse && selectedCourse.type === "video" ? (
        <React.Fragment key="detail-video">
          <VideoPlayerView course={selectedCourse} onBack={() => setSelectedCourse(null)} />
        </React.Fragment>
      ) : selectedCourse && selectedCourse.type === "live" ? (
        <React.Fragment key="detail-live">
          <LiveRoomView course={selectedCourse} onBack={() => setSelectedCourse(null)} />
        </React.Fragment>
      ) : (
        <motion.div 
          key="list"
          initial={{ opacity: 0 }}
          animate={{ opacity: 1 }}
          exit={{ opacity: 0 }}
          className="flex-1 flex flex-col h-full bg-[#09090b] text-gray-200 overflow-hidden"
        >
          {/* Header */}
          <div className="h-16 border-b border-white/5 flex items-center justify-between px-8 bg-[#09090b]/90 backdrop-blur-xl shrink-0 sticky top-0 z-50">
            <div className="flex items-center gap-6">
              <h2 className="text-[17px] font-bold tracking-tight text-white flex items-center gap-2.5">
                <div className="w-8 h-8 rounded-lg bg-gradient-to-tr from-indigo-500 to-purple-500 flex items-center justify-center text-white shadow-lg">
                  <PlayCircle size={18} className="fill-white/20" />
                </div>
                精选发现
              </h2>
            </div>
            <div className="flex items-center gap-6 text-[14px]">
              <div className="flex items-center gap-2 text-gray-400 hover:text-white cursor-pointer transition-colors font-medium">
                <Award size={18} />
                <span>学习成就</span>
              </div>
            </div>
          </div>

          <div className="flex-1 overflow-y-auto custom-scrollbar p-6 lg:p-8">
            <div className="max-w-7xl mx-auto space-y-12">
              
              {/* Recently Learning (Progress Context) */}
              <section>
                <div className="flex items-center justify-between mb-5">
                  <h3 className="font-bold text-white text-[16px] flex items-center gap-2">
                    <Clock size={18} className="text-indigo-400" /> 继续学习
                  </h3>
                  <button className="text-[13px] text-gray-500 hover:text-indigo-400 transition-colors font-medium">查看全部进度 &rarr;</button>
                </div>
                <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-5">
                  {courses.filter(c => c.progress > 0 && c.progress < 100).map(course => (
                    <div 
                      key={`recent-${course.id}`}
                      onClick={() => {
                        void courseService.getCourseDetail(course.id).then((detail) => {
                          setSelectedCourse(detail ?? course);
                        });
                      }}
                      className="bg-[#121214] border border-white/5 hover:border-white/10 p-4 rounded-2xl cursor-pointer group flex gap-4 items-center shadow-lg transition-colors"
                    >
                      <div className="w-28 h-[72px] shrink-0 rounded-xl overflow-hidden relative">
                        <img src={course.cover} className="w-full h-full object-cover group-hover:scale-105 transition-transform" />
                        <div className="absolute inset-0 bg-black/40 flex items-center justify-center opacity-0 group-hover:opacity-100 transition-opacity">
                          <Play size={20} className="fill-white text-white" />
                        </div>
                      </div>
                      <div className="flex-1 min-w-0">
                        <h4 className="text-[14px] font-bold text-gray-200 line-clamp-1 mb-1 group-hover:text-white transition-colors">{course.title}</h4>
                        <div className="text-[12px] text-gray-500 mb-2">已学 4/12 课时</div>
                        <div className="h-1.5 w-full bg-[#18181b] rounded-full overflow-hidden">
                          <div className="h-full bg-indigo-500 rounded-full" style={{ width: `${course.progress}%` }}></div>
                        </div>
                      </div>
                    </div>
                  ))}
                </div>
              </section>

              {/* Featured / Live Hero */}
              {featuredCourse && (
                <section 
                  className="relative rounded-[2rem] overflow-hidden group border border-white/5 bg-[#121214] shadow-2xl cursor-pointer min-h-[400px] flex items-end"
                  onClick={() => {
                    if (!featuredCourse) {
                      return;
                    }
                    void courseService.getCourseDetail(featuredCourse.id).then((detail) => {
                      setSelectedCourse(detail ?? featuredCourse);
                    });
                  }}
                >
                  <div className="absolute inset-0 z-0 bg-black">
                    <img src={featuredCourse.cover} alt="Hero" className="w-full h-full object-cover opacity-40 group-hover:opacity-50 group-hover:scale-105 transition-all duration-1000 ease-out" />
                    <div className="absolute inset-0 bg-gradient-to-t from-black via-black/40 to-transparent mix-blend-multiply"></div>
                    <div className="absolute inset-0 bg-gradient-to-r from-black/80 to-transparent"></div>
                  </div>
                  
                  <div className="relative z-10 p-10 md:p-14 w-full flex flex-col items-start gap-4">
                    <div className="flex gap-3 mb-1 items-center">
                      <span className="flex items-center gap-1.5 px-3 py-1 bg-red-600/90 backdrop-blur-md border border-red-500/50 text-white rounded text-[11px] font-bold tracking-widest uppercase shadow-lg shadow-red-900/30">
                        <span className="w-1.5 h-1.5 rounded-full bg-white animate-pulse"></span>
                        正在直播
                      </span>
                    </div>
                    <h1 className="text-3xl md:text-5xl font-bold text-white leading-tight mb-2 tracking-tight max-w-3xl drop-shadow-lg">
                      {featuredCourse.title}
                    </h1>
                    
                    <div className="flex items-center gap-5 mt-2">
                      <p className="text-gray-300 flex items-center gap-2 font-medium text-[15px]">
                        <span className="w-6 h-6 rounded-full bg-gradient-to-tr from-indigo-500 to-purple-500 flex items-center justify-center text-white font-bold text-[10px] shadow-inner">{featuredCourse.instructor?.[0] || 'M'}</span>
                        {featuredCourse.instructor}
                      </p>
                      <div className="w-1 h-1 rounded-full bg-gray-600"></div>
                      <div className="flex items-center gap-1.5 text-gray-300 font-medium text-[14px]">
                        <Users size={16} className="text-gray-400" /> {featuredCourse.viewers.toLocaleString()} 人正在观看
                      </div>
                    </div>

                    <button className="mt-8 flex items-center gap-2 px-8 py-3.5 bg-white text-black hover:bg-gray-200 rounded-xl font-bold transition-all group-hover:scale-105 active:scale-95 shadow-xl">
                      <Play size={20} className="fill-black" />
                      进入沉浸式课堂
                    </button>
                  </div>
                </section>
              )}

              {/* Course Catalog */}
              <section className="space-y-6">
                {/* Modern Taxonomy Tabs */}
                <div className="flex items-center justify-between sticky top-0 py-3 bg-[#09090b]/90 backdrop-blur-xl z-20">
                  <div className="flex gap-2 overflow-x-auto custom-scrollbar">
                    {categories.map(cat => (
                      <button
                        key={cat.id}
                        onClick={(e) => {
                          e.stopPropagation();
                          setActiveCategory(cat.id);
                        }}
                        className={`px-5 py-2.5 rounded-xl text-[14px] font-bold transition-all whitespace-nowrap tracking-wide ${
                          activeCategory === cat.id 
                            ? "bg-white text-black shadow-lg" 
                            : "bg-[#18181b] text-gray-400 hover:text-white hover:bg-[#27272a] border border-transparent hover:border-white/5"
                        }`}
                      >
                        {cat.name}
                      </button>
                    ))}
                  </div>
                  <div className="hidden md:flex items-center gap-4 text-[13px] font-medium text-gray-400">
                    <button className="flex items-center gap-1.5 hover:text-white transition-colors"><TrendingUp size={16} /> 最新上架</button>
                    <div className="w-px h-3 bg-white/10"></div>
                    <button className="flex items-center gap-1.5 hover:text-white transition-colors"><Star size={16} /> 评分最高</button>
                  </div>
                </div>

                {/* Grid layout with premium cards */}
                <motion.div layout className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-6">
                  <AnimatePresence>
                    {filteredCourses.map(course => (
                      <motion.div 
                        layout 
                        initial={{ opacity: 0, scale: 0.95 }}
                        animate={{ opacity: 1, scale: 1 }}
                        exit={{ opacity: 0, scale: 0.95 }}
                        transition={{ duration: 0.3 }}
                        key={course.id} 
                        onClick={() => {
                        void courseService.getCourseDetail(course.id).then((detail) => {
                          setSelectedCourse(detail ?? course);
                        });
                      }}
                        className="bg-[#121214] rounded-2xl overflow-hidden border border-white/5 hover:border-white/20 hover:-translate-y-1 transition-all duration-300 group cursor-pointer flex flex-col h-full shadow-lg relative"
                      >
                        <div className="aspect-video overflow-hidden relative">
                          <img src={course.cover} alt={course.title} className="w-full h-full object-cover group-hover:scale-105 transition-transform duration-700" />
                          
                          <div className="absolute top-3 left-3 flex gap-2">
                            {course.type === "live" ? (
                              <div className="px-2 py-0.5 bg-red-600/90 backdrop-blur-md rounded text-[10px] font-bold tracking-widest text-white flex items-center gap-1 shadow-lg h-6">
                                <span className="w-1.5 h-1.5 rounded-full bg-white animate-pulse"></span>
                                LIVE
                              </div>
                            ) : (
                              <div className="px-2.5 py-0.5 bg-black/60 backdrop-blur-md rounded border border-white/10 text-[10px] font-bold text-gray-200 tracking-wider h-6 flex items-center">
                                {course.level}
                              </div>
                            )}
                          </div>
                          
                          {/* Hover Play Button */}
                          <div className="absolute inset-0 bg-black/40 opacity-0 group-hover:opacity-100 transition-opacity flex items-center justify-center backdrop-blur-[2px]">
                            <div className="w-14 h-14 bg-white/20 backdrop-blur-xl rounded-full flex items-center justify-center border border-white/30 shadow-2xl scale-90 group-hover:scale-100 transition-transform">
                              <Play size={24} className="text-white ml-0.5 fill-white" />
                            </div>
                          </div>
                        </div>
                        
                        <div className="p-5 flex flex-col flex-1 relative bg-gradient-to-b from-[#121214] to-[#09090b]">
                          {/* Tags */}
                          {course.tags && course.tags.length > 0 && course.type !== 'live' && (
                            <div className="flex gap-2 mb-3">
                              {course.tags.map((tag: string) => (
                                <span key={tag} className="text-[10px] font-bold text-indigo-400 bg-indigo-400/10 px-2 py-0.5 rounded">
                                  {tag}
                                </span>
                              ))}
                            </div>
                          )}

                          <h3 className="font-bold text-gray-100 leading-snug mb-2 group-hover:text-white transition-colors line-clamp-2 text-[15px]">
                            {course.title}
                          </h3>
                          <p className="text-[13px] text-gray-500 mb-5 flex-1 line-clamp-1 font-medium">
                            {course.instructor}
                          </p>
                          
                          {course.progress === 100 && (
                            <div className="absolute top-0 left-0 w-full bg-green-500/10 border-b border-green-500/20 text-green-500 text-[10px] font-bold text-center uppercase tracking-wider py-1">
                              已学完
                            </div>
                          )}
                          
                          <div className="flex items-center justify-between text-[11px] font-medium text-gray-500 pt-4 border-t border-white/5 mt-auto">
                            <span className="flex items-center gap-1.5">
                              {course.type === "live" ? <Users size={14} /> : <BookOpen size={14} />} 
                              {course.type === "live" ? `${course.viewers.toLocaleString()} 观看` : `${course.students.toLocaleString()} 在学`}
                            </span>
                            <span className="flex items-center gap-1.5">
                               {course.type !== "live" && <><Clock size={14} /> {course.duration}</>}
                            </span>
                          </div>
                        </div>
                      </motion.div>
                    ))}
                  </AnimatePresence>
                </motion.div>
                
                {filteredCourses.length === 0 && (
                  <div className="py-24 text-center border border-dashed border-white/10 rounded-3xl bg-[#121214] flex flex-col items-center">
                    <BookOpen size={48} className="mb-4 text-gray-700 opacity-50" />
                    <p className="text-lg font-medium text-gray-300">系列课程筹备中</p>
                    <p className="text-sm mt-2 font-medium text-gray-500">即将上线，敬请期待</p>
                  </div>
                )}
              </section>
            </div>
          </div>
        </motion.div>
      )}
    </AnimatePresence>
  );
};

