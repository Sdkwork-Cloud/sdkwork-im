import React from 'react';
import { Share } from 'lucide-react';

export const SignaturePadMobileQR: React.FC = () => {
  return (
    <div className="flex flex-col items-center bg-[#181818] border border-white/10 rounded-2xl p-6 w-72 shrink-0 shadow-lg">
        <h3 className="text-gray-200 font-medium mb-1">离座/异地签名</h3>
        <p className="text-gray-500 text-xs text-center mb-4">使用手机扫描二维码，在移动端完成签名，签名将自动同步至此页面。</p>
        
        <div className="bg-white p-2 rounded-xl mb-4 w-full aspect-square flex items-center justify-center">
            <div className="w-full h-full border-4 border-black relative">
              <div className="absolute top-0 left-0 w-6 h-6 border-t-4 border-l-4 border-black"></div>
              <div className="absolute top-0 right-0 w-6 h-6 border-t-4 border-r-4 border-black"></div>
              <div className="absolute bottom-0 left-0 w-6 h-6 border-b-4 border-l-4 border-black"></div>
              <div className="absolute bottom-0 right-0 w-6 h-6 border-b-4 border-r-4 border-black"></div>
              <div className="w-full h-full flex items-center justify-center p-2">
                  <div className="w-full h-full bg-[url('data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSIxMDAlIiBoZWlnaHQ9IjEwMCUiPgo8cmVjdCB3aWR0aD0iMTAwJSIgaGVpZ2h0PSIxMDAlIiBmaWxsPSJub25lIiAvPgo8cGF0aCBkPSJNMCAwaDEwdjEwSDB6bTIwIDBoMTB2MTBIMjB6bTIwIDBoMTB2MTBIMDB6TTAgMjBoMTB2MTBIMHptNDAgMGgxMHYxMEg0MHpNMCA0MGgxMHYxMEgwcjMwIDBoMTB2MTBIMzB6IiBmaWxsPSJibGFjayIvPgo8L3N2Zz4=')] opacity-80" style={{ backgroundSize: '16px 16px' }}></div>
              </div>
            </div>
        </div>
        
        <button className="w-full flex items-center justify-center gap-2 text-sm text-indigo-400 bg-indigo-500/10 hover:bg-indigo-500/20 py-2 rounded-lg transition-colors border border-indigo-500/20">
            <Share size={16} /> 复制签名链接
        </button>
    </div>
  );
};
