import React, { useState } from 'react';
import { Loader2, Search, UserPlus } from 'lucide-react';
import { motion } from 'motion/react';
import { Avatar } from '@sdkwork/clawchat-pc-commons';
import { toast } from './Toast';
import { contactService } from '../services/ContactService';
import { ModalWrapper } from './ModalWrapper';
import type { User } from '@sdkwork/clawchat-pc-types';

function buildSearchResultDescription(user: User): string {
  return user.email
    || user.phone
    || user.position
    || user.company
    || user.location
    || user.motto
    || user.id;
}

export const AddFriendModal: React.FC<{ isOpen: boolean; onClose: () => void }> = ({ isOpen, onClose }) => {
  const [searchQuery, setSearchQuery] = useState('');
  const [isSearching, setIsSearching] = useState(false);
  const [searchNotice, setSearchNotice] = useState<{ type: 'loading' | 'empty'; message: string } | null>(null);
  const [result, setResult] = useState<any>(null);

  const handleSearch = async () => {
    const normalizedQuery = searchQuery.trim();
    if (!normalizedQuery) return;
    setIsSearching(true);
    setResult(null);
    setSearchNotice({ type: 'loading', message: '正在搜索联系人...' });
    try {
      const results = await contactService.searchContacts(normalizedQuery);
      if (results.length > 0) {
        setSearchNotice(null);
        setResult({
          id: results[0].id,
          name: results[0].name,
          avatar: results[0].avatar,
          desc: buildSearchResultDescription(results[0])
        });
      } else {
        setSearchNotice({ type: 'empty', message: `未找到与 ${normalizedQuery} 匹配的联系人` });
      }
    } catch (error) {
      setSearchNotice(null);
      toast('搜索失败', 'error');
    } finally {
      setIsSearching(false);
    }
  };

  // Reset state when modal opens/closes
  React.useEffect(() => {
    if (!isOpen) {
      setSearchQuery('');
      setResult(null);
      setSearchNotice(null);
    }
  }, [isOpen]);

  return (
    <ModalWrapper isOpen={isOpen} onClose={onClose} title="添加朋友">
      <div className="flex gap-2 mb-6">
        <div className="relative flex-1">
          <Search size={16} className="absolute left-3 top-1/2 -translate-y-1/2 text-gray-500" />
          <input 
            type="text" 
            placeholder="输入手机号/ClawChat号" 
            value={searchQuery}
            onChange={e => {
              setSearchQuery(e.target.value);
              setSearchNotice(null);
            }}
            onKeyDown={e => e.key === 'Enter' && handleSearch()}
            className="w-full bg-[#181818] border border-white/5 rounded-lg py-2 pl-9 pr-4 text-sm text-gray-200 outline-none focus:border-white/20 transition-colors" 
          />
        </div>
        <button 
          onClick={handleSearch}
          disabled={!searchQuery.trim() || isSearching}
          className="px-4 py-2 bg-[#00b42a] hover:bg-[#009a24] disabled:bg-[#00b42a]/50 text-white rounded-lg text-sm transition-colors"
        >
          {isSearching ? '搜索中...' : '搜索'}
        </button>
      </div>

      {searchNotice && (
        <motion.div
          initial={{ opacity: 0, y: -4 }}
          animate={{ opacity: 1, y: 0 }}
          className={`-mt-3 mb-4 flex items-center gap-2 rounded-lg border px-3 py-2 text-xs ${
            searchNotice.type === 'loading'
              ? 'border-white/5 bg-white/5 text-gray-400'
              : 'border-amber-500/20 bg-amber-500/10 text-amber-300'
          }`}
          aria-live="polite"
        >
          {searchNotice.type === 'loading' && <Loader2 size={14} className="animate-spin" />}
          <span>{searchNotice.message}</span>
        </motion.div>
      )}

      {result && (
        <motion.div 
          initial={{ opacity: 0, y: 10 }} animate={{ opacity: 1, y: 0 }}
          className="flex items-center justify-between p-4 bg-[#181818] rounded-lg border border-white/5"
        >
          <div className="flex items-center gap-3">
            <Avatar src={result.avatar} alt={result.name} className="w-10 h-10 rounded" />
            <div>
              <div className="text-gray-200 text-sm font-medium">{result.name}</div>
              <div className="text-gray-500 text-xs mt-0.5">{result.desc}</div>
            </div>
          </div>
          <button 
            className="px-3 py-1.5 bg-white/10 hover:bg-white/20 text-blue-400 rounded text-sm transition-colors flex items-center gap-1"
            onClick={async () => {
              try {
                await contactService.addFriend(result.id);
                toast(`已发送好友请求给 ${result.name}`, 'success');
                onClose();
              } catch (error) {
                toast('发送请求失败', 'error');
              }
            }}
          >
            <UserPlus size={14} />
            添加
          </button>
        </motion.div>
      )}
    </ModalWrapper>
  );
};
