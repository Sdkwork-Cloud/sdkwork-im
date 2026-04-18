import { cleanup, fireEvent, render, screen } from '@testing-library/react';
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest';
import { AudioUpload, DocumentUpload, FileUpload, ImageUpload, VideoUpload } from './index';

const originalCreateObjectURL = URL.createObjectURL;
const originalRevokeObjectURL = URL.revokeObjectURL;

beforeEach(() => {
  Object.defineProperty(URL, 'createObjectURL', {
    configurable: true,
    value: vi.fn((file: Blob) => `blob:${file instanceof File ? file.name : 'preview'}`),
  });
  Object.defineProperty(URL, 'revokeObjectURL', {
    configurable: true,
    value: vi.fn(),
  });
});

afterEach(() => {
  Object.defineProperty(URL, 'createObjectURL', {
    configurable: true,
    value: originalCreateObjectURL,
  });
  Object.defineProperty(URL, 'revokeObjectURL', {
    configurable: true,
    value: originalRevokeObjectURL,
  });
  cleanup();
});

describe('FileUpload', () => {
  it('adds selected files into the shared desktop upload list and allows removing them', () => {
    const handleValueChange = vi.fn();
    const file = new File(['desktop'], 'notes.txt', { type: 'text/plain' });

    render(<FileUpload label="Upload assets" onValueChange={handleValueChange} />);

    fireEvent.change(screen.getByLabelText('Upload assets'), {
      target: {
        files: [file],
      },
    });

    expect(screen.getByText('notes.txt')).toBeInTheDocument();
    expect(handleValueChange).toHaveBeenCalledWith(
      expect.arrayContaining([expect.objectContaining({ name: 'notes.txt', status: 'idle' })]),
    );

    fireEvent.click(screen.getByRole('button', { name: 'Remove notes.txt' }));

    expect(screen.queryByText('notes.txt')).not.toBeInTheDocument();
    expect(handleValueChange).toHaveBeenLastCalledWith([]);
  });

  it('renders image previews through the specialized image upload preset', () => {
    const file = new File(['image-data'], 'cover.png', { type: 'image/png' });

    render(<ImageUpload label="Upload images" />);

    fireEvent.change(screen.getByLabelText('Upload images'), {
      target: {
        files: [file],
      },
    });

    expect(screen.getByRole('img', { name: 'Preview cover.png' })).toHaveAttribute('src', 'blob:cover.png');
    expect(screen.getByText('cover.png')).toBeInTheDocument();
  });

  it('renders video previews through the specialized video upload preset', () => {
    const file = new File(['video-data'], 'intro.mp4', { type: 'video/mp4' });

    render(<VideoUpload label="Upload videos" />);

    fireEvent.change(screen.getByLabelText('Upload videos'), {
      target: {
        files: [file],
      },
    });

    expect(screen.getByLabelText('Preview intro.mp4')).toHaveAttribute('src', 'blob:intro.mp4');
    expect(screen.getByText('intro.mp4')).toBeInTheDocument();
  });

  it('renders audio previews through the specialized audio upload preset', () => {
    const file = new File(['audio-data'], 'voice.mp3', { type: 'audio/mpeg' });

    render(<AudioUpload label="Upload audio" />);

    fireEvent.change(screen.getByLabelText('Upload audio'), {
      target: {
        files: [file],
      },
    });

    expect(screen.getByLabelText('Preview voice.mp3')).toHaveAttribute('src', 'blob:voice.mp3');
    expect(screen.getByText('voice.mp3')).toBeInTheDocument();
  });

  it('rejects unsupported files through the document upload preset', () => {
    const handleReject = vi.fn();
    const file = new File(['image-data'], 'poster.png', { type: 'image/png' });

    render(<DocumentUpload label="Upload documents" onReject={handleReject} />);

    fireEvent.change(screen.getByLabelText('Upload documents'), {
      target: {
        files: [file],
      },
    });

    expect(handleReject).toHaveBeenCalledWith([
      expect.objectContaining({
        file,
        reason: 'invalid-type',
      }),
    ]);
    expect(screen.queryByText('poster.png')).not.toBeInTheDocument();
  });

  it('replaces the current file when the max file count is one', () => {
    const firstFile = new File(['first'], 'cover-a.png', { type: 'image/png' });
    const secondFile = new File(['second'], 'cover-b.png', { type: 'image/png' });

    render(<ImageUpload label="Single image" maxFiles={1} />);

    fireEvent.change(screen.getByLabelText('Single image'), {
      target: {
        files: [firstFile],
      },
    });

    expect(screen.getByText('cover-a.png')).toBeInTheDocument();

    fireEvent.change(screen.getByLabelText('Single image'), {
      target: {
        files: [secondFile],
      },
    });

    expect(screen.queryByText('cover-a.png')).not.toBeInTheDocument();
    expect(screen.getByText('cover-b.png')).toBeInTheDocument();
  });

  it('supports clearing all selected files from the upload list', () => {
    const firstFile = new File(['one'], 'brief.pdf', { type: 'application/pdf' });
    const secondFile = new File(['two'], 'notes.txt', { type: 'text/plain' });
    const handleValueChange = vi.fn();

    render(<DocumentUpload clearable label="Project files" multiple onValueChange={handleValueChange} />);

    fireEvent.change(screen.getByLabelText('Project files'), {
      target: {
        files: [firstFile, secondFile],
      },
    });

    expect(screen.getByText('brief.pdf')).toBeInTheDocument();
    expect(screen.getByText('notes.txt')).toBeInTheDocument();

    fireEvent.click(screen.getByRole('button', { name: 'Clear all files' }));

    expect(screen.queryByText('brief.pdf')).not.toBeInTheDocument();
    expect(screen.queryByText('notes.txt')).not.toBeInTheDocument();
    expect(handleValueChange).toHaveBeenLastCalledWith([]);
  });

  it('supports paste upload for clipboard images', () => {
    const file = new File(['clipboard-image'], 'pasted.png', { type: 'image/png' });

    render(<ImageUpload allowPaste label="Paste images" />);

    const trigger = screen.getByText('Click to upload or drag images here').closest('[role="button"]');

    expect(trigger).not.toBeNull();

    fireEvent.paste(trigger as HTMLElement, {
      clipboardData: {
        files: [file],
      },
    });

    expect(screen.getByText('pasted.png')).toBeInTheDocument();
  });

  it('supports directory selection attributes for batch uploads', () => {
    render(<DocumentUpload directory label="Upload folders" />);

    const input = screen.getByLabelText('Upload folders');

    expect(input).toHaveAttribute('webkitdirectory');
    expect(input).toHaveAttribute('directory');
  });

  it('renders inline rejection feedback when files are rejected', () => {
    const file = new File(['image-data'], 'poster.png', { type: 'image/png' });

    render(<DocumentUpload label="Reject notice" />);

    fireEvent.change(screen.getByLabelText('Reject notice'), {
      target: {
        files: [file],
      },
    });

    expect(screen.getByRole('alert')).toHaveTextContent('Some files could not be added.');
    expect(screen.getByRole('alert')).toHaveTextContent('poster.png is not an accepted file type.');
  });

  it('renders preview and download actions for uploaded files', () => {
    const handlePreview = vi.fn();
    const handleDownload = vi.fn();
    const item = {
      id: 'remote-file',
      kind: 'document' as const,
      name: 'brief.pdf',
      size: 2048,
      status: 'success' as const,
      type: 'application/pdf',
      url: 'https://cdn.sdkwork.local/brief.pdf',
    };

    render(
      <FileUpload
        label="Remote files"
        onDownloadItem={handleDownload}
        onPreviewItem={handlePreview}
        value={[item]}
      />,
    );

    fireEvent.click(screen.getByRole('button', { name: 'Preview brief.pdf' }));
    fireEvent.click(screen.getByRole('button', { name: 'Download brief.pdf' }));

    expect(handlePreview).toHaveBeenCalledWith(expect.objectContaining({ id: 'remote-file', name: 'brief.pdf' }));
    expect(handleDownload).toHaveBeenCalledWith(expect.objectContaining({ id: 'remote-file', name: 'brief.pdf' }));
  });

  it('renders retry actions for error-state items', () => {
    const handleRetry = vi.fn();
    const item = {
      error: 'Upload failed',
      id: 'failed-file',
      kind: 'document' as const,
      name: 'archive.zip',
      size: 4096,
      status: 'error' as const,
      type: 'application/zip',
    };

    render(<FileUpload label="Retry files" onRetryItem={handleRetry} value={[item]} />);

    fireEvent.click(screen.getByRole('button', { name: 'Retry archive.zip' }));

    expect(handleRetry).toHaveBeenCalledWith(expect.objectContaining({ id: 'failed-file', name: 'archive.zip' }));
  });
});
