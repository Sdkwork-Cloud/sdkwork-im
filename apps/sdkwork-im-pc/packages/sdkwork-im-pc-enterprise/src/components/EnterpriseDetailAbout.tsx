import React from 'react';
import { useTranslation } from 'react-i18next';
import { Building2, MapPin, Globe, Phone, Mail, Calendar, Award, ShieldCheck, Users, Image as ImageIcon } from 'lucide-react';
import { EnterpriseData } from './EnterpriseDetail';

interface EnterpriseDetailAboutProps {
  enterprise: EnterpriseData;
}

export const EnterpriseDetailAbout: React.FC<EnterpriseDetailAboutProps> = ({ enterprise }) => {
  const { t } = useTranslation('enterprise');

  return (
    <div className="flex flex-col gap-8">
      <section className="bg-white dark:bg-[#18181b] rounded-2xl p-8 border border-gray-200 dark:border-white/5 shadow-sm animate-in fade-in slide-in-from-bottom-4 duration-300">
        <h3 className="text-xl font-bold text-gray-900 dark:text-white mb-6 flex items-center gap-2">
          <Building2 className="text-indigo-500" size={22} />
          {t('detail.about')}
        </h3>
        <p className="text-gray-700 dark:text-gray-300 leading-[1.8] text-[15px] whitespace-pre-wrap">
          {enterprise.description}
        </p>
        <div className="mt-8 grid grid-cols-2 md:grid-cols-4 gap-4 border-t border-gray-100 dark:border-white/5 pt-8">
          <div className="flex flex-col gap-1">
            <span className="text-sm text-gray-500 dark:text-gray-400 flex items-center gap-1"><Calendar size={14} /> {t('detail.establishedDate')}</span>
            <span className="font-bold text-gray-900 dark:text-gray-100">2015-08-12</span>
          </div>
          <div className="flex flex-col gap-1">
            <span className="text-sm text-gray-500 dark:text-gray-400 flex items-center gap-1"><Award size={14} /> {t('detail.registeredCapital')}</span>
            <span className="font-bold text-gray-900 dark:text-gray-100">5000万人民币</span>
          </div>
          <div className="flex flex-col gap-1">
            <span className="text-sm text-gray-500 dark:text-gray-400 flex items-center gap-1"><Users size={14} /> {t('detail.legalRepresentative')}</span>
            <span className="font-bold text-gray-900 dark:text-gray-100">张三</span>
          </div>
          <div className="flex flex-col gap-1">
            <span className="text-sm text-gray-500 dark:text-gray-400 flex items-center gap-1"><ShieldCheck size={14} /> {t('detail.operatingStatus')}</span>
            <span className="font-bold text-green-600 dark:text-green-400">开业</span>
          </div>
        </div>
      </section>

      <section className="bg-white dark:bg-[#18181b] rounded-2xl p-8 border border-gray-200 dark:border-white/5 shadow-sm animate-in fade-in slide-in-from-bottom-4 duration-300 delay-75">
        <h3 className="text-xl font-bold text-gray-900 dark:text-white mb-6 flex items-center gap-2">
          <ImageIcon className="text-indigo-500" size={22} />
          {t('detail.album')}
        </h3>
        <div className="grid grid-cols-2 md:grid-cols-3 gap-4">
          <div className="aspect-[4/3] rounded-xl overflow-hidden bg-gray-100 dark:bg-white/5 relative group cursor-pointer">
            <img src="https://images.unsplash.com/photo-1497366216548-37526070297c?q=80&w=800&auto=format&fit=crop" alt="办公环境" className="w-full h-full object-cover transition-transform duration-500 group-hover:scale-110" />
            <div className="absolute inset-0 bg-black/40 opacity-0 group-hover:opacity-100 transition-opacity flex items-center justify-center">
              <span className="text-white font-medium">{t('detail.officeEnv', '办公环境')}</span>
            </div>
          </div>
          <div className="aspect-[4/3] rounded-xl overflow-hidden bg-gray-100 dark:bg-white/5 relative group cursor-pointer">
            <img src="https://images.unsplash.com/photo-1573164713988-8665fc963095?q=80&w=800&auto=format&fit=crop" alt="研发中心" className="w-full h-full object-cover transition-transform duration-500 group-hover:scale-110" />
            <div className="absolute inset-0 bg-black/40 opacity-0 group-hover:opacity-100 transition-opacity flex items-center justify-center">
              <span className="text-white font-medium">{t('detail.rdCenter', '研发中心')}</span>
            </div>
          </div>
          <div className="aspect-[4/3] rounded-xl overflow-hidden bg-gray-100 dark:bg-white/5 relative group cursor-pointer">
            <img src="https://images.unsplash.com/photo-1522071820081-009f0129c71c?q=80&w=800&auto=format&fit=crop" alt="团队风采" className="w-full h-full object-cover transition-transform duration-500 group-hover:scale-110" />
            <div className="absolute inset-0 bg-black/40 opacity-0 group-hover:opacity-100 transition-opacity flex items-center justify-center">
              <span className="text-white font-medium">{t('detail.teamStyle', '团队风采')}</span>
            </div>
          </div>
        </div>
      </section>

      <section className="bg-white dark:bg-[#18181b] rounded-2xl p-8 border border-gray-200 dark:border-white/5 shadow-sm animate-in fade-in slide-in-from-bottom-4 duration-300 delay-150">
        <h3 className="text-xl font-bold text-gray-900 dark:text-white mb-6 flex items-center gap-2">
          <MapPin className="text-indigo-500" size={22} />
          {t('detail.contactInfo')}
        </h3>
        <div className="grid grid-cols-1 md:grid-cols-2 gap-8">
          <div className="flex items-start gap-4">
            <div className="w-12 h-12 rounded-xl bg-gray-50 dark:bg-white/5 border border-gray-100 dark:border-white/5 flex items-center justify-center shrink-0">
              <Globe className="text-gray-500 dark:text-gray-400" size={20} />
            </div>
            <div className="flex flex-col justify-center h-12">
              <div className="text-sm font-medium text-gray-500 dark:text-gray-400 mb-0.5">{t('detail.website')}</div>
              <a href={`http://${enterprise.website}`} target="_blank" rel="noopener noreferrer" className="text-[15px] font-bold text-gray-900 dark:text-gray-100 hover:text-indigo-600 dark:hover:text-indigo-400 transition-colors">
                {enterprise.website}
              </a>
            </div>
          </div>
          <div className="flex items-start gap-4">
            <div className="w-12 h-12 rounded-xl bg-gray-50 dark:bg-white/5 border border-gray-100 dark:border-white/5 flex items-center justify-center shrink-0">
              <Phone className="text-gray-500 dark:text-gray-400" size={20} />
            </div>
            <div className="flex flex-col justify-center h-12">
              <div className="text-sm font-medium text-gray-500 dark:text-gray-400 mb-0.5">{t('detail.phone')}</div>
              <div className="text-[15px] font-bold text-gray-900 dark:text-gray-100">400-123-4567</div>
            </div>
          </div>
          <div className="flex items-start gap-4">
            <div className="w-12 h-12 rounded-xl bg-gray-50 dark:bg-white/5 border border-gray-100 dark:border-white/5 flex items-center justify-center shrink-0">
              <Mail className="text-gray-500 dark:text-gray-400" size={20} />
            </div>
            <div className="flex flex-col justify-center h-12">
              <div className="text-sm font-medium text-gray-500 dark:text-gray-400 mb-0.5">{t('detail.email')}</div>
              <div className="text-[15px] font-bold text-gray-900 dark:text-gray-100">contact@{enterprise.website.replace('www.', '')}</div>
            </div>
          </div>
          <div className="flex items-start gap-4">
            <div className="w-12 h-12 rounded-xl bg-gray-50 dark:bg-white/5 border border-gray-100 dark:border-white/5 flex items-center justify-center shrink-0">
              <MapPin className="text-gray-500 dark:text-gray-400" size={20} />
            </div>
            <div className="flex flex-col justify-center h-12">
              <div className="text-sm font-medium text-gray-500 dark:text-gray-400 mb-0.5">{t('detail.address')}</div>
              <div className="text-[15px] font-bold text-gray-900 dark:text-gray-100">{enterprise.location}科技园区 A座 801</div>
            </div>
          </div>
        </div>
      </section>
    </div>
  );
};
