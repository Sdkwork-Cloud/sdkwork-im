import { FileUpload } from './file-upload';
import type { SpecializedUploadProps } from './upload-types';
import { getUploadVariantDefaults } from './upload-utils';

export type ImageUploadProps = SpecializedUploadProps;
export type VideoUploadProps = SpecializedUploadProps;
export type AudioUploadProps = SpecializedUploadProps;
export type DocumentUploadProps = SpecializedUploadProps;

function ImageUpload({ accept, listType, ...props }: ImageUploadProps) {
  const defaults = getUploadVariantDefaults('image');

  return <FileUpload accept={accept ?? defaults.accept} data-sdk-ui="image-upload" listType={listType ?? defaults.listType} variant="image" {...props} />;
}

function VideoUpload({ accept, listType, ...props }: VideoUploadProps) {
  const defaults = getUploadVariantDefaults('video');

  return <FileUpload accept={accept ?? defaults.accept} data-sdk-ui="video-upload" listType={listType ?? defaults.listType} variant="video" {...props} />;
}

function AudioUpload({ accept, listType, ...props }: AudioUploadProps) {
  const defaults = getUploadVariantDefaults('audio');

  return <FileUpload accept={accept ?? defaults.accept} data-sdk-ui="audio-upload" listType={listType ?? defaults.listType} variant="audio" {...props} />;
}

function DocumentUpload({ accept, listType, ...props }: DocumentUploadProps) {
  const defaults = getUploadVariantDefaults('document');

  return <FileUpload accept={accept ?? defaults.accept} data-sdk-ui="document-upload" listType={listType ?? defaults.listType} variant="document" {...props} />;
}

ImageUpload.displayName = 'ImageUpload';
VideoUpload.displayName = 'VideoUpload';
AudioUpload.displayName = 'AudioUpload';
DocumentUpload.displayName = 'DocumentUpload';

export { AudioUpload, DocumentUpload, ImageUpload, VideoUpload };
