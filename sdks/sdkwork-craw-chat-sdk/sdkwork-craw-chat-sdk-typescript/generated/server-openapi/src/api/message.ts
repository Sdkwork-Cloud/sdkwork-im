import { backendApiPath } from './paths';
import type { HttpClient } from '../http/client';
import type { QueryParams } from '../types/common';
import type { EditMessageRequest, MessageMutationResult } from '../types';


export class MessageApi {
  private client: HttpClient;
  
  constructor(client: HttpClient) { 
    this.client = client; 
  }

/** Edit a posted message */
  async edit(messageId: string | number, body: EditMessageRequest): Promise<MessageMutationResult> {
    return this.client.post<MessageMutationResult>(backendApiPath(`/messages/${messageId}/edit`), body, undefined, undefined, 'application/json');
  }

/** Recall a posted message */
  async recall(messageId: string | number): Promise<MessageMutationResult> {
    return this.client.post<MessageMutationResult>(backendApiPath(`/messages/${messageId}/recall`));
  }
}

export function createMessageApi(client: HttpClient): MessageApi {
  return new MessageApi(client);
}
