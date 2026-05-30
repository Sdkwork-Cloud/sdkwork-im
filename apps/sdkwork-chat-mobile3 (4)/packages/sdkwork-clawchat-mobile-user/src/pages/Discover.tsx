import React from "react";
import { useNavigate } from "react-router";
import {
  ChevronRight,
  Camera,
  Scan,
  Gamepad2,
  Search,
  Video,
  ShoppingBag,
} from "lucide-react";
import { cn } from "@sdkwork/clawchat-mobile-commons";

export const Discover: React.FC = () => {
  const navigate = useNavigate();
  const ListItem = ({
    icon: Icon,
    label,
    rightElement,
    colorClass = "text-text-main",
    hasBorder = true,
    onClick,
  }: any) => (
    <>
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
      {hasBorder && <div className="h-[1px] bg-border-color ml-14" />}
    </>
  );

  return (
    <div className="flex flex-col h-full bg-bg-color overflow-y-auto pb-[84px]">
      {/* Header */}
      <header className="h-[56px] px-4 flex items-center justify-center glass-header sticky top-0 z-10 shrink-0 pt-safe relative">
        <div className="absolute left-1/2 -translate-x-1/2 flex items-center justify-center pointer-events-none">
          <h1 className="text-[17px] font-semibold text-text-main">发现</h1>
        </div>
      </header>

      <div className="flex flex-col mt-2">
        {/* Moments */}
        <div className="mb-2 border-y border-border-color bg-chat-other-bg">
          <ListItem
            icon={Camera}
            label="朋友圈"
            colorClass="text-[#2B5CE7]"
            hasBorder={false}
            onClick={() => navigate("/discover/moments")}
            rightElement={
              <div className="relative">
                <img
                  src="https://picsum.photos/seed/moment/32/32"
                  alt="New moment"
                  className="w-8 h-8 rounded-md"
                />
                <div className="absolute -top-1 -right-1 w-2.5 h-2.5 bg-accent-red rounded-full border border-chat-other-bg" />
              </div>
            }
          />
        </div>

        {/* Channels */}
        <div className="mb-2 border-y border-border-color flex flex-col bg-chat-other-bg">
          <ListItem
            icon={Video}
            label="作品"
            hasBorder={false}
            colorClass="text-[#FF7D00]"
            onClick={() => navigate("/discover/channels")}
          />
        </div>

        {/* Scan & Search */}
        <div className="mb-2 border-y border-border-color flex flex-col bg-chat-other-bg">
          <ListItem
            icon={Scan}
            label="扫一扫"
            colorClass="text-[#2B5CE7]"
            onClick={() => navigate("/scan")}
          />
          <ListItem
            icon={Search}
            label="搜一搜"
            colorClass="text-[#F53F3F]"
            hasBorder={false}
            onClick={() => navigate("/discover/search")}
          />
        </div>

        {/* Games & Shopping */}
        <div className="mb-2 border-y border-border-color flex flex-col bg-chat-other-bg">
          <ListItem
            icon={Gamepad2}
            label="游戏"
            colorClass="text-[#00B42A]"
            onClick={() => navigate("/discover/games")}
          />
          <ListItem
            icon={ShoppingBag}
            label="购物"
            colorClass="text-[#FF7D00]"
            hasBorder={false}
            onClick={() => navigate("/discover/shopping")}
          />
        </div>
      </div>
    </div>
  );
};
