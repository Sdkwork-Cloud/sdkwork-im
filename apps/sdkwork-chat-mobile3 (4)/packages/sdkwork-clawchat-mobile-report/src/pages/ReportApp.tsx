import React, { useState } from "react";
import {
  PageLayout,
  IconButton,
  cn,
  showToast,
} from "@sdkwork/clawchat-mobile-commons";
import {
  Plus,
  Search,
  Filter,
  FileText,
  CheckCircle2,
  LayoutTemplate,
  Briefcase,
  FileSignature,
  ChevronRight,
} from "lucide-react";
import { ReportService, ReportItem } from "../services/ReportService";
import { motion } from "motion/react";
import { useNavigate } from "react-router";

export const ReportApp = () => {
  const navigate = useNavigate();
  const [activeTab, setActiveTab] = useState<
    "待我查阅" | "我发出的" | "抄送我的"
  >("待我查阅");
  const [reports, setReports] = useState<ReportItem[]>([]);
  const [isLoading, setIsLoading] = useState(true);

  React.useEffect(() => {
    setIsLoading(true);
    ReportService.getReports().then((data) => {
      setReports(data);
      setIsLoading(false);
    });
  }, []);

  const getTypeIcon = (type: string) => {
    switch (type) {
      case "日报":
        return <LayoutTemplate className="w-5 h-5 text-indigo-500" />;
      case "周报":
        return <Briefcase className="w-5 h-5 text-blue-500" />;
      case "月报":
        return <FileSignature className="w-5 h-5 text-orange-500" />;
      default:
        return <FileText className="w-5 h-5 text-primary-blue" />;
    }
  };

  return (
    <PageLayout title="汇报错题">
      <div className="flex flex-col h-full bg-[#f5f6f8] dark:bg-[#1a1b1c]">
        {/* Header Stats */}
        <div className="bg-primary-blue px-6 pt-4 pb-12 flex justify-between items-center text-white">
          <div>
            <div className="text-[32px] font-medium tracking-tight leading-none mb-1">
              5
            </div>
            <div className="text-[13px] opacity-80">未读汇报 (件)</div>
          </div>
          <div className="flex gap-4">
            <div className="flex flex-col items-center">
              <div className="text-[20px] font-medium leading-none mb-1">2</div>
              <div className="text-[12px] opacity-80">我发出的</div>
            </div>
            <div className="flex flex-col items-center">
              <div className="text-[20px] font-medium leading-none mb-1">
                10+
              </div>
              <div className="text-[12px] opacity-80">收到的</div>
            </div>
          </div>
        </div>

        <div className="flex-1 overflow-y-auto px-4 -mt-6">
          <div className="flex bg-white dark:bg-[#2c2d2e] rounded-xl shadow-sm mb-4 px-2 py-1">
            {["待我查阅", "我发出的", "抄送我的"].map((tab) => (
              <button
                key={tab}
                className={cn(
                  "flex-1 text-[15px] py-2.5 relative text-center transition-colors rounded-lg",
                  activeTab === tab
                    ? "text-primary-blue font-medium bg-primary-blue/5"
                    : "text-text-sub",
                )}
                onClick={() => setActiveTab(tab as any)}
              >
                {tab}
              </button>
            ))}
          </div>

          <div className="flex justify-between items-center mb-3 px-1">
            <h2 className="text-[14px] font-medium text-text-sub">
              全部记录 ({reports.length})
            </h2>
            <div className="flex gap-2">
              <IconButton
                icon={<Filter className="w-4 h-4 text-text-sub" />}
                className="bg-white dark:bg-[#2c2d2e] p-1.5 w-auto h-auto rounded-md shadow-sm"
              />
              <IconButton
                icon={<Search className="w-4 h-4 text-text-sub" />}
                className="bg-white dark:bg-[#2c2d2e] p-1.5 w-auto h-auto rounded-md shadow-sm"
              />
            </div>
          </div>

          <div className="flex flex-col gap-3 pb-20">
            {isLoading ? (
              <div className="flex flex-col items-center justify-center py-20 text-text-sub opacity-70">
                <div className="w-8 h-8 rounded-full border-4 border-text-sub border-t-white animate-spin mb-3"></div>
                <span className="text-[14px]">加载中...</span>
              </div>
            ) : reports.length > 0 ? (
              reports.map((report) => (
                <motion.div
                  key={report.id}
                  whileTap={{ scale: 0.98 }}
                  onClick={() => navigate(`/workspace/report/${report.id}`)}
                  className={cn(
                    "bg-white dark:bg-[#2c2d2e] p-4 rounded-xl cursor-pointer shadow-sm border",
                    report.isRead
                      ? "border-border-color/30"
                      : "border-primary-blue/20",
                  )}
                >
                  <div className="flex justify-between items-start mb-3">
                    <div className="flex items-center gap-3">
                      <div className="relative">
                        <div className="w-10 h-10 rounded-xl bg-gray-100 dark:bg-[#3a3b3c] flex items-center justify-center">
                          {getTypeIcon(report.type)}
                        </div>
                        {!report.isRead && (
                          <div className="absolute -top-1 -right-1 w-3 h-3 bg-red-500 rounded-full border-2 border-white dark:border-[#2c2d2e]" />
                        )}
                      </div>
                      <div>
                        <div className="text-[16px] font-medium text-text-main leading-tight mb-1 flex items-center gap-2">
                          {report.reporter} 的 {report.type}
                        </div>
                        <div className="text-[13px] text-text-sub font-mono">
                          {report.date}
                        </div>
                      </div>
                    </div>
                  </div>
                  <div className="text-[14px] text-text-main bg-[#f8f9fa] dark:bg-[#202122] p-3 rounded-lg flex items-center justify-between">
                    <span className="truncate pr-4 line-clamp-1">
                      {report.summary}
                    </span>
                    <ChevronRight className="w-4 h-4 text-text-sub shrink-0" />
                  </div>
                </motion.div>
              ))
            ) : (
              <div className="flex flex-col items-center justify-center py-20 text-text-sub opacity-70">
                <FileText className="w-12 h-12 mb-3 stroke-current opacity-40" />
                <span className="text-[14px]">暂无汇报记录</span>
              </div>
            )}
          </div>
        </div>

        <motion.button
          whileTap={{ scale: 0.9 }}
          whileHover={{ scale: 1.05 }}
          onClick={() => navigate("/workspace/report/create")}
          className="absolute bottom-6 right-6 w-14 h-14 bg-gradient-to-tr from-blue-600 to-primary-blue text-white rounded-full flex items-center justify-center shadow-lg shadow-blue-500/30 z-10"
        >
          <Plus className="w-7 h-7" />
        </motion.button>
      </div>
    </PageLayout>
  );
};
