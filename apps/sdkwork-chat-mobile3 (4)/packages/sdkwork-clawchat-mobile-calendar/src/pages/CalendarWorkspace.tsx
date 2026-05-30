import React, { useState, useEffect } from "react";
import {
  ChevronLeft,
  ChevronRight,
  Plus,
  Search,
  MoreHorizontal,
  Calendar as CalendarIcon,
  Clock,
  MapPin,
  Users,
  X,
  Trash2,
} from "lucide-react";
import { cn, IconButton, showToast } from "@sdkwork/clawchat-mobile-commons";
import { useNavigate } from "react-router";
import { motion, AnimatePresence } from "motion/react";
import { CalendarService, type Schedule } from "../services/CalendarService";
import { format } from "date-fns";

export const CalendarWorkspace: React.FC = () => {
  const navigate = useNavigate();
  const [currentDate, setCurrentDate] = useState(new Date());
  const [schedules, setSchedules] = useState<Schedule[]>([]);
  const [loading, setLoading] = useState(true);
  const [indicators, setIndicators] = useState<string[]>([]);

  const [isAdding, setIsAdding] = useState(false);
  const [newTitle, setNewTitle] = useState("");
  const [newTime, setNewTime] = useState("");

  const loadData = async () => {
    setLoading(true);
    const data = await CalendarService.getSchedulesByDate(currentDate);
    setSchedules(data);
    setLoading(false);
  };

  useEffect(() => {
    loadData();
  }, [currentDate]);

  useEffect(() => {
    const loadIndicators = async () => {
      const year = currentDate.getFullYear();
      const month = currentDate.getMonth();
      const dates = await CalendarService.getIndicatorsForMonth(year, month);
      setIndicators(dates);
    };
    loadIndicators();
  }, [currentDate.getFullYear(), currentDate.getMonth(), schedules.length]); // Refresh indicators when schedules change

  const handleAddSchedule = async () => {
    if (!newTitle.trim()) {
      showToast("请输入日程标题");
      return;
    }
    await CalendarService.addSchedule({
      title: newTitle,
      time: newTime || "全天",
      type: "event",
      color: "bg-blue-500",
      date: format(currentDate, "yyyy-MM-dd"),
    });
    showToast("添加成功");
    setIsAdding(false);
    setNewTitle("");
    setNewTime("");
    loadData();
  };

  const handleDeleteSchedule = async (id: number) => {
    await CalendarService.deleteSchedule(id);
    showToast("删除成功");
    loadData();
  };

  // Very simplistic month calendar generation
  const getDaysInMonth = (year: number, month: number) =>
    new Date(year, month + 1, 0).getDate();
  const getFirstDayOfMonth = (year: number, month: number) =>
    new Date(year, month, 1).getDay();

  const year = currentDate.getFullYear();
  const month = currentDate.getMonth();

  const daysInMonth = getDaysInMonth(year, month);
  const firstDay = getFirstDayOfMonth(year, month);

  const days = [];
  // padding days
  const prevMonthDays = getDaysInMonth(year, month - 1);
  for (let i = 0; i < firstDay; i++) {
    days.push({
      day: prevMonthDays - firstDay + i + 1,
      currentMonth: false,
      dateStr: "",
    });
  }
  for (let i = 1; i <= daysInMonth; i++) {
    days.push({
      day: i,
      currentMonth: true,
      isToday:
        i === new Date().getDate() &&
        month === new Date().getMonth() &&
        year === new Date().getFullYear(),
      dateStr: format(new Date(year, month, i), "yyyy-MM-dd"),
    });
  }
  const remainingSlots = 42 - days.length;
  for (let i = 1; i <= remainingSlots; i++) {
    days.push({ day: i, currentMonth: false, dateStr: "" });
  }

  const prevMonth = () =>
    setCurrentDate(new Date(year, month - 1, currentDate.getDate()));
  const nextMonth = () =>
    setCurrentDate(new Date(year, month + 1, currentDate.getDate()));
  const selectDay = (day: number) => setCurrentDate(new Date(year, month, day));

  const weekDays = ["日", "一", "二", "三", "四", "五", "六"];

  return (
    <div className="flex flex-col h-full bg-bg-color font-sans relative animate-in slide-in-from-right z-10 w-full absolute inset-0">
      {/* Header */}
      <header className="h-[44px] flex items-center justify-between sticky top-0 shrink-0 pt-safe px-2 z-20 bg-bg-color border-b border-border-color">
        <div className="flex items-center z-10 w-[80px]">
          <IconButton
            icon={
              <ChevronLeft className="w-7 h-7 text-text-main" strokeWidth={2} />
            }
            onClick={() => navigate(-1)}
          />
        </div>
        <div className="flex items-center justify-center font-medium text-[17px] pointer-events-none flex-1 gap-2">
          <span className="cursor-pointer pointer-events-auto">{`${year}年${month + 1}月`}</span>
        </div>
        <div className="flex justify-end z-10 w-[80px] pr-2 gap-2">
          <IconButton icon={<Search className="w-5 h-5 text-text-main" />} />
          <IconButton
            icon={<Plus className="w-6 h-6 text-text-main" />}
            onClick={() => setIsAdding(true)}
          />
        </div>
      </header>

      <div className="flex-1 overflow-y-auto pb-safe flex flex-col">
        {/* Calendar Grid */}
        <div className="bg-bg-color px-4 pb-4 border-b border-border-color shadow-sm z-10">
          <div className="flex items-center justify-between py-2">
            <IconButton
              icon={<ChevronLeft className="w-5 h-5 text-text-sub" />}
              onClick={prevMonth}
            />
            <div className="text-[15px] font-bold tracking-wide">
              {currentDate.toLocaleDateString("zh-CN", {
                month: "long",
                year: "numeric",
              })}
            </div>
            <IconButton
              icon={<ChevronRight className="w-5 h-5 text-text-sub" />}
              onClick={nextMonth}
            />
          </div>

          <div className="grid grid-cols-7 mb-2">
            {weekDays.map((day) => (
              <div
                key={day}
                className="text-center text-[12px] text-text-sub font-medium py-2"
              >
                {day}
              </div>
            ))}
          </div>

          <div className="grid grid-cols-7 gap-y-2">
            {days.map((item, idx) => (
              <div
                key={idx}
                className="flex flex-col items-center justify-center h-10 w-full relative cursor-pointer"
                onClick={() => item.currentMonth && selectDay(item.day)}
              >
                <div
                  className={cn(
                    "w-8 h-8 rounded-full flex items-center justify-center text-[15px] transition-colors",
                    item.isToday
                      ? "bg-primary-blue text-white font-bold"
                      : item.currentMonth && item.day === currentDate.getDate()
                        ? "border border-primary-blue text-primary-blue"
                        : "",
                    !item.currentMonth
                      ? "text-text-sub/40"
                      : !item.isToday &&
                          item.day !== currentDate.getDate() &&
                          "text-text-main font-medium",
                  )}
                >
                  {item.day}
                </div>
                {/* Indicator dot */}
                {item.currentMonth &&
                  indicators.includes(item.dateStr) &&
                  !item.isToday && (
                    <div className="w-1 h-1 rounded-full bg-blue-500 absolute bottom-0"></div>
                  )}
                {item.currentMonth &&
                  indicators.includes(item.dateStr) &&
                  item.isToday && (
                    <div className="w-1 h-1 rounded-full bg-white absolute bottom-1"></div>
                  )}
              </div>
            ))}
          </div>
        </div>

        {/* Schedule List */}
        <div className="flex-1 bg-[#F5F6F8] dark:bg-black p-4 flex flex-col gap-3">
          <div className="text-[14px] text-text-sub font-medium mb-1">
            {itemIsToday(currentDate)
              ? "今天"
              : `${month + 1}月${currentDate.getDate()}日`}{" "}
            的日程
          </div>

          {loading ? (
            <div className="flex flex-col items-center justify-center py-12 text-text-sub opacity-70">
              <div className="w-8 h-8 rounded-full border-4 border-text-sub border-t-transparent animate-spin mb-3"></div>
              <span className="text-[14px]">加载中...</span>
            </div>
          ) : schedules.length > 0 ? (
            schedules.map((schedule) => (
              <div
                key={schedule.id}
                className="bg-bg-color rounded-xl p-4 shadow-sm border border-border-color flex items-stretch gap-3 cursor-pointer active:scale-[0.98] transition-all relative group"
              >
                <div
                  className={cn("w-1 rounded-full shrink-0", schedule.color)}
                />
                <div className="flex flex-col flex-1">
                  <span className="text-[16px] font-bold text-text-main mb-1.5">
                    {schedule.title}
                  </span>
                  <div className="flex items-center text-[13px] text-text-sub gap-1.5">
                    <Clock className="w-3.5 h-3.5" />
                    <span>{schedule.time}</span>
                  </div>
                </div>
                <div
                  className="absolute right-3 top-1/2 -translate-y-1/2 p-2 hover:bg-black/5 dark:hover:bg-white/5 rounded-full"
                  onClick={(e) => {
                    e.stopPropagation();
                    handleDeleteSchedule(schedule.id);
                  }}
                >
                  <Trash2 className="w-4 h-4 text-red-500 opacity-60 hover:opacity-100 transition-opacity" />
                </div>
              </div>
            ))
          ) : (
            <div className="flex flex-col items-center justify-center py-12 text-text-sub opacity-70">
              <CalendarIcon className="w-12 h-12 mb-3 stroke-current opacity-40" />
              <span className="text-[14px]">暂无日程安排</span>
            </div>
          )}

          {/* Create Button (Floating inside list) */}
          <div className="mt-4 flex justify-center">
            <button
              className="flex items-center gap-1.5 text-primary-blue text-[14px] font-medium py-2 px-4 rounded-full bg-blue-50 dark:bg-blue-900/20 active:opacity-80 transition-opacity"
              onClick={() => setIsAdding(true)}
            >
              <Plus className="w-4 h-4" />
              添加日程
            </button>
          </div>
        </div>
      </div>

      <AnimatePresence>
        {isAdding && (
          <motion.div
            initial={{ opacity: 0 }}
            animate={{ opacity: 1 }}
            exit={{ opacity: 0 }}
            className="fixed inset-0 z-50 bg-black/50 flex items-center justify-center p-4"
            onClick={() => setIsAdding(false)}
          >
            <motion.div
              initial={{ scale: 0.95 }}
              animate={{ scale: 1 }}
              exit={{ scale: 0.95 }}
              onClick={(e) => e.stopPropagation()}
              className="bg-bg-color w-full max-w-sm rounded-2xl p-5 shadow-xl flex flex-col"
            >
              <div className="flex justify-between items-center mb-4">
                <h3 className="text-lg font-bold">新建日程</h3>
                <IconButton
                  icon={<X className="w-5 h-5" />}
                  onClick={() => setIsAdding(false)}
                />
              </div>
              <div className="flex flex-col gap-4">
                <input
                  type="text"
                  placeholder="标题"
                  className="bg-chat-other-bg rounded-lg px-4 py-3 text-[15px] outline-none"
                  value={newTitle}
                  onChange={(e) => setNewTitle(e.target.value)}
                  autoFocus
                />
                <input
                  type="text"
                  placeholder="时间 (例如：10:00 - 11:00)"
                  className="bg-chat-other-bg rounded-lg px-4 py-3 text-[15px] outline-none"
                  value={newTime}
                  onChange={(e) => setNewTime(e.target.value)}
                />
                <div className="text-sm text-text-sub mt-2 mb-4">
                  将会添加到 {format(currentDate, "yyyy年MM月dd日")}
                </div>
                <button
                  className="bg-primary-blue text-white w-full rounded-full py-3 font-semibold disabled:opacity-50"
                  onClick={handleAddSchedule}
                  disabled={!newTitle.trim()}
                >
                  保存
                </button>
              </div>
            </motion.div>
          </motion.div>
        )}
      </AnimatePresence>
    </div>
  );
};

// Helper
const itemIsToday = (date: Date) => {
  const today = new Date();
  return (
    date.getDate() === today.getDate() &&
    date.getMonth() === today.getMonth() &&
    date.getFullYear() === today.getFullYear()
  );
};
