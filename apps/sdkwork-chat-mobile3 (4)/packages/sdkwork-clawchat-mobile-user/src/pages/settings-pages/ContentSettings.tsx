import React, { useState, useEffect } from "react";
import { useNavigate } from "react-router";
import { SettingsService } from "../../services/SettingsService";
import {
  PageLayout,
  Group,
  ListItem,
  ToggleItem,
} from "../../components/SettingsCommons";

export const Notifications = () => {
  const [sound, setSound] = useState(true);
  const [vibrate, setVibrate] = useState(true);
  const [preview, setPreview] = useState(true);
  const [newMsg, setNewMsg] = useState(true);
  const [callInvite, setCallInvite] = useState(true);

  return (
    <PageLayout title="新消息通知">
      <Group>
        <ToggleItem
          label="接收新消息通知"
          checked={newMsg}
          onChange={setNewMsg}
        />
        <ToggleItem
          label="接收语音和视频通话邀请通知"
          checked={callInvite}
          onChange={setCallInvite}
          hideBorder
        />
      </Group>
      <Group>
        <ToggleItem
          label="通知显示消息详情"
          checked={preview}
          onChange={setPreview}
          hideBorder
        />
      </Group>
      <Group>
        <ToggleItem label="声音" checked={sound} onChange={setSound} />
        <ToggleItem
          label="震动"
          checked={vibrate}
          onChange={setVibrate}
          hideBorder
        />
      </Group>
    </PageLayout>
  );
};

export const ChatSettings = () => {
  const navigate = useNavigate();
  return (
    <PageLayout title="聊天">
      <Group>
        <ListItem
          label="聊天背景"
          hideBorder
          onClick={() => navigate("/settings/chat/background")}
        />
      </Group>
      <Group>
        <ListItem
          label="表情管理"
          hideBorder
          onClick={() => navigate("/settings/chat/emoji")}
        />
      </Group>
      <Group>
        <ListItem
          label="清空聊天记录"
          hideBorder
          onClick={() => navigate("/settings/chat/clear")}
        />
      </Group>
    </PageLayout>
  );
};

export const General = () => {
  const navigate = useNavigate();
  const [landscape, setLandscape] = useState(false);
  const [isDark, setIsDark] = useState(true);

  useEffect(() => {
    SettingsService.getSettings().then((s) => setLandscape(s.landscape));
    const isDarkMode = document.documentElement.classList.contains("dark");
    setIsDark(isDarkMode);
  }, []);

  const handleLandscapeToggle = async (val: boolean) => {
    setLandscape(val);
    await SettingsService.updateSettings({ landscape: val });
  };

  const handleThemeToggle = (checked: boolean) => {
    setIsDark(checked);
    if (checked) {
      document.documentElement.classList.add("dark");
    } else {
      document.documentElement.classList.remove("dark");
    }
  };

  return (
    <PageLayout title="通用">
      <Group>
        <ToggleItem
          label="深色模式"
          checked={isDark}
          onChange={handleThemeToggle}
        />
        <ListItem label="多语言" rightText="简体中文" />
        <ListItem
          label="字体大小"
          hideBorder
          onClick={() => navigate("/settings/general/font-size")}
        />
      </Group>
      <Group>
        <ListItem
          label="照片、视频、文件和通话"
          hideBorder
          onClick={() => navigate("/settings/general/media")}
        />
      </Group>
      <Group>
        <ToggleItem
          label="开启横屏模式"
          checked={landscape}
          onChange={handleLandscapeToggle}
          hideBorder
        />
      </Group>
      <Group>
        <ListItem
          label="存储空间"
          hideBorder
          onClick={() => navigate("/settings/general/storage")}
        />
      </Group>
    </PageLayout>
  );
};

export const Plugins = () => {
  const [kanYiKan, setKanYiKan] = useState(true);
  const [souYiSou, setSouYiSou] = useState(true);
  return (
    <PageLayout title="插件">
      <Group>
        <div className="flex items-center px-4 py-3.5 bg-chat-other-bg border-b border-border-color/60">
          <div className="w-10 h-10 bg-green-500 rounded-lg flex items-center justify-center mr-3">
            <span className="text-white text-xl">📰</span>
          </div>
          <div className="flex-1">
            <h4 className="text-[16px] text-text-main">看一看</h4>
            <p className="text-[13px] text-text-sub">发现朋友关注的热点</p>
          </div>
          <div
            className={`w-12 h-6 rounded-full relative cursor-pointer transition-colors ${kanYiKan ? "bg-[#00B42A]" : "bg-gray-300"}`}
            onClick={() => setKanYiKan(!kanYiKan)}
          >
            <div
              className={`absolute top-1 w-4 h-4 rounded-full bg-white transition-transform ${kanYiKan ? "left-7" : "left-1"}`}
            />
          </div>
        </div>
        <div className="flex items-center px-4 py-3.5 bg-chat-other-bg">
          <div className="w-10 h-10 bg-orange-500 rounded-lg flex items-center justify-center mr-3">
            <span className="text-white text-xl">🔍</span>
          </div>
          <div className="flex-1">
            <h4 className="text-[16px] text-text-main">搜一搜</h4>
            <p className="text-[13px] text-text-sub">搜索文章、小程序等</p>
          </div>
          <div
            className={`w-12 h-6 rounded-full relative cursor-pointer transition-colors ${souYiSou ? "bg-[#00B42A]" : "bg-gray-300"}`}
            onClick={() => setSouYiSou(!souYiSou)}
          >
            <div
              className={`absolute top-1 w-4 h-4 rounded-full bg-white transition-transform ${souYiSou ? "left-7" : "left-1"}`}
            />
          </div>
        </div>
      </Group>
    </PageLayout>
  );
};
