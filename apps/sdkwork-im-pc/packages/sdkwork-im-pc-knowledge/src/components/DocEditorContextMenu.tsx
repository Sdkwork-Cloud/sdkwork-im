import React from 'react';
import { Sparkles, Edit2, Columns, Eye, Bot } from 'lucide-react';

interface DocEditorContextMenuProps {
  contextMenu: { x: number, y: number } | null;
  setContextMenu: (menu: { x: number, y: number } | null) => void;
  contextMenuInstruction: string;
  setContextMenuInstruction: (instruction: string) => void;
  executeAiAction: (action: string, instruction?: string) => void;
}

export const DocEditorContextMenu: React.FC<DocEditorContextMenuProps> = ({
  contextMenu,
  setContextMenu,
  contextMenuInstruction,
  setContextMenuInstruction,
  executeAiAction
}) => {
  if (!contextMenu) return null;

  return (
    <div 
      className="fixed z-[100] w-[320px] bg-white dark:bg-[#1e1e1e] border border-gray-200 dark:border-white/10 shadow-2xl rounded-xl overflow-hidden flex flex-col"
      style={{ top: contextMenu.y, left: contextMenu.x }}
      onClick={e => e.stopPropagation()}
      onMouseDown={e => e.stopPropagation()}
    >
        <div className="p-3 border-b border-white/5 flex items-center gap-2 bg-[#2a2a2a]/50">
          <Sparkles size={16} className="text-indigo-400 shrink-0" />
          <input 
            autoFocus
            className="w-full bg-transparent border-none text-gray-200 text-sm py-1 outline-none placeholder:text-gray-500"
            placeholder="给 AI 的指令..."
            value={contextMenuInstruction}
            onChange={e => setContextMenuInstruction(e.target.value)}
            onKeyDown={e => {
              if (e.key === 'Enter' && contextMenuInstruction.trim()) {
                executeAiAction('instruct', contextMenuInstruction);
                setContextMenu(null);
              } else if (e.key === 'Escape') {
                setContextMenu(null);
              }
            }}
          />
        </div>
        <div className="p-1.5 flex flex-col max-h-[200px] overflow-y-auto custom-scrollbar">
          <div className="px-3 md:px-3 py-1.5 text-[10px] font-semibold text-gray-500/80 uppercase tracking-widest">快捷建议</div>
          <button onMouseDown={(e) => { e.preventDefault(); executeAiAction('instruct', '请帮我创作一段文字'); setContextMenu(null); }} className="px-3 py-2 text-left text-sm hover:bg-indigo-500/10 text-gray-300 rounded-lg transition-colors flex items-center gap-2 m-0.5"><Edit2 size={14} className="text-gray-400"/> 帮我创作</button>
          <button onMouseDown={(e) => { e.preventDefault(); executeAiAction('instruct', '根据上文继续扩写'); setContextMenu(null); }} className="px-3 py-2 text-left text-sm hover:bg-indigo-500/10 text-gray-300 rounded-lg transition-colors flex items-center gap-2 m-0.5"><Columns size={14} className="text-gray-400"/> 继续扩写</button>
          <button onMouseDown={(e) => { e.preventDefault(); executeAiAction('instruct', '总结上文的内容'); setContextMenu(null); }} className="px-3 py-2 text-left text-sm hover:bg-indigo-500/10 text-gray-300 rounded-lg transition-colors flex items-center gap-2 m-0.5"><Eye size={14} className="text-gray-400"/> 总结上文</button>
          <button onMouseDown={(e) => { e.preventDefault(); executeAiAction('instruct', '将上文翻译为英文'); setContextMenu(null); }} className="px-3 py-2 text-left text-sm hover:bg-indigo-500/10 text-gray-300 rounded-lg transition-colors flex items-center gap-2 m-0.5"><Bot size={14} className="text-gray-400"/> 翻译上文</button>
        </div>
    </div>
  );
};
