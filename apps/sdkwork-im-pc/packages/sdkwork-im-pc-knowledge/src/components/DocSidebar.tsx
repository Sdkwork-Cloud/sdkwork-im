import React, { useState, useEffect, useMemo, useRef } from 'react';
import { ChevronRight, FileText, Hash, Plus, Trash2, MoreVertical, Image as ImageIcon, Music, Video, Folder, FolderOpen, ChevronDown, FolderPlus, Search, X, Upload } from 'lucide-react';
import { cn } from '@sdkwork/im-pc-commons';
import { KnowledgeBase, KnowledgeDoc } from '../services/KnowledgeService';

interface DocSidebarProps {
  selectedBase: KnowledgeBase;
  docs: KnowledgeDoc[];
  isLoading?: boolean;
  selectedDoc: KnowledgeDoc | null;
  setSelectedDoc: (doc: KnowledgeDoc | null) => void;
  onUploadFile: (e: React.ChangeEvent<HTMLInputElement>, parentId?: string) => void;
  onCreateDocClick: () => void;
  onCreateFolderClick: (parentId: string | null) => void;
  onDeleteDoc: (doc: KnowledgeDoc) => void;
}

export const DocSidebar: React.FC<DocSidebarProps> = ({
  selectedBase,
  docs,
  isLoading,
  selectedDoc,
  setSelectedDoc,
  onUploadFile,
  onCreateDocClick,
  onCreateFolderClick,
  onDeleteDoc
}) => {
  const [currentFolderId, setCurrentFolderId] = useState<string | null>(selectedDoc?.parentId || null);
  const [searchQuery, setSearchQuery] = useState('');
  const [isSearchVisible, setIsSearchVisible] = useState(false);
  const searchInputRef = useRef<HTMLInputElement>(null);

  useEffect(() => {
    if (selectedDoc && selectedDoc.parentId !== undefined) {
      setCurrentFolderId(selectedDoc.parentId || null);
    }
  }, [selectedDoc]);

  useEffect(() => {
    if (isSearchVisible && searchInputRef.current) {
      searchInputRef.current.focus();
    }
  }, [isSearchVisible]);

  const displayedDocs = useMemo(() => {
    if (searchQuery.trim()) {
      return docs.filter(d => d.title.toLowerCase().includes(searchQuery.toLowerCase()));
    }
    // Sort logic: Folders first, then files
    const currentDocs = docs.filter(d => (d.parentId || null) === currentFolderId);
    return currentDocs.sort((a, b) => {
      if (a.type === 'folder' && b.type !== 'folder') return -1;
      if (a.type !== 'folder' && b.type === 'folder') return 1;
      return a.title.localeCompare(b.title);
    });
  }, [docs, currentFolderId, searchQuery]);

  const folderPath = useMemo(() => {
    const path: KnowledgeDoc[] = [];
    let current = currentFolderId;
    while (current) {
      const folder = docs.find(d => d.id === current);
      if (folder) {
        path.unshift(folder);
        current = folder.parentId || null;
      } else {
        break;
      }
    }
    return path;
  }, [docs, currentFolderId]);

  const getFileIcon = (mime?: string) => {
    if (!mime) return <FileText size={15} />;
    if (mime.startsWith('image/')) return <ImageIcon size={15} />;
    if (mime.startsWith('video/')) return <Video size={15} />;
    if (mime.startsWith('audio/')) return <Music size={15} />;
    return <FileText size={15} />;
  };

  const handleDocClick = (doc: KnowledgeDoc) => {
    if (doc.type === 'folder') {
      setCurrentFolderId(doc.id);
      setSearchQuery('');
      setIsSearchVisible(false);
    } else {
      setSelectedDoc(doc);
    }
  };

  const handleSearchCancel = () => {
    setIsSearchVisible(false);
    setSearchQuery('');
  };

  return (
    <div className="w-[280px] shrink-0 border-r border-gray-200 dark:border-white/5 bg-[#fafafa] dark:bg-[#1a1a1a] flex flex-col hidden lg:flex">
      {/* Header */}
      <div 
        className="h-14 px-4 border-b border-gray-200 dark:border-white/5 flex items-center shrink-0 bg-white dark:bg-[#1f1f21] group cursor-pointer hover:bg-gray-50 dark:hover:bg-white/[0.02] transition-colors"
        onClick={() => setSelectedDoc(null)}
      >
        <div className="w-8 h-8 rounded-lg bg-gray-100 dark:bg-white/10 flex items-center justify-center text-gray-500 dark:text-gray-400 mr-3 group-hover:bg-indigo-50 dark:group-hover:bg-indigo-500/20 group-hover:text-indigo-600 dark:group-hover:text-indigo-400 transition-colors">
          <ChevronRight size={18} className="rotate-180" />
        </div>
        <div className="flex-1 min-w-0">
          <h2 className="text-[15px] font-bold text-gray-900 dark:text-gray-100 truncate group-hover:text-indigo-600 dark:group-hover:text-indigo-400 transition-colors">{selectedBase.name}</h2>
        </div>
      </div>
      
      {/* Breadcrumb Area */}
      {!searchQuery && isSearchVisible === false && (
        <div className="px-5 py-2.5 flex flex-wrap items-center gap-1 bg-white dark:bg-[#1f1f21] border-b border-gray-100 dark:border-white/[0.02]">
          <button 
            onClick={() => setCurrentFolderId(null)} 
            className={cn("text-[13px] font-medium transition-colors truncate max-w-[100px]", currentFolderId === null ? "text-gray-900 dark:text-gray-100 font-bold" : "text-gray-500 dark:text-gray-400 hover:text-indigo-600 dark:hover:text-indigo-400")}
            title="根目录"
          >
            {selectedBase.name}
          </button>
          {folderPath.map((folder, index) => (
            <React.Fragment key={folder.id}>
              <ChevronRight size={14} className="text-gray-400 dark:text-gray-600 shrink-0" />
              <button 
                onClick={() => setCurrentFolderId(folder.id)} 
                className={cn("text-[13px] font-medium transition-colors truncate max-w-[100px]", currentFolderId === folder.id ? "text-gray-900 dark:text-gray-100 font-bold" : "text-gray-500 dark:text-gray-400 hover:text-indigo-600 dark:hover:text-indigo-400")}
                title={folder.title}
              >
                {folder.title}
              </button>
            </React.Fragment>
          ))}
        </div>
      )}

      {/* Actions / Search Bar */}
      <div className="px-4 py-3 flex items-center gap-1.5 h-[56px] bg-[#fafafa] dark:bg-[#1a1a1a]">
        {isSearchVisible ? (
          <div className="relative flex items-center w-full bg-white dark:bg-[#28282b] border border-indigo-200 dark:border-indigo-500/30 rounded-xl shadow-sm overflow-hidden animate-in fade-in slide-in-from-right-4 duration-300">
            <div className="pl-3 py-2 flex items-center justify-center text-indigo-500">
              <Search size={16} />
            </div>
            <input 
              ref={searchInputRef}
              type="text" 
              placeholder="搜索文档和目录..." 
              value={searchQuery}
              onChange={(e) => setSearchQuery(e.target.value)}
              className="w-full bg-transparent border-none pl-2 pr-10 py-2 text-[13px] text-gray-900 dark:text-gray-100 focus:outline-none focus:ring-0 placeholder:text-gray-400 h-full"
            />
            {searchQuery && (
              <button 
                onClick={() => setSearchQuery('')}
                className="absolute right-8 text-gray-400 hover:text-gray-600 dark:hover:text-gray-200 transition-colors"
              >
                <X size={14} />
              </button>
            )}
            <button 
              onClick={handleSearchCancel}
              className="absolute right-0 top-0 bottom-0 px-3 text-[13px] font-medium text-gray-500 hover:text-gray-800 dark:text-gray-400 dark:hover:text-gray-200 hover:bg-gray-50 dark:hover:bg-white/5 transition-colors border-l border-gray-100 dark:border-white/5"
            >
              取消
            </button>
          </div>
        ) : (
          <div className="flex items-center w-full gap-2 animate-in fade-in zoom-in-95 duration-200">
           <button 
             className="flex-1 flex items-center justify-center gap-1.5 py-2 bg-white dark:bg-[#28282b] border border-gray-200 dark:border-white/10 hover:border-indigo-200 dark:hover:border-indigo-500/30 hover:bg-indigo-50/50 dark:hover:bg-indigo-500/10 text-gray-700 dark:text-gray-300 hover:text-indigo-600 dark:hover:text-indigo-400 rounded-xl text-[13px] font-bold transition-all shadow-sm active:scale-95"
             onClick={onCreateDocClick}
           >
             <FileText size={15} /> 新建
           </button>
           <button 
             className="w-9 h-9 flex items-center justify-center bg-white dark:bg-[#28282b] border border-gray-200 dark:border-white/10 hover:border-indigo-200 dark:hover:border-indigo-500/30 hover:bg-indigo-50/50 dark:hover:bg-indigo-500/10 text-gray-600 dark:text-gray-400 hover:text-indigo-600 dark:hover:text-indigo-400 rounded-xl transition-all shadow-sm relative overflow-hidden shrink-0"
             title="上传文件"
           >
             <input type="file" className="absolute inset-0 opacity-0 cursor-pointer z-10" onChange={(e) => onUploadFile(e, currentFolderId || undefined)} />
             <Upload size={16} /> 
           </button>
           <button 
             className="w-9 h-9 flex items-center justify-center bg-white dark:bg-[#28282b] border border-gray-200 dark:border-white/10 hover:border-indigo-200 dark:hover:border-indigo-500/30 hover:bg-indigo-50/50 dark:hover:bg-indigo-500/10 text-gray-600 dark:text-gray-400 hover:text-indigo-600 dark:hover:text-indigo-400 rounded-xl transition-all shadow-sm shrink-0"
             onClick={() => onCreateFolderClick(currentFolderId)}
             title="新建文件夹"
           >
             <FolderPlus size={16} />
           </button>
           <button 
             className="w-9 h-9 flex items-center justify-center bg-white dark:bg-[#28282b] border border-gray-200 dark:border-white/10 hover:border-indigo-200 dark:hover:border-indigo-500/30 hover:bg-indigo-50/50 dark:hover:bg-indigo-500/10 text-gray-600 dark:text-gray-400 hover:text-indigo-600 dark:hover:text-indigo-400 rounded-xl transition-all shadow-sm shrink-0"
             onClick={() => setIsSearchVisible(true)}
             title="搜索"
           >
             <Search size={16} />
           </button>
          </div>
        )}
      </div>

      <div className="flex-1 p-2 overflow-y-auto custom-scrollbar flex flex-col gap-1 mt-1 pb-6">
        {isLoading ? (
           <div className="flex flex-col gap-2 p-1">
             {[...Array(6)].map((_, i) => (
                <div key={`sk-ds-${i}`} className="flex items-center gap-3 px-3 py-2.5">
                   <div className="w-4 h-4 bg-gray-200 dark:bg-white/10 rounded animate-pulse shrink-0" />
                   <div className="h-3.5 bg-gray-200 dark:bg-white/10 rounded animate-pulse w-full max-w-[140px]" />
                </div>
             ))}
           </div>
        ) : (
          <div className="flex flex-col gap-0.5">
            {displayedDocs.map(doc => (
              <div 
                key={doc.id}
                onClick={() => handleDocClick(doc)}
                className={cn(
                  "px-3 py-2 rounded-xl cursor-pointer text-[13px] transition-all duration-200 flex items-center gap-2.5 group relative",
                  selectedDoc?.id === doc.id 
                    ? "bg-indigo-50 dark:bg-indigo-500/10 text-indigo-700 dark:text-indigo-400 font-bold border border-indigo-100/50 dark:border-indigo-500/20 shadow-sm" 
                    : "text-gray-700 dark:text-gray-300 hover:bg-gray-200/50 dark:hover:bg-white/[0.04] border border-transparent hover:border-gray-200 dark:hover:border-white/5 font-medium"
                )}
              >
                <div className={cn(
                  "shrink-0 flex items-center justify-center transition-colors",
                  selectedDoc?.id === doc.id ? "text-indigo-500 dark:text-indigo-400" : (doc.type === 'folder' ? 'text-blue-500 dark:text-blue-400' : 'text-gray-400 dark:text-gray-500 group-hover:text-gray-600 dark:group-hover:text-gray-300')
                )}>
                  {doc.type === 'folder' ? <Folder size={16} className="fill-current opacity-20" /> : doc.type === 'file' ? getFileIcon(doc.fileMimeType) : <Hash size={16} />}
                </div>
                <div className="flex-1 min-w-0 truncate tracking-tight">{doc.title}</div>

                <div className="absolute right-2 opacity-0 group-hover:opacity-100 transition-opacity flex items-center">
                   <button 
                     className="p-1.5 text-gray-400 hover:text-red-500 hover:bg-red-50 dark:hover:text-red-400 dark:hover:bg-red-400/10 rounded-lg transition-colors"
                     onClick={(e) => { e.stopPropagation(); onDeleteDoc(doc); }}
                     title="删除"
                   >
                     <Trash2 size={14} />
                   </button>
                </div>
              </div>
            ))}
            {displayedDocs.length === 0 && (
              <div className="flex flex-col items-center justify-center py-12 px-4 text-center">
                <div className="w-12 h-12 bg-gray-50 dark:bg-white/5 rounded-full flex items-center justify-center text-gray-400 dark:text-gray-500 mb-3">
                  {searchQuery ? <Search size={20} /> : <FolderOpen size={20} />}
                </div>
                <p className="text-[13px] font-medium text-gray-500 dark:text-gray-400 mb-1">
                  {searchQuery ? '未找到匹配结果' : '当前目录为空'}
                </p>
                {searchQuery && (
                   <button 
                     onClick={() => setSearchQuery('')}
                     className="text-xs text-indigo-500 hover:underline"
                   >
                     清除搜索
                   </button>
                )}
              </div>
            )}
          </div>
        )}
      </div>
    </div>
  );
};

