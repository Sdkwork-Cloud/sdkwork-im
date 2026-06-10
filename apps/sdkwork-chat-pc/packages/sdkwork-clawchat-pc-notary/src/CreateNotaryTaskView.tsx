import React, { useState, useRef, useEffect } from 'react';
import { ChevronLeft, ChevronRight, Check, Plus, Video, Trash2, User as UserIcon, ShieldCheck, FileText, Camera, Loader2, X, Edit, Folder, Cloud, Upload, QrCode, Smartphone, PenTool } from 'lucide-react';
import { motion, AnimatePresence } from 'motion/react';
import { cn } from '@sdkwork/clawchat-pc-commons';
import { CallOverlay, createDefaultAvatar, toast } from '@sdkwork/clawchat-pc-chat';
import { Party, NotaryDocument } from '@sdkwork/clawchat-pc-types';
import { notaryService } from './services/NotaryService';

import { PartyDrawer } from './PartyDrawer';

import { SignaturePad } from './SignaturePad';

const DEFAULT_NOTARY_CALLER_AVATAR = createDefaultAvatar('user');

export const CreateNotaryTaskView: React.FC<{ onBack: () => void, onSuccess?: () => void }> = ({ onBack, onSuccess }) => {
  const [step, setStep] = useState(1);
  const [businessType, setBusinessType] = useState('');
  const [notary, setNotary] = useState('');

  const [notaryStaffMembers, setNotaryStaffMembers] = useState<Array<{ membershipId: string; userId?: string; displayName: string; status?: string; roles?: string[]; positions?: string[]; departments?: string[]; notaryStaffRole?: string }>>([]);

  const [selectedNotaryStaff, setSelectedNotaryStaff] = useState<{ membershipId: string; userId?: string; displayName: string; status?: string; roles?: string[]; positions?: string[]; departments?: string[]; notaryStaffRole?: string } | null>(null);


  useEffect(() => {

    let disposed = false;

    notaryService.getStaff({ staffRole: 'notary' })

      .then((staff) => {

        if (!disposed) {

          setNotaryStaffMembers(staff);

        }

      })

      .catch((error) => {

        console.error('Failed to load notary staff', error);

      });

    return () => {

      disposed = true;

    };

  }, []);
  const [showNotaryDrawer, setShowNotaryDrawer] = useState(false);
  const [parties, setParties] = useState<Party[]>([]);
  const [appInfo, setAppInfo] = useState('');
  const [isSubmitting, setIsSubmitting] = useState(false);
  
  const [showAddPartyPanel, setShowAddPartyPanel] = useState(false);
  const [isPartyReadOnly, setIsPartyReadOnly] = useState(false);
  const [editingPartyId, setEditingPartyId] = useState<string | null>(null);
  // Video Call States
  const [activeCall, setActiveCall] = useState<{isOpen: boolean, name: string}>({ isOpen: false, name: '' });
  const [activeQrCodeParty, setActiveQrCodeParty] = useState<Party | null>(null);
  
  // Attachments
  const [attachments, setAttachments] = useState<{ id: string; name: string; url: string; type: string; file: File; partyId?: string }[]>([]);
  const taskAttachRef = useRef<HTMLInputElement>(null);
  const partyDriveUploadRef = useRef<HTMLInputElement>(null);
  const [activePreview, setActivePreview] = useState<{ url: string; type: string; name: string } | null>(null);
  
  const handleTaskAttachmentUpload = (e: React.ChangeEvent<HTMLInputElement>) => {
    const files = Array.from<File>(e.target.files || []);
    const newItems = files.map(file => {
      const isVideo = file.type.startsWith('video/');
      const isPdf = file.type === 'application/pdf';
      return {
        id: Date.now() + Math.random().toString(),
        url: URL.createObjectURL(file),
        name: file.name,
        type: isVideo ? 'video' : isPdf ? 'pdf' : 'image',
        file
      };
    });
    setAttachments(prev => [...prev, ...newItems]);
    e.target.value = '';
  };
  
  // Party Drive Directory
  const [activeDriveParty, setActiveDriveParty] = useState<Party | null>(null);
  const partyDriveDocuments = attachments.filter((attachment) => attachment.partyId === activeDriveParty?.id);

  const handlePartyDriveAttachmentUpload = (e: React.ChangeEvent<HTMLInputElement>) => {
    if (!activeDriveParty?.id) {
      e.target.value = '';
      return;
    }
    const files = Array.from<File>(e.target.files || []);
    const newItems = files.map(file => {
      const isVideo = file.type.startsWith('video/');
      const isPdf = file.type === 'application/pdf';
      return {
        id: Date.now() + Math.random().toString(),
        url: URL.createObjectURL(file),
        name: file.name,
        type: isVideo ? 'video' : isPdf ? 'pdf' : 'image',
        file,
        partyId: activeDriveParty.id
      };
    });
    setAttachments(prev => [...prev, ...newItems]);
    e.target.value = '';
  };

  const [activeSignParty, setActiveSignParty] = useState<Party | null>(null);

  const handleAddParty = () => {
    setEditingPartyId(null);
    setIsPartyReadOnly(false);
    setShowAddPartyPanel(true);
  };
  
  const handleEditParty = (party: Party) => {
    setEditingPartyId(party.id);
    setIsPartyReadOnly(false);
    setShowAddPartyPanel(true);
  };

  const handleViewParty = (party: Party) => {
    setEditingPartyId(party.id);
    setIsPartyReadOnly(true);
    setShowAddPartyPanel(true);
  };

  const handleSaveParty = (partyData: Party) => {
    if (editingPartyId) {
      setParties(parties.map(p => p.id === editingPartyId ? partyData : p));
    } else {
      setParties([...parties, partyData]);
      setActiveSignParty(partyData);
    }
    setShowAddPartyPanel(false);
    setEditingPartyId(null);
  };

  const handleRemoveParty = (id: string) => {
    setParties(parties.filter(p => p.id !== id));
  };

  const handleVideoCall = (party: Party) => {
    if (!notary || notary === '系统分配' || notary === '未分配') {
      toast('请先选择承办公证员后再发起视频通话。', 'error');
      return;
    }
    // Enter IM video call between current logged in user and the selected notary
    setActiveCall({ isOpen: true, name: notary });
  };

  const nextStep = () => setStep(s => Math.min(s + 1, 4));
  const prevStep = () => setStep(s => Math.max(s - 1, 1));

  if (activeSignParty) {
    return (
      <SignaturePad 
        title={`当事人在线签名 - ${activeSignParty.name}`}
        subtitle={<><PenTool size={16} /> 请当事人 <span className="text-indigo-400 font-medium">"{activeSignParty.name}"</span> 使用正楷书写姓名</>}
        onCancel={() => setActiveSignParty(null)}
        onSave={(imgUrl) => {
          toast('签名已保存', 'success');
          setParties(parties.map(p => p.id === activeSignParty.id ? { ...p, signatureUrl: imgUrl } : p));
          setActiveSignParty(null);
        }}
      />
    );
  }

  return (
    <div className="flex w-full h-full flex-col bg-[#1e1e1e] relative min-h-0 min-w-0">
      <div className="flex items-center px-6 min-h-[64px] border-b border-white/5 bg-[#181818] shrink-0">
        <button onClick={onBack} className="flex items-center text-gray-400 hover:text-gray-100 transition-colors">
          <ChevronLeft size={20} className="mr-1" />
          <span>返回</span>
        </button>
        <div className="mx-auto text-lg font-medium text-gray-200">新建公证业务</div>
        <div className="w-[60px]" /> {/* Spacer for centering */}
      </div>

      <div className="flex-1 overflow-y-auto custom-scrollbar p-6 lg:p-8 flex flex-col items-center">
        <div className="w-full h-full flex flex-col max-w-[1600px]">
          
          {/* Progress Steps */}
          <div className="flex items-center w-full mb-12">
            {[1, 2, 3, 4].map((i) => (
              <React.Fragment key={i}>
                <div className="flex flex-col items-center relative z-10 w-24">
                  <div className={cn(
                    "w-10 h-10 rounded-full flex items-center justify-center font-medium text-sm transition-colors border-2",
                    step > i ? "bg-indigo-600 border-indigo-600 text-white" : step === i ? "bg-indigo-600/20 border-indigo-500 text-indigo-400 font-bold" : "bg-[#2b2b2d] border-white/10 text-gray-500"
                  )}>
                    {step > i ? <Check size={18} /> : i}
                  </div>
                  <div className={cn(
                    "mt-3 text-sm font-medium whitespace-nowrap",
                    step >= i ? "text-gray-200" : "text-gray-500"
                  )}>
                    {i === 1 && "选择业务"}
                    {i === 2 && "当事人"}
                    {i === 3 && "填写信息"}
                    {i === 4 && "确认完成"}
                  </div>
                </div>
                {i < 4 && (
                  <div className="flex-1 h-[2px] mx-2 bg-[#2b2b2d] relative -top-[12px]">
                    <div 
                      className="absolute left-0 top-0 bottom-0 bg-indigo-600 transition-all duration-300" 
                      style={{ width: step > i ? '100%' : '0%' }}
                    />
                  </div>
                )}
              </React.Fragment>
            ))}
          </div>

          {/* Form Area */}
          <div className="w-full bg-[#2b2b2d] rounded-xl border border-white/5 p-8 shadow-sm flex flex-col">
            
            {/* Step 1 */}
            {step === 1 && (
              <motion.div initial={{ opacity: 0, y: 10 }} animate={{ opacity: 1, y: 0 }} className="flex flex-col gap-6">
                <h3 className="text-xl font-medium text-gray-200 mb-2">选择公证业务类型</h3>
                <div className="grid grid-cols-2 gap-4">
                  {['电子合同存证', '知识产权确权公证', '电子证据固化', '商业秘密确权', '抽奖过程摇号公证', '遗嘱公证'].map(type => (
                    <div 
                      key={type}
                      onClick={() => setBusinessType(type)}
                      className={cn(
                        "p-5 rounded-xl border cursor-pointer transition-all flex flex-col gap-2 relative overflow-hidden",
                        businessType === type ? "bg-indigo-500/10 border-indigo-500 text-indigo-400" : "bg-[#181818] border-white/5 text-gray-300 hover:border-white/20 hover:bg-[#202020]"
                      )}
                    >
                      <div className="font-medium text-[16px]">{type}</div>
                      <div className="text-xs text-gray-500">适用于企业和个人的标准公证流程...</div>
                      {businessType === type && (
                        <div className="absolute top-0 right-0 w-0 h-0 border-t-[24px] border-r-[24px] border-t-indigo-500 border-r-transparent">
                          <Check size={12} className="absolute -top-[20px] -right-[5px] text-white" />
                        </div>
                      )}
                    </div>
                  ))}
                </div>
              </motion.div>
            )}

            {/* Step 2 */}
            {step === 2 && (
              <motion.div initial={{ opacity: 0, y: 10 }} animate={{ opacity: 1, y: 0 }} className="flex flex-col gap-6">
                <h3 className="text-xl font-medium text-gray-200 mb-2">选择公证员与绑定当事人</h3>
                
                <div className="flex flex-col gap-2 relative">
                  <label className="text-sm font-medium text-gray-400">选择承办公证员 <span className="text-red-500">*</span></label>
                  <button 
                    onClick={() => setShowNotaryDrawer(true)}
                    className="w-full bg-[#181818] border border-white/10 rounded-lg px-4 py-3 text-sm flex justify-between items-center text-gray-200 outline-none hover:border-white/20 transition-colors"
                  >
                    <span>{notary ? `已选公证员: ${notary}` : '点击选择承办公证员...'}</span>
                    <ChevronLeft className="rotate-180 text-gray-500" size={16} />
                  </button>
                  
                  {/* Notary Picker Drawer Overlay */}
                  <AnimatePresence>
                    {showNotaryDrawer && (
                      <>
                        <motion.div 
                          initial={{ opacity: 0 }}
                          animate={{ opacity: 1 }}
                          exit={{ opacity: 0 }}
                          onClick={() => setShowNotaryDrawer(false)}
                          className="fixed inset-0 bg-black/40 backdrop-blur-sm z-40"
                        />
                        <motion.div 
                          initial={{ x: '100%' }}
                          animate={{ x: 0 }}
                          exit={{ x: '100%' }}
                          transition={{ type: 'spring', damping: 25, stiffness: 200 }}
                          className="fixed right-0 top-0 bottom-0 w-[400px] bg-[#222224] border-l border-white/5 z-50 flex flex-col shadow-2xl"
                        >
                          <div className="flex justify-between items-center p-6 border-b border-white/5 bg-[#2b2b2d] shrink-0">
                            <h3 className="text-lg font-medium text-gray-200">选择公证员</h3>
                            <button onClick={() => setShowNotaryDrawer(false)} className="text-gray-400 hover:text-white p-1 rounded-full hover:bg-white/10 transition-colors">
                              <span className="sr-only">Close</span>
                              <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round"><path d="M18 6 6 18"/><path d="m6 6 12 12"/></svg>
                            </button>
                          </div>
                          
                          <div className="p-4 border-b border-white/5 shrink-0 bg-[#2b2b2d]">
                             <div className="relative">
                               <input 
                                 type="text" 
                                 placeholder="搜索姓名、拼音缩写..." 
                                 className="w-full bg-[#181818] border border-white/10 rounded-lg pl-9 pr-4 py-2 text-sm text-gray-200 outline-none focus:border-indigo-500"
                               />
                               <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round" className="absolute left-3 top-1/2 -translate-y-1/2 text-gray-500"><circle cx="11" cy="11" r="8"/><path d="m21 21-4.3-4.3"/></svg>
                             </div>
                          </div>

                          <div className="flex-1 overflow-y-auto custom-scrollbar">
                             <div className="px-4 py-2 flex flex-col gap-1">
                               {notaryStaffMembers.map((staff) => (
                                 <div
                                   key={staff.membershipId}
                                   onClick={() => {
                                     setSelectedNotaryStaff(staff);
                                     setNotary(staff.displayName);
                                     setShowNotaryDrawer(false);
                                   }}
                                   className="flex items-center gap-3 p-3 hover:bg-white/5 rounded-lg cursor-pointer transition-colors group"
                                 >
                                   <div className="w-10 h-10 rounded-full bg-indigo-500/20 text-indigo-400 flex items-center justify-center font-medium">{(staff.displayName || staff.membershipId).slice(0, 1).toUpperCase()}</div>
                                   <div>
                                     <div className="text-sm font-medium text-gray-200 group-hover:text-indigo-400 transition-colors">{staff.displayName}</div>
                                     <div className="text-xs text-gray-500">{[staff.notaryStaffRole, ...(staff.positions ?? []), ...(staff.departments ?? [])].filter(Boolean).join(' / ') || staff.status}</div>
                                   </div>
                                 </div>
                               ))}
                             </div>
                          </div>
                        </motion.div>
                      </>
                    )}
                  </AnimatePresence>
                </div>

                <div className="flex flex-col gap-4 mt-4">
                  <div className="flex justify-between items-center">
                    <label className="text-sm font-medium text-gray-400">当事人列表</label>
                    <button onClick={handleAddParty} className="flex items-center gap-1.5 text-xs text-white bg-indigo-600 hover:bg-indigo-700 px-3 py-1.5 rounded-lg transition-colors">
                      <Plus size={14} /> 添加当事人
                    </button>
                  </div>
                  
                  {parties.length === 0 ? (
                    <div className="py-8 text-center border border-dashed border-white/10 rounded-xl text-sm text-gray-500 bg-[#181818]/50">
                      尚未添加任何当事人，请点击上方按钮添加
                    </div>
                  ) : (
                    <div className="flex flex-col gap-3">
                      <AnimatePresence>
                        {parties.map(party => (
                          <motion.div 
                            key={party.id}
                            initial={{ opacity: 0, height: 0 }}
                            animate={{ opacity: 1, height: 'auto' }}
                            exit={{ opacity: 0, height: 0 }}
                            onDoubleClick={() => handleEditParty(party)}
                            onClick={() => handleViewParty(party)}
                            className="bg-[#181818] border border-white/5 rounded-xl p-4 flex items-center justify-between transition-colors hover:border-white/10 cursor-pointer"
                          >
                            <div className="flex items-center gap-4">
                              <div className="w-10 h-10 rounded-full bg-indigo-500/10 flex items-center justify-center text-indigo-400">
                                <UserIcon size={20} />
                              </div>
                              <div>
                                <div className="text-gray-200 font-medium text-sm flex items-center gap-2">
                                  {party.name} <span className="px-2 py-0.5 rounded text-[10px] bg-white/10 text-gray-400">{party.role}</span>
                                </div>
                                <div className="text-gray-500 text-xs mt-1">身份证号: {party.identityId}</div>
                              </div>
                            </div>
                            <div className="flex items-center gap-2" onClick={e => e.stopPropagation()}>
                              {party.signatureUrl && (
                                <div className="px-2 py-1 bg-teal-500/10 text-teal-400 rounded-lg text-xs font-medium border border-teal-500/20 flex items-center gap-1">
                                  <Check size={14} /> 已签名
                                </div>
                              )}
                              <button onClick={() => setActiveSignParty(party)} className="p-2 bg-orange-500/10 text-orange-400 hover:bg-orange-500/20 rounded-lg transition-colors flex items-center gap-1.5 text-xs font-medium border border-orange-500/20">
                                <PenTool size={14} /> 签名
                              </button>
                              <button onClick={() => setActiveDriveParty(party)} className="p-2 bg-blue-500/10 text-blue-400 hover:bg-blue-500/20 rounded-lg transition-colors flex items-center gap-1.5 text-xs font-medium border border-blue-500/20">
                                <Folder size={14} /> 附件上传 (网盘)
                              </button>
                              <button onClick={() => setActiveQrCodeParty(party)} className="p-2 bg-purple-500/10 text-purple-400 hover:bg-purple-500/20 rounded-lg transition-colors flex items-center gap-1.5 text-xs font-medium border border-purple-500/20">
                                <QrCode size={14} /> 视频通话二维码
                              </button>
                              <button onClick={() => handleVideoCall(party)} className="p-2 bg-green-500/10 text-green-500 hover:bg-green-500/20 rounded-lg transition-colors flex items-center gap-1.5 text-xs font-medium border border-green-500/20">
                                <Video size={14} /> 立即通话
                              </button>
                              <button onClick={() => handleEditParty(party)} className="p-2 bg-indigo-500/10 text-indigo-400 hover:bg-indigo-500/20 rounded-lg transition-colors border border-indigo-500/20" title="编辑">
                                <Edit size={16} />
                              </button>
                              <button onClick={() => handleRemoveParty(party.id)} className="p-2 bg-red-500/10 text-red-500 hover:bg-red-500/20 rounded-lg transition-colors border border-red-500/20" title="移除">
                                <Trash2 size={16} />
                              </button>
                            </div>
                          </motion.div>
                        ))}
                      </AnimatePresence>
                    </div>
                  )}
                </div>

              </motion.div>
            )}

            {/* Step 3 */}
            {step === 3 && (
              <motion.div initial={{ opacity: 0, y: 10 }} animate={{ opacity: 1, y: 0 }} className="flex flex-col gap-6">
                <h3 className="text-xl font-medium text-gray-200 mb-2">填写详细申请信息</h3>
                <div className="flex flex-col gap-2">
                  <label className="text-sm font-medium text-gray-400">业务描述或申请事由</label>
                  <textarea 
                    value={appInfo}
                    onChange={(e) => setAppInfo(e.target.value)}
                    placeholder="请输入详细的业务背景、目的以及申请公证的具体诉求..."
                    className="w-full h-32 bg-[#181818] border border-white/10 rounded-lg p-4 text-sm text-gray-200 outline-none focus:border-indigo-500 focus:ring-1 focus:ring-indigo-500 resize-none custom-scrollbar"
                  />
                </div>
                <div className="flex flex-col gap-2 mt-4">
                  <div className="flex justify-between items-center">
                    <label className="text-sm font-medium text-gray-400">上传附件办理材料 (选填)</label>
                    {attachments.length > 0 && (
                      <button onClick={() => taskAttachRef.current?.click()} className="flex items-center gap-1.5 text-xs text-indigo-400 bg-indigo-500/10 hover:bg-indigo-500/20 px-3 py-1.5 rounded-lg transition-colors border border-indigo-500/20">
                        <Plus size={14} /> 继续添加文件
                      </button>
                    )}
                  </div>
                  <input type="file" multiple className="hidden" ref={taskAttachRef} onChange={handleTaskAttachmentUpload} accept="image/*,video/*,application/pdf" />

                  {attachments.length > 0 ? (
                    <div className="flex flex-col gap-2 mt-2">
                       <div className="border border-white/10 rounded-xl overflow-hidden bg-[#181818]/80">
                         {attachments.map((att, idx) => (
                           <div key={att.id} className={`flex items-center justify-between p-3 hover:bg-white/5 transition-colors group ${idx !== attachments.length - 1 ? 'border-b border-white/5' : ''}`}>
                             <div 
                               className="flex items-center gap-4 flex-1 min-w-0 cursor-pointer"
                               onClick={() => (att.type === 'image' || att.type === 'video') ? setActivePreview(att) : window.open(att.url, '_blank')}
                             >
                                <div className="w-10 h-10 rounded-lg bg-[#2b2b2d] flex items-center justify-center shrink-0 border border-white/5">
                                  {att.type === 'image' ? <Camera size={18} className="text-indigo-400" /> : att.type === 'video' ? <Video size={18} className="text-indigo-400" /> : <FileText size={18} className="text-indigo-400" />}
                                </div>
                                <div className="flex flex-col min-w-0">
                                  <span className="text-sm font-medium text-gray-200 truncate group-hover:text-indigo-400 transition-colors">{att.name}</span>
                                  <span className="text-xs text-gray-500 mt-0.5 uppercase">{att.type} • 待上传</span>
                                </div>
                             </div>
                             <button 
                                onClick={(e) => { e.stopPropagation(); setAttachments(prev => prev.filter(p => p.id !== att.id)); }} 
                                className="p-2 text-gray-400 hover:text-red-400 hover:bg-red-500/10 rounded-lg transition-colors shrink-0 "
                                title="删除附件"
                              >
                                <Trash2 size={16} />
                              </button>
                           </div>
                         ))}
                       </div>
                    </div>
                  ) : (
                    <div onClick={() => taskAttachRef.current?.click()} className="border-2 border-dashed border-white/10 rounded-xl p-8 flex flex-col items-center justify-center text-gray-400 hover:border-indigo-500/50 hover:bg-indigo-500/5 hover:text-indigo-400 transition-colors cursor-pointer bg-[#181818]/50 mt-2">
                      <Upload size={24} className="mb-3" />
                      <span className="text-sm font-medium">点击此处上方文件，或将文件拖至此区域</span>
                      <span className="text-xs mt-2 text-gray-500 text-center max-w-sm">支持上传图片 (JPG, PNG)、视频 (MP4)、文档 (PDF) 等辅助证明材料，单文件最大不超过 50MB。</span>
                    </div>
                  )}
                </div>
              </motion.div>
            )}

            {/* Step 4 */}
            {step === 4 && (
              <motion.div initial={{ opacity: 0, y: 10 }} animate={{ opacity: 1, y: 0 }} className="flex flex-col gap-8">
                <div className="text-center">
                  <h3 className="text-xl font-medium text-gray-200 mb-2">确认业务信息</h3>
                  <p className="text-sm text-gray-500">请仔细核对以下电子卷宗和流转信息，确认无误后即可立项办理。</p>
                </div>
                <div className="bg-[#181818] border border-white/5 rounded-xl p-6 flex flex-col gap-4 text-sm">
                  <div className="flex">
                    <span className="w-24 text-gray-500">公证类型</span>
                    <span className="text-gray-200 font-medium">{businessType || '--'}</span>
                  </div>
                  <div className="flex">
                    <span className="w-24 text-gray-500">承办公证员</span>
                    <span className="text-gray-200">{notary || '--'}</span>
                  </div>
                  <div className="flex">
                    <span className="w-24 text-gray-500 mt-2">当事人</span>
                    <div className="flex flex-col gap-2 flex-1">
                      {parties.length === 0 ? <span className="text-gray-500 mt-2">--</span> : parties.map(p => (
                        <div 
                          key={p.id} 
                          onClick={() => handleViewParty(p)}
                          className="bg-white/5 border border-white/10 px-4 py-3 rounded-xl flex items-center justify-between cursor-pointer hover:bg-white/10 hover:border-indigo-500/30 transition-colors group"
                        >
                          <div className="flex flex-col gap-1.5">
                            <div className="flex items-center gap-2">
                              <span className="font-medium text-gray-200 group-hover:text-indigo-400 transition-colors">{p.name}</span>
                              <span className="text-xs text-indigo-400 bg-indigo-500/10 px-2 py-0.5 rounded border border-indigo-500/20">{p.role}</span>
                              {p.signatureUrl && (
                                <img src={p.signatureUrl} alt={`Signature for ${p.name}`} className="h-6 object-contain bg-white/90 rounded border border-white/20 ml-2" />
                              )}
                            </div>
                            <div className="text-xs text-gray-500 flex items-center gap-3">
                              <span>身份证: {p.identityId}</span>
                              <span>手机号: {p.phone}</span>
                            </div>
                          </div>
                          <ChevronRight size={18} className="text-gray-500 group-hover:text-indigo-400 transition-colors" />
                        </div>
                      ))}
                    </div>
                  </div>
                  <div className="flex">
                    <span className="w-24 text-gray-500 shrink-0">申请事由</span>
                    <span className="text-gray-300 bg-white/5 p-3 rounded-lg flex-1 min-h-[60px] line-clamp-3">{appInfo || '--'}</span>
                  </div>
                  {attachments.length > 0 && (
                    <div className="flex">
                      <span className="w-24 text-gray-500 shrink-0 mt-3">相关附件</span>
                      <div className="flex flex-col gap-3 flex-1 mt-1">
                        {attachments.map(att => (
                          <div 
                            key={att.id} 
                            className="bg-white/5 border border-white/10 p-3 rounded-xl flex items-center justify-between cursor-pointer hover:bg-white/10 hover:border-indigo-500/30 transition-colors group"
                            onClick={() => (att.type === 'image' || att.type === 'video') ? setActivePreview(att) : window.open(att.url, '_blank')}
                          >
                            <div className="flex items-center gap-4 min-w-0">
                              <div className="w-10 h-10 rounded-lg bg-[#2b2b2d] flex items-center justify-center shrink-0 border border-white/5 group-hover:border-indigo-500/30 transition-colors">
                                {att.type === 'image' ? <Camera size={16} className="text-indigo-400" /> : att.type === 'video' ? <Video size={16} className="text-indigo-400" /> : <FileText size={16} className="text-indigo-400" />}
                              </div>
                              <span className="text-sm text-gray-300 font-medium truncate group-hover:text-indigo-400 transition-colors">{att.name}</span>
                            </div>
                            <span className="text-xs text-gray-500 uppercase">{att.type}</span>
                          </div>
                        ))}
                      </div>
                    </div>
                  )}
                </div>
              </motion.div>
            )}

            {/* Nav Buttons */}
            <div className="flex items-center justify-between mt-12 pt-6 border-t border-white/5">
              <button 
                onClick={prevStep}
                disabled={step === 1}
                className="px-6 py-2.5 rounded-lg text-sm text-gray-300 font-medium hover:bg-white/10 disabled:opacity-30 disabled:hover:bg-transparent transition-colors"
              >
                上一步
              </button>
              
              {step < 4 ? (
                <button 
                  onClick={nextStep}
                  className="px-8 py-2.5 rounded-lg text-sm text-white font-medium bg-indigo-600 hover:bg-indigo-700 transition-colors shadow-md"
                >
                  下一步
                </button>
              ) : (
                <button 
                  onClick={async () => {
                    setIsSubmitting(true);
                    try {
                      await notaryService.createTask({
                        type: businessType,
                        notary: notary,
                        parties: parties,
                        remarks: appInfo,
                        title: businessType + '办理',
                        primaryNotaryMembershipId: selectedNotaryStaff?.membershipId,
                        documents: attachments.map((attachment) => ({
                          name: attachment.file.name,
                          size: `${Math.max(1, Math.round(attachment.file.size / 1024))} KB`,
                          category: 'evidence',
                          materialCode: attachment.file.name,
                          partyId: attachment.partyId,
                          file: attachment.file,
                        } as unknown as NotaryDocument))
                      });
                      if (onSuccess) {
                        onSuccess();
                      } else {
                        onBack();
                      }
                    } catch (error) {
                      console.error("Failed to create task", error);
                    } finally {
                      setIsSubmitting(false);
                    }
                  }}
                  disabled={isSubmitting}
                  className="px-8 py-2.5 rounded-lg text-sm text-white font-medium bg-green-600 hover:bg-green-700 transition-colors shadow-md flex items-center gap-2 disabled:opacity-50"
                >
                  {isSubmitting ? <Loader2 size={16} className="animate-spin" /> : <Check size={16} />}
                  {isSubmitting ? '提交中...' : '完成公证办理'}
                </button>
              )}
            </div>
            
            {/* Common UI Elements */}
            <CallOverlay 
              conversationId={`notary-${activeCall.name || 'video-call'}`}
              isOpen={activeCall.isOpen} 
              type="video" 
              callerName={activeCall.name} 
              callerAvatar={DEFAULT_NOTARY_CALLER_AVATAR}
              onClose={() => setActiveCall({ isOpen: false, name: '' })} 
            />

            {/* Drive Modal Simulation */}
            <AnimatePresence>
              {activePreview && (
                <div className="fixed inset-0 z-[300] flex items-center justify-center p-8">
                  <motion.div 
                    initial={{ opacity: 0 }} animate={{ opacity: 1 }} exit={{ opacity: 0 }}
                    onClick={() => setActivePreview(null)}
                    className="absolute inset-0 bg-black/80 backdrop-blur-md"
                  />
                  <motion.div 
                    initial={{ opacity: 0, scale: 0.95 }}
                    animate={{ opacity: 1, scale: 1 }}
                    exit={{ opacity: 0, scale: 0.95 }}
                    className="relative w-full max-w-4xl max-h-[90vh] bg-transparent flex flex-col items-center justify-center pointer-events-none"
                  >
                    <button onClick={() => setActivePreview(null)} className="absolute -top-12 right-0 text-white hover:text-gray-300 p-2 pointer-events-auto">
                      <X size={28} />
                    </button>
                    <div className="w-full max-h-[85vh] flex items-center justify-center pointer-events-auto rounded-lg overflow-hidden shadow-2xl bg-black border border-white/10">
                      {activePreview.type === 'image' ? (
                        <img src={activePreview.url} alt="Preview" className="max-w-full max-h-full object-contain" />
                      ) : activePreview.type === 'video' ? (
                        <video src={activePreview.url} controls autoPlay className="max-w-full max-h-full object-contain" />
                      ) : null}
                    </div>
                    {activePreview.name && (
                      <div className="absolute -bottom-10 left-0 right-0 text-center text-gray-300 text-sm font-medium">
                        {activePreview.name}
                      </div>
                    )}
                  </motion.div>
                </div>
              )}
            </AnimatePresence>

            <AnimatePresence>
              {activeDriveParty && (
                <div className="fixed inset-0 z-50 flex items-center justify-center p-8">
                  <motion.div 
                    initial={{ opacity: 0 }} animate={{ opacity: 1 }} exit={{ opacity: 0 }}
                    onClick={() => setActiveDriveParty(null)}
                    className="absolute inset-0 bg-black/60 backdrop-blur-sm"
                  />
                  <motion.div 
                    initial={{ opacity: 0, scale: 0.95 }}
                    animate={{ opacity: 1, scale: 1 }}
                    exit={{ opacity: 0, scale: 0.95 }}
                    className="relative w-full max-w-5xl h-full max-h-[800px] bg-[#1e1e1e] rounded-2xl shadow-2xl border border-white/10 flex flex-col overflow-hidden"
                  >
                    {/* Party staged file header */}
                    <input id="notary-party-create-drive-upload" type="file" multiple className="hidden" ref={partyDriveUploadRef} onChange={handlePartyDriveAttachmentUpload} accept="image/*,video/*,application/pdf" />
                    <div className="h-16 px-6 border-b border-white/5 bg-[#181818] flex items-center justify-between shrink-0">
                      <div className="flex items-center gap-3 text-lg font-medium text-gray-200">
                        <Cloud size={24} className="text-cyan-400" />
                        企业云盘
                        <span className="text-gray-500">/</span>
                        <span className="text-gray-400 font-normal">公证业务-当事人目录</span>
                        <span className="text-gray-500">/</span>
                        <span className="text-indigo-400">{activeDriveParty.name}</span>
                      </div>
                      <div className="flex items-center gap-4">
                        <button onClick={() => partyDriveUploadRef.current?.click()} className="bg-cyan-600 hover:bg-cyan-700 text-white font-medium px-4 py-2 flex items-center gap-2 rounded-lg transition-colors text-sm shadow-md">
                          <Upload size={16} /> 在该目录中上传
                        </button>
                        <button onClick={() => setActiveDriveParty(null)} className="text-gray-400 hover:text-white p-2 rounded-lg hover:bg-white/10">
                          <X size={20} />
                        </button>
                      </div>
                    </div>
                    {/* Party staged file content */}
                    <div className="flex-1 p-8 overflow-y-auto bg-[#1e1e1e] flex flex-col items-center justify-center border-t border-black/20">
                      <div className="w-24 h-24 rounded-full bg-[#2b2b2d] flex items-center justify-center mb-6 border border-white/5 shadow-inner">
                         <Folder size={48} className="text-cyan-500/50" />
                      </div>
                      <h3 className="text-xl font-medium text-gray-300 mb-2">"{activeDriveParty.name}" 的专属存证网盘目录</h3>
                      <p className="text-gray-500 text-sm max-w-md text-center mb-8">
                        此目录用于存放该当事人在本次业务中的所有关联附件信息。上传的文件将自动归档至涉密保险箱。
                      </p>
                      
                      {partyDriveDocuments.length === 0 ? (
                      <button onClick={() => partyDriveUploadRef.current?.click()} className="px-6 py-3 bg-[#2b2b2d] border border-cyan-500/30 text-cyan-400 rounded-xl hover:bg-cyan-500/10 transition-colors flex items-center gap-2">
                         <Plus size={18} /> 点击上传附件到当前目录
                      </button>
                      ) : (
                        <div className="w-full max-w-3xl flex flex-col gap-2">
                          {partyDriveDocuments.map((attachment) => (
                            <div key={attachment.id} className="bg-[#181818] p-3 rounded-lg border border-white/5 flex items-center justify-between group w-full">
                              <div className="flex items-center gap-3 min-w-0">
                                <FileText size={18} className="text-gray-400 group-hover:text-cyan-400 transition-colors shrink-0" />
                                <div className="min-w-0">
                                  <div className="text-sm text-gray-300 group-hover:text-gray-100 transition-colors truncate">{attachment.name}</div>
                                  <div className="text-xs text-gray-500">{`${Math.max(1, Math.round(attachment.file.size / 1024))} KB`}</div>
                                </div>
                              </div>
                            </div>
                          ))}
                          <button onClick={() => partyDriveUploadRef.current?.click()} className="mt-4 self-center px-6 py-3 bg-[#2b2b2d] border border-cyan-500/30 text-cyan-400 rounded-xl hover:bg-cyan-500/10 transition-colors flex items-center gap-2">
                            <Plus size={18} /> 继续上传附件到当前目录
                          </button>
                        </div>
                      )}
                    </div>
                  </motion.div>
                </div>
              )}
            </AnimatePresence>

            {/* Video Call QR Code Overlay */}
            <AnimatePresence>
              {activeQrCodeParty && (
                <div className="fixed inset-0 z-[400] flex items-center justify-center p-8">
                  <motion.div 
                    initial={{ opacity: 0 }} animate={{ opacity: 1 }} exit={{ opacity: 0 }}
                    onClick={() => setActiveQrCodeParty(null)}
                    className="absolute inset-0 bg-black/80 backdrop-blur-md"
                  />
                  <motion.div 
                    initial={{ opacity: 0, scale: 0.95, y: 20 }}
                    animate={{ opacity: 1, scale: 1, y: 0 }}
                    exit={{ opacity: 0, scale: 0.95, y: 20 }}
                    className="relative w-full max-w-sm bg-[#1e1e1e] rounded-2xl shadow-2xl border border-white/10 flex flex-col items-center overflow-hidden"
                  >
                    <div className="w-full flex items-center justify-between px-6 py-4 border-b border-white/5 bg-[#181818]">
                      <div className="flex items-center gap-2 text-gray-200 font-medium">
                        <Smartphone size={18} className="text-purple-400" />
                        <span>扫描二维码进行视频通话</span>
                      </div>
                      <button onClick={() => setActiveQrCodeParty(null)} className="text-gray-500 hover:text-white transition-colors">
                        <X size={20} />
                      </button>
                    </div>
                    
                    <div className="p-8 flex flex-col items-center w-full">
                      <div className="mb-6 flex flex-col items-center text-center">
                         <div className="w-12 h-12 rounded-full bg-indigo-500/10 flex items-center justify-center text-indigo-400 mb-3 border border-indigo-500/20">
                            <UserIcon size={24} />
                         </div>
                         <h3 className="text-lg font-medium text-white mb-1">{activeQrCodeParty.name}</h3>
                         <p className="text-xs text-gray-400">"{activeQrCodeParty.role}" 角色专用通话二维码</p>
                      </div>

                      <div className="bg-white p-4 rounded-xl shadow-inner mb-6">
                         {/* Mock QR Code UI */}
                         <div className="w-48 h-48 border-4 border-black relative">
                            <div className="absolute top-0 left-0 w-8 h-8 border-t-4 border-l-4 border-black"></div>
                            <div className="absolute top-0 right-0 w-8 h-8 border-t-4 border-r-4 border-black"></div>
                            <div className="absolute bottom-0 left-0 w-8 h-8 border-b-4 border-l-4 border-black"></div>
                            <div className="absolute bottom-0 right-0 w-8 h-8 border-b-4 border-r-4 border-black"></div>
                            <div className="w-full h-full flex items-center justify-center p-2">
                               <div className="w-full h-full bg-[url('data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSIxMDAlIiBoZWlnaHQ9IjEwMCUiPgo8cmVjdCB3aWR0aD0iMTAwJSIgaGVpZ2h0PSIxMDAlIiBmaWxsPSJub25lIiAvPgo8cGF0aCBkPSJNMCAwaDEwdjEwSDB6bTIwIDBoMTB2MTBIMjB6bTIwIDBoMTB2MTBIMDB6TTAgMjBoMTB2MTBIMHptNDAgMGgxMHYxMEg0MHpNMCA0MGgxMHYxMEgwcjMwIDBoMTB2MTBIMzB6IiBmaWxsPSJibGFjayIvPgo8L3N2Zz4=')] opacity-80" style={{ backgroundSize: '20px 20px' }}></div>
                               <div className="absolute inset-0 m-auto w-12 h-12 bg-white rounded flex items-center justify-center border-2 border-black">
                                 <Video size={24} className="text-black" />
                               </div>
                            </div>
                         </div>
                      </div>

                      <div className="text-xs text-gray-500 text-center max-w-xs leading-relaxed">
                        请使用<span className="text-indigo-400 px-1">专用App</span>或<span className="text-indigo-400 px-1">微信小程序</span>扫描此二维码，进入专线加密视频通话通道。
                      </div>
                    </div>
                  </motion.div>
                </div>
              )}
            </AnimatePresence>

            {/* Add Party Drawer Overlay */}
            <PartyDrawer 
              isOpen={showAddPartyPanel}
              onClose={() => setShowAddPartyPanel(false)}
              party={editingPartyId ? parties.find(p => p.id === editingPartyId) || null : null}
              onSave={handleSaveParty}
              readOnly={isPartyReadOnly}
              onSign={setActiveSignParty}
            />

          </div>
        </div>
      </div>
    </div>
  );
};
