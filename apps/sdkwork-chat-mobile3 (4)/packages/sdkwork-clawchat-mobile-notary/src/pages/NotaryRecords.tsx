import React, { useState, useRef, useEffect, useCallback } from "react";
import {
  FileText,
  ArrowRight,
  Clock,
  CheckCircle2,
  XCircle,
  Loader2,
} from "lucide-react";
import { useNavigate } from "react-router";
import { IconButton, cn } from "@sdkwork/clawchat-mobile-commons";
import { notaryService } from "../services/notaryService";
import { motion } from "motion/react";

const STATUS_MAP: Record<string, any> = {
  processing: {
    label: "处理中",
    icon: Clock,
    color: "text-orange-500",
    bg: "bg-orange-500/10",
  },
  completed: {
    label: "已完成",
    icon: CheckCircle2,
    color: "text-green-500",
    bg: "bg-green-500/10",
  },
  cancelled: {
    label: "已取消",
    icon: XCircle,
    color: "text-gray-500",
    bg: "bg-gray-500/10",
  },
};

export const NotaryRecords: React.FC = () => {
  const navigate = useNavigate();
  const [activeTab, setActiveTab] = useState<string>("all");
  const [items, setItems] = useState<any[]>([]);
  const [isLoadingMore, setIsLoadingMore] = useState(false);
  const [loading, setLoading] = useState(true);
  const [page, setPage] = useState(1);
  const [hasMore, setHasMore] = useState(true);

  const [tabs, setTabs] = useState<{ id: string; label: string }[]>([]);

  const loadMoreRef = useRef<HTMLDivElement>(null);

  const touchStartX = useRef<number | null>(null);
  const touchStartY = useRef<number | null>(null);

  useEffect(() => {
    notaryService.getRecordTabs().then((data) => setTabs(data as any[]));
  }, []);

  useEffect(() => {
    let mounted = true;
    setLoading(true);
    setPage(1);
    setItems([]);
    setHasMore(true);

    notaryService.getNotaryRecords(activeTab, 1).then((data: any) => {
      if (mounted) {
        setItems(data.records || []);
        setHasMore(data.hasMore);
        setLoading(false);
      }
    });
    return () => {
      mounted = false;
    };
  }, [activeTab]);

  const handleTouchStart = (e: React.TouchEvent) => {
    touchStartX.current = e.touches[0].clientX;
    touchStartY.current = e.touches[0].clientY;
  };

  const handleTouchEnd = (e: React.TouchEvent) => {
    if (touchStartX.current === null || touchStartY.current === null) return;
    const touchEndX = e.changedTouches[0].clientX;
    const touchEndY = e.changedTouches[0].clientY;

    const diffX = touchStartX.current - touchEndX;
    const diffY = touchStartY.current - touchEndY;

    if (Math.abs(diffX) > Math.abs(diffY) && Math.abs(diffX) > 50) {
      const currentIndex = tabs.findIndex((t) => t.id === activeTab);
      if (diffX > 0) {
        if (currentIndex < tabs.length - 1)
          setActiveTab(tabs[currentIndex + 1].id);
      } else {
        if (currentIndex > 0) setActiveTab(tabs[currentIndex - 1].id);
      }
    }
    touchStartX.current = null;
    touchStartY.current = null;
  };

  const loadMoreData = useCallback(() => {
    if (isLoadingMore || !hasMore || loading) return;
    setIsLoadingMore(true);
    const nextPage = page + 1;
    notaryService.getNotaryRecords(activeTab, nextPage).then((data: any) => {
      setItems((prev) => [...prev, ...(data.records || [])]);
      setHasMore(data.hasMore);
      setPage(nextPage);
      setIsLoadingMore(false);
    });
  }, [isLoadingMore, hasMore, loading, activeTab, page]);

  useEffect(() => {
    const observer = new IntersectionObserver(
      (entries) => {
        const target = entries[0];
        if (target.isIntersecting) {
          loadMoreData();
        }
      },
      { threshold: 0.1 },
    );

    if (loadMoreRef.current) observer.observe(loadMoreRef.current);
    return () => observer.disconnect();
  }, [loadMoreData]);

  const filteredRecords = items; // Backend mock already filters

  return (
    <div className="flex flex-col h-full bg-bg-color relative">
      <header className="h-[56px] flex items-center justify-between px-1 glass-header shrink-0 pt-safe z-20">
        <div className="flex items-center z-10 flex-1"></div>
        <div className="absolute left-1/2 -translate-x-1/2 flex flex-col items-center pointer-events-none">
          <h1 className="text-[17px] font-medium text-text-main">公证记录</h1>
        </div>
        <div className="flex-1" />
      </header>

      <div className="flex-1 overflow-y-auto relative z-10 pb-[90px]">
        {/* Statistics */}
        <div className="p-4 flex flex-col gap-3">
          <div className="grid grid-cols-2 gap-3">
            <div className="bg-gradient-to-br from-primary-blue/10 to-indigo-500/10 border border-primary-blue/20 rounded-2xl p-4 flex flex-col justify-between h-[104px]">
              <div className="flex items-center gap-2">
                <div className="w-1.5 h-4 bg-primary-blue rounded-full" />
                <span className="text-[14px] text-primary-blue font-bold">
                  累计公证数
                </span>
              </div>
              <span className="text-[32px] font-bold text-primary-blue font-mono tracking-tight">
                128
              </span>
            </div>
            <div className="bg-chat-other-bg border border-border-color rounded-2xl p-4 flex flex-col justify-between h-[104px]">
              <div className="flex items-center gap-2">
                <div className="w-1.5 h-4 bg-orange-500 rounded-full" />
                <span className="text-[14px] text-text-main font-bold">
                  处理中队列
                </span>
              </div>
              <span className="text-[32px] font-bold text-text-main font-mono tracking-tight">
                3
              </span>
            </div>
          </div>

          <div className="grid grid-cols-3 gap-3">
            <div className="bg-chat-other-bg border border-border-color/50 rounded-xl p-3.5 flex flex-col justify-between h-[84px]">
              <span className="text-[12px] text-text-sub font-medium">
                今日新增
              </span>
              <div className="flex items-end gap-1">
                <span className="text-[22px] font-bold text-text-main font-mono leading-none">
                  2
                </span>
                <span className="text-[10px] text-green-500 font-bold mb-0.5">
                  +2
                </span>
              </div>
            </div>
            <div className="bg-chat-other-bg border border-border-color/50 rounded-xl p-3.5 flex flex-col justify-between h-[84px]">
              <span className="text-[12px] text-text-sub font-medium">
                本周新增
              </span>
              <div className="flex items-end gap-1">
                <span className="text-[22px] font-bold text-text-main font-mono leading-none">
                  15
                </span>
                <span className="text-[10px] text-green-500 font-bold mb-0.5">
                  +5
                </span>
              </div>
            </div>
            <div className="bg-chat-other-bg border border-border-color/50 rounded-xl p-3.5 flex flex-col justify-between h-[84px]">
              <span className="text-[12px] text-text-sub font-medium">
                本月累计
              </span>
              <div className="flex items-end gap-1">
                <span className="text-[22px] font-bold text-text-main font-mono leading-none">
                  48
                </span>
                <span className="text-[10px] text-green-500 font-bold mb-0.5">
                  +12
                </span>
              </div>
            </div>
          </div>
        </div>

        {/* Tabs */}
        <div className="px-4 border-b border-border-color flex gap-6 sticky top-0 glass-header z-10 transition-colors">
          {tabs.map((tab) => (
            <div
              key={tab.id}
              onClick={() => setActiveTab(tab.id)}
              className={cn(
                "py-3 text-[15px] font-medium transition-colors relative cursor-pointer",
                activeTab === tab.id ? "text-primary-blue" : "text-text-sub",
              )}
            >
              {tab.label}
              {activeTab === tab.id && (
                <div className="absolute left-0 right-0 bottom-0 flex justify-center">
                  <motion.div
                    layoutId="notaryRecordsTab"
                    className="w-4 h-0.5 bg-primary-blue rounded-full"
                  />
                </div>
              )}
            </div>
          ))}
        </div>

        {/* List */}
        <div
          className="flex flex-col min-h-[50vh]"
          onTouchStart={handleTouchStart}
          onTouchEnd={handleTouchEnd}
        >
          {loading && (
            <div className="p-4 text-center text-text-sub">
              <Loader2 className="w-5 h-5 mx-auto animate-spin" />
            </div>
          )}
          {!loading &&
            filteredRecords.map((record, idx) => {
              const statusInfo =
                STATUS_MAP[record.status] || STATUS_MAP["processing"];
              const Icon = statusInfo.icon;
              return (
                <div
                  key={record.id}
                  onClick={() => navigate(`/notary/detail/${record.id}`)}
                  className={cn(
                    "px-4 py-3.5 flex items-center gap-4 active:bg-active-bg transition-colors cursor-pointer",
                    idx !== filteredRecords.length - 1
                      ? "border-b border-border-color/50"
                      : "",
                  )}
                >
                  <div
                    className={cn(
                      "w-12 h-12 rounded-xl flex items-center justify-center shrink-0",
                      statusInfo.bg,
                    )}
                  >
                    <Icon className={cn("w-6 h-6", statusInfo.color)} />
                  </div>
                  <div className="flex-1 min-w-0">
                    <h3 className="text-[16px] font-bold text-text-main truncate">
                      {record.title}
                    </h3>
                    <p className="text-[13px] text-text-sub mt-1">
                      {record.date}
                    </p>
                  </div>
                  <div className="flex items-center gap-1">
                    <span
                      className={cn(
                        "text-[13px] font-medium",
                        statusInfo.color,
                      )}
                    >
                      {statusInfo.label}
                    </span>
                    <ArrowRight className="w-4 h-4 text-text-sub opacity-50" />
                  </div>
                </div>
              );
            })}

          {/* Infinite Scroll Trigger */}
          <div
            ref={loadMoreRef}
            className="h-16 flex items-center justify-center"
          >
            {isLoadingMore ? (
              <div className="flex items-center gap-2 text-text-sub">
                <Loader2 className="w-4 h-4 animate-spin" />
                <span className="text-[13px]">正在加载更多...</span>
              </div>
            ) : hasMore ? (
              <span className="text-[12px] text-text-sub opacity-50">
                上滑加载更多
              </span>
            ) : items.length > 0 ? (
              <span className="text-[12px] text-text-sub opacity-50 relative z-0">
                — 到底了 —
              </span>
            ) : (
              <div className="flex flex-col items-center justify-center pt-20 pb-10 text-text-sub opacity-70 relative z-0">
                <FileText className="w-12 h-12 mb-3 stroke-current opacity-40" />
                <span className="text-[14px]">暂无公证记录</span>
              </div>
            )}
          </div>
        </div>
      </div>
    </div>
  );
};
