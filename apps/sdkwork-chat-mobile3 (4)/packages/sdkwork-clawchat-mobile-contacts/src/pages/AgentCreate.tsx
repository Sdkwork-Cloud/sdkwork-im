import React, { useState, useRef } from "react";
import { useNavigate } from "react-router";
import {
  ChevronLeft,
  Camera,
  Bot,
  Sparkles,
  MessageSquare,
  Settings2,
  ChevronRight,
  FileText,
  UploadCloud,
  Mic,
  Globe,
  Image as ImageIcon,
  Search,
  X,
  PlusCircle,
  Play,
  Square,
} from "lucide-react";
import { IconButton, cn, showToast } from "@sdkwork/clawchat-mobile-commons";
import { motion, AnimatePresence } from "motion/react";
import { AgentService } from "../services/AgentService";
import {
  VoiceService,
  type VoiceCategory,
} from "@sdkwork/clawchat-mobile-user";
import { VoiceSelectionPage } from "../components/VoiceSelectionPage";

export const AgentCreate: React.FC = () => {
  const navigate = useNavigate();
  const [name, setName] = useState("");
  const [prompt, setPrompt] = useState("");
  const [greeting, setGreeting] = useState("");

  // Advanced Settings State
  const [showAdvanced, setShowAdvanced] = useState(false);
  const [temperature, setTemperature] = useState(0.7);
  const [maxTokens, setMaxTokens] = useState(2048);
  const [voice, setVoice] = useState({ id: "female1", label: "温柔女声" });
  const [tools, setTools] = useState({ webSearch: true, imageGen: false });
  const [isCreating, setIsCreating] = useState(false);
  const [showVoiceSelection, setShowVoiceSelection] = useState(false);
  const [avatarPreview, setAvatarPreview] = useState<string | null>(null);
  const fileInputRef = useRef<HTMLInputElement>(null);

  const handleAvatarSelect = (e: React.ChangeEvent<HTMLInputElement>) => {
    const file = e.target.files?.[0];
    if (file) {
      const url = URL.createObjectURL(file);
      setAvatarPreview(url);
    }
  };

  const handleCreate = async () => {
    if (!name.trim() || !prompt.trim() || isCreating) return;
    setIsCreating(true);
    try {
      const chat = await AgentService.createAgentChat(
        name.trim(),
        greeting.trim(),
      );
      navigate(`/chat/${chat.id}`, { replace: true });
    } catch (error) {
      console.error(error);
      showToast("创建失败");
      setIsCreating(false);
    }
  };

  const Switch = ({
    checked,
    onChange,
  }: {
    checked: boolean;
    onChange: (c: boolean) => void;
  }) => (
    <div
      onClick={(e) => {
        e.stopPropagation();
        onChange(!checked);
      }}
      className={cn(
        "w-12 h-6 rounded-full transition-colors relative cursor-pointer shrink-0",
        checked ? "bg-primary-blue" : "bg-gray-300 dark:bg-gray-600",
      )}
    >
      <div
        className={cn(
          "absolute top-1 w-4 h-4 rounded-full bg-white transition-transform shadow-sm",
          checked ? "left-7" : "left-1",
        )}
      />
    </div>
  );

  return (
    <div className="flex flex-col h-full bg-bg-color overflow-y-auto">
      {/* Header */}
      <header className="h-[56px] flex items-center justify-between px-1 glass-header sticky top-0 z-10 shrink-0 pt-safe">
        <div className="flex items-center z-10 flex-1">
          <IconButton
            icon={<ChevronLeft className="w-6 h-6 text-text-main" />}
            onClick={() => navigate(-1)}
          />
        </div>
        <div className="absolute left-1/2 -translate-x-1/2 flex items-center justify-center pointer-events-none">
          <h2 className="text-[17px] font-medium text-text-main">创建智能体</h2>
        </div>
        <div className="flex items-center justify-end z-10 flex-1 pr-3">
          <button
            onClick={handleCreate}
            disabled={!name.trim() || !prompt.trim() || isCreating}
            className={cn(
              "px-3 py-1.5 rounded-md text-[14px] font-medium transition-colors",
              name.trim() && prompt.trim() && !isCreating
                ? "bg-primary-blue text-white active:bg-blue-600"
                : "bg-black/5 dark:bg-white/5 text-text-sub cursor-not-allowed",
            )}
          >
            {isCreating ? "创建中..." : "完成"}
          </button>
        </div>
      </header>

      <div className="flex flex-col px-4 py-6 gap-6 pb-[84px]">
        {/* Avatar Upload */}
        <div className="flex flex-col items-center gap-3">
          <input
            type="file"
            accept="image/*"
            className="hidden"
            ref={fileInputRef}
            onChange={handleAvatarSelect}
          />
          <div
            onClick={() => fileInputRef.current?.click()}
            className="w-20 h-20 rounded-2xl bg-chat-other-bg border border-border-color flex items-center justify-center relative overflow-hidden cursor-pointer active:scale-95 transition-transform group"
          >
            {avatarPreview ? (
              <img
                src={avatarPreview}
                alt="Avatar Preview"
                className="w-full h-full object-cover"
              />
            ) : (
              <Bot className="w-10 h-10 text-text-sub opacity-50" />
            )}
            <div className="absolute inset-0 bg-black/40 opacity-0 group-hover:opacity-100 flex items-center justify-center transition-opacity">
              <Camera className="w-6 h-6 text-white drop-shadow-md" />
            </div>
          </div>
          <span className="text-[13px] text-text-sub">设置头像</span>
        </div>

        {/* Form Fields */}
        <div className="flex flex-col gap-4">
          {/* Name */}
          <div className="flex flex-col gap-2">
            <label className="text-[14px] font-medium text-text-main ml-1">
              智能体名称
            </label>
            <div className="bg-chat-other-bg rounded-xl px-4 py-3 border border-border-color focus-within:border-primary-blue transition-colors">
              <input
                type="text"
                placeholder="例如：前端开发助手"
                className="w-full bg-transparent text-[16px] text-text-main focus:outline-none placeholder:text-text-sub"
                value={name}
                onChange={(e) => setName(e.target.value)}
              />
            </div>
          </div>

          {/* Prompt/Persona */}
          <div className="flex flex-col gap-2">
            <div className="flex items-center justify-between ml-1">
              <label className="text-[14px] font-medium text-text-main flex items-center gap-1.5">
                <Sparkles className="w-4 h-4 text-primary-blue" />
                人设与回复逻辑
              </label>
              <span className="text-[12px] text-text-sub">
                {prompt.length}/2000
              </span>
            </div>
            <div className="bg-chat-other-bg rounded-xl px-4 py-3 border border-border-color focus-within:border-primary-blue transition-colors">
              <textarea
                placeholder="详细描述这个智能体的角色、专业领域、语气和回复规则..."
                className="w-full bg-transparent text-[16px] text-text-main focus:outline-none placeholder:text-text-sub resize-none min-h-[120px]"
                value={prompt}
                onChange={(e) => setPrompt(e.target.value)}
              />
            </div>
          </div>

          {/* Greeting */}
          <div className="flex flex-col gap-2">
            <label className="text-[14px] font-medium text-text-main flex items-center gap-1.5 ml-1">
              <MessageSquare className="w-4 h-4 text-text-sub" />
              开场白
            </label>
            <div className="bg-chat-other-bg rounded-xl px-4 py-3 border border-border-color focus-within:border-primary-blue transition-colors">
              <input
                type="text"
                placeholder="你好！我是你的前端开发助手..."
                className="w-full bg-transparent text-[16px] text-text-main focus:outline-none placeholder:text-text-sub"
                value={greeting}
                onChange={(e) => setGreeting(e.target.value)}
              />
            </div>
          </div>

          {/* Knowledge Base */}
          <div className="flex flex-col gap-2">
            <label className="text-[14px] font-medium text-text-main flex items-center gap-1.5 ml-1">
              <FileText className="w-4 h-4 text-primary-blue" />
              专属知识库
            </label>
            <div className="bg-chat-other-bg rounded-xl px-4 py-5 border border-border-color border-dashed flex flex-col items-center justify-center gap-3 cursor-pointer active:bg-active-bg transition-colors">
              <div className="w-12 h-12 rounded-full bg-primary-blue/10 flex items-center justify-center">
                <UploadCloud className="w-6 h-6 text-primary-blue" />
              </div>
              <div className="flex flex-col items-center gap-1">
                <span className="text-[15px] font-medium text-text-main">
                  上传文档或数据
                </span>
                <span className="text-[12px] text-text-sub text-center leading-relaxed">
                  支持 PDF, Word, TXT 等格式
                  <br />
                  让智能体基于你的专属数据进行回答
                </span>
              </div>
            </div>
          </div>

          {/* Voice Cell inside normal flow (before advanced) */}
          <div className="flex flex-col gap-2 mt-2">
            <div
              onClick={() => setShowVoiceSelection(true)}
              className="flex items-center justify-between px-4 py-3.5 bg-chat-other-bg border border-border-color rounded-xl active:bg-active-bg transition-colors cursor-pointer"
            >
              <div className="flex flex-col gap-1">
                <div className="flex items-center gap-2">
                  <Mic className="w-5 h-5 text-primary-blue" />
                  <span className="text-[16px] text-text-main font-medium">
                    配置音色
                  </span>
                </div>
                <span className="text-[12px] text-text-sub">
                  决定智能体发声沟通的声库质感
                </span>
              </div>
              <div className="flex items-center gap-2">
                <span className="text-[15px] text-text-main">
                  {voice.label}
                </span>
                <ChevronRight className="w-5 h-5 opacity-50 text-text-sub" />
              </div>
            </div>
          </div>

          {/* Advanced Settings */}
          <div className="mt-2 flex flex-col">
            <div
              onClick={() => setShowAdvanced(!showAdvanced)}
              className={cn(
                "flex items-center justify-between px-4 py-3.5 bg-chat-other-bg border border-border-color active:bg-active-bg transition-all cursor-pointer",
                showAdvanced
                  ? "rounded-t-xl border-b-transparent"
                  : "rounded-xl",
              )}
            >
              <div className="flex items-center gap-3">
                <Settings2 className="w-5 h-5 text-text-main" />
                <span className="text-[16px] text-text-main">高级设置</span>
              </div>
              <div className="flex items-center gap-2 text-text-sub">
                <span className="text-[14px]">音色、能力等</span>
                <ChevronRight
                  className={cn(
                    "w-5 h-5 opacity-50 transition-transform duration-300",
                    showAdvanced && "rotate-90",
                  )}
                />
              </div>
            </div>

            <AnimatePresence>
              {showAdvanced && (
                <motion.div
                  initial={{ height: 0, opacity: 0 }}
                  animate={{ height: "auto", opacity: 1 }}
                  exit={{ height: 0, opacity: 0 }}
                  transition={{ duration: 0.2, ease: "easeInOut" }}
                  className="overflow-hidden bg-chat-other-bg border border-t-0 border-border-color rounded-b-xl"
                >
                  <div className="flex flex-col gap-6 p-4 pt-2">
                    {/* Tools / Plugins */}
                    <div className="flex flex-col gap-3">
                      <label className="text-[13px] font-medium text-text-sub">
                        扩展能力
                      </label>

                      <div className="flex items-center justify-between">
                        <div className="flex items-center gap-2">
                          <Globe className="w-4 h-4 text-text-main" />
                          <span className="text-[14px] text-text-main">
                            联网搜索
                          </span>
                        </div>
                        <Switch
                          checked={tools.webSearch}
                          onChange={(c) => setTools({ ...tools, webSearch: c })}
                        />
                      </div>

                      <div className="flex items-center justify-between">
                        <div className="flex items-center gap-2">
                          <ImageIcon className="w-4 h-4 text-text-main" />
                          <span className="text-[14px] text-text-main">
                            图像生成
                          </span>
                        </div>
                        <Switch
                          checked={tools.imageGen}
                          onChange={(c) => setTools({ ...tools, imageGen: c })}
                        />
                      </div>
                    </div>

                    {/* Temperature Slider */}
                    <div className="flex flex-col gap-2.5">
                      <div className="flex justify-between items-center">
                        <label className="text-[13px] font-medium text-text-sub">
                          温度 (Temperature)
                        </label>
                        <span className="text-[14px] text-primary-blue font-medium">
                          {temperature.toFixed(1)}
                        </span>
                      </div>
                      <input
                        type="range"
                        min="0"
                        max="1"
                        step="0.1"
                        value={temperature}
                        onChange={(e) =>
                          setTemperature(parseFloat(e.target.value))
                        }
                        className="w-full accent-primary-blue"
                      />
                      <div className="flex justify-between text-[11px] text-text-sub/70">
                        <span>精确/保守</span>
                        <span>发散/创造性</span>
                      </div>
                    </div>

                    {/* Max Tokens */}
                    <div className="flex flex-col gap-2.5">
                      <div className="flex justify-between items-center">
                        <label className="text-[13px] font-medium text-text-sub">
                          最大回复长度 (Max Tokens)
                        </label>
                        <span className="text-[14px] text-primary-blue font-medium">
                          {maxTokens}
                        </span>
                      </div>
                      <input
                        type="range"
                        min="256"
                        max="8192"
                        step="256"
                        value={maxTokens}
                        onChange={(e) => setMaxTokens(parseInt(e.target.value))}
                        className="w-full accent-primary-blue"
                      />
                    </div>
                  </div>
                </motion.div>
              )}
            </AnimatePresence>
          </div>
        </div>
      </div>

      {/* Voice Selection Fullscreen Overlay */}
      {showVoiceSelection && (
        <VoiceSelectionPage
          currentVoiceId={voice.id}
          onSelect={(v) => {
            setVoice(v);
            setShowVoiceSelection(false);
          }}
          onClose={() => setShowVoiceSelection(false)}
        />
      )}
    </div>
  );
};
