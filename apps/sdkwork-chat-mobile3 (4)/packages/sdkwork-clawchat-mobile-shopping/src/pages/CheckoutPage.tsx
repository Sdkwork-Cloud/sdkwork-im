import React, { useState, useEffect } from "react";
import { useNavigate, useSearchParams } from "react-router";
import { ChevronLeft, MapPin, ChevronRight, Store } from "lucide-react";
import { ProductService } from "../services/ProductService";
import { useCartStore } from "../store/useCartStore";
import { Product, Shop, CartItem } from "../types";
import { OrderService } from "@sdkwork/clawchat-mobile-orders";

export const CheckoutPage = () => {
  const navigate = useNavigate();
  const [searchParams] = useSearchParams();
  const productId = searchParams.get("productId");
  const skuId = searchParams.get("skuId");
  const quantity = parseInt(searchParams.get("quantity") || "1", 10);
  const isFromCart = searchParams.get("from") === "cart";

  const { getCheckedItems, clearCart } = useCartStore();
  const cartItems = getCheckedItems();

  const [singleProduct, setSingleProduct] = useState<Product | null>(null);
  const [shop, setShop] = useState<Shop | null>(null);

  // Items to checkout
  const [displayItems, setDisplayItems] = useState<any[]>([]);

  useEffect(() => {
    if (productId && !isFromCart) {
      ProductService.getProductById(productId).then((p) => {
        setSingleProduct(p);
        if (p) {
          const sku = p.skus?.find(s => s.id === skuId);
          setDisplayItems([{ product: p, quantity, sku }]);
        }
        if (p?.shopId) {
          ProductService.getShopById(p.shopId).then(setShop);
        }
      });
    } else if (isFromCart) {
      setDisplayItems(cartItems);
      // Mock single shop for simplicity of UI if from cart
      if (cartItems.length > 0 && cartItems[0].product.shopId) {
        ProductService.getShopById(cartItems[0].product.shopId).then(setShop);
      }
    }
  }, [productId, skuId, isFromCart]);

  const handleSubmitOrder = async () => {
    try {
      // 1. Create a real pending payment order in the databases/localstorage
      const newOrder = await OrderService.createOrder({
        shopName: shop?.name || "官方推荐自营店",
        isVirtual: displayItems.some(item => item.product.isVirtual),
        items: displayItems.map((item, idx) => ({
          id: item.product.id || `${Date.now()}_${idx}`,
          image: item.sku?.image || item.product.image,
          title: item.product.title,
          specs: item.sku ? Object.values(item.sku.specValues || {}).map(vId => {
            const spec = item.product.specs?.find(s => s.options.some(o => o.id === vId));
            return spec?.options.find(o => o.id === vId)?.name || vId;
          }).join(", ") : "默认规格",
          price: parseFloat(item.sku?.price || item.product.price),
          quantity: item.quantity,
          virtualType: item.product.virtualType,
        })),
        totalAmount: parseFloat(totalPrice),
        shippingFee: 0,
        address: displayItems.some(item => item.product.isVirtual) ? undefined : {
          name: "张三",
          phone: "138****8000",
          detail: "浙江省 杭州市 余杭区 仓前街道 梦想小镇天使村11号",
        },
      });

      // 2. Clear checked items from cart if checked out from cart
      if (isFromCart) {
        clearCart();
      }

      // 3. Navigate to Cashier with the actual order ID
      navigate(`/cashier?orderId=${newOrder.id}&amount=${totalPrice}`);
    } catch (e) {
      console.error("Failed to create order on checkout:", e);
      // Fallback in case of failure
      navigate(`/cashier?amount=${totalPrice}`);
    }
  };

  if (displayItems.length === 0) {
    return (
      <div className="flex flex-col h-full bg-bg-color items-center justify-center text-text-sub opacity-70">
        <div className="w-8 h-8 rounded-full border-4 border-text-sub border-t-transparent animate-spin mb-3"></div>
        <span className="text-[14px]">加载确认订单中...</span>
      </div>
    );
  }

  const totalQuantity = displayItems.reduce(
    (acc, item) => acc + item.quantity,
    0,
  );
  const totalPrice = displayItems
    .reduce(
      (acc, item) => acc + parseFloat(item.sku?.price || item.product.price) * item.quantity,
      0,
    )
    .toFixed(2);

  const isVirtualOrder = displayItems.some(i => i.product.isVirtual);

  return (
    <div className="flex flex-col h-full bg-bg-color relative">
      <header className="flex items-center justify-between px-2 pt-safe h-[56px] border-b border-border-color bg-chat-other-bg shrink-0">
        <div
          className="w-10 h-10 flex items-center justify-center cursor-pointer"
          onClick={() => navigate(-1)}
        >
          <ChevronLeft className="w-6 h-6 text-text-main" />
        </div>
        <span className="text-[17px] font-medium text-text-main">确认订单</span>
        <div className="w-10 h-10" />
      </header>

      <div className="flex-1 overflow-y-auto px-3 py-3 pb-[80px]">
        {/* Address Mock */}
        {!isVirtualOrder && (
          <div className="bg-chat-other-bg rounded-xl p-4 mb-3 flex items-center gap-3 cursor-pointer active:scale-[0.98] transition-transform">
            <div className="w-8 h-8 rounded-full bg-[#FA5151]/10 flex items-center justify-center shrink-0">
              <MapPin className="w-4 h-4 text-[#FA5151]" />
            </div>
            <div className="flex-1">
              <div className="flex items-center gap-2 mb-1">
                <span className="text-[16px] font-medium text-text-main">
                  张三
                </span>
                <span className="text-[14px] text-text-sub">138****8000</span>
              </div>
              <div className="text-[13px] text-text-main line-clamp-1">
                浙江省 杭州市 余杭区 仓前街道 梦想小镇天使村11号
              </div>
            </div>
            <ChevronRight className="w-5 h-5 text-text-sub/60" />
          </div>
        )}

        {/* Order Items */}
        <div className="bg-chat-other-bg rounded-xl p-4 mb-3">
          {shop && (
            <div className="flex items-center gap-2 mb-3">
              <Store className="w-4 h-4 text-text-main" />
              <span className="text-[14px] font-medium text-text-main">
                {shop.name}
              </span>
            </div>
          )}

          <div className="flex flex-col gap-4">
            {displayItems.map((item, idx) => (
              <div key={idx} className="flex gap-3">
                <img
                  src={item.sku?.image || item.product.image}
                  className="w-[80px] h-[80px] rounded-lg border border-border-color/30 object-cover"
                />
                <div className="flex-1 flex flex-col pt-1">
                  <span className="text-[14px] text-text-main leading-tight line-clamp-2 mb-1">
                    {item.product.title}
                  </span>
                  <span className="text-[12px] text-text-sub bg-bg-color w-max px-1.5 py-0.5 rounded-sm line-clamp-1">
                    {item.sku ? Object.values(item.sku.specValues || {}).map(vId => {
                      const spec = item.product.specs?.find(s => s.options.some(o => o.id === vId));
                      return spec?.options.find(o => o.id === vId)?.name || vId;
                    }).join(", ") : "默认规格"}
                  </span>
                  <div className="flex items-center justify-between mt-auto">
                    <span className="text-[16px] font-bold text-text-main">
                      <span className="text-[12px]">¥</span>
                      {parseFloat(item.sku?.price || item.product.price)}
                    </span>
                    <span className="text-[13px] text-text-sub">
                      x{item.quantity}
                    </span>
                  </div>
                </div>
              </div>
            ))}
          </div>

          {!isVirtualOrder && (
            <div className="flex justify-between items-center mt-4 pt-4 border-t border-border-color/50">
              <span className="text-[14px] text-text-main">配送服务</span>
              <span className="text-[13px] text-text-sub">
                普通快递 免费送达 <ChevronRight className="w-4 h-4 inline" />
              </span>
            </div>
          )}
          <div className="flex justify-between items-center mt-3">
            <span className="text-[14px] text-text-main">买家留言</span>
            <span className="text-[13px] text-text-sub">
              无留言 <ChevronRight className="w-4 h-4 inline" />
            </span>
          </div>
        </div>

        {/* Total Section */}
        <div className="bg-chat-other-bg rounded-xl p-4">
          <div className="flex justify-between items-center mb-3">
            <span className="text-[14px] text-text-sub">商品总价</span>
            <span className="text-[14px] text-text-main">¥{totalPrice}</span>
          </div>
          <div className="flex justify-between items-center">
            <span className="text-[14px] text-text-sub">运费</span>
            <span className="text-[14px] text-text-main">¥0.00</span>
          </div>
          <div className="flex justify-end items-center mt-4 pt-3 border-t border-border-color">
            <span className="text-[14px] text-text-main">合计：</span>
            <span className="text-[18px] font-bold text-[#FA5151]">
              <span className="text-[13px]">¥</span>
              {totalPrice}
            </span>
          </div>
        </div>
      </div>

      {/* Bottom Bar */}
      <div className="absolute bottom-0 left-0 right-0 bg-chat-other-bg border-t border-border-color pb-safe px-4 py-2 flex items-center justify-end h-[60px]">
        <div className="flex items-center mr-4">
          <span className="text-[13px] text-text-sub mr-1">
            共{totalQuantity}件,
          </span>
          <span className="text-[14px] text-text-main">合计:</span>
          <span className="text-[18px] font-bold text-[#FA5151] ml-1">
            <span className="text-[13px]">¥</span>
            {totalPrice}
          </span>
        </div>
        <button
          className="px-6 h-[40px] rounded-full text-[14px] font-medium bg-[#FA5151] text-white active:scale-95 transition-transform"
          onClick={handleSubmitOrder}
        >
          提交订单
        </button>
      </div>
    </div>
  );
};
