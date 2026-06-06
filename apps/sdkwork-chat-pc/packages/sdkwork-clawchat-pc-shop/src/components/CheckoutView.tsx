import React, { useState } from "react";
import { motion, AnimatePresence } from "motion/react";
import {
  ArrowLeft,
  MapPin,
  ChevronRight,
  Smartphone,
} from "lucide-react";
import { CashierView } from "./CashierView";
import { useTranslation } from "react-i18next";
import { CheckoutAddressModal } from "./CheckoutAddressModal";
import { CheckoutProductList } from "./CheckoutProductList";
import { CheckoutOrderSummary } from "./CheckoutOrderSummary";

const MOCK_ADDRESSES = [
  {
    id: "1",
    name: "张三",
    phone: "138 0000 0000",
    address: "北京市朝阳区星爪科技园 1 号楼 101室",
    isDefault: true,
  },
  {
    id: "2",
    name: "李四",
    phone: "139 1111 2222",
    address: "上海市浦东新区陆家嘴银城中路 100号",
    isDefault: false,
  },
];

export const CheckoutView = ({
  products,
  selectedItems,
  totalPrice,
  onBack,
  onComplete,
}: any) => {
  const { t } = useTranslation(["checkout", "common"]);
  const [addresses, setAddresses] = useState(MOCK_ADDRESSES);
  const [selectedAddressId, setSelectedAddressId] = useState(
    MOCK_ADDRESSES[0].id,
  );
  const [showAddressModal, setShowAddressModal] = useState(false);
  const [showCashier, setShowCashier] = useState(false);

  const [addressForm, setAddressForm] = useState<any>(null);

  const addressObj =
    addresses.find((a) => a.id === selectedAddressId) || addresses[0];

  const handleSaveAddress = (e: React.FormEvent) => {
    e.preventDefault();
    if (addressForm.id) {
      setAddresses((prev) =>
        prev.map((a) => (a.id === addressForm.id ? { ...addressForm, isDefault: addressForm.isDefault ? true : false } : addressForm.isDefault ? { ...a, isDefault: false } : a))
      );
    } else {
      const newAddr = { ...addressForm, id: Date.now().toString() };
      setAddresses((prev) =>
        addressForm.isDefault ? prev.map((a) => ({ ...a, isDefault: false })).concat(newAddr) : [...prev, newAddr]
      );
      setSelectedAddressId(newAddr.id);
    }
    setAddressForm(null);
  };

  const handleDeleteAddress = (e: React.MouseEvent, id: string) => {
    e.stopPropagation();
    setAddresses((prev) => prev.filter((a) => a.id !== id));
    if (selectedAddressId === id) {
      setSelectedAddressId(addresses.find(a => a.id !== id)?.id || "");
    }
  };

  const handleSetDefault = (e: React.MouseEvent, id: string) => {
    e.stopPropagation();
    setAddresses((prev) => prev.map((a) => ({ ...a, isDefault: a.id === id })));
  };

  const isAllVirtual =
    selectedItems.length > 0 &&
    selectedItems.every((item: any) => {
      const p = products.find((prod: any) => prod.id === item.productId);
      return p?.isVirtual;
    });

  return (
    <motion.div
      initial={{ opacity: 0, x: 20 }}
      animate={{ opacity: 1, x: 0 }}
      exit={{ opacity: 0, x: -20 }}
      className="absolute inset-0 bg-[#1e1e20] z-20 flex flex-col"
    >
      <AnimatePresence>
        {showAddressModal && (
          <CheckoutAddressModal
            showAddressModal={showAddressModal}
            setShowAddressModal={setShowAddressModal}
            addressForm={addressForm}
            setAddressForm={setAddressForm}
            handleSaveAddress={handleSaveAddress}
            addresses={addresses}
            selectedAddressId={selectedAddressId}
            setSelectedAddressId={setSelectedAddressId}
            handleSetDefault={handleSetDefault}
            handleDeleteAddress={handleDeleteAddress}
          />
        )}
      </AnimatePresence>

      <AnimatePresence>
        {showCashier && (
          <CashierView
            amount={totalPrice}
            onCancel={() => setShowCashier(false)}
            onComplete={(method: string) => onComplete(method)}
          />
        )}
      </AnimatePresence>

      <div className="h-16 border-b border-white/5 flex items-center px-8 shrink-0 bg-[#1e1e20] shadow-sm z-10 sticky top-0">
        <button
          onClick={onBack}
          className="flex items-center gap-2 hover:bg-white/5 px-3 py-1.5 rounded-full transition-colors text-gray-400 hover:text-white"
        >
          <ArrowLeft size={18} />
          <span className="text-sm font-medium">返回购物中心</span>
        </button>
      </div>

      <div className="flex-1 overflow-y-auto p-6 flex justify-center custom-scrollbar bg-[#1e1e20]">
        <div className="w-full max-w-6xl flex flex-col lg:flex-row gap-8">
          <div className="flex-1 space-y-6">
            <h2 className="text-xl font-bold text-gray-100 mb-6 hidden lg:block tracking-tight">
              确认订单信息
            </h2>
            {!isAllVirtual ? (
              <div
                onClick={() => setShowAddressModal(true)}
                className="bg-[#2b2b2d] rounded-3xl p-8 border border-white/5 relative overflow-hidden group cursor-pointer hover:border-pink-500/50 transition-all shadow-xl shadow-black/10 hover:shadow-pink-500/5"
              >
                <div className="absolute top-0 left-0 w-full h-1 bg-[url('data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSIzMCIgaGVpZ2h0PSIxMCI+PHBhdGggZD0iTTAgMGgzMGwxMCAxMEgxMHoiIGZpbGw9IiNmZmRhZGIiLz48cGF0aCBkPSJNMzAgMGgzdjEwaC0zeiIgZmlsbD0iI2ZmZmZmZiIvPjxwYXRoIGQ9Ik0zMyAwaDMwbDEwIDEwSDQSDQzeiIgZmlsbD0iIzllYzhmOCIvPjxwYXRoIGQ9Ik02MyAwaDN2MTBoLTN6IiBmaWxsPSIjZmZmZmZmIi8+PC9zdmc+')] bg-repeat-x opacity-60"></div>
                <div className="flex items-center gap-6 mt-2">
                  <div className="w-14 h-14 rounded-full bg-gradient-to-br from-pink-500/20 to-pink-500/5 flex items-center justify-center shrink-0 shadow-inner">
                    <MapPin size={24} className="text-pink-500" />
                  </div>
                  <div className="flex-1 min-w-0">
                    <div className="flex items-end gap-3 mb-2">
                      <span className="text-xl font-bold text-gray-100">
                        {addressObj?.name || t("checkout:selectAddress", { defaultValue: "选择收货地址" })}
                      </span>
                      {addressObj?.phone && (
                        <span className="text-base text-gray-400 font-mono">
                          {addressObj.phone}
                        </span>
                      )}
                    </div>
                    {addressObj?.address ? (
                      <p className="text-base text-gray-300 leading-relaxed truncate">
                        {addressObj.address}
                      </p>
                    ) : (
                      <p className="text-base text-gray-400 leading-relaxed truncate">
                        {t("checkout:addNewAddress", { defaultValue: "添加新地址" })}
                      </p>
                    )}
                  </div>
                  <ChevronRight
                    size={24}
                    className="text-gray-500 group-hover:text-pink-400 transform group-hover:translate-x-1 transition-all"
                  />
                </div>
              </div>
            ) : (
              <div className="bg-[#2b2b2d] rounded-3xl p-8 border border-white/5 relative overflow-hidden shadow-xl shadow-black/10">
                <div className="flex items-center gap-6">
                  <div className="w-14 h-14 rounded-full bg-gradient-to-br from-blue-500/20 to-blue-500/5 flex items-center justify-center shrink-0">
                    <Smartphone size={24} className="text-blue-500" />
                  </div>
                  <div className="flex-1 min-w-0">
                    <div className="flex items-center gap-3 mb-1">
                      <span className="text-xl font-bold text-gray-100">
                        {t("checkout:rechargeAccount")}
                      </span>
                    </div>
                    <p className="text-base text-gray-400 leading-relaxed truncate">
                      {t("checkout:currentAccount", {
                        account: "138 **** 0000",
                      })}
                    </p>
                  </div>
                </div>
              </div>
            )}

            <CheckoutProductList selectedItems={selectedItems} products={products} />
          </div>

          <CheckoutOrderSummary totalPrice={totalPrice} isAllVirtual={isAllVirtual} setShowCashier={setShowCashier} />
        </div>
      </div>
    </motion.div>
  );
};
