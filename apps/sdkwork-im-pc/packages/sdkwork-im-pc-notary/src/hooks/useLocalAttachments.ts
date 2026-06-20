import { useCallback, useRef, useState } from 'react';
import type { LocalAttachment } from '../types';

function inferAttachmentType(file: File): LocalAttachment['type'] {
  if (file.type.startsWith('video/')) {
    return 'video';
  }
  if (file.type === 'application/pdf') {
    return 'pdf';
  }
  if (file.type.startsWith('image/')) {
    return 'image';
  }
  return 'file';
}

export function useLocalAttachments() {
  const [attachments, setAttachments] = useState<LocalAttachment[]>([]);
  const sequenceRef = useRef(0);

  const createId = useCallback(() => {
    sequenceRef.current += 1;
    return `notary-attachment-${sequenceRef.current}`;
  }, []);

  const addFiles = useCallback((files: File[], partyId?: string) => {
    const nextItems = files.map((file) => ({
      id: createId(),
      url: URL.createObjectURL(file),
      name: file.name,
      type: inferAttachmentType(file),
      file,
      partyId,
    }));
    setAttachments((prev) => [...prev, ...nextItems]);
  }, [createId]);

  const removeAttachment = useCallback((id: string) => {
    setAttachments((prev) => prev.filter((item) => item.id !== id));
  }, []);

  const resetAttachments = useCallback(() => {
    setAttachments([]);
  }, []);

  return {
    attachments,
    addFiles,
    removeAttachment,
    resetAttachments,
  };
}
