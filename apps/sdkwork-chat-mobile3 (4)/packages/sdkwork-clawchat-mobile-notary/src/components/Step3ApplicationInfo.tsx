import React from "react";
import { motion } from "motion/react";

interface Step3ApplicationInfoProps {
  applicationInfo: string;
  setApplicationInfo: (info: string) => void;
}

export const Step3ApplicationInfo: React.FC<Step3ApplicationInfoProps> = ({
  applicationInfo,
  setApplicationInfo,
}) => {
  return (
    <motion.div
      key="step3"
      initial={{ opacity: 0, x: 20 }}
      animate={{ opacity: 1, x: 0 }}
      exit={{ opacity: 0, x: -20 }}
      className="flex flex-col gap-4"
    >
      <h2 className="text-[18px] font-bold">填写申请信息</h2>
      <div className="flex flex-col gap-2">
        <span className="text-[14px] text-text-sub font-medium">
          申办诉求与详细描述
        </span>
        <textarea
          value={applicationInfo}
          onChange={(e) => setApplicationInfo(e.target.value)}
          placeholder="请输入您要办理此项公证的具体内容和要求..."
          className="w-full bg-input-bg border border-border-color rounded-xl p-4 text-[15px] h-32 outline-none focus:border-primary-blue resize-none"
        />
      </div>
    </motion.div>
  );
};
