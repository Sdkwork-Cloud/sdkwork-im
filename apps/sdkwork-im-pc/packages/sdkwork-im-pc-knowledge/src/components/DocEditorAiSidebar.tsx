import React from 'react';
import { Bot, Send, Loader2 } from 'lucide-react';

interface DocEditorAiSidebarProps {
  isAiPanelOpen: boolean;
  aiInstruction: string;
  setAiInstruction: (instruction: string) => void;
  aiLoading: boolean;
  executeAiAction: (action: string, instruction?: string) => void;
}

export const DocEditorAiSidebar: React.FC<DocEditorAiSidebarProps> = ({
  isAiPanelOpen,
  aiInstruction,
  setAiInstruction,
  aiLoading,
  executeAiAction
}) => {
  if (!isAiPanelOpen) return null;

  return (
    <div className="w-[320px] lg:w-[380px] bg-[#1a1a1a] border-l border-white/5 flex flex-col min-h-0 shadow-2xl z-20 shrink-0">
      <div className="h-16 flex items-center gap-3 px-6 border-b border-gray-200 dark:border-white/5 shrink-0 bg-white dark:bg-[#1e1e1e]">
        <div className="w-8 h-8 rounded-full bg-indigo-500/20 flex items-center justify-center text-indigo-400">
          <Bot size={18} />
        </div>
        <div>
          <h3 className="text-sm font-medium text-gray-200">AI 写作助手</h3>
          <p className="text-xs text-gray-500">Sdkwork IM 智能体</p>
        </div>
      </div>
      
      <div className="flex-1 overflow-y-auto custom-scrollbar p-6 space-y-6">
        <div className="flex gap-3">
            <div className="w-8 h-8 rounded-full bg-indigo-500/20 flex items-center justify-center text-indigo-400 shrink-0">
              <Bot size={16} />
            </div>
            <div className="bg-[#2a2a2a] p-4 rounded-2xl rounded-tl-sm text-sm text-gray-300 leading-relaxed">
              你好！我是你的专属文档创作助手。我可以帮你：
              <ul className="mt-3 space-y-2 list-disc list-inside text-gray-400">
                <li>根据指令生成大纲甚至正文</li>
                <li>润色和优化你选中的段落</li>
                <li>翻译、精简或扩写内容</li>
              </ul>
              <p className="mt-3 text-indigo-400 text-xs">💡 提示：在左侧编辑器选中文本后，还可以激活快捷悬浮指令。</p>
            </div>
        </div>

        {/* Example Instructions */}
        <div className="grid grid-cols-1 gap-2 pt-2">
            <button onClick={() => { setAiInstruction("请帮我为这篇文档编写一份详细的内容大纲结尾。"); executeAiAction('instruct', "请帮我为这篇文档编写一份详细的内容大纲结尾。") }} className="text-left px-4 py-3 bg-white/5 hover:bg-white/10 border border-white/5 rounded-xl text-sm text-gray-300 transition-colors">
              📝 生成文档大纲
            </button>
            <button onClick={() => { setAiInstruction("帮我检查文档中的错别字和语法错误，并直接修复。"); executeAiAction('instruct', "帮我检查文档中的错别字和语法错误，并直接修复。") }} className="text-left px-4 py-3 bg-white/5 hover:bg-white/10 border border-white/5 rounded-xl text-sm text-gray-300 transition-colors">
              ✨ 检查错别字与语法
            </button>
            <button onClick={() => { setAiInstruction("为当前的文档内容生成一个具有吸引力的标题和摘要。"); executeAiAction('instruct', "为当前的文档内容生成一个具有吸引力的标题和摘要。") }} className="text-left px-4 py-3 bg-white/5 hover:bg-white/10 border border-white/5 rounded-xl text-sm text-gray-300 transition-colors">
              🎯 生成摘要与标题
            </button>
        </div>
      </div>
      
      <div className="p-4 border-t border-gray-200 dark:border-white/5 bg-gray-50 dark:bg-[#1e1e1e]">
        <div className="relative flex items-end">
          <textarea 
            rows={1}
            className="w-full bg-white dark:bg-[#141414] border border-gray-200 dark:border-white/10 text-gray-900 dark:text-gray-200 text-sm rounded-xl py-3 pl-4 pr-12 outline-none focus:border-indigo-500/50 focus:ring-1 focus:ring-indigo-500/50 transition-all placeholder:text-gray-400 dark:placeholder:text-gray-600 resize-none overflow-hidden max-h-32 shadow-sm"
            style={{ minHeight: '46px' }}
            placeholder="输入指令，如：帮我起草一个通知..."
            value={aiInstruction}
            onChange={e => {
              setAiInstruction(e.target.value);
              e.target.style.height = 'auto';
              e.target.style.height = `${Math.min(e.target.scrollHeight, 128)}px`;
            }}
            onKeyDown={e => {
              if (e.key === 'Enter' && !e.shiftKey) {
                e.preventDefault();
                if (aiInstruction.trim() && !aiLoading) {
                  executeAiAction('instruct', aiInstruction);
                }
              }
            }}
            disabled={aiLoading}
          />
          <button 
            className="absolute right-2 bottom-2 h-8 w-8 flex items-center justify-center text-gray-400 hover:text-indigo-400 transition-colors disabled:opacity-50 disabled:hover:text-gray-400"
            disabled={!aiInstruction.trim() || aiLoading}
            onClick={() => executeAiAction('instruct', aiInstruction)}
          >
            {aiLoading ? <Loader2 size={16} className="animate-spin" /> : <Send size={16} />}
          </button>
        </div>
        <p className="text-center text-[10px] text-gray-600 mt-3">生成的内容将直接插入到光标位置</p>
      </div>
    </div>
  );
};
