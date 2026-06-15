import React, { useState, useMemo } from 'react';
import { BookOpen, Edit3, FileText, Folder, Globe, Lock, Plus, Trash2, Image as ImageIcon, Music, Video, MessageSquare, ChevronRight, LayoutGrid, List } from 'lucide-react';
import { KnowledgeBase, KnowledgeDoc } from '../services/KnowledgeService';
import { cn } from '@sdkwork/im-pc-commons';

interface BaseOverviewProps {
  selectedBase: KnowledgeBase;
  docs: KnowledgeDoc[];
  isLoading?: boolean;
  onUploadFile: (e: React.ChangeEvent<HTMLInputElement>, parentId?: string) => void;
  onCreateDocClick: () => void;
  onCreateFolderClick: (parentId: string | null) => void;
  onSelectDoc: (doc: KnowledgeDoc) => void;
  onDeleteDoc: (doc: KnowledgeDoc) => void;
  onChatClick?: () => void;
}

export const BaseOverview: React.FC<BaseOverviewProps> = ({
  selectedBase,
  docs,
  isLoading,
  onUploadFile,
  onCreateDocClick,
  onCreateFolderClick,
  onSelectDoc,
  onDeleteDoc,
  onChatClick
}) => {
  const [currentFolderId, setCurrentFolderId] = useState<string | null>(null);
  const [viewMode, setViewMode] = useState<'grid' | 'list'>('grid');

  const displayedDocs = useMemo(() => {
    return docs.filter(doc => (doc.parentId || null) === currentFolderId);
  }, [docs, currentFolderId]);

  const folders = useMemo(() => displayedDocs.filter(d => d.type === 'folder'), [displayedDocs]);
  const files = useMemo(() => displayedDocs.filter(d => d.type !== 'folder'), [displayedDocs]);

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
    if (!mime) return <FileText size={18} />;
    if (mime.startsWith('image/')) return <ImageIcon size={18} />;
    if (mime.startsWith('video/')) return <Video size={18} />;
    if (mime.startsWith('audio/')) return <Music size={18} />;
    return <FileText size={18} />;
  };

  return (
    <div className="flex-1 overflow-y-auto custom-scrollbar p-10 bg-gray-50 dark:bg-[#1e1e1e]">
      <div className="w-full max-w-[1400px] mx-auto space-y-10">
         <div className="flex flex-col gap-6">
           <div className="flex items-center gap-4">
             <div className="w-16 h-16 rounded-2xl bg-white dark:bg-white/5 shadow-sm dark:shadow-inner border border-gray-200 dark:border-white/10 flex items-center justify-center text-3xl overflow-hidden shrink-0">
               {selectedBase.logo.startsWith('http') || selectedBase.logo.startsWith('data:') ? (
                 <img src={selectedBase.logo} alt={selectedBase.name} className="w-full h-full object-cover" />
               ) : (
                 <span>{selectedBase.logo}</span>
               )}
             </div>
             <div>
               <h2 className="text-3xl font-semibold text-gray-900 dark:text-gray-100">{selectedBase.name}</h2>
               <p className="text-gray-500 dark:text-gray-400 mt-2 text-sm max-w-2xl">{selectedBase.description || '暂无描述'}</p>
             </div>
           </div>
           <div className="flex items-center gap-4 border-b border-gray-200 dark:border-white/5 pb-4">
             <div className="flex items-center gap-2 text-sm text-gray-500 dark:text-gray-400">
               {selectedBase.type === 'team' ? <Globe size={16} /> : <Lock size={16} />}
               {selectedBase.type === 'team' ? '团队知识库' : '个人知识库'}
             </div>
             <div className="flex items-center gap-2 text-sm text-gray-500 dark:text-gray-400">
               <Folder size={16} /> {docs.length} 篇文档
             </div>
             
             <div className="flex items-center bg-gray-100 dark:bg-[#1a1a1a] rounded-lg p-1 ml-4 border border-gray-200 dark:border-white/5">
               <button 
                 onClick={() => setViewMode('grid')}
                 className={cn("p-1.5 rounded-md transition-colors", viewMode === 'grid' ? "bg-white dark:bg-[#2a2a2a] text-indigo-600 dark:text-indigo-400 shadow-sm" : "text-gray-500 hover:text-gray-900 dark:hover:text-gray-300")}
                 title="网格视图"
               >
                 <LayoutGrid size={16} />
               </button>
               <button 
                 onClick={() => setViewMode('list')}
                 className={cn("p-1.5 rounded-md transition-colors", viewMode === 'list' ? "bg-white dark:bg-[#2a2a2a] text-indigo-600 dark:text-indigo-400 shadow-sm" : "text-gray-500 hover:text-gray-900 dark:hover:text-gray-300")}
                 title="列表视图"
               >
                 <List size={16} />
               </button>
             </div>

             <div className="flex items-center ml-auto gap-2">
               {onChatClick && (
                 <button 
                   onClick={onChatClick}
                   className="flex items-center gap-2 px-4 py-2 bg-indigo-50 dark:bg-indigo-500/10 hover:bg-indigo-100 dark:hover:bg-indigo-500/20 text-indigo-600 dark:text-indigo-400 rounded-lg text-sm transition-colors font-medium border border-indigo-100 dark:border-indigo-500/20"
                 >
                   <MessageSquare size={16} /> 对话查询
                 </button>
               )}
               <button className="flex items-center gap-2 px-4 py-1.5 bg-indigo-50 dark:bg-indigo-500/10 hover:bg-indigo-100 dark:hover:bg-indigo-500/20 text-indigo-600 dark:text-indigo-400 rounded-lg text-sm border border-indigo-200 dark:border-indigo-500/20 transition-colors shadow-sm ml-auto relative overflow-hidden group">
                 <input type="file" className="absolute inset-0 opacity-0 cursor-pointer z-10" onChange={(e) => onUploadFile(e, currentFolderId || undefined)} title="上传文件" />
                 <Plus size={16} className="group-hover:scale-110 transition-transform" /> 上传
               </button>
               <button 
                 onClick={() => onCreateFolderClick(currentFolderId)}
                 className="flex items-center gap-2 px-4 py-2 bg-indigo-50 dark:bg-[#2a2a2a] hover:bg-indigo-100 dark:hover:bg-[#333] text-indigo-600 dark:text-gray-300 rounded-lg text-sm transition-all font-medium border border-indigo-200 dark:border-white/5"
               >
                 <Folder size={16} /> 新建文件夹
               </button>
               <button 
                 onClick={onCreateDocClick}
                 className="flex items-center gap-2 px-4 py-2 bg-indigo-600 hover:bg-indigo-700 text-white rounded-lg text-sm border border-indigo-600 transition-all font-medium shadow-md shadow-indigo-500/20 px-6"
               >
                 <Edit3 size={16} /> 新建文档
               </button>
             </div>
           </div>
           
           {/* Breadcrumbs for folder navigation */}
           {(folderPath.length > 0) && (
             <div className="flex items-center gap-2 text-sm text-gray-600 dark:text-gray-400 font-medium">
               <button onClick={() => setCurrentFolderId(null)} className="hover:text-indigo-600 dark:hover:text-indigo-400 transition-colors">根目录</button>
               {folderPath.map(folder => (
                 <React.Fragment key={folder.id}>
                   <ChevronRight size={14} className="text-gray-400" />
                   <button onClick={() => setCurrentFolderId(folder.id)} className="hover:text-indigo-600 dark:hover:text-indigo-400 transition-colors">
                     {folder.title}
                   </button>
                 </React.Fragment>
               ))}
             </div>
           )}
         </div>

         {isLoading ? (
            <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 2xl:grid-cols-5 gap-6">
              {[1, 2, 3, 4, 5].map(i => (
                 <div key={`sk-doc-${i}`} className="p-5 rounded-2xl bg-white dark:bg-[#282828] border border-gray-200 dark:border-white/5 flex flex-col h-[200px]">
                   <div className="h-6 w-3/4 bg-gray-200 dark:bg-white/5 rounded animate-pulse mb-4" />
                   <div className="h-4 w-full bg-gray-200 dark:bg-white/5 rounded animate-pulse mb-2" />
                   <div className="h-4 w-5/6 bg-gray-200 dark:bg-white/5 rounded animate-pulse mb-2" />
                   <div className="flex justify-between mt-auto pt-4 border-t border-gray-100 dark:border-white/5">
                      <div className="h-3 w-12 bg-gray-200 dark:bg-white/5 rounded animate-pulse" />
                      <div className="h-3 w-20 bg-gray-200 dark:bg-white/5 rounded animate-pulse" />
                   </div>
                 </div>
              ))}
            </div>
         ) : displayedDocs.length === 0 ? (
            <div className="text-center py-32 px-4 border border-gray-200 dark:border-white/5 rounded-3xl border-dashed bg-gradient-to-b from-white to-gray-50 dark:from-white/[0.02] dark:to-transparent">
              <div className="w-20 h-20 rounded-full bg-indigo-50 dark:bg-white/5 flex items-center justify-center mx-auto mb-6 shadow-inner border border-indigo-100 dark:border-white/5">
                <FileText size={32} className="text-indigo-500 dark:text-indigo-400/80" />
              </div>
              <h3 className="text-xl font-semibold text-gray-800 dark:text-gray-200 mb-3 tracking-wide">空空如也的目录</h3>
              <p className="text-gray-500 text-[15px] mb-8 max-w-sm mx-auto leading-relaxed">在这里新建文档或者上传资料，开始沉淀你的经验吧。</p>
               <div className="flex justify-center gap-4">
                  <button className="flex items-center gap-2 px-6 py-2.5 bg-white dark:bg-[#2a2a2a] hover:bg-gray-50 dark:hover:bg-[#333] border border-gray-200 dark:border-white/5 text-gray-700 dark:text-gray-300 rounded-xl text-sm transition-colors relative overflow-hidden font-medium">
                   <input type="file" className="absolute inset-0 opacity-0 cursor-pointer z-10" onChange={(e) => onUploadFile(e, currentFolderId || undefined)} title="上传文件" />
                   <Plus size={16} /> 上传文件
                  </button>
                  <button 
                    onClick={() => onCreateFolderClick(currentFolderId)}
                    className="flex items-center gap-2 px-6 py-2.5 bg-white dark:bg-[#2a2a2a] hover:bg-gray-50 dark:hover:bg-[#333] border border-gray-200 dark:border-white/5 text-gray-700 dark:text-gray-300 rounded-xl text-sm transition-colors font-medium"
                  >
                   <Folder size={16} /> 新建文件夹
                  </button>
                  <button 
                    onClick={onCreateDocClick}
                    className="flex items-center gap-2 px-6 py-2.5 bg-indigo-600 hover:bg-indigo-700 text-white rounded-xl text-sm transition-all shadow-md shadow-indigo-500/20 font-medium"
                  >
                   <Edit3 size={16} /> 写新文档
                  </button>
               </div>
            </div>
         ) : (
            <div className="flex flex-col gap-8">
              {/* Folders Section */}
              {folders.length > 0 && viewMode === 'grid' && (
                <div>
                  <h3 className="text-sm font-medium text-gray-500 mb-4">文件夹</h3>
                  <div className="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-4 xl:grid-cols-5 2xl:grid-cols-6 gap-4">
                    {folders.map(doc => (
                      <div 
                        key={doc.id}
                        onClick={() => setCurrentFolderId(doc.id)}
                        className="p-4 rounded-xl bg-white dark:bg-[#282828] border border-gray-200 dark:border-white/5 hover:border-indigo-400 dark:hover:border-indigo-500/30 cursor-pointer transition-all hover:shadow-md group flex relative overflow-hidden items-center gap-3"
                      >
                        <Folder className="text-indigo-400 shrink-0" size={24} />
                        <span className="font-medium text-gray-800 dark:text-gray-200 truncate flex-1">{doc.title}</span>
                        <button 
                         className="p-1.5 text-gray-400 hover:text-red-500 hover:bg-red-50 dark:hover:bg-red-400/10 rounded-lg opacity-0 group-hover:opacity-100 transition-all shadow-sm bg-white/90 dark:bg-[#282828]/80 drop-shadow-md backdrop-blur-sm shrink-0"
                         onClick={(e) => { e.stopPropagation(); onDeleteDoc(doc); }}
                         title="删除"
                       >
                         <Trash2 size={16} />
                       </button>
                      </div>
                    ))}
                  </div>
                </div>
              )}

              {/* Files Section Grid View */}
              {viewMode === 'grid' && (
                <div>
                  {folders.length > 0 && <h3 className="text-sm font-medium text-gray-500 mb-4">文件</h3>}
                  <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 2xl:grid-cols-5 gap-6">
                      {files.map(doc => {
                         return (
                           <div 
                             key={doc.id}
                             onClick={() => onSelectDoc(doc)}
                             className="p-5 rounded-2xl bg-white dark:bg-[#282828] border border-gray-200 dark:border-white/5 hover:border-indigo-400 dark:hover:border-indigo-500/30 cursor-pointer transition-all hover:shadow-lg dark:hover:shadow-xl group flex flex-col h-[200px] relative overflow-hidden"
                           >
                             <h3 className="text-lg font-medium text-gray-800 dark:text-gray-200 group-hover:text-indigo-600 dark:group-hover:text-indigo-400 transition-colors mb-2 line-clamp-2 pr-6">
                               <span className="inline-flex mr-2 mt-0.5 align-top opacity-70">
                                 {doc.type === 'file' ? getFileIcon(doc.fileMimeType) : <BookOpen size={18} />}
                               </span>
                               {doc.title}
                             </h3>
                             <p className="text-sm text-gray-500 line-clamp-3 mb-4 flex-1 break-words">
                               {doc.type === 'file' ? `文件大小: ${doc.fileSize}` : doc.content.replace(/#.*\n/g, '')}
                             </p>
                             <div className="flex items-center justify-between text-xs text-gray-500 dark:text-gray-400 mt-auto pt-4 border-t border-gray-100 dark:border-white/5">
                                <span className="flex items-center gap-1">{doc.author}</span>
                                <span>{new Date(doc.updatedAt).toLocaleDateString()}</span>
                             </div>

                             <button 
                               className="absolute top-4 right-4 p-1.5 text-gray-400 hover:text-red-500 hover:bg-red-50 dark:hover:bg-red-400/10 rounded-lg opacity-0 group-hover:opacity-100 transition-all shadow-sm bg-white/90 dark:bg-[#282828]/80 drop-shadow-md backdrop-blur-sm"
                               onClick={(e) => { e.stopPropagation(); onDeleteDoc(doc); }}
                               title="删除"
                             >
                               <Trash2 size={16} />
                             </button>
                           </div>
                         );
                      })}
                      <label className="p-5 rounded-2xl border-2 border-dashed border-gray-300 dark:border-white/10 hover:border-indigo-400 dark:hover:border-indigo-500/50 hover:bg-indigo-50/50 dark:hover:bg-white/5 cursor-pointer transition-all hover:shadow-lg dark:hover:shadow-xl flex flex-col items-center justify-center h-[200px] text-gray-400 hover:text-indigo-600 dark:text-gray-500 dark:hover:text-indigo-400 group relative">
                         <input type="file" className="absolute inset-0 w-full h-full opacity-0 cursor-pointer z-10" onChange={(e) => onUploadFile(e, currentFolderId || undefined)} title="上传文件" />
                         <div className="w-12 h-12 rounded-full bg-gray-100 dark:bg-white/5 group-hover:bg-indigo-100 dark:group-hover:bg-indigo-500/10 flex items-center justify-center mb-3 transition-colors">
                           <Plus size={24} className="text-gray-400 group-hover:text-indigo-600 dark:text-gray-500 dark:group-hover:text-indigo-400" />
                         </div>
                         <span className="font-medium text-[15px] group-hover:text-indigo-600 dark:group-hover:text-indigo-400">点击或拖拽上传</span>
                       </label>
                  </div>
                </div>
              )}

              {/* List View */}
              {viewMode === 'list' && (
                <div className="bg-white dark:bg-[#282828] border border-gray-200 dark:border-white/5 rounded-2xl overflow-hidden shadow-sm">
                  <div className="grid grid-cols-12 gap-4 px-6 py-4 border-b border-gray-100 dark:border-white/5 bg-gray-50/50 dark:bg-white/[0.02]">
                    <div className="col-span-6 md:col-span-5 font-medium text-xs text-gray-500 uppercase tracking-wider">名称</div>
                    <div className="col-span-3 hidden md:block font-medium text-xs text-gray-500 uppercase tracking-wider">所有者</div>
                    <div className="col-span-3 md:col-span-2 font-medium text-xs text-gray-500 uppercase tracking-wider">最后修改</div>
                    <div className="col-span-2 md:col-span-1 font-medium text-xs text-gray-500 uppercase tracking-wider">大小</div>
                    <div className="col-span-1 font-medium text-xs text-gray-500 text-right uppercase tracking-wider"></div>
                  </div>
                  <div className="divide-y divide-gray-100 dark:divide-white/5">
                    {[...folders, ...files].map(doc => {
                       return (
                         <div 
                           key={doc.id}
                           onClick={() => {
                             if (doc.type === 'folder') setCurrentFolderId(doc.id);
                             else onSelectDoc(doc);
                           }}
                           className="grid grid-cols-12 gap-4 px-6 py-3 items-center hover:bg-gray-50 dark:hover:bg-white/5 cursor-pointer transition-colors group"
                         >
                           <div className="col-span-6 md:col-span-5 flex items-center gap-3 pr-4">
                             <div className="w-8 h-8 rounded-lg bg-gray-100 dark:bg-white/5 flex items-center justify-center shrink-0">
                               {doc.type === 'folder' ? <Folder size={16} className="text-indigo-400" /> : doc.type === 'file' ? getFileIcon(doc.fileMimeType) : <BookOpen size={16} className="text-indigo-400" />}
                             </div>
                             <span className="font-medium text-gray-900 dark:text-gray-200 truncate">{doc.title}</span>
                           </div>
                           <div className="col-span-3 hidden md:block text-sm text-gray-500 truncate">{doc.author}</div>
                           <div className="col-span-3 md:col-span-2 text-sm text-gray-500">{new Date(doc.updatedAt).toLocaleDateString()}</div>
                           <div className="col-span-2 md:col-span-1 text-sm text-gray-500 truncate">
                             {doc.type === 'folder' ? '-' : doc.type === 'file' ? doc.fileSize : '—'}
                           </div>
                           <div className="col-span-1 text-right">
                             <button 
                               className="p-1.5 text-gray-400 hover:text-red-500 hover:bg-red-50 dark:hover:bg-red-400/10 rounded transition-colors opacity-0 group-hover:opacity-100"
                               onClick={(e) => { e.stopPropagation(); onDeleteDoc(doc); }}
                               title="删除"
                             >
                               <Trash2 size={16} />
                             </button>
                           </div>
                         </div>
                       );
                    })}
                  </div>
                </div>
              )}
            </div>
         )}
      </div>
    </div>
  );
};
