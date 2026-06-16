/**
 * PrintPartyPage - Single party page in the print overlay
 */
import React from 'react';
import { useTranslation } from 'react-i18next';
import type { Party } from '@sdkwork/im-pc-types';
import { EMPTY_NOTARY_PRINT_IMAGE_URL } from '../../constants';
import type { PartyIdentityMediaUrls } from '../../types';

export interface PrintPartyPageProps {
  /** The party to render */
  party: Party;
  /** Media URLs for this party's identity photos */
  mediaUrls?: PartyIdentityMediaUrls;
}

export const PrintPartyPage: React.FC<PrintPartyPageProps> = ({
  party,
  mediaUrls,
}) => {
  const { t } = useTranslation('notary');

  return (
    <div className="w-[794px] min-h-[1123px] shrink-0 bg-white text-[#333] shadow-[0_10px_40px_rgba(0,0,0,0.5)] relative font-sans break-after-page print:w-full print:min-h-0 print:shadow-none print:m-0 print:p-[40px] p-[80px_100px]">
      {/* Title */}
      <h1 className="text-[28px] font-bold text-center mb-16 tracking-widest text-[#333]">
        {t('print.faceComparisonVerification')}
      </h1>

      {/* Party details + ID photo */}
      <div className="flex gap-6 mb-8 text-[15px] leading-10">
        <div className="flex-1 flex flex-col pt-2">
          {/* Name / Gender row */}
          <div className="flex border-b border-gray-100 pb-2 mb-2 items-center">
            <div className="w-24 font-bold text-gray-600">{t('party.name')}</div>
            <div className="flex-1 text-center font-medium text-black">{party.name}</div>
            <div className="w-20 font-bold text-gray-600">{t('party.gender')}</div>
            <div className="w-20 font-medium text-black">{party.gender || t('party.male')}</div>
          </div>

          {/* Birth date / Ethnicity row */}
          <div className="flex border-b border-gray-100 pb-2 mb-2 items-center">
            <div className="w-24 font-bold text-gray-600">{t('print.birthDate')}</div>
            <div className="flex-1 text-center font-medium text-black">
              {party.identityId
                ? new Date(
                    parseInt(party.identityId.substring(6, 10)),
                    parseInt(party.identityId.substring(10, 12)) - 1,
                    parseInt(party.identityId.substring(12, 14)),
                  ).toLocaleDateString()
                : '—'}
            </div>
            <div className="w-20 font-bold text-gray-600">{t('print.ethnicity')}</div>
            <div className="w-20 font-medium text-black">{party.ethnicity || '—'}</div>
          </div>

          {/* Address row */}
          <div className="flex border-b border-gray-100 pb-2 mb-2 items-center">
            <div className="w-24 font-bold text-gray-600">{t('print.address')}</div>
            <div className="flex-1 text-center font-medium text-black leading-relaxed" style={{ textAlignLast: 'center' }}>
              {party.address || '—'}
            </div>
          </div>

          {/* ID number row */}
          <div className="flex border-b border-gray-100 pb-2 mb-2 items-center">
            <div className="w-24 font-bold text-gray-600">{t('print.idNumber')}</div>
            <div className="flex-1 text-center font-mono font-medium text-black tracking-wider">{party.identityId}</div>
          </div>

          {/* Issuing authority + valid period row */}
          <div className="flex border-b border-gray-100 pb-2 items-center">
            <div className="w-24 font-bold text-gray-600">{t('print.issuingAuthority')}</div>
            <div className="flex-1 text-center text-[13px] font-medium text-black overflow-hidden whitespace-nowrap text-ellipsis">
              {party.identityIssuingAuthority || '—'}
            </div>
            <div className="w-24 font-bold text-right pr-4 text-gray-600">{t('print.validPeriod')}</div>
            <div className="w-36 text-sm font-medium text-black text-right pr-2">
              {party.identityValidDateStart && party.identityValidDateEnd
                ? `${party.identityValidDateStart}-${party.identityValidDateEnd}`
                : '—'}
            </div>
          </div>
        </div>

        {/* ID photo */}
        <div className="w-[124px] h-[164px] border border-gray-300 bg-gray-50 flex-shrink-0 p-1">
          <img
            src={mediaUrls?.identityFrontUrl ?? EMPTY_NOTARY_PRINT_IMAGE_URL}
            className="w-full h-full object-cover grayscale-[0.2]"
            alt={t('print.idPhoto')}
          />
        </div>
      </div>

      {/* Live capture photo */}
      <div className="flex text-[15px] mb-12">
        <div className="w-24 font-bold text-gray-600 pt-[140px] leading-7 text-left">{t('detail.liveCapture')}</div>
        <div className="pl-[50px] flex-1">
          <img
            src={mediaUrls?.faceImageUrl ?? EMPTY_NOTARY_PRINT_IMAGE_URL}
            className="w-[280px] h-[360px] object-cover border border-gray-300 shadow-sm"
            alt={t('detail.liveCapture')}
          />
        </div>
      </div>

      {/* Comparison score + result */}
      <div className="flex text-[15px] mb-8 items-center">
        <div className="w-24 font-bold text-gray-600">{t('print.comparisonScore')}</div>
        <div className="w-[180px] text-gray-800 font-mono text-lg">{party.identityVerificationScore ?? '—'}</div>
        <div className="w-24 font-bold text-right pr-6 text-gray-600">{t('print.comparisonResult')}</div>
        <div className="flex-1 font-medium text-black text-left pl-6">
          {party.identityVerificationStatus === 'verified' ? t('print.success') : '—'}
        </div>
      </div>

      {/* Comparison method + reference threshold */}
      <div className="flex text-[15px] mb-16 items-center">
        <div className="w-24 font-bold text-gray-600">{t('print.comparisonMethod')}</div>
        <div className="w-[180px] font-medium text-black">{t('print.faceComparison')}</div>
        <div className="w-24 font-bold text-right pr-6 text-gray-600">{t('print.referenceThreshold')}</div>
        <div className="flex-1 text-gray-800 font-mono text-left pl-6">68</div>
      </div>

      {/* Signature */}
      <div className="flex text-[15px] items-start pb-12">
        <div className="w-[120px] text-center font-bold text-gray-600 pt-8">{t('print.partySignature')}</div>
        <div className="pt-4 flex-1 text-center pr-20">
          <div
            className="font-[cursive] text-[64px] opacity-90 px-4 text-black italic -rotate-2 inline-block"
            style={{ fontFamily: "'Dancing Script', 'Caveat', cursive" }}
          >
            {party.name}
          </div>
        </div>
      </div>
    </div>
  );
};