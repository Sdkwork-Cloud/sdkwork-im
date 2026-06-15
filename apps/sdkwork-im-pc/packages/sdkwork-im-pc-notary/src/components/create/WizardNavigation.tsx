/**
 * WizardNavigation - Prev/Next/Submit buttons for the wizard
 */
import React from 'react';
import { useTranslation } from 'react-i18next';
import { Loader2 } from 'lucide-react';

export interface WizardNavigationProps {
  /** Current step (1-based) */
  currentStep: number;
  /** Total number of steps */
  totalSteps: number;
  /** Whether submission is in progress */
  isSubmitting: boolean;
  /** Called when Previous button is clicked */
  onPrev: () => void;
  /** Called when Next button is clicked */
  onNext: () => void;
  /** Called when Submit button is clicked (final step) */
  onSubmit: () => void;
}

export const WizardNavigation: React.FC<WizardNavigationProps> = ({
  currentStep,
  totalSteps,
  isSubmitting,
  onPrev,
  onNext,
  onSubmit,
}) => {
  const { t } = useTranslation('notary');

  return (
    <div className="flex items-center justify-between mt-12 pt-6 border-t border-white/5">
      {/* Previous */}
      <button
        onClick={onPrev}
        disabled={currentStep === 1}
        className="px-6 py-2.5 rounded-lg text-sm text-gray-300 font-medium hover:bg-white/10 disabled:opacity-30 disabled:hover:bg-transparent transition-colors"
      >
        {t('createTask.prevStep')}
      </button>

      {/* Next or Submit */}
      {currentStep < totalSteps ? (
        <button
          onClick={onNext}
          className="px-8 py-2.5 rounded-lg text-sm text-white font-medium bg-indigo-600 hover:bg-indigo-700 transition-colors shadow-md"
        >
          {t('createTask.nextStep')}
        </button>
      ) : (
        <button
          onClick={onSubmit}
          disabled={isSubmitting}
          className="px-8 py-2.5 rounded-lg text-sm text-white font-medium bg-indigo-600 hover:bg-indigo-700 disabled:opacity-50 transition-colors shadow-md flex items-center gap-2"
        >
          {isSubmitting && <Loader2 size={16} className="animate-spin" />}
          {t('createTask.submit')}
        </button>
      )}
    </div>
  );
};