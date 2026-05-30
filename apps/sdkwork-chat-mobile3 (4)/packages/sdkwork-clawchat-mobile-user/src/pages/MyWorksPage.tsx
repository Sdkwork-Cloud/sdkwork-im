import React, { useState, useEffect } from "react";
import { useNavigate } from "react-router";
import {
  Play,
  FileText,
  Headphones,
  Image as ImageIcon,
  Heart,
  Eye,
  MessageSquare,
  MoreHorizontal,
  Trash2,
  ChevronLeft,
  FolderOpen,
} from "lucide-react";
import { WorkService, Work } from "../services/WorkService";
import { cn, showToast } from "@sdkwork/clawchat-mobile-commons";

export const MyWorksPage = () => {
  const navigate = useNavigate();
  const [works, setWorks] = useState<Work[]>([]);
  const [loading, setLoading] = useState(true);
  const [activeTab, setActiveTab] = useState<
    "all" | "video" | "article" | "audio" | "ai_image"
  >("all");
  const [deleteData, setDeleteData] = useState<{
    show: boolean;
    workId: string | null;
  }>({ show: false, workId: null });

  useEffect(() => {
    loadWorks();
  }, []);

  const loadWorks = async () => {
    setLoading(true);
    try {
      const data = await WorkService.getMyWorks();
      setWorks(data);
    } catch (error) {
      showToast("加载失败");
    } finally {
      setLoading(false);
    }
  };

  const handleDelete = async () => {
    if (!deleteData.workId) return;
    try {
      await WorkService.deleteWork(deleteData.workId);
      setWorks(works.filter((w) => w.id !== deleteData.workId));
      setDeleteData({ show: false, workId: null });
    } catch (error) {
      showToast("删除失败");
    }
  };

  const getWorkIcon = (type: Work["type"]) => {
    switch (type) {
      case "video":
        return <Play className="w-3.5 h-3.5 text-white fill-current" />;
      case "article":
        return <FileText className="w-3.5 h-3.5 text-white" />;
      case "audio":
        return <Headphones className="w-3.5 h-3.5 text-white" />;
      case "ai_image":
        return <ImageIcon className="w-3.5 h-3.5 text-white" />;
    }
  };

  const formatNumber = (num: number) => {
    if (num >= 10000) return (num / 10000).toFixed(1) + "w";
    return num.toString();
  };

  const filteredWorks = works.filter(
    (w) => activeTab === "all" || w.type === activeTab,
  );

  const tabs = [
    { id: "all", label: "全部" },
    { id: "video", label: "视频" },
    { id: "article", label: "图文" },
    { id: "audio", label: "音频" },
    { id: "ai_image", label: "AI作画" },
  ];

  return (
    <div className="flex flex-col h-full bg-bg-color">
      <header className="flex items-center justify-between px-2 pt-safe h-[56px] border-b border-border-color bg-chat-other-bg shrink-0">
        <div
          className="w-10 h-10 flex items-center justify-center cursor-pointer"
          onClick={() => navigate(-1)}
        >
          <ChevronLeft className="w-6 h-6 text-text-main" />
        </div>
        <span className="text-[17px] font-medium text-text-main">我的作品</span>
        <div className="w-10 h-10" />
      </header>

      {/* Tabs */}
      <div className="flex items-center px-4 h-[44px] border-b border-border-color bg-bg-color shrink-0 sticky top-0 z-10 gap-6 overflow-x-auto no-scrollbar">
        {tabs.map((tab) => (
          <div
            key={tab.id}
            className={cn(
              "h-full flex items-center relative whitespace-nowrap cursor-pointer transition-colors",
              activeTab === tab.id
                ? "text-text-main font-medium"
                : "text-text-sub",
            )}
            onClick={() => setActiveTab(tab.id as any)}
          >
            <span className="text-[15px]">{tab.label}</span>
            {activeTab === tab.id && (
              <div className="absolute left-0 right-0 bottom-0 flex justify-center">
                <div className="w-4 h-0.5 bg-primary-blue rounded-full" />
              </div>
            )}
          </div>
        ))}
      </div>

      <div className="flex-1 overflow-y-auto bg-chat-other-bg pb-12">
        {loading ? (
          <div className="flex flex-col items-center justify-center py-20 text-text-sub opacity-70">
            <div className="w-8 h-8 rounded-full border-4 border-text-sub border-t-transparent animate-spin mb-3"></div>
            <span className="text-[14px]">加载中...</span>
          </div>
        ) : filteredWorks.length > 0 ? (
          <div className="grid grid-cols-2 gap-[2px]">
            {filteredWorks.map((work) => (
              <div
                key={work.id}
                className="bg-bg-color overflow-hidden flex flex-col active:opacity-80 transition-opacity cursor-pointer"
              >
                {/* Cover Area */}
                <div
                  className="w-full aspect-[3/4] relative bg-cover bg-center"
                  style={{ backgroundImage: `url(${work.coverUrl})` }}
                >
                  {/* Overlay Gradient for Type Icon */}
                  <div className="absolute inset-0 bg-gradient-to-b from-black/20 via-transparent to-black/60 pointer-events-none" />

                  {/* Type Icon Badge */}
                  <div className="absolute top-2 right-2 w-6 h-6 rounded-full bg-black/40 backdrop-blur-md flex items-center justify-center">
                    {getWorkIcon(work.type)}
                  </div>

                  {/* Stats overlay */}
                  <div className="absolute bottom-2 left-2 right-2 flex items-center gap-2 text-white/90">
                    <div className="flex items-center gap-1">
                      <Eye className="w-3 h-3" />
                      <span className="text-[11px] font-medium">
                        {formatNumber(work.views)}
                      </span>
                    </div>
                  </div>
                </div>

                {/* Content Area */}
                <div className="p-2.5 flex flex-col flex-1">
                  <span className="text-[13px] font-medium text-text-main line-clamp-2 leading-snug mb-2 flex-1">
                    {work.title}
                  </span>

                  <div className="flex items-center justify-between mt-auto pt-2 border-t border-border-color/30">
                    <div className="flex items-center gap-3">
                      <div className="flex items-center gap-1 text-text-sub">
                        <Heart className="w-3.5 h-3.5" />
                        <span className="text-[11px]">
                          {formatNumber(work.likes)}
                        </span>
                      </div>
                      <div className="flex items-center gap-1 text-text-sub">
                        <MessageSquare className="w-3.5 h-3.5" />
                        <span className="text-[11px]">
                          {formatNumber(work.comments)}
                        </span>
                      </div>
                    </div>

                    <div
                      className="p-1 -mr-1 rounded-full active:bg-chat-active-bg transition-colors"
                      onClick={(e) => {
                        e.stopPropagation();
                        setDeleteData({ show: true, workId: work.id });
                      }}
                    >
                      <MoreHorizontal className="w-4 h-4 text-text-sub" />
                    </div>
                  </div>
                </div>
              </div>
            ))}
          </div>
        ) : (
          <div className="flex flex-col items-center justify-center py-20 text-text-sub opacity-70">
            <FolderOpen className="w-12 h-12 mb-3 stroke-current opacity-40" />
            <p className="text-[14px]">
              暂无
              {activeTab !== "all" &&
                tabs.find((t) => t.id === activeTab)?.label}
              作品
            </p>
          </div>
        )}
      </div>

      {/* Action Sheet - Delete work */}
      {deleteData.show && (
        <div className="absolute inset-0 z-50 flex flex-col justify-end">
          <div
            className="absolute inset-0 bg-black/40 backdrop-blur-sm"
            onClick={() => setDeleteData({ show: false, workId: null })}
          />
          <div className="relative bg-chat-other-bg w-full pb-safe rounded-t-2xl animate-in slide-in-from-bottom duration-300">
            <div className="flex flex-col items-center p-4 py-5 border-b border-border-color">
              <span className="text-[14px] text-text-sub mb-1">管理作品</span>
            </div>

            <div
              className="flex items-center justify-center p-4 bg-bg-color active:bg-chat-active-bg transition-colors cursor-pointer border-b border-border-color"
              onClick={handleDelete}
            >
              <span className="text-[16px] text-[#FA5151]">删除此作品</span>
            </div>

            <div className="h-2 bg-chat-other-bg" />

            <div
              className="flex items-center justify-center p-4 bg-bg-color active:bg-chat-active-bg transition-colors cursor-pointer"
              onClick={() => setDeleteData({ show: false, workId: null })}
            >
              <span className="text-[16px] text-text-main">取消</span>
            </div>
          </div>
        </div>
      )}
    </div>
  );
};
