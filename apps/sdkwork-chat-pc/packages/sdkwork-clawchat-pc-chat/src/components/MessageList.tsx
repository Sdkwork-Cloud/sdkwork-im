import React, { useState, useEffect, useRef } from 'react';
import { motion, AnimatePresence } from 'motion/react';
import { contactService } from '../services/ContactService';
import { chatService } from '../services/ChatService';
import { favoriteService } from '../services/FavoriteService';
import { parseGroupInviteDescriptor } from '../services/GroupService';
import { Avatar, MediaViewer } from '@sdkwork/clawchat-pc-commons';
import { cn } from '@sdkwork/clawchat-pc-commons';
import type { Message, User } from '@sdkwork/clawchat-pc-types';
import { ContextMenu, ContextMenuItem } from './ContextMenu';
import { Copy, Reply, Forward, CheckSquare, Trash2, X, Check, Play, FileText, LayoutTemplate, Volume2, Video, Phone, Download, Smile, Star } from 'lucide-react';
import { toast } from './Toast';
import { ForwardModal } from './ForwardModal';
import { TextMessageItem, ImageMessageItem, VideoMessageItem, VoiceMessageItem, VideoCallMessageItem, LinkMessageItem, AppletMessageItem, CardMessageItem, FileMessageItem, MusicMessageItem } from './MessageItems';

interface MessageListProps {
  chatId: string;
  fallbackMessages?: Message[];
  refreshKey?: number;
  searchQuery?: string;
  senderProfiles?: Record<string, User>;
  onReply?: (msg: Message, senderName: string) => void;
  onOpenGroupInvite?: (groupId: string) => Promise<void>;
}

const EMPTY_MESSAGES: Message[] = [];
const EMPTY_SENDER_PROFILES: Record<string, User> = {};
const RTC_CALL_DESCRIPTOR_PREFIX = 'rtc-call:';

type RtcCallDisplayState = 'accepted' | 'ended' | 'rejected' | 'started' | 'syncing';

interface RtcCallDescriptor {
  actorId?: string;
  initiatorId?: string;
  mode?: string;
  receiverId?: string;
  signalType: string;
  state: RtcCallDisplayState;
}

function mergeDisplayMessages(messages: Message[], fallbackMessages: Message[]): Message[] {
  if (fallbackMessages.length === 0) {
    return messages;
  }

  const messageIds = new Set(messages.map((message) => message.id));
  return [
    ...fallbackMessages.filter((message) => !messageIds.has(message.id)),
    ...messages,
  ].sort((left, right) => left.timestamp - right.timestamp);
}

function parseJsonRecord(value: unknown): Record<string, unknown> | undefined {
  if (value && typeof value === 'object' && !Array.isArray(value)) {
    return value as Record<string, unknown>;
  }
  if (typeof value !== 'string' || value.trim().length === 0) {
    return undefined;
  }
  try {
    const parsed: unknown = JSON.parse(value);
    return parsed && typeof parsed === 'object' && !Array.isArray(parsed)
      ? parsed as Record<string, unknown>
      : undefined;
  } catch {
    return undefined;
  }
}

function pickString(...values: unknown[]): string | undefined {
  for (const value of values) {
    if (typeof value === 'string' && value.trim().length > 0) {
      return value.trim();
    }
  }
  return undefined;
}

function readRtcCallDescriptor(message: Message): RtcCallDescriptor | undefined {
  if (message.type !== 'video_call' || !message.desc?.startsWith(RTC_CALL_DESCRIPTOR_PREFIX)) {
    return undefined;
  }

  try {
    const parsed = parseJsonRecord(decodeURIComponent(message.desc.slice(RTC_CALL_DESCRIPTOR_PREFIX.length)));
    const state = pickString(parsed?.state);
    const signalType = pickString(parsed?.signalType) ?? 'rtc.signal';
    if (
      state === 'accepted'
      || state === 'ended'
      || state === 'rejected'
      || state === 'started'
      || state === 'syncing'
    ) {
      return {
        actorId: pickString(parsed?.actorId),
        initiatorId: pickString(parsed?.initiatorId),
        mode: pickString(parsed?.mode),
        receiverId: pickString(parsed?.receiverId),
        signalType,
        state,
      };
    }
  } catch {
    return undefined;
  }

  return undefined;
}

function collectRtcParticipantIds(messages: Message[]): string[] {
  const participantIds = new Set<string>();
  for (const message of messages) {
    const descriptor = readRtcCallDescriptor(message);
    if (!descriptor) {
      continue;
    }
    for (const participantId of [descriptor.actorId, descriptor.initiatorId, descriptor.receiverId]) {
      if (participantId) {
        participantIds.add(participantId);
      }
    }
  }
  return Array.from(participantIds);
}

function isVideoRtcMode(value: string | undefined): boolean {
  return Boolean(value && /video/iu.test(value));
}

function formatRtcCallMode(value: string | undefined): string {
  return isVideoRtcMode(value) ? '视频通话' : '语音通话';
}

function replaceParticipantId(content: string, participantId: string | undefined, displayName: string): string {
  if (!participantId || participantId === displayName) {
    return content;
  }
  return content.split(participantId).join(displayName);
}

function formatVideoCallMessageContent(
  message: Message,
  resolveDisplayName: (participantId: string | undefined, fallback: string) => string,
): string {
  const descriptor = readRtcCallDescriptor(message);
  if (!descriptor) {
    return message.content;
  }

  const mode = formatRtcCallMode(descriptor.mode);
  const initiator = resolveDisplayName(descriptor.initiatorId ?? message.senderId, '发起方');
  const receiver = descriptor.receiverId ? resolveDisplayName(descriptor.receiverId, descriptor.receiverId) : undefined;
  const actor = resolveDisplayName(descriptor.actorId, '对方');
  const callSubject = receiver
    ? `${initiator} 向 ${receiver} 发起的${mode}`
    : `${initiator} 发起的${mode}`;

  switch (descriptor.state) {
    case 'accepted':
      return `${callSubject}，${actor} 已接通`;
    case 'rejected':
      return `${callSubject}，${actor} 已拒绝`;
    case 'ended':
      return `${callSubject}，${actor} 已挂断`;
    case 'started':
      return receiver
        ? `${initiator} 向 ${receiver} 发起了${mode}`
        : `${initiator} 发起了${mode}`;
    case 'syncing':
    default:
      return `${callSubject}正在同步`;
  }
}

function formatVideoCallMessage(
  message: Message,
  resolveDisplayName: (participantId: string | undefined, fallback: string) => string,
): Message {
  if (message.type !== 'video_call') {
    return message;
  }
  const descriptor = readRtcCallDescriptor(message);
  if (descriptor) {
    return {
      ...message,
      content: formatVideoCallMessageContent(message, resolveDisplayName),
    };
  }

  let content = message.content;
  for (const participantId of [message.senderId]) {
    content = replaceParticipantId(content, participantId, resolveDisplayName(participantId, participantId));
  }
  return { ...message, content };
}

function isCurrentUserMessage(message: Message, currentUser: User | null): boolean {
  if (!currentUser) {
    return false;
  }
  return message.senderId === currentUser.id || message.senderId === currentUser.chatId;
}

export const MessageList: React.FC<MessageListProps> = ({
  chatId,
  fallbackMessages = EMPTY_MESSAGES,
  refreshKey = 0,
  searchQuery = '',
  senderProfiles = EMPTY_SENDER_PROFILES,
  onReply,
  onOpenGroupInvite,
}) => {
  const [messages, setMessages] = useState<Message[]>([]);
  const [usersMap, setUsersMap] = useState<Record<string, User>>({});
  const [currentUser, setCurrentUser] = useState<User | null>(null);
  const [loading, setLoading] = useState(true);
  const [contextMenu, setContextMenu] = useState<{x: number, y: number, msg: Message} | null>(null);
  const [isMultiSelect, setIsMultiSelect] = useState(false);
  const [selectedIds, setSelectedIds] = useState<Set<string>>(new Set());
  const [forwardMessages, setForwardMessages] = useState<Message[]>([]);
  const [isForwardModalOpen, setIsForwardModalOpen] = useState(false);
  const messagesEndRef = useRef<HTMLDivElement>(null);

  const [viewerState, setViewerState] = useState({ isOpen: false, currentIndex: 0 });
  const fallbackMessageIds = React.useMemo(
    () => new Set(fallbackMessages.map((message) => message.id)),
    [fallbackMessages],
  );

  const scrollToBottom = () => {
    messagesEndRef.current?.scrollIntoView({ behavior: 'smooth' });
  };

  useEffect(() => {
    setCurrentUser(contactService.getCurrentUser());
    contactService.getContacts()
      .then(users => {
        const map: Record<string, User> = {};
        users.forEach(u => {
          map[u.id] = u;
          if (u.chatId) {
            map[u.chatId] = u;
          }
        });
        setUsersMap(map);
      })
      .catch(() => {
        setUsersMap({});
      });
  }, []);

  useEffect(() => {
    let isMounted = true;
    const loadMessages = async () => {
      setLoading(true);
      try {
        const data = await chatService.getMessages(chatId);
        if (!isMounted) {
          return;
        }
        setMessages(mergeDisplayMessages(data, fallbackMessages));
        setTimeout(scrollToBottom, 50);
      } catch {
        if (isMounted) {
          setMessages(fallbackMessages);
          toast('加载消息失败', 'error');
        }
      } finally {
        if (isMounted) {
          setLoading(false);
        }
      }
    };
    void loadMessages();
    return () => {
      isMounted = false;
    };
  }, [chatId, fallbackMessages, refreshKey]);

  useEffect(() => {
    const unsubscribe = chatService.subscribeMessages(chatId, (message) => {
      setMessages(prev => {
        const byId = new Map(prev.map(item => [item.id, item]));
        byId.set(message.id, { ...byId.get(message.id), ...message });
        return mergeDisplayMessages(
          Array.from(byId.values()).sort((left, right) => left.timestamp - right.timestamp),
          fallbackMessages,
        );
      });
    });

    return () => {
      unsubscribe();
    };
  }, [chatId, fallbackMessages]);

  useEffect(() => {
    scrollToBottom();
  }, [messages]);

  useEffect(() => {
    let isMounted = true;
    const missingParticipantIds = collectRtcParticipantIds(messages)
      .filter((participantId) => {
        if (currentUser && (participantId === currentUser.id || participantId === currentUser.chatId)) {
          return false;
        }
        return !usersMap[participantId] && !senderProfiles[participantId];
      });

    if (missingParticipantIds.length === 0) {
      return () => {
        isMounted = false;
      };
    }

    void Promise.all(missingParticipantIds.map((participantId) => contactService.getUserById(participantId)))
      .then((users) => {
        if (!isMounted) {
          return;
        }
        const resolvedUsers = users.filter((user): user is User => Boolean(user));
        if (resolvedUsers.length === 0) {
          return;
        }
        setUsersMap((previousUsersMap) => {
          const nextUsersMap = { ...previousUsersMap };
          for (const user of resolvedUsers) {
            nextUsersMap[user.id] = user;
            if (user.chatId) {
              nextUsersMap[user.chatId] = user;
            }
          }
          return nextUsersMap;
        });
      })
      .catch(() => undefined);

    return () => {
      isMounted = false;
    };
  }, [currentUser, messages, senderProfiles, usersMap]);

  const handleContextMenu = (e: React.MouseEvent, msg: Message) => {
    e.preventDefault();
    if (isMultiSelect) return;
    setContextMenu({ x: e.clientX, y: e.clientY, msg });
  };

  const handleDelete = async (idsToDelete: Set<string>) => {
    try {
      await Promise.all(Array.from(idsToDelete).map((messageId) => chatService.deleteMessage(chatId, messageId)));
      setMessages(prev => prev.filter(msg => !idsToDelete.has(msg.id)));
      toast(idsToDelete.size > 1 ? `已删除 ${idsToDelete.size} 条消息` : '消息已删除', 'success');
      setIsMultiSelect(false);
      setSelectedIds(new Set());
    } catch {
      toast('删除消息失败', 'error');
    }
  };

  const handleForward = (messagesToForward: Message[]) => {
    setForwardMessages(messagesToForward);
    setIsForwardModalOpen(true);
    setIsMultiSelect(false);
    setSelectedIds(new Set());
  };

  const handleGroupInviteClick = async (msg: Message) => {
    const descriptor = parseGroupInviteDescriptor(msg);
    if (!descriptor || !onOpenGroupInvite) {
      return;
    }
    try {
      await onOpenGroupInvite(descriptor.groupId);
    } catch {
      toast('打开群聊失败', 'error');
    }
  };

  const getContextMenuItems = (): ContextMenuItem[] => {
    if (!contextMenu) return [];
    const sender = usersMap[contextMenu.msg.senderId];
    const isFallbackMessage = fallbackMessageIds.has(contextMenu.msg.id);
    const copyItem: ContextMenuItem = {
      id: 'copy',
      label: '复制',
      icon: <Copy size={14} />,
      onClick: () => {
        navigator.clipboard.writeText(contextMenu.msg.content);
        toast('已复制', 'success');
      },
    };
    if (isFallbackMessage) {
      return [copyItem];
    }
    return [
      copyItem,
      { id: 'reply', label: '回复', icon: <Reply size={14} />, onClick: () => { if (onReply) onReply(contextMenu.msg, sender?.name || '未知用户'); } },
      { id: 'reaction', label: '表情回应', icon: <Smile size={14} />, onClick: () => { void handleReaction(contextMenu.msg.id, '👍'); } },
      { id: 'forward', label: '转发', icon: <Forward size={14} />, onClick: () => handleForward([contextMenu.msg]) },
      { id: 'favorite', label: '收藏', icon: <Star size={14} />, onClick: async () => {
          try {
            await favoriteService.addFavorite({
               type: contextMenu.msg.type === 'link' || contextMenu.msg.type === 'music' ? 'link' : contextMenu.msg.type === 'image' || contextMenu.msg.type === 'video' ? 'image' : contextMenu.msg.type === 'file' ? 'file' : 'chat',
               title: contextMenu.msg.fileName || contextMenu.msg.content.substring(0, 20),
               content: contextMenu.msg.content,
               conversationId: contextMenu.msg.chatId ?? chatId,
               messageId: contextMenu.msg.id,
               source: sender?.name || '未知用户'
            });
            toast('已收藏', 'success');
          } catch {
            toast('收藏失败', 'error');
          }
      } },
      { id: 'select', label: '多选', icon: <CheckSquare size={14} />, onClick: () => { setIsMultiSelect(true); setSelectedIds(new Set([contextMenu.msg.id])); } },
      { id: 'div1', label: '', divider: true, onClick: () => {} },
      { id: 'delete', label: '删除', icon: <Trash2 size={14} />, danger: true, onClick: () => handleDelete(new Set([contextMenu.msg.id])) },
    ];
  };

  const toggleSelect = (id: string) => {
    const next = new Set(selectedIds);
    if (next.has(id)) next.delete(id);
    else next.add(id);
    setSelectedIds(next);
  };

  const formatTime = (timestamp: number) => {
    const date = new Date(timestamp);
    return `${date.getHours().toString().padStart(2, '0')}:${date.getMinutes().toString().padStart(2, '0')}`;
  };

  const mediaMessages = messages.filter(m => m.type === 'image' || m.type === 'video');
  const mediaItems = mediaMessages.map(m => ({
    id: m.id,
    type: m.type as 'image' | 'video',
    src: m.content || '',
    name: m.fileName || (m.type === 'image' ? '图片' : '视频')
  }));

  const handleMediaClick = (msg: Message) => {
    const index = mediaMessages.findIndex(m => m.id === msg.id);
    if (index !== -1) {
      setViewerState({ isOpen: true, currentIndex: index });
    }
  };

  const handleReaction = async (messageId: string, emoji: string) => {
    const msg = messages.find(m => m.id === messageId);
    if (!msg) return;
    
    try {
      const reaction = msg.reactions?.find(r => r.emoji === emoji);
      if (reaction && reaction.hasReacted) {
        await chatService.removeReaction(chatId, messageId, emoji);
      } else {
        await chatService.addReaction(chatId, messageId, emoji);
      }
    } catch {
      toast('表情回应失败', 'error');
      return;
    }
    
    // Optimistic update
    setMessages(prev => prev.map(m => {
      if (m.id !== messageId) return m;
      const nextReactions = [...(m.reactions || [])];
      const rIdx = nextReactions.findIndex(r => r.emoji === emoji);
      if (rIdx >= 0) {
        if (nextReactions[rIdx].hasReacted) {
           nextReactions[rIdx].count--;
           nextReactions[rIdx].hasReacted = false;
           if (nextReactions[rIdx].count <= 0) nextReactions.splice(rIdx, 1);
        } else {
           nextReactions[rIdx].count++;
           nextReactions[rIdx].hasReacted = true;
        }
      } else {
        nextReactions.push({ emoji, count: 1, hasReacted: true });
      }
      return { ...m, reactions: nextReactions };
    }));
  };

  const filteredMessages = React.useMemo(() => {
    return messages.filter(msg => {
      if (!searchQuery.trim()) return true;
      return msg.content?.toLowerCase().includes(searchQuery.toLowerCase()) || msg.fileName?.toLowerCase().includes(searchQuery.toLowerCase());
    });
  }, [messages, searchQuery]);

  return (
    <div className="flex-1 min-h-0 overflow-y-auto p-6 flex flex-col bg-[#1e1e1e] custom-scrollbar relative">
      {loading && <div className="text-center text-[12px] text-gray-500 my-4">加载中...</div>}
      {!loading && filteredMessages.length > 0 && <div className="text-center text-[12px] text-gray-500 my-4">{formatTime(filteredMessages[0].timestamp)}</div>}
      
      <AnimatePresence initial={false}>
      {filteredMessages.map((msg, index) => {
        const isMe = isCurrentUserMessage(msg, currentUser);
        const sender = isMe ? currentUser : (
          senderProfiles[msg.senderId] ?? usersMap[msg.senderId]
        );
        const showTime = index === 0 || msg.timestamp - filteredMessages[index - 1].timestamp > 1000 * 60 * 5;
        const resolveDisplayName = (participantId: string | undefined, fallback: string) => {
          if (!participantId) {
            return fallback;
          }
          if (currentUser && (participantId === currentUser.id || participantId === currentUser.chatId)) {
            return currentUser.name;
          }
          return senderProfiles[participantId]?.name
            ?? usersMap[participantId]?.name
            ?? fallback;
        };
        const displayMessage = formatVideoCallMessage(msg, resolveDisplayName);

        return (
          <React.Fragment key={msg.id}>
            {showTime && index > 0 && (
              <div className="text-center text-[12px] text-gray-500 my-4">{formatTime(msg.timestamp)}</div>
            )}
            
            <motion.div 
              id={`msg-${msg.id}`}
              initial={{ opacity: 0, y: 10 }}
              animate={{ opacity: 1, y: 0 }}
              transition={{ duration: 0.2 }}
              className={cn(
                "flex mb-6 group relative rounded-lg transition-all duration-300",
                isMe ? "flex-row-reverse" : "flex-row",
                isMultiSelect && "hover:bg-white/5 cursor-pointer p-2 -mx-2"
              )}
              onClick={() => isMultiSelect && toggleSelect(msg.id)}
              onContextMenu={(e) => handleContextMenu(e, msg)}
            >
              {isMultiSelect && (
                <div className={cn("flex items-center justify-center w-8 shrink-0", isMe ? "ml-2" : "mr-2")}>
                  <div className={cn("w-5 h-5 rounded-full border flex items-center justify-center transition-colors", selectedIds.has(msg.id) ? "bg-[#00b42a] border-[#00b42a]" : "border-gray-500")}>
                    {selectedIds.has(msg.id) && <Check size={12} className="text-white" />}
                  </div>
                </div>
              )}
              <Avatar src={sender?.avatar} alt={sender?.name} className={cn("w-[36px] h-[36px] rounded shrink-0 bg-[#2b2b2d] text-white text-[12px] mt-1", isMe ? "ml-3" : "mr-3")} />
              
              <div className={cn("flex flex-col flex-1 min-w-0", isMe ? "items-end" : "items-start")}>
                {!isMe && sender?.name && (
                  <div className="flex items-center gap-2 mb-1 px-1">
                    <span className="text-[12px] text-gray-400 font-medium">{sender.name}</span>
                  </div>
                )}
                
                {msg.replyTo && (
                  <div 
                    onClick={(e) => {
                       e.stopPropagation();
                       const el = document.getElementById(`msg-${msg.replyTo!.id}`);
                       if (el) {
                         el.scrollIntoView({ behavior: 'smooth', block: 'center' });
                         el.classList.add('ring-2', 'ring-[#00b42a]', 'ring-opacity-50', 'bg-white/5');
                         setTimeout(() => {
                           el.classList.remove('ring-2', 'ring-[#00b42a]', 'ring-opacity-50', 'bg-white/5');
                         }, 2000);
                       }
                    }}
                    className={cn("mb-1.5 px-3 py-1.5 bg-white/5 border-gray-500 rounded text-[12px] text-gray-400 max-w-full truncate cursor-pointer hover:bg-white/10 transition-colors", isMe ? "border-r-2" : "border-l-2")}>
                    <span className="font-medium mr-1">{msg.replyTo.senderName}:</span>
                    {msg.replyTo.content}
                  </div>
                )}

                <div className="relative flex items-center">
                  <div>
                    {msg.type === 'text' && <TextMessageItem msg={msg} isMe={isMe} />}
                    {msg.type === 'image' && <ImageMessageItem msg={msg} isMe={isMe} onMediaClick={handleMediaClick} />}
                    {msg.type === 'video' && <VideoMessageItem msg={msg} isMe={isMe} onMediaClick={handleMediaClick} />}
                    {msg.type === 'voice' && <VoiceMessageItem msg={msg} isMe={isMe} />}
                    {msg.type === 'video_call' && <VideoCallMessageItem msg={displayMessage} isMe={isMe} />}
                    {msg.type === 'link' && <LinkMessageItem msg={msg} isMe={isMe} />}
                    {msg.type === 'applet' && <AppletMessageItem msg={msg} isMe={isMe} />}
                    {msg.type === 'card' && <CardMessageItem
                      msg={msg}
                      isMe={isMe}
                      onClick={parseGroupInviteDescriptor(msg) ? () => {
                        void handleGroupInviteClick(msg);
                      } : undefined}
                    />}
                    {msg.type === 'file' && <FileMessageItem msg={msg} isMe={isMe} />}
                    {msg.type === 'music' && <MusicMessageItem msg={msg} isMe={isMe} allMessages={messages} />}
                  </div>
                </div>

                {msg.reactions && msg.reactions.length > 0 && (
                  <div className={cn("flex flex-wrap gap-1 mt-1", isMe ? "justify-end" : "justify-start")}>
                    {msg.reactions.map(r => (
                      <button 
                         key={r.emoji}
                         onClick={() => handleReaction(msg.id, r.emoji)}
                         className={cn(
                           "flex items-center gap-1 px-2 py-0.5 rounded-full text-xs transition-colors border",
                           r.hasReacted 
                             ? "bg-indigo-500/20 text-indigo-300 border-indigo-500/30" 
                             : "bg-white/5 text-gray-400 border-white/5 hover:bg-white/10"
                         )}
                      >
                         <span>{r.emoji}</span>
                         <span className="font-medium">{r.count}</span>
                      </button>
                    ))}
                  </div>
                )}
              </div>
            </motion.div>
          </React.Fragment>
        );
      })}
      </AnimatePresence>

      {isMultiSelect && (
        <div className="sticky bottom-4 left-1/2 -translate-x-1/2 w-max bg-[#2b2b2d] border border-white/10 rounded-full shadow-2xl px-6 py-3 flex items-center gap-6 z-50 mx-auto mt-auto">
          <span className="text-sm text-gray-300">已选择 {selectedIds.size} 条</span>
          <div className="w-px h-4 bg-white/10" />
          <button 
            className="flex items-center gap-2 text-sm text-gray-300 hover:text-white transition-colors disabled:opacity-50" 
            onClick={() => handleForward(messages.filter(m => selectedIds.has(m.id)))}
            disabled={selectedIds.size === 0}
          >
            <Forward size={16} /> 转发
          </button>
          <button 
            className="flex items-center gap-2 text-sm text-red-400 hover:text-red-300 transition-colors disabled:opacity-50" 
            onClick={() => handleDelete(selectedIds)}
            disabled={selectedIds.size === 0}
          >
            <Trash2 size={16} /> 删除
          </button>
          <div className="w-px h-4 bg-white/10" />
          <button className="p-1 text-gray-400 hover:text-white transition-colors" onClick={() => { setIsMultiSelect(false); setSelectedIds(new Set()); }}>
            <X size={18} />
          </button>
        </div>
      )}

      {contextMenu && (
        <ContextMenu 
          x={contextMenu.x} 
          y={contextMenu.y} 
          items={getContextMenuItems()} 
          onClose={() => setContextMenu(null)} 
        />
      )}

      <ForwardModal 
        isOpen={isForwardModalOpen} 
        onClose={() => setIsForwardModalOpen(false)} 
        messages={forwardMessages} 
      />
      
      <MediaViewer 
        isOpen={viewerState.isOpen}
        items={mediaItems}
        currentIndex={viewerState.currentIndex}
        onIndexChange={(idx) => setViewerState(prev => ({ ...prev, currentIndex: idx }))}
        onClose={() => setViewerState(prev => ({ ...prev, isOpen: false }))}
      />

      <div ref={messagesEndRef} />
    </div>
  );
};
