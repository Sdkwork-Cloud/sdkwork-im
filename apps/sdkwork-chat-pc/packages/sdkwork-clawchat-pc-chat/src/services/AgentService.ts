import {
  getAgentAppSdkClientWithSession,
  type SdkworkAgentAppClient,
} from '@sdkwork/clawchat-pc-core';
import type {
  AgentRecord,
  CreateAgentProviderBindingRequest,
  CreateAgentRequest,
  UpdateAgentRequest,
} from '@sdkwork/agent-app-sdk';

export interface AgentConfig {
  id?: string;
  name: string;
  description: string;
  avatar: string;
  type: 'normal' | 'independent';
  systemPrompt?: string;
  knowledgeBaseIds?: string[];
  author?: string;
  users?: string;
  color?: string;
  iconName?: string;
  categoryId?: string;
  welcomeMessage?: string;
}

export interface AgentPreviewResponseRequest {
  config: AgentConfig;
  content: string;
  debugMode?: boolean;
  memoryEnabled?: boolean;
  model?: string;
  temperature?: number;
}

export interface AgentPreviewResponse {
  content: string;
  executionId: string;
  outputPayload?: unknown;
}

export interface AgentPromptOptimizeRequest {
  config: AgentConfig;
  prompt: string;
}

export interface AgentPromptOptimizeResult {
  executionId: string;
  optimizedPrompt: string;
  outputPayload?: unknown;
}

export interface AgentService {
  createAgent(config: AgentConfig): Promise<AgentConfig>;
  updateAgent(id: string, config: Partial<AgentConfig>): Promise<AgentConfig>;
  publishAgent(id: string): Promise<void>;
  getAgents(): Promise<AgentConfig[]>;
  getMarketAgents(): Promise<AgentConfig[]>;
  deleteAgent(id: string): Promise<void>;
  requestPreviewResponse(request: AgentPreviewResponseRequest): Promise<AgentPreviewResponse>;
  optimizePrompt(request: AgentPromptOptimizeRequest): Promise<AgentPromptOptimizeResult>;
}

export class AgentRuntimeExecutionUnavailableError extends Error {
  constructor(operation: string, executionId: string) {
    super(`Agent runtime execution did not return a ${operation}: ${executionId}.`);
    this.name = 'AgentRuntimeExecutionUnavailableError';
  }
}

type AgentCatalogScope = 'market' | 'mine';
type RecordLike = Record<string, unknown>;

const DEFAULT_AGENT_TENANT_ID = '0';
const DEFAULT_AGENT_ORGANIZATION_ID = '0';
const DEFAULT_AGENT_OWNER_USER_ID = '0';
const DEFAULT_AGENT_BINDING_ID = 'binding.manifest.default';
const DEFAULT_AGENT_PROVIDER_ID = 'provider.agent.manifest';
const DEFAULT_AGENT_CONFIGURATION_PROFILE_ID = 'profile.agent.manifest.default';
const DEFAULT_AGENT_PROVIDER_CAPABILITIES = ['model.chat', 'tool.invoke'] as const;
const AGENT_UI_CONFIG_CONSTRAINT_PREFIX = 'sdkwork.agent.pc.config:';

function createExecutionId(prefix: string): string {
  if (typeof crypto !== 'undefined' && typeof crypto.randomUUID === 'function') {
    return `${prefix}.${crypto.randomUUID().toLowerCase()}`;
  }
  return `${prefix}.${Math.trunc(performance.now()).toString(36)}`;
}

function isRecord(value: unknown): value is RecordLike {
  return typeof value === 'object' && value !== null && !Array.isArray(value);
}

function asString(value: unknown): string | undefined {
  return typeof value === 'string' && value.trim().length > 0 ? value.trim() : undefined;
}

function asStringArray(value: unknown): string[] | undefined {
  if (!Array.isArray(value)) {
    return undefined;
  }
  const strings = value
    .map((item) => asString(item))
    .filter((item): item is string => Boolean(item));
  return strings.length > 0 ? strings : undefined;
}

function asStringFrom(record: RecordLike, keys: string[]): string | undefined {
  for (const key of keys) {
    const value = asString(record[key]);
    if (value) {
      return value;
    }
  }
  return undefined;
}

function pickOutputString(output: unknown, keys: string[]): string | undefined {
  const direct = asString(output);
  if (direct) {
    return direct;
  }

  if (!isRecord(output)) {
    return undefined;
  }

  for (const key of keys) {
    const value = output[key];
    const normalized = asString(value);
    if (normalized) {
      return normalized;
    }
    if (isRecord(value)) {
      const nested = pickOutputString(value, keys);
      if (nested) {
        return nested;
      }
    }
  }

  return undefined;
}

function requireAgentId(config: AgentConfig): string {
  const agentId = asString(config.id);
  if (agentId) {
    return agentId;
  }
  throw new Error('Agent id is required for backend agent runtime execution');
}

function createAgentId(name: string): string {
  const normalizedName = name
    .trim()
    .toLowerCase()
    .replace(/[^a-z0-9]+/gu, '-')
    .replace(/^-+|-+$/gu, '')
    .slice(0, 48);
  const suffix =
    typeof crypto !== 'undefined' && typeof crypto.randomUUID === 'function'
      ? crypto.randomUUID().replace(/-/gu, '').slice(0, 12)
      : Math.trunc(performance.now()).toString(36);
  return `agent.pc.${normalizedName || 'managed'}.${suffix}`;
}

function createDeploymentId(agentId: string): string {
  const normalizedAgentId = agentId
    .trim()
    .toLowerCase()
    .replace(/[^a-z0-9.]+/gu, '.')
    .replace(/^\.+|\.+$/gu, '');
  const suffix =
    typeof crypto !== 'undefined' && typeof crypto.randomUUID === 'function'
      ? crypto.randomUUID().replace(/-/gu, '').slice(0, 12)
      : Math.trunc(performance.now()).toString(36);
  return `deployment.${normalizedAgentId || 'agent'}.${suffix}`;
}

function buildAgentRuntimeInputPayload(
  config: AgentConfig,
  extra: Record<string, unknown>,
): Record<string, unknown> {
  return {
    agent: {
      id: config.id,
      name: config.name,
      description: config.description,
      avatar: config.avatar,
      type: config.type,
      systemPrompt: config.systemPrompt,
      knowledgeBaseIds: config.knowledgeBaseIds,
      categoryId: config.categoryId,
      welcomeMessage: config.welcomeMessage,
    },
    ...extra,
  };
}

function asAgentType(value: unknown): AgentConfig['type'] {
  return value === 'independent' ? 'independent' : 'normal';
}

function normalizeManifest(value: unknown): RecordLike {
  return isRecord(value) ? value : {};
}

function parseAgentUiConfigFromIntent(value: unknown): Partial<AgentConfig> {
  if (!isRecord(value) || !Array.isArray(value.constraints)) {
    return {};
  }
  const encoded = value.constraints
    .map((item) => asString(item))
    .find((item): item is string => Boolean(item?.startsWith(AGENT_UI_CONFIG_CONSTRAINT_PREFIX)));
  if (!encoded) {
    return {};
  }
  try {
    const parsed = JSON.parse(encoded.slice(AGENT_UI_CONFIG_CONSTRAINT_PREFIX.length)) as unknown;
    return isRecord(parsed) ? (parsed as Partial<AgentConfig>) : {};
  } catch {
    return {};
  }
}

function normalizeAgentFromAgentRecord(record: AgentRecord): AgentConfig {
  const manifest = normalizeManifest(record.manifest);
  const uiConfig = parseAgentUiConfigFromIntent(record.defaultCodeTaskIntent);
  const categoryId = asString(uiConfig.categoryId) ?? (Array.isArray(record.tags) ? record.tags[0] : undefined);
  return {
    id: record.agentId,
    name: record.displayName,
    description: record.description ?? '',
    avatar: asString(uiConfig.avatar) ?? '',
    type: asAgentType(uiConfig.type ?? record.implementationKind),
    systemPrompt: asString(uiConfig.systemPrompt) ?? asString(manifest.description),
    knowledgeBaseIds: asStringArray(uiConfig.knowledgeBaseIds),
    author: asString(uiConfig.author),
    users: asString(uiConfig.users),
    color: asString(uiConfig.color),
    iconName: asString(uiConfig.iconName),
    categoryId,
    welcomeMessage: asString(uiConfig.welcomeMessage),
  };
}

function extractAgentRecordItems(value: unknown): AgentRecord[] {
  const rawItems = isRecord(value) ? extractArray(value.data) : [];
  return rawItems.filter((item): item is AgentRecord => isRecord(item) && Boolean(asString(item.agentId)));
}

function buildAgentManifest(config: Partial<AgentConfig>, agentId: string): Record<string, unknown> {
  return {
    schema_version: '1.0.0',
    manifest_type: 'agent',
    agent_id: agentId,
    name: agentId,
    display_name: config.name ?? agentId,
    description: config.systemPrompt ?? config.description ?? '',
    version: '0.1.0',
    domain: 'intelligence',
    required_capabilities: [{ capability_id: 'model.chat' }],
    optional_capabilities: [
      { capability_id: 'tool.invoke' },
      { capability_id: 'knowledge.read' },
      { capability_id: 'memory.query' },
    ],
    event_families: ['agent.lifecycle'],
    owner: { name: config.author ?? 'sdkwork-chat-pc' },
    status: 'active',
  };
}

function buildAgentUiConfig(config: Partial<AgentConfig>): Record<string, unknown> {
  return {
    avatar: config.avatar,
    categoryId: config.categoryId,
    color: config.color,
    iconName: config.iconName,
    knowledgeBaseIds: config.knowledgeBaseIds,
    systemPrompt: config.systemPrompt,
    type: config.type,
    welcomeMessage: config.welcomeMessage,
  };
}

function buildAgentDefaultCodeTaskIntent(config: Partial<AgentConfig>): Record<string, unknown> {
  const uiConfig = buildAgentUiConfig(config);
  return {
    prompt: config.systemPrompt ?? config.description ?? 'Run the managed agent according to its manifest.',
    contextPaths: config.knowledgeBaseIds ?? [],
    constraints: [
      `agent.type=${config.type ?? 'normal'}`,
      `${AGENT_UI_CONFIG_CONSTRAINT_PREFIX}${JSON.stringify(uiConfig)}`,
    ],
  };
}

function buildCreateAgentRequest(config: AgentConfig, agentId: string): CreateAgentRequest {
  const categoryId = asString(config.categoryId);
  return {
    agentId,
    organizationId: DEFAULT_AGENT_ORGANIZATION_ID,
    ownerUserId: DEFAULT_AGENT_OWNER_USER_ID,
    code: agentId,
    displayName: config.name,
    description: config.description,
    manifest: buildAgentManifest(config, agentId),
    defaultCodeTaskIntent: buildAgentDefaultCodeTaskIntent(config),
    implementationProviderId: null,
    implementationKind: 'manifest-only',
    visibility: 'private',
    tags: categoryId ? [categoryId] : [],
    requestedAt: new Date().toISOString(),
  };
}

function buildUpdateAgentRequest(id: string, config: Partial<AgentConfig>): UpdateAgentRequest {
  const categoryId = asString(config.categoryId);
  return {
    ...(typeof config.name === 'string' ? { displayName: config.name } : {}),
    ...(typeof config.description === 'string' ? { description: config.description } : {}),
    ...(categoryId ? { tags: [categoryId] } : {}),
    manifest: buildAgentManifest({ ...config, id }, id),
    defaultCodeTaskIntent: buildAgentDefaultCodeTaskIntent(config),
    requestedAt: new Date().toISOString(),
  };
}

function buildDefaultProviderBindingRequest(): CreateAgentProviderBindingRequest {
  return {
    bindingId: DEFAULT_AGENT_BINDING_ID,
    providerId: DEFAULT_AGENT_PROVIDER_ID,
    implementationKind: 'manifest-only',
    configurationProfileId: DEFAULT_AGENT_CONFIGURATION_PROFILE_ID,
    capabilities: [...DEFAULT_AGENT_PROVIDER_CAPABILITIES],
    makeDefault: true,
    requestedAt: new Date().toISOString(),
  };
}

function isExistingProviderBindingConflict(error: unknown): boolean {
  if (!isRecord(error)) {
    return false;
  }

  const status = error.status ?? error.statusCode;
  if (status !== 409 && status !== '409') {
    return false;
  }

  const response = isRecord(error.response) ? error.response : undefined;
  const body = isRecord(error.body) ? error.body : undefined;
  const detail =
    asString(error.detail) ??
    asString(error.message) ??
    (response ? asString(response.detail) ?? asString(response.message) : undefined) ??
    (body ? asString(body.detail) ?? asString(body.message) : undefined);
  return detail?.toLowerCase().includes('provider binding already exists') ?? false;
}

function asCatalogScope(record: RecordLike): AgentCatalogScope | undefined {
  const rawScope = asStringFrom(record, [
    'scope',
    'visibility',
    'catalogScope',
    'catalog_scope',
    'source',
    'ownerScope',
    'owner_scope',
  ])?.toLowerCase();

  if (rawScope) {
    if (['market', 'public', 'published', 'store', 'discover'].includes(rawScope)) {
      return 'market';
    }
    if (['mine', 'my', 'owned', 'private', 'workspace', 'draft'].includes(rawScope)) {
      return 'mine';
    }
  }

  if (record.isMine === true || record.mine === true || record.owned === true || record.isOwner === true) {
    return 'mine';
  }
  if (record.isMarket === true || record.market === true || record.public === true || record.published === true) {
    return 'market';
  }

  return undefined;
}

function extractArray(value: unknown): unknown[] {
  if (Array.isArray(value)) {
    return value;
  }
  if (isRecord(value)) {
    for (const key of ['items', 'data', 'agents', 'records', 'list']) {
      const nested = value[key];
      if (Array.isArray(nested)) {
        return nested;
      }
    }
  }
  return [];
}

function collectAgentRecords(snapshot: unknown, keys: string[]): RecordLike[] {
  if (!isRecord(snapshot)) {
    return [];
  }

  const result: RecordLike[] = [];
  for (const key of keys) {
    for (const item of extractArray(snapshot[key])) {
      if (isRecord(item)) {
        result.push(item);
      }
    }
  }
  return result;
}

function normalizeAgent(record: RecordLike): AgentConfig | undefined {
  const id = asStringFrom(record, ['id', 'agentId', 'agent_id', 'subjectId', 'subject_id']);
  const name = asStringFrom(record, ['name', 'displayName', 'display_name', 'title']);

  if (!id || !name) {
    return undefined;
  }

  return {
    id,
    name,
    description: asStringFrom(record, ['description', 'desc', 'summary', 'intro']) ?? '',
    avatar: asStringFrom(record, ['avatar', 'avatarUrl', 'avatar_url', 'iconUrl', 'icon_url']) ?? '',
    type: asAgentType(record.type),
    systemPrompt: asStringFrom(record, ['systemPrompt', 'system_prompt', 'prompt']),
    author: asStringFrom(record, ['author', 'authorName', 'author_name', 'ownerName', 'owner_name']),
    users: asStringFrom(record, ['users', 'usageCountText', 'usage_count_text', 'installCountText', 'install_count_text']),
    color: asStringFrom(record, ['color', 'colorClass', 'color_class']),
    iconName: asStringFrom(record, ['iconName', 'icon_name', 'icon']),
    categoryId: asStringFrom(record, ['categoryId', 'category_id', 'category']),
    welcomeMessage: asStringFrom(record, ['welcomeMessage', 'welcome_message', 'greeting']),
  };
}

function uniqueAgents(records: RecordLike[]): AgentConfig[] {
  const agents = new Map<string, AgentConfig>();

  for (const record of records) {
    const agent = normalizeAgent(record);
    if (agent.id) {
      agents.set(agent.id, agent);
    }
  }

  return Array.from(agents.values());
}

export function parseAgentCatalogSnapshot(snapshot: unknown, scope: AgentCatalogScope): AgentConfig[] {
  const scopedKeys =
    scope === 'market'
      ? ['marketAgents', 'market_agents', 'publicAgents', 'public_agents', 'publishedAgents', 'published_agents']
      : ['myAgents', 'my_agents', 'ownedAgents', 'owned_agents', 'workspaceAgents', 'workspace_agents'];

  const explicitRecords = collectAgentRecords(snapshot, scopedKeys);
  const sharedRecords = collectAgentRecords(snapshot, ['agents', 'agentCatalog', 'agent_catalog', 'catalog']);

  return uniqueAgents([
    ...sharedRecords.filter((record) => asCatalogScope(record) === scope),
    ...explicitRecords,
  ]);
}

class SdkworkAgentService implements AgentService {
  constructor(
    private readonly getAgentClient: () => SdkworkAgentAppClient = getAgentAppSdkClientWithSession,
  ) {}

  async getAgents(): Promise<AgentConfig[]> {
    const response = await this.getAgentClient().ai.agents.list({
      tenantId: DEFAULT_AGENT_TENANT_ID,
      ownerUserId: DEFAULT_AGENT_OWNER_USER_ID,
      page: 1,
      pageSize: 100,
    });
    return extractAgentRecordItems(response).map(normalizeAgentFromAgentRecord);
  }

  async getMarketAgents(): Promise<AgentConfig[]> {
    const response = await this.getAgentClient().ai.agents.list({
      tenantId: DEFAULT_AGENT_TENANT_ID,
      page: 1,
      pageSize: 100,
    });
    return extractAgentRecordItems(response)
      .filter((record) => record.visibility === 'public')
      .map(normalizeAgentFromAgentRecord);
  }

  async createAgent(config: AgentConfig): Promise<AgentConfig> {
    const agentId = asString(config.id) ?? createAgentId(config.name);
    const response = await this.getAgentClient().ai.agents.create(
      buildCreateAgentRequest(config, agentId),
      { tenantId: DEFAULT_AGENT_TENANT_ID },
    );
    return normalizeAgentFromAgentRecord(response.data);
  }

  async updateAgent(id: string, config: Partial<AgentConfig>): Promise<AgentConfig> {
    const response = await this.getAgentClient().ai.agents.update(
      id,
      buildUpdateAgentRequest(id, config),
      { tenantId: DEFAULT_AGENT_TENANT_ID },
    );
    return normalizeAgentFromAgentRecord(response.data);
  }

  async publishAgent(id: string): Promise<void> {
    try {
      await this.getAgentClient().ai.agents.providerBindings.create(
        id,
        buildDefaultProviderBindingRequest(),
        { tenantId: DEFAULT_AGENT_TENANT_ID },
      );
    } catch (error) {
      if (!isExistingProviderBindingConflict(error)) {
        throw error;
      }
    }

    await this.getAgentClient().ai.agents.deployments.create(
      id,
      {
        deploymentId: createDeploymentId(id),
        bindingId: DEFAULT_AGENT_BINDING_ID,
        requestedAt: new Date().toISOString(),
      },
      { tenantId: DEFAULT_AGENT_TENANT_ID },
    );
  }

  async deleteAgent(id: string): Promise<void> {
    await this.getAgentClient().ai.agents.delete(id, { tenantId: DEFAULT_AGENT_TENANT_ID });
  }

  async requestPreviewResponse(request: AgentPreviewResponseRequest): Promise<AgentPreviewResponse> {
    const agentId = requireAgentId(request.config);
    const executionId = createExecutionId('execution.pc.agent.preview');
    const response = await this.getAgentClient().ai.agents.previewResponses.create(
      agentId,
      {
        executionId,
        content: request.content,
        debugMode: request.debugMode ?? false,
        memoryEnabled: request.memoryEnabled ?? false,
        model: request.model,
        temperature: request.temperature,
        inputPayload: buildAgentRuntimeInputPayload(request.config, {
          content: request.content,
          debugMode: request.debugMode ?? false,
          memoryEnabled: request.memoryEnabled ?? false,
          model: request.model,
          temperature: request.temperature,
        }),
        requestedAt: new Date().toISOString(),
      },
      { tenantId: DEFAULT_AGENT_TENANT_ID },
    );
    const outputPayload = response.data.outputPayload;
    const content = pickOutputString(outputPayload, [
      'reply',
      'content',
      'message',
      'text',
      'answer',
      'response',
      'output',
    ]);
    if (!content) {
      throw new AgentRuntimeExecutionUnavailableError('preview response', response.data.executionId);
    }
    return {
      content,
      executionId: response.data.executionId,
      outputPayload,
    };
  }

  async optimizePrompt(request: AgentPromptOptimizeRequest): Promise<AgentPromptOptimizeResult> {
    const agentId = requireAgentId(request.config);
    const executionId = createExecutionId('execution.pc.agent.prompt');
    const response = await this.getAgentClient().ai.agents.promptOptimizations.create(
      agentId,
      {
        executionId,
        prompt: request.prompt,
        inputPayload: buildAgentRuntimeInputPayload(request.config, {
          prompt: request.prompt,
        }),
        requestedAt: new Date().toISOString(),
      },
      { tenantId: DEFAULT_AGENT_TENANT_ID },
    );
    const outputPayload = response.data.outputPayload;
    const optimizedPrompt = pickOutputString(outputPayload, [
      'optimizedPrompt',
      'optimized_prompt',
      'prompt',
      'content',
      'text',
      'output',
    ]);
    if (!optimizedPrompt) {
      throw new AgentRuntimeExecutionUnavailableError('prompt optimization', response.data.executionId);
    }
    return {
      executionId: response.data.executionId,
      optimizedPrompt,
      outputPayload,
    };
  }
}

export function createSdkworkAgentService(
  getAgentClient?: () => SdkworkAgentAppClient,
): AgentService {
  return new SdkworkAgentService(getAgentClient);
}

export const agentService = createSdkworkAgentService();
