import type { NotaryPcHostAdapter, NotaryCallOverlayProps, NotaryMediaPreviewProps } from '@sdkwork/notary-pc-commons';
import type { ComponentType } from 'react';
import { CallOverlay, createDefaultAvatar, toast as imToast } from '@sdkwork/im-pc-chat';
import {
  cn,
  MediaViewer,
  resolvePersistedLanguage,
  sanitizeMessageLinkHref,
  SDKWORK_IM_PC_LANGUAGE_CHANGED_EVENT,
} from '@sdkwork/im-pc-commons';
import { bootstrapNotaryPcForIm } from '@sdkwork/im-pc-core';

const SUPPORTED_LANGUAGES = ['zh-CN', 'en-US'] as const;

function createImNotaryPcHostAdapter(): NotaryPcHostAdapter {
  return {
    toast(message, variant = 'info') {
      imToast(message, variant);
    },
    openExternalUrl(url) {
      const safeUrl = sanitizeMessageLinkHref(url);
      if (safeUrl) {
        window.open(safeUrl, '_blank', 'noopener,noreferrer');
      }
    },
    createDefaultAvatar(seed: string) {
      return createDefaultAvatar(seed === 'user' ? 'user' : 'user');
    },
    CallOverlay: CallOverlay as ComponentType<NotaryCallOverlayProps>,
    MediaViewer: MediaViewer as ComponentType<NotaryMediaPreviewProps>,
    sanitizeLinkHref(url) {
      return sanitizeMessageLinkHref(url) ?? '';
    },
    cn,
    resolveInitialLanguage() {
      return resolvePersistedLanguage(SUPPORTED_LANGUAGES, 'zh-CN');
    },
    onLanguageChange(listener) {
      const handler = (event: Event) => {
        const lang = (event as CustomEvent<{ lang?: string }>).detail?.lang;
        if (typeof lang === 'string') {
          listener(lang);
        }
      };
      window.addEventListener(SDKWORK_IM_PC_LANGUAGE_CHANGED_EVENT, handler);
      return () => window.removeEventListener(SDKWORK_IM_PC_LANGUAGE_CHANGED_EVENT, handler);
    },
  };
}

export function bootstrapImNotaryPcIntegration(): void {
  bootstrapNotaryPcForIm(createImNotaryPcHostAdapter());
}
