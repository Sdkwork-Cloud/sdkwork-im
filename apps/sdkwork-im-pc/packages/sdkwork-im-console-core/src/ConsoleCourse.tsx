import React from 'react';
import { Plus, Video } from 'lucide-react';
import { ConsoleContractEmptyState } from '@sdkwork/im-pc-commons';

export const ConsoleCourse: React.FC = () => (
  <div className="flex flex-col h-full bg-console-bg-panel rounded-xl border border-console-border overflow-hidden">
    <div className="p-6 border-b border-console-border flex flex-wrap gap-4 items-center justify-between bg-console-bg-panel/50">
      <div className="flex items-center gap-3">
        <Video size={22} className="text-blue-600" />
        <div>
          <h2 className="text-lg font-semibold text-console-text-main mb-1">课程管理</h2>
          <p className="text-sm text-console-text-muted">管理企业的内部培训与对外售卖的在线课程、直播培训</p>
        </div>
      </div>
      <button
        type="button"
        disabled
        className="text-console-text-muted px-4 py-2 rounded-lg text-sm font-medium border border-console-border opacity-60 cursor-not-allowed flex items-center gap-2"
      >
        <Plus size={16} />
        创建课程
      </button>
    </div>
    <ConsoleContractEmptyState title="课程管理合约暂未开放" />
  </div>
);
