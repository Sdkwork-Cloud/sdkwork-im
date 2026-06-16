/**
 * PaneTabSwitcher - Tab switcher for the detail pane (Parties / Materials)
 */
import React from 'react';
import { motion } from 'motion/react';

export interface TabItem {
  id: string;
  label: string;
}

export interface PaneTabSwitcherProps {
  activeTab: string;
  tabs: TabItem[];
  onTabChange: (tabId: string) => void;
}

export const PaneTabSwitcher: React.FC<PaneTabSwitcherProps> = ({
  activeTab,
  tabs,
  onTabChange,
}) => {
  return (
    <div className="flex gap-6 border-b border-white/5 px-6">
      {tabs.map((tab) => (
        <button
          key={tab.id}
          onClick={() => onTabChange(tab.id)}
          className={`pb-3 text-sm font-medium transition-colors relative ${
            activeTab === tab.id ? 'text-indigo-400' : 'text-gray-400 hover:text-gray-200'
          }`}
        >
          {tab.label}
          {activeTab === tab.id && (
            <motion.div
              layoutId="activeTab"
              className="absolute bottom-0 left-0 right-0 h-0.5 bg-indigo-500 rounded-full"
            />
          )}
        </button>
      ))}
    </div>
  );
};