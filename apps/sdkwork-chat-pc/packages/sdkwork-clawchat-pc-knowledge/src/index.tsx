import React, { useState, useEffect, useRef } from 'react';
import { BookOpen, Plus, MessageSquare, X, Send } from 'lucide-react';
import { toast, ForwardModal } from '@sdkwork/clawchat-pc-chat';
import { knowledgeService, KnowledgeBase, KnowledgeDoc } from './services/KnowledgeService';
import type { Message } from '@sdkwork/clawchat-pc-types';
import { MainSidebar } from './components/MainSidebar';
import { DocSidebar } from './components/DocSidebar';
import { BaseOverview } from './components/BaseOverview';
import { DocEditor } from './components/DocEditor';
import { DocViewer } from './components/DocViewer';
import { CreateKBModal } from './components/CreateKBModal';
import { KnowledgeChatPanel } from './components/KnowledgeChatPanel';

export { knowledgeService };
export type { KnowledgeBase, KnowledgeDoc };

export const KnowledgeView: React.FC = () => {
  const [bases, setBases] = useState<KnowledgeBase[]>([]);
  const [selectedBase, setSelectedBase] = useState<KnowledgeBase | null>(null);
  const [docs, setDocs] = useState<KnowledgeDoc[]>([]);
  const [selectedDoc, setSelectedDoc] = useState<KnowledgeDoc | null>(null);
  const [isLoadingBases, setIsLoadingBases] = useState(true);
  const [isLoadingDocs, setIsLoadingDocs] = useState(false);
  const [isSidebarOpen, setIsSidebarOpen] = useState(true);
  const [isForwardModalOpen, setIsForwardModalOpen] = useState(false);
  const [isChatOpen, setIsChatOpen] = useState(true);
  const [chatInput, setChatInput] = useState('');
  const [chatMessages, setChatMessages] = useState<{ role: 'user' | 'bot'; text: string }[]>([]);

  // Search State
  const [searchQuery, setSearchQuery] = useState('');

  // Modals state
  const [isCreateKBModalOpen, setIsCreateKBModalOpen] = useState(false);
  const [newKBData, setNewKBData] = useState({ name: '', description: '', type: 'team' as 'team' | 'personal', logo: '🚀' });

  // Editor state
  const [isEditingDoc, setIsEditingDoc] = useState(false);
  const [editDocData, setEditDocData] = useState({ title: '', content: '' });

  // Drag and Drop state
  const [isDragging, setIsDragging] = useState(false);

  useEffect(() => {
    setIsLoadingBases(true);
    knowledgeService.getBases().then(data => {
      setBases(data);
      if (data.length > 0) {
        setSelectedBase(data[0]);
      }
      setIsLoadingBases(false);
    });
  }, []);

  useEffect(() => {
    if (selectedBase) {
      setIsLoadingDocs(true);
      knowledgeService.getDocs(selectedBase.id).then(data => {
        setDocs(data);
        if(data.length > 0 && !selectedDoc && !isEditingDoc) {
           setSelectedDoc(data[0]);
        }
        setIsLoadingDocs(false);
      });
      // reset chat when base changes
      setChatMessages([{ role: 'bot', text: `你好，我已经学习了【${selectedBase.name}】中的知识，你可以向我提问关于其中的任何内容。` }]);
    }
  }, [selectedBase]);

  const dragCounter = useRef(0);

  const handleDragEnter = (e: React.DragEvent) => {
    e.preventDefault();
    if (!selectedBase) return;
    dragCounter.current += 1;
    if (e.dataTransfer.items && e.dataTransfer.items.length > 0) {
      setIsDragging(true);
    }
  };

  const handleDragOver = (e: React.DragEvent) => {
    e.preventDefault();
    e.dataTransfer.dropEffect = 'copy';
  };

  const handleDragLeave = (e: React.DragEvent) => {
    e.preventDefault();
    dragCounter.current -= 1;
    if (dragCounter.current === 0) {
      setIsDragging(false);
    }
  };

  const handleDrop = (e: React.DragEvent) => {
    e.preventDefault();
    dragCounter.current = 0;
    setIsDragging(false);
    
    if (!selectedBase) return;

    if (e.dataTransfer.files && e.dataTransfer.files.length > 0) {
      const file = e.dataTransfer.files[0];
      toast(`成功上传文件: ${file.name}`, 'success');
      
      const fileData: Partial<KnowledgeDoc> = {
        baseId: selectedBase.id,
        title: file.name,
        type: 'file',
        fileName: file.name,
        fileSize: (file.size / 1024 / 1024).toFixed(2) + ' MB',
        fileUrl: URL.createObjectURL(file),
        fileMimeType: file.type
      };
      
      knowledgeService.createDoc(fileData).then(newDoc => {
        setDocs([newDoc, ...docs]);
        setSelectedDoc(newDoc);
      });
    }
  };

  const teamBases = bases.filter(b => b.type === 'team' && b.name.toLowerCase().includes(searchQuery.toLowerCase()));
  const personalBases = bases.filter(b => b.type === 'personal' && b.name.toLowerCase().includes(searchQuery.toLowerCase()));

  const getBaseIcon = (base: KnowledgeBase) => {
    if (base.logo.startsWith('http') || base.logo.startsWith('data:')) {
       return <img src={base.logo} alt={base.name} className="w-5 h-5 rounded-md object-cover" />;
    }
    if (base.logo) {
       return <span className="text-xl flex items-center justify-center">{base.logo}</span>;
    }
    return <BookOpen size={20} className="text-gray-400" />;
  };

  const handleUploadFile = (e: React.ChangeEvent<HTMLInputElement>, parentId?: string) => {
    if (e.target.files && e.target.files.length > 0) {
      const file = e.target.files[0];
      toast(`成功上传文件: ${file.name}`, 'success');
      
      const fileData: Partial<KnowledgeDoc> = {
        baseId: selectedBase!.id,
        title: file.name,
        type: 'file',
        fileName: file.name,
        fileSize: (file.size / 1024 / 1024).toFixed(2) + ' MB',
        fileUrl: URL.createObjectURL(file),
        fileMimeType: file.type,
        parentId: parentId
      };
      
      knowledgeService.createDoc(fileData).then(newDoc => {
        setDocs([newDoc, ...docs]);
      });
    }
  };

  const handleCreateFolder = (parentId: string | null) => {
    const folderName = window.prompt("请输入文件夹名称");
    if (!folderName || !folderName.trim()) return;

    knowledgeService.createDoc({
      baseId: selectedBase!.id,
      title: folderName.trim(),
      type: 'folder',
      parentId: parentId || undefined
    }).then(newFolder => {
      setDocs([newFolder, ...docs]);
      toast('文件夹创建成功', 'success');
    });
  };

  const handleCreateKB = () => {
    if (!newKBData.name.trim()) {
      toast('请输入知识库名称', 'error');
      return;
    }
    
    knowledgeService.createBase(newKBData).then(newBase => {
      setBases([newBase, ...bases]);
      setSelectedBase(newBase);
      setSelectedDoc(null);
      setIsCreateKBModalOpen(false);
      setNewKBData({ name: '', description: '', type: 'team', logo: '🚀' });
      toast('知识库创建成功', 'success');
    });
  };

  const handleSaveDoc = () => {
    if (!editDocData.title.trim()) {
      toast('请输入文档标题', 'error');
      return;
    }

    const docData: Partial<KnowledgeDoc> = {
      baseId: selectedBase!.id,
      title: editDocData.title,
      content: editDocData.content,
      type: 'markdown'
    };

    if (selectedDoc) {
      knowledgeService.updateDoc(selectedDoc.id, docData).then(updatedDoc => {
        setDocs(docs.map(d => d.id === updatedDoc.id ? (updatedDoc as KnowledgeDoc) : d));
        setSelectedDoc(updatedDoc as KnowledgeDoc);
        setIsEditingDoc(false);
        toast('文档已保存', 'success');
      });
    } else {
      knowledgeService.createDoc(docData).then(newDoc => {
        setDocs([newDoc, ...docs]);
        setSelectedDoc(newDoc);
        setIsEditingDoc(false);
        toast('文档创建成功', 'success');
      });
    }
  };

  const getDocAsMessage = (): Message => {
     if(selectedDoc?.type === 'file') {
        return {
           id: `doc-share-${Date.now()}`,
           chatId: 'forward',
           senderId: 'me',
           content: `[文件] ${selectedDoc.title}`,
           type: 'file',
           timestamp: Date.now(),
           fileName: selectedDoc.fileName,
           fileSize: selectedDoc.fileSize,
           fileUrl: selectedDoc.fileUrl
        }
     }
     
     return {
        id: `doc-share-${Date.now()}`,
        chatId: 'forward',
        senderId: 'me',
        content: `【知识库分享】 ${selectedDoc?.title}\n\n${selectedDoc?.content.substring(0, 100) || ''}...`,
        type: 'text',
        timestamp: Date.now()
     }
  }

  const handleDeleteKB = (base: KnowledgeBase) => {
    knowledgeService.deleteBase(base.id).then(() => {
      setBases(bases.filter(b => b.id !== base.id));
      if (selectedBase?.id === base.id) {
         setSelectedBase(null);
      }
      toast('知识库已删除', 'success');
    });
  };

  const handleDeleteDoc = (doc: KnowledgeDoc) => {
    knowledgeService.deleteDoc(doc.id).then(() => {
      setDocs(docs.filter(d => d.id !== doc.id));
      if (selectedDoc?.id === doc.id) {
         setSelectedDoc(null);
      }
      toast('文档已删除', 'success');
    });
  };

  const handleSendChat = () => {
    if (!chatInput.trim()) return;
    const msg = chatInput.trim();
    setChatMessages(prev => [...prev, { role: 'user', text: msg }]);
    setChatInput('');
    setTimeout(() => {
      setChatMessages(prev => [...prev, { role: 'bot', text: `这是根据知识库内容对“${msg}”的自动回复解答。` }]);
    }, 1000);
  };

  return (
    <div 
      className="flex-1 flex bg-white dark:bg-[#1e1e1e] min-w-0 min-h-0 text-gray-900 dark:text-gray-200 relative"
      onDragEnter={handleDragEnter}
      onDragOver={handleDragOver}
      onDragLeave={handleDragLeave}
      onDrop={handleDrop}
    >
      
      {/* Drag & Drop Overlay */}
      {isDragging && selectedBase && (
        <div className="absolute inset-0 bg-white/60 dark:bg-[#1e1e1e]/60 backdrop-blur-sm z-50 flex items-center justify-center p-8 transition-all pointer-events-none">
          <div className="w-full max-w-2xl h-full max-h-[400px] border-4 border-dashed border-indigo-500/50 bg-white/80 dark:bg-[#2a2a2a]/80 shadow-2xl rounded-3xl flex flex-col items-center justify-center gap-6">
             <div className="w-24 h-24 bg-indigo-50 dark:bg-indigo-500/20 rounded-full flex items-center justify-center text-indigo-500 dark:text-indigo-400">
               <Plus size={48} className="drop-shadow-lg" />
             </div>
             <h3 className="text-3xl font-bold text-gray-900 dark:text-gray-100">松开鼠标上传文件</h3>
             <p className="text-lg text-gray-500 dark:text-gray-400">文件将上传至知识库：<span className="text-indigo-600 dark:text-indigo-400 font-medium">{selectedBase.name}</span></p>
          </div>
        </div>
      )}

      {/* Sidebar: List of knowledge bases */}
      {isSidebarOpen && (
        <MainSidebar 
          bases={bases}
          selectedBase={selectedBase}
          isLoading={isLoadingBases}
          searchQuery={searchQuery}
          setSearchQuery={setSearchQuery}
          onSelectBase={(b) => { setSelectedBase(b); setSelectedDoc(null); setIsEditingDoc(false); }}
          onCreateKBClick={(type?: 'team' | 'personal') => {
            setNewKBData(prev => ({ ...prev, type: type || 'team' }));
            setIsCreateKBModalOpen(true);
          }}
          onCreateNoteClick={() => {
            if (!selectedBase) {
              toast('请先选择或新建一个知识库', 'error');
              return;
            }
            setSelectedDoc(null);
            setEditDocData({ title: '', content: '' });
            setIsEditingDoc(true);
          }}
          onDeleteKB={handleDeleteKB}
          getBaseIcon={getBaseIcon}
        />
      )}

      {/* Main Area: Doc List / Viewer / Editor */}
      <div className="flex-1 flex min-w-0 min-h-0 relative">
         {!selectedBase && bases.length === 0 ? (
           <div className="flex-1 flex flex-col items-center justify-center bg-gray-50 dark:bg-[#1e1e1e]">
              <div className="w-24 h-24 rounded-full bg-indigo-50 dark:bg-indigo-500/10 flex items-center justify-center mb-6">
                 <BookOpen size={40} className="text-indigo-500 dark:text-indigo-400" />
               </div>
               <h2 className="text-2xl font-semibold mb-2 text-gray-900 dark:text-gray-100">知识中心</h2>
               <p className="text-gray-500 mb-8 max-w-md text-center">沉淀团队知识，积累个人经验。创建一个知识库开始吧。</p>
               <button 
                 onClick={() => setIsCreateKBModalOpen(true)}
                 className="px-6 py-2.5 bg-indigo-600 hover:bg-indigo-700 dark:hover:bg-indigo-500 text-white rounded-xl transition-colors font-medium flex items-center gap-2 shadow-lg shadow-indigo-500/20"
               >
                 <Plus size={18} /> 新建知识库
               </button>
           </div>
         ) : !selectedDoc && !isEditingDoc ? (
            /* Knowledge Base Home showing docs */
            <BaseOverview 
              selectedBase={selectedBase!}
              docs={docs}
              isLoading={isLoadingDocs}
              onUploadFile={handleUploadFile}
              onCreateFolderClick={handleCreateFolder}
              onCreateDocClick={() => {
                 setSelectedDoc(null);
                 setEditDocData({ title: '', content: '' });
                 setIsEditingDoc(true);
              }}
              onSelectDoc={(doc) => setSelectedDoc(doc)}
              onDeleteDoc={handleDeleteDoc}
              onChatClick={() => setIsChatOpen(!isChatOpen)}
            />
         ) : isEditingDoc ? (
            <DocEditor 
              selectedBase={selectedBase!}
              selectedDoc={selectedDoc}
              editDocData={editDocData}
              setEditDocData={setEditDocData}
              onSave={handleSaveDoc}
              onCancel={() => setIsEditingDoc(false)}
            />
         ) : (
            /* Document Reader */
            <div className="flex-1 flex bg-gray-50 dark:bg-[#1e1e1e] min-w-0 min-h-0 relative">
               {/* Doc Navigation Sidebar */}
               <DocSidebar 
                 selectedBase={selectedBase!}
                 docs={docs}
                 isLoading={isLoadingDocs}
                 selectedDoc={selectedDoc}
                 setSelectedDoc={setSelectedDoc}
                 onUploadFile={handleUploadFile}
                 onCreateFolderClick={handleCreateFolder}
                 onCreateDocClick={() => {
                   setSelectedDoc(null);
                   setEditDocData({ title: '', content: '' });
                   setIsEditingDoc(true);
                 }}
                 onDeleteDoc={handleDeleteDoc}
               />

               {/* Editor / Reader Area */}
               <DocViewer 
                 selectedBase={selectedBase!}
                 selectedDoc={selectedDoc!}
                 onClose={() => setSelectedDoc(null)}
                 onForward={() => setIsForwardModalOpen(true)}
                 onEdit={() => {
                   setEditDocData({ title: selectedDoc!.title, content: selectedDoc!.content });
                   setIsEditingDoc(true);
                 }}
                 onDelete={() => handleDeleteDoc(selectedDoc!)}
               />
               
               {/* Floating Chat Button when viewing a doc */}
               {!isChatOpen && (
                 <button 
                   onClick={() => setIsChatOpen(true)}
                   className="absolute bottom-10 right-10 bg-indigo-600 hover:bg-indigo-700 text-white p-4 rounded-full shadow-2xl transition-transform hover:scale-105 z-40"
                   title="向知识库提问"
                 >
                   <MessageSquare size={24} />
                 </button>
               )}
            </div>
         )}
         
         {/* Knowledge Chat Panel */}
         {isChatOpen && selectedBase && (
           <KnowledgeChatPanel 
             selectedBaseName={selectedBase.name}
             chatMessages={chatMessages}
             chatInput={chatInput}
             setChatInput={setChatInput}
             onSendChat={handleSendChat}
             onClose={() => setIsChatOpen(false)}
           />
         )}
      </div>

      <ForwardModal 
        isOpen={isForwardModalOpen} 
        onClose={() => setIsForwardModalOpen(false)} 
        messages={[getDocAsMessage()]} 
      />

      <CreateKBModal 
        isOpen={isCreateKBModalOpen}
        onClose={() => setIsCreateKBModalOpen(false)}
        newKBData={newKBData}
        setNewKBData={setNewKBData}
        onCreate={handleCreateKB}
      />
    </div>
  );
};

