import React from 'react';
import { motion } from 'motion/react';
import { Search, Plus, MoreHorizontal } from 'lucide-react';
import { Chat } from '@sdkwork/clawchat-pc-types';
import { Avatar } from '@sdkwork/clawchat-pc-commons';

export interface ChatRightPanelProps {
  activeChat: Chat;
  onSetModal: (modal: 'search'|'editName'|'editNotice'|'addMember'|null, inputVal: string) => void;
  onToggleMute: () => Promise<void>;
  onTogglePin: () => Promise<void>;
  onDeleteChat: () => Promise<void>;
}

export const ChatRightPanel: React.FC<ChatRightPanelProps> = ({
  activeChat,
  onSetModal,
  onToggleMute,
  onTogglePin,
  onDeleteChat
}) => {
  return (
    <motion.div
      initial={{ width: 0, opacity: 0 }}
      animate={{ width: 300, opacity: 1 }}
      exit={{ width: 0, opacity: 0 }}
      transition={{ duration: 0.2 }}
      className="h-full border-l border-white/5 bg-[#181818] overflow-y-auto custom-scrollbar flex-shrink-0"
    >
      <div className="p-6 flex flex-col items-center">
         <Avatar src={activeChat.avatar} alt={activeChat.name} className="w-20 h-20 rounded-2xl bg-[#2b2b2d] mb-4 shadow-lg" />
         <h3 className="text-lg font-medium text-gray-200 mb-1">{activeChat.name}</h3>
         <p className="text-xs text-gray-500 mb-6">ID: {activeChat.id}</p>
         
         <div className="w-full flex justify-center gap-6 mb-8">
            <div className="flex flex-col items-center gap-2 cursor-pointer group" onClick={() => onSetModal('search', '')}>
               <div className="w-10 h-10 rounded-full bg-[#2b2b2d] flex items-center justify-center group-hover:bg-white/10 transition-colors">
                  <Search size={18} className="text-gray-400 group-hover:text-gray-200" />
               </div>
               <span className="text-xs text-gray-400 group-hover:text-gray-200">查找聊天</span>
            </div>
            {activeChat.type === 'group' && (
              <div className="flex flex-col items-center gap-2 cursor-pointer group" onClick={() => onSetModal('addMember', '')}>
                 <div className="w-10 h-10 rounded-full bg-[#2b2b2d] flex items-center justify-center group-hover:bg-white/10 transition-colors">
                    <Plus size={18} className="text-gray-400 group-hover:text-gray-200" />
                 </div>
                 <span className="text-xs text-gray-400 group-hover:text-gray-200">添加成员</span>
              </div>
            )}
         </div>
         
         <div className="w-full space-y-4">
            <div className="flex items-center justify-between py-3 border-b border-white/5 cursor-pointer hover:bg-white/5 px-2 -mx-2 rounded transition-colors group" onClick={() => onSetModal('editName', activeChat.name)}>
               <span className="text-sm text-gray-300">{activeChat.type === 'group' ? '群聊名称' : '设置备注'}</span>
               <div className="flex items-center gap-2 text-gray-500">
                 <span className="text-sm overflow-hidden text-ellipsis whitespace-nowrap max-w-[100px]">{activeChat.name}</span>
                 <MoreHorizontal size={16} className="opacity-0 group-hover:opacity-100 transition-opacity" />
               </div>
            </div>
            {activeChat.type === 'group' && (
              <div className="flex items-center justify-between py-3 border-b border-white/5 cursor-pointer hover:bg-white/5 px-2 -mx-2 rounded transition-colors group" onClick={() => onSetModal('editNotice', activeChat.notice || '暂无公告')}>
                 <span className="text-sm text-gray-300">群公告</span>
                 <div className="flex items-center gap-2 text-gray-500">
                   <span className="text-sm overflow-hidden text-ellipsis whitespace-nowrap max-w-[100px]">{activeChat.notice || '暂无公告'}</span>
                   <MoreHorizontal size={16} className="opacity-0 group-hover:opacity-100 transition-opacity" />
                 </div>
              </div>
            )}
            <div className="flex items-center justify-between py-3 border-b border-white/5">
               <span className="text-sm text-gray-300">消息免打扰</span>
               <div 
                 className={`w-10 h-5 rounded-full relative cursor-pointer pt-0.5 px-0.5 transition-colors ${activeChat.isMuted ? 'bg-[#00b42a]' : 'bg-[#2b2b2d] hover:bg-white/10'}`}
                 onClick={() => void onToggleMute()}
               >
                  <div className={`w-4 h-4 bg-white rounded-full absolute top-0.5 transition-all ${activeChat.isMuted ? 'right-0.5' : 'left-0.5'}`} />
               </div>
            </div>
            <div className="flex items-center justify-between py-3 border-b border-white/5">
               <span className="text-sm text-gray-300">置顶聊天</span>
               <div 
                 className={`w-10 h-5 rounded-full relative cursor-pointer pt-0.5 px-0.5 transition-colors ${activeChat.isPinned ? 'bg-[#00b42a]' : 'bg-[#2b2b2d] hover:bg-white/10'}`}
                 onClick={() => void onTogglePin()}
               >
                  <div className={`w-4 h-4 bg-white rounded-full absolute top-0.5 transition-all ${activeChat.isPinned ? 'right-0.5' : 'left-0.5'}`} />
               </div>
            </div>
         </div>
         
         <button 
           className="w-full py-3 mt-8 text-red-500 text-sm font-medium hover:bg-red-500/10 rounded-lg transition-colors border border-transparent hover:border-red-500/20"
           onClick={() => void onDeleteChat()}
         >
            {activeChat.type === 'group' ? '退出群聊' : '删除聊天'}
         </button>
      </div>
    </motion.div>
  );
};
