import React, { useState, useEffect } from 'react';
import { motion, AnimatePresence } from 'motion/react';
import { Phone, Video, Mic, MicOff, VideoOff, MonitorUp, PhoneOff, Maximize, Minimize, Smartphone, Monitor } from 'lucide-react';
import { Avatar } from '@sdkwork/clawchat-pc-commons';
import { toast } from './Toast';
import { callService, type SdkworkCallSnapshot } from '../services/CallService';

export type CallType = 'voice' | 'video';

interface CallOverlayProps {
  conversationId: string;
  isOpen: boolean;
  mode?: 'incoming' | 'outgoing';
  type: CallType;
  callerName: string;
  callerAvatar: string;
  onClose: () => void;
}

export const CallOverlay: React.FC<CallOverlayProps> = ({
  conversationId,
  isOpen,
  mode = 'outgoing',
  type,
  callerName,
  callerAvatar,
  onClose,
}) => {
  const [callState, setCallState] = useState<'ringing' | 'connected'>('ringing');
  const [callSnapshot, setCallSnapshot] = useState<SdkworkCallSnapshot>(callService.getSnapshot());
  const [isMuted, setIsMuted] = useState(false);
  const [isVideoOff, setIsVideoOff] = useState(type === 'voice');
  const [viewMode, setViewMode] = useState<'mobile' | 'desktop' | 'fullscreen'>('mobile');
  const [callDuration, setCallDuration] = useState(0);

  useEffect(() => {
    return callService.subscribe((snapshot) => {
      setCallSnapshot(snapshot);
      setIsMuted(snapshot.isAudioMuted);
      setIsVideoOff(snapshot.isVideoMuted);
      setCallState(snapshot.state === 'connected' ? 'connected' : 'ringing');
      if (snapshot.state === 'errored' && snapshot.errorMessage) {
        toast(snapshot.errorMessage, 'error');
      }
      if (snapshot.state === 'ended' || snapshot.state === 'rejected' || snapshot.state === 'errored') {
        setCallDuration(0);
      }
    });
  }, []);

  // Reset state when opened and start the SDK-backed outgoing call when needed.
  useEffect(() => {
    if (!isOpen) {
      return;
    }

    setCallState('ringing');
    setIsMuted(false);
    setIsVideoOff(type === 'voice');
    setCallDuration(0);
    setViewMode('mobile'); // Default to mobile mode

    if (mode === 'outgoing') {
      void callService.startOutgoingCall({
        conversationId,
        targetName: callerName,
        type,
      });
    }
  }, [callerName, conversationId, isOpen, mode, type]);

  // Timer for connected state
  useEffect(() => {
    let interval: NodeJS.Timeout;
    if (callState === 'connected') {
      interval = setInterval(() => {
        setCallDuration(prev => prev + 1);
      }, 1000);
    }
    return () => clearInterval(interval);
  }, [callState]);

  const formatTime = (seconds: number) => {
    const mins = Math.floor(seconds / 60);
    const secs = seconds % 60;
    return `${mins.toString().padStart(2, '0')}:${secs.toString().padStart(2, '0')}`;
  };

  const getContainerClasses = () => {
    switch (viewMode) {
      case 'fullscreen':
        return 'inset-0 rounded-none';
      case 'desktop':
        return 'top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 w-[800px] h-[500px] rounded-2xl border border-white/10';
      case 'mobile':
      default:
        return 'top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 w-[360px] h-[680px] rounded-[3rem] border-[8px] border-[#222] shadow-2xl';
    }
  };

  const isMobile = viewMode === 'mobile';
  const controlBtnClass = isMobile ? 'w-12 h-12' : 'w-14 h-14';
  const hangupBtnClass = isMobile ? 'w-14 h-14 ml-1' : 'w-16 h-16 ml-4';
  const iconSize = isMobile ? 20 : 24;
  const hangupIconSize = isMobile ? 24 : 28;
  const isIncomingRinging = mode === 'incoming' && callSnapshot.state === 'ringing';
  const statusText = callSnapshot.state === 'connecting'
    ? '正在连接...'
    : callSnapshot.state === 'errored'
      ? '通话连接失败'
      : '正在呼叫...';

  return (
    <AnimatePresence>
      {isOpen && (
        <motion.div
          initial={{ opacity: 0, scale: 0.95 }}
          animate={{ opacity: 1, scale: 1 }}
          exit={{ opacity: 0, scale: 0.95 }}
          transition={{ duration: 0.2, ease: "easeOut" }}
          className={`fixed z-50 flex flex-col overflow-hidden shadow-2xl bg-[#181818] ${getContainerClasses()}`}
        >
        {/* Header */}
        <div className="absolute top-0 left-0 right-0 p-4 flex justify-between items-center z-30 bg-gradient-to-b from-black/60 to-transparent">
          <div className="text-white font-medium text-lg drop-shadow-md">
            {type === 'video' ? '视频通话' : '语音通话'}
          </div>
          <div className="flex items-center gap-2">
            {viewMode !== 'fullscreen' && (
              <button 
                onClick={() => setViewMode(isMobile ? 'desktop' : 'mobile')}
                className="w-8 h-8 flex items-center justify-center rounded-full bg-black/20 hover:bg-black/40 text-white transition-colors backdrop-blur-sm"
                title={isMobile ? "切换至桌面模式" : "切换至手机模式"}
              >
                {isMobile ? <Monitor size={16} /> : <Smartphone size={16} />}
              </button>
            )}
            <button 
              onClick={() => setViewMode(viewMode === 'fullscreen' ? 'desktop' : 'fullscreen')}
              className="w-8 h-8 flex items-center justify-center rounded-full bg-black/20 hover:bg-black/40 text-white transition-colors backdrop-blur-sm"
              title={viewMode === 'fullscreen' ? "退出全屏" : "全屏"}
            >
              {viewMode === 'fullscreen' ? <Minimize size={16} /> : <Maximize size={16} />}
            </button>
          </div>
        </div>

        {/* Main Content Area */}
        <div className="flex-1 relative flex items-center justify-center bg-[#111]">
          {/* Background Blur for Voice Call or Video Off */}
          {(type === 'voice' || isVideoOff) && (
            <div 
              className="absolute inset-0 bg-cover bg-center opacity-20 blur-3xl"
              style={{ backgroundImage: `url(${callerAvatar})` }}
            />
          )}

          {/* Caller Info (Ringing or Voice Call) */}
          {(callState === 'ringing' || type === 'voice' || isVideoOff) ? (
            <div className="relative z-10 flex flex-col items-center">
              <motion.div
                animate={callState === 'ringing' ? { scale: [1, 1.1, 1] } : {}}
                transition={{ repeat: Infinity, duration: 2 }}
                className="relative"
              >
                <Avatar 
                  src={callerAvatar} 
                  alt={callerName} 
                  className={`${isMobile ? 'w-24 h-24' : 'w-32 h-32'} rounded-full border-4 border-white/10 shadow-2xl transition-all`}
                />
                {callState === 'ringing' && (
                  <div className="absolute inset-0 rounded-full border-2 border-blue-500 animate-ping opacity-75" />
                )}
              </motion.div>
              <h2 className={`mt-6 font-medium text-white drop-shadow-lg ${isMobile ? 'text-2xl' : 'text-3xl'}`}>{callerName}</h2>
              <p className={`mt-2 text-gray-400 ${isMobile ? 'text-base' : 'text-lg'}`}>
                {callState === 'ringing' ? statusText : formatTime(callDuration)}
              </p>
            </div>
          ) : (
            /* Simulated Video Feed */
            <div className="absolute inset-0 bg-[#222] flex items-center justify-center">
              {/* Main Video (Remote) */}
              <div className="absolute inset-0 flex items-center justify-center">
                <Avatar src={callerAvatar} alt={callerName} className="w-full h-full object-cover opacity-50 blur-sm" />
                <div className="absolute inset-0 flex items-center justify-center">
                  <span className="text-white/30 text-2xl">对方视频画面</span>
                </div>
              </div>
              
              {/* Picture in Picture (Local) */}
              <div className={`absolute bg-black overflow-hidden shadow-2xl z-20 transition-all duration-300 ${
                isMobile 
                  ? 'top-20 right-4 w-24 h-36 rounded-lg border border-white/20' 
                  : 'bottom-24 right-6 w-48 h-72 rounded-xl border-2 border-white/20'
              }`}>
                <div className="absolute inset-0 flex items-center justify-center text-white/30">
                  我
                </div>
              </div>
            </div>
          )}
        </div>

        {/* Controls Bar */}
        <div className={`absolute bottom-0 left-0 right-0 flex justify-center items-center bg-gradient-to-t from-black/80 to-transparent z-30 ${isMobile ? 'p-6 gap-3' : 'p-6 gap-6'}`}>
          {isIncomingRinging && (
            <button
              onClick={() => {
                void callService.acceptIncomingCall().catch((error) => {
                  toast(error instanceof Error ? error.message : 'RTC accept failed', 'error');
                });
              }}
              className={`${hangupBtnClass} rounded-full bg-emerald-500 hover:bg-emerald-600 text-white flex items-center justify-center shadow-lg shadow-emerald-500/20 transition-all hover:scale-105`}
              title="Accept"
            >
              <Phone size={hangupIconSize} />
            </button>
          )}
          {!isIncomingRinging && (
            <>
          <button 
            onClick={() => {
              const nextMuted = !isMuted;
              setIsMuted(nextMuted);
              void callService.setAudioMuted(nextMuted).catch((error) => {
                toast(error instanceof Error ? error.message : '静音设置失败', 'error');
              });
            }}
            className={`${controlBtnClass} rounded-full flex items-center justify-center transition-all ${
              isMuted ? 'bg-white text-black' : 'bg-white/10 hover:bg-white/20 text-white backdrop-blur-md'
            }`}
            title={isMuted ? "取消静音" : "静音"}
          >
            {isMuted ? <MicOff size={iconSize} /> : <Mic size={iconSize} />}
          </button>

          <button 
            onClick={() => {
              const nextVideoOff = !isVideoOff;
              setIsVideoOff(nextVideoOff);
              void callService.setVideoMuted(nextVideoOff).catch((error) => {
                toast(error instanceof Error ? error.message : '视频设置失败', 'error');
              });
            }}
            className={`${controlBtnClass} rounded-full flex items-center justify-center transition-all ${
              isVideoOff ? 'bg-white text-black' : 'bg-white/10 hover:bg-white/20 text-white backdrop-blur-md'
            }`}
            title={isVideoOff ? "开启视频" : "关闭视频"}
          >
            {isVideoOff ? <VideoOff size={iconSize} /> : <Video size={iconSize} />}
          </button>

          <button 
            className={`${controlBtnClass} rounded-full bg-white/10 hover:bg-white/20 text-white backdrop-blur-md flex items-center justify-center transition-all`}
            title="共享屏幕"
            onClick={async () => {
              if (navigator.mediaDevices && navigator.mediaDevices.getDisplayMedia) {
                 try {
                    const stream = await navigator.mediaDevices.getDisplayMedia({ video: true });
                    toast('屏幕共享已启动', 'success');
                    // We just close it when user stops
                    stream.getVideoTracks()[0].onended = () => {
                       toast('屏幕共享已结束', 'success');
                    };
                 } catch (e: any) {
                    if (e?.message?.includes('display-capture')) {
                       toast('无权限进行共享（或在新标签页中打开应用重试）', 'error');
                    } else {
                       toast('取消屏幕共享', 'success');
                    }
                 }
              } else {
                 toast('当前浏览器不支持屏幕共享', 'error');
              }
            }}
          >
            <MonitorUp size={iconSize} />
          </button>
            </>
          )}

          <button 
            onClick={() => {
              const task = isIncomingRinging
                ? callService.rejectIncomingCall({ reason: 'user_reject' })
                : callService.endCall({ reason: 'user_hangup' });
              void task.finally(onClose);
            }}
            className={`${hangupBtnClass} rounded-full bg-red-500 hover:bg-red-600 text-white flex items-center justify-center shadow-lg shadow-red-500/20 transition-all hover:scale-105`}
            title="挂断"
          >
            <PhoneOff size={hangupIconSize} />
          </button>
        </div>
        </motion.div>
      )}
    </AnimatePresence>
  );
};
