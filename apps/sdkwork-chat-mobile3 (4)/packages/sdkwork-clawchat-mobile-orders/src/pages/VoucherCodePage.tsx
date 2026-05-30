import React from "react";
import { useParams, useNavigate } from "react-router";
import { ChevronLeft, Copy } from "lucide-react";
import { showToast } from "@sdkwork/clawchat-mobile-commons";

export const VoucherCodePage: React.FC = () => {
  const { code } = useParams();
  const navigate = useNavigate();

  return (
    <div className="fixed inset-0 z-50 flex flex-col bg-[#F7F7F7] dark:bg-[#121212]">
      <header className="flex items-center justify-between px-2 pt-safe h-[56px] shrink-0 bg-white dark:bg-[#1E1E1E]">
        <div className="flex items-center z-10 flex-1">
          <button onClick={() => navigate(-1)} className="p-2 active:opacity-70">
            <ChevronLeft className="w-6 h-6 text-text-main" />
          </button>
        </div>
        <div className="absolute left-1/2 -translate-x-1/2">
          <h2 className="text-[17px] font-semibold text-text-main">
            向商家出示此码
          </h2>
        </div>
        <div className="flex-1" />
      </header>
      
      <div className="flex-1 p-4 flex flex-col items-center justify-center">
        <div className="bg-white dark:bg-[#1E1E1E] rounded-2xl w-full max-w-[320px] p-8 flex flex-col items-center shadow-sm">
          <div className="flex items-center justify-between bg-black/5 dark:bg-white/5 rounded-lg w-full mb-8 px-4 py-3 gap-2">
            <div className="text-[18px] font-mono tracking-widest font-bold text-text-main flex-1 text-center truncate">
              {code}
            </div>
            <button
              onClick={async () => {
                if (!code) return;
                try {
                  await navigator.clipboard.writeText(code);
                  showToast("复制成功");
                } catch (e) {
                  showToast("复制失败");
                }
              }}
              className="flex items-center justify-center w-8 h-8 rounded-full bg-black/5 dark:bg-white/10 active:opacity-70 shrink-0"
            >
              <Copy className="w-4 h-4 text-text-main" />
            </button>
          </div>
          
          <div className="w-[220px] h-[220px] bg-white rounded-lg flex items-center justify-center border-[6px] border-white shadow-sm mb-6 relative overflow-hidden">
             <div className="absolute inset-0 bg-[url('https://cdn-icons-png.flaticon.com/512/714/714390.png')] bg-contain bg-center bg-no-repeat opacity-80 mix-blend-multiply"></div>
          </div>
          
          <p className="text-[14px] text-text-sub mt-2 text-center leading-relaxed">
            请在门店向收银员出示此码
            <br />
            或将上面数字告知店员进行核销
          </p>
        </div>
      </div>
    </div>
  );
};
