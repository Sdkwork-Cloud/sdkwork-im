import { ImWebSocketAuthOptions } from './types.js';
export function normalizeImWebSocketAuthOptions(value) {
    const init = value ?? {};
    switch (value?.mode ?? 'automatic') {
        case 'headerBearer':
            return ImWebSocketAuthOptions.headerBearer(init);
        case 'queryBearer':
            return ImWebSocketAuthOptions.queryBearer(init);
        case 'none':
            return ImWebSocketAuthOptions.none(init);
        case 'automatic':
        default:
            return ImWebSocketAuthOptions.automatic(init);
    }
}
export function resolveAutomaticImWebSocketAuthMode({ hasSocket = false, hasWebSocketFactory = false, } = {}) {
    if (isBrowserLikeRuntime() && !hasSocket && !hasWebSocketFactory) {
        return 'queryBearer';
    }
    return 'headerBearer';
}
function isBrowserLikeRuntime() {
    return (typeof window === 'object'
        && window !== null
        && typeof document === 'object'
        && document !== null);
}
