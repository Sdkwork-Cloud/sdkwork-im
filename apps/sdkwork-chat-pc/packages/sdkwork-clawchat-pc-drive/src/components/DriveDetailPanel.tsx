import React from 'react';
import { Cloud, Folder, File, FileText, Image, Download, Share2, Trash2, Edit2, Info, X } from 'lucide-react';
import { toast } from '@sdkwork/clawchat-pc-chat';

interface DriveDetailPanelProps {
  selectedItem: { type: 'folder' | 'file', id: string, name: string, size?: string, time?: string, fileType?: string };
  setSelectedItem: (item: any) => void;
  breadcrumbs: { id: string, name: string }[];
  setNewFolderName: (name: string) => void;
  setShowRenameModal: (show: boolean) => void;
  handleDownload: (e: React.MouseEvent, fileName: string) => void;
  handleDelete: (type: 'folder' | 'file', id: string) => void;
}

export const DriveDetailPanel: React.FC<DriveDetailPanelProps> = ({
  selectedItem,
  setSelectedItem,
  breadcrumbs,
  setNewFolderName,
  setShowRenameModal,
  handleDownload,
  handleDelete
}) => {
  const getFileIcon = (type: string) => {
    switch (type) {
      case 'pdf': return <FileText size={64} className="text-red-400 drop-shadow-md mb-4" />;
      case 'word': return <FileText size={64} className="text-blue-400 drop-shadow-md mb-4" />;
      case 'image': return <Image size={64} className="text-green-400 drop-shadow-md mb-4" />;
      case 'excel': return <File size={64} className="text-green-500 drop-shadow-md mb-4" />;
      default: return <File size={64} className="text-gray-400 drop-shadow-md mb-4" />;
    }
  };

  return (
    <div className="w-[300px] border-l border-white/5 bg-[#181818] p-6 flex flex-col shrink-0 overflow-y-auto custom-scrollbar animate-in slide-in-from-right duration-200">
       <div className="flex items-center justify-between mb-8">
          <h3 className="text-gray-200 font-medium flex items-center gap-2">
             <Info size={16} className="text-cyan-400" />
             详细信息
          </h3>
          <button 
            onClick={() => setSelectedItem(null)}
            className="text-gray-500 hover:text-white transition-colors p-1 rounded-md hover:bg-white/10"
          >
            <X size={16} />
          </button>
       </div>
       
       <div className="flex flex-col items-center justify-center py-6 bg-[#2b2b2d] rounded-xl border border-white/5 mb-6 shadow-inner">
          {selectedItem.type === 'folder' ? (
            <Folder size={64} className="text-yellow-500 drop-shadow-md mb-4" />
          ) : (
            getFileIcon(selectedItem.fileType || '')
          )}
          <div className="text-gray-200 font-medium text-center mt-4 px-4 break-words w-full">{selectedItem.name}</div>
       </div>
       
       <div className="space-y-4">
          <div className="pb-4 border-b border-white/5">
             <div className="text-xs text-gray-500 mb-1 font-medium">类型</div>
             <div className="text-sm text-gray-200 capitalize">
                {selectedItem.type === 'folder' ? '文件夹' : `${selectedItem.fileType?.toUpperCase() || '未知'} 文件`}
             </div>
          </div>
          
          {selectedItem.size && (
            <div className="pb-4 border-b border-white/5">
               <div className="text-xs text-gray-500 mb-1 font-medium">大小</div>
               <div className="text-sm text-gray-200 font-mono">{selectedItem.size}</div>
            </div>
          )}
          
          {selectedItem.time && (
            <div className="pb-4 border-b border-white/5">
               <div className="text-xs text-gray-500 mb-1 font-medium">修改时间</div>
               <div className="text-sm text-gray-200">{selectedItem.time}</div>
            </div>
          )}
          
          <div className="pb-4 border-b border-white/5">
             <div className="text-xs text-gray-500 mb-1 font-medium">位置</div>
             <div className="text-sm text-gray-200 flex items-center gap-1">
                <Cloud size={14} className="text-cyan-400" /> 
                {breadcrumbs.map(b => b.name).join(' / ')}
             </div>
          </div>
       </div>
       
       <div className="mt-8 flex flex-col gap-2">
          <button 
            onClick={() => {
               setNewFolderName(selectedItem.name);
               setShowRenameModal(true);
            }}
            className="w-full py-2 bg-[#2b2b2d] hover:bg-[#343438] text-gray-200 border border-white/10 rounded-lg text-sm font-medium transition-colors flex items-center justify-center gap-2">
             <Edit2 size={16} /> 重命名
          </button>
          <button 
            onClick={() => {
              navigator.clipboard.writeText(`https://drive.sdkwork.com/share/${selectedItem.id}`);
              toast('链接已复制到剪贴板', 'success');
            }}
            className="w-full py-2 bg-[#2b2b2d] hover:bg-[#343438] text-gray-200 border border-white/10 rounded-lg text-sm font-medium transition-colors flex items-center justify-center gap-2">
             <Share2 size={16} /> 分享链接
          </button>
          {selectedItem.type === 'file' && (
            <button 
              onClick={(e) => handleDownload(e, selectedItem.name)}
              className="w-full py-2 bg-[#2b2b2d] hover:bg-[#343438] text-gray-200 border border-white/10 rounded-lg text-sm font-medium transition-colors flex items-center justify-center gap-2">
               <Download size={16} /> 下载文件
            </button>
          )}
          <button 
            onClick={() => handleDelete(selectedItem.type, selectedItem.id)}
            className="w-full py-2 bg-red-900/20 hover:bg-red-900/40 text-red-400 border border-red-500/20 rounded-lg text-sm font-medium transition-colors flex items-center justify-center gap-2 mt-4"
          >
             <Trash2 size={16} /> 删除
          </button>
       </div>
    </div>
  );
};
