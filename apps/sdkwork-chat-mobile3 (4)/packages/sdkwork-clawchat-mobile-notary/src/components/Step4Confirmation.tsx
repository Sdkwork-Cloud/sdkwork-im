import React from "react";
import { motion } from "motion/react";
import { NotarySelectionParams } from "../pages/NotarySearchList";

interface Step4ConfirmationProps {
  notaryTypes: any[];
  selectedType: string;
  selectedNotaryObj: any;
  parties: any[];
  applicationInfo: string;
}

export const Step4Confirmation: React.FC<Step4ConfirmationProps> = ({
  notaryTypes,
  selectedType,
  selectedNotaryObj,
  parties,
  applicationInfo,
}) => {
  return (
    <motion.div
      key="step4"
      initial={{ opacity: 0, x: 20 }}
      animate={{ opacity: 1, x: 0 }}
      exit={{ opacity: 0, x: -20 }}
      className="flex flex-col gap-4"
    >
      <h2 className="text-[18px] font-bold">确认业务信息</h2>
      <div className="bg-chat-other-bg border border-border-color rounded-xl p-4 flex flex-col gap-4 text-[14px]">
        <div className="flex flex-col gap-1">
          <span className="text-text-sub">业务类型</span>
          <span className="font-medium">
            {notaryTypes.find((t) => t.id === selectedType)?.name}
          </span>
        </div>
        <div className="h-[1px] w-full bg-border-color" />
        <div className="flex flex-col gap-1">
          <span className="text-text-sub">承办公证员</span>
          <span className="font-medium">{selectedNotaryObj?.name}</span>
        </div>
        <div className="h-[1px] w-full bg-border-color" />
        <div className="flex flex-col gap-1">
          <span className="text-text-sub">当事人 ({parties.length}人)</span>
          <span className="font-medium">
            {parties.map((p) => p.name).join("、")}
          </span>
        </div>
        <div className="h-[1px] w-full bg-border-color" />
        <div className="flex flex-col gap-1">
          <span className="text-text-sub">申请描述</span>
          <p className="font-medium whitespace-pre-wrap">{applicationInfo}</p>
        </div>
      </div>
    </motion.div>
  );
};
