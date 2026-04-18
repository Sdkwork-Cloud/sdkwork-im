export class CrawChatAuthModule {
    context;
    constructor(context) {
        this.context = context;
    }
    async login(body) {
        const session = await this.context.backendClient.auth.login(body);
        if (session.accessToken) {
            this.useToken(session.accessToken);
        }
        return session;
    }
    me() {
        return this.context.backendClient.auth.me();
    }
    useToken(token) {
        this.context.setAuthToken(token);
    }
    clearToken() {
        this.context.clearAuthToken();
    }
}
