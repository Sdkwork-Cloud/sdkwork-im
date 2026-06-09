import React from 'react';
import { MessageSquare, Send, X } from 'lucide-react';

interface KnowledgeChatPanelProps {
  selectedBaseName: string;
  chatMessages: { role: 'user' | 'bot'; text: string }[];
  chatInput: string;
  isSending?: boolean;
  setChatInput: (val: string) => void;
  onSendChat: () => void | Promise<void>;
  onClose: () => void;
}

export const KnowledgeChatPanel: React.FC<KnowledgeChatPanelProps> = ({
  selectedBaseName,
  chatMessages,
  chatInput,
  isSending = false,
  setChatInput,
  onSendChat,
  onClose,
}) => {
  return (
    <div className="w-[320px] md:w-[400px] bg-white dark:bg-[#1a1a1a] border-l border-gray-200 dark:border-white/5 flex flex-col shadow-xl z-50 shrink-0">
      <div className="flex items-center justify-between p-4 border-b border-gray-200 dark:border-white/5 shrink-0">
        <div className="flex items-center gap-2">
          <div className="w-8 h-8 rounded-full bg-indigo-100 dark:bg-indigo-500/20 flex items-center justify-center text-indigo-600 dark:text-indigo-400">
            <MessageSquare size={16} />
          </div>
          <h3 className="font-semibold text-gray-900 dark:text-gray-100">Knowledge chat</h3>
        </div>
        <button
          onClick={onClose}
          className="p-1.5 text-gray-400 hover:text-gray-600 dark:hover:text-gray-200 hover:bg-gray-100 dark:hover:bg-white/10 rounded-lg transition-colors"
          aria-label="Close knowledge chat"
        >
          <X size={18} />
        </button>
      </div>

      <div className="flex-1 overflow-y-auto p-4 flex flex-col gap-4">
        {chatMessages.map((msg, idx) => (
          <div key={`${msg.role}-${idx}`} className={`flex ${msg.role === 'user' ? 'justify-end' : 'justify-start'}`}>
            <div className={`max-w-[85%] rounded-2xl px-4 py-2.5 text-sm ${msg.role === 'user' ? 'bg-indigo-600 text-white rounded-br-none' : 'bg-gray-100 dark:bg-[#2a2a2a] text-gray-800 dark:text-gray-200 rounded-bl-none'}`}>
              {msg.text}
            </div>
          </div>
        ))}
      </div>

      <div className="p-4 border-t border-gray-200 dark:border-white/5 shrink-0 bg-gray-50 dark:bg-transparent">
        <div className="flex items-center gap-2 bg-white dark:bg-[#282828] border border-gray-200 dark:border-white/10 rounded-xl p-1.5 focus-within:border-indigo-500/50 focus-within:ring-1 focus-within:ring-indigo-500/50 shadow-sm transition-all text-gray-900 dark:text-gray-200">
          <input
            type="text"
            value={chatInput}
            onChange={(event) => setChatInput(event.target.value)}
            onKeyDown={(event) => {
              if (event.key === 'Enter') {
                event.preventDefault();
                void onSendChat();
              }
            }}
            placeholder={`Ask ${selectedBaseName}...`}
            className="flex-1 bg-transparent px-2 py-1 outline-none text-sm"
          />
          <button
            onClick={() => void onSendChat()}
            disabled={isSending || !chatInput.trim()}
            aria-label="Send knowledge question"
            className="p-1.5 bg-indigo-600 hover:bg-indigo-700 disabled:bg-gray-300 dark:disabled:bg-white/10 text-white rounded-lg transition-colors"
          >
            <Send size={16} />
          </button>
        </div>
      </div>
    </div>
  );
};
