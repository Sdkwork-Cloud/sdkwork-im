import React, { useState, useEffect } from "react";
import { useNavigate } from "react-router";
import { ChevronLeft, UserRound, Plus, MoreHorizontal } from "lucide-react";
import { IconButton } from "@sdkwork/clawchat-mobile-commons";
import { CharacterService, type Character } from "../services/CharacterService";

export const MyCharacters: React.FC = () => {
  const navigate = useNavigate();
  const [characters, setCharacters] = useState<Character[]>([]);

  useEffect(() => {
    CharacterService.getCharacters().then(setCharacters);
  }, []);

  const CharacterCard = ({
    src,
    name,
    desc,
  }: {
    src: string;
    name: string;
    desc: string;
  }) => (
    <div className="bg-white dark:bg-[#1A1A1A] px-4 py-3.5 flex items-center gap-4 border-b border-border-color last:border-b-0 active:bg-active-bg transition-colors cursor-pointer">
      <img
        src={src}
        className="w-12 h-12 rounded-full object-cover shrink-0"
        alt="character"
      />
      <div className="flex-1 min-w-0">
        <h3 className="text-[16px] font-medium text-text-main truncate">
          {name}
        </h3>
        <p className="text-[13px] text-text-sub truncate mt-0.5">{desc}</p>
      </div>
      <MoreHorizontal className="w-5 h-5 text-text-sub shrink-0" />
    </div>
  );

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
          <h1 className="text-[17px] font-medium text-text-main">我的角色</h1>
        </div>
        <div className="flex items-center justify-end z-10 flex-1 pr-2">
          <IconButton
            icon={<Plus className="w-5 h-5 text-text-main" />}
            onClick={() => navigate("/me/characters/create")}
          />
        </div>
      </header>

      {/* Content */}
      <div className="flex-1 overflow-y-auto pb-8 mt-2">
        <div className="flex flex-col border-y border-border-color">
          {characters.map((char) => (
            <CharacterCard
              key={char.id}
              src={char.avatar}
              name={char.name}
              desc={char.desc}
            />
          ))}
        </div>
      </div>
    </div>
  );
};
