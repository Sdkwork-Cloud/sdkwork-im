import React, { useState } from 'react';
import { Edit2, Folder, X } from 'lucide-react';

interface DriveModalsProps {
  showNewFolderModal: boolean;
  setShowNewFolderModal: (show: boolean) => void;
  showRenameModal: boolean;
  setShowRenameModal: (show: boolean) => void;
  newFolderName: string;
  setNewFolderName: (name: string) => void;
  handleCreateFolder: () => void | Promise<void>;
  selectedItem: { type: 'folder' | 'file'; id: string; name: string } | null;
  handleRenameSubmit: (newName: string) => void | Promise<void>;
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
  handleRenameSubmit,
}) => {
  const [isSubmitting, setIsSubmitting] = useState(false);

  const submitCreateFolder = async () => {
    if (!newFolderName.trim() || isSubmitting) {
      return;
    }

    setIsSubmitting(true);
    try {
      await handleCreateFolder();
    } finally {
      setIsSubmitting(false);
    }
  };

  const submitRename = async () => {
    if (!selectedItem || isSubmitting) {
      return;
    }

    const nextName = newFolderName.trim();
    if (!nextName || nextName === selectedItem.name) {
      setShowRenameModal(false);
      return;
    }

    setIsSubmitting(true);
    try {
      await handleRenameSubmit(nextName);
    } finally {
      setIsSubmitting(false);
    }
  };

  return (
    <>
      {showRenameModal && selectedItem && (
        <div className="absolute inset-0 bg-black/60 backdrop-blur-sm flex justify-center items-center z-50">
          <div className="bg-[#1e1e1e] border border-white/10 rounded-xl w-[400px] shadow-2xl p-6 animate-in zoom-in-95 duration-200">
            <div className="flex justify-between items-center mb-6">
              <h2 className="text-lg font-medium text-gray-100 flex items-center gap-2">
                <Edit2 size={20} className="text-gray-400" />
                Rename
              </h2>
              <button
                onClick={() => setShowRenameModal(false)}
                className="text-gray-500 hover:text-white p-1 rounded-md transition-colors"
                disabled={isSubmitting}
              >
                <X size={20} />
              </button>
            </div>
            <input
              type="text"
              value={newFolderName}
              onChange={(event) => setNewFolderName(event.target.value)}
              placeholder="Enter a new name"
              className="w-full bg-[#2b2b2d] border border-white/10 rounded-lg px-4 py-3 text-sm text-gray-200 outline-none focus:border-cyan-500 focus:bg-[#1e1e1e] transition-all placeholder:text-gray-500 mb-6 font-medium"
              autoFocus
              disabled={isSubmitting}
              onKeyDown={(event) => {
                if (event.key === 'Enter') {
                  void submitRename();
                }
              }}
            />
            <div className="flex gap-3 justify-end">
              <button
                onClick={() => setShowRenameModal(false)}
                disabled={isSubmitting}
                className="px-4 py-2 rounded-lg text-sm font-medium text-gray-400 hover:text-white hover:bg-white/5 disabled:opacity-50 transition-colors"
              >
                Cancel
              </button>
              <button
                onClick={() => {
                  void submitRename();
                }}
                disabled={isSubmitting}
                className="bg-cyan-600 hover:bg-cyan-500 disabled:bg-cyan-600/50 text-white px-5 py-2 rounded-lg text-sm font-medium transition-colors shadow-lg shadow-cyan-500/20"
              >
                {isSubmitting ? 'Saving...' : 'Save'}
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
                New folder
              </h2>
              <button
                onClick={() => setShowNewFolderModal(false)}
                className="text-gray-500 hover:text-white p-1 rounded-md transition-colors"
                disabled={isSubmitting}
              >
                <X size={20} />
              </button>
            </div>
            <input
              type="text"
              value={newFolderName}
              onChange={(event) => setNewFolderName(event.target.value)}
              placeholder="Enter a folder name"
              className="w-full bg-[#2b2b2d] border border-white/10 rounded-lg px-4 py-3 text-sm text-gray-200 outline-none focus:border-cyan-500 focus:bg-[#1e1e1e] transition-all placeholder:text-gray-500 mb-6 font-medium"
              autoFocus
              disabled={isSubmitting}
              onKeyDown={(event) => {
                if (event.key === 'Enter') {
                  void submitCreateFolder();
                }
              }}
            />
            <div className="flex gap-3 justify-end">
              <button
                onClick={() => setShowNewFolderModal(false)}
                disabled={isSubmitting}
                className="px-4 py-2 rounded-lg text-sm font-medium text-gray-400 hover:text-white hover:bg-white/5 disabled:opacity-50 transition-colors"
              >
                Cancel
              </button>
              <button
                onClick={() => {
                  void submitCreateFolder();
                }}
                disabled={isSubmitting}
                className="bg-cyan-600 hover:bg-cyan-500 disabled:bg-cyan-600/50 text-white px-5 py-2 rounded-lg text-sm font-medium transition-colors shadow-lg shadow-cyan-500/20"
              >
                {isSubmitting ? 'Creating...' : 'Create'}
              </button>
            </div>
          </div>
        </div>
      )}
    </>
  );
};
