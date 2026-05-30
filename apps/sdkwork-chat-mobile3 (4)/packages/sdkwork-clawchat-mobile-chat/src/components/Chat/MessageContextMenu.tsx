import React from "react";
import { motion, AnimatePresence } from "motion/react";
import { Copy, Reply, Forward, Star, Trash2 } from "lucide-react";
import { showToast, cn } from "@sdkwork/clawchat-mobile-commons";
import type { Message } from "@sdkwork/clawchat-mobile-types";

interface MessageContextMenuProps {
  contextMenu: {
    isOpen: boolean;
    x: number;
    y: number;
    messageId: string | null;
  };
  messages: Message[];
  onClose: () => void;
  onCopy: (msgId: string) => void;
  onReply: (msg: Message) => void;
  onStar: (msgId: string) => void;
  onDelete: (msgId: string) => void;
}

export const MessageContextMenu: React.FC<MessageContextMenuProps> = ({
  contextMenu,
  messages,
  onClose,
  onCopy,
  onReply,
  onStar,
  onDelete,
}) => {
  return (
    <AnimatePresence>
      {contextMenu.isOpen && (
        <>
          <motion.div
            initial={{ opacity: 0 }}
            animate={{ opacity: 1 }}
            exit={{ opacity: 0 }}
            transition={{ duration: 0.15 }}
            className="fixed inset-0 z-40 bg-black/20 dark:bg-black/40 backdrop-blur-sm"
            onClick={onClose}
            onTouchStart={onClose}
          />
          <motion.div
            initial={{ opacity: 0, scale: 0.9, y: 10 }}
            animate={{ opacity: 1, scale: 1, y: 0 }}
            exit={{ opacity: 0, scale: 0.9, y: 10 }}
            transition={{ type: "spring", stiffness: 400, damping: 25 }}
            style={{ left: contextMenu.x, top: contextMenu.y }}
            className="fixed z-50 w-40 bg-white/95 dark:bg-[#2C2C2C]/95 backdrop-blur-xl rounded-2xl shadow-2xl border border-black/5 dark:border-white/10 overflow-hidden"
          >
            <div className="flex flex-col">
              {messages.find((m) => m.id === contextMenu.messageId)?.type ===
                "text" && (
                <>
                  <div
                    className="flex items-center justify-between px-4 py-3.5 text-[15px] text-text-main active:bg-black/5 dark:active:bg-white/5 transition-colors cursor-pointer"
                    onClick={(e) => {
                      e.stopPropagation();
                      if (contextMenu.messageId) onCopy(contextMenu.messageId);
                    }}
                  >
                    <span>复制</span>
                    <Copy className="w-4 h-4 opacity-70" />
                  </div>
                  <div className="h-[1px] bg-black/5 dark:bg-white/5 mx-4" />
                </>
              )}
              <div
                className="flex items-center justify-between px-4 py-3.5 text-[15px] text-text-main active:bg-black/5 dark:active:bg-white/5 transition-colors cursor-pointer"
                onClick={(e) => {
                  e.stopPropagation();
                  const msg = messages.find(
                    (m) => m.id === contextMenu.messageId,
                  );
                  if (msg) onReply(msg);
                  onClose();
                }}
              >
                <span>回复</span>
                <Reply className="w-4 h-4 opacity-70" />
              </div>
              <div className="h-[1px] bg-black/5 dark:bg-white/5 mx-4" />
              <div
                className="flex items-center justify-between px-4 py-3.5 text-[15px] text-text-main active:bg-black/5 dark:active:bg-white/5 transition-colors cursor-pointer"
                onClick={(e) => {
                  e.stopPropagation();
                  showToast("由于演示版本限制，暂不支持转发");
                  onClose();
                }}
              >
                <span>转发</span>
                <Forward className="w-4 h-4 opacity-70" />
              </div>
              <div className="h-[1px] bg-black/5 dark:bg-white/5 mx-4" />
              <div
                className="flex items-center justify-between px-4 py-3.5 text-[15px] text-text-main active:bg-black/5 dark:active:bg-white/5 transition-colors cursor-pointer"
                onClick={(e) => {
                  e.stopPropagation();
                  if (contextMenu.messageId) onStar(contextMenu.messageId);
                }}
              >
                <span>
                  {messages.find((m) => m.id === contextMenu.messageId)
                    ?.isStarred
                    ? "取消收藏"
                    : "收藏"}
                </span>
                <Star
                  className={cn(
                    "w-4 h-4 opacity-70",
                    messages.find((m) => m.id === contextMenu.messageId)
                      ?.isStarred && "fill-current text-yellow-500 opacity-100",
                  )}
                />
              </div>
              <div className="h-[1px] bg-black/5 dark:bg-white/5 mx-4" />
              <div
                className="flex items-center justify-between px-4 py-3.5 text-[15px] text-accent-red active:bg-red-50 dark:active:bg-red-950/30 transition-colors cursor-pointer"
                onClick={(e) => {
                  e.stopPropagation();
                  if (contextMenu.messageId) onDelete(contextMenu.messageId);
                }}
              >
                <span>删除</span>
                <Trash2 className="w-4 h-4" />
              </div>
            </div>
          </motion.div>
        </>
      )}
    </AnimatePresence>
  );
};
