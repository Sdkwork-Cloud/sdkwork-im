import React, { useState, useEffect, useRef } from "react";
import {
  X,
  Play,
  Square,
  Check,
  Sparkles,
  AudioLines,
  Mic,
  ChevronRight,
} from "lucide-react";
import { IconButton, cn } from "@sdkwork/clawchat-mobile-commons";
import { motion, AnimatePresence } from "motion/react";

export const VoiceSelectorModal = ({
  isOpen,
  onClose,
  selectedId,
  onSelect,
  voices,
  setVoices,
}: any) => {
  const [step, setStep] = useState<"list" | "record" | "processing">("list");
  const [playingId, setPlayingId] = useState<string | null>(null);
  const [recordingTime, setRecordingTime] = useState(0);
  const [isRecording, setIsRecording] = useState(false);
  const timerRef = useRef<NodeJS.Timeout | null>(null);

  const handlePlay = (id: string, e: React.MouseEvent) => {
    e.stopPropagation();
    if (playingId === id) {
      setPlayingId(null);
    } else {
      setPlayingId(id);
      setTimeout(() => setPlayingId(null), 3000);
    }
  };

  const closeSelector = () => {
    setStep("list");
    setPlayingId(null);
    setIsRecording(false);
    if (timerRef.current) clearInterval(timerRef.current);
    onClose();
  };

  const startQuickClone = () => {
    setStep("record");
    setRecordingTime(0);
    setIsRecording(false);
  };

  const toggleRecording = () => {
    if (!isRecording) {
      setIsRecording(true);
      setRecordingTime(0);
      timerRef.current = setInterval(
        () => setRecordingTime((prev) => prev + 1),
        1000,
      );
    } else {
      setIsRecording(false);
      if (timerRef.current) clearInterval(timerRef.current);
      setStep("processing");
      setTimeout(() => {
        const newVoice = {
          id: Date.now().toString(),
          name: "实时极速克隆专属音",
          type: "专属声音",
        };
        setVoices((prev: any[]) => [newVoice, ...prev]);
        onSelect(newVoice.id);
        closeSelector();
      }, 2500);
    }
  };

  useEffect(() => {
    return () => {
      if (timerRef.current) clearInterval(timerRef.current);
    };
  }, []);

  return (
    <AnimatePresence>
      {isOpen && (
        <>
          <motion.div
            initial={{ opacity: 0 }}
            animate={{ opacity: 1 }}
            exit={{ opacity: 0 }}
            className="fixed inset-0 bg-black/60 z-[100]"
            onClick={closeSelector}
          />
          <motion.div
            initial={{ y: "100%" }}
            animate={{ y: 0 }}
            exit={{ y: "100%" }}
            transition={{ type: "spring", damping: 25, stiffness: 200 }}
            className="fixed bottom-0 left-0 w-full bg-bg-color rounded-t-[32px] z-[101] flex flex-col h-[85vh] overflow-hidden"
          >
            <div className="h-14 flex items-center justify-between px-6 shrink-0 relative border-b border-border-color/30">
              <span className="text-[17px] font-bold text-text-main">
                {step === "list" ? "选择角色声音" : "实时极速克隆"}
              </span>
              <IconButton
                icon={<X className="w-6 h-6 text-text-sub" />}
                onClick={closeSelector}
                className="-mr-2"
              />
            </div>

            <div className="flex-1 overflow-y-auto relative bg-bg-color">
              <AnimatePresence mode="wait">
                {step === "list" && (
                  <motion.div
                    key="list"
                    initial={{ opacity: 0, x: -20 }}
                    animate={{ opacity: 1, x: 0 }}
                    exit={{ opacity: 0, x: -20 }}
                    className="p-4 flex flex-col gap-3 pb-safe min-h-full"
                  >
                    <button
                      onClick={startQuickClone}
                      className="bg-gradient-to-r from-primary-blue/5 to-indigo-500/5 active:from-primary-blue/10 active:to-indigo-500/10 border border-primary-blue/30 rounded-2xl p-4 flex items-center justify-between transition-colors text-left group"
                    >
                      <div className="flex items-center gap-3">
                        <div className="w-10 h-10 rounded-full bg-gradient-to-tr from-primary-blue to-indigo-500 flex items-center justify-center shadow-md shadow-primary-blue/20">
                          <Sparkles className="w-5 h-5 text-white" />
                        </div>
                        <div className="flex flex-col">
                          <span className="text-[16px] font-bold text-text-main">
                            没有满意的声音？
                          </span>
                          <span className="text-[13px] text-text-sub">
                            点击进行实时5秒极速克隆
                          </span>
                        </div>
                      </div>
                      <ChevronRight className="w-5 h-5 text-primary-blue opacity-50 group-hover:opacity-100 transition-opacity" />
                    </button>

                    <div className="h-[1px] bg-border-color/50 my-2 mx-2" />

                    {voices.map((v: any) => (
                      <div
                        key={v.id}
                        className={cn(
                          "bg-chat-other-bg p-4 rounded-xl flex items-center justify-between border transition-all cursor-pointer",
                          selectedId === v.id
                            ? "border-primary-blue/60 bg-primary-blue/5"
                            : "border-border-color/50 active:bg-active-bg",
                        )}
                        onClick={() => onSelect(v.id)}
                      >
                        <div className="flex items-center gap-3">
                          <div
                            className={cn(
                              "w-10 h-10 rounded-full flex items-center justify-center relative",
                              selectedId === v.id
                                ? "bg-primary-blue/20"
                                : "bg-black/5 dark:bg-white/5",
                            )}
                          >
                            <Mic
                              className={cn(
                                "w-5 h-5",
                                selectedId === v.id
                                  ? "text-primary-blue"
                                  : "text-text-sub",
                              )}
                            />
                            {playingId === v.id && (
                              <motion.div
                                className="absolute inset-0 rounded-full border-2 border-primary-blue"
                                animate={{
                                  scale: [1, 1.2, 1],
                                  opacity: [0.5, 0, 0.5],
                                }}
                                transition={{ repeat: Infinity, duration: 1 }}
                              />
                            )}
                          </div>
                          <div className="flex flex-col">
                            <span
                              className={cn(
                                "text-[16px] font-medium",
                                selectedId === v.id
                                  ? "text-primary-blue"
                                  : "text-text-main",
                              )}
                            >
                              {v.name}
                            </span>
                            <span className="text-[12px] text-text-sub mt-0.5">
                              {v.type}
                            </span>
                          </div>
                        </div>
                        <div className="flex items-center gap-4">
                          <div
                            className="w-8 h-8 rounded-full bg-black/5 dark:bg-white/5 flex items-center justify-center active:scale-95 transition-transform"
                            onClick={(e) => handlePlay(v.id, e)}
                          >
                            {playingId === v.id ? (
                              <Square className="w-3.5 h-3.5 text-text-main fill-current" />
                            ) : (
                              <Play className="w-4 h-4 text-text-main ml-0.5" />
                            )}
                          </div>
                          <div
                            className={cn(
                              "w-5 h-5 rounded-full border-2 flex items-center justify-center transition-colors",
                              selectedId === v.id
                                ? "border-primary-blue bg-primary-blue"
                                : "border-border-color",
                            )}
                          >
                            {selectedId === v.id && (
                              <Check className="w-3.5 h-3.5 text-white stroke-[3.5]" />
                            )}
                          </div>
                        </div>
                      </div>
                    ))}
                  </motion.div>
                )}

                {step === "record" && (
                  <motion.div
                    key="record"
                    initial={{ opacity: 0, x: 20 }}
                    animate={{ opacity: 1, x: 0 }}
                    exit={{ opacity: 0, x: 20 }}
                    className="p-6 flex flex-col items-center min-h-[400px]"
                  >
                    <div className="bg-chat-other-bg rounded-2xl p-6 shadow-sm border border-border-color w-full mb-8">
                      <h3 className="text-[14px] font-bold text-text-sub mb-3 flex items-center justify-center gap-1.5">
                        <AudioLines className="w-4 h-4 text-primary-blue" />{" "}
                        请用正常语速朗读下方文字（约5秒）
                      </h3>
                      <p className="text-[20px] leading-relaxed text-text-main font-serif tracking-wide text-center">
                        “你好，我是您的贴身助理，很高兴认识你。以后请多指教呀！”
                      </p>
                    </div>

                    <div className="flex flex-col items-center mt-4">
                      <div className="text-[36px] font-mono font-medium text-text-main mb-6">
                        00:0{recordingTime}
                      </div>
                      <button
                        onClick={toggleRecording}
                        className="relative w-24 h-24 bg-red-500 rounded-full flex items-center justify-center active:scale-95 transition-transform shadow-xl shadow-red-500/20"
                      >
                        {isRecording && (
                          <motion.div
                            className="absolute inset-0 bg-red-500 rounded-full"
                            animate={{
                              scale: [1, 1.4, 1],
                              opacity: [0.5, 0, 0.5],
                            }}
                            transition={{ repeat: Infinity, duration: 1.5 }}
                          />
                        )}
                        {isRecording ? (
                          <Square className="w-8 h-8 text-white fill-current relative z-10" />
                        ) : (
                          <Mic className="w-10 h-10 text-white relative z-10" />
                        )}
                      </button>
                      <p className="text-[14px] text-text-sub mt-6">
                        {isRecording
                          ? "正在录音，点击停止并极速生成..."
                          : "点击开始录音"}
                      </p>
                    </div>
                  </motion.div>
                )}

                {step === "processing" && (
                  <motion.div
                    key="processing"
                    initial={{ opacity: 0, scale: 0.9 }}
                    animate={{ opacity: 1, scale: 1 }}
                    className="p-10 flex flex-col items-center justify-center min-h-[400px]"
                  >
                    <div className="relative w-32 h-32 flex items-center justify-center mb-6">
                      <div className="absolute inset-0 border-[4px] border-primary-blue/20 rounded-full" />
                      <motion.div
                        className="absolute inset-0 border-[4px] border-primary-blue rounded-full border-t-transparent"
                        animate={{ rotate: 360 }}
                        transition={{
                          repeat: Infinity,
                          duration: 1,
                          ease: "linear",
                        }}
                      />
                      <Sparkles className="w-10 h-10 text-primary-blue" />
                    </div>
                    <h2 className="text-[20px] font-bold text-text-main mb-2">
                      正在建立数字特征
                    </h2>
                    <p className="text-[14px] text-text-sub">
                      极速克隆进行中，请稍候...
                    </p>
                  </motion.div>
                )}
              </AnimatePresence>
            </div>
          </motion.div>
        </>
      )}
    </AnimatePresence>
  );
};
