import React, { useState } from "react";
import { X, Search, Video, Heart, MessageCircle, Send } from "lucide-react";
import { IconButton, Avatar, cn } from "@sdkwork/clawchat-mobile-commons";
import { motion } from "motion/react";

export const ChannelsPage = () => {
  const [activeTab, setActiveTab] = useState<"关注" | "朋友" | "推荐">("推荐");

  const DUMMY_VIDEOS = [
    {
      id: 1,
      title: "如何用 AI 提升 10 倍开发效率？",
      author: "AI 极客",
      avatar: "https://picsum.photos/seed/v1/100",
      likes: 12400,
      comments: 856,
      shares: 3200,
      bg: "bg-blue-900",
    },
    {
      id: 2,
      title: "周末露营 Vlog：远离城市喧嚣",
      author: "户外旅行",
      avatar: "https://picsum.photos/seed/v2/100",
      likes: 8900,
      comments: 231,
      shares: 1400,
      bg: "bg-emerald-900",
    },
    {
      id: 3,
      title: "3分钟学会做低脂健康餐",
      author: "健康生活",
      avatar: "https://picsum.photos/seed/v3/100",
      likes: 45000,
      comments: 3200,
      shares: 12000,
      bg: "bg-rose-900",
    },
  ];

  return (
    <div className="flex flex-col h-full bg-black text-white relative overflow-hidden">
      {/* Header */}
      <div className="absolute top-0 left-0 right-0 z-20 flex justify-between items-center px-4 pt-safe h-14 bg-gradient-to-b from-black/50 to-transparent">
        <IconButton
          icon={<X className="w-6 h-6 text-white" />}
          className="w-10 h-10 -ml-2"
          onClick={() => window.history.back()}
        />
        <div className="flex gap-6 text-[16px] font-medium opacity-90">
          {["关注", "朋友", "推荐"].map((tab) => (
            <div
              key={tab}
              className="relative cursor-pointer"
              onClick={() => setActiveTab(tab as any)}
            >
              <span
                className={cn(
                  activeTab === tab ? "text-white" : "text-white/60",
                )}
              >
                {tab}
              </span>
              {activeTab === tab && (
                <div className="absolute -bottom-1.5 left-0 right-0 flex justify-center">
                  <motion.div
                    layoutId="channelTab"
                    className="w-4 h-0.5 bg-white rounded-full"
                  />
                </div>
              )}
            </div>
          ))}
        </div>
        <IconButton
          icon={<Search className="w-6 h-6 text-white" />}
          className="w-10 h-10 -mr-2"
        />
      </div>

      {/* Video Feed */}
      <div className="flex-1 overflow-y-scroll snap-y snap-mandatory no-scrollbar relative w-full h-full">
        {DUMMY_VIDEOS.map((video) => (
          <div
            key={video.id}
            className={cn(
              "w-full h-full snap-start relative flex items-center justify-center",
              video.bg,
            )}
          >
            {/* Play indicator */}
            <div className="absolute inset-0 flex items-center justify-center pointer-events-none opacity-20">
              <Video className="w-24 h-24 text-white" />
            </div>

            {/* Sidebar actions */}
            <div className="absolute right-4 bottom-24 flex flex-col items-center gap-6 z-10">
              <div className="relative">
                <Avatar
                  src={video.avatar}
                  className="w-11 h-11 border border-white"
                />
                <div className="absolute -bottom-2 left-1/2 -translate-x-1/2 w-4 h-4 bg-red-500 rounded-full flex items-center justify-center text-white text-xs border border-black cursor-pointer">
                  +
                </div>
              </div>
              <div className="flex flex-col items-center gap-1 cursor-pointer active:scale-90 transition-transform">
                <Heart className="w-8 h-8 drop-shadow-md text-white" />
                <span className="text-xs font-medium drop-shadow-md">
                  {video.likes > 10000
                    ? (video.likes / 10000).toFixed(1) + "w"
                    : video.likes}
                </span>
              </div>
              <div className="flex flex-col items-center gap-1 cursor-pointer active:scale-90 transition-transform">
                <MessageCircle className="w-8 h-8 drop-shadow-md text-white" />
                <span className="text-xs font-medium drop-shadow-md">
                  {video.comments}
                </span>
              </div>
              <div className="flex flex-col items-center gap-1 cursor-pointer active:scale-90 transition-transform">
                <Send className="w-8 h-8 drop-shadow-md text-white" />
                <span className="text-xs font-medium drop-shadow-md">
                  {video.shares}
                </span>
              </div>
            </div>

            {/* Bottom info */}
            <div className="absolute left-4 bottom-16 right-20 z-10">
              <h3 className="font-bold text-base mb-1.5 drop-shadow-md">
                @{video.author}
              </h3>
              <p className="text-sm line-clamp-2 drop-shadow-md leading-relaxed">
                {video.title}
              </p>
            </div>
          </div>
        ))}
      </div>

      {/* Progress Bar (fake) */}
      <div className="absolute bottom-10 left-0 right-0 h-0.5 bg-white/20 z-20">
        <div className="h-full bg-white/60 w-1/3" />
      </div>
    </div>
  );
};
