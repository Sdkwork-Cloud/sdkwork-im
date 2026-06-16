/**
 * WizardStepper - 4-step progress indicator for the create task wizard
 */
import React from 'react';
import { useTranslation } from 'react-i18next';
import { Check } from 'lucide-react';
import { cn } from '@sdkwork/im-pc-commons';

export interface WizardStepperProps {
  /** Current step (1-4) */
  currentStep: number;
  /** Total number of steps */
  totalSteps: number;
  /** Labels for each step */
  stepLabels: string[];
}

export const WizardStepper: React.FC<WizardStepperProps> = ({
  currentStep,
  totalSteps,
  stepLabels,
}) => {
  return (
    <div className="flex items-center w-full mb-12">
      {Array.from({ length: totalSteps }).map((_, i) => {
        const stepNum = i + 1;
        return (
          <React.Fragment key={stepNum}>
            <div className="flex flex-col items-center relative z-10 w-24">
              <div
                className={cn(
                  'w-10 h-10 rounded-full flex items-center justify-center font-medium text-sm transition-colors border-2',
                  currentStep > stepNum
                    ? 'bg-indigo-600 border-indigo-600 text-white'
                    : currentStep === stepNum
                      ? 'bg-indigo-600/20 border-indigo-500 text-indigo-400 font-bold'
                      : 'bg-[#2b2b2d] border-white/10 text-gray-500',
                )}
              >
                {currentStep > stepNum ? <Check size={18} /> : stepNum}
              </div>
              <div
                className={cn(
                  'mt-3 text-sm font-medium whitespace-nowrap',
                  currentStep >= stepNum ? 'text-gray-200' : 'text-gray-500',
                )}
              >
                {stepLabels[i]}
              </div>
            </div>
            {stepNum < totalSteps && (
              <div className="flex-1 h-[2px] mx-2 bg-[#2b2b2d] relative -top-[12px]">
                <div
                  className="absolute left-0 top-0 bottom-0 bg-indigo-600 transition-all duration-300"
                  style={{ width: currentStep > stepNum ? '100%' : '0%' }}
                />
              </div>
            )}
          </React.Fragment>
        );
      })}
    </div>
  );
};