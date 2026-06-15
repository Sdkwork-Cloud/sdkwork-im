import React, { useState, useEffect } from 'react';
import { Search, Check } from 'lucide-react';
import { Avatar } from '@sdkwork/im-pc-commons';
import { toast } from './Toast';
import { chatService } from '../services/ChatService';
import { ModalWrapper } from './ModalWrapper';
import type { Chat, Message } from '@sdkwork/im-pc-types';

export const ForwardModal: React.FC<{ isOpen: boolean; onClose: () => void; messages: Message[] }> = ({ isOpen, onClose, messages }) => {
  const [selected, setSelected] = useState<Set<string>>(new Set());
  const [searchQuery, setSearchQuery] = useState('');
  const [chats, setChats] = useState<Chat[]>([]);
  const [loading, setLoading] = useState(false);
  const [forwarding, setForwarding] = useState(false);

  useEffect(() => {
    if (isOpen) {
      setLoading(true);
      chatService.getChats()
        .then(data => {
          setChats(data);
        })
        .catch(() => {
          setChats([]);
          toast('加载会话失败', 'error');
        })
        .finally(() => setLoading(false));
    } else {
      setSelected(new Set());
      setSearchQuery('');
    }
  }, [isOpen]);

  const toggleSelect = (id: string) => {
    const next = new Set(selected);
    if (next.has(id)) next.delete(id);
    else next.add(id);
    setSelected(next);
  };

  const filteredChats = chats.filter(c => c.name.toLowerCase().includes(searchQuery.toLowerCase()));

  const handleForward = async () => {
    if (selected.size === 0 || messages.length === 0) return;
    setForwarding(true);
    try {
      await chatService.forwardMessages(Array.from(selected), messages);
      toast(`成功转发给 ${selected.size} 个会话`, 'success');
      onClose();
    } catch (error) {
      toast('转发失败', 'error');
    } finally {
      setForwarding(false);
    }
  };

  return (
    <ModalWrapper 
      isOpen={isOpen} 
      onClose={onClose} 
      title={`转发 ${messages.length} 条消息`} 
      width="w-[480px]"
      footer={
        <>
          <button onClick={onClose} className="px-4 py-2 rounded bg-white/5 text-gray-300 hover:bg-white/10 transition-colors text-sm">取消</button>
          <button 
            disabled={selected.size === 0 || forwarding}
            className={`px-4 py-2 rounded text-white transition-colors text-sm ${selected.size > 0 && !forwarding ? 'bg-[#00b42a] hover:bg-[#009a24]' : 'bg-[#00b42a]/50 cursor-not-allowed'}`}
            onClick={handleForward}
          >
            {forwarding ? '转发中...' : `发送 (${selected.size})`}
          </button>
        </>
      }
    >
      <div className="relative mb-4">
        <Search size={16} className="absolute left-3 top-1/2 -translate-y-1/2 text-gray-500" />
        <input 
          type="text" 
          placeholder="搜索会话" 
          value={searchQuery}
          onChange={e => setSearchQuery(e.target.value)}
          className="w-full bg-[#181818] border border-white/5 rounded-lg py-2 pl-9 pr-4 text-sm text-gray-200 outline-none focus:border-white/20 transition-colors" 
        />
      </div>
      <div className="space-y-1">
        {loading ? (
          <div className="text-center py-8 text-gray-500 text-sm">加载中...</div>
        ) : filteredChats.length > 0 ? (
          filteredChats.map(chat => (
            <div 
              key={chat.id} 
              className="flex items-center gap-3 p-2 rounded-lg hover:bg-white/5 cursor-pointer transition-colors"
              onClick={() => toggleSelect(chat.id)}
            >
              <div className={`w-5 h-5 rounded-full border flex items-center justify-center transition-colors ${selected.has(chat.id) ? 'bg-[#00b42a] border-[#00b42a]' : 'border-gray-500'}`}>
                {selected.has(chat.id) && <Check size={12} className="text-white" />}
              </div>
              <Avatar src={chat.avatar} alt={chat.name} className="w-8 h-8 rounded bg-[#2b2b2d]" />
              <span className="text-gray-200 text-sm">{chat.name}</span>
            </div>
          ))
        ) : (
          <div className="text-center py-8 text-gray-500 text-sm">未找到会话</div>
        )}
      </div>
    </ModalWrapper>
  );
};
