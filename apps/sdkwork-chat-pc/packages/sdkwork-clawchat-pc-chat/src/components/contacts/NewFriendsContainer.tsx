import React, { useCallback, useState, useEffect } from 'react';
import { Plus } from 'lucide-react';
import { Avatar } from '@sdkwork/clawchat-pc-commons';
import { toast } from '../Toast';
import { contactService } from '../../services/ContactService';
import type { FriendRequest } from '../../services/ContactService';

export const NewFriendsContainer: React.FC<{ onAddFriend?: () => void }> = ({ onAddFriend }) => {
  const [requests, setRequests] = useState<FriendRequest[]>([]);
  const [loading, setLoading] = useState(true);

  const refreshRequests = useCallback((showLoading = false) => {
    if (showLoading) {
      setLoading(true);
    }
    return contactService.getFriendRequests()
      .then(data => {
        setRequests(data);
      })
      .catch(() => {
        setRequests([]);
        toast('加载好友申请失败', 'error');
      })
      .finally(() => setLoading(false));
  }, []);

  useEffect(() => {
    void refreshRequests(true);
    return contactService.subscribePendingFriendRequestCount(() => {
      void refreshRequests();
    });
  }, [refreshRequests]);

  return (
    <div className="flex-1 flex flex-col bg-[#1e1e1e] min-w-0 h-full">
      <div className="px-8 py-6 border-b border-white/5 shrink-0 flex items-center justify-between">
        <div>
          <h2 className="text-xl font-medium text-gray-200">新的朋友</h2>
          <p className="text-sm text-gray-500 mt-1">查看和处理好友申请</p>
        </div>
        <button 
          onClick={() => onAddFriend?.()}
          className="flex items-center gap-2 px-4 py-2 bg-indigo-500 hover:bg-indigo-600 text-white text-sm font-medium rounded-lg transition-colors shadow-lg shadow-indigo-500/20"
        >
          <Plus size={16} /> 添加朋友
        </button>
      </div>
      <div className="flex-1 overflow-y-auto custom-scrollbar p-8">
        <div className="flex flex-col gap-4 max-w-3xl">
          {loading ? (
            <div className="text-sm text-gray-500">加载中...</div>
          ) : requests.map(req => (
            <div key={req.id} className="flex items-center justify-between p-4 rounded-2xl bg-white/5 hover:bg-white/10 transition-colors border border-white/5">
              <div className="flex items-center gap-4">
                <Avatar src={req.avatar} alt={req.name} fallback={req.name.charAt(0)} className="w-12 h-12 rounded-full" />
                <div>
                  <h4 className="text-md font-medium text-gray-200">{req.name}</h4>
                  <p className="text-sm text-gray-500 mt-0.5">{req.msg}</p>
                </div>
              </div>
              <div>
                {req.status === 'pending' ? (
                  <div className="flex items-center gap-2">
                    <button 
                      onClick={async () => {
                        try {
                          await contactService.handleFriendRequest(req.id, 'reject');
                          toast('已拒绝申请', 'success');
                          setRequests(requests.map(r => r.id === req.id ? { ...r, status: 'rejected' } : r));
                        } catch {
                          toast('处理好友申请失败', 'error');
                        }
                      }}
                      className="px-4 py-1.5 text-sm font-medium bg-white/10 hover:bg-white/20 text-gray-300 rounded-lg transition-colors"
                    >
                      拒绝
                    </button>
                    <button 
                      onClick={async () => {
                        try {
                          await contactService.handleFriendRequest(req.id, 'accept');
                          toast('已通过申请', 'success');
                          setRequests(requests.map(r => r.id === req.id ? { ...r, status: 'added' } : r));
                        } catch {
                          toast('处理好友申请失败', 'error');
                        }
                      }}
                      className="px-4 py-1.5 text-sm font-medium bg-indigo-500/10 hover:bg-indigo-500/20 text-indigo-400 rounded-lg transition-colors"
                    >
                      同意
                    </button>
                  </div>
                ) : (
                  <span className="text-xs text-gray-500 px-4">{req.status === 'added' ? '已添加' : '已拒绝'}</span>
                )}
              </div>
            </div>
          ))}
        </div>
      </div>
    </div>
  );
};
