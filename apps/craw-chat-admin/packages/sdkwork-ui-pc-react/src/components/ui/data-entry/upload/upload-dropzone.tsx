import * as React from 'react';
import { FileText, ImagePlus, Music4, UploadCloud, Video } from 'lucide-react';
import { cn } from '../../../../lib/utils';
import type { FileUploadVariant } from './upload-types';

interface UploadDropzoneProps {
  caption?: string;
  description: string;
  disabled?: boolean;
  isDragging: boolean;
  onBrowse: () => void;
  onDragLeave: () => void;
  onDragOver: (event: React.DragEvent<HTMLDivElement>) => void;
  onDrop: (event: React.DragEvent<HTMLDivElement>) => void;
  onPaste?: (event: React.ClipboardEvent<HTMLDivElement>) => void;
  title: string;
  variant: FileUploadVariant;
}

function getDropzoneIcon(variant: FileUploadVariant) {
  if (variant === 'image') {
    return ImagePlus;
  }

  if (variant === 'video') {
    return Video;
  }

  if (variant === 'audio') {
    return Music4;
  }

  if (variant === 'document') {
    return FileText;
  }

  return UploadCloud;
}

function UploadDropzone({
  caption,
  description,
  disabled = false,
  isDragging,
  onBrowse,
  onDragLeave,
  onDragOver,
  onDrop,
  onPaste,
  title,
  variant,
}: UploadDropzoneProps) {
  const Icon = getDropzoneIcon(variant);

  return (
    <div
      aria-disabled={disabled}
      className={cn(
        'rounded-[var(--sdk-radius-control)] border border-dashed px-5 py-6 transition-colors',
        isDragging
          ? 'border-[var(--sdk-color-brand-primary)] bg-[var(--sdk-color-brand-primary-soft)]'
          : 'border-[var(--sdk-color-border-strong)] bg-[var(--sdk-color-surface-panel)]',
        disabled && 'cursor-not-allowed opacity-60',
      )}
      data-sdk-ui="upload-dropzone"
      onClick={() => {
        if (!disabled) {
          onBrowse();
        }
      }}
      onDragLeave={onDragLeave}
      onDragOver={onDragOver}
      onDrop={onDrop}
      onPaste={onPaste}
      onKeyDown={(event) => {
        if (disabled) {
          return;
        }

        if (event.key === 'Enter' || event.key === ' ') {
          event.preventDefault();
          onBrowse();
        }
      }}
      role="button"
      tabIndex={disabled ? -1 : 0}
    >
      <div className="flex items-start gap-4">
        <div className="rounded-[var(--sdk-radius-control)] bg-[var(--sdk-color-brand-primary-soft)] p-3 text-[var(--sdk-color-brand-primary)]">
          <Icon className="h-5 w-5" />
        </div>
        <div className="space-y-1">
          <p className="text-sm font-medium text-[var(--sdk-color-text-primary)]">{title}</p>
          <p className="text-sm text-[var(--sdk-color-text-secondary)]">{description}</p>
          {caption ? <p className="text-xs text-[var(--sdk-color-text-muted)]">{caption}</p> : null}
        </div>
      </div>
    </div>
  );
}

UploadDropzone.displayName = 'UploadDropzone';

export { UploadDropzone };
