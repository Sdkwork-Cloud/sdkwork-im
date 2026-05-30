import React, { useState, useEffect } from "react";
import { useNavigate } from "react-router";
import {
  ChevronLeft,
  Edit3,
  History,
  Loader2,
  Copy,
  Check,
  MessageSquareMore,
  Languages,
  Trash2,
  RefreshCw,
} from "lucide-react";
import { IconButton, showToast } from "@sdkwork/clawchat-mobile-commons";
import {
  AIWritingService,
  WritingTask,
  AIWritingOptions,
} from "../services/AIWritingService";
import { motion, AnimatePresence } from "motion/react";

export const AIWritingPage: React.FC = () => {
  const navigate = useNavigate();
  const [topic, setTopic] = useState("");
  const [style, setStyle] = useState("Professional");
  const [length, setLength] = useState<AIWritingOptions["length"]>("medium");
  const [language, setLanguage] =
    useState<AIWritingOptions["language"]>("Chinese");

  const [isGenerating, setIsGenerating] = useState(false);
  const [currentTask, setCurrentTask] = useState<WritingTask | null>(null);
  const [history, setHistory] = useState<WritingTask[]>([]);
  const [copied, setCopied] = useState(false);
  const [realtimeContent, setRealtimeContent] = useState("");

  const styles = [
    "Professional",
    "Casual",
    "Creative",
    "Academic",
    "Humorous",
    "Persuasive",
  ];
  const lengths: AIWritingOptions["length"][] = ["short", "medium", "long"];
  const languages: AIWritingOptions["language"][] = ["Chinese", "English"];

  useEffect(() => {
    AIWritingService.getHistory().then(setHistory);
  }, []);

  const handleGenerate = async () => {
    if (!topic.trim()) return showToast("请输入写作主题或要求");
    setIsGenerating(true);
    setRealtimeContent("");

    const options: AIWritingOptions = { topic, style, length, language };
    setCurrentTask({
      id: "temp",
      options,
      status: "generating",
      createdAt: Date.now(),
    });
    setCopied(false);

    try {
      const task = await AIWritingService.generateArticle(options, (chunk) => {
        setRealtimeContent(chunk);
      });
      setCurrentTask(task);
      setRealtimeContent("");
      setHistory((prev) => [task, ...prev.filter((t) => t.id !== "temp")]);
      showToast("创作完成！");
    } catch (err) {
      showToast("生成失败，请重试");
      setCurrentTask(null);
    } finally {
      setIsGenerating(false);
    }
  };

  const handleCopy = (content?: string) => {
    if (content) {
      navigator.clipboard.writeText(content);
      setCopied(true);
      showToast("已复制到剪贴板");
      setTimeout(() => setCopied(false), 2000);
    }
  };

  const handleDelete = (e: React.MouseEvent, id: string) => {
    e.stopPropagation();
    AIWritingService.deleteFromHistory(id);
    setHistory((prev) => prev.filter((t) => t.id !== id));
    if (currentTask?.id === id) {
      setCurrentTask(null);
      setTopic("");
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
          AI 智能写作
        </span>
        <IconButton
          icon={<History className="w-5 h-5 text-text-main" />}
          onClick={() => {
            if (history.length === 0) {
              showToast("暂无历史记录");
            } else {
              document.getElementById("history-section")?.scrollIntoView({ behavior: "smooth" });
            }
          }}
        />
      </header>

      <div className="flex-1 overflow-y-auto flex flex-col gap-4 relative pb-safe">
        <div className="bg-bg-color p-4 shadow-sm">
          <div className="flex flex-col gap-5">
            <div>
              <label className="text-sm font-medium text-text-main flex items-center gap-1.5 mb-2">
                <MessageSquareMore className="w-4 h-4 text-primary-blue" />
                Topic or Instructions
              </label>
              <div className="bg-input-bg border border-border-color rounded-xl p-3 focus-within:border-primary-blue transition-all shadow-sm">
                <textarea
                  className="w-full bg-transparent outline-none resize-none text-[15px] text-text-main min-h-[80px] placeholder-text-sub"
                  placeholder="E.g., Write a highly engaging paragraph about the future of remote work..."
                  value={topic}
                  onChange={(e) => setTopic(e.target.value)}
                />
              </div>
            </div>

            <div className="flex gap-4">
              <div className="flex-1">
                <label className="text-sm font-medium text-text-main block mb-2">
                  Length
                </label>
                <div className="flex bg-input-bg rounded-lg p-1 border border-border-color">
                  {lengths.map((l) => (
                    <button
                      key={l}
                      onClick={() => setLength(l)}
                      className={`flex-1 py-1.5 rounded-md text-[13px] font-medium capitalize transition-colors ${length === l ? "bg-bg-color shadow-sm text-text-main" : "text-text-sub"}`}
                    >
                      {l}
                    </button>
                  ))}
                </div>
              </div>
              <div className="flex-1">
                <label className="text-sm font-medium text-text-main flex items-center gap-1 mb-2">
                  <Languages className="w-4 h-4" /> Language
                </label>
                <div className="flex bg-input-bg rounded-lg p-1 border border-border-color">
                  {languages.map((l) => (
                    <button
                      key={l}
                      onClick={() => setLanguage(l)}
                      className={`flex-1 py-1.5 rounded-md text-[13px] font-medium transition-colors ${language === l ? "bg-bg-color shadow-sm text-text-main" : "text-text-sub"}`}
                    >
                      {l === "Chinese" ? "中文" : "ENG"}
                    </button>
                  ))}
                </div>
              </div>
            </div>

            <div>
              <label className="text-sm font-medium text-text-main block mb-2">
                Tone & Style
              </label>
              <div className="flex flex-wrap gap-2">
                {styles.map((s) => (
                  <button
                    key={s}
                    onClick={() => setStyle(s)}
                    className={`px-3 py-1.5 rounded-xl text-[13px] font-medium transition-colors ${style === s ? "bg-primary-blue text-white shadow-md shadow-primary-blue/20" : "bg-input-bg text-text-main border border-border-color hover:bg-active-bg"}`}
                  >
                    {s}
                  </button>
                ))}
              </div>
            </div>

            <button
              disabled={isGenerating || !topic.trim()}
              onClick={handleGenerate}
              className="w-full h-[48px] rounded-xl bg-gradient-to-r from-blue-600 to-indigo-600 text-white font-bold flex items-center justify-center gap-2 disabled:opacity-50 active:scale-[0.98] transition-all mt-1 shadow-md"
            >
              {isGenerating ? (
                <Loader2 className="w-5 h-5 animate-spin" />
              ) : (
                <Edit3 className="w-5 h-5" />
              )}
              {isGenerating ? "Drafting..." : "Start Writing"}
            </button>
          </div>
        </div>

        <div className="px-4 pb-6">
          <AnimatePresence>
            {currentTask && (
              <motion.div
                initial={{ opacity: 0, y: 20 }}
                animate={{ opacity: 1, y: 0 }}
                className="bg-bg-color rounded-2xl border border-border-color shadow-sm relative pt-12 overflow-hidden flex flex-col mb-6"
              >
                <div className="absolute top-0 left-0 right-0 h-10 border-b border-border-color flex items-center justify-between px-4 bg-active-bg">
                  <div className="flex space-x-1.5">
                    <div className="w-2.5 h-2.5 rounded-full bg-red-400"></div>
                    <div className="w-2.5 h-2.5 rounded-full bg-amber-400"></div>
                    <div className="w-2.5 h-2.5 rounded-full bg-green-400"></div>
                  </div>
                  <div className="flex items-center gap-3">
                    <span className="text-[11px] font-medium text-text-sub uppercase tracking-wider">
                      {currentTask.options.style}
                    </span>
                    {currentTask.status === "completed" && (
                      <>
                        <button
                          onClick={handleGenerate}
                          className="text-text-sub hover:text-primary-blue transition-colors active:scale-95 bg-bg-color p-1.5 rounded-md border border-border-color shadow-sm"
                          title="重新生成"
                        >
                          <RefreshCw className="w-3.5 h-3.5" />
                        </button>
                        <button
                          onClick={() => handleCopy(currentTask.content)}
                          className="text-text-sub hover:text-text-main transition-colors active:scale-95 bg-bg-color p-1.5 rounded-md border border-border-color shadow-sm"
                          title="复制文本"
                        >
                          {copied ? (
                            <Check className="w-3.5 h-3.5 text-green-500" />
                          ) : (
                            <Copy className="w-3.5 h-3.5" />
                          )}
                        </button>
                      </>
                    )}
                  </div>
                </div>

                <div className="p-4 min-h-[120px]">
                  {currentTask.status === "generating" && !realtimeContent ? (
                    <div className="flex flex-col items-center justify-center py-6 text-text-sub h-full">
                      <Loader2 className="w-8 h-8 animate-spin mb-3 text-primary-blue" />
                      <span className="text-[13px]">
                        Analyzing requirements...
                      </span>
                    </div>
                  ) : (
                    <div className="text-[15px] text-text-main leading-relaxed font-sans flex flex-col gap-2 relative whitespace-pre-wrap break-words">
                      {/* Very simple markdown parser for bold, italic and lists */}
                      {(realtimeContent || currentTask.content || "")
                        .split("\n")
                        .map((line, i) => {
                          const renderInlineText = (text: string) => {
                            // Match bold **text** or italic *text*
                            const parts = text.split(/(\*\*.*?\*\*|\*.*?\*)/g);
                            return parts.map((part, idx) => {
                              if (
                                part.startsWith("**") &&
                                part.endsWith("**")
                              ) {
                                return (
                                  <strong key={idx}>{part.slice(2, -2)}</strong>
                                );
                              } else if (
                                part.startsWith("*") &&
                                part.endsWith("*")
                              ) {
                                return <em key={idx}>{part.slice(1, -1)}</em>;
                              }
                              return (
                                <span
                                  key={idx}
                                  dangerouslySetInnerHTML={{ __html: part }}
                                />
                              );
                            });
                          };

                          if (line.startsWith("# ")) {
                            return (
                              <h1
                                key={i}
                                className="text-xl font-bold mt-4 mb-2"
                              >
                                {renderInlineText(line.replace("# ", ""))}
                              </h1>
                            );
                          } else if (line.startsWith("## ")) {
                            return (
                              <h2
                                key={i}
                                className="text-lg font-bold mt-3 mb-1"
                              >
                                {renderInlineText(line.replace("## ", ""))}
                              </h2>
                            );
                          } else if (line.startsWith("### ")) {
                            return (
                              <h3
                                key={i}
                                className="text-base font-bold mt-2 mb-1"
                              >
                                {renderInlineText(line.replace("### ", ""))}
                              </h3>
                            );
                          } else if (line.startsWith("*Conclusion*")) {
                            return (
                              <strong
                                key={i}
                                className="italic block mt-3 mb-1"
                              >
                                Conclusion
                              </strong>
                            );
                          } else if (line.match(/^(\d+\.|-)\s/)) {
                            return (
                              <p
                                key={i}
                                className="pl-4 relative before:content-['•'] before:absolute before:left-0 before:text-text-sub my-1"
                              >
                                {renderInlineText(
                                  line.replace(/^(\d+\.|-)\s/, ""),
                                )}
                              </p>
                            );
                          } else if (line.trim() === "") {
                            return <br key={i} />;
                          }
                          return (
                            <p key={i} className="min-h-[1em] mb-2">
                              {renderInlineText(line)}
                            </p>
                          );
                        })}

                      {currentTask.status === "generating" && (
                        <span className="inline-block w-2 bg-primary-blue h-[15px] animate-pulse ml-1 align-middle" />
                      )}
                    </div>
                  )}
                </div>
              </motion.div>
            )}
          </AnimatePresence>

          {!isGenerating && (
            <div id="history-section" className="flex flex-col gap-3">
              {history.length > 0 ? (
                <>
                  <h3 className="text-[16px] font-bold text-text-main">
                    History
                  </h3>
                  <div className="flex flex-col gap-3">
                    {history.map((item, index) => (
                      <div
                        key={item.id}
                        className="bg-bg-color border border-border-color rounded-xl p-3 shadow-sm cursor-pointer active:bg-active-bg transition-colors"
                        onClick={() => {
                          setTopic(item.options.topic);
                          setStyle(item.options.style);
                          setLength(item.options.length);
                          setLanguage(item.options.language);
                          setCurrentTask(item);
                          setRealtimeContent("");
                        }}
                      >
                        <div className="flex justify-between items-start mb-2 pr-6 relative group">
                          <span className="text-[14px] font-bold text-text-main line-clamp-1 flex-1">
                            {item.options.topic}
                          </span>
                          <span className="text-[10px] bg-active-bg text-text-sub px-2 py-0.5 rounded ml-2 whitespace-nowrap shrink-0">
                            {item.options.style}
                          </span>
                          <button
                            onClick={(e) => handleDelete(e, item.id)}
                            className="absolute top-0 right-0 text-text-sub opacity-50 hover:opacity-100 transition-opacity active:text-red-500 hover:text-red-500 z-10 p-1"
                          >
                            <Trash2 className="w-4 h-4" />
                          </button>
                        </div>
                        <p className="text-[12px] text-text-sub line-clamp-2 leading-relaxed">
                          {item.content}
                        </p>
                      </div>
                    ))}
                  </div>
                </>
              ) : !currentTask ? (
                <div className="pt-6 flex flex-col items-center justify-center opacity-70">
                  <MessageSquareMore className="w-12 h-12 text-text-sub mb-3 opacity-50" />
                  <h3 className="text-sm font-medium text-text-sub mb-4">
                    No recent history. Try these:
                  </h3>
                  <div className="flex flex-wrap gap-2 justify-center px-4">
                    {[
                      "A creative story about Mars",
                      "Professional email to a client",
                      "Casual blog about coffee",
                      "The impact of AI on design",
                      "How to learn React fast",
                    ].map((suggestion, i) => (
                      <button
                        key={i}
                        onClick={() => setTopic(suggestion)}
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
