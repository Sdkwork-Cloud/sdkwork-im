import React, { useState, useEffect } from "react";
import { PageLayout, IconButton, cn } from "@sdkwork/clawchat-mobile-commons";
import {
  Search,
  Filter,
  Plus,
  FileText,
  CheckCircle2,
  XCircle,
  Clock,
  Plane,
  ShoppingCart,
  UserCheck,
  ChevronRight,
} from "lucide-react";
import {
  ApprovalService,
  ApprovalItem,
  ApprovalStatus,
} from "../services/ApprovalService";
import { motion } from "motion/react";
import { useNavigate } from "react-router";

export const ApprovalApp = () => {
  const navigate = useNavigate();
  const [activeTab, setActiveTab] = useState<
    "待我审批" | "我发起的" | "抄送我的"
  >("待我审批");
  const [approvals, setApprovals] = useState<ApprovalItem[]>([]);
  const [isLoading, setIsLoading] = useState(true);

  useEffect(() => {
    setIsLoading(true);
    ApprovalService.getApprovals().then((data) => {
      setApprovals(data);
      setIsLoading(false);
    });
  }, []);

  const getStatusIcon = (status: ApprovalStatus) => {
    switch (status) {
      case "pending":
        return <Clock className="w-4 h-4 text-orange-500" />;
      case "approved":
        return <CheckCircle2 className="w-4 h-4 text-emerald-500" />;
      case "rejected":
        return <XCircle className="w-4 h-4 text-rose-500" />;
      case "withdrawn":
        return <XCircle className="w-4 h-4 text-gray-400" />;
    }
  };

  const getStatusText = (status: ApprovalStatus) => {
    switch (status) {
      case "pending":
        return "待审批";
      case "approved":
        return "已同意";
      case "rejected":
        return "已拒绝";
      case "withdrawn":
        return "已撤回";
    }
  };

  const getTypeIcon = (type: string) => {
    switch (type) {
      case "请假":
        return <UserCheck className="w-5 h-5 text-indigo-500" />;
      case "报销":
        return <Plane className="w-5 h-5 text-blue-500" />;
      case "采购":
        return <ShoppingCart className="w-5 h-5 text-orange-500" />;
      default:
        return <FileText className="w-5 h-5 text-primary-blue" />;
    }
  };

  return (
    <PageLayout title="审批">
      <div className="flex flex-col h-full bg-[#f5f6f8] dark:bg-[#1a1b1c]">
        {/* Header Stats */}
        <div className="bg-primary-blue px-6 pt-4 pb-12 flex justify-between items-center text-white">
          <div>
            <div className="text-[32px] font-medium tracking-tight leading-none mb-1">
              12
            </div>
            <div className="text-[13px] opacity-80">待处理审批 (件)</div>
          </div>
          <div className="flex gap-4">
            <div className="flex flex-col items-center">
              <div className="text-[20px] font-medium leading-none mb-1">3</div>
              <div className="text-[12px] opacity-80">我发起的</div>
            </div>
            <div className="flex flex-col items-center">
              <div className="text-[20px] font-medium leading-none mb-1">8</div>
              <div className="text-[12px] opacity-80">抄送我的</div>
            </div>
          </div>
        </div>

        <div className="flex-1 overflow-y-auto px-4 -mt-6">
          <div className="flex bg-white dark:bg-[#2c2d2e] rounded-xl shadow-sm mb-4 px-2 py-1">
            {["待我审批", "我发起的", "抄送我的"].map((tab) => (
              <button
                key={tab}
                className={cn(
                  "flex-1 text-[15px] py-2.5 relative text-center transition-colors rounded-lg",
                  activeTab === tab
                    ? "text-primary-blue font-medium bg-primary-blue/5"
                    : "text-text-sub",
                )}
                onClick={() => setActiveTab(tab as any)}
              >
                {tab}
              </button>
            ))}
          </div>

          <div className="flex justify-between items-center mb-3 px-1">
            <h2 className="text-[14px] font-medium text-text-sub">
              全部记录 ({approvals.length})
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
            ) : approvals.length > 0 ? (
              approvals.map((approval) => (
                <motion.div
                  key={approval.id}
                  whileTap={{ scale: 0.98 }}
                  onClick={() => navigate(`/workspace/approval/${approval.id}`)}
                  className="bg-white dark:bg-[#2c2d2e] p-4 rounded-xl cursor-pointer shadow-sm border border-border-color/30"
                >
                  <div className="flex justify-between items-start mb-3">
                    <div className="flex items-center gap-3">
                      <div className="w-10 h-10 rounded-xl bg-gray-100 dark:bg-[#3a3b3c] flex items-center justify-center">
                        {getTypeIcon(approval.type)}
                      </div>
                      <div>
                        <div className="text-[16px] font-medium text-text-main leading-tight mb-1">
                          {approval.applicant} 的 {approval.type}
                        </div>
                        <div className="text-[13px] text-text-sub font-mono">
                          {approval.date}
                        </div>
                      </div>
                    </div>
                    <div className="flex flex-col items-end">
                      <div className="flex items-center gap-1.5 text-[14px] font-medium mb-1">
                        {getStatusIcon(approval.status)}
                        <span
                          className={cn(
                            approval.status === "pending" && "text-orange-500",
                            approval.status === "approved" &&
                              "text-emerald-500",
                            approval.status === "rejected" && "text-rose-500",
                            approval.status === "withdrawn" && "text-gray-400",
                          )}
                        >
                          {getStatusText(approval.status)}
                        </span>
                      </div>
                    </div>
                  </div>
                  <div className="text-[14px] text-text-main bg-[#f8f9fa] dark:bg-[#202122] p-3 rounded-lg flex items-center justify-between">
                    <span className="truncate pr-4">{approval.title}</span>
                    <ChevronRight className="w-4 h-4 text-text-sub shrink-0" />
                  </div>
                </motion.div>
              ))
            ) : (
              <div className="flex flex-col items-center justify-center py-20 text-text-sub opacity-70">
                <FileText className="w-12 h-12 mb-3 stroke-current opacity-40" />
                <span className="text-[14px]">暂无审批记录</span>
              </div>
            )}
          </div>
        </div>

        <motion.button
          whileTap={{ scale: 0.9 }}
          whileHover={{ scale: 1.05 }}
          onClick={() => navigate("/workspace/approval/create")}
          className="absolute bottom-6 right-6 w-14 h-14 bg-gradient-to-tr from-blue-600 to-primary-blue text-white rounded-full flex items-center justify-center shadow-lg shadow-blue-500/30 z-10"
        >
          <Plus className="w-7 h-7" />
        </motion.button>
      </div>
    </PageLayout>
  );
};
