import React, { useState, useEffect } from "react";
import { motion, AnimatePresence } from "motion/react";
import {
  Store,
  Smartphone,
  Wallet,
  ArrowLeft,
  Check,
  Scan,
} from "lucide-react";
import { useTranslation } from "react-i18next";

export const CashierView = ({
  amount,
  onCancel,
  onComplete,
  orderId = "ORD-" + Math.floor(Math.random() * 10000000),
}: any) => {
  const { t, i18n } = useTranslation(["checkout", "common"]);
  const [paymentMethod, setPaymentMethod] = useState<"wechat" | "alipay">(
    "wechat",
  );
  const [step, setStep] = useState<"scan" | "processing" | "success">("scan");

  useEffect(() => {
    const timer = setTimeout(() => {
      setStep("processing");
      setTimeout(() => setStep("success"), 1500);
    }, 5000); // 模拟轮询获取支付状态的过程
    return () => clearTimeout(timer);
  }, []);

  useEffect(() => {
    if (step === "success") {
      const timer = setTimeout(() => onComplete(paymentMethod), 1500);
      return () => clearTimeout(timer);
    }
  }, [step, onComplete, paymentMethod]);

  return (
    <motion.div
      initial={{ opacity: 0, x: 20 }}
      animate={{ opacity: 1, x: 0 }}
      exit={{ opacity: 0, x: -20 }}
      className="absolute inset-0 bg-[#f5f5f5] dark:bg-[#1e1e20] z-30 flex flex-col"
    >
      {/* Header */}
      <div className="h-16 bg-white dark:bg-[#2b2b2d] border-b border-gray-200 dark:border-white/5 flex items-center px-10 shrink-0">
        <div className="flex items-center gap-4">
          <button
            onClick={onCancel}
            className="p-2 -ml-2 text-gray-500 hover:bg-gray-100 dark:text-gray-400 dark:hover:bg-white/10 dark:hover:text-white rounded-full transition-colors"
          >
            <ArrowLeft size={20} />
          </button>
          <div className="flex items-center gap-2">
            <Store size={24} className="text-pink-600" />
            <span className="text-xl font-medium text-gray-900 dark:text-gray-100">
              {t("checkout:cashierTitle")}
            </span>
          </div>
        </div>
      </div>

      {/* Content */}
      <div className="flex-1 overflow-y-auto p-8 flex justify-center custom-scrollbar">
        <div className="w-full max-w-5xl space-y-6">
          {/* Order Summary */}
          <div className="bg-white dark:bg-[#2b2b2d] rounded-xl p-8 shadow-sm border border-gray-100 dark:border-white/5 flex justify-between items-center">
            <div>
              <div className="text-gray-900 dark:text-gray-100 font-medium text-lg mb-3">
                {t("checkout:orderTitle")}
              </div>
              <div className="text-sm text-gray-500 flex gap-6">
                <span>{t("checkout:payee")}</span>
                <span>{t("checkout:orderNo", { orderId })}</span>
              </div>
            </div>
            <div className="text-right">
              <div className="text-sm text-gray-500 mb-1">
                {t("checkout:amountPayable")}
              </div>
              <div className="text-4xl font-bold text-[#ff5000] dark:text-pink-500 font-sans tracking-tight">
                <span className="text-2xl mr-1 font-medium">
                  {t("common:currencySymbol")}
                </span>
                {amount.toLocaleString(
                  i18n.language === "en-US" ? "en-US" : "zh-CN",
                  { minimumFractionDigits: 2 },
                )}
              </div>
            </div>
          </div>

          {/* Payment Methods & QR Code */}
          <div className="bg-white dark:bg-[#2b2b2d] rounded-xl shadow-sm border border-gray-100 dark:border-white/5 min-h-[450px] flex overflow-hidden">
            {/* Left: Methods */}
            <div className="w-56 bg-gray-50 dark:bg-black/20 border-r border-gray-100 dark:border-white/5 flex flex-col">
              <button
                onClick={() => setPaymentMethod("wechat")}
                className={`h-[72px] px-8 flex items-center gap-3 transition-colors border-l-[3px] \${paymentMethod === 'wechat' ? 'bg-white dark:bg-[#2b2b2d] border-[#09b83e] text-gray-900 dark:text-white font-medium shadow-[4px_0_12px_rgba(0,0,0,0.02)]' : 'border-transparent text-gray-600 dark:text-gray-400 hover:bg-gray-100 dark:hover:bg-white/5'}`}
              >
                <Smartphone
                  size={24}
                  className={paymentMethod === "wechat" ? "text-[#09b83e]" : ""}
                />
                <span className="text-base">{t("checkout:wechatPay")}</span>
              </button>
              <button
                onClick={() => setPaymentMethod("alipay")}
                className={`h-[72px] px-8 flex items-center gap-3 transition-colors border-l-[3px] \${paymentMethod === 'alipay' ? 'bg-white dark:bg-[#2b2b2d] border-[#1677ff] text-gray-900 dark:text-white font-medium shadow-[4px_0_12px_rgba(0,0,0,0.02)]' : 'border-transparent text-gray-600 dark:text-gray-400 hover:bg-gray-100 dark:hover:bg-white/5'}`}
              >
                <Wallet
                  size={24}
                  className={paymentMethod === "alipay" ? "text-[#1677ff]" : ""}
                />
                <span className="text-base">{t("checkout:alipay")}</span>
              </button>
            </div>

            {/* Right: Payment Area */}
            <div className="flex-1 p-12 flex items-center justify-center relative">
              <AnimatePresence mode="wait">
                {step === "scan" && (
                  <motion.div
                    key="scan"
                    initial={{ opacity: 0, scale: 0.95 }}
                    animate={{ opacity: 1, scale: 1 }}
                    exit={{ opacity: 0 }}
                    className="flex gap-20 items-center justify-center w-full"
                  >
                    <div className="flex flex-col items-center">
                      <div className="text-lg font-medium text-gray-900 dark:text-white mb-6">
                        {t("checkout:scanToPay", {
                          method:
                            paymentMethod === "wechat"
                              ? t("checkout:method_wechat")
                              : t("checkout:method_alipay"),
                        })}
                      </div>
                      <div
                        className={`p-4 border-[3px] rounded-2xl \${paymentMethod === 'wechat' ? 'border-[#09b83e]/20 dark:border-[#09b83e]/40' : 'border-[#1677ff]/20 dark:border-[#1677ff]/40'}`}
                      >
                        <div className="w-56 h-56 bg-white shrink-0 object-contain relative flex items-center justify-center rounded-xl overflow-hidden shadow-inner">
                          <div className="grid grid-cols-6 grid-rows-6 gap-0.5 w-full h-full opacity-80 mix-blend-multiply p-2">
                            {Array.from({ length: 36 }).map((_, i) => (
                              <div
                                key={i}
                                className="bg-black rounded-sm"
                                style={{ opacity: Math.random() > 0.4 ? 1 : 0 }}
                              />
                            ))}
                          </div>
                          <div
                            className={`absolute inset-0 block animate-scan pointer-events-none bg-gradient-to-t from-transparent \${paymentMethod === 'wechat' ? 'to-[#09b83e]/20' : 'to-[#1677ff]/20'}`}
                            style={{ height: "50%" }}
                          />
                          <div className="absolute w-12 h-12 bg-white rounded-lg flex items-center justify-center p-1.5 shadow-md">
                            {paymentMethod === "wechat" ? (
                              <Store size={30} className="text-[#09b83e]" />
                            ) : (
                              <Store size={30} className="text-[#1677ff]" />
                            )}
                          </div>
                        </div>
                      </div>
                      <div className="mt-8 flex items-center justify-center gap-2 py-2 px-4 rounded-full bg-gray-50 dark:bg-white/5 border border-gray-100 dark:border-white/5 text-sm text-gray-600 dark:text-gray-400">
                        <Scan size={16} /> {t("checkout:realNameNotice")}
                      </div>
                    </div>
                  </motion.div>
                )}

                {step === "processing" && (
                  <motion.div
                    key="processing"
                    initial={{ opacity: 0, y: 20 }}
                    animate={{ opacity: 1, y: 0 }}
                    exit={{ opacity: 0, y: -20 }}
                    className="flex flex-col items-center"
                  >
                    <div className="w-16 h-16 border-4 border-gray-200 dark:border-white/10 border-t-pink-500 rounded-full animate-spin mb-6" />
                    <div className="text-xl font-medium text-gray-900 dark:text-gray-200 font-sans tracking-wide">
                      {t("checkout:processing")}
                    </div>
                  </motion.div>
                )}

                {step === "success" && (
                  <motion.div
                    key="success"
                    initial={{ opacity: 0, scale: 0.8 }}
                    animate={{ opacity: 1, scale: 1 }}
                    className="flex flex-col items-center"
                  >
                    <div className="w-24 h-24 bg-[#09b83e] rounded-full flex items-center justify-center mb-6 shadow-xl shadow-[#09b83e]/30">
                      <Check size={48} className="text-white" strokeWidth={4} />
                    </div>
                    <div className="text-2xl font-bold text-gray-900 dark:text-white mb-3 tracking-wide">
                      {t("checkout:paySuccess")}
                    </div>
                    <div className="text-gray-500 dark:text-gray-400 font-medium">
                      {t("checkout:redirecting")}
                    </div>
                  </motion.div>
                )}
              </AnimatePresence>
            </div>
          </div>
        </div>
      </div>
    </motion.div>
  );
};
