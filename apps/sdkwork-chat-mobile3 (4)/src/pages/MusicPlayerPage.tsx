import React, { useState } from 'react';
import { useNavigate } from 'react-router';
import { motion } from 'motion/react';
import { ChevronDown, Play, Pause, SkipBack, SkipForward, Repeat, Shuffle, ListMusic, Heart } from 'lucide-react';
import { useAudioStore } from '@sdkwork/clawchat-mobile-core';
import { IconButton } from '@sdkwork/clawchat-mobile-commons';

export const MusicPlayerPage: React.FC = () => {
  const navigate = useNavigate();
  const currentTrack = useAudioStore(s => s.currentTrack);
  const isPlaying = useAudioStore(s => s.isPlaying);
  const progress = useAudioStore(s => s.progress);
  const duration = useAudioStore(s => s.duration);
  const pause = useAudioStore(s => s.pause);
  const resume = useAudioStore(s => s.resume);
  const seek = useAudioStore(s => s.seek);
  
  const [isLiked, setIsLiked] = useState(false);

  if (!currentTrack) {
    return (
      <div className="flex flex-col h-full bg-[#121212] items-center justify-center text-white">
        <p>暂无播放内容</p>
        <button onClick={() => navigate(-1)} className="mt-4 px-4 py-2 bg-white/10 rounded-full">返回</button>
      </div>
    );
  }

  const formatTime = (seconds: number) => {
    const m = Math.floor(seconds / 60).toString().padStart(2, '0');
    const s = Math.floor(seconds % 60).toString().padStart(2, '0');
    return `${m}:${s}`;
  };

  const handleSeek = (e: React.ChangeEvent<HTMLInputElement>) => {
    seek(Number(e.target.value));
  };

  return (
    <div className="flex flex-col h-full bg-[#121212] text-white relative overflow-hidden">
      {/* Background Blur */}
      <div 
        className="absolute inset-0 z-0 opacity-40 scale-110 blur-3xl"
        style={{
          backgroundImage: `url(${currentTrack.coverUrl})`,
          backgroundSize: 'cover',
          backgroundPosition: 'center',
        }}
      />
      <div className="absolute inset-0 bg-gradient-to-b from-black/20 via-[#121212]/80 to-[#121212] z-0" />

      {/* Header */}
      <header className="h-[56px] flex items-center justify-between px-4 pt-safe shrink-0 relative z-10">
        <IconButton icon={<ChevronDown className="w-8 h-8 text-white" />} onClick={() => navigate(-1)} />
        <div className="flex flex-col items-center">
          <span className="text-[12px] opacity-70">正在播放</span>
          <span className="text-[14px] font-medium">{currentTrack.title}</span>
        </div>
        <IconButton icon={<MoreVertical className="w-6 h-6 text-white" />} onClick={() => {}} />
      </header>

      {/* Content */}
      <div className="flex-1 flex flex-col items-center justify-between px-8 py-8 relative z-10 overflow-y-auto">
        {/* Cover */}
        <motion.div 
          className="w-full aspect-square max-w-[320px] rounded-3xl overflow-hidden shadow-2xl mx-auto"
          animate={{ scale: isPlaying ? 1 : 0.95 }}
          transition={{ type: "spring", bounce: 0.4 }}
        >
          <img src={currentTrack.coverUrl} alt="Cover" className="w-full h-full object-cover" />
        </motion.div>

        {/* Info & Controls Config */}
        <div className="w-full flex flex-col gap-6 mt-8">
          {/* Song Info */}
          <div className="flex items-center justify-between">
            <div className="flex flex-col min-w-0 pr-4">
              <h1 className="text-[24px] font-bold truncate">{currentTrack.title}</h1>
              <p className="text-[16px] text-white/70 truncate">{currentTrack.artist}</p>
            </div>
            <IconButton 
              icon={<Heart className={`w-7 h-7 ${isLiked ? 'fill-[#1ED760] text-[#1ED760]' : 'text-white'}`} />} 
              onClick={() => setIsLiked(!isLiked)} 
            />
          </div>

          {/* Progress */}
          <div className="flex flex-col gap-2">
            <input 
              type="range" 
              min="0" 
              max={duration || 100} 
              value={progress} 
              onChange={handleSeek}
              className="w-full h-1.5 bg-white/20 rounded-full appearance-none accent-white cursor-pointer"
            />
            <div className="flex justify-between text-[12px] text-white/50 font-medium">
              <span>{formatTime(progress)}</span>
              <span>{formatTime(duration)}</span>
            </div>
          </div>

          {/* Controls */}
          <div className="flex items-center justify-between px-2">
            <IconButton icon={<Shuffle className="w-6 h-6 text-white/70" />} onClick={() => {}} />
            <IconButton icon={<SkipBack className="w-8 h-8 text-white fill-white" />} onClick={() => {}} />
            <div 
              className="w-16 h-16 rounded-full bg-white text-black flex items-center justify-center cursor-pointer active:scale-95 transition-transform"
              onClick={isPlaying ? pause : resume}
            >
              {isPlaying ? <Pause className="w-8 h-8 fill-black" /> : <Play className="w-8 h-8 fill-black ml-1" />}
            </div>
            <IconButton icon={<SkipForward className="w-8 h-8 text-white fill-white" />} onClick={() => {}} />
            <IconButton icon={<Repeat className="w-6 h-6 text-white/70" />} onClick={() => {}} />
          </div>

          {/* Bottom Actions */}
          <div className="flex items-center justify-between px-2 pt-4 opacity-80">
            <IconButton icon={<DevicesIcon className="w-5 h-5 text-white" />} onClick={() => {}} />
            <IconButton icon={<ListMusic className="w-6 h-6 text-white" />} onClick={() => {}} />
          </div>
        </div>
      </div>
    </div>
  );
};

const MoreVertical = ({ className }: { className?: string }) => (
  <svg viewBox="0 0 24 24" className={className} fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
    <circle cx="12" cy="12" r="1"></circle>
    <circle cx="12" cy="5" r="1"></circle>
    <circle cx="12" cy="19" r="1"></circle>
  </svg>
);

const DevicesIcon = ({ className }: { className?: string }) => (
  <svg viewBox="0 0 24 24" className={className} fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
    <path d="M4 6h16M4 12h16M4 18h16" />
  </svg>
);
