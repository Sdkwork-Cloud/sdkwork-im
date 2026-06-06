import React, { useState, useRef, useEffect } from 'react';
import { BookOpen, Plus, Search, Trash2, FolderPlus, FileText } from 'lucide-react';
import { cn } from '@sdkwork/clawchat-pc-commons';
import { KnowledgeBase } from '../services/KnowledgeService';

interface MainSidebarProps {
  bases: KnowledgeBase[];
  selectedBase: KnowledgeBase | null;
  isLoading?: boolean;
  searchQuery: string;
  setSearchQuery: (query: string) => void;
  onSelectBase: (base: KnowledgeBase) => void;
  onCreateKBClick: (type?: 'team' | 'personal') => void;
  onCreateNoteClick: () => void;
  onDeleteKB: (base: KnowledgeBase) => void;
  getBaseIcon: (base: KnowledgeBase) => React.ReactNode;
}

export const MainSidebar: React.FC<MainSidebarProps> = ({
  bases,
  selectedBase,
  isLoading,
  searchQuery,
  setSearchQuery,
  onSelectBase,
  onCreateKBClick,
  onCreateNoteClick,
  onDeleteKB,
  getBaseIcon
}) => {
  const [isDropdownOpen, setIsDropdownOpen] = useState(false);
  const dropdownRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    const handleClickOutside = (event: MouseEvent) => {
       if (dropdownRef.current && !dropdownRef.current.contains(event.target as Node)) {
          setIsDropdownOpen(false);
       }
    };
    document.addEventListener("mousedown", handleClickOutside);
    return () => document.removeEventListener("mousedown", handleClickOutside);
  }, []);

  const teamBases = bases.filter(b => b.type === 'team' && b.name.toLowerCase().includes(searchQuery.toLowerCase()));
  const personalBases = bases.filter(b => b.type === 'personal' && b.name.toLowerCase().includes(searchQuery.toLowerCase()));

  return (
    <div className="w-[280px] shrink-0 border-r border-gray-200 dark:border-white/5 bg-gray-50 dark:bg-[#181818] flex flex-col pt-4">
       <div className="px-5 mb-4">
          <div className="flex items-center justify-between mb-4">
             <h1 className="text-lg font-medium tracking-wide flex items-center gap-2 text-gray-900 dark:text-gray-100">
                <BookOpen size={20} className="text-indigo-600 dark:text-indigo-400" />
                <span>知识中心</span>
             </h1>
             <div className="relative" ref={dropdownRef}>
               <button 
                  onClick={() => setIsDropdownOpen(!isDropdownOpen)}
                  className={cn("w-7 h-7 rounded bg-indigo-50 dark:bg-indigo-500/10 text-indigo-600 dark:text-indigo-400 flex items-center justify-center hover:bg-indigo-100 dark:hover:bg-indigo-500/20 transition-colors shadow-sm", isDropdownOpen && "bg-indigo-100 dark:bg-indigo-500/20")}
                  title="新建"
               >
                   <Plus size={16} />
               </button>
               
               {isDropdownOpen && (
                 <div className="absolute right-0 top-full mt-2 w-40 bg-white dark:bg-[#2a2a2a] border border-gray-200 dark:border-white/10 rounded-xl shadow-xl py-1 z-50">
                    <button 
                      className="w-full px-3 py-2 text-left text-sm text-gray-700 dark:text-gray-200 hover:bg-gray-50 dark:hover:bg-white/5 flex items-center gap-2"
                      onClick={() => {
                        setIsDropdownOpen(false);
                        onCreateKBClick();
                      }}
                    >
                      <FolderPlus size={14} className="text-indigo-600 dark:text-indigo-400" /> 新建知识库
                    </button>
                    <button 
                      className="w-full px-3 py-2 text-left text-sm text-gray-700 dark:text-gray-200 hover:bg-gray-50 dark:hover:bg-white/5 flex items-center gap-2"
                      onClick={() => {
                        setIsDropdownOpen(false);
                        onCreateNoteClick();
                      }}
                    >
                      <FileText size={14} className="text-indigo-600 dark:text-indigo-400" /> 新建笔记
                    </button>
                 </div>
               )}
             </div>
          </div>
          <div className="relative">
             <span className="absolute left-3 top-1/2 -translate-y-1/2 text-gray-400 dark:text-gray-500"><Search size={14} /></span>
             <input 
                type="text" 
                placeholder="搜索知识库..." 
                className="w-full bg-white dark:bg-[#2a2a2a] border border-gray-200 dark:border-white/5 rounded-lg py-1.5 pl-9 pr-3 text-sm text-gray-900 dark:text-gray-200 outline-none focus:border-indigo-500/50 focus:ring-1 focus:ring-indigo-500/50 shadow-sm transition-all placeholder:text-gray-400 dark:placeholder:text-gray-500"
                value={searchQuery}
                onChange={e => setSearchQuery(e.target.value)}
             />
          </div>
       </div>
       
       <div className="flex-1 overflow-y-auto custom-scrollbar px-3 flex flex-col gap-6 pt-2 pb-6">
         {isLoading ? (
           <div className="flex flex-col gap-4">
              <div className="flex flex-col gap-2">
                 <div className="h-4 w-20 bg-gray-200 dark:bg-white/5 rounded mx-3 mb-2 animate-pulse" />
                 {[1, 2, 3].map(i => (
                    <div key={`sk-t-${i}`} className="flex items-center gap-3 px-3 py-2">
                       <div className="w-8 h-8 rounded-lg bg-gray-200 dark:bg-white/5 animate-pulse shrink-0" />
                       <div className="h-4 bg-gray-200 dark:bg-white/5 rounded animate-pulse w-full max-w-[120px]" />
                    </div>
                 ))}
              </div>
           </div>
         ) : (
           <>
             {/* Team section */}
             {(teamBases.length > 0 || searchQuery === '') && (
               <div>
                 <div className="px-3 text-[11px] font-semibold text-gray-500 uppercase tracking-widest mb-3 mt-1 flex items-center justify-between group/header">
                   团队知识库
                   <button 
                     onClick={(e) => { e.stopPropagation(); onCreateKBClick('team'); }}
                     className="opacity-0 group-hover/header:opacity-100 hover:text-indigo-600 dark:hover:text-indigo-400 hover:bg-indigo-50 dark:hover:bg-indigo-500/10 p-0.5 rounded transition-all"
                     title="新建团队知识库"
                   >
                     <Plus size={14} />
                   </button>
                 </div>
             <div className="flex flex-col gap-1">
               {teamBases.map(b => (
                  <div 
                    key={b.id} 
                    onClick={() => onSelectBase(b)}
                    className={cn(
                      "flex items-center gap-3 px-3 py-2 rounded-xl cursor-pointer transition-all duration-200 group relative",
                      selectedBase?.id === b.id 
                        ? "bg-indigo-50 dark:bg-indigo-500/10 border-indigo-100 dark:border-indigo-500/20 border shadow-sm dark:shadow-[inset_0_1px_0_0_rgba(255,255,255,0.05)]" 
                        : "hover:bg-gray-100 dark:hover:bg-white/5 border border-transparent"
                    )}
                  >
                    <div className={cn(
                      "w-8 h-8 flex items-center justify-center rounded-lg shadow-sm text-sm transition-colors",
                      selectedBase?.id === b.id ? "bg-indigo-100 dark:bg-indigo-500/20 text-indigo-600 dark:text-indigo-400" : "bg-white dark:bg-[#242424] text-gray-600 dark:text-gray-300 border border-gray-200 dark:border-white/5"
                    )}>
                       {getBaseIcon(b)}
                    </div>
                    <div className="flex-1 min-w-0">
                       <div className={cn("text-sm truncate", selectedBase?.id === b.id ? "text-indigo-700 dark:text-indigo-100 font-medium" : "text-gray-600 dark:text-gray-400 group-hover:text-gray-900 dark:group-hover:text-gray-200")}>{b.name}</div>
                    </div>
                    
                    <div className="absolute right-2 opacity-0 group-hover:opacity-100 transition-opacity">
                       <button 
                         className="p-1.5 text-gray-400 hover:text-red-500 dark:hover:text-red-400 hover:bg-red-50 dark:hover:bg-red-400/10 rounded-lg transition-colors"
                         onClick={(e) => { e.stopPropagation(); onDeleteKB(b); }}
                         title="删除"
                       >
                         <Trash2 size={14} />
                       </button>
                    </div>
                  </div>
               ))}
               {teamBases.length === 0 && searchQuery === '' && (
                 <div className="text-gray-400 dark:text-gray-500 text-[11px] px-3 py-2 text-center border-dashed border border-gray-200 dark:border-white/5 rounded-xl mx-2 mb-2">暂无团队知识库</div>
               )}
             </div>
           </div>
         )}

         {/* Personal section */}
         {(personalBases.length > 0 || searchQuery === '') && (
           <div className="mt-4">
             <div className="px-3 text-[11px] font-semibold text-gray-500 uppercase tracking-widest mb-3 flex items-center justify-between group/header">
               个人知识空间
               <button 
                 onClick={(e) => { e.stopPropagation(); onCreateKBClick('personal'); }}
                 className="opacity-0 group-hover/header:opacity-100 hover:text-indigo-600 dark:hover:text-indigo-400 hover:bg-indigo-50 dark:hover:bg-indigo-500/10 p-0.5 rounded transition-all"
                 title="新建个人空间"
               >
                 <Plus size={14} />
               </button>
             </div>
             <div className="flex flex-col gap-1">
               {personalBases.map(b => (
                  <div 
                    key={b.id} 
                    onClick={() => onSelectBase(b)}
                    className={cn(
                      "flex items-center gap-3 px-3 py-2 rounded-xl cursor-pointer transition-all duration-200 group relative",
                      selectedBase?.id === b.id 
                        ? "bg-indigo-50 dark:bg-indigo-500/10 border-indigo-100 dark:border-indigo-500/20 border shadow-sm dark:shadow-[inset_0_1px_0_0_rgba(255,255,255,0.05)]" 
                        : "hover:bg-gray-100 dark:hover:bg-white/5 border border-transparent"
                    )}
                  >
                    <div className={cn(
                      "w-8 h-8 flex items-center justify-center rounded-lg shadow-sm text-sm transition-colors",
                      selectedBase?.id === b.id ? "bg-indigo-100 dark:bg-indigo-500/20 text-indigo-600 dark:text-indigo-400" : "bg-white dark:bg-[#242424] text-gray-600 dark:text-gray-300 border border-gray-200 dark:border-white/5"
                    )}>
                       {getBaseIcon(b)}
                    </div>
                    <div className="flex-1 min-w-0">
                       <div className={cn("text-sm truncate", selectedBase?.id === b.id ? "text-indigo-700 dark:text-indigo-100 font-medium" : "text-gray-600 dark:text-gray-400 group-hover:text-gray-900 dark:group-hover:text-gray-200")}>{b.name}</div>
                    </div>

                    <div className="absolute right-2 opacity-0 group-hover:opacity-100 transition-opacity">
                       <button 
                         className="p-1.5 text-gray-400 hover:text-red-500 dark:hover:text-red-400 hover:bg-red-50 dark:hover:bg-red-400/10 rounded-lg transition-colors"
                         onClick={(e) => { e.stopPropagation(); onDeleteKB(b); }}
                         title="删除"
                       >
                         <Trash2 size={14} />
                       </button>
                    </div>
                  </div>
               ))}
               {personalBases.length === 0 && searchQuery === '' && (
                 <div className="text-gray-400 dark:text-gray-500 text-[11px] px-3 py-2 text-center border-dashed border border-gray-200 dark:border-white/5 rounded-xl mx-2 mb-2">暂无个人知识空间</div>
               )}
             </div>
           </div>
         )}

         {teamBases.length === 0 && personalBases.length === 0 && searchQuery !== '' && (
           <div className="text-center py-10 px-4">
              <div className="w-12 h-12 rounded-full bg-gray-100 dark:bg-white/5 flex items-center justify-center mx-auto mb-3 shadow-sm">
                <Search size={20} className="text-gray-400 dark:text-gray-500" />
              </div>
              <p className="text-gray-500 text-sm">找不到相关的知识库</p>
           </div>
         )}
         </>
       )}
       </div>
    </div>
  );
};
