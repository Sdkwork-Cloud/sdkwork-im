import React, { useState } from "react";
import { cn } from "@sdkwork/clawchat-mobile-commons";

export const CustomDatePicker = ({
  initialValue,
  onConfirm,
  onCancel,
  defaultYearOffset = -30,
}: {
  initialValue: string;
  onConfirm: (date: string) => void;
  onCancel: () => void;
  defaultYearOffset?: number;
}) => {
  const [tempDate, setTempDate] = useState(() => {
    if (initialValue) {
      const [y, m, d] = initialValue.split("-");
      return { year: parseInt(y), month: parseInt(m), day: parseInt(d) };
    }
    const currentYear = new Date().getFullYear();
    return { year: currentYear + defaultYearOffset, month: 1, day: 1 };
  });

  const getDaysInMonth = (year: number, month: number) => {
    return new Date(year, month, 0).getDate();
  };

  return (
    <>
      <div className="flex h-[350px] relative px-4 w-full max-w-md mx-auto">
        {/* Selection Highlight */}
        <div className="absolute top-1/2 -mt-6 h-12 left-4 right-4 bg-primary-blue/5 border-y border-primary-blue/20 pointer-events-none rounded-lg" />

        {/* Year */}
        <div
          className="flex-1 h-full overflow-y-auto no-scrollbar relative snap-y snap-mandatory py-[151px]"
          style={{ scrollBehavior: "smooth" }}
        >
          {Array.from(
            { length: 120 },
            (_, i) => new Date().getFullYear() - i,
          ).map((y) => (
            <div
              key={y}
              onClick={() => setTempDate((prev) => ({ ...prev, year: y }))}
              className={cn(
                "h-12 flex items-center justify-center text-[16px] snap-center cursor-pointer",
                tempDate.year === y
                  ? "font-bold text-primary-blue scale-110 shadow-sm"
                  : "font-medium text-text-sub opacity-70",
              )}
            >
              {y}年
            </div>
          ))}
        </div>

        {/* Month */}
        <div
          className="flex-1 h-full overflow-y-auto no-scrollbar relative snap-y snap-mandatory py-[151px]"
          style={{ scrollBehavior: "smooth" }}
        >
          {Array.from({ length: 12 }, (_, i) => i + 1).map((m) => (
            <div
              key={m}
              onClick={() =>
                setTempDate((prev) => ({
                  ...prev,
                  month: m,
                  day: Math.min(prev.day, getDaysInMonth(prev.year, m)),
                }))
              }
              className={cn(
                "h-12 flex items-center justify-center text-[16px] snap-center cursor-pointer",
                tempDate.month === m
                  ? "font-bold text-primary-blue scale-110 shadow-sm"
                  : "font-medium text-text-sub opacity-70",
              )}
            >
              {m}月
            </div>
          ))}
        </div>

        {/* Day */}
        <div
          className="flex-1 h-full overflow-y-auto no-scrollbar relative snap-y snap-mandatory py-[151px]"
          style={{ scrollBehavior: "smooth" }}
        >
          {Array.from(
            { length: getDaysInMonth(tempDate.year, tempDate.month) },
            (_, i) => i + 1,
          ).map((d) => (
            <div
              key={d}
              onClick={() => setTempDate((prev) => ({ ...prev, day: d }))}
              className={cn(
                "h-12 flex items-center justify-center text-[16px] snap-center cursor-pointer",
                tempDate.day === d
                  ? "font-bold text-primary-blue scale-110 shadow-sm"
                  : "font-medium text-text-sub opacity-70",
              )}
            >
              {d}日
            </div>
          ))}
        </div>
      </div>

      {/* Invisible interceptor block: moving confirm/cancel logic correctly up to parent, this acts as the bottom sheet interceptor update */}
      <div className="absolute top-0 left-0 right-0 h-14 flex items-center justify-between px-4 z-10 pointer-events-none">
        <button
          onClick={onCancel}
          className="text-[15px] text-transparent font-medium px-2 py-1 pointer-events-auto h-full absolute left-4 w-16"
        />
        <button
          onClick={() => {
            const formatted = `${tempDate.year}-${String(tempDate.month).padStart(2, "0")}-${String(tempDate.day).padStart(2, "0")}`;
            onConfirm(formatted);
          }}
          className="text-[15px] text-transparent font-medium px-2 py-1 pointer-events-auto h-full absolute right-4 w-16"
        />
      </div>
    </>
  );
};
