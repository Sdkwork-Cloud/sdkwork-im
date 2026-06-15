const EMPTY_NOTARY_PRINT_IMAGE_URL = 'data:image/gif;base64,R0lGODlhAQABAIAAAAAAAP///ywAAAAAAQABAAACAUwAOw==';
import React, { useState, useEffect } from 'react';
import { Search, Filter, ShieldCheck, Download, ChevronLeft, ChevronRight, FileText, CheckCircle2, Clock, AlertCircle, FileSignature, X, MoreHorizontal, FileCheck, CheckSquare, Shield, Activity, Hash, Layers, User as UserIcon, Video, Printer, Edit, Folder, Cloud, Plus, Upload, PenTool } from 'lucide-react';
import { motion, AnimatePresence } from 'motion/react';
import { CreateNotaryTaskView } from './CreateNotaryTaskView';
import { NotaryTask, Party, NotaryDocument } from '@sdkwork/im-pc-types';
import { notaryService } from './services/NotaryService';
import { CallOverlay, createDefaultAvatar, toast } from '@sdkwork/im-pc-chat';
import { MediaViewer } from '@sdkwork/im-pc-commons';
import { PartyDrawer } from './PartyDrawer';
import { SignaturePad } from './SignaturePad';

const DEFAULT_NOTARY_CALLER_AVATAR = createDefaultAvatar('user');

const getStatusBadge = (status: NotaryTask['status']) => {
  switch (status) {
    case 'PENDING_REVIEW': return <span className="px-2 py-1 rounded bg-orange-500/20 text-orange-400 text-xs font-medium border border-orange-500/20">待审核</span>;
    case 'PROCESSING': return <span className="px-2 py-1 rounded bg-indigo-500/20 text-indigo-400 text-xs font-medium border border-indigo-500/20">办理中</span>;
    case 'COMPLETED': return <span className="px-2 py-1 rounded bg-green-500/20 text-green-400 text-xs font-medium border border-green-500/20">已出证</span>;
    case 'REJECTED': return <span className="px-2 py-1 rounded bg-red-500/20 text-red-400 text-xs font-medium border border-red-500/20">已驳回</span>;
  }
};

export const NotaryView: React.FC = () => {
  const [activeView, setActiveView] = useState<'list' | 'create'>('list');
  const [searchTerm, setSearchTerm] = useState('');
  const [typeFilter, setTypeFilter] = useState('ALL');
  const [selectedTask, setSelectedTask] = useState<NotaryTask | null>(null);
  const [activePaneTab, setActivePaneTab] = useState<'parties' | 'materials'>('parties');
  const [pageSize, setPageSize] = useState(20);
  const [currentPage, setCurrentPage] = useState(1);
  const [activeDropdown, setActiveDropdown] = useState<string | null>(null);
  const [expandedParty, setExpandedParty] = useState<string | null>(null);
  const [printTask, setPrintTask] = useState<NotaryTask | null>(null);
  const [partyIdentityMediaUrls, setPartyIdentityMediaUrls] = useState<Record<string, { identityFrontUrl?: string; identityBackUrl?: string; faceImageUrl?: string }>>({});

  const [editingPartyId, setEditingPartyId] = useState<string | null>(null);
  const [activeCall, setActiveCall] = useState<{isOpen: boolean, name: string, conversationId?: string, inviteUrl?: string}>({ isOpen: false, name: '' });
  const [activeDriveParty, setActiveDriveParty] = useState<Party | null>(null);
  const [partyDriveDocuments, setPartyDriveDocuments] = useState<NotaryDocument[]>([]);
  const [partyDriveLoading, setPartyDriveLoading] = useState(false);
  const [activeSignParty, setActiveSignParty] = useState<Party | null>(null);
  const [activeSignInviteUrl, setActiveSignInviteUrl] = useState<string | undefined>(undefined);
  const [previewMedia, setPreviewMedia] = useState<{isOpen: boolean, type: 'image' | 'video', url: string, name: string}>({ isOpen: false, type: 'image', url: '', name: '' });

  const [tasks, setTasks] = useState<NotaryTask[]>([]);
  const [loading, setLoading] = useState(false);

  // Close dropdown on click outside
  useEffect(() => {
    const handleClickOutside = () => setActiveDropdown(null);
    document.addEventListener('click', handleClickOutside);
    return () => document.removeEventListener('click', handleClickOutside);
  }, []);

  const fetchTasks = async () => {
    setLoading(true);
    try {
      const data = await notaryService.getTasks({ status: typeFilter, searchTerm });
      setTasks(data);
    } catch (error) {
      console.error('Failed to fetch tasks:', error);
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    if (activeView === 'list') {
      fetchTasks();
      setCurrentPage(1);
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [activeView, typeFilter, searchTerm]);

  // Re-fetch selected task internally to update pane if needed
  useEffect(() => {
    if (selectedTask) {
      const liveTask = tasks.find(t => t.id === selectedTask.id);
      if (liveTask && liveTask.status !== selectedTask.status) {
         setSelectedTask(liveTask);
      }
    }
  }, [tasks, selectedTask]);

  useEffect(() => {
    let disposed = false;
    const loadPartyIdentityMediaUrls = async () => {
      if (!printTask?.id || !printTask.parties?.length) {
        setPartyIdentityMediaUrls({});
        return;
      }
      const entries = await Promise.all(
        printTask.parties.map(async (party) => [
          party.id,
          await notaryService.getPartyIdentityMediaUrls(printTask.id, party.id, { disposition: 'inline' }),
        ] as const),
      );
      if (!disposed) {
        setPartyIdentityMediaUrls(Object.fromEntries(entries));
      }
    };
    void loadPartyIdentityMediaUrls();
    return () => {
      disposed = true;
    };
  }, [printTask]);
  const handleCreateSuccess = async () => {
    setActiveView('list');
    await fetchTasks();
  };

  const handleEditParty = (party: Party) => {
    setEditingPartyId(party.id);
  };

  const handleSaveParty = async (partyData: Party) => {
    if (!selectedTask) return;
    const targetId = editingPartyId || partyData.id;
    const newParties = selectedTask.parties?.map(p => p.id === targetId ? partyData : p) || [];
    const updatedTask = { ...selectedTask, parties: newParties };
    setSelectedTask(updatedTask);
    setEditingPartyId(null);
    try {
      const savedTask = await notaryService.updateTask(selectedTask.id, { parties: newParties });
      setSelectedTask(savedTask);
      setTasks(prev => prev.map((task) => task.id === savedTask.id ? savedTask : task));
    } catch (error) {
      console.error("Failed to save edited party", error);
    }
  };

  const loadPartyDriveDocuments = async () => {
    if (!selectedTask?.id || !activeDriveParty?.id) {
      setPartyDriveDocuments([]);
      return;
    }
    setPartyDriveLoading(true);
    try {
      const documents = await notaryService.listPartyDocuments(selectedTask.id, activeDriveParty.id);
      setPartyDriveDocuments(documents);
    } catch (error) {
      console.error('Failed to load party Drive documents:', error);
      toast('当事人网盘目录加载失败', 'error');
    } finally {
      setPartyDriveLoading(false);
    }
  };

  useEffect(() => {
    void loadPartyDriveDocuments();
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [selectedTask?.id, activeDriveParty?.id]);

  const handlePartyDriveUpload = async (event: React.ChangeEvent<HTMLInputElement>) => {
    if (!selectedTask?.id || !activeDriveParty?.id) {
      event.target.value = '';
      return;
    }
    const files = Array.from<File>(event.target.files ?? []);
    if (files.length === 0) {
      event.target.value = '';
      return;
    }
    setPartyDriveLoading(true);
    try {
      let refreshedTask = selectedTask;
      for (const file of files) {
        refreshedTask = await notaryService.uploadPartyDocument(selectedTask.id, activeDriveParty.id, file);
      }
      setSelectedTask(refreshedTask);
      setTasks(prev => prev.map((task) => task.id === refreshedTask.id ? refreshedTask : task));
      const documents = await notaryService.listPartyDocuments(refreshedTask.id, activeDriveParty.id);
      setPartyDriveDocuments(documents);
      toast('附件已上传到当事人网盘目录', 'success');
    } catch (error) {
      console.error('Failed to upload party Drive document:', error);
      toast('当事人网盘附件上传失败', 'error');
    } finally {
      setPartyDriveLoading(false);
      event.target.value = '';
    }
  };
  if (activeView === 'create') {
    return <CreateNotaryTaskView onBack={() => setActiveView('list')} onSuccess={handleCreateSuccess} />;
  }

  if (activeSignParty) {
    return (
      <SignaturePad 
        title={`当事人在线签名 - ${activeSignParty.name}`}
        subtitle={<><PenTool size={16} /> 请当事人 <span className="text-indigo-400 font-medium">"{activeSignParty.name}"</span> 使用正楷书写姓名</>}
        mobileSignatureUrl={activeSignInviteUrl}
        onCancel={() => {
          setActiveSignInviteUrl(undefined);
          setActiveSignParty(null);
        }}
        onSave={(imgUrl) => {
          toast('签名已保存', 'success');
          handleSaveParty({ ...activeSignParty, signatureUrl: imgUrl });
          setActiveSignInviteUrl(undefined);
          setActiveSignParty(null);
        }}
      />
    );
  }

  // Calculate stats based on tasks state
  const completedCount = tasks.filter(t => t.status === 'COMPLETED').length;
  const processingCount = tasks.filter(t => t.status === 'PROCESSING').length;
  const pendingCount = tasks.filter(t => t.status === 'PENDING_REVIEW').length;

  // Pagination logic
  const totalPages = Math.ceil(tasks.length / pageSize) || 1;
  const paginatedTasks = tasks.slice((currentPage - 1) * pageSize, currentPage * pageSize);

  return (
    <>
    <div className="flex-1 flex flex-col bg-[#1e1e1e] min-w-0 min-h-0 overflow-hidden relative print:block print:overflow-visible print:h-auto print:min-h-0">
      
      {/* Main container */}
      <div className={`flex w-full h-full relative print:block print:overflow-visible print:h-auto print:min-h-0 ${printTask ? 'print:hidden' : ''}`}>
        
        <div className="flex-1 w-full h-full p-6 lg:p-8 flex flex-col gap-6 min-h-0 relative">
          
          {/* Top Header & Stats */}
          <div className="flex flex-col gap-6 shrink-0">
            <div className="flex items-center justify-between">
              <h1 className="text-2xl font-medium text-gray-100 flex items-center gap-2">
                <ShieldCheck className="text-indigo-500" />
                公证业务工作台
              </h1>
              <div className="flex items-center gap-3">
                <button 
                  className="px-4 py-2 bg-[#2b2b2d] hover:bg-white/10 text-gray-200 text-sm rounded flex items-center gap-2 transition-colors border border-white/5"
                >
                  <Download size={16} />业务月报
                </button>
                <button 
                  onClick={() => setActiveView('create')}
                  className="px-4 py-2 bg-indigo-600 hover:bg-indigo-700 text-white text-sm rounded flex items-center gap-2 transition-colors shadow-[0_0_15px_rgba(79,70,229,0.3)]"
                >
                  <FileSignature size={16} />新建
                </button>
              </div>
            </div>

            <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
              <div className="bg-gradient-to-br from-[#2b2b2d] to-[#252527] rounded-xl p-5 border border-white/5 shadow-sm relative overflow-hidden group">
                <div className="absolute top-0 right-0 p-4 opacity-5 group-hover:opacity-10 transition-opacity">
                  <Clock size={48} />
                </div>
                <div className="flex items-center gap-3 text-gray-400 mb-2">
                  <Clock size={16} className="text-orange-400" />
                  <span className="text-sm font-medium">待审核队列</span>
                </div>
                <div className="text-3xl font-semibold text-gray-100">24</div>
                <div className="text-xs text-gray-500 mt-2">预计处理时长 2.5 小时</div>
              </div>
              
              <div className="bg-gradient-to-br from-[#2b2b2d] to-[#252527] rounded-xl p-5 border border-white/5 shadow-sm relative overflow-hidden group">
                <div className="absolute top-0 right-0 p-4 opacity-5 group-hover:opacity-10 transition-opacity">
                  <ShieldCheck size={48} />
                </div>
                <div className="flex items-center gap-3 text-gray-400 mb-2">
                  <ShieldCheck size={16} className="text-green-400" />
                  <span className="text-sm font-medium">今日已出证</span>
                </div>
                <div className="text-3xl font-semibold text-gray-100">89</div>
                <div className="text-xs text-green-500 mt-2 flex items-center gap-1">+12 较昨日</div>
              </div>
              
              <div className="bg-gradient-to-br from-[#2b2b2d] to-[#252527] rounded-xl p-5 border border-white/5 shadow-sm relative overflow-hidden group">
                <div className="absolute top-0 right-0 p-4 opacity-5 group-hover:opacity-10 transition-opacity">
                  <AlertCircle size={48} />
                </div>
                <div className="flex items-center gap-3 text-gray-400 mb-2">
                  <AlertCircle size={16} className="text-red-400" />
                  <span className="text-sm font-medium">异常卷宗拦截</span>
                </div>
                <div className="text-3xl font-semibold text-gray-100">3</div>
                <div className="text-xs text-gray-500 mt-2">风控模型自动拦截</div>
              </div>
              
              <div className="bg-gradient-to-br from-[#2b2b2d] to-[#252527] rounded-xl p-5 border border-white/5 shadow-sm relative overflow-hidden group">
                <div className="absolute top-0 right-0 p-4 opacity-5 group-hover:opacity-10 transition-opacity">
                  <Layers size={48} />
                </div>
                <div className="flex items-center gap-3 text-gray-400 mb-2">
                  <Layers size={16} className="text-indigo-400" />
                  <span className="text-sm font-medium">本月存证量</span>
                </div>
                <div className="text-3xl font-semibold text-gray-100">12,408</div>
                <div className="text-xs text-gray-500 mt-2 flex items-center gap-1">
                  <div className="w-1.5 h-1.5 rounded-full bg-green-500"></div> Blockchain Sync OK
                </div>
              </div>
            </div>
          </div>

          {/* Filter Bar */}
          <div className="bg-[#2b2b2d] rounded-xl p-4 border border-white/5 flex flex-wrap items-center gap-4 shrink-0">
            <div className="relative flex-1 min-w-[240px]">
              <Search className="absolute left-3 top-1/2 -translate-y-1/2 text-gray-500" size={16} />
              <input 
                type="text" 
                placeholder="搜索流水卷宗号 / 机构名称 / 存证标的..." 
                value={searchTerm}
                onChange={(e) => setSearchTerm(e.target.value)}
                className="w-full bg-[#181818] border border-white/10 rounded-lg pl-9 pr-4 py-2 text-sm text-gray-200 outline-none focus:border-indigo-500 focus:ring-1 focus:ring-indigo-500 transition-all placeholder:text-gray-500"
              />
            </div>
            
            <select 
              value={typeFilter}
              onChange={(e) => setTypeFilter(e.target.value)}
              className="bg-[#181818] border border-white/10 rounded-lg px-4 py-2 text-sm text-gray-200 outline-none focus:border-indigo-500 focus:ring-1 focus:ring-indigo-500"
            >
              <option value="ALL">全部业务类型</option>
              <option value="ELECTRONIC">电子合同存证</option>
              <option value="IPR">知识产权确权公证</option>
              <option value="EVIDENCE">电子证据固化</option>
            </select>

            <select className="bg-[#181818] border border-white/10 rounded-lg px-4 py-2 text-sm text-gray-200 outline-none focus:border-indigo-500 focus:ring-1 focus:ring-indigo-500">
               <option value="ALL">全部节点状态</option>
               <option value="PENDING">待资审 / 风控</option>
               <option value="PROCESSING">公证员办理中</option>
               <option value="COMPLETED">区块链已出证</option>
            </select>
            
            <div className="flex items-center gap-2">
              <button className="px-4 py-2 bg-[#181818] border border-white/10 hover:bg-white/5 text-gray-200 text-sm rounded-lg flex items-center gap-2 transition-colors">
                <Filter size={16} /> 高级漏斗
              </button>
            </div>
          </div>

          {/* Task Table */}
          <div className="bg-[#2b2b2d] rounded-xl border border-white/5 flex-1 flex flex-col min-h-0 shadow-[0_4px_20px_rgba(0,0,0,0.2)]">
            <div className="flex-1 overflow-auto custom-scrollbar">
              <table className="w-full text-left border-collapse whitespace-nowrap">
                <thead>
                  <tr className="bg-[#1e1e1e]/90 text-gray-400 text-[13px] border-b border-white/5 sticky top-0 z-10 backdrop-blur-md">
                    <th className="px-6 py-4 font-medium">公证编号</th>
                    <th className="px-6 py-4 font-medium">标题</th>
                    <th className="px-6 py-4 font-medium">公证业务</th>
                    <th className="px-6 py-4 font-medium">当事人</th>
                    <th className="px-6 py-4 font-medium">公证员</th>
                    <th className="px-6 py-4 font-medium">状态</th>
                    <th className="px-6 py-4 font-medium">备注</th>
                    <th className="px-6 py-4 font-medium">操作</th>
                  </tr>
                </thead>
                <tbody>
                  {paginatedTasks.map((task, idx) => (
                    <tr 
                      key={task.id} 
                      onClick={() => setSelectedTask(task)}
                      className={`text-sm group transition-colors cursor-pointer
                        ${selectedTask?.id === task.id ? 'bg-indigo-500/10' : 'hover:bg-white/5'}
                        ${idx !== tasks.length - 1 ? 'border-b border-white/5' : ''}
                      `}
                    >
                      <td className="px-6 py-4">
                        <div className="flex items-center gap-2 text-indigo-400 font-medium">
                          <FileCheck size={16} />
                          {(task as any).caseNo ?? task.id}
                        </div>
                      </td>
                      <td className="px-6 py-4 text-gray-200 font-medium max-w-[200px] truncate" title={task.title}>
                        {task.title}
                      </td>
                      <td className="px-6 py-4 text-gray-300">
                        <span className="bg-white/5 px-2 py-1 rounded text-xs border border-white/5">{task.type}</span>
                      </td>
                      <td className="px-6 py-4 text-gray-400 max-w-[150px] truncate" title={task.parties?.map(p => p.name).join(', ') || '--'}>
                        {task.parties && task.parties.length > 0 ? task.parties.map(p => p.name).join(', ') : '--'}
                      </td>
                      <td className="px-6 py-4 text-gray-400">
                        {task.notary}
                      </td>
                      <td className="px-6 py-4">
                        {getStatusBadge(task.status)}
                      </td>
                      <td className="px-6 py-4 text-gray-400 max-w-[150px] truncate" title={task.remarks}>
                        {task.remarks || '--'}
                      </td>
                      <td className="px-6 py-4 relative">
                        <div className="flex items-center gap-3">
                          <button 
                            onClick={(e) => {
                              e.stopPropagation();
                              setSelectedTask(task);
                            }}
                            className="text-indigo-400 hover:text-indigo-300 font-medium transition-colors p-1 rounded hover:bg-indigo-500/10"
                          >
                            查看详情
                          </button>
                          <div className="relative">
                            <button 
                               className="text-gray-400 hover:text-gray-200 transition-colors p-1 rounded hover:bg-white/10"
                               onClick={(e) => {
                                 e.stopPropagation();
                                 setActiveDropdown(activeDropdown === task.id ? null : task.id);
                               }}
                            >
                              <MoreHorizontal size={16} />
                            </button>
                            <AnimatePresence>
                               {activeDropdown === task.id && (
                                 <motion.div
                                   initial={{ opacity: 0, y: 5 }}
                                   animate={{ opacity: 1, y: 0 }}
                                   exit={{ opacity: 0, y: 5 }}
                                   transition={{ duration: 0.15 }}
                                   className="absolute right-0 top-full mt-1 w-40 bg-[#2b2b2d] border border-white/10 shadow-xl rounded-lg py-1 z-50 overflow-hidden"
                                 >
                                   <div
                                     onClick={async (e) => {
                                       e.stopPropagation();
                                       setActiveDropdown(null);
                                       toast('开始下载材料...', 'success');

                                       const { downloadUrl } = await notaryService.downloadDocuments(task.id);

                                       if (downloadUrl) {

                                         window.open(downloadUrl, '_blank');

                                       }
                                     }}
                                     className="px-4 py-2 hover:bg-white/10 cursor-pointer text-gray-300 hover:text-white transition-colors flex items-center gap-2"
                                   >
                                      <Download size={14} /> 下载材料压缩包
                                   </div>
                                   <div 
                                     onClick={(e) => { e.stopPropagation(); setActiveDropdown(null); setPrintTask(task); }}
                                     className="px-4 py-2 hover:bg-white/10 cursor-pointer text-gray-300 hover:text-white transition-colors flex items-center gap-2"
                                   >
                                      <User size={14} /> 打印当事人信息
                                   </div>
                                   <div className="h-px bg-white/10 my-1 mx-2" />
                                   <div 
                                     onClick={(e) => { 
                                        e.stopPropagation(); 
                                        setActiveDropdown(null); 
                                        notaryService.deleteTask(task.id).then(() => {
                                          setTasks(tasks.filter(t => t.id !== task.id));
                                          toast('流水已删除', 'success');
                                        });
                                     }}
                                     className="px-4 py-2 hover:bg-red-500/20 text-red-400 cursor-pointer transition-colors flex items-center gap-2"
                                   >
                                      <X size={14} /> 删除
                                   </div>
                                 </motion.div>
                               )}
                            </AnimatePresence>
                          </div>
                        </div>
                      </td>
                    </tr>
                  ))}
                </tbody>
              </table>
            </div>
            
            {/* Pagination */}
            <div className="p-4 border-t border-white/5 flex items-center justify-between text-sm text-gray-400 bg-[#181818]/60 shrink-0">
              <div className="flex items-center gap-4">
                <span>共检索到 {tasks.length} 份电子卷宗</span>
                <div className="flex items-center gap-2">
                  <span className="text-gray-500">每页显示</span>
                  <select 
                    value={pageSize}
                    onChange={(e) => {
                      setPageSize(Number(e.target.value));
                      setCurrentPage(1);
                    }}
                    className="bg-[#181818] border border-white/10 rounded px-2 py-1 text-sm text-gray-200 outline-none focus:border-indigo-500 hover:border-white/20 cursor-pointer"
                  >
                    <option value={10}>10</option>
                    <option value={20}>20</option>
                    <option value={50}>50</option>
                    <option value={100}>100</option>
                  </select>
                  <span className="text-gray-500">条</span>
                </div>
              </div>
              <div className="flex items-center gap-2">
                <button 
                  onClick={() => setCurrentPage(p => Math.max(1, p - 1))}
                  disabled={currentPage === 1}
                  className="p-1.5 rounded hover:bg-white/10 disabled:opacity-50 transition-colors border border-transparent hover:border-white/10"
                >
                  <ChevronLeft size={16} />
                </button>
                <div className="flex items-center gap-1">
                  {Array.from({ length: totalPages }).map((_, i) => {
                    const page = i + 1;
                    if (page === 1 || page === totalPages || (page >= currentPage - 1 && page <= currentPage + 1)) {
                      return (
                        <button 
                          key={page}
                          onClick={() => setCurrentPage(page)}
                          className={`w-7 h-7 rounded flex items-center justify-center transition-colors ${currentPage === page ? 'bg-indigo-500 text-white font-medium shadow-sm' : 'hover:bg-white/10 text-gray-300'}`}
                        >
                          {page}
                        </button>
                      );
                    } else if (page === currentPage - 2 || page === currentPage + 2) {
                      return <span key={`ellipsis-${page}`} className="px-1 text-gray-500">...</span>;
                    }
                    return null;
                  })}
                </div>
                <button 
                  onClick={() => setCurrentPage(p => Math.min(totalPages, p + 1))}
                  disabled={currentPage === totalPages}
                  className="p-1.5 rounded hover:bg-white/10 disabled:opacity-50 transition-colors border border-transparent hover:border-white/10"
                >
                  <ChevronRight size={16} />
                </button>
              </div>
            </div>
          </div>
        </div>

        {/* Right Side Detail Pane */}
        <AnimatePresence>
          {selectedTask && (
            <>
              {/* Backdrop */}
              <motion.div 
                initial={{ opacity: 0 }}
                animate={{ opacity: 1 }}
                exit={{ opacity: 0 }}
                transition={{ duration: 0.2 }}
                onClick={() => setSelectedTask(null)}
                className="absolute inset-0 bg-black/40 backdrop-blur-[2px] z-10"
              />
              
              {/* Drawer */}
              <motion.div 
                initial={{ x: '100%', opacity: 0 }}
                animate={{ x: 0, opacity: 1 }}
                exit={{ x: '100%', opacity: 0 }}
                transition={{ type: 'spring', damping: 25, stiffness: 200 }}
                className="absolute right-0 top-0 bottom-0 w-[720px] xl:w-[860px] bg-[#222224] border-l border-white/5 flex flex-col shadow-[-10px_0_30px_rgba(0,0,0,0.5)] z-20"
              >
              {/* Pane Header */}
              <div className="flex items-center justify-between p-6 border-b border-white/5 shrink-0 bg-[#2b2b2d]">
                <div className="flex items-center gap-3">
                  <div className="w-10 h-10 rounded-lg bg-indigo-500/20 flex items-center justify-center text-indigo-400">
                    <Shield size={20} />
                  </div>
                  <div>
                    <h2 className="text-lg font-medium text-gray-100 leading-tight">公证详情</h2>
                    <p className="text-xs text-gray-400 font-mono mt-1">{(selectedTask as any).caseNo ?? selectedTask.id}</p>
                  </div>
                </div>
                <button 
                  onClick={() => setSelectedTask(null)}
                  className="w-8 h-8 rounded-full flex items-center justify-center hover:bg-white/10 text-gray-400 hover:text-gray-200 transition-colors"
                >
                  <X size={18} />
                </button>
              </div>

              {/* Pane Content */}
              <div className="flex-1 overflow-y-auto custom-scrollbar flex flex-col relative">

                {/* Base Info Section Above Tabs */}
                <div className="p-6 pb-2 shrink-0 bg-[#2b2b2d]">
                  <div className="bg-[#181818] border border-white/5 p-4 rounded-xl grid grid-cols-2 gap-4 text-sm relative">
                    <div className="absolute top-0 right-0 w-32 h-32 bg-indigo-500/5 rounded-full blur-2xl pointer-events-none -mr-10 -mt-10" />
                    <div>
                       <div className="text-gray-500 mb-1 flex items-center gap-1.5"><Layers size={14}/> 公证业务</div>
                       <div className="text-gray-200 font-medium pl-5">{selectedTask.type}</div>
                    </div>
                    <div>
                       <div className="text-gray-500 mb-1 flex items-center gap-1.5"><Hash size={14}/> 公证编号</div>
                       <div className="text-gray-200 font-mono text-xs mt-0.5 pl-5">{(selectedTask as any).caseNo ?? selectedTask.id}</div>
                    </div>
                    <div>
                       <div className="text-gray-500 mb-1 flex items-center gap-1.5"><UserIcon size={14}/> 申请主体</div>
                       <div className="text-gray-200 pl-5">{selectedTask.applicant}</div>
                    </div>
                    <div>
                       <div className="text-gray-500 mb-1 flex items-center gap-1.5"><Activity size={14}/> 当前状态</div>
                       <div className="mt-0.5 pl-5">{getStatusBadge(selectedTask.status)}</div>
                    </div>
                    <div>
                       <div className="text-gray-500 mb-1 flex items-center gap-1.5"><UserIcon size={14}/> 公证员</div>
                       <div className="text-gray-200 pl-5">{selectedTask.notary || '未分配'}</div>
                    </div>
                    <div>
                       <div className="text-gray-500 mb-1 flex items-center gap-1.5"><Clock size={14}/> 办理时间</div>
                       <div className="text-gray-400 font-mono text-xs mt-0.5 pl-5">{selectedTask.processTime || selectedTask.createTime + ':00'}</div>
                    </div>
                    <div className="col-span-2">
                       <div className="text-gray-500 mb-1 flex items-center gap-1.5"><Hash size={14}/> 链上哈希</div>
                       <div className="text-gray-400 font-mono text-xs mt-0.5 pl-5 break-all">{selectedTask.hash}</div>
                    </div>
                  </div>
                </div>
                
                {/* Tabs Header */}
                <div className="flex bg-[#2b2b2d] px-6 py-4 sticky top-0 z-10 border-b border-white/5">
                  <div className="flex bg-[#181818] p-1 rounded-xl border border-white/5 relative">
                    {[
                      { id: 'parties', label: '当事人列表' },
                      { id: 'materials', label: '公证材料' }
                    ].map((tab) => {
                      const isActive = activePaneTab === tab.id;
                      return (
                        <button
                          key={tab.id}
                          onClick={() => setActivePaneTab(tab.id as 'parties' | 'materials')}
                          className={`relative px-6 py-2 text-sm font-medium rounded-lg transition-colors z-10 ${
                            isActive ? 'text-indigo-300' : 'text-gray-400 hover:text-gray-200'
                          }`}
                        >
                          {isActive && (
                            <motion.div
                              layoutId="activePaneTabIndicator"
                              className="absolute inset-0 bg-indigo-500/20 border border-indigo-500/30 rounded-lg -z-10"
                              initial={false}
                              transition={{ type: "spring", bounce: 0.2, duration: 0.6 }}
                            />
                          )}
                          {tab.label}
                        </button>
                      );
                    })}
                  </div>
                </div>
                
                <div className="p-6 flex flex-col gap-8">
                  {/* Parties Tab */}
                  {activePaneTab === 'parties' && (
                    <div className="flex flex-col gap-4">
                      {(!selectedTask.parties || selectedTask.parties.length === 0) ? (
                         <div className="text-sm text-gray-500 text-center py-8 bg-[#181818]/50 rounded-lg border border-dashed border-white/5">
                           暂无当事人信息
                         </div>
                      ) : (
                        selectedTask.parties.map((party) => (
                          <div 
                            key={party.id} 
                            onDoubleClick={() => handleEditParty(party)}
                            className="bg-[#181818] p-4 rounded-xl border border-white/5 flex flex-col gap-3 group/party hover:border-indigo-500/30 transition-colors"
                          >
                            <div className="flex items-center justify-between border-b border-white/5 pb-3">
                               <div className="flex items-center gap-3">
                                  <div className="w-10 h-10 rounded-full bg-indigo-500/10 flex items-center justify-center text-indigo-400 text-lg font-medium">
                                    {party.name.charAt(0)}
                                  </div>
                                  <div>
                                     <div className="text-sm font-medium text-gray-200 flex items-center gap-2">
                                       {party.name} 
                                       <span className="px-1.5 py-0.5 rounded text-[10px] bg-white/10 text-gray-400">{party.role}</span>
                                       <span className="px-1.5 py-0.5 rounded text-[10px] bg-white/10 text-gray-400">{party.gender}</span>
                                     </div>
                                     <div className="text-xs text-gray-500 mt-1 font-mono">{party.identityId}</div>
                                  </div>
                               </div>
                               <div className="flex items-center gap-2">
                                 {party.signatureUrl && (
                                   <div className="px-2 py-1 bg-teal-500/10 text-teal-400 rounded-lg text-xs font-medium border border-teal-500/20 flex items-center gap-1">
                                     <CheckCircle2 size={14} /> 已签名
                                   </div>
                                 )}
                                 {selectedTask.status !== 'COMPLETED' && selectedTask.status !== 'REJECTED' && (
                                   <button onClick={async () => {
                                      const invite = await notaryService.createSignatureInvite(selectedTask.id, party.id);
                                      setActiveSignInviteUrl(invite.signingUrl ?? invite.inviteUrl);
                                      setActiveSignParty(party);
                                    }} className="px-2 py-1 bg-orange-500/10 text-orange-400 hover:bg-orange-500/20 rounded-lg transition-colors border border-orange-500/20 shrink-0 text-[11px] font-medium flex items-center gap-1 " title="签名">
                                     <PenTool size={14} /> 签名
                                   </button>
                                 )}
                                 <button onClick={() => setActiveDriveParty(party)} className="p-2 bg-blue-500/10 text-blue-400 hover:bg-blue-500/20 rounded-lg transition-colors flex items-center gap-1.5 text-xs font-medium border border-blue-500/20 " title="网盘目录">
                                   <Folder size={14} />
                                 </button>
                                 <button onClick={() => handleEditParty(party)} className="p-2 bg-indigo-500/10 text-indigo-400 hover:bg-indigo-500/20 rounded-lg transition-colors border border-indigo-500/20 " title="编辑">
                                   <Edit size={14} />
                                 </button>
                                 <button 
                                   onClick={async () => {
            if (!selectedTask.notary || selectedTask.notary === '系统分配' || selectedTask.notary === '未分配') toast('请先选择承办公证员后再发起视频通话。', 'error');
            else {
              const invite = await notaryService.createVideoInvite(selectedTask.id, party.id);
              setActiveCall({
                isOpen: true,
                name: party.name,
                conversationId: invite.conversationId,
                inviteUrl: invite.inviteUrl,
              });
            }
          }}
                                   className="px-3 py-1.5 bg-green-500/10 text-green-500 hover:bg-green-500/20 rounded-lg text-[11px] font-medium transition-colors flex items-center gap-1.5 shrink-0"
                                 >
                                    <Video size={14} /> 视频
                                 </button>
                               </div>
                            </div>
                            <div className="text-xs text-gray-400 flex items-center justify-between">
                                <div className="flex items-center gap-2">
                                  <span className="w-16">联系电话：</span>
                                  <span className="text-gray-200 font-mono">{party.phone}</span>
                                </div>
                                <button 
                                  onClick={() => setExpandedParty(expandedParty === party.id ? null : party.id)}
                                  className="text-indigo-400 hover:text-indigo-300 font-medium transition-colors"
                                >
                                  {expandedParty === party.id ? '收起检查信息' : '查看检查信息'}
                                </button>
                            </div>
                            
                            {/* Expanded party info */}
                            <AnimatePresence>
                              {expandedParty === party.id && (
                                <motion.div 
                                  initial={{ height: 0, opacity: 0 }}
                                  animate={{ height: 'auto', opacity: 1 }}
                                  exit={{ height: 0, opacity: 0 }}
                                  className="overflow-hidden"
                                >
                                  <div className="mt-3 bg-black/20 p-4 rounded-lg border border-white/5 flex flex-col gap-4 text-xs">
                                     <div className="flex justify-between items-start">
                                        <div>
                                          <div className="text-gray-500 mb-1">实名鉴权状态</div>
                                          <div className="flex items-center gap-1.5 text-green-500 font-medium">
                                            <ShieldCheck size={14} /> 通过公安部库比对 (98.5%)
                                          </div>
                                        </div>
                                        <div className="text-right">
                                          <div className="text-gray-500 mb-1">人脸活体采集</div>
                                          <div className="text-gray-300">2026-04-17 11:28:45</div>
                                        </div>
                                     </div>
                                     <div className="grid grid-cols-2 gap-4 pt-3 border-t border-white/5">
                                        <div>
                                          <div className="text-gray-500 mb-1">证件照片 (人像面/国徽面)</div>
                                          <div className="flex gap-2 min-h-[60px]">
                                            <div className="w-20 h-12 bg-white/5 rounded border border-white/10 flex items-center justify-center text-gray-500">已存档</div>
                                            <div className="w-20 h-12 bg-white/5 rounded border border-white/10 flex items-center justify-center text-gray-500">已存档</div>
                                          </div>
                                        </div>
                                        <div>
                                          <div className="text-gray-500 mb-1">现场采集抓拍</div>
                                          <div className="w-16 h-16 bg-white/5 rounded border border-white/10 flex items-center justify-center text-gray-500">活体</div>
                                        </div>
                                     </div>
                                  </div>
                                </motion.div>
                              )}
                            </AnimatePresence>
                          </div>
                        ))
                      )}
                    </div>
                  )}

                  {/* Materials Tab */}
                  {activePaneTab === 'materials' && (
                    <div className="flex flex-col gap-6">
                      <div className="flex justify-end">
                        <button 
                          onClick={async () => {
                            toast('服务端正在打包所有附件...', 'success');

                            const { downloadUrl } = await notaryService.downloadDocuments(selectedTask.id);

                            if (downloadUrl) {

                              toast('附件包准备就绪，开始下载。', 'success');

                              window.open(downloadUrl, '_blank');

                            }
                          }}
                          className="px-3 py-1.5 bg-indigo-500/10 hover:bg-indigo-500/20 text-indigo-400 rounded flex items-center gap-1.5 text-xs font-medium cursor-pointer transition-colors border border-indigo-500/20"
                        >
                          <Download size={14} /> 打包下载全部附件
                        </button>
                      </div>
                      {['identity', 'evidence', 'notary'].map(categoryKey => {
                         const categoryDocs = selectedTask.documents.filter(d => d.category === categoryKey);
                         if (categoryDocs.length === 0) return null;
                         
                         let title = '';
                         let icon = null;
                         if (categoryKey === 'identity') { title = '身份证明材料'; icon = <UserIcon size={16} className="text-indigo-400"/>; }
                         if (categoryKey === 'evidence') { title = '业务证据材料'; icon = <Layers size={16} className="text-orange-400"/>; }
                         if (categoryKey === 'notary') { title = '公证送达文书'; icon = <FileSignature size={16} className="text-indigo-400"/>; }

                         return (
                           <div key={categoryKey}>
                             <h4 className="text-sm font-medium text-gray-300 mb-3 flex items-center gap-2">
                               {icon} {title}
                             </h4>
                             <div className="flex flex-col gap-2">
                               {categoryDocs.map((doc, i) => (
                                 <div key={i} className="bg-[#181818] p-3 rounded-lg border border-white/5 flex items-center justify-between group">
                                   <div className="flex items-center gap-3 min-w-0">
                                     <FileText size={18} className="text-gray-400 group-hover:text-indigo-400 transition-colors shrink-0" />
                                     <div className="min-w-0">
                                       <div className="text-sm text-gray-300 group-hover:text-gray-100 transition-colors truncate cursor-pointer hover:underline" onClick={async () => {
                                         const { previewUrl, downloadUrl, url } = await notaryService.getDocumentUrl(selectedTask.id, doc.name, { disposition: 'inline' });
                                         const resolvedUrl = previewUrl ?? downloadUrl ?? url;
                                         if (resolvedUrl) {
                                           setPreviewMedia({isOpen: true, type: doc.name.endsWith('.mp4') ? 'video' : 'image', url: resolvedUrl, name: doc.name});
                                         }
                                       }}>{doc.name}</div>
                                       <div className="text-xs text-gray-500">{doc.size}</div>
                                     </div>
                                   </div>
                                   <div className="flex items-center gap-4 shrink-0 pl-2">
                                     <div className="text-xs">
                                       {doc.status === 'verified' && <span className="text-green-500 flex items-center gap-1"><CheckCircle2 size={12}/> 已核验</span>}
                                       {doc.status === 'pending' && <span className="text-orange-500 flex items-center gap-1"><Clock size={12}/> 校验中</span>}
                                       {doc.status === 'error' && <span className="text-red-500 flex items-center gap-1"><AlertCircle size={12}/> 异常</span>}
                                     </div>
                                     <button 
                                       onClick={async () => {
                                         toast(`开始下载: ${doc.name}`, 'success');
                                         const { downloadUrl, url } = await notaryService.getDocumentUrl(selectedTask.id, doc.name, { disposition: 'attachment' });
                                         const resolvedUrl = downloadUrl ?? url;
                                         if (resolvedUrl) {
                                           window.open(resolvedUrl, '_blank');
                                         }
                                       }}
                                       className="p-1.5 text-gray-500 hover:text-indigo-400 hover:bg-indigo-500/10 rounded transition-colors opacity-0 group-hover:opacity-100"
                                       title="下载此附件"
                                     >
                                       <Download size={16} />
                                     </button>
                                   </div>
                                 </div>
                               ))}
                             </div>
                           </div>
                         );
                      })}
                    </div>
                  )}

                </div>
              </div>

              {/* Action Footer */}
              <div className="p-6 border-t border-white/5 shrink-0 bg-[#2b2b2d] flex justify-between items-center gap-3">
                <button 
                  onClick={() => setPrintTask(selectedTask)}
                  className="px-4 py-2 text-sm text-gray-300 hover:text-white hover:bg-white/5 rounded border border-white/10 transition-colors flex items-center gap-2"
                >
                  <Printer size={16} /> 打印当事人信息
                </button>
                <div className="flex gap-3">
                  {selectedTask.status === 'PENDING_REVIEW' && (
                    <>
                      <button 
                        onClick={async () => {
                          const updated = await notaryService.updateTaskStatus(selectedTask.id, 'REJECTED');
                          setSelectedTask(updated);

                          setTasks(prev => prev.map((task) => task.id === updated.id ? updated : task));
                          fetchTasks();
                        }}
                        className="px-4 py-2 text-sm text-red-400 hover:text-red-300 hover:bg-red-500/10 rounded border border-red-500/20 transition-colors"
                      >
                        退回补充材料
                      </button>
                      <button 
                        onClick={async () => {
                          const updated = await notaryService.updateTaskStatus(selectedTask.id, 'PROCESSING');
                          setSelectedTask(updated);

                          setTasks(prev => prev.map((task) => task.id === updated.id ? updated : task));
                          fetchTasks();
                        }}
                        className="px-6 py-2 text-sm text-white bg-indigo-600 hover:bg-indigo-700 rounded shadow-md transition-colors"
                      >
                        转入办理状态
                      </button>
                    </>
                  )}
                  {selectedTask.status === 'PROCESSING' && (
                     <button 
                        onClick={async () => {
                          const updated = await notaryService.updateTaskStatus(selectedTask.id, 'COMPLETED');
                          setSelectedTask(updated);

                          setTasks(prev => prev.map((task) => task.id === updated.id ? updated : task));
                          fetchTasks();
                        }}
                        className="px-6 py-2 text-sm text-white bg-green-600 hover:bg-green-700 rounded shadow-md transition-colors"
                     >
                        人工核验发证
                     </button>
                  )}
                </div>
              </div>

            </motion.div>
            </>
          )}
        </AnimatePresence>

        {/* Party Drive Directory */}
        <AnimatePresence>
          {activeDriveParty && selectedTask && (
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
                <input type="file" multiple className="hidden" onChange={handlePartyDriveUpload} accept="image/*,video/*,application/pdf" id="notary-party-drive-upload" />
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
                    <button onClick={() => document.getElementById('notary-party-drive-upload')?.click()} className="bg-cyan-600 hover:bg-cyan-700 text-white font-medium px-4 py-2 flex items-center gap-2 rounded-lg transition-colors text-sm shadow-md">
                      <Upload size={16} /> 在该目录中上传
                    </button>
                    <button onClick={() => setActiveDriveParty(null)} className="text-gray-400 hover:text-white p-2 rounded-lg hover:bg-white/10">
                      <X size={20} />
                    </button>
                  </div>
                </div>
                <div className="flex-1 p-8 overflow-y-auto bg-[#1e1e1e] flex flex-col items-center justify-center border-t border-black/20">
                  <div className="w-24 h-24 rounded-full bg-[#2b2b2d] flex items-center justify-center mb-6 border border-white/5 shadow-inner">
                    <Folder size={48} className="text-cyan-500/50" />
                  </div>
                  <h3 className="text-xl font-medium text-gray-300 mb-2">"{activeDriveParty.name}" 的专属存证网盘目录</h3>
                  <p className="text-gray-500 text-sm max-w-md text-center mb-8">
                    此目录用于存放该当事人在本次业务中的所有关联附件信息。上传的文件将自动归档至公证专属文件夹。
                  </p>
                  {partyDriveDocuments.length === 0 ? (
                    <button onClick={() => document.getElementById('notary-party-drive-upload')?.click()} className="px-6 py-3 bg-[#2b2b2d] border border-cyan-500/30 text-cyan-400 rounded-xl hover:bg-cyan-500/10 transition-colors flex items-center gap-2">
                      <Plus size={18} /> {partyDriveLoading ? '目录加载中...' : '点击上传附件到当前目录'}
                    </button>
                  ) : (
                    <div className="w-full max-w-3xl flex flex-col gap-2">
                      {partyDriveDocuments.map((doc, index) => (
                        <div key={`${doc.name}-${index}`} className="bg-[#181818] p-3 rounded-lg border border-white/5 flex items-center justify-between group w-full">
                          <div className="flex items-center gap-3 min-w-0">
                            <FileText size={18} className="text-gray-400 group-hover:text-cyan-400 transition-colors shrink-0" />
                            <div className="min-w-0">
                              <div className="text-sm text-gray-300 group-hover:text-gray-100 transition-colors truncate cursor-pointer hover:underline" onClick={async () => {
                                const { previewUrl, downloadUrl, url } = await notaryService.getDocumentUrl(selectedTask.id, doc.name, { disposition: 'inline' });
                                const resolvedUrl = previewUrl ?? downloadUrl ?? url;
                                if (resolvedUrl) {
                                  setPreviewMedia({isOpen: true, type: doc.name.endsWith('.mp4') ? 'video' : 'image', url: resolvedUrl, name: doc.name});
                                }
                              }}>{doc.name}</div>
                              <div className="text-xs text-gray-500">{doc.size}</div>
                            </div>
                          </div>
                          <button
                            onClick={async () => {
                              const { downloadUrl, url } = await notaryService.getDocumentUrl(selectedTask.id, doc.name, { disposition: 'attachment' });
                              const resolvedUrl = downloadUrl ?? url;
                              if (resolvedUrl) {
                                window.open(resolvedUrl, '_blank');
                              }
                            }}
                            className="p-1.5 text-gray-500 hover:text-cyan-400 hover:bg-cyan-500/10 rounded transition-colors opacity-0 group-hover:opacity-100"
                            title="下载此附件"
                          >
                            <Download size={16} />
                          </button>
                        </div>
                      ))}
                      <button onClick={() => document.getElementById('notary-party-drive-upload')?.click()} className="mt-4 self-center px-6 py-3 bg-[#2b2b2d] border border-cyan-500/30 text-cyan-400 rounded-xl hover:bg-cyan-500/10 transition-colors flex items-center gap-2">
                        <Plus size={18} /> {partyDriveLoading ? '上传处理中...' : '继续上传附件到当前目录'}
                      </button>
                    </div>
                  )}
                </div>
              </motion.div>
            </div>
          )}
        </AnimatePresence>
        {/* Print Overlay */}
        <AnimatePresence>
          {printTask && (
            <motion.div 
              initial={{ opacity: 0, scale: 0.95 }}
              animate={{ opacity: 1, scale: 1 }}
              exit={{ opacity: 0, scale: 0.95 }}
              transition={{ type: 'spring', damping: 25, stiffness: 200 }}
              className="fixed inset-0 bg-[#222] z-[200] flex flex-col overflow-hidden print:static print:bg-white print:z-auto print:overflow-visible print:block"
            >
              {/* Header */}
              <div className="h-16 bg-[#181818] border-b border-white/10 flex items-center justify-between px-6 shrink-0 shadow-sm z-10 print:hidden">
                 <div className="flex items-center gap-4 text-gray-200">
                    <button onClick={() => setPrintTask(null)} className="p-2 hover:bg-white/10 rounded-lg transition-colors">
                       <ChevronLeft size={20} />
                    </button>
                    <h2 className="font-medium text-lg">打印当事人比对验证表</h2>
                    <span className="text-gray-500 text-sm">共 {printTask.parties?.length || 0} 页</span>
                 </div>
                 <div className="flex items-center gap-4">
                    <span className="text-orange-400/90 text-[13px] flex items-center gap-1.5 font-medium bg-orange-500/10 px-3 py-1.5 rounded-md">
                      <AlertCircle size={15} /> 预览环境无法直接唤起打印机，请点击右上角「在新标签页中打开」
                    </span>
                    <button 
                      onClick={() => {
                        window.print();
                      }}
                      className="px-6 py-2 bg-indigo-600 hover:bg-indigo-700 text-white rounded-lg shadow-md flex items-center gap-2 text-sm transition-colors"
                    >
                      <Printer size={16} /> 开始打印
                    </button>
                 </div>
              </div>

              {/* Scrollable Print Pages */}
              <div className="flex-1 overflow-y-auto custom-scrollbar p-10 print:p-0 print:overflow-visible print:bg-white" id="print-root-container">
                 <div className="flex flex-col items-center gap-8 print:block print:gap-0">
                    {(!printTask.parties || printTask.parties.length === 0) ? (
                       <div className="text-gray-400 mt-20 print:hidden">该卷宗暂无当事人信息</div>
                    ) : (
                       printTask.parties.map((party, idx) => (
                          <div key={party.id} className="w-[794px] min-h-[1123px] shrink-0 bg-white text-[#333] shadow-[0_10px_40px_rgba(0,0,0,0.5)] relative font-sans break-after-page print:w-full print:min-h-0 print:shadow-none print:m-0 print:p-[40px] p-[80px_100px]">
                             <h1 className="text-[28px] font-bold text-center mb-16 tracking-widest text-[#333]">人脸比对验证表</h1>
                             
                             <div className="flex gap-6 mb-8 text-[15px] leading-10">
                                <div className="flex-1 flex flex-col pt-2">
                                   <div className="flex border-b border-gray-100 pb-2 mb-2 items-center">
                                      <div className="w-24 font-bold text-gray-600">姓名</div>
                                      <div className="flex-1 text-center font-medium text-black">{party.name}</div>
                                      <div className="w-20 font-bold text-gray-600">性别</div>
                                      <div className="w-20 font-medium text-black">{party.gender || '男'}</div>
                                   </div>
                                   
                                   <div className="flex border-b border-gray-100 pb-2 mb-2 items-center">
                                      <div className="w-24 font-bold text-gray-600">出生日期</div>
                                      <div className="flex-1 text-center font-medium text-black">
                                         {party.identityId ? `${party.identityId.substring(6, 10)}年${party.identityId.substring(10, 12)}月${party.identityId.substring(12, 14)}日` : '1974年09月22日'}
                                      </div>
                                      <div className="w-20 font-bold text-gray-600">民族</div>
                                      <div className="w-20 font-medium text-black">汉</div>
                                   </div>
                                   
                                   <div className="flex border-b border-gray-100 pb-2 mb-2 items-center">
                                      <div className="w-24 font-bold text-gray-600">住址</div>
                                      <div className="flex-1 text-center font-medium text-black leading-relaxed" style={{ textAlignLast: 'center' }}>内蒙古包头市昆都仑区北沙梁朝阳佳苑2栋307号</div>
                                   </div>
                                   
                                   <div className="flex border-b border-gray-100 pb-2 mb-2 items-center">
                                      <div className="w-24 font-bold text-gray-600">身份证号</div>
                                      <div className="flex-1 text-center font-mono font-medium text-black tracking-wider">{party.identityId}</div>
                                   </div>
                                   
                                   <div className="flex border-b border-gray-100 pb-2 items-center">
                                      <div className="w-24 font-bold text-gray-600">签发机关</div>
                                      <div className="flex-1 text-center text-[13px] font-medium text-black overflow-hidden whitespace-nowrap text-ellipsis">包头市公安局昆都仑分局</div>
                                      <div className="w-24 font-bold text-right pr-4 text-gray-600">有效期限</div>
                                      <div className="w-36 text-sm font-medium text-black text-right pr-2">2023.03.24-长期</div>
                                   </div>
                                </div>
                                
                                <div className="w-[124px] h-[164px] border border-gray-300 bg-gray-50 flex-shrink-0 p-1">
                                   <img src={partyIdentityMediaUrls[party.id]?.identityFrontUrl ?? EMPTY_NOTARY_PRINT_IMAGE_URL} className="w-full h-full object-cover grayscale-[0.2]" alt="身份证照片" />
                                </div>
                             </div>
                      
                             <div className="flex text-[15px] mb-12">
                                <div className="w-24 font-bold text-gray-600 pt-[140px] leading-7 text-left">现场采集照<br/>片</div>
                                <div className="pl-[50px] flex-1">
                                   <img src={partyIdentityMediaUrls[party.id]?.faceImageUrl ?? EMPTY_NOTARY_PRINT_IMAGE_URL} className="w-[280px] h-[360px] object-cover border border-gray-300 shadow-sm" alt="现场采集" />
                                </div>
                             </div>
                      
                             <div className="flex text-[15px] mb-8 items-center">
                                <div className="w-24 font-bold text-gray-600">对比分值</div>
                                <div className="w-[180px] text-gray-800 font-mono text-lg">96.66</div>
                                <div className="w-24 font-bold text-right pr-6 text-gray-600">对比结果</div>
                                <div className="flex-1 font-medium text-black text-left pl-6">成功</div>
                             </div>
                      
                             <div className="flex text-[15px] mb-16 items-center">
                                <div className="w-24 font-bold text-gray-600">对比方式</div>
                                <div className="w-[180px] font-medium text-black">人脸对比</div>
                                <div className="w-24 font-bold text-right pr-6 text-gray-600">参考阈值</div>
                                <div className="flex-1 text-gray-800 font-mono text-left pl-6">68</div>
                             </div>
                      
                             <div className="flex text-[15px] items-start pb-12">
                                <div className="w-[120px] text-center font-bold text-gray-600 pt-8">当事人签名:</div>
                                <div className="pt-4 flex-1 text-center pr-20">
                                  <div className="font-[cursive] text-[64px] opacity-90 px-4 text-black italic -rotate-2 inline-block" style={{ fontFamily: "'Dancing Script', 'Caveat', cursive" }}>
                                     {party.name}
                                  </div>
                                </div>
                             </div>
                          </div>
                       ))
                    )}
                 </div>
              </div>
            </motion.div>
          )}
        </AnimatePresence>
        
        <PartyDrawer 
          isOpen={!!editingPartyId}
          onClose={() => setEditingPartyId(null)}
          party={editingPartyId ? selectedTask?.parties?.find(p => p.id === editingPartyId) || null : null}
          onSave={handleSaveParty}
          onSign={setActiveSignParty}
          readOnly={selectedTask?.status === 'COMPLETED' || selectedTask?.status === 'REJECTED'}
        />
        
        <CallOverlay 
          conversationId={activeCall.conversationId ?? `notary-${activeCall.name || 'video-call'}`}
          isOpen={activeCall.isOpen}
          type="video"
          callerName={activeCall.name}
          callerAvatar={DEFAULT_NOTARY_CALLER_AVATAR}
          onClose={() => setActiveCall({ isOpen: false, name: '' })}
        />

        {/* Generic Media Preview */}
        <MediaViewer
          isOpen={previewMedia.isOpen}
          type={previewMedia.type}
          src={previewMedia.url}
          fileName={previewMedia.name}
          onClose={() => setPreviewMedia(prev => ({ ...prev, isOpen: false }))}
        />
      </div>
    </div>
    </>
  );
};

// Polyfill internal User icon since it wasn't extracted to top import explicitly
const User = ({ size, className }: { size: number, className?: string }) => (
  <svg xmlns="http://www.w3.org/2000/svg" width={size} height={size} viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round" className={className}>
    <path d="M19 21v-2a4 4 0 0 0-4-4H9a4 4 0 0 0-4 4v2"/><circle cx="12" cy="7" r="4"/>
  </svg>
)
