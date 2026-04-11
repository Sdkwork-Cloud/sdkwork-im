import { backendApiPath } from './paths';
import type { HttpClient } from '../http/client';
import type { QueryParams } from '../types/common';
import type { InboxResponse } from '../types';


export class InboxApi {
  private client: HttpClient;
  
  constructor(client: HttpClient) { 
    this.client = client; 
  }

/** Get inbox entries */
  async getInbox(): Promise<InboxResponse> {
    return this.client.get<InboxResponse>(backendApiPath(`/inbox`));
  }
}

export function createInboxApi(client: HttpClient): InboxApi {
  return new InboxApi(client);
}
