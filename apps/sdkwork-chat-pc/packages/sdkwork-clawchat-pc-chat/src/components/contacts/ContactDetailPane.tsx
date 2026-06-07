import React, { useEffect, useState } from 'react';
import {
  Building2,
  ChevronRight,
  Hash,
  Mail,
  MessageSquare,
  MoreVertical,
  Phone,
  Star,
  Video,
} from 'lucide-react';
import { motion } from 'motion/react';
import { Avatar, cn } from '@sdkwork/clawchat-pc-commons';
import { toast } from '../Toast';
import { contactService } from '../../services/ContactService';
import type { User as UserType } from '@sdkwork/clawchat-pc-types';
import { PromptModal, usePrompt } from '../PromptModal';

export const ContactDetailPane: React.FC<{
  user: UserType;
  departmentName: string;
  fullWidth?: boolean;
  onSendMessage?: (user: UserType) => void;
  onStartCall?: (type: 'voice' | 'video', user: UserType) => void;
  onAppSelect?: (appId: string) => void;
}> = ({ user, departmentName, fullWidth, onSendMessage, onStartCall, onAppSelect }) => {
  const [showMoreMenu, setShowMoreMenu] = useState(false);
  const [isStarred, setIsStarred] = useState(false);
  const { promptConfig, customPrompt, closePrompt } = usePrompt();
  const displayUserChatId = user.chatId ?? '';

  useEffect(() => {
    contactService.getStarredContacts()
      .then((starred) => {
        setIsStarred(starred.some((starredUser) => starredUser.id === user.id));
      })
      .catch(() => setIsStarred(false));
  }, [user.id]);

  const handleToggleStar = async () => {
    const newStatus = !isStarred;
    try {
      await contactService.toggleStarContact(user.id, newStatus);
      setIsStarred(newStatus);
      toast(newStatus ? 'Contact starred' : 'Contact unstarred', 'success');
    } catch {
      toast('Contact update failed', 'error');
    }
  };

  const copyDisplayUserChatId = () => {
    setShowMoreMenu(false);
    if (!displayUserChatId) {
      toast('Chat ID is not ready. Please try again.', 'error');
      return;
    }
    void navigator.clipboard.writeText(displayUserChatId);
    toast('Chat ID copied', 'success');
  };

  const startVoiceCall = () => {
    if (onStartCall) {
      onStartCall('voice', user);
      return;
    }
    toast('Voice calling is unavailable', 'error');
  };

  const startVideoCall = () => {
    if (onStartCall) {
      onStartCall('video', user);
      return;
    }
    toast('Video calling is unavailable', 'error');
  };

  return (
    <div className={cn(
      'relative flex shrink-0 flex-col bg-[#1e1e1e] transition-all',
      fullWidth
        ? 'flex-1 border-none shadow-[inset_1px_0_0_rgb(255,255,255,0.05)]'
        : 'w-[360px] border-l border-white/5 shadow-2xl lg:w-[420px]',
    )}>
      <div className="absolute inset-0 flex flex-col items-center">
        <div className={cn(
          'relative flex h-full w-full flex-col',
          fullWidth ? 'max-w-[420px] border-x border-white/5 bg-[#1e1e1e] shadow-2xl' : '',
        )}>
          <div className="relative flex shrink-0 flex-col items-center overflow-hidden border-b border-white/5 p-8 text-center">
            <div className="absolute right-4 top-4 z-20 flex items-center gap-2">
              <button
                onClick={handleToggleStar}
                className="rounded-full bg-white/5 p-2 text-gray-400 shadow-sm transition-colors hover:bg-white/10 hover:text-yellow-400"
                title={isStarred ? 'Unstar contact' : 'Star contact'}
              >
                <Star size={18} className={cn('transition-colors', isStarred && 'fill-yellow-400 text-yellow-400')} />
              </button>
              <div className="relative">
                <button
                  onClick={() => setShowMoreMenu(!showMoreMenu)}
                  className="rounded-full bg-white/5 p-2 text-gray-400 shadow-sm transition-colors hover:bg-white/10 hover:text-white"
                  title="More"
                >
                  <MoreVertical size={18} />
                </button>
                {showMoreMenu && (
                  <>
                    <div className="fixed inset-0 z-40" onClick={() => setShowMoreMenu(false)} />
                    <motion.div
                      initial={{ opacity: 0, scale: 0.95 }}
                      animate={{ opacity: 1, scale: 1 }}
                      className="absolute right-0 top-10 z-50 w-48 rounded-xl border border-white/10 bg-[#282828] p-1.5 shadow-xl"
                    >
                      <button
                        className="w-full rounded-lg px-3 py-2 text-left text-sm text-gray-300 transition-colors hover:bg-white/10 hover:text-white"
                        onClick={copyDisplayUserChatId}
                      >
                        Copy Chat ID
                      </button>
                      <button
                        className="w-full rounded-lg px-3 py-2 text-left text-sm text-gray-300 transition-colors hover:bg-white/10 hover:text-white"
                        onClick={() => {
                          setShowMoreMenu(false);
                          customPrompt('Set remark', user.name, async (name) => {
                            try {
                              if (name?.trim()) {
                                await contactService.setContactRemark(user.id, name.trim());
                                toast('Remark updated', 'success');
                              }
                            } catch {
                              toast('Remark update failed', 'error');
                            } finally {
                              closePrompt();
                            }
                          });
                        }}
                      >
                        Set Remark
                      </button>
                      <button
                        className="w-full rounded-lg px-3 py-2 text-left text-sm text-gray-300 transition-colors hover:bg-white/10 hover:text-white"
                        onClick={async () => {
                          setShowMoreMenu(false);
                          try {
                            await contactService.recommendToFriend(user.id);
                            toast('Recommendation sent', 'success');
                          } catch {
                            toast('Recommendation failed', 'error');
                          }
                        }}
                      >
                        Recommend
                      </button>
                      <div className="mx-2 my-1 h-px bg-white/10" />
                      <button
                        className="w-full rounded-lg px-3 py-2 text-left text-sm text-red-400 transition-colors hover:bg-red-500/10"
                        onClick={async () => {
                          setShowMoreMenu(false);
                          try {
                            await contactService.addToBlacklist(user.id);
                            toast('Added to blacklist', 'success');
                          } catch {
                            toast('Blacklist update failed', 'error');
                          }
                        }}
                      >
                        Add to Blacklist
                      </button>
                      <button
                        className="w-full rounded-lg px-3 py-2 text-left text-sm text-red-500 transition-colors hover:bg-red-500/10"
                        onClick={async () => {
                          setShowMoreMenu(false);
                          try {
                            await contactService.deleteContact(user.id);
                            toast('Contact deleted', 'success');
                          } catch {
                            toast('Delete contact failed', 'error');
                          }
                        }}
                      >
                        Delete
                      </button>
                    </motion.div>
                  </>
                )}
              </div>
            </div>

            <motion.div
              key={user.id}
              initial={{ scale: 0.9, opacity: 0 }}
              animate={{ scale: 1, opacity: 1 }}
              className="relative z-10 transition-transform"
            >
              <Avatar
                src={user.avatar}
                alt={user.name}
                className="mb-5 h-[100px] w-[100px] rounded-[2rem] border border-white/10 bg-[#2b2b2d] shadow-xl"
              />
              <div
                className={cn(
                  'absolute bottom-0 right-0 h-5 w-5 rounded-full border-4 border-[#1e1e1e]',
                  user.status === 'online' ? 'bg-green-500' : 'bg-gray-500',
                )}
                title={user.status}
              />
            </motion.div>

            <h2 className="z-10 mb-1 flex items-center gap-2 text-2xl font-semibold text-gray-100">
              {user.name}
            </h2>
            <div className="z-10 mb-6 text-sm font-medium text-indigo-400">{user.position || 'Unknown position'}</div>

            <div className="z-10 flex w-full items-center justify-center gap-3">
              <button
                onClick={() => {
                  if (onSendMessage) {
                    onSendMessage(user);
                  } else {
                    toast('Messaging is unavailable', 'error');
                  }
                }}
                className="flex flex-1 items-center justify-center gap-2 rounded-xl bg-indigo-600 py-2.5 font-medium text-white shadow-lg shadow-indigo-500/20 transition-all hover:bg-indigo-500 active:scale-[0.98]"
              >
                <MessageSquare size={18} />
                Message
              </button>
              <div className="flex gap-2">
                <button
                  onClick={startVoiceCall}
                  className="flex h-11 w-11 items-center justify-center rounded-xl border border-white/5 bg-white/5 text-gray-300 transition-all hover:bg-white/10 active:scale-[0.98]"
                  title="Voice call"
                >
                  <Phone size={18} />
                </button>
                <button
                  onClick={startVideoCall}
                  className="flex h-11 w-11 items-center justify-center rounded-xl border border-white/5 bg-white/5 text-gray-300 transition-all hover:bg-white/10 active:scale-[0.98]"
                  title="Video call"
                >
                  <Video size={18} />
                </button>
              </div>
            </div>
          </div>

          <div className="custom-scrollbar flex flex-1 flex-col gap-6 overflow-y-auto p-8">
            <div className="space-y-4">
              <h3 className="mb-2 text-xs font-semibold uppercase tracking-wider text-gray-500">Basic Info</h3>
              <div className="flex flex-col gap-3">
                <div className="flex items-start gap-4 rounded-xl p-3 transition-colors hover:bg-white/5">
                  <Hash size={18} className="mt-0.5 shrink-0 text-gray-500" />
                  <div className="min-w-0">
                    <div className="mb-0.5 text-xs text-gray-500">Chat ID</div>
                    <div className="break-all font-mono text-sm text-gray-200">{displayUserChatId}</div>
                  </div>
                </div>
                <div className="flex items-start gap-4 rounded-xl p-3 transition-colors hover:bg-white/5">
                  <Building2 size={18} className="mt-0.5 shrink-0 text-gray-500" />
                  <div>
                    <div className="mb-0.5 text-xs text-gray-500">Department</div>
                    <div className="text-sm text-gray-200">{departmentName}</div>
                  </div>
                </div>
                {user.company && (
                  <div className="flex items-start gap-4 rounded-xl p-3 transition-colors hover:bg-white/5">
                    <Building2 size={18} className="mt-0.5 shrink-0 text-gray-500" />
                    <div>
                      <div className="mb-0.5 text-xs text-gray-500">Company</div>
                      <div className="text-sm text-gray-200">{user.company}</div>
                    </div>
                  </div>
                )}
                {user.location && (
                  <div className="flex items-start gap-4 rounded-xl p-3 transition-colors hover:bg-white/5">
                    <Hash size={18} className="mt-0.5 shrink-0 text-gray-500" />
                    <div>
                      <div className="mb-0.5 text-xs text-gray-500">Location</div>
                      <div className="text-sm text-gray-200">{user.location}</div>
                    </div>
                  </div>
                )}
                {user.motto && (
                  <div className="flex items-start gap-4 rounded-xl p-3 transition-colors hover:bg-white/5">
                    <MessageSquare size={18} className="mt-0.5 shrink-0 text-gray-500" />
                    <div>
                      <div className="mb-0.5 text-xs text-gray-500">Signature</div>
                      <div className="text-sm italic text-gray-200">"{user.motto}"</div>
                    </div>
                  </div>
                )}
                {user.email && (
                  <div
                    className="group flex cursor-pointer items-start gap-4 rounded-xl p-3 transition-colors hover:bg-white/5"
                    onClick={() => (onAppSelect ? onAppSelect('mail') : toast('Mail app selected', 'success'))}
                  >
                    <Mail size={18} className="mt-0.5 shrink-0 text-gray-500" />
                    <div className="flex-1">
                      <div className="mb-0.5 text-xs text-gray-500">Email</div>
                      <div className="text-sm text-gray-200">{user.email}</div>
                    </div>
                    <ChevronRight size={16} className="text-gray-600 opacity-0 transition-opacity group-hover:opacity-100" />
                  </div>
                )}
                {user.phone && (
                  <div
                    className="group flex cursor-pointer items-start gap-4 rounded-xl p-3 transition-colors hover:bg-white/5"
                    onClick={startVoiceCall}
                  >
                    <Phone size={18} className="mt-0.5 shrink-0 text-gray-500" />
                    <div className="flex-1">
                      <div className="mb-0.5 text-xs text-gray-500">Phone</div>
                      <div className="font-mono text-sm text-gray-200">{user.phone}</div>
                    </div>
                    <ChevronRight size={16} className="text-gray-600 opacity-0 transition-opacity group-hover:opacity-100" />
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
