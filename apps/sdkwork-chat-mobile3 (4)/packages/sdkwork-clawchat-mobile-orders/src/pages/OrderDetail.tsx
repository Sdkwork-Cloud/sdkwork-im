import React from "react";
import { useNavigate, useParams } from "react-router";
import { ChevronLeft, MoreHorizontal, Clock, ShieldCheck } from "lucide-react";
import { IconButton, cn, showToast } from "@sdkwork/clawchat-mobile-commons";
import { OrderService, type Order } from "../services/OrderService";
import { OrderAddressCard } from "../components/OrderAddressCard";
import { OrderItemsCard } from "../components/OrderItemsCard";
import { OrderInfoCards } from "../components/OrderInfoCards";
import { OrderReviewModal } from "../components/OrderReviewModal";

export const OrderDetail: React.FC = () => {
  const navigate = useNavigate();
  const { id } = useParams<{ id: string }>();
  const [order, setOrder] = React.useState<Order | null>(null);
  const [loading, setLoading] = React.useState(true);
  const [showReviewModal, setShowReviewModal] = React.useState(false);
  const [rating, setRating] = React.useState(0);
  const [reviewText, setReviewText] = React.useState("");

  const fetchOrder = async () => {
    if (!id) return;
    const data = await OrderService.getOrderById(id);
    setOrder(data || null);
    setLoading(false);
  };

  React.useEffect(() => {
    fetchOrder();
  }, [id]);

  const handleAction = async (
    action: () => Promise<void>,
    successMsg: string,
    goBack = false,
  ) => {
    try {
      await action();
      showToast(successMsg);
      if (goBack) {
        setTimeout(() => navigate(-1), 1000);
      } else {
        fetchOrder();
      }
    } catch (err) {
      showToast("操作失败");
    }
  };

  if (loading) {
    return (
      <div className="flex flex-col h-full bg-bg-color items-center justify-center text-text-sub opacity-70">
        <div className="w-8 h-8 rounded-full border-4 border-text-sub border-t-white animate-spin mb-3"></div>
        <span className="text-[14px]">加载中...</span>
      </div>
    );
  }

  if (!order) {
    return (
      <div className="flex flex-col h-full bg-bg-color items-center justify-center">
        <p className="text-text-sub">订单不存在</p>
        <button
          onClick={() => navigate(-1)}
          className="mt-4 px-4 py-2 bg-primary-blue text-white rounded-full"
        >
          返回
        </button>
      </div>
    );
  }

  const getStatusColor = (status: string) => {
    switch (status) {
      case "pending_payment":
        return "bg-orange-500";
      case "to_ship":
        return "bg-blue-500";
      case "to_receive":
        return "bg-blue-500";
      case "to_review":
        return "bg-green-500";
      case "completed":
        return "bg-green-500";
      case "cancelled":
        return "bg-gray-500";
      default:
        return "bg-primary-blue";
    }
  };

  const renderActionButtons = (currentOrder: Order) => {
    switch (currentOrder.status) {
      case "pending_payment":
        return (
          <>
            <button
              onClick={() =>
                handleAction(
                  () => OrderService.modifyAddress(currentOrder.id),
                  "地址修改成功",
                )
              }
              className="px-5 py-2 rounded-full border border-border-color text-[14px] text-text-main font-medium active:bg-active-bg transition-colors"
            >
              修改地址
            </button>
            <button
              onClick={() =>
                handleAction(
                  () => OrderService.cancelOrder(currentOrder.id),
                  "订单已取消",
                )
              }
              className="px-5 py-2 rounded-full border border-border-color text-[14px] text-text-main font-medium active:bg-active-bg transition-colors"
            >
              取消订单
            </button>
            <button
              onClick={() =>
                handleAction(
                  () => OrderService.payOrder(currentOrder.id),
                  "支付成功",
                )
              }
              className="px-5 py-2 rounded-full border border-primary-blue bg-primary-blue text-white text-[14px] font-medium active:opacity-80 transition-opacity"
            >
              立即付款
            </button>
          </>
        );
      case "to_ship":
        return (
          <>
            <button
              onClick={() =>
                handleAction(
                  () => OrderService.modifyAddress(currentOrder.id),
                  "地址修改成功",
                )
              }
              className="px-5 py-2 rounded-full border border-border-color text-[14px] text-text-main font-medium active:bg-active-bg transition-colors"
            >
              修改地址
            </button>
            <button
              onClick={() =>
                handleAction(
                  () => OrderService.remindShipping(currentOrder.id),
                  "已提醒卖家发货",
                )
              }
              className="px-5 py-2 rounded-full border border-border-color text-[14px] text-text-main font-medium active:bg-active-bg transition-colors"
            >
              催发货
            </button>
          </>
        );
      case "to_receive":
        return (
          <>
            <button
              onClick={() => showToast("目前物流状态：运送中")}
              className="px-5 py-2 rounded-full border border-border-color text-[14px] text-text-main font-medium active:bg-active-bg transition-colors"
            >
              查看物流
            </button>
            <button
              onClick={() =>
                handleAction(
                  () => OrderService.confirmReceipt(currentOrder.id),
                  "已确认收货",
                )
              }
              className="px-5 py-2 rounded-full border border-primary-blue text-primary-blue text-[14px] font-medium active:bg-primary-blue/10 transition-colors"
            >
              确认收货
            </button>
          </>
        );
      case "to_review":
        return (
          <>
            <button
              onClick={() =>
                handleAction(
                  () => OrderService.applyRefund(currentOrder.id),
                  "已提交售后申请",
                )
              }
              className="px-5 py-2 rounded-full border border-border-color text-[14px] text-text-main font-medium active:bg-active-bg transition-colors"
            >
              申请售后
            </button>
            <button
              onClick={() => setShowReviewModal(true)}
              className="px-5 py-2 rounded-full border border-primary-blue text-primary-blue text-[14px] font-medium active:bg-primary-blue/10 transition-colors"
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
            onClick={() =>
              handleAction(
                () => OrderService.deleteOrder(currentOrder.id),
                "订单已删除",
                true,
              )
            }
            className="px-5 py-2 rounded-full border border-border-color text-[14px] text-text-main font-medium active:bg-active-bg transition-colors"
          >
            删除订单
          </button>
        );
      default:
        return null;
    }
  };

  return (
    <div className="flex flex-col h-full bg-[#F2F2F2] dark:bg-[#121212]">
      {/* Header */}
      <header
        className={cn(
          "sticky top-0 z-10 shrink-0 pt-safe transition-colors",
          getStatusColor(order.status),
        )}
      >
        <div className="h-[44px] px-1 flex items-center justify-between relative">
          <div className="flex items-center z-10 flex-1">
            <IconButton
              icon={
                <ChevronLeft className="w-6 h-6 text-white" strokeWidth={2.5} />
              }
              onClick={() => navigate(-1)}
            />
          </div>
          <div className="absolute left-1/2 -translate-x-1/2 flex items-center justify-center pointer-events-none">
            <h2 className="text-[17px] font-semibold text-white">订单详情</h2>
          </div>
          <div className="flex items-center justify-end z-10 flex-1 pr-1">
            <IconButton
              icon={<MoreHorizontal className="w-5 h-5 text-white" />}
            />
          </div>
        </div>
      </header>

      <div className="flex-1 overflow-y-auto pb-[80px]">
        {/* Status Section */}
        <div
          className={cn(
            "px-6 py-8 text-white flex flex-col gap-2",
            getStatusColor(order.status),
          )}
        >
          <div className="flex items-center gap-2">
            {order.status === "pending_payment" && (
              <Clock className="w-6 h-6" />
            )}
            {order.status === "to_review" && (
              <ShieldCheck className="w-6 h-6" />
            )}
            <h1 className="text-[22px] font-bold">{order.statusText}</h1>
          </div>
          {order.status === "pending_payment" && (
            <p className="text-[13px] opacity-90">
              需付款: ¥{order.totalAmount.toFixed(2)}，剩余 23小时59分 自动关闭
            </p>
          )}
        </div>

        <div className="px-3 -mt-4 relative z-10 flex flex-col gap-3">
          {order.address && <OrderAddressCard address={order.address} />}
          <OrderItemsCard order={order} />
          <OrderInfoCards order={order} />
        </div>
      </div>

      {/* Bottom Action Bar */}
      <div className="fixed bottom-0 left-0 right-0 bg-white dark:bg-[#1E1E1E] border-t border-border-color/50 pb-safe z-20">
        <div className="h-[60px] px-4 flex items-center justify-end gap-2.5">
          {renderActionButtons(order)}
        </div>
      </div>

      <OrderReviewModal
        order={showReviewModal ? order : null}
        onClose={() => setShowReviewModal(false)}
        onSubmit={async (r, text) => {
          await handleAction(
            () => OrderService.reviewOrder(order.id),
            "评价提交成功",
          );
        }}
      />
    </div>
  );
};
