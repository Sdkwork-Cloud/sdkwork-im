import React from 'react';
import { X } from 'lucide-react';

interface SignaturePadHeaderProps {
  title: string;
  onCancel: () => void;
  onSave: () => void;
  hasDrawn: boolean;
}

export const SignaturePadHeader: React.FC<SignaturePadHeaderProps> = ({ title, onCancel, onSave, hasDrawn }) => {
  return (
    <div className="flex items-center justify-between px-6 min-h-[64px] border-b border-white/5 bg-[#181818] shrink-0">
      <button onClick={onCancel} className="flex items-center text-gray-400 hover:text-gray-100 transition-colors py-2 px-3 hover:bg-white/5 rounded-lg -ml-3">
        <X size={20} className="mr-1.5" />
        <span className="font-medium text-sm text-gray-300">取消签署</span>
      </button>
      <div className="text-sm font-semibold tracking-wide text-gray-200">
        <span className="opacity-50 mr-2 border border-white/10 px-2 py-0.5 rounded text-xs uppercase">SECURE</span>
        {title}
      </div>
      <div className="w-[100px]" /> {/* Spacer for balance */}
    </div>
  );
};
