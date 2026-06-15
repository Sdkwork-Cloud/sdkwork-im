import type { NotaryDocument, NotaryTask, Party, TimelineEvent } from '@sdkwork/im-pc-types';
import {
  createNotaryApi,
  type CreateNotaryApiOptions,
} from '@sdkwork/notary-app-sdk';
import {
  getAppbaseAppSdkClient,
  getDriveAppSdkClient,
  getNotaryAppSdkClient,
  resolveAppSdkTenantId,
} from '@sdkwork/im-pc-core';

export interface NotaryService {
  getTasks(filters?: { status?: string; searchTerm?: string; pageSize?: number; cursor?: string }): Promise<NotaryTask[]>;
  getTaskById(taskId: string): Promise<NotaryTask | null>;
  getStaff(filters?: { staffRole?: 'notary' | 'assistant' | 'reviewer' | 'approver'; searchTerm?: string; pageSize?: number; cursor?: string }): Promise<NotaryStaffOption[]>;
  createTask(data: CreateNotaryTaskInput): Promise<NotaryTask>;
  updateTaskStatus(taskId: string, status: NotaryTask['status']): Promise<NotaryTask>;
  updateTask(taskId: string, updates: Partial<NotaryTask>): Promise<NotaryTask>;
  addParty(taskId: string, party: Omit<Party, 'id'>): Promise<NotaryTask>;
  addDocument(taskId: string, doc: Omit<NotaryDocument, 'status'>): Promise<NotaryTask>;
  listPartyDocuments(taskId: string, partyId: string): Promise<NotaryDocument[]>;
  uploadPartyDocument(taskId: string, partyId: string, file: unknown): Promise<NotaryTask>;
  createVideoInvite(
    taskId: string,
    partyId: string,
  ): Promise<{ conversationId?: string; inviteUrl?: string; inviteId?: string; expiresAt?: string }>;
  createSignatureInvite(
    taskId: string,
    partyId: string,
  ): Promise<{ inviteUrl?: string; signingUrl?: string; inviteId?: string; expiresAt?: string }>;
  downloadDocuments(taskId: string): Promise<{ downloadUrl?: string; packageId?: string; status?: string }>;
  getDocumentUrl(
    taskId: string,
    documentName: string,
    options?: { disposition?: 'inline' | 'attachment' },
  ): Promise<{ url?: string; downloadUrl?: string; previewUrl?: string }>;
  getPartyIdentityMediaUrls(
    taskId: string,
    partyId: string,
    options?: { disposition?: 'inline' | 'attachment' },
  ): Promise<{ identityFrontUrl?: string; identityBackUrl?: string; faceImageUrl?: string }>;
  deleteTask(taskId: string): Promise<void>;
  removeDocument(taskId: string, documentName: string): Promise<NotaryTask>;
}

export interface CreateNotaryTaskInput extends Partial<NotaryTask> {
  primaryNotaryMembershipId?: string;
  notaryMembershipId?: string;
  selectedNotaryStaff?: unknown;
  notaryStaff?: unknown;
}

export interface NotaryStaffOption {
  membershipId: string;
  userId: string;
  displayName: string;
  status: string;
  roles: string[];
  positions: string[];
  departments: string[];
  notaryStaffRole?: string;
}

export interface CreateChatPcNotaryServiceOptions {
  notary: CreateNotaryApiOptions['notary'];
  drive: unknown;
  commerce?: CreateNotaryApiOptions['commerce'];
  appbase: unknown;
  defaultSkuId?: string;
  skuIdsByType?: Record<string, string>;
  tenantId?: string;
  tenantIdProvider?: () => string | undefined;
}

const DEFAULT_SKU_IDS_BY_TYPE: Record<string, string> = {
  '电子合同存证': 'sku-notary-electronic-contract',
  '知识产权确权公证': 'sku-notary-ipr',
  '电子证据固化': 'sku-notary-evidence',
  '商业秘密确权': 'sku-notary-trade-secret',
  '抽奖过程摇号公证': 'sku-notary-lottery',
  '遗嘱公证': 'sku-notary-will',
  'Electronic Contract Preservation': 'sku-notary-electronic-contract',
  'Intellectual Property Confirmation': 'sku-notary-ipr',
  'Electronic Evidence Preservation': 'sku-notary-evidence',
  'Trade Secret Confirmation': 'sku-notary-trade-secret',
  'Lottery Process Notarization': 'sku-notary-lottery',
  'Will Notarization': 'sku-notary-will',
};

const FILTER_SKU_IDS_BY_TYPE: Record<string, string> = {
  ELECTRONIC: 'sku-notary-electronic-contract',
  IPR: 'sku-notary-ipr',
  EVIDENCE: 'sku-notary-evidence',
  ...DEFAULT_SKU_IDS_BY_TYPE,
};

const VALID_CASE_STATUSES = new Set([
  'PENDING_REVIEW',
  'PROCESSING',
  'COMPLETED',
  'REJECTED',
  'CANCELLED',
  'CREATE_FAILED',
]);

const PARTY_IDENTITY_MATERIAL_CODES = {
  identityFront: 'identity_front',
  identityBack: 'identity_back',
  faceImage: 'face_capture',
} as const;

export function createChatPcNotaryService(
  options: CreateChatPcNotaryServiceOptions,
): NotaryService {
  const notaryApi = createNotaryApi({
    notary: options.notary,
    drive: options.drive as CreateNotaryApiOptions['drive'],
    commerce: options.commerce,
    appbase: options.appbase as CreateNotaryApiOptions['appbase'],
  });
  const defaultSkuId = options.defaultSkuId ?? 'sku-notary-general';
  const skuIdsByType = {
    ...DEFAULT_SKU_IDS_BY_TYPE,
    ...(options.skuIdsByType ?? {}),
  };
  const driveListScope = { driveSpaceType: 'notary' };

  async function loadTask(taskId: string): Promise<NotaryTask> {
    const caseRecord = await notaryApi.getCase(taskId);
    const fileResponse = await notaryApi.listCaseFiles(taskId, driveListScope);
    return mapCaseToTask({
      ...asRecord(caseRecord),
      documents: extractItems(fileResponse),
    });
  }

  async function syncParties(taskId: string, parties: Party[]): Promise<void> {
    const currentTask = mapCaseToTask(await notaryApi.getCase(taskId));
    const currentParties = currentTask.parties ?? [];
    const currentById = new Map(
      currentParties
        .filter((party) => party.id)
        .map((party) => [party.id, party]),
    );
    const retainedIds = new Set<string>();

    for (const party of parties) {
      if (party.id && currentById.has(party.id)) {
        retainedIds.add(party.id);
        await notaryApi.updateParty(taskId, party.id, mapPartyToUpdateRequest(party));
        await syncPartySignature(taskId, party.id, party, currentById.get(party.id));
        await syncPartyIdentityMedia(taskId, party.id, party);
      } else {
        const detail = mapCaseToTask(await notaryApi.addParty(taskId, mapPartyToCreateRequest(party)));
        const createdParty = findMatchingParty(detail.parties ?? [], party);
        if (createdParty?.id) {
          await syncPartySignature(taskId, createdParty.id, party);
          await syncPartyIdentityMedia(taskId, createdParty.id, party);
        }
      }
    }

    await Promise.all(
      currentParties
        .filter((party) => party.id && !retainedIds.has(party.id))
        .map((party) => notaryApi.deleteParty(taskId, party.id)),
    );
  }

  function resolveTenantIdForDriveCommand(): string | undefined {
    const value = options.tenantIdProvider?.() ?? options.tenantId;
    return typeof value === 'string' && value.trim() ? value.trim() : undefined;
  }

  async function syncInitialPartySignatures(taskId: string, parties: Party[]): Promise<void> {
    const signedParties = parties.filter((party) => hasSignature(party.signatureUrl));
    if (signedParties.length === 0) {
      return;
    }
    const createdTask = mapCaseToTask(await notaryApi.getCase(taskId));
    for (const party of signedParties) {
      const createdParty = findMatchingParty(createdTask.parties ?? [], party);
      if (createdParty?.id) {
        await syncPartySignature(taskId, createdParty.id, party, createdParty);
      }
    }
  }

  async function syncInitialPartyIdentityMedia(taskId: string, parties: Party[]): Promise<void> {
    const partiesWithIdentityMedia = parties.filter(
      (party) => extractPartyIdentityDocuments(party).length > 0,
    );
    if (partiesWithIdentityMedia.length === 0) {
      return;
    }
    const createdTask = mapCaseToTask(await notaryApi.getCase(taskId));
    for (const party of partiesWithIdentityMedia) {
      const createdParty = findMatchingParty(createdTask.parties ?? [], party);
      if (createdParty?.id) {
        await syncPartyIdentityMedia(taskId, createdParty.id, party);
      }
    }
  }

  async function syncPartySignature(
    taskId: string,
    partyId: string,
    party: Partial<Party>,
    currentParty?: Party,
  ): Promise<void> {
    if (!hasSignature(party.signatureUrl) || party.signatureUrl === currentParty?.signatureUrl) {
      return;
    }
    await notaryApi.attachPartySignature(taskId, partyId, {
      signatureUrl: party.signatureUrl,
      source: 'sdkwork-im-pc',
    });
  }

  async function syncPartyIdentityMedia(
    taskId: string,
    partyId: string,
    party: Partial<Party>,
  ): Promise<void> {
    const documents = extractPartyIdentityDocuments(party);
    if (documents.length === 0) {
      return;
    }
    await Promise.all(
      documents.map((document) =>
        notaryApi.uploadCaseFile({
          caseId: taskId,
          file: document.file,
          category: 'identity',
          materialCode: document.materialCode,
          partyId,
          source: 'sdkwork-im-pc',
        }),
      ),
    );
  }

  async function syncCaseAssignments(taskId: string, data: CreateNotaryTaskInput): Promise<void> {
    const primaryNotaryMembershipId = resolvePrimaryNotaryMembershipId(data);
    if (!primaryNotaryMembershipId) {
      return;
    }
    await notaryApi.assignCase(taskId, {
      organizationMembershipId: primaryNotaryMembershipId,
      assignmentRole: 'primary_notary',
    });
  }

  return {
    async getTasks(filters?: { status?: string; searchTerm?: string; pageSize?: number; cursor?: string }): Promise<NotaryTask[]> {
      const response = await notaryApi.listCases(resolveListCaseQuery(filters));
      return extractItems(response).map(mapCaseToTask);
    },

    async getTaskById(taskId: string): Promise<NotaryTask | null> {
      try {
        return await loadTask(taskId);
      } catch (error) {
        if (isNotFound(error)) {
          return null;
        }
        throw error;
      }
    },

    async getStaff(filters: { staffRole?: 'notary' | 'assistant' | 'reviewer' | 'approver'; searchTerm?: string; pageSize?: number; cursor?: string } = {}): Promise<NotaryStaffOption[]> {
      const response = await notaryApi.listStaff({
        staffRole: filters.staffRole,
        q: filters.searchTerm,
        pageSize: filters.pageSize,
        cursor: filters.cursor,
      });
      return extractItems(response).map(mapStaffMember);
    },

    async createTask(data: CreateNotaryTaskInput): Promise<NotaryTask> {
      const creationContext = {
        documents: data.documents ?? [],
      };
      const caseRecord = await notaryApi.createCase({
        skuId: resolveSkuId(data.type, defaultSkuId, skuIdsByType),
        title: data.title ?? `${data.type ?? 'General Notary'} Notary Case`,
        applicantName: data.applicant ?? firstPartyName(data.parties) ?? 'Unknown Applicant',
        remarks: data.remarks,
        parties: (data.parties ?? []).map(mapPartyToCreateRequest),
        driveFolderName: data.title ?? data.type ?? 'Notary Case',
        primaryNotaryMembershipId: resolvePrimaryNotaryMembershipId(data),
        idempotencyKey: buildIdempotencyKey(data),
      });
      const createdTask = mapCaseToTask(caseRecord);
      await syncCaseAssignments(createdTask.id, data);
      await syncInitialPartySignatures(createdTask.id, data.parties ?? []);
      await syncInitialPartyIdentityMedia(createdTask.id, data.parties ?? []);
      const documents = creationContext.documents;
      if (documents.length > 0) {
        const createdTaskForDocumentUpload = documents.some(hasDocumentPartyId)
          ? await loadTask(createdTask.id)
          : createdTask;
        const createdPartyIdByClientPartyId = mapCreatedPartyIds(data.parties, createdTaskForDocumentUpload.parties ?? []);
        await Promise.all(
          documents.map((document) =>
            notaryApi.uploadCaseFile({
              caseId: createdTask.id,
              file: (document as any).file ?? document,
              category: document.category,
              materialCode: (document as any).materialCode ?? document.name,
              partyId: resolveCreationDocumentPartyId(document, createdPartyIdByClientPartyId),
              source: 'sdkwork-im-pc',
            }),
          ),
        );
        const refreshedCase = await notaryApi.getCase(createdTask.id);
        const fileResponse = await notaryApi.listCaseFiles(createdTask.id, driveListScope);
        return mapCaseToTask({
          ...asRecord(refreshedCase),
          documents: extractItems(fileResponse),
        });
      }
      return loadTask(createdTask.id);
    },

    async updateTaskStatus(taskId: string, status: NotaryTask['status']): Promise<NotaryTask> {
      if (status === 'PROCESSING') {
        return mapCaseToTask(await notaryApi.acceptCase(taskId));
      }
      if (status === 'REJECTED') {
        return mapCaseToTask(
          await notaryApi.rejectCase(taskId, {
            reason: 'Materials need correction',
          }),
        );
      }
      if (status === 'COMPLETED') {
        return mapCaseToTask(
          await notaryApi.completeCase(taskId, {
            result: 'Manual verification completed',
          }),
        );
      }
      return mapCaseToTask(await notaryApi.updateCase(taskId, { status }));
    },

    async updateTask(taskId: string, updates: Partial<NotaryTask>): Promise<NotaryTask> {
      if (typeof updates.title !== 'undefined' || typeof updates.remarks !== 'undefined') {
        await notaryApi.updateCase(taskId, {
          title: updates.title,
          remarks: updates.remarks,
        });
      }
      if (updates.parties) {
        await syncParties(taskId, updates.parties);
      }
      return loadTask(taskId);
    },

    async addParty(taskId: string, party: Omit<Party, 'id'>): Promise<NotaryTask> {
      const detail = mapCaseToTask(await notaryApi.addParty(taskId, mapPartyToCreateRequest(party)));
      const createdParty = findMatchingParty(detail.parties ?? [], party);
      if (createdParty?.id) {
        await syncPartySignature(taskId, createdParty.id, party);
        await syncPartyIdentityMedia(taskId, createdParty.id, party);
      }
      return loadTask(taskId);
    },

    async addDocument(taskId: string, doc: Omit<NotaryDocument, 'status'>): Promise<NotaryTask> {
      await notaryApi.uploadCaseFile({
        caseId: taskId,
        file: (doc as any).file ?? doc,
        category: doc.category,
        materialCode: (doc as any).materialCode ?? doc.name,
        partyId: (doc as any).partyId,
        source: 'sdkwork-im-pc',
      });
      return loadTask(taskId);
    },

    async listPartyDocuments(taskId: string, partyId: string): Promise<NotaryDocument[]> {
      const fileResponse = await notaryApi.listCaseFiles(taskId, driveListScope);
      return extractItems(fileResponse)
        .map(asRecord)
        .filter((document) => stringValue(document.partyId) === partyId)
        .map(mapDocument);
    },

    async uploadPartyDocument(taskId: string, partyId: string, file: unknown): Promise<NotaryTask> {
      await notaryApi.uploadCaseFile({
        caseId: taskId,
        file,
        category: 'evidence',
        materialCode: resolveFileMaterialCode(file),
        partyId,
        source: 'sdkwork-im-pc',
      });
      return loadTask(taskId);
    },

    async createVideoInvite(
      taskId: string,
      partyId: string,
    ): Promise<{ conversationId?: string; inviteUrl?: string; inviteId?: string; expiresAt?: string }> {
      const invite = asRecord(
        await notaryApi.createPartyVideoInvite(taskId, partyId, {
          purpose: 'identity_verification',
        }),
      );
      return {
        conversationId: optionalString(invite.conversationId),
        inviteUrl: optionalString(invite.inviteUrl),
        inviteId: optionalString(invite.inviteId),
        expiresAt: optionalString(invite.expiresAt),
      };
    },

    async createSignatureInvite(
      taskId: string,
      partyId: string,
    ): Promise<{ inviteUrl?: string; signingUrl?: string; inviteId?: string; expiresAt?: string }> {
      const invite = asRecord(
        await notaryApi.createPartySignatureInvite(taskId, partyId, {
          purpose: 'remote_signature',
        }),
      );
      return {
        inviteUrl: optionalString(invite.inviteUrl),
        signingUrl: optionalString(invite.signingUrl),
        inviteId: optionalString(invite.inviteId),
        expiresAt: optionalString(invite.expiresAt),
      };
    },

    async downloadDocuments(
      taskId: string,
    ): Promise<{ downloadUrl?: string; packageId?: string; status?: string }> {
      const downloadPackage = asRecord(
        await notaryApi.createDownloadPackage(taskId, {}),
      );
      return {
        downloadUrl: optionalString(downloadPackage.downloadUrl),
        packageId: optionalString(downloadPackage.packageId),
        status: optionalString(downloadPackage.status),
      };
    },

    async getDocumentUrl(
      taskId: string,
      documentName: string,
      options: { disposition?: 'inline' | 'attachment' } = {},
    ): Promise<{ url?: string; downloadUrl?: string; previewUrl?: string }> {
      const fileResponse = await notaryApi.listCaseFiles(taskId, driveListScope);
      const document = findDocumentRecord(fileResponse, documentName);
      const nodeId = optionalString(document?.nodeId ?? document?.driveNodeId ?? document?.id);
      if (!nodeId) {
        throw new Error(`Notary document is missing a Drive node id: ${documentName}`);
      }
      const downloadUrlResponse = asRecord(
        await notaryApi.createCaseFileDownloadUrl(taskId, {
          nodeId,
          tenantId: resolveTenantIdForDriveCommand(),
          disposition: options.disposition,
        }),
      );
      const downloadUrl = optionalString(downloadUrlResponse.downloadUrl ?? downloadUrlResponse.url);
      const previewUrl = optionalString(downloadUrlResponse.previewUrl ?? downloadUrlResponse.url ?? downloadUrl);
      return {
        url: optionalString(downloadUrlResponse.url ?? previewUrl ?? downloadUrl),
        downloadUrl,
        previewUrl,
      };
    },

    async getPartyIdentityMediaUrls(
      taskId: string,
      partyId: string,
      options: { disposition?: 'inline' | 'attachment' } = {},
    ): Promise<{ identityFrontUrl?: string; identityBackUrl?: string; faceImageUrl?: string }> {
      const fileResponse = await notaryApi.listCaseFiles(taskId, { ...driveListScope, category: 'identity' });
      const documents = extractItems(fileResponse)
        .map(asRecord)
        .filter((document) => stringValue(document.partyId) === partyId);

      async function resolveMaterialUrl(materialCode: string): Promise<string | undefined> {
        const document = documents.find((item) => stringValue(item.materialCode) === materialCode);
        const nodeId = optionalString(document?.nodeId ?? document?.driveNodeId ?? document?.id);
        if (!nodeId) {
          return undefined;
        }
        const downloadUrlResponse = asRecord(
          await notaryApi.createCaseFileDownloadUrl(taskId, {
            nodeId,
            tenantId: resolveTenantIdForDriveCommand(),
            disposition: options.disposition,
          }),
        );
        return optionalString(
          downloadUrlResponse.previewUrl ?? downloadUrlResponse.downloadUrl ?? downloadUrlResponse.url,
        );
      }

      return {
        identityFrontUrl: await resolveMaterialUrl(PARTY_IDENTITY_MATERIAL_CODES.identityFront),
        identityBackUrl: await resolveMaterialUrl(PARTY_IDENTITY_MATERIAL_CODES.identityBack),
        faceImageUrl: await resolveMaterialUrl(PARTY_IDENTITY_MATERIAL_CODES.faceImage),
      };
    },

    async deleteTask(taskId: string): Promise<void> {
      await notaryApi.updateCase(taskId, { status: 'CANCELLED' });
    },

    async removeDocument(taskId: string, documentName: string): Promise<NotaryTask> {
      const fileResponse = await notaryApi.listCaseFiles(taskId, driveListScope);
      const document = findDocumentRecord(fileResponse, documentName);
      const nodeId = optionalString(document?.nodeId ?? document?.driveNodeId ?? document?.id);
      if (!nodeId) {
        throw new Error(`Notary document is missing a Drive node id: ${documentName}`);
      }
      await notaryApi.deleteCaseFile(taskId, {
        nodeId,
        tenantId: resolveTenantIdForDriveCommand(),
      });
      return loadTask(taskId);
    },
  };
}

function resolveListCaseQuery(filters?: { status?: string; searchTerm?: string; pageSize?: number; cursor?: string }): Record<string, unknown> {
  const selectedFilter = filters?.status && filters.status !== 'ALL' ? filters.status : undefined;
  const query: Record<string, unknown> = {
    q: filters?.searchTerm,
    pageSize: filters?.pageSize ?? 50,
    cursor: filters?.cursor,
  };

  if (!selectedFilter) {
    return query;
  }
  if (VALID_CASE_STATUSES.has(selectedFilter)) {
    return {
      ...query,
      status: selectedFilter,
    };
  }
  const skuId = FILTER_SKU_IDS_BY_TYPE[selectedFilter];
  return skuId
    ? {
        ...query,
        skuId,
      }
    : query;
}

let delegate: NotaryService | null = null;

function getDelegate(): NotaryService {
  if (!delegate) {
    delegate = createChatPcNotaryService({
      notary: getNotaryAppSdkClient().notary,
      drive: getDriveAppSdkClient(),
      appbase: getAppbaseAppSdkClient(),
      defaultSkuId: 'sku-notary-general',
      tenantIdProvider: resolveAppSdkTenantId,
    });
  }
  return delegate;
}

export function resetNotaryService(): void {
  delegate = null;
}

export const notaryService: NotaryService = {
  getTasks(filters) {
    return getDelegate().getTasks(filters);
  },
  getTaskById(taskId) {
    return getDelegate().getTaskById(taskId);
  },
  getStaff(filters) {
    return getDelegate().getStaff(filters);
  },
  createTask(data) {
    return getDelegate().createTask(data);
  },
  updateTaskStatus(taskId, status) {
    return getDelegate().updateTaskStatus(taskId, status);
  },
  updateTask(taskId, updates) {
    return getDelegate().updateTask(taskId, updates);
  },
  addParty(taskId, party) {
    return getDelegate().addParty(taskId, party);
  },
  addDocument(taskId, doc) {
    return getDelegate().addDocument(taskId, doc);
  },
  listPartyDocuments(taskId, partyId) {
    return getDelegate().listPartyDocuments(taskId, partyId);
  },
  uploadPartyDocument(taskId, partyId, file) {
    return getDelegate().uploadPartyDocument(taskId, partyId, file);
  },
  createVideoInvite(taskId, partyId) {
    return getDelegate().createVideoInvite(taskId, partyId);
  },
  createSignatureInvite(taskId, partyId) {
    return getDelegate().createSignatureInvite(taskId, partyId);
  },
  downloadDocuments(taskId) {
    return getDelegate().downloadDocuments(taskId);
  },
  deleteTask(taskId) {
    return getDelegate().deleteTask(taskId);
  },
  removeDocument(taskId, documentName) {
    return getDelegate().removeDocument(taskId, documentName);
  },
  getDocumentUrl(taskId, documentName, options) {
    return getDelegate().getDocumentUrl(taskId, documentName, options);
  },
  getPartyIdentityMediaUrls(taskId, partyId, options) {
    return getDelegate().getPartyIdentityMediaUrls(taskId, partyId, options);
  },
};

function mapCaseToTask(caseRecord: unknown): NotaryTask {
  const record = asRecord(caseRecord);
  const documents = extractItems(record.documents).map(mapDocument);
  const timeline = extractItems(record.timeline).map(mapTimelineEvent);
  const parties = extractItems(record.parties).map(mapParty);
  const task: NotaryTask = {
    id: stringValue(record.caseId ?? record.id),
    createTime: stringValue(record.createTime ?? record.createdAt ?? record.submittedAt),
    processTime: optionalString(record.processTime ?? record.updatedAt),
    applicant: stringValue(record.applicantName ?? record.applicant),
    title: stringValue(record.title),
    notary: stringValue(record.primaryNotaryName ?? record.notary ?? 'System Assigned'),
    remarks: stringValue(record.remarks),
    type: stringValue(record.matterTitle ?? record.type),
    status: mapStatus(record.status),
    fee: numberValue(record.feeAmount ?? record.fee),
    hash: optionalString(record.chainHash ?? record.hash),
    parties,
    documents,
    timeline,
  };

  return Object.assign(task, {
    caseNo: optionalString(record.caseNo),
    caseId: optionalString(record.caseId ?? record.id),
    orderId: optionalString(record.orderId),
    orderItemId: optionalString(record.orderItemId),
    skuId: optionalString(record.skuId),
    driveSpaceId: optionalString(record.driveSpaceId),
    driveFolderNodeId: optionalString(record.driveFolderNodeId),
  });
}

function mapStaffMember(value: unknown): NotaryStaffOption {
  const record = asRecord(value);
  return {
    membershipId: stringValue(record.membershipId ?? record.organizationMembershipId),
    userId: stringValue(record.userId),
    displayName: stringValue(record.displayName ?? record.name ?? record.membershipId),
    status: stringValue(record.status ?? 'active'),
    roles: arrayOfStrings(record.roles),
    positions: arrayOfStrings(record.positions),
    departments: arrayOfStrings(record.departments),
    notaryStaffRole: optionalString(record.notaryStaffRole ?? record.staffRole),
  };
}

function resolvePrimaryNotaryMembershipId(data: CreateNotaryTaskInput): string | undefined {
  const record = asRecord(data);
  const staffRecord = asRecord(record.selectedNotaryStaff ?? record.notaryStaff);
  return optionalString(
    record.primaryNotaryMembershipId
      ?? record.notaryMembershipId
      ?? staffRecord.membershipId
      ?? staffRecord.organizationMembershipId,
  );
}

function mapParty(value: unknown): Party {
  const record = asRecord(value);
  return {
    id: stringValue(record.partyId ?? record.id),
    name: stringValue(record.name),
    role: stringValue(record.role ?? record.partyRole),
    identityId: stringValue(record.identityId ?? record.identityNoMasked ?? record.identityNo),
    phone: optionalString(record.phone ?? record.phoneMasked),
    gender: optionalString(record.gender),
    birthDate: optionalString(record.birthDate),
    address: optionalString(record.address ?? record.addressSnapshot),
    remarks: optionalString(record.remarks),
    identityValidDateStart: optionalString(record.identityValidDateStart),
    identityValidDateEnd: optionalString(record.identityValidDateEnd),
    signatureUrl: optionalString(record.signatureUrl),
  };
}

function mapPartyToCreateRequest(party: Partial<Party>): Record<string, unknown> {
  return {
    name: party.name,
    role: party.role,
    identityNo: party.identityId,
    phone: party.phone,
    gender: party.gender,
    birthDate: party.birthDate,
    address: party.address,
    remarks: party.remarks,
    identityValidDateStart: party.identityValidDateStart,
    identityValidDateEnd: party.identityValidDateEnd,
  };
}

function mapPartyToUpdateRequest(party: Partial<Party>): Record<string, unknown> {
  return {
    name: party.name,
    role: party.role,
    phone: party.phone,
    address: party.address,
    remarks: party.remarks,
    identityValidDateStart: party.identityValidDateStart,
    identityValidDateEnd: party.identityValidDateEnd,
  };
}

function mapDocument(value: unknown): NotaryDocument {
  const record = asRecord(value);
  return Object.assign({
    name: stringValue(record.name ?? record.nodeName),
    size: stringValue(record.size ?? record.sizeLabel),
    status: mapDocumentStatus(record.status ?? record.reviewStatus),
    category: mapDocumentCategory(record.category),
  }, {
    materialCode: optionalString(record.materialCode),
    partyId: optionalString(record.partyId),
    nodeId: optionalString(record.nodeId ?? record.driveNodeId ?? record.id),
    driveNodeId: optionalString(record.driveNodeId ?? record.nodeId ?? record.id),
    driveSpaceId: optionalString(record.driveSpaceId),
    driveSpaceType: optionalString(record.driveSpaceType),
    parentNodeId: optionalString(record.parentNodeId),
  });
}

function mapTimelineEvent(value: unknown): TimelineEvent {
  const record = asRecord(value);
  return {
    time: stringValue(record.time ?? record.occurredAt),
    event: stringValue(record.event ?? record.eventTitle),
    actor: stringValue(record.actor ?? record.actorDisplayName),
  };
}

function resolveSkuId(
  type: string | undefined,
  defaultSkuId: string,
  skuIdsByType: Record<string, string>,
): string {
  const normalized = (type ?? '').trim();
  if (!normalized) {
    return defaultSkuId;
  }
  return skuIdsByType[normalized] ?? defaultSkuId;
}

function buildIdempotencyKey(data: Partial<NotaryTask>): string {
  const seed = [
    data.type,
    data.title,
    data.applicant,
    firstPartyName(data.parties),
    data.createTime,
  ]
    .filter(Boolean)
    .join(':');
  return `notary-chat-pc:${seed || Date.now().toString(36)}`;
}

function firstPartyName(parties: Party[] | undefined): string | undefined {
  return parties?.find((party) => party.name)?.name;
}

function findMatchingParty(parties: Party[], target: Partial<Party>): Party | undefined {
  if (target.id) {
    const byId = parties.find((party) => party.id === target.id);
    if (byId) {
      return byId;
    }
  }
  return parties.find((party) =>
    party.name === target.name
    && party.role === target.role
    && party.identityId === target.identityId
  ) ?? parties.find((party) => party.name === target.name && party.role === target.role);
}

function findDocumentRecord(fileResponse: unknown, documentName: string): Record<string, unknown> | undefined {
  return extractItems(fileResponse)
    .map(asRecord)
    .find((item) => stringValue(item.name ?? item.nodeName) === documentName);
}

function hasDocumentPartyId(document: unknown): boolean {
  return Boolean(optionalString(asRecord(document).partyId));
}

function mapCreatedPartyIds(
  sourceParties: Party[] | undefined,
  createdParties: Party[],
): Map<string, string> {
  const createdPartyIdByClientPartyId = new Map<string, string>();
  for (const sourceParty of sourceParties ?? []) {
    if (!sourceParty.id) {
      continue;
    }
    const createdParty = findMatchingParty(createdParties, sourceParty);
    if (createdParty?.id) {
      createdPartyIdByClientPartyId.set(sourceParty.id, createdParty.id);
    }
  }
  return createdPartyIdByClientPartyId;
}

function resolveCreationDocumentPartyId(
  document: unknown,
  createdPartyIdByClientPartyId: Map<string, string>,
): string | undefined {
  const partyId = optionalString(asRecord(document).partyId);
  return partyId ? createdPartyIdByClientPartyId.get(partyId) ?? partyId : undefined;
}

function resolveFileMaterialCode(file: unknown): string {
  const record = asRecord(file);
  return optionalString(record.name ?? record.fileName ?? record.filename) ?? 'party_attachment';
}

function extractPartyIdentityDocuments(party: Partial<Party>): Array<{ file: unknown; materialCode: string }> {
  const record = asRecord(party);
  const candidates = [
    {
      file: record.identityFrontFile ?? record.identityFrontDataUrl,
      materialCode: PARTY_IDENTITY_MATERIAL_CODES.identityFront,
    },
    {
      file: record.identityBackFile ?? record.identityBackDataUrl,
      materialCode: PARTY_IDENTITY_MATERIAL_CODES.identityBack,
    },
    {
      file: record.faceImageFile ?? record.faceImageDataUrl,
      materialCode: PARTY_IDENTITY_MATERIAL_CODES.faceImage,
    },
  ];
  return candidates
    .filter((item) => Boolean(item.file))
    .map((item) => ({ file: item.file, materialCode: item.materialCode }));
}

function hasSignature(value: unknown): value is string {
  return typeof value === 'string' && value.trim().length > 0;
}

function extractItems(value: unknown): unknown[] {
  if (Array.isArray(value)) {
    return value;
  }
  const record = asRecord(value);
  if (Array.isArray(record.items)) {
    return record.items;
  }
  if (Array.isArray(record.data)) {
    return record.data;
  }
  return [];
}

function arrayOfStrings(value: unknown): string[] {
  return Array.isArray(value)
    ? value.filter((item): item is string => typeof item === 'string')
    : [];
}

function mapStatus(value: unknown): NotaryTask['status'] {
  if (value === 'PROCESSING' || value === 'processing') {
    return 'PROCESSING';
  }
  if (value === 'COMPLETED' || value === 'completed') {
    return 'COMPLETED';
  }
  if (
    value === 'REJECTED' ||
    value === 'rejected' ||
    value === 'CANCELLED' ||
    value === 'cancelled' ||
    value === 'CREATE_FAILED' ||
    value === 'create_failed'
  ) {
    return 'REJECTED';
  }
  return 'PENDING_REVIEW';
}

function mapDocumentStatus(value: unknown): NotaryDocument['status'] {
  if (value === 'verified' || value === 'VERIFIED') {
    return 'verified';
  }
  if (value === 'error' || value === 'ERROR' || value === 'failed') {
    return 'error';
  }
  return 'pending';
}

function mapDocumentCategory(value: unknown): NotaryDocument['category'] {
  if (value === 'identity' || value === 'evidence' || value === 'notary') {
    return value;
  }
  return 'evidence';
}

function asRecord(value: unknown): Record<string, unknown> {
  return value && typeof value === 'object' && !Array.isArray(value)
    ? (value as Record<string, unknown>)
    : {};
}

function stringValue(value: unknown): string {
  return typeof value === 'string' ? value : '';
}

function optionalString(value: unknown): string | undefined {
  return typeof value === 'string' && value.length > 0 ? value : undefined;
}

function numberValue(value: unknown): number {
  if (typeof value === 'number') {
    return value;
  }
  if (typeof value === 'string') {
    const parsed = Number(value);
    return Number.isFinite(parsed) ? parsed : 0;
  }
  return 0;
}

function isNotFound(error: unknown): boolean {
  const record = asRecord(error);
  return record.code === 'not-found' || record.status === 404;
}
