import React, { useState } from "react";
import {
  showPrompt,
  PageLayout,
  showToast,
  ActionSheet,
  cn,
} from "@sdkwork/clawchat-mobile-commons";
import { useNavigate } from "react-router";
import {
  MeetingService,
  CreateMeetingRequest,
} from "../services/MeetingService";
import { Users } from "lucide-react";

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

export const CreateMeeting = () => {
  const navigate = useNavigate();
  const [formData, setFormData] = useState<CreateMeetingRequest>({
    title: "",
    description: "",
    startTime: new Date().toISOString().slice(0, 16),
    endTime: new Date(Date.now() + 3600000).toISOString().slice(0, 16),
    roomId: "",
    attendeeIds: [],
  });
  const [loading, setLoading] = useState(false);

  const handleSubmit = async () => {
    if (!formData.title) return showToast("请输入会议主题");
    setLoading(true);
    try {
      await MeetingService.createMeeting(formData);
      showToast("会议预约成功");
      navigate(-1);
    } catch (e) {
      const error = e as Error;
      showToast(error.message || "预约失败");
    } finally {
      setLoading(false);
    }
  };

  return (
    <PageLayout title="预约会议">
      <div className="flex flex-col h-full bg-bg-color overflow-y-auto pb-8">
        <div className="bg-white dark:bg-[#1a1b1c] mt-2 border-y border-border-color/30">
          <FormItem label="会议主题" required>
            <input
              type="text"
              placeholder="请输入会议主题"
              className="w-full text-[16px] bg-transparent outline-none py-1"
              value={formData.title}
              onChange={(e) =>
                setFormData((s) => ({ ...s, title: e.target.value }))
              }
            />
          </FormItem>

          <FormItem label="开始时间" required>
            <input
              type="datetime-local"
              className="w-full text-[16px] bg-transparent outline-none py-1 text-text-main"
              value={formData.startTime}
              onChange={(e) =>
                setFormData((s) => ({ ...s, startTime: e.target.value }))
              }
            />
          </FormItem>

          <FormItem label="结束时间" required>
            <input
              type="datetime-local"
              className="w-full text-[16px] bg-transparent outline-none py-1 text-text-main"
              value={formData.endTime}
              onChange={(e) =>
                setFormData((s) => ({ ...s, endTime: e.target.value }))
              }
            />
          </FormItem>

          <FormItem label="会议内容">
            <textarea
              placeholder="请输入会议内容与议程描述"
              className="w-full text-[16px] bg-transparent outline-none py-1 min-h-[80px]"
              value={formData.description}
              onChange={(e) =>
                setFormData((s) => ({ ...s, description: e.target.value }))
              }
            />
          </FormItem>
        </div>

        <div className="bg-white dark:bg-[#1a1b1c] mt-2 border-y border-border-color/30">
          <FormItem
            label="会议会议室"
            onClick={async () => {
              const room = await showPrompt(
                "请输入会议室名称",
                formData.roomId,
              );
              if (room) setFormData((s) => ({ ...s, roomId: room }));
            }}
          >
            <div className="flex justify-between items-center w-full">
              <span
                className={formData.roomId ? "text-text-main" : "text-text-sub"}
              >
                {formData.roomId || "请选择"}
              </span>
            </div>
          </FormItem>
        </div>

        <div className="bg-white dark:bg-[#1a1b1c] mt-2 border-y border-border-color/30 p-4">
          <div className="text-[15px] text-text-main font-medium mb-3">
            参会人
          </div>
          <div className="flex gap-2 flex-wrap items-center">
            {formData.attendeeIds?.map((attendee, i) => (
              <div key={i} className="relative group">
                <div className="w-12 h-12 rounded-full bg-primary-blue/10 text-primary-blue flex flex-col items-center justify-center text-[10px] whitespace-nowrap overflow-hidden text-ellipsis shadow-sm ring-1 ring-primary-blue/20">
                  {attendee.slice(0, 2)}
                </div>
                <div
                  className="absolute -top-1 -right-1 bg-red-500 rounded-full w-4 h-4 flex items-center justify-center text-white cursor-pointer"
                  onClick={() =>
                    setFormData((s) => ({
                      ...s,
                      attendeeIds: s.attendeeIds?.filter(
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
              className="w-12 h-12 rounded-full bg-bg-color flex items-center justify-center cursor-pointer border border-dashed border-border-color shrink-0"
              onClick={async () => {
                const name = await showPrompt("请输入参会人姓名");
                if (name && name.trim()) {
                  setFormData((s) => ({
                    ...s,
                    attendeeIds: [...(s.attendeeIds || []), name.trim()],
                  }));
                  showToast(`已添加联系人: ${name}`);
                }
              }}
            >
              <Users className="w-5 h-5 text-text-sub" />
            </div>
          </div>
        </div>

        <div className="p-6 mt-8">
          <button
            className="w-full bg-primary-blue text-white rounded-lg py-3 font-medium active:bg-primary-blue/90"
            onClick={handleSubmit}
            disabled={loading}
          >
            {loading ? "提交中..." : "提交预约"}
          </button>
        </div>
      </div>
    </PageLayout>
  );
};
