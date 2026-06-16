import React, { useState, useRef, useEffect } from 'react';
import { motion, AnimatePresence } from 'motion/react';
import { Plus, Trash2, ShieldCheck, User as UserIcon, FileText, Camera, Loader2, X, PenTool } from 'lucide-react';
import { Party } from '@sdkwork/im-pc-types';
import { toast } from '@sdkwork/im-pc-chat';

interface PartyDrawerProps {
  isOpen: boolean;
  onClose: () => void;
  party: Partial<Party> | null;
  onSave: (party: Party) => void;
  readOnly?: boolean;
  onSign?: (party: Party) => void;
}

export const PartyDrawer: React.FC<PartyDrawerProps> = ({ isOpen, onClose, party, onSave, readOnly, onSign }) => {
  const [newParty, setNewParty] = useState({ 
    name: '', phone: '', identityId: '', address: '', role: '申请人', gender: '男', birthDate: '', remarks: '', identityValidDateStart: '', identityValidDateEnd: ''
  });

  const [idFront, setIdFront] = useState<string | null>(null);
  const [idFrontFile, setIdFrontFile] = useState<File | null>(null);
  const [idBack, setIdBack] = useState<string | null>(null);
  const [idBackFile, setIdBackFile] = useState<File | null>(null);
  const [faceImage, setFaceImage] = useState<string | null>(null);
  const [faceImageDataUrl, setFaceImageDataUrl] = useState<string | null>(null);
  const [attachments, setAttachments] = useState<{ id: string, url: string, name: string, file: File }[]>([]);
  
  const [isCameraOpen, setIsCameraOpen] = useState(false);
  const [isComparing, setIsComparing] = useState(false);
  const [compareResult, setCompareResult] = useState<number | null>(null);
  
  const idFrontRef = useRef<HTMLInputElement>(null);
  const idBackRef = useRef<HTMLInputElement>(null);
  const attachRef = useRef<HTMLInputElement>(null);
  const localAttachmentIdSequenceRef = useRef(0);
  
  const videoRef = useRef<HTMLVideoElement>(null);
  const streamRef = useRef<MediaStream | null>(null);
  const canvasRef = useRef<HTMLCanvasElement>(null);

  const createLocalAttachmentId = () => {
    localAttachmentIdSequenceRef.current += 1;
    return `party-attachment-${localAttachmentIdSequenceRef.current}`;
  };

  useEffect(() => {
    if (isOpen) {
      if (party) {
        setNewParty({
          name: party.name || '',
          phone: party.phone || '',
          identityId: party.identityId || '',
          address: party.address || '',
          role: party.role || '申请人',
          gender: party.gender || '男',
          birthDate: party.birthDate || '',
          remarks: party.remarks || '',
          identityValidDateStart: party.identityValidDateStart || '',
          identityValidDateEnd: party.identityValidDateEnd || ''
        });
      } else {
        setNewParty({ name: '', phone: '', identityId: '', address: '', role: '申请人', gender: '男', birthDate: '', remarks: '', identityValidDateStart: '', identityValidDateEnd: '' });
      }
      setIdFront(null);
      setIdFrontFile(null);
      setIdBack(null);
      setIdBackFile(null);
      setFaceImage(null);
      setFaceImageDataUrl(null);
      setAttachments([]);
      setCompareResult(null);
    }
  }, [isOpen, party]);

  useEffect(() => {
    return () => {
      if (streamRef.current) {
        streamRef.current.getTracks().forEach(track => track.stop());
      }
    };
  }, []);

  const handleFileUpload = (e: React.ChangeEvent<HTMLInputElement>, setter: (val: string | null) => void) => {
    const file = e.target.files?.[0];
    if (file) {
      const previewUrl = URL.createObjectURL(file);
      setter(previewUrl);
      if (setter === setIdFront) {
        setIdFrontFile(file);
        setCompareResult(null);
      }
      if (setter === setIdBack) {
        setIdBackFile(file);
      }
    }
    e.target.value = '';
  };

  const handleAttachUpload = (e: React.ChangeEvent<HTMLInputElement>) => {
    const files = Array.from<File>(e.target.files || []);
    const newItems = files.map(file => ({
      id: createLocalAttachmentId(),
      url: URL.createObjectURL(file),
      name: file.name,
      file
    }));
    setAttachments(prev => [...prev, ...newItems]);
    e.target.value = '';
  };

  const startCamera = async () => {
    setIsCameraOpen(true);
    try {
      const stream = await navigator.mediaDevices.getUserMedia({ video: true });
      streamRef.current = stream;
      if (videoRef.current) {
        videoRef.current.srcObject = stream;
      }
    } catch (err) {
      toast('无法访问摄像头，请检查权限。', 'error');
      setIsCameraOpen(false);
    }
  };

  const stopCamera = () => {
    if (streamRef.current) {
      streamRef.current.getTracks().forEach(track => track.stop());
      streamRef.current = null;
    }
    setIsCameraOpen(false);
  };

  const captureFace = () => {
    if (videoRef.current && canvasRef.current) {
      const ctx = canvasRef.current.getContext('2d');
      if (ctx) {
        canvasRef.current.width = videoRef.current.videoWidth;
        canvasRef.current.height = videoRef.current.videoHeight;
        ctx.drawImage(videoRef.current, 0, 0);
        const capturedFaceImage = canvasRef.current.toDataURL('image/jpeg');
        setFaceImage(capturedFaceImage);
        setFaceImageDataUrl(capturedFaceImage);
        setCompareResult(null);
        stopCamera();
      }
    }
  };

  const handleCompare = () => {
    if (!faceImage || !idFront) return;
    setIsComparing(false);
    setCompareResult(null);
    toast('身份材料将在提交后由公证服务完成核验。', 'info');
  };

  const submitParty = () => {
    if (!newParty.name || !newParty.identityId) return;
    const submittedParty: Party = {
      ...newParty,
      id: party?.id || Date.now().toString(),
      identityFrontFile: idFrontFile ?? undefined,
      identityBackFile: idBackFile ?? undefined,
      faceImageDataUrl: faceImageDataUrl ?? undefined
    } as Party;
    onSave(submittedParty);
  };

  return (
    <AnimatePresence>
      {isOpen && (
        <>
          <motion.div 
            initial={{ opacity: 0 }}
            animate={{ opacity: 1 }}
            exit={{ opacity: 0 }}
            onClick={onClose}
            className="fixed inset-0 bg-black/60 backdrop-blur-sm z-[200]"
          />
          <motion.div 
            initial={{ x: '100%' }}
            animate={{ x: 0 }}
            exit={{ x: '100%' }}
            transition={{ type: 'spring', damping: 25, stiffness: 200 }}
            className="fixed right-0 top-0 bottom-0 w-[640px] bg-[#222224] border-l border-white/5 z-[210] flex flex-col shadow-2xl"
          >
            <div className="flex justify-between items-center p-6 border-b border-white/5 bg-[#2b2b2d] shrink-0">
              <h3 className="text-lg font-medium text-gray-200">
                {readOnly ? '查看当事人信息' : (party ? '编辑当事人' : '添加当事人')}
              </h3>
              <button onClick={onClose} className="text-gray-400 hover:text-white p-1 rounded-full hover:bg-white/10 transition-colors">
                <span className="sr-only">Close</span>
                <X size={20} />
              </button>
            </div>
            
            <div className="flex-1 overflow-y-auto custom-scrollbar p-6 flex flex-col gap-8">
               
               {/* 模块1: 身份验证与人脸比对 */}
               <div className="flex flex-col gap-4">
                 <h4 className="text-sm font-semibold text-gray-300 flex items-center gap-2"><ShieldCheck size={16} className="text-indigo-400" /> 身份验证鉴权</h4>
                 <div className="grid grid-cols-3 gap-4">
                    <div className="flex flex-col gap-2 relative">
                     <input type="file" accept="image/*" className="hidden" ref={idFrontRef} disabled={readOnly} onChange={e => handleFileUpload(e, setIdFront)} />
                     {idFront ? (
                       <div className="border border-white/10 bg-black/50 rounded-xl relative overflow-hidden aspect-[1.58/1] group">
                         <img src={idFront} alt="人像面" className="w-full h-full object-contain" />
                         {!readOnly && <button onClick={() => { setIdFront(null); setIdFrontFile(null); setCompareResult(null); }} className="absolute top-1 right-1 p-1 bg-red-500/80 text-white rounded-full opacity-0 group-hover:opacity-100 transition-opacity"><Trash2 size={12}/></button>}
                       </div>
                     ) : (
                       <div onClick={() => !readOnly && idFrontRef.current?.click()} className={`border border-dashed border-white/10 bg-[#181818]/50 rounded-xl flex flex-col items-center justify-center p-4 text-gray-500 ${readOnly ? 'cursor-not-allowed opacity-50' : 'cursor-pointer hover:border-white/30 hover:bg-white/5'} transition-colors aspect-[1.58/1]`}>
                         <Plus size={20} className="mb-1" />
                         <span className="text-[11px] font-medium">人像面</span>
                       </div>
                     )}
                   </div>
                   <div className="flex flex-col gap-2 relative">
                     <input type="file" accept="image/*" className="hidden" ref={idBackRef} disabled={readOnly} onChange={e => handleFileUpload(e, setIdBack)} />
                     {idBack ? (
                       <div className="border border-white/10 bg-black/50 rounded-xl relative overflow-hidden aspect-[1.58/1] group">
                         <img src={idBack} alt="国徽面" className="w-full h-full object-contain" />
                         {!readOnly && <button onClick={() => { setIdBack(null); setIdBackFile(null); }} className="absolute top-1 right-1 p-1 bg-red-500/80 text-white rounded-full opacity-0 group-hover:opacity-100 transition-opacity"><Trash2 size={12}/></button>}
                       </div>
                     ) : (
                       <div onClick={() => !readOnly && idBackRef.current?.click()} className={`border border-dashed border-white/10 bg-[#181818]/50 rounded-xl flex flex-col items-center justify-center p-4 text-gray-500 ${readOnly ? 'cursor-not-allowed opacity-50' : 'cursor-pointer hover:border-white/30 hover:bg-white/5'} transition-colors aspect-[1.58/1]`}>
                         <Plus size={20} className="mb-1" />
                         <span className="text-[11px] font-medium">国徽面</span>
                       </div>
                     )}
                   </div>
                   <div className="flex flex-col gap-2 relative">
                     {faceImage ? (
                       <div className="border border-indigo-500/30 bg-black/50 rounded-xl relative overflow-hidden aspect-[1.58/1] group">
                         <img src={faceImage} alt="现场人脸" className="w-full h-full object-cover" />
                         {!readOnly && <button onClick={() => { setFaceImage(null); setFaceImageDataUrl(null); setCompareResult(null); }} className="absolute top-1 right-1 p-1 bg-red-500/80 text-white rounded-full opacity-0 group-hover:opacity-100 transition-opacity"><Trash2 size={12}/></button>}
                       </div>
                     ) : (
                       <div onClick={() => !readOnly && startCamera()} className={`border ${readOnly ? 'border-dashed border-white/10 bg-white/5 text-gray-600 cursor-not-allowed' : 'border-dashed border-indigo-500/30 bg-indigo-500/5 cursor-pointer hover:border-indigo-500/50 hover:bg-indigo-500/10 text-indigo-400'} rounded-xl flex flex-col items-center justify-center p-4 transition-colors aspect-[1.58/1] relative`}>
                         <Camera size={20} className="mb-1" />
                         <span className="text-[11px] font-medium">现场人脸采集</span>
                       </div>
                     )}
                   </div>
                 </div>
                 <div className="bg-[#181818] border border-white/5 p-3 rounded-lg flex items-center justify-between">
                    <span className="text-xs text-gray-400">AI 活体人脸比对结果</span>
                    <div className="flex items-center gap-2">
                      {isComparing ? (
                        <span className="text-xs text-indigo-400 flex items-center gap-1">
                          <Loader2 size={12} className="animate-spin" /> 比对中...
                        </span>
                      ) : compareResult !== null ? (
                        <>
                          <div className="w-16 h-1.5 bg-white/10 rounded-full overflow-hidden">
                            <div className="h-full bg-green-500 transition-all duration-500" style={{ width: `${compareResult}%` }} />
                          </div>
                          <span className="text-xs font-mono text-green-500 flex items-center gap-1">{compareResult}% <span className="opacity-80">(匹配通过)</span></span>
                        </>
                      ) : (
                        <button 
                          disabled={!faceImage || !idFront || readOnly} 
                          onClick={handleCompare}
                          title={!idFront ? '请先上传人像面供AI比对' : ''}
                          className="px-3 py-1 bg-indigo-500/10 text-indigo-400 hover:bg-indigo-500/20 disabled:opacity-50 disabled:cursor-not-allowed text-xs font-medium rounded transition-colors"
                        >
                          点击进行比对
                        </button>
                      )}
                    </div>
                 </div>
               </div>

               <div className="h-[1px] bg-white/5 w-full"></div>

               {/* 模块2: 基础属性 */}
               <div className="flex flex-col gap-4">
                 <h4 className="text-sm font-semibold text-gray-300 flex items-center gap-2"><UserIcon size={16} className="text-indigo-400" /> 基础信息设定</h4>
                 <div className="grid grid-cols-2 gap-x-4 gap-y-4">
                   <div className="flex flex-col gap-2 col-span-2">
                     <label className="text-xs font-medium text-gray-400">联系手机 <span className="text-red-500">*</span></label>
                     <input type="text" readOnly={readOnly} value={newParty.phone} onChange={e => setNewParty({...newParty, phone: e.target.value})} placeholder="11位号码" className="w-full bg-[#181818] border border-white/10 rounded-lg px-3 py-2 text-sm text-gray-200 outline-none focus:border-indigo-500 font-mono read-only:bg-white/5 read-only:border-transparent read-only:text-gray-400" />
                   </div>
                   <div className="flex flex-col gap-2">
                     <label className="text-xs font-medium text-gray-400">姓名 <span className="text-red-500">*</span></label>
                     <input type="text" readOnly={readOnly} value={newParty.name} onChange={e => setNewParty({...newParty, name: e.target.value})} placeholder="识别或输入..." className="w-full bg-[#181818] border border-white/10 rounded-lg px-3 py-2 text-sm text-gray-200 outline-none focus:border-indigo-500 read-only:bg-white/5 read-only:border-transparent read-only:text-gray-400" />
                   </div>
                   <div className="flex flex-col gap-2">
                     <label className="text-xs font-medium text-gray-400">身份证号 <span className="text-red-500">*</span></label>
                     <input type="text" readOnly={readOnly} value={newParty.identityId} onChange={e => setNewParty({...newParty, identityId: e.target.value})} placeholder="识别或输入..." className="w-full bg-[#181818] border border-white/10 rounded-lg px-3 py-2 text-sm text-gray-200 outline-none focus:border-indigo-500 font-mono read-only:bg-white/5 read-only:border-transparent read-only:text-gray-400" />
                   </div>
                   <div className="flex flex-col gap-2">
                     <label className="text-xs font-medium text-gray-400">性别</label>
                     <select disabled={readOnly} value={newParty.gender} onChange={e => setNewParty({...newParty, gender: e.target.value})} className="w-full bg-[#181818] border border-white/10 rounded-lg px-3 py-2 text-sm text-gray-200 outline-none focus:border-indigo-500 disabled:opacity-75 disabled:bg-white/5">
                       <option value="男">男</option>
                       <option value="女">女</option>
                     </select>
                   </div>
                   <div className="flex flex-col gap-2">
                     <label className="text-xs font-medium text-gray-400">出生日期</label>
                     <input type="date" readOnly={readOnly} value={newParty.birthDate} onChange={e => setNewParty({...newParty, birthDate: e.target.value})} className="w-full bg-[#181818] border border-white/10 rounded-lg px-3 py-2 text-sm text-gray-200 outline-none focus:border-indigo-500 [color-scheme:dark] read-only:bg-white/5 read-only:border-transparent read-only:text-gray-400" />
                   </div>
                   <div className="flex flex-col gap-2 col-span-2">
                     <label className="text-xs font-medium text-gray-400">身份证有效起始日期</label>
                     <input type="date" readOnly={readOnly} value={newParty.identityValidDateStart} onChange={e => setNewParty({...newParty, identityValidDateStart: e.target.value})} className="w-full bg-[#181818] border border-white/10 rounded-lg px-3 py-2 text-sm text-gray-200 outline-none focus:border-indigo-500 [color-scheme:dark] read-only:bg-white/5 read-only:border-transparent read-only:text-gray-400" />
                   </div>
                   <div className="flex flex-col gap-2 col-span-2">
                     <label className="text-xs font-medium text-gray-400">身份证有效结束日期</label>
                     <input type="date" readOnly={readOnly} value={newParty.identityValidDateEnd} onChange={e => setNewParty({...newParty, identityValidDateEnd: e.target.value})} className="w-full bg-[#181818] border border-white/10 rounded-lg px-3 py-2 text-sm text-gray-200 outline-none focus:border-indigo-500 [color-scheme:dark] read-only:bg-white/5 read-only:border-transparent read-only:text-gray-400" />
                   </div>
                   <div className="flex flex-col gap-2 col-span-2">
                     <label className="text-xs font-medium text-gray-400">常住地址 (户籍/现住址)</label>
                     <input type="text" readOnly={readOnly} value={newParty.address} onChange={e => setNewParty({...newParty, address: e.target.value})} placeholder="详细地址..." className="w-full bg-[#181818] border border-white/10 rounded-lg px-3 py-2 text-sm text-gray-200 outline-none focus:border-indigo-500 read-only:bg-white/5 read-only:border-transparent read-only:text-gray-400" />
                   </div>
                 </div>
               </div>

               <div className="h-[1px] bg-white/5 w-full"></div>

               {/* 模块3: 扩展信息与附件 */}
               <div className="flex flex-col gap-4">
                 <h4 className="text-sm font-semibold text-gray-300 flex items-center gap-2"><FileText size={16} className="text-indigo-400" /> 辅助材料与备注</h4>
                 <div className="flex flex-col gap-2">
                   <label className="text-xs font-medium text-gray-400">备注说明</label>
                   <textarea readOnly={readOnly} value={newParty.remarks} onChange={e => setNewParty({...newParty, remarks: e.target.value})} placeholder="当前当事人的其他情况说明（如：行动不便、需要上门服务等）..." className="w-full h-20 bg-[#181818] border border-white/10 rounded-lg px-3 py-2 text-sm text-gray-200 outline-none focus:border-indigo-500 resize-none custom-scrollbar read-only:bg-white/5 read-only:border-transparent read-only:text-gray-400" />
                 </div>
                 <div className="flex flex-col gap-2 mt-2">
                   <label className="text-xs font-medium text-gray-400">其他证明附件 (如亲属关系证明、授权书等)</label>
                   <input disabled={readOnly} type="file" multiple className="hidden" ref={attachRef} onChange={handleAttachUpload} accept="image/*,application/pdf" />
                   
                   {attachments.length > 0 && (
                     <div className="flex flex-col gap-2 mb-2">
                       {attachments.map(att => (
                          <div key={att.id} className="flex items-center justify-between p-2.5 bg-[#181818] border border-white/10 rounded-lg group hover:border-white/20 transition-colors">
                             <div className="flex items-center gap-3 min-w-0">
                                <div className="w-10 h-10 rounded shrink-0 bg-black/50 border border-white/5 flex items-center justify-center overflow-hidden p-0.5">
                                   <img src={att.url} alt={att.name} className="w-full h-full object-contain" />
                                </div>
                                <span className="text-sm text-gray-300 truncate">{att.name}</span>
                             </div>
                             {!readOnly && (
                               <button 
                                 onClick={() => setAttachments(prev => prev.filter(p => p.id !== att.id))} 
                                 className="p-2 text-gray-500 hover:text-red-400 hover:bg-red-400/10 rounded-lg transition-colors shrink-0"
                                 title="删除附件"
                               >
                                 <Trash2 size={16} />
                               </button>
                             )}
                          </div>
                       ))}
                     </div>
                   )}
                   
                   {!readOnly && (
                     <div onClick={() => attachRef.current?.click()} className="border border-dashed border-white/10 bg-[#181818]/50 rounded-xl flex flex-col items-center justify-center p-6 text-gray-500 cursor-pointer hover:border-white/30 hover:bg-white/5 transition-colors">
                       <Plus size={20} className="mb-2" />
                       <span className="text-[12px]">拖拽或点击上传附加材料</span>
                     </div>
                   )}
                 </div>
               </div>

            </div>

            <div className="p-6 border-t border-white/5 bg-[#2b2b2d] shrink-0 flex justify-between items-center">
               <div>
                  {party && onSign && !readOnly && (
                    <button onClick={() => { onClose(); onSign(party as Party); }} className="px-4 py-2 rounded-lg text-sm text-yellow-400 bg-yellow-400/10 hover:bg-yellow-400/20 transition-colors border border-yellow-400/20 flex items-center gap-2">
                       <PenTool size={16} /> 在线签名
                    </button>
                  )}
               </div>
               <div className="flex justify-end gap-3">
                 <button onClick={onClose} className="px-6 py-2 rounded-lg text-sm text-gray-300 border border-white/10 hover:bg-white/5 transition-colors">
                   {readOnly ? '关闭' : '取消'}
                 </button>
                 {!readOnly && (
                   <button onClick={submitParty} disabled={!newParty.name || !newParty.identityId} className="px-6 py-2 rounded-lg text-sm text-white bg-indigo-600 hover:bg-indigo-700 disabled:opacity-50 transition-colors shadow-md">
                     {party ? '保存记录' : '保存并添加'}
                   </button>
                 )}
               </div>
            </div>
          </motion.div>
          
          {/* Camera Modal */}
          <AnimatePresence>
            {isCameraOpen && (
              <motion.div initial={{opacity: 0}} animate={{opacity: 1}} exit={{opacity: 0}} className="fixed inset-0 z-[220] bg-black/95 flex flex-col">
                <div className="flex justify-between items-center p-4">
                  <h3 className="text-white font-medium">现场人脸采集</h3>
                  <button onClick={stopCamera} className="text-gray-400 hover:text-white"><X size={24}/></button>
                </div>
                <div className="flex-1 flex justify-center items-center overflow-hidden relative">
                  <video ref={videoRef} autoPlay playsInline muted className="h-full w-full object-cover max-w-4xl mx-auto rounded-lg shadow-2xl scale-x-[-1]" />
                  <canvas ref={canvasRef} className="hidden" />
                  
                  <div className="absolute inset-0 pointer-events-none flex justify-center items-center">
                     <div className="w-[300px] h-[400px] border-2 border-dashed border-indigo-500/50 rounded-full flex items-center justify-center shadow-[0_0_0_9999px_rgba(0,0,0,0.5)]">
                       <div className="absolute bottom-4 left-0 right-0 text-center text-sm text-white/70">请将面部对准框内</div>
                     </div>
                  </div>
                </div>
                <div className="p-8 flex justify-center">
                  <button onClick={captureFace} className="w-16 h-16 rounded-full border-4 border-white bg-indigo-500 hover:bg-indigo-600 transition-colors shadow-lg active:scale-95" />
                </div>
              </motion.div>
            )}
          </AnimatePresence>
        </>
      )}
    </AnimatePresence>
  );
};
