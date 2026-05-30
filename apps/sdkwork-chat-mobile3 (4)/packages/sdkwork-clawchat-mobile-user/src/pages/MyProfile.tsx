import React, { useState, useEffect } from "react";
import { useNavigate } from "react-router";
import { ChevronLeft, ChevronRight, QrCode } from "lucide-react";
import { Avatar, IconButton, cn } from "@sdkwork/clawchat-mobile-commons";
import { ProfileService, type UserProfile } from "../services/ProfileService";

export const MyProfile: React.FC = () => {
  const navigate = useNavigate();
  const [profile, setProfile] = useState<UserProfile | null>(null);

  useEffect(() => {
    ProfileService.getUserProfile().then(setProfile);
  }, []);

  const ListItem = ({
    label,
    rightText,
    rightElement,
    onClick,
  }: {
    label: React.ReactNode;
    rightText?: React.ReactNode;
    rightElement?: React.ReactNode;
    onClick?: () => void;
  }) => (
    <div
      onClick={onClick}
      className="flex items-center justify-between px-4 py-3.5 bg-chat-other-bg active:bg-active-bg transition-colors cursor-pointer border-b border-border-color last:border-none"
    >
      <span className="text-[16px] text-text-main">{label}</span>
      <div className="flex items-center gap-2 text-text-sub">
        {rightText && <span className="text-[15px]">{rightText}</span>}
        {rightElement}
        <ChevronRight className="w-5 h-5 opacity-50" />
      </div>
    </div>
  );

  return (
    <div className="flex flex-col h-full bg-bg-color overflow-y-auto">
      {/* Header */}
      <header className="h-[56px] flex items-center justify-between px-1 glass-header sticky top-0 z-10 shrink-0 pt-safe">
        <div className="flex items-center z-10 flex-1">
          <IconButton
            icon={<ChevronLeft className="w-6 h-6 text-text-main" />}
            onClick={() => navigate(-1)}
          />
        </div>
        <div className="absolute left-1/2 -translate-x-1/2 flex items-center justify-center pointer-events-none">
          <h2 className="text-[17px] font-medium text-text-main">个人信息</h2>
        </div>
        <div className="flex-1" />
      </header>

      <div className="flex flex-col pb-8 mt-2">
        {/* Group 1 */}
        <div className="mb-2 border-y border-border-color flex flex-col">
          <div
            onClick={() => navigate("/my-profile/avatar")}
            className="flex items-center justify-between px-4 py-3 bg-chat-other-bg active:bg-active-bg transition-colors cursor-pointer border-b border-border-color"
          >
            <span className="text-[16px] text-text-main">头像</span>
            <div className="flex items-center gap-2 text-text-sub">
              <Avatar
                src={profile?.avatar || "https://picsum.photos/seed/me/200/200"}
                size="md"
                className="w-14 h-14 rounded-xl"
              />
              <ChevronRight className="w-5 h-5 opacity-50" />
            </div>
          </div>
          <ListItem
            label="名字"
            rightText={profile?.name || "User"}
            onClick={() => navigate("/my-profile/name")}
          />
          <ListItem
            label="拍一拍"
            onClick={() => navigate("/my-profile/tickle")}
          />
          <ListItem
            label="微信号"
            rightText={profile?.wechatId || "wxid_123456789"}
          />
          <ListItem
            label="我的二维码"
            rightElement={<QrCode className="w-5 h-5" />}
            onClick={() => navigate("/my-profile/qrcode")}
          />
          <ListItem label="更多" onClick={() => navigate("/my-profile/more")} />
        </div>

        {/* Group 2 */}
        <div className="mb-2 border-y border-border-color flex flex-col">
          <ListItem
            label="来电铃声"
            onClick={() => navigate("/my-profile/ringtone")}
          />
        </div>

        {/* Group 3 */}
        <div className="border-y border-border-color flex flex-col">
          <ListItem
            label="微信豆"
            onClick={() => navigate("/my-profile/beans")}
          />
          <ListItem
            label="我的地址"
            onClick={() => navigate("/my-profile/address")}
          />
        </div>
      </div>
    </div>
  );
};
