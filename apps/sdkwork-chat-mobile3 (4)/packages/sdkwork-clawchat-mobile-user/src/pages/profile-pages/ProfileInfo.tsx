import React, { useState, useEffect } from "react";
import { useNavigate } from "react-router";
import { QrCode } from "lucide-react";
import {
  Avatar,
  showToast,
  showPrompt,
} from "@sdkwork/clawchat-mobile-commons";
import {
  ProfileService,
  type UserProfile,
} from "../../services/ProfileService";
import { PageLayout, Group, ListItem } from "../../components/SettingsCommons";

export const ProfileAvatar = () => {
  const [profile, setProfile] = useState<UserProfile | null>(null);

  useEffect(() => {
    ProfileService.getUserProfile().then(setProfile);
  }, []);

  return (
    <PageLayout title="个人头像">
      <div className="flex flex-col items-center justify-center py-20">
        <Avatar
          src={profile?.avatar || "https://picsum.photos/seed/me/200/200"}
          size="lg"
          className="w-64 h-64 rounded-xl shadow-lg"
        />
        <button
          className="mt-12 w-[200px] h-12 bg-chat-other-bg text-text-main rounded-lg font-medium active:bg-active-bg transition-colors border border-border-color"
          onClick={async () => {
            const url = await showPrompt(
              "请输入新头像的图片网址",
              profile?.avatar || "https://picsum.photos/seed/new/200",
            );
            if (url) {
              ProfileService.updateUserProfile({ avatar: url });
              showToast("已应用新头像");
              window.location.reload();
            }
          }}
        >
          更换头像
        </button>
      </div>
    </PageLayout>
  );
};

export const ProfileName = () => {
  const navigate = useNavigate();
  const [name, setName] = useState("");

  useEffect(() => {
    ProfileService.getUserProfile().then((p) => setName(p.name));
  }, []);

  const handleSave = async () => {
    await ProfileService.updateUserProfile({ name });
    showToast("已保存修改");
    navigate(-1);
  };

  return (
    <PageLayout title="更改名字">
      <div className="px-4 py-6">
        <input
          type="text"
          value={name}
          onChange={(e) => setName(e.target.value)}
          className="w-full bg-transparent border-b-2 border-[#00B42A] text-[18px] text-text-main pb-2 outline-none"
        />
        <p className="text-[13px] text-text-sub mt-2">
          好名字可以让你的朋友更容易记住你。
        </p>
        <button
          onClick={handleSave}
          className="mt-8 w-full h-12 bg-[#00B42A] text-white rounded-lg font-medium active:opacity-80 transition-opacity"
        >
          保存
        </button>
      </div>
    </PageLayout>
  );
};

export const ProfileTickle = () => {
  const [tickle, setTickle] = useState("");
  const navigate = useNavigate();
  return (
    <PageLayout title="拍一拍">
      <div className="px-4 py-6">
        <div className="flex items-center gap-2 mb-2">
          <span className="text-[16px] text-text-main">朋友拍了拍我</span>
          <input
            type="text"
            value={tickle}
            onChange={(e) => setTickle(e.target.value)}
            placeholder="的肩膀"
            className="flex-1 bg-chat-other-bg px-3 py-2 rounded-lg text-[16px] text-text-main outline-none border border-border-color focus:border-[#00B42A] transition-colors"
          />
        </div>
        <p className="text-[13px] text-text-sub">
          设置后，朋友拍你时将显示该文案。
        </p>
        <button
          className="mt-8 w-full h-12 bg-[#00B42A] text-white rounded-lg font-medium active:opacity-80 transition-opacity"
          onClick={async () => {
            showToast("已保存修改");
            navigate(-1);
          }}
        >
          完成
        </button>
      </div>
    </PageLayout>
  );
};

export const ProfileQRCode = () => {
  const [profile, setProfile] = useState<UserProfile | null>(null);

  useEffect(() => {
    ProfileService.getUserProfile().then(setProfile);
  }, []);

  return (
    <PageLayout title="我的二维码">
      <div className="flex flex-col items-center py-10 px-4">
        <div className="w-full max-w-[320px] bg-chat-other-bg rounded-2xl shadow-sm border border-border-color p-6">
          <div className="flex items-center gap-4 mb-6">
            <Avatar
              src={profile?.avatar || "https://picsum.photos/seed/me/200/200"}
              size="md"
              className="w-14 h-14 rounded-xl"
            />
            <div>
              <h3 className="text-[18px] font-bold text-text-main">
                {profile?.name || "User"}
              </h3>
              <p className="text-[13px] text-text-sub">
                {profile?.region || "北京 海淀"}
              </p>
            </div>
          </div>
          <div
            className="w-full aspect-square bg-white rounded-xl flex items-center justify-center p-4"
            onClick={() => showToast("已保存二维码到相册")}
          >
            <QrCode className="w-full h-full text-black" />
          </div>
          <p className="text-[13px] text-text-sub text-center mt-6">
            扫一扫上面的二维码图案，加我为朋友
          </p>
        </div>
      </div>
    </PageLayout>
  );
};

export const ProfileMore = () => {
  const navigate = useNavigate();
  const [profile, setProfile] = useState<UserProfile | null>(null);

  useEffect(() => {
    ProfileService.getUserProfile().then(setProfile);
  }, []);

  return (
    <PageLayout title="更多信息">
      <Group>
        <ListItem
          label="性别"
          rightText={profile?.gender || "未设置"}
          onClick={() => navigate("/my-profile/more/gender")}
        />
        <ListItem
          label="地区"
          rightText={profile?.region || "未设置"}
          onClick={() => navigate("/my-profile/more/region")}
        />
        <ListItem
          label="个性签名"
          rightText={profile?.signature || "未填写"}
          hideBorder
          onClick={() => navigate("/my-profile/more/signature")}
        />
      </Group>
    </PageLayout>
  );
};

export const ProfileRingtone = () => (
  <PageLayout title="来电铃声">
    <div className="flex flex-col items-center py-20">
      <div className="w-20 h-20 bg-primary-blue/10 rounded-full flex items-center justify-center mb-6">
        <span className="text-primary-blue text-3xl">🎵</span>
      </div>
      <h3 className="text-[18px] font-medium text-text-main mb-2">默认铃声</h3>
      <p className="text-[14px] text-text-sub mb-8">当前使用系统默认铃声</p>
      <button
        className="w-[200px] h-12 bg-chat-other-bg text-text-main rounded-lg font-medium active:bg-active-bg transition-colors border border-border-color"
        onClick={async () => {
          const ringtone = await showPrompt("请输入新的铃声名称");
          if (ringtone) {
            showToast(`已应用新铃声: ${ringtone}`);
          }
        }}
      >
        更换铃声
      </button>
    </div>
  </PageLayout>
);
