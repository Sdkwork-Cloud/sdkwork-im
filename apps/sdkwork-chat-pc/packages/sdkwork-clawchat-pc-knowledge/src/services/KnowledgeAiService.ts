import type {
  CreateAgentPreviewResponseRequest,
  SdkworkAppClient as SdkworkAgentAppClient,
} from '@sdkwork/agent-app-sdk';
import { getAgentAppSdkClientWithSession } from '@sdkwork/clawchat-pc-core/sdk/agentAppSdkClient';
import {
  readAppSdkSessionTokens,
  resolveAppSdkTenantId,
} from '@sdkwork/clawchat-pc-core/sdk/session';

export interface KnowledgeAiDocumentActionRequest {
  action: string;
  content: string;
  context: string;
  instruction?: string;
}

export interface KnowledgeAiIconRequest {
  description: string;
}

export interface KnowledgeAiService {
  generateKnowledgeBaseIcon(request: KnowledgeAiIconRequest): Promise<string>;
  runDocumentAction(request: KnowledgeAiDocumentActionRequest): Promise<string>;
}

interface KnowledgeAiServiceOptions {
  client?: SdkworkAgentAppClient;
  tenantId?: string;
}

const DEFAULT_TENANT_ID = '0';
const KNOWLEDGE_ASSISTANT_AGENT_ID = 'agent.pc.knowledge.assistant';

function createExecutionId(kind: string): string {
  if (typeof crypto === 'undefined' || typeof crypto.randomUUID !== 'function') {
    throw new Error('Knowledge AI execution id generation requires crypto.randomUUID.');
  }
  const suffix = crypto.randomUUID().replace(/-/gu, '').toLowerCase();
  return `execution.pc.knowledge.${kind}.${suffix}`;
}

function isRecord(value: unknown): value is Record<string, unknown> {
  return typeof value === 'object' && value !== null && !Array.isArray(value);
}

function asString(value: unknown): string | undefined {
  return typeof value === 'string' && value.trim().length > 0 ? value.trim() : undefined;
}

function pickOutputString(output: unknown): string | undefined {
  const direct = asString(output);
  if (direct) {
    return direct;
  }
  if (!isRecord(output)) {
    return undefined;
  }
  for (const key of ['result', 'content', 'text', 'answer', 'message', 'output', 'icon', 'imageUrl', 'image_url']) {
    const value = output[key];
    const normalized = asString(value);
    if (normalized) {
      return normalized;
    }
    if (isRecord(value)) {
      const nested = pickOutputString(value);
      if (nested) {
        return nested;
      }
    }
  }
  return undefined;
}

function resolveTenantId(explicitTenantId?: string): string {
  return explicitTenantId ?? resolveAppSdkTenantId(readAppSdkSessionTokens()) ?? DEFAULT_TENANT_ID;
}

class SdkworkKnowledgeAiService implements KnowledgeAiService {
  constructor(private readonly options: KnowledgeAiServiceOptions = {}) {}

  async generateKnowledgeBaseIcon(request: KnowledgeAiIconRequest): Promise<string> {
    const description = request.description.trim();
    if (!description) {
      throw new Error('Knowledge base icon description is required.');
    }
    return this.executePreview('icon', {
      executionId: createExecutionId('icon'),
      content: `Generate a compact knowledge base icon for: ${description}`,
      debugMode: false,
      memoryEnabled: false,
      inputPayload: {
        operation: 'knowledge_base_icon',
        description,
      },
      requestedAt: new Date().toISOString(),
    });
  }

  async runDocumentAction(request: KnowledgeAiDocumentActionRequest): Promise<string> {
    const instruction = request.instruction?.trim();
    if (!request.content.trim() && !instruction) {
      throw new Error('Document AI action requires selected content or an instruction.');
    }
    return this.executePreview('document', {
      executionId: createExecutionId('document'),
      content: instruction ?? request.content,
      debugMode: false,
      memoryEnabled: true,
      inputPayload: {
        operation: 'knowledge_document_action',
        action: request.action,
        content: request.content,
        context: request.context,
        instruction,
      },
      requestedAt: new Date().toISOString(),
    });
  }

  private async executePreview(kind: string, body: CreateAgentPreviewResponseRequest): Promise<string> {
    const client = this.options.client ?? getAgentAppSdkClientWithSession();
    const response = await client.ai.agents.previewResponses.create(
      KNOWLEDGE_ASSISTANT_AGENT_ID,
      body,
      { tenantId: resolveTenantId(this.options.tenantId) },
    );
    const result = pickOutputString(response.data.outputPayload);
    if (!result) {
      throw new Error(`Knowledge AI ${kind} action did not return usable output.`);
    }
    return result;
  }
}

export function createKnowledgeAiService(options: KnowledgeAiServiceOptions = {}): KnowledgeAiService {
  return new SdkworkKnowledgeAiService(options);
}

export const knowledgeAiService = createKnowledgeAiService();
