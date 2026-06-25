import React, { useState } from "react";
import { ChevronLeft, Play, Users, MessageSquare, Heart, Share2, Expand, MoreHorizontal, Maximize, Settings, Zap, Focus, ArrowLeft, Gift, Flame, Trophy, CheckCircle2 } from "lucide-react";
import { motion, AnimatePresence } from "motion/react";
import { Avatar } from "@sdkwork/im-pc-commons";
import { readAppSdkSessionTokens } from "@sdkwork/im-pc-core";
import { courseInteractionService } from "../services/CourseService";

interface LiveRoomViewProps {
  course: any;
  onBack: () => void;
}

export const LiveRoomView: React.FC<LiveRoomViewProps> = ({ course, onBack }) => {
  const [messages, setMessages] = useState<any[]>(course.messages || []);
  const [inputText, setInputText] = useState("");
  const [hearts, setHearts] = useState<{id: number, x: number}[]>([]);
  const [followed, setFollowed] = useState(false);
  const sessionUser = readAppSdkSessionTokens()?.user;
  const currentUserName = sessionUser?.displayName ?? sessionUser?.nickname ?? sessionUser?.name ?? "Me";

  const handleSend = async () => {
    if (!inputText.trim()) return;
    try {
      await courseInteractionService.sendLiveMessage(course.id, inputText.trim());
      setInputText("");
    } catch {
      return;
    }
  };

  const handleLike = () => {
    const newHeart = { id: Date.now(), x: Math.random() * 40 - 20 };
    setHearts([...hearts, newHeart]);
    setTimeout(() => {
      setHearts(current => current.filter(h => h.id !== newHeart.id));
    }, 2000);
  };

  return (
    <motion.div 
      initial={{ opacity: 0, scale: 0.98 }}
      animate={{ opacity: 1, scale: 1 }}
      exit={{ opacity: 0, scale: 0.98 }}
      transition={{ duration: 0.3 }}
      className="flex-1 flex h-full bg-black overflow-hidden relative"
    >
      {/* Left: Immersive Live Video Area */}
      <div className="flex-1 relative flex flex-col items-center justify-center bg-black/95 group overflow-hidden">
        
        {/* Main Stream Image */}
        <img src={course.cover} alt="Live Stream" className="absolute inset-0 w-full h-full object-contain opacity-90 blur-3xl scale-110 z-0" />
        <img src={course.cover} alt="Live Stream" className="relative z-10 max-w-full max-h-full object-contain opacity-100" />
        
        {/* Gradients for UI */}
        <div className="absolute inset-0 bg-gradient-to-t from-black/80 via-transparent to-black/40 z-20 pointer-events-none"></div>

        {/* Top Info Bar */}
        <div className="absolute top-0 left-0 right-0 p-6 flex justify-between items-start z-30">
          <div className="flex items-center gap-4">
            <button 
              onClick={onBack}
              className="flex items-center justify-center w-10 h-10 rounded-full bg-black/40 hover:bg-black/60 backdrop-blur-md text-white transition-all shadow-lg"
            >
              <ArrowLeft size={20} className="hover:-translate-x-0.5 transition-transform" />
            </button>
            <div className="flex items-center gap-3 bg-black/40 backdrop-blur-md rounded-full p-1 pr-4 border border-white/10 cursor-pointer hover:bg-black/60 transition-colors">
              <Avatar
                src={undefined}
                alt={course.instructor}
                fallback={course.instructor?.charAt(0) ?? "?"}
                size="sm"
                shape="circle"
                className="w-9 h-9"
              />
              <div className="pr-2">
                <h2 className="text-[14px] font-bold text-white leading-tight flex items-center gap-1">
                  {course.instructor}
                  <div className="flex items-center gap-1 bg-red-500 px-1 py-0.5 rounded ml-1">
                    <span className="w-1.5 h-1.5 rounded-full bg-white animate-pulse"></span>
                    <span className="text-[8px] font-bold text-white uppercase leading-none">Live</span>
                  </div>
                </h2>
                <p className="text-[11px] text-gray-300 font-medium">{course.viewers?.toLocaleString() ?? 0} 观看</p>
              </div>
              <button 
                onClick={() => setFollowed(!followed)}
                className={`w-8 h-8 rounded-full flex items-center justify-center text-white transition-colors shadow-lg ${
                  followed ? "bg-white/10 hover:bg-white/20" : "bg-indigo-500 hover:bg-indigo-400"
                }`}
              >
                {followed ? <CheckCircle2 size={14} className="text-white/80" /> : <span className="text-sm font-bold leading-none -mt-0.5">+</span>}
              </button>
            </div>
          </div>

          <div className="flex items-center gap-3 bg-black/40 backdrop-blur-md px-3 py-1.5 rounded-full border border-white/10">
            <div className="flex -space-x-2 mr-1">
              {[1,2,3].map(i => (
                <div key={i} className="w-6 h-6 rounded-full bg-gradient-to-tr from-blue-400 to-purple-400 border border-white/20 shadow-sm z-10 flex items-center justify-center text-[10px] font-bold text-white">
                  {i}
                </div>
              ))}
            </div>
            <div className="text-[12px] font-bold text-orange-400 flex items-center gap-1">
              <Flame size={14} className="fill-orange-400" />
              {course.viewers?.toLocaleString() || "12,500"}
            </div>
          </div>
        </div>

        {/* Live Title & Stats */}
        <div className="absolute top-24 left-6 z-30 pointer-events-none">
          <div className="bg-black/40 backdrop-blur-md px-4 py-2 rounded-xl border border-white/10 shadow-lg">
            <div className="text-white text-sm font-bold max-w-[300px] leading-snug">{course.title}</div>
            <div className="flex items-center gap-3 mt-1.5">
              <div className="text-[10px] text-white/70 font-mono bg-white/10 px-1.5 py-0.5 rounded">1080P PRO</div>
              <div className="flex items-center gap-1 text-[11px] text-yellow-400 font-bold">
                <Trophy size={10} /> 小时榜第3名
              </div>
            </div>
          </div>
        </div>

        {/* Teacher Cam PIP (Picture in Picture) */}
        <div className="absolute top-24 right-6 w-40 aspect-[3/4] bg-gradient-to-br from-indigo-900 to-purple-900 rounded-xl overflow-hidden border border-white/20 shadow-2xl group/pip cursor-move pointer-events-auto z-40 flex items-center justify-center">
            <Avatar
              src={undefined}
              alt={course.instructor}
              fallback={course.instructor?.charAt(0) ?? "?"}
              size="lg"
              shape="circle"
            />
            <div className="absolute bottom-1 left-1 right-1 flex items-center justify-between pointer-events-none">
              <div className="text-[10px] text-white font-medium bg-black/60 backdrop-blur-md px-1.5 py-0.5 rounded">
                讲师镜头
              </div>
            </div>
        </div>

        {/* Gift Animation Area */}
        {messages.length > 0 ? null : (
        <div className="absolute bottom-32 left-6 flex flex-col gap-2 pointer-events-none z-30">
          <div className="bg-black/40 backdrop-blur-md rounded-xl px-4 py-2 border border-white/10 text-[12px] text-gray-300">
            直播互动消息将在课程直播聊天合约开放后显示
          </div>
        </div>
        )}

        {/* Bottom Controls */}
        <div className="absolute bottom-0 left-0 right-0 h-24 bg-gradient-to-t from-black/90 to-transparent flex items-end justify-between px-6 pb-6 z-40 transition-opacity duration-300">
          <div className="flex items-center gap-4">
            <button className="w-10 h-10 rounded-full bg-white/10 hover:bg-white/20 flex items-center justify-center transition-colors text-white backdrop-blur-md">
              <Play size={18} className="fill-white ml-0.5" />
            </button>
            <div className="text-white/90 text-[13px] font-bold font-mono tracking-wide bg-black/40 px-3 py-1.5 rounded-full backdrop-blur-md border border-white/5">
              直播进行中 • 01:24:30
            </div>
          </div>
          <div className="flex items-center gap-3">
            <button className="w-10 h-10 rounded-full bg-black/40 hover:bg-white/10 backdrop-blur-md border border-white/10 flex items-center justify-center text-white transition-colors">
              <Settings size={18} />
            </button>
            <button className="w-10 h-10 rounded-full bg-black/40 hover:bg-white/10 backdrop-blur-md border border-white/10 flex items-center justify-center text-white transition-colors">
              <Maximize size={18} />
            </button>
          </div>
        </div>
      </div>

      {/* Right: Interaction Panel (Chat & Gifts) */}
      <div className="w-[380px] bg-[#121214] border-l border-white/5 flex flex-col shrink-0 relative z-50">
        
        {/* Chat Header */}
        <div className="h-14 border-b border-white/5 flex items-center justify-between px-6 bg-[#18181b]/50">
          <div className="flex gap-4">
            <button className="text-[15px] font-bold text-white border-b-2 border-indigo-500 h-14">实时互动</button>
            <button className="text-[15px] font-bold text-gray-500 hover:text-gray-300 transition-colors h-14">连麦大厅(2)</button>
          </div>
          <button className="text-gray-500 hover:text-white transition-colors"><MoreHorizontal size={18} /></button>
        </div>

        {/* Notice */}
        <div className="px-4 py-2 bg-indigo-500/10 border-b border-indigo-500/10">
          <p className="text-[12px] text-indigo-400 flex items-center gap-1.5 font-medium">
            <div className="w-4 h-4 rounded-full bg-indigo-400 text-black flex items-center justify-center text-[10px] font-bold shrink-0">公</div>
            欢迎来到直播间！请大家文明发言，积极讨论技术问题。
          </p>
        </div>

        {/* Chat Messages */}
        <div className="flex-1 overflow-y-auto custom-scrollbar p-4 flex flex-col gap-3">
            {messages.map((msg) => (
              <motion.div 
                initial={{ opacity: 0, x: -10 }}
                animate={{ opacity: 1, x: 0 }}
                key={msg.id} 
                className={`flex gap-2`}
              >
                <div className="w-7 h-7 rounded-full bg-white/10 shrink-0 mt-0.5 overflow-hidden">
                   <Avatar src={undefined} alt={msg.user} fallback={msg.user?.charAt(0) ?? "?"} size="sm" shape="circle" />
                </div>
                <div className={`p-2.5 rounded-2xl rounded-tl-sm text-[13px] leading-relaxed block overflow-hidden ${msg.user === currentUserName ? 'bg-indigo-500 text-white' : 'bg-white/5 text-gray-200'}`}>
                  {msg.user !== currentUserName && (
                    <div className="flex items-center gap-1.5 mb-1">
                      {msg.isPro && (
                        <span className="bg-gradient-to-r from-yellow-500 to-orange-500 text-black text-[9px] font-bold px-1.5 py-0 rounded-sm">粉丝</span>
                      )}
                      <span className="font-semibold text-gray-400 text-[11px]">{msg.user}</span>
                    </div>
                  )}
                  <div className="break-words font-medium">{msg.text}</div>
                </div>
              </motion.div>
            ))}
        </div>

        {/* Chat Input & Interactions */}
        <div className="p-4 bg-[#18181b] border-t border-white/5">
          {/* Quick Actions / Gifts */}
          <div className="flex gap-2 mb-3 overflow-x-auto custom-scrollbar pb-1">
            <button className="shrink-0 flex items-center gap-1.5 bg-yellow-500/10 text-yellow-500 border border-yellow-500/20 px-3 py-1.5 rounded-full text-xs font-bold hover:bg-yellow-500/20 transition-colors shadow-sm">
              <Gift size={14} /> 送礼物
            </button>
            {["666 🚀", "老师太棒了", "求解答", "听懂了"].map(quickMsg => (
              <button 
                key={quickMsg}
                onClick={() => setInputText(quickMsg)}
                className="shrink-0 bg-white/5 text-gray-300 hover:text-white border border-white/10 px-3 py-1.5 rounded-full text-xs font-medium hover:bg-white/10 transition-colors"
              >
                {quickMsg}
              </button>
            ))}
          </div>

          <div className="flex items-end gap-2 bg-[#09090b] rounded-2xl border border-white/10 p-1.5 shadow-inner">
            <textarea 
              value={inputText}
              onChange={(e) => setInputText(e.target.value)}
              onKeyDown={(e) => {
                if (e.key === 'Enter' && !e.shiftKey) {
                  e.preventDefault();
                  handleSend();
                }
              }}
              placeholder="发句友善的话与大家互动..."
              className="flex-1 bg-transparent text-[13px] text-white px-2 py-1.5 outline-none placeholder:text-gray-600 resize-none h-9 min-h-[36px] max-h-[100px] custom-scrollbar"
              rows={1}
            />
            <button 
              onClick={handleSend}
              className={`w-9 h-9 flex items-center justify-center rounded-xl shrink-0 transition-all ${
                inputText.trim() ? "bg-indigo-600 hover:bg-indigo-500 text-white shadow-lg shadow-indigo-500/20" : "bg-white/10 text-gray-500"
              }`}
            >
              <Zap size={14} className={inputText.trim() ? "fill-white" : ""} />
            </button>
          </div>

          <div className="mt-4 flex items-center justify-between px-2 relative">
            <div className="flex items-center gap-3">
              <button className="text-gray-400 hover:text-white transition-colors">
                <Share2 size={18} />
              </button>
              <div className="text-[10px] text-gray-500 font-medium tracking-wide flex items-center gap-1.5 ml-2 border-l border-white/10 pl-3">
                <span className="w-1.5 h-1.5 rounded-full bg-green-500"></span>
                连接极速
              </div>
            </div>
            
            {/* Like Button */}
            <button 
              onClick={handleLike}
              className="w-11 h-11 flex items-center justify-center rounded-full bg-gradient-to-tr from-pink-500 to-red-500 text-white hover:scale-105 active:scale-90 transition-all group relative z-50 shadow-lg shadow-red-500/20"
            >
              <Heart size={22} className="fill-white" />
            </button>

            {/* Floating Hearts Container */}
            <div className="absolute right-0 bottom-12 w-12 h-40 pointer-events-none overflow-visible">
              <AnimatePresence>
                {hearts.map(heart => (
                  <motion.div
                    key={heart.id}
                    initial={{ opacity: 0, y: 0, scale: 0.5, x: heart.x }}
                    animate={{ opacity: [0, 1, 0.5, 0], y: -200, scale: 1.5, x: heart.x + (Math.random() * 40 - 20) }}
                    exit={{ opacity: 0 }}
                    transition={{ duration: 2, ease: "easeOut" }}
                    className="absolute bottom-0 left-1/2 -translate-x-1/2 drop-shadow-lg"
                  >
                    <Heart size={28} className="fill-red-500 text-red-500" />
                  </motion.div>
                ))}
              </AnimatePresence>
            </div>
          </div>
        </div>
      </div>
    </motion.div>
  );
};


