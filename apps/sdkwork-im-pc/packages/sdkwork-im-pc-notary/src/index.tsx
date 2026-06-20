import './i18n';
import React, { useCallback, useEffect, useState } from 'react';
import { useTranslation } from 'react-i18next';
import { PenTool } from 'lucide-react';
import type { NotaryDocument, NotaryTask, Party } from '@sdkwork/im-pc-types';
import { CallOverlay, toast } from '@sdkwork/im-pc-chat';
import { MediaViewer } from '@sdkwork/im-pc-commons';
import { CreateNotaryTaskView } from './CreateNotaryTaskView';
import { PartyDrawer } from './PartyDrawer';
import { SignaturePad } from './SignaturePad';
import { DEFAULT_NOTARY_CALLER_AVATAR } from './constants';
import {
  DetailPane,
  type DetailPaneTab,
  NotaryFilterBar,
  NotaryHeader,
  NotaryTaskTable,
  PrintOverlay,
} from './components/list';
import { VideoCallQROverlay } from './components/create';
import { NotaryPickerDrawer } from './components/create/NotaryPickerDrawer';
import { PartyDriveModal } from './components/shared/PartyDriveModal';
import { notaryService, type NotaryStaffOption } from './services/NotaryService';
import type { ActiveCallState, MediaPreviewState, NotaryMatterOption, NotaryStats, PartyIdentityMediaUrls } from './types';
import { renderStatusBadge } from './utils/renderStatusBadge';
import { isNotaryAssigned } from './utils/isNotaryAssigned';

const EMPTY_STATS: NotaryStats = {
  pendingCount: 0,
  completedCount: 0,
  rejectedCount: 0,
  totalCount: 0,
};

export const NotaryView: React.FC = () => {
  const { t } = useTranslation('notary');

  const [activeView, setActiveView] = useState<'list' | 'create'>('list');
  const [searchTerm, setSearchTerm] = useState('');
  const [debouncedSearchTerm, setDebouncedSearchTerm] = useState('');
  const [typeFilter, setTypeFilter] = useState('ALL');
  const [statusFilter, setStatusFilter] = useState('ALL');
  const [selectedTask, setSelectedTask] = useState<NotaryTask | null>(null);
  const [activePaneTab, setActivePaneTab] = useState<DetailPaneTab>('parties');
  const [pageSize, setPageSize] = useState(20);
  const [currentPage, setCurrentPage] = useState(1);
  const [activeDropdown, setActiveDropdown] = useState<string | null>(null);
  const [expandedParty, setExpandedParty] = useState<string | null>(null);
  const [printTask, setPrintTask] = useState<NotaryTask | null>(null);
  const [partyIdentityMediaUrls, setPartyIdentityMediaUrls] = useState<Record<string, PartyIdentityMediaUrls>>({});
  const [expandedPartyMediaUrls, setExpandedPartyMediaUrls] = useState<PartyIdentityMediaUrls | null>(null);

  const [editingPartyId, setEditingPartyId] = useState<string | null>(null);
  const [activeCall, setActiveCall] = useState<ActiveCallState>({ isOpen: false, name: '' });
  const [activeVideoQrParty, setActiveVideoQrParty] = useState<Party | null>(null);
  const [activeDriveParty, setActiveDriveParty] = useState<Party | null>(null);
  const [partyDriveDocuments, setPartyDriveDocuments] = useState<NotaryDocument[]>([]);
  const [partyDriveLoading, setPartyDriveLoading] = useState(false);
  const [activeSignParty, setActiveSignParty] = useState<Party | null>(null);
  const [activeSignInviteUrl, setActiveSignInviteUrl] = useState<string | undefined>(undefined);
  const [previewMedia, setPreviewMedia] = useState<MediaPreviewState>({
    isOpen: false,
    type: 'image',
    url: '',
    name: '',
  });

  const [tasks, setTasks] = useState<NotaryTask[]>([]);
  const [loading, setLoading] = useState(false);
  const [stats, setStats] = useState<NotaryStats>(EMPTY_STATS);
  const [statsLoading, setStatsLoading] = useState(false);
  const [matters, setMatters] = useState<NotaryMatterOption[]>([]);
  const [mattersLoading, setMattersLoading] = useState(false);

  const [detailLoading, setDetailLoading] = useState(false);
  const [printMediaLoading, setPrintMediaLoading] = useState(false);
  const [expandedPartyMediaLoading, setExpandedPartyMediaLoading] = useState(false);
  const [showNotaryPicker, setShowNotaryPicker] = useState(false);
  const [notaryStaffMembers, setNotaryStaffMembers] = useState<NotaryStaffOption[]>([]);

  const getStatusBadge = useCallback(
    (status: NotaryTask['status']) => renderStatusBadge(status, t),
    [t],
  );

  useEffect(() => {
    const handleClickOutside = () => setActiveDropdown(null);
    document.addEventListener('click', handleClickOutside);
    return () => document.removeEventListener('click', handleClickOutside);
  }, []);

  useEffect(() => {
    const timer = window.setTimeout(() => setDebouncedSearchTerm(searchTerm.trim()), 300);
    return () => window.clearTimeout(timer);
  }, [searchTerm]);

  const fetchStats = useCallback(async () => {
    setStatsLoading(true);
    try {
      setStats(await notaryService.getDashboardStatistics());
    } catch (error) {
      console.error('Failed to fetch notary dashboard statistics:', error);
      toast(t('toast.statsLoadFailed'), 'error');
    } finally {
      setStatsLoading(false);
    }
  }, [t]);

  const fetchTasks = useCallback(async () => {
    setLoading(true);
    try {
      const data = await notaryService.getTasks({
        businessType: typeFilter,
        status: statusFilter,
        searchTerm: debouncedSearchTerm,
      });
      setTasks(data);
    } catch (error) {
      console.error('Failed to fetch tasks:', error);
      toast(t('toast.tasksLoadFailed'), 'error');
    } finally {
      setLoading(false);
    }
  }, [debouncedSearchTerm, statusFilter, typeFilter, t]);

  const fetchMatters = useCallback(async () => {
    setMattersLoading(true);
    try {
      setMatters(await notaryService.getMatters());
    } catch (error) {
      console.error('Failed to fetch notary matters:', error);
      toast(t('toast.wizardLoadFailed'), 'error');
    } finally {
      setMattersLoading(false);
    }
  }, [t]);

  useEffect(() => {
    if (activeView === 'list') {
      void fetchStats();
      void fetchMatters();
      void fetchTasks();
      setCurrentPage(1);
    }
  }, [activeView, fetchMatters, fetchStats, fetchTasks]);

  useEffect(() => {
    if (activeView === 'list') {
      setCurrentPage(1);
    }
  }, [activeView, debouncedSearchTerm, typeFilter, statusFilter]);

  useEffect(() => {
    if (!selectedTask) {
      return;
    }
    const liveTask = tasks.find((task) => task.id === selectedTask.id);
    if (liveTask && liveTask.status !== selectedTask.status) {
      setSelectedTask(liveTask);
    }
  }, [selectedTask, tasks]);

  useEffect(() => {
    let disposed = false;
    const loadPartyIdentityMediaUrls = async () => {
      if (!printTask?.id || !printTask.parties?.length) {
        setPartyIdentityMediaUrls({});
        setPrintMediaLoading(false);
        return;
      }
      setPrintMediaLoading(true);
      try {
        const entries = await Promise.all(
          printTask.parties.map(async (party) => [
            party.id,
            await notaryService.getPartyIdentityMediaUrls(printTask.id, party.id, { disposition: 'inline' }),
          ] as const),
        );
        if (!disposed) {
          setPartyIdentityMediaUrls(Object.fromEntries(entries));
        }
      } catch (error) {
        console.error('Failed to load party identity media for print:', error);
        toast(t('toast.printMediaLoadFailed'), 'error');
        if (!disposed) {
          setPartyIdentityMediaUrls({});
        }
      } finally {
        if (!disposed) {
          setPrintMediaLoading(false);
        }
      }
    };
    void loadPartyIdentityMediaUrls();
    return () => {
      disposed = true;
    };
  }, [printTask, t]);

  useEffect(() => {
    let disposed = false;
    const loadExpandedPartyMedia = async () => {
      if (!selectedTask?.id || !expandedParty) {
        setExpandedPartyMediaUrls(null);
        setExpandedPartyMediaLoading(false);
        return;
      }
      setExpandedPartyMediaLoading(true);
      try {
        const urls = await notaryService.getPartyIdentityMediaUrls(selectedTask.id, expandedParty, {
          disposition: 'inline',
        });
        if (!disposed) {
          setExpandedPartyMediaUrls(urls);
        }
      } catch (error) {
        console.error('Failed to load expanded party identity media:', error);
        if (!disposed) {
          setExpandedPartyMediaUrls(null);
        }
      } finally {
        if (!disposed) {
          setExpandedPartyMediaLoading(false);
        }
      }
    };
    void loadExpandedPartyMedia();
    return () => {
      disposed = true;
    };
  }, [selectedTask?.id, expandedParty]);

  useEffect(() => {
    void loadPartyDriveDocuments();
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [selectedTask?.id, activeDriveParty?.id]);

  const loadPartyDriveDocuments = async () => {
    if (!selectedTask?.id || !activeDriveParty?.id) {
      setPartyDriveDocuments([]);
      return;
    }
    setPartyDriveLoading(true);
    try {
      setPartyDriveDocuments(await notaryService.listPartyDocuments(selectedTask.id, activeDriveParty.id));
    } catch (error) {
      console.error('Failed to load party Drive documents:', error);
      toast(t('toast.partyDriveLoadFailed'), 'error');
    } finally {
      setPartyDriveLoading(false);
    }
  };

  const handleSelectTask = async (task: NotaryTask) => {
    setSelectedTask(task);
    setActivePaneTab('parties');
    setExpandedParty(null);
    setDetailLoading(true);
    try {
      const detail = await notaryService.getTaskById(task.id);
      if (detail) {
        setSelectedTask(detail);
      }
    } catch (error) {
      console.error('Failed to load task detail:', error);
      toast(t('toast.taskDetailLoadFailed'), 'error');
    } finally {
      setDetailLoading(false);
    }
  };

  const handleCreateSuccess = async () => {
    setActiveView('list');
    await Promise.all([fetchStats(), fetchMatters(), fetchTasks()]);
  };

  const handleMonthlyReport = async () => {
    toast(t('toast.monthlyReportDownloading'), 'success');
    try {
      const report = await notaryService.getMonthlyReport();
      if (report.downloadUrl) {
        toast(t('toast.monthlyReportReady'), 'success');
        window.open(report.downloadUrl, '_blank');
      }
    } catch (error) {
      console.error('Failed to download monthly report:', error);
      toast(t('toast.monthlyReportFailed'), 'error');
    }
  };

  const handleEditParty = (party: Party) => {
    setEditingPartyId(party.id);
  };

  const handleSaveParty = async (partyData: Party) => {
    if (!selectedTask) {
      return;
    }
    const targetId = editingPartyId || partyData.id;
    const newParties = selectedTask.parties?.map((party) => (party.id === targetId ? partyData : party)) || [];
    setSelectedTask({ ...selectedTask, parties: newParties });
    setEditingPartyId(null);
    try {
      const savedTask = await notaryService.updateTask(selectedTask.id, { parties: newParties });
      setSelectedTask(savedTask);
      setTasks((prev) => prev.map((task) => (task.id === savedTask.id ? savedTask : task)));
    } catch (error) {
      console.error('Failed to save edited party', error);
      toast(t('toast.partySaveFailed'), 'error');
    }
  };

  const handlePartyDriveUpload = async (files: File[]) => {
    if (!selectedTask?.id || !activeDriveParty?.id || files.length === 0) {
      return;
    }
    setPartyDriveLoading(true);
    try {
      let refreshedTask = selectedTask;
      for (const file of files) {
        refreshedTask = await notaryService.uploadPartyDocument(selectedTask.id, activeDriveParty.id, file);
      }
      setSelectedTask(refreshedTask);
      setTasks((prev) => prev.map((task) => (task.id === refreshedTask.id ? refreshedTask : task)));
      setPartyDriveDocuments(await notaryService.listPartyDocuments(refreshedTask.id, activeDriveParty.id));
      toast(t('toast.attachmentUploadedToDrive'), 'success');
    } catch (error) {
      console.error('Failed to upload party Drive document:', error);
      toast(t('toast.attachmentUploadFailed'), 'error');
    } finally {
      setPartyDriveLoading(false);
    }
  };

  const handleStatusChange = async (status: NotaryTask['status']) => {
    if (!selectedTask) {
      return;
    }
    try {
      const updated = await notaryService.updateTaskStatus(selectedTask.id, status);
      setSelectedTask(updated);
      setTasks((prev) => prev.map((task) => (task.id === updated.id ? updated : task)));
      await Promise.all([fetchStats(), fetchTasks()]);
    } catch (error) {
      console.error('Failed to update task status:', error);
      toast(t('toast.statusUpdateFailed'), 'error');
    }
  };

  const openNotaryPicker = async () => {
    try {
      if (notaryStaffMembers.length === 0) {
        setNotaryStaffMembers(await notaryService.getStaff({ staffRole: 'notary' }));
      }
      setShowNotaryPicker(true);
    } catch (error) {
      console.error('Failed to load notary staff:', error);
      toast(t('toast.wizardLoadFailed'), 'error');
    }
  };

  const handleAssignNotary = async (staff: NotaryStaffOption) => {
    if (!selectedTask) {
      return;
    }
    try {
      const updated = await notaryService.assignNotary(selectedTask.id, staff.membershipId);
      setSelectedTask(updated);
      setTasks((prev) => prev.map((task) => (task.id === updated.id ? updated : task)));
      setShowNotaryPicker(false);
      toast(t('toast.notaryAssigned'), 'success');
    } catch (error) {
      console.error('Failed to assign notary:', error);
      toast(t('toast.notaryAssignFailed'), 'error');
    }
  };

  const handlePreviewDocument = async (doc: NotaryDocument) => {
    if (!selectedTask) {
      return;
    }
    try {
      const { previewUrl, downloadUrl, url } = await notaryService.getDocumentUrl(selectedTask.id, doc.name, {
        disposition: 'inline',
      });
      const resolvedUrl = previewUrl ?? downloadUrl ?? url;
      if (resolvedUrl) {
        setPreviewMedia({
          isOpen: true,
          type: doc.name.endsWith('.mp4') ? 'video' : 'image',
          url: resolvedUrl,
          name: doc.name,
        });
      }
    } catch (error) {
      console.error('Failed to preview document:', error);
      toast(t('toast.documentPreviewFailed'), 'error');
    }
  };

  const handleDownloadDocument = async (doc: NotaryDocument) => {
    if (!selectedTask) {
      return;
    }
    try {
      toast(t('detail.downloadingFile', { name: doc.name }), 'success');
      const { downloadUrl, url } = await notaryService.getDocumentUrl(selectedTask.id, doc.name, {
        disposition: 'attachment',
      });
      const resolvedUrl = downloadUrl ?? url;
      if (resolvedUrl) {
        window.open(resolvedUrl, '_blank');
      }
    } catch (error) {
      console.error('Failed to download document:', error);
      toast(t('toast.documentDownloadFailed'), 'error');
    }
  };

  const handleDownloadAllMaterials = async () => {
    if (!selectedTask) {
      return;
    }
    try {
      toast(t('toast.packagingAttachments'), 'success');
      const { downloadUrl } = await notaryService.downloadDocuments(selectedTask.id);
      if (downloadUrl) {
        toast(t('toast.attachmentPackageReady'), 'success');
        window.open(downloadUrl, '_blank');
      }
    } catch (error) {
      console.error('Failed to download all materials:', error);
      toast(t('toast.documentDownloadFailed'), 'error');
    }
  };

  const handleSignParty = async (party: Party) => {
    if (!selectedTask) {
      return;
    }
    try {
      const invite = await notaryService.createSignatureInvite(selectedTask.id, party.id);
      setActiveSignInviteUrl(invite.signingUrl ?? invite.inviteUrl);
      setActiveSignParty(party);
    } catch (error) {
      console.error('Failed to create signature invite:', error);
      toast(t('toast.signatureInviteFailed'), 'error');
    }
  };

  const handleVideoCall = async (party: Party) => {
    if (!selectedTask) {
      return;
    }
    if (!selectedTask.notary || !isNotaryAssigned(selectedTask, [t('createTask.systemAssigned'), t('createTask.unassigned')])) {
      toast(t('createTask.selectNotaryFirst'), 'error');
      return;
    }
    try {
      const invite = await notaryService.createVideoInvite(selectedTask.id, party.id);
      setActiveCall({
        isOpen: true,
        name: party.name,
        conversationId: invite.conversationId,
        inviteUrl: invite.inviteUrl,
      });
      setActiveVideoQrParty(party);
    } catch (error) {
      console.error('Failed to create video invite:', error);
      toast(t('toast.videoCallFailed'), 'error');
    }
  };

  const openPrintTask = async (task: NotaryTask) => {
    setDetailLoading(true);
    try {
      const detail = await notaryService.getTaskById(task.id);
      setPrintTask(detail ?? task);
    } catch (error) {
      console.error('Failed to load print task detail:', error);
      toast(t('toast.taskDetailLoadFailed'), 'error');
      setPrintTask(task);
    } finally {
      setDetailLoading(false);
    }
  };

  if (activeView === 'create') {
    return <CreateNotaryTaskView onBack={() => setActiveView('list')} onSuccess={handleCreateSuccess} />;
  }

  if (activeSignParty) {
    return (
      <SignaturePad
        title={t('createTask.partyOnlineSignature', { name: activeSignParty.name })}
        subtitle={(
          <>
            <PenTool size={16} />
            {' '}
            {t('createTask.pleaseWriteName', { name: activeSignParty.name })}
          </>
        )}
        mobileSignatureUrl={activeSignInviteUrl}
        onCancel={() => {
          setActiveSignInviteUrl(undefined);
          setActiveSignParty(null);
        }}
        onSave={(imgUrl) => {
          toast(t('signaturePad.signatureSaved'), 'success');
          void handleSaveParty({ ...activeSignParty, signatureUrl: imgUrl });
          setActiveSignInviteUrl(undefined);
          setActiveSignParty(null);
        }}
      />
    );
  }

  const totalPages = Math.ceil(tasks.length / pageSize) || 1;
  const paginatedTasks = tasks.slice((currentPage - 1) * pageSize, currentPage * pageSize);

  return (
    <>
      <div className="flex-1 flex flex-col bg-[#1e1e1e] min-w-0 min-h-0 overflow-hidden relative print:block print:overflow-visible print:h-auto print:min-h-0">
        <div className={`flex w-full h-full relative print:block print:overflow-visible print:h-auto print:min-h-0 ${printTask ? 'print:hidden' : ''}`}>
          <div className="flex-1 w-full h-full p-4 sm:p-6 lg:p-8 flex flex-col gap-6 min-h-0 relative">
            <NotaryHeader
              stats={stats}
              loading={statsLoading}
              onCreateTask={() => setActiveView('create')}
              onMonthlyReport={() => void handleMonthlyReport()}
            />

            <NotaryFilterBar
              searchTerm={searchTerm}
              typeFilter={typeFilter}
              statusFilter={statusFilter}
              matters={matters}
              mattersLoading={mattersLoading}
              onSearchChange={setSearchTerm}
              onTypeFilterChange={setTypeFilter}
              onStatusFilterChange={setStatusFilter}
            />

            <NotaryTaskTable
              tasks={tasks}
              paginatedTasks={paginatedTasks}
              loading={loading}
              selectedTask={selectedTask}
              activeDropdown={activeDropdown}
              pageSize={pageSize}
              currentPage={currentPage}
              totalPages={totalPages}
              getStatusBadge={getStatusBadge}
              onSelectTask={(task) => void handleSelectTask(task)}
              onDropdownToggle={setActiveDropdown}
              onDownloadMaterials={async (task) => {
                try {
                  toast(t('toast.downloadingMaterials'), 'success');
                  const { downloadUrl } = await notaryService.downloadDocuments(task.id);
                  if (downloadUrl) {
                    window.open(downloadUrl, '_blank');
                  }
                } catch (error) {
                  console.error('Failed to download materials:', error);
                  toast(t('toast.documentDownloadFailed'), 'error');
                }
              }}
              onPrintPartyInfo={(task) => void openPrintTask(task)}
              onDeleteTask={async (task) => {
                if (!window.confirm(t('table.cancelConfirm'))) {
                  return;
                }
                try {
                  await notaryService.deleteTask(task.id);
                  setTasks((prev) => prev.filter((item) => item.id !== task.id));
                  if (selectedTask?.id === task.id) {
                    setSelectedTask(null);
                  }
                  toast(t('toast.taskDeleted'), 'success');
                  void fetchStats();
                } catch (error) {
                  console.error('Failed to delete task:', error);
                  toast(t('toast.cancelFailed'), 'error');
                }
              }}
              onPageSizeChange={(size) => {
                setPageSize(size);
                setCurrentPage(1);
              }}
              onPageChange={setCurrentPage}
            />

            {loading && (
              <div className="absolute inset-0 z-30 flex items-center justify-center bg-black/20 backdrop-blur-[1px] pointer-events-none">
                <div className="px-4 py-2 rounded-lg bg-[#2b2b2d] border border-white/10 text-sm text-gray-300">
                  {t('stats.loading')}
                </div>
              </div>
            )}
          </div>

          {selectedTask && (
            <DetailPane
              task={selectedTask}
              loading={detailLoading}
              activeTab={activePaneTab}
              expandedParty={expandedParty}
              expandedPartyMediaLoading={expandedPartyMediaLoading}
              expandedPartyMediaUrls={expandedPartyMediaUrls}
              getStatusBadge={getStatusBadge}
              canAssignNotary={selectedTask.status !== 'COMPLETED' && selectedTask.status !== 'REJECTED'}
              onAssignNotary={() => void openNotaryPicker()}
              onClose={() => setSelectedTask(null)}
              onTabChange={setActivePaneTab}
              onExpandParty={(partyId) => setExpandedParty((current) => (current === partyId ? null : partyId))}
              onEditParty={handleEditParty}
              onSignParty={(party) => void handleSignParty(party)}
              onDriveParty={setActiveDriveParty}
              onVideoCall={(party) => void handleVideoCall(party)}
              onPrint={() => void openPrintTask(selectedTask)}
              onStatusChange={(status) => void handleStatusChange(status)}
              onPreviewDocument={(doc) => void handlePreviewDocument(doc)}
              onDownloadDocument={(doc) => void handleDownloadDocument(doc)}
              onDownloadAllMaterials={() => void handleDownloadAllMaterials()}
            />
          )}

          <NotaryPickerDrawer
            isOpen={showNotaryPicker}
            staff={notaryStaffMembers}
            selected={
              notaryStaffMembers.find((member) => member.membershipId === selectedTask?.primaryNotaryMembershipId) ?? null
            }
            onClose={() => setShowNotaryPicker(false)}
            onSelect={(staff) => void handleAssignNotary(staff)}
          />

          <PartyDriveModal
            isOpen={Boolean(activeDriveParty && selectedTask)}
            party={activeDriveParty}
            documents={partyDriveDocuments}
            loading={partyDriveLoading}
            onClose={() => setActiveDriveParty(null)}
            onUpload={(files) => void handlePartyDriveUpload(files)}
            onPreview={(doc) => {
              if ('nodeId' in doc || 'driveNodeId' in doc) {
                void handlePreviewDocument(doc as NotaryDocument);
              }
            }}
            onDownload={(doc) => void handleDownloadDocument(doc)}
          />

          {printTask && (
            <PrintOverlay
              task={printTask}
              partyMediaUrls={partyIdentityMediaUrls}
              loading={printMediaLoading}
              onClose={() => setPrintTask(null)}
            />
          )}

          <PartyDrawer
            isOpen={Boolean(editingPartyId)}
            onClose={() => setEditingPartyId(null)}
            party={editingPartyId ? selectedTask?.parties?.find((party) => party.id === editingPartyId) || null : null}
            onSave={handleSaveParty}
            onSign={setActiveSignParty}
            readOnly={selectedTask?.status === 'COMPLETED' || selectedTask?.status === 'REJECTED'}
          />

          <CallOverlay
            conversationId={activeCall.conversationId ?? `notary-${activeCall.name}`}
            isOpen={activeCall.isOpen}
            type="video"
            callerName={activeCall.name}
            callerAvatar={DEFAULT_NOTARY_CALLER_AVATAR}
            onClose={() => {
              setActiveCall({ isOpen: false, name: '' });
              setActiveVideoQrParty(null);
            }}
          />

          <VideoCallQROverlay
            isOpen={Boolean(activeVideoQrParty)}
            party={activeVideoQrParty}
            inviteUrl={activeCall.inviteUrl}
            onClose={() => setActiveVideoQrParty(null)}
          />

          <MediaViewer
            isOpen={previewMedia.isOpen}
            type={previewMedia.type}
            src={previewMedia.url}
            fileName={previewMedia.name}
            onClose={() => setPreviewMedia((prev) => ({ ...prev, isOpen: false }))}
          />
        </div>
      </div>
    </>
  );
};
