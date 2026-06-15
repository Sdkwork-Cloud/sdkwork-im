import React, { useEffect, useRef, useState } from 'react';
import { BookOpen, MessageSquare, Plus } from 'lucide-react';
import { ForwardModal, toast } from '@sdkwork/im-pc-chat';
import type { Message } from '@sdkwork/im-pc-types';
import { BaseOverview } from './components/BaseOverview';
import { CreateKBModal } from './components/CreateKBModal';
import { DocEditor } from './components/DocEditor';
import { DocSidebar } from './components/DocSidebar';
import { DocViewer } from './components/DocViewer';
import { KnowledgeChatPanel } from './components/KnowledgeChatPanel';
import { MainSidebar } from './components/MainSidebar';
import { knowledgeAiService } from './services/KnowledgeAiService';
import {
  createKnowledgeBaseResourceId,
  createKnowledgeDocumentResourceId,
  knowledgeService,
  type KnowledgeBase,
  type KnowledgeDoc,
} from './services/KnowledgeService';

export { knowledgeAiService, knowledgeService };
export type { KnowledgeBase, KnowledgeDoc };

type KnowledgeChatMessage = { role: 'user' | 'bot'; text: string };

function getErrorMessage(error: unknown, fallback: string): string {
  return error instanceof Error && error.message ? error.message : fallback;
}

function createInitialChatMessage(baseName: string): KnowledgeChatMessage {
  return {
    role: 'bot',
    text: `Knowledge base "${baseName}" is ready. Ask a question about its documents.`,
  };
}

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
  const [chatMessages, setChatMessages] = useState<KnowledgeChatMessage[]>([]);
  const [isSendingChat, setIsSendingChat] = useState(false);
  const [searchQuery, setSearchQuery] = useState('');
  const [isCreateKBModalOpen, setIsCreateKBModalOpen] = useState(false);
  const [newKBData, setNewKBData] = useState({
    description: '',
    logo: 'KB',
    name: '',
    type: 'team' as 'team' | 'personal',
  });
  const [isEditingDoc, setIsEditingDoc] = useState(false);
  const [editDocData, setEditDocData] = useState({ title: '', content: '' });
  const [isDragging, setIsDragging] = useState(false);
  const dragCounter = useRef(0);

  useEffect(() => {
    let isActive = true;
    setIsLoadingBases(true);
    knowledgeService.getBases()
      .then((data) => {
        if (!isActive) {
          return;
        }
        setBases(data);
        setSelectedBase(data[0] ?? null);
      })
      .catch((error) => {
        if (isActive) {
          toast(getErrorMessage(error, 'Failed to load knowledge bases.'), 'error');
        }
      })
      .finally(() => {
        if (isActive) {
          setIsLoadingBases(false);
        }
      });
    return () => {
      isActive = false;
    };
  }, []);

  useEffect(() => {
    if (!selectedBase) {
      setDocs([]);
      setSelectedDoc(null);
      setChatMessages([]);
      return;
    }

    let isActive = true;
    setIsLoadingDocs(true);
    knowledgeService.getDocs(selectedBase.id)
      .then((data) => {
        if (!isActive) {
          return;
        }
        setDocs(data);
        setSelectedDoc((currentDoc) => currentDoc ?? data[0] ?? null);
      })
      .catch((error) => {
        if (isActive) {
          toast(getErrorMessage(error, 'Failed to load knowledge documents.'), 'error');
        }
      })
      .finally(() => {
        if (isActive) {
          setIsLoadingDocs(false);
        }
      });
    setChatMessages([createInitialChatMessage(selectedBase.name)]);

    return () => {
      isActive = false;
    };
  }, [selectedBase?.id]);

  const notifyKnowledgeFileUploadUnavailable = (fileName?: string) => {
    const suffix = fileName ? `: ${fileName}` : '';
    toast(`Knowledge file upload requires Drive-backed upload integration before creating a file document${suffix}.`, 'error');
  };

  const buildKnowledgeChatContext = (): string => {
    const documentContext = docs
      .filter((doc) => doc.type !== 'folder')
      .slice(0, 12)
      .map((doc) => {
        const content = doc.content?.trim();
        return content ? `# ${doc.title}\n${content}` : `# ${doc.title}`;
      });
    if (documentContext.length > 0) {
      return documentContext.join('\n\n');
    }
    return selectedBase?.description || selectedBase?.name || '';
  };

  const handleDragEnter = (event: React.DragEvent) => {
    event.preventDefault();
    if (!selectedBase) {
      return;
    }
    dragCounter.current += 1;
    if (event.dataTransfer.items && event.dataTransfer.items.length > 0) {
      setIsDragging(true);
    }
  };

  const handleDragOver = (event: React.DragEvent) => {
    event.preventDefault();
    event.dataTransfer.dropEffect = 'copy';
  };

  const handleDragLeave = (event: React.DragEvent) => {
    event.preventDefault();
    dragCounter.current -= 1;
    if (dragCounter.current === 0) {
      setIsDragging(false);
    }
  };

  const handleDrop = (event: React.DragEvent) => {
    event.preventDefault();
    dragCounter.current = 0;
    setIsDragging(false);
    if (!selectedBase || !event.dataTransfer.files || event.dataTransfer.files.length === 0) {
      return;
    }
    notifyKnowledgeFileUploadUnavailable(event.dataTransfer.files[0]?.name);
  };

  const handleUploadFile = (event: React.ChangeEvent<HTMLInputElement>) => {
    const file = event.target.files?.[0];
    if (file) {
      notifyKnowledgeFileUploadUnavailable(file.name);
      event.target.value = '';
    }
  };

  const handleCreateFolder = async (parentId: string | null) => {
    if (!selectedBase) {
      toast('Please select a knowledge base first.', 'error');
      return;
    }
    const folderName = window.prompt('Enter folder name');
    const title = folderName?.trim();
    if (!title) {
      return;
    }

    try {
      const newFolder = await knowledgeService.createDoc({
        baseId: selectedBase.id,
        id: createKnowledgeDocumentResourceId(title),
        parentId: parentId || undefined,
        title,
        type: 'folder',
      });
      setDocs((previousDocs) => [newFolder, ...previousDocs]);
      toast('Folder created.', 'success');
    } catch (error) {
      toast(getErrorMessage(error, 'Failed to create folder.'), 'error');
    }
  };

  const handleCreateKB = async () => {
    const name = newKBData.name.trim();
    if (!name) {
      toast('Enter a knowledge base name.', 'error');
      return;
    }

    try {
      const newBase = await knowledgeService.createBase({
        ...newKBData,
        id: createKnowledgeBaseResourceId(name),
        name,
      });
      setBases((previousBases) => [newBase, ...previousBases]);
      setSelectedBase(newBase);
      setSelectedDoc(null);
      setIsCreateKBModalOpen(false);
      setNewKBData({ description: '', logo: 'KB', name: '', type: 'team' });
      toast('Knowledge base created.', 'success');
    } catch (error) {
      toast(getErrorMessage(error, 'Failed to create knowledge base.'), 'error');
    }
  };

  const handleSaveDoc = async () => {
    if (!selectedBase) {
      toast('Please select a knowledge base first.', 'error');
      return;
    }
    const title = editDocData.title.trim();
    if (!title) {
      toast('Enter a document title.', 'error');
      return;
    }

    const docData: Partial<KnowledgeDoc> = {
      baseId: selectedBase.id,
      content: editDocData.content,
      parentId: selectedDoc?.parentId,
      tags: selectedDoc?.tags ?? [],
      title,
      type: 'markdown',
      version: selectedDoc?.version,
    };

    try {
      if (selectedDoc) {
        const updatedDoc = await knowledgeService.updateDoc(selectedDoc.id, docData);
        setDocs((previousDocs) => previousDocs.map((doc) => (doc.id === updatedDoc.id ? updatedDoc : doc)));
        setSelectedDoc(updatedDoc);
        setIsEditingDoc(false);
        toast('Document saved.', 'success');
        return;
      }

      const newDoc = await knowledgeService.createDoc({
        ...docData,
        id: createKnowledgeDocumentResourceId(title),
      });
      setDocs((previousDocs) => [newDoc, ...previousDocs]);
      setSelectedDoc(newDoc);
      setIsEditingDoc(false);
      toast('Document created.', 'success');
    } catch (error) {
      toast(getErrorMessage(error, 'Failed to save document.'), 'error');
    }
  };

  const getDocAsMessage = (): Message => {
    const doc = selectedDoc;
    if (doc?.type === 'file') {
      return {
        chatId: 'forward',
        content: `[File] ${doc.title}`,
        fileName: doc.fileName,
        fileSize: doc.fileSize,
        fileUrl: doc.fileUrl,
        id: `doc-share-${doc.id}`,
        senderId: 'me',
        timestamp: doc.updatedAt,
        type: 'file',
      };
    }

    return {
      chatId: 'forward',
      content: `[Knowledge] ${doc?.title ?? ''}\n\n${doc?.content.substring(0, 100) || ''}...`,
      id: `doc-share-${doc?.id ?? 'empty'}`,
      senderId: 'me',
      timestamp: doc?.updatedAt ?? 0,
      type: 'text',
    };
  };

  const handleDeleteKB = async (base: KnowledgeBase) => {
    try {
      await knowledgeService.deleteBase(base.id, base.version);
      setBases((previousBases) => previousBases.filter((item) => item.id !== base.id));
      setSelectedBase((currentBase) => (currentBase?.id === base.id ? null : currentBase));
      setSelectedDoc((currentDoc) => (currentDoc?.baseId === base.id ? null : currentDoc));
      toast('Knowledge base deleted.', 'success');
    } catch (error) {
      toast(getErrorMessage(error, 'Failed to delete knowledge base.'), 'error');
    }
  };

  const handleDeleteDoc = async (doc: KnowledgeDoc) => {
    try {
      await knowledgeService.deleteDoc(doc.id, doc.version);
      setDocs((previousDocs) => previousDocs.filter((item) => item.id !== doc.id));
      setSelectedDoc((currentDoc) => (currentDoc?.id === doc.id ? null : currentDoc));
      toast('Document deleted.', 'success');
    } catch (error) {
      toast(getErrorMessage(error, 'Failed to delete document.'), 'error');
    }
  };

  const handleSendChat = async () => {
    if (!selectedBase || !chatInput.trim() || isSendingChat) {
      return;
    }
    const message = chatInput.trim();
    setChatMessages((previousMessages) => [...previousMessages, { role: 'user', text: message }]);
    setChatInput('');
    setIsSendingChat(true);

    try {
      const answer = await knowledgeAiService.runDocumentAction({
        action: 'knowledge_chat_answer',
        content: message,
        context: buildKnowledgeChatContext(),
        instruction: `Answer the user's question using knowledge base "${selectedBase.name}" and cite uncertainty when the supplied context is insufficient.`,
      });
      setChatMessages((previousMessages) => [...previousMessages, { role: 'bot', text: answer }]);
    } catch (error) {
      const errorMessage = getErrorMessage(error, 'Knowledge assistant failed to answer.');
      toast(errorMessage, 'error');
      setChatMessages((previousMessages) => [...previousMessages, { role: 'bot', text: errorMessage }]);
    } finally {
      setIsSendingChat(false);
    }
  };

  const getBaseIcon = (base: KnowledgeBase) => {
    if (base.logo.startsWith('http') || base.logo.startsWith('data:')) {
      return <img src={base.logo} alt={base.name} className="w-5 h-5 rounded-md object-cover" />;
    }
    if (base.logo) {
      return <span className="text-xl flex items-center justify-center">{base.logo}</span>;
    }
    return <BookOpen size={20} className="text-gray-400" />;
  };

  return (
    <div
      className="flex-1 flex bg-white dark:bg-[#1e1e1e] min-w-0 min-h-0 text-gray-900 dark:text-gray-200 relative"
      onDragEnter={handleDragEnter}
      onDragOver={handleDragOver}
      onDragLeave={handleDragLeave}
      onDrop={handleDrop}
    >
      {isDragging && selectedBase && (
        <div className="absolute inset-0 bg-white/60 dark:bg-[#1e1e1e]/60 backdrop-blur-sm z-50 flex items-center justify-center p-8 transition-all pointer-events-none">
          <div className="w-full max-w-2xl h-full max-h-[400px] border-4 border-dashed border-indigo-500/50 bg-white/80 dark:bg-[#2a2a2a]/80 shadow-2xl rounded-3xl flex flex-col items-center justify-center gap-6">
            <div className="w-24 h-24 bg-indigo-50 dark:bg-indigo-500/20 rounded-full flex items-center justify-center text-indigo-500 dark:text-indigo-400">
              <Plus size={48} className="drop-shadow-lg" />
            </div>
            <h3 className="text-3xl font-bold text-gray-900 dark:text-gray-100">Drive upload required</h3>
            <p className="text-lg text-gray-500 dark:text-gray-400">
              Files must be uploaded through Drive before they can be attached to {selectedBase.name}.
            </p>
          </div>
        </div>
      )}

      {isSidebarOpen && (
        <MainSidebar
          bases={bases}
          selectedBase={selectedBase}
          isLoading={isLoadingBases}
          searchQuery={searchQuery}
          setSearchQuery={setSearchQuery}
          onSelectBase={(base) => {
            setSelectedBase(base);
            setSelectedDoc(null);
            setIsEditingDoc(false);
          }}
          onCreateKBClick={(type?: 'team' | 'personal') => {
            setNewKBData((previous) => ({ ...previous, type: type || 'team' }));
            setIsCreateKBModalOpen(true);
          }}
          onCreateNoteClick={() => {
            if (!selectedBase) {
              toast('Please select or create a knowledge base first.', 'error');
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

      <div className="flex-1 flex min-w-0 min-h-0 relative">
        {!selectedBase && bases.length === 0 ? (
          <div className="flex-1 flex flex-col items-center justify-center bg-gray-50 dark:bg-[#1e1e1e]">
            <div className="w-24 h-24 rounded-full bg-indigo-50 dark:bg-indigo-500/10 flex items-center justify-center mb-6">
              <BookOpen size={40} className="text-indigo-500 dark:text-indigo-400" />
            </div>
            <h2 className="text-2xl font-semibold mb-2 text-gray-900 dark:text-gray-100">Knowledge center</h2>
            <p className="text-gray-500 mb-8 max-w-md text-center">Create a knowledge base and persist documents through the app SDK.</p>
            <button
              onClick={() => setIsCreateKBModalOpen(true)}
              className="px-6 py-2.5 bg-indigo-600 hover:bg-indigo-700 dark:hover:bg-indigo-500 text-white rounded-xl transition-colors font-medium flex items-center gap-2 shadow-lg shadow-indigo-500/20"
            >
              <Plus size={18} /> New knowledge base
            </button>
          </div>
        ) : !selectedDoc && !isEditingDoc ? (
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
          <div className="flex-1 flex bg-gray-50 dark:bg-[#1e1e1e] min-w-0 min-h-0 relative">
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

            <DocViewer
              selectedBase={selectedBase!}
              selectedDoc={selectedDoc!}
              onClose={() => setSelectedDoc(null)}
              onForward={() => setIsForwardModalOpen(true)}
              onEdit={() => {
                setEditDocData({ title: selectedDoc!.title, content: selectedDoc!.content });
                setIsEditingDoc(true);
              }}
              onDelete={() => void handleDeleteDoc(selectedDoc!)}
            />

            {!isChatOpen && (
              <button
                onClick={() => setIsChatOpen(true)}
                className="absolute bottom-10 right-10 bg-indigo-600 hover:bg-indigo-700 text-white p-4 rounded-full shadow-2xl transition-transform hover:scale-105 z-40"
                title="Ask knowledge base"
              >
                <MessageSquare size={24} />
              </button>
            )}
          </div>
        )}

        {isChatOpen && selectedBase && (
          <KnowledgeChatPanel
            selectedBaseName={selectedBase.name}
            chatMessages={chatMessages}
            chatInput={chatInput}
            isSending={isSendingChat}
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
