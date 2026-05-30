import React, { useState, useEffect } from "react";
import { useNavigate } from "react-router";
import {
  ChevronLeft,
  Image as ImageIcon,
  History,
  Loader2,
  Sparkles,
  Download,
  Settings2,
  Trash2,
} from "lucide-react";
import { IconButton, cn, showToast } from "@sdkwork/clawchat-mobile-commons";
import {
  AIImageService,
  ImageTask,
  AIImageOptions,
} from "../services/AIImageService";
import { motion, AnimatePresence } from "motion/react";

export const AIImagePage: React.FC = () => {
  const navigate = useNavigate();
  const [prompt, setPrompt] = useState("");
  const [negativePrompt, setNegativePrompt] = useState("");
  const [aspectRatio, setAspectRatio] =
    useState<AIImageOptions["aspectRatio"]>("1:1");
  const [style, setStyle] = useState("Photography");
  const [showAdvanced, setShowAdvanced] = useState(false);

  const [isGenerating, setIsGenerating] = useState(false);
  const [currentProgress, setCurrentProgress] = useState(0);
  const [currentTask, setCurrentTask] = useState<ImageTask | null>(null);
  const [history, setHistory] = useState<ImageTask[]>([]);
  const [isOptimizingPrompt, setIsOptimizingPrompt] = useState(false);

  const handleOptimizePrompt = async () => {
    if (!prompt.trim()) return showToast("请输入创作提示词以进行优化");
    setIsOptimizingPrompt(true);
    try {
      const optimized = await AIImageService.optimizePrompt(prompt);
      setPrompt(optimized);
      showToast("提示词优化成功！");
    } catch (e) {
      showToast("优化失败，请重试");
    } finally {
      setIsOptimizingPrompt(false);
    }
  };

  const styles = [
    "Photography",
    "Anime",
    "Cyberpunk",
    "Oil Painting",
    "3D Render",
    "Pixel Art",
  ];
  const ratios: AIImageOptions["aspectRatio"][] = [
    "1:1",
    "16:9",
    "9:16",
    "4:3",
  ];

  useEffect(() => {
    AIImageService.getHistory().then(setHistory);
  }, []);

  const handleGenerate = async () => {
    if (!prompt.trim()) return showToast("请输入创作提示词");
    setIsGenerating(true);
    setCurrentProgress(0);

    const options: AIImageOptions = {
      prompt,
      negativePrompt,
      aspectRatio,
      style,
    };
    setCurrentTask({
      id: "temp",
      options,
      status: "generating",
      progress: 0,
      createdAt: Date.now(),
    });

    try {
      const task = await AIImageService.generateImage(options, (p) =>
        setCurrentProgress(p),
      );
      setCurrentTask(task);
      setHistory((prev) => [task, ...prev.filter((t) => t.id !== "temp")]);
      showToast("图片生成完成！");
    } catch (err) {
      showToast("生成失败，请重试");
      setCurrentTask(null);
    } finally {
      setIsGenerating(false);
      setCurrentProgress(0);
    }
  };

  const downloadImage = async (url?: string) => {
    if (!url) return;
    try {
      const resp = await fetch(url);
      const blob = await resp.blob();
      const objUrl = window.URL.createObjectURL(blob);
      const a = document.createElement("a");
      a.href = objUrl;
      a.download = `ai_image_${Date.now()}.png`;
      document.body.appendChild(a);
      a.click();
      document.body.removeChild(a);
      window.URL.revokeObjectURL(objUrl);
      showToast("保存成功");
    } catch (e) {
      console.warn("Fetch failed, opening in new tab", e);
      window.open(url, "_blank", "noopener,noreferrer");
      showToast("将用浏览器打开下载");
    }
  };

  const handleDelete = (e: React.MouseEvent, id: string) => {
    e.stopPropagation();
    AIImageService.deleteFromHistory(id);
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
        <span className="font-medium text-[17px] text-text-main">AI 绘画</span>
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
        {/* Settings Block */}
        <div className="bg-bg-color p-4 shadow-sm flex flex-col gap-4">
          <div>
            <label className="text-sm font-medium text-text-main flex items-center justify-between mb-2">
              <span>
                Prompt <span className="text-red-500">*</span>
              </span>
            </label>
            <div className="bg-input-bg border border-border-color rounded-2xl p-3 focus-within:border-[#07C160] transition-colors relative">
              <textarea
                className="w-full bg-transparent outline-none resize-none text-[15px] text-text-main min-h-[90px] placeholder-text-sub"
                placeholder="A magical forest with glowing plants, cinematic lighting, 8k..."
                value={prompt}
                onChange={(e) => setPrompt(e.target.value)}
              />
              <button
                onClick={handleOptimizePrompt}
                disabled={isOptimizingPrompt}
                className="absolute bottom-3 right-3 text-[#07C160] bg-[#07C160]/10 p-1.5 rounded-full hover:bg-[#07C160]/20 transition-colors disabled:opacity-40"
                title="Optimize prompt"
              >
                <Sparkles className={cn("w-4 h-4", isOptimizingPrompt && "animate-spin")} />
              </button>
            </div>
          </div>

          <div className="flex gap-2">
            {ratios.map((ratio) => (
              <button
                key={ratio}
                onClick={() => setAspectRatio(ratio)}
                className={`flex-1 py-1.5 rounded-lg border text-[13px] font-medium transition-all ${aspectRatio === ratio ? "border-[#07C160] text-[#07C160] bg-[#07C160]/5" : "border-border-color text-text-sub bg-input-bg"}`}
              >
                {ratio}
              </button>
            ))}
          </div>

          <div>
            <label className="text-sm font-medium text-text-main block mb-2">
              Art Style
            </label>
            <div className="flex overflow-x-auto no-scrollbar gap-2 pb-1 -mx-2 px-2">
              {styles.map((s) => (
                <button
                  key={s}
                  onClick={() => setStyle(s)}
                  className={`px-4 py-1.5 rounded-full text-[13px] shrink-0 whitespace-nowrap transition-colors ${style === s ? "bg-[#07C160] text-white font-medium" : "bg-input-bg text-text-main border border-border-color active:bg-active-bg"}`}
                >
                  {s}
                </button>
              ))}
            </div>
          </div>

          <div className="pt-2 border-t border-border-color">
            <button
              className="flex items-center gap-1.5 text-sm text-text-sub font-medium active:opacity-70 transition-opacity"
              onClick={() => setShowAdvanced(!showAdvanced)}
            >
              <Settings2 className="w-4 h-4" />
              Advanced Settings
            </button>

            <AnimatePresence>
              {showAdvanced && (
                <motion.div
                  initial={{ height: 0, opacity: 0 }}
                  animate={{ height: "auto", opacity: 1 }}
                  exit={{ height: 0, opacity: 0 }}
                  className="overflow-hidden"
                >
                  <div className="pt-3">
                    <label className="text-[13px] text-text-main mb-1.5 block">
                      Negative Prompt
                    </label>
                    <input
                      type="text"
                      className="w-full bg-input-bg border border-border-color rounded-xl px-3 py-2 text-[14px] text-text-main focus:border-[#07C160] outline-none transition-colors"
                      placeholder="ugly, blurry, low res, bad anatomy..."
                      value={negativePrompt}
                      onChange={(e) => setNegativePrompt(e.target.value)}
                    />
                  </div>
                </motion.div>
              )}
            </AnimatePresence>
          </div>

          <button
            disabled={isGenerating || !prompt.trim()}
            onClick={handleGenerate}
            className="w-full h-[46px] rounded-xl bg-[#07C160] text-white font-bold flex items-center justify-center gap-2 disabled:opacity-50 active:scale-[0.98] transition-all shadow-sm shadow-[#07C160]/20"
          >
            {isGenerating ? (
              <Loader2 className="w-5 h-5 animate-spin" />
            ) : (
              <ImageIcon className="w-5 h-5" />
            )}
            {isGenerating ? "Drawing..." : "Generate Image"}
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
                <div className="flex justify-between items-center bg-bg-color px-4 py-2.5 rounded-t-xl border-x border-t border-border-color shadow-sm -mb-4 z-10 relative">
                  <h3 className="text-[15px] font-bold text-text-main bg-gradient-to-r from-[#07C160] to-teal-500 bg-clip-text text-transparent">
                    Generated Art
                  </h3>
                  {currentTask.status === "completed" && (
                    <button
                      onClick={() => {
                        downloadImage(currentTask.imageUrl);
                      }}
                      className="text-[#576B95] text-sm font-medium flex items-center gap-1 active:opacity-70 bg-[#576B95]/10 px-3 py-1.5 rounded-lg border border-[#576B95]/20 shadow-sm"
                    >
                      <Download className="w-3.5 h-3.5" /> 保存图片
                    </button>
                  )}
                </div>

                <div
                  className={cn(
                    "bg-input-bg rounded-b-xl rounded-t-sm overflow-hidden border border-border-color relative flex items-center justify-center mx-auto shadow-md",
                    currentTask.options.aspectRatio === "16:9"
                      ? "w-full aspect-video"
                      : currentTask.options.aspectRatio === "9:16"
                        ? "w-[75%] aspect-[9/16]"
                        : currentTask.options.aspectRatio === "4:3"
                          ? "w-full aspect-[4/3]"
                          : "w-full aspect-square",
                  )}
                >
                  {currentTask.status === "generating" ? (
                    <div className="flex flex-col items-center justify-center text-text-sub w-full h-full p-8 absolute inset-0 bg-bg-color/50 backdrop-blur-md">
                      <Loader2 className="w-10 h-10 animate-spin mb-4 text-[#07C160]" />
                      <div className="w-[120px] max-w-full h-1.5 bg-border-color rounded-full overflow-hidden mb-2">
                        <div
                          className="h-full bg-[#07C160] transition-all duration-300"
                          style={{ width: `${currentProgress}%` }}
                        />
                      </div>
                      <span className="text-[13px] font-medium text-[#07C160]">
                        {currentProgress}%
                      </span>
                    </div>
                  ) : (
                    <img
                      src={currentTask.imageUrl}
                      alt="Generated Result"
                      className="w-full h-full object-cover"
                    />
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
                    className="text-[16px] font-bold text-text-main pb-1"
                  >
                    Gallery
                  </h3>
                  <div className="grid grid-cols-2 lg:grid-cols-3 gap-3">
                    {history.map((item) => (
                      <div
                        key={item.id}
                        className="rounded-xl overflow-hidden border border-border-color relative cursor-pointer group shadow-sm bg-bg-color"
                        style={{
                          aspectRatio: item.options.aspectRatio.replace(
                            ":",
                            "/",
                          ),
                        }}
                        onClick={() => {
                          setPrompt(item.options.prompt);
                          setAspectRatio(item.options.aspectRatio);
                          setStyle(item.options.style);
                          setCurrentTask(item);
                        }}
                      >
                        <img
                          src={item.imageUrl}
                          className="w-full h-full object-cover group-active:scale-[1.02] transition-transform duration-300"
                        />
                        <div className="absolute inset-0 bg-gradient-to-t from-black/70 via-black/10 to-transparent opacity-0 group-hover:opacity-100 group-active:opacity-100 transition-opacity flex flex-col justify-end p-2.5">
                          <span className="text-[9px] font-medium text-white/80 uppercase tracking-wider mb-0.5">
                            {item.options.style}
                          </span>
                          <p className="text-[11px] text-white line-clamp-2 leading-tight">
                            {item.options.prompt}
                          </p>
                        </div>
                        <button
                          onClick={(e) => handleDelete(e, item.id)}
                          className="absolute top-1.5 right-1.5 bg-black/40 p-1.5 rounded-full text-white/80 hover:bg-black/80 hover:text-red-400 active:text-red-400 active:bg-black/80 backdrop-blur z-10 opacity-0 group-hover:opacity-100 group-active:opacity-100 transition-opacity"
                        >
                          <Trash2 className="w-3.5 h-3.5" />
                        </button>
                      </div>
                    ))}
                  </div>
                </>
              ) : !currentTask ? (
                <div className="pt-6 flex flex-col items-center justify-center opacity-70">
                  <ImageIcon className="w-12 h-12 text-text-sub mb-3 opacity-50" />
                  <h3 className="text-sm font-medium text-text-sub mb-4">
                    No recent art. Try these:
                  </h3>
                  <div className="flex flex-wrap gap-2 justify-center px-4">
                    {[
                      "A cyberpunk city with flying cars",
                      "A cute cat wearing sunglasses",
                      "A magical glowing forest",
                      "A cozy cabin in the snow",
                      "An astronaut on Mars",
                    ].map((suggestion, i) => (
                      <button
                        key={i}
                        onClick={() => setPrompt(suggestion)}
                        className="bg-active-bg border border-border-color px-3 py-1.5 rounded-full text-xs text-text-main hover:border-[#07C160] transition-colors active:scale-95"
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
