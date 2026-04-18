package com.sdkwork.craw.chat.backend;

import com.sdkwork.common.core.Types;
import com.sdkwork.craw.chat.backend.http.HttpClient;
import com.sdkwork.craw.chat.backend.api.AuthApi;
import com.sdkwork.craw.chat.backend.api.PortalApi;
import com.sdkwork.craw.chat.backend.api.SessionApi;
import com.sdkwork.craw.chat.backend.api.PresenceApi;
import com.sdkwork.craw.chat.backend.api.RealtimeApi;
import com.sdkwork.craw.chat.backend.api.DeviceApi;
import com.sdkwork.craw.chat.backend.api.InboxApi;
import com.sdkwork.craw.chat.backend.api.ConversationApi;
import com.sdkwork.craw.chat.backend.api.MessageApi;
import com.sdkwork.craw.chat.backend.api.MediaApi;
import com.sdkwork.craw.chat.backend.api.StreamApi;
import com.sdkwork.craw.chat.backend.api.RtcApi;

public class SdkworkBackendClient {
    private final HttpClient httpClient;
    private AuthApi auth;
    private PortalApi portal;
    private SessionApi session;
    private PresenceApi presence;
    private RealtimeApi realtime;
    private DeviceApi device;
    private InboxApi inbox;
    private ConversationApi conversation;
    private MessageApi message;
    private MediaApi media;
    private StreamApi stream;
    private RtcApi rtc;

    public SdkworkBackendClient(String baseUrl) {
        this.httpClient = new HttpClient(baseUrl);
        this.auth = new AuthApi(httpClient);
        this.portal = new PortalApi(httpClient);
        this.session = new SessionApi(httpClient);
        this.presence = new PresenceApi(httpClient);
        this.realtime = new RealtimeApi(httpClient);
        this.device = new DeviceApi(httpClient);
        this.inbox = new InboxApi(httpClient);
        this.conversation = new ConversationApi(httpClient);
        this.message = new MessageApi(httpClient);
        this.media = new MediaApi(httpClient);
        this.stream = new StreamApi(httpClient);
        this.rtc = new RtcApi(httpClient);
    }

    public SdkworkBackendClient(Types.SdkConfig config) {
        this.httpClient = new HttpClient(config);
        this.auth = new AuthApi(httpClient);
        this.portal = new PortalApi(httpClient);
        this.session = new SessionApi(httpClient);
        this.presence = new PresenceApi(httpClient);
        this.realtime = new RealtimeApi(httpClient);
        this.device = new DeviceApi(httpClient);
        this.inbox = new InboxApi(httpClient);
        this.conversation = new ConversationApi(httpClient);
        this.message = new MessageApi(httpClient);
        this.media = new MediaApi(httpClient);
        this.stream = new StreamApi(httpClient);
        this.rtc = new RtcApi(httpClient);
    }

    public AuthApi getAuth() {
        return this.auth;
    }

    public PortalApi getPortal() {
        return this.portal;
    }

    public SessionApi getSession() {
        return this.session;
    }

    public PresenceApi getPresence() {
        return this.presence;
    }

    public RealtimeApi getRealtime() {
        return this.realtime;
    }

    public DeviceApi getDevice() {
        return this.device;
    }

    public InboxApi getInbox() {
        return this.inbox;
    }

    public ConversationApi getConversation() {
        return this.conversation;
    }

    public MessageApi getMessage() {
        return this.message;
    }

    public MediaApi getMedia() {
        return this.media;
    }

    public StreamApi getStream() {
        return this.stream;
    }

    public RtcApi getRtc() {
        return this.rtc;
    }

    public SdkworkBackendClient setApiKey(String apiKey) {
        httpClient.setApiKey(apiKey);
        return this;
    }

    public SdkworkBackendClient setAuthToken(String token) {
        httpClient.setAuthToken(token);
        return this;
    }

    public SdkworkBackendClient setAccessToken(String token) {
        httpClient.setAccessToken(token);
        return this;
    }

    public SdkworkBackendClient setHeader(String key, String value) {
        httpClient.setHeader(key, value);
        return this;
    }

    public HttpClient getHttpClient() {
        return httpClient;
    }
}
