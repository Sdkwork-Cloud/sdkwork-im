export const DEFAULT_TIMEOUT = 15_000;
export class AdminApiError extends Error {
    status;
    payload;
    constructor(status, payload, message) {
        super(message ?? `control-plane request failed with status ${status}.`);
        this.name = 'AdminApiError';
        this.status = status;
        this.payload = payload;
    }
}
