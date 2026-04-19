import { ImAdminSdkContext, resolveBackendClient } from './sdk-context.js';
export class ImAdminSdkClient {
    context;
    backendClient;
    auth;
    users;
    marketing;
    tenants;
    access;
    routing;
    catalog;
    usage;
    billing;
    operations;
    storage;
    constructor(options) {
        this.context = new ImAdminSdkContext(options.backendClient);
        this.backendClient = options.backendClient;
        this.auth = options.backendClient.auth;
        this.users = options.backendClient.users;
        this.marketing = options.backendClient.marketing;
        this.tenants = options.backendClient.tenants;
        this.access = options.backendClient.access;
        this.routing = options.backendClient.routing;
        this.catalog = options.backendClient.catalog;
        this.usage = options.backendClient.usage;
        this.billing = options.backendClient.billing;
        this.operations = options.backendClient.operations;
        this.storage = options.backendClient.storage;
    }
    static async create(options) {
        return new ImAdminSdkClient({
            backendClient: await resolveBackendClient(options),
        });
    }
    setAuthToken(token) {
        this.context.setAuthToken(token);
        return this;
    }
}
export async function createImAdminSdkClient(options) {
    return ImAdminSdkClient.create(options);
}
