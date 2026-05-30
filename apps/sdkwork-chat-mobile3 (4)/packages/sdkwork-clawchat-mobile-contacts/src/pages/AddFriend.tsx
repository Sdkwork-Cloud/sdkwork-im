import React, { useState } from "react";
import { useNavigate } from "react-router";
import {
  ChevronLeft,
  Search,
  QrCode,
  UserPlus,
  Smartphone,
  ChevronRight,
  User,
} from "lucide-react";
import {
  IconButton,
  Avatar,
  cn,
  showToast,
} from "@sdkwork/clawchat-mobile-commons";
import { ContactService } from "../services/ContactService";

export const AddFriend: React.FC = () => {
  const navigate = useNavigate();
  const [searchQuery, setSearchQuery] = useState("");
  const [isSearching, setIsSearching] = useState(false);
  const [searchResult, setSearchResult] = useState<any>(null);
  const [isAdding, setIsAdding] = useState(false);

  const handleSearch = async () => {
    if (!searchQuery.trim()) return;
    setIsSearching(true);
    const result = await ContactService.searchFriend(searchQuery);
    setSearchResult(result);
    setIsSearching(false);
  };

  const handleAddFriend = async () => {
    if (!searchResult || isAdding) return;
    setIsAdding(true);
    try {
      await ContactService.addFriend(searchResult.name);
      // Navigate back to contacts or show success
      navigate("/workspace/contacts", { replace: true });
    } catch (e) {
      console.error(e);
      showToast("添加失败");
      setIsAdding(false);
    }
  };

  const ListItem = ({
    icon: Icon,
    title,
    subtitle,
    colorClass,
  }: {
    icon: React.ElementType;
    title: string;
    subtitle?: string;
    colorClass?: string;
  }) => (
    <div className="flex items-center gap-4 px-4 py-3.5 bg-chat-other-bg active:bg-active-bg transition-colors cursor-pointer border-b border-border-color last:border-none">
      <div
        className={cn(
          "w-10 h-10 rounded-lg flex items-center justify-center shrink-0",
          colorClass,
        )}
      >
        <Icon className="w-6 h-6 text-white" />
      </div>
      <div className="flex flex-col flex-1 min-w-0">
        <span className="text-[16px] font-medium text-text-main">{title}</span>
        <span className="text-[13px] text-text-sub truncate">{subtitle}</span>
      </div>
      <ChevronRight className="w-5 h-5 text-text-sub opacity-50" />
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
          <h2 className="text-[17px] font-medium text-text-main">添加朋友</h2>
        </div>
        <div className="flex-1" />
      </header>

      <div className="flex flex-col px-0 sm:px-4 pb-8">
        {/* Search Bar */}
        <div className="px-4 py-3 sm:px-0">
          <div className="flex items-center gap-2 bg-chat-other-bg rounded-xl px-3 py-2.5 border border-border-color focus-within:border-primary-blue transition-colors">
            <Search className="w-5 h-5 text-text-sub shrink-0" />
            <input
              type="text"
              placeholder="微信号/手机号"
              className="flex-1 bg-transparent text-[16px] text-text-main focus:outline-none placeholder:text-text-sub"
              value={searchQuery}
              onChange={(e) => {
                setSearchQuery(e.target.value);
                setSearchResult(null);
              }}
              onKeyDown={(e) => {
                if (e.key === "Enter") {
                  handleSearch();
                }
              }}
            />
            {searchQuery && (
              <button
                onClick={handleSearch}
                className="text-primary-blue text-[14px] font-medium px-2"
              >
                搜索
              </button>
            )}
          </div>

          <div className="flex items-center justify-center gap-2 mt-4 text-[14px] text-text-sub">
            <span>我的微信号：wxid_123456789</span>
            <QrCode className="w-4 h-4 text-primary-blue cursor-pointer active:opacity-70" />
          </div>
        </div>

        {/* Search Result */}
        {isSearching && (
          <div className="px-4 py-8 flex justify-center">
            <div className="w-6 h-6 border-2 border-primary-blue border-t-transparent rounded-full animate-spin" />
          </div>
        )}

        {searchResult && !isSearching && (
          <div className="mt-4 sm:rounded-xl overflow-hidden border-y sm:border border-border-color bg-chat-other-bg flex flex-col">
            <div className="flex items-center gap-4 px-4 py-4 border-b border-border-color">
              <Avatar src={searchResult.avatar} size="lg" />
              <div className="flex flex-col flex-1 min-w-0">
                <span className="text-[16px] font-medium text-text-main">
                  {searchResult.name}
                </span>
                <span className="text-[13px] text-text-sub truncate">
                  微信号: {searchQuery}
                </span>
              </div>
            </div>
            <div
              onClick={handleAddFriend}
              className="px-4 py-3.5 flex items-center justify-center text-primary-blue font-medium text-[16px] active:bg-active-bg transition-colors cursor-pointer"
            >
              {isAdding ? "添加中..." : "添加到通讯录"}
            </div>
          </div>
        )}

        {/* Options */}
        {!searchResult && !isSearching && (
          <div className="mt-4 sm:rounded-xl overflow-hidden border-y sm:border border-border-color flex flex-col">
            <ListItem
              icon={QrCode}
              title="扫一扫"
              subtitle="扫描二维码名片"
              colorClass="bg-[#2B5CE7]"
            />
            <ListItem
              icon={Smartphone}
              title="手机联系人"
              subtitle="添加或邀请通讯录中的朋友"
              colorClass="bg-[#00B42A]"
            />
            <ListItem
              icon={UserPlus}
              title="企业微信联系人"
              subtitle="通过手机号搜索企业微信用户"
              colorClass="bg-[#FF7D00]"
            />
          </div>
        )}
      </div>
    </div>
  );
};
