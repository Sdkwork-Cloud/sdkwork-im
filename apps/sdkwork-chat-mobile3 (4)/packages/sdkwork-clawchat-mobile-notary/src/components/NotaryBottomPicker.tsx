import React from "react";
import { cn } from "@sdkwork/clawchat-mobile-commons";
import { CustomDatePicker } from "./CustomDatePicker";

interface NotaryBottomPickerProps {
  pickerType: "gender" | "dob" | "idStartDate" | "idEndDate" | null;
  formData: any;
  setFormData: React.Dispatch<React.SetStateAction<any>>;
  setPickerType: React.Dispatch<React.SetStateAction<any>>;
}

export const NotaryBottomPicker: React.FC<NotaryBottomPickerProps> = ({
  pickerType,
  formData,
  setFormData,
  setPickerType,
}) => {
  if (!pickerType) return null;

  return (
    <>
      <div
        className="fixed inset-0 bg-black/40 z-[250] animate-in fade-in"
        onClick={() => setPickerType(null)}
      />
      <div className="fixed bottom-0 left-0 right-0 z-[300] bg-bg-color rounded-t-2xl flex flex-col animate-in slide-in-from-bottom pb-safe max-h-[85vh]">
        <div className="flex items-center justify-between px-4 h-14 border-b border-border-color shrink-0 relative z-20">
          <button
            onClick={() => setPickerType(null)}
            className={cn(
              "text-[15px] font-medium px-2 py-1 active:opacity-70",
              pickerType === "dob" ||
                pickerType === "idStartDate" ||
                pickerType === "idEndDate"
                ? "opacity-0 pointer-events-none"
                : "text-text-sub",
            )}
          >
            取消
          </button>
          <span className="font-bold text-[16px] pointer-events-none">
            {pickerType === "gender"
              ? "选择性别"
              : pickerType === "dob"
                ? "选择出生日期"
                : pickerType === "idStartDate"
                  ? "选择开始日期"
                  : pickerType === "idEndDate"
                    ? "选择结束日期"
                    : ""}
          </span>
          <div className="flex items-center">
            {pickerType === "idEndDate" && (
              <button
                onClick={() => {
                  setFormData((prev: any) => ({ ...prev, idEndDate: "长期" }));
                  setPickerType(null);
                }}
                className="text-[13px] text-[#FA5151] mr-3 font-medium active:opacity-70 border border-[#FA5151]/30 px-2 py-0.5 rounded"
              >
                设为长期
              </button>
            )}
            <button
              onClick={() => {
                setPickerType(null);
              }}
              className={cn(
                "text-[15px] font-medium px-2 py-1 active:opacity-70",
                pickerType === "dob" ||
                  pickerType === "idStartDate" ||
                  pickerType === "idEndDate"
                  ? "opacity-0 pointer-events-none"
                  : "text-primary-blue",
              )}
            >
              确定
            </button>
          </div>
        </div>

        {pickerType === "gender" && (
          <div className="flex flex-col py-6 px-6 gap-3 min-h-[220px] w-full max-w-sm mx-auto justify-center">
            {["男", "女", "未知"].map((g) => (
              <div
                key={g}
                onClick={() => {
                  setFormData((prev: any) => ({ ...prev, gender: g }));
                  setPickerType(null);
                }}
                className={cn(
                  "h-14 w-full rounded-2xl flex items-center justify-center font-bold text-[16px] transition-all cursor-pointer shadow-sm active:scale-[0.98]",
                  formData.gender === g
                    ? "bg-primary-blue text-white ring-2 ring-primary-blue/30 ring-offset-2 dark:ring-offset-black"
                    : "bg-input-bg text-text-main hover:bg-black/5 dark:hover:bg-white/5 border border-border-color/50",
                )}
              >
                {g}
              </div>
            ))}
          </div>
        )}

        {(pickerType === "dob" ||
          pickerType === "idStartDate" ||
          pickerType === "idEndDate") && (
          <CustomDatePicker
            initialValue={
              pickerType === "dob"
                ? formData.dob
                : pickerType === "idStartDate"
                  ? formData.idStartDate
                  : formData.idEndDate === "长期"
                    ? ""
                    : formData.idEndDate
            }
            onConfirm={(formatted) => {
              if (pickerType === "dob") {
                setFormData((prev: any) => ({ ...prev, dob: formatted }));
              } else if (pickerType === "idStartDate") {
                setFormData((prev: any) => ({
                  ...prev,
                  idStartDate: formatted,
                }));
              } else if (pickerType === "idEndDate") {
                setFormData((prev: any) => ({ ...prev, idEndDate: formatted }));
              }
              setPickerType(null);
            }}
            onCancel={() => setPickerType(null)}
            defaultYearOffset={
              pickerType === "idEndDate"
                ? 20
                : pickerType === "idStartDate"
                  ? -1
                  : -30
            }
          />
        )}
      </div>
    </>
  );
};
