import * as React from 'react';
import { cn } from '../../../../lib/utils';
import { Button } from '../../button';
import { Label } from '../../label';
import { InlineAlert } from '../../feedback/inline-alert';
import { UploadDropzone } from './upload-dropzone';
import type {
  FileUploadItem,
  FileUploadRootProps,
  FileUploadRejection,
} from './upload-types';
import { useUploadPreviews } from './use-upload-previews';
import { UploadItem } from './upload-item';
import {
  createFileId,
  formatConstraintText,
  formatFileSize,
  getClipboardFiles,
  getUploadVariantDefaults,
  inferUploadItemKind,
  matchesAccept,
  normalizeAccept,
} from './upload-utils';

export type FileUploadProps = FileUploadRootProps;

function useControllableFileList(value: FileUploadItem[] | undefined, defaultValue: FileUploadItem[], onValueChange?: (value: FileUploadItem[]) => void) {
  const [internalValue, setInternalValue] = React.useState(defaultValue);
  const isControlled = value !== undefined;
  const currentValue = isControlled ? value : internalValue;

  const setValue = React.useCallback(
    (nextValue: FileUploadItem[]) => {
      if (!isControlled) {
        setInternalValue(nextValue);
      }

      onValueChange?.(nextValue);
    },
    [isControlled, onValueChange],
  );

  return [currentValue, setValue] as const;
}

const FileUpload = React.forwardRef<HTMLDivElement, FileUploadProps>(({
  accept,
  allowPaste = false,
  className,
  clearable = false,
  defaultValue = [],
  description,
  directory = false,
  disabled = false,
  emptyStateDescription,
  emptyStateTitle,
  label,
  listType,
  maxFiles = Number.POSITIVE_INFINITY,
  maxSize = Number.POSITIVE_INFINITY,
  multiple = true,
  onClear,
  onDownloadItem,
  onPreviewItem,
  onReject,
  onRetryItem,
  onValueChange,
  rejectionTitle = 'Some files could not be added.',
  replaceOnMax,
  showRejections = true,
  value,
  variant = 'file',
  ...props
}, ref) => {
  const variantDefaults = getUploadVariantDefaults(variant);
  const resolvedAccept = accept ?? variantDefaults.accept;
  const resolvedAcceptText = normalizeAccept(resolvedAccept);
  const resolvedListType = listType ?? variantDefaults.listType;
  const resolvedLabel = label ?? variantDefaults.label;
  const resolvedEmptyStateTitle = emptyStateTitle ?? variantDefaults.emptyStateTitle;
  const resolvedEmptyStateDescription =
    emptyStateDescription ?? (resolvedAcceptText === normalizeAccept(variantDefaults.accept) ? variantDefaults.helperText : `Accepted: ${resolvedAcceptText}`);
  const resolvedReplaceOnMax = replaceOnMax ?? maxFiles === 1;
  const constraintText = formatConstraintText(maxFiles, maxSize);
  const inputId = React.useId();
  const [items, setItems] = useControllableFileList(value, defaultValue, onValueChange);
  const [isDragging, setIsDragging] = React.useState(false);
  const [lastRejections, setLastRejections] = React.useState<FileUploadRejection[]>([]);
  const inputRef = React.useRef<HTMLInputElement>(null);
  const { getPreviewUrl } = useUploadPreviews(items, variant);
  const selectionDisabled = disabled || (items.length >= maxFiles && !(resolvedReplaceOnMax && maxFiles === 1));
  const directoryAttributes = directory ? ({ directory: '', webkitdirectory: '' } as Record<string, string>) : {};

  const appendFiles = React.useCallback(
    (nextFiles: FileList | File[] | null) => {
      if (!nextFiles || (Array.isArray(nextFiles) && !nextFiles.length) || ('length' in nextFiles && !nextFiles.length)) {
        return;
      }

      const files = Array.isArray(nextFiles) ? nextFiles : Array.from(nextFiles);
      const effectiveReplaceSingle = resolvedReplaceOnMax && maxFiles === 1;
      const nextItems = effectiveReplaceSingle ? [] : [...items];
      const rejections: FileUploadRejection[] = [];
      const incomingFiles = files.slice(0, multiple ? undefined : 1);

      incomingFiles.forEach((file) => {
        if (nextItems.length >= maxFiles) {
          rejections.push({
            file,
            message: `Maximum ${maxFiles} file${maxFiles === 1 ? '' : 's'} allowed.`,
            reason: 'max-files',
          });
          return;
        }

        if (file.size > maxSize) {
          rejections.push({
            file,
            message: `${file.name} exceeds the ${formatFileSize(maxSize)} limit.`,
            reason: 'file-too-large',
          });
          return;
        }

        if (!matchesAccept(file, resolvedAccept)) {
          rejections.push({
            file,
            message: `${file.name} is not an accepted file type.`,
            reason: 'invalid-type',
          });
          return;
        }

        nextItems.push({
          file,
          id: createFileId(),
          kind: inferUploadItemKind(file, variant),
          name: file.name,
          size: file.size,
          status: 'idle',
          type: file.type,
        });
      });

      setItems(nextItems);
      setLastRejections(rejections);

      if (rejections.length) {
        onReject?.(rejections);
      }

      if (inputRef.current) {
        inputRef.current.value = '';
      }
    },
    [items, maxFiles, maxSize, multiple, onReject, resolvedAccept, resolvedReplaceOnMax, setItems, variant],
  );

  const removeItem = React.useCallback(
    (itemId: string) => {
      setItems(items.filter((item) => item.id !== itemId));
    },
    [items, setItems],
  );

  const clearItems = React.useCallback(() => {
    setItems([]);
    setLastRejections([]);
    onClear?.();

    if (inputRef.current) {
      inputRef.current.value = '';
    }
  }, [onClear, setItems]);

  return (
    <div ref={ref} className={cn('space-y-3', className)} data-sdk-ui="file-upload" {...props}>
      <div className="flex items-start justify-between gap-3">
        <div className="space-y-1">
          <Label htmlFor={inputId}>{resolvedLabel}</Label>
          {description ? <p className="text-sm text-[var(--sdk-color-text-secondary)]">{description}</p> : null}
        </div>
        {clearable && items.length ? (
          <Button onClick={clearItems} size="sm" type="button" variant="ghost">
            Clear all files
          </Button>
        ) : null}
      </div>

      <input
        {...directoryAttributes}
        accept={resolvedAcceptText === '*' ? undefined : resolvedAcceptText}
        aria-label={resolvedLabel}
        className="sr-only"
        disabled={selectionDisabled}
        id={inputId}
        multiple={multiple}
        onChange={(event) => appendFiles(event.target.files)}
        ref={inputRef}
        type="file"
      />

      <UploadDropzone
        caption={
          items.length >= maxFiles && Number.isFinite(maxFiles)
            ? `Maximum ${maxFiles} file${maxFiles === 1 ? '' : 's'} selected. Remove one to add another.`
            : constraintText
        }
        description={resolvedEmptyStateDescription}
        disabled={selectionDisabled}
        isDragging={isDragging}
        onBrowse={() => inputRef.current?.click()}
        onDragLeave={() => setIsDragging(false)}
        onDragOver={(event) => {
          event.preventDefault();
          if (!selectionDisabled) {
            setIsDragging(true);
          }
        }}
        onDrop={(event) => {
          event.preventDefault();
          setIsDragging(false);

          if (!selectionDisabled) {
            appendFiles(event.dataTransfer.files);
          }
        }}
        onPaste={(event) => {
          if (!allowPaste || selectionDisabled) {
            return;
          }

          const clipboardFiles = getClipboardFiles(event.clipboardData);

          if (!clipboardFiles.length) {
            return;
          }

          event.preventDefault();
          appendFiles(clipboardFiles);
        }}
        title={
          items.length >= maxFiles && Number.isFinite(maxFiles)
            ? resolvedReplaceOnMax && maxFiles === 1
              ? 'Select a new file to replace the current one'
              : `Maximum ${maxFiles} file${maxFiles === 1 ? '' : 's'} selected`
            : resolvedEmptyStateTitle
        }
        variant={variant}
      />

      {showRejections && lastRejections.length ? (
        <InlineAlert
          description={
            <ul className="space-y-1">
              {lastRejections.map((rejection) => (
                <li key={`${rejection.file.name}-${rejection.reason}`}>{rejection.message}</li>
              ))}
            </ul>
          }
          title={rejectionTitle}
          tone="warning"
        />
      ) : null}

      {items.length ? (
        <div className={cn(resolvedListType === 'grid' ? 'grid gap-3 sm:grid-cols-2 xl:grid-cols-3' : 'space-y-2')} data-sdk-region="file-upload-list">
          {items.map((item) => (
            <UploadItem
              disabled={disabled}
              fallbackVariant={variant}
              item={item}
              key={item.id}
              listType={resolvedListType}
              onDownload={onDownloadItem}
              onRemove={removeItem}
              onPreview={onPreviewItem}
              onRetry={onRetryItem}
              previewUrl={getPreviewUrl(item)}
            />
          ))}
        </div>
      ) : null}
    </div>
  );
});

export { FileUpload };
FileUpload.displayName = 'FileUpload';
