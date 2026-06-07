import {
  getAiotAppSdkClientWithSession,
  type SdkworkAiotAppClient,
} from "@sdkwork/clawchat-pc-core/sdk/aiotAppSdkClient";
import {
  getAppSdkClientWithSession,
  type SdkworkImAppClient,
} from "@sdkwork/clawchat-pc-core/sdk/appSdkClient";
import {
  readAppSdkSessionTokens,
  resolveAppSdkOrganizationId,
  resolveAppSdkTenantId,
  resolveAppSdkUserId,
} from "@sdkwork/clawchat-pc-core/sdk/session";
import { resolveSdkworkChatPcClientId } from "./ClientIdentityService";

export const ALL_APP_MODULES = [
  "chat",
  "workspace",
  "contacts",
  "knowledge",
  "drive",
  "agent",
  "favorites",
  "orders",
  "shop",
  "calendar",
  "notary",
  "enterprise",
  "devices",
  "community",
  "voice",
  "course",
];

export const DEFAULT_SIDEBAR_MODULES = [
  "chat",
  "workspace",
  "contacts",
  "knowledge",
  "drive",
  "agent",
  "favorites",
];

const SUPPORTED_LANGUAGES = new Set(["zh-CN", "en-US"]);
const SETTINGS_CHANGED_EVENT = "sdkwork-chat-pc:settings-changed";
const SETTINGS_STORAGE_KEY = "clawchat-settings";
const LEGACY_AUTOFILLED_SIDEBAR_MODULES = [
  "chat",
  "workspace",
  "orders",
  "shop",
  "calendar",
  "notary",
  "knowledge",
  "enterprise",
  "devices",
  "community",
  "voice",
  "agent",
  "course",
  "contacts",
  "favorites",
];

function normalizeSettingsLanguage(lang: unknown) {
  return typeof lang === "string" && SUPPORTED_LANGUAGES.has(lang)
    ? lang
    : "zh-CN";
}

function getLocalStorage(): Storage | undefined {
  return typeof localStorage === "undefined" ? undefined : localStorage;
}

function notifySettingsChanged(settings: AppSettings) {
  if (typeof window === "undefined") return;
  window.dispatchEvent(new CustomEvent(SETTINGS_CHANGED_EVENT, {
    detail: { settings },
  }));
}

export interface AppSettings {
  lang: string;
  autoStart: boolean;
  notifySound: boolean;
  notifyDesktop: boolean;
  privacyRequireAuth: boolean;
  privacyShowOnline: boolean;
  theme: "system" | "dark" | "light";
  sidebarModules: string[];
}

export interface DeviceInfo {
  id: string;
  name: string;
  time: string;
  isCurrent?: boolean;
}

export interface SettingsService {
  getSettings(): Promise<AppSettings>;
  updateSettings(updates: Partial<AppSettings>): Promise<AppSettings>;
  getDevices(): Promise<DeviceInfo[]>;
  removeDevice(deviceId: string): Promise<void>;
  clearCache(): Promise<void>;
  getServerModules(): Promise<string[]>;
}

type RecordLike = Record<string, unknown>;

interface AiotAppContext {
  dataScope?: string;
  organizationId: string;
  permissionScope: string;
  tenantId: string;
  userId?: string;
}

interface AiotAppRequestParams {
  xSdkworkTenantId: string;
  xSdkworkOrganizationId: string;
  xSdkworkUserId?: string;
  xSdkworkDataScope?: string;
  xSdkworkPermissionScope: string;
  idempotencyKey?: string;
}

const DEFAULT_AIOT_CONTEXT: AiotAppContext = {
  tenantId: "20001",
  organizationId: "30001",
  permissionScope: "iot.twins.read",
};

function isRecord(value: unknown): value is RecordLike {
  return typeof value === "object" && value !== null && !Array.isArray(value);
}

function parseJsonRecord(value: string | undefined): RecordLike {
  if (!value) return {};
  try {
    const parsed = JSON.parse(value) as unknown;
    return isRecord(parsed) ? parsed : {};
  } catch {
    return {};
  }
}

function pickString(value: unknown): string | undefined {
  return typeof value === "string" && value.trim().length > 0
    ? value.trim()
    : undefined;
}

function pickStringArray(value: unknown): string[] {
  if (!Array.isArray(value)) return [];
  return value
    .map((item) => pickString(item))
    .filter((item): item is string => Boolean(item));
}

function uniqueKnownModules(modules: string[]): string[] {
  const known = new Set(ALL_APP_MODULES);
  const seen = new Set<string>();
  const result: string[] = [];
  for (const moduleId of modules) {
    if (!known.has(moduleId) || seen.has(moduleId)) continue;
    seen.add(moduleId);
    result.push(moduleId);
  }
  return result;
}

function hasSameModuleSet(left: string[], right: string[]): boolean {
  if (left.length !== right.length) return false;
  const leftSet = new Set(left);
  return right.every((moduleId) => leftSet.has(moduleId));
}

function normalizeSidebarModules(
  value: unknown,
  options: { migrateLegacyAllModules?: boolean } = {},
): string[] {
  const modules = uniqueKnownModules(pickStringArray(value));
  if (modules.length === 0) {
    return [...DEFAULT_SIDEBAR_MODULES];
  }
  if (options.migrateLegacyAllModules && (
    hasSameModuleSet(modules, ALL_APP_MODULES) ||
    hasSameModuleSet(modules, LEGACY_AUTOFILLED_SIDEBAR_MODULES)
  )) {
    return [...DEFAULT_SIDEBAR_MODULES];
  }
  return modules.includes("chat") ? modules : ["chat", ...modules];
}

function collectModules(snapshot: unknown): string[] {
  if (!isRecord(snapshot)) return [];
  for (const key of [
    "enabledModules",
    "enabled_modules",
    "sidebarModules",
    "sidebar_modules",
    "modules",
  ]) {
    const modules = pickStringArray(snapshot[key]);
    if (modules.length > 0) return modules;
    if (isRecord(snapshot[key])) {
      const nestedModules = pickStringArray((snapshot[key] as RecordLike).items);
      if (nestedModules.length > 0) return nestedModules;
    }
  }
  return [];
}

function collectDeviceInfo(...states: RecordLike[]): DeviceInfo[] {
  const devices = new Map<string, DeviceInfo>();
  for (const state of states) {
    const candidates = [
      state.loginDevices,
      state.login_devices,
      state.devices,
      isRecord(state.sessions) ? state.sessions.items : undefined,
    ];
    for (const candidate of candidates) {
      if (!Array.isArray(candidate)) continue;
      for (const item of candidate) {
        if (!isRecord(item)) continue;
        const id = pickString(item.id)
          ?? pickString(item.deviceId)
          ?? pickString(item.device_id);
        if (!id) continue;
        const isCurrent = item.isCurrent === true || item.current === true;
        devices.set(id, {
          id,
          name: pickString(item.name)
            ?? pickString(item.deviceName)
            ?? pickString(item.device_name)
            ?? id,
          time: pickString(item.time)
            ?? pickString(item.lastSeenAt)
            ?? pickString(item.last_seen_at)
            ?? pickString(item.updatedAt)
            ?? "",
          ...(isCurrent ? { isCurrent } : {}),
        });
      }
    }
  }
  return Array.from(devices.values());
}

function collectTwinStateRecords(twin: unknown): RecordLike[] {
  const envelope = isRecord(twin) && "data" in twin ? twin.data : twin;
  const record = isRecord(envelope) ? envelope : {};
  return [
    record.reported,
    record.reportedState,
    record.reported_state,
    parseJsonRecord(pickString(record.reportedStateJson)),
    parseJsonRecord(pickString(record.reported_state_json)),
    record.desired,
    record.desiredState,
    record.desired_state,
    parseJsonRecord(pickString(record.desiredStateJson)),
    parseJsonRecord(pickString(record.desired_state_json)),
  ].filter(isRecord);
}

function resolveAiotContext(permissionScope: string): AiotAppContext {
  const session = readAppSdkSessionTokens();
  return {
    ...DEFAULT_AIOT_CONTEXT,
    tenantId: resolveAppSdkTenantId(session) ?? DEFAULT_AIOT_CONTEXT.tenantId,
    organizationId: resolveAppSdkOrganizationId(session) ?? DEFAULT_AIOT_CONTEXT.organizationId,
    ...(resolveAppSdkUserId(session) ? { userId: resolveAppSdkUserId(session) } : {}),
    permissionScope,
  };
}

function toAiotRequestParams(
  context: AiotAppContext,
  options: { idempotencyKey?: string } = {},
): AiotAppRequestParams {
  return {
    xSdkworkTenantId: context.tenantId,
    xSdkworkOrganizationId: context.organizationId,
    xSdkworkUserId: context.userId,
    xSdkworkDataScope: context.dataScope,
    xSdkworkPermissionScope: context.permissionScope,
    idempotencyKey: options.idempotencyKey,
  };
}

class SdkworkSettingsService implements SettingsService {
  private get defaultSettings(): AppSettings {
    return {
      lang: "zh-CN",
      autoStart: true,
      notifySound: true,
      notifyDesktop: true,
      privacyRequireAuth: true,
      privacyShowOnline: true,
      theme: "system",
      sidebarModules: [...DEFAULT_SIDEBAR_MODULES],
    };
  }

  private settings: AppSettings = this.loadSettings();

  constructor(
    private readonly getClient: () => SdkworkImAppClient = getAppSdkClientWithSession,
    private readonly getAiotClient: () => SdkworkAiotAppClient = getAiotAppSdkClientWithSession,
    private readonly resolveClientId: () => string = resolveSdkworkChatPcClientId,
  ) {
    setTimeout(() => this.initTheme(), 0);
  }

  private loadSettings(): AppSettings {
    try {
      const stored = getLocalStorage()?.getItem(SETTINGS_STORAGE_KEY);
      if (stored) {
        const parsed = JSON.parse(stored) as Partial<AppSettings>;
        return {
          ...this.defaultSettings,
          ...parsed,
          lang: normalizeSettingsLanguage(parsed.lang),
          sidebarModules: normalizeSidebarModules(parsed.sidebarModules, {
            migrateLegacyAllModules: true,
          }),
        };
      }
    } catch {}
    return this.defaultSettings;
  }

  private initTheme() {
    this.applyTheme(this.settings.theme);
  }

  applyTheme(theme: "system" | "dark" | "light") {
    if (typeof window === "undefined" || typeof document === "undefined") {
      return;
    }
    let mode = theme;
    if (theme === "system") {
      mode = window.matchMedia("(prefers-color-scheme: light)").matches
        ? "light"
        : "dark";
    }
    if (mode === "light") {
      document.documentElement.classList.add("light-mode");
    } else {
      document.documentElement.classList.remove("light-mode");
    }
  }

  async getSettings(): Promise<AppSettings> {
    return { ...this.settings };
  }

  async updateSettings(updates: Partial<AppSettings>): Promise<AppSettings> {
    const sidebarModules = updates.sidebarModules === undefined
      ? this.settings.sidebarModules
      : normalizeSidebarModules(updates.sidebarModules);
    this.settings = {
      ...this.settings,
      ...updates,
      lang: normalizeSettingsLanguage(updates.lang ?? this.settings.lang),
      sidebarModules,
    };
    try {
      getLocalStorage()?.setItem(
        SETTINGS_STORAGE_KEY,
        JSON.stringify(this.settings),
      );
    } catch {}
    this.applyTheme(this.settings.theme);
    notifySettingsChanged(this.settings);
    return { ...this.settings };
  }

  async getDevices(): Promise<DeviceInfo[]> {
    try {
      const twin = await this.getAiotClient().iot.devices.twin.retrieve(
        this.resolveClientId(),
        toAiotRequestParams(resolveAiotContext("iot.twins.read")),
      );
      return collectDeviceInfo(...collectTwinStateRecords(twin));
    } catch {
      return [];
    }
  }

  async removeDevice(deviceId: string): Promise<void> {
    const normalizedDeviceId = deviceId.trim();
    if (!normalizedDeviceId) {
      throw new Error("Device id is required");
    }
    await this.getAiotClient().iot.devices.commands.create(
      this.resolveClientId(),
      {
        capabilityName: "login-sessions",
        commandName: "disable-login-device",
        payload: {
          disabledLoginDeviceIds: [normalizedDeviceId],
        },
      },
      toAiotRequestParams(resolveAiotContext("iot.commands.execute"), {
        idempotencyKey: `disable-login-device:${normalizedDeviceId}`,
      }),
    );
  }

  async clearCache(): Promise<void> {
    if (typeof caches === "undefined") {
      return;
    }
    const keys = await caches.keys();
    await Promise.all(keys.map((key) => caches.delete(key)));
  }

  async getServerModules(): Promise<string[]> {
    try {
      const modules = collectModules(await this.getClient().portal.home.retrieve());
      if (modules.length > 0) {
        return modules;
      }
    } catch (error) {
      console.warn("Failed to fetch server modules config through app SDK, using defaults", error);
    }
    return [...ALL_APP_MODULES];
  }
}

export function createSdkworkSettingsService(
  getClient?: () => SdkworkImAppClient,
  getAiotClient?: () => SdkworkAiotAppClient,
  resolveClientId?: () => string,
): SettingsService {
  return new SdkworkSettingsService(getClient, getAiotClient, resolveClientId);
}

export const settingsService = createSdkworkSettingsService();
