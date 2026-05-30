import React, { useState, useRef, useEffect } from "react";
import { useNavigate } from "react-router";
import {
  ChevronLeft,
  Mic,
  UploadCloud,
  Square,
  Play,
  RotateCcw,
  CheckCircle2,
  Sparkles,
} from "lucide-react";
import { IconButton, cn } from "@sdkwork/clawchat-mobile-commons";
import { motion, AnimatePresence } from "motion/react";

export const CreateVoice: React.FC = () => {
  const navigate = useNavigate();
  const [activeTab, setActiveTab] = useState<"record" | "upload">("record");
  const [recordingState, setRecordingState] = useState<
    "idle" | "recording" | "recorded" | "processing" | "done"
  >("idle");
  const [timer, setTimer] = useState(0);
  const timerRef = useRef<NodeJS.Timeout | null>(null);

  useEffect(() => {
    return () => {
      if (timerRef.current) clearInterval(timerRef.current);
    };
  }, []);

  const formatTime = (seconds: number) => {
    const mins = Math.floor(seconds / 60);
    const secs = seconds % 60;
    return `${mins.toString().padStart(2, "0")}:${secs.toString().padStart(2, "0")}`;
  };

  const startRecording = () => {
    setRecordingState("recording");
    setTimer(0);
    timerRef.current = setInterval(() => {
      setTimer((prev) => prev + 1);
    }, 1000);
  };

  const stopRecording = () => {
    setRecordingState("recorded");
    if (timerRef.current) clearInterval(timerRef.current);
  };

  const reRecord = () => {
    setRecordingState("idle");
    setTimer(0);
  };

  const [isPreviewPlaying, setIsPreviewPlaying] = useState(false);

  const togglePreview = () => {
    if (isPreviewPlaying) {
      setIsPreviewPlaying(false);
    } else {
      setIsPreviewPlaying(true);
      setTimeout(() => {
        setIsPreviewPlaying(false);
      }, 3000); // end after 3s
    }
  };

  const [voiceName, setVoiceName] = useState("");
  const [voiceDesc, setVoiceDesc] = useState("");

  const cloneVoice = () => {
    setIsPreviewPlaying(false);
    setRecordingState("processing");
    setTimeout(() => {
      setRecordingState("done");
    }, 2500); // simulate 2.5s clone process
  };

  const handleUpload = () => {
    const input = document.createElement("input");
    input.type = "file";
    input.accept = "audio/*";
    input.onchange = () => {
      setRecordingState("processing");
      setTimeout(() => {
        setRecordingState("done");
      }, 2500);
    };
    input.click();
  };

  const saveVoice = async () => {
    if (!voiceName.trim()) return;
    const { VoiceService } = await import("../services/VoiceService");
    await VoiceService.addCustomVoice(voiceName, voiceDesc || "新克隆的音色");
    navigate("/me/voices");
  };

  return (
    <div className="flex flex-col h-full bg-bg-color relative">
      {/* Header */}
      <header className="h-[56px] flex items-center justify-between px-1 glass-header sticky top-0 z-10 shrink-0 pt-safe relative">
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
          <h1 className="text-[17px] font-medium text-text-main">声音克隆</h1>
        </div>
        <div className="flex-1" />
      </header>

      {/* Tabs */}
      {recordingState !== "processing" && recordingState !== "done" && (
        <div className="px-4 py-3 shrink-0 flex items-center justify-center gap-6">
          <button
            className={cn(
              "text-[16px] font-medium transition-colors",
              activeTab === "record"
                ? "text-primary-blue text-[17px]"
                : "text-text-sub",
            )}
            onClick={() => {
              setActiveTab("record");
              setRecordingState("idle");
            }}
          >
            录音克隆
          </button>
          <button
            className={cn(
              "text-[16px] font-medium transition-colors",
              activeTab === "upload"
                ? "text-primary-blue text-[17px]"
                : "text-text-sub",
            )}
            onClick={() => {
              setActiveTab("upload");
              setRecordingState("idle");
            }}
          >
            上传音频
          </button>
        </div>
      )}

      {/* Main Content Area */}
      <div className="flex-1 flex flex-col p-6 pb-safe overflow-hidden relative">
        <AnimatePresence mode="wait">
          {recordingState === "processing" && (
            <motion.div
              key="processing"
              initial={{ opacity: 0, scale: 0.9 }}
              animate={{ opacity: 1, scale: 1 }}
              exit={{ opacity: 0 }}
              className="flex flex-col items-center justify-center h-full gap-6 w-full"
            >
              <div className="relative w-32 h-32 flex items-center justify-center">
                <div className="absolute inset-0 border-[4px] border-primary-blue/20 rounded-full" />
                <motion.div
                  className="absolute inset-0 border-[4px] border-primary-blue rounded-full border-t-transparent"
                  animate={{ rotate: 360 }}
                  transition={{ repeat: Infinity, duration: 1, ease: "linear" }}
                />
                <Sparkles className="w-10 h-10 text-primary-blue" />
              </div>
              <div className="text-center">
                <h2 className="text-[20px] font-bold text-text-main mb-2">
                  AI 正在克隆您的声音
                </h2>
                <p className="text-[14px] text-text-sub">
                  分析音色特征并建立数字模型，请稍候...
                </p>
              </div>
            </motion.div>
          )}

          {recordingState === "done" && (
            <motion.div
              key="done"
              initial={{ opacity: 0, scale: 0.9 }}
              animate={{ opacity: 1, scale: 1 }}
              exit={{ opacity: 0 }}
              className="flex flex-col gap-6 w-full h-full"
            >
              <div className="flex flex-col items-center text-center mt-2">
                <h2 className="text-[20px] font-bold text-text-main mb-1">
                  声音克隆完成
                </h2>
                <p className="text-[14px] text-text-sub">
                  请完善您的专属声音信息
                </p>
              </div>

              <div className="flex flex-col gap-4 flex-1">
                <div className="flex flex-col gap-2">
                  <label className="text-[14px] font-medium text-text-main ml-1">
                    语音头像
                  </label>
                  <div className="w-20 h-20 bg-chat-other-bg border border-border-color rounded-2xl flex items-center justify-center overflow-hidden active:opacity-70 transition-opacity cursor-pointer mx-auto mb-2">
                    <UploadCloud className="w-8 h-8 text-text-sub opacity-50" />
                  </div>
                </div>

                <div className="flex flex-col gap-2">
                  <label className="text-[14px] font-medium text-text-main ml-1">
                    声音名称
                  </label>
                  <input
                    type="text"
                    value={voiceName}
                    onChange={(e) => setVoiceName(e.target.value)}
                    placeholder="例如：治愈系睡前故事音"
                    className="w-full bg-chat-other-bg border border-border-color rounded-xl px-4 py-3.5 text-[15px] text-text-main outline-none focus:border-primary-blue transition-colors"
                  />
                </div>

                <div className="flex flex-col gap-2">
                  <label className="text-[14px] font-medium text-text-main ml-1">
                    声音简介
                  </label>
                  <textarea
                    value={voiceDesc}
                    onChange={(e) => setVoiceDesc(e.target.value)}
                    placeholder="描述一下这个声音的特点或用途..."
                    rows={3}
                    className="w-full bg-chat-other-bg border border-border-color rounded-xl px-4 py-3.5 text-[15px] text-text-main outline-none focus:border-primary-blue transition-colors resize-none mb-4"
                  />
                </div>
              </div>

              <div className="mt-auto shrink-0 mb-4">
                <button
                  onClick={saveVoice}
                  disabled={!voiceName.trim()}
                  className="w-full py-3.5 bg-primary-blue text-white rounded-full font-bold text-[16px] shadow-lg shadow-primary-blue/20 active:opacity-80 transition-opacity disabled:opacity-50"
                >
                  保存我的专属声音
                </button>
              </div>
            </motion.div>
          )}

          {recordingState !== "processing" &&
            recordingState !== "done" &&
            activeTab === "record" && (
              <motion.div
                key="record-mode"
                initial={{ opacity: 0 }}
                animate={{ opacity: 1 }}
                exit={{ opacity: 0 }}
                className="w-full flex-1 flex flex-col min-h-0"
              >
                {/* Teleprompter / Read text */}
                <div className="flex-1 bg-chat-other-bg rounded-2xl p-6 shadow-sm border border-border-color overflow-y-auto w-full">
                  <h3 className="text-[15px] font-bold text-text-main mb-4 flex items-center justify-center">
                    请用自然的语调朗读以下内容，保持环境安静
                  </h3>
                  <p className="text-[18px] leading-relaxed text-text-main/90 font-serif tracking-wide text-center">
                    “生活就像海洋，只有意志坚强的人，才能到达彼岸。无论遇到什么样的困难，我们都要保持微笑，勇敢前行。今天是个好日子，阳光明媚，微风不燥。希望每一个听到我声音的人，都能感受到温暖和力量。”
                  </p>
                </div>

                {/* Recording Controls */}
                <div className="h-[200px] shrink-0 flex flex-col items-center justify-center mt-6 w-full gap-4">
                  <div className="text-[32px] font-mono text-text-main">
                    {formatTime(timer)}
                  </div>

                  {recordingState === "idle" && (
                    <button
                      onClick={startRecording}
                      className="flex flex-col items-center gap-3 active:opacity-70 transition-opacity"
                    >
                      <div className="w-20 h-20 bg-red-500 rounded-full flex items-center justify-center shadow-lg shadow-red-500/20">
                        <Mic className="w-8 h-8 text-white" />
                      </div>
                      <span className="text-[14px] font-medium text-text-sub">
                        点击开始录音
                      </span>
                    </button>
                  )}

                  {recordingState === "recording" && (
                    <div className="flex flex-col items-center gap-3">
                      <button
                        onClick={stopRecording}
                        className="relative w-20 h-20 bg-red-500 rounded-full flex items-center justify-center active:scale-95 transition-transform shadow-lg shadow-red-500/20"
                      >
                        <motion.div
                          className="absolute inset-0 bg-red-500 rounded-full"
                          animate={{
                            scale: [1, 1.4, 1],
                            opacity: [0.5, 0, 0.5],
                          }}
                          transition={{ repeat: Infinity, duration: 1.5 }}
                        />
                        <Square className="w-8 h-8 text-white fill-current relative z-10" />
                      </button>
                      <span className="text-[14px] font-bold text-red-500 tracking-wide">
                        录音中...，点击结束
                      </span>
                    </div>
                  )}

                  {recordingState === "recorded" && (
                    <div className="flex items-center justify-center gap-6 sm:gap-10 w-full px-2">
                      <button
                        onClick={reRecord}
                        className="flex flex-col items-center gap-2.5 active:opacity-70 transition-opacity"
                      >
                        <div className="w-14 h-14 bg-chat-other-bg border border-border-color rounded-full flex items-center justify-center shadow-sm">
                          <RotateCcw className="w-6 h-6 text-text-sub" />
                        </div>
                        <span className="text-[13px] font-medium text-text-sub whitespace-nowrap">
                          重录
                        </span>
                      </button>

                      <button
                        onClick={togglePreview}
                        className="flex flex-col items-center gap-2.5 active:scale-95 transition-transform"
                      >
                        <div className="w-[72px] h-[72px] bg-primary-blue rounded-full flex items-center justify-center shadow-xl shadow-primary-blue/30 relative">
                          {isPreviewPlaying && (
                            <motion.div
                              className="absolute inset-0 border-[3px] border-primary-blue rounded-full"
                              animate={{
                                scale: [1, 1.3, 1],
                                opacity: [0.6, 0, 0.6],
                              }}
                              transition={{ repeat: Infinity, duration: 1.5 }}
                            />
                          )}
                          {isPreviewPlaying ? (
                            <Square className="w-8 h-8 text-white fill-current relative z-10" />
                          ) : (
                            <Play className="w-8 h-8 text-white fill-current ml-1 relative z-10" />
                          )}
                        </div>
                        <span className="text-[14px] font-bold text-primary-blue tracking-wide whitespace-nowrap">
                          {isPreviewPlaying ? "停止播放" : "试听录音"}
                        </span>
                      </button>

                      <button
                        onClick={cloneVoice}
                        className="flex flex-col items-center gap-2.5 active:opacity-70 transition-opacity"
                      >
                        <div className="w-14 h-14 bg-text-main rounded-full flex items-center justify-center shadow-md">
                          <Sparkles className="w-6 h-6 text-bg-color" />
                        </div>
                        <span className="text-[13px] font-bold text-text-main whitespace-nowrap">
                          生成声音
                        </span>
                      </button>
                    </div>
                  )}
                </div>
              </motion.div>
            )}

          {recordingState !== "processing" &&
            recordingState !== "done" &&
            activeTab === "upload" && (
              <motion.div
                key="upload-mode"
                initial={{ opacity: 0 }}
                animate={{ opacity: 1 }}
                exit={{ opacity: 0 }}
                className="w-full flex-1 flex flex-col items-center justify-center gap-8 h-full min-h-0"
              >
                <div className="w-32 h-32 bg-primary-blue/5 rounded-full flex items-center justify-center border-2 border-dashed border-primary-blue/30">
                  <UploadCloud className="w-12 h-12 text-primary-blue" />
                </div>
                <div className="text-center px-4">
                  <h3 className="text-[18px] font-bold text-text-main mb-2">
                    上传本地音频
                  </h3>
                  <p className="text-[14px] text-text-sub leading-relaxed">
                    请上传包含清晰人声的音频文件
                    <br />
                    建议时长 1 分钟到 3 分钟
                    <br />
                    支持 MP3, WAV, M4A 格式
                  </p>
                </div>
                <button
                  onClick={handleUpload}
                  className="px-10 py-3.5 bg-primary-blue text-white rounded-full font-bold text-[16px] shadow-lg shadow-primary-blue/20 active:opacity-80 transition-opacity whitespace-nowrap"
                >
                  选择文件并开始生成
                </button>
              </motion.div>
            )}
        </AnimatePresence>
      </div>
    </div>
  );
};
