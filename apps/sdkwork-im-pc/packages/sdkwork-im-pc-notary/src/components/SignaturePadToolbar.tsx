import React from 'react';
import { useTranslation } from 'react-i18next';
import { RotateCcw, ChevronDown, Palette, Maximize, CheckCircle2, Check } from 'lucide-react';

interface SignaturePadToolbarProps {
  subtitle?: React.ReactNode;
  aspectRatio: string;
  setAspectRatio: (ratio: string) => void;
  penColor: string;
  setPenColor: (color: string) => void;
  clearCanvas: () => void;
  ratios: { label: string; value: string }[];
  colors: { label: string; value: string }[];
  openDropdown: 'ratio' | 'color' | null;
  setOpenDropdown: (dropdown: 'ratio' | 'color' | null) => void;
  onSave: () => void;
  hasDrawn: boolean;
}

export const SignaturePadToolbar: React.FC<SignaturePadToolbarProps> = ({
  subtitle,
  aspectRatio,
  setAspectRatio,
  penColor,
  setPenColor,
  clearCanvas,
  ratios,
  colors,
  openDropdown,
  setOpenDropdown,
  onSave,
  hasDrawn,
}) => {
  const { t } = useTranslation('notary');

  return (
    <div className="mt-8 flex flex-col items-center gap-6 w-full relative z-10">
      <div className="flex items-center justify-between w-full">
        <div className="text-gray-400 text-sm flex items-center gap-2">
          {subtitle}
        </div>
        <div className="flex items-center gap-3">
          <div className="relative" onClick={(event) => event.stopPropagation()}>
            <button
              type="button"
              onClick={() => setOpenDropdown(openDropdown === 'ratio' ? null : 'ratio')}
              className="flex items-center gap-2 px-3 py-2 bg-[#2a2a2d] border border-white/5 shadow-sm rounded-lg hover:bg-white/10 text-gray-300 text-xs transition-colors focus:outline-none focus:ring-1 focus:ring-indigo-500/50"
            >
              <Maximize className="w-3.5 h-3.5" />
              <span>{ratios.find((ratio) => ratio.value === aspectRatio)?.label}</span>
              <ChevronDown className="w-3 h-3 text-gray-500" />
            </button>
            {openDropdown === 'ratio' && (
              <div className="absolute bottom-full left-0 mb-2 w-28 bg-[#2a2a2d] border border-white/10 rounded-lg shadow-xl overflow-hidden py-1 z-20">
                {ratios.map((ratio) => (
                  <button
                    key={ratio.value}
                    type="button"
                    onClick={() => {
                      setAspectRatio(ratio.value);
                      setOpenDropdown(null);
                    }}
                    className={`w-full text-left px-3 py-2 text-xs hover:bg-white/5 transition-colors ${aspectRatio === ratio.value ? 'text-indigo-400 bg-indigo-500/10' : 'text-gray-300'}`}
                  >
                    {ratio.label}
                  </button>
                ))}
              </div>
            )}
          </div>

          <div className="relative" onClick={(event) => event.stopPropagation()}>
            <button
              type="button"
              onClick={() => setOpenDropdown(openDropdown === 'color' ? null : 'color')}
              className="flex items-center gap-2 px-3 py-2 bg-[#2a2a2d] border border-white/5 shadow-sm rounded-lg hover:bg-white/10 text-gray-300 text-xs transition-colors focus:outline-none focus:ring-1 focus:ring-indigo-500/50"
            >
              <Palette className="w-3.5 h-3.5" />
              <div className="flex items-center gap-1.5">
                <div className="w-3.5 h-3.5 rounded-full border border-gray-600 shadow-sm" style={{ backgroundColor: penColor }} />
              </div>
              <ChevronDown className="w-3 h-3 text-gray-500" />
            </button>
            {openDropdown === 'color' && (
              <div className="absolute bottom-full left-0 mb-2 w-32 bg-[#2a2a2d] border border-white/10 rounded-lg shadow-xl overflow-hidden py-1 z-20">
                {colors.map((color) => (
                  <button
                    key={color.value}
                    type="button"
                    onClick={() => {
                      setPenColor(color.value);
                      setOpenDropdown(null);
                    }}
                    className={`w-full flex items-center justify-between px-3 py-2 text-xs hover:bg-white/5 transition-colors ${penColor === color.value ? 'text-indigo-400 bg-indigo-500/10' : 'text-gray-300'}`}
                  >
                    <div className="flex items-center gap-2">
                      <div className="w-3.5 h-3.5 rounded-full border border-gray-500 shadow-sm" style={{ backgroundColor: color.value }} />
                      <span>{color.label}</span>
                    </div>
                    {penColor === color.value && <CheckCircle2 className="w-3.5 h-3.5 text-indigo-400" />}
                  </button>
                ))}
              </div>
            )}
          </div>

          <div className="w-px h-5 bg-white/10 mx-1" />

          <button
            type="button"
            onClick={clearCanvas}
            className="flex items-center gap-1.5 text-xs font-medium text-gray-400 hover:text-gray-200 transition-colors bg-[#2a2a2d] hover:bg-white/10 px-3 py-2 rounded-lg border border-white/5 shadow-sm focus:outline-none focus:ring-1 focus:ring-indigo-500/50"
          >
            <RotateCcw size={14} />
            <span>{t('signaturePad.clearRewrite')}</span>
          </button>
        </div>
      </div>

      <button
        type="button"
        onClick={onSave}
        disabled={!hasDrawn}
        className={`w-full sm:w-auto min-w-[240px] flex items-center justify-center gap-2 px-8 py-3.5 rounded-xl font-medium text-base shadow-xl transition-all duration-300 ${
          hasDrawn
            ? 'bg-gradient-to-r from-indigo-500 to-purple-600 hover:from-indigo-400 hover:to-purple-500 text-white shadow-indigo-500/20 hover:shadow-indigo-500/40 hover:-translate-y-0.5'
            : 'bg-[#2a2a2d] text-gray-500 cursor-not-allowed border border-white/5 shadow-none'
        }`}
      >
        <Check size={18} />
        {hasDrawn ? t('signaturePad.confirmSubmit') : t('signaturePad.signHereHint')}
      </button>
    </div>
  );
};
