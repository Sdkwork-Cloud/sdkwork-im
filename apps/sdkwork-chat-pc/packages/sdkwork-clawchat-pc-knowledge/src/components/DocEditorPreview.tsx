import React from 'react';
import ReactMarkdown from 'react-markdown';
import { cn } from '@sdkwork/clawchat-pc-commons';

interface DocEditorPreviewProps {
  activeTab: 'edit' | 'preview' | 'split';
  title: string;
  content: string;
}

export const DocEditorPreview: React.FC<DocEditorPreviewProps> = ({ activeTab, title, content }) => {
  if (activeTab !== 'preview' && activeTab !== 'split') return null;

  return (
    <div className={cn("flex flex-col min-h-0 bg-white dark:bg-[#141414]", activeTab === 'split' ? "flex-1" : "w-full max-w-[1000px] mx-auto")}>
       <div className="flex-1 overflow-y-auto custom-scrollbar p-8 md:p-12 lg:p-16">
          {title && (
            <h1 className="text-3xl md:text-4xl lg:text-5xl font-bold text-gray-100 mb-8 pb-6 border-b border-white/5">
              {title}
            </h1>
          )}
          <div className="prose dark:prose-invert prose-indigo max-w-none prose-p:text-gray-700 dark:prose-p:text-gray-300 prose-headings:text-gray-900 dark:prose-headings:text-gray-100 prose-a:text-indigo-600 dark:prose-a:text-indigo-400 prose-strong:text-gray-900 dark:prose-strong:text-gray-200 prose-code:text-indigo-600 dark:prose-code:text-indigo-300 prose-code:bg-indigo-50 dark:prose-code:bg-indigo-500/10 prose-code:px-1.5 prose-code:py-0.5 prose-code:rounded prose-pre:bg-gray-50 dark:prose-pre:bg-[#181818] prose-pre:border prose-pre:border-gray-200 dark:prose-pre:border-white/5 prose-img:rounded-2xl prose-img:max-w-full prose-img:mx-auto prose-img:shadow-lg dark:prose-img:shadow-xl">
            {content ? (
              <ReactMarkdown>{content}</ReactMarkdown>
            ) : (
              <div className="text-gray-600 italic">预览区域。左侧输入的内容将在这里实时渲染...</div>
            )}
          </div>
       </div>
    </div>
  );
};
