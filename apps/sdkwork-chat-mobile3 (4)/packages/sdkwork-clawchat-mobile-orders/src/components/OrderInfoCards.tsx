import React from "react";
import type { Order } from "../services/OrderService";

interface OrderInfoCardsProps {
  order: Order;
}

export const OrderInfoCards: React.FC<OrderInfoCardsProps> = ({ order }) => {
  return (
    <>
      <div className="bg-white dark:bg-[#1E1E1E] rounded-xl p-4 shadow-sm flex flex-col gap-3">
        <h3 className="text-[14px] font-bold text-text-main mb-1">订单信息</h3>
        <div className="flex items-center justify-between">
          <span className="text-[13px] text-text-sub">订单编号</span>
          <div className="flex items-center gap-2">
            <span className="text-[13px] text-text-main">{order.id}</span>
            <span className="text-[11px] text-primary-blue border border-primary-blue/30 px-1.5 py-0.5 rounded cursor-pointer active:bg-primary-blue/10">
              复制
            </span>
          </div>
        </div>
        <div className="flex items-center justify-between">
          <span className="text-[13px] text-text-sub">创建时间</span>
          <span className="text-[13px] text-text-main">{order.createTime}</span>
        </div>
        {order.payTime && (
          <div className="flex items-center justify-between">
            <span className="text-[13px] text-text-sub">付款时间</span>
            <span className="text-[13px] text-text-main">{order.payTime}</span>
          </div>
        )}
        {order.shipTime && (
          <div className="flex items-center justify-between">
            <span className="text-[13px] text-text-sub">发货时间</span>
            <span className="text-[13px] text-text-main">{order.shipTime}</span>
          </div>
        )}
      </div>

      <div className="bg-white dark:bg-[#1E1E1E] rounded-xl p-4 shadow-sm flex flex-col gap-3">
        <div className="flex items-center justify-between">
          <span className="text-[13px] text-text-sub">商品总价</span>
          <span className="text-[13px] text-text-main">
            ¥{order.totalAmount.toFixed(2)}
          </span>
        </div>
        <div className="flex items-center justify-between">
          <span className="text-[13px] text-text-sub">运费</span>
          <span className="text-[13px] text-text-main">
            ¥{order.shippingFee.toFixed(2)}
          </span>
        </div>
        <div className="pt-3 border-t border-border-color/50 flex items-center justify-between">
          <span className="text-[14px] font-bold text-text-main">实付款</span>
          <span className="text-[18px] font-bold text-[#FA5151]">
            ¥{(order.totalAmount + order.shippingFee).toFixed(2)}
          </span>
        </div>
      </div>
    </>
  );
};
