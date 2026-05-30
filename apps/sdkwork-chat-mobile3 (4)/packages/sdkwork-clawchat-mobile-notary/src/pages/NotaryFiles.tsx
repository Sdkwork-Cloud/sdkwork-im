import React, { useState, useEffect } from "react";
import {
  Folder,
  MoreVertical,
  FileText,
  Search,
  Plus,
  Image as ImageIcon,
  File,
  ChevronLeft,
  X,
} from "lucide-react";
import { showPrompt, cn } from "@sdkwork/clawchat-mobile-commons";
import {
  IconButton,
  showToast,
  ActionSheet,
} from "@sdkwork/clawchat-mobile-commons";
import { useNavigate } from "react-router";
import { notaryService } from "../services/notaryService";

export const NotaryFiles: React.FC = () => {
  const navigate = useNavigate();
  const [items, setItems] = useState<any[]>([]);
  const [isLoading, setIsLoading] = useState(true);
  const [isActionSheetOpen, setIsActionSheetOpen] = useState(false);
  const [isSearchOpen, setIsSearchOpen] = useState(false);
  const [searchQuery, setSearchQuery] = useState("");
  const [activeItem, setActiveItem] = useState<any>(null);

  useEffect(() => {
    setIsLoading(true);
    notaryService.getCloudFiles().then((data) => {
      setItems(data as any[]);
      setIsLoading(false);
    });
  }, []);

  const filteredItems = items.filter((item) =>
    item.name.toLowerCase().includes(searchQuery.toLowerCase()),
  );

  const getIcon = (type: string) => {
    switch (type) {
      case "folder":
        return Folder;
      case "image":
        return ImageIcon;
      case "pdf":
        return FileText;
      case "doc":
        return File;
      default:
        return File;
    }
  };

  return (
    <div className="flex flex-col h-full bg-bg-color relative">
      {/* Header */}
      <header className="h-[56px] flex items-center justify-between px-1 glass-header sticky top-0 z-10 shrink-0 pt-safe relative">
        {isSearchOpen ? (
          <div className="flex items-center w-full px-2 gap-2">
            <div className="flex-1 bg-chat-other-bg h-[36px] rounded-full flex items-center px-3 border border-border-color">
              <Search className="w-4 h-4 text-text-sub shrink-0" />
              <input
                autoFocus
                type="text"
                placeholder="搜索文件"
                className="flex-1 bg-transparent px-2 text-[14px] outline-none text-text-main"
                value={searchQuery}
                onChange={(e) => setSearchQuery(e.target.value)}
              />
              {searchQuery && (
                <X
                  className="w-4 h-4 text-text-sub cursor-pointer"
                  onClick={() => setSearchQuery("")}
                />
              )}
            </div>
            <div
              className="text-[15px] text-text-sub cursor-pointer px-2"
              onClick={async () => {
                setIsSearchOpen(false);
                setSearchQuery("");
              }}
            >
              取消
            </div>
          </div>
        ) : (
          <>
            <div className="flex items-center z-10"></div>
            <div className="absolute left-1/2 -translate-x-1/2 flex items-center pointer-events-none">
              <span className="text-[17px] font-bold text-text-main">
                公证云盘
              </span>
            </div>
            <div className="flex items-center gap-1 z-10 pr-1">
              <IconButton
                icon={<Search className="w-5 h-5 text-text-main" />}
                onClick={() => setIsSearchOpen(true)}
              />
              <IconButton
                icon={<Plus className="w-6 h-6 text-text-main" />}
                onClick={() => setIsActionSheetOpen(true)}
              />
            </div>
          </>
        )}
      </header>

      <div className="flex-1 overflow-y-auto pb-[90px]">
        {/* File List */}
        <div className="flex flex-col">
          {isLoading ? (
            <div className="flex flex-col items-center justify-center py-20 text-text-sub opacity-70">
              <div className="w-8 h-8 rounded-full border-4 border-text-sub border-t-transparent animate-spin mb-3"></div>
              <p className="text-[14px]">加载中...</p>
            </div>
          ) : (
            filteredItems.map((item, index) => {
              const Icon = getIcon(item.type);
              return (
              <div
                key={item.id}
                className="flex items-center pl-4 pr-2 py-3 active:bg-active-bg transition-colors cursor-pointer group"
              >
                <div
                  className={cn(
                    "w-12 h-12 rounded-[14px] flex items-center justify-center shrink-0 mr-3",
                    item.bg || "bg-transparent",
                  )}
                >
                  <Icon
                    className={cn("w-8 h-8", item.iconColor, item.fill)}
                    strokeWidth={item.type === "folder" ? 1.5 : 2}
                  />
                </div>
                <div
                  className={cn(
                    "flex-1 min-w-0 pb-3 flex items-center justify-between pt-1",
                    index !== filteredItems.length - 1
                      ? "border-b border-border-color/50"
                      : "",
                  )}
                >
                  <div className="flex flex-col min-w-0 pr-2">
                    <span className="text-[16px] text-text-main font-medium truncate tracking-wide">
                      {item.name}
                    </span>
                    <div className="flex items-center gap-2 mt-1">
                      <span className="text-[12px] text-text-sub/80">
                        {item.date}
                      </span>
                      {item.size !== "-" && (
                        <span className="text-[12px] text-text-sub/80">
                          {item.size}
                        </span>
                      )}
                    </div>
                  </div>
                  <IconButton
                    icon={<MoreVertical className="w-5 h-5 text-text-sub" />}
                    onClick={async (e) => {
                      e.stopPropagation();
                      setActiveItem(item);
                    }}
                  />
                </div>
              </div>
            );
            })
          )}

          {!isLoading && filteredItems.length === 0 && (
            <div className="flex flex-col items-center justify-center py-20 text-text-sub opacity-70">
              <Folder
                className="w-12 h-12 mb-3 opacity-40 stroke-current"
                strokeWidth={1.5}
              />
              <p className="text-[14px]">没有找到对应的文件</p>
            </div>
          )}
        </div>
      </div>

      <ActionSheet
        isOpen={activeItem !== null}
        onClose={() => setActiveItem(null)}
        options={[
          { label: "分享", onClick: () => showToast("链接已复制到剪贴板") },
          {
            label: "重命名",
            onClick: async () => {
              const newName = await showPrompt(
                "请输入新名称",
                activeItem?.name,
              );
              if (newName && newName.trim()) {
                await notaryService.renameCloudFile(activeItem.id, newName);
                notaryService
                  .getCloudFiles()
                  .then((data) => setItems(data as any[]));
                showToast("已重命名");
              }
            },
          },
          {
            label: "删除",
            danger: true,
            onClick: async () => {
              await notaryService.deleteCloudFile(activeItem?.id);
              notaryService
                .getCloudFiles()
                .then((data) => setItems(data as any[]));
              showToast("文件已删除");
            },
          },
        ]}
      />

      <ActionSheet
        isOpen={isActionSheetOpen}
        onClose={() => setIsActionSheetOpen(false)}
        title="添加"
        options={[
          {
            label: "新建文件夹",
            onClick: async () => {
              const name = await showPrompt("请输入文件夹名称");
              if (name) {
                const newFolder: any = {
                  id: Math.random().toString(),
                  name: name,
                  type: "folder",
                  size: "-",
                  date: "刚刚",
                  uploadTime: "刚刚",
                  uploader: "我",
                  iconColor: "text-yellow-400",
                  fill: "fill-yellow-400",
                };
                await notaryService.addCloudFile(newFolder);
                notaryService
                  .getCloudFiles()
                  .then((data) => setItems(data as any[]));
                showToast("已新建文件夹");
              }
            },
          },
          {
            label: "上传文件/照片",
            onClick: async () => {
              const newFile: any = {
                id: Math.random().toString(),
                name: `新上传文件_${Date.now()}.png`,
                type: "image",
                size: "1.2 MB",
                date: "刚刚",
                uploadTime: "刚刚",
                uploader: "我",
                iconColor: "text-green-500",
                bg: "bg-green-500/10",
              };
              await notaryService.addCloudFile(newFile);
              notaryService
                .getCloudFiles()
                .then((data) => setItems(data as any[]));
              showToast("上传成功！");
            },
          },
        ]}
      />
    </div>
  );
};
