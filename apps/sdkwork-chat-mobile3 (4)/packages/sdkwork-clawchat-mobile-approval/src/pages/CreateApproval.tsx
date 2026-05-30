import React, { useState } from "react";
import {
  showPrompt,
  PageLayout,
  showToast,
  ActionSheet,
} from "@sdkwork/clawchat-mobile-commons";
import { useNavigate } from "react-router";
import {
  ApprovalService,
  SubmitApprovalRequest,
} from "../services/ApprovalService";
import { Users, FileImage } from "lucide-react";

const FormItem = ({
  label,
  children,
  required = false,
  onClick,
}: {
  label: string;
  children?: React.ReactNode;
  required?: boolean;
  onClick?: () => void;
}) => (
  <div
    className="flex items-center px-4 py-3 border-b border-border-color/30 last:border-b-0 bg-white dark:bg-[#1a1b1c] active:bg-gray-50 dark:active:bg-[#202122] transition-colors"
    onClick={onClick}
  >
    <div className="w-[80px] shrink-0 text-[15px] text-text-main flex items-center">
      {required && <span className="text-rose-500 mr-1">*</span>}
      {label}
    </div>
    <div className="flex-1 flex items-center min-w-0">{children}</div>
  </div>
);

export const CreateApproval = () => {
  const navigate = useNavigate();
  const [formData, setFormData] = useState<SubmitApprovalRequest>({
    title: "",
    type: "请假",
    content: "",
    approverIds: [],
    attachments: [],
  });
  const [isTypeSheetOpen, setIsTypeSheetOpen] = useState(false);
  const [loading, setLoading] = useState(false);

  const handleSubmit = async () => {
    if (!formData.title) return showToast("请输入审批标题");
    if (!formData.content) return showToast("请输入审批内容");
    setLoading(true);
    try {
      await ApprovalService.submitApproval(formData);
      showToast("提交成功");
      navigate(-1);
    } catch (e) {
      const error = e as Error;
      showToast(error.message || "提交失败");
    } finally {
      setLoading(false);
    }
  };

  return (
    <PageLayout title={`发起${formData.type}`}>
      <div className="flex flex-col h-full bg-bg-color overflow-y-auto pb-8">
        <div className="bg-white dark:bg-[#1a1b1c] mt-2 border-y border-border-color/30">
          <FormItem label="审批类型" onClick={() => setIsTypeSheetOpen(true)}>
            <div className="flex justify-between items-center w-full">
              <span className="text-text-main">{formData.type}</span>
            </div>
          </FormItem>
          <FormItem label="标题" required>
            <input
              type="text"
              placeholder="请输入标题 (如: 张三的请假申请)"
              className="w-full text-[16px] bg-transparent outline-none py-1 text-text-main"
              value={formData.title}
              onChange={(e) =>
                setFormData((s) => ({ ...s, title: e.target.value }))
              }
            />
          </FormItem>

          <FormItem label="详细内容" required>
            <textarea
              placeholder="请输入详细的审批内容、时长或报销明细..."
              className="w-full text-[16px] bg-transparent outline-none py-1 min-h-[100px] text-text-main"
              value={formData.content}
              onChange={(e) =>
                setFormData((s) => ({ ...s, content: e.target.value }))
              }
            />
          </FormItem>
        </div>

        <div className="bg-white dark:bg-[#1a1b1c] mt-2 border-y border-border-color/30 p-4">
          <div className="text-[15px] text-text-main font-medium mb-3">
            附件图片
          </div>
          <div className="flex gap-2 flex-wrap">
            {formData.attachments?.map((url, i) => (
              <div key={i} className="w-16 h-16 rounded-xl relative group">
                <img
                  src={url}
                  className="w-full h-full object-cover rounded-xl border border-border-color/20"
                />
                <div
                  className="absolute -top-2 -right-2 bg-red-500 rounded-full w-5 h-5 flex items-center justify-center text-white cursor-pointer"
                  onClick={() =>
                    setFormData((s) => ({
                      ...s,
                      attachments: s.attachments?.filter(
                        (_, index) => index !== i,
                      ),
                    }))
                  }
                >
                  <span className="text-xs font-bold leading-none">
                    &times;
                  </span>
                </div>
              </div>
            ))}
            <label className="w-16 h-16 rounded-xl bg-bg-color flex items-center justify-center cursor-pointer border border-dashed border-border-color relative">
              <FileImage className="w-6 h-6 text-text-sub" />
              <input
                type="file"
                className="hidden"
                accept="image/*"
                multiple
                onChange={(e) => {
                  const files = Array.from(e.target.files || []);
                  const urls = files.map((f) => window.URL.createObjectURL(f));
                  setFormData((s) => ({
                    ...s,
                    attachments: [...(s.attachments || []), ...urls],
                  }));
                }}
              />
            </label>
          </div>
        </div>

        <div className="bg-white dark:bg-[#1a1b1c] mt-2 border-y border-border-color/30 p-4">
          <div className="text-[15px] text-text-main font-medium mb-3">
            审批人
          </div>
          <div className="flex gap-2 flex-wrap items-center">
            {formData.approverIds?.map((approver, i) => (
              <div key={i} className="relative group">
                <div className="w-12 h-12 rounded-full bg-primary-blue/10 text-primary-blue flex flex-col items-center justify-center text-[10px] whitespace-nowrap overflow-hidden text-ellipsis shadow-sm ring-1 ring-primary-blue/20">
                  {approver.slice(0, 2)}
                </div>
                <div
                  className="absolute -top-1 -right-1 bg-red-500 rounded-full w-4 h-4 flex items-center justify-center text-white cursor-pointer"
                  onClick={() =>
                    setFormData((s) => ({
                      ...s,
                      approverIds: s.approverIds?.filter(
                        (_, index) => index !== i,
                      ),
                    }))
                  }
                >
                  <span className="text-[10px] font-bold leading-none">
                    &times;
                  </span>
                </div>
              </div>
            ))}
            <div
              className="w-12 h-12 rounded-full bg-bg-color flex items-center justify-center cursor-pointer border border-dashed border-border-color"
              onClick={async () => {
                const name = await showPrompt("请输入审批人姓名");
                if (name && name.trim()) {
                  setFormData((s) => ({
                    ...s,
                    approverIds: [...(s.approverIds || []), name.trim()],
                  }));
                  showToast(`已添加审批人: ${name}`);
                }
              }}
            >
              <Users className="w-5 h-5 text-text-sub" />
            </div>
          </div>
        </div>

        <div className="p-6 mt-4">
          <button
            className="w-full bg-primary-blue text-white rounded-lg py-3 font-medium active:bg-primary-blue/90"
            onClick={handleSubmit}
            disabled={loading}
          >
            {loading ? "提交中..." : "提交审批"}
          </button>
        </div>
      </div>

      <ActionSheet
        isOpen={isTypeSheetOpen}
        onClose={() => setIsTypeSheetOpen(false)}
        title="选择审批类型"
        options={[
          {
            label: "请假",
            onClick: () => setFormData((s) => ({ ...s, type: "请假" })),
          },
          {
            label: "报销",
            onClick: () => setFormData((s) => ({ ...s, type: "报销" })),
          },
          {
            label: "采购",
            onClick: () => setFormData((s) => ({ ...s, type: "采购" })),
          },
          {
            label: "用车",
            onClick: () => setFormData((s) => ({ ...s, type: "用车" })),
          },
          {
            label: "通用审批",
            onClick: () => setFormData((s) => ({ ...s, type: "通用审批" })),
          },
        ]}
      />
    </PageLayout>
  );
};
