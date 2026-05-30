import React from "react";
import { ChevronRight } from "lucide-react";
import { cn } from "@sdkwork/clawchat-mobile-commons";

interface BasicInfoSectionProps {
  formData: any;
  setFullPageEditor: React.Dispatch<React.SetStateAction<any>>;
  setTempDate: React.Dispatch<React.SetStateAction<any>>;
  setPickerType: React.Dispatch<React.SetStateAction<any>>;
}

export const BasicInfoSection: React.FC<BasicInfoSectionProps> = ({
  formData,
  setFullPageEditor,
  setTempDate,
  setPickerType,
}) => {
  return (
    <div className="bg-bg-color px-4 flex flex-col mb-2">
      <div
        className="flex items-center min-h-[54px] border-b border-border-color last:border-b-0 cursor-pointer active:bg-active-bg transition-colors"
        onClick={() =>
          setFullPageEditor({
            field: "phone",
            title: "联系手机",
            placeholder: "11位号码",
            value: formData.phone,
            inputType: "tel",
          })
        }
      >
        <label className="text-[15px] text-text-main w-[100px] shrink-0">
          联系手机 <span className="text-red-500">*</span>
        </label>
        <div className="flex-1 flex items-center justify-end text-[15px]">
          {formData.phone ? (
            <span className="text-text-main">{formData.phone}</span>
          ) : (
            <span className="text-text-sub">11位号码</span>
          )}
          <ChevronRight className="w-5 h-5 text-text-sub ml-1 shrink-0" />
        </div>
      </div>

      <div
        className="flex items-center min-h-[54px] border-b border-border-color last:border-b-0 cursor-pointer active:bg-active-bg transition-colors"
        onClick={() =>
          setFullPageEditor({
            field: "name",
            title: "姓名",
            placeholder: "识别或输入姓名",
            value: formData.name,
          })
        }
      >
        <label className="text-[15px] text-text-main w-[100px] shrink-0">
          姓名 <span className="text-red-500">*</span>
        </label>
        <div className="flex-1 flex items-center justify-end text-[15px]">
          {formData.name ? (
            <span className="text-text-main">{formData.name}</span>
          ) : (
            <span className="text-text-sub">识别或输入</span>
          )}
          <ChevronRight className="w-5 h-5 text-text-sub ml-1 shrink-0" />
        </div>
      </div>

      <div
        className="flex items-center min-h-[54px] border-b border-border-color last:border-b-0 cursor-pointer active:bg-active-bg transition-colors"
        onClick={() =>
          setFullPageEditor({
            field: "idCard",
            title: "身份证号",
            placeholder: "识别或输入身份证号",
            value: formData.idCard,
          })
        }
      >
        <label className="text-[15px] text-text-main w-[100px] shrink-0">
          身份证号 <span className="text-red-500">*</span>
        </label>
        <div className="flex-1 flex items-center justify-end text-[15px]">
          {formData.idCard ? (
            <span className="text-text-main">{formData.idCard}</span>
          ) : (
            <span className="text-text-sub">识别或输入</span>
          )}
          <ChevronRight className="w-5 h-5 text-text-sub ml-1 shrink-0" />
        </div>
      </div>

      <div
        className="flex items-center min-h-[54px] border-b border-border-color last:border-b-0 cursor-pointer active:bg-active-bg transition-colors"
        onClick={() => {
          if (formData.idStartDate) {
            const [y, m, d] = formData.idStartDate.split("-");
            setTempDate({
              year: parseInt(y),
              month: parseInt(m),
              day: parseInt(d),
            });
          } else {
            setTempDate({ year: 2020, month: 1, day: 1 });
          }
          setPickerType("idStartDate");
        }}
      >
        <label className="text-[15px] text-text-main w-[110px] shrink-0">
          证件开始日期 <span className="text-red-500">*</span>
        </label>
        <div className="flex-1 flex items-center justify-end text-[15px]">
          {formData.idStartDate ? (
            <span className="text-text-main">{formData.idStartDate}</span>
          ) : (
            <span className="text-text-sub">请选择开始日期</span>
          )}
          <ChevronRight className="w-5 h-5 text-text-sub ml-1 shrink-0" />
        </div>
      </div>

      <div
        className="flex items-center min-h-[54px] border-b border-border-color last:border-b-0 cursor-pointer active:bg-active-bg transition-colors"
        onClick={() => {
          if (formData.idEndDate && formData.idEndDate !== "长期") {
            const [y, m, d] = formData.idEndDate.split("-");
            setTempDate({
              year: parseInt(y),
              month: parseInt(m),
              day: parseInt(d),
            });
          } else {
            setTempDate({ year: 2040, month: 1, day: 1 });
          }
          setPickerType("idEndDate");
        }}
      >
        <label className="text-[15px] text-text-main w-[110px] shrink-0">
          证件结束日期 <span className="text-red-500">*</span>
        </label>
        <div className="flex-1 flex items-center justify-end text-[15px]">
          {formData.idEndDate ? (
            <span className="text-text-main">{formData.idEndDate}</span>
          ) : (
            <span className="text-text-sub">请选择结束日期</span>
          )}
          <ChevronRight className="w-5 h-5 text-text-sub ml-1 shrink-0" />
        </div>
      </div>

      <div
        className="flex items-center min-h-[54px] border-b border-border-color last:border-b-0 cursor-pointer active:bg-active-bg transition-colors"
        onClick={() => setPickerType("gender")}
      >
        <label className="text-[15px] text-text-main w-[100px] shrink-0">
          性别
        </label>
        <div className="flex-1 flex items-center justify-end text-[15px]">
          {formData.gender ? (
            <span className="text-text-main">{formData.gender}</span>
          ) : (
            <span className="text-text-sub">请选择</span>
          )}
          <ChevronRight className="w-5 h-5 text-text-sub ml-1 shrink-0" />
        </div>
      </div>

      <div
        className="flex items-center min-h-[54px] border-b border-border-color last:border-b-0 cursor-pointer active:bg-active-bg transition-colors"
        onClick={() => {
          if (formData.dob) {
            const [y, m, d] = formData.dob.split("-");
            setTempDate({
              year: parseInt(y),
              month: parseInt(m),
              day: parseInt(d),
            });
          } else {
            setTempDate({ year: 1990, month: 1, day: 1 });
          }
          setPickerType("dob");
        }}
      >
        <label className="text-[15px] text-text-main w-[100px] shrink-0">
          出生日期
        </label>
        <div className="flex-1 flex items-center justify-end text-[15px]">
          {formData.dob ? (
            <span className="text-text-main">{formData.dob}</span>
          ) : (
            <span className="text-text-sub">请选择</span>
          )}
          <ChevronRight className="w-5 h-5 text-text-sub ml-1 shrink-0" />
        </div>
      </div>

      <div
        className="flex items-center min-h-[54px] border-b border-border-color last:border-b-0 cursor-pointer active:bg-active-bg transition-colors"
        onClick={() =>
          setFullPageEditor({
            field: "address",
            title: "常住地址",
            placeholder: "户籍或现住址",
            value: formData.address,
            isTextArea: true,
          })
        }
      >
        <label className="text-[15px] text-text-main w-[100px] shrink-0">
          常住地址
        </label>
        <div className="flex-1 flex items-center justify-end text-[15px]">
          {formData.address ? (
            <span className="text-text-main truncate max-w-[150px]">
              {formData.address}
            </span>
          ) : (
            <span className="text-text-sub">户籍或现住址</span>
          )}
          <ChevronRight className="w-5 h-5 text-text-sub ml-1 shrink-0" />
        </div>
      </div>
    </div>
  );
};
