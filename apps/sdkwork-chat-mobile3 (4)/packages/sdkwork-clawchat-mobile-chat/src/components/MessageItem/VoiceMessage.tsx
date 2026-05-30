import React, { useState, useRef } from "react";
import type { Message } from "@sdkwork/clawchat-mobile-types";
import { Mic } from "lucide-react";
import { cn, showToast } from "@sdkwork/clawchat-mobile-commons";

export const VoiceMessage = ({
  msg,
  isMe,
}: {
  msg: Message;
  isMe: boolean;
}) => {
  const [isPlaying, setIsPlaying] = useState(false);
  const audioRef = useRef<HTMLAudioElement>(null);

  const togglePlay = async () => {
    if (!audioRef.current) return;
    if (isPlaying) {
      audioRef.current.pause();
      audioRef.current.currentTime = 0;
      setIsPlaying(false);
    } else {
      try {
        await audioRef.current.play();
        setIsPlaying(true);
      } catch (e) {
        console.error("Audio play error", e);
        showToast("无法播放语音");
        setIsPlaying(false);
      }
    }
  };

  return (
    <div
      className="flex items-center gap-2 cursor-pointer"
      onClick={togglePlay}
    >
      <audio
        ref={audioRef}
        src={msg.content}
        onEnded={() => setIsPlaying(false)}
        preload="none"
      />
      <Mic className={cn("w-4 h-4", isMe ? "text-white" : "text-text-sub")} />
      <span className="font-medium text-[15px]">{msg.metadata?.duration}</span>
      <div className="flex gap-1 ml-2">
        <div
          className={cn(
            "w-1 h-3 rounded-full transition-all duration-200",
            isMe ? "bg-white/70" : "bg-text-sub/40",
            isPlaying ? "animate-bounce" : "",
          )}
          style={{ animationDelay: "0ms" }}
        />
        <div
          className={cn(
            "w-1 h-3 rounded-full transition-all duration-200",
            isMe ? "bg-white/70" : "bg-text-sub/40",
            isPlaying ? "animate-bounce" : "",
          )}
          style={{ animationDelay: "150ms" }}
        />
        <div
          className={cn(
            "w-1 h-3 rounded-full transition-all duration-200",
            isMe ? "bg-white/70" : "bg-text-sub/40",
            isPlaying ? "animate-bounce" : "",
          )}
          style={{ animationDelay: "300ms" }}
        />
      </div>
    </div>
  );
};
