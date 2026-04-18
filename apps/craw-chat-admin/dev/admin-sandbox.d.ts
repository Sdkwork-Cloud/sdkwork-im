export function isAdminSandboxEnabled(
  env?: Record<string, string | undefined>,
): boolean;

export function resolveAdminSandboxCredentials(options?: {
  env?: Record<string, string | undefined>;
  sandboxCredentials?: {
    email?: string;
    password?: string;
  };
  seed?: unknown;
}): {
  email: string;
  password: string;
  source: string;
};

export function getAdminSandboxCredentials(state: unknown): {
  email: string;
  password: string;
  source: string;
};

export function createAdminSandboxState(options?: {
  env?: Record<string, string | undefined>;
  sandboxCredentials?: {
    email?: string;
    password?: string;
  };
}): unknown;

export function createAdminSandboxMiddleware(options?: {
  state?: unknown;
  onSandboxCredentialsResolved?: (credentials: {
    email: string;
    password: string;
    source: string;
  }) => void;
}): (
  req: AsyncIterable<Uint8Array> & {
    url?: string;
    method?: string;
    headers?: unknown;
  },
  res: {
    statusCode: number;
    setHeader(name: string, value: string): void;
    end(body?: string): void;
  },
  next: () => void,
) => void | Promise<void>;
