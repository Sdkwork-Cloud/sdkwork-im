import React, { useState, useEffect } from "react";
import { useNavigate, useParams } from "react-router";
import { Mic, MicOff, Volume2, VolumeX, PhoneOff, Video } from "lucide-react";
import { motion } from "motion/react";
import { cn } from "@sdkwork/clawchat-mobile-commons";
import { ChatService } from "../services/ChatService";

export const VoiceCall: React.FC = () => {
  const { id } = useParams();
  const navigate = useNavigate();
  const [isMuted, setIsMuted] = useState(false);
  const [isSpeaker, setIsSpeaker] = useState(false);
  const [callState, setCallState] = useState<"calling" | "connected">(
    "calling",
  );
  const [duration, setDuration] = useState(0);
  const [chat, setChat] = useState<any>(null);

  // Load real chat / user info
  useEffect(() => {
    if (id) {
      ChatService.getChatById(id).then((c) => {
        if (c) {
          setChat(c);
        }
      });
    }
  }, [id]);

  // Simulate connection after 3 seconds
  useEffect(() => {
    if (callState === "calling") {
      const timer = setTimeout(() => {
        setCallState("connected");
      }, 3000);
      return () => clearTimeout(timer);
    }
  }, [callState]);

  // Timer for connected state
  useEffect(() => {
    if (callState === "connected") {
      const interval = setInterval(() => {
        setDuration((prev) => prev + 1);
      }, 1000);
      return () => clearInterval(interval);
    }
  }, [callState]);

  const formatDuration = (seconds: number) => {
    const m = Math.floor(seconds / 60)
      .toString()
      .padStart(2, "0");
    const s = (seconds % 60).toString().padStart(2, "0");
    return `${m}:${s}`;
  };

  const handleHangUp = () => {
    navigate(-1);
  };

  const ControlButton = ({
    icon: Icon,
    label,
    isActive,
    onClick,
    isDanger,
  }: any) => (
    <div className="flex flex-col items-center gap-2">
      <div
        onClick={onClick}
        className={cn(
          "w-16 h-16 rounded-full flex items-center justify-center cursor-pointer transition-colors",
          isDanger
            ? "bg-red-500 text-white active:bg-red-600"
            : isActive
              ? "bg-white text-black active:bg-gray-200"
              : "bg-white/20 text-white active:bg-white/30 backdrop-blur-md",
        )}
      >
        <Icon className="w-7 h-7" />
      </div>
      <span className="text-[13px] text-white/80">{label}</span>
    </div>
  );

  const displayName = chat ? chat.name : "未知联系人";
  const displayAvatar = chat ? chat.avatar : "https://picsum.photos/seed/sarah/200/200";

  return (
    <div className="flex flex-col h-full bg-gray-900 relative overflow-hidden">
      {/* Blurred Background */}
      <div className="absolute inset-0 z-0">
        <img
          src={displayAvatar}
          alt="Background"
          className="w-full h-full object-cover opacity-40 blur-3xl scale-110"
        />
        <div className="absolute inset-0 bg-black/40" />
      </div>

      {/* Header */}
      <div className="relative z-10 pt-safe px-4 flex justify-between items-center h-14">
        <div className="w-8" /> {/* Spacer */}
        <div className="flex items-center gap-1.5 text-white/70 text-[12px] font-medium bg-black/20 px-3 py-1 rounded-full backdrop-blur-md">
          <svg
            className="w-3.5 h-3.5"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            strokeWidth="2"
            strokeLinecap="round"
            strokeLinejoin="round"
          >
            <rect x="3" y="11" width="18" height="11" rx="2" ry="2"></rect>
            <path d="M7 11V7a5 5 0 0 1 10 0v4"></path>
          </svg>
          端到端加密
        </div>
        <div className="w-8" /> {/* Spacer */}
      </div>

      {/* Main Content */}
      <div className="relative z-10 flex-1 flex flex-col items-center justify-center pb-20">
        <div className="relative mb-8">
          {/* Pulsing rings when calling */}
          {callState === "calling" && (
            <>
              <motion.div
                animate={{ scale: [1, 1.5, 2], opacity: [0.5, 0.2, 0] }}
                transition={{ duration: 2, repeat: Infinity, ease: "easeOut" }}
                className="absolute inset-0 bg-white/20 rounded-full"
              />
              <motion.div
                animate={{ scale: [1, 1.5, 2], opacity: [0.5, 0.2, 0] }}
                transition={{
                  duration: 2,
                  repeat: Infinity,
                  ease: "easeOut",
                  delay: 1,
                }}
                className="absolute inset-0 bg-white/20 rounded-full"
              />
            </>
          )}
          <img
            src={displayAvatar}
            alt={displayName}
            className="w-32 h-32 rounded-full object-cover border-4 border-white/10 shadow-2xl relative z-10"
          />
        </div>

        <h2 className="text-3xl font-bold text-white mb-2">{displayName}</h2>
        <p className="text-lg text-white/80">
          {callState === "calling" ? "正在呼叫..." : formatDuration(duration)}
        </p>
      </div>


      {/* Controls */}
      <div className="relative z-10 pb-[calc(40px+env(safe-area-inset-bottom))] px-8">
        <div className="grid grid-cols-3 gap-y-8 gap-x-4 place-items-center mb-10">
          <ControlButton
            icon={isMuted ? MicOff : Mic}
            label={isMuted ? "已静音" : "静音"}
            isActive={isMuted}
            onClick={() => setIsMuted(!isMuted)}
          />
          <ControlButton
            icon={Video}
            label="视频通话"
            isActive={false}
            onClick={() => navigate(`/call/video/${id}`, { replace: true })}
          />
          <ControlButton
            icon={isSpeaker ? Volume2 : VolumeX}
            label={isSpeaker ? "扬声器" : "听筒"}
            isActive={isSpeaker}
            onClick={() => setIsSpeaker(!isSpeaker)}
          />
        </div>

        <div className="flex justify-center">
          <ControlButton
            icon={PhoneOff}
            label=""
            isDanger={true}
            onClick={handleHangUp}
          />
        </div>
      </div>
    </div>
  );
};
