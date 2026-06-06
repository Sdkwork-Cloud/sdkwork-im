import React, { useState, useRef } from 'react';
import { Check, Globe, Lock, X, Upload, Sparkles, Loader2, Image as ImageIcon } from 'lucide-react';
import { motion, AnimatePresence } from 'motion/react';
import { cn } from '@sdkwork/clawchat-pc-commons';
import { toast } from '@sdkwork/clawchat-pc-chat';

interface CreateKBModalProps {
  isOpen: boolean;
  onClose: () => void;
  newKBData: { name: string; description: string; type: 'team' | 'personal'; logo: string };
  setNewKBData: (data: { name: string; description: string; type: 'team' | 'personal'; logo: string }) => void;
  onCreate: () => void;
}

export const CreateKBModal: React.FC<CreateKBModalProps> = ({
  isOpen,
  onClose,
  newKBData,
  setNewKBData,
  onCreate
}) => {
  const [isGeneratingIcon, setIsGeneratingIcon] = useState(false);
  const fileInputRef = useRef<HTMLInputElement>(null);

  const handleUploadIcon = (e: React.ChangeEvent<HTMLInputElement>) => {
    const file = e.target.files?.[0];
    if (!file) return;
    const reader = new FileReader();
    reader.onload = (ev) => {
      const b64 = ev.target?.result as string;
      setNewKBData({...newKBData, logo: b64});
    };
    reader.readAsDataURL(file);
    // Reset file input
    if (fileInputRef.current) {
       fileInputRef.current.value = '';
    }
  };

  const generateAIIcon = async () => {
    if (!newKBData.name) {
       toast('请先输入知识库名称，AI 会根据名称生成更好的图标', 'info');
       return;
    }
    setIsGeneratingIcon(true);
    try {
      const res = await fetch('/api/agent/icon', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ description: newKBData.name + (newKBData.description ? ` (${newKBData.description})` : '') })
      });
      const data = await res.json();
      if (data.result) {
        setNewKBData({...newKBData, logo: data.result});
      } else {
        toast('图标生成失败，请重试', 'error');
      }
    } catch (e) {
      toast('网络错误，无法生成图标', 'error');
    } finally {
      setIsGeneratingIcon(false);
    }
  };

  return (
    <AnimatePresence>
      {isOpen && (
        <div className="fixed inset-0 z-50 flex items-center justify-center p-4">
          <motion.div 
            initial={{ opacity: 0 }} 
            animate={{ opacity: 1 }} 
            exit={{ opacity: 0 }} 
            className="absolute inset-0 bg-black/60 backdrop-blur-sm"
            onClick={onClose}
          />
          <motion.div 
            initial={{ opacity: 0, scale: 0.95, y: 20 }} 
            animate={{ opacity: 1, scale: 1, y: 0 }} 
            exit={{ opacity: 0, scale: 0.95, y: 20 }} 
            className="bg-white dark:bg-[#242424] border border-gray-200 dark:border-white/10 rounded-2xl w-full max-w-md shadow-2xl relative z-10 overflow-hidden"
          >
             <div className="flex items-center justify-between p-5 border-b border-gray-200 dark:border-white/5 bg-gray-50 dark:bg-transparent">
               <h2 className="text-xl font-semibold text-gray-900 dark:text-gray-100">新建知识库</h2>
               <button className="text-gray-500 dark:text-gray-400 hover:text-gray-900 dark:hover:text-white transition-colors" onClick={onClose}>
                 <X size={20} />
               </button>
             </div>
             
             <div className="p-6 space-y-5">
               <div>
                 <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1.5">知识库名称 <span className="text-red-500">*</span></label>
                 <input 
                   className="w-full bg-white dark:bg-[#1a1a1a] border border-gray-200 dark:border-white/10 rounded-xl px-4 py-2.5 text-gray-900 dark:text-gray-200 outline-none focus:border-indigo-500 transition-colors shadow-sm dark:shadow-none"
                   placeholder="例如：前端开发规范、产品资源合集"
                   value={newKBData.name}
                   onChange={e => setNewKBData({...newKBData, name: e.target.value})}
                   autoFocus
                 />
               </div>
               
               <div>
                 <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1.5">描述 (可选)</label>
                 <textarea 
                   className="w-full bg-white dark:bg-[#1a1a1a] border border-gray-200 dark:border-white/10 rounded-xl px-4 py-2.5 text-gray-900 dark:text-gray-200 outline-none focus:border-indigo-500 transition-colors resize-none h-20 shadow-sm dark:shadow-none"
                   placeholder="简要描述知识库的用途和内容..."
                   value={newKBData.description}
                   onChange={e => setNewKBData({...newKBData, description: e.target.value})}
                 />
               </div>

               <div>
                 <label className="flex text-sm font-medium text-gray-700 dark:text-gray-300 mb-1.5 items-center justify-between">
                    <span>图标</span>
                    <div className="flex items-center gap-2">
                      <input 
                        type="file" 
                        accept="image/*" 
                        className="hidden" 
                        ref={fileInputRef}
                        onChange={handleUploadIcon}
                      />
                      <button
                        type="button"
                        onClick={() => fileInputRef.current?.click()}
                        className="text-xs text-gray-500 dark:text-gray-400 hover:text-gray-900 dark:hover:text-white flex items-center gap-1 transition-colors"
                      >
                        <Upload size={12} /> 本地上传
                      </button>
                      <button
                        type="button"
                        onClick={generateAIIcon}
                        disabled={isGeneratingIcon}
                        className="text-xs text-indigo-600 dark:text-indigo-400 hover:text-indigo-700 dark:hover:text-indigo-300 flex items-center gap-1 transition-colors disabled:opacity-50"
                      >
                        {isGeneratingIcon ? <Loader2 size={12} className="animate-spin" /> : <Sparkles size={12} />}
                        AI 生成
                      </button>
                    </div>
                 </label>
                 <div className="flex gap-2 flex-wrap items-center">
                   {(newKBData.logo.startsWith('http') || newKBData.logo.startsWith('data:')) && (
                      <button 
                        type="button"
                        className="w-10 h-10 rounded-xl flex items-center justify-center bg-indigo-50 dark:bg-indigo-500/20 border-indigo-200 dark:border-indigo-500/50 border overflow-hidden shrink-0 relative group shadow-sm dark:shadow-none"
                      >
                        <img src={newKBData.logo} alt="Custom Logo" className="w-full h-full object-cover" />
                      </button>
                   )}
                   {['🚀', '💡', '📚', '🎯', '🛠', '📊', '🤝', '🔥'].map(emoji => (
                     <button 
                       key={emoji}
                       type="button"
                       className={cn(
                         "w-10 h-10 rounded-xl flex items-center justify-center text-xl transition-all shrink-0 shadow-sm dark:shadow-none",
                         newKBData.logo === emoji ? "bg-indigo-50 dark:bg-indigo-500/20 border-indigo-200 dark:border-indigo-500/50 border" : "bg-white dark:bg-white/5 border-gray-200 dark:border-transparent border hover:bg-gray-50 dark:hover:bg-white/10"
                       )}
                       onClick={() => setNewKBData({...newKBData, logo: emoji})}
                     >
                       {emoji}
                     </button>
                   ))}
                 </div>
               </div>

               <div>
                 <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">可见范围</label>
                 <div className="grid grid-cols-2 gap-3">
                   <button
                     className={cn(
                       "flex items-center gap-3 p-3 rounded-xl border text-left transition-all shadow-sm dark:shadow-none bg-white",
                       newKBData.type === 'team' ? "bg-indigo-50 dark:bg-indigo-500/10 border-indigo-200 dark:border-indigo-500/30 text-indigo-700 dark:text-indigo-100" : "dark:bg-white/5 border-gray-200 dark:border-transparent text-gray-600 dark:text-gray-400 hover:bg-gray-50 dark:hover:bg-white/10"
                     )}
                     onClick={() => setNewKBData({...newKBData, type: 'team'})}
                   >
                     <div className={cn("p-2 rounded-lg", newKBData.type === 'team' ? "bg-indigo-100 dark:bg-indigo-500/20 text-indigo-600 dark:text-indigo-400" : "bg-gray-100 dark:bg-white/5")}>
                       <Globe size={20} />
                     </div>
                     <div>
                       <div className="font-medium text-sm">团队公开</div>
                       <div className="text-xs text-gray-500 dark:opacity-70 mt-0.5">所有人可见</div>
                     </div>
                   </button>
                   <button
                     className={cn(
                       "flex items-center gap-3 p-3 rounded-xl border text-left transition-all shadow-sm dark:shadow-none bg-white",
                       newKBData.type === 'personal' ? "bg-indigo-50 dark:bg-indigo-500/10 border-indigo-200 dark:border-indigo-500/30 text-indigo-700 dark:text-indigo-100" : "dark:bg-white/5 border-gray-200 dark:border-transparent text-gray-600 dark:text-gray-400 hover:bg-gray-50 dark:hover:bg-white/10"
                     )}
                     onClick={() => setNewKBData({...newKBData, type: 'personal'})}
                   >
                     <div className={cn("p-2 rounded-lg", newKBData.type === 'personal' ? "bg-indigo-100 dark:bg-indigo-500/20 text-indigo-600 dark:text-indigo-400" : "bg-gray-100 dark:bg-white/5")}>
                       <Lock size={20} />
                     </div>
                     <div>
                       <div className="font-medium text-sm">仅自己可见</div>
                       <div className="text-xs text-gray-500 dark:opacity-70 mt-0.5">私密知识库</div>
                     </div>
                   </button>
                 </div>
               </div>
             </div>

             <div className="p-5 border-t border-gray-200 dark:border-white/5 flex items-center justify-end gap-3 bg-gray-50 dark:bg-[#202020]">
               <button 
                 className="px-5 py-2 text-sm font-medium text-gray-600 dark:text-gray-300 hover:text-gray-900 dark:hover:text-white transition-colors"
                 onClick={onClose}
               >
                 取消
               </button>
               <button 
                 className="px-6 py-2 bg-indigo-600 hover:bg-indigo-500 text-white text-sm font-medium rounded-xl transition-colors shadow-lg shadow-indigo-500/20 flex items-center gap-2"
                 onClick={onCreate}
               >
                 <Check size={16} /> 创建
               </button>
             </div>
          </motion.div>
        </div>
      )}
    </AnimatePresence>
  );
};
