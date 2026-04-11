export function isAdminSandboxEnabled(
  env?: Record<string, string | undefined>,
): boolean;

export function createAdminSandboxMiddleware(options?: {
  state?: unknown;
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
