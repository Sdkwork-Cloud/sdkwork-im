import React, { useState, useEffect } from 'react';
import { User, Users, Building2, Search, Tag, UserPlus, Star } from 'lucide-react';
import { motion, AnimatePresence } from 'motion/react';
import { Avatar } from '@sdkwork/clawchat-pc-commons';
import { cn } from '@sdkwork/clawchat-pc-commons';
import { toast } from '../components/Toast';
import { contactService } from '../services/ContactService';
import type { Chat, User as UserType } from '@sdkwork/clawchat-pc-types';

import { TagsContainer } from '../components/contacts/TagsContainer';
import { NewFriendsContainer } from '../components/contacts/NewFriendsContainer';
import { GroupsContainer } from '../components/contacts/GroupsContainer';
import { ContactDetailPane } from '../components/contacts/ContactDetailPane';
import { OrgContainer } from '../components/contacts/OrgContainer';
import { AllContactsContainer } from '../components/contacts/AllContactsContainer';

const menuItems = [
  { id: 'all', name: '全部好友', icon: <User size={18} />, color: 'bg-indigo-500' },
  { id: 'new', name: '新的朋友', icon: <UserPlus size={18} />, color: 'bg-orange-500' },
  { id: 'groups', name: '我的群组', icon: <Users size={18} />, color: 'bg-green-500' },
  { id: 'org', name: '组织架构', icon: <Building2 size={18} />, color: 'bg-blue-500' },
  { id: 'tags', name: '标签', icon: <Tag size={18} />, color: 'bg-purple-500' },
];

export const ContactsView: React.FC<{
  onSendMessage?: (user: UserType) => void;
  onStartCall?: (type: 'voice' | 'video', user: UserType) => void;
  onAddFriend?: () => void;
  onAppSelect?: (appId: string) => void;
  onOpenGroup?: (group: Chat) => void;
  searchQuery?: string;
}> = ({ onSendMessage, onStartCall, onAddFriend, onAppSelect, onOpenGroup, searchQuery = '' }) => {
  const [activeId, setActiveId] = useState('all');
  const [starredContacts, setStarredContacts] = useState<UserType[]>([]);
  const [loading, setLoading] = useState(true);
  
  // Global selected user state for the right panel across different views
  const [selectedUser, setSelectedUser] = useState<{ user: UserType, deptName: string } | null>(null);

  // Clear selected user when active tab changes to a main tab
  useEffect(() => {
    if (activeId !== '_starred') {
      setSelectedUser(null);
    }
  }, [activeId]);

  useEffect(() => {
    const loadData = async () => {
      setLoading(true);
      try {
        const contacts = await contactService.getStarredContacts();
        setStarredContacts(contacts);
      } catch (error) {
        toast('加载联系人失败', 'error');
      } finally {
        setLoading(false);
      }
    };
    loadData();
  }, []);

  return (
    <div className="flex flex-1 min-h-0">
      {/* Left List */}
      <div className="flex w-[280px] shrink-0 flex-col bg-[#202020] border-r border-white/5 min-h-0">
        <div className="flex-1 overflow-y-auto custom-scrollbar py-2">
          {menuItems.map(item => (
            <div 
              key={item.id}
              onClick={() => setActiveId(item.id)}
              className={cn(
                "flex items-center px-4 py-3 cursor-pointer transition-colors hover:bg-white/5",
                activeId === item.id && "bg-white/10 hover:bg-white/10"
              )}
            >
              <div className={cn("w-[36px] h-[36px] rounded-lg flex items-center justify-center text-white shrink-0 mr-3", item.color)}>
                {item.icon}
              </div>
              <span className="text-[14px] text-gray-200">{item.name}</span>
            </div>
          ))}
          
          <div className="px-4 py-2 mt-4 text-xs text-gray-500 font-medium flex items-center justify-between">
             星标联系人
          </div>
          {loading ? (
            <div className="px-4 py-3 text-sm text-gray-500">加载中...</div>
          ) : (
            starredContacts.map(contact => (
              <div 
                key={contact.id} 
                className={cn(
                  "flex items-center px-4 py-3 cursor-pointer transition-colors hover:bg-white/5",
                  selectedUser?.user.id === contact.id && activeId === '_starred' && "bg-white/10"
                )}
                onClick={() => {
                  setActiveId('_starred');
                  setSelectedUser({ user: contact, deptName: '星标联系人' });
                }}
              >
                <Avatar src={contact.avatar} alt={contact.name} className="w-[36px] h-[36px] rounded-lg mr-3 bg-[#2b2b2d]" />
                <span className="text-[14px] text-gray-200 truncate">{contact.name}</span>
              </div>
            ))
          )}
        </div>
      </div>
      
      {/* Main Content Area */}
      {activeId !== '_starred' && (
        <AnimatePresence mode="wait">
          <motion.div
            key={activeId}
            initial={{ opacity: 0, y: 10 }}
            animate={{ opacity: 1, y: 0 }}
            exit={{ opacity: 0, y: -10 }}
            transition={{ duration: 0.2 }}
            className="flex-1 flex flex-col bg-[#1e1e1e] min-w-0"
          >
            {activeId === 'org' ? (
               <OrgContainer onUserSelect={(u, d) => setSelectedUser({user: u, deptName: d})} selectedUserId={selectedUser?.user.id || null} onSendMessage={onSendMessage} searchQuery={searchQuery} />
            ) : activeId === 'all' ? (
               <AllContactsContainer onUserSelect={(u, d) => setSelectedUser({user: u, deptName: d})} selectedUserId={selectedUser?.user.id || null} searchQuery={searchQuery} />
            ) : activeId === 'tags' ? (
               <TagsContainer searchQuery={searchQuery} />
            ) : activeId === 'new' ? (
               <NewFriendsContainer onAddFriend={onAddFriend} />
            ) : activeId === 'groups' ? (
               <GroupsContainer searchQuery={searchQuery} onOpenGroup={onOpenGroup} />
            ) : (
              <div className="flex-1 flex flex-col min-w-0 items-center justify-center">
                <Building2 size={64} className="text-gray-600 mb-4" />
                <h3 className="text-xl text-gray-300 font-medium mb-2">
                  {menuItems.find(m => m.id === activeId)?.name || '未选定内容'}
                </h3>
                <p className="text-gray-500 text-sm">此功能模块无对应视图</p>
              </div>
            )}
          </motion.div>
        </AnimatePresence>
      )}

      {/* Global Right Panel for selected user across Left Sidebar options */}
      {selectedUser ? (
        <ContactDetailPane user={selectedUser.user} departmentName={selectedUser.deptName} fullWidth={activeId === '_starred'} onSendMessage={onSendMessage} onStartCall={onStartCall} onAppSelect={onAppSelect} />
      ) : activeId === '_starred' ? (
         <motion.div 
            initial={{ opacity: 0, scale: 0.95 }}
            animate={{ opacity: 1, scale: 1 }}
            transition={{ duration: 0.3 }}
            className="flex-1 flex flex-col bg-[#1e1e1e] min-w-0 items-center justify-center"
         >
            <User size={64} className="text-gray-600 mb-4 opacity-50" />
            <h3 className="text-xl text-gray-300 font-medium mb-2">星标联系人</h3>
            <p className="text-gray-500 text-sm">请从左侧选择一个星标联系人查看</p>
         </motion.div>
      ) : activeId === 'org' || activeId === 'all' ? (
         <motion.div 
            initial={{ opacity: 0, x: 20 }}
            animate={{ opacity: 1, x: 0 }}
            transition={{ duration: 0.3 }}
            className="w-[360px] lg:w-[420px] flex-shrink-0 bg-[#1e1e1e] flex flex-col items-center justify-center border-l border-white/5 border-dashed hidden md:flex"
         >
            <User size={64} className="text-gray-700 mx-auto mb-4 opacity-50" />
            <p className="text-gray-500 text-sm font-medium">请选择一个联系人以查看详细信息</p>
         </motion.div>
      ) : null}
    </div>
  );
};
