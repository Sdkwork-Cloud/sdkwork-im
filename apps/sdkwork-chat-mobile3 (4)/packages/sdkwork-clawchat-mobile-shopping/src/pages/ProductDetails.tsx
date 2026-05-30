import React, { useEffect, useState } from "react";
import { useNavigate, useParams } from "react-router";
import {
  ChevronLeft,
  ShoppingCart,
  Share2,
  Store,
  Headphones,
  ChevronRight,
  X,
} from "lucide-react";
import { IconButton, showToast } from "@sdkwork/clawchat-mobile-commons";
import { ProductService } from "../services/ProductService";
import { useCartStore } from "../store/useCartStore";
import { Product, Shop } from "../types";

export const ProductDetails = () => {
  const { id } = useParams<{ id: string }>();
  const navigate = useNavigate();
  const [product, setProduct] = useState<Product | null>(null);
  const [shop, setShop] = useState<Shop | null>(null);
  const { addToCart, items, loadCart } = useCartStore();
  const cartItemCount = items.reduce((acc, item) => acc + item.quantity, 0);

  // SKU Panel State
  const [showSkuPanel, setShowSkuPanel] = useState(false);
  const [skuAction, setSkuAction] = useState<"cart" | "buy">("buy");
  const [quantity, setQuantity] = useState(1);
  const [selectedSpecs, setSelectedSpecs] = useState<Record<string, string>>({});

  useEffect(() => {
    if (id) {
      ProductService.getProductById(id).then((p) => {
        setProduct(p);
        if (p?.shopId) {
          ProductService.getShopById(p.shopId).then(setShop);
        }
        if (p?.specs && p.specs.length > 0) {
          const defaultSpecs: Record<string, string> = {};
          p.specs.forEach(s => {
            if (s.options.length > 0) defaultSpecs[s.id] = s.options[0].id;
          });
          setSelectedSpecs(defaultSpecs);
        }
      });
    }
    loadCart();
  }, [id]);

  const currentSku = product?.skus?.find(sku => 
    Object.keys(selectedSpecs).every(key => sku.specValues[key] === selectedSpecs[key])
  );

  const displayPrice = currentSku?.price || product?.price;
  const displayOriginalPrice = currentSku?.originalPrice || product?.originalPrice;

  const handleAddToCartClick = () => {
    setSkuAction("cart");
    setShowSkuPanel(true);
  };

  const handleBuyNowClick = () => {
    setSkuAction("buy");
    setShowSkuPanel(true);
  };

  const handleConfirmSku = async () => {
    if (!product) return;
    
    if (product.specs && !currentSku) {
      showToast("请选择完整的商品规格");
      return;
    }

    setShowSkuPanel(false);

    if (skuAction === "cart") {
      await addToCart(product, quantity, currentSku, selectedSpecs);
      showToast("已加入购物车");
    } else {
      const skuQuery = currentSku ? `&skuId=${currentSku.id}` : '';
      navigate(`/checkout?productId=${product.id}&quantity=${quantity}${skuQuery}`);
    }
  };

  if (!product) {
    return (
      <div className="flex flex-col h-full bg-bg-color items-center justify-center text-text-sub opacity-70">
        <div className="w-8 h-8 rounded-full border-4 border-text-sub border-t-transparent animate-spin mb-3"></div>
        <span className="text-[14px]">加载中...</span>
      </div>
    );
  }

  return (
    <div className="flex flex-col h-full bg-bg-color relative">
      {/* Header */}
      <header className="absolute top-0 left-0 right-0 z-10 flex items-center justify-between px-2 pt-safe h-[56px] text-white">
        <div
          className="w-8 h-8 rounded-full bg-black/30 flex items-center justify-center backdrop-blur-sm cursor-pointer ml-2"
          onClick={() => navigate(-1)}
        >
          <ChevronLeft className="w-5 h-5" />
        </div>
        <div className="w-8 h-8 rounded-full bg-black/30 flex items-center justify-center backdrop-blur-sm cursor-pointer mr-2">
          <Share2 className="w-4 h-4" />
        </div>
      </header>

      <div className="flex-1 overflow-y-auto pb-[70px]">
        {/* Product Image */}
        <div className="w-full aspect-square bg-chat-other-bg">
          <img
            src={product.image}
            className="w-full h-full object-cover"
            alt={product.title}
          />
        </div>

        {/* Product Info */}
        <div className="bg-chat-other-bg p-4 mb-2">
          <div className="flex items-baseline mb-2">
            <span className="text-[#FA5151] font-bold text-[24px]">
              <span className="text-[16px]">¥</span>
              {displayPrice}
            </span>
            {displayOriginalPrice && (
              <span className="text-text-sub text-[14px] line-through ml-2">
                ¥{displayOriginalPrice}
              </span>
            )}
            {product.isVirtual && (
              <span className="ml-3 border border-[#FA5151] text-[#FA5151] rounded-sm px-1 py-[1px] text-[10px]">电子虚拟商品</span>
            )}
          </div>
          <h1 className="text-[17px] font-medium text-text-main leading-tight mb-2">
            {product.title}
          </h1>
          <div className="flex justify-between items-center text-[13px] text-text-sub">
            <span>{product.sales}</span>
            <span>{product.isVirtual ? '自动发码' : '发货地：浙江杭州'}</span>
          </div>
        </div>

        {/* Shop Info */}
        {shop && (
          <div
            className="bg-chat-other-bg p-4 mb-2 cursor-pointer active:bg-chat-active-bg transition-colors"
            onClick={() => navigate(`/shop/${shop.id}`)}
          >
            <div className="flex items-center gap-3 mb-3">
              <img
                src={shop.logo}
                className="w-12 h-12 rounded-lg border border-border-color/30 object-cover"
                alt={shop.name}
              />
              <div className="flex-1 flex flex-col justify-center">
                <div className="flex items-center gap-1.5">
                  <span className="text-[16px] font-medium text-text-main leading-tight">
                    {shop.name}
                  </span>
                  {shop.isOfficial && (
                    <span className="bg-[#FA5151] text-white text-[10px] px-1 py-0.5 rounded-sm line-height-none">
                      官方
                    </span>
                  )}
                </div>
                <div className="flex items-center gap-3 text-[12px] text-text-sub mt-1">
                  <span>粉丝数 {shop.fansCount}</span>
                  <span>综合评分 {shop.rating}</span>
                </div>
              </div>
              <ChevronRight className="w-5 h-5 text-text-sub/60" />
            </div>
            <div className="flex items-center gap-4">
              <div
                className="flex-1 h-8 rounded-full border border-border-color flex items-center justify-center text-[13px] text-text-main hover:bg-chat-other-bg transition-colors"
                onClick={(e) => {
                  e.stopPropagation();
                  navigate(`/shop/${shop.id}`);
                }}
              >
                进店逛逛
              </div>
              <div
                className="flex-1 h-8 rounded-full border border-border-color flex items-center justify-center text-[13px] text-text-main hover:bg-chat-other-bg transition-colors"
                onClick={(e) => {
                  e.stopPropagation();
                  navigate(`/shop-chat/${shop.id}`);
                }}
              >
                联系客服
              </div>
            </div>
          </div>
        )}

        {/* Details Mock */}
        <div className="bg-chat-other-bg p-4">
          <h2 className="text-[15px] font-medium mb-3">商品详情</h2>
          <p className="text-[14px] text-text-sub leading-relaxed whitespace-pre-wrap">
            {product.description || "这里是商品详情..."}
            <br />
            <br />
            规格：默认规格
            <br />
            品牌：严选品牌
            <br />
            毛重：0.5kg
          </p>
          <img
            src={product.image}
            className="w-full mt-4 rounded-lg object-cover"
          />
        </div>
      </div>

      {/* Bottom Actions */}
      <div className="absolute bottom-0 left-0 right-0 bg-chat-other-bg border-t border-border-color pb-safe px-2 py-2 flex items-center z-40">
        <div className="flex gap-4 pr-3 border-r border-border-color/50 px-2">
          {shop && (
            <>
              <div
                className="flex flex-col items-center justify-center text-text-sub cursor-pointer"
                onClick={() => navigate(`/shop/${shop.id}`)}
              >
                <Store className="w-5 h-5 mb-1" />
                <span className="text-[10px]">店铺</span>
              </div>
              <div
                className="flex flex-col items-center justify-center text-text-sub cursor-pointer"
                onClick={() => navigate(`/shop-chat/${shop.id}`)}
              >
                <Headphones className="w-5 h-5 mb-1" />
                <span className="text-[10px]">客服</span>
              </div>
            </>
          )}
          <div
            className="flex flex-col items-center justify-center text-text-sub cursor-pointer relative"
            onClick={() => navigate("/cart")}
          >
            <ShoppingCart className="w-5 h-5 mb-1 ml-1 text-text-main" />
            <span className="text-[10px] ml-1">购物车</span>
            {cartItemCount > 0 && (
              <span className="absolute -top-1 right-0 bg-[#FA5151] text-white text-[10px] scale-[0.8] px-1.5 py-0.5 rounded-full border border-white">
                {cartItemCount}
              </span>
            )}
          </div>
        </div>
        <div className="flex-1 flex gap-2 pl-3">
          <button
            className="flex-1 py-2 rounded-full text-[14px] font-medium bg-[#FFAA00] text-white active:scale-95 transition-transform"
            onClick={handleAddToCartClick}
          >
            加入购物车
          </button>
          <button
            className="flex-1 py-2 rounded-full text-[14px] font-medium bg-[#FA5151] text-white active:scale-95 transition-transform"
            onClick={handleBuyNowClick}
          >
            立即购买
          </button>
        </div>
      </div>

      {/* SKU Panel Overlay */}
      {showSkuPanel && (
        <div className="absolute inset-0 z-50 flex flex-col justify-end">
          {/* Overlay Background */}
          <div
            className="absolute inset-0 bg-black/40 backdrop-blur-sm"
            onClick={() => setShowSkuPanel(false)}
          />

          {/* Panel Content */}
          <div className="relative bg-bg-color rounded-t-2xl w-full min-h-[400px] flex flex-col pt-4 pb-safe animate-in slide-in-from-bottom duration-300">
            {/* Close Button */}
            <div
              className="absolute top-3 right-3 w-8 h-8 flex items-center justify-center rounded-full bg-chat-other-bg cursor-pointer"
              onClick={() => setShowSkuPanel(false)}
            >
              <X className="w-5 h-5 text-text-sub" />
            </div>

            {/* Product Header */}
            <div className="flex px-4 gap-3 mb-6">
              <img
                src={currentSku?.image || product.image}
                className="w-[100px] h-[100px] rounded-lg border-2 border-border-color/30 object-cover -mt-8 bg-chat-other-bg"
              />
              <div className="flex flex-col justify-end">
                <span className="text-[#FA5151] font-bold text-[22px] leading-none mb-1">
                  <span className="text-[14px]">¥</span>
                  {displayPrice}
                </span>
                <span className="text-[13px] text-text-sub mb-1">库存充足</span>
                <span className="text-[13px] text-text-main line-clamp-2 pr-6">
                  已选: {currentSku ? Object.values(currentSku.specValues).map(vId => {
                    for(const s of product.specs || []) {
                      const opt = s.options.find(o => o.id === vId);
                      if (opt) return opt.name;
                    }
                    return vId;
                  }).join(", ") : "默认规格"}，{quantity}件
                </span>
              </div>
            </div>

            <div className="flex-1 overflow-y-auto px-4">
              {product.specs && product.specs.length > 0 ? (
                product.specs.map(spec => (
                  <div key={spec.id} className="mb-6">
                    <h3 className="text-[15px] font-medium text-text-main mb-3">
                      {spec.name}
                    </h3>
                    <div className="flex flex-wrap gap-3">
                      {spec.options.map(opt => {
                        const isSelected = selectedSpecs[spec.id] === opt.id;
                        return (
                          <span
                            key={opt.id}
                            onClick={() => setSelectedSpecs({ ...selectedSpecs, [spec.id]: opt.id })}
                            className={`px-4 py-1.5 rounded-full text-[13px] border cursor-pointer transition-colors ${
                              isSelected 
                                ? "bg-[#FA5151]/10 text-[#FA5151] border-[#FA5151]/50" 
                                : "bg-bg-color text-text-main border-border-color"
                            }`}
                          >
                            {opt.name}
                          </span>
                        );
                      })}
                    </div>
                  </div>
                ))
              ) : (
                <div className="mb-6">
                  <h3 className="text-[15px] font-medium text-text-main mb-3">规格</h3>
                  <div className="flex flex-wrap gap-3">
                    <span className="bg-[#FA5151]/10 text-[#FA5151] border border-[#FA5151]/50 px-4 py-1.5 rounded-full text-[13px]">
                      默认规格
                    </span>
                  </div>
                </div>
              )}

              {/* Quantity */}
              <div className="flex items-center justify-between py-4 border-t border-border-color">
                <span className="text-[15px] font-medium text-text-main">
                  购买数量
                </span>
                <div className="flex items-center">
                  <button
                    className="w-8 h-8 flex items-center justify-center bg-chat-other-bg text-[18px] text-text-main active:bg-chat-active-bg transition-colors disabled:opacity-50"
                    onClick={() => setQuantity(Math.max(1, quantity - 1))}
                    disabled={quantity <= 1}
                  >
                    -
                  </button>
                  <span className="w-10 h-8 flex items-center justify-center text-[14px] bg-chat-other-bg border-x border-border-color text-text-main">
                    {quantity}
                  </span>
                  <button
                    className="w-8 h-8 flex items-center justify-center bg-chat-other-bg text-[18px] text-text-main active:bg-chat-active-bg transition-colors"
                    onClick={() => setQuantity(quantity + 1)}
                  >
                    +
                  </button>
                </div>
              </div>
            </div>

            {/* Confirm Button */}
            <div className="px-4 py-2 border-t border-border-color">
              <button
                className={`w-full py-2.5 rounded-full text-[15px] font-medium text-white shadow-sm active:scale-[0.98] transition-transform ${skuAction === "cart" ? "bg-[#FFAA00]" : "bg-[#FA5151]"}`}
                onClick={handleConfirmSku}
              >
                确定
              </button>
            </div>
          </div>
        </div>
      )}
    </div>
  );
};
