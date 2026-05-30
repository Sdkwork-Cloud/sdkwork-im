import { useNavigate } from "react-router";
import { useParams } from "react-router";
import React, { useState, useEffect } from "react";
import {} from "react-router";
import {
  ChevronLeft,
  MoreHorizontal,
  Video,
  ShieldCheck,
  Loader2,
  File,
} from "lucide-react";
import {
  IconButton,
  cn,
  MediaPreview,
  showToast,
  ActionSheet,
  showPrompt,
  showConfirm,
} from "@sdkwork/clawchat-mobile-commons";
import { motion, AnimatePresence } from "motion/react";
import { NotaryPartyParams } from "./NotaryAddParty";
import {
  notaryService,
  NotaryDetailData,
  NotaryFile,
} from "../services/notaryService";
import { NotaryFileItem } from "../components/NotaryFileItem";

export const NotaryDetail: React.FC = () => {
  const navigate = useNavigate();
  const { id } = useParams();

  const [detail, setDetail] = useState<NotaryDetailData | null>(null);
  const [loading, setLoading] = useState(true);
  const [activeTab, setActiveTab] = useState<"parties" | "materials">(
    "parties",
  );
  const [isActionSheetOpen, setIsActionSheetOpen] = useState(false);

  // Preview state
  const [previewMedia, setPreviewMedia] = useState<{
    type: string;
    url: string;
    name?: string;
  } | null>(null);

  const touchStartX = React.useRef<number | null>(null);
  const touchStartY = React.useRef<number | null>(null);

  const handleTouchStart = (e: React.TouchEvent) => {
    touchStartX.current = e.touches[0].clientX;
    touchStartY.current = e.touches[0].clientY;
  };

  const handleTouchEnd = (e: React.TouchEvent) => {
    if (touchStartX.current === null || touchStartY.current === null) return;
    const touchEndX = e.changedTouches[0].clientX;
    const touchEndY = e.changedTouches[0].clientY;

    const diffX = touchStartX.current - touchEndX;
    const diffY = touchStartY.current - touchEndY;

    if (Math.abs(diffX) > Math.abs(diffY) && Math.abs(diffX) > 50) {
      if (diffX > 0 && activeTab === "parties") {
        setActiveTab("materials");
      } else if (diffX < 0 && activeTab === "materials") {
        setActiveTab("parties");
      }
    }
    touchStartX.current = null;
    touchStartY.current = null;
  };

  useEffect(() => {
    const loadData = async () => {
      setLoading(true);
      try {
        const data = await notaryService.getNotaryDetail(id || "1");
        setDetail(data);
      } catch (e) {
        console.error(e);
      } finally {
        setLoading(false);
      }
    };
    loadData();
  }, [id]);

  const handleEditParty = (p: any) => {
    // Pass context to the AddParty page for editing
    NotaryPartyParams.editData = p;
    NotaryPartyParams.onEdit = (updated: any) => {
      showToast("当事人信息已更新");
    };
    navigate("/notary/add-party");
  };

  const handleFileClick = (file: NotaryFile) => {
    if (file.fileType === "image" || file.fileType === "video") {
      // Provide mockup URLs based on file type for preview
      const previewUrl =
        file.fileType === "image"
          ? "https://picsum.photos/seed/notaryfile/800/1200"
          : "https://www.w3schools.com/html/mov_bbb.mp4";

      setPreviewMedia({
        type: file.fileType,
        url: previewUrl,
        name: file.name,
      });
    } else {
      showToast(`正在外部应用中打开:\n${file.name}`);
    }
  };

  if (loading || !detail) {
    return (
      <div className="flex flex-col h-full bg-[#f4f6f9] dark:bg-black font-sans relative animate-in slide-in-from-right z-10 w-full absolute inset-0 items-center justify-center">
        <Loader2 className="w-8 h-8 animate-spin text-primary-blue" />
        <span className="mt-4 text-[14px] text-text-sub">
          正在加载公证详情...
        </span>
      </div>
    );
  }

  return (
    <div className="flex flex-col h-full bg-[#f4f6f9] dark:bg-black font-sans text-text-main relative animate-in slide-in-from-right z-10 w-full absolute inset-0">
      {/* Header */}
      <header className="h-[44px] flex items-center justify-between sticky top-0 shrink-0 pt-safe px-2 z-20 glass-header border-b border-border-color">
        <div className="flex items-center z-10 w-[80px]">
          <IconButton
            icon={
              <ChevronLeft className="w-7 h-7 text-text-main" strokeWidth={2} />
            }
            onClick={() => navigate(-1)}
          />
        </div>
        <div className="flex items-center justify-center font-medium text-[17px] pointer-events-none flex-1">
          公证详情
        </div>
        <div className="flex justify-end z-10 w-[80px] pl-2">
          {/* Mini-program style capsule button */}
          <div className="flex items-center bg-black/5 dark:bg-white/10 rounded-full h-[32px] border border-black/5 dark:border-white/10 overflow-hidden shrink-0 mt-1">
            <div
              className="flex items-center justify-center w-[40px] h-full cursor-pointer active:bg-black/10 transition-colors"
              onClick={() => setIsActionSheetOpen(true)}
            >
              <MoreHorizontal className="w-5 h-5 text-text-main" />
            </div>
            <div className="w-[1px] h-4 bg-black/10 dark:bg-white/10" />
            <div
              className="flex items-center justify-center w-[40px] h-full cursor-pointer active:bg-black/10 transition-colors"
              onClick={() => navigate("/workspace")}
            >
              <div className="w-5 h-5 rounded-full border border-text-main/80 flex items-center justify-center">
                <div className="w-2 h-2 rounded-full bg-text-main/80" />
              </div>
            </div>
          </div>
        </div>
      </header>

      <div className="flex-1 overflow-y-auto">
        {/* Main Info Block */}
        <div className="bg-bg-color px-5 pt-5 pb-6 mb-2 border-b border-border-color">
          <h1 className="text-[20px] font-bold mb-6">{detail.title}</h1>

          <div className="flex flex-col gap-4 text-[15px]">
            <div className="flex items-start gap-4">
              <span className="w-[85px] text-text-sub shrink-0">时间</span>
              <span className="flex-1 text-text-main">{detail.time}</span>
            </div>
            <div className="flex items-start gap-4">
              <span className="w-[85px] text-text-sub shrink-0">公证事项</span>
              <span className="flex-1 text-text-main font-medium">
                {detail.item}
              </span>
            </div>
            <div className="flex items-start gap-4">
              <span className="w-[85px] text-text-sub shrink-0">公证员</span>
              <span className="flex-1 text-text-main">{detail.notaryName}</span>
            </div>
            <div className="flex items-start gap-4">
              <span className="w-[85px] text-text-sub shrink-0">
                公证员编号
              </span>
              <span className="flex-1 text-text-main break-all">
                {detail.notaryNo}
              </span>
            </div>
            <div className="flex items-center gap-4">
              <span className="w-[85px] text-text-sub shrink-0">状态</span>
              <div className="px-2 py-0.5 border border-border-color bg-input-bg text-text-sub rounded-sm text-[13px]">
                {detail.status}
              </div>
            </div>
            <div className="flex items-start gap-4">
              <span className="w-[85px] text-text-sub shrink-0">备注</span>
              <span className="flex-1 text-text-main leading-relaxed">
                {detail.remarks}
              </span>
            </div>
          </div>
        </div>

        {/* Tabs block */}
        <div className="bg-bg-color min-h-[500px]">
          <div className="flex items-center border-b border-border-color px-10 pt-2 sticky top-0 bg-bg-color z-10">
            <div
              className={cn(
                "flex-1 py-3 text-center text-[16px] font-medium relative cursor-pointer",
                activeTab === "parties" ? "text-primary-blue" : "text-text-sub",
              )}
              onClick={() => setActiveTab("parties")}
            >
              当事人
              {activeTab === "parties" && (
                <div className="absolute left-0 right-0 bottom-0 flex justify-center">
                  <motion.div
                    layoutId="detailTabs"
                    className="w-6 h-[3px] bg-primary-blue rounded-t-full"
                  />
                </div>
              )}
            </div>
            <div
              className={cn(
                "flex-1 py-3 text-center text-[16px] font-medium relative cursor-pointer",
                activeTab === "materials"
                  ? "text-primary-blue"
                  : "text-text-sub",
              )}
              onClick={() => setActiveTab("materials")}
            >
              公证材料
              {activeTab === "materials" && (
                <div className="absolute left-0 right-0 bottom-0 flex justify-center">
                  <motion.div
                    layoutId="detailTabs"
                    className="w-6 h-[3px] bg-primary-blue rounded-t-full"
                  />
                </div>
              )}
            </div>
          </div>

          {/* Tab Content */}
          <div
            className="bg-bg-color min-h-[300px]"
            onTouchStart={handleTouchStart}
            onTouchEnd={handleTouchEnd}
          >
            <AnimatePresence mode="wait">
              {activeTab === "parties" && (
                <motion.div
                  key="parties"
                  initial={{ opacity: 0, y: 10 }}
                  animate={{ opacity: 1, y: 0 }}
                  exit={{ opacity: 0, y: -10 }}
                  transition={{ duration: 0.2 }}
                  className="flex flex-col gap-3 p-4 bg-[#f4f6f9] dark:bg-black"
                >
                  {detail.parties.map((p, i) => (
                    <div
                      key={i}
                      className="bg-bg-color rounded-xl p-4 flex gap-4 shadow-sm border border-border-color cursor-pointer active:scale-[0.98] transition-all"
                      onClick={() => handleEditParty(p)}
                    >
                      <div className="w-[84px] h-[84px] shrink-0 bg-chat-other-bg rounded-lg overflow-hidden border border-border-color/50">
                        <img
                          src={p.avatar}
                          alt="avatar"
                          className="w-full h-full object-cover"
                        />
                      </div>
                      <div className="flex flex-col justify-between flex-1 py-0.5 relative min-w-0">
                        <div className="flex flex-col items-start gap-1.5 w-full">
                          <div className="flex items-start justify-between w-full">
                            <span className="text-[17px] font-bold text-text-main truncate pr-2">
                              {p.name}
                            </span>
                            <div className="px-1.5 py-0.5 border border-green-500/30 bg-green-500/10 text-green-600 dark:text-green-400 rounded text-[11px] font-medium whitespace-nowrap flex items-center gap-1 shrink-0">
                              <ShieldCheck className="w-3 h-3" />
                              {p.status}
                            </div>
                          </div>
                          <span className="text-[13px] text-text-sub">
                            性别：{p.gender}
                          </span>
                        </div>

                        <div className="flex items-center justify-end mt-auto pt-2">
                          <button
                            onClick={async (e) => {
                              e.stopPropagation();
                              navigate(`/call/video-notary-${p.id}`);
                            }}
                            className="flex items-center justify-center h-8 px-4 rounded-lg bg-primary-blue text-white text-[13px] font-bold active:opacity-80 transition-opacity shadow-sm"
                          >
                            <Video className="w-4 h-4 mr-1.5" /> 开始视频
                          </button>
                        </div>
                      </div>
                    </div>
                  ))}
                </motion.div>
              )}
              {activeTab === "materials" && (
                <motion.div
                  key="materials"
                  initial={{ opacity: 0, y: 10 }}
                  animate={{ opacity: 1, y: 0 }}
                  exit={{ opacity: 0, y: -10 }}
                  transition={{ duration: 0.2 }}
                  className="flex flex-col"
                >
                  {detail.materials && detail.materials.length > 0 ? (
                    detail.materials.map((file) => (
                      <NotaryFileItem
                        key={file.id}
                        file={file}
                        onClick={handleFileClick}
                      />
                    ))
                  ) : (
                    <div className="flex flex-col items-center justify-center py-20 text-text-sub opacity-70">
                      <File className="w-12 h-12 mb-3 stroke-current opacity-40" />
                      <span className="text-[14px]">暂无相对应的公证材料</span>
                    </div>
                  )}
                </motion.div>
              )}
            </AnimatePresence>
          </div>
        </div>
      </div>

      {/* Media Preview Overlay */}
      <MediaPreview
        media={previewMedia as any}
        onClose={() => setPreviewMedia(null)}
      />

      <ActionSheet
        isOpen={isActionSheetOpen}
        onClose={() => setIsActionSheetOpen(false)}
        title="详细操作"
        options={[
          {
            label: "分享公证",
            onClick: async () => {
              await showPrompt(
                "分享链接：",
                "https://clawchat.sdkwork.com/notary/" + id,
              );
            },
          },
          {
            label: "复制公证号",
            onClick: () => {
              if (detail) {
                navigator.clipboard.writeText(detail.id);
                showToast("已复制：" + detail.id);
              }
            },
          },
          {
            label: "发起视频通话",
            onClick: () => navigate(`/call/video-notary-${id}`),
          },
          {
            label: "撤销",
            danger: true,
            onClick: async () => {
              const confirm = await showConfirm("确定要撤销公证吗？");
              if (confirm) {
                await notaryService.updateRecordStatus(id!, "cancelled");
                showToast("该公证已申请撤销");
                navigate(-1);
              }
            },
          },
        ]}
      />
    </div>
  );
};
