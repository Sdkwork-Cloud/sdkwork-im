import React, { useState } from "react";
import { PageLayout, Group, ListItem } from "../../components/SettingsCommons";
import { Check } from "lucide-react";
import { showPrompt, showToast } from "@sdkwork/clawchat-mobile-commons";
import { useNavigate } from "react-router";

export const Gender = () => {
  const [gender, setGender] = useState("男");
  return (
    <PageLayout title="设置性别">
      <Group>
        <div
          onClick={() => setGender("男")}
          className="flex items-center justify-between px-4 py-3.5 bg-chat-other-bg border-b border-border-color/60 cursor-pointer"
        >
          <span className="text-[16px] text-text-main">男</span>
          {gender === "男" && <Check className="w-5 h-5 text-[#00B42A]" />}
        </div>
        <div
          onClick={() => setGender("女")}
          className="flex items-center justify-between px-4 py-3.5 bg-chat-other-bg cursor-pointer"
        >
          <span className="text-[16px] text-text-main">女</span>
          {gender === "女" && <Check className="w-5 h-5 text-[#00B42A]" />}
        </div>
      </Group>
    </PageLayout>
  );
};

export const Region = () => (
  <PageLayout title="设置地区">
    <Group>
      <ListItem label="中国大陆" rightText="北京" hideBorder />
    </Group>
  </PageLayout>
);

export const Signature = () => (
  <PageLayout title="个性签名">
    <div className="p-4">
      <textarea
        className="w-full h-32 bg-chat-other-bg p-4 rounded-xl text-text-main outline-none resize-none"
        placeholder="介绍一下自己吧..."
      ></textarea>
      <button
        className="mt-6 w-full h-12 bg-[#00B42A] text-white rounded-lg font-medium active:opacity-80 transition-opacity"
        onClick={() => showToast("保存成功")}
      >
        保存
      </button>
    </div>
  </PageLayout>
);
