import React, { useRef, useState, useEffect, useCallback } from 'react';
import SignaturePadLibrary from 'signature_pad';
import { SignaturePadHeader } from './components/SignaturePadHeader';
import { SignaturePadToolbar } from './components/SignaturePadToolbar';
import { SignaturePadMobileQR } from './components/SignaturePadMobileQR';

interface SignaturePadProps {
  onSave: (imgUrl: string) => void;
  onCancel: () => void;
  title: string;
  subtitle?: React.ReactNode;
  mobileSignatureUrl?: string;
}

const PEN_COLORS = [
  { label: '黑色', value: '#000000' },
  { label: '蓝色', value: '#0056b3' },
  { label: '红色', value: '#d32f2f' }
];

const ASPECT_RATIOS = [
  { label: '2:1', value: 'aspect-[2/1]' },
  { label: '16:9', value: 'aspect-video' },
  { label: '4:3', value: 'aspect-[4/3]' },
  { label: '1:1', value: 'aspect-square' },
];

export const SignaturePad: React.FC<SignaturePadProps> = ({ onSave, onCancel, title, subtitle, mobileSignatureUrl }) => {
  const containerRef = useRef<HTMLDivElement>(null);
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const signaturePadRef = useRef<SignaturePadLibrary | null>(null);
  const [hasDrawn, setHasDrawn] = useState(false);
  const [penColor, setPenColor] = useState(PEN_COLORS[0].value);
  const [aspectRatio, setAspectRatio] = useState(ASPECT_RATIOS[0].value);
  const [openDropdown, setOpenDropdown] = useState<'ratio' | 'color' | null>(null);

  useEffect(() => {
    const handleClickOutside = () => setOpenDropdown(null);
    if (openDropdown) {
      document.addEventListener('click', handleClickOutside);
    }
    return () => document.removeEventListener('click', handleClickOutside);
  }, [openDropdown]);

  const resizeCanvas = useCallback(() => {
    const canvas = canvasRef.current;
    if (canvas) {
      const ratio = Math.max(window.devicePixelRatio || 1, 1);
      // Save data if there's any
      const data = signaturePadRef.current?.toData();
      
      canvas.width = canvas.offsetWidth * ratio;
      canvas.height = canvas.offsetHeight * ratio;
      canvas.getContext("2d")?.scale(ratio, ratio);
      
      signaturePadRef.current?.clear();
      if (data && data.length > 0) {
        signaturePadRef.current?.fromData(data);
      } else {
        setHasDrawn(false);
      }
    }
  }, []);

  useEffect(() => {
    if (canvasRef.current && !signaturePadRef.current) {
      signaturePadRef.current = new SignaturePadLibrary(canvasRef.current, {
        penColor: penColor,
        backgroundColor: '#ffffff',
        minWidth: 1.5,
        maxWidth: 4,
        velocityFilterWeight: 0.7,
      });

      signaturePadRef.current.addEventListener("beginStroke", () => {
        setHasDrawn(true);
      });

      setTimeout(resizeCanvas, 50);
      window.addEventListener("resize", resizeCanvas);
    }
    
    return () => {
      window.removeEventListener("resize", resizeCanvas);
    };
  }, [resizeCanvas]);

  useEffect(() => {
    if (signaturePadRef.current) {
      signaturePadRef.current.penColor = penColor;
    }
  }, [penColor]);

  useEffect(() => {
    // Re-initialize or resize the canvas when aspect ratio changes
    setTimeout(resizeCanvas, 50);
  }, [aspectRatio, resizeCanvas]);

  const clearCanvas = () => {
    if (signaturePadRef.current) {
      signaturePadRef.current.clear();
      setHasDrawn(false);
    }
  };

  const handleSave = () => {
    if (!hasDrawn || !signaturePadRef.current) return;
    if (signaturePadRef.current.isEmpty()) return;
    onSave(signaturePadRef.current.toDataURL('image/png'));
  };

  return (
    <div className="flex w-full h-full flex-col bg-[#1e1e1e] relative min-h-0 min-w-0">
        <SignaturePadHeader 
          title={title}
          onCancel={onCancel}
          onSave={handleSave}
          hasDrawn={hasDrawn}
        />

        <div className="flex-1 overflow-y-auto p-8 flex items-center justify-center gap-8 flex-col lg:flex-row">
            <div className="flex flex-col items-center max-w-3xl w-full">
              <div 
                ref={containerRef}
                className={`w-full ${aspectRatio} bg-white rounded-2xl shadow-[0_8px_30px_rgb(0,0,0,0.12)] relative overflow-hidden group cursor-crosshair border border-gray-200 ring-4 ring-white/5 transition-all duration-300 ${hasDrawn ? 'border-indigo-500/50 ring-indigo-500/20' : ''}`}
              >
                {!hasDrawn && (
                   <div className="absolute inset-0 flex flex-col items-center justify-center pointer-events-none select-none opacity-40">
                     <div className="w-16 h-16 border-2 border-dashed border-gray-400 rounded-full flex items-center justify-center mb-4">
                        <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round" className="text-gray-400"><path d="M21.174 6.812a1 1 0 0 0-3.986-3.987L3.842 16.174a2 2 0 0 0-.5.83l-1.321 4.352a.5.5 0 0 0 .623.622l4.353-1.32a2 2 0 0 0 .83-.497z"/><path d="m15 5 4 4"/></svg>
                     </div>
                     <div className="text-gray-500 font-medium text-xl md:text-2xl tracking-widest text-center px-4">请平滑书写，签署您的全名</div>
                   </div>
                )}
                <canvas
                  ref={canvasRef}
                  className="w-full h-full touch-none relative z-10 mix-blend-multiply"
                  style={{ touchAction: 'none' }}
                />
              </div>
              <SignaturePadToolbar 
                subtitle={subtitle}
                aspectRatio={aspectRatio}
                setAspectRatio={setAspectRatio}
                penColor={penColor}
                setPenColor={setPenColor}
                clearCanvas={clearCanvas}
                ratios={ASPECT_RATIOS}
                colors={PEN_COLORS}
                openDropdown={openDropdown}
                setOpenDropdown={setOpenDropdown}
                onSave={handleSave}
                hasDrawn={hasDrawn}
              />
            </div>

            <SignaturePadMobileQR signatureUrl={mobileSignatureUrl} />
        </div>
    </div>
  );
};

