import React from "react";
import {
  PageLayout,
  Avatar,
  cn,
  showToast,
} from "@sdkwork/clawchat-mobile-commons";
import {
  User,
  Briefcase,
  GraduationCap,
  MapPin,
  Calendar,
  Clock,
  Download,
  ChevronRight,
  Check,
  X,
} from "lucide-react";
import { useParams, useNavigate } from "react-router";
import { RecruitmentService } from "../services/RecruitmentService";

export const CandidateDetail = () => {
  const { id } = useParams();
  const navigate = useNavigate();
  const [candidate, setCandidate] = React.useState<any>(null);

  React.useEffect(() => {
    RecruitmentService.getCandidates().then((data) => {
      setCandidate(data.find((c) => c.id === id) || data[0]);
    });
  }, [id]);

  if (!candidate)
    return (
      <PageLayout title="候选人详情">
        <div className="flex flex-col h-full bg-bg-color items-center justify-center text-text-sub opacity-70">
          <div className="w-8 h-8 rounded-full border-4 border-text-sub border-t-transparent animate-spin mb-3"></div>
          <span className="text-[14px]">加载中...</span>
        </div>
      </PageLayout>
    );

  return (
    <PageLayout title="候选人详情">
      <div className="p-4">
        {/* Header */}
        <div className="bg-white dark:bg-[#2c2d2e] rounded-xl p-5 mb-4 shadow-sm border border-border-color/30 flex items-start gap-4">
          <Avatar
            src={candidate.avatar}
            fallback={candidate.name.substring(0, 1)}
            size="xl"
          />
          <div className="flex-1">
            <h2 className="text-xl font-bold text-text-main mb-1">
              {candidate.name}
            </h2>
            <div className="text-[15px] text-text-sub font-medium mb-2">
              {candidate.jobTitle}
            </div>
            <div className="flex flex-wrap gap-2">
              <span className="bg-bg-color px-2 py-1 rounded text-xs text-text-sub flex items-center gap-1">
                <Briefcase className="w-3 h-3" /> {candidate.experience}
              </span>
              <span className="bg-bg-color px-2 py-1 rounded text-xs text-text-sub flex items-center gap-1">
                <GraduationCap className="w-3 h-3" /> {candidate.education}
              </span>
            </div>
          </div>
        </div>

        {/* Process */}
        <div className="bg-white dark:bg-[#2c2d2e] rounded-xl p-4 mb-4 shadow-sm border border-border-color/30">
          <h3 className="text-[15px] font-bold text-text-main mb-4 border-l-4 border-primary-blue pl-2 leading-tight">
            当前进度
          </h3>
          <div className="flex items-center gap-4">
            <div className="w-10 h-10 rounded-full bg-blue-50 dark:bg-blue-900/30 flex items-center justify-center shrink-0">
              <Clock className="w-5 h-5 text-primary-blue" />
            </div>
            <div className="flex-1">
              <div className="text-[15px] text-text-main font-medium">
                {candidate.stage}
              </div>
              <div className="text-[13px] text-text-sub mt-0.5">
                更新于: {candidate.date}
              </div>
            </div>
            <ChevronRight className="w-5 h-5 text-border-color" />
          </div>
        </div>

        {/* Base Info */}
        <div className="bg-white dark:bg-[#2c2d2e] rounded-xl p-4 mb-4 shadow-sm border border-border-color/30">
          <h3 className="text-[15px] font-bold text-text-main mb-4 border-l-4 border-primary-blue pl-2 leading-tight">
            基本信息
          </h3>
          <div className="space-y-4">
            <div className="flex items-center gap-3">
              <User className="w-5 h-5 text-text-sub" />
              <div className="flex-1 border-b border-border-color/50 pb-3 flex justify-between">
                <span className="text-text-main">年龄</span>
                <span className="text-text-sub">28岁</span>
              </div>
            </div>
            <div className="flex items-center gap-3">
              <MapPin className="w-5 h-5 text-text-sub" />
              <div className="flex-1 border-b border-border-color/50 pb-3 flex justify-between">
                <span className="text-text-main">居住地</span>
                <span className="text-text-sub">深圳市南山区</span>
              </div>
            </div>
            <div className="flex items-center gap-3">
              <Calendar className="w-5 h-5 text-text-sub" />
              <div className="flex-1 border-border-color/50 flex justify-between">
                <span className="text-text-main">到岗时间</span>
                <span className="text-text-sub">随时</span>
              </div>
            </div>
          </div>
        </div>

        {/* Resume */}
        <div
          className="bg-white dark:bg-[#2c2d2e] rounded-xl p-4 mb-6 shadow-sm border border-border-color/30 flex justify-between items-center active:scale-95 transition-transform"
          onClick={() => showToast("已开始下载简历附件")}
        >
          <div className="flex items-center gap-3">
            <div className="w-10 h-10 bg-primary-blue/10 rounded-lg flex items-center justify-center">
              <Download className="w-5 h-5 text-primary-blue" />
            </div>
            <div>
              <div className="text-[15px] font-medium text-text-main">
                查看完整简历
              </div>
              <div className="text-[12px] text-text-sub mt-0.5">
                PDF · 1.2MB
              </div>
            </div>
          </div>
          <ChevronRight className="w-4 h-4 text-text-sub" />
        </div>

        {/* Actions */}
        <div className="flex gap-3">
          <button
            className="flex-1 bg-white dark:bg-[#2c2d2e] border border-border-color text-text-main py-3 rounded-lg font-medium active:bg-bg-color flex justify-center items-center gap-2"
            onClick={async () => {
              await RecruitmentService.updateCandidateStage(
                candidate.id,
                "已淘汰",
              );
              showToast("已淘汰");
              navigate(-1);
            }}
          >
            <X className="w-4 h-4" /> 淘汰
          </button>
          <button
            className="flex-1 bg-primary-blue text-white py-3 rounded-lg font-medium active:opacity-90 flex justify-center items-center gap-2"
            onClick={async () => {
              await RecruitmentService.updateCandidateStage(
                candidate.id,
                "已推进下阶段",
              );
              showToast("已推进到下一阶段");
              navigate(-1);
            }}
          >
            <Check className="w-4 h-4" /> 推进下阶段
          </button>
        </div>
      </div>
    </PageLayout>
  );
};
