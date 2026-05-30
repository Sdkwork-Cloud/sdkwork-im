import React from "react";
import { PageLayout } from "../../components/SettingsCommons";
import { showToast } from "@sdkwork/clawchat-mobile-commons";
import { useNavigate } from "react-router";

export const WechatID = () => {
  const navigate = useNavigate();
  return (
    <PageLayout title="修改微信号">
      <div className="p-4">
        <input
          type="text"
          defaultValue="wxid_123456789"
          className="w-full bg-transparent border-b-2 border-[#00B42A] text-[18px] text-text-main pb-2 outline-none"
        />
        <p className="text-[13px] text-text-sub mt-2">
          微信号是账号的唯一凭证，一年只能修改一次。
        </p>
        <button
          className="mt-8 w-full h-12 bg-[#00B42A] text-white rounded-lg font-medium active:opacity-80 transition-opacity"
          onClick={async () => {
            showToast("修改申请已提交");
            navigate(-1);
          }}
        >
          保存
        </button>
      </div>
    </PageLayout>
  );
};

export const ResetVoiceLock = () => {
  const navigate = useNavigate();
  return (
    <PageLayout title="重设声音锁">
      <div className="flex flex-col items-center py-20">
        <p className="text-[24px] font-medium text-text-main mb-12">
          5 8 2 9 0 1
        </p>
        <div
          className="w-24 h-24 bg-[#00B42A] rounded-full flex items-center justify-center active:scale-95 transition-transform cursor-pointer shadow-lg shadow-green-500/30"
          onClick={async () => {
            showToast("正在录音识别中...");
            setTimeout(() => {
              showToast("声音锁设置成功");
              navigate(-1);
            }, 2000);
          }}
        >
          <span className="text-white text-3xl">🎤</span>
        </div>
        <p className="text-[14px] text-text-sub mt-8">
          按住上方按钮，匀速读出上方数字
        </p>
      </div>
    </PageLayout>
  );
};

export const BindQQ = () => {
  const navigate = useNavigate();
  return (
    <PageLayout title="绑定QQ号">
      <div className="p-4">
        <input
          type="text"
          placeholder="QQ号"
          className="w-full bg-chat-other-bg p-4 rounded-xl text-text-main outline-none mb-4"
        />
        <input
          type="password"
          placeholder="QQ密码"
          className="w-full bg-chat-other-bg p-4 rounded-xl text-text-main outline-none mb-8"
        />
        <button
          className="w-full h-12 bg-[#00B42A] text-white rounded-lg font-medium active:opacity-80 transition-opacity"
          onClick={async () => {
            showToast("绑定成功");
            navigate(-1);
          }}
        >
          绑定
        </button>
      </div>
    </PageLayout>
  );
};

export const BindEmail = () => {
  const navigate = useNavigate();
  return (
    <PageLayout title="绑定邮箱">
      <div className="p-4">
        <input
          type="email"
          placeholder="请输入邮箱地址"
          className="w-full bg-chat-other-bg p-4 rounded-xl text-text-main outline-none mb-8"
        />
        <button
          className="w-full h-12 bg-[#00B42A] text-white rounded-lg font-medium active:opacity-80 transition-opacity"
          onClick={async () => {
            showToast("验证邮件已发送");
            navigate(-1);
          }}
        >
          发送验证邮件
        </button>
      </div>
    </PageLayout>
  );
};

export const RecoverPassword = () => {
  const navigate = useNavigate();
  return (
    <PageLayout title="恢复账号密码">
      <div className="p-6 text-center">
        <div className="w-16 h-16 bg-primary-blue/10 rounded-full flex items-center justify-center mx-auto mb-6">
          <span className="text-primary-blue text-2xl">🛡️</span>
        </div>
        <h3 className="text-[18px] font-medium text-text-main mb-4">
          账号申诉
        </h3>
        <p className="text-[14px] text-text-sub mb-8">
          如果你的手机号、QQ、邮箱均无法使用，可以通过申诉找回密码。
        </p>
        <button
          className="w-full h-12 bg-[#00B42A] text-white rounded-lg font-medium active:opacity-80 transition-opacity"
          onClick={async () => {
            showToast("申诉请求已提交");
            navigate(-1);
          }}
        >
          开始申诉
        </button>
      </div>
    </PageLayout>
  );
};

export const DeleteAccount = () => {
  const navigate = useNavigate();
  return (
    <PageLayout title="注销账号">
      <div className="p-6 text-center">
        <div className="w-16 h-16 bg-red-500/10 rounded-full flex items-center justify-center mx-auto mb-6">
          <span className="text-red-500 text-2xl">⚠️</span>
        </div>
        <h3 className="text-[18px] font-medium text-text-main mb-4">
          注销 ClawChat 账号
        </h3>
        <p className="text-[14px] text-text-sub mb-8 text-left leading-relaxed">
          注销后，你的账号将被永久删除，无法恢复。
          <br />
          <br />
          1. 你的所有聊天记录、通讯录将被清空。
          <br />
          2. 你的微信豆、余额将被清空。
          <br />
          3. 绑定的第三方应用将解除授权。
        </p>
        <button
          className="w-full h-12 bg-[#FA5151] text-white rounded-lg font-medium active:opacity-80 transition-opacity"
          onClick={async () => {
            showToast("注销已进入最后确认流程");
            navigate(-1);
          }}
        >
          申请注销
        </button>
      </div>
    </PageLayout>
  );
};
