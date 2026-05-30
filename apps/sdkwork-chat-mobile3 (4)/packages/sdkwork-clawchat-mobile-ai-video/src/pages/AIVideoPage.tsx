import React, { useState, useEffect } from "react";
import { useNavigate } from "react-router";
import {
  ChevronLeft,
  Wand2,
  History,
  PlayCircle,
  Loader2,
  Trash2,
  Download,
} from "lucide-react";
import { IconButton, showToast } from "@sdkwork/clawchat-mobile-commons";
import {
  AIVideoService,
  VideoTask,
  AIVideoOptions,
} from "../services/AIVideoService";
import { motion, AnimatePresence } from "motion/react";

export const AIVideoPage: React.FC = () => {
  const navigate = useNavigate();
  const [prompt, setPrompt] = useState("");
  const [style, setStyle] = useState("Cinematic");
  const [aspectRatio, setAspectRatio] =
    useState<AIVideoOptions["aspectRatio"]>("16:9");

  const [isGenerating, setIsGenerating] = useState(false);
  const [currentProgress, setCurrentProgress] = useState(0);
  const [currentTask, setCurrentTask] = useState<VideoTask | null>(null);
  const [history, setHistory] = useState<VideoTask[]>([]);

  const styles = ["Cinematic", "Anime", "3D Animation", "Drone", "Time Lapse"];
  const ratios: AIVideoOptions["aspectRatio"][] = ["16:9", "9:16", "1:1"];

  useEffect(() => {
    AIVideoService.getHistory().then(setHistory);
  }, []);

  const handleGenerate = async () => {
    if (!prompt.trim()) return showToast("请输入创作提示词");
    setIsGenerating(true);
    setCurrentProgress(0);

    const options: AIVideoOptions = { prompt, style, aspectRatio };
    setCurrentTask({
      id: "temp",
      options,
      status: "generating",
      progress: 0,
      createdAt: Date.now(),
      estimatedTimeSec: 15,
    });

    try {
      const task = await AIVideoService.generateVideo(options, (p) => {
        setCurrentProgress(p);
      });
      setCurrentTask(task);
      setHistory((prev) => [task, ...prev.filter((t) => t.id !== "temp")]);
      showToast("视频生成完成！");
    } catch (err) {
      showToast("生成失败，请重试");
      setCurrentTask(null);
    } finally {
      setIsGenerating(false);
      setCurrentProgress(0);
    }
  };

  const downloadVideo = async (url?: string) => {
    if (!url) return;
    try {
      const resp = await fetch(url);
      const blob = await resp.blob();
      const objUrl = window.URL.createObjectURL(blob);
      const a = document.createElement("a");
      a.href = objUrl;
      a.download = `ai_video_${Date.now()}.mp4`;
      document.body.appendChild(a);
      a.click();
      document.body.removeChild(a);
      window.URL.revokeObjectURL(objUrl);
      showToast("保存成功");
    } catch (e) {
      console.warn("Fetch failed, opening in new tab", e);
      window.open(url, "_blank");
      showToast("将用浏览器打开下载");
    }
  };

  const handleDelete = (e: React.MouseEvent, id: string) => {
    e.stopPropagation();
    AIVideoService.deleteFromHistory(id);
    setHistory((prev) => prev.filter((t) => t.id !== id));
    if (currentTask?.id === id) {
      setCurrentTask(null);
      setPrompt("");
    }
  };

  return (
    <div className="flex flex-col h-full bg-[#f4f6f9] dark:bg-black pt-safe">
      <header className="h-[44px] flex items-center justify-between px-2 shrink-0 bg-bg-color border-b border-border-color">
        <IconButton
          icon={<ChevronLeft className="w-6 h-6 text-text-main" />}
          onClick={() => navigate(-1)}
        />
        <span className="font-medium text-[17px] text-text-main">
          AI 视频创作
        </span>
        <IconButton
          icon={<History className="w-5 h-5 text-text-main" />}
          onClick={() =>
            document
              .getElementById("history-section")
              ?.scrollIntoView({ behavior: "smooth" })
          }
        />
      </header>

      <div className="flex-1 overflow-y-auto flex flex-col gap-4 pb-safe">
        {/* Settings Panel */}
        <div className="bg-bg-color p-4 shadow-sm flex flex-col gap-4">
          <div>
            <label className="text-sm font-medium text-text-main flex items-center justify-between mb-2">
              <span>
                Prompt{" "}
                <span className="text-xs bg-[#e8eaf6] text-primary-blue px-1.5 py-0.5 rounded ml-1">
                  Beta
                </span>
              </span>
            </label>
            <div className="bg-input-bg border border-border-color rounded-xl p-3 focus-within:border-primary-blue transition-colors">
              <textarea
                className="w-full bg-transparent outline-none resize-none text-[15px] text-text-main min-h-[80px] placeholder-text-sub"
                placeholder="A futuristic city with flying cars at sunset..."
                value={prompt}
                onChange={(e) => setPrompt(e.target.value)}
              />
            </div>
          </div>

          <div>
            <label className="text-sm font-medium text-text-main block mb-2">
              Style
            </label>
            <div className="flex overflow-x-auto no-scrollbar gap-2 pb-1 -mx-2 px-2">
              {styles.map((s) => (
                <button
                  key={s}
                  onClick={() => setStyle(s)}
                  className={`px-4 py-1.5 rounded-full text-sm shrink-0 whitespace-nowrap transition-colors ${style === s ? "bg-primary-blue text-white font-medium" : "bg-input-bg text-text-main border border-border-color active:bg-active-bg"}`}
                >
                  {s}
                </button>
              ))}
            </div>
          </div>

          <div>
            <label className="text-sm font-medium text-text-main block mb-2">
              Format
            </label>
            <div className="flex gap-2">
              {ratios.map((r) => (
                <button
                  key={r}
                  onClick={() => setAspectRatio(r)}
                  className={`flex-1 py-1.5 rounded-[10px] text-sm font-medium transition-colors ${aspectRatio === r ? "bg-primary-blue/10 text-primary-blue border border-primary-blue/30" : "bg-input-bg text-text-sub border border-border-color"}`}
                >
                  {r}
                </button>
              ))}
            </div>
          </div>

          <button
            disabled={isGenerating || !prompt.trim()}
            onClick={handleGenerate}
            className="w-full h-[46px] rounded-xl bg-primary-blue text-white font-bold flex items-center justify-center gap-2 disabled:opacity-50 active:scale-[0.98] transition-all mt-2 shadow-sm"
          >
            {isGenerating ? (
              <Loader2 className="w-5 h-5 animate-spin" />
            ) : (
              <Wand2 className="w-5 h-5" />
            )}
            {isGenerating ? "Generating..." : "Generate Video"}
          </button>
        </div>

        <div className="px-4 pb-6">
          <AnimatePresence>
            {currentTask && (
              <motion.div
                initial={{ opacity: 0, y: 20 }}
                animate={{ opacity: 1, y: 0 }}
                className="flex flex-col gap-3 mb-6"
              >
                <div className="flex items-center justify-between">
                  <h3 className="text-[16px] font-bold text-text-main">
                    Result
                  </h3>
                  {isGenerating ? (
                    <span className="text-xs text-primary-blue font-medium bg-primary-blue/10 px-2 py-0.5 rounded-full animate-pulse">
                      Processing {Math.round(currentProgress)}%
                    </span>
                  ) : (
                    <button
                      onClick={() => downloadVideo(currentTask.videoUrl)}
                      className="text-text-sub hover:text-text-main active:scale-95 transition-all flex items-center gap-1.5 bg-bg-color border border-border-color px-2.5 py-1.5 rounded-lg shadow-sm"
                    >
                      <Download className="w-3.5 h-3.5" />
                      <span className="text-xs font-medium">保存</span>
                    </button>
                  )}
                </div>

                <div
                  className={`bg-black rounded-2xl overflow-hidden relative border border-border-color flex items-center justify-center shadow-md mx-auto ${currentTask.options.aspectRatio === "9:16" ? "w-[70%] aspect-[9/16]" : currentTask.options.aspectRatio === "1:1" ? "w-full aspect-square" : "w-full aspect-video"}`}
                >
                  {currentTask.status === "generating" ? (
                    <div className="flex flex-col items-center justify-center absolute inset-0 bg-black/80 backdrop-blur-sm z-10 text-white gap-3">
                      <div className="w-12 h-12 relative flex items-center justify-center">
                        <div className="absolute inset-0 border-4 border-white/20 rounded-full" />
                        <div className="absolute inset-0 border-4 border-primary-blue rounded-full border-t-transparent animate-spin" />
                      </div>
                      <div className="text-sm tracking-widest font-mono">
                        RENDERING
                      </div>
                      <div className="w-3/4 max-w-[200px] h-1.5 bg-white/20 rounded-full overflow-hidden mt-1">
                        <div
                          className="h-full bg-primary-blue transition-all duration-300"
                          style={{ width: `${currentProgress}%` }}
                        />
                      </div>
                    </div>
                  ) : (
                    <>
                      <video
                        src={currentTask.videoUrl}
                        poster={currentTask.thumbnailUrl}
                        controls
                        autoPlay
                        loop
                        className="w-full h-full object-contain bg-black"
                      />
                      <div className="absolute top-3 left-3 bg-black/60 backdrop-blur text-white/90 text-[11px] font-medium px-2 py-1 rounded">
                        {currentTask.options.style}
                      </div>
                    </>
                  )}
                </div>
              </motion.div>
            )}
          </AnimatePresence>

          {!isGenerating && (
            <div className="flex flex-col gap-3">
              {history.length > 0 ? (
                <>
                  <h3
                    id="history-section"
                    className="text-[16px] font-bold text-text-main"
                  >
                    History
                  </h3>
                  <div className="grid grid-cols-2 gap-3">
                    {history.map((item) => (
                      <div
                        key={item.id}
                        className="flex flex-col gap-2 group cursor-pointer"
                        onClick={() => {
                          setPrompt(item.options.prompt);
                          setStyle(item.options.style);
                          setAspectRatio(item.options.aspectRatio);
                          setCurrentTask(item);
                        }}
                      >
                        <div
                          className={`rounded-xl overflow-hidden relative border border-border-color bg-black ${item.options.aspectRatio === "9:16" ? "aspect-[9/16]" : item.options.aspectRatio === "1:1" ? "aspect-square" : "aspect-video"}`}
                        >
                          {item.thumbnailUrl?.includes("video") ||
                          item.thumbnailUrl?.endsWith(".mp4") ? (
                            <video
                              src={item.thumbnailUrl}
                              className="w-full h-full object-cover opacity-80 group-active:opacity-100 transition-opacity"
                              muted
                              playsInline
                            />
                          ) : (
                            <img
                              src={item.thumbnailUrl}
                              className="w-full h-full object-cover opacity-80 group-active:opacity-100 transition-opacity"
                            />
                          )}
                          <div className="absolute inset-0 flex items-center justify-center">
                            <PlayCircle className="w-8 h-8 text-white/60 drop-shadow-md" />
                          </div>
                          <button
                            onClick={(e) => handleDelete(e, item.id)}
                            className="absolute top-1.5 right-1.5 bg-black/40 p-1.5 rounded-full text-white/80 hover:bg-black/80 hover:text-red-400 active:text-red-400 active:bg-black/80 backdrop-blur z-10 opacity-0 group-hover:opacity-100 md:opacity-100 transition-opacity"
                          >
                            <Trash2 className="w-3.5 h-3.5" />
                          </button>
                        </div>
                        <p className="text-xs text-text-main line-clamp-2 px-1 font-medium">
                          {item.options.prompt}
                        </p>
                      </div>
                    ))}
                  </div>
                </>
              ) : !currentTask ? (
                <div className="pt-6 flex flex-col items-center justify-center opacity-70">
                  <PlayCircle className="w-12 h-12 text-text-sub mb-3 opacity-50" />
                  <h3 className="text-sm font-medium text-text-sub mb-4">
                    No recent videos. Try these:
                  </h3>
                  <div className="flex flex-wrap gap-2 justify-center px-4">
                    {[
                      "A futuristic city at night",
                      "A quiet forest stream",
                      "Beautiful ocean waves",
                      "A blooming yellow flower",
                      "A cute dog playing in water",
                    ].map((suggestion, i) => (
                      <button
                        key={i}
                        onClick={() => setPrompt(suggestion)}
                        className="bg-active-bg border border-border-color px-3 py-1.5 rounded-full text-xs text-text-main hover:border-primary-blue transition-colors active:scale-95"
                      >
                        {suggestion}
                      </button>
                    ))}
                  </div>
                </div>
              ) : null}
            </div>
          )}
        </div>
      </div>
    </div>
  );
};
