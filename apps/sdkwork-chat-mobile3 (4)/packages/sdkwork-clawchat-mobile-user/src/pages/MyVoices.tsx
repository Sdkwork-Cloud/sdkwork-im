import React, { useState, useEffect } from "react";
import { useNavigate } from "react-router";
import { ChevronLeft, Mic, Play, Plus, Square } from "lucide-react";
import { IconButton } from "@sdkwork/clawchat-mobile-commons";
import { motion } from "motion/react";
import { VoiceService, type VoiceInfo } from "../services/VoiceService";

export const MyVoices: React.FC = () => {
  const navigate = useNavigate();
  const [playingId, setPlayingId] = useState<string | null>(null);
  const [voices, setVoices] = useState<VoiceInfo[]>([]);

  useEffect(() => {
    VoiceService.getVoiceCategories().then((cats) => {
      const myCat = cats.find((c) => c.id === "my");
      if (myCat) setVoices(myCat.voices);
    });
  }, []);

  const handlePlay = (id: string) => {
    if (playingId === id) {
      setPlayingId(null);
    } else {
      setPlayingId(id);
      setTimeout(() => {
        setPlayingId(null);
      }, 3000); // simulate 3s audio
    }
  };

  const VoiceCard = ({ id, name, type }: any) => {
    const isPlaying = playingId === id;
    return (
      <div
        className="bg-white dark:bg-[#1A1A1A] px-4 py-3.5 flex items-center justify-between border-b border-border-color last:border-b-0 active:bg-active-bg transition-colors cursor-pointer"
        onClick={() => handlePlay(id)}
      >
        <div className="flex items-center gap-3">
          <div className="w-10 h-10 rounded-full bg-primary-blue/10 flex items-center justify-center relative">
            <Mic className="w-5 h-5 text-primary-blue" />
            {isPlaying && (
              <motion.div
                className="absolute inset-0 rounded-full border-2 border-primary-blue"
                animate={{ scale: [1, 1.2, 1], opacity: [0.5, 0, 0.5] }}
                transition={{ repeat: Infinity, duration: 1 }}
              />
            )}
          </div>
          <div className="flex flex-col">
            <span className="text-[16px] font-medium text-text-main">
              {name}
            </span>
            <span className="text-[12px] text-text-sub">{type}</span>
          </div>
        </div>
        <div className="w-8 h-8 rounded-full bg-black/5 dark:bg-white/5 flex items-center justify-center active:scale-95 transition-transform">
          {isPlaying ? (
            <Square className="w-3.5 h-3.5 text-text-main fill-current" />
          ) : (
            <Play className="w-4 h-4 text-text-main ml-0.5" />
          )}
        </div>
      </div>
    );
  };

  return (
    <div className="flex flex-col h-full bg-[#f2f2f2] dark:bg-[#121212]">
      {/* Header */}
      <header className="h-[56px] flex items-center justify-between px-1 glass-header sticky top-0 z-10 shrink-0 pt-safe">
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
          <h1 className="text-[17px] font-medium text-text-main">我的声音</h1>
        </div>
        <div className="flex items-center justify-end z-10 flex-1 pr-2">
          <IconButton
            icon={<Plus className="w-5 h-5 text-text-main" />}
            onClick={() => navigate("/me/voices/create")}
          />
        </div>
      </header>

      {/* Content */}
      <div className="flex-1 overflow-y-auto pb-8 mt-2">
        <div className="flex flex-col border-y border-border-color">
          {voices.map((voice) => (
            <VoiceCard
              key={voice.id}
              id={voice.id}
              name={voice.label}
              type={voice.desc}
            />
          ))}
        </div>
      </div>
    </div>
  );
};
