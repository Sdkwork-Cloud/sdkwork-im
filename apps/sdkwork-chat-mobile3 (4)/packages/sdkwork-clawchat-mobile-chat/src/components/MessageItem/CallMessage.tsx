import React from "react";
import type { Message } from "@sdkwork/clawchat-mobile-types";
import { Phone, Video } from "lucide-react";

export const CallMessage = ({ msg }: { msg: Message }) => (
  <div className="flex items-center gap-2">
    {msg.metadata?.isVideo ? (
      <Video className="w-5 h-5" />
    ) : (
      <Phone className="w-5 h-5" />
    )}
    <span>
      {msg.content} {msg.metadata?.duration && `· ${msg.metadata.duration}`}
    </span>
  </div>
);
