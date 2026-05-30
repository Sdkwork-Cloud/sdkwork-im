import React, { useState, useRef, useEffect } from "react";
import { useNavigate } from "react-router";
import { ChevronLeft, X } from "lucide-react";
import { IconButton, cn, showToast } from "@sdkwork/clawchat-mobile-commons";
import { NotaryFullPageEditor } from "../components/NotaryFullPageEditor";
import { NotaryBottomPicker } from "../components/NotaryBottomPicker";
import { IdentityVerificationSection } from "../components/IdentityVerificationSection";
import { BasicInfoSection } from "../components/BasicInfoSection";
import { AccessoriesRemarksSection } from "../components/AccessoriesRemarksSection";

export const NotaryPartyParams = {
  editData: null as any,
  onAdd: (party: any) => {},
  onEdit: (party: any) => {},
};

export const NotaryAddParty: React.FC = () => {
  const navigate = useNavigate();

  const [formData, setFormData] = useState(() => {
    if (NotaryPartyParams.editData) {
      return {
        name: NotaryPartyParams.editData.name || "",
        idCard: NotaryPartyParams.editData.idCard || "",
        gender: NotaryPartyParams.editData.gender || "男",
        dob: NotaryPartyParams.editData.dob || "",
        idStartDate: NotaryPartyParams.editData.idStartDate || "",
        idEndDate: NotaryPartyParams.editData.idEndDate || "",
        phone: NotaryPartyParams.editData.phone || "",
        address: NotaryPartyParams.editData.address || "",
        remarks: NotaryPartyParams.editData.remarks || "",
      };
    }
    return {
      name: "",
      idCard: "",
      gender: "男",
      dob: "",
      idStartDate: "",
      idEndDate: "",
      phone: "",
      address: "",
      remarks: "",
    };
  });

  const [faceScore, setFaceScore] = useState<number | null>(() => {
    if (NotaryPartyParams.editData?.faceScore) {
      return parseFloat(NotaryPartyParams.editData.faceScore);
    }
    return null;
  });
  const [isScanning, setIsScanning] = useState(false);
  const [pickerType, setPickerType] = useState<
    "gender" | "dob" | "idStartDate" | "idEndDate" | null
  >(null);
  const [tempDate, setTempDate] = useState({ year: 1990, month: 1, day: 1 });

  const [fullPageEditor, setFullPageEditor] = useState<{
    field: keyof typeof formData;
    title: string;
    placeholder: string;
    value: string;
    isTextArea?: boolean;
    inputType?: string;
  } | null>(null);

  const idFrontRef = useRef<HTMLInputElement>(null);
  const idBackRef = useRef<HTMLInputElement>(null);
  const attachmentRef = useRef<HTMLInputElement>(null);
  const faceRef = useRef<HTMLInputElement>(null);

  const [idFrontPreview, setIdFrontPreview] = useState<string | null>(
    NotaryPartyParams.editData?.idFrontPreview || null,
  );
  const [idBackPreview, setIdBackPreview] = useState<string | null>(
    NotaryPartyParams.editData?.idBackPreview || null,
  );
  const [facePreview, setFacePreview] = useState<string | null>(
    NotaryPartyParams.editData?.facePreview || null,
  );
  const [attachments, setAttachments] = useState<
    { name: string; url: string }[]
  >(NotaryPartyParams.editData?.attachments || []);
  const [fullscreenImage, setFullscreenImage] = useState<string | null>(null);

  // Cleanup object URLs on unmount
  useEffect(() => {
    return () => {};
  }, []);

  const handleFileChange = (
    e: React.ChangeEvent<HTMLInputElement>,
    setter: React.Dispatch<React.SetStateAction<string | null>>,
    existingUrl: string | null,
    side?: "front" | "back",
  ) => {
    const file = e.target.files?.[0];
    if (file) {
      setter(URL.createObjectURL(file));

      showToast("正在自动识别...");
      setTimeout(() => {
        if (side === "front") {
          setFormData((prev) => ({
            ...prev,
            name: "李小明",
            idCard: "11010519900101234X",
            gender: "男",
            dob: "1990-01-01",
            address: "北京市朝阳区建国路88号",
          }));
          showToast("人像面识别成功");
        } else if (side === "back") {
          setFormData((prev) => ({
            ...prev,
            idStartDate: "2020-01-01",
            idEndDate: "2040-01-01",
          }));
          showToast("国徽面识别成功");
        }
      }, 1000);
    }
  };

  const handleAttachmentsChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const files = e.target.files;
    if (files && files.length > 0) {
      const newAttachments = Array.from(files).map((file: File) => ({
        name: file.name,
        url: URL.createObjectURL(file), // Will generate preview if it's an image
      }));
      setAttachments((prev) => [...prev, ...newAttachments]);
    }
  };

  const handleFaceChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const file = e.target.files?.[0];
    if (file) {
      setFacePreview(URL.createObjectURL(file));
      setFaceScore(null);
    }
  };

  const handleStartComparison = () => {
    if (!facePreview) return;
    setIsScanning(true);
    setTimeout(() => {
      setFaceScore(98.5);
      setIsScanning(false);
    }, 1500);
  };

  const handleSave = () => {
    if (!idFrontPreview) return showToast("请上传身份证人像面");
    if (!idBackPreview) return showToast("请上传身份证国徽面");
    if (!facePreview) return showToast("请完成实人拍照采集");
    if (!formData.name || formData.name.trim().length < 2)
      return showToast("请输入真实有效的姓名");
    if (
      !formData.idCard ||
      !/(^\d{15}$)|(^\d{18}$)|(^\d{17}(\d|X|x)$)/.test(formData.idCard)
    )
      return showToast("请输入正确的身份证号码");
    if (!formData.phone || !/^1\d{10}$/.test(formData.phone))
      return showToast("请输入有效的11位手机号");

    if (!formData.idStartDate || !formData.idEndDate)
      return showToast("请填写完整的身份证有效时间");

    const partyData = {
      id: NotaryPartyParams.editData
        ? NotaryPartyParams.editData.id
        : Date.now().toString(),
      name: formData.name,
      idCard: formData.idCard,
      gender: formData.gender,
      dob: formData.dob,
      idStartDate: formData.idStartDate,
      idEndDate: formData.idEndDate,
      phone: formData.phone,
      address: formData.address,
      remarks: formData.remarks,
      faceScore: faceScore ? faceScore.toFixed(2) : null,
      attachmentsCount: attachments.length,
      attachments: attachments,
      idFrontPreview,
      idBackPreview,
      facePreview,
    };

    if (NotaryPartyParams.editData && NotaryPartyParams.onEdit) {
      NotaryPartyParams.onEdit(partyData);
    } else if (NotaryPartyParams.onAdd) {
      NotaryPartyParams.onAdd(partyData);
    }

    navigate(-1);
  };

  return (
    <div className="flex flex-col h-full bg-[#f4f6f9] dark:bg-black text-text-main fixed inset-0 z-[100] animate-in slide-in-from-right">
      <header className="h-[44px] flex items-center justify-between sticky top-0 shrink-0 pt-safe px-1 z-20 bg-bg-color border-b border-border-color">
        <div className="flex items-center z-10 flex-1">
          <IconButton
            icon={
              <ChevronLeft
                className="w-6 h-6 text-text-main"
                strokeWidth={2.5}
              />
            }
            onClick={() => navigate(-1)}
          />
        </div>
        <div className="flex items-center justify-center font-bold text-[17px] pointer-events-none">
          {NotaryPartyParams.editData ? "编辑当事人" : "添加当事人"}
        </div>
        <div className="flex justify-end z-10 flex-1 pr-4"></div>
      </header>

      <div className="flex-1 overflow-y-auto pb-24 relative z-0">
        <div className="flex flex-col gap-2">
          {/* Section 1: Identity Verification */}
          <IdentityVerificationSection
            idFrontRef={idFrontRef}
            idBackRef={idBackRef}
            faceRef={faceRef}
            idFrontPreview={idFrontPreview}
            idBackPreview={idBackPreview}
            facePreview={facePreview}
            faceScore={faceScore}
            isScanning={isScanning}
            setIdFrontPreview={setIdFrontPreview}
            setIdBackPreview={setIdBackPreview}
            setFacePreview={setFacePreview}
            setFaceScore={setFaceScore}
            setFullscreenImage={setFullscreenImage}
            handleFileChange={handleFileChange}
            handleFaceChange={handleFaceChange}
            handleStartComparison={handleStartComparison}
          />

          {/* Section 2: Basic Info (Cell Layout) */}
          <BasicInfoSection
            formData={formData}
            setFullPageEditor={setFullPageEditor}
            setTempDate={setTempDate}
            setPickerType={setPickerType}
          />

          {/* Section 3: Accessories & Remarks */}
          <AccessoriesRemarksSection
            formData={formData}
            attachments={attachments}
            setFullPageEditor={setFullPageEditor}
            setAttachments={setAttachments}
            attachmentRef={attachmentRef}
            handleAttachmentsChange={handleAttachmentsChange}
          />
        </div>
      </div>

      {/* Fixed Bottom Operations */}
      <div className="fixed bottom-0 left-0 right-0 p-3 bg-bg-color border-t border-border-color pb-safe z-20 flex gap-3 shadow-[0_-4px_20px_rgba(0,0,0,0.03)] dark:shadow-none">
        <button
          onClick={() => navigate(-1)}
          className="flex-[1] h-12 rounded-xl font-bold text-[15px] flex items-center justify-center bg-active-bg text-text-main active:opacity-70 transition-opacity"
        >
          取消
        </button>
        <button
          onClick={handleSave}
          className="flex-[2] h-12 rounded-xl font-bold text-[15px] flex items-center justify-center transition-opacity shadow-sm bg-primary-blue text-white active:scale-[0.98]"
        >
          保存并添加
        </button>
      </div>

      {/* Fullscreen Image Preview Overlay */}
      {fullscreenImage && (
        <div
          className="fixed inset-0 z-[200] bg-black/90 flex flex-col animate-in fade-in cursor-pointer"
          onClick={() => setFullscreenImage(null)}
        >
          <div className="h-14 flex items-center justify-end px-4 pt-safe safe-area-top">
            <button
              className="text-white p-2"
              onClick={(e) => {
                e.stopPropagation();
                setFullscreenImage(null);
              }}
            >
              <X className="w-8 h-8" />
            </button>
          </div>
          <div className="flex-1 flex items-center justify-center p-4">
            <img
              src={fullscreenImage}
              alt="预览"
              className="max-w-full max-h-full object-contain"
            />
          </div>
        </div>
      )}

      {/* Full Page Editor Overlay */}
      {fullPageEditor && (
        <NotaryFullPageEditor
          field={fullPageEditor.field}
          title={fullPageEditor.title}
          placeholder={fullPageEditor.placeholder}
          value={fullPageEditor.value}
          isTextArea={fullPageEditor.isTextArea}
          inputType={fullPageEditor.inputType}
          onChange={(val) =>
            setFullPageEditor({ ...fullPageEditor, value: val })
          }
          onSave={() => {
            setFormData((prev) => ({
              ...prev,
              [fullPageEditor.field]: fullPageEditor.value,
            }));
            setFullPageEditor(null);
          }}
          onClose={() => setFullPageEditor(null)}
        />
      )}

      {/* Pickers Overlay */}
      <NotaryBottomPicker
        pickerType={pickerType}
        formData={formData}
        setFormData={setFormData}
        setPickerType={setPickerType}
      />
    </div>
  );
};
