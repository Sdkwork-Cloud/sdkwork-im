import React, { useState } from "react";
import { useNavigate } from "react-router";
import {
  PageLayout,
  Group,
  ListItem,
  ToggleItem,
} from "../../components/SettingsCommons";

export const FriendPermissions = () => {
  const navigate = useNavigate();
  const [phone, setPhone] = useState(true);
  const [wxid, setWxid] = useState(true);
  return (
    <PageLayout title="朋友权限">
      <Group>
        <ListItem
          label="通讯录黑名单"
          hideBorder
          onClick={() => navigate("/settings/friend-permissions/blacklist")}
        />
      </Group>
      <div className="px-4 py-2 text-[13px] text-text-sub">添加我的方式</div>
      <Group>
        <ToggleItem label="手机号" checked={phone} onChange={setPhone} />
        <ToggleItem
          label="微信号"
          checked={wxid}
          onChange={setWxid}
          hideBorder
        />
      </Group>
    </PageLayout>
  );
};

export const Privacy = () => {
  const navigate = useNavigate();
  return (
    <PageLayout title="个人信息与权限">
      <Group>
        <ListItem
          label="系统权限管理"
          onClick={() => navigate("/settings/privacy/system")}
        />
        <ListItem
          label="授权管理"
          hideBorder
          onClick={() => navigate("/settings/privacy/auth")}
        />
      </Group>
      <Group>
        <ListItem
          label="个性化广告管理"
          hideBorder
          onClick={() => navigate("/settings/privacy/ads")}
        />
      </Group>
    </PageLayout>
  );
};

export const InfoCollection = () => (
  <PageLayout title="个人信息收集清单">
    <div className="p-4">
      <h3 className="text-[18px] font-bold text-text-main mb-4">
        个人信息收集清单
      </h3>
      <p className="text-[14px] text-text-sub mb-6 leading-relaxed">
        为了向您提供 ClawChat 的各项服务，我们需要收集您的以下个人信息：
      </p>
      <Group>
        <ListItem label="基本信息" rightText="头像、昵称、性别、地区" />
        <ListItem label="设备信息" rightText="设备型号、操作系统" />
        <ListItem label="网络信息" rightText="IP地址、网络类型" />
        <ListItem label="日志信息" rightText="操作日志、崩溃日志" hideBorder />
      </Group>
    </div>
  </PageLayout>
);

export const ThirdPartySharing = () => (
  <PageLayout title="第三方信息共享清单">
    <div className="p-4">
      <h3 className="text-[18px] font-bold text-text-main mb-4">
        第三方信息共享清单
      </h3>
      <p className="text-[14px] text-text-sub mb-6 leading-relaxed">
        在为您提供服务时，我们可能会与以下第三方共享您的必要信息：
      </p>
      <Group>
        <ListItem label="地图服务提供商" rightText="位置信息" />
        <ListItem label="推送服务提供商" rightText="设备标识符" />
        <ListItem label="支付服务提供商" rightText="订单信息" hideBorder />
      </Group>
    </div>
  </PageLayout>
);
