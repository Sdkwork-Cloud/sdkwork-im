import React, { useState } from "react";
import { useNavigate, useSearchParams } from "react-router";
import { ChevronLeft } from "lucide-react";
import { showToast } from "@sdkwork/clawchat-mobile-commons";
import { OrderService } from "@sdkwork/clawchat-mobile-orders";

export const CashierPage = () => {
  const navigate = useNavigate();
  const [searchParams] = useSearchParams();
  const amount = searchParams.get("amount") || "0.00";
  const orderId = searchParams.get("orderId");

  const [selectedPayment, setSelectedPayment] = useState("wechat");
  const [showPassword, setShowPassword] = useState(false);
  const [password, setPassword] = useState("");

  const handleConfirmPayment = () => {
    setShowPassword(true);
  };

  const handlePasswordSubmit = () => {
    if (password.length !== 6) {
      showToast("请输入6位支付密码");
      return;
    }
    setShowPassword(false);
    showToast("支付处理中...");
    setTimeout(async () => {
      if (orderId) {
        try {
          await OrderService.payOrder(orderId);
          const order = await OrderService.getOrderById(orderId);
          
          // Send automatic voucher to chat
          if (order && order.isVirtual) {
            const { ProductService } = await import("../services/ProductService");
            const { ChatService } = await import("@sdkwork/clawchat-mobile-chat");
            let targetShopChatId = "shop_1";
            let navigateToChatId = targetShopChatId;
            let groupChatCreated = false;

            for (const item of order.items) {
              if (item.virtualType === 'coupon') {
                const voucherCodeStr = item.voucherCodes ? item.voucherCodes.map(v => v.code).join(", ") : '请联系客服获取';
                ProductService.sendCustomMessage(targetShopChatId, {
                  id: `msg_${Date.now()}_${Math.random()}`,
                  content: `[系统发货] 您购买的【${item.title}】\n规格: ${item.specs}\n券码: ${voucherCodeStr}`,
                  senderId: targetShopChatId,
                  senderType: "agent",
                  timestamp: Date.now()
                });
              } else if (item.virtualType === 'group_chat') {
                const groupName = item.specs ? item.specs : item.title;
                const newChat = await ChatService.joinOrCreateGroupChat(groupName);
                if (newChat) {
                   navigateToChatId = newChat.id;
                   groupChatCreated = true;
                }
              }
            }

            if (groupChatCreated) {
              navigate(`/chat/${navigateToChatId}`, { replace: true });
              return;
            }
          }
        } catch (e) {
          console.error("Failed to mark order as paid", e);
        }
      }
      showToast("支付成功！");
      setTimeout(async () => {
        if (orderId) {
          const order = await OrderService.getOrderById(orderId);
          if (order && order.isVirtual) {
            navigate(`/shop-chat/shop_1`, { replace: true });
            return;
          }
        }
        navigate("/me/orders", { replace: true });
      }, 1000);
    }, 1500);
  };

  return (
    <div className="flex flex-col h-full bg-bg-color relative">
      <header className="flex items-center justify-between px-2 pt-safe h-[56px] border-b border-border-color bg-chat-other-bg shrink-0">
        <div
          className="w-10 h-10 flex items-center justify-center cursor-pointer"
          onClick={() => navigate(-1)}
        >
          <ChevronLeft className="w-6 h-6 text-text-main" />
        </div>
        <span className="text-[17px] font-medium text-text-main">收银台</span>
        <div className="w-10 h-10" />
      </header>

      <div className="flex-1 overflow-y-auto pb-safe">
        <div className="flex flex-col items-center py-10 bg-chat-other-bg mb-3 border-b border-border-color/50">
          <span className="text-[14px] text-text-main mb-2">需支付</span>
          <span className="text-[36px] font-bold text-text-main">
            <span className="text-[24px] mr-1">¥</span>
            {amount}
          </span>
        </div>

        <div className="px-4">
          <div className="text-[14px] text-text-sub mb-3 ml-1">
            请选择支付方式
          </div>

          <div className="bg-chat-other-bg rounded-xl overflow-hidden shadow-sm">
            {/* WeChat Pay */}
            <div
              className="flex items-center justify-between p-4 border-b border-border-color/50 active:bg-chat-active-bg transition-colors cursor-pointer"
              onClick={() => setSelectedPayment("wechat")}
            >
              <div className="flex items-center gap-3">
                <div className="w-8 h-8 rounded-full bg-[#07C160]/10 flex items-center justify-center">
                  <svg
                    className="w-5 h-5 text-[#07C160]"
                    viewBox="0 0 24 24"
                    fill="currentColor"
                  >
                    <path d="M8.5,14.5 C7.11928813,14.5 6,13.3807119 6,12 C6,10.6192881 7.11928813,9.5 8.5,9.5 C9.88071187,9.5 11,10.6192881 11,12 C11,13.3807119 9.88071187,14.5 8.5,14.5 Z M15.5,14.5 C14.1192881,14.5 13,13.3807119 13,12 C13,10.6192881 14.1192881,9.5 15.5,9.5 C16.8807119,9.5 18,10.6192881 18,12 C18,13.3807119 16.8807119,14.5 15.5,14.5 Z M12,2 C17.5228475,2 22,6.4771525 22,12 C22,17.5228475 17.5228475,22 12,22 C6.4771525,22 2,17.5228475 2,12 C2,6.4771525 6.4771525,2 12,2 Z" />
                  </svg>
                </div>
                <span className="text-[15px] text-text-main font-medium">
                  微信支付
                </span>
              </div>
              <div
                className={`w-5 h-5 rounded-full border flex items-center justify-center ${selectedPayment === "wechat" ? "bg-[#07C160] border-[#07C160]" : "border-border-color bg-bg-color"}`}
              >
                {selectedPayment === "wechat" && (
                  <svg
                    className="w-3 h-3 text-white"
                    fill="none"
                    viewBox="0 0 24 24"
                    stroke="currentColor"
                  >
                    <path
                      strokeLinecap="round"
                      strokeLinejoin="round"
                      strokeWidth={3}
                      d="M5 13l4 4L19 7"
                    />
                  </svg>
                )}
              </div>
            </div>

            {/* Alipay */}
            <div
              className="flex items-center justify-between p-4 border-b border-border-color/50 active:bg-chat-active-bg transition-colors cursor-pointer"
              onClick={() => setSelectedPayment("alipay")}
            >
              <div className="flex items-center gap-3">
                <div className="w-8 h-8 rounded-full bg-[#1677FF]/10 flex items-center justify-center">
                  <svg
                    className="w-5 h-5 text-[#1677FF]"
                    viewBox="0 0 24 24"
                    fill="currentColor"
                  >
                    <path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm0 18c-4.41 0-8-3.59-8-8s3.59-8 8-8 8 3.59 8 8-3.59 8-8 8zm-1-13h2v6h-2zm0 8h2v2h-2z" />
                  </svg>
                </div>
                <span className="text-[15px] text-text-main font-medium">
                  支付宝
                </span>
              </div>
              <div
                className={`w-5 h-5 rounded-full border flex items-center justify-center ${selectedPayment === "alipay" ? "bg-[#07C160] border-[#07C160]" : "border-border-color bg-bg-color"}`}
              >
                {selectedPayment === "alipay" && (
                  <svg
                    className="w-3 h-3 text-white"
                    fill="none"
                    viewBox="0 0 24 24"
                    stroke="currentColor"
                  >
                    <path
                      strokeLinecap="round"
                      strokeLinejoin="round"
                      strokeWidth={3}
                      d="M5 13l4 4L19 7"
                    />
                  </svg>
                )}
              </div>
            </div>

            {/* Balance */}
            <div
              className="flex items-center justify-between p-4 active:bg-chat-active-bg transition-colors cursor-pointer"
              onClick={() => setSelectedPayment("balance")}
            >
              <div className="flex items-center gap-3">
                <div className="w-8 h-8 rounded-full bg-[#FFAA00]/10 flex items-center justify-center">
                  <svg
                    className="w-5 h-5 text-[#FFAA00]"
                    viewBox="0 0 24 24"
                    fill="currentColor"
                  >
                    <path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm0 18c-4.41 0-8-3.59-8-8s3.59-8 8-8 8 3.59 8 8-3.59 8-8 8zm-1.5 5.5v2h-2v2h2v4h3v-2h-2.5v-2h2.5c1.1 0 2-.9 2-2s-.9-2-2-2h-3v-2h-1.5z" />
                  </svg>
                </div>
                <div className="flex flex-col">
                  <span className="text-[15px] text-text-main font-medium">
                    零钱
                  </span>
                  <span className="text-[12px] text-text-sub mt-0.5">
                    可用余额 ¥120.00
                  </span>
                </div>
              </div>
              <div
                className={`w-5 h-5 rounded-full border flex items-center justify-center ${selectedPayment === "balance" ? "bg-[#07C160] border-[#07C160]" : "border-border-color bg-bg-color"}`}
              >
                {selectedPayment === "balance" && (
                  <svg
                    className="w-3 h-3 text-white"
                    fill="none"
                    viewBox="0 0 24 24"
                    stroke="currentColor"
                  >
                    <path
                      strokeLinecap="round"
                      strokeLinejoin="round"
                      strokeWidth={3}
                      d="M5 13l4 4L19 7"
                    />
                  </svg>
                )}
              </div>
            </div>
          </div>

          <div className="mt-8">
            <button
              className="w-full py-[14px] rounded-xl text-[16px] font-medium bg-[#07C160] text-white active:scale-[0.98] transition-transform"
              onClick={handleConfirmPayment}
            >
              确认支付 ¥{amount}
            </button>
          </div>
        </div>
      </div>

      {showPassword && (
        <div className="fixed inset-0 bg-black/50 z-50 flex items-center justify-center">
          <div className="bg-chat-other-bg border border-border-color/60 text-text-main rounded-xl w-[300px] p-6 flex flex-col items-center">
            <h3 className="text-[18px] font-medium mb-4 text-text-main">
              请输入支付密码
            </h3>
            <p className="text-[28px] font-bold mb-6 text-text-main">
              ¥{amount}
            </p>
            <input
              type="password"
              maxLength={6}
              className="w-full h-12 border border-border-color bg-bg-color text-text-main rounded-lg text-center text-[24px] tracking-[1em] outline-none focus:border-[#07C160]"
              value={password}
              onChange={(e) => setPassword(e.target.value.replace(/\D/g, ""))}
              autoFocus
            />
            <div className="flex gap-4 w-full mt-6">
              <button
                className="flex-1 h-10 border border-border-color text-text-main rounded-lg active:bg-hover-bg"
                onClick={() => setShowPassword(false)}
              >
                取消
              </button>
              <button
                className="flex-1 h-10 bg-[#07C160] text-white rounded-lg active:opacity-90"
                onClick={handlePasswordSubmit}
              >
                确认
              </button>
            </div>
          </div>
        </div>
      )}

    </div>
  );
};
