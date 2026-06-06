import React from 'react';
import { FileText, ArrowLeft, Edit2, Columns, Eye, Sparkles, Save } from 'lucide-react';
import { cn } from '@sdkwork/clawchat-pc-commons';
import { KnowledgeBase, KnowledgeDoc } from '../services/KnowledgeService';

interface DocEditorHeaderProps {
  selectedBase: KnowledgeBase;
  selectedDoc: KnowledgeDoc | null;
  activeTab: 'edit' | 'preview' | 'split';
  setActiveTab: (tab: 'edit' | 'preview' | 'split') => void;
  isAiPanelOpen: boolean;
  setIsAiPanelOpen: (isOpen: boolean) => void;
  editDocTitle: string;
  onSave: () => void;
  onCancel: () => void;
}

export const DocEditorHeader: React.FC<DocEditorHeaderProps> = ({
  selectedBase,
  selectedDoc,
  activeTab,
  setActiveTab,
  isAiPanelOpen,
  setIsAiPanelOpen,
  editDocTitle,
  onSave,
  onCancel
}) => {
  return (
    <div className="h-16 flex items-center justify-between px-6 border-b border-gray-200 dark:border-white/5 shrink-0 bg-white dark:bg-[#1e1e1e] relative z-10">
      <div className="flex items-center gap-3">
         <button 
           onClick={onCancel}
           className="p-2 text-gray-400 hover:text-gray-200 hover:bg-white/5 rounded-lg transition-colors mr-2 lg:hidden"
         >
           <ArrowLeft size={18} />
         </button>
         <div className="flex items-center gap-2 text-xs text-gray-500 font-medium">
           <span className="flex items-center gap-1.5"><FileText size={14}/> {selectedBase.name}</span>
           <span className="text-gray-600">/</span>
           <span className="text-gray-300">{selectedDoc ? '编辑文档' : '新建文档'}</span>
         </div>
      </div>
      
      <div className="hidden md:flex items-center justify-center absolute left-1/2 -translate-x-1/2">
         <div className="flex bg-gray-100 dark:bg-[#141414] p-1 rounded-lg border border-gray-200 dark:border-white/5">
            <button 
              className={cn(
                "flex items-center gap-1.5 px-4 py-1.5 text-xs font-medium rounded-md transition-colors",
                activeTab === 'edit' ? "bg-white dark:bg-[#2a2a2a] text-gray-900 dark:text-gray-200 shadow-sm" : "text-gray-500 hover:text-gray-900 dark:hover:text-gray-300 hover:bg-white/50 dark:hover:bg-white/5"
              )}
              onClick={() => setActiveTab('edit')}
            >
              <Edit2 size={14} /> 撰写
            </button>
            <button 
              className={cn(
                "flex items-center gap-1.5 px-4 py-1.5 text-xs font-medium rounded-md transition-colors",
                activeTab === 'split' ? "bg-white dark:bg-[#2a2a2a] text-gray-900 dark:text-gray-200 shadow-sm" : "text-gray-500 hover:text-gray-900 dark:hover:text-gray-300 hover:bg-white/50 dark:hover:bg-white/5"
              )}
              onClick={() => setActiveTab('split')}
            >
              <Columns size={14} /> 双栏
            </button>
            <button 
              className={cn(
                "flex items-center gap-1.5 px-4 py-1.5 text-xs font-medium rounded-md transition-colors",
                activeTab === 'preview' ? "bg-white dark:bg-[#2a2a2a] text-gray-900 dark:text-gray-200 shadow-sm" : "text-gray-500 hover:text-gray-900 dark:hover:text-gray-300 hover:bg-white/50 dark:hover:bg-white/5"
              )}
              onClick={() => setActiveTab('preview')}
            >
              <Eye size={14} /> 预览
            </button>
         </div>
      </div>

      <div className="flex items-center gap-3">
         <button 
           className={cn("hidden md:flex items-center gap-2 px-4 py-2 text-sm rounded-lg transition-colors", isAiPanelOpen ? "bg-indigo-500/20 text-indigo-400" : "text-gray-400 hover:text-gray-200 hover:bg-white/5")}
           onClick={() => setIsAiPanelOpen(!isAiPanelOpen)}
         >
            <Sparkles size={16} />
            AI 助手
         </button>
         <button 
           className="hidden md:block px-4 py-2 text-sm text-gray-400 hover:text-gray-200 hover:bg-white/5 rounded-lg transition-colors"
           onClick={onCancel}
         >
            取消
         </button>
         <button 
           className="px-5 py-2 bg-indigo-600 hover:bg-indigo-500 text-white rounded-lg transition-colors text-sm font-medium shadow-sm flex items-center gap-2"
           onClick={onSave}
           disabled={!editDocTitle.trim()}
         >
            <Save size={16} /> 保存
         </button>
      </div>
    </div>
  );
};
