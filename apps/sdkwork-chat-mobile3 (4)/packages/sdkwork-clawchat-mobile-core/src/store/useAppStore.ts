import { create } from "zustand";
import type { User } from "@sdkwork/clawchat-mobile-types";

interface AppState {
  currentUser: User | null;
  setCurrentUser: (user: User | null) => void;
}

export const useAppStore = create<AppState>((set) => ({
  currentUser: {
    id: "u1",
    name: "Alex Chen",
    avatar: "https://picsum.photos/seed/alex/200/200",
    status: "online",
  },
  setCurrentUser: (user) => set({ currentUser: user }),
}));
