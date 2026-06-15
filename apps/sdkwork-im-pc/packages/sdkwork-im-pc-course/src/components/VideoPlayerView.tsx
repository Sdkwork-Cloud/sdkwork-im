import React, { useState, useEffect } from "react";
import { ChevronLeft, Play, Pause, Maximize, Volume2, Settings, MessageSquare, Share2, CheckCircle2, Circle, SkipForward, Clock, Star, Users, ArrowLeft, ThumbsUp, Heart, MessageCircle, X } from "lucide-react";
import { motion, AnimatePresence } from "motion/react";

interface VideoPlayerViewProps {
  course: any;
  onBack: () => void;
}

export const VideoPlayerView: React.FC<VideoPlayerViewProps> = ({ course, onBack }) => {
  const [isPlaying, setIsPlaying] = useState(false);
  const [playbackRate, setPlaybackRate] = useState(1);
  const [showSettings, setShowSettings] = useState(false);
  
  // Interactive states
  const [liked, setLiked] = useState(false);
  const [favorite, setFavorite] = useState(false);
  const [likeCount, setLikeCount] = useState(Math.floor(Math.random() * 50000) + 10000);
  const [favoriteCount, setFavoriteCount] = useState(Math.floor(Math.random() * 20000) + 5000);
  const [commentCount, setCommentCount] = useState(Math.floor(Math.random() * 10000) + 2000);
  
  const [activeTab, setActiveTab] = useState<"comments" | "chapters">("comments");
  const [inputText, setInputText] = useState("");
  const [followed, setFollowed] = useState(false);
  
  const [comments, setComments] = useState<any[]>(course.comments || []);
  const chapters = course.chapters || [];

  const handleLikeVideo = () => {
    setLiked(!liked);
    setLikeCount(prev => liked ? prev - 1 : prev + 1);
  };

  const handleFavoriteVideo = () => {
    setFavorite(!favorite);
    setFavoriteCount(prev => favorite ? prev - 1 : prev + 1);
  };

  const handleLikeComment = (id: number) => {
    setComments(comments.map(c => {
      if (c.id === id) {
        return { ...c, isLiked: !c.isLiked, likes: c.isLiked ? c.likes - 1 : c.likes + 1 };
      }
      return c;
    }));
  };

  const handleSendComment = () => {
    if (!inputText.trim()) return;
    const newComment = {
      id: Date.now(),
      user: "我",
      avatar: "https://images.unsplash.com/photo-1633332755192-727a05c4013d?auto=format&fit=crop&q=80&w=100",
      text: inputText,
      time: "刚刚",
      likes: 0,
      isLiked: false
    };
    setComments([newComment, ...comments]);
    setCommentCount(prev => prev + 1);
    setInputText("");
  };

  const formatNumber = (num: number) => {
    return num >= 10000 ? (num / 10000).toFixed(1) + 'w' : num.toString();
  };

  return (
    <motion.div 
      initial={{ opacity: 0, scale: 0.98 }}
      animate={{ opacity: 1, scale: 1 }}
      exit={{ opacity: 0, scale: 0.98 }}
      transition={{ duration: 0.3, ease: "easeOut" }}
      className="flex-1 flex h-full bg-[#09090b] overflow-hidden"
    >
      {/* Left: Immersive Player Area */}
      <div className="flex-1 flex flex-col relative h-full bg-black/95">
        {/* Header overlay for back button */}
        <div className="absolute top-0 left-0 right-0 h-24 bg-gradient-to-b from-black/80 to-transparent z-50 pointer-events-none flex items-start p-6">
          <button 
            onClick={onBack}
            className="group flex items-center justify-center w-10 h-10 rounded-full bg-black/40 hover:bg-black/60 backdrop-blur-md text-white transition-all pointer-events-auto shadow-lg border border-white/5 hover:border-white/20"
          >
            <ArrowLeft size={18} className="group-hover:-translate-x-0.5 transition-transform" />
          </button>
        </div>

        {/* Video Container */}
        <div className="flex-1 relative flex items-center justify-center w-full h-full group bg-black overflow-hidden">
          {/* Subtle Glow behind video */}
          <div className="absolute inset-0 bg-indigo-900/10 blur-3xl rounded-full scale-110"></div>
          
          <img src={course.cover} alt="Course Cover" className="relative z-10 max-w-full max-h-[100%] h-auto object-contain transition-opacity duration-300 shadow-2xl" style={{ opacity: isPlaying ? 0.3 : 1 }} />
          
          <div className="absolute inset-0 bg-transparent flex items-center justify-center cursor-pointer z-20" onClick={() => setIsPlaying(!isPlaying)}>
            {/* Play Pause Pulse Indicator */}
            <button 
              className={`w-20 h-20 rounded-full flex items-center justify-center backdrop-blur-md transition-all duration-300 ${
                isPlaying ? "bg-white/10 opacity-0 group-hover:opacity-100 scale-90 hover:bg-white/20 hover:scale-100" : "bg-white/20 hover:bg-white/30 scale-100 shadow-[0_0_40px_rgba(255,255,255,0.2)]"
              }`}
            >
              {isPlaying ? <Pause size={32} className="fill-white" /> : <Play size={36} className="fill-white ml-2" />}
            </button>
          </div>

          {/* Bottom Controls */}
          <div className="absolute bottom-0 left-0 right-0 h-32 bg-gradient-to-t from-black via-black/80 to-transparent px-6 pb-6 opacity-0 group-hover:opacity-100 transition-opacity duration-300 flex flex-col justify-end gap-4 z-30 pointer-events-auto">
            {/* Progress Bar Area */}
            <div className="relative pt-2 pb-2 group/progress cursor-pointer">
              {/* Timestamp Hover Hint (simulated) */}
              <div className="absolute top-0 left-[45%] -translate-x-1/2 -translate-y-full mb-2 bg-white text-black text-[10px] font-bold px-2 py-1 rounded shadow-lg opacity-0 group-hover/progress:opacity-100 transition-opacity">
                12:45
              </div>
              
              <div className="h-1 bg-white/20 rounded-full w-full relative transition-all duration-200 group-hover/progress:h-1.5">
                {/* Buffered */}
                <div className="absolute top-0 left-0 h-full bg-white/30 rounded-full w-[60%]"></div>
                {/* Played */}
                <div className="absolute top-0 left-0 h-full bg-indigo-500 rounded-full w-[45%] shadow-[0_0_10px_rgba(99,102,241,0.5)]"></div>
                {/* Handle */}
                <div className="absolute top-1/2 left-[45%] -translate-x-1/2 -translate-y-1/2 w-3.5 h-3.5 bg-white rounded-full shadow-md opacity-0 group-hover/progress:opacity-100 transition-transform scale-50 group-hover/progress:scale-100"></div>
              </div>
            </div>
            
            <div className="flex items-center justify-between">
              <div className="flex items-center gap-5 text-white">
                <button onClick={() => setIsPlaying(!isPlaying)} className="hover:text-indigo-400 transition-colors">
                  {isPlaying ? <Pause size={22} className="fill-current" /> : <Play size={22} className="fill-current" />}
                </button>
                <button className="hover:text-indigo-400 transition-colors">
                  <SkipForward size={20} className="fill-current" />
                </button>
                <span className="text-[13px] font-medium font-mono tracking-wide opacity-90 pt-0.5">12:45 <span className="opacity-50 mx-1">/</span> 25:30</span>
              </div>
              
              <div className="flex items-center gap-5 text-white">
                <div className="flex items-center gap-2 group/volume relative">
                  <button className="hover:text-indigo-400 transition-colors"><Volume2 size={20} /></button>
                  <div className="w-0 overflow-hidden group-hover/volume:w-16 transition-all duration-300 flex items-center">
                    <div className="w-12 h-1 bg-white/20 rounded-full relative cursor-pointer">
                      <div className="absolute top-0 left-0 h-full w-[70%] bg-indigo-400 rounded-full"></div>
                    </div>
                  </div>
                </div>
                <div className="flex items-center gap-1 bg-white/10 px-2.5 py-1 rounded text-[13px] font-medium cursor-pointer hover:bg-white/20 relative transition-colors border border-white/5">
                  <span onClick={() => setShowSettings(!showSettings)}>{playbackRate}x</span>
                  <AnimatePresence>
                    {showSettings && (
                      <motion.div 
                        initial={{ opacity: 0, y: 10, scale: 0.95 }}
                        animate={{ opacity: 1, y: 0, scale: 1 }}
                        exit={{ opacity: 0, y: 10, scale: 0.95 }}
                        className="absolute bottom-full right-0 mb-2 bg-[#18181b] backdrop-blur-xl rounded-xl overflow-hidden py-1.5 w-20 text-center border border-white/10 shadow-2xl"
                      >
                        {[2, 1.5, 1.25, 1, 0.75].map(r => (
                          <div key={r} onClick={() => { setPlaybackRate(r); setShowSettings(false); }} className={`py-1.5 text-[13px] font-medium hover:bg-white/5 cursor-pointer ${playbackRate === r ? 'text-indigo-400 font-bold' : 'text-gray-300'}`}>{r}x 速跑</div>
                        ))}
                      </motion.div>
                    )}
                  </AnimatePresence>
                </div>
                <button className="hover:text-indigo-400 transition-colors"><Settings size={20} /></button>
                <button className="hover:text-indigo-400 transition-colors"><Maximize size={20} /></button>
              </div>
            </div>
          </div>
        </div>
      </div>

      {/* Right: Interaction Sidebar (Bilibili / Douyin premium style) */}
      <div className="w-[400px] bg-[#121214] border-l border-white/5 flex flex-col shrink-0 relative z-20 shadow-2xl">
        
        {/* Author Info & Video Meta */}
        <div className="p-6 border-b border-white/5 bg-gradient-to-b from-[#18181b]/80 to-[#121214]">
          <div className="flex items-center justify-between mb-5">
            <div className="flex items-center gap-3 cursor-pointer group">
              <div className="w-12 h-12 rounded-full overflow-hidden border-2 border-white/10 group-hover:border-indigo-400/50 transition-colors shadow-md">
                 <img src={`https://ui-avatars.com/api/?name=${course.instructor}&background=random`} alt="Author" className="w-full h-full object-cover" />
              </div>
              <div className="flex flex-col justify-center">
                <h3 className="font-bold text-[15px] text-white flex items-center gap-2">
                  {course.instructor.split('·')[0].trim()}
                  <span className="bg-gradient-to-r from-orange-400 to-red-500 text-black text-[9px] px-1.5 py-0.5 rounded uppercase font-bold tracking-wider shadow-sm">UP</span>
                </h3>
                <p className="text-[12px] text-gray-500 font-medium">14.5w 粉丝追踪</p>
              </div>
            </div>
            <button 
              onClick={() => setFollowed(!followed)}
              className={`px-5 py-1.5 font-bold text-[13px] rounded-full transition-colors shadow-sm whitespace-nowrap ${
                followed 
                  ? "bg-white/10 text-white hover:bg-white/20 border border-white/5" 
                  : "bg-white text-black hover:bg-gray-200"
              }`}
            >
              {followed ? "已关注" : "+ 关注"}
            </button>
          </div>

          <h1 className="text-[16px] text-white font-bold leading-snug mb-3">
            {course.title} {course.level && `【${course.level}】`}
          </h1>
          <div className="flex flex-wrap gap-2 mb-4">
            {course.tags?.map((tag: string) => (
              <span key={tag} className="text-[11px] font-bold text-indigo-400 bg-indigo-500/10 px-2 py-0.5 rounded px-2">#{tag}</span>
            ))}
          </div>
          <div className="text-[11px] font-medium text-gray-500 flex items-center gap-3">
            <span className="flex items-center gap-1"><Users size={12}/> {course.students?.toLocaleString() || "1.2w"} 人在学</span>
            <span className="flex items-center gap-1"><Clock size={12}/> {course.updatedAt || "12小时前"} 更新</span>
          </div>
        </div>

        {/* Action Bar (Horizontal in right sidebar) */}
        <div className="flex items-center justify-between px-8 py-4 border-b border-white/5 bg-[#18181b]/30">
          <button onClick={handleLikeVideo} className="flex gap-2 items-center group transition-transform active:scale-95">
            <div className={`w-9 h-9 rounded-full flex items-center justify-center transition-colors ${liked ? 'bg-red-500/10' : 'bg-white/5 group-hover:bg-white/10'}`}>
              <Heart size={18} className={liked ? "fill-red-500 text-red-500" : "text-white"} />
            </div>
            <span className={`text-[13px] font-bold ${liked ? "text-red-500" : "text-gray-400"}`}>{formatNumber(likeCount)}</span>
          </button>
          <button className="flex gap-2 items-center group transition-transform active:scale-95">
            <div className={`w-9 h-9 rounded-full flex items-center justify-center bg-white/5 group-hover:bg-white/10 transition-colors`}>
              <MessageCircle size={18} className="text-white" />
            </div>
            <span className="text-[13px] font-bold text-gray-400">{formatNumber(commentCount)}</span>
          </button>
          <button onClick={handleFavoriteVideo} className="flex gap-2 items-center group transition-transform active:scale-95">
            <div className={`w-9 h-9 rounded-full flex items-center justify-center transition-colors ${favorite ? 'bg-yellow-500/10' : 'bg-white/5 group-hover:bg-white/10'}`}>
              <Star size={18} className={favorite ? "fill-yellow-500 text-yellow-500" : "text-white"} />
            </div>
            <span className={`text-[13px] font-bold ${favorite ? "text-yellow-500" : "text-gray-400"}`}>收藏</span>
          </button>
          <button className="w-9 h-9 flex items-center justify-center rounded-full bg-white/5 hover:bg-white/10 transition-colors text-white">
            <Share2 size={16} />
          </button>
        </div>

        {/* Tabs for Comments / Chapters */}
        <div className="flex px-6 pt-2 bg-[#121214] sticky top-0 z-10">
          <button 
            onClick={() => setActiveTab("comments")}
            className={`mr-6 py-3 text-[14px] font-bold relative transition-colors ${activeTab === "comments" ? "text-white" : "text-gray-500 hover:text-gray-300"}`}
          >
            互动评论 <span className="text-[10px] ml-1 bg-white/10 px-1.5 py-0.5 rounded">{formatNumber(commentCount)}</span>
            {activeTab === "comments" && <div className="absolute bottom-0 left-1/2 -translate-x-1/2 w-6 h-[3px] bg-indigo-500 rounded-t-full"></div>}
          </button>
          <button 
            onClick={() => setActiveTab("chapters")}
            className={`py-3 text-[14px] font-bold relative transition-colors ${activeTab === "chapters" ? "text-white" : "text-gray-500 hover:text-gray-300"}`}
          >
            课程目录
            {activeTab === "chapters" && <div className="absolute bottom-0 left-1/2 -translate-x-1/2 w-6 h-[3px] bg-indigo-500 rounded-t-full"></div>}
          </button>
        </div>

        {/* Scrollable Content */}
        <div className="flex-1 overflow-y-auto custom-scrollbar bg-[#121214]">
          {activeTab === "comments" ? (
            <div className="p-6 space-y-6">
              {comments.length === 0 ? (
                <div className="py-12 text-center text-gray-500 text-[13px]">暂无评论，快来抢沙发吧~</div>
              ) : (
                comments.map(comment => (
                  <div key={comment.id} className="flex gap-3 group/comment">
                    <img src={comment.avatar} alt="Avatar" className="w-9 h-9 rounded-full object-cover shrink-0 border border-white/5" />
                    <div className="flex-1 min-w-0">
                      <div className="flex items-center gap-2 mb-1.5">
                        <span className="text-[13px] font-bold text-gray-400 hover:text-white transition-colors cursor-pointer">{comment.user}</span>
                        {comment.user === "我" && <span className="text-[9px] bg-indigo-500/20 text-indigo-400 px-1.5 rounded-sm">UP 主</span>}
                      </div>
                      <div className="text-[14px] text-gray-200 leading-relaxed mb-2 break-words font-medium">{comment.text}</div>
                      <div className="flex items-center justify-between mt-1">
                        <span className="text-[11px] text-gray-500 font-medium">{comment.time}</span>
                        <div className="flex items-center gap-4">
                          <button 
                            onClick={() => handleLikeComment(comment.id)}
                            className={`flex items-center gap-1.5 transition-colors ${comment.isLiked ? 'text-red-500' : 'text-gray-500 hover:text-gray-300'}`}
                          >
                            <Heart size={14} className={comment.isLiked ? "fill-current" : ""} />
                            <span className="text-[12px] font-medium">{comment.likes > 0 ? comment.likes : '赞'}</span>
                          </button>
                          <button className="text-gray-500 hover:text-gray-300 opacity-0 group-hover/comment:opacity-100 transition-opacity">
                            <MessageCircle size={14} />
                          </button>
                        </div>
                      </div>
                    </div>
                  </div>
                ))
              )}
            </div>
          ) : (
            <div className="p-6 space-y-8">
              {chapters.length === 0 ? (
                <div className="py-12 text-center text-gray-500 text-[13px]">暂无章节信息</div>
              ) : (
                chapters.map((chapter: any, chapterIdx: number) => (
                  <div key={chapterIdx} className="relative">
                    <h4 className="text-[14px] font-bold text-white mb-4 ml-1 flex items-center gap-2">
                      <div className="w-1.5 h-1.5 rounded-full bg-indigo-500"></div>
                      {chapter.title}
                    </h4>
                    <div className="space-y-2 relative before:absolute before:left-[11px] before:top-4 before:bottom-4 before:w-px before:bg-white/10">
                      {chapter.lessons.map((lesson: any) => (
                        <div 
                          key={lesson.id} 
                          className={`group relative flex items-start gap-4 p-3 rounded-xl cursor-pointer transition-all border border-transparent ${
                            lesson.status === "playing" 
                              ? "bg-indigo-500/10 border-indigo-500/20 shadow-inner block" 
                              : "hover:bg-white/5"
                          }`}
                        >
                          <div className="shrink-0 mt-0.5 flex items-center justify-center w-6 h-6 bg-[#121214] relative z-10">
                            {lesson.status === "completed" && <CheckCircle2 size={16} className="text-green-500" />}
                            {lesson.status === "playing" && (
                              <div className="w-full h-full flex items-center justify-center">
                                <span className="flex gap-0.5 items-end h-3">
                                  <span className="w-0.5 h-full bg-indigo-400 animate-[bounce_1s_infinite]"></span>
                                  <span className="w-0.5 h-[60%] bg-indigo-400 animate-[bounce_1s_infinite_0.2s]"></span>
                                  <span className="w-0.5 h-[80%] bg-indigo-400 animate-[bounce_1s_infinite_0.4s]"></span>
                                </span>
                              </div>
                            )}
                            {lesson.status === "locked" && <span className="text-[11px] font-mono text-gray-600 font-medium">0{lesson.id.replace('l','')}</span>}
                          </div>
                          <div className="flex-1 min-w-0">
                            <div className={`text-[13px] font-bold line-clamp-2 leading-snug mb-1 transition-colors ${
                              lesson.status === "playing" ? "text-indigo-400" :
                              lesson.status === "locked" ? "text-gray-500" : "text-gray-200 group-hover:text-white"
                            }`}>
                              {lesson.title}
                            </div>
                            <div className={`text-[11px] font-mono font-medium ${lesson.status === 'playing' ? 'text-indigo-400/70' : 'text-gray-600'}`}>
                              {lesson.duration}
                            </div>
                          </div>
                          {lesson.status === "playing" && (
                            <div className="absolute right-3 top-1/2 -translate-y-1/2 text-[10px] text-indigo-400/80 font-bold uppercase tracking-wider hidden sm:block">
                              正在播放
                            </div>
                          )}
                        </div>
                      ))}
                    </div>
                  </div>
                ))
              )}
            </div>
          )}
        </div>

        {/* Comment Input */}
        {activeTab === "comments" && (
          <div className="p-4 bg-[#18181b] border-t border-white/5">
            <div className="flex items-center gap-3">
              <img src="https://images.unsplash.com/photo-1633332755192-727a05c4013d?auto=format&fit=crop&q=80&w=100" className="w-8 h-8 rounded-full border border-white/10" alt="Me" />
              <div className="flex-1 flex gap-2 relative bg-black/50 border border-white/10 focus-within:border-indigo-500/50 rounded-full transition-colors pl-4 pr-1">
                <input 
                  type="text" 
                  value={inputText}
                  onChange={(e) => setInputText(e.target.value)}
                  onKeyDown={(e) => e.key === 'Enter' && handleSendComment()}
                  placeholder="善语结善缘，恶言伤人心..."
                  className="flex-1 bg-transparent py-2.5 text-[13px] text-white outline-none placeholder:text-gray-600 font-medium"
                />
                <button 
                  onClick={handleSendComment}
                  className={`w-8 h-8 my-1 rounded-full flex items-center justify-center shrink-0 transition-all ${
                    inputText.trim() ? "bg-indigo-500 text-white shadow-lg shadow-indigo-500/20" : "bg-white/5 text-gray-500"
                  }`}
                >
                  <ArrowLeft size={16} className="rotate-180" />
                </button>
              </div>
            </div>
          </div>
        )}
      </div>

    </motion.div>
  );
};


