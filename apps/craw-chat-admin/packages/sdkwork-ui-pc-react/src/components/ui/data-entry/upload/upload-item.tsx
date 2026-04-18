import * as React from 'react';
import {
  Download,
  Eye,
  FileArchive,
  FileAudio,
  FileCode2,
  FileImage,
  FileJson,
  FileSpreadsheet,
  FileText,
  FileVideo,
  RotateCcw,
  X,
} from 'lucide-react';
import { cn } from '../../../../lib/utils';
import { Button } from '../../button';
import { Progress } from '../../progress';
import type { FileUploadItem, FileUploadItemActionHandler, FileUploadListType, FileUploadVariant } from './upload-types';
import { formatFileSize, formatStatusLabel, getFileExtensionLabel, inferUploadItemKind } from './upload-utils';

interface UploadItemProps {
  disabled?: boolean;
  fallbackVariant: FileUploadVariant;
  item: FileUploadItem;
  listType: FileUploadListType;
  onDownload?: FileUploadItemActionHandler;
  onRemove: (itemId: string) => void;
  onPreview?: FileUploadItemActionHandler;
  onRetry?: FileUploadItemActionHandler;
  previewUrl?: string;
}

function formatMetadataLine(parts: Array<string | undefined | null | false>) {
  return parts.filter(Boolean).join(' / ');
}

function openPreviewUrl(url: string) {
  if (typeof window === 'undefined' || typeof window.open !== 'function') {
    return;
  }

  window.open(url, '_blank', 'noopener,noreferrer');
}

function downloadFileUrl(url: string, name: string) {
  if (typeof document === 'undefined') {
    return;
  }

  const link = document.createElement('a');

  link.href = url;
  link.download = name;
  link.rel = 'noopener noreferrer';
  link.target = '_blank';
  link.click();
}

function getDocumentIcon(name: string) {
  const extension = name.toLowerCase();

  if (extension.endsWith('.zip') || extension.endsWith('.rar') || extension.endsWith('.7z')) {
    return FileArchive;
  }

  if (extension.endsWith('.xls') || extension.endsWith('.xlsx') || extension.endsWith('.csv')) {
    return FileSpreadsheet;
  }

  if (extension.endsWith('.json')) {
    return FileJson;
  }

  if (extension.endsWith('.md')) {
    return FileCode2;
  }

  return FileText;
}

function getFileIcon(item: FileUploadItem, kind: FileUploadVariant) {
  if (kind === 'image') {
    return FileImage;
  }

  if (kind === 'video') {
    return FileVideo;
  }

  if (kind === 'audio') {
    return FileAudio;
  }

  return getDocumentIcon(item.name);
}

function renderMediaPreview(item: FileUploadItem, kind: FileUploadVariant, previewUrl: string | undefined, compact = false) {
  if (kind === 'image' && previewUrl) {
    return <img alt={`Preview ${item.name}`} className="h-full w-full object-cover" src={previewUrl} />;
  }

  if (kind === 'video' && previewUrl) {
    return (
      <video
        aria-label={`Preview ${item.name}`}
        className="h-full w-full object-cover"
        controls={!compact}
        muted
        playsInline
        preload="metadata"
        src={previewUrl}
      />
    );
  }

  return null;
}

function UploadItem({ disabled = false, fallbackVariant, item, listType, onDownload, onRemove, onPreview, onRetry, previewUrl }: UploadItemProps) {
  const kind = inferUploadItemKind(item, fallbackVariant);
  const Icon = getFileIcon(item, kind);
  const hasProgress = item.status === 'uploading' && typeof item.progress === 'number';
  const statusLabel = item.status === 'error' && item.error ? item.error : formatStatusLabel(item.status, item.progress);
  const mediaPreview = renderMediaPreview(item, kind, previewUrl, listType === 'grid');
  const isGrid = listType === 'grid';
  const previewActionUrl = previewUrl ?? item.previewUrl ?? item.url;
  const downloadActionUrl = item.url ?? previewUrl ?? item.previewUrl;
  const hasExplicitPreviewSource = Boolean(item.previewUrl || item.url);
  const hasExplicitDownloadSource = Boolean(item.url || item.previewUrl);
  const canPreview = Boolean(onPreview || hasExplicitPreviewSource);
  const canDownload = Boolean(onDownload || hasExplicitDownloadSource);
  const canRetry = Boolean(onRetry && item.status === 'error' && !disabled);
  const previewActionLabel =
    mediaPreview || (kind === 'audio' && previewUrl) ? `Open preview ${item.name}` : `Preview ${item.name}`;

  const actionRow = canPreview || canDownload || canRetry ? (
    <div className="flex flex-wrap items-center gap-2">
      {canPreview ? (
        <Button
          aria-label={previewActionLabel}
          onClick={() => {
            if (onPreview) {
              onPreview(item);
              return;
            }

            if (previewActionUrl) {
              openPreviewUrl(previewActionUrl);
            }
          }}
          size="sm"
          type="button"
          variant="ghost"
        >
          <Eye className="h-4 w-4" />
          <span>Preview</span>
        </Button>
      ) : null}
      {canDownload ? (
        <Button
          aria-label={`Download ${item.name}`}
          onClick={() => {
            if (onDownload) {
              onDownload(item);
              return;
            }

            if (downloadActionUrl) {
              downloadFileUrl(downloadActionUrl, item.name);
            }
          }}
          size="sm"
          type="button"
          variant="ghost"
        >
          <Download className="h-4 w-4" />
          <span>Download</span>
        </Button>
      ) : null}
      {canRetry ? (
        <Button aria-label={`Retry ${item.name}`} onClick={() => onRetry?.(item)} size="sm" type="button" variant="outline">
          <RotateCcw className="h-4 w-4" />
          <span>Retry</span>
        </Button>
      ) : null}
    </div>
  ) : null;

  if (isGrid) {
    return (
      <div className="rounded-[var(--sdk-radius-control)] border border-[var(--sdk-color-border-default)] bg-[var(--sdk-color-surface-panel)] shadow-[var(--sdk-shadow-sm)]" data-sdk-ui="upload-item" data-sdk-variant="grid">
        <div className="relative aspect-square overflow-hidden rounded-t-[var(--sdk-radius-control)] bg-[var(--sdk-color-surface-panel-muted)]">
          {mediaPreview ? (
            mediaPreview
          ) : (
            <div className="flex h-full w-full items-center justify-center text-[var(--sdk-color-text-secondary)]">
              <Icon className="h-10 w-10" />
            </div>
          )}
          {!disabled ? (
            <button
              className="absolute right-2 top-2 rounded-full bg-[var(--sdk-color-surface-panel)]/90 p-2 text-[var(--sdk-color-text-secondary)] shadow-[var(--sdk-shadow-sm)] transition-colors hover:text-[var(--sdk-color-text-primary)]"
              onClick={() => onRemove(item.id)}
              type="button"
            >
              <X className="h-4 w-4" />
              <span className="sr-only">Remove {item.name}</span>
            </button>
          ) : null}
        </div>
        <div className="space-y-2 px-4 py-3">
          <div className="space-y-1">
            <p className="truncate text-sm font-medium text-[var(--sdk-color-text-primary)]">{item.name}</p>
            <p className="text-xs text-[var(--sdk-color-text-secondary)]">{formatMetadataLine([formatFileSize(item.size), statusLabel])}</p>
          </div>
          {kind === 'audio' && previewUrl ? (
            <audio aria-label={`Preview ${item.name}`} className="w-full" controls preload="metadata" src={previewUrl} />
          ) : null}
          {hasProgress ? <Progress value={item.progress} /> : null}
          {actionRow}
        </div>
      </div>
    );
  }

  return (
    <div className="rounded-[var(--sdk-radius-control)] border border-[var(--sdk-color-border-default)] bg-[var(--sdk-color-surface-panel)] px-4 py-3" data-sdk-ui="upload-item" data-sdk-variant="list">
      <div className="flex items-start gap-3">
        <div
          className={cn(
            'overflow-hidden rounded-[var(--sdk-radius-control)] bg-[var(--sdk-color-surface-panel-muted)] text-[var(--sdk-color-text-secondary)]',
            kind === 'video' ? 'aspect-video w-28 shrink-0' : 'flex h-12 w-12 shrink-0 items-center justify-center',
          )}
        >
          {mediaPreview ? mediaPreview : <Icon className="h-5 w-5" />}
        </div>
        <div className="min-w-0 flex-1 space-y-2">
          <div className="flex items-start justify-between gap-3">
            <div className="min-w-0 space-y-1">
              <p className="truncate text-sm font-medium text-[var(--sdk-color-text-primary)]">{item.name}</p>
              <p className="text-xs text-[var(--sdk-color-text-secondary)]">
                {formatMetadataLine([formatFileSize(item.size), getFileExtensionLabel(item.name), statusLabel])}
              </p>
            </div>
            {!disabled ? (
              <button
                className="rounded-full p-2 text-[var(--sdk-color-text-muted)] transition-colors hover:bg-[var(--sdk-color-brand-primary-soft)] hover:text-[var(--sdk-color-text-primary)]"
                onClick={() => onRemove(item.id)}
                type="button"
              >
                <X className="h-4 w-4" />
                <span className="sr-only">Remove {item.name}</span>
              </button>
            ) : null}
          </div>
          {kind === 'audio' && previewUrl ? (
            <audio aria-label={`Preview ${item.name}`} className="w-full" controls preload="metadata" src={previewUrl} />
          ) : null}
          {hasProgress ? <Progress value={item.progress} /> : null}
          {actionRow}
        </div>
      </div>
    </div>
  );
}

UploadItem.displayName = 'UploadItem';

export { UploadItem };
