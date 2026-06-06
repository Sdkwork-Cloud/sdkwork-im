import React, { useState, useEffect } from 'react';
import { Building2, Hash, ChevronRight, MessageSquare, Phone, Video, Mail, Star, MoreVertical } from 'lucide-react';
import { motion } from 'motion/react';
import { Avatar } from '@sdkwork/clawchat-pc-commons';
import { cn } from '@sdkwork/clawchat-pc-commons';
import { toast } from '../Toast';
import { contactService } from '../../services/ContactService';
import type { User as UserType } from '@sdkwork/clawchat-pc-types';
import { PromptModal, usePrompt } from '../PromptModal';

export const ContactDetailPane: React.FC<{ user: UserType; departmentName: string; fullWidth?: boolean; onSendMessage?: (user: UserType) => void; onStartCall?: (type: 'voice' | 'video', user: UserType) => void; onAppSelect?: (appId: string) => void }> = ({ user, departmentName, fullWidth, onSendMessage, onStartCall, onAppSelect }) => {
  const [showMoreMenu, setShowMoreMenu] = useState(false);
  const [isStarred, setIsStarred] = useState(false);
  const { promptConfig, customPrompt, closePrompt } = usePrompt();

  useEffect(() => {
    contactService.getStarredContacts()
      .then(starred => {
        setIsStarred(starred.some(u => u.id === user.id));
      })
      .catch(() => setIsStarred(false));
  }, [user.id]);

  const handleToggleStar = async () => {
    const newStatus = !isStarred;
    try {
      await contactService.toggleStarContact(user.id, newStatus);
      setIsStarred(newStatus);
      toast(newStatus ? '已设为星标联系人' : '已取消星标', 'success');
    } catch(e) {
      toast('操作失败', 'error');
    }
  };

  return (
    <div className={cn("flex-shrink-0 bg-[#1e1e1e] flex flex-col relative transition-all", fullWidth ? "flex-1 border-none shadow-[inset_1px_0_0_rgb(255,255,255,0.05)]" : "w-[360px] lg:w-[420px] border-l border-white/5 shadow-2xl")}>
      <div className="absolute inset-0 flex flex-col items-center bg-gradient-to-b from-[#2b2b2d]/50 to-[#1e1e1e]">
         <div className={cn("w-full h-full flex flex-col relative", fullWidth ? "max-w-[420px] border-x border-white/5 shadow-2xl bg-[#1e1e1e]" : "")}>
           <div className="p-8 border-b border-white/5 shrink-0 flex flex-col items-center text-center relative overflow-hidden">
             {/* Action Buttons Toggles at top right */}
             <div className="absolute top-4 right-4 flex items-center gap-2 z-20">
               <button 
                 onClick={handleToggleStar}
                 className="p-2 rounded-full bg-white/5 hover:bg-white/10 text-gray-400 hover:text-yellow-400 transition-colors shadow-sm"
                 title={isStarred ? "取消星标" : "设为星标"}
               >
                 <Star size={18} className={cn("transition-colors", isStarred && "fill-yellow-400 text-yellow-400")} />
               </button>
               <div className="relative">
                 <button 
                   onClick={() => setShowMoreMenu(!showMoreMenu)}
                   className="p-2 rounded-full bg-white/5 hover:bg-white/10 text-gray-400 hover:text-white transition-colors shadow-sm"
                   title="更多"
                 >
                   <MoreVertical size={18} />
                 </button>
                 {showMoreMenu && (
                   <>
                     <div className="fixed inset-0 z-40" onClick={() => setShowMoreMenu(false)} />
                     <motion.div initial={{ opacity: 0, scale: 0.95 }} animate={{ opacity: 1, scale: 1 }} className="absolute top-10 right-0 w-48 bg-[#282828] border border-white/10 rounded-xl shadow-xl z-50 p-1.5">
                               <button className="w-full text-left px-3 py-2 text-sm text-gray-300 hover:text-white hover:bg-white/10 rounded-lg transition-colors" onClick={() => { setShowMoreMenu(false); navigator.clipboard.writeText(user.id); toast('已复制微信号', 'success'); }}>复制微信号</button>
                               <button className="w-full text-left px-3 py-2 text-sm text-gray-300 hover:text-white hover:bg-white/10 rounded-lg transition-colors" onClick={() => { 
                                  setShowMoreMenu(false); 
                                  customPrompt("设置备注", user.name, async (name) => {
                                     try {
                                       if(name && name.trim()) {
                                          await contactService.setContactRemark(user.id, name.trim());
                                          toast('已设置备注', 'success');
                                       }
                                     } catch {
                                       toast('设置备注失败', 'error');
                                     } finally {
                                       closePrompt();
                                     }
                                  });
                               }}>设置备注和标签</button>
                               <button className="w-full text-left px-3 py-2 text-sm text-gray-300 hover:text-white hover:bg-white/10 rounded-lg transition-colors" onClick={async () => {
                                  setShowMoreMenu(false);
                                  try {
                                    await contactService.recommendToFriend(user.id);
                                    toast('已推荐给朋友', 'success');
                                  } catch {
                                    toast('推荐失败', 'error');
                                  }
                               }}>推荐给朋友</button>
                               <div className="h-px bg-white/10 my-1 mx-2" />
                               <button className="w-full text-left px-3 py-2 text-sm text-red-400 hover:bg-red-500/10 rounded-lg transition-colors" onClick={async () => {
                                  setShowMoreMenu(false);
                                  try {
                                    await contactService.addToBlacklist(user.id);
                                    toast('已加入黑名单', 'success');
                                  } catch {
                                    toast('加入黑名单失败', 'error');
                                  }
                               }}>加入黑名单</button>
                               <button className="w-full text-left px-3 py-2 text-sm text-red-500 hover:bg-red-500/10 rounded-lg transition-colors" onClick={async () => { 
                                  setShowMoreMenu(false); 
                                  try {
                                    await contactService.deleteContact(user.id);
                                    toast('已删除联系人', 'success'); 
                                  } catch(e) {
                                    toast('删除失败', 'error');
                                  }
                               }}>删除</button>
                     </motion.div>
                   </>
                 )}
               </div>
             </div>

             {/* Abstract Background Blur */}
             <div className="absolute top-0 right-0 w-48 h-48 bg-indigo-500/10 rounded-full blur-3xl pointer-events-none -mt-20 -mr-20" />
             <div className="absolute bottom-0 left-0 w-32 h-32 bg-purple-500/10 rounded-full blur-2xl pointer-events-none mb-10 -ml-10" />

             <motion.div key={user.id} initial={{ scale: 0.9, opacity: 0 }} animate={{ scale: 1, opacity: 1 }} className="relative z-10 transition-transform">
               <Avatar src={user.avatar} alt={user.name} className="w-[100px] h-[100px] rounded-[2rem] bg-[#2b2b2d] mb-5 shadow-xl border border-white/10" />
               <div className={cn("absolute bottom-0 right-0 w-5 h-5 rounded-full border-4 border-[#1e1e1e]", user.status === 'online' ? "bg-green-500" : "bg-gray-500")} title={user.status} />
             </motion.div>
             
             <h2 className="text-2xl font-semibold text-gray-100 flex items-center gap-2 mb-1 z-10">
               {user.name}
             </h2>
             <div className="text-indigo-400 text-sm font-medium mb-6 z-10">{user.position || '未知职务'}</div>
             
             <div className="flex items-center justify-center gap-3 w-full z-10">
               <button 
                 onClick={() => {
                   if (onSendMessage) onSendMessage(user);
                   else toast('聊天能力未接入，无法发起会话', 'error');
                 }}
                 className="flex-1 bg-indigo-600 hover:bg-indigo-500 text-white font-medium py-2.5 rounded-xl flex items-center justify-center gap-2 transition-all shadow-lg shadow-indigo-500/20 active:scale-[0.98]"
               >
                 <MessageSquare size={18} /> 发消息
               </button>
               <div className="flex gap-2">
                 <button 
                   onClick={() => {
                     if (onStartCall) onStartCall('voice', user);
                     else toast('RTC 通话能力未接入，无法发起语音通话', 'error');
                   }}
                   className="w-11 h-11 bg-white/5 hover:bg-white/10 border border-white/5 text-gray-300 rounded-xl flex items-center justify-center transition-all active:scale-[0.98]"
                   title="语音呼叫"
                 >
                   <Phone size={18} />
                 </button>
                 <button 
                   onClick={() => {
                     if (onStartCall) onStartCall('video', user);
                     else toast('RTC 通话能力未接入，无法发起视频通话', 'error');
                   }}
                   className="w-11 h-11 bg-white/5 hover:bg-white/10 border border-white/5 text-gray-300 rounded-xl flex items-center justify-center transition-all active:scale-[0.98]"
                   title="视频呼叫"
                 >
                   <Video size={18} />
                 </button>
               </div>
             </div>
           </div>
           
           <div className="flex-1 p-8 overflow-y-auto custom-scrollbar flex flex-col gap-6">
             <div className="space-y-4">
               <h3 className="text-xs font-semibold text-gray-500 uppercase tracking-wider mb-2">基本信息</h3>
               
               <div className="flex flex-col gap-3">
                 <div className="flex items-start gap-4 p-3 rounded-xl hover:bg-white/5 transition-colors">
                   <Building2 size={18} className="text-gray-500 mt-0.5 shrink-0" />
                   <div>
                     <div className="text-xs text-gray-500 mb-0.5">所属部门</div>
                     <div className="text-sm text-gray-200">{departmentName}</div>
                   </div>
                 </div>
                 {user.company && (
                   <div className="flex items-start gap-4 p-3 rounded-xl hover:bg-white/5 transition-colors">
                     <Building2 size={18} className="text-gray-500 mt-0.5 shrink-0" />
                     <div>
                       <div className="text-xs text-gray-500 mb-0.5">所在公司</div>
                       <div className="text-sm text-gray-200">{user.company}</div>
                     </div>
                   </div>
                 )}
                 {user.location && (
                   <div className="flex items-start gap-4 p-3 rounded-xl hover:bg-white/5 transition-colors">
                     <Hash size={18} className="text-gray-500 mt-0.5 shrink-0" />
                     <div>
                       <div className="text-xs text-gray-500 mb-0.5">办公地点</div>
                       <div className="text-sm text-gray-200">{user.location}</div>
                     </div>
                   </div>
                 )}
                 {user.motto && (
                   <div className="flex items-start gap-4 p-3 rounded-xl hover:bg-white/5 transition-colors">
                     <MessageSquare size={18} className="text-gray-500 mt-0.5 shrink-0" />
                     <div>
                       <div className="text-xs text-gray-500 mb-0.5">个性签名</div>
                       <div className="text-sm text-gray-200 italic">"{user.motto}"</div>
                     </div>
                   </div>
                 )}
                 
                 {user.email && (
                   <div className="flex items-start gap-4 p-3 rounded-xl hover:bg-white/5 transition-colors group cursor-pointer" onClick={() => onAppSelect ? onAppSelect('mail') : toast('发送邮件', 'success')}>
                     <Mail size={18} className="text-gray-500 mt-0.5 shrink-0" />
                     <div className="flex-1">
                       <div className="text-xs text-gray-500 mb-0.5">企业邮箱</div>
                       <div className="text-sm text-gray-200">{user.email}</div>
                     </div>
                     <ChevronRight size={16} className="text-gray-600 opacity-0 group-hover:opacity-100 transition-opacity" />
                   </div>
                 )}

                 {user.phone && (
                   <div className="flex items-start gap-4 p-3 rounded-xl hover:bg-white/5 transition-colors group cursor-pointer" onClick={() => {
                     if (onStartCall) onStartCall('voice', user);
                     else toast('RTC 通话能力未接入，无法发起语音通话', 'error');
                   }}>
                     <Phone size={18} className="text-gray-500 mt-0.5 shrink-0" />
                     <div className="flex-1">
                       <div className="text-xs text-gray-500 mb-0.5">手机号码</div>
                       <div className="text-sm text-gray-200 font-mono">{user.phone}</div>
                     </div>
                     <ChevronRight size={16} className="text-gray-600 opacity-0 group-hover:opacity-100 transition-opacity" />
                   </div>
                 )}
               </div>
             </div>
           </div>
         </div>
      </div>
      <PromptModal {...promptConfig} onCancel={closePrompt} />
    </div>
  );
};
