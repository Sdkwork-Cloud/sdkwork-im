import React, { useState, useEffect } from "react";
import { useNavigate } from "react-router";
import { ChevronLeft, Search, Check } from "lucide-react";
import { Avatar, IconButton, cn } from "@sdkwork/clawchat-mobile-commons";
import { ContactService } from "@sdkwork/clawchat-mobile-contacts";
import { ChatService } from "../services/ChatService";
import type { User } from "@sdkwork/clawchat-mobile-types";

import { showToast } from "@sdkwork/clawchat-mobile-commons";

export const CreateGroupChat: React.FC = () => {
  const navigate = useNavigate();
  const [searchQuery, setSearchQuery] = useState("");
  const [selectedIds, setSelectedIds] = useState<Set<string>>(new Set());
  const [contacts, setContacts] = useState<User[]>([]);
  const [isCreating, setIsCreating] = useState(false);

  useEffect(() => {
    ContactService.getContacts().then(setContacts);
  }, []);

  const handleCreate = async () => {
    if (selectedIds.size === 0 || isCreating) return;
    setIsCreating(true);
    try {
      const chat = await ChatService.createGroupChat(
        "群聊",
        Array.from(selectedIds),
      );
      showToast("群聊创建成功");
      navigate(`/chat/${chat.id}`, { replace: true });
    } catch (error) {
      console.error(error);
      showToast("创建失败，请重试");
      setIsCreating(false);
    }
  };

  const toggleSelection = (id: string) => {
    const newSet = new Set(selectedIds);
    if (newSet.has(id)) {
      newSet.delete(id);
    } else {
      newSet.add(id);
    }
    setSelectedIds(newSet);
  };

  const filteredContacts = contacts.filter((c) =>
    c.name.toLowerCase().includes(searchQuery.toLowerCase()),
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
          <h2 className="text-[17px] font-medium text-text-main">发起群聊</h2>
        </div>
        <div className="flex items-center justify-end z-10 flex-1 pr-3">
          <button
            onClick={handleCreate}
            disabled={selectedIds.size === 0 || isCreating}
            className={cn(
              "px-3 py-1.5 rounded-md text-[14px] font-medium transition-colors",
              selectedIds.size > 0 && !isCreating
                ? "bg-primary-blue text-white active:bg-blue-600"
                : "bg-black/5 dark:bg-white/5 text-text-sub cursor-not-allowed",
            )}
          >
            {isCreating
              ? "创建中..."
              : `完成 ${selectedIds.size > 0 ? `(${selectedIds.size})` : ""}`}
          </button>
        </div>
      </header>

      {/* Selected Contacts Horizontal Scroll */}
      {selectedIds.size > 0 && (
        <div className="flex gap-3 px-4 py-3 overflow-x-auto no-scrollbar border-b border-border-color bg-chat-other-bg shrink-0">
          {Array.from(selectedIds).map((id: string) => {
            const contact = contacts.find((c) => c.id === id);
            if (!contact) return null;
            return (
              <div
                key={id}
                className="relative shrink-0 animate-in fade-in zoom-in duration-200"
                onClick={() => toggleSelection(id)}
              >
                <Avatar src={contact.avatar} size="md" className="w-12 h-12" />
                <div className="absolute -top-1 -right-1 w-4 h-4 bg-bg-color rounded-full flex items-center justify-center border border-border-color shadow-sm">
                  <div className="w-2.5 h-[1.5px] bg-text-sub rotate-45 absolute" />
                  <div className="w-2.5 h-[1.5px] bg-text-sub -rotate-45 absolute" />
                </div>
              </div>
            );
          })}
        </div>
      )}

      {/* Search Bar */}
      <div className="px-4 py-2 bg-bg-color sticky top-[56px] z-10">
        <div className="flex items-center gap-2 bg-chat-other-bg rounded-xl px-3 py-2 border border-border-color focus-within:border-primary-blue transition-colors">
          <Search className="w-4 h-4 text-text-sub shrink-0" />
          <input
            type="text"
            placeholder="搜索联系人"
            className="flex-1 bg-transparent text-[15px] text-text-main focus:outline-none placeholder:text-text-sub"
            value={searchQuery}
            onChange={(e) => setSearchQuery(e.target.value)}
          />
        </div>
      </div>

      {/* Contact List */}
      <div className="flex flex-col pb-8">
        {filteredContacts.map((contact) => {
          const isSelected = selectedIds.has(contact.id);
          return (
            <div
              key={contact.id}
              onClick={() => toggleSelection(contact.id)}
              className="flex items-center gap-3 px-4 py-2.5 active:bg-active-bg transition-colors cursor-pointer"
            >
              <div
                className={cn(
                  "w-5 h-5 rounded-full border flex items-center justify-center shrink-0 transition-colors",
                  isSelected
                    ? "bg-primary-blue border-primary-blue"
                    : "border-text-sub/50",
                )}
              >
                {isSelected && (
                  <Check className="w-3.5 h-3.5 text-white" strokeWidth={3} />
                )}
              </div>
              <Avatar src={contact.avatar} size="md" />
              <span className="text-[16px] text-text-main font-medium border-b border-border-color flex-1 py-3">
                {contact.name}
              </span>
            </div>
          );
        })}
      </div>
    </div>
  );
};
