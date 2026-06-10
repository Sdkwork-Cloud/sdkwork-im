import { getNotaryAppSdkClient } from '@sdkwork/clawchat-pc-core';

export interface NotaryAccessState {
  visible: boolean;
  canUseNotary: boolean;
  canShowNotaryMenu: boolean;
  organizationVerified: boolean;
  notaryBusinessEnabled: boolean;
  memberId?: string;
  reason?: string;
}

export interface NotaryAccessService {
  getAccess(force?: boolean): Promise<NotaryAccessState>;
  canShowNotaryMenu(force?: boolean): Promise<boolean>;
  canUseNotary(force?: boolean): Promise<boolean>;
  subscribe(listener: (state: NotaryAccessState) => void): () => void;
  reset(): void;
}

const DENIED_ACCESS: NotaryAccessState = {
  visible: false,
  canUseNotary: false,
  canShowNotaryMenu: false,
  organizationVerified: false,
  notaryBusinessEnabled: false,
  reason: 'notary_access_unavailable',
};

function asRecord(value: unknown): Record<string, unknown> {
  return value && typeof value === 'object' && !Array.isArray(value)
    ? (value as Record<string, unknown>)
    : {};
}

function optionalString(value: unknown): string | undefined {
  return typeof value === 'string' && value.trim().length > 0 ? value.trim() : undefined;
}

function mapAccess(value: unknown): NotaryAccessState {
  const record = asRecord(value);
  const organizationVerified = record.organizationVerified === true;
  const notaryBusinessEnabled = record.notaryBusinessEnabled === true;
  const hasMember = Boolean(optionalString(record.memberId));
  const visible =
    record.visible === true &&
    organizationVerified &&
    notaryBusinessEnabled &&
    hasMember;

  return {
    visible,
    canUseNotary: visible,
    canShowNotaryMenu: visible,
    organizationVerified,
    notaryBusinessEnabled,
    memberId: optionalString(record.memberId),
    reason: optionalString(record.reason),
  };
}

class SdkworkNotaryAccessService implements NotaryAccessService {
  private cached: NotaryAccessState | null = null;
  private inFlight: Promise<NotaryAccessState> | null = null;
  private readonly listeners = new Set<(state: NotaryAccessState) => void>();

  async getAccess(force = false): Promise<NotaryAccessState> {
    if (!force && this.cached) {
      return this.cached;
    }
    if (!force && this.inFlight) {
      return this.inFlight;
    }

    this.inFlight = getNotaryAppSdkClient()
      .notary.access.retrieve()
      .then((access) => mapAccess(access))
      .catch(() => DENIED_ACCESS)
      .then((access) => {
        this.cached = access;
        this.emit(access);
        return access;
      })
      .finally(() => {
        this.inFlight = null;
      });

    return this.inFlight;
  }

  async canShowNotaryMenu(force = false): Promise<boolean> {
    return (await this.getAccess(force)).canShowNotaryMenu;
  }

  async canUseNotary(force = false): Promise<boolean> {
    return (await this.getAccess(force)).canUseNotary;
  }

  subscribe(listener: (state: NotaryAccessState) => void): () => void {
    this.listeners.add(listener);
    if (this.cached) {
      listener(this.cached);
    }
    return () => {
      this.listeners.delete(listener);
    };
  }

  reset(): void {
    this.cached = null;
    this.inFlight = null;
    this.emit(DENIED_ACCESS);
  }

  private emit(state: NotaryAccessState): void {
    for (const listener of this.listeners) {
      listener(state);
    }
  }
}

export function createNotaryAccessService(): NotaryAccessService {
  return new SdkworkNotaryAccessService();
}

export const notaryAccessService = createNotaryAccessService();
