import React, { useState, useEffect } from "react";
import {
  PageLayout,
  IconButton,
  cn,
  showToast,
  ActionSheet,
} from "@sdkwork/clawchat-mobile-commons";
import {
  Search,
  Mic,
  Play,
  Pause,
  FileAudio,
  FileText,
  ChevronRight,
  Hash,
  Square,
} from "lucide-react";
import {
  VoiceSummaryService,
  VoiceSummaryRecord,
} from "../services/VoiceSummaryService";
import { motion, AnimatePresence } from "motion/react";

export const VoiceSummaryApp = () => {
  const [summaries, setSummaries] = useState<VoiceSummaryRecord[]>([]);
  const [playingId, setPlayingId] = useState<string | null>(null);
  const [isRecording, setIsRecording] = useState(false);
  const [showSearch, setShowSearch] = useState(false);
  const [searchWord, setSearchWord] = useState("");

  useEffect(() => {
    VoiceSummaryService.getSummaries().then(setSummaries);
  }, []);

  const handlePlayToggle = (e: React.MouseEvent, id: string) => {
    e.stopPropagation();
    if (playingId === id) setPlayingId(null);
    else setPlayingId(id);
  };

  const filteredSummaries = summaries.filter(
    (s) =>
      s.title.includes(searchWord) ||
      s.summary.includes(searchWord) ||
      s.keywords.some((k) => k.includes(searchWord)),
  );

  const handleRecordToggle = () => {
    if (isRecording) {
      setIsRecording(false);
      showToast("录音已保存并开始AI摘要分析");
      // mock new record
      setTimeout(() => {
        setSummaries((prev) => [
          {
            id: Math.random().toString(),
            title: "新录音_" + new Date().toLocaleTimeString(),
            date: "刚刚",
            duration: "00:05",
            summary:
              "用户录制了一段简短的新录音，这通常用于口述笔记备忘。录音内容似乎包含了几个核心待办事项或者是简短的心得体会。AI已成功为您转录并提炼了核心意图。",
            keywords: ["语音备忘", "待办", "日常"],
          },
          ...prev,
        ]);
        showToast("AI摘要分析完成");
      }, 1500);
    } else {
      setIsRecording(true);
      showToast("开始录音...");
    }
  };

  return (
    <PageLayout title="语音摘要">
      <div className="flex flex-col h-full bg-[#f5f6f8] dark:bg-[#1a1b1c]">
        {/* Header Stats */}
        <div className="bg-primary-blue px-6 pt-4 pb-12 flex justify-between items-center text-white relative overflow-hidden">
          <div className="absolute top-0 right-0 p-4 opacity-10 blur-xl">
            <Mic className="w-32 h-32" />
          </div>
          <div className="relative z-10">
            <div className="text-[32px] font-medium tracking-tight leading-none mb-1">
              {summaries.length}
            </div>
            <div className="text-[13px] opacity-80">已生成摘要 (条)</div>
          </div>
          <div className="flex gap-4 relative z-10">
            <div className="flex flex-col items-center">
              <div className="text-[20px] font-medium leading-none mb-1">
                10h
              </div>
              <div className="text-[12px] opacity-80">处理时长</div>
            </div>
          </div>
        </div>

        <div className="flex-1 overflow-y-auto px-4 -mt-6">
          <div className="flex justify-between items-center mb-3 mt-4 px-1">
            <h2 className="text-[14px] font-medium text-text-sub">
              全部录音 ({filteredSummaries.length})
            </h2>
            <div className="flex gap-2 items-center">
              {showSearch && (
                <input
                  type="text"
                  value={searchWord}
                  onChange={(e) => setSearchWord(e.target.value)}
                  placeholder="搜索录音..."
                  className="bg-white dark:bg-[#2c2d2e] px-3 py-1 text-[13px] rounded-md outline-none text-text-main shadow-sm w-32"
                />
              )}
              <IconButton
                icon={<Search className="w-4 h-4 text-text-sub" />}
                className="bg-white dark:bg-[#2c2d2e] p-1.5 w-auto h-auto rounded-md shadow-sm"
                onClick={() => setShowSearch(!showSearch)}
              />
            </div>
          </div>

          <div className="flex flex-col gap-3 pb-20">
            {filteredSummaries.length > 0 ? (
              filteredSummaries.map((summary) => (
                <motion.div
                  key={summary.id}
                  whileTap={{ scale: 0.98 }}
                  className="bg-white dark:bg-[#2c2d2e] p-4 rounded-xl cursor-default shadow-sm border border-border-color/30"
                >
                  <div className="flex justify-between items-start mb-3">
                    <div className="flex items-center gap-3">
                      <div className="w-10 h-10 rounded-xl bg-indigo-50 dark:bg-indigo-500/10 flex items-center justify-center relative overflow-hidden">
                        <FileAudio className="w-5 h-5 text-indigo-500 relative z-10" />
                        {playingId === summary.id && (
                          <div className="absolute bottom-0 left-0 right-0 h-1 bg-indigo-500 opacity-50 animate-pulse" />
                        )}
                      </div>
                      <div>
                        <div className="text-[16px] font-medium text-text-main leading-tight mb-1">
                          {summary.title}
                        </div>
                        <div className="text-[13px] text-text-sub flex items-center gap-2">
                          <span>{summary.date}</span>
                          <span className="w-1 h-1 bg-border-color rounded-full" />
                          <span>{summary.duration}</span>
                        </div>
                      </div>
                    </div>
                    <IconButton
                      icon={
                        playingId === summary.id ? (
                          <Pause className="w-5 h-5 text-text-sub" />
                        ) : (
                          <Play className="w-5 h-5 text-text-sub" />
                        )
                      }
                      className="w-8 h-8 -mr-2 bg-gray-50 dark:bg-[#3a3b3c]"
                      onClick={(e) => handlePlayToggle(e, summary.id)}
                    />
                  </div>

                  <div className="text-[14px] text-text-main bg-blue-50/50 dark:bg-blue-900/10 p-3 rounded-lg flex flex-col gap-2 border border-blue-100 dark:border-blue-800/30">
                    <div className="flex items-center gap-1 text-primary-blue font-medium mb-1 border-b border-blue-100 dark:border-blue-800/30 pb-2">
                      <FileText className="w-4 h-4" /> AI 核心摘要
                    </div>
                    <p className="text-[13px] leading-relaxed text-text-main">
                      {summary.summary}
                    </p>

                    <div className="flex flex-wrap gap-2 mt-1">
                      {summary.keywords.map((kw) => (
                        <span
                          key={kw}
                          className="text-[11px] bg-white dark:bg-[#202122] text-text-sub px-2 py-0.5 rounded border border-border-color/50 flex items-center gap-0.5"
                        >
                          <Hash className="w-3 h-3 text-primary-blue/70" /> {kw}
                        </span>
                      ))}
                    </div>
                  </div>
                </motion.div>
              ))
            ) : (
              <div className="flex flex-col items-center justify-center py-20 text-text-sub opacity-70">
                <FileAudio className="w-12 h-12 mb-3 stroke-current opacity-40" />
                <span className="text-[14px]">暂无AI语音摘要</span>
              </div>
            )}
          </div>
        </div>

        <motion.button
          whileTap={{ scale: 0.9 }}
          whileHover={{ scale: 1.05 }}
          onClick={handleRecordToggle}
          className={cn(
            "absolute bottom-6 right-6 w-14 h-14 text-white rounded-full flex items-center justify-center shadow-lg z-10 transition-colors",
            isRecording
              ? "bg-rose-500 shadow-rose-500/30 animate-pulse"
              : "bg-gradient-to-tr from-emerald-500 to-emerald-400 shadow-emerald-500/30",
          )}
        >
          {isRecording ? (
            <Square className="w-5 h-5 fill-current" />
          ) : (
            <Mic className="w-6 h-6" />
          )}
        </motion.button>

        <AnimatePresence>
          {isRecording && (
            <motion.div
              initial={{ opacity: 0, y: 20 }}
              animate={{ opacity: 1, y: 0 }}
              exit={{ opacity: 0, y: 20 }}
              className="absolute bottom-24 right-4 bg-gray-900/90 text-white px-4 py-2 rounded-full text-[13px] shadow-lg flex items-center gap-2 pointer-events-none"
            >
              <span className="w-2 h-2 rounded-full bg-rose-500 animate-pulse" />
              正在录音中...
            </motion.div>
          )}
        </AnimatePresence>
      </div>
    </PageLayout>
  );
};
