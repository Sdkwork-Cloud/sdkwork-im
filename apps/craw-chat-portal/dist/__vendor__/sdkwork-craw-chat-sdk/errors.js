export class CrawChatSdkError extends Error {
    code;
    details;
    name = 'CrawChatSdkError';
    constructor(code, message, details) {
        super(message);
        this.code = code;
        this.details = details;
    }
}
