import { useNavigate } from "react-router";
import { useParams } from "react-router";
import React, { useState, useEffect } from "react";
import {} from "react-router";
import {
  ChevronLeft,
  Search,
  Bell,
  Pin,
  Image as ImageIcon,
  Trash2,
  Plus,
  ChevronRight,
  Settings2,
  EyeOff,
  X,
} from "lucide-react";
import {
  Avatar,
  IconButton,
  cn,
  showConfirm,
  ListItem,
  Switch,
} from "@sdkwork/clawchat-mobile-commons";
import { ChatService } from "../services/ChatService";
import type { Chat, Message } from "@sdkwork/clawchat-mobile-types";
import { SearchHistoryOverlay } from "../components/Chat/SearchHistoryOverlay";

export const ChatProfile: React.FC = () => {
  const { id } = useParams();
  const navigate = useNavigate();
  const [chat, setChat] = useState<Chat | null>(null);
  const [isMuted, setIsMuted] = useState(false);
  const [isPinned, setIsPinned] = useState(false);
  const [showAvatar, setShowAvatar] = useState(true);
  const [cleanMode, setCleanMode] = useState(false);

  // Search state
  const [showSearch, setShowSearch] = useState(false);
  const [searchQuery, setSearchQuery] = useState("");
  const [searchResults, setSearchResults] = useState<Message[]>([]);
  const [isSearching, setIsSearching] = useState(false);

  useEffect(() => {
    if (id) {
      ChatService.getChatById(id).then((c) => {
        if (c) {
          setChat(c);
          setShowAvatar(c.settings?.showAvatar ?? true);
          setCleanMode(c.settings?.cleanMode ?? false);
        }
      });
    }
  }, [id]);

  useEffect(() => {
    if (showSearch && searchQuery.trim() && id) {
      setIsSearching(true);
      const timer = setTimeout(async () => {
        const results = await ChatService.searchChatHistory(id, searchQuery);
        setSearchResults(results);
        setIsSearching(false);
      }, 300);
      return () => clearTimeout(timer);
    } else {
      setSearchResults([]);
    }
  }, [searchQuery, showSearch, id]);

  const handleUpdateSettings = async (updates: Partial<Chat["settings"]>) => {
    if (id) {
      await ChatService.updateChatSettings(id, updates);
    }
  };

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
          <h2 className="text-[17px] font-medium text-text-main">聊天信息</h2>
        </div>
        <div className="flex-1" />
      </header>

      <div className="flex flex-col px-0 sm:px-4 pb-8">
        {/* Members */}
        <div className="bg-chat-other-bg px-4 py-6 mb-2 sm:mb-4 sm:rounded-xl sm:mt-2 border-y sm:border border-border-color">
          <div className="flex flex-wrap gap-5">
            <div className="flex flex-col items-center gap-2 w-[52px]">
              <Avatar
                src={chat ? chat.avatar : "https://picsum.photos/seed/sarah/200/200"}
                size="lg"
                className="w-[52px] h-[52px] rounded-xl"
              />
              <span className="text-[12px] text-text-sub truncate w-full text-center">
                {chat ? chat.name : "加载中..."}
              </span>
            </div>
            <div className="flex flex-col items-center gap-2 w-[52px]">
              <div className="w-[52px] h-[52px] rounded-xl border border-dashed border-border-color flex items-center justify-center text-text-sub active:bg-active-bg cursor-pointer transition-colors">
                <Plus className="w-6 h-6" />
              </div>
              <span className="text-[12px] text-text-sub truncate w-full text-center">
                添加
              </span>
            </div>
          </div>
        </div>

        {/* Settings Group 1 */}
        <div className="mb-2 sm:mb-4 sm:rounded-xl overflow-hidden border-y sm:border border-border-color flex flex-col">
          <ListItem
            icon={Search}
            label="查找聊天记录"
            onClick={() => setShowSearch(true)}
          />
        </div>

        {/* Settings Group 2 */}
        <div className="mb-2 sm:mb-4 sm:rounded-xl overflow-hidden border-y sm:border border-border-color flex flex-col">
          <ListItem
            icon={Bell}
            label="消息免打扰"
            rightElement={<Switch checked={isMuted} onChange={setIsMuted} />}
          />
          <ListItem
            icon={Pin}
            label="置顶聊天"
            rightElement={<Switch checked={isPinned} onChange={setIsPinned} />}
          />
        </div>

        {/* Settings Group 3 (Display Settings) */}
        <div className="mb-2 sm:mb-4 sm:rounded-xl overflow-hidden border-y sm:border border-border-color flex flex-col">
          <ListItem
            icon={Settings2}
            label="显示头像"
            rightElement={
              <Switch
                checked={showAvatar}
                onChange={(val: boolean) => {
                  setShowAvatar(val);
                  handleUpdateSettings({ showAvatar: val });
                }}
              />
            }
          />
          <ListItem
            icon={EyeOff}
            label="清爽模式"
            rightElement={
              <Switch
                checked={cleanMode}
                onChange={(val: boolean) => {
                  setCleanMode(val);
                  handleUpdateSettings({ cleanMode: val });
                }}
              />
            }
          />
        </div>

        {/* Settings Group 4 */}
        <div className="mb-6 sm:mb-8 sm:rounded-xl overflow-hidden border-y sm:border border-border-color flex flex-col">
          <ListItem icon={ImageIcon} label="设置当前聊天背景" />
        </div>

        {/* Danger Zone */}
        <div className="sm:rounded-xl overflow-hidden border-y sm:border border-border-color flex flex-col">
          <ListItem
            icon={Trash2}
            label="清空聊天记录"
            danger={true}
            rightElement={<div />}
            onClick={async () => {
              if (await showConfirm("确定要清空聊天记录吗？")) {
                ChatService.clearChatHistory(id as string).then(() => {
                  navigate(`/chat/${id}`, { replace: true });
                });
              }
            }}
          />
        </div>
      </div>

      <SearchHistoryOverlay
        id={id as string}
        chat={chat}
        showSearch={showSearch}
        setShowSearch={setShowSearch}
        searchQuery={searchQuery}
        setSearchQuery={setSearchQuery}
        isSearching={isSearching}
        searchResults={searchResults}
      />
    </div>
  );
};
