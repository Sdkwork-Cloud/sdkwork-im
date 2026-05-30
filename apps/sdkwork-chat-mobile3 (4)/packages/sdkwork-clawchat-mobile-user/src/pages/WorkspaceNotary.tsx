import React from "react";
import { useNavigate } from "react-router";
import {
  ChevronLeft,
  Info,
  FileText,
  UploadCloud,
  CheckCircle2,
} from "lucide-react";
import { IconButton } from "@sdkwork/clawchat-mobile-commons";
import { motion } from "motion/react";

export const WorkspaceNotary: React.FC = () => {
  const navigate = useNavigate();

  const ActionCard = ({
    icon: Icon,
    title,
    desc,
    color,
  }: {
    icon: React.ElementType;
    title: string;
    desc: string;
    color?: string;
  }) => (
    <motion.div
      onClick={() => navigate("/notary/create")}
      whileTap={{ scale: 0.98 }}
      className="bg-chat-other-bg rounded-2xl p-4 shadow-sm border border-border-color flex items-center gap-4 cursor-pointer"
    >
      <div
        className={`w-12 h-12 rounded-full flex items-center justify-center bg-black/5 dark:bg-white/5`}
      >
        <Icon className={`w-6 h-6 ${color}`} />
      </div>
      <div className="flex-1">
        <h3 className="text-[16px] font-bold text-text-main">{title}</h3>
        <p className="text-[13px] text-text-sub mt-0.5">{desc}</p>
      </div>
    </motion.div>
  );

  return (
    <div className="flex flex-col h-full bg-bg-color">
      {/* Header */}
      <header className="h-[44px] flex items-center justify-between glass-header sticky top-0 z-10 shrink-0 pt-safe px-1 relative">
        <div className="flex items-center z-10 flex-1">
          <IconButton
            icon={
              <ChevronLeft
                className="w-6 h-6 text-text-main"
                strokeWidth={2.5}
              />
            }
            onClick={() => navigate(-1)}
          />
        </div>
        <div className="absolute left-1/2 -translate-x-1/2 flex items-center justify-center pointer-events-none">
          <h1 className="text-[17px] font-semibold text-text-main">公证业务</h1>
        </div>
        <div className="flex items-center justify-end z-10 flex-1 pr-1">
          <IconButton icon={<Info className="w-5 h-5 text-text-main" />} />
        </div>
      </header>

      {/* Content */}
      <div className="flex-1 overflow-y-auto px-4 pt-4 pb-12 flex flex-col gap-4">
        <div className="rounded-2xl p-5 bg-gradient-to-br from-blue-500 to-indigo-600 text-white shadow-md relative overflow-hidden">
          <div className="relative z-10">
            <h2 className="text-[20px] font-bold mb-1">在线公证服务</h2>
            <p className="text-[14px] text-white/80">
              高效便捷、具备法律效力的电子公证
            </p>
          </div>
          <div className="absolute right-[-20px] top-[-20px] w-32 h-32 bg-white/10 rounded-full blur-2xl" />
        </div>

        <h2 className="text-[15px] font-bold text-text-main mt-4 px-1">
          办理业务
        </h2>
        <div className="flex flex-col gap-3">
          <ActionCard
            icon={FileText}
            title="合同审查与公证"
            desc="保障商业合同法律效力"
            color="text-blue-500"
          />
          <ActionCard
            icon={UploadCloud}
            title="电子证据存证"
            desc="一键上传，司法链上固证"
            color="text-indigo-500"
          />
          <ActionCard
            icon={CheckCircle2}
            title="资质认证审核"
            desc="企业及个人资质在线核实"
            color="text-green-500"
          />
        </div>
      </div>
    </div>
  );
};
