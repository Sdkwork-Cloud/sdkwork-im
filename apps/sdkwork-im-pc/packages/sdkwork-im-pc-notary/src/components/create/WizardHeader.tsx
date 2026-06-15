/**
 * WizardHeader - Header bar for the create task wizard
 */
import React from 'react';
import { useTranslation } from 'react-i18next';
import { ChevronLeft } from 'lucide-react';

export interface WizardHeaderProps {
  /** Called when back button is clicked */
  onBack: () => void;
  /** Title displayed in center */
  title: string;
}

export const WizardHeader: React.FC<WizardHeaderProps> = ({ onBack, title }) => {
  const { t } = useTranslation('notary');

  return (
    <div className="flex items-center px-6 min-h-[64px] border-b border-white/5 bg-[#181818] shrink-0">
      <button onClick={onBack} className="flex items-center text-gray-400 hover:text-gray-100 transition-colors">
        <ChevronLeft size={20} className="mr-1" />
        <span>{t('createTask.back')}</span>
      </button>
      <div className="mx-auto text-lg font-medium text-gray-200">{title}</div>
      <div className="w-[60px]" /> {/* Spacer for centering */}
    </div>
  );
};