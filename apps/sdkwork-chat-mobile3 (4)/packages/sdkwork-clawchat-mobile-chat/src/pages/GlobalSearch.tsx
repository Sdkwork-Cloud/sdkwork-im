import React, { useState, useRef, useEffect } from "react";
import { useNavigate } from "react-router";
import { Search, X, MessageSquare, User } from "lucide-react";
import { ChatService } from "../services/ChatService";
import { ContactService } from "@sdkwork/clawchat-mobile-contacts";
import { Avatar } from "@sdkwork/clawchat-mobile-commons";
import type { Chat, User as UserType } from "@sdkwork/clawchat-mobile-types";

export const GlobalSearch: React.FC = () => {
  const navigate = useNavigate();
  const [query, setQuery] = useState("");
  const inputRef = useRef<HTMLInputElement>(null);

  const [chats, setChats] = useState<Chat[]>([]);
  const [contacts, setContacts] = useState<UserType[]>([]);
  const [isSearching, setIsSearching] = useState(false);

  useEffect(() => {
    inputRef.current?.focus();
  }, []);

  useEffect(() => {
    const doSearch = async () => {
      if (!query.trim()) {
        setChats([]);
        setContacts([]);
        return;
      }
      setIsSearching(true);
      const searchChatsPromise = ChatService.searchChats(query);
      const searchContactsPromise = ContactService.searchContacts(query);

      const [searchedChats, searchedContacts] = await Promise.all([
        searchChatsPromise,
        searchContactsPromise,
      ]);

      setChats(searchedChats);
      setContacts(searchedContacts);

      setIsSearching(false);
    };

    const timer = setTimeout(doSearch, 300);
    return () => clearTimeout(timer);
  }, [query]);

  return (
    <div className="flex flex-col h-full bg-bg-color">
      {/* Header */}
      <header className="h-[56px] flex items-center px-3 glass-header sticky top-0 z-10 shrink-0 pt-safe gap-3">
        <div className="flex-1 flex items-center bg-chat-other-bg rounded-lg h-9 px-2.5 border border-border-color transition-colors focus-within:border-primary-blue focus-within:bg-bg-color">
          <Search className="w-4 h-4 text-text-sub shrink-0" />
          <input
            ref={inputRef}
            type="text"
            value={query}
            onChange={(e) => setQuery(e.target.value)}
            placeholder="搜索"
            className="flex-1 bg-transparent border-none outline-none px-2 text-[16px] text-text-main placeholder:text-text-sub min-w-0"
          />
          {query && (
            <div
              onClick={() => setQuery("")}
              className="p-1 cursor-pointer shrink-0"
            >
              <X className="w-3.5 h-3.5 text-white bg-black/20 dark:bg-white/20 rounded-full p-0.5" />
            </div>
          )}
        </div>
        <button
          onClick={() => navigate(-1)}
          className="text-[16px] text-text-main font-medium whitespace-nowrap shrink-0 active:opacity-70"
        >
          取消
        </button>
      </header>

      {/* Content */}
      <div className="flex-1 overflow-y-auto">
        {!query ? (
          <div className="p-6">
            <h3 className="text-center text-[13px] text-text-sub mb-6">
              搜索指定内容
            </h3>
            <div className="grid grid-cols-3 gap-y-6 text-center">
              <span className="text-[15px] text-primary-blue active:opacity-70 cursor-pointer">
                朋友圈
              </span>
              <span className="text-[15px] text-primary-blue active:opacity-70 cursor-pointer border-x border-border-color">
                文章
              </span>
              <span className="text-[15px] text-primary-blue active:opacity-70 cursor-pointer">
                公众号
              </span>
              <span className="text-[15px] text-primary-blue active:opacity-70 cursor-pointer">
                小程序
              </span>
              <span className="text-[15px] text-primary-blue active:opacity-70 cursor-pointer border-x border-border-color">
                音乐
              </span>
              <span className="text-[15px] text-primary-blue active:opacity-70 cursor-pointer">
                表情
              </span>
            </div>
          </div>
        ) : (
          <div className="p-4">
            {isSearching ? (
              <div className="flex justify-center p-4">
                <div className="w-6 h-6 border-2 border-primary-blue border-t-transparent rounded-full animate-spin"></div>
              </div>
            ) : (
              <div className="space-y-6">
                {contacts.length > 0 && (
                  <div>
                    <h3 className="text-[14px] text-text-sub mb-3 px-2 flex items-center gap-2">
                      <User className="w-4 h-4" /> 联系人
                    </h3>
                    <div className="bg-bg-color rounded-xl overflow-hidden">
                      {contacts.map((contact, i) => (
                        <div
                          key={contact.id}
                          className="flex items-center gap-3 p-3 active:bg-active-bg cursor-pointer"
                          onClick={() =>
                            navigate("/workspace/contacts", {
                              state: { searchUser: contact.id },
                            })
                          }
                        >
                          <Avatar
                            src={contact.avatar}
                            fallback={contact.name[0]}
                          />
                          <span className="text-[16px] text-text-main font-medium">
                            {contact.name}
                          </span>
                        </div>
                      ))}
                    </div>
                  </div>
                )}

                {chats.length > 0 && (
                  <div>
                    <h3 className="text-[14px] text-text-sub mb-3 px-2 flex items-center gap-2">
                      <MessageSquare className="w-4 h-4" /> 聊天记录
                    </h3>
                    <div className="bg-bg-color rounded-xl overflow-hidden">
                      {chats.map((chat) => {
                        const chatName =
                          chat.type === "group"
                            ? chat.name
                            : chat.participants[0]?.name;
                        const avatar =
                          chat.type === "group"
                            ? chat.avatar
                            : chat.participants[0]?.avatar;
                        return (
                          <div
                            key={chat.id}
                            className="flex items-center gap-3 p-3 active:bg-active-bg cursor-pointer"
                            onClick={() => navigate(`/chat/${chat.id}`)}
                          >
                            <Avatar src={avatar} fallback={chatName?.[0]} />
                            <span className="text-[16px] text-text-main font-medium">
                              {chatName}
                            </span>
                          </div>
                        );
                      })}
                    </div>
                  </div>
                )}

                {chats.length === 0 && contacts.length === 0 && (
                  <div className="flex flex-col items-center justify-center py-10 text-text-sub">
                    <Search className="w-10 h-10 mb-3 opacity-20" />
                    <p className="text-[15px]">未找到关于 "{query}" 的结果</p>
                  </div>
                )}
              </div>
            )}
          </div>
        )}
      </div>
    </div>
  );
};
