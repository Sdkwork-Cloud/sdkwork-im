import React, { useEffect, useState } from "react";
import { PageLayout, showToast, cn } from "@sdkwork/clawchat-mobile-commons";
import { useNavigate, useParams } from "react-router";
import { ApprovalService, ApprovalItem } from "../services/ApprovalService";
import { CheckCircle2, XCircle, Clock, Check, X } from "lucide-react";

export const ApprovalDetail = () => {
  const { id } = useParams();
  const navigate = useNavigate();
  const [approval, setApproval] = useState<ApprovalItem | null>(null);
  const [submitting, setSubmitting] = useState(false);

  useEffect(() => {
    if (id) {
      ApprovalService.getApprovalDetail(id).then(setApproval);
    }
  }, [id]);

  const handleAction = async (action: "approve" | "reject") => {
    if (!id) return;
    setSubmitting(true);
    try {
      await ApprovalService.handleApproval({ id, action, comment: "" });
      showToast(action === "approve" ? "已同意" : "已拒绝");
      navigate(-1);
    } catch (e) {
      showToast("操作失败");
    } finally {
      setSubmitting(false);
    }
  };

  if (!approval)
    return (
      <PageLayout title="审批详情">
        <div className="flex flex-col h-full items-center justify-center text-text-sub opacity-70">
          <div className="w-8 h-8 rounded-full border-4 border-text-sub border-t-transparent animate-spin mb-3"></div>
          <span className="text-[14px]">加载中...</span>
        </div>
      </PageLayout>
    );

  return (
    <PageLayout title="审批详情">
      <div className="flex flex-col h-full bg-bg-color overflow-y-auto pb-[100px]">
        <div className="bg-white dark:bg-[#1a1b1c] p-5 pb-6 border-b border-border-color/30">
          <div className="flex items-center justify-between mb-4">
            <div className="flex items-center gap-3">
              <div className="w-12 h-12 rounded-xl bg-primary-blue/10 flex items-center justify-center text-primary-blue text-[18px] font-medium">
                {approval.applicant.charAt(0)}
              </div>
              <div>
                <div className="text-[16px] font-medium text-text-main leading-tight mb-1">
                  {approval.applicant}
                </div>
                <div className="text-[13px] text-text-sub">
                  {approval.department}
                </div>
              </div>
            </div>

            <span
              className={cn(
                "text-[14px] font-medium px-3 py-1 rounded-full",
                approval.status === "pending" && "bg-orange-50 text-orange-500",
                approval.status === "approved" &&
                  "bg-emerald-50 text-emerald-500",
                approval.status === "rejected" && "bg-rose-50 text-rose-500",
              )}
            >
              {approval.status === "pending"
                ? "待审批"
                : approval.status === "approved"
                  ? "已同意"
                  : "已拒绝"}
            </span>
          </div>

          <h2 className="text-[16px] font-medium text-text-main leading-relaxed border-t border-border-color/30 pt-4 mb-2">
            {approval.title}
          </h2>
          <div className="text-[15px] text-text-main/80 leading-relaxed whitespace-pre-wrap">
            {approval.content}
          </div>
        </div>

        <div className="mt-4 px-4">
          <h3 className="text-[14px] font-medium text-text-sub mb-4">
            审批流程
          </h3>
          <div className="flex flex-col gap-5 pl-2 relative border-l-2 border-gray-200 dark:border-gray-800 ml-4 pb-4">
            <div className="relative">
              <div className="absolute -left-[19px] w-8 h-8 rounded-full bg-blue-500 flex items-center justify-center text-white text-[12px] shadow-sm">
                发
              </div>
              <div className="pl-6">
                <div className="flex justify-between items-start mb-1">
                  <span className="text-[15px] font-medium text-text-main">
                    {approval.applicant} (发起申请)
                  </span>
                  <span className="text-[12px] text-text-sub">
                    {approval.date}
                  </span>
                </div>
              </div>
            </div>

            {approval.history.map((record, i) => (
              <div key={i} className="relative mt-5">
                <div className={cn(
                  "absolute -left-[19px] w-8 h-8 rounded-full flex items-center justify-center text-white text-[12px] shadow-sm",
                  record.action === "reject" ? "bg-rose-500" : "bg-emerald-500"
                )}>
                  {record.action === "reject" ? <X className="w-4 h-4" /> : <Check className="w-4 h-4" />}
                </div>
                <div className="pl-6">
                  <div className="flex justify-between items-start mb-1">
                    <span className="text-[15px] font-medium text-text-main">
                      {record.name} {record.action === "reject" ? "(已拒绝)" : "(已同意)"}
                    </span>
                    <span className="text-[12px] text-text-sub">
                      {record.actionTime}
                    </span>
                  </div>
                  {record.comment && (
                    <div className="text-[14px] text-text-sub mt-1 bg-gray-50 dark:bg-gray-800 p-2 rounded">
                      {record.comment}
                    </div>
                  )}
                </div>
              </div>
            ))}

            {approval.status === "pending" && (
              <div className="relative mt-5">
                <div className="absolute -left-[19px] w-8 h-8 rounded-full bg-orange-400 flex items-center justify-center text-white text-[12px] shadow-sm">
                  <Clock className="w-4 h-4" />
                </div>
                <div className="pl-6">
                  <div className="flex justify-between items-start mb-1">
                    <span className="text-[15px] font-medium text-orange-500">
                      当前审批轮到你
                    </span>
                  </div>
                </div>
              </div>
            )}
          </div>
        </div>
      </div>

      {approval.status === "pending" && (
        <div className="absolute bottom-0 left-0 right-0 bg-white dark:bg-[#1a1b1c] border-t border-border-color/30 p-4 pb-safe flex gap-3">
          <button
            className="flex-1 bg-white border border-rose-500 text-rose-500 rounded-lg py-3 font-medium active:bg-rose-50 dark:bg-transparent dark:active:bg-rose-500/10 disabled:opacity-50"
            disabled={submitting}
            onClick={() => handleAction("reject")}
          >
            拒绝
          </button>
          <button
            className="flex-1 bg-primary-blue text-white rounded-lg py-3 font-medium active:bg-primary-blue/90 disabled:opacity-50"
            disabled={submitting}
            onClick={() => handleAction("approve")}
          >
            同意
          </button>
        </div>
      )}
    </PageLayout>
  );
};
