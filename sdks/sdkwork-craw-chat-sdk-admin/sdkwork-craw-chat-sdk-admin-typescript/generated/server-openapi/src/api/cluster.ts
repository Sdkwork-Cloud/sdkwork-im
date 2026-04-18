import { backendApiPath } from './paths';
import type { HttpClient } from '../http/client';
import type { QueryParams } from '../types/common';


export class ClusterApi {
  private client: HttpClient;
  
  constructor(client: HttpClient) { 
    this.client = client; 
  }

/** Post nodes {node_id} activate */
  async postApiV1ControlNodesIdActivate(nodeId: string | number): Promise<void> {
    return this.client.post<void>(backendApiPath(`/api/v1/control/nodes/${nodeId}/activate`));
  }

/** Post nodes {node_id} drain */
  async postApiV1ControlNodesIdDrain(nodeId: string | number): Promise<void> {
    return this.client.post<void>(backendApiPath(`/api/v1/control/nodes/${nodeId}/drain`));
  }

/** Post nodes {node_id} routes migrate */
  async postApiV1ControlNodesIdRoutesMigrate(nodeId: string | number): Promise<void> {
    return this.client.post<void>(backendApiPath(`/api/v1/control/nodes/${nodeId}/routes/migrate`));
  }
}

export function createClusterApi(client: HttpClient): ClusterApi {
  return new ClusterApi(client);
}
