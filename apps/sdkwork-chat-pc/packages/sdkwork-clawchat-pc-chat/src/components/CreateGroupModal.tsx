import React, { useState, useEffect } from 'react';
import { Search, Check } from 'lucide-react';
import { Avatar } from '@sdkwork/clawchat-pc-commons';
import { toast } from './Toast';
import { contactService } from '../services/ContactService';
import { groupService } from '../services/GroupService';
import { ModalWrapper } from './ModalWrapper';
import type { Chat, User } from '@sdkwork/clawchat-pc-types';

export const CreateGroupModal: React.FC<{
  isOpen: boolean;
  onClose: () => void;
  onCreated?: (group: Chat) => void | Promise<void>;
}> = ({ isOpen, onClose, onCreated }) => {
  const [selected, setSelected] = useState<Set<string>>(new Set());
  const [searchQuery, setSearchQuery] = useState('');
  const [contacts, setContacts] = useState<User[]>([]);
  const [loading, setLoading] = useState(false);
  const [creating, setCreating] = useState(false);

  useEffect(() => {
    if (isOpen) {
      setLoading(true);
      contactService.getContacts()
        .then(data => {
          setContacts(data);
        })
        .catch(() => {
          setContacts([]);
          toast('加载联系人失败', 'error');
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

  const filteredContacts = contacts.filter(c => c.name.toLowerCase().includes(searchQuery.toLowerCase()));

  const handleCreate = async () => {
    if (selected.size === 0) return;
    setCreating(true);
    try {
      const group = await groupService.createGroup('', Array.from(selected));
      await onCreated?.(group);
      toast(`成功发起群聊 (${selected.size}人)`, 'success');
      onClose();
    } catch (error) {
      toast('发起群聊失败', 'error');
    } finally {
      setCreating(false);
    }
  };

  return (
    <ModalWrapper 
      isOpen={isOpen} 
      onClose={onClose} 
      title="发起群聊" 
      width="w-[480px]"
      footer={
        <>
          <button onClick={onClose} className="px-4 py-2 rounded bg-white/5 text-gray-300 hover:bg-white/10 transition-colors text-sm">取消</button>
          <button 
            disabled={selected.size === 0 || creating}
            className={`px-4 py-2 rounded text-white transition-colors text-sm ${selected.size > 0 && !creating ? 'bg-[#00b42a] hover:bg-[#009a24]' : 'bg-[#00b42a]/50 cursor-not-allowed'}`}
            onClick={handleCreate}
          >
            {creating ? '创建中...' : `创建 (${selected.size})`}
          </button>
        </>
      }
    >
      <div className="relative mb-4">
        <Search size={16} className="absolute left-3 top-1/2 -translate-y-1/2 text-gray-500" />
        <input 
          type="text" 
          placeholder="搜索联系人" 
          value={searchQuery}
          onChange={e => setSearchQuery(e.target.value)}
          className="w-full bg-[#181818] border border-white/5 rounded-lg py-2 pl-9 pr-4 text-sm text-gray-200 outline-none focus:border-white/20 transition-colors" 
        />
      </div>
      <div className="space-y-1">
        {loading ? (
          <div className="text-center py-8 text-gray-500 text-sm">加载中...</div>
        ) : filteredContacts.length > 0 ? (
          filteredContacts.map(contact => (
            <div 
              key={contact.id} 
              className="flex items-center gap-3 p-2 rounded-lg hover:bg-white/5 cursor-pointer transition-colors"
              onClick={() => toggleSelect(contact.id)}
            >
              <div className={`w-5 h-5 rounded-full border flex items-center justify-center transition-colors ${selected.has(contact.id) ? 'bg-[#00b42a] border-[#00b42a]' : 'border-gray-500'}`}>
                {selected.has(contact.id) && <Check size={12} className="text-white" />}
              </div>
              <Avatar src={contact.avatar} alt={contact.name} className="w-8 h-8 rounded" />
              <span className="text-gray-200 text-sm">{contact.name}</span>
            </div>
          ))
        ) : (
          <div className="text-center py-8 text-gray-500 text-sm">未找到联系人</div>
        )}
      </div>
    </ModalWrapper>
  );
};
