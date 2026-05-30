import React from "react";
import type { Message } from "@sdkwork/clawchat-mobile-types";

export const TextMessage = ({ msg }: { msg: Message }) => (
  <span className="whitespace-pre-wrap">{msg.content}</span>
);
