import React, { useState, useEffect } from "react";
import {
  showPrompt,
  PageLayout,
  IconButton,
  cn,
  showToast,
  ActionSheet,
} from "@sdkwork/clawchat-mobile-commons";
import {
  Folder,
  File as FileIcon,
  FileText,
  Image as ImageIcon,
  Video,
  Search,
  Filter,
  MoreHorizontal,
  Plus,
  CloudUpload,
  Clock,
  HardDrive,
  FilterIcon,
  FolderOpen,
  PieChart,
  Database,
  Zap,
} from "lucide-react";
import { CloudDriveService, CloudFile } from "../../services/CloudDriveService";
import { motion } from "motion/react";

export const CloudDriveApp = () => {
  const [activeTab, setActiveTab] = useState<"recent" | "files" | "shared">(
    "files",
  );
  const [files, setFiles] = useState<CloudFile[]>([]);
  const [isLoading, setIsLoading] = useState(true);
  const [isFabSheetOpen, setIsFabSheetOpen] = useState(false);
  const [activeFile, setActiveFile] = useState<string | null>(null);

  useEffect(() => {
    setIsLoading(true);
    CloudDriveService.getFiles().then(data => {
      setFiles(data);
      setIsLoading(false);
    });
  }, []);

  const getFileIcon = (type: string) => {
    switch (type) {
      case "folder":
        return (
          <Folder className="w-6 h-6 text-yellow-500 fill-yellow-500/20" />
        );
      case "pdf":
        return <FileText className="w-6 h-6 text-rose-500" />;
      case "video":
        return <Video className="w-6 h-6 text-indigo-500" />;
      case "excel":
        return <PieChart className="w-6 h-6 text-emerald-500" />;
      case "image":
        return <ImageIcon className="w-6 h-6 text-blue-500" />;
      default:
        return <FileIcon className="w-6 h-6 text-slate-500" />;
    }
  };

  return (
    <PageLayout title="云盘">
      <div className="flex flex-col h-full bg-[#f5f6f8] dark:bg-[#1a1b1c]">
        {/* Storage usage */}
        <div className="bg-primary-blue px-6 pt-4 pb-12 text-white">
          <div className="flex items-center justify-between mb-4">
            <div className="flex items-center gap-2">
              <Database className="w-5 h-5 opacity-90" />
              <span className="font-medium text-[16px]">个人空间</span>
            </div>
            <div className="text-[14px] opacity-80 font-mono">
              15.4 GB / 100 GB
            </div>
          </div>
          <div className="w-full bg-white/20 rounded-full h-2 overflow-hidden mb-2">
            <div className="bg-white h-full rounded-full w-[15.4%]" />
          </div>
          <div className="flex justify-between text-[12px] opacity-70">
            <span>已用 15.4%</span>
            <span>可用 84.6 GB</span>
          </div>
        </div>

        {/* Action Buttons */}
        <div className="px-4 -mt-6 mb-4">
          <div className="bg-white dark:bg-[#2c2d2e] rounded-xl shadow-sm grid grid-cols-4 py-4 px-2">
            {[
              {
                icon: Clock,
                label: "最近",
                id: "recent",
                color: "text-blue-500",
                bg: "bg-blue-50",
              },
              {
                icon: FolderOpen,
                label: "文件",
                id: "files",
                color: "text-amber-500",
                bg: "bg-amber-50",
              },
              {
                icon: CloudUpload,
                label: "传输",
                id: "transfer",
                color: "text-emerald-500",
                bg: "bg-emerald-50",
              },
              {
                icon: Zap,
                label: "快捷",
                id: "quick",
                color: "text-purple-500",
                bg: "bg-purple-50",
              },
            ].map((item) => (
              <div
                key={item.id}
                className="flex flex-col items-center gap-2 cursor-pointer group"
                onClick={() =>
                  item.id === "recent" || item.id === "files"
                    ? setActiveTab(item.id as any)
                    : null
                }
              >
                <div
                  className={cn(
                    "w-12 h-12 rounded-2xl flex items-center justify-center transition-all duration-200",
                    activeTab === item.id
                      ? `${item.color} ${item.bg} dark:bg-[#3a3b3c] shadow-sm scale-110`
                      : "bg-gray-50 dark:bg-[#3a3b3c] text-text-sub group-hover:scale-105 group-active:scale-95",
                  )}
                >
                  <item.icon className="w-6 h-6" />
                </div>
                <span
                  className={cn(
                    "text-[12px]",
                    activeTab === item.id
                      ? "text-text-main font-medium"
                      : "text-text-sub",
                  )}
                >
                  {item.label}
                </span>
              </div>
            ))}
          </div>
        </div>

        {/* File List */}
        <div className="flex-1 overflow-y-auto px-4 pb-20">
          <div className="flex items-center justify-between py-3 px-1">
            <h2 className="text-[14px] font-medium text-text-sub">
              {activeTab === "recent" ? "最近使用" : "全部文件"}
            </h2>
            <div className="flex gap-2">
              <IconButton
                icon={<Filter className="w-4 h-4 text-text-sub" />}
                className="bg-white dark:bg-[#2c2d2e] p-1.5 w-auto h-auto rounded-md shadow-sm"
              />
              <IconButton
                icon={<Search className="w-4 h-4 text-text-sub" />}
                className="bg-white dark:bg-[#2c2d2e] p-1.5 w-auto h-auto rounded-md shadow-sm"
              />
            </div>
          </div>

          <div className="flex flex-col gap-2">
            {isLoading ? (
              <div className="flex flex-col items-center justify-center py-20 text-text-sub opacity-70">
                <div className="w-8 h-8 rounded-full border-4 border-text-sub border-t-transparent animate-spin mb-3"></div>
                <p className="text-[14px]">加载中...</p>
              </div>
            ) : files.length > 0 ? (
              files.map((file) => (
                <motion.div
                  key={file.id}
                  whileTap={{ scale: 0.98 }}
                  className="flex items-center gap-3 p-4 bg-white dark:bg-[#2c2d2e] rounded-xl cursor-pointer shadow-sm border border-border-color/30"
                >
                  <div className="w-12 h-12 rounded-xl bg-gray-50 dark:bg-[#3a3b3c] flex items-center justify-center shrink-0">
                    {getFileIcon(file.type)}
                  </div>
                  <div className="flex-1 min-w-0 pr-2">
                    <div className="text-[15px] font-medium text-text-main truncate mb-1">
                      {file.name}
                    </div>
                    <div className="flex items-center gap-2 text-[12px] text-text-sub font-mono">
                      <span>{file.date}</span>
                      <span className="w-1 h-1 rounded-full bg-border-color" />
                      <span>{file.size}</span>
                    </div>
                  </div>
                  <IconButton
                    icon={<MoreHorizontal className="w-5 h-5 text-text-sub" />}
                    className="w-8 h-8 -mr-2"
                    onClick={async (e) => {
                      e.stopPropagation();
                      setActiveFile(file.id);
                    }}
                  />
                </motion.div>
              ))
            ) : (
              <div className="flex flex-col items-center justify-center py-20 text-text-sub opacity-70">
                <HardDrive className="w-12 h-12 mb-3 stroke-current opacity-40" />
                <span className="text-[14px]">云盘暂无文件</span>
              </div>
            )}
          </div>
        </div>

        {/* FAB */}
        <motion.button
          whileTap={{ scale: 0.9 }}
          whileHover={{ scale: 1.05 }}
          onClick={() => setIsFabSheetOpen(true)}
          className="absolute bottom-6 right-6 w-14 h-14 bg-gradient-to-tr from-blue-600 to-primary-blue text-white rounded-full flex items-center justify-center shadow-lg shadow-blue-500/30 z-10"
        >
          <Plus className="w-7 h-7" />
        </motion.button>
      </div>

      <ActionSheet
        isOpen={isFabSheetOpen}
        onClose={() => setIsFabSheetOpen(false)}
        title="上传文件"
        options={[
          {
            label: "新建文件夹",
            onClick: async () => {
              const folder = await CloudDriveService.createFolder("新建文件夹");
              setFiles(await CloudDriveService.getFiles());
              showToast("已新建文件夹");
            },
          },
          {
            label: "上传文件",
            onClick: async () => {
              const file = await CloudDriveService.uploadFile(
                new File([""], `新文件_${Date.now()}.txt`, {
                  type: "text/plain",
                }),
              );
              setFiles(await CloudDriveService.getFiles());
              showToast("文件已上传");
            },
          },
        ]}
      />

      <ActionSheet
        isOpen={activeFile !== null}
        onClose={() => setActiveFile(null)}
        title="文件操作"
        options={[
          { label: "分享", onClick: () => showToast("链接已复制到剪贴板") },
          {
            label: "重命名",
            onClick: async () => {
              const fileData = files.find((f) => f.id === activeFile);
              if (activeFile && fileData) {
                const newName = await showPrompt("请输入新名称", fileData.name);
                if (newName && newName.trim()) {
                  await CloudDriveService.renameFile(
                    activeFile,
                    newName.trim(),
                  );
                  setFiles(await CloudDriveService.getFiles());
                }
              }
            },
          },
          {
            label: "删除",
            danger: true,
            onClick: async () => {
              if (activeFile) {
                await CloudDriveService.deleteFile(activeFile);
                setFiles(await CloudDriveService.getFiles());
                showToast("文件已删除");
              }
            },
          },
        ]}
      />
    </PageLayout>
  );
};
