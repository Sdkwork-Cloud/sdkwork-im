import { useNavigate } from "react-router";
import React, { useState, useEffect } from "react";
import {} from "react-router";
import {
  ChevronLeft,
  Search,
  Wallet,
  Smartphone,
  Zap,
  Umbrella,
  Coffee,
  QrCode,
} from "lucide-react";
import {
  IconButton,
  cn,
  showToast,
  showPrompt,
} from "@sdkwork/clawchat-mobile-commons";
import { LifeService, type LifeServiceItem } from "../services/LifeService";

const ICON_MAP: Record<string, any> = {
  Smartphone: Smartphone,
  Zap: Zap,
  Umbrella: Umbrella,
  Coffee: Coffee,
};

const PageLayout = ({
  title,
  children,
  bgClass = "bg-bg-color",
}: {
  title?: string;
  children: React.ReactNode;
  bgClass?: string;
}) => {
  const navigate = useNavigate();
  return (
    <div className={`flex flex-col h-full ${bgClass} overflow-y-auto`}>
      <header className="h-[56px] flex items-center justify-between px-1 glass-header sticky top-0 z-10 shrink-0 pt-safe">
        <div className="flex items-center z-10 flex-1">
          <IconButton
            icon={
              <ChevronLeft
                className="w-6 h-6 text-text-main"
                strokeWidth={2.5}
              />
            }
            onClick={() => navigate(-1)}
          />
        </div>
        <div className="absolute left-1/2 -translate-x-1/2 flex items-center justify-center pointer-events-none">
          <h2 className="text-[17px] font-medium text-text-main">{title}</h2>
        </div>
        <div className="flex-1" />
      </header>
      <div className="flex flex-col flex-1">{children}</div>
    </div>
  );
};

// Services
export const ServicesPage = () => {
  const [services, setServices] = useState<LifeServiceItem[]>([]);
  const [showQr, setShowQr] = useState(false);

  useEffect(() => {
    LifeService.getLifeServices().then(setServices);
  }, []);

  return (
    <PageLayout title="服务" bgClass="bg-[#F3F3F3] dark:bg-black">
      <div className="p-4">
        <div className="bg-[#00C25F] rounded-xl p-6 text-white mb-4 shadow-sm flex justify-between items-center">
          <div>
            <div className="flex items-center gap-2 mb-2 opacity-90">
              <Wallet className="w-5 h-5" />
              <span className="text-[15px]">钱包</span>
            </div>
            <div className="text-[28px] font-bold">¥ 0.00</div>
          </div>
          <div className="flex gap-4">
            <div
              className="flex flex-col items-center gap-1 cursor-pointer active:opacity-70"
              onClick={() => setShowQr(true)}
            >
              <div className="w-10 h-10 bg-white/20 rounded-full flex items-center justify-center">
                <span className="text-[18px] font-medium">收</span>
              </div>
              <span className="text-[12px]">收付款</span>
            </div>
          </div>
        </div>

        <div className="bg-white dark:bg-[#1A1A1A] rounded-xl p-4 shadow-sm">
          <h3 className="text-[14px] text-text-sub mb-4 font-medium">
            生活服务
          </h3>
          <div className="grid grid-cols-4 gap-y-6">
            {services.map((item, i) => {
              const Icon = ICON_MAP[item.iconName] || Smartphone;
              return (
                <div
                  key={i}
                  className="flex flex-col items-center gap-2 cursor-pointer active:opacity-70"
                  onClick={async () => {
                    if (item.label === "手机充值") {
                      const phone = await showPrompt("请输入您要充值的手机号");
                      if (phone) {
                        const amount = await showPrompt("请输入充值金额", "50");
                        if (amount)
                          showToast(
                            `已提交充值请求：号码 ${phone}，面额 ${amount}元`,
                          );
                      }
                    } else if (item.label === "生活缴费") {
                      await showPrompt("请输入缴费户号", "H102930492");
                      showToast("缴费信息查询中...");
                    } else if (item.label === "外卖") {
                      showToast("即将打开外卖小程序...");
                    } else {
                      showToast(`正在打开${item.label}...`);
                    }
                  }}
                >
                  <Icon className={cn("w-7 h-7", item.color)} />
                  <span className="text-[12px] text-text-main">
                    {item.label}
                  </span>
                </div>
              );
            })}
          </div>
        </div>
      </div>
      {showQr && (
        <div
          className="fixed inset-0 z-50 bg-[#00C25F] flex flex-col items-center p-6 text-white"
          onClick={() => setShowQr(false)}
        >
          <div className="mt-20 text-[20px] font-medium mb-10">向商家付款</div>
          <div
            className="bg-white p-6 rounded-2xl w-full max-w-[320px] flex flex-col items-center shadow-xl pt-10 pb-10"
            onClick={(e) => e.stopPropagation()}
          >
            <div className="w-full h-20 bg-gray-100 flex items-center justify-center text-black tracking-[0.5em] font-mono text-xl mb-6">
              1234 5678 ******
            </div>
            <QrCode className="w-48 h-48 text-black" />
            <div className="text-gray-500 mt-6 text-sm">点击刷新</div>
          </div>
          <div
            className="mt-8 text-sm opacity-80 decoration-solid underline cursor-pointer"
            onClick={() => setShowQr(false)}
          >
            暂不付款
          </div>
        </div>
      )}
    </PageLayout>
  );
};

// Favorites
export const FavoritesPage = () => {
  const FAVORITES = [
    {
      title: "如何高效利用时间工作？",
      type: "文章",
      time: "昨天",
      source: "效率黑客",
    },
    {
      title: "公司年度旅游照片合集",
      type: "相册",
      time: "2023-10-01",
      source: "HR 部门",
    },
    {
      title: "王总语音记录",
      type: "语音",
      time: "2023-09-15",
      source: "微信聊天",
    },
  ];
  return (
    <PageLayout title="收藏">
      <div className="p-3 border-b border-border-color">
        <div className="bg-chat-other-bg rounded-lg h-9 flex items-center px-3 gap-2">
          <Search className="w-4 h-4 text-text-sub" />
          <input
            type="text"
            placeholder="搜索"
            className="bg-transparent flex-1 text-[14px] text-text-main outline-none"
          />
        </div>
      </div>
      <div className="flex-1 overflow-y-auto w-full">
        {FAVORITES.map((item, i) => (
          <div
            key={i}
            className="flex flex-col p-4 border-b border-border-color/50 active:bg-chat-other-bg transition-colors cursor-pointer"
            onClick={() => showToast(`打开: ${item.title}`)}
          >
            <span className="font-medium text-text-main text-[16px] mb-1.5">
              {item.title}
            </span>
            <div className="flex justify-between items-center text-[12px] text-text-sub">
              <span>
                [{item.type}] {item.source}
              </span>
              <span>{item.time}</span>
            </div>
          </div>
        ))}
      </div>
    </PageLayout>
  );
};

// My Agents
export const MyAgentsPage = () => {
  const navigate = useNavigate();
  const [agents, setAgents] = useState([
    {
      id: 1,
      name: "开发助手",
      desc: "全栈开发答疑解惑",
      icon: "🤖",
      type: "效率",
    },
    {
      id: 2,
      name: "小红书文案",
      desc: "一键生成爆款笔记",
      icon: "✍️",
      type: "创作",
    },
    {
      id: 3,
      name: "语言翻译官",
      desc: "精通多国语言互译",
      icon: "🌍",
      type: "工具",
    },
    {
      id: 4,
      name: "塔罗占卜师",
      desc: "探索内心的指引",
      icon: "🔮",
      type: "娱乐",
    },
  ]);

  return (
    <PageLayout title="我的智能体">
      {agents.length > 0 ? (
        <div className="flex-1 overflow-y-auto pb-12 bg-chat-other-bg">
          <div className="flex flex-col gap-[2px]">
            <div
              className="bg-bg-color px-4 py-4 flex items-center gap-3 active:bg-active-bg transition-colors cursor-pointer"
              onClick={() => navigate("/agent/create")}
            >
              <div className="w-12 h-12 bg-gray-100 dark:bg-[#1A1A1A] rounded-xl flex items-center justify-center border border-border-color border-dashed">
                <span className="text-[20px]">➕</span>
              </div>
              <div className="flex-1 font-medium text-text-main text-[16px]">
                创建新智能体
              </div>
            </div>

            {agents.map((agent) => (
              <div
                key={agent.id}
                className="bg-bg-color px-4 py-3.5 flex items-center gap-3 active:bg-active-bg transition-colors cursor-pointer"
              >
                <div className="w-12 h-12 bg-gray-100 dark:bg-[#1A1A1A] rounded-xl flex items-center justify-center border border-border-color">
                  <span className="text-[24px]">{agent.icon}</span>
                </div>
                <div className="flex-1 min-w-0 flex flex-col justify-center">
                  <div className="flex items-center justify-between mb-0.5">
                    <span className="font-medium text-text-main text-[16px] truncate">
                      {agent.name}
                    </span>
                    <span className="text-[11px] text-primary-blue bg-primary-blue/10 px-1.5 py-0.5 rounded border border-primary-blue/20">
                      {agent.type}
                    </span>
                  </div>
                  <p className="text-[13px] text-text-sub truncate">
                    {agent.desc}
                  </p>
                </div>
              </div>
            ))}
          </div>
        </div>
      ) : (
        <div className="flex flex-col items-center justify-center flex-1 py-20 w-full h-full">
          <div className="w-20 h-20 bg-chat-other-bg rounded-[24px] flex items-center justify-center mb-6 shadow-sm border border-border-color">
            <span className="text-4xl">🤖</span>
          </div>
          <h3 className="text-lg font-bold text-text-main mb-2">
            打造专属 AI 助手
          </h3>
          <p className="text-[14px] text-text-sub mb-8 max-w-[200px] text-center leading-relaxed">
            定制懂你的 AI 智能体，提升工作生活效率
          </p>
          <button
            onClick={() => navigate("/agent/create")}
            className="px-8 h-12 bg-primary-blue text-white rounded-full font-medium active:scale-95 transition-transform shadow-lg shadow-blue-500/30 flex items-center justify-center"
          >
            立即创建
          </button>
        </div>
      )}
    </PageLayout>
  );
};

// Emoji
export const EmojiPage = () => {
  const PACKS = [
    {
      title: "打工人的日常",
      author: "小李打工记",
      img: "https://picsum.photos/seed/e1/100",
    },
    {
      title: "萌宠猫咪大赏",
      author: "喵星人俱乐部",
      img: "https://picsum.photos/seed/e2/100",
    },
    {
      title: "社交悍匪专用包",
      author: "社牛局",
      img: "https://picsum.photos/seed/e3/100",
    },
  ];
  return (
    <PageLayout title="表情">
      <div className="flex border-b border-border-color stick top-0 bg-bg-color z-10 w-full">
        <div className="flex-1 py-3 text-center text-primary-blue border-b-2 border-primary-blue font-medium text-[15px]">
          精选表情
        </div>
        <div className="flex-1 py-3 text-center text-text-main text-[15px] opacity-70">
          更多表情
        </div>
      </div>
      <div className="flex-1 overflow-y-auto w-full p-4">
        <h3 className="font-bold text-text-main text-lg mb-4">热门推荐</h3>
        <div className="flex flex-col gap-5 w-full">
          {PACKS.map((pack, i) => (
            <div
              key={i}
              className="flex items-center gap-3 active:scale-95 transition-transform cursor-pointer"
              onClick={() => showToast(`已添加：${pack.title}`)}
            >
              <img
                src={pack.img}
                className="w-16 h-16 rounded-xl border border-border-color object-cover"
              />
              <div className="flex-1">
                <h4 className="font-bold text-text-main text-base mb-1">
                  {pack.title}
                </h4>
                <p className="text-[12px] text-text-sub">@{pack.author}</p>
              </div>
              <button className="px-4 py-1.5 h-8 bg-black/5 dark:bg-white/10 text-primary-blue font-medium rounded-full text-[13px] shrink-0 border border-primary-blue/20">
                添加
              </button>
            </div>
          ))}
        </div>
      </div>
    </PageLayout>
  );
};
