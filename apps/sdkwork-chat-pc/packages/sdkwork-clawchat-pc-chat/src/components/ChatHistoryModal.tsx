import React, { useState, useEffect } from 'react';
import { X, Search, Calendar, Image as ImageIcon, Link2, FileText, Music, LayoutList } from 'lucide-react';
import { motion, AnimatePresence } from 'motion/react';
import { Message } from '@sdkwork/clawchat-pc-types';
import { chatService } from '../services/ChatService';
import { cn } from '@sdkwork/clawchat-pc-commons';
import { toast } from './Toast';

interface ChatHistoryModalProps {
  isOpen: boolean;
  onClose: () => void;
  chatId: string;
}

type TabType = 'all' | 'image' | 'link' | 'file' | 'music';

const TABS: { id: TabType; label: string; icon: React.ReactNode }[] = [
  { id: 'all', label: '全部', icon: <LayoutList size={14} /> },
  { id: 'image', label: '图片/视频', icon: <ImageIcon size={14} /> },
  { id: 'link', label: '链接', icon: <Link2 size={14} /> },
  { id: 'file', label: '文件', icon: <FileText size={14} /> },
  { id: 'music', label: '音乐', icon: <Music size={14} /> },
];

export const ChatHistoryModal: React.FC<ChatHistoryModalProps> = ({ isOpen, onClose, chatId }) => {
  const [messages, setMessages] = useState<Message[]>([]);
  const [searchTerm, setSearchTerm] = useState('');
  const [activeTab, setActiveTab] = useState<TabType>('all');

  useEffect(() => {
    if (isOpen) {
      chatService.getMessages(chatId)
        .then(setMessages)
        .catch(() => {
          setMessages([]);
          toast('加载聊天记录失败', 'error');
        });
    }
  }, [isOpen, chatId]);

  const filteredMessages = messages.filter(m => {
    const matchesSearch = (m.content && m.content.toLowerCase().includes(searchTerm.toLowerCase())) || 
                          (m.fileName && m.fileName.toLowerCase().includes(searchTerm.toLowerCase()));
    if (!matchesSearch) return false;
    
    if (activeTab === 'all') return true;
    if (activeTab === 'image') return m.type === 'image' || m.type === 'video';
    if (activeTab === 'link') return m.type === 'link';
    if (activeTab === 'file') return m.type === 'file';
    if (activeTab === 'music') return m.type === 'music';
    
    return true;
  });

  return (
    <AnimatePresence>
      {isOpen && (
        <>
          <motion.div 
            initial={{ opacity: 0 }}
            animate={{ opacity: 1 }}
            exit={{ opacity: 0 }}
            onClick={onClose}
            className="fixed inset-0 bg-black/50 z-40"
          />
          <motion.div 
            initial={{ opacity: 0, scale: 0.95, y: 20 }}
            animate={{ opacity: 1, scale: 1, y: 0 }}
            exit={{ opacity: 0, scale: 0.95, y: 20 }}
            className="fixed left-1/2 top-1/2 -translate-x-1/2 -translate-y-1/2 w-[600px] h-[600px] bg-[#222224] rounded-2xl shadow-2xl z-50 flex flex-col overflow-hidden border border-white/5"
          >
            {/* Header */}
            <div className="h-14 px-5 border-b border-white/5 flex items-center justify-between shrink-0 bg-[#2b2b2d]">
              <h3 className="text-gray-200 font-medium">聊天记录</h3>
              <button 
                onClick={onClose}
                className="w-8 h-8 rounded-full flex items-center justify-center text-gray-400 hover:text-white hover:bg-white/10 transition-colors"
              >
                <X size={18} />
              </button>
            </div>

            {/* Search and Tabs */}
            <div className="p-4 border-b border-white/5 bg-[#222224] shrink-0">
              <div className="relative mb-3 flex items-center gap-2">
                <div className="relative flex-1">
                  <Search size={16} className="absolute left-3 top-1/2 -translate-y-1/2 text-gray-500" />
                  <input 
                    type="text" 
                    value={searchTerm}
                    onChange={e => setSearchTerm(e.target.value)}
                    placeholder="搜索聊天记录..." 
                    className="w-full bg-[#181818] border border-white/10 rounded-lg pl-9 pr-4 py-2 text-sm text-gray-200 outline-none focus:border-indigo-500 transition-colors"
                  />
                </div>
                <button 
                  className="relative w-9 h-9 rounded-lg border border-white/10 flex items-center justify-center text-gray-400 hover:text-white hover:bg-white/5 transition-colors shrink-0 overflow-hidden"
                  title="按日期筛选"
                >
                  <Calendar size={16} className="pointer-events-none" />
                  <input 
                    type="date"
                    className="absolute inset-0 opacity-0 cursor-pointer w-full h-full"
                    onChange={(e) => {
                      if (e.target.value) {
                         setSearchTerm(e.target.value);
                      }
                    }}
                  />
                </button>
              </div>
              <div className="flex items-center gap-2 overflow-x-auto custom-scrollbar pb-1">
                {TABS.map(tab => (
                  <button
                    key={tab.id}
                    onClick={() => setActiveTab(tab.id)}
                    className={cn(
                      "flex items-center gap-1.5 px-3 py-1.5 rounded-full text-[13px] font-medium transition-colors whitespace-nowrap",
                      activeTab === tab.id 
                        ? "bg-indigo-500/20 text-indigo-400 border border-indigo-500/30" 
                        : "bg-white/5 text-gray-400 border border-transparent hover:bg-white/10 hover:text-gray-200"
                    )}
                  >
                    {tab.icon}
                    {tab.label}
                  </button>
                ))}
              </div>
            </div>

            {/* Message List */}
            <div className="flex-1 overflow-y-auto custom-scrollbar p-2">
              {filteredMessages.length === 0 ? (
                <div className="flex flex-col items-center justify-center h-full text-gray-500 text-sm">
                  <Search size={32} className="mb-3 opacity-20" />
                  暂无相关记录
                </div>
              ) : (
                <div className="flex flex-col gap-1">
                  {filteredMessages.map(msg => (
                    <div key={msg.id} className="p-3 hover:bg-white/5 rounded-lg transition-colors flex gap-3 group">
                      <div className="w-10 h-10 rounded-full bg-indigo-500/20 shrink-0 overflow-hidden flex items-center justify-center border border-indigo-500/10">
                        <img 
                          src={`https://api.dicebear.com/7.x/avataaars/svg?seed=${msg.senderId}`} 
                          alt="avatar" 
                          className="w-full h-full object-cover"
                        />
                      </div>
                      <div className="flex-1 min-w-0">
                        <div className="flex items-center justify-between gap-2 mb-1">
                          <span className="text-sm font-medium text-gray-300 truncate">{msg.senderId === 'me' ? '我' : '用户'}</span>
                          <span className="text-[11px] text-gray-500 shrink-0">{new Date(msg.timestamp).toLocaleString(undefined, { month: 'short', day: 'numeric', hour: '2-digit', minute: '2-digit' })}</span>
                        </div>
                        
                        {/* Rendering abstract based on type */}
                        <div className="text-[13px] text-gray-300 break-words leading-relaxed">
                          {msg.type === 'text' && msg.content}
                          {msg.type === 'image' && (
                            <div className="flex items-center gap-2 mt-1">
                               <img src={msg.content} className="h-16 w-auto object-cover rounded-md border border-white/10" referrerPolicy="no-referrer" />
                            </div>
                          )}
                          {msg.type === 'video' && (
                            <div className="flex items-center gap-2 mt-1 text-indigo-400">
                               <ImageIcon size={14} /> [视频消息]
                            </div>
                          )}
                          {msg.type === 'link' && (
                            <a href={msg.content} target="_blank" rel="noreferrer" className="flex flex-col gap-1 mt-1 p-2 bg-white/5 rounded-md border border-white/5 hover:border-indigo-500/30 transition-colors">
                              <span className="font-medium text-indigo-400 truncate">{msg.fileName}</span>
                              <span className="text-xs text-gray-500 line-clamp-1">{msg.desc}</span>
                            </a>
                          )}
                          {msg.type === 'file' && (
                            <div className="flex items-center gap-3 mt-1 p-2 bg-white/5 rounded-md border border-white/5">
                              <div className="w-8 h-8 rounded bg-[#2b2b2d] flex items-center justify-center shrink-0">
                                <FileText size={16} className="text-gray-400" />
                              </div>
                              <div className="min-w-0 flex-1">
                                <div className="truncate font-medium">{msg.fileName}</div>
                                <div className="text-[11px] text-gray-500">{msg.fileSize}</div>
                              </div>
                            </div>
                          )}
                          {msg.type === 'music' && (
                            <div className="flex items-center gap-3 mt-1 p-2 bg-white/5 rounded-md border border-white/5">
                              <img src={msg.coverUrl} className="w-8 h-8 rounded object-cover shrink-0" />
                              <div className="min-w-0 flex-1">
                                <div className="truncate font-medium">{msg.fileName}</div>
                                <div className="text-[11px] text-gray-500 flex items-center gap-1"><Music size={10} /> {msg.desc}</div>
                              </div>
                            </div>
                          )}
                          {['voice', 'video_call', 'applet', 'card'].includes(msg.type!) && (
                            <span className="text-gray-500 italic">[{msg.type === 'voice' ? '语音消息' : msg.type === 'video_call' ? '通话记录' : msg.type === 'applet' ? '小程序' : '名片'}]</span>
                          )}
                        </div>
                      </div>
                    </div>
                  ))}
                </div>
              )}
            </div>
          </motion.div>
        </>
      )}
    </AnimatePresence>
  );
};
