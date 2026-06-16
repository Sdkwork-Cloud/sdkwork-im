/**
 * useCameraCapture - Custom hook for camera-based face capture
 *
 * Encapsulates all camera lifecycle management logic including:
 * - Opening/closing camera stream
 * - Capturing frames from video
 * - Cleanup on unmount
 */
import { useCallback, useEffect, useRef, useState } from 'react';

export interface UseCameraCaptureResult {
  /** Whether the camera overlay is currently open */
  isCameraOpen: boolean;
  /** Captured face image as data URL (JPEG) */
  faceImage: string | null;
  /** Ref for the video element displaying the stream */
  videoRef: React.RefObject<HTMLVideoElement | null>;
  /** Ref for the off-screen canvas used for frame capture */
  canvasRef: React.RefObject<HTMLCanvasElement | null>;
  /** Open camera and start stream */
  startCamera: () => Promise<void>;
  /** Stop camera stream and close overlay */
  stopCamera: () => void;
  /** Capture current video frame to canvas and export as JPEG */
  captureFace: () => void;
  /** Clear the captured face image */
  clearFaceImage: () => void;
}

export function useCameraCapture(): UseCameraCaptureResult {
  const [isCameraOpen, setIsCameraOpen] = useState(false);
  const [faceImage, setFaceImage] = useState<string | null>(null);

  const videoRef = useRef<HTMLVideoElement | null>(null);
  const canvasRef = useRef<HTMLCanvasElement | null>(null);
  const streamRef = useRef<MediaStream | null>(null);

  const startCamera = useCallback(async () => {
    setIsCameraOpen(true);
    setFaceImage(null);
    try {
      const stream = await navigator.mediaDevices.getUserMedia({ video: true });
      streamRef.current = stream;
      if (videoRef.current) {
        videoRef.current.srcObject = stream;
      }
    } catch {
      setIsCameraOpen(false);
      throw new Error('Camera access denied');
    }
  }, []);

  const stopCamera = useCallback(() => {
    if (streamRef.current) {
      streamRef.current.getTracks().forEach((track) => track.stop());
      streamRef.current = null;
    }
    setIsCameraOpen(false);
  }, []);

  const captureFace = useCallback(() => {
    if (videoRef.current && canvasRef.current) {
      const context = canvasRef.current.getContext('2d');
      if (context) {
        canvasRef.current.width = videoRef.current.videoWidth;
        canvasRef.current.height = videoRef.current.videoHeight;
        context.drawImage(videoRef.current, 0, 0);
        const capturedFaceImage = canvasRef.current.toDataURL('image/jpeg');
        setFaceImage(capturedFaceImage);
        stopCamera();
      }
    }
  }, [stopCamera]);

  const clearFaceImage = useCallback(() => {
    setFaceImage(null);
  }, []);

  // Cleanup: stop camera stream on unmount
  useEffect(() => {
    return () => {
      if (streamRef.current) {
        streamRef.current.getTracks().forEach((track) => track.stop());
      }
    };
  }, []);

  return {
    isCameraOpen,
    faceImage,
    videoRef,
    canvasRef,
    startCamera,
    stopCamera,
    captureFace,
    clearFaceImage,
  };
}