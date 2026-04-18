import type {
  CrawChatAuthLoginRequest,
  CrawChatAuthLoginResult,
  CrawChatAuthSession,
} from './types.js';
import type { CrawChatSdkContext } from './sdk-context.js';

export class CrawChatAuthModule {
  constructor(private readonly context: CrawChatSdkContext) {}

  async login(body: CrawChatAuthLoginRequest): Promise<CrawChatAuthLoginResult> {
    const session = await this.context.backendClient.auth.login(body);
    if (session.accessToken) {
      this.useToken(session.accessToken);
    }
    return session;
  }

  me(): Promise<CrawChatAuthSession> {
    return this.context.backendClient.auth.me();
  }

  useToken(token: string): void {
    this.context.setAuthToken(token);
  }

  clearToken(): void {
    this.context.clearAuthToken();
  }
}
