import React, { useState, useEffect } from "react";
import { useNavigate } from "react-router";
import { PageLayout, Group, ListItem, ToggleItem } from "./SettingsSubPages";
import { SettingsService } from "../services/SettingsService";
import { showToast } from "@sdkwork/clawchat-mobile-commons";

export const ChatBackground = () => (
  <PageLayout title="聊天背景">
    <Group>
      <ListItem label="选择背景图" onClick={() => showToast("已应用")} />
      <ListItem label="从手机相册选择" onClick={() => showToast("已应用")} />
      <ListItem label="拍一张" hideBorder onClick={() => showToast("已应用")} />
    </Group>
    <Group>
      <ListItem
        label="将背景应用到所有聊天场景"
        hideBorder
        onClick={() => showToast("全局应用成功")}
      />
    </Group>
  </PageLayout>
);

export const EmojiManagement = () => {
  const navigate = useNavigate();
  return (
    <PageLayout title="表情管理">
      <div className="flex flex-col items-center py-20 bg-bg-color h-full">
        <div className="flex gap-2 mb-8">
          <div className="w-16 h-16 rounded-xl bg-chat-other-bg flex items-center justify-center text-3xl shadow-sm border border-border-color">
            😀
          </div>
          <div className="w-16 h-16 rounded-xl bg-chat-other-bg flex items-center justify-center text-3xl shadow-sm border border-border-color">
            🤣
          </div>
          <div className="w-16 h-16 rounded-xl bg-chat-other-bg flex items-center justify-center text-3xl shadow-sm border border-border-color">
            😎
          </div>
        </div>
        <p className="text-[15px] text-text-sub mb-8">
          管理已有表情或添加新表情
        </p>
        <button
          onClick={() => navigate("/me/emoji")}
          className="w-[200px] h-12 bg-primary-blue text-white rounded-full font-medium active:scale-95 transition-transform"
        >
          去表情商店发现更多
        </button>
      </div>
    </PageLayout>
  );
};

export const ClearChatHistory = () => {
  const navigate = useNavigate();
  return (
    <PageLayout title="清空聊天记录">
      <div className="flex flex-col items-center py-10 px-4">
        <p className="text-[15px] text-text-main text-center mb-8">
          将清空所有个人和群聊的聊天记录，此操作不可恢复。
        </p>
        <button
          className="w-full h-12 bg-[#FA5151] text-white rounded-lg font-medium active:opacity-80 transition-opacity"
          onClick={() => {
            showToast("已清空");
            navigate(-1);
          }}
        >
          清空全部聊天记录
        </button>
      </div>
    </PageLayout>
  );
};

export const FontSize = () => {
  const [fontSize, setFontSize] = useState(2);

  useEffect(() => {
    SettingsService.getSettings().then((s) => setFontSize(s.fontSize));
  }, []);

  const handleFontSizeChange = async (
    e: React.ChangeEvent<HTMLInputElement>,
  ) => {
    const val = parseInt(e.target.value, 10);
    setFontSize(val);
    await SettingsService.updateSettings({ fontSize: val });
  };

  return (
    <PageLayout title="字体大小">
      <div className="flex flex-col h-full">
        <div className="flex-1 p-6 flex flex-col gap-4">
          <div className="bg-chat-other-bg p-4 rounded-xl self-start max-w-[80%]">
            <p
              className="text-text-main"
              style={{ fontSize: `${14 + (fontSize - 2) * 2}px` }}
            >
              预览字体大小
            </p>
          </div>
          <div className="bg-primary-blue p-4 rounded-xl self-end max-w-[80%]">
            <p
              className="text-white"
              style={{ fontSize: `${14 + (fontSize - 2) * 2}px` }}
            >
              拖动下方滑块调整字体大小
            </p>
          </div>
        </div>
        <div className="bg-chat-other-bg p-8 border-t border-border-color">
          <div className="flex items-center justify-between text-text-main mb-4">
            <span className="text-[12px]">A</span>
            <span className="text-[16px]">标准</span>
            <span className="text-[24px]">A</span>
          </div>
          <input
            type="range"
            min="1"
            max="5"
            value={fontSize}
            onChange={handleFontSizeChange}
            className="w-full accent-[#00B42A]"
          />
        </div>
      </div>
    </PageLayout>
  );
};

export const MediaSettings = () => {
  const [autoDownload, setAutoDownload] = useState(true);
  const [savePhoto, setSavePhoto] = useState(true);
  const [saveVideo, setSaveVideo] = useState(true);

  useEffect(() => {
    SettingsService.getSettings().then((s) => {
      setAutoDownload(s.autoDownload);
      setSavePhoto(s.savePhoto);
      setSaveVideo(s.saveVideo);
    });
  }, []);

  const handleToggle = async (key: string, val: boolean) => {
    if (key === "autoDownload") setAutoDownload(val);
    if (key === "savePhoto") setSavePhoto(val);
    if (key === "saveVideo") setSaveVideo(val);
    await SettingsService.updateSettings({ [key]: val });
  };

  return (
    <PageLayout title="照片、视频、文件和通话">
      <Group>
        <ToggleItem
          label="自动下载"
          checked={autoDownload}
          onChange={(v: boolean) => handleToggle("autoDownload", v)}
          hideBorder
        />
      </Group>
      <p className="text-[13px] text-text-sub px-4 mb-4">
        在其他设备查看的照片、视频和文件在手机上自动下载。
      </p>
      <Group>
        <ToggleItem
          label="照片"
          checked={savePhoto}
          onChange={(v: boolean) => handleToggle("savePhoto", v)}
        />
        <ToggleItem
          label="视频"
          checked={saveVideo}
          onChange={(v: boolean) => handleToggle("saveVideo", v)}
          hideBorder
        />
      </Group>
      <p className="text-[13px] text-text-sub px-4 mb-4">
        拍摄或编辑后的照片和视频保存到系统相册。
      </p>
    </PageLayout>
  );
};

export const StorageSpace = () => {
  const navigate = useNavigate();
  const [cleared, setCleared] = useState(false);
  return (
    <PageLayout title="存储空间">
      <div className="flex flex-col items-center py-10 px-4">
        <div className="w-32 h-32 rounded-full border-[12px] border-[#00B42A] flex items-center justify-center mb-6">
          <div className="text-center">
            <div className="text-[24px] font-bold text-text-main">
              {cleared ? "1.2" : "2.4"}
            </div>
            <div className="text-[12px] text-text-sub">GB</div>
          </div>
        </div>
        <h3 className="text-[18px] font-medium text-text-main mb-2">
          ClawChat 已用空间
        </h3>
        <p className="text-[14px] text-text-sub text-center mb-8">
          手机剩余空间 {cleared ? "129.2 GB" : "128 GB"}
        </p>
        <button
          className="w-full h-12 bg-[#00B42A] text-white rounded-lg font-medium active:opacity-80 transition-opacity mb-4"
          disabled={cleared}
          onClick={() => {
            setCleared(true);
            showToast("缓存已清理");
          }}
        >
          {cleared ? "清理缓存 (0 B)" : "清理缓存 (1.2 GB)"}
        </button>
        <button
          onClick={() => navigate("/settings/general/storage/chat")}
          className="w-full h-12 bg-chat-other-bg text-text-main rounded-lg font-medium active:bg-active-bg transition-colors border border-border-color"
        >
          管理聊天记录 (1.2 GB)
        </button>
      </div>
    </PageLayout>
  );
};

export const SystemPermissions = () => (
  <PageLayout title="系统权限管理">
    <Group>
      <ListItem label="相册" rightText="已授权" />
      <ListItem label="相机" rightText="已授权" />
      <ListItem label="麦克风" rightText="已授权" />
      <ListItem label="位置信息" rightText="使用应用期间" hideBorder />
    </Group>
  </PageLayout>
);

export const AuthManagement = () => {
  const [apps, setApps] = useState([
    {
      id: 1,
      name: "WPS 办公助手",
      desc: "获取你的基础信息(昵称、头像)",
      color: "bg-blue-500",
      letter: "W",
    },
    {
      id: 2,
      name: "滴滴出行",
      desc: "获取你的位置信息和基础信息",
      color: "bg-orange-500",
      letter: "D",
    },
    {
      id: 3,
      name: "京东购物",
      desc: "获取你的基础信息",
      color: "bg-red-500",
      letter: "J",
    },
  ]);

  return (
    <PageLayout title="授权管理">
      <div className="p-4">
        <h3 className="text-[13px] text-text-sub mb-2 ml-1">
          你已授权以下应用
        </h3>
        <div className="bg-chat-other-bg rounded-xl overflow-hidden">
          {apps.map((app) => (
            <div
              key={app.id}
              className="flex items-center justify-between p-4 bg-chat-other-bg active:bg-active-bg transition-colors cursor-pointer border-b border-border-color last:border-0"
            >
              <div className="flex items-center">
                <div
                  className={`w-12 h-12 ${app.color} rounded-lg flex items-center justify-center mr-4`}
                >
                  <span className="text-white font-bold">{app.letter}</span>
                </div>
                <div>
                  <h4 className="text-[16px] font-medium text-text-main">
                    {app.name}
                  </h4>
                  <p className="text-[13px] text-text-sub mt-0.5">{app.desc}</p>
                </div>
              </div>
              <button
                className="text-[14px] text-[#FA5151] font-medium active:opacity-70 px-3 py-1.5 rounded-full bg-[#FA5151]/10"
                onClick={() => {
                  setApps(apps.filter((x) => x.id !== app.id));
                  showToast("已解除授权");
                }}
              >
                解除
              </button>
            </div>
          ))}
          {apps.length === 0 && (
            <div className="p-8 text-center text-text-sub">暂无授权应用</div>
          )}
        </div>
        <p className="text-[13px] text-text-sub text-center mt-6">
          以上应用可通过 ClawChat 快速登录并获取相关信息。
        </p>
      </div>
    </PageLayout>
  );
};

export const AdManagement = () => {
  const [adEnabled, setAdEnabled] = useState(true);
  return (
    <PageLayout title="个性化广告管理">
      <Group>
        <ToggleItem
          label="个性化广告"
          checked={adEnabled}
          onChange={setAdEnabled}
          hideBorder
        />
      </Group>
      <p className="text-[13px] text-text-sub px-4 mb-4">
        关闭后，您仍然会看到广告，但相关性会降低。
      </p>
    </PageLayout>
  );
};
