import React from "react";
import { useNavigate } from "react-router";
import { PageLayout, Group, ListItem } from "../../components/SettingsCommons";

export const HelpFeedback = () => {
  const navigate = useNavigate();
  return (
    <PageLayout title="帮助与反馈">
      <Group>
        <ListItem
          label="常见问题"
          onClick={() => navigate("/settings/help/faq")}
        />
        <ListItem
          label="意见反馈"
          hideBorder
          onClick={() => navigate("/settings/help/feedback")}
        />
      </Group>
    </PageLayout>
  );
};

export const About = () => {
  const navigate = useNavigate();
  return (
    <PageLayout title="关于 ClawChat">
      <div className="flex flex-col items-center py-10">
        <div className="w-20 h-20 bg-primary-blue rounded-2xl flex items-center justify-center mb-4 shadow-lg">
          <span className="text-white text-3xl font-bold">C</span>
        </div>
        <h3 className="text-[20px] font-bold text-text-main mb-1">ClawChat</h3>
        <p className="text-[14px] text-text-sub mb-8">Version 1.0.0</p>
      </div>
      <Group>
        <ListItem
          label="功能介绍"
          onClick={() => navigate("/settings/about/features")}
        />
        <ListItem
          label="投诉"
          onClick={() => navigate("/settings/about/complain")}
        />
        <ListItem
          label="软件许可及服务协议"
          onClick={() => navigate("/settings/about/tos")}
        />
        <ListItem
          label="隐私保护指引"
          hideBorder
          onClick={() => navigate("/settings/about/privacy")}
        />
      </Group>
    </PageLayout>
  );
};
