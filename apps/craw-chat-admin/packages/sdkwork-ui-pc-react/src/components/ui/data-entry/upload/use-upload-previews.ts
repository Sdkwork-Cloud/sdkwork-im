import * as React from 'react';
import type { FileUploadItem, FileUploadVariant } from './upload-types';
import { inferUploadItemKind, isPreviewableKind } from './upload-utils';

function revokeObjectUrl(url: string) {
  if (typeof URL !== 'undefined' && typeof URL.revokeObjectURL === 'function') {
    URL.revokeObjectURL(url);
  }
}

export function useUploadPreviews(items: FileUploadItem[], fallbackVariant: FileUploadVariant) {
  const generatedUrlsRef = React.useRef<Record<string, string>>({});
  const [generatedUrls, setGeneratedUrls] = React.useState<Record<string, string>>({});

  React.useEffect(() => {
    const nextGeneratedUrls = { ...generatedUrlsRef.current };
    let hasChanges = false;
    const itemIds = new Set(items.map((item) => item.id));

    items.forEach((item) => {
      const explicitPreviewUrl = item.previewUrl ?? item.url;
      const kind = inferUploadItemKind(item, fallbackVariant);

      if (explicitPreviewUrl || !item.file || !isPreviewableKind(kind)) {
        if (nextGeneratedUrls[item.id]) {
          revokeObjectUrl(nextGeneratedUrls[item.id]);
          delete nextGeneratedUrls[item.id];
          hasChanges = true;
        }

        return;
      }

      if (nextGeneratedUrls[item.id] || typeof URL === 'undefined' || typeof URL.createObjectURL !== 'function') {
        return;
      }

      nextGeneratedUrls[item.id] = URL.createObjectURL(item.file);
      hasChanges = true;
    });

    Object.entries(nextGeneratedUrls).forEach(([id, url]) => {
      if (itemIds.has(id)) {
        return;
      }

      revokeObjectUrl(url);
      delete nextGeneratedUrls[id];
      hasChanges = true;
    });

    if (hasChanges) {
      generatedUrlsRef.current = nextGeneratedUrls;
      setGeneratedUrls(nextGeneratedUrls);
    }
  }, [fallbackVariant, items]);

  React.useEffect(
    () => () => {
      Object.values(generatedUrlsRef.current).forEach((url) => revokeObjectUrl(url));
      generatedUrlsRef.current = {};
    },
    [],
  );

  return {
    getPreviewUrl(item: FileUploadItem) {
      return item.previewUrl ?? item.url ?? generatedUrls[item.id];
    },
  };
}
