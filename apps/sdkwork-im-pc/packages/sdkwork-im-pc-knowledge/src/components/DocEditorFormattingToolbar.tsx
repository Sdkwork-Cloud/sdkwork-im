import React from 'react';
import { Editor } from '@tiptap/react';
import { Bold, Italic, List, ListOrdered, Quote, Code, Type, Link as LinkIcon, Image as ImageIcon } from 'lucide-react';
import { cn } from '@sdkwork/im-pc-commons';

interface DocEditorFormattingToolbarProps {
  editor: Editor | null;
}

export const DocEditorFormattingToolbar: React.FC<DocEditorFormattingToolbarProps> = ({ editor }) => {
  if (!editor) return null;

  return (
    <div className="px-4 md:px-8 py-2 border-y border-gray-200 dark:border-white/5 bg-gray-50 dark:bg-[#181818] flex items-center gap-1 text-gray-500 dark:text-gray-400 shrink-0 overflow-x-auto custom-scrollbar">
       <button onClick={() => editor.chain().focus().toggleBold().run()} className={cn("p-1.5 hover:text-gray-200 hover:bg-white/5 rounded transition-colors", editor.isActive('bold') && 'bg-white/10 text-gray-100')} title="粗体 (Ctrl+B)"><Bold size={16} /></button>
       <button onClick={() => editor.chain().focus().toggleItalic().run()} className={cn("p-1.5 hover:text-gray-200 hover:bg-white/5 rounded transition-colors", editor.isActive('italic') && 'bg-white/10 text-gray-100')} title="斜体 (Ctrl+I)"><Italic size={16} /></button>
       <div className="w-px h-4 bg-white/10 mx-1"></div>
       <button onClick={() => editor.chain().focus().toggleBulletList().run()} className={cn("p-1.5 hover:text-gray-200 hover:bg-white/5 rounded transition-colors", editor.isActive('bulletList') && 'bg-white/10 text-gray-100')} title="无序列表"><List size={16} /></button>
       <button onClick={() => editor.chain().focus().toggleOrderedList().run()} className={cn("p-1.5 hover:text-gray-200 hover:bg-white/5 rounded transition-colors", editor.isActive('orderedList') && 'bg-white/10 text-gray-100')} title="有序列表"><ListOrdered size={16} /></button>
       <div className="w-px h-4 bg-white/10 mx-1"></div>
       <button onClick={() => editor.chain().focus().toggleBlockquote().run()} className={cn("p-1.5 hover:text-gray-200 hover:bg-white/5 rounded transition-colors", editor.isActive('blockquote') && 'bg-white/10 text-gray-100')} title="引用"><Quote size={16} /></button>
       <button onClick={() => editor.chain().focus().toggleCode().run()} className={cn("p-1.5 hover:text-gray-200 hover:bg-white/5 rounded transition-colors", editor.isActive('code') && 'bg-white/10 text-gray-100')} title="行内代码"><Code size={16} /></button>
       <button onClick={() => editor.chain().focus().toggleCodeBlock().run()} className={cn("p-1.5 hover:text-gray-200 hover:bg-white/5 rounded transition-colors", editor.isActive('codeBlock') && 'bg-white/10 text-gray-100')} title="代码块"><Type size={16} /></button>
       <div className="w-px h-4 bg-white/10 mx-1"></div>
       <button onClick={() => { const url = window.prompt('URL'); if(url) editor.chain().focus().setLink({ href: url }).run(); }} className={cn("p-1.5 hover:text-gray-200 hover:bg-white/5 rounded transition-colors", editor.isActive('link') && 'bg-white/10 text-gray-100')} title="插入链接"><LinkIcon size={16} /></button>
       <button onClick={() => { const url = window.prompt('Image URL'); if(url) editor.chain().focus().setImage({ src: url }).run(); }} className="p-1.5 hover:text-gray-200 hover:bg-white/5 rounded transition-colors" title="插入图片"><ImageIcon size={16} /></button>
    </div>
  );
};
