import React from "react";
import { useNavigate } from "react-router";
import {
  ChevronRight,
  CircleUserRound,
  Settings,
  Wallet,
  Bookmark,
  Smile,
  QrCode,
  Bot,
  Folder,
  Package,
  UserRound,
  Mic,
} from "lucide-react";
import { Avatar, cn } from "@sdkwork/clawchat-mobile-commons";
import { useAppStore } from "@sdkwork/clawchat-mobile-core";

export const Me: React.FC = () => {
  const { currentUser } = useAppStore();
  const navigate = useNavigate();

  const ListItem = ({
    icon: Icon,
    label,
    onClick,
    rightElement,
    colorClass = "text-text-main",
  }: any) => (
    <div
      onClick={onClick}
      className="flex items-center justify-between px-4 py-3.5 bg-chat-other-bg active:bg-active-bg transition-colors cursor-pointer"
    >
      <div className="flex items-center gap-3">
        <Icon className={cn("w-6 h-6", colorClass)} />
        <span className="text-[16px] text-text-main">{label}</span>
      </div>
      <div className="flex items-center gap-2 text-text-sub">
        {rightElement}
        <ChevronRight className="w-5 h-5 opacity-50" />
      </div>
    </div>
  );

  return (
    <div className="flex flex-col h-full bg-bg-color overflow-y-auto pb-[84px]">
      {/* Header */}
      <header className="h-[56px] px-4 flex items-center justify-center glass-header sticky top-0 z-10 shrink-0 pt-safe relative">
        <div className="absolute left-1/2 -translate-x-1/2 flex items-center justify-center pointer-events-none">
          <h1 className="text-[17px] font-semibold text-text-main">我</h1>
        </div>
      </header>

      <div className="flex flex-col">
        {/* Profile Section */}
        <div
          onClick={() => navigate("/my-profile")}
          className="bg-chat-other-bg px-4 py-8 mb-2 flex items-center justify-between active:bg-active-bg transition-colors cursor-pointer border-b border-border-color"
        >
          <div className="flex items-center gap-4 flex-1 min-w-0">
            <Avatar
              src={
                currentUser?.avatar || "https://picsum.photos/seed/me/200/200"
              }
              size="lg"
              className="w-16 h-16 rounded-xl shrink-0"
            />
            <div className="flex flex-col justify-center min-w-0 flex-1">
              <h2 className="text-[20px] font-bold text-text-main mb-1 truncate">
                {currentUser?.name || "User"}
              </h2>
              <p className="text-[14px] text-text-sub truncate">
                微信号: wxid_123456789
              </p>
            </div>
          </div>
          <div className="flex items-center gap-3 text-text-sub">
            <QrCode className="w-5 h-5" />
            <ChevronRight className="w-5 h-5 opacity-50" />
          </div>
        </div>

        {/* Services */}
        <div className="mb-2 border-y border-border-color flex flex-col bg-chat-other-bg">
          <ListItem
            icon={Wallet}
            label="服务"
            colorClass="text-[#2B5CE7]"
            onClick={() => navigate("/me/services")}
          />
          <div className="h-[1px] bg-border-color ml-14" />
          <ListItem
            icon={Package}
            label="订单中心"
            colorClass="text-[#FF7D00]"
            onClick={() => navigate("/me/orders")}
          />
        </div>

        {/* AI Assets */}
        <div className="mb-2 border-y border-border-color flex flex-col bg-chat-other-bg">
          <ListItem
            icon={UserRound}
            label="我的角色"
            colorClass="text-[#10B981]"
            onClick={() => navigate("/me/characters")}
          />
          <div className="h-[1px] bg-border-color ml-14" />
          <ListItem
            icon={Mic}
            label="我的声音"
            colorClass="text-[#8B5CF6]"
            onClick={() => navigate("/me/voices")}
          />
        </div>

        {/* Features */}
        <div className="mb-2 border-y border-border-color flex flex-col bg-chat-other-bg">
          <ListItem
            icon={Bookmark}
            label="收藏"
            colorClass="text-[#F53F3F]"
            onClick={() => navigate("/me/favorites")}
          />
          <div className="h-[1px] bg-border-color ml-14" />
          <ListItem
            icon={Bot}
            label="智能体"
            colorClass="text-[#2B5CE7]"
            onClick={() => navigate("/me/agents")}
          />
          <div className="h-[1px] bg-border-color ml-14" />
          <ListItem
            icon={Folder}
            label="我的作品"
            colorClass="text-[#8B5CF6]"
            onClick={() => navigate("/me/works")}
          />
          <div className="h-[1px] bg-border-color ml-14" />
          <ListItem
            icon={Smile}
            label="表情"
            colorClass="text-[#FF7D00]"
            onClick={() => navigate("/me/emoji")}
          />
        </div>

        {/* Settings */}
        <div className="mb-6 border-y border-border-color flex flex-col bg-chat-other-bg">
          <ListItem
            icon={Settings}
            label="设置"
            colorClass="text-[#00B42A]"
            onClick={() => navigate("/settings")}
          />
        </div>
      </div>
    </div>
  );
};
