import React, { useState, useEffect } from "react";
import { PageLayout, IconButton, cn } from "@sdkwork/clawchat-mobile-commons";
import { MapPin, Clock, Calendar, CheckCircle2 } from "lucide-react";
import {
  AttendanceService,
  AttendanceRecord,
} from "../services/AttendanceService";
import { motion } from "motion/react";

export const AttendanceApp = () => {
  const [records, setRecords] = useState<AttendanceRecord[]>([]);
  const [time, setTime] = useState(new Date());

  useEffect(() => {
    AttendanceService.getRecords().then(setRecords);

    const timer = setInterval(() => {
      setTime(new Date());
    }, 1000);
    return () => clearInterval(timer);
  }, []);

  const handleClockIn = async () => {
    await AttendanceService.clockIn();
    const latest = await AttendanceService.getRecords();
    setRecords(latest);
  };

  const todayRecords = records.filter(
    (r) => r.date === new Date().toISOString().split("T")[0],
  );
  const hasPunchedIn = todayRecords.some((r) => r.type === "in");
  const hasPunchedOut = todayRecords.some((r) => r.type === "out");
  const isDoneToday = hasPunchedIn && hasPunchedOut;

  return (
    <PageLayout title="打卡">
      <div className="flex flex-col h-full bg-bg-color">
        {/* Header summary */}
        <div className="bg-primary-blue text-white p-6 pb-8 rounded-b-[2rem] shadow-sm">
          <div className="flex justify-between items-center mb-6">
            <div className="flex items-center gap-2">
              <Calendar className="w-5 h-5 opacity-90" />
              <span className="font-medium">
                {time.toLocaleDateString("zh-CN", {
                  month: "long",
                  day: "numeric",
                  weekday: "long",
                })}
              </span>
            </div>
          </div>

          <div className="flex flex-col items-center justify-center pt-2">
            <div className="text-[48px] font-mono font-medium tracking-tight">
              {time.toLocaleTimeString("zh-CN", {
                hour12: false,
                hour: "2-digit",
                minute: "2-digit",
                second: "2-digit",
              })}
            </div>
            <div className="flex items-center gap-1.5 mt-2 text-white/80 text-[14px] bg-white/10 px-3 py-1 rounded-full">
              <MapPin className="w-4 h-4" />
              腾讯滨海大厦 (考勤范围内部)
            </div>
          </div>
        </div>

        <div className="flex-1 flex flex-col items-center pt-12 px-6">
          {/* Punch Button */}
          <motion.div whileTap={{ scale: 0.95 }} className="mb-12">
            <button
              onClick={handleClockIn}
              disabled={isDoneToday}
              className={cn(
                "w-40 h-40 rounded-full flex flex-col items-center justify-center text-white shadow-[0_8px_30px_rgb(0,0,0,0.12)] transition-colors",
                isDoneToday
                  ? "bg-slate-400 shadow-slate-400/30"
                  : hasPunchedIn
                    ? "bg-orange-500 shadow-orange-500/30"
                    : "bg-gradient-to-tr from-blue-600 to-primary-blue shadow-blue-500/30",
              )}
            >
              {isDoneToday ? (
                <>
                  <CheckCircle2 className="w-10 h-10 mb-2" />
                  <span className="text-[18px] font-medium">已完成打卡</span>
                </>
              ) : hasPunchedIn ? (
                <>
                  <span className="text-[20px] font-medium mb-1">下班打卡</span>
                  <span className="text-[13px] opacity-80">18:00</span>
                </>
              ) : (
                <>
                  <span className="text-[20px] font-medium mb-1">上班打卡</span>
                  <span className="text-[13px] opacity-80">09:00</span>
                </>
              )}
            </button>
          </motion.div>

          <div className="w-full bg-chat-other-bg rounded-2xl p-4 shadow-sm border border-border-color/50">
            <h3 className="text-[15px] font-medium text-text-main mb-4 flex items-center gap-2">
              <Clock className="w-4 h-4 text-primary-blue" />
              今日打卡记录
            </h3>

            <div className="flex flex-col gap-4 relative">
              {/* Timeline line */}
              <div className="absolute left-[7px] top-2 bottom-2 w-[2px] bg-border-color border-dashed"></div>

              {todayRecords.map((record, index) => (
                <div key={record.id} className="flex gap-4 relative z-10">
                  <div className="w-4 h-4 rounded-full bg-primary-blue border-4 border-chat-other-bg shrink-0 mt-0.5" />
                  <div>
                    <div className="text-[15px] font-medium text-text-main mb-1">
                      {record.type === "in" ? "上班打卡" : "下班打卡"}{" "}
                      {record.time}
                    </div>
                    <div className="flex items-center gap-1 text-[13px] text-text-sub">
                      <MapPin className="w-3.5 h-3.5" />
                      {record.location}
                    </div>
                  </div>
                </div>
              ))}

              {todayRecords.length === 0 && (
                <div className="flex flex-col items-center justify-center py-6 text-text-sub bg-bg-color/50 rounded-xl border border-dashed border-border-color">
                  <Clock className="w-8 h-8 mb-2 opacity-20" />
                  <span className="text-[13px]">今日暂无打卡记录</span>
                </div>
              )}
            </div>
          </div>
        </div>
      </div>
    </PageLayout>
  );
};
