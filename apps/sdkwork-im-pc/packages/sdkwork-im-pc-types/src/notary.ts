export interface Party {
  id: string;
  name: string;
  role: string;
  identityId: string;
  phone?: string;
  gender?: string;
  birthDate?: string;
  address?: string;
  remarks?: string;
  identityValidDateStart?: string;
  identityValidDateEnd?: string;
  signatureUrl?: string;
  ethnicity?: string;
  identityIssuingAuthority?: string;
  identityVerificationScore?: number;
  identityVerificationStatus?: string;
  faceCaptureTime?: string;
  /** Local identity media captured before case persistence */
  identityFrontFile?: File;
  identityBackFile?: File;
  faceImageDataUrl?: string;
}

export interface NotaryDocument {
  name: string;
  size: string;
  status: 'verified' | 'pending' | 'error';
  category: 'identity' | 'evidence' | 'notary';
  materialCode?: string;
  partyId?: string;
  file?: File;
  nodeId?: string;
  driveNodeId?: string;
}

export interface TimelineEvent {
  time: string;
  event: string;
  actor: string;
}

export interface NotaryTask {
  id: string;
  createTime: string;
  processTime?: string;
  applicant: string;
  title: string;
  notary: string;
  remarks: string;
  type: string;
  status: 'PENDING_REVIEW' | 'PROCESSING' | 'COMPLETED' | 'REJECTED';
  fee: number;
  hash?: string;
  parties?: Party[];
  documents: NotaryDocument[];
  timeline: TimelineEvent[];
  caseNo?: string;
  caseId?: string;
  orderId?: string;
  orderItemId?: string;
  skuId?: string;
  driveSpaceId?: string;
  driveFolderNodeId?: string;
  primaryNotaryMembershipId?: string;
}
