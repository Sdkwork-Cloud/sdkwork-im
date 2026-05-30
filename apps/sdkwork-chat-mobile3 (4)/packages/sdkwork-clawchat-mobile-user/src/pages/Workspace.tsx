import React, { useState } from "react";
import { useNavigate } from "react-router";
import {
  Calendar,
  CheckSquare,
  FileText,
  Cloud,
  Video,
  Users,
  Briefcase,
  Plus,
  Search,
  Wand2,
  Mic,
  Sparkles,
  Scale,
  MessageSquare,
  UserPlus,
} from "lucide-react";
import { IconButton, showToast } from "@sdkwork/clawchat-mobile-commons";

export const Workspace: React.FC = () => {
  const navigate = useNavigate();
  const [showMenu, setShowMenu] = useState(false);

  const AppIcon = ({
    icon: Icon,
    label,
    colorClass,
    bgClass,
    onClick,
  }: any) => (
    <div
      className="flex flex-col items-center gap-2 cursor-pointer active:opacity-70 transition-opacity"
      onClick={onClick}
    >
      <div
        className={`w-12 h-12 rounded-2xl flex items-center justify-center ${bgClass}`}
      >
        <Icon className={`w-6 h-6 ${colorClass}`} />
      </div>
      <span className="text-[12px] text-text-main">{label}</span>
    </div>
  );

  return (
    <div className="flex flex-col h-full bg-bg-color overflow-y-auto pb-[84px]">
      {/* Header */}
      <header className="h-[56px] px-4 flex items-center justify-between sticky top-0 z-10 shrink-0 pt-safe relative bg-bg-color">
        <div className="absolute left-1/2 -translate-x-1/2 flex items-center justify-center pointer-events-none">
          <h1 className="text-[17px] font-semibold text-text-main">工作台</h1>
        </div>
        <div className="flex-1" />
        <div className="flex gap-2 relative z-10">
          <IconButton
            icon={<Search className="w-5 h-5 text-text-main" />}
            className="bg-black/5 dark:bg-white/5 w-8 h-8 p-0"
            onClick={() => navigate("/search")}
          />
          <IconButton
            icon={<Plus className="w-5 h-5 text-text-main" />}
            className="bg-black/5 dark:bg-white/5 w-8 h-8 p-0"
            onClick={() => setShowMenu(!showMenu)}
          />
          {showMenu && (
            <>
              <div
                className="fixed inset-0 z-40"
                onClick={() => setShowMenu(false)}
              />
              <div className="absolute top-12 right-0 bg-[#4C4C4C] rounded-lg w-36 shadow-lg z-50 overflow-hidden text-white">
                <div
                  className="px-4 py-3 border-b border-black/20 flex items-center gap-3 active:bg-black/20"
                  onClick={() => {
                    setShowMenu(false);
                    navigate("/create-group");
                  }}
                >
                  <MessageSquare className="w-5 h-5" />
                  <span className="text-[15px]">发起群聊</span>
                </div>
                <div
                  className="px-4 py-3 flex items-center gap-3 active:bg-black/20"
                  onClick={() => {
                    setShowMenu(false);
                    navigate("/add-friend");
                  }}
                >
                  <UserPlus className="w-5 h-5" />
                  <span className="text-[15px]">添加朋友</span>
                </div>
              </div>
            </>
          )}
        </div>
      </header>

      <div className="flex flex-col pb-6 px-4">
        {/* AI Tools */}
        <div className="pt-2 pb-6">
          <div className="flex items-center justify-between mb-5">
            <h3 className="text-[15px] font-bold text-text-main flex items-center gap-1.5">
              <Sparkles className="w-4 h-4 text-purple-500" />
              AI工具
            </h3>
          </div>
          <div className="grid grid-cols-4 gap-y-6">
            <AppIcon
              icon={Video}
              label="AI 视频"
              colorClass="text-indigo-500"
              bgClass="bg-indigo-500/10"
              onClick={() => navigate("/ai/video")}
            />
            <AppIcon
              icon={Wand2}
              label="AI 绘图"
              colorClass="text-pink-500"
              bgClass="bg-pink-500/10"
              onClick={() => navigate("/ai/image")}
            />
            <AppIcon
              icon={FileText}
              label="智能写作"
              colorClass="text-orange-500"
              bgClass="bg-orange-500/10"
              onClick={() => navigate("/ai/writing")}
            />
            <AppIcon
              icon={Mic}
              label="语音摘要"
              colorClass="text-emerald-500"
              bgClass="bg-emerald-500/10"
              onClick={() => navigate("/workspace/voice-summary")}
            />
          </div>
        </div>

        {/* Common Apps */}
        <div className="pt-4 pb-6">
          <h3 className="text-[15px] font-bold text-text-main mb-5">
            常用应用
          </h3>
          <div className="grid grid-cols-4 gap-y-6">
            <AppIcon
              icon={Scale}
              label="公证业务"
              colorClass="text-[#3b82f6]"
              bgClass="bg-[#3b82f6]/10"
              onClick={() => navigate("/notary")}
            />
            <AppIcon
              icon={Calendar}
              label="日历"
              colorClass="text-blue-500"
              bgClass="bg-blue-500/10"
              onClick={() => navigate("/calendar")}
            />
            <AppIcon
              icon={CheckSquare}
              label="审批"
              colorClass="text-orange-500"
              bgClass="bg-orange-500/10"
              onClick={() => navigate("/workspace/approval")}
            />
            <AppIcon
              icon={Briefcase}
              label="打卡"
              colorClass="text-green-500"
              bgClass="bg-green-500/10"
              onClick={() => navigate("/workspace/attendance")}
            />
            <AppIcon
              icon={FileText}
              label="汇报"
              colorClass="text-purple-500"
              bgClass="bg-purple-500/10"
              onClick={() => navigate("/workspace/report")}
            />
            <AppIcon
              icon={Cloud}
              label="云盘"
              colorClass="text-cyan-500"
              bgClass="bg-cyan-500/10"
              onClick={() => navigate("/workspace/drive")}
            />
            <AppIcon
              icon={Video}
              label="会议"
              colorClass="text-indigo-500"
              bgClass="bg-indigo-500/10"
              onClick={() => navigate("/workspace/meeting")}
            />
            <AppIcon
              icon={Users}
              label="通讯录"
              colorClass="text-pink-500"
              bgClass="bg-pink-500/10"
              onClick={() => navigate("/workspace/contacts")}
            />
          </div>
        </div>

        {/* HR & Admin */}
        <div className="pt-4 pb-6">
          <h3 className="text-[15px] font-bold text-text-main mb-5">
            人事行政
          </h3>
          <div className="grid grid-cols-4 gap-y-6">
            <AppIcon
              icon={FileText}
              label="请假"
              colorClass="text-orange-500"
              bgClass="bg-orange-500/10"
              onClick={() => navigate("/workspace/approval")}
            />
            <AppIcon
              icon={Briefcase}
              label="出差"
              colorClass="text-blue-500"
              bgClass="bg-blue-500/10"
              onClick={() => navigate("/workspace/approval")}
            />
            <AppIcon
              icon={CheckSquare}
              label="报销"
              colorClass="text-green-500"
              bgClass="bg-green-500/10"
              onClick={() => navigate("/workspace/approval")}
            />
            <AppIcon
              icon={Users}
              label="招聘"
              colorClass="text-purple-500"
              bgClass="bg-purple-500/10"
              onClick={() => navigate("/workspace/recruitment")}
            />
          </div>
        </div>
      </div>
    </div>
  );
};
