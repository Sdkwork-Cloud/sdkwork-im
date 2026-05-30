import React, { useEffect, useState } from "react";
import { useNavigate, useParams } from "react-router";
import { ChevronLeft, Share2, Search, Package } from "lucide-react";
import { ProductService } from "../services/ProductService";
import { Product, Shop } from "../types";

export const ShopDetails = () => {
  const { id } = useParams<{ id: string }>();
  const navigate = useNavigate();
  const [shop, setShop] = useState<Shop | null>(null);
  const [products, setProducts] = useState<Product[]>([]);

  useEffect(() => {
    if (id) {
      ProductService.getShopById(id).then(setShop);
      ProductService.getProductsByShop(id).then(setProducts);
    }
  }, [id]);

  if (!shop) {
    return (
      <div className="flex flex-col h-full bg-bg-color items-center justify-center text-text-sub opacity-70">
        <div className="w-8 h-8 rounded-full border-4 border-text-sub border-t-transparent animate-spin mb-3"></div>
        <span className="text-[14px]">加载中...</span>
      </div>
    );
  }

  return (
    <div className="flex flex-col h-full bg-bg-color relative">
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

      <div className="flex-1 overflow-y-auto">
        {/* Shop Header Banner */}
        <div className="w-full relative h-[180px] bg-gradient-to-br from-gray-700 to-gray-900 pt-safe">
          <img
            src={shop.logo}
            className="absolute inset-0 w-full h-full object-cover opacity-30"
            blur-radius="10"
          />
          <div className="absolute inset-0 bg-black/20" />
          <div className="absolute bottom-4 left-4 right-4 flex items-end gap-3 z-10">
            <img
              src={shop.logo}
              className="w-[60px] h-[60px] rounded-lg border-2 border-white object-cover shadow-sm bg-white"
            />
            <div className="flex-1 text-white pb-1">
              <h1 className="text-[18px] font-medium leading-tight mb-1 shadow-sm">
                {shop.name}
              </h1>
              <div className="flex items-center gap-2 text-[12px] opacity-90">
                <span>粉丝 {shop.fansCount}</span>
                <span>评价 {shop.rating}</span>
              </div>
            </div>
            <button className="h-7 px-4 rounded-full bg-[#FA5151] text-white text-[13px] font-medium active:scale-95 transition-transform mb-1 flex items-center justify-center">
              关注
            </button>
          </div>
        </div>

        {/* Shop Details & Tags */}
        <div className="bg-chat-other-bg px-4 py-3 border-b border-border-color">
          <div className="flex gap-2 mb-2">
            {shop.isOfficial && (
              <span className="text-[10px] bg-[#FA5151]/10 text-[#FA5151] px-1.5 py-0.5 rounded-sm">
                官方
              </span>
            )}
            {shop.tags?.map((t) => (
              <span
                key={t}
                className="text-[10px] bg-chat-other-bg text-text-sub px-1.5 py-0.5 rounded-sm"
              >
                {t}
              </span>
            ))}
          </div>
          {shop.description && (
            <p className="text-[13px] text-text-sub leading-normal line-clamp-2">
              {shop.description}
            </p>
          )}
        </div>

        {/* Search Bar */}
        <div className="bg-chat-other-bg p-3 border-b border-border-color sticky top-0 z-10">
          <div className="bg-chat-other-bg rounded-full h-9 flex items-center px-4 gap-2">
            <Search className="w-4 h-4 text-text-sub" />
            <input
              type="text"
              placeholder="搜索店铺内商品"
              className="bg-transparent flex-1 text-[14px] text-text-main outline-none"
            />
          </div>
        </div>

        {/* Products Grid */}
        <div className="p-3 grid grid-cols-2 gap-3 pb-safe">
          {products.map((p) => (
            <div
              key={p.id}
              className="bg-chat-other-bg rounded-xl overflow-hidden shadow-sm border border-border-color/30 flex flex-col active:scale-[0.98] transition-transform cursor-pointer"
              onClick={() => navigate(`/product/${p.id}`)}
            >
              <div className="w-full aspect-square relative bg-chat-other-bg">
                <img src={p.image} className="w-full h-full object-cover" />
              </div>
              <div className="p-3 flex flex-col py-3">
                <span className="text-[14px] text-text-main font-medium leading-tight mb-2 line-clamp-2">
                  {p.title}
                </span>
                <div className="flex items-center justify-between mt-auto">
                  <span className="text-[#FA5151] font-bold text-[16px]">
                    <span className="text-[12px]">¥</span>
                    {p.price}
                  </span>
                  <span className="text-[11px] text-text-sub">{p.sales}</span>
                </div>
              </div>
            </div>
          ))}
          {products.length === 0 && (
            <div className="col-span-2 py-20 flex flex-col items-center justify-center text-text-sub opacity-70">
              <Package className="w-12 h-12 mb-3 stroke-current opacity-40" />
              <span className="text-[14px]">暂无商品</span>
            </div>
          )}
        </div>
      </div>
    </div>
  );
};
