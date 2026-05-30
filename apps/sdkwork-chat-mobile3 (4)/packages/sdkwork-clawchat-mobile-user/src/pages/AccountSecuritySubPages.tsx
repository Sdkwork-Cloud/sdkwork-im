import { useNavigate } from "react-router";
import React, { useState } from "react";
import {} from "react-router";
import { PageLayout, Group, ListItem, ToggleItem } from "./SettingsSubPages";
import { showToast, showPrompt } from "@sdkwork/clawchat-mobile-commons";

export const ChangePhoneNumber = () => {
  const [step, setStep] = useState(1);
  const [phone, setPhone] = useState("");
  const [code, setCode] = useState("");

  const handleSubmit = () => {
    if (!phone || !code) return showToast("请输入完整信息");
    showToast("手机号已更变");
    // normally navigate back
  };

  return (
    <PageLayout title="绑定手机号">
      {step === 1 ? (
        <div className="flex flex-col items-center py-10 px-4">
          <div className="w-16 h-16 bg-primary-blue/10 rounded-full flex items-center justify-center mb-6">
            <span className="text-primary-blue text-3xl">📱</span>
          </div>
          <h3 className="text-[18px] font-medium text-text-main mb-2">
            你的手机号码：+86 138****8888
          </h3>
          <p className="text-[14px] text-text-sub text-center mb-8">
            绑定的手机号可用于登录 ClawChat，或找回密码。
          </p>
          <button
            onClick={() => setStep(2)}
            className="w-full h-12 bg-[#00B42A] text-white rounded-lg font-medium active:opacity-80 transition-opacity"
          >
            更换手机号
          </button>
        </div>
      ) : (
        <div className="px-4 py-6">
          <h3 className="text-[20px] font-medium text-text-main mb-6">
            验证新手机号
          </h3>
          <div className="flex items-center border-b border-border-color py-3 mb-4">
            <span className="text-[16px] text-text-main mr-4">+86</span>
            <input
              type="tel"
              placeholder="请填写手机号"
              value={phone}
              onChange={(e) => setPhone(e.target.value)}
              className="flex-1 bg-transparent text-[16px] text-text-main outline-none"
            />
          </div>
          <div className="flex items-center border-b border-border-color py-3 mb-8">
            <input
              type="text"
              placeholder="验证码"
              value={code}
              onChange={(e) => setCode(e.target.value)}
              className="flex-1 bg-transparent text-[16px] text-text-main outline-none"
            />
            <button
              className="text-[#00B42A] text-[15px] font-medium ml-4"
              onClick={() => showToast("验证码已发送")}
            >
              获取验证码
            </button>
          </div>
          <button
            className="w-full h-12 bg-[#00B42A] text-white rounded-lg font-medium active:opacity-80 transition-opacity"
            onClick={handleSubmit}
          >
            提交
          </button>
        </div>
      )}
    </PageLayout>
  );
};

export const ChangePassword = () => {
  return (
    <PageLayout title="设置密码">
      <div className="px-4 py-6">
        <div className="border-b border-border-color py-3 mb-2">
          <input
            type="password"
            placeholder="请填写原密码"
            className="w-full bg-transparent text-[16px] text-text-main outline-none"
          />
        </div>
        <div className="border-b border-border-color py-3 mb-2">
          <input
            type="password"
            placeholder="请填写新密码"
            className="w-full bg-transparent text-[16px] text-text-main outline-none"
          />
        </div>
        <div className="border-b border-border-color py-3 mb-8">
          <input
            type="password"
            placeholder="请再次填写新密码"
            className="w-full bg-transparent text-[16px] text-text-main outline-none"
          />
        </div>
        <p className="text-[13px] text-text-sub mb-8">
          密码必须包含字母和数字，且长度不少于8位。
        </p>
        <button
          className="w-full h-12 bg-[#00B42A] text-white rounded-lg font-medium active:opacity-80 transition-opacity"
          onClick={() => showToast("操作已执行")}
        >
          完成
        </button>
      </div>
    </PageLayout>
  );
};

export const VoiceLock = () => {
  const navigate = useNavigate();
  const [enabled, setEnabled] = useState(false);
  return (
    <PageLayout title="声音锁">
      <Group className="mt-4">
        <ToggleItem
          label="登录 ClawChat"
          checked={enabled}
          onChange={setEnabled}
          hideBorder
        />
      </Group>
      <p className="text-[13px] text-text-sub px-4 mb-8">
        开启后，可以使用声音解锁应用或验证身份。
      </p>
      <Group>
        <ListItem
          label="重设声音锁"
          hideBorder
          onClick={() => navigate("/settings/account/voice-lock/reset")}
        />
      </Group>
    </PageLayout>
  );
};

export const EmergencyContacts = () => {
  const [contacts, setContacts] = useState([
    { name: "爸爸", phone: "138****0001", relation: "父亲" },
    { name: "李小明", phone: "139****0002", relation: "朋友" },
  ]);

  return (
    <PageLayout title="应急联系人">
      <div className="flex flex-col h-full bg-bg-color">
        <div className="p-4 bg-chat-other-bg border-b border-border-color">
          <p className="text-[14px] text-text-sub leading-relaxed">
            当你的账号存在安全风险或无法登录时，可通过应急联系人辅助验证身份，恢复账号访问权限。
          </p>
        </div>

        <div className="flex-1 overflow-y-auto w-full mt-2">
          {contacts.map((contact, i) => (
            <div
              key={i}
              className="flex justify-between items-center p-4 bg-chat-other-bg border-b border-border-color active:bg-active-bg transition-colors"
            >
              <div>
                <span className="text-[16px] font-medium text-text-main flex items-center gap-2">
                  {contact.name}
                  <span className="text-[11px] bg-primary-blue/10 text-primary-blue px-1.5 py-0.5 rounded-sm">
                    {contact.relation}
                  </span>
                </span>
                <p className="text-[13px] text-text-sub mt-1">
                  {contact.phone}
                </p>
              </div>
              <button
                className="text-[13px] text-[#FA5151] px-3 py-1.5 rounded-full border border-border-color active:opacity-70"
                onClick={async () => {
                  showToast("已移除联系人");
                  setContacts(contacts.filter((_, idx) => idx !== i));
                }}
              >
                移除
              </button>
            </div>
          ))}

          <div className="p-6 flex justify-center">
            <button
              className="w-full h-12 bg-chat-other-bg text-text-main rounded-xl font-medium active:bg-active-bg transition-colors border border-border-color flex justify-center items-center gap-2"
              onClick={async () => {
                const name = await showPrompt("请输入应急联系人姓名");
                if (name) {
                  const phone = await showPrompt("请输入联系人手机号");
                  if (phone) {
                    setContacts([
                      ...contacts,
                      { name, phone, relation: "朋友" },
                    ]);
                    showToast("添加成功");
                  }
                }
              }}
            >
              添加应急联系人
            </button>
          </div>
        </div>
      </div>
    </PageLayout>
  );
};

export const MoreSecurity = () => {
  const navigate = useNavigate();
  return (
    <PageLayout title="更多安全设置">
      <Group>
        <ListItem
          label="QQ号"
          rightText="未绑定"
          onClick={() => navigate("/settings/account/more/qq")}
        />
        <ListItem
          label="邮件地址"
          rightText="未绑定"
          hideBorder
          onClick={() => navigate("/settings/account/more/email")}
        />
      </Group>
      <Group>
        <ListItem
          label="恢复账号密码"
          hideBorder
          onClick={() => navigate("/settings/account/more/recover")}
        />
      </Group>
      <Group>
        <ListItem
          label="注销账号"
          hideBorder
          onClick={() => navigate("/settings/account/more/delete")}
        />
      </Group>
    </PageLayout>
  );
};
