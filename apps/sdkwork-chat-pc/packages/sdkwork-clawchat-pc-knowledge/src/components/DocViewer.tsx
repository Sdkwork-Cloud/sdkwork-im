import React, { useState } from 'react';
import Markdown from 'react-markdown';
import { ChevronRight, Clock, Edit3, FileText, Globe, Lock, Search, Share2, Trash2, Download, Image as ImageIcon, Music, Video, ZoomIn, ZoomOut } from 'lucide-react';
import { KnowledgeBase, KnowledgeDoc } from '../services/KnowledgeService';

interface DocViewerProps {
  selectedBase: KnowledgeBase;
  selectedDoc: KnowledgeDoc;
  onClose: () => void;
  onForward: () => void;
  onEdit: () => void;
  onDelete: () => void;
}

export const DocViewer: React.FC<DocViewerProps> = ({
  selectedBase,
  selectedDoc,
  onClose,
  onForward,
  onEdit,
  onDelete
}) => {
  const [isZoomed, setIsZoomed] = useState(false);

  const renderFilePreview = () => {
    const { fileMimeType, fileUrl, fileName, fileSize } = selectedDoc;
    
    if (!fileUrl) return null;

    if (fileMimeType?.startsWith('image/')) {
      return (
        <div className="flex flex-col gap-6">
          <div className="flex flex-col relative group">
            <div className={`flex justify-center bg-gray-50 dark:bg-[#141414] rounded-2xl border border-gray-200 dark:border-white/5 p-2 md:p-4 shadow-sm dark:shadow-inner transition-all duration-300 custom-scrollbar ${isZoomed ? 'overflow-auto max-h-[90vh]' : 'overflow-hidden max-h-[80vh]'}`}>
              <img 
                src={fileUrl} 
                alt={fileName} 
                onClick={() => setIsZoomed(!isZoomed)}
                className={`rounded-lg shadow-xl dark:shadow-2xl transition-all duration-300 cursor-zoom-in ${isZoomed ? 'max-w-none w-auto h-auto' : 'max-w-full h-auto object-contain'}`} 
                style={isZoomed ? { cursor: 'zoom-out' } : {}}
              />
            </div>
            <button 
              onClick={() => setIsZoomed(!isZoomed)}
              className="absolute top-4 right-4 p-2.5 bg-white/50 dark:bg-black/50 hover:bg-white/80 dark:hover:bg-black/80 backdrop-blur-md rounded-xl text-gray-800 dark:text-white opacity-0 group-hover:opacity-100 transition-all shadow-xl border border-gray-200 dark:border-white/10"
              title={isZoomed ? "缩小" : "放大原始尺寸"}
            >
              {isZoomed ? <ZoomOut size={20} /> : <ZoomIn size={20} />}
            </button>
          </div>
          <div className="flex justify-center">
            <a href={fileUrl} download={fileName} target="_blank" rel="noreferrer" className="px-6 py-2 bg-white dark:bg-[#2a2a2a] hover:bg-gray-50 dark:hover:bg-[#333] border border-gray-200 dark:border-white/5 rounded-xl transition-colors font-medium flex items-center gap-2 text-sm text-gray-700 dark:text-gray-300 shadow-sm">
              <Download size={16} /> 下载原图 ({fileSize})
            </a>
          </div>
        </div>
      );
    }

    if (fileMimeType?.startsWith('video/')) {
      return (
        <div className="flex flex-col gap-4">
          <div className="flex justify-center bg-black dark:bg-[#0a0a0a] rounded-2xl border border-gray-200 dark:border-white/5 shadow-xl w-full h-[70vh] md:h-[85vh] overflow-hidden">
            <video src={fileUrl} controls className="w-full h-full object-contain rounded-2xl outline-none" />
          </div>
          <div className="flex justify-center">
            <a href={fileUrl} download={fileName} target="_blank" rel="noreferrer" className="px-6 py-2 bg-white dark:bg-[#2a2a2a] hover:bg-gray-50 dark:hover:bg-[#333] border border-gray-200 dark:border-white/5 rounded-xl transition-colors font-medium flex items-center gap-2 text-sm text-gray-700 dark:text-gray-300 shadow-sm">
              <Download size={16} /> 下载视频 ({fileSize})
            </a>
          </div>
        </div>
      );
    }

    if (fileMimeType?.startsWith('audio/')) {
      return (
        <div className="flex flex-col items-center justify-center py-16 bg-gray-50 dark:bg-[#181818] rounded-2xl border border-gray-200 dark:border-white/5 px-8 shadow-sm dark:shadow-inner overflow-hidden relative">
           <div className="absolute inset-0 bg-gradient-to-br from-indigo-500/5 to-purple-500/5 pointer-events-none" />
           <div className="w-24 h-24 bg-white dark:bg-gradient-to-br dark:from-[#2a2a2a] dark:to-[#202020] border border-gray-100 dark:border-white/10 rounded-full flex items-center justify-center mb-8 shadow-lg dark:shadow-xl relative z-10">
             <Music size={36} className="text-indigo-500 dark:text-indigo-400" />
           </div>
           <h3 className="text-xl font-medium text-gray-800 dark:text-gray-200 mb-2 relative z-10">{fileName}</h3>
           <p className="text-gray-500 text-sm mb-10 font-mono relative z-10">{fileSize}</p>
           <audio src={fileUrl} controls className="w-full max-w-md relative z-10" />
           <div className="flex justify-center mt-8 relative z-10">
             <a href={fileUrl} download={fileName} target="_blank" rel="noreferrer" className="px-6 py-2 bg-white dark:bg-[#2a2a2a] hover:bg-gray-50 dark:hover:bg-[#333] border border-gray-200 dark:border-white/5 rounded-xl transition-colors font-medium flex items-center gap-2 text-sm text-gray-700 dark:text-gray-300 shadow-sm">
               <Download size={16} /> 下载音频
             </a>
           </div>
        </div>
      );
    }

    if (fileMimeType === 'application/pdf') {
      return (
        <div className="flex flex-col gap-4 h-[85vh]">
          <div className="flex-1 bg-white dark:bg-[#181818] rounded-2xl border border-gray-200 dark:border-white/5 overflow-hidden shadow-lg dark:shadow-xl">
            <iframe src={fileUrl} className="w-full h-full border-none" title={fileName} />
          </div>
          <div className="flex justify-end shrink-0">
            <a href={fileUrl} download={fileName} target="_blank" rel="noreferrer" className="px-6 py-2 bg-white dark:bg-[#2a2a2a] hover:bg-gray-50 dark:hover:bg-[#333] border border-gray-200 dark:border-white/5 rounded-xl transition-colors font-medium flex items-center gap-2 text-sm text-gray-700 dark:text-gray-300 shadow-sm">
              <Download size={16} /> 新窗口打开或下载
            </a>
          </div>
        </div>
      );
    }

    // Default fallback for unknown files
    return (
      <div className="flex flex-col items-center justify-center py-24 border border-gray-200 dark:border-white/5 rounded-3xl bg-gray-50 dark:bg-[#1e1e1e] shadow-lg dark:shadow-xl drop-shadow-sm mt-4 bg-gradient-to-b from-white to-gray-50 dark:from-white/[0.02] dark:to-transparent">
        <div className="w-28 h-28 bg-white dark:bg-gradient-to-br dark:from-[#2a2a2a] dark:to-[#202020] border border-gray-100 dark:border-white/10 rounded-3xl flex items-center justify-center mb-8 text-gray-400 shadow-sm dark:shadow-inner">
          <FileText size={56} className="drop-shadow-md" />
        </div>
        <h3 className="text-2xl font-bold text-gray-800 dark:text-gray-200 mb-3">{fileName}</h3>
        <p className="text-gray-500 font-medium mb-10 bg-white dark:bg-white/5 px-4 py-1.5 rounded-full text-sm font-mono border border-gray-200 dark:border-white/5 shadow-sm">{fileSize}</p>
        <div className="flex gap-4">
          <button className="px-8 py-3 bg-white dark:bg-[#2a2a2a] border border-gray-200 dark:border-white/5 rounded-xl font-medium flex items-center gap-2 text-gray-400 dark:text-gray-500 cursor-not-allowed shadow-sm">
            <Search size={18} /> 暂不支持预览
          </button>
          <a href={fileUrl} download={fileName} target="_blank" rel="noreferrer" className="px-8 py-3 bg-indigo-600 hover:bg-indigo-700 dark:hover:bg-indigo-500 rounded-xl transition-colors font-medium shadow-md shadow-indigo-500/20 text-white flex items-center gap-2">
            <Download size={18} /> 文件下载
          </a>
        </div>
      </div>
    );
  };
  return (
    <div className="flex-1 overflow-y-auto custom-scrollbar bg-white dark:bg-[#1e1e1e] min-w-0 min-h-0">
      <div className="w-full max-w-[1600px] mx-auto px-4 py-4 md:px-8 md:py-6">
        <div className="flex flex-col gap-4 mb-6 pb-4 border-b border-gray-200 dark:border-white/5">
           <div className="flex items-center justify-between">
             <div className="flex border border-gray-200 dark:border-white/10 rounded-lg overflow-hidden lg:hidden mb-4">
                <button className="px-3 py-1.5 text-xs font-medium bg-gray-100 dark:bg-white/5 hover:bg-gray-200 dark:hover:bg-white/10 text-gray-600 dark:text-gray-300 flex items-center gap-2 shadow-sm" onClick={onClose}>
                  <ChevronRight size={14} className="rotate-180" /> 返回目录
                </button>
             </div>
             <div className="flex items-center gap-2 mb-2 w-full lg:w-auto ml-auto">
                <button 
                  className="p-2 text-gray-500 hover:text-gray-700 dark:text-gray-400 dark:hover:text-gray-200 hover:bg-gray-100 dark:hover:bg-white/5 rounded-lg transition-colors shadow-sm"
                  onClick={onForward}
                  title="分享发送给好友"
                >
                  <Share2 size={18} />
                </button>
                {selectedDoc.type !== 'file' && selectedDoc.type !== 'folder' && (
                  <button 
                    className="flex items-center gap-2 px-4 py-2 bg-indigo-600 hover:bg-indigo-700 dark:hover:bg-indigo-500 text-white rounded-lg transition-colors text-sm font-medium shadow-sm"
                    onClick={onEdit}
                  >
                    <Edit3 size={16} /> 编辑
                  </button>
                )}
                <button 
                  className="p-2 text-gray-500 hover:text-red-500 dark:text-gray-400 dark:hover:text-red-400 hover:bg-red-50 dark:hover:bg-red-400/10 rounded-lg transition-colors shadow-sm"
                  onClick={onDelete}
                  title="删除文档"
                >
                  <Trash2 size={18} />
                </button>
             </div>
           </div>

           <h1 className="text-3xl md:text-5xl font-bold text-gray-900 dark:text-gray-100 leading-tight tracking-tight">{selectedDoc.title}</h1>
           
           <div className="flex flex-wrap items-center gap-x-4 gap-y-2 text-sm text-gray-500 mt-2">
              <span className="flex items-center gap-1.5 px-3 py-1 bg-gray-100 dark:bg-white/5 rounded-full border border-gray-200 dark:border-white/5 shadow-sm">
                {selectedBase.type === 'team' ? <Globe size={14} /> : <Lock size={14} />} 
                {selectedBase.type === 'team' ? '团队可见' : '仅自己可见'}
              </span>
              <span className="flex items-center gap-1.5"><Clock size={14} /> 更新于 {new Date(selectedDoc.updatedAt || Date.now()).toLocaleString()}</span>
              <span className="flex items-center gap-1.5 border-l border-gray-300 dark:border-white/10 pl-4">{selectedDoc.author}</span>
           </div>
        </div>
        
        {selectedDoc.type === 'file' ? (
          <div className="mt-4">
            {renderFilePreview()}
          </div>
        ) : selectedDoc.type === 'folder' ? (
          <div className="mt-4 text-center py-10 bg-gray-50 dark:bg-transparent rounded-2xl border border-dashed border-gray-200 dark:border-white/10">
            <h3 className="text-lg font-medium text-gray-500 dark:text-gray-400">请在目录侧边栏中点击文件夹或展开查看子项</h3>
          </div>
        ) : (
          <div className="prose dark:prose-invert prose-indigo max-w-none prose-p:text-gray-700 dark:prose-p:text-gray-300 prose-headings:text-gray-900 dark:prose-headings:text-gray-100 prose-a:text-indigo-600 dark:prose-a:text-indigo-400 prose-strong:text-gray-900 dark:prose-strong:text-gray-200 prose-code:text-indigo-600 dark:prose-code:text-indigo-300 prose-code:bg-indigo-50 dark:prose-code:bg-indigo-500/10 prose-code:px-1.5 prose-code:py-0.5 prose-code:rounded prose-pre:bg-gray-50 dark:prose-pre:bg-[#181818] prose-pre:border prose-pre:border-gray-200 dark:prose-pre:border-white/5 prose-img:rounded-2xl prose-img:max-w-full prose-img:mx-auto prose-img:shadow-lg dark:prose-img:shadow-xl">
             {selectedDoc.content ? (
                <Markdown>{selectedDoc.content}</Markdown>
             ) : (
                <div className="text-gray-400 dark:text-gray-500 italic">这是一篇空文档...</div>
             )}
          </div>
        )}
      </div>
    </div>
  );
};
