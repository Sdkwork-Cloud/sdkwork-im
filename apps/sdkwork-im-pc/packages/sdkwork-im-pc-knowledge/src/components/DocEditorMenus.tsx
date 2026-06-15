import React from 'react';
import { Editor } from '@tiptap/react';
import { BubbleMenu, FloatingMenu } from '@tiptap/react/menus';
import { Sparkles, Bold, Italic, Code, List, ListOrdered, Loader2, Bot, Columns, Eye, Edit2 } from 'lucide-react';
import { cn } from '@sdkwork/im-pc-commons';

interface DocEditorMenusProps {
  editor: Editor | null;
  setContextMenu: (pos: { x: number, y: number } | null) => void;
  aiLoading: boolean;
  bubbleAiOpen: boolean;
  setBubbleAiOpen: (open: boolean) => void;
  executeAiAction: (action: string, instruction?: string) => void;
}

export const DocEditorMenus: React.FC<DocEditorMenusProps> = ({
  editor,
  setContextMenu,
  aiLoading,
  bubbleAiOpen,
  setBubbleAiOpen,
  executeAiAction
}) => {
  if (!editor) return null;

  return (
    <>
      <FloatingMenu 
        className="flex items-center gap-1 bg-white dark:bg-[#2a2a2a] border border-gray-200 dark:border-white/10 shadow-lg rounded-lg p-1.5 backdrop-blur-md z-40 transition-opacity" 
        editor={editor} 
      >
        <button onMouseDown={(e) => { e.preventDefault(); editor.chain().focus().toggleHeading({ level: 1 }).run(); }} className={cn("p-1.5 hover:text-gray-900 dark:hover:text-gray-200 hover:bg-gray-100 dark:hover:bg-white/5 rounded transition-colors text-xs text-gray-400 font-medium", editor.isActive('heading', { level: 1 }) && 'bg-gray-100 dark:bg-white/10 text-gray-900 dark:text-gray-100')} title="一级标题">H1</button>
        <button onMouseDown={(e) => { e.preventDefault(); editor.chain().focus().toggleHeading({ level: 2 }).run(); }} className={cn("p-1.5 hover:text-gray-900 dark:hover:text-gray-200 hover:bg-gray-100 dark:hover:bg-white/5 rounded transition-colors text-xs text-gray-400 font-medium", editor.isActive('heading', { level: 2 }) && 'bg-gray-100 dark:bg-white/10 text-gray-900 dark:text-gray-100')} title="二级标题">H2</button>
        <button onMouseDown={(e) => { e.preventDefault(); editor.chain().focus().toggleHeading({ level: 3 }).run(); }} className={cn("p-1.5 hover:text-gray-900 dark:hover:text-gray-200 hover:bg-gray-100 dark:hover:bg-white/5 rounded transition-colors text-xs text-gray-400 font-medium", editor.isActive('heading', { level: 3 }) && 'bg-gray-100 dark:bg-white/10 text-gray-900 dark:text-gray-100')} title="三级标题">H3</button>
        <div className="w-px h-4 bg-gray-200 dark:bg-white/10 mx-1"></div>
        <button onMouseDown={(e) => { e.preventDefault(); editor.chain().focus().toggleBulletList().run(); }} className="p-1.5 hover:text-gray-900 dark:hover:text-gray-200 hover:bg-gray-100 dark:hover:bg-white/5 rounded transition-colors text-gray-400" title="无序列表"><List size={14} /></button>
        <button onMouseDown={(e) => { e.preventDefault(); editor.chain().focus().toggleCodeBlock().run(); }} className="p-1.5 hover:text-gray-900 dark:hover:text-gray-200 hover:bg-gray-100 dark:hover:bg-white/5 rounded transition-colors text-gray-400" title="代码块"><Code size={14} /></button>
        <div className="w-px h-4 bg-gray-200 dark:bg-white/10 mx-1"></div>
        <button onMouseDown={(e) => {
          e.preventDefault();
          const { from } = editor.state.selection;
          const coords = editor.view.coordsAtPos(from);
          setContextMenu({ x: coords.left + 20, y: coords.top + 20 });
        }} className="p-1.5 hover:text-indigo-300 hover:bg-indigo-500/20 text-indigo-400 rounded transition-colors" title="AI 指令">
          <Sparkles size={14} />
        </button>
      </FloatingMenu>

      <BubbleMenu 
        className="flex items-center gap-1 bg-white dark:bg-[#2a2a2a] border border-gray-200 dark:border-white/10 shadow-2xl rounded-lg p-1.5 backdrop-blur-md z-50 transition-all" 
        editor={editor}
      >
        {aiLoading ? (
          <span className="text-xs font-medium text-indigo-600 dark:text-indigo-400 px-3 py-1 flex items-center gap-2">
            <Loader2 size={14} className="animate-spin" /> 处理中...
          </span>
        ) : (
          <>
            {/* Formatting */}
            <button onMouseDown={(e) => { e.preventDefault(); editor.chain().focus().toggleBold().run(); }} className={cn("p-1.5 hover:text-gray-900 dark:hover:text-gray-200 hover:bg-gray-100 dark:hover:bg-white/5 rounded transition-colors", editor.isActive('bold') && 'bg-gray-100 dark:bg-white/10 text-gray-900 dark:text-gray-100')} title="粗体"><Bold size={14} /></button>
            <button onMouseDown={(e) => { e.preventDefault(); editor.chain().focus().toggleItalic().run(); }} className={cn("p-1.5 hover:text-gray-900 dark:hover:text-gray-200 hover:bg-gray-100 dark:hover:bg-white/5 rounded transition-colors", editor.isActive('italic') && 'bg-gray-100 dark:bg-white/10 text-gray-900 dark:text-gray-100')} title="斜体"><Italic size={14} /></button>
            <button onMouseDown={(e) => { e.preventDefault(); editor.chain().focus().toggleCode().run(); }} className={cn("p-1.5 hover:text-gray-900 dark:hover:text-gray-200 hover:bg-gray-100 dark:hover:bg-white/5 rounded transition-colors", editor.isActive('code') && 'bg-gray-100 dark:bg-white/10 text-gray-900 dark:text-gray-100')} title="行内代码"><Code size={14} /></button>
            
            <div className="w-px h-4 bg-gray-200 dark:bg-white/10 mx-1"></div>
            
            {/* AI Quick Actions */}
            <div className="flex items-center relative">
              <button 
                onMouseDown={(e) => { e.preventDefault(); setBubbleAiOpen(!bubbleAiOpen); }}
                className={cn(
                  "px-2.5 py-1.5 text-xs hover:bg-indigo-50 dark:hover:bg-indigo-500/20 rounded transition-colors flex items-center gap-1 font-medium",
                  bubbleAiOpen ? "bg-indigo-50 dark:bg-indigo-500/20 text-indigo-600 dark:text-indigo-200" : "text-indigo-600 dark:text-indigo-300"
                )}
              >
                <Sparkles size={14}/> 快捷指令
              </button>
              {bubbleAiOpen && (
                <div className="absolute top-full left-0 pt-1 w-32">
                  <div className="flex flex-col bg-white dark:bg-[#2a2a2a] border border-gray-200 dark:border-white/10 shadow-xl rounded-lg p-1">
                    <button onMouseDown={(e) => { e.preventDefault(); executeAiAction('rewrite'); setBubbleAiOpen(false); }} className="px-3 py-1.5 text-left text-xs hover:bg-indigo-50 dark:hover:bg-indigo-500/20 text-indigo-600 dark:text-indigo-300 rounded transition-colors">润色</button>
                    <button onMouseDown={(e) => { e.preventDefault(); executeAiAction('expand'); setBubbleAiOpen(false); }} className="px-3 py-1.5 text-left text-xs hover:bg-indigo-50 dark:hover:bg-indigo-500/20 text-indigo-600 dark:text-indigo-300 rounded transition-colors">扩写</button>
                    <button onMouseDown={(e) => { e.preventDefault(); executeAiAction('summarize'); setBubbleAiOpen(false); }} className="px-3 py-1.5 text-left text-xs hover:bg-indigo-50 dark:hover:bg-indigo-500/20 text-indigo-600 dark:text-indigo-300 rounded transition-colors">总结提炼</button>
                    <button onMouseDown={(e) => { e.preventDefault(); executeAiAction('translate'); setBubbleAiOpen(false); }} className="px-3 py-1.5 text-left text-xs hover:bg-indigo-50 dark:hover:bg-indigo-500/20 text-indigo-600 dark:text-indigo-300 rounded transition-colors">中英互译</button>
                  </div>
                </div>
              )}
            </div>
          </>
        )}
      </BubbleMenu>
    </>
  );
};
