/**
 * DrawerShell - Reusable animated slide-in drawer wrapper
 *
 * Provides the common overlay backdrop + sliding panel pattern
 * used in DetailPane, NotaryPickerDrawer, etc.
 */
import React from 'react';
import { motion } from 'motion/react';

export interface DrawerShellProps {
  /** Whether the drawer is open */
  isOpen: boolean;
  /** Called when the backdrop or close trigger is clicked */
  onClose: () => void;
  /** Drawer width (default: 480px) */
  width?: number;
  /** Content rendered inside the drawer */
  children: React.ReactNode;
}

export const DrawerShell: React.FC<DrawerShellProps> = ({
  isOpen,
  onClose,
  width = 480,
  children,
}) => {
  if (!isOpen) return null;

  return (
    <>
      {/* Backdrop */}
      <div
        className="fixed inset-0 bg-black/60 backdrop-blur-sm z-[100]"
        onClick={onClose}
      />

      {/* Drawer panel */}
      <motion.div
        className="fixed top-0 right-0 h-full bg-white shadow-2xl z-[101] flex flex-col"
        style={{ width }}
        initial={{ x: width }}
        animate={{ x: 0 }}
        exit={{ x: width }}
        transition={{ type: 'spring', damping: 25, stiffness: 300 }}
      >
        {children}
      </motion.div>
    </>
  );
};