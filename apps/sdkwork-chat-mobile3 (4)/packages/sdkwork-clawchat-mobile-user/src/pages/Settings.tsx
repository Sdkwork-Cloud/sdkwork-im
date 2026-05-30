import React, { useState, useEffect } from "react";
import { useNavigate } from "react-router";
import { ChevronLeft, ChevronRight } from "lucide-react";
import { IconButton, cn } from "@sdkwork/clawchat-mobile-commons";
import { SettingsService } from "../services/SettingsService";
import { AuthService } from "../services/AuthService";

export const SettingsPage: React.FC = () => {
  const navigate = useNavigate();
  const [isLoggingOut, setIsLoggingOut] = useState(false);
  const [teenMode, setTeenMode] = useState(false);
  const [elderlyMode, setElderlyMode] = useState(false);

  useEffect(() => {
    SettingsService.getSettings().then((s) => {
      setTeenMode(s.teenMode);
      setElderlyMode(s.elderlyMode);
    });
  }, []);

  const handleLogout = async () => {
    if (isLoggingOut) return;
    setIsLoggingOut(true);
    await AuthService.logout();
    setIsLoggingOut(false);
    navigate("/login", { replace: true });
  };

  const ListItem = ({
    label,
    rightText,
    onClick,
    center,
    danger,
    hideBorder,
  }: any) => (
    <div
      onClick={onClick}
      className={cn(
        "flex items-center px-4 py-3.5 bg-chat-other-bg active:bg-active-bg transition-colors cursor-pointer",
        !hideBorder && "border-b border-border-color/60",
        center ? "justify-center" : "justify-between",
      )}
    >
      <span
        className={cn(
          "text-[16px]",
          danger ? "text-[#FA5151] font-medium" : "text-text-main",
        )}
      >
        {label}
      </span>
      {!center && (
        <div className="flex items-center gap-1.5 text-text-sub">
          {rightText && <span className="text-[15px]">{rightText}</span>}
          <ChevronRight
            className="w-[18px] h-[18px] opacity-40"
            strokeWidth={2.5}
          />
        </div>
      )}
    </div>
  );

  const Group = ({
    children,
    className,
  }: {
    children: React.ReactNode;
    className?: string;
  }) => (
    <div
      className={cn(
        "mb-2 border-y border-border-color/60 flex flex-col",
        className,
      )}
    >
      {children}
    </div>
  );

  return (
    <div className="flex flex-col h-full bg-bg-color overflow-y-auto">
      {/* Header */}
      <header className="h-[56px] flex items-center justify-between px-1 glass-header sticky top-0 z-10 shrink-0 pt-safe">
        <div className="flex items-center z-10 flex-1">
          <IconButton
            icon={
              <ChevronLeft
                className="w-6 h-6 text-text-main"
                strokeWidth={2.5}
              />
            }
            onClick={() => navigate(-1)}
          />
        </div>
        <div className="absolute left-1/2 -translate-x-1/2 flex items-center justify-center pointer-events-none">
          <h2 className="text-[17px] font-medium text-text-main">设置</h2>
        </div>
        <div className="flex-1" />
      </header>

      <div className="flex flex-col pb-12 mt-2">
        <Group>
          <ListItem
            label="账号与安全"
            rightText="已保护"
            hideBorder
            onClick={() => navigate("/settings/account")}
          />
        </Group>

        <Group>
          <ListItem
            label="青少年模式"
            rightText={teenMode ? "已开启" : "未开启"}
            onClick={() => navigate("/settings/teen-mode")}
          />
          <ListItem
            label="关怀模式"
            rightText={elderlyMode ? "已开启" : "未开启"}
            hideBorder
            onClick={() => navigate("/settings/elderly-mode")}
          />
        </Group>

        <Group>
          <ListItem
            label="新消息通知"
            onClick={() => navigate("/settings/notifications")}
          />
          <ListItem label="聊天" onClick={() => navigate("/settings/chat")} />
          <ListItem
            label="设备"
            onClick={() => navigate("/settings/devices")}
          />
          <ListItem
            label="通用"
            hideBorder
            onClick={() => navigate("/settings/general")}
          />
        </Group>

        <Group>
          <ListItem
            label="朋友权限"
            onClick={() => navigate("/settings/friend-permissions")}
          />
          <ListItem
            label="个人信息与权限"
            onClick={() => navigate("/settings/privacy")}
          />
          <ListItem
            label="个人信息收集清单"
            onClick={() => navigate("/settings/info-collection")}
          />
          <ListItem
            label="第三方信息共享清单"
            hideBorder
            onClick={() => navigate("/settings/third-party-sharing")}
          />
        </Group>

        <Group>
          <ListItem
            label="插件"
            hideBorder
            onClick={() => navigate("/settings/plugins")}
          />
        </Group>

        <Group>
          <ListItem
            label="帮助与反馈"
            onClick={() => navigate("/settings/help")}
          />
          <ListItem
            label="关于 ClawChat"
            rightText="版本 1.0.0"
            hideBorder
            onClick={() => navigate("/settings/about")}
          />
        </Group>

        <Group className="mt-4">
          <ListItem
            label="切换账号"
            center
            onClick={() => navigate("/settings/switch-account")}
          />
          <ListItem
            label={isLoggingOut ? "退出中..." : "退出登录"}
            center
            danger
            hideBorder
            onClick={handleLogout}
          />
        </Group>
      </div>
    </div>
  );
};
