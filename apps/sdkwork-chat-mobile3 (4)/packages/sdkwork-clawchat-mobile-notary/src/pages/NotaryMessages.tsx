import React, { useState, useEffect } from "react";
import {
  Bell,
  ShieldAlert,
  CheckCircle,
  ChevronLeft,
  Info,
} from "lucide-react";
import { IconButton, cn } from "@sdkwork/clawchat-mobile-commons";
import { useNavigate } from "react-router";
import { notaryService } from "../services/notaryService";

const MESSAGE_TYPE_STYLE: Record<string, any> = {
  补充材料通知: { icon: Info, color: "text-blue-500" },
  预约成功提醒: { icon: CheckCircle, color: "text-green-500" },
  申办进度更新: { icon: Bell, color: "text-orange-500" },
  default: { icon: Bell, color: "text-text-sub" },
};

export const NotaryMessages: React.FC = () => {
  const navigate = useNavigate();
  const [messages, setMessages] = useState<any[]>([]);

  useEffect(() => {
    notaryService
      .getNotaryMessages()
      .then((data) => setMessages(data as any[]));
  }, []);

  return (
    <div className="flex flex-col h-full bg-bg-color">
      <header className="h-[56px] flex items-center justify-between px-1 glass-header sticky top-0 z-10 shrink-0 pt-safe relative">
        <div className="flex items-center z-10 flex-1"></div>
        <div className="absolute left-1/2 -translate-x-1/2 flex items-center pointer-events-none">
          <h1 className="text-[17px] font-medium text-text-main">消息通知</h1>
        </div>
        <div className="flex-1" />
      </header>

      <div className="flex-1 overflow-y-auto pb-[90px] flex flex-col">
        {messages.map((msg, idx) => {
          const style =
            MESSAGE_TYPE_STYLE[msg.title] || MESSAGE_TYPE_STYLE["default"];
          const Icon = style.icon;
          return (
            <div
              key={msg.id}
              className={cn(
                "px-4 py-4",
                !msg.unread ? "bg-bg-color" : "bg-primary-blue/5",
                idx !== messages.length - 1
                  ? "border-b border-border-color/50"
                  : "",
              )}
            >
              <div className="flex items-center justify-between mb-1.5">
                <div className="flex items-center gap-2">
                  <Icon className={cn("w-5 h-5", style.color)} />
                  <span className="text-[16px] font-bold text-text-main">
                    {msg.title}
                  </span>
                  {msg.unread && (
                    <div className="w-1.5 h-1.5 bg-red-500 rounded-full" />
                  )}
                </div>
                <span className="text-[12px] text-text-sub">{msg.time}</span>
              </div>
              <p className="text-[14px] text-text-sub leading-relaxed pl-7">
                {msg.content}
              </p>
            </div>
          );
        })}
      </div>
    </div>
  );
};
