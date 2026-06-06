import React, { useState, useEffect } from 'react';
import { motion, AnimatePresence } from 'motion/react';

let addToast: (message: string, type?: 'info' | 'success' | 'error') => void;

export const ToastContainer: React.FC = () => {
  const [toasts, setToasts] = useState<{ id: string; message: string; type: 'info' | 'success' | 'error' }[]>([]);

  useEffect(() => {
    addToast = (message, type = 'info') => {
      const id = Date.now().toString();
      setToasts(prev => [...prev, { id, message, type }]);
      setTimeout(() => {
        setToasts(prev => prev.filter(t => t.id !== id));
      }, 3000);
    };
  }, []);

  return (
    <div className="fixed top-10 left-1/2 -translate-x-1/2 z-[9999] flex flex-col gap-2 pointer-events-none">
      <AnimatePresence>
        {toasts.map(toast => (
          <motion.div
            key={toast.id}
            initial={{ opacity: 0, y: -20, scale: 0.9 }}
            animate={{ opacity: 1, y: 0, scale: 1 }}
            exit={{ opacity: 0, y: -20, scale: 0.9 }}
            className={`px-4 py-2 rounded-full shadow-lg text-sm font-medium text-white flex items-center gap-2 ${
              toast.type === 'success' ? 'bg-[#00b42a]' : 
              toast.type === 'error' ? 'bg-red-500' : 
              'bg-[#2b2b2d] border border-white/10'
            }`}
          >
            {toast.message}
          </motion.div>
        ))}
      </AnimatePresence>
    </div>
  );
};

export const toast = (message: string, type: 'info' | 'success' | 'error' = 'info') => {
  if (addToast) {
    addToast(message, type);
  } else {
    console.log(`[Toast] ${type}: ${message}`);
  }
};
