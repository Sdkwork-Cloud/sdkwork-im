import React from "react";
import { PageLayout } from "../../components/SettingsCommons";
import { Gamepad2, ChevronRight } from "lucide-react";
import { cn } from "@sdkwork/clawchat-mobile-commons";

export const GamesPage = () => {
  const GAMES = [
    {
      title: "跳一跳",
      icon: "https://picsum.photos/seed/g1/100",
      players: "1.2亿",
      tags: ["休闲", "竞技"],
      color: "text-green-500",
    },
    {
      title: "欢乐斗地主",
      icon: "https://picsum.photos/seed/g2/100",
      players: "5000万",
      tags: ["棋牌", "休闲"],
      color: "text-red-500",
    },
    {
      title: "羊了个羊",
      icon: "https://picsum.photos/seed/g3/100",
      players: "1000万",
      tags: ["益智", "挑战"],
      color: "text-emerald-500",
    },
    {
      title: "王者荣耀",
      icon: "https://picsum.photos/seed/g4/100",
      players: "2亿",
      tags: ["竞技", "动作"],
      color: "text-orange-500",
    },
    {
      title: "和平精英",
      icon: "https://picsum.photos/seed/g5/100",
      players: "1.5亿",
      tags: ["射击", "生存"],
      color: "text-yellow-500",
    },
  ];

  return (
    <PageLayout title="游戏">
      <div className="flex flex-col h-full bg-[#f5f6f8] dark:bg-[#1a1b1c] overflow-y-auto">
        <div className="p-4 flex flex-col gap-4">
          <div className="w-full h-32 bg-gradient-to-br from-indigo-500 to-purple-500 rounded-xl flex flex-col items-start justify-center p-6 text-white relative overflow-hidden">
            <div className="absolute top-0 right-0 p-4 opacity-20">
              <Gamepad2 className="w-32 h-32" />
            </div>
            <h2 className="text-xl font-bold mb-2 relative z-10">新游推荐</h2>
            <p className="text-sm opacity-80 relative z-10">
              探索本周最热火爆游戏
            </p>
          </div>

          <div className="bg-white dark:bg-[#2c2d2e] rounded-xl p-4 shadow-sm">
            <div className="flex justify-between items-center mb-4">
              <h3 className="font-bold text-text-main text-[16px]">热门游戏</h3>
              <span className="text-text-sub text-[13px] flex items-center">
                全部 <ChevronRight className="w-4 h-4 ml-0.5" />
              </span>
            </div>

            <div className="flex flex-col gap-4">
              {GAMES.map((game, i) => (
                <div
                  key={i}
                  className="flex items-center gap-3 active:scale-95 transition-transform cursor-pointer"
                >
                  <span
                    className={cn(
                      "font-bold text-lg w-4 text-center shrink-0",
                      i < 3 ? game.color : "text-text-sub",
                    )}
                  >
                    {i + 1}
                  </span>
                  <img
                    src={game.icon}
                    className="w-14 h-14 rounded-[14px]"
                    alt={game.title}
                  />
                  <div className="flex-1">
                    <h4 className="font-bold text-text-main text-[15px] mb-1">
                      {game.title}
                    </h4>
                    <div className="flex gap-2 text-[12px] text-text-sub">
                      <span>{game.players} 人在玩</span>
                      <span className="flex gap-1">
                        {game.tags.map((t) => (
                          <span
                            key={t}
                            className="bg-black/5 dark:bg-white/5 px-1.5 py-0.5 rounded text-[10px]"
                          >
                            {t}
                          </span>
                        ))}
                      </span>
                    </div>
                  </div>
                  <button className="px-4 py-1.5 bg-primary-blue/10 text-primary-blue font-medium rounded-full text-[13px] shrink-0">
                    玩一玩
                  </button>
                </div>
              ))}
            </div>
          </div>
        </div>
      </div>
    </PageLayout>
  );
};
