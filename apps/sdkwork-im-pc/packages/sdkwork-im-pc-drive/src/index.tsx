import React, { useEffect, useRef, useState } from 'react';
import {
  ChevronRight,
  Cloud,
  Download,
  File,
  FileText,
  Folder,
  Grid,
  Image,
  List,
  Search,
  Trash2,
  Upload,
} from 'lucide-react';
import { toast } from '@sdkwork/im-pc-chat';
import { DriveDetailPanel } from './components/DriveDetailPanel';
import { DriveModals } from './components/DriveModals';
import { driveService, type DriveFileItem, type FolderItem } from './services/DriveService';

export interface SelectedDriveItem {
  type: 'folder' | 'file';
  id: string;
  name: string;
  size?: string;
  time?: string;
  fileType?: DriveFileItem['type'];
}

export interface DriveBreadcrumb {
  id: string;
  name: string;
}

export const DriveView: React.FC = () => {
  const [folders, setFolders] = useState<FolderItem[]>([]);
  const [recentFiles, setRecentFiles] = useState<DriveFileItem[]>([]);
  const [viewMode, setViewMode] = useState<'grid' | 'list'>('grid');
  const [selectedItem, setSelectedItem] = useState<SelectedDriveItem | null>(null);
  const [showNewFolderModal, setShowNewFolderModal] = useState(false);
  const [showRenameModal, setShowRenameModal] = useState(false);
  const [newFolderName, setNewFolderName] = useState('');
  const [isUploading, setIsUploading] = useState(false);
  const fileInputRef = useRef<HTMLInputElement>(null);
  const [breadcrumbs, setBreadcrumbs] = useState<DriveBreadcrumb[]>([{ id: 'root', name: 'My files' }]);

  const refreshData = async () => {
    try {
      const [nextFolders, nextRecentFiles] = await Promise.all([
        driveService.getFolders(),
        driveService.getRecentFiles(),
      ]);
      setFolders(nextFolders);
      setRecentFiles(nextRecentFiles);
    } catch (err) {
      toast('Drive data failed to load.', 'error');
    }
  };

  useEffect(() => {
    void refreshData();
  }, []);

  const getFileIcon = (type: string) => {
    switch (type) {
      case 'pdf':
        return <FileText size={40} className="text-red-400" />;
      case 'word':
        return <FileText size={40} className="text-blue-400" />;
      case 'image':
        return <Image size={40} className="text-green-400" />;
      case 'excel':
        return <File size={40} className="text-green-500" />;
      default:
        return <File size={40} className="text-gray-400" />;
    }
  };

  const getFileIconSmall = (type: string) => {
    switch (type) {
      case 'pdf':
        return <FileText size={20} className="text-red-400" />;
      case 'word':
        return <FileText size={20} className="text-blue-400" />;
      case 'image':
        return <Image size={20} className="text-green-400" />;
      case 'excel':
        return <File size={20} className="text-green-500" />;
      default:
        return <File size={20} className="text-gray-400" />;
    }
  };

  const handleCreateFolder = async () => {
    if (!newFolderName.trim()) {
      return;
    }

    try {
      await driveService.createFolder(newFolderName);
      setNewFolderName('');
      setShowNewFolderModal(false);
      toast('Folder created.', 'success');
      await refreshData();
    } catch (err) {
      toast('Folder creation failed.', 'error');
      throw err;
    }
  };

  const handleDownload = async (event: React.MouseEvent, fileId: string, fileName: string) => {
    event.stopPropagation();

    try {
      const grant = await driveService.createDownload(fileId);
      window.open(grant.downloadUrl, '_blank', 'noopener,noreferrer');
      toast(`Download started: ${fileName}`, 'success');
    } catch (err) {
      toast('Download failed.', 'error');
    }
  };

  const handleShare = async () => {
    toast('Drive share links are unavailable until the Drive SDK returns a copyable delivery URL.', 'error');
  };

  const handleFileUpload = async (event: React.ChangeEvent<HTMLInputElement>) => {
    const file = event.target.files?.[0];
    if (!file) {
      return;
    }

    setIsUploading(true);
    try {
      await driveService.uploadFile(file);
      toast(`Uploaded: ${file.name}`, 'success');
      await refreshData();
    } catch (err) {
      toast('Upload failed.', 'error');
    } finally {
      setIsUploading(false);
      event.currentTarget.value = '';
    }
  };

  const handleDelete = async (type: 'folder' | 'file', id: string) => {
    try {
      if (type === 'folder') {
        await driveService.deleteFolder(id);
      } else {
        await driveService.deleteFile(id);
      }
      toast('Deleted.', 'success');
      if (selectedItem?.id === id) {
        setSelectedItem(null);
      }
      await refreshData();
    } catch (err) {
      toast('Delete failed.', 'error');
    }
  };

  const handleRenameSubmit = async (newName: string) => {
    if (!selectedItem) {
      return;
    }

    try {
      if (selectedItem.type === 'folder') {
        await driveService.renameFolder(selectedItem.id, newName);
        setFolders((currentFolders) => currentFolders.map((folder) => (
          folder.id === selectedItem.id ? { ...folder, name: newName } : folder
        )));
      } else {
        await driveService.renameFile(selectedItem.id, newName);
        setRecentFiles((currentFiles) => currentFiles.map((file) => (
          file.id === selectedItem.id ? { ...file, name: newName } : file
        )));
      }
      setSelectedItem({ ...selectedItem, name: newName });
      setShowRenameModal(false);
      toast(`Renamed to ${newName}.`, 'success');
    } catch (err) {
      toast('Rename failed.', 'error');
      throw err;
    }
  };

  return (
    <div className="flex-1 flex flex-col bg-[#1e1e1e] min-w-0 min-h-0 relative">
      <input type="file" ref={fileInputRef} className="hidden" onChange={handleFileUpload} />

      <div className="h-16 px-6 border-b border-white/5 bg-[#181818] flex items-center justify-between shrink-0">
        <div className="flex items-center gap-2 text-xl font-medium text-gray-200">
          <Cloud size={24} className="text-cyan-400" /> Drive
        </div>
        <div className="flex items-center gap-3">
          <div className="relative group">
            <Search
              className="absolute left-3 top-1/2 -translate-y-1/2 text-gray-500 group-focus-within:text-cyan-400 transition-colors"
              size={16}
            />
            <input
              type="text"
              placeholder="Search files..."
              className="w-[280px] bg-[#2b2b2d] border border-transparent focus:border-cyan-500/50 rounded-lg pl-9 pr-4 py-2 text-sm text-gray-200 outline-none transition-all placeholder:text-gray-500 focus:bg-[#1e1e1e] focus:shadow-[0_0_0_2px_rgba(34,211,238,0.2)]"
            />
          </div>

          <div className="h-6 w-px bg-white/10 mx-2" />

          <button
            onClick={() => setShowNewFolderModal(true)}
            className="text-gray-400 hover:text-gray-200 p-2 hover:bg-white/5 rounded-lg transition-colors flex items-center gap-2 text-sm font-medium"
          >
            <Folder size={18} /> New
          </button>

          <button
            onClick={() => fileInputRef.current?.click()}
            disabled={isUploading}
            className="bg-cyan-600 hover:bg-cyan-500 disabled:bg-cyan-600/50 text-white font-medium px-4 py-2 flex items-center gap-2 rounded-lg transition-colors shadow-lg shadow-cyan-500/20 active:scale-95"
          >
            {isUploading ? <Upload size={16} className="animate-bounce" /> : <Upload size={16} />}
            {isUploading ? 'Uploading...' : 'Upload file'}
          </button>
        </div>
      </div>

      <div className="flex-1 flex min-h-0">
        <div className="flex-1 flex flex-col min-w-0 bg-[#1e1e1e]">
          <div className="h-14 px-6 border-b border-white/5 flex items-center justify-between shrink-0">
            <div className="flex items-center gap-1 text-sm text-gray-400">
              {breadcrumbs.map((crumb, index) => (
                <React.Fragment key={crumb.id}>
                  <span
                    className="hover:text-cyan-400 cursor-pointer transition-colors max-w-[150px] truncate"
                    onClick={() => {
                      setBreadcrumbs(breadcrumbs.slice(0, index + 1));
                    }}
                  >
                    {crumb.name}
                  </span>
                  {index < breadcrumbs.length - 1 && <ChevronRight size={14} className="text-gray-600" />}
                </React.Fragment>
              ))}
            </div>
            <div className="flex items-center gap-1 bg-[#2b2b2d] p-1 rounded-lg border border-white/5">
              <button
                onClick={() => setViewMode('grid')}
                className={`p-1.5 rounded-md transition-colors ${viewMode === 'grid' ? 'bg-[#3b3b3d] text-cyan-400' : 'text-gray-500 hover:text-gray-300'}`}
              >
                <Grid size={16} />
              </button>
              <button
                onClick={() => setViewMode('list')}
                className={`p-1.5 rounded-md transition-colors ${viewMode === 'list' ? 'bg-[#3b3b3d] text-cyan-400' : 'text-gray-500 hover:text-gray-300'}`}
              >
                <List size={16} />
              </button>
            </div>
          </div>

          <div className="flex-1 overflow-y-auto p-6 md:p-8 custom-scrollbar">
            {folders.length > 0 && (
              <div className="mb-10">
                <div className="flex items-center justify-between mb-4 px-1">
                  <h2 className="text-gray-300 text-sm font-semibold tracking-wide">Folders</h2>
                </div>
                {viewMode === 'grid' ? (
                  <div className="grid grid-cols-2 md:grid-cols-4 xl:grid-cols-5 gap-4">
                    {folders.map((folder) => (
                      <div
                        key={folder.id}
                        onClick={() => setSelectedItem({ type: 'folder', id: folder.id, name: folder.name })}
                        onDoubleClick={() => setBreadcrumbs([...breadcrumbs, { id: folder.id, name: folder.name }])}
                        className={`bg-[#2b2b2d] border p-4 rounded-xl flex flex-col gap-3 cursor-pointer transition-all group ${selectedItem?.id === folder.id ? 'border-cyan-500 bg-cyan-900/10 shadow-[0_0_15px_rgba(34,211,238,0.1)]' : 'border-white/5 hover:border-white/20 hover:bg-[#343438]'}`}
                      >
                        <div className="flex justify-between items-start">
                          <Folder size={36} className="text-yellow-500 group-hover:scale-105 transition-transform" />
                          <div className="opacity-0 group-hover:opacity-100 transition-opacity">
                            <button
                              onClick={(event) => {
                                event.stopPropagation();
                                void handleDelete('folder', folder.id);
                              }}
                              className="text-gray-500 hover:text-red-400 transition-colors p-1"
                            >
                              <Trash2 size={16} />
                            </button>
                          </div>
                        </div>
                        <div>
                          <div className="text-sm text-gray-200 font-medium truncate group-hover:text-cyan-50 transition-colors">
                            {folder.name}
                          </div>
                          <div className="text-xs text-gray-500 mt-1">{folder.fileCount} items</div>
                        </div>
                      </div>
                    ))}
                  </div>
                ) : (
                  <div className="bg-[#2b2b2d] border border-white/5 rounded-xl overflow-hidden">
                    <div className="grid grid-cols-12 gap-4 p-4 border-b border-white/5 text-gray-400 text-xs font-semibold bg-[#1e1e1e]/80 uppercase tracking-wider">
                      <div className="col-span-8 px-2">Name</div>
                      <div className="col-span-4 px-2">Items</div>
                    </div>
                    {folders.map((folder) => (
                      <div
                        key={folder.id}
                        onClick={() => setSelectedItem({ type: 'folder', id: folder.id, name: folder.name })}
                        onDoubleClick={() => setBreadcrumbs([...breadcrumbs, { id: folder.id, name: folder.name }])}
                        className={`grid grid-cols-12 gap-4 p-3 border-b border-white/5 items-center cursor-pointer transition-colors group ${selectedItem?.id === folder.id ? 'bg-cyan-900/20' : 'hover:bg-white/5'}`}
                      >
                        <div className="col-span-8 flex items-center gap-3 px-2">
                          <Folder size={20} className="text-yellow-500" />
                          <span className={`text-sm font-medium ${selectedItem?.id === folder.id ? 'text-cyan-400' : 'text-gray-200'}`}>
                            {folder.name}
                          </span>
                        </div>
                        <div className="col-span-2 text-sm text-gray-400 px-2">{folder.fileCount}</div>
                        <div className="col-span-2 flex justify-end px-2">
                          <button
                            onClick={(event) => {
                              event.stopPropagation();
                              void handleDelete('folder', folder.id);
                            }}
                            className="text-gray-500 hover:text-red-400 p-1 opacity-0 group-hover:opacity-100 transition-opacity"
                          >
                            <Trash2 size={16} />
                          </button>
                        </div>
                      </div>
                    ))}
                  </div>
                )}
              </div>
            )}

            <div>
              <div className="flex items-center justify-between mb-4 px-1">
                <h2 className="text-gray-300 text-sm font-semibold tracking-wide">Recent files</h2>
              </div>

              {viewMode === 'grid' ? (
                <div className="grid grid-cols-2 md:grid-cols-4 xl:grid-cols-5 gap-4">
                  {recentFiles.map((file) => (
                    <div
                      key={file.id}
                      onClick={() => setSelectedItem({
                        type: 'file',
                        id: file.id,
                        name: file.name,
                        size: file.size,
                        time: file.time,
                        fileType: file.type,
                      })}
                      className={`bg-[#2b2b2d] border rounded-xl flex flex-col cursor-pointer transition-all overflow-hidden group ${selectedItem?.id === file.id ? 'border-cyan-500 shadow-[0_0_15px_rgba(34,211,238,0.1)]' : 'border-white/5 hover:border-white/20 hover:bg-[#343438]'}`}
                    >
                      <div className="h-32 bg-[#1e1e1e]/50 flex items-center justify-center relative">
                        {getFileIcon(file.type)}
                        <div className="absolute top-2 right-2 flex opacity-0 group-hover:opacity-100 transition-opacity bg-[#2b2b2d] rounded-lg border border-white/10 p-1 shadow-lg">
                          <button
                            onClick={(event) => {
                              void handleDownload(event, file.id, file.name);
                            }}
                            className="text-gray-400 hover:text-cyan-400 p-1 transition-colors"
                          >
                            <Download size={14} />
                          </button>
                          <button
                            onClick={(event) => {
                              event.stopPropagation();
                              void handleDelete('file', file.id);
                            }}
                            className="text-gray-400 hover:text-red-400 p-1 transition-colors"
                          >
                            <Trash2 size={14} />
                          </button>
                        </div>
                      </div>
                      <div className="p-4 bg-[#2b2b2d] group-hover:bg-[#343438] transition-colors border-t border-white/5">
                        <div className="text-sm text-gray-200 font-medium truncate mb-1" title={file.name}>
                          {file.name}
                        </div>
                        <div className="text-xs text-gray-500 font-mono">{file.size} - {file.time}</div>
                      </div>
                    </div>
                  ))}
                </div>
              ) : (
                <div className="bg-[#2b2b2d] border border-white/5 rounded-xl overflow-hidden">
                  <div className="grid grid-cols-12 gap-4 p-4 border-b border-white/5 text-gray-400 text-xs font-semibold bg-[#1e1e1e]/80 uppercase tracking-wider">
                    <div className="col-span-6 px-2">Name</div>
                    <div className="col-span-2 px-2">Size</div>
                    <div className="col-span-3 px-2">Updated</div>
                    <div className="col-span-1 text-right px-2">Actions</div>
                  </div>

                  {recentFiles.map((file) => (
                    <div
                      key={file.id}
                      onClick={() => setSelectedItem({
                        type: 'file',
                        id: file.id,
                        name: file.name,
                        size: file.size,
                        time: file.time,
                        fileType: file.type,
                      })}
                      className={`grid grid-cols-12 gap-4 p-3 border-b border-white/5 items-center cursor-pointer transition-colors group last:border-b-0 ${selectedItem?.id === file.id ? 'bg-cyan-900/20' : 'hover:bg-white/5'}`}
                    >
                      <div className="col-span-6 flex items-center gap-3 px-2">
                        {getFileIconSmall(file.type)}
                        <span className={`text-sm font-medium truncate ${selectedItem?.id === file.id ? 'text-cyan-400' : 'text-gray-200'}`}>
                          {file.name}
                        </span>
                      </div>
                      <div className="col-span-2 text-sm text-gray-400 font-mono px-2">{file.size}</div>
                      <div className="col-span-3 text-sm text-gray-400 px-2">{file.time}</div>
                      <div className="col-span-1 flex items-center justify-end gap-1 px-2 opacity-0 group-hover:opacity-100 transition-opacity">
                        <button
                          onClick={(event) => {
                            void handleDownload(event, file.id, file.name);
                          }}
                          className="text-gray-400 hover:text-cyan-400 p-1.5 rounded-lg hover:bg-white/10 transition-colors"
                        >
                          <Download size={14} />
                        </button>
                        <button
                          onClick={(event) => {
                            event.stopPropagation();
                            void handleDelete('file', file.id);
                          }}
                          className="text-gray-400 hover:text-red-400 p-1.5 rounded-lg hover:bg-white/10 transition-colors"
                        >
                          <Trash2 size={14} />
                        </button>
                      </div>
                    </div>
                  ))}
                </div>
              )}
            </div>
          </div>
        </div>

        {selectedItem && (
          <DriveDetailPanel
            selectedItem={selectedItem}
            setSelectedItem={setSelectedItem}
            breadcrumbs={breadcrumbs}
            setNewFolderName={setNewFolderName}
            setShowRenameModal={setShowRenameModal}
            handleDownload={handleDownload}
            handleShare={handleShare}
            handleDelete={handleDelete}
          />
        )}
      </div>

      <DriveModals
        showNewFolderModal={showNewFolderModal}
        setShowNewFolderModal={setShowNewFolderModal}
        showRenameModal={showRenameModal}
        setShowRenameModal={setShowRenameModal}
        newFolderName={newFolderName}
        setNewFolderName={setNewFolderName}
        handleCreateFolder={handleCreateFolder}
        selectedItem={selectedItem}
        handleRenameSubmit={handleRenameSubmit}
      />
    </div>
  );
};
