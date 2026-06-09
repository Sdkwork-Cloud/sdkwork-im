import React, { useState, useRef, useEffect } from 'react';
import { useEditor, EditorContent } from '@tiptap/react';
import StarterKit from '@tiptap/starter-kit';
import Link from '@tiptap/extension-link';
import Image from '@tiptap/extension-image';
import Placeholder from '@tiptap/extension-placeholder';
import { Markdown } from 'tiptap-markdown';
import { KnowledgeBase, KnowledgeDoc } from '../services/KnowledgeService';
import { cn } from '@sdkwork/clawchat-pc-commons';
import { DocEditorHeader } from './DocEditorHeader';
import { DocEditorFormattingToolbar } from './DocEditorFormattingToolbar';
import { DocEditorMenus } from './DocEditorMenus';
import { DocEditorContextMenu } from './DocEditorContextMenu';
import { DocEditorPreview } from './DocEditorPreview';
import { DocEditorAiSidebar } from './DocEditorAiSidebar';
import { toast } from '@sdkwork/clawchat-pc-chat';
import { knowledgeAiService } from '../services/KnowledgeAiService';

interface DocEditorProps {
  selectedBase: KnowledgeBase;
  selectedDoc: KnowledgeDoc | null;
  editDocData: { title: string; content: string };
  setEditDocData: (data: { title: string; content: string }) => void;
  onSave: () => void;
  onCancel: () => void;
}

export const DocEditor: React.FC<DocEditorProps> = ({
  selectedBase,
  selectedDoc,
  editDocData,
  setEditDocData,
  onSave,
  onCancel
}) => {
  const [activeTab, setActiveTab] = useState<'edit' | 'preview' | 'split'>('split');
  const [isAiPanelOpen, setIsAiPanelOpen] = useState(false);
  const [aiLoading, setAiLoading] = useState(false);
  const [aiInstruction, setAiInstruction] = useState('');
  const [selectionRange, setSelectionRange] = useState<{ start: number, end: number } | null>(null);
  const [contextMenu, setContextMenu] = useState<{ x: number, y: number } | null>(null);
  const [contextMenuInstruction, setContextMenuInstruction] = useState('');
  const [bubbleAiOpen, setBubbleAiOpen] = useState(false);

  const isUpdating = useRef(false);

  useEffect(() => {
    const handleClickOutside = (e: MouseEvent) => {
      setContextMenu(null);
    };
    document.addEventListener('mousedown', handleClickOutside);
    return () => document.removeEventListener('mousedown', handleClickOutside);
  }, []);

  const editor = useEditor({
    extensions: [
      StarterKit,
      Link.configure({ openOnClick: false }),
      Image,
      Placeholder.configure({
        placeholder: ({ node }) => {
          if (node.type.name === 'heading') {
            return `标题 ${node.attrs.level}`
          }
          return '输入文本或使用左侧按钮插入块...'
        },
        emptyEditorClass: 'is-editor-empty',
      }),
      Markdown.configure({
        html: true,
      }),
    ],
    content: editDocData.content,
    editorProps: {
      attributes: {
        class: 'prose prose-invert prose-indigo max-w-none focus:outline-none w-full h-full text-[15px] leading-relaxed',
      },
    },
    onUpdate: ({ editor }) => {
      isUpdating.current = true;
      const markdown = (editor.storage as any).markdown.getMarkdown();
      setEditDocData({ ...editDocData, content: markdown });

      const { empty, from, to } = editor.state.selection;
      if (!empty) {
        setSelectionRange({ start: from, end: to });
      } else {
        setSelectionRange(null);
      }

      queueMicrotask(() => {
        isUpdating.current = false;
      });
    },
    onSelectionUpdate: ({ editor }) => {
      const { empty, from, to } = editor.state.selection;
      if (!empty) {
        setSelectionRange({ start: from, end: to });
      } else {
        setSelectionRange(null);
      }
    }
  });

  useEffect(() => {
    if (editor && editDocData.content !== (editor.storage as any).markdown.getMarkdown() && !isUpdating.current) {
      editor.commands.setContent(editDocData.content);
    }
  }, [editDocData.content, editor]);

  const executeAiAction = async (action: string, customInstruction?: string) => {
    let contentToProcess = '';

    if (selectionRange && editor) {
        const { from, to } = editor.state.selection;
        contentToProcess = editor.state.doc.textBetween(from, to, ' ');
    } else if (!customInstruction) {
        return;
    }

    setAiLoading(true);

    const plainText = editor ? editor.getText() : editDocData.content;
    const cursorPos = editor ? editor.state.selection.from : 0;

    const contextStart = Math.max(0, cursorPos - 500);
    const contextEnd = Math.min(plainText.length, cursorPos + 500);
    const contextContent = plainText.substring(contextStart, contextEnd);

    try {
        const result = await knowledgeAiService.runDocumentAction({
            action,
            content: contentToProcess,
            context: contextContent,
            instruction: customInstruction
        });
        if (editor) {
           editor.commands.focus();
           editor.commands.insertContent(result);
        }

        setSelectionRange(null);
        setAiInstruction('');
    } catch (e) {
        const message = e instanceof Error && e.message ? e.message : 'AI action failed';
        toast(message, 'error');
    } finally {
        setAiLoading(false);
    }
  };

  return (
    <>
      <style>{`
        .prose p.is-editor-empty:first-child::before {
          content: attr(data-placeholder);
          float: left;
          color: #9CA3AF;
          pointer-events: none;
          height: 0;
        }
        .prose h1.is-editor-empty::before,
        .prose h2.is-editor-empty::before,
        .prose h3.is-editor-empty::before {
          content: attr(data-placeholder);
          float: left;
          color: #9CA3AF;
          pointer-events: none;
          height: 0;
        }
        @media (prefers-color-scheme: dark) {
          .prose p.is-editor-empty:first-child::before,
          .prose h1.is-editor-empty::before,
          .prose h2.is-editor-empty::before,
          .prose h3.is-editor-empty::before {
            color: #52525B;
          }
        }
      `}</style>
      <div className="flex-1 flex flex-col bg-white dark:bg-[#1e1e1e] min-w-0 min-h-0">
        <DocEditorHeader
          selectedBase={selectedBase}
          selectedDoc={selectedDoc}
          activeTab={activeTab}
          setActiveTab={setActiveTab}
          isAiPanelOpen={isAiPanelOpen}
          setIsAiPanelOpen={setIsAiPanelOpen}
          editDocTitle={editDocData.title}
          onSave={onSave}
          onCancel={onCancel}
        />

        <div className="flex-1 flex min-h-0 bg-gray-50 dark:bg-[#141414]">
          {/* Editor Pane */}
          {(activeTab === 'edit' || activeTab === 'split') && (
            <div className={cn("flex flex-col min-h-0", activeTab === 'split' ? "w-1/2 border-r border-gray-200 dark:border-white/5" : "w-full")}>
               <div className="flex flex-col h-full bg-white dark:bg-[#1e1e1e]">
                 <div className="px-6 md:px-10 py-6 shrink-0">
                   <input
                     className="w-full text-3xl md:text-4xl lg:text-5xl font-bold bg-transparent border-none outline-none text-gray-900 dark:text-gray-100 placeholder-gray-300 dark:placeholder:text-gray-600 transition-colors focus:placeholder-gray-400 dark:focus:placeholder:text-gray-500"
                     placeholder="输入文档标题..."
                     value={editDocData.title}
                     onChange={e => setEditDocData({...editDocData, title: e.target.value})}
                     autoFocus
                   />
                 </div>

                 <DocEditorFormattingToolbar editor={editor} />

                 <div
                   className="flex-1 overflow-auto custom-scrollbar relative p-6 md:p-10"
                   onContextMenu={(e) => {
                     e.preventDefault();
                     setContextMenu({ x: e.clientX, y: e.clientY });
                     setContextMenuInstruction('');
                   }}
                 >
                     <DocEditorMenus
                        editor={editor}
                        setContextMenu={setContextMenu}
                        aiLoading={aiLoading}
                        bubbleAiOpen={bubbleAiOpen}
                        setBubbleAiOpen={setBubbleAiOpen}
                        executeAiAction={executeAiAction}
                     />

                     {editor && <EditorContent editor={editor} className="w-full h-full pb-32" />}

                     <DocEditorContextMenu
                        contextMenu={contextMenu}
                        setContextMenu={setContextMenu}
                        contextMenuInstruction={contextMenuInstruction}
                        setContextMenuInstruction={setContextMenuInstruction}
                        executeAiAction={executeAiAction}
                     />
                 </div>
               </div>
            </div>
          )}

          <DocEditorPreview
            activeTab={activeTab}
            title={editDocData.title}
            content={editDocData.content}
          />

          <DocEditorAiSidebar
            isAiPanelOpen={isAiPanelOpen}
            aiInstruction={aiInstruction}
            setAiInstruction={setAiInstruction}
            aiLoading={aiLoading}
            executeAiAction={executeAiAction}
          />
        </div>
      </div>
    </>
  );
};

