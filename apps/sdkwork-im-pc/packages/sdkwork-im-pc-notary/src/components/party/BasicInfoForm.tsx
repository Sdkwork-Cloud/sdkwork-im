/**
 * BasicInfoForm - Form fields for party basic information
 *
 * Contains phone, name, ID number, gender, birth date, valid dates, and address.
 */
import React from 'react';
import { useTranslation } from 'react-i18next';
import { User as UserIcon } from 'lucide-react';
import type { PartyFormState } from '../../types';

export interface BasicInfoFormProps {
  /** Form state */
  formState: PartyFormState;
  /** Whether fields are read-only */
  readOnly: boolean;
  /** Called when form state changes */
  onChange: (state: PartyFormState) => void;
}

export const BasicInfoForm: React.FC<BasicInfoFormProps> = ({
  formState,
  readOnly,
  onChange,
}) => {
  const { t } = useTranslation('notary');

  const updateField = (field: keyof PartyFormState, value: string) => {
    onChange({ ...formState, [field]: value });
  };

  return (
    <div className="flex flex-col gap-4">
      {/* Section header */}
      <h4 className="text-sm font-semibold text-gray-300 flex items-center gap-2">
        <UserIcon size={16} className="text-indigo-400" /> {t('party.basicInfo')}
      </h4>

      {/* Form fields grid */}
      <div className="grid grid-cols-2 gap-x-4 gap-y-4">
        {/* Phone (full width) */}
        <div className="flex flex-col gap-2 col-span-2">
          <label className="text-xs font-medium text-gray-400">
            {t('party.phone')} <span className="text-red-500">*</span>
          </label>
          <input
            type="text"
            readOnly={readOnly}
            value={formState.phone}
            onChange={(e) => updateField('phone', e.target.value)}
            placeholder={t('party.phonePlaceholder')}
            className="w-full bg-[#181818] border border-white/10 rounded-lg px-3 py-2 text-sm text-gray-200 outline-none focus:border-indigo-500 font-mono read-only:bg-white/5 read-only:border-transparent read-only:text-gray-400"
          />
        </div>

        {/* Name */}
        <div className="flex flex-col gap-2">
          <label className="text-xs font-medium text-gray-400">
            {t('party.name')} <span className="text-red-500">*</span>
          </label>
          <input
            type="text"
            readOnly={readOnly}
            value={formState.name}
            onChange={(e) => updateField('name', e.target.value)}
            placeholder={t('party.namePlaceholder')}
            className="w-full bg-[#181818] border border-white/10 rounded-lg px-3 py-2 text-sm text-gray-200 outline-none focus:border-indigo-500 read-only:bg-white/5 read-only:border-transparent read-only:text-gray-400"
          />
        </div>

        {/* ID Number */}
        <div className="flex flex-col gap-2">
          <label className="text-xs font-medium text-gray-400">
            {t('party.idNumber')} <span className="text-red-500">*</span>
          </label>
          <input
            type="text"
            readOnly={readOnly}
            value={formState.identityId}
            onChange={(e) => updateField('identityId', e.target.value)}
            placeholder={t('party.namePlaceholder')}
            className="w-full bg-[#181818] border border-white/10 rounded-lg px-3 py-2 text-sm text-gray-200 outline-none focus:border-indigo-500 font-mono read-only:bg-white/5 read-only:border-transparent read-only:text-gray-400"
          />
        </div>

        {/* Gender */}
        <div className="flex flex-col gap-2">
          <label className="text-xs font-medium text-gray-400">{t('party.gender')}</label>
          <select
            disabled={readOnly}
            value={formState.gender}
            onChange={(e) => updateField('gender', e.target.value)}
            className="w-full bg-[#181818] border border-white/10 rounded-lg px-3 py-2 text-sm text-gray-200 outline-none focus:border-indigo-500 disabled:opacity-75 disabled:bg-white/5"
          >
            <option value={t('party.male')}>{t('party.male')}</option>
            <option value={t('party.female')}>{t('party.female')}</option>
          </select>
        </div>

        {/* Birth Date */}
        <div className="flex flex-col gap-2">
          <label className="text-xs font-medium text-gray-400">{t('party.birthDate')}</label>
          <input
            type="date"
            readOnly={readOnly}
            value={formState.birthDate}
            onChange={(e) => updateField('birthDate', e.target.value)}
            className="w-full bg-[#181818] border border-white/10 rounded-lg px-3 py-2 text-sm text-gray-200 outline-none focus:border-indigo-500 [color-scheme:dark] read-only:bg-white/5 read-only:border-transparent read-only:text-gray-400"
          />
        </div>

        {/* ID Valid Start (full width) */}
        <div className="flex flex-col gap-2 col-span-2">
          <label className="text-xs font-medium text-gray-400">{t('party.idValidStart')}</label>
          <input
            type="date"
            readOnly={readOnly}
            value={formState.identityValidDateStart}
            onChange={(e) => updateField('identityValidDateStart', e.target.value)}
            className="w-full bg-[#181818] border border-white/10 rounded-lg px-3 py-2 text-sm text-gray-200 outline-none focus:border-indigo-500 [color-scheme:dark] read-only:bg-white/5 read-only:border-transparent read-only:text-gray-400"
          />
        </div>

        {/* ID Valid End (full width) */}
        <div className="flex flex-col gap-2 col-span-2">
          <label className="text-xs font-medium text-gray-400">{t('party.idValidEnd')}</label>
          <input
            type="date"
            readOnly={readOnly}
            value={formState.identityValidDateEnd}
            onChange={(e) => updateField('identityValidDateEnd', e.target.value)}
            className="w-full bg-[#181818] border border-white/10 rounded-lg px-3 py-2 text-sm text-gray-200 outline-none focus:border-indigo-500 [color-scheme:dark] read-only:bg-white/5 read-only:border-transparent read-only:text-gray-400"
          />
        </div>

        {/* Address (full width) */}
        <div className="flex flex-col gap-2 col-span-2">
          <label className="text-xs font-medium text-gray-400">{t('party.address')}</label>
          <input
            type="text"
            readOnly={readOnly}
            value={formState.address}
            onChange={(e) => updateField('address', e.target.value)}
            placeholder={t('party.addressPlaceholder')}
            className="w-full bg-[#181818] border border-white/10 rounded-lg px-3 py-2 text-sm text-gray-200 outline-none focus:border-indigo-500 read-only:bg-white/5 read-only:border-transparent read-only:text-gray-400"
          />
        </div>
      </div>
    </div>
  );
};