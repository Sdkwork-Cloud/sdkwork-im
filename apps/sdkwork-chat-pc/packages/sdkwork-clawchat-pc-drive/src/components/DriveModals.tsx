import React from 'react';
import { Folder, Edit2, X } from 'lucide-react';
import { toast } from '@sdkwork/clawchat-pc-chat';

interface DriveModalsProps {
  showNewFolderModal: boolean;
  setShowNewFolderModal: (show: boolean) => void;
  showRenameModal: boolean;
  setShowRenameModal: (show: boolean) => void;
  newFolderName: string;
  setNewFolderName: (name: string) => void;
  handleCreateFolder: () => void;
  selectedItem: { type: 'folder' | 'file', id: string, name: string } | null;
  handleRenameSubmit: (newName: string) => void;
}

export const DriveModals: React.FC<DriveModalsProps> = ({
  showNewFolderModal,
  setShowNewFolderModal,
  showRenameModal,
  setShowRenameModal,
  newFolderName,
  setNewFolderName,
  handleCreateFolder,
  selectedItem,
  handleRenameSubmit
}) => {
  return (
    <>
      {showRenameModal && selectedItem && (
        <div className="absolute inset-0 bg-black/60 backdrop-blur-sm flex justify-center items-center z-50">
           <div className="bg-[#1e1e1e] border border-white/10 rounded-xl w-[400px] shadow-2xl p-6 animate-in zoom-in-95 duration-200">
              <div className="flex justify-between items-center mb-6">
                 <h2 className="text-lg font-medium text-gray-100 flex items-center gap-2">
                   <Edit2 size={20} className="text-gray-400" />
                   重命名
                 </h2>
                 <button onClick={() => setShowRenameModal(false)} className="text-gray-500 hover:text-white p-1 rounded-md transition-colors"><X size={20} /></button>
              </div>
              <input 
                type="text" 
                value={newFolderName}
                onChange={e => setNewFolderName(e.target.value)}
                placeholder="请输入新名称" 
                className="w-full bg-[#2b2b2d] border border-white/10 rounded-lg px-4 py-3 text-sm text-gray-200 outline-none focus:border-cyan-500 focus:bg-[#1e1e1e] transition-all placeholder:text-gray-500 mb-6 font-medium"
                autoFocus
                onKeyDown={(e) => {
                  if (e.key === 'Enter') {
                     if (newFolderName && newFolderName.trim() && newFolderName.trim() !== selectedItem.name) {
                        handleRenameSubmit(newFolderName.trim());
                     }
                     setShowRenameModal(false);
                  }
                }}
              />
              <div className="flex gap-3 justify-end">
                 <button 
                   onClick={() => setShowRenameModal(false)}
                   className="px-4 py-2 rounded-lg text-sm font-medium text-gray-400 hover:text-white hover:bg-white/5 transition-colors"
                 >
                   取消
                 </button>
                 <button 
                   onClick={() => {
                     if (newFolderName && newFolderName.trim() && newFolderName.trim() !== selectedItem.name) {
                        handleRenameSubmit(newFolderName.trim());
                     }
                     setShowRenameModal(false);
                   }}
                   className="bg-cyan-600 hover:bg-cyan-500 text-white px-5 py-2 rounded-lg text-sm font-medium transition-colors shadow-lg shadow-cyan-500/20"
                 >
                   保存
                 </button>
              </div>
           </div>
        </div>
      )}
      
      {showNewFolderModal && (
        <div className="absolute inset-0 bg-black/60 backdrop-blur-sm flex justify-center items-center z-50">
           <div className="bg-[#1e1e1e] border border-white/10 rounded-xl w-[400px] shadow-2xl p-6 animate-in zoom-in-95 duration-200">
              <div className="flex justify-between items-center mb-6">
                 <h2 className="text-lg font-medium text-gray-100 flex items-center gap-2">
                   <Folder size={20} className="text-yellow-500" />
                   新建文件夹
                 </h2>
                 <button onClick={() => setShowNewFolderModal(false)} className="text-gray-500 hover:text-white p-1 rounded-md transition-colors"><X size={20} /></button>
              </div>
              <input 
                type="text" 
                value={newFolderName}
                onChange={e => setNewFolderName(e.target.value)}
                placeholder="请输入文件夹名称" 
                className="w-full bg-[#2b2b2d] border border-white/10 rounded-lg px-4 py-3 text-sm text-gray-200 outline-none focus:border-cyan-500 focus:bg-[#1e1e1e] transition-all placeholder:text-gray-500 mb-6 font-medium"
                autoFocus
                onKeyDown={(e) => {
                  if (e.key === 'Enter') handleCreateFolder();
                }}
              />
              <div className="flex gap-3 justify-end">
                 <button 
                   onClick={() => setShowNewFolderModal(false)}
                   className="px-4 py-2 rounded-lg text-sm font-medium text-gray-400 hover:text-white hover:bg-white/5 transition-colors"
                 >
                   取消
                 </button>
                 <button 
                   onClick={handleCreateFolder}
                   className="bg-cyan-600 hover:bg-cyan-500 text-white px-5 py-2 rounded-lg text-sm font-medium transition-colors shadow-lg shadow-cyan-500/20"
                 >
                   创建
                 </button>
              </div>
           </div>
        </div>
      )}
    </>
  );
};
