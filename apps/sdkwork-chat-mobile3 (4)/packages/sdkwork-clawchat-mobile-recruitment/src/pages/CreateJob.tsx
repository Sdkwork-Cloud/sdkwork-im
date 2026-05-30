import React, { useState } from "react";
import {
  showPrompt,
  PageLayout,
  showToast,
} from "@sdkwork/clawchat-mobile-commons";
import {
  Briefcase,
  Building,
  MapPin,
  DollarSign,
  GraduationCap,
  Clock,
} from "lucide-react";
import { useNavigate } from "react-router";

export const CreateJob = () => {
  const navigate = useNavigate();
  const [formData, setFormData] = useState({
    title: "",
    department: "",
    location: "深圳",
    salary: "",
    experience: "不限",
    education: "本科",
  });

  const handleSubmit = () => {
    if (!formData.title) return showToast("请输入职位名称");
    if (!formData.department) return showToast("请输入所属部门");

    showToast("发布成功");
    navigate(-1);
  };

  return (
    <PageLayout
      title="发布职位"
      rightElement={
        <span
          className="text-[16px] text-accent-blue font-medium active:opacity-60 cursor-pointer"
          onClick={handleSubmit}
        >
          发布
        </span>
      }
    >
      <div className="p-4 space-y-4">
        {/* Basic Info */}
        <div className="bg-white dark:bg-[#2c2d2e] rounded-xl overflow-hidden shadow-sm border border-border-color/30">
          <div className="flex items-center px-4 py-3.5 border-b border-border-color/30">
            <Briefcase className="w-5 h-5 text-text-sub mr-3 shrink-0" />
            <input
              type="text"
              className="flex-1 bg-transparent border-none outline-none text-[16px] text-text-main placeholder:text-text-sub/50"
              placeholder="职位名称 (如: 高级前端开发工程师)"
              value={formData.title}
              onChange={(e) =>
                setFormData({ ...formData, title: e.target.value })
              }
            />
          </div>
          <div className="flex items-center px-4 py-3.5 border-b border-border-color/30">
            <Building className="w-5 h-5 text-text-sub mr-3 shrink-0" />
            <input
              type="text"
              className="flex-1 bg-transparent border-none outline-none text-[16px] text-text-main placeholder:text-text-sub/50"
              placeholder="所属部门"
              value={formData.department}
              onChange={(e) =>
                setFormData({ ...formData, department: e.target.value })
              }
            />
          </div>
          <div className="flex items-center px-4 py-3.5 border-b border-border-color/30">
            <MapPin className="w-5 h-5 text-text-sub mr-3 shrink-0" />
            <input
              type="text"
              className="flex-1 bg-transparent border-none outline-none text-[16px] text-text-main placeholder:text-text-sub/50"
              placeholder="工作城市"
              value={formData.location}
              onChange={(e) =>
                setFormData({ ...formData, location: e.target.value })
              }
            />
          </div>
          <div className="flex items-center px-4 py-3.5">
            <DollarSign className="w-5 h-5 text-text-sub mr-3 shrink-0" />
            <input
              type="text"
              className="flex-1 bg-transparent border-none outline-none text-[16px] text-text-main placeholder:text-text-sub/50"
              placeholder="薪资范围 (如: 15k-25k)"
              value={formData.salary}
              onChange={(e) =>
                setFormData({ ...formData, salary: e.target.value })
              }
            />
          </div>
        </div>

        {/* Requirements */}
        <div className="bg-white dark:bg-[#2c2d2e] rounded-xl overflow-hidden shadow-sm border border-border-color/30">
          <div
            className="flex items-center justify-between px-4 py-3.5 border-b border-border-color/30 cursor-pointer active:bg-bg-color"
            onClick={async () => {
              const exp = await showPrompt(
                "请输入经验要求",
                formData.experience,
              );
              if (exp) setFormData({ ...formData, experience: exp });
            }}
          >
            <div className="flex items-center">
              <Clock className="w-5 h-5 text-text-sub mr-3 shrink-0" />
              <span className="text-[16px] text-text-main font-medium">
                经验要求
              </span>
            </div>
            <span className="text-text-sub">{formData.experience} &gt;</span>
          </div>
          <div
            className="flex items-center justify-between px-4 py-3.5 cursor-pointer active:bg-bg-color"
            onClick={async () => {
              const edu = await showPrompt(
                "请输入学历要求",
                formData.education,
              );
              if (edu) setFormData({ ...formData, education: edu });
            }}
          >
            <div className="flex items-center">
              <GraduationCap className="w-5 h-5 text-text-sub mr-3 shrink-0" />
              <span className="text-[16px] text-text-main font-medium">
                学历要求
              </span>
            </div>
            <span className="text-text-sub">{formData.education} &gt;</span>
          </div>
        </div>

        {/* Job Description */}
        <div className="bg-white dark:bg-[#2c2d2e] rounded-xl p-4 shadow-sm border border-border-color/30">
          <textarea
            className="w-full bg-transparent border-none outline-none text-[15px] text-text-main placeholder:text-text-sub/50 resize-none h-40"
            placeholder="请输入职位描述和任职要求..."
          />
        </div>
      </div>
    </PageLayout>
  );
};
