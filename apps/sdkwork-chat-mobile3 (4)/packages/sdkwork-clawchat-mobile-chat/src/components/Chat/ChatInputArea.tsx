import React, { useRef, useState, useEffect } from "react";
import { motion, AnimatePresence } from "motion/react";
import { EditorContent, Editor } from "@tiptap/react";
import {
  Keyboard,
  Mic,
  Smile,
  PlusCircle,
  Send,
  X,
  ImageIcon,
  Video,
  Music,
  Folder,
  Phone,
  Link as LinkIcon,
  ShoppingBag,
} from "lucide-react";
import { IconButton, cn } from "@sdkwork/clawchat-mobile-commons";
import type { Message, User, Chat } from "@sdkwork/clawchat-mobile-types";

interface ChatInputAreaProps {
  id?: string;
  currentUser: User | null;
  chat: Chat | null;
  replyingTo: Message | null;
  setReplyingTo: (msg: Message | null) => void;
  editor: Editor | null;
  inputValue: string;
  isVoiceMode: boolean;
  setIsVoiceMode: (mode: boolean) => void;
  activePanel: "none" | "emoji" | "action";
  setActivePanel: (panel: "none" | "emoji" | "action") => void;
  isRecording: boolean;
  startRecording: () => void;
  handleSendVoice: () => void;
  cancelRecording: () => void;
  handleSend: () => void;
  handleSendCustom: (
    type: Message["type"],
    content: string,
    metadata?: Record<string, any>,
  ) => void;
  emojis: string[];
}

export const ChatInputArea: React.FC<ChatInputAreaProps> = ({
  id,
  currentUser,
  chat,
  replyingTo,
  setReplyingTo,
  editor,
  inputValue,
  isVoiceMode,
  setIsVoiceMode,
  activePanel,
  setActivePanel,
  isRecording,
  startRecording,
  handleSendVoice,
  cancelRecording,
  handleSend,
  handleSendCustom,
  emojis,
}) => {
  const togglePanel = (panel: "emoji" | "action") => {
    if (activePanel === panel) {
      setActivePanel("none");
      editor?.commands.focus();
    } else {
      setActivePanel(panel);
      setIsVoiceMode(false);
    }
  };

  const handleInputFocus = () => {
    setActivePanel("none");
  };

  const ActionItem = ({ icon: Icon, label, onClick }: any) => (
    <div
      className="flex flex-col items-center gap-2 cursor-pointer active:opacity-70"
      onClick={onClick}
    >
      <div className="w-14 h-14 bg-white dark:bg-[#2C2C2C] rounded-2xl flex items-center justify-center shadow-sm">
        <Icon className="w-7 h-7 text-text-main" />
      </div>
      <span className="text-[12px] text-text-sub">{label}</span>
    </div>
  );

  return (
    <div className="bg-input-bg border-t border-border-color shrink-0 flex flex-col pb-safe transition-all duration-300">
      <AnimatePresence>
        {replyingTo && (
          <motion.div
            initial={{ opacity: 0, height: 0 }}
            animate={{ opacity: 1, height: "auto" }}
            exit={{ opacity: 0, height: 0 }}
            className="px-3 py-2 bg-black/5 dark:bg-white/5 border-b border-border-color flex items-center justify-between overflow-hidden"
          >
            <div className="flex flex-col text-sm truncate pr-4">
              <span className="text-text-sub font-medium text-[12px] mb-0.5">
                回复{" "}
                {replyingTo.senderId === currentUser?.id
                  ? "我"
                  : chat?.participants.find((p) => p.id === replyingTo.senderId)
                      ?.name || "未知"}
                :
              </span>
              <span className="text-text-main truncate text-[14px]">
                {replyingTo.type === "text"
                  ? replyingTo.content
                  : `[${replyingTo.type === "image" ? "图片" : replyingTo.type === "voice" ? "语音" : replyingTo.type === "video" ? "视频" : replyingTo.type === "file" ? "文件" : "媒体消息"}]`}
              </span>
            </div>
            <IconButton
              icon={<X className="w-5 h-5 text-text-sub" />}
              onClick={() => setReplyingTo(null)}
              className="shrink-0 p-1 bg-black/5 dark:bg-white/10 rounded-full hover:bg-black/10 dark:hover:bg-white/20"
            />
          </motion.div>
        )}
      </AnimatePresence>

      <div className="px-2 py-2 flex items-end gap-1.5">
        <IconButton
          icon={
            isVoiceMode ? (
              <Keyboard className="w-7 h-7 text-text-main" />
            ) : (
              <Mic className="w-7 h-7 text-text-main" />
            )
          }
          onClick={() => {
            setIsVoiceMode(!isVoiceMode);
            setActivePanel("none");
          }}
          className="shrink-0 mb-0.5"
        />

        <div className="flex-1 flex items-end min-h-[40px] py-1">
          {isVoiceMode ? (
            <motion.button
              className={cn(
                "w-full h-10 rounded-lg font-bold text-[16px] transition-colors select-none flex items-center justify-center gap-2",
                isRecording
                  ? "bg-primary-blue text-white shadow-lg shadow-primary-blue/30"
                  : "bg-chat-other-bg text-text-main border border-border-color",
              )}
              animate={{ scale: isRecording ? 0.96 : 1 }}
              transition={{ type: "spring", stiffness: 400, damping: 25 }}
              onPointerDown={(e) => {
                e.preventDefault();
                startRecording();
              }}
              onPointerUp={() => {
                if (isRecording) {
                  handleSendVoice();
                }
              }}
              onPointerLeave={() => {
                if (isRecording) {
                  cancelRecording();
                }
              }}
              onPointerCancel={() => {
                if (isRecording) {
                  cancelRecording();
                }
              }}
            >
              {isRecording ? (
                <>
                  <Mic className="w-5 h-5 animate-pulse" />
                  松开 发送
                </>
              ) : (
                "按住 说话"
              )}
            </motion.button>
          ) : (
            <div
              className="w-full"
              onKeyDown={(e) => {
                if (e.key === "Enter" && !e.shiftKey) {
                  e.preventDefault();
                  handleSend();
                }
              }}
              onClick={handleInputFocus}
            >
              <EditorContent editor={editor} />
            </div>
          )}
        </div>

        <IconButton
          icon={<Smile className="w-7 h-7 text-text-main" />}
          onClick={() => togglePanel("emoji")}
          className="shrink-0 mb-0.5"
        />

        {!isVoiceMode && inputValue.trim() ? (
          <button
            onClick={handleSend}
            className="shrink-0 w-12 h-9 bg-primary-blue rounded-lg flex items-center justify-center text-white mb-1.5 mr-1 active:opacity-80 transition-opacity"
          >
            <Send className="w-4 h-4" />
          </button>
        ) : (
          <IconButton
            icon={<PlusCircle className="w-7 h-7 text-text-main" />}
            onClick={() => togglePanel("action")}
            className="shrink-0 mb-0.5"
          />
        )}
      </div>

      <AnimatePresence initial={false}>
        {activePanel === "action" && (
          <motion.div
            initial={{ height: 0, opacity: 0 }}
            animate={{ height: 256, opacity: 1 }}
            exit={{ height: 0, opacity: 0 }}
            transition={{ duration: 0.2, ease: "easeOut" }}
            className="bg-input-bg border-t border-border-color overflow-hidden"
          >
            <div className="grid grid-cols-4 gap-y-6 p-6 h-64">
              <ActionItem
                icon={ImageIcon}
                label="照片"
                onClick={() =>
                  handleSendCustom(
                    "image",
                    "https://picsum.photos/seed/newimg/800/450",
                  )
                }
              />
              <ActionItem
                icon={Video}
                label="视频"
                onClick={() =>
                  handleSendCustom(
                    "video",
                    "https://storage.googleapis.com/gtv-videos-bucket/sample/BigBuckBunny.mp4",
                    {
                      coverUrl: "https://picsum.photos/seed/vid/300/400",
                      duration: "0:10",
                    },
                  )
                }
              />
              <ActionItem
                icon={Music}
                label="音乐"
                onClick={() =>
                  handleSendCustom(
                    "music",
                    "https://assets.mixkit.co/active_storage/sfx/2869/2869-preview.mp3",
                    {
                      title: "Mixkit Tech House",
                      artist: "Mixkit Author",
                      coverUrl: "https://picsum.photos/seed/song/300/300",
                    },
                  )
                }
              />
              <ActionItem
                icon={Folder}
                label="文件"
                onClick={() =>
                  handleSendCustom("file", "财务报表_2026.xlsx", {
                    size: "1.2 MB",
                    ext: "xlsx",
                  })
                }
              />
              <ActionItem
                icon={LinkIcon}
                label="链接"
                onClick={() =>
                  handleSendCustom("link", "https://example.com/article", {
                    title: "这是一篇非常有趣的文章",
                    desc: "阅读这篇文章可以让你学到很多关于宇宙的奥秘...",
                    image: "https://picsum.photos/seed/link/100/100",
                  })
                }
              />
              <ActionItem
                icon={ShoppingBag}
                label="小程序"
                onClick={() =>
                  handleSendCustom("miniapp", "爪子商城", {
                    title: "超级划算的商品，快来抢购！",
                    desc: "限时秒杀，仅此一天",
                    icon: "https://picsum.photos/seed/mini/50/50",
                    image: "https://picsum.photos/seed/mini2/300/200",
                  })
                }
              />
              <ActionItem
                icon={Phone}
                label="语音通话"
                onClick={() =>
                  handleSendCustom("call", "语音通话", {
                    status: "missed",
                  })
                }
              />
            </div>
          </motion.div>
        )}

        {activePanel === "emoji" && (
          <motion.div
            initial={{ height: 0, opacity: 0 }}
            animate={{ height: 256, opacity: 1 }}
            exit={{ height: 0, opacity: 0 }}
            transition={{ duration: 0.2, ease: "easeOut" }}
            className="bg-input-bg border-t border-border-color overflow-y-auto"
          >
            <div className="grid grid-cols-8 gap-4 p-4 h-64">
              {emojis.map((e) => (
                <span
                  key={e}
                  className="text-2xl cursor-pointer active:scale-90 transition-transform text-center"
                  onClick={() => {
                    editor?.commands.insertContent(e);
                  }}
                >
                  {e}
                </span>
              ))}
            </div>
          </motion.div>
        )}
      </AnimatePresence>
    </div>
  );
};
