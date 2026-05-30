/**
 * @license
 * SPDX-License-Identifier: Apache-2.0
 */

import React, { useState, useEffect } from 'react';
import { BrowserRouter, Routes, Route, useNavigate, useLocation } from 'react-router';
import { MessageCircle, Bot, LayoutGrid, Compass, UserRound, Play, Pause, Contact, X } from 'lucide-react';
import { ChatList, ChatDetail, GlobalSearch, VoiceCall, VideoCall, ChatProfile, CreateGroupChat } from '@sdkwork/clawchat-mobile-chat';
import { AddressBook, AgentList, AgentSearch, AgentCreate, AddFriend, Scan } from '@sdkwork/clawchat-mobile-contacts';
import { Workspace, WorkspaceNotary, Discover, Me, AuthPage, MyCharacters, CreateCharacter, MyVoices, CreateVoice, SettingsPage, MyProfile, AccountSecurity, TeenMode, ElderlyMode, Notifications, ChatSettings, Devices, General, FriendPermissions, Privacy, InfoCollection, ThirdPartySharing, Plugins, HelpFeedback, About, SwitchAccount, ProfileAvatar, ProfileName, ProfileTickle, ProfileQRCode, ProfileMore, ProfileRingtone, ProfileBeans, ProfileAddress, ChangePhoneNumber, ChangePassword, VoiceLock, EmergencyContacts, MoreSecurity, ChatBackground, EmojiManagement, ClearChatHistory, FontSize, MediaSettings, StorageSpace, SystemPermissions, AuthManagement, AdManagement, Blacklist, FAQ, Feedback, Features, Complain, TOS, PrivacyPolicy, WechatID, ResetVoiceLock, BindQQ, BindEmail, RecoverPassword, DeleteAccount, ManageChatHistory, Gender, Region, Signature, ServicesPage, FavoritesPage, MyAgentsPage, MyWorksPage, EmojiPage, MomentsPage, ChannelsPage, SearchPage, GamesPage, CloudDriveApp } from '@sdkwork/clawchat-mobile-user';
import { ApprovalApp, CreateApproval, ApprovalDetail } from '@sdkwork/clawchat-mobile-approval';
import { ReportApp, CreateReport, ReportDetail } from '@sdkwork/clawchat-mobile-report';
import { AttendanceApp } from '@sdkwork/clawchat-mobile-attendance';
import { ShoppingPage, ShoppingCartPage, ProductDetails, ShopDetails, CustomerServiceChat, CheckoutPage, CashierPage } from '@sdkwork/clawchat-mobile-shopping';
import { OrderCenter, OrderDetail, VoucherCodePage } from '@sdkwork/clawchat-mobile-orders';
import { NotaryLayout, NotaryRecords, NotaryFiles, NotaryMessages, NotaryMe, CreateNotaryProcess, NotarySearchList, NotaryAddParty, NotaryDetail, NotaryVideoCall } from '@sdkwork/clawchat-mobile-notary';
import { CalendarWorkspace } from '@sdkwork/clawchat-mobile-calendar';
import { MeetingApp, CreateMeeting, MeetingDetail } from '@sdkwork/clawchat-mobile-meeting';
import { RecruitmentApp, CandidateDetail, CreateJob } from '@sdkwork/clawchat-mobile-recruitment';
import { VoiceSummaryApp } from '@sdkwork/clawchat-mobile-ai-voice';
import { AIVideoPage } from '@sdkwork/clawchat-mobile-ai-video';
import { AIImagePage } from '@sdkwork/clawchat-mobile-ai-image';
import { AIWritingPage } from '@sdkwork/clawchat-mobile-ai-writing';
import { cn } from '@sdkwork/clawchat-mobile-commons';
import { useAudioStore } from '@sdkwork/clawchat-mobile-core';
import { MusicPlayerPage } from './pages/MusicPlayerPage';
import { AuthGuard } from './AuthGuard';

// Custom filled SVG variants for the active tabs
const TabSolidMessage = ({ className }: any) => (
  <svg viewBox="0 0 24 24" className={className} stroke="none">
    <path d="M7.9 20A9 9 0 1 0 4 16.1L2 22Z" fill="currentColor" />
  </svg>
);

const TabSolidBot = ({ className }: any) => (
  <svg viewBox="0 0 24 24" className={className} fill="none">
    <path d="M12 2v6" stroke="currentColor" strokeWidth="2" strokeLinecap="round" />
    <path d="M8 8V6a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2" stroke="currentColor" strokeWidth="2" strokeLinecap="round" />
    <rect x="3" y="10" width="18" height="12" rx="2" fill="currentColor" stroke="currentColor" strokeWidth="2" />
    <circle cx="8.5" cy="15.5" r="1.5" fill="white" />
    <circle cx="15.5" cy="15.5" r="1.5" fill="white" />
    <path d="M8 15.5h.01M16 15.5h.01" stroke="currentColor" strokeWidth="0" />
  </svg>
);

const TabSolidContact = ({ className }: any) => (
  <svg viewBox="0 0 24 24" className={className} fill="none">
    <rect x="3" y="2" width="18" height="20" rx="3" fill="currentColor" />
    <circle cx="12" cy="10" r="3" fill="#fff" />
    <path d="M7 18c0-2.5 3-4 5-4s5 1.5 5 4" stroke="#fff" strokeWidth="2" strokeLinecap="round" />
  </svg>
);

const TabSolidWorkspace = ({ className }: any) => (
  <svg viewBox="0 0 24 24" className={className} stroke="none">
    <rect x="3" y="3" width="7" height="7" rx="1" fill="currentColor" />
    <rect x="14" y="3" width="7" height="7" rx="1" fill="currentColor" />
    <rect x="14" y="14" width="7" height="7" rx="1" fill="currentColor" />
    <rect x="3" y="14" width="7" height="7" rx="1" fill="currentColor" />
  </svg>
);

const TabSolidDiscover = ({ className }: any) => (
  <svg viewBox="0 0 24 24" className={className} fill="none">
    <circle cx="12" cy="12" r="10" stroke="currentColor" strokeWidth="2" fill="transparent" />
    <polygon points="16.24 7.76 14.12 14.12 7.76 16.24 9.88 9.88 16.24 7.76" fill="currentColor" />
  </svg>
);

const TabSolidUser = ({ className }: any) => (
  <svg viewBox="0 0 24 24" className={className} fill="none">
    <circle cx="12" cy="7" r="5" fill="currentColor" />
    <path d="M20 21a8 8 0 0 0-16 0" fill="currentColor" stroke="currentColor" strokeWidth="2" strokeLinecap="round" />
  </svg>
);

const TabBar = () => {
  const navigate = useNavigate();
  const location = useLocation();

  // Don't show tab bar on detail pages
  if (location.pathname.startsWith('/chat/') || location.pathname.startsWith('/workspace/') || location.pathname.startsWith('/call/') || location.pathname === '/search' || location.pathname === '/agent-search' || location.pathname.startsWith('/settings') || location.pathname === '/agent/create' || location.pathname === '/add-friend' || location.pathname === '/create-group' || location.pathname === '/scan' || location.pathname.startsWith('/my-profile') || location.pathname.startsWith('/me/') || location.pathname.startsWith('/discover/') || location.pathname.startsWith('/notary') || location.pathname === '/music-player' || location.pathname === '/login' || location.pathname.startsWith('/product/') || location.pathname.startsWith('/shop/') || location.pathname.startsWith('/shop-chat/') || location.pathname === '/cart' || location.pathname === '/checkout' || location.pathname === '/cashier' || location.pathname.startsWith('/ai/')) return null;

  const tabs = [
    { id: 'chat', outline: MessageCircle, solid: TabSolidMessage, label: '微信', path: '/' },
    { id: 'agents', outline: Bot, solid: TabSolidBot, label: '智能体', path: '/agents' },
    { id: 'workspace', outline: LayoutGrid, solid: TabSolidWorkspace, label: '工作台', path: '/workspace' },
    { id: 'discover', outline: Compass, solid: TabSolidDiscover, label: '发现', path: '/discover' },
    { id: 'me', outline: UserRound, solid: TabSolidUser, label: '我', path: '/me' },
  ];

  return (
    <nav className="w-full pb-safe pt-2 flex justify-around items-start glass-tab-bar z-40 shrink-0 absolute bottom-0 left-0">
      {tabs.map((tab) => {
        const isActive = location.pathname === tab.path || (tab.path === '/' && location.pathname === '');
        const Icon = isActive ? tab.solid : tab.outline;
        return (
          <div
            key={tab.id}
            onClick={() => navigate(tab.path)}
            className={cn(
              "flex flex-col items-center gap-1 text-[10px] cursor-pointer transition-colors mb-1",
              isActive ? "text-primary-blue" : "text-text-sub"
            )}
          >
            <Icon 
              className={cn("w-6 h-6 transition-all", isActive ? "opacity-100 scale-110" : "opacity-50 scale-100")} 
              strokeWidth={isActive ? undefined : 1.5}
            />
            <span>{tab.label}</span>
          </div>
        );
      })}
    </nav>
  );
};

const GlobalMiniPlayer = () => {
  const navigate = useNavigate();
  const location = useLocation();
  const currentTrack = useAudioStore(s => s.currentTrack);
  const isPlaying = useAudioStore(s => s.isPlaying);
  const pause = useAudioStore(s => s.pause);
  const resume = useAudioStore(s => s.resume);
  const stop = useAudioStore(s => s.stop);

  // Hidden on player page
  if (!currentTrack || location.pathname === '/music-player') return null;

  return (
    <div 
      className="absolute top-[80px] right-0 z-[100] flex items-center bg-bg-color/90 dark:bg-[#2c2c2e]/95 backdrop-blur-xl border border-r-0 border-border-color shadow-sm rounded-l-full py-1.5 pl-2 pr-1 cursor-pointer shadow-[0_2px_8px_rgba(0,0,0,0.08)] active:scale-95 transition-transform"
      onClick={() => navigate('/music-player')}
    >
      <div className={cn("w-8 h-8 rounded-full overflow-hidden shrink-0 border border-border-color/50 relative", isPlaying && "animate-spin")} style={{ animationDuration: '5s' }}>
        <img src={currentTrack.coverUrl} alt="Cover" className="w-full h-full object-cover" />
      </div>
      <div 
        className="w-6 h-6 flex items-center justify-center shrink-0 ml-1 text-text-sub"
        onClick={(e) => {
          e.stopPropagation();
          isPlaying ? pause() : resume();
        }}
      >
        {isPlaying ? <Pause className="w-3.5 h-3.5 text-text-main fill-current" /> : <Play className="w-3.5 h-3.5 text-text-main fill-current ml-0.5" />}
      </div>
      <div 
        className="w-6 h-6 flex items-center justify-center shrink-0 text-text-sub/60 hover:text-text-main hover:bg-chat-active-bg rounded-full transition-colors"
        onClick={(e) => {
          e.stopPropagation();
          stop();
        }}
      >
        <X className="w-3.5 h-3.5" />
      </div>
    </div>
  );
};

export default function App() {
  const initAudio = useAudioStore(s => s.initAudio);
  
  useEffect(() => {
    initAudio();
  }, [initAudio]);

  return (
    <BrowserRouter>
      <div className="w-full h-full flex flex-col relative bg-bg-color overflow-hidden">
        <GlobalMiniPlayer />
        <div className="flex-1 overflow-hidden relative flex flex-col">
          <AuthGuard>
            <Routes>
              <Route path="/login" element={<AuthPage />} />
              <Route path="/" element={<ChatList />} />
            <Route path="/chat/:id" element={<ChatDetail />} />
            <Route path="/chat/:id/profile" element={<ChatProfile />} />
            <Route path="/create-group" element={<CreateGroupChat />} />
            <Route path="/call/voice/:id" element={<VoiceCall />} />
            <Route path="/call/video/:id" element={<VideoCall />} />
            <Route path="/search" element={<GlobalSearch />} />
            <Route path="/agent-search" element={<AgentSearch />} />
            <Route path="/agent/create" element={<AgentCreate />} />
            <Route path="/add-friend" element={<AddFriend />} />
            <Route path="/scan" element={<Scan />} />
            <Route path="/agents" element={<AgentList />} />
            <Route path="/workspace/contacts" element={<AddressBook />} />
            <Route path="/workspace" element={<Workspace />} />
            <Route path="/workspace/notary" element={<WorkspaceNotary />} />
            <Route path="/workspace/approval" element={<ApprovalApp />} />
            <Route path="/workspace/approval/create" element={<CreateApproval />} />
            <Route path="/workspace/approval/:id" element={<ApprovalDetail />} />
            <Route path="/workspace/report" element={<ReportApp />} />
            <Route path="/workspace/report/create" element={<CreateReport />} />
            <Route path="/workspace/report/:id" element={<ReportDetail />} />
            <Route path="/workspace/attendance" element={<AttendanceApp />} />
            <Route path="/workspace/drive" element={<CloudDriveApp />} />
            <Route path="/workspace/meeting" element={<MeetingApp />} />
            <Route path="/workspace/meeting/create" element={<CreateMeeting />} />
            <Route path="/workspace/meeting/:id" element={<MeetingDetail />} />
            <Route path="/workspace/recruitment" element={<RecruitmentApp />} />
            <Route path="/workspace/recruitment/create" element={<CreateJob />} />
            <Route path="/workspace/recruitment/:id" element={<CandidateDetail />} />
            <Route path="/workspace/voice-summary" element={<VoiceSummaryApp />} />
            <Route path="/calendar" element={<CalendarWorkspace />} />

            <Route path="/ai/video" element={<AIVideoPage />} />
            <Route path="/ai/image" element={<AIImagePage />} />
            <Route path="/ai/writing" element={<AIWritingPage />} />
            
            <Route path="/notary" element={<NotaryLayout />}>
              <Route index element={<NotaryRecords />} />
              <Route path="files" element={<NotaryFiles />} />
              <Route path="messages" element={<NotaryMessages />} />
              <Route path="me" element={<NotaryMe />} />
            </Route>
            <Route path="/notary/create" element={<CreateNotaryProcess />} />
            <Route path="/notary/select-notary" element={<NotarySearchList />} />
            <Route path="/notary/add-party" element={<NotaryAddParty />} />
            <Route path="/notary/detail/:id" element={<NotaryDetail />} />
            <Route path="/call/video-notary-:id" element={<NotaryVideoCall />} />

            <Route path="/discover" element={<Discover />} />
            <Route path="/discover/moments" element={<MomentsPage />} />
            <Route path="/discover/channels" element={<ChannelsPage />} />
            <Route path="/discover/search" element={<SearchPage />} />
            <Route path="/discover/games" element={<GamesPage />} />
            <Route path="/discover/shopping" element={<ShoppingPage />} />
            <Route path="/cart" element={<ShoppingCartPage />} />
            <Route path="/checkout" element={<CheckoutPage />} />
            <Route path="/cashier" element={<CashierPage />} />
            <Route path="/product/:id" element={<ProductDetails />} />
            <Route path="/shop/:id" element={<ShopDetails />} />
            <Route path="/shop-chat/:id" element={<CustomerServiceChat />} />
            <Route path="/me" element={<Me />} />
            <Route path="/me/characters" element={<MyCharacters />} />
            <Route path="/me/characters/create" element={<CreateCharacter />} />
            <Route path="/me/voices" element={<MyVoices />} />
            <Route path="/me/voices/create" element={<CreateVoice />} />
            <Route path="/me/services" element={<ServicesPage />} />
            <Route path="/me/orders" element={<OrderCenter />} />
            <Route path="/me/orders/:id" element={<OrderDetail />} />
            <Route path="/me/orders/:id/voucher/:code" element={<VoucherCodePage />} />
            <Route path="/me/favorites" element={<FavoritesPage />} />
            <Route path="/me/agents" element={<MyAgentsPage />} />
            <Route path="/me/works" element={<MyWorksPage />} />
            <Route path="/me/emoji" element={<EmojiPage />} />
            <Route path="/my-profile" element={<MyProfile />} />
            <Route path="/my-profile/avatar" element={<ProfileAvatar />} />
            <Route path="/my-profile/name" element={<ProfileName />} />
            <Route path="/my-profile/tickle" element={<ProfileTickle />} />
            <Route path="/my-profile/qrcode" element={<ProfileQRCode />} />
            <Route path="/my-profile/more" element={<ProfileMore />} />
            <Route path="/my-profile/more/gender" element={<Gender />} />
            <Route path="/my-profile/more/region" element={<Region />} />
            <Route path="/my-profile/more/signature" element={<Signature />} />
            <Route path="/my-profile/ringtone" element={<ProfileRingtone />} />
            <Route path="/my-profile/beans" element={<ProfileBeans />} />
            <Route path="/my-profile/address" element={<ProfileAddress />} />
            <Route path="/settings" element={<SettingsPage />} />
            <Route path="/settings/account" element={<AccountSecurity />} />
            <Route path="/settings/account/wechat-id" element={<WechatID />} />
            <Route path="/settings/account/phone" element={<ChangePhoneNumber />} />
            <Route path="/settings/account/password" element={<ChangePassword />} />
            <Route path="/settings/account/voice-lock" element={<VoiceLock />} />
            <Route path="/settings/account/voice-lock/reset" element={<ResetVoiceLock />} />
            <Route path="/settings/account/emergency" element={<EmergencyContacts />} />
            <Route path="/settings/account/more" element={<MoreSecurity />} />
            <Route path="/settings/account/more/qq" element={<BindQQ />} />
            <Route path="/settings/account/more/email" element={<BindEmail />} />
            <Route path="/settings/account/more/recover" element={<RecoverPassword />} />
            <Route path="/settings/account/more/delete" element={<DeleteAccount />} />
            <Route path="/settings/teen-mode" element={<TeenMode />} />
            <Route path="/settings/elderly-mode" element={<ElderlyMode />} />
            <Route path="/settings/notifications" element={<Notifications />} />
            <Route path="/settings/chat" element={<ChatSettings />} />
            <Route path="/settings/chat/background" element={<ChatBackground />} />
            <Route path="/settings/chat/emoji" element={<EmojiManagement />} />
            <Route path="/settings/chat/clear" element={<ClearChatHistory />} />
            <Route path="/settings/devices" element={<Devices />} />
            <Route path="/settings/general" element={<General />} />
            <Route path="/settings/general/font-size" element={<FontSize />} />
            <Route path="/settings/general/media" element={<MediaSettings />} />
            <Route path="/settings/general/storage" element={<StorageSpace />} />
            <Route path="/settings/general/storage/chat" element={<ManageChatHistory />} />
            <Route path="/settings/friend-permissions" element={<FriendPermissions />} />
            <Route path="/settings/friend-permissions/blacklist" element={<Blacklist />} />
            <Route path="/settings/privacy" element={<Privacy />} />
            <Route path="/settings/privacy/system" element={<SystemPermissions />} />
            <Route path="/settings/privacy/auth" element={<AuthManagement />} />
            <Route path="/settings/privacy/ads" element={<AdManagement />} />
            <Route path="/settings/info-collection" element={<InfoCollection />} />
            <Route path="/settings/third-party-sharing" element={<ThirdPartySharing />} />
            <Route path="/settings/plugins" element={<Plugins />} />
            <Route path="/settings/help" element={<HelpFeedback />} />
            <Route path="/settings/help/faq" element={<FAQ />} />
            <Route path="/settings/help/feedback" element={<Feedback />} />
            <Route path="/settings/about" element={<About />} />
            <Route path="/settings/about/features" element={<Features />} />
            <Route path="/settings/about/complain" element={<Complain />} />
            <Route path="/settings/about/tos" element={<TOS />} />
            <Route path="/settings/about/privacy" element={<PrivacyPolicy />} />
            <Route path="/settings/switch-account" element={<SwitchAccount />} />
            <Route path="/music-player" element={<MusicPlayerPage />} />
          </Routes>
          </AuthGuard>
        </div>
        <TabBar />
      </div>
    </BrowserRouter>
  );
}
