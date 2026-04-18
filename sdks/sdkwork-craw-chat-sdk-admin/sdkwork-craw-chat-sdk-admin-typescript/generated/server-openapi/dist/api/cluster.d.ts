import type { HttpClient } from '../http/client';
export declare class ClusterApi {
    private client;
    constructor(client: HttpClient);
    /** Post nodes {node_id} activate */
    postApiV1ControlNodesIdActivate(nodeId: string | number): Promise<void>;
    /** Post nodes {node_id} drain */
    postApiV1ControlNodesIdDrain(nodeId: string | number): Promise<void>;
    /** Post nodes {node_id} routes migrate */
    postApiV1ControlNodesIdRoutesMigrate(nodeId: string | number): Promise<void>;
}
export declare function createClusterApi(client: HttpClient): ClusterApi;
//# sourceMappingURL=cluster.d.ts.map