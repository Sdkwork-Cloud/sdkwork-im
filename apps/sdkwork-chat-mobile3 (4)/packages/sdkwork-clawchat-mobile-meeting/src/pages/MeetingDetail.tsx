import React, { useEffect, useState } from "react";
import { PageLayout, showToast, cn } from "@sdkwork/clawchat-mobile-commons";
import { useNavigate, useParams } from "react-router";
import { MeetingService, MeetingRecord } from "../services/MeetingService";
import { Clock, MapPin, Users, Play } from "lucide-react";

export const MeetingDetail = () => {
  const { id } = useParams();
  const navigate = useNavigate();
  const [meeting, setMeeting] = useState<MeetingRecord | null>(null);

  useEffect(() => {
    if (id) {
      MeetingService.getMeetingDetail(id).then(setMeeting);
    }
  }, [id]);

  if (!meeting)
    return (
      <PageLayout title="会议简要">
        <div className="flex flex-col h-full bg-bg-color items-center justify-center text-text-sub opacity-70">
          <div className="w-8 h-8 rounded-full border-4 border-text-sub border-t-transparent animate-spin mb-3"></div>
          <span className="text-[14px]">加载中...</span>
        </div>
      </PageLayout>
    );

  return (
    <PageLayout title="会议简要">
      <div className="flex flex-col h-full bg-bg-color overflow-y-auto">
        <div className="bg-white dark:bg-[#1a1b1c] p-5 pb-8 mb-2">
          <div className="flex items-center gap-2 mb-2">
            <span
              className={cn(
                "text-[12px] px-2 py-1 rounded shrink-0",
                meeting.status === "upcoming" &&
                  "bg-blue-50 text-blue-600 dark:bg-blue-500/10",
                meeting.status === "ongoing" &&
                  "bg-green-50 text-green-600 dark:bg-green-500/10",
                meeting.status === "finished" &&
                  "bg-gray-100 text-gray-500 dark:bg-gray-800",
                meeting.status === "cancelled" &&
                  "bg-red-50 text-red-500 dark:bg-red-500/10",
              )}
            >
              {meeting.status === "upcoming" && "未开始"}
              {meeting.status === "ongoing" && "进行中"}
              {meeting.status === "finished" && "已结束"}
              {meeting.status === "cancelled" && "已取消"}
            </span>
          </div>
          <h1 className="text-[22px] font-medium text-text-main leading-tight mb-5">
            {meeting.title}
          </h1>

          <div className="flex flex-col gap-3">
            <div className="flex items-start gap-3 text-[14px]">
              <Clock className="w-5 h-5 text-gray-400 shrink-0" />
              <div>
                <div className="text-text-main">
                  {meeting.date} {meeting.time}
                </div>
                <div className="text-text-sub mt-0.5">
                  发起人: {meeting.organizerName || "Admin"}
                </div>
              </div>
            </div>

            <div className="flex items-start gap-3 text-[14px]">
              <MapPin className="w-5 h-5 text-gray-400 shrink-0" />
              <div className="text-text-main">{meeting.room}</div>
            </div>
          </div>
        </div>

        <div className="bg-white dark:bg-[#1a1b1c] p-4 mb-2">
          <h3 className="text-[15px] font-medium text-text-main mb-4">
            参会人员 ({meeting.attendees.length}人)
          </h3>
          <div className="flex gap-4 overflow-x-auto pb-2">
            {meeting.attendees.map((a) => (
              <div
                key={a.id}
                className="flex flex-col items-center gap-1 shrink-0"
              >
                <img
                  src={a.avatar}
                  className="w-12 h-12 rounded-full object-cover bg-gray-100"
                />
                <span className="text-[12px] text-text-sub truncate w-14 text-center">
                  {a.name}
                </span>
              </div>
            ))}
          </div>
        </div>

        {meeting.description && (
          <div className="bg-white dark:bg-[#1a1b1c] p-4 mb-2">
            <h3 className="text-[15px] font-medium text-text-main mb-3">
              会议议程
            </h3>
            <div className="text-[14px] text-text-main leading-relaxed whitespace-pre-wrap">
              {meeting.description}
            </div>
          </div>
        )}

        {meeting.status !== "finished" && meeting.status !== "cancelled" && (
          <div className="p-6 mt-4">
            <button
              className="w-full bg-primary-blue text-white rounded-lg py-3 font-medium active:bg-primary-blue/90 flex items-center justify-center gap-2"
              onClick={() => navigate(`/call/video/${meeting.id}`)}
            >
              <Play className="w-5 h-5" />
              加入会议
            </button>
          </div>
        )}
      </div>
    </PageLayout>
  );
};
