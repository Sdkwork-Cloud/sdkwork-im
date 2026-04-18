import * as React from 'react';

export type FileUploadStatus = 'idle' | 'uploading' | 'success' | 'error';

export type FileUploadVariant = 'file' | 'image' | 'video' | 'audio' | 'document';

export type FileUploadListType = 'list' | 'grid';

export interface FileUploadItem {
  id: string;
  name: string;
  size: number;
  type: string;
  status: FileUploadStatus;
  progress?: number;
  error?: string;
  file?: File;
  url?: string;
  previewUrl?: string;
  kind?: FileUploadVariant;
}

export interface FileUploadRejection {
  file: File;
  message: string;
  reason: 'invalid-type' | 'max-files' | 'file-too-large';
}

export type FileUploadItemActionHandler = (item: FileUploadItem) => void;
export type FileUploadClearHandler = () => void;
export type FileUploadRejectHandler = (rejections: FileUploadRejection[]) => void;
export type FileUploadValueChangeHandler = (value: FileUploadItem[]) => void;

export interface FileUploadRootProps extends Omit<React.HTMLAttributes<HTMLDivElement>, 'defaultValue' | 'onChange'> {
  allowPaste?: boolean;
  label?: string;
  clearable?: boolean;
  description?: string;
  directory?: boolean;
  accept?: string | string[];
  defaultValue?: FileUploadItem[];
  disabled?: boolean;
  emptyStateDescription?: string;
  emptyStateTitle?: string;
  listType?: FileUploadListType;
  maxFiles?: number;
  maxSize?: number;
  multiple?: boolean;
  onClear?: FileUploadClearHandler;
  onDownloadItem?: FileUploadItemActionHandler;
  onPreviewItem?: FileUploadItemActionHandler;
  onReject?: FileUploadRejectHandler;
  onRetryItem?: FileUploadItemActionHandler;
  onValueChange?: FileUploadValueChangeHandler;
  rejectionTitle?: string;
  replaceOnMax?: boolean;
  showRejections?: boolean;
  value?: FileUploadItem[];
  variant?: FileUploadVariant;
}

export interface SpecializedUploadProps extends Omit<FileUploadRootProps, 'accept' | 'variant'> {
  accept?: string | string[];
}
