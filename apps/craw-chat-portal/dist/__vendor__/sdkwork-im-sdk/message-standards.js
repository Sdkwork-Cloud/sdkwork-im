export const CRAW_CHAT_JSON_ENCODING = 'application/json';
export const CRAW_CHAT_MESSAGE_SCHEMA_LOCATION = 'urn:sdkwork:craw-chat:message:location';
export const CRAW_CHAT_MESSAGE_SCHEMA_LINK = 'urn:sdkwork:craw-chat:message:link';
export const CRAW_CHAT_MESSAGE_SCHEMA_CARD = 'urn:sdkwork:craw-chat:message:card';
export const CRAW_CHAT_MESSAGE_SCHEMA_MUSIC = 'urn:sdkwork:craw-chat:message:music';
export const CRAW_CHAT_MESSAGE_SCHEMA_CONTACT = 'urn:sdkwork:craw-chat:message:contact';
export const CRAW_CHAT_MESSAGE_SCHEMA_STICKER = 'urn:sdkwork:craw-chat:message:sticker';
export const CRAW_CHAT_MESSAGE_SCHEMA_VOICE = 'urn:sdkwork:craw-chat:message:voice';
export const CRAW_CHAT_MESSAGE_SCHEMA_AGENT = 'urn:sdkwork:craw-chat:message:agent';
export const CRAW_CHAT_MESSAGE_SCHEMA_AGENT_STATE = 'urn:sdkwork:craw-chat:message:agent_state';
export const CRAW_CHAT_MESSAGE_SCHEMA_AGENT_HANDOFF = 'urn:sdkwork:craw-chat:message:agent_handoff';
export const CRAW_CHAT_MESSAGE_SCHEMA_AI_TEXT = 'urn:sdkwork:craw-chat:message:ai_text';
export const CRAW_CHAT_MESSAGE_SCHEMA_AI_IMAGE_GENERATION = 'urn:sdkwork:craw-chat:message:ai_image_generation';
export const CRAW_CHAT_MESSAGE_SCHEMA_AI_VIDEO_GENERATION = 'urn:sdkwork:craw-chat:message:ai_video_generation';
export const CRAW_CHAT_MESSAGE_SCHEMA_TOOL_RESULT = 'urn:sdkwork:craw-chat:message:tool_result';
export const CRAW_CHAT_MESSAGE_SCHEMA_WORKFLOW_EVENT = 'urn:sdkwork:craw-chat:message:workflow_event';
export const CRAW_CHAT_CUSTOM_MESSAGE_SCHEMA_PREFIX = 'urn:sdkwork:craw-chat:message:custom:';
const STRUCTURED_MESSAGE_SCHEMA_BY_KIND = {
    location: CRAW_CHAT_MESSAGE_SCHEMA_LOCATION,
    link: CRAW_CHAT_MESSAGE_SCHEMA_LINK,
    card: CRAW_CHAT_MESSAGE_SCHEMA_CARD,
    music: CRAW_CHAT_MESSAGE_SCHEMA_MUSIC,
    contact: CRAW_CHAT_MESSAGE_SCHEMA_CONTACT,
    sticker: CRAW_CHAT_MESSAGE_SCHEMA_STICKER,
    voice: CRAW_CHAT_MESSAGE_SCHEMA_VOICE,
    agent: CRAW_CHAT_MESSAGE_SCHEMA_AGENT,
    agent_state: CRAW_CHAT_MESSAGE_SCHEMA_AGENT_STATE,
    agent_handoff: CRAW_CHAT_MESSAGE_SCHEMA_AGENT_HANDOFF,
    ai_text: CRAW_CHAT_MESSAGE_SCHEMA_AI_TEXT,
    ai_image_generation: CRAW_CHAT_MESSAGE_SCHEMA_AI_IMAGE_GENERATION,
    ai_video_generation: CRAW_CHAT_MESSAGE_SCHEMA_AI_VIDEO_GENERATION,
    tool_result: CRAW_CHAT_MESSAGE_SCHEMA_TOOL_RESULT,
    workflow_event: CRAW_CHAT_MESSAGE_SCHEMA_WORKFLOW_EVENT,
};
const STRUCTURED_MESSAGE_KIND_BY_SCHEMA = Object.fromEntries(Object.entries(STRUCTURED_MESSAGE_SCHEMA_BY_KIND).map(([kind, schemaRef]) => [
    schemaRef,
    kind,
]));
export function resolveStructuredMessageSchema(kind) {
    return STRUCTURED_MESSAGE_SCHEMA_BY_KIND[kind];
}
export function resolveStructuredMessageKind(schemaRef) {
    if (!schemaRef) {
        return undefined;
    }
    if (schemaRef in STRUCTURED_MESSAGE_KIND_BY_SCHEMA) {
        return STRUCTURED_MESSAGE_KIND_BY_SCHEMA[schemaRef];
    }
    if (schemaRef.startsWith(CRAW_CHAT_CUSTOM_MESSAGE_SCHEMA_PREFIX)) {
        return 'custom';
    }
    return undefined;
}
export function isCustomMessageSchema(schemaRef) {
    return Boolean(schemaRef && schemaRef.startsWith(CRAW_CHAT_CUSTOM_MESSAGE_SCHEMA_PREFIX));
}
export function buildCustomMessageSchema(customType) {
    return `${CRAW_CHAT_CUSTOM_MESSAGE_SCHEMA_PREFIX}${customType}`;
}
export function extractCustomMessageType(schemaRef) {
    if (!isCustomMessageSchema(schemaRef)) {
        return undefined;
    }
    return schemaRef?.slice(CRAW_CHAT_CUSTOM_MESSAGE_SCHEMA_PREFIX.length);
}
