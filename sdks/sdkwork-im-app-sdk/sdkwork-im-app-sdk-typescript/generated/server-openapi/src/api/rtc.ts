import { appApiPath } from './paths';
import type { HttpClient } from '../http/client';

import type { ProviderCallbacksCreateResponse, ProviderHealthRetrieveResponse } from '../types';


export class RtcProviderHealthApi {
  private client: HttpClient;
  
  constructor(client: HttpClient) { 
    this.client = client; 
  }


/** Retrieve RTC provider health */
  async retrieve(): Promise<ProviderHealthRetrieveResponse> {
    return this.client.get<ProviderHealthRetrieveResponse>(appApiPath(`/rtc/provider_health`));
  }
}

export class RtcProviderCallbacksApi {
  private client: HttpClient;
  
  constructor(client: HttpClient) { 
    this.client = client; 
  }


/** Map RTC provider callback */
  async create(): Promise<ProviderCallbacksCreateResponse> {
    return this.client.post<ProviderCallbacksCreateResponse>(appApiPath(`/rtc/provider_callbacks`));
  }
}

export class RtcApi {
  private client: HttpClient;
  public readonly providerCallbacks: RtcProviderCallbacksApi;
  public readonly providerHealth: RtcProviderHealthApi;
  
  constructor(client: HttpClient) { 
    this.client = client;
    this.providerCallbacks = new RtcProviderCallbacksApi(client);
    this.providerHealth = new RtcProviderHealthApi(client); 
  }

}

export function createRtcApi(client: HttpClient): RtcApi {
  return new RtcApi(client);
}

function appendQueryString(path: string, rawQueryString: string): string {
  const query = rawQueryString.replace(/^\?+/, '');
  if (!query) {
    return path;
  }
  return path.includes('?') ? `${path}&${query}` : `${path}?${query}`;
}
