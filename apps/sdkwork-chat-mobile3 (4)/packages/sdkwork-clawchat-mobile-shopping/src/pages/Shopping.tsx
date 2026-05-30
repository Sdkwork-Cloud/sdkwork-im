import React, { useState, useEffect } from "react";
import { useNavigate } from "react-router";
import { Search, ShoppingCart, ChevronLeft } from "lucide-react";
import { cn, IconButton } from "@sdkwork/clawchat-mobile-commons";
import { ProductService } from "../services/ProductService";
import { Product } from "../types";
import { useCartStore } from "../store/useCartStore";

const PageLayout = ({
  title,
  children,
  rightElement = null,
}: {
  title?: string;
  children: React.ReactNode;
  rightElement?: React.ReactNode;
}) => {
  const navigate = useNavigate();
  return (
    <div className="flex flex-col h-full bg-bg-color overflow-y-auto">
      <header className="flex items-center px-2 pt-safe h-[56px] shrink-0 sticky top-0 bg-bg-color/80 backdrop-blur-md z-10">
        <IconButton
          icon={<ChevronLeft className="w-6 h-6 text-text-main" />}
          onClick={() => navigate(-1)}
        />
        <div className="absolute left-1/2 -translate-x-1/2 flex items-center justify-center pointer-events-none">
          <h2 className="text-[17px] font-medium text-text-main">{title}</h2>
        </div>
        <div className="flex-1 flex justify-end pr-1">{rightElement}</div>
      </header>
      <div className="flex flex-col px-0 sm:px-4 pb-12 mt-2">{children}</div>
    </div>
  );
};

export const ShoppingPage = () => {
  const [products, setProducts] = useState<Product[]>([]);
  const [categories, setCategories] = useState<string[]>([]);
  const [isLoading, setIsLoading] = useState(true);
  const navigate = useNavigate();
  const { items, loadCart } = useCartStore();

  useEffect(() => {
    setIsLoading(true);
    Promise.all([
      ProductService.getProducts().then(setProducts),
      ProductService.getCategories().then(setCategories)
    ]).then(() => setIsLoading(false));
    loadCart();
  }, []);

  const cartItemCount = items.reduce((acc, item) => acc + item.quantity, 0);

  return (
    <PageLayout
      rightElement={
        <div
          className="relative cursor-pointer"
          onClick={() => navigate("/cart")}
        >
          <IconButton
            icon={<ShoppingCart className="w-[22px] h-[22px] text-text-main" />}
          />
          {cartItemCount > 0 && (
            <span className="absolute top-1 right-1 bg-[#FA5151] text-white text-[10px] scale-90 px-1.5 py-0.5 rounded-full border border-white pointer-events-none">
              {cartItemCount}
            </span>
          )}
        </div>
      }
    >
      {/* Search & Banner */}
      <div className="px-4 py-2">
        <div className="bg-chat-other-bg rounded-lg h-10 flex items-center px-4 gap-2 mb-4">
          <Search className="w-5 h-5 text-text-sub" />
          <input
            type="text"
            placeholder="搜索好物..."
            className="bg-transparent flex-1 text-[15px] text-text-main outline-none"
          />
        </div>
        <div className="w-full h-32 rounded-xl bg-gradient-to-r from-blue-500 to-indigo-600 relative overflow-hidden mb-6 flex items-center px-6 shadow-sm">
          <img
            src="https://picsum.photos/seed/salebanner/800/300"
            className="absolute inset-0 w-full h-full object-cover opacity-60 mix-blend-overlay"
          />
          <div className="relative z-10 text-white">
            <h2 className="text-2xl font-bold italic drop-shadow-md">
              春季大促
            </h2>
            <p className="text-sm opacity-90 drop-shadow-md">
              满199减50 / 限时包邮
            </p>
          </div>
        </div>
      </div>

      {/* Categories */}
      <div className="flex gap-4 px-4 overflow-x-auto no-scrollbar mb-4 pb-2">
        {categories.map((cat, i) => (
          <div
            key={i}
            className={cn(
              "px-4 py-1.5 rounded-full whitespace-nowrap text-[14px] cursor-pointer transition-colors active:scale-95",
              i === 0
                ? "bg-primary-blue text-white font-medium shadow-sm"
                : "bg-chat-other-bg text-text-sub shadow-sm",
            )}
          >
            {cat}
          </div>
        ))}
      </div>

      {/* Grid */}
      <div className="px-4 pb-10 grid grid-cols-2 gap-3 min-h-[300px]">
        {isLoading ? (
          <div className="col-span-2 flex flex-col items-center justify-center py-20 text-text-sub opacity-70">
            <div className="w-8 h-8 rounded-full border-4 border-text-sub border-t-transparent animate-spin mb-3"></div>
            <p className="text-[14px]">加载中...</p>
          </div>
        ) : products.map((p) => (
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
      </div>
    </PageLayout>
  );
};
