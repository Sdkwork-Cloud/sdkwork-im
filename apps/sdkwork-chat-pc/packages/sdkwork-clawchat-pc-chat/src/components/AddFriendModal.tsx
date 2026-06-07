import React, { useState } from 'react';
import { Loader2, Search, UserPlus } from 'lucide-react';
import { motion } from 'motion/react';
import { Avatar } from '@sdkwork/clawchat-pc-commons';
import { toast } from './Toast';
import { contactService } from '../services/ContactService';
import { ModalWrapper } from './ModalWrapper';
import type { User } from '@sdkwork/clawchat-pc-types';

interface AddFriendSearchResult {
  avatar?: string;
  desc: string;
  id: string;
  name: string;
}

function buildSearchResultDescription(user: User): string {
  return user.chatId
    || user.email
    || user.phone
    || user.position
    || user.company
    || user.location
    || user.motto
    || user.name;
}

export const AddFriendModal: React.FC<{ isOpen: boolean; onClose: () => void }> = ({ isOpen, onClose }) => {
  const [searchQuery, setSearchQuery] = useState('');
  const [isSearching, setIsSearching] = useState(false);
  const [searchNotice, setSearchNotice] = useState<{ type: 'loading' | 'empty'; message: string } | null>(null);
  const [result, setResult] = useState<AddFriendSearchResult | null>(null);

  const handleSearch = async () => {
    const normalizedQuery = searchQuery.trim();
    if (!normalizedQuery) {
      return;
    }

    setIsSearching(true);
    setResult(null);
    setSearchNotice({ type: 'loading', message: 'Searching contacts...' });
    try {
      const results = await contactService.searchContacts(normalizedQuery);
      if (results.length > 0) {
        setSearchNotice(null);
        setResult({
          id: results[0].id,
          name: results[0].name,
          avatar: results[0].avatar,
          desc: buildSearchResultDescription(results[0]),
        });
      } else {
        setSearchNotice({ type: 'empty', message: `No contact found for ${normalizedQuery}` });
      }
    } catch {
      setSearchNotice(null);
      toast('Search failed', 'error');
    } finally {
      setIsSearching(false);
    }
  };

  React.useEffect(() => {
    if (!isOpen) {
      setSearchQuery('');
      setResult(null);
      setSearchNotice(null);
    }
  }, [isOpen]);

  return (
    <ModalWrapper isOpen={isOpen} onClose={onClose} title="Add Friend">
      <div className="mb-6 flex gap-2">
        <div className="relative flex-1">
          <Search size={16} className="absolute left-3 top-1/2 -translate-y-1/2 text-gray-500" />
          <input
            type="text"
            placeholder="Email, Chat ID, or phone"
            value={searchQuery}
            onChange={(event) => {
              setSearchQuery(event.target.value);
              setSearchNotice(null);
            }}
            onKeyDown={(event) => {
              if (event.key === 'Enter') {
                void handleSearch();
              }
            }}
            className="w-full rounded-lg border border-white/5 bg-[#181818] py-2 pl-9 pr-4 text-sm text-gray-200 outline-none transition-colors focus:border-white/20"
          />
        </div>
        <button
          onClick={handleSearch}
          disabled={!searchQuery.trim() || isSearching}
          className="rounded-lg bg-[#00b42a] px-4 py-2 text-sm text-white transition-colors hover:bg-[#009a24] disabled:bg-[#00b42a]/50"
        >
          {isSearching ? 'Searching...' : 'Search'}
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
          initial={{ opacity: 0, y: 10 }}
          animate={{ opacity: 1, y: 0 }}
          className="flex items-center justify-between rounded-lg border border-white/5 bg-[#181818] p-4"
        >
          <div className="flex min-w-0 items-center gap-3">
            <Avatar src={result.avatar} alt={result.name} className="h-10 w-10 rounded" />
            <div className="min-w-0">
              <div className="truncate text-sm font-medium text-gray-200">{result.name}</div>
              <div className="mt-0.5 truncate text-xs text-gray-500">{result.desc}</div>
            </div>
          </div>
          <button
            className="flex shrink-0 items-center gap-1 rounded bg-white/10 px-3 py-1.5 text-sm text-blue-400 transition-colors hover:bg-white/20"
            onClick={async () => {
              try {
                await contactService.addFriend(result.id);
                toast(`Friend request sent to ${result.name}`, 'success');
                onClose();
              } catch {
                toast('Friend request failed', 'error');
              }
            }}
          >
            <UserPlus size={14} />
            Add
          </button>
        </motion.div>
      )}
    </ModalWrapper>
  );
};
