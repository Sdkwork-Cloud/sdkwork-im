import React from "react";
import { motion, AnimatePresence } from "motion/react";
import { X } from "lucide-react";

interface FullscreenMediaOverlayProps {
  media: {
    type: "image" | "video";
    url: string;
  } | null;
  onClose: () => void;
}

export const FullscreenMediaOverlay: React.FC<FullscreenMediaOverlayProps> = ({
  media,
  onClose,
}) => {
  return (
    <AnimatePresence>
      {media && (
        <motion.div
          initial={{ opacity: 0 }}
          animate={{ opacity: 1 }}
          exit={{ opacity: 0 }}
          className="fixed inset-0 z-[200] bg-black flex flex-col cursor-pointer"
          onClick={onClose}
        >
          <div className="h-14 flex items-center justify-end px-4 pt-safe safe-area-top absolute top-0 inset-x-0 z-10 bg-gradient-to-b from-black/50 to-transparent pointer-events-none">
            <button
              className="text-white p-2 pointer-events-auto"
              onClick={(e) => {
                e.stopPropagation();
                onClose();
              }}
            >
              <X className="w-8 h-8 drop-shadow-md" />
            </button>
          </div>
          <div
            className="flex-1 flex items-center justify-center w-full h-full absolute inset-0 cursor-default bg-black"
            onClick={(e) => e.stopPropagation()}
          >
            {media.type === "image" && (
              <img
                src={media.url}
                alt="Fullscreen Preview"
                className="w-full h-full object-contain select-none"
              />
            )}
            {media.type === "video" && (
              <video
                src={media.url}
                controls
                autoPlay
                playsInline
                className="w-full h-full object-contain"
              />
            )}
          </div>
        </motion.div>
      )}
    </AnimatePresence>
  );
};
