import React, { useState, useEffect } from "react";
import { useNavigate, useParams } from "react-router";
import {
  Mic,
  MicOff,
  PhoneOff,
  Video,
  VideoOff,
  SwitchCamera,
} from "lucide-react";
import { motion } from "motion/react";
import { cn, showToast } from "@sdkwork/clawchat-mobile-commons";
import { ChatService } from "../services/ChatService";

export const VideoCall: React.FC = () => {
  const { id } = useParams();
  const navigate = useNavigate();
  const [isMuted, setIsMuted] = useState(false);
  const [isVideoOff, setIsVideoOff] = useState(false);
  const [isFrontCamera, setIsFrontCamera] = useState(true);
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

  // Simulate connection after 2 seconds
  useEffect(() => {
    if (callState === "calling") {
      const timer = setTimeout(() => {
        setCallState("connected");
      }, 2000);
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
    isActive,
    onClick,
    isDanger,
  }: {
    icon: React.ElementType;
    isActive?: boolean;
    onClick?: () => void;
    isDanger?: boolean;
  }) => (
    <div
      onClick={onClick}
      className={cn(
        "w-14 h-14 rounded-full flex items-center justify-center cursor-pointer transition-colors shadow-lg backdrop-blur-md",
        isDanger
          ? "bg-red-500 text-white active:bg-red-600"
          : isActive
            ? "bg-white text-black active:bg-gray-200"
            : "bg-black/40 text-white border border-white/10 active:bg-black/60",
      )}
    >
      <Icon className="w-6 h-6" />
    </div>
  );

  const displayName = chat ? chat.name : "未知联系人";
  const displayAvatar = chat ? chat.avatar : "https://picsum.photos/seed/sarah-video/800/1600";

  return (
    <div className="flex flex-col h-full bg-black relative overflow-hidden">
      {/* Remote Video (Background) */}
      <div className="absolute inset-0 z-0">
        <img
          src={displayAvatar}
          alt="Remote Video"
          className="w-full h-full object-cover"
        />
        {/* Gradient overlay for text readability */}
        <div className="absolute inset-0 bg-gradient-to-b from-black/60 via-transparent to-black/80" />
      </div>

      {/* Local Video (PIP) */}
      {callState === "connected" && !isVideoOff && (
        <motion.div
          drag
          dragConstraints={{ top: 60, left: 20, right: 20, bottom: 200 }}
          dragElastic={0.1}
          initial={{ opacity: 0, scale: 0.8 }}
          animate={{ opacity: 1, scale: 1 }}
          className="absolute top-20 right-4 w-28 h-40 bg-gray-800 rounded-xl overflow-hidden shadow-2xl border border-white/20 z-20 cursor-grab active:cursor-grabbing"
        >
          <img
            src="https://picsum.photos/seed/me-video/300/400"
            alt="Local Video"
            className={cn(
              "w-full h-full object-cover transition-transform duration-300",
              isFrontCamera ? "scale-x-[-1]" : "scale-x-100",
            )}
          />
        </motion.div>
      )}

      {/* Header */}
      <div className="relative z-10 pt-safe px-6 flex flex-col items-center mt-4">
        <h2 className="text-xl font-bold text-white drop-shadow-md">
          {displayName}
        </h2>

        <p className="text-sm text-white/80 drop-shadow-md mt-1">
          {callState === "calling"
            ? "正在等待对方接受邀请..."
            : formatDuration(duration)}
        </p>
      </div>

      {/* Controls */}
      <div className="absolute bottom-0 left-0 right-0 z-10 pb-[calc(40px+env(safe-area-inset-bottom))] px-8">
        <div className="flex items-center justify-between max-w-[320px] mx-auto">
          <ControlButton
            icon={SwitchCamera}
            isActive={!isFrontCamera}
            onClick={() => setIsFrontCamera(!isFrontCamera)}
          />
          <ControlButton
            icon={isVideoOff ? VideoOff : Video}
            isActive={isVideoOff}
            onClick={() => setIsVideoOff(!isVideoOff)}
          />
          <ControlButton
            icon={isMuted ? MicOff : Mic}
            isActive={isMuted}
            onClick={() => setIsMuted(!isMuted)}
          />
          <ControlButton
            icon={PhoneOff}
            isDanger={true}
            onClick={handleHangUp}
          />
        </div>
      </div>
    </div>
  );
};
