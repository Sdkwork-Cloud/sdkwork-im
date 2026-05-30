import React, { useState, useRef, useEffect } from "react";
import { useNavigate } from "react-router";
import {
  Search,
  UserPlus,
  Users,
  Tags,
  Building2,
  Plus,
  ChevronLeft,
} from "lucide-react";
import { IconButton, cn } from "@sdkwork/clawchat-mobile-commons";
import { motion, AnimatePresence } from "motion/react";
import { ContactService, type Contact } from "../services/ContactService";

const INDEX_ALPHABET = [
  "↑",
  "☆",
  "A",
  "B",
  "C",
  "D",
  "E",
  "F",
  "G",
  "H",
  "I",
  "J",
  "K",
  "L",
  "M",
  "N",
  "O",
  "P",
  "Q",
  "R",
  "S",
  "T",
  "U",
  "V",
  "W",
  "X",
  "Y",
  "Z",
  "#",
];

export const AddressBook: React.FC = () => {
  const navigate = useNavigate();
  const [activeLetter, setActiveLetter] = useState<string | null>(null);
  const scrollRef = useRef<HTMLDivElement>(null);
  const letterIndicatorTimeout = useRef<any>(null);
  const [contactsData, setContactsData] = useState<Record<string, Contact[]>>(
    {},
  );
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    const loadData = async () => {
      const data = await ContactService.getContactsDict();
      setContactsData(data);
      setLoading(false);
    };
    loadData();
  }, []);

  const handleIndexClick = (letter: string) => {
    setActiveLetter(letter);
    if (letterIndicatorTimeout.current)
      clearTimeout(letterIndicatorTimeout.current);
    letterIndicatorTimeout.current = setTimeout(
      () => setActiveLetter(null),
      800,
    );

    // Scroll logic
    if (letter === "↑") {
      scrollRef.current?.scrollTo({ top: 0, behavior: "smooth" });
      return;
    }

    const section = document.getElementById(`contact-section-${letter}`);
    if (section && scrollRef.current) {
      // considering sticky header height
      const offsetTop = section.offsetTop - 52;
      scrollRef.current.scrollTo({ top: offsetTop, behavior: "smooth" });
    }
  };

  const TopFunctionRow = ({
    icon: Icon,
    title,
    bgColor,
    onClick,
  }: {
    icon: React.ElementType;
    title: string;
    bgColor: string;
    onClick: () => void;
  }) => (
    <div
      className="flex items-center pl-4 pr-3 py-2.5 bg-bg-color active:bg-active-bg transition-colors cursor-pointer"
      onClick={onClick}
    >
      <div
        className={cn(
          "w-10 h-10 rounded-[10px] flex items-center justify-center shrink-0 mr-3.5",
          bgColor,
        )}
      >
        <Icon className="w-5 h-5 text-white" />
      </div>
      <div className="flex-1 border-b border-border-color/50 min-h-[44px] flex items-center">
        <span className="text-[16px] text-text-main">{title}</span>
      </div>
    </div>
  );

  const ContactRow: React.FC<{
    contact: Contact;
    isLast: boolean;
  }> = ({ contact, isLast }) => (
    <div
      className="flex items-center pl-4 pr-3 bg-bg-color active:bg-active-bg transition-colors cursor-pointer"
      onClick={() => navigate(`/chat/${contact.id}/profile`)}
    >
      <div className="w-10 h-10 rounded-md overflow-hidden shrink-0 mr-3.5 my-2">
        <img
          src={contact.avatar}
          alt={contact.name}
          className="w-full h-full object-cover"
        />
      </div>
      <div
        className={cn(
          "flex-1 min-h-[56px] flex items-center",
          !isLast && "border-b border-border-color/50",
        )}
      >
        <span className="text-[16px] text-text-main">{contact.name}</span>
      </div>
    </div>
  );

  return (
    <div className="flex flex-col h-full bg-bg-color relative">
      {/* Header */}
      <header className="h-[52px] flex items-center justify-between px-2 bg-bg-color/90 backdrop-blur-md sticky top-0 z-20 shrink-0 pt-safe">
        <div className="flex items-center z-10 w-[80px]">
          <IconButton
            icon={
              <ChevronLeft className="w-7 h-7 text-text-main" strokeWidth={2} />
            }
            onClick={() => navigate(-1)}
          />
        </div>
        <div className="absolute left-1/2 -translate-x-1/2 font-semibold text-[17px] text-text-main pointer-events-none">
          通讯录
        </div>
        <div className="flex items-center justify-end z-10 w-[80px] gap-1 pr-2">
          <IconButton
            icon={<Search className="w-5 h-5 text-text-main" />}
            onClick={() => navigate("/search")}
          />
          <IconButton
            icon={<Plus className="w-6 h-6 text-text-main" />}
            onClick={() => navigate("/add-friend")}
          />
        </div>
      </header>

      {/* Main Content Area */}
      <div
        className="flex-1 overflow-y-auto no-scrollbar relative"
        ref={scrollRef}
      >
        {/* Search Bar Placeholder (Static) */}
        <div className="px-3 py-2 bg-bg-color">
          <div
            className="h-9 w-full bg-chat-other-bg rounded-lg flex items-center justify-center gap-1.5 cursor-pointer active:opacity-70"
            onClick={() => navigate("/search")}
          >
            <Search className="w-4 h-4 text-text-sub" />
            <span className="text-[15px] text-text-sub">搜索</span>
          </div>
        </div>

        {/* Function Rows */}
        <div className="flex flex-col mb-1">
          <TopFunctionRow
            icon={UserPlus}
            title="新的朋友"
            bgColor="bg-[#FA9D3B]"
            onClick={() => navigate("/add-friend")}
          />
          <TopFunctionRow
            icon={Users}
            title="群聊"
            bgColor="bg-[#07C160]"
            onClick={() => {}}
          />
          <TopFunctionRow
            icon={Tags}
            title="标签"
            bgColor="bg-[#10aeff]"
            onClick={() => {}}
          />
          <TopFunctionRow
            icon={Building2}
            title="公众号"
            bgColor="bg-[#10aeff]"
            onClick={() => {}}
          />
          {/* Bottom border fix for the last function row */}
          <div className="pl-4 bg-bg-color">
            <div className="border-b border-border-color/50 w-full" />
          </div>
        </div>

        {/* Contact List Sections */}
        {Object.keys(contactsData)
          .sort()
          .map((letter) => (
            <div key={letter} id={`contact-section-${letter}`}>
              <div className="h-7 bg-[#EDEDED] dark:bg-[#1A1A1A] flex items-center pl-4 sticky top-0 z-10">
                <span className="text-[13px] font-semibold text-text-sub">
                  {letter}
                </span>
              </div>
              <div className="flex flex-col">
                {contactsData[letter].map((contact, index) => (
                  <ContactRow
                    key={contact.id}
                    contact={contact}
                    isLast={index === contactsData[letter].length - 1}
                  />
                ))}
              </div>
            </div>
          ))}

        {/* Footer padding */}
        <div className="h-[40px] flex items-center justify-center pb-safe mb-4">
          <span className="text-[14px] text-text-sub">
            {Object.values(contactsData).flat().length} 位联系人
          </span>
        </div>
      </div>

      {/* Right Alphabet Index */}
      <div className="absolute right-0 top-1/2 -translate-y-1/2 flex flex-col items-center justify-center w-6 z-30 pt-safe font-sans pb-10">
        {INDEX_ALPHABET.map((letter) => (
          <div
            key={letter}
            className="text-[10px] h-[16px] flex items-center justify-center text-text-sub/80 cursor-pointer w-full hover:bg-black/10 dark:hover:bg-white/10"
            onClick={() => handleIndexClick(letter)}
          >
            {letter}
          </div>
        ))}
      </div>

      {/* Center Letter Indicator (Pop-up) */}
      <AnimatePresence>
        {activeLetter && (
          <motion.div
            initial={{ opacity: 0, scale: 0.8 }}
            animate={{ opacity: 1, scale: 1 }}
            exit={{ opacity: 0, scale: 0.8 }}
            className="absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 w-16 h-16 bg-black/60 backdrop-blur-md rounded-xl flex items-center justify-center z-50 shadow-2xl pointer-events-none"
          >
            <span className="text-white text-3xl font-bold">
              {activeLetter}
            </span>
          </motion.div>
        )}
      </AnimatePresence>
    </div>
  );
};
