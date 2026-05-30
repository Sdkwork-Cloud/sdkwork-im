import type { HttpClient } from '../http/client';
import type { AgentToolCall, AppendAgentResponseDeltaRequest, AutomationExecution, AutomationExecutionRequestResponse, CompleteAgentResponseRequest, CompleteAgentToolCallRequest, RequestAgentToolCallRequest, RequestAutomationExecution, StartAgentResponseRequest, StreamFrame, StreamSession } from '../types';
export declare class AutomationExecutionsApi {
    private client;
    constructor(client: HttpClient);
    /** Request an automation execution */
    create(body: RequestAutomationExecution): Promise<AutomationExecutionRequestResponse>;
    /** Get an automation execution */
    retrieve(executionId: string): Promise<AutomationExecution>;
}
export declare class AutomationAgentToolCallsApi {
    private client;
    constructor(client: HttpClient);
    /** Request an agent tool call */
    create(body: RequestAgentToolCallRequest): Promise<AgentToolCall>;
    /** Complete an agent tool call */
    complete(executionId: string, toolCallId: string, body: CompleteAgentToolCallRequest): Promise<AgentToolCall>;
}
export declare class AutomationAgentResponsesFramesApi {
    private client;
    constructor(client: HttpClient);
    /** Append a frame to an agent response stream */
    create(streamId: string, body: AppendAgentResponseDeltaRequest): Promise<StreamFrame>;
}
export declare class AutomationAgentResponsesApi {
    private client;
    readonly frames: AutomationAgentResponsesFramesApi;
    constructor(client: HttpClient);
    /** Start an agent response stream */
    create(body: StartAgentResponseRequest): Promise<StreamSession>;
    /** Complete an agent response stream */
    complete(streamId: string, body: CompleteAgentResponseRequest): Promise<StreamSession>;
}
export declare class AutomationApi {
    private client;
    readonly agentResponses: AutomationAgentResponsesApi;
    readonly agentToolCalls: AutomationAgentToolCallsApi;
    readonly executions: AutomationExecutionsApi;
    constructor(client: HttpClient);
}
export declare function createAutomationApi(client: HttpClient): AutomationApi;
//# sourceMappingURL=automation.d.ts.map