import React, { useState, useEffect } from "react";
import {
  PageLayout,
  IconButton,
  cn,
  showToast,
} from "@sdkwork/clawchat-mobile-commons";
import {
  Plus,
  Search,
  MapPin,
  Clock,
  Video,
  Users,
  Play,
  StopCircle,
} from "lucide-react";
import { MeetingService, MeetingRecord } from "../services/MeetingService";
import { motion } from "motion/react";
import { useNavigate } from "react-router";

export const MeetingApp = () => {
  const navigate = useNavigate();
  const [activeTab, setActiveTab] = useState<"upcoming" | "finished">(
    "upcoming",
  );
  const [meetings, setMeetings] = useState<MeetingRecord[]>([]);
  const [isLoading, setIsLoading] = useState(true);

  useEffect(() => {
    setIsLoading(true);
    MeetingService.getMeetings().then((data) => {
      setMeetings(data);
      setIsLoading(false);
    });
  }, []);

  const filteredMeetings = meetings.filter((m) =>
    activeTab === "upcoming"
      ? m.status === "upcoming" || m.status === "ongoing"
      : m.status === "finished",
  );

  return (
    <PageLayout title="会议预定">
      <div className="flex flex-col h-full bg-[#f5f6f8] dark:bg-[#1a1b1c]">
        {/* Header Stats */}
        <div className="bg-primary-blue px-6 pt-4 pb-12 flex justify-between items-center text-white">
          <div>
            <div className="text-[32px] font-medium tracking-tight leading-none mb-1">
              {
                meetings.filter(
                  (m) => m.status === "upcoming" || m.status === "ongoing",
                ).length
              }
            </div>
            <div className="text-[13px] opacity-80">今日即将开始 (场)</div>
          </div>
          <div className="w-12 h-12 bg-white/20 rounded-full flex items-center justify-center">
            <Video className="w-6 h-6 text-white" />
          </div>
        </div>

        <div className="flex-1 overflow-y-auto px-4 -mt-6">
          <div className="flex bg-white dark:bg-[#2c2d2e] rounded-xl shadow-sm mb-4 px-2 py-1">
            {["upcoming", "finished"].map((tab) => (
              <button
                key={tab}
                className={cn(
                  "flex-1 text-[15px] py-2.5 relative text-center transition-colors rounded-lg",
                  activeTab === tab
                    ? "text-primary-blue font-medium bg-primary-blue/5"
                    : "text-text-sub",
                )}
                onClick={() => setActiveTab(tab as any)}
              >
                {tab === "upcoming" ? "待参加" : "已结束"}
              </button>
            ))}
          </div>

          <div className="flex flex-col gap-3 pb-20">
            {isLoading ? (
              <div className="flex flex-col items-center justify-center py-20 text-text-sub opacity-70">
                <div className="w-8 h-8 rounded-full border-4 border-text-sub border-t-white animate-spin mb-3"></div>
                <span className="text-[14px]">加载中...</span>
              </div>
            ) : filteredMeetings.length > 0 ? (
              filteredMeetings.map((meeting) => (
              <motion.div
                key={meeting.id}
                whileTap={{ scale: 0.98 }}
                onClick={() => navigate(`/workspace/meeting/${meeting.id}`)}
                className="bg-white dark:bg-[#2c2d2e] p-4 rounded-xl cursor-pointer shadow-sm border border-border-color/30"
              >
                <div className="flex justify-between items-start mb-3">
                  <div>
                    <h3 className="text-[17px] font-medium text-text-main mb-1.5">
                      {meeting.title}
                    </h3>
                    <div className="flex items-center gap-2 text-[13px] text-text-sub font-mono bg-gray-50 dark:bg-[#202122] px-2 py-1 rounded inline-flex">
                      <Clock className="w-3.5 h-3.5" />
                      {meeting.date} {meeting.time}
                    </div>
                  </div>
                  <div
                    className={cn(
                      "text-[12px] px-2 py-1 rounded shrink-0",
                      meeting.status === "upcoming" &&
                        "bg-blue-50 text-blue-600 dark:bg-blue-500/10",
                      meeting.status === "ongoing" &&
                        "bg-green-50 text-green-600 dark:bg-green-500/10",
                      meeting.status === "finished" &&
                        "bg-gray-100 text-gray-500 dark:bg-gray-800",
                    )}
                  >
                    {meeting.status === "upcoming"
                      ? "即将开始"
                      : meeting.status === "ongoing"
                        ? "进行中"
                        : "已结束"}
                  </div>
                </div>

                <div className="flex items-center gap-1.5 text-[13px] text-text-sub mb-3">
                  <MapPin className="w-4 h-4 shrink-0" />
                  <span className="truncate">{meeting.room}</span>
                </div>

                <div className="flex justify-between items-center pt-3 border-t border-border-color">
                  <div className="flex items-center gap-2 text-[13px] text-text-sub">
                    <Users className="w-4 h-4" />
                    <span>{meeting.attendees.length} 人参与</span>
                  </div>
                  {meeting.status === "upcoming" && (
                    <button
                      onClick={(e) => {
                        e.stopPropagation();
                        navigate(`/call/video/${meeting.id}`);
                      }}
                      className="bg-primary-blue text-white px-4 py-1.5 rounded-full text-[13px] font-medium active:scale-95 transition-transform flex items-center gap-1.5"
                    >
                      <Play className="w-3.5 h-3.5" /> 加入会议
                    </button>
                  )}
                </div>
              </motion.div>
            ))
            ) : (
              <div className="flex flex-col items-center py-20 text-text-sub opacity-70">
                <Video className="w-12 h-12 mb-3 stroke-current opacity-40" />
                <span className="text-[14px]">暂无相关会议记录</span>
              </div>
            )}
          </div>
        </div>

        <motion.button
          whileTap={{ scale: 0.9 }}
          whileHover={{ scale: 1.05 }}
          onClick={() => navigate("/workspace/meeting/create")}
          className="absolute bottom-6 right-6 w-14 h-14 bg-gradient-to-tr from-blue-600 to-primary-blue text-white rounded-full flex items-center justify-center shadow-lg shadow-blue-500/30 z-10"
        >
          <Plus className="w-7 h-7" />
        </motion.button>
      </div>
    </PageLayout>
  );
};
