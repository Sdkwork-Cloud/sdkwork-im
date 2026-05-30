import React from "react";
import type { Message } from "@sdkwork/clawchat-mobile-types";
import { Play } from "lucide-react";
import { cn } from "@sdkwork/clawchat-mobile-commons";
import { useAudioStore } from "@sdkwork/clawchat-mobile-core";
import { useNavigate } from "react-router";

export const MusicMessage = ({
  msg,
  isMe,
}: {
  msg: Message;
  isMe: boolean;
}) => {
  const currentTrack = useAudioStore((s) => s.currentTrack);
  const isGlobalPlaying = useAudioStore((s) => s.isPlaying);
  const playMusic = useAudioStore((s) => s.playMusic);
  const pause = useAudioStore((s) => s.pause);
  const navigate = useNavigate();

  const isThisPlaying = currentTrack?.id === msg.id && isGlobalPlaying;

  const handlePlayClick = (e: React.MouseEvent) => {
    e.stopPropagation();
    if (isThisPlaying) {
      pause();
    } else {
      playMusic({
        id: msg.id,
        title: msg.metadata?.title || "未知歌曲",
        artist: msg.metadata?.artist || "未知艺术家",
        coverUrl:
          msg.metadata?.coverUrl || "https://picsum.photos/seed/music/300/300",
        audioUrl:
          msg.content ||
          "https://www.soundhelix.com/examples/mp3/SoundHelix-Song-1.mp3",
      });
      navigate("/music-player");
    }
  };

  return (
    <div
      className="flex items-center gap-3 min-w-[200px] cursor-pointer"
      onClick={handlePlayClick}
    >
      <div className="w-12 h-12 rounded-lg shrink-0 overflow-hidden relative border border-black/10 dark:border-white/10">
        <img
          src={
            msg.metadata?.coverUrl || "https://picsum.photos/seed/music/200/200"
          }
          className="w-full h-full object-cover"
        />
        <div className="absolute inset-0 bg-black/30 flex items-center justify-center">
          {isThisPlaying ? (
            <div className="w-4 h-4 bg-white/90 rounded-sm animate-pulse" />
          ) : (
            <Play className="w-6 h-6 text-white ml-0.5" />
          )}
        </div>
      </div>
      <div className="flex-1 flex flex-col min-w-0">
        <span
          className="text-[15px] font-bold truncate leading-tight mb-0.5"
          style={{ color: isMe ? "white" : "inherit" }}
        >
          {msg.metadata?.title || "未知歌曲"}
        </span>
        <span
          className={cn(
            "text-[12px]",
            isMe ? "text-white/70" : "text-text-sub",
          )}
        >
          {msg.metadata?.artist || "未知艺术家"}
        </span>
      </div>
    </div>
  );
};
