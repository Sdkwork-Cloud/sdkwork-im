import React, { useState, useRef, useEffect } from "react";
import { useTranslation } from "react-i18next";
import { useNavigate } from "react-router-dom";
import { motion, AnimatePresence } from "motion/react";
import { Sidebar } from "../components/Sidebar";
import { ChatList } from "../components/ChatList";
import { ChatWindow } from "../components/ChatWindow";
import { ChatRightPanel } from "../components/ChatRightPanel";
import { WindowControls } from "../components/WindowControls";
import { CallOverlay, CallType } from "../components/CallOverlay";
import { CreateGroupModal } from "../components/CreateGroupModal";
import { AddFriendModal } from "../components/AddFriendModal";
import { CreateAgentModal } from "../components/CreateAgentModal";
import { SettingsModal } from "../components/SettingsModal";
import { AgentView, Agent } from "./AgentView";
import { CreateAgentView } from "./CreateAgentView";
import { VoiceMarketView, Voice } from "@sdkwork/clawchat-pc-voice";
import { ContactsView } from "./ContactsView";
import { FavoritesView } from "./FavoritesView";
import { WorkspaceView } from "@sdkwork/clawchat-pc-workspace";
import { OrdersView } from "@sdkwork/clawchat-pc-orders";
import { NotaryView } from "@sdkwork/clawchat-pc-notary";
import { MailView } from "@sdkwork/clawchat-pc-mail";
import { DriveView } from "@sdkwork/clawchat-pc-drive";
import { CalendarView } from "@sdkwork/clawchat-pc-calendar";
import { ShopView } from "@sdkwork/clawchat-pc-shop";
import { ApprovalsView } from "@sdkwork/clawchat-pc-approvals";
import { DevicesView } from "@sdkwork/clawchat-pc-devices";
import { CommunityView } from "@sdkwork/clawchat-pc-community";
import { ReportsView } from "@sdkwork/clawchat-pc-reports";
import { AttendanceView } from "@sdkwork/clawchat-pc-attendance";
import { KnowledgeView } from "@sdkwork/clawchat-pc-knowledge";
import { CourseView } from "@sdkwork/clawchat-pc-course";
import { EnterpriseView } from "@sdkwork/clawchat-pc-enterprise";
import { VideoGenView } from "@sdkwork/clawchat-pc-video-gen";
import { ImageGenView } from "@sdkwork/clawchat-pc-image-gen";
import { VoiceGenView } from "@sdkwork/clawchat-pc-voice-gen";
import { MusicGenView } from "@sdkwork/clawchat-pc-music-gen";
import { WritingView } from "@sdkwork/clawchat-pc-writing";
import { chatService } from "../services/ChatService";
import { callService } from "../services/CallService";
import { groupService } from "../services/GroupService";
import { imSyncCoordinatorService } from "../services/ImSyncCoordinatorService";
import { appAuthService, isSdkworkChatDesktopRuntime } from "@sdkwork/clawchat-pc-core";
import { Chat } from "@sdkwork/clawchat-pc-types";
import { Avatar, IconButton } from "@sdkwork/clawchat-pc-commons";
import {
  Search,
  Plus,
  Phone,
  Video,
  MoreHorizontal,
  MessageSquarePlus,
  UserPlus,
  Bot,
  X,
} from "lucide-react";
import { ToastContainer, toast } from "../components/Toast";
import { MusicPlayer } from "../components/MusicPlayer";

import i18n from '../i18n';
import { I18nextProvider } from "react-i18next";

const ChatLayoutComponent: React.FC = () => {
  const { t } = useTranslation();
  const navigate = useNavigate();
  const shouldRenderDesktopAppHeader = isSdkworkChatDesktopRuntime();
  const [activeTab, setActiveTab] = useState("chat");
  const [chats, setChats] = useState<Chat[]>([]);
  const [activeChat, setActiveChat] = useState<Chat | null>(null);

  // Call State
  const [isCallOpen, setIsCallOpen] = useState(false);
  const [callMode, setCallMode] = useState<'incoming' | 'outgoing'>('outgoing');
  const [callType, setCallType] = useState<CallType>("voice");
  const [callTarget, setCallTarget] = useState<{
    name: string;
    avatar: string;
    id: string;
  } | null>(null);

  // Plus Menu State
  const [isPlusMenuOpen, setIsPlusMenuOpen] = useState(false);
  const plusMenuRef = useRef<HTMLDivElement>(null);

  // Action Modals State
  const [isCreateGroupOpen, setIsCreateGroupOpen] = useState(false);
  const [isAddFriendOpen, setIsAddFriendOpen] = useState(false);
  const [isCreateAgentModalOpen, setIsCreateAgentModalOpen] = useState(false);
  const [isSettingsOpen, setIsSettingsOpen] = useState(false);
  const [showRHSPanel, setShowRHSPanel] = useState(false);
  const [searchQuery, setSearchQuery] = useState("");
  const [editAgentId, setEditAgentId] = useState<string | undefined>();
  const [activeModal, setActiveModal] = useState<
    "search" | "editName" | "editNotice" | "addMember" | null
  >(null);
  const [modalInput, setModalInput] = useState("");

  useEffect(() => {
    let isMounted = true;
    const loadChats = async () => {
      await imSyncCoordinatorService.syncStartup();
      const data = await chatService.getChats();
      if (!isMounted) {
        return;
      }
      setChats(data);
      if (data.length > 0) {
        setActiveChat(data[0]);
      }
    };

    void loadChats().catch(() => {
      if (!isMounted) {
        return;
      }
      chatService.getChats().then((data) => {
        if (!isMounted) {
          return;
        }
        setChats(data);
        if (data.length > 0) {
          setActiveChat(data[0]);
        }
      });
    });

    return () => {
      isMounted = false;
    };
  }, []);

  useEffect(() => {
    const openSettingsFromTray = () => {
      setActiveTab("chat");
      setIsSettingsOpen(true);
    };

    window.addEventListener("sdkwork-chat-pc:open-settings", openSettingsFromTray);
    if (sessionStorage.getItem("sdkwork-chat-pc:pending-open-settings")) {
      sessionStorage.removeItem("sdkwork-chat-pc:pending-open-settings");
      openSettingsFromTray();
    }
    return () => {
      window.removeEventListener("sdkwork-chat-pc:open-settings", openSettingsFromTray);
    };
  }, []);

  useEffect(() => {
    const handleClickOutside = (event: MouseEvent) => {
      if (
        plusMenuRef.current &&
        !plusMenuRef.current.contains(event.target as Node)
      ) {
        setIsPlusMenuOpen(false);
      }
    };
    if (isPlusMenuOpen) {
      document.addEventListener("mousedown", handleClickOutside);
    }
    return () => {
      document.removeEventListener("mousedown", handleClickOutside);
    };
  }, [isPlusMenuOpen]);

  const handleStartCall = (
    type: CallType,
    target?: { name: string; avatar: string; id: string },
  ) => {
    setCallMode("outgoing");
    setCallType(type);
    if (target) {
      setCallTarget(target);
    } else if (activeChat) {
      setCallTarget({
        name: activeChat.name,
        avatar: activeChat.avatar || "",
        id: activeChat.id,
      });
    }
    setIsCallOpen(true);
  };

  useEffect(() => {
    const conversationIds = chats
      .map((chat) => chat.id)
      .filter((conversationId): conversationId is string => Boolean(conversationId));

    void callService.watchIncomingCalls(conversationIds).catch((error) => {
      toast(error instanceof Error ? error.message : "RTC call watch failed", "error");
    });
  }, [chats]);

  useEffect(() => {
    return callService.subscribe((snapshot) => {
      if (snapshot.direction === 'incoming' && snapshot.state === 'ringing' && snapshot.rtcSessionId) {
        const incomingChat = snapshot.conversationId
          ? chats.find((chat) => chat.id === snapshot.conversationId)
          : undefined;
        setCallMode('incoming');
        setCallType(snapshot.type ?? 'voice');
        setCallTarget({
          id: snapshot.conversationId ?? activeChat?.id ?? snapshot.rtcSessionId,
          name: snapshot.targetName ?? incomingChat?.name ?? activeChat?.name ?? 'Incoming call',
          avatar: incomingChat?.avatar ?? activeChat?.avatar ?? '',
        });
        setIsCallOpen(true);
      }
    });
  }, [activeChat?.avatar, activeChat?.id, activeChat?.name, chats]);

  const handleStartAgentChat = async (agent: Agent) => {
    try {
      const agentChat = await chatService.startAgentChat(agent);
      const refreshedChats = await chatService.getChats().catch(() => chats);
      const nextChats = refreshedChats.some((chat) => chat.id === agentChat.id)
        ? refreshedChats.map((chat) => chat.id === agentChat.id ? { ...chat, ...agentChat } : chat)
        : [agentChat, ...refreshedChats];
      setChats(nextChats);
      setActiveChat(nextChats.find((chat) => chat.id === agentChat.id) ?? agentChat);
      setActiveTab("chat");
    } catch {
      toast("发起智能体聊天失败", "error");
    }
  };

  const handleLogout = async () => {
    try {
      await appAuthService.logout();
      toast("已退出登录", "success");
    } catch {
      toast("已清除本地登录状态", "success");
    } finally {
      setIsSettingsOpen(false);
      navigate("/auth/login?redirect=%2F", { replace: true });
    }
  };

  const renderHeaderContent = () => {
    const titles: Record<string, string> = {
      agent: t('sidebar.agent'),
      voice: t('sidebar.voice'),
      knowledge: t('sidebar.knowledge'),
      contacts: t('sidebar.contacts'),
      favorites: t('sidebar.favorites'),
    };

    return (
      <>
        <div className="w-[280px] h-full shrink-0 flex items-center px-4 gap-2 bg-[#202020] border-r border-white/5">
          <div className="relative flex-1 group">
            <div className="absolute inset-y-0 left-2 flex items-center pointer-events-none text-gray-500">
              <Search size={14} />
            </div>
            <input
              type="text"
              placeholder="搜索"
              value={searchQuery}
              onChange={(e) => setSearchQuery(e.target.value)}
              className="w-full bg-[#181818] text-[13px] text-gray-200 rounded py-1.5 pl-7 pr-3 outline-none placeholder:text-gray-500 border border-white/5 focus:border-white/10"
            />
          </div>

          <div className="relative" ref={plusMenuRef}>
            <button
              className={`w-[28px] h-[28px] flex items-center justify-center rounded border transition-colors ${isPlusMenuOpen ? "bg-white/10 border-white/10 text-gray-200" : "bg-[#181818] border-white/5 text-gray-400 hover:text-gray-200 hover:bg-white/5"}`}
              onClick={() => setIsPlusMenuOpen(!isPlusMenuOpen)}
            >
              <Plus size={16} />
            </button>

            <AnimatePresence>
              {isPlusMenuOpen && (
                <motion.div
                  initial={{ opacity: 0, y: 5, scale: 0.95 }}
                  animate={{ opacity: 1, y: 0, scale: 1 }}
                  exit={{ opacity: 0, y: 5, scale: 0.95 }}
                  transition={{ duration: 0.15, ease: "easeOut" }}
                  className="absolute top-full right-0 mt-2 w-36 bg-[#2b2b2d] border border-white/10 rounded-xl shadow-2xl overflow-hidden z-50 py-1"
                >
                  <button
                    className="w-full px-4 py-2.5 flex items-center gap-3 text-[14px] text-gray-200 hover:bg-white/5 transition-colors"
                    onClick={() => {
                      setIsCreateGroupOpen(true);
                      setIsPlusMenuOpen(false);
                    }}
                  >
                    <MessageSquarePlus size={16} className="text-gray-400" />
                    <span>发起群聊</span>
                  </button>
                  <button
                    className="w-full px-4 py-2.5 flex items-center gap-3 text-[14px] text-gray-200 hover:bg-white/5 transition-colors"
                    onClick={() => {
                      setIsAddFriendOpen(true);
                      setIsPlusMenuOpen(false);
                    }}
                  >
                    <UserPlus size={16} className="text-gray-400" />
                    <span>添加朋友</span>
                  </button>
                  <button
                    className="w-full px-4 py-2.5 flex items-center gap-3 text-[14px] text-gray-200 hover:bg-white/5 transition-colors"
                    onClick={() => {
                      setIsCreateAgentModalOpen(true);
                      setIsPlusMenuOpen(false);
                    }}
                  >
                    <Bot size={16} className="text-gray-400" />
                    <span>创建智能体</span>
                  </button>
                </motion.div>
              )}
            </AnimatePresence>
          </div>
        </div>

        <div className="flex-1 h-full flex items-center justify-between pl-6 pr-0 bg-[#1e1e1e]">
          {activeTab === "chat" && activeChat ? (
            <>
              <div className="flex items-center gap-3">
                <div className="text-[18px] text-gray-200 font-medium tracking-wide">
                  {activeChat.name}
                </div>
              </div>

              <div className="flex items-center gap-1 text-gray-400 mr-4">
                <IconButton
                  title="搜索"
                  className="w-[36px] h-[36px] hover:bg-white/5"
                  onClick={() => {
                    setActiveModal("search");
                    setModalInput("");
                  }}
                >
                  <Search size={18} />
                </IconButton>
                <IconButton
                  title="语音通话"
                  className="w-[36px] h-[36px] hover:bg-white/5"
                  onClick={() => handleStartCall("voice")}
                >
                  <Phone size={18} />
                </IconButton>
                <IconButton
                  title="视频通话"
                  className="w-[36px] h-[36px] hover:bg-white/5"
                  onClick={() => handleStartCall("video")}
                >
                  <Video size={18} />
                </IconButton>
                <IconButton
                  title="更多"
                  className="w-[36px] h-[36px] hover:bg-white/5"
                  onClick={() => setShowRHSPanel(!showRHSPanel)}
                >
                  <MoreHorizontal size={18} />
                </IconButton>
              </div>
            </>
          ) : (
            <div className="text-[18px] text-gray-200 font-medium tracking-wide">
              {titles[activeTab] || ""}
            </div>
          )}
        </div>
      </>
    );
  };

  const renderMainContent = () => {
    switch (activeTab) {
      case "agent":
        return (
          <AgentView
            onStartChat={handleStartAgentChat}
            onCreateAgent={() => {
              setEditAgentId(undefined);
              setIsCreateAgentModalOpen(true);
            }}
            onEditAgent={(id) => {
              setEditAgentId(id);
              setActiveTab("create-agent");
            }}
          />
        );
      case "create-agent":
        return <CreateAgentView onBack={() => { setActiveTab("agent"); setEditAgentId(undefined); }} initialAgentId={editAgentId} />;
      case "voice":
        return (
          <VoiceMarketView
            onSelectVoice={(voice) => {
              toast(`正在加载声音: ${voice.name}`, "success");
            }}
            onCreateVoice={() => {
              toast("即将开启声音克隆功能", "success");
            }}
          />
        );
      case "workspace":
        return (
          <WorkspaceView
            onAppSelect={(appId) => {
              if (appId === "notary") setActiveTab("notary");
              else if (appId === "mail") setActiveTab("mail");
              else if (appId === "drive") setActiveTab("drive");
              else if (appId === "calendar") setActiveTab("calendar");
              else if (appId === "approval") setActiveTab("approval");
              else if (appId === "report") setActiveTab("report");
              else if (appId === "attendance") setActiveTab("attendance");
              else if (appId === "knowledge") setActiveTab("knowledge");
              else if (appId === "devices") setActiveTab("devices");
              else if (appId === "community") setActiveTab("community");
              else if (appId === "videogen") setActiveTab("videogen");
              else if (appId === "imagegen") setActiveTab("imagegen");
              else if (appId === "voicegen") setActiveTab("voicegen");
              else if (appId === "musicgen") setActiveTab("musicgen");
              else if (appId === "writing") setActiveTab("writing");
              else toast(`应用 [${appId}] 未在当前宿主环境注册。`, "error");
            }}
          />
        );
      case "orders":
        return <OrdersView />;
      case "shop":
        return <ShopView onNavigateToOrders={() => setActiveTab("orders")} />;
      case "notary":
        return <NotaryView />;
      case "mail":
        return <MailView />;
      case "drive":
        return <DriveView />;
      case "calendar":
        return <CalendarView />;
      case "approval":
        return <ApprovalsView />;
      case "report":
        return <ReportsView />;
      case "attendance":
        return <AttendanceView />;
      case "knowledge":
        return <KnowledgeView />;
      case "course":
        return <CourseView />;
      case "enterprise":
        return <EnterpriseView 
          onStartChat={async (enterpriseId, enterpriseName) => {
            try {
              const enterpriseChat = await chatService.startEnterpriseChat({
                id: enterpriseId,
                name: enterpriseName,
              });
              const refreshedChats = await chatService.getChats().catch(() => chats);
              const nextChats = refreshedChats.some((chat) => chat.id === enterpriseChat.id)
                ? refreshedChats.map((chat) => chat.id === enterpriseChat.id ? { ...chat, ...enterpriseChat } : chat)
                : [enterpriseChat, ...refreshedChats];
              setChats(nextChats);
              setActiveChat(nextChats.find((chat) => chat.id === enterpriseChat.id) ?? enterpriseChat);
              setActiveTab("chat");
            } catch {
              toast("发起企业聊天失败", "error");
            }
          }}
          onCall={(id, name) => {
            toast(`正在拨打企业 ${name} 电话...`, "success");
          }}
        />;
      case "devices":
        return <DevicesView onEditAgent={(id) => {
          setEditAgentId(id);
          setActiveTab("create-agent");
        }} />;
      case "community":
        return <CommunityView />;
      case "videogen":
        return <VideoGenView />;
      case "imagegen":
        return <ImageGenView />;
      case "voicegen":
        return <VoiceGenView />;
      case "musicgen":
        return <MusicGenView />;
      case "writing":
        return <WritingView />;
      case "contacts":
        return (
          <ContactsView
            searchQuery={searchQuery}
            onSendMessage={async (user) => {
              try {
                const directChat = await chatService.startDirectChat(user);
                const refreshedChats = await chatService.getChats().catch(() => chats);
                const nextChats = refreshedChats.some((chat) => chat.id === directChat.id)
                  ? refreshedChats.map((chat) => chat.id === directChat.id ? { ...chat, ...directChat } : chat)
                  : [directChat, ...refreshedChats];
                setChats(nextChats);
                setActiveChat(nextChats.find((chat) => chat.id === directChat.id) ?? directChat);
                setActiveTab("chat");
              } catch {
                toast("发起聊天失败", "error");
              }
            }}
            onStartCall={(type, user) =>
              handleStartCall(type, {
                name: user.name,
                avatar: user.avatar || "",
                id: user.id,
              })
            }
            onAddFriend={() => setIsAddFriendOpen(true)}
            onAppSelect={(appId) => {
              if (appId === "mail") setActiveTab("mail");
            }}
            onOpenGroup={async (group) => {
              const refreshedChats = await chatService.getChats().catch(() => chats);
              const nextChats = refreshedChats.some((chat) => chat.id === group.id)
                ? refreshedChats.map((chat) => chat.id === group.id ? { ...chat, ...group } : chat)
                : [group, ...refreshedChats];
              setChats(nextChats);
              setActiveChat(nextChats.find((chat) => chat.id === group.id) ?? group);
              setActiveTab("chat");
            }}
          />
        );
      case "favorites":
        return <FavoritesView searchQuery={searchQuery} />;
      default:
        return (
          <div className="flex-1 flex items-center justify-center bg-[#1e1e1e]">
            <div className="text-gray-500 text-xl capitalize">
              {activeTab} Content
            </div>
          </div>
        );
    }
  };

  return (
    <div className="flex flex-col h-screen w-full overflow-hidden bg-[#1e1e1e] font-sans text-gray-200 print:overflow-visible print:h-auto print:min-h-0">
      {shouldRenderDesktopAppHeader && (
        <div className="h-[32px] w-full flex shrink-0 bg-[#181818] border-b border-white/5 drag-region justify-between items-center z-50 print:hidden">
          <div className="text-[12px] text-gray-400 pl-4 font-medium tracking-widest select-none">
            CLAWCHAT
          </div>
          <div className="h-full no-drag">
            <WindowControls />
          </div>
        </div>
      )}

      <div className="flex flex-1 min-h-0 relative print:overflow-visible print:h-auto print:min-h-0 print:block">
        <div className="h-full shrink-0 flex flex-col z-20 print:hidden">
          <Sidebar
            activeTab={activeTab}
            onTabChange={setActiveTab}
            onLogout={handleLogout}
            onOpenSettings={() => setIsSettingsOpen(true)}
            chatUnreadCount={chats.reduce(
              (acc, c) =>
                acc + (c.unreadCount || 0) + ((c.unreadCount || 0) > 0 || !c.isMarkedUnread ? 0 : 1),
              0,
            )}
          />
        </div>

        <div className="flex flex-col flex-1 min-w-0 min-h-0 relative print:overflow-visible print:h-auto print:min-h-0 print:block">
          {/* We only render the Unified App Header for non-fullscreen tabs */}
          {![
            "orders",
            "notary",
            "workspace",
            "calendar",
            "shop",
            "drive",
            "approval",
            "report",
            "attendance",
            "knowledge",
            "create-agent",
            "course",
            "enterprise",
            "voice",
            "videogen",
            "imagegen",
            "voicegen",
            "musicgen",
            "writing",
          ].includes(activeTab) && (
            <div className="h-[64px] w-full flex items-center shrink-0 border-b border-white/5 bg-[#1e1e1e] relative print:hidden">
              {renderHeaderContent()}
            </div>
          )}

          <div className="flex flex-row flex-1 min-h-0 min-w-0 relative print:overflow-visible print:h-auto print:min-h-0 print:block">
            {activeTab === "chat" ? (
              <>
                <ChatList
                  chats={chats}
                  activeChatId={activeChat?.id}
                  onChatSelect={setActiveChat}
                  searchQuery={searchQuery}
                  onChatsChange={() => {
                    chatService.getChats().then((data) => {
                      setChats(data);
                      if (activeChat) {
                        const updated = data.find(
                          (c) => c.id === activeChat.id,
                        );
                        if (updated) setActiveChat(updated);
                      }
                    });
                  }}
                />
                {activeChat ? (
                  <ChatWindow chat={activeChat} />
                ) : (
                  <motion.div
                    initial={{ opacity: 0, scale: 0.95 }}
                    animate={{ opacity: 1, scale: 1 }}
                    transition={{ duration: 0.3 }}
                    className="flex-1 flex flex-col items-center justify-center bg-[#1e1e1e] gap-4"
                  >
                    <div className="w-24 h-24 rounded-full bg-white/5 flex items-center justify-center">
                      <MessageSquarePlus size={36} className="text-gray-500" />
                    </div>
                    <div className="text-gray-500 text-sm">
                      选择一个会话开始聊天
                    </div>
                  </motion.div>
                )}
              </>
            ) : (
              renderMainContent()
            )}

            {/* RHS Chat Panel */}
            <AnimatePresence>
              {activeTab === "chat" && showRHSPanel && activeChat && (
                <ChatRightPanel
                  activeChat={activeChat}
                  onSetModal={(modal, inputVal) => {
                    setActiveModal(modal);
                    setModalInput(inputVal);
                  }}
                  onToggleMute={async () => {
                    const nextMuted = !activeChat.isMuted;
                    try {
                      await chatService.muteChat(activeChat.id, nextMuted);
                      setChats(
                        chats.map((c) =>
                          c.id === activeChat.id ? { ...c, isMuted: nextMuted } : c,
                        ),
                      );
                      setActiveChat({ ...activeChat, isMuted: nextMuted });
                      toast(
                        nextMuted ? "已开启免打扰" : "已取消免打扰",
                        "success",
                      );
                    } catch {
                      toast("设置免打扰失败", "error");
                    }
                  }}
                  onTogglePin={async () => {
                    const nextPinned = !activeChat.isPinned;
                    try {
                      await chatService.pinChat(activeChat.id, nextPinned);
                      setChats(
                        chats.map((c) =>
                          c.id === activeChat.id ? { ...c, isPinned: nextPinned } : c,
                        ),
                      );
                      setActiveChat({ ...activeChat, isPinned: nextPinned });
                      toast(
                        nextPinned ? "已置顶聊天" : "已取消置顶",
                        "success",
                      );
                    } catch {
                      toast("设置置顶失败", "error");
                    }
                  }}
                  onDeleteChat={async () => {
                    try {
                      await chatService.deleteChat(activeChat.id);
                      setChats(chats.filter((c) => c.id !== activeChat.id));
                      setActiveChat(null);
                      setShowRHSPanel(false);
                      toast("已退出群聊", "success");
                    } catch {
                      toast("删除聊天失败", "error");
                    }
                  }}
                />
              )}
            </AnimatePresence>
          </div>
        </div>

        {/* Call Overlay */}
        {callTarget && (
          <CallOverlay
            conversationId={callTarget.id}
            isOpen={isCallOpen}
            mode={callMode}
            type={callType}
            callerName={callTarget.name}
            callerAvatar={callTarget.avatar}
            onClose={() => {
              setIsCallOpen(false);
              setCallMode("outgoing");
            }}
          />
        )}

        {/* Action Modals */}
        <CreateGroupModal
          isOpen={isCreateGroupOpen}
          onClose={() => setIsCreateGroupOpen(false)}
          onCreated={async (group) => {
            const refreshedChats = await chatService.getChats().catch(() => chats);
            const nextChats = refreshedChats.some((chat) => chat.id === group.id)
              ? refreshedChats.map((chat) => chat.id === group.id ? { ...chat, ...group } : chat)
              : [group, ...refreshedChats];
            setChats(nextChats);
            setActiveChat(nextChats.find((chat) => chat.id === group.id) ?? group);
            setActiveTab("chat");
          }}
        />
        <AddFriendModal
          isOpen={isAddFriendOpen}
          onClose={() => setIsAddFriendOpen(false)}
        />
        <SettingsModal
          isOpen={isSettingsOpen}
          onClose={() => setIsSettingsOpen(false)}
          onLogout={handleLogout}
        />
        <CreateAgentModal
          isOpen={isCreateAgentModalOpen}
          onClose={() => setIsCreateAgentModalOpen(false)}
          onSuccess={() => {
            setIsCreateAgentModalOpen(false);
            setActiveTab("create-agent");
          }}
        />
        {/* Custom inline Modals */}
        <AnimatePresence>
          {activeModal && activeChat && (
            <div className="fixed inset-0 z-50 flex items-center justify-center">
              <div
                className="absolute inset-0 bg-black/60 backdrop-blur-sm"
                onClick={() => setActiveModal(null)}
              />
              <motion.div
                initial={{ opacity: 0, scale: 0.95 }}
                animate={{ opacity: 1, scale: 1 }}
                exit={{ opacity: 0, scale: 0.95 }}
                className="relative bg-[#282828] border border-white/10 rounded-2xl w-full max-w-md shadow-2xl p-6"
              >
                <div className="flex justify-between items-center mb-4">
                  <h3 className="text-lg font-medium text-gray-100">
                    {activeModal === "search" && "查找聊天记录"}
                    {activeModal === "addMember" && "添加群成员"}
                    {activeModal === "editName" &&
                      (activeChat.type === "group"
                        ? "修改群聊名称"
                        : "设置备注")}
                    {activeModal === "editNotice" && "编辑群公告"}
                  </h3>
                  <button
                    onClick={() => setActiveModal(null)}
                    className="p-1 text-gray-400 hover:text-gray-100 transition-colors"
                  >
                    <X size={20} />
                  </button>
                </div>

                {activeModal === "search" && (
                  <div>
                    <input
                      type="text"
                      placeholder="搜索消息..."
                      className="w-full bg-[#181818] border border-white/10 rounded-xl px-4 py-2.5 text-sm text-gray-200 outline-none focus:border-indigo-500/50 focus:ring-1 focus:ring-indigo-500/50 transition-all mb-4"
                      value={modalInput}
                      onChange={(e) => setModalInput(e.target.value)}
                      onKeyDown={(e) => {
                        if (e.key === "Enter") {
                          if (!modalInput.trim()) return;
                          setActiveModal(null);
                          toast(`正在搜索: ${modalInput}`, "success");
                        }
                      }}
                      autoFocus
                    />
                    <div className="flex justify-end gap-3 mt-4">
                      <button
                        onClick={() => setActiveModal(null)}
                        className="px-5 py-2 text-sm text-gray-400 hover:text-gray-200 transition-colors"
                      >
                        取消
                      </button>
                      <button
                        onClick={() => {
                          if (!modalInput.trim()) return;
                          setActiveModal(null);
                          toast(`正在搜索: ${modalInput}`, "success");
                        }}
                        disabled={!modalInput.trim()}
                        className="px-5 py-2 text-sm bg-indigo-600 hover:bg-indigo-500 disabled:bg-white/10 disabled:text-gray-500 text-white rounded-xl transition-colors font-medium"
                      >
                        搜索
                      </button>
                    </div>
                  </div>
                )}

                {activeModal === "addMember" && (
                  <div>
                    <input
                      type="text"
                      placeholder="输入成员账号、手机号或邮箱..."
                      className="w-full bg-[#181818] border border-white/10 rounded-xl px-4 py-2.5 text-sm text-gray-200 outline-none focus:border-indigo-500/50 focus:ring-1 focus:ring-indigo-500/50 transition-all mb-4"
                      value={modalInput}
                      onChange={(e) => setModalInput(e.target.value)}
                    />
                    <div className="flex justify-end gap-3 mt-6">
                      <button
                        onClick={() => setActiveModal(null)}
                        className="px-5 py-2 text-sm text-gray-300 hover:bg-white/5 rounded-xl transition-colors"
                      >
                        取消
                      </button>
                      <button
                        onClick={async () => {
                          setActiveModal(null);
                          const addedIds = modalInput
                            .split(",")
                            .map((s) => s.trim())
                            .filter(Boolean);
                          if (addedIds.length > 0) {
                            try {
                              const resolvedMemberIds = await groupService.addMembersBySearchQuery(activeChat.id, addedIds);
                              const refreshedGroups = await groupService.getGroups();
                              const refreshedChat = refreshedGroups.find(
                                (group) => group.id === activeChat.id,
                              );
                              const nextChat = refreshedChat ?? activeChat;
                              setChats(
                                chats.map((c) =>
                                  c.id === activeChat.id ? { ...c, ...nextChat } : c,
                                ),
                              );
                              setActiveChat(nextChat);
                              toast(
                                `已邀请 ${resolvedMemberIds.length} 名成员`,
                                "success",
                              );
                            } catch {
                              toast(`邀请成员失败`, "error");
                            }
                          } else {
                            toast(`请填写邀请成员信息`, "error");
                          }
                        }}
                        className="px-5 py-2 text-sm bg-indigo-600 hover:bg-indigo-500 text-white rounded-xl transition-colors font-medium"
                      >
                        邀请
                      </button>
                    </div>
                  </div>
                )}

                {activeModal === "editName" && (
                  <div>
                    <input
                      type="text"
                      placeholder={
                        activeChat.type === "group" ? "群聊名称" : "备注名称"
                      }
                      className="w-full bg-[#181818] border border-white/10 rounded-xl px-4 py-2.5 text-sm text-gray-200 outline-none focus:border-indigo-500/50 focus:ring-1 focus:ring-indigo-500/50 transition-all mb-4"
                      value={modalInput}
                      onChange={(e) => setModalInput(e.target.value)}
                    />
                    <div className="flex justify-end gap-3 mt-6">
                      <button
                        onClick={() => setActiveModal(null)}
                        className="px-5 py-2 text-sm text-gray-300 hover:bg-white/5 rounded-xl transition-colors"
                      >
                        取消
                      </button>
                      <button
                        onClick={async () => {
                          setActiveModal(null);
                          try {
                            await chatService.updateChat(activeChat.id, {
                              name: modalInput,
                            });
                            setChats(
                              chats.map((c) =>
                                c.id === activeChat.id
                                  ? { ...c, name: modalInput }
                                  : c,
                              ),
                            );
                            setActiveChat({ ...activeChat, name: modalInput });
                            toast(
                              activeChat.type === "group"
                                ? `已修改群名称为: ${modalInput}`
                                : `已修改备注为: ${modalInput}`,
                              "success",
                            );
                          } catch {
                            toast("保存名称失败", "error");
                          }
                        }}
                        className="px-5 py-2 text-sm bg-indigo-600 hover:bg-indigo-500 text-white rounded-xl transition-colors font-medium"
                      >
                        保存
                      </button>
                    </div>
                  </div>
                )}

                {activeModal === "editNotice" && (
                  <div>
                    <textarea
                      placeholder="填写群公告..."
                      className="w-full bg-[#181818] border border-white/10 rounded-xl px-4 py-2.5 text-sm text-gray-200 outline-none focus:border-indigo-500/50 focus:ring-1 focus:ring-indigo-500/50 transition-all mb-4 min-h-[120px] resize-none"
                      value={modalInput === "暂无公告" ? "" : modalInput}
                      onChange={(e) => setModalInput(e.target.value)}
                    />
                    <div className="flex justify-end gap-3 mt-6">
                      <button
                        onClick={() => setActiveModal(null)}
                        className="px-5 py-2 text-sm text-gray-300 hover:bg-white/5 rounded-xl transition-colors"
                      >
                        取消
                      </button>
                      <button
                        onClick={async () => {
                          setActiveModal(null);
                          try {
                            await chatService.updateChat(activeChat.id, {
                              notice: modalInput,
                            });
                            setChats(
                              chats.map((c) =>
                                c.id === activeChat.id
                                  ? { ...c, notice: modalInput }
                                  : c,
                              ),
                            );
                            setActiveChat({ ...activeChat, notice: modalInput });
                            toast(`群公告已更新`, "success");
                          } catch {
                            toast("更新群公告失败", "error");
                          }
                        }}
                        className="px-5 py-2 text-sm bg-indigo-600 hover:bg-indigo-500 text-white rounded-xl transition-colors font-medium"
                      >
                        发布群公告
                      </button>
                    </div>
                  </div>
                )}
              </motion.div>
            </div>
          )}
        </AnimatePresence>

        <ToastContainer />
        <MusicPlayer />
      </div>
    </div>
  );
};

export const ChatLayout: React.FC = () => {
  return (
    <I18nextProvider i18n={i18n}>
      <ChatLayoutComponent />
    </I18nextProvider>
  );
};
