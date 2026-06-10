/**
 * @license
 * SPDX-License-Identifier: Apache-2.0
 */

import { useEffect } from 'react';
import { BrowserRouter, Routes, Route, useNavigate } from 'react-router-dom';
import { ChatLayout } from '@sdkwork/clawchat-pc-chat';
import { ConsoleLayout } from '@sdkwork/clawchat-console-core';
import { AdminLayout } from '@sdkwork/clawchat-admin-core';
import { AuthGate } from './AuthGate';

type TauriUnlisten = () => void;
type TauriListen = (event: string, handler: () => void) => Promise<TauriUnlisten>;

const TRAY_PENDING_SETTINGS_STORAGE_KEY = 'sdkwork-chat-pc:pending-open-settings';

function resolveTauriListen(): TauriListen | null {
  return (globalThis as {
    __TAURI__?: {
      event?: {
        listen?: TauriListen;
      };
    };
  }).__TAURI__?.event?.listen ?? null;
}

function useTauriTrayNavigationBridge() {
  const navigate = useNavigate();

  useEffect(() => {
    const listen = resolveTauriListen();
    if (!listen) {
      return;
    }

    const unlisteners: TauriUnlisten[] = [];
    let disposed = false;

    void listen('sdkwork-chat-pc://tray/open-chat', () => {
      navigate('/', { replace: false });
    }).then((unlisten) => {
      if (disposed) {
        unlisten();
      } else {
        unlisteners.push(unlisten);
      }
    });

    void listen('sdkwork-chat-pc://tray/open-settings', () => {
      sessionStorage.setItem(TRAY_PENDING_SETTINGS_STORAGE_KEY, '1');
      navigate('/', { replace: false });
      window.dispatchEvent(new CustomEvent('sdkwork-chat-pc:open-settings'));
    }).then((unlisten) => {
      if (disposed) {
        unlisten();
      } else {
        unlisteners.push(unlisten);
      }
    });

    void listen('sdkwork-chat-pc://tray/show-active-call', () => {
      navigate('/', { replace: false });
      window.dispatchEvent(new CustomEvent('sdkwork-chat-pc:show-active-call'));
    }).then((unlisten) => {
      if (disposed) {
        unlisten();
      } else {
        unlisteners.push(unlisten);
      }
    });

    return () => {
      disposed = true;
      for (const unlisten of unlisteners.splice(0)) {
        unlisten();
      }
    };
  }, [navigate]);
}

function ConsoleApp() {
  const navigate = useNavigate();
  return <ConsoleLayout onSwitchToClient={() => navigate('/')} />;
}

function AdminApp() {
  const navigate = useNavigate();
  return <AdminLayout onSwitchToClient={() => navigate('/')} />;
}

function AppRoutes() {
  useTauriTrayNavigationBridge();

  return (
    <AuthGate>
      <Routes>
        <Route path="/console/*" element={<ConsoleApp />} />
        <Route path="/admin/*" element={<AdminApp />} />
        <Route path="/*" element={<ChatLayout />} />
      </Routes>
    </AuthGate>
  );
}

export default function App() {
  return (
    <BrowserRouter>
      <AppRoutes />
    </BrowserRouter>
  );
}
