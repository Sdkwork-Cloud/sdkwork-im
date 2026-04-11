export interface UpdateReadCursorRequest {
  readSeq: number;
  lastReadMessageId?: string;
}
