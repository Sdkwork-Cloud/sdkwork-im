import { appApiPath } from './paths';
import type { HttpClient } from '../http/client';

import type { AccessProviderHealthRetrieveResponse, ProtocolDownlinkCreateResponse, ProtocolProviderHealthRetrieveResponse, ProtocolUplinkCreateResponse } from '../types';


export class IotProtocolDownlinkApi {
  private client: HttpClient;
  
  constructor(client: HttpClient) { 
    this.client = client; 
  }


/** Ingest IoT protocol downlink */
  async create(): Promise<ProtocolDownlinkCreateResponse> {
    return this.client.post<ProtocolDownlinkCreateResponse>(appApiPath(`/iot/protocol/downlink`));
  }
}

export class IotProtocolUplinkApi {
  private client: HttpClient;
  
  constructor(client: HttpClient) { 
    this.client = client; 
  }


/** Ingest IoT protocol uplink */
  async create(): Promise<ProtocolUplinkCreateResponse> {
    return this.client.post<ProtocolUplinkCreateResponse>(appApiPath(`/iot/protocol/uplink`));
  }
}

export class IotProtocolApi {
  private client: HttpClient;
  public readonly uplink: IotProtocolUplinkApi;
  public readonly downlink: IotProtocolDownlinkApi;
  
  constructor(client: HttpClient) { 
    this.client = client;
    this.uplink = new IotProtocolUplinkApi(client);
    this.downlink = new IotProtocolDownlinkApi(client); 
  }

}

export class IotProtocolProviderHealthApi {
  private client: HttpClient;
  
  constructor(client: HttpClient) { 
    this.client = client; 
  }


/** Retrieve IoT protocol provider health */
  async retrieve(): Promise<ProtocolProviderHealthRetrieveResponse> {
    return this.client.get<ProtocolProviderHealthRetrieveResponse>(appApiPath(`/iot/protocol/provider_health`));
  }
}

export class IotAccessProviderHealthApi {
  private client: HttpClient;
  
  constructor(client: HttpClient) { 
    this.client = client; 
  }


/** Retrieve IoT access provider health */
  async retrieve(): Promise<AccessProviderHealthRetrieveResponse> {
    return this.client.get<AccessProviderHealthRetrieveResponse>(appApiPath(`/iot/access/provider_health`));
  }
}

export class IotApi {
  private client: HttpClient;
  public readonly accessProviderHealth: IotAccessProviderHealthApi;
  public readonly protocolProviderHealth: IotProtocolProviderHealthApi;
  public readonly protocol: IotProtocolApi;
  
  constructor(client: HttpClient) { 
    this.client = client;
    this.accessProviderHealth = new IotAccessProviderHealthApi(client);
    this.protocolProviderHealth = new IotProtocolProviderHealthApi(client);
    this.protocol = new IotProtocolApi(client); 
  }

}

export function createIotApi(client: HttpClient): IotApi {
  return new IotApi(client);
}

function appendQueryString(path: string, rawQueryString: string): string {
  const query = rawQueryString.replace(/^\?+/, '');
  if (!query) {
    return path;
  }
  return path.includes('?') ? `${path}&${query}` : `${path}?${query}`;
}
