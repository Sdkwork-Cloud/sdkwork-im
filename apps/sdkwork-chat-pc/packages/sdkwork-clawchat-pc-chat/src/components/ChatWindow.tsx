import React, { useState } from 'react';
import { motion, AnimatePresence } from 'motion/react';
import { Chat, Message } from '@sdkwork/clawchat-pc-types';
import { MessageList } from './MessageList';
import { MessageInput } from './MessageInput';
import { chatService } from '../services/ChatService';
import { toast } from './Toast';
import { ChatHistoryModal } from './ChatHistoryModal';

interface ChatWindowProps {
  chat: Chat;
  messageSearchQuery?: string;
}

export const ChatWindow: React.FC<ChatWindowProps> = ({ chat, messageSearchQuery = '' }) => {
  const [refreshKey, setRefreshKey] = useState(0);
  const [replyingTo, setReplyingTo] = useState<Message['replyTo'] | undefined>();
  const [isHistoryOpen, setIsHistoryOpen] = useState(false);
  const [isTyping, setIsTyping] = useState(false);

  const handleSend = async (content: string, type: Message['type'] = 'text', extraInfo?: Partial<Message>) => {
    try {
      await chatService.sendMessage(chat.id, content, type, replyingTo, extraInfo);
      setReplyingTo(undefined);
      setRefreshKey(prev => prev + 1);
    } catch (error) {
      toast('\u53d1\u9001\u5931\u8d25', 'error');
    }
  };

  return (
    <div className="flex-1 flex flex-col bg-[#1e1e1e] min-w-0 min-h-0 relative">
      {/* Messages */}
      <MessageList
        chatId={chat.id}
        refreshKey={refreshKey}
        searchQuery={messageSearchQuery}
        onReply={(msg, senderName) => setReplyingTo({ id: msg.id, senderName, content: msg.content })}
      />

      {/* Typing Indicator */}
      <div className="relative w-full z-10 pointer-events-none">
        <AnimatePresence>
          {isTyping && (
            <motion.div
              initial={{ opacity: 0, y: 10 }}
              animate={{ opacity: 1, y: 0 }}
              exit={{ opacity: 0, scale: 0.95 }}
              className="absolute bottom-4 left-8 flex items-center gap-2 bg-[#2b2b2d] px-4 py-2 rounded-2xl rounded-tl-sm border border-white/5 shadow-sm max-w-max pointer-events-auto"
            >
              <div className="flex gap-1.5 items-center justify-center h-4">
                 <div className="w-1.5 h-1.5 bg-gray-400 rounded-full animate-bounce" style={{ animationDelay: '0ms' }} />
                 <div className="w-1.5 h-1.5 bg-gray-400 rounded-full animate-bounce" style={{ animationDelay: '150ms' }} />
                 <div className="w-1.5 h-1.5 bg-gray-400 rounded-full animate-bounce" style={{ animationDelay: '300ms' }} />
              </div>
              <span className="text-xs text-gray-400 ml-1">{'\u5bf9\u65b9\u6b63\u5728\u8f93\u5165...'}</span>
            </motion.div>
          )}
        </AnimatePresence>
      </div>

      {/* Input Area */}
      <MessageInput
        onSend={handleSend}
        replyingTo={replyingTo}
        isTyping={isTyping}
        onStop={() => {
           setIsTyping(false);
        }}
        onCancelReply={() => setReplyingTo(undefined)}
        onHistoryClick={() => setIsHistoryOpen(true)}
      />

      <ChatHistoryModal
        isOpen={isHistoryOpen}
        onClose={() => setIsHistoryOpen(false)}
        chatId={chat.id}
      />
    </div>
  );
};
