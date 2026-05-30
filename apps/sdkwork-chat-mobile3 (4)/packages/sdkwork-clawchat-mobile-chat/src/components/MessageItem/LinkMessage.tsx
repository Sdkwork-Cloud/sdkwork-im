import React from "react";
import type { Message } from "@sdkwork/clawchat-mobile-types";
import { Link as LinkIcon } from "lucide-react";
import { cn } from "@sdkwork/clawchat-mobile-commons";

export const LinkMessage = ({ msg, isMe }: { msg: Message; isMe: boolean }) => (
  <div className="flex flex-col gap-2 min-w-[200px] cursor-pointer">
    <span className="font-bold text-[15px] line-clamp-2">
      {msg.metadata?.title || msg.content}
    </span>
    <div className="flex items-start justify-between gap-3">
      <span
        className={cn(
          "text-[13px] line-clamp-2",
          isMe ? "text-white/80" : "text-text-sub",
        )}
      >
        {msg.metadata?.desc || msg.content}
      </span>
      <div
        className={cn(
          "w-10 h-10 rounded shrink-0 flex items-center justify-center",
          isMe ? "bg-white/20" : "bg-black/5 dark:bg-white/5",
        )}
      >
        {msg.metadata?.image ? (
          <img
            src={msg.metadata.image}
            className="w-full h-full object-cover rounded"
          />
        ) : (
          <LinkIcon className="w-5 h-5" />
        )}
      </div>
    </div>
  </div>
);
