import React, { useState, useMemo, useEffect } from 'react';
import { motion, AnimatePresence } from 'motion/react';
import { Chat } from '@sdkwork/clawchat-pc-types';
import { Avatar } from '@sdkwork/clawchat-pc-commons';
import { cn } from '@sdkwork/clawchat-pc-commons';
import { ContextMenu, ContextMenuItem } from './ContextMenu';
import { Pin, BellOff, Trash2, CheckCircle, MessageCircle } from 'lucide-react';
import { toast } from './Toast';
import { chatService } from '../services/ChatService';

interface ChatListProps {
  chats: Chat[];
  activeChatId?: string;
  onChatSelect: (chat: Chat) => void;
  onChatsChange?: () => void;
  searchQuery?: string;
}

export const ChatList: React.FC<ChatListProps> = ({ chats, activeChatId, onChatSelect, onChatsChange, searchQuery = '' }) => {
  const [contextMenu, setContextMenu] = useState<{x: number, y: number, chat: Chat} | null>(null);

  const handleContextMenu = (e: React.MouseEvent, chat: Chat) => {
    e.preventDefault();
    setContextMenu({ x: e.clientX, y: e.clientY, chat });
  };

  const getContextMenuItems = (): ContextMenuItem[] => {
    if (!contextMenu) return [];
    const chat = contextMenu.chat;
    const isPinned = !!chat.isPinned;
    const isUnread = chat.unreadCount > 0 || !!chat.isMarkedUnread;
    const isMuted = !!chat.isMuted;

    return [
      { 
        id: 'pin', 
        label: isPinned ? '取消置顶' : '置顶会话', 
        icon: <Pin size={14} className={isPinned ? "rotate-45" : ""} />, 
        onClick: async () => {
          try {
            await chatService.pinChat(chat.id, !isPinned);
            if (onChatsChange) onChatsChange();
            toast(isPinned ? '已取消置顶' : '已置顶', 'success');
          } catch {
            toast('会话操作失败', 'error');
          }
        } 
      },
      { 
        id: 'read', 
        label: isUnread ? '标为已读' : '标为未读', 
        icon: isUnread ? <CheckCircle size={14} /> : <MessageCircle size={14} />, 
        onClick: async () => {
          try {
            if (isUnread) {
              await chatService.markAsRead(chat.id);
            } else {
              await chatService.markAsUnread(chat.id);
            }
            if (onChatsChange) onChatsChange();
            toast(isUnread ? '已标为已读' : '已标为未读', 'success');
          } catch {
            toast('会话操作失败', 'error');
          }
        } 
      },
      { 
        id: 'mute', 
        label: isMuted ? '取消免打扰' : '消息免打扰', 
        icon: <BellOff size={14} />, 
        onClick: async () => {
          try {
            await chatService.muteChat(chat.id, !isMuted);
            if (onChatsChange) onChatsChange();
            toast(isMuted ? '已取消免打扰' : '已开启免打扰', 'success');
          } catch {
            toast('会话操作失败', 'error');
          }
        } 
      },
      { id: 'div1', label: '', divider: true, onClick: () => {} },
      { 
        id: 'delete', 
        label: '删除会话', 
        icon: <Trash2 size={14} />, 
        danger: true, 
        onClick: async () => {
          try {
            await chatService.deleteChat(chat.id);
            if (onChatsChange) onChatsChange();
            toast('会话已删除', 'success');
          } catch {
            toast('会话操作失败', 'error');
          }
        } 
      },
    ];
  };

  const formatTime = (timestamp: number) => {
    const date = new Date(timestamp);
    const now = new Date();
    
    if (date.toDateString() === now.toDateString()) {
      return `${date.getHours().toString().padStart(2, '0')}:${date.getMinutes().toString().padStart(2, '0')}`;
    }
    
    const yesterday = new Date(now);
    yesterday.setDate(yesterday.getDate() - 1);
    if (date.toDateString() === yesterday.toDateString()) {
      return '昨天';
    }
    
    const days = ['周日', '周一', '周二', '周三', '周四', '周五', '周六'];
    return days[date.getDay()];
  };

  const sortedChats = useMemo(() => {
    return [...chats]
      .filter(chat => {
        if (!searchQuery.trim()) return true;
        const query = searchQuery.toLowerCase();
        return chat.name.toLowerCase().includes(query) || (chat.lastMessage?.content && typeof chat.lastMessage.content === 'string' && chat.lastMessage.content.toLowerCase().includes(query));
      })
      .sort((a, b) => {
        const aPinned = !!a.isPinned;
        const bPinned = !!b.isPinned;
        if (aPinned && !bPinned) return -1;
        if (!aPinned && bPinned) return 1;
        return b.updatedAt - a.updatedAt;
      });
  }, [chats, searchQuery]);

  return (
    <div className="flex w-[280px] shrink-0 flex-col bg-[#202020] border-r border-white/5 min-h-0">
      <div className="flex-1 overflow-y-auto custom-scrollbar relative">
        <AnimatePresence>
        {sortedChats.length === 0 ? (
          <motion.div 
            initial={{ opacity: 0 }}
            animate={{ opacity: 1 }}
            className="absolute inset-0 flex flex-col items-center justify-center text-center p-6"
          >
            <div className="w-16 h-16 rounded-full bg-white/5 flex items-center justify-center mb-3">
              <MessageCircle size={28} className="text-gray-500" />
            </div>
            <p className="text-sm font-medium text-gray-400 mb-1">
              {searchQuery ? '没有找到匹配的会话' : '暂无消息'}
            </p>
            <p className="text-[12px] text-gray-500">
              {searchQuery ? '换个关键词试试' : '去通讯录找人聊聊吧'}
            </p>
          </motion.div>
        ) : (
          sortedChats.map((chat) => {
          const isPinned = !!chat.isPinned;
          const isUnread = chat.unreadCount > 0 || !!chat.isMarkedUnread;
          const isMuted = !!chat.isMuted;

          return (
            <motion.div 
              layout
              initial={{ opacity: 0, y: 10 }}
              animate={{ opacity: 1, y: 0 }}
              exit={{ opacity: 0, scale: 0.9 }}
              transition={{ duration: 0.2 }}
              key={chat.id}
              onClick={() => {
                onChatSelect(chat);
                if (isUnread) {
                  void chatService.markAsRead(chat.id).then(() => {
                    if (onChatsChange) onChatsChange();
                  }).catch(() => toast('标记已读失败', 'error'));
                }
              }}
              onContextMenu={(e) => handleContextMenu(e, chat)}
              className={cn(
                "flex items-center px-3 py-3 cursor-pointer transition-colors hover:bg-white/5 relative",
                activeChatId === chat.id && "bg-white/10 hover:bg-white/10",
                isPinned && activeChatId !== chat.id && "bg-[#2b2b2d] hover:bg-[#323234]"
              )}
            >
              <div className="relative shrink-0 mr-3">
                <Avatar src={chat.avatar} alt={chat.name} className="w-[40px] h-[40px] rounded bg-[#2b2b2d] text-white font-bold" />
                {isUnread && (
                  <div className={cn(
                    "absolute -top-1 -right-1 rounded-full border-2 border-[#202020]",
                    isMuted ? "w-2.5 h-2.5 bg-red-500" : "px-1.5 min-w-[18px] h-[18px] bg-red-500 text-white text-[10px] font-bold flex items-center justify-center"
                  )}>
                    {!isMuted && (chat.unreadCount || 1)}
                  </div>
                )}
              </div>
              <div className="flex-1 min-w-0 flex flex-col justify-center">
                <div className="flex items-center justify-between mb-1">
                  <div className="flex items-center gap-1 min-w-0">
                    <span className="text-[14px] text-gray-200 truncate">{chat.name}</span>
                    {isMuted && <BellOff size={12} className="text-gray-500 shrink-0" />}
                  </div>
                  <span className="text-[12px] text-gray-500 shrink-0 ml-2">{formatTime(chat.updatedAt)}</span>
                </div>
                <div className="text-[12px] text-gray-500 truncate">
                  {chat.lastMessage?.content}
                </div>
              </div>
            </motion.div>
          );
        }))}
        </AnimatePresence>
      </div>
      {contextMenu && (
        <ContextMenu 
          x={contextMenu.x} 
          y={contextMenu.y} 
          items={getContextMenuItems()} 
          onClose={() => setContextMenu(null)} 
        />
      )}
    </div>
  );
};
