import React from "react";
import { UserPlus, User, Plus, ChevronRight, Video } from "lucide-react";
import { cn } from "@sdkwork/clawchat-mobile-commons";
import { motion } from "motion/react";
import { NotarySelectionParams } from "../pages/NotarySearchList";

interface Step2NotaryPartiesProps {
  selectedNotary: string;
  setSelectedNotary: (id: string) => void;
  selectedNotaryObj: any;
  setSelectedNotaryObj: (obj: any) => void;
  parties: any[];
  handleAddParty: () => void;
  handleEditParty: (party: any) => void;
  handleVideoCall: (party: any) => void;
  navigate: (path: string) => void;
  GLOBAL_STORE: any;
}

export const Step2NotaryParties: React.FC<Step2NotaryPartiesProps> = ({
  selectedNotary,
  setSelectedNotary,
  selectedNotaryObj,
  setSelectedNotaryObj,
  parties,
  handleAddParty,
  handleEditParty,
  handleVideoCall,
  navigate,
  GLOBAL_STORE,
}) => {
  return (
    <motion.div
      key="step2"
      initial={{ opacity: 0, x: 20 }}
      animate={{ opacity: 1, x: 0 }}
      exit={{ opacity: 0, x: -20 }}
      className="flex flex-col gap-4"
    >
      <h2 className="text-[18px] font-bold">公证员与当事人</h2>

      <div className="flex flex-col gap-2">
        <span className="text-[14px] text-text-sub font-medium">
          选择对接公证员
        </span>
        <div
          onClick={() => {
            NotarySelectionParams.selectedId = selectedNotary;
            NotarySelectionParams.onSelect = (id, obj) => {
              GLOBAL_STORE.selectedNotary = id;
              GLOBAL_STORE.selectedNotaryObj = obj;
              NotarySelectionParams.selectedNotaryObj = obj;
              setSelectedNotary(id);
              setSelectedNotaryObj(obj);
            };
            navigate("/notary/select-notary");
          }}
          className="w-full bg-input-bg border border-border-color rounded-xl px-4 py-3 min-h-[48px] flex items-center justify-between cursor-pointer active:opacity-70 transition-opacity"
        >
          {selectedNotary ? (
            <div className="flex flex-col">
              <span className="text-[15px] font-medium text-text-main">
                {selectedNotaryObj?.name}
              </span>
              <span className="text-[12px] text-text-sub">
                {selectedNotaryObj?.org}
              </span>
            </div>
          ) : (
            <span className="text-[15px] text-text-sub">请选择公证员</span>
          )}
          <ChevronRight className="w-5 h-5 text-text-sub opacity-50" />
        </div>
      </div>

      <div className="flex flex-col gap-3 mt-2">
        <div className="flex items-center justify-between">
          <span className="text-[14px] text-text-sub font-medium">
            当事人列表
          </span>
          <button
            onClick={handleAddParty}
            className="flex items-center gap-1 text-[13px] text-primary-blue font-medium active:opacity-70"
          >
            <UserPlus className="w-4 h-4" /> 添加当事人
          </button>
        </div>

        {parties.length === 0 ? (
          <div className="py-10 flex flex-col items-center justify-center border border-dashed border-border-color rounded-xl bg-chat-other-bg">
            <div className="w-16 h-16 bg-bg-color rounded-full shadow-sm flex items-center justify-center mb-3">
              <User className="w-8 h-8 text-text-sub opacity-30" />
            </div>
            <span className="text-[14px] text-text-sub mb-4">
              暂无当事人，请添加
            </span>
            <button
              onClick={handleAddParty}
              className="bg-primary-blue text-white w-10 h-10 rounded-full flex items-center justify-center active:scale-95 transition-transform shadow-sm"
            >
              <Plus className="w-6 h-6 outline-none" strokeWidth={2.5} />
            </button>
          </div>
        ) : (
          <div className="flex flex-col gap-2">
            {parties.map((p) => (
              <div
                key={p.id}
                onClick={() => handleEditParty(p)}
                className="bg-bg-color border border-border-color rounded-xl p-4 flex flex-col gap-3 cursor-pointer active:bg-active-bg transition-colors shadow-sm relative overflow-hidden"
              >
                {p.faceScore && (
                  <div className="absolute top-0 right-0 bg-green-500/10 text-green-600 dark:text-green-500 text-[11px] font-bold px-2 py-1 rounded-bl-lg">
                    认证通过
                  </div>
                )}
                <div className="flex items-center justify-between">
                  <div className="flex items-center gap-3">
                    <div className="w-12 h-12 bg-gradient-to-br from-indigo-400 to-primary-blue rounded-xl flex items-center justify-center text-white font-bold text-[18px] shadow-sm">
                      {p.name.slice(0, 1)}
                    </div>
                    <div className="flex flex-col">
                      <div className="flex items-center gap-2">
                        <span className="text-[16px] font-bold text-text-main">
                          {p.name}
                        </span>
                        {p.gender && (
                          <span className="text-[12px] bg-primary-blue/10 text-primary-blue px-1.5 py-0.5 rounded-md font-medium">
                            {p.gender}
                          </span>
                        )}
                      </div>
                      <span className="text-[13px] text-text-sub font-mono mt-0.5">
                        {p.idCard &&
                          p.idCard.replace(/^(.{4})(.*)(.{4})$/, "$1****$3")}
                      </span>
                    </div>
                  </div>
                </div>

                <div className="grid grid-cols-2 gap-2 mt-1 pt-3 border-t border-border-color/50">
                  <div className="flex flex-col">
                    <span className="text-[11px] text-text-sub mb-0.5">
                      联系手机
                    </span>
                    <span className="text-[13px] text-text-main font-medium">
                      {p.phone || "未填写"}
                    </span>
                  </div>
                  <div className="flex flex-col">
                    <span className="text-[11px] text-text-sub mb-0.5">
                      出生日期
                    </span>
                    <span className="text-[13px] text-text-main font-medium">
                      {p.dob || "未填写"}
                    </span>
                  </div>
                </div>

                <div className="flex items-center justify-between bg-input-bg rounded-lg p-2 mt-1">
                  <div className="flex items-center gap-2">
                    <div
                      className={cn(
                        "w-2 h-2 rounded-full",
                        p.faceScore ? "bg-green-500" : "bg-orange-400",
                      )}
                    />
                    <span className="text-[12px] text-text-sub shrink-0">
                      {p.faceScore ? `相似度 ${p.faceScore}%` : "未比对"}{" "}
                      {p.attachmentsCount && p.attachmentsCount > 0
                        ? `· 附件 ${p.attachmentsCount}`
                        : ""}
                    </span>
                  </div>
                  <div className="flex items-center gap-3">
                    <div
                      onClick={(e) => {
                        e.stopPropagation();
                        handleVideoCall(p);
                      }}
                      className="flex items-center gap-1 text-[12px] text-primary-blue font-medium bg-primary-blue/10 px-2 py-0.5 rounded-full active:opacity-70"
                    >
                      <Video className="w-3.5 h-3.5" /> 通话
                    </div>
                    <div className="flex items-center gap-0.5 text-[13px] text-text-sub font-medium">
                      编辑 <ChevronRight className="w-4 h-4" />
                    </div>
                  </div>
                </div>
              </div>
            ))}
          </div>
        )}
      </div>
    </motion.div>
  );
};
