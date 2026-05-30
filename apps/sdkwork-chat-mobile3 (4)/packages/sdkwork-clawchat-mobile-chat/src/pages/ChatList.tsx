import React, { useState, useRef, useEffect, useCallback } from "react";
import { useNavigate } from "react-router";
import {
  Search,
  PlusCircle,
  MessageSquarePlus,
  UserPlus,
  Bot,
  Scan,
  Pin,
  BellOff,
  Trash2,
} from "lucide-react";
import { format } from "date-fns";
import {
  Avatar,
  Badge,
  IconButton,
  cn,
} from "@sdkwork/clawchat-mobile-commons";
import type { Chat } from "@sdkwork/clawchat-mobile-types";
import { ChatService } from "../services/ChatService";
import { motion, AnimatePresence } from "motion/react";
import { ChatListContextMenu } from "../components/Chat/ChatListContextMenu";
import { AddMenu } from "../components/Chat/AddMenu";

export const ChatList: React.FC = () => {
  const navigate = useNavigate();
  const [isMenuOpen, setIsMenuOpen] = useState(false);
  const [chats, setChats] = useState<Chat[]>([]);
  const menuRef = useRef<HTMLDivElement>(null);

  // Context Menu State
  const [contextMenu, setContextMenu] = useState<{
    isOpen: boolean;
    x: number;
    y: number;
    chatId: string | null;
  }>({ isOpen: false, x: 0, y: 0, chatId: null });

  const longPressTimer = useRef<NodeJS.Timeout | null>(null);

  const loadChats = useCallback(() => {
    ChatService.getChats().then((data) => {
      // Sort pinned chats to top
      const sorted = [...data].sort((a, b) => {
        if (a.isPinned && !b.isPinned) return -1;
        if (!a.isPinned && b.isPinned) return 1;
        return 0; // Keep original order for others
      });
      setChats(sorted);
    });
  }, []);

  useEffect(() => {
    loadChats();
  }, [loadChats]);

  // Handle click outside to close menu
  useEffect(() => {
    const handleClickOutside = (event: MouseEvent | TouchEvent) => {
      if (menuRef.current && !menuRef.current.contains(event.target as Node)) {
        setIsMenuOpen(false);
      }
      if (contextMenu.isOpen) {
        setContextMenu((prev) => ({ ...prev, isOpen: false }));
      }
    };

    if (isMenuOpen || contextMenu.isOpen) {
      document.addEventListener("mousedown", handleClickOutside);
      document.addEventListener("touchstart", handleClickOutside);
    }

    return () => {
      document.removeEventListener("mousedown", handleClickOutside);
      document.removeEventListener("touchstart", handleClickOutside);
    };
  }, [isMenuOpen, contextMenu.isOpen]);

  const handleTouchStart = useCallback(
    (e: React.TouchEvent | React.MouseEvent, chatId: string) => {
      if (longPressTimer.current) clearTimeout(longPressTimer.current);

      // Get coordinates
      let clientX, clientY;
      if ("touches" in e) {
        clientX = e.touches[0].clientX;
        clientY = e.touches[0].clientY;
      } else {
        clientX = (e as React.MouseEvent).clientX;
        clientY = (e as React.MouseEvent).clientY;
      }

      longPressTimer.current = setTimeout(() => {
        if (navigator.vibrate) navigator.vibrate(50);

        // Calculate position to prevent overflowing screen edges
        const menuWidth = 180;
        const menuHeight = 160;
        const x = Math.min(clientX, window.innerWidth - menuWidth - 20);
        const y = Math.min(clientY, window.innerHeight - menuHeight - 20);

        setContextMenu({
          isOpen: true,
          x: Math.max(20, x),
          y: Math.max(20, y),
          chatId,
        });
      }, 500); // 500ms long press
    },
    [],
  );

  const handleTouchEnd = useCallback(() => {
    if (longPressTimer.current) {
      clearTimeout(longPressTimer.current);
      longPressTimer.current = null;
    }
  }, []);

  const handleTouchMove = useCallback(() => {
    if (longPressTimer.current) {
      clearTimeout(longPressTimer.current);
      longPressTimer.current = null;
    }
  }, []);

  const handlePinChat = async (chatId: string, isPinned: boolean) => {
    await ChatService.pinChat(chatId, isPinned);
    loadChats();
    setContextMenu((prev) => ({ ...prev, isOpen: false }));
  };

  const handleMarkAsUnread = async (chatId: string) => {
    await ChatService.markAsUnread(chatId);
    loadChats();
    setContextMenu((prev) => ({ ...prev, isOpen: false }));
  };

  const handleDeleteChat = async (chatId: string) => {
    await ChatService.deleteChat(chatId);
    loadChats();
    setContextMenu((prev) => ({ ...prev, isOpen: false }));
  };

  return (
    <div className="flex flex-col h-full bg-bg-color">
      {/* Header */}
      <header className="h-[56px] px-4 flex items-center justify-between glass-header sticky top-0 z-10 shrink-0 pt-safe relative">
        <div className="absolute left-1/2 -translate-x-1/2 flex items-center justify-center pointer-events-none">
          <h1 className="text-[17px] font-semibold text-text-main">ClawChat</h1>
        </div>
        <div className="flex-1" />
        <div className="flex gap-2 relative z-10" ref={menuRef}>
          <IconButton
            icon={<Search className="w-5 h-5 text-text-main" />}
            className="bg-black/5 dark:bg-white/5 w-8 h-8 p-0"
            onClick={() => navigate("/search")}
          />
          <AddMenu isMenuOpen={isMenuOpen} setIsMenuOpen={setIsMenuOpen} />
        </div>
      </header>

      {/* Chat List */}
      <div className="flex-1 overflow-y-auto pt-1 pb-[84px]">
        {chats.map((chat, index) => {
          const isGroup = chat.type === "group";
          const name = isGroup ? chat.name : chat.participants[0]?.name;
          const avatar = isGroup ? chat.avatar : chat.participants[0]?.avatar;
          const isOnline =
            !isGroup && chat.participants[0]?.status === "online";
          const isPinned = chat.isPinned;

          let timeStr = "";
          if (chat.lastMessage?.timestamp) {
            const date = new Date(chat.lastMessage.timestamp);
            const now = new Date();
            const isToday = date.toDateString() === now.toDateString();
            const isYesterday = new Date(now.getTime() - 86400000).toDateString() === date.toDateString();
            
            if (isToday) {
              timeStr = format(date, "HH:mm");
            } else if (isYesterday) {
              timeStr = "昨天";
            } else if (now.getFullYear() === date.getFullYear()) {
              timeStr = format(date, "MM-dd");
            } else {
              timeStr = format(date, "yyyy-MM-dd");
            }
          }

          return (
            <div
              key={chat.id}
              onClick={() => {
                if (!contextMenu.isOpen) navigate(`/chat/${chat.id}`);
              }}
              onContextMenu={(e) => {
                e.preventDefault();
                handleTouchStart(e, chat.id);
              }}
              onTouchStart={(e) => handleTouchStart(e, chat.id)}
              onTouchEnd={handleTouchEnd}
              onTouchMove={handleTouchMove}
              onMouseDown={(e) => handleTouchStart(e, chat.id)}
              onMouseUp={handleTouchEnd}
              onMouseLeave={handleTouchEnd}
              className={cn(
                "flex px-4 py-3 gap-3 border-b border-border-color transition-all cursor-pointer select-none",
                isPinned ? "bg-black/5 dark:bg-white/5" : "bg-chat-other-bg",
                contextMenu.isOpen && contextMenu.chatId === chat.id
                  ? "bg-active-bg scale-[0.98]"
                  : "active:bg-active-bg",
              )}
            >
              <div className="relative shrink-0">
                <Avatar src={avatar} alt={name} size="lg" />
                {(isOnline || index === 0) && (
                  <div className="absolute -bottom-0.5 -right-0.5 w-3 h-3 bg-accent-green border-2 border-bg-color rounded-full" />
                )}
              </div>

              <div className="flex-1 flex flex-col justify-between min-w-0 py-0.5">
                <div className="flex justify-between items-center">
                  <span className="font-semibold text-[15px] text-text-main truncate">
                    {name}
                  </span>
                  <span className="text-[11px] text-text-sub shrink-0 ml-2">
                    {timeStr}
                  </span>
                </div>
                <div className="flex justify-between items-center mt-1">
                  <span className="text-[13px] text-text-sub truncate">
                    {chat.lastMessage?.content.includes("@我") ? (
                      <>
                        <span className="text-primary-blue">@我</span>
                        {chat.lastMessage.content.replace("@我", "")}
                      </>
                    ) : (
                      chat.lastMessage?.content
                    )}
                  </span>
                  {chat.unreadCount > 0 && (
                    <div className="bg-accent-red text-white text-[11px] font-medium h-[18px] min-w-[18px] px-[5px] rounded-full flex items-center justify-center ml-2 shrink-0 leading-none">
                      {chat.unreadCount > 99 ? "99+" : chat.unreadCount}
                    </div>
                  )}
                </div>
              </div>
            </div>
          );
        })}
      </div>

      <ChatListContextMenu
        contextMenu={contextMenu}
        setContextMenu={setContextMenu}
        chats={chats}
        handlePinChat={handlePinChat}
        handleMarkAsUnread={handleMarkAsUnread}
        handleDeleteChat={handleDeleteChat}
      />
    </div>
  );
};
