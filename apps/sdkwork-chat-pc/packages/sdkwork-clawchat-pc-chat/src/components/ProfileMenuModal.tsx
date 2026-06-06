import React from "react";
import { motion, AnimatePresence } from "motion/react";
import { Avatar } from "@sdkwork/clawchat-pc-commons";
import { Star, Settings } from "lucide-react";
import { toast } from "./Toast";
import type { User } from "@sdkwork/clawchat-pc-types";

interface ProfileMenuModalProps {
  currentUser: User;
  onClose: () => void;
  onLogout: () => void | Promise<void>;
  onTabChange: (tab: string) => void;
  onOpenSettings?: () => void;
}

export const ProfileMenuModal: React.FC<ProfileMenuModalProps> = ({
  currentUser,
  onClose,
  onLogout,
  onTabChange,
  onOpenSettings,
}) => {
  const copyCurrentUserId = async () => {
    try {
      await navigator.clipboard.writeText(currentUser.id);
      toast("已复制ID", "success");
    } catch {
      toast("复制ID失败", "error");
    }
  };

  return (
    <>
      <motion.div
        initial={{ opacity: 0 }}
        animate={{ opacity: 1 }}
        exit={{ opacity: 0 }}
        className="fixed inset-0 z-40"
        onClick={onClose}
      />
      <motion.div
        initial={{ opacity: 0, x: -20 }}
        animate={{ opacity: 1, x: 0 }}
        exit={{ opacity: 0, x: -20 }}
        transition={{ type: "spring", damping: 25, stiffness: 300 }}
        className="absolute top-12 left-16 w-80 bg-[#1e1e1e] border border-white/10 rounded-2xl shadow-2xl overflow-hidden z-50"
      >
        <div className="p-5 flex items-center gap-4 border-b border-white/5">
          <Avatar
            src={currentUser.avatar}
            alt={currentUser.name}
            className="w-16 h-16 rounded-xl bg-[#2b2b2d]"
          />
          <div className="flex flex-col flex-1 min-w-0">
            <h3 className="text-xl font-bold text-gray-100 truncate">
              {currentUser.name}
            </h3>
            <button
              type="button"
              title="复制ID"
              onClick={copyCurrentUserId}
              className="mt-1 text-xs text-gray-500 hover:text-gray-300 transition-colors flex items-center gap-1 min-w-0 text-left"
            >
              <span className="shrink-0">ID:</span>
              <span className="truncate font-mono">{currentUser.id}</span>
            </button>
            <div className="text-sm text-gray-400 mt-1 flex items-center gap-2">
              <div className="w-2 h-2 rounded-full bg-[#00b42a]" /> 在线办公中
            </div>
          </div>
        </div>

        <div className="p-2 border-b border-white/5 grid grid-cols-2 gap-1 text-center">
          <div
            className="flex flex-col items-center p-3 hover:bg-white/5 rounded-xl cursor-pointer transition-colors"
            onClick={() => {
              onClose();
              onTabChange("favorites");
            }}
          >
            <Star size={20} className="text-gray-400 mb-1" />
            <span className="text-xs text-gray-400">收藏</span>
          </div>
          <div
            className="flex flex-col items-center p-3 hover:bg-white/5 rounded-xl cursor-pointer transition-colors"
            onClick={() => {
              onClose();
              if (onOpenSettings) onOpenSettings();
            }}
          >
            <Settings size={20} className="text-gray-400 mb-1" />
            <span className="text-xs text-gray-400">设置</span>
          </div>
        </div>

        <div className="p-2">
          <button className="w-full px-4 py-2.5 text-left text-sm text-gray-300 hover:text-white hover:bg-white/5 transition-colors rounded-lg flex items-center gap-3">
            <span className="w-5 flex justify-center">
              <div className="w-2.5 h-2.5 rounded-full bg-red-500 border-2 border-[#1e1e1e]" />
            </span>
            忙碌
          </button>
          <button className="w-full px-4 py-2.5 text-left text-sm text-gray-300 hover:text-white hover:bg-white/5 transition-colors rounded-lg flex items-center gap-3">
            <span className="w-5 flex justify-center">
              <div className="w-2.5 h-2.5 rounded-full bg-yellow-500 border-2 border-[#1e1e1e]" />
            </span>
            离开
          </button>
          <div className="h-px bg-white/5 my-2 mx-2" />
          <button
            onClick={() => {
              onClose();
              void onLogout();
            }}
            className="w-full px-4 py-2 text-left text-sm text-red-400 hover:bg-red-500/10 transition-colors rounded-lg flex items-center gap-3"
          >
            退出登录
          </button>
        </div>
      </motion.div>
    </>
  );
};
