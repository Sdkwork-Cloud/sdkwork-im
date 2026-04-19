export class ImSdkError extends Error {
    code;
    details;
    name = 'ImSdkError';
    constructor(code, message, details) {
        super(message);
        this.code = code;
        this.details = details;
    }
}
