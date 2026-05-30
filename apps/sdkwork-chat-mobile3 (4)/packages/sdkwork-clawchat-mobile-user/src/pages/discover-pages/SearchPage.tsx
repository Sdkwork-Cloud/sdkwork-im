import React from "react";
import { PageLayout } from "../../components/SettingsCommons";
import { Search } from "lucide-react";

export const SearchPage = () => (
  <PageLayout title="搜一搜">
    <div className="p-4">
      <div className="bg-chat-other-bg rounded-lg h-10 flex items-center px-3 gap-2 mb-6">
        <Search className="w-5 h-5 text-text-sub" />
        <input
          type="text"
          placeholder="搜索文章、小程序等"
          className="bg-transparent flex-1 text-[15px] text-text-main outline-none"
        />
      </div>
      <h3 className="text-[14px] text-text-sub mb-4">搜索指定内容</h3>
      <div className="grid grid-cols-3 gap-4 text-center">
        <span className="text-[#2B5CE7] text-[15px]">朋友圈</span>
        <span className="text-[#2B5CE7] text-[15px]">文章</span>
        <span className="text-[#2B5CE7] text-[15px]">公众号</span>
        <span className="text-[#2B5CE7] text-[15px]">小程序</span>
        <span className="text-[#2B5CE7] text-[15px]">音乐</span>
        <span className="text-[#2B5CE7] text-[15px]">表情</span>
      </div>
    </div>
  </PageLayout>
);
