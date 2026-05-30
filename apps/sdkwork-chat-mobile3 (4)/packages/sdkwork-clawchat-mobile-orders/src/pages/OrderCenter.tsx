import React, { useState, useRef, useEffect } from "react";
import { useNavigate } from "react-router";
import { ChevronLeft, Search, MoreHorizontal, Package, ScanLine } from "lucide-react";
import { IconButton, cn, showToast } from "@sdkwork/clawchat-mobile-commons";
import { motion, AnimatePresence } from "motion/react";
import { OrderService, type Order } from "../services/OrderService";
import { OrderReviewModal } from "../components/OrderReviewModal";
import { OrderCard } from "../components/OrderCard";
import { VoucherRedeemModal } from "../components/VoucherRedeemModal";

export const OrderCenter: React.FC = () => {
  const navigate = useNavigate();
  const [activeTab, setActiveTab] = useState("all");
  const [tabs, setTabs] = useState<{ id: string; label: string }[]>([]);
  const [orders, setOrders] = useState<Order[]>([]);
  const [isLoading, setIsLoading] = useState(true);
  const [reviewOrder, setReviewOrder] = useState<Order | null>(null);
  const [showRedeem, setShowRedeem] = useState(false);
  const scrollContainerRef = useRef<HTMLDivElement>(null);

  const fetchOrders = async () => {
    setIsLoading(true);
    const data = await OrderService.getOrders();
    setOrders(data);
    setIsLoading(false);
  };

  React.useEffect(() => {
    OrderService.getOrderTabs().then(setTabs);
    fetchOrders();
  }, []);

  const handleAction = async (
    e: React.MouseEvent,
    action: () => Promise<void>,
    successMsg: string,
  ) => {
    e.stopPropagation();
    try {
      await action();
      showToast(successMsg);
      fetchOrders();
    } catch (err) {
      showToast("操作失败");
    }
  };

  const filteredOrders =
    activeTab === "all" ? orders : orders.filter((o) => o.status === activeTab);

  const handleTabClick = (
    tabId: string,
    event: React.MouseEvent<HTMLDivElement>,
  ) => {
    setActiveTab(tabId);
    const container = scrollContainerRef.current;
    const element = event.currentTarget;
    if (container && element) {
      const containerWidth = container.offsetWidth;
      const elementOffset = element.offsetLeft;
      const elementWidth = element.offsetWidth;
      const scrollPos = elementOffset - containerWidth / 2 + elementWidth / 2;
      container.scrollTo({ left: scrollPos, behavior: "smooth" });
    }
  };

  const renderActionButtons = (order: Order) => {
    switch (order.status) {
      case "pending_payment":
        return (
          <>
            <button
              onClick={(e) =>
                handleAction(
                  e,
                  () => OrderService.modifyAddress(order.id),
                  "地址修改成功",
                )
              }
              className="px-4 py-1.5 rounded-full border border-border-color text-[13px] text-text-main font-medium active:bg-active-bg transition-colors"
            >
              修改地址
            </button>
            <button
              onClick={(e) =>
                handleAction(
                  e,
                  () => OrderService.cancelOrder(order.id),
                  "订单已取消",
                )
              }
              className="px-4 py-1.5 rounded-full border border-border-color text-[13px] text-text-main font-medium active:bg-active-bg transition-colors"
            >
              取消订单
            </button>
            <button
              onClick={(e) =>
                handleAction(
                  e,
                  () => OrderService.payOrder(order.id),
                  "支付成功",
                )
              }
              className="px-4 py-1.5 rounded-full border border-primary-blue bg-primary-blue text-white text-[13px] font-medium active:opacity-80 transition-opacity"
            >
              付款
            </button>
          </>
        );
      case "to_ship":
        return (
          <>
            <button
              onClick={(e) =>
                handleAction(
                  e,
                  () => OrderService.modifyAddress(order.id),
                  "地址修改成功",
                )
              }
              className="px-4 py-1.5 rounded-full border border-border-color text-[13px] text-text-main font-medium active:bg-active-bg transition-colors"
            >
              修改地址
            </button>
            <button
              onClick={(e) =>
                handleAction(
                  e,
                  () => OrderService.remindShipping(order.id),
                  "已提醒卖家发货",
                )
              }
              className="px-4 py-1.5 rounded-full border border-border-color text-[13px] text-text-main font-medium active:bg-active-bg transition-colors"
            >
              催发货
            </button>
          </>
        );
      case "to_receive":
        return (
          <>
            <button
              onClick={(e) => {
                e.stopPropagation();
                showToast("目前物流状态：运送中");
              }}
              className="px-4 py-1.5 rounded-full border border-border-color text-[13px] text-text-main font-medium active:bg-active-bg transition-colors"
            >
              查看物流
            </button>
            <button
              onClick={(e) =>
                handleAction(
                  e,
                  () => OrderService.confirmReceipt(order.id),
                  "已确认收货",
                )
              }
              className="px-4 py-1.5 rounded-full border border-primary-blue text-primary-blue text-[13px] font-medium active:bg-primary-blue/10 transition-colors"
            >
              确认收货
            </button>
          </>
        );
      case "to_review":
        return (
          <>
            <button
              onClick={(e) =>
                handleAction(
                  e,
                  () => OrderService.applyRefund(order.id),
                  "已提交售后申请",
                )
              }
              className="px-4 py-1.5 rounded-full border border-border-color text-[13px] text-text-main font-medium active:bg-active-bg transition-colors"
            >
              申请售后
            </button>
            <button
              onClick={(e) => {
                e.stopPropagation();
                setReviewOrder(order);
              }}
              className="px-4 py-1.5 rounded-full border border-primary-blue text-primary-blue text-[13px] font-medium active:bg-primary-blue/10 transition-colors"
            >
              评价
            </button>
          </>
        );
      case "cancelled":
      case "completed":
      case "refunded":
        return (
          <button
            onClick={(e) =>
              handleAction(
                e,
                () => OrderService.deleteOrder(order.id),
                "订单已删除",
              )
            }
            className="px-4 py-1.5 rounded-full border border-border-color text-[13px] text-text-main font-medium active:bg-active-bg transition-colors"
          >
            删除订单
          </button>
        );
      default:
        return null;
    }
  };

  return (
    <div className="flex flex-col h-full bg-bg-color">
      {/* Header */}
      <header className="bg-bg-color sticky top-0 z-10 shrink-0 pt-safe">
        <div className="h-[44px] px-1 flex items-center justify-between relative">
          <div className="flex items-center z-10 flex-1">
            <IconButton
              icon={
                <ChevronLeft
                  className="w-6 h-6 text-text-main"
                  strokeWidth={2.5}
                />
              }
              onClick={() => navigate(-1)}
            />
          </div>
          <div className="absolute left-1/2 -translate-x-1/2 flex items-center justify-center pointer-events-none">
            <h2 className="text-[17px] font-semibold text-text-main">
              我的订单
            </h2>
          </div>
          <div className="flex items-center justify-end z-10 flex-1 pr-1">
            <IconButton 
              icon={<ScanLine className="w-5 h-5 text-text-main" />} 
              onClick={() => setShowRedeem(true)}
            />
            <IconButton icon={<Search className="w-5 h-5 text-text-main" />} />
            <IconButton
              icon={<MoreHorizontal className="w-5 h-5 text-text-main" />}
            />
          </div>
        </div>

        {/* Tabs */}
        <div className="h-[44px] flex items-center relative border-b border-border-color/50">
          <div
            ref={scrollContainerRef}
            className="flex-1 overflow-x-auto no-scrollbar flex items-center h-full px-4 scroll-smooth"
          >
            <div className="flex gap-8 h-full items-center min-w-max">
              {tabs.map((tab) => (
                <div
                  key={tab.id}
                  onClick={(e) => handleTabClick(tab.id, e)}
                  className="relative h-full flex items-center cursor-pointer whitespace-nowrap"
                >
                  <span
                    className={cn(
                      "text-[14px] transition-colors",
                      activeTab === tab.id
                        ? "font-semibold text-text-main"
                        : "font-medium text-text-sub",
                    )}
                  >
                    {tab.label}
                  </span>
                  {activeTab === tab.id && (
                    <div className="absolute left-0 right-0 bottom-0 flex justify-center">
                      <motion.div
                        layoutId="orderTabIndicator"
                        className="w-6 h-[3px] bg-primary-blue rounded-t-full"
                      />
                    </div>
                  )}
                </div>
              ))}
            </div>
          </div>
        </div>
      </header>

      {/* Order List */}
      <div className="flex-1 overflow-y-auto bg-[#F2F2F2] dark:bg-[#121212]">
        <div className="p-3 flex flex-col gap-3 pb-12">
          <AnimatePresence mode="popLayout">
            {isLoading ? (
              <motion.div
                initial={{ opacity: 0 }}
                animate={{ opacity: 1 }}
                exit={{ opacity: 0 }}
                className="flex flex-col items-center justify-center py-20 text-text-sub opacity-70"
              >
                <div className="w-8 h-8 rounded-full border-4 border-text-sub border-t-transparent animate-spin mb-3"></div>
                <p className="text-[14px]">加载中...</p>
              </motion.div>
            ) : filteredOrders.length > 0 ? (
              filteredOrders.map((order) => (
                <OrderCard
                  key={order.id}
                  order={order}
                  onClick={() => navigate(`/me/orders/${order.id}`)}
                  renderActionButtons={renderActionButtons}
                />
              ))
            ) : (
              <motion.div
                initial={{ opacity: 0 }}
                animate={{ opacity: 1 }}
                className="flex flex-col items-center justify-center py-20 text-text-sub opacity-70"
              >
                <Package
                  className="w-12 h-12 mb-3 opacity-40 stroke-current"
                  strokeWidth={2}
                />
                <p className="text-[14px]">暂无订单数据</p>
              </motion.div>
            )}
          </AnimatePresence>
        </div>
      </div>

      <OrderReviewModal
        order={reviewOrder}
        onClose={() => setReviewOrder(null)}
        onSubmit={async (rating, reviewText) => {
          if (!reviewOrder) return;
          await OrderService.reviewOrder(reviewOrder.id);
          showToast("评价提交成功");
          fetchOrders();
        }}
      />
      
      <VoucherRedeemModal
        isOpen={showRedeem}
        onClose={() => {
          setShowRedeem(false);
          fetchOrders();
        }}
      />
    </div>
  );
};
