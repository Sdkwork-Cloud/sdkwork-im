import React, { useState, useEffect } from "react";
import {
  PageLayout,
  IconButton,
  cn,
  Avatar,
  showToast,
} from "@sdkwork/clawchat-mobile-commons";
import {
  Search,
  Filter,
  Plus,
  Briefcase,
  ChevronRight,
  User,
  GraduationCap,
  Clock,
} from "lucide-react";
import {
  RecruitmentService,
  CandidateRecord,
} from "../services/RecruitmentService";
import { motion } from "motion/react";
import { useNavigate } from "react-router";

export const RecruitmentApp = () => {
  const navigate = useNavigate();
  const [candidates, setCandidates] = useState<CandidateRecord[]>([]);
  const [isLoading, setIsLoading] = useState(true);

  useEffect(() => {
    setIsLoading(true);
    RecruitmentService.getCandidates().then((data) => {
      setCandidates(data);
      setIsLoading(false);
    });
  }, []);

  return (
    <PageLayout title="招聘管理">
      <div className="flex flex-col h-full bg-[#f5f6f8] dark:bg-[#1a1b1c]">
        {/* Header Stats */}
        <div className="bg-primary-blue px-6 pt-4 pb-12 flex justify-between items-center text-white">
          <div>
            <div className="text-[32px] font-medium tracking-tight leading-none mb-1">
              12
            </div>
            <div className="text-[13px] opacity-80">进行中候选人</div>
          </div>
          <div className="flex gap-4">
            <div className="flex flex-col items-center">
              <div className="text-[20px] font-medium leading-none mb-1">2</div>
              <div className="text-[12px] opacity-80">今日面试</div>
            </div>
            <div className="flex flex-col items-center">
              <div className="text-[20px] font-medium leading-none mb-1">5</div>
              <div className="text-[12px] opacity-80">待筛选</div>
            </div>
          </div>
        </div>

        <div className="flex-1 overflow-y-auto px-4 -mt-6">
          <div className="flex justify-between items-center mb-3 mt-4 px-1">
            <h2 className="text-[14px] font-medium text-text-sub">
              候选人列表 ({candidates.length})
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

          <div className="flex flex-col gap-3 pb-20">
            {isLoading ? (
              <div className="flex flex-col items-center justify-center py-20 text-text-sub opacity-70">
                <div className="w-8 h-8 rounded-full border-4 border-text-sub border-t-white animate-spin mb-3"></div>
                <span className="text-[14px]">加载中...</span>
              </div>
            ) : candidates.length > 0 ? (
              candidates.map((candidate) => (
                <motion.div
                  key={candidate.id}
                  whileTap={{ scale: 0.98 }}
                  onClick={() =>
                    navigate(`/workspace/recruitment/${candidate.id}`)
                  }
                  className="bg-white dark:bg-[#2c2d2e] p-4 rounded-xl cursor-pointer shadow-sm border border-border-color/30"
                >
                  <div className="flex justify-between items-start mb-3">
                    <div className="flex items-center gap-3">
                      <Avatar
                        src={candidate.avatar}
                        fallback={candidate.name.charAt(0)}
                        size="md"
                      />
                      <div>
                        <div className="text-[16px] font-medium text-text-main leading-tight mb-1">
                          {candidate.name}
                        </div>
                        <div className="text-[13px] text-text-sub flex items-center gap-2">
                          <span>{candidate.experience}</span>
                          <span className="w-1 h-1 bg-border-color rounded-full" />
                          <span>{candidate.education}</span>
                        </div>
                      </div>
                    </div>
                    <div className="flex flex-col items-end">
                      <span className="text-[14px] font-medium text-primary-blue mb-1">
                        {candidate.stage}
                      </span>
                    </div>
                  </div>

                  <div className="text-[14px] text-text-main bg-[#f8f9fa] dark:bg-[#202122] p-3 rounded-lg flex flex-col gap-2">
                    <div className="flex items-center gap-2 text-[13px]">
                      <Briefcase className="w-4 h-4 text-text-sub" />
                      <span>投递：{candidate.jobTitle}</span>
                    </div>
                    <div className="flex items-center gap-2 text-[13px]">
                      <Clock className="w-4 h-4 text-text-sub" />
                      <span className="text-orange-600 dark:text-orange-400">
                        {candidate.date}
                      </span>
                    </div>
                  </div>
                </motion.div>
              ))
            ) : (
              <div className="flex flex-col items-center justify-center py-20 text-text-sub opacity-70">
                <Briefcase className="w-12 h-12 mb-3 stroke-current opacity-40" />
                <span className="text-[14px]">暂无招聘候选人信息</span>
              </div>
            )}
          </div>
        </div>

        <motion.button
          whileTap={{ scale: 0.9 }}
          whileHover={{ scale: 1.05 }}
          onClick={() => navigate("/workspace/recruitment/create")}
          className="absolute bottom-6 right-6 w-14 h-14 bg-gradient-to-tr from-blue-600 to-primary-blue text-white rounded-full flex items-center justify-center shadow-lg shadow-blue-500/30 z-10"
        >
          <Plus className="w-7 h-7" />
        </motion.button>
      </div>
    </PageLayout>
  );
};
