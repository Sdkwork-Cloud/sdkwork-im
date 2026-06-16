/**
 * FaceCaptureOverlay - Full-screen camera overlay for face capture
 *
 * Displays a live camera feed with face alignment guide and capture button.
 * Uses refs from useCameraCapture hook.
 */
import React from 'react';
import { useTranslation } from 'react-i18next';
import { motion, AnimatePresence } from 'motion/react';
import { X } from 'lucide-react';

export interface FaceCaptureOverlayProps {
  /** Whether the overlay is open */
  isOpen: boolean;
  /** Ref for the video element */
  videoRef: React.RefObject<HTMLVideoElement | null>;
  /** Ref for the off-screen canvas */
  canvasRef: React.RefObject<HTMLCanvasElement | null>;
  /** Called when the capture button is clicked */
  onCapture: () => void;
  /** Called when the close button is clicked (should stop camera) */
  onClose: () => void;
}

export const FaceCaptureOverlay: React.FC<FaceCaptureOverlayProps> = ({
  isOpen,
  videoRef,
  canvasRef,
  onCapture,
  onClose,
}) => {
  const { t } = useTranslation('notary');

  return (
    <AnimatePresence>
      {isOpen && (
        <motion.div
          initial={{ opacity: 0 }}
          animate={{ opacity: 1 }}
          exit={{ opacity: 0 }}
          className="fixed inset-0 z-[220] bg-black/95 flex flex-col"
        >
          {/* Header */}
          <div className="flex justify-between items-center p-4">
            <h3 className="text-white font-medium">{t('party.liveFaceCapture')}</h3>
            <button onClick={onClose} className="text-gray-400 hover:text-white">
              <X size={24} />
            </button>
          </div>

          {/* Camera feed */}
          <div className="flex-1 flex justify-center items-center overflow-hidden relative">
            <video
              ref={videoRef}
              autoPlay
              playsInline
              muted
              className="h-full w-full object-cover max-w-4xl mx-auto rounded-lg shadow-2xl scale-x-[-1]"
            />
            <canvas ref={canvasRef} className="hidden" />

            {/* Face alignment guide */}
            <div className="absolute inset-0 pointer-events-none flex justify-center items-center">
              <div className="w-[300px] h-[400px] border-2 border-dashed border-indigo-500/50 rounded-full flex items-center justify-center shadow-[0_0_0_9999px_rgba(0,0,0,0.5)]">
                <div className="absolute bottom-4 left-0 right-0 text-center text-sm text-white/70">
                  {t('party.alignFaceHint')}
                </div>
              </div>
            </div>
          </div>

          {/* Capture button */}
          <div className="p-8 flex justify-center">
            <button
              onClick={onCapture}
              className="w-16 h-16 rounded-full border-4 border-white bg-indigo-500 hover:bg-indigo-600 transition-colors shadow-lg active:scale-95"
            />
          </div>
        </motion.div>
      )}
    </AnimatePresence>
  );
};