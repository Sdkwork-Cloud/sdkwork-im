import { useCallback, useEffect, useState, type ReactNode } from 'react';
import { readPersistedSettingsRecord } from '@sdkwork/im-pc-commons';
import { appAuthService } from '@sdkwork/im-pc-core';

interface PrivacyLockGateProps {
  children: ReactNode;
}

function isPrivacyLockEnabled(): boolean {
  const settings = readPersistedSettingsRecord();
  return settings?.privacyRequireAuth !== false;
}

export function PrivacyLockGate({ children }: PrivacyLockGateProps) {
  const [locked, setLocked] = useState(false);
  const [unlocking, setUnlocking] = useState(false);

  useEffect(() => {
    const lockIfEnabled = () => {
      if (!isPrivacyLockEnabled()) {
        setLocked(false);
        return;
      }

      setLocked(true);
    };

    const handleVisibilityChange = () => {
      if (!isPrivacyLockEnabled()) {
        setLocked(false);
        return;
      }

      if (document.visibilityState === 'hidden') {
        setLocked(true);
      }
    };

    const handleWindowBlur = () => {
      if (!isPrivacyLockEnabled()) {
        return;
      }

      lockIfEnabled();
    };

    handleVisibilityChange();
    document.addEventListener('visibilitychange', handleVisibilityChange);
    window.addEventListener('blur', handleWindowBlur);
    return () => {
      document.removeEventListener('visibilitychange', handleVisibilityChange);
      window.removeEventListener('blur', handleWindowBlur);
    };
  }, []);

  const unlock = useCallback(async () => {
    setUnlocking(true);
    try {
      const session = await appAuthService.getCurrentSession();
      if (!session?.authToken || !session?.accessToken) {
        setLocked(true);
        return;
      }
      setLocked(false);
    } finally {
      setUnlocking(false);
    }
  }, []);

  return (
    <>
      {children}
      {locked ? (
        <div className="fixed inset-0 z-[9999] flex items-center justify-center bg-black/80 backdrop-blur-md">
          <button
            type="button"
            className="rounded-xl border border-white/20 bg-[#2b2b2d] px-6 py-4 text-sm text-gray-100 hover:bg-white/10"
            disabled={unlocking}
            onClick={() => {
              void unlock();
            }}
          >
            {unlocking ? 'Unlocking...' : 'Tap to unlock'}
          </button>
        </div>
      ) : null}
    </>
  );
}
