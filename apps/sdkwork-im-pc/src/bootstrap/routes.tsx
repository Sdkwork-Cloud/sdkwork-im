import { Route, Routes, useNavigate } from 'react-router-dom';
import { ChatLayout } from '@sdkwork/im-pc-chat';
import { ConsoleLayout } from '@sdkwork/im-console-core';
import { AdminLayout } from '@sdkwork/im-admin-core';
import { AuthGate } from '../AuthGate';
import { useTauriTrayNavigationBridge } from './trayNavigation';

function ConsoleApp() {
  const navigate = useNavigate();
  return <ConsoleLayout onSwitchToClient={() => navigate('/')} />;
}

function AdminApp() {
  const navigate = useNavigate();
  return <AdminLayout onSwitchToClient={() => navigate('/')} />;
}

export function AppRoutes() {
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
