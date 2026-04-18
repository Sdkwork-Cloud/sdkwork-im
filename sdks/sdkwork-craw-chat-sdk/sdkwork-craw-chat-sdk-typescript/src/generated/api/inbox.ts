import { backendApiPath } from './paths.js';
import type { HttpClient } from '../http/client.js';
import type { QueryParams } from '../types/common.js';
import type { InboxResponse } from '../types/index.js';


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
