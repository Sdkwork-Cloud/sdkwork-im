import type { FileUploadItem, FileUploadListType, FileUploadStatus, FileUploadVariant } from './upload-types';

export const DOCUMENT_ACCEPT_PRESET = [
  '.pdf',
  '.doc',
  '.docx',
  '.xls',
  '.xlsx',
  '.ppt',
  '.pptx',
  '.csv',
  '.txt',
  '.md',
  '.json',
  '.zip',
  '.rar',
  '.7z',
  'application/pdf',
  'application/msword',
  'application/vnd.openxmlformats-officedocument.wordprocessingml.document',
  'application/vnd.ms-excel',
  'application/vnd.openxmlformats-officedocument.spreadsheetml.sheet',
  'application/vnd.ms-powerpoint',
  'application/vnd.openxmlformats-officedocument.presentationml.presentation',
  'text/plain',
  'text/markdown',
  'text/csv',
  'application/json',
  'application/zip',
  'application/x-rar-compressed',
  'application/x-7z-compressed',
];

interface UploadVariantDefaults {
  accept: string | string[];
  label: string;
  listType: FileUploadListType;
  emptyStateTitle: string;
  helperText: string;
}

const uploadVariantDefaults: Record<FileUploadVariant, UploadVariantDefaults> = {
  file: {
    accept: '*',
    label: 'Upload files',
    listType: 'list',
    emptyStateTitle: 'Click to upload or drag files here',
    helperText: 'All file types are supported.',
  },
  image: {
    accept: 'image/*',
    label: 'Upload images',
    listType: 'grid',
    emptyStateTitle: 'Click to upload or drag images here',
    helperText: 'Image formats such as PNG, JPG, GIF, WEBP, and SVG are supported.',
  },
  video: {
    accept: 'video/*',
    label: 'Upload videos',
    listType: 'list',
    emptyStateTitle: 'Click to upload or drag videos here',
    helperText: 'MP4, MOV, WEBM, and other browser-supported video formats are supported.',
  },
  audio: {
    accept: 'audio/*',
    label: 'Upload audio',
    listType: 'list',
    emptyStateTitle: 'Click to upload or drag audio files here',
    helperText: 'MP3, WAV, AAC, FLAC, and other browser-supported audio formats are supported.',
  },
  document: {
    accept: DOCUMENT_ACCEPT_PRESET,
    label: 'Upload documents',
    listType: 'list',
    emptyStateTitle: 'Click to upload or drag documents here',
    helperText: 'PDF, Office files, markdown, text, JSON, and archive attachments are supported.',
  },
};

function getFileExtension(name: string) {
  const fileName = name.toLowerCase();
  const extensionIndex = fileName.lastIndexOf('.');

  if (extensionIndex === -1) {
    return '';
  }

  return fileName.slice(extensionIndex);
}

export function createFileId() {
  if (typeof crypto !== 'undefined' && 'randomUUID' in crypto) {
    return crypto.randomUUID();
  }

  return `sdk-file-${Math.random().toString(36).slice(2, 10)}`;
}

export function formatFileSize(size: number) {
  if (size < 1024) {
    return `${size} B`;
  }

  if (size < 1024 * 1024) {
    return `${(size / 1024).toFixed(1)} KB`;
  }

  if (size < 1024 * 1024 * 1024) {
    return `${(size / (1024 * 1024)).toFixed(1)} MB`;
  }

  return `${(size / (1024 * 1024 * 1024)).toFixed(1)} GB`;
}

export function normalizeAccept(accept: string | string[] | undefined) {
  if (!accept) {
    return '*';
  }

  if (Array.isArray(accept)) {
    return accept.join(',');
  }

  return accept;
}

export function matchesAccept(file: File, accept: string | string[] | undefined) {
  const normalizedAccept = normalizeAccept(accept);

  if (!normalizedAccept || normalizedAccept === '*') {
    return true;
  }

  return normalizedAccept
    .split(',')
    .map((item) => item.trim())
    .filter(Boolean)
    .some((rule) => {
      if (rule.startsWith('.')) {
        return file.name.toLowerCase().endsWith(rule.toLowerCase());
      }

      if (rule.endsWith('/*')) {
        return file.type.startsWith(rule.slice(0, -1));
      }

      return file.type === rule;
    });
}

export function getUploadVariantDefaults(variant: FileUploadVariant) {
  return uploadVariantDefaults[variant];
}

export function inferUploadItemKind(
  item: Pick<FileUploadItem, 'kind' | 'type' | 'name'> | Pick<File, 'type' | 'name'>,
  fallbackVariant: FileUploadVariant,
): FileUploadVariant {
  if ('kind' in item && item.kind) {
    return item.kind;
  }

  if (item.type.startsWith('image/')) {
    return 'image';
  }

  if (item.type.startsWith('video/')) {
    return 'video';
  }

  if (item.type.startsWith('audio/')) {
    return 'audio';
  }

  if (DOCUMENT_ACCEPT_PRESET.includes(getFileExtension(item.name))) {
    return 'document';
  }

  return fallbackVariant;
}

export function isPreviewableKind(kind: FileUploadVariant) {
  return kind === 'image' || kind === 'video' || kind === 'audio';
}

export function formatStatusLabel(status: FileUploadStatus, progress?: number) {
  if (status === 'uploading') {
    return typeof progress === 'number' ? `Uploading ${Math.round(progress)}%` : 'Uploading';
  }

  if (status === 'success') {
    return 'Uploaded';
  }

  if (status === 'error') {
    return 'Failed';
  }

  return 'Ready';
}

export function formatConstraintText(maxFiles: number, maxSize: number) {
  const parts: string[] = [];

  if (Number.isFinite(maxFiles)) {
    parts.push(`Up to ${maxFiles} file${maxFiles === 1 ? '' : 's'}`);
  }

  if (Number.isFinite(maxSize)) {
    parts.push(`Max ${formatFileSize(maxSize)} each`);
  }

  return parts.join(' / ');
}

export function getFileExtensionLabel(name: string) {
  const extension = getFileExtension(name);

  if (!extension) {
    return 'FILE';
  }

  return extension.replace('.', '').toUpperCase();
}

export function getClipboardFiles(clipboardData: DataTransfer | null | undefined) {
  if (!clipboardData) {
    return [];
  }

  if (clipboardData.files?.length) {
    return Array.from(clipboardData.files);
  }

  if (!clipboardData.items?.length) {
    return [];
  }

  return Array.from(clipboardData.items)
    .map((item) => item.getAsFile())
    .filter((file): file is File => file instanceof File);
}
