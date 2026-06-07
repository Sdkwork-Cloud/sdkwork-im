package com.sdkwork.im.sdk.generated;

import com.sdkwork.common.core.Types;
import com.sdkwork.im.sdk.generated.http.HttpClient;
import com.sdkwork.im.sdk.generated.api.DeviceApi;
import com.sdkwork.im.sdk.generated.api.PresenceApi;
import com.sdkwork.im.sdk.generated.api.RealtimeApi;
import com.sdkwork.im.sdk.generated.api.RtcApi;
import com.sdkwork.im.sdk.generated.api.SocialApi;
import com.sdkwork.im.sdk.generated.api.ChatApi;
import com.sdkwork.im.sdk.generated.api.StreamsApi;

public class SdkworkImClient {
    private final HttpClient httpClient;
    private DeviceApi device;
    private PresenceApi presence;
    private RealtimeApi realtime;
    private RtcApi rtc;
    private SocialApi social;
    private ChatApi chat;
    private StreamsApi streams;

    public SdkworkImClient(String baseUrl) {
        this.httpClient = new HttpClient(baseUrl);
        this.device = new DeviceApi(httpClient);
        this.presence = new PresenceApi(httpClient);
        this.realtime = new RealtimeApi(httpClient);
        this.rtc = new RtcApi(httpClient);
        this.social = new SocialApi(httpClient);
        this.chat = new ChatApi(httpClient);
        this.streams = new StreamsApi(httpClient);
    }

    public SdkworkImClient(Types.SdkConfig config) {
        this.httpClient = new HttpClient(config);
        this.device = new DeviceApi(httpClient);
        this.presence = new PresenceApi(httpClient);
        this.realtime = new RealtimeApi(httpClient);
        this.rtc = new RtcApi(httpClient);
        this.social = new SocialApi(httpClient);
        this.chat = new ChatApi(httpClient);
        this.streams = new StreamsApi(httpClient);
    }

    public DeviceApi getDevice() {
        return this.device;
    }

    public PresenceApi getPresence() {
        return this.presence;
    }

    public RealtimeApi getRealtime() {
        return this.realtime;
    }

    public RtcApi getRtc() {
        return this.rtc;
    }

    public SocialApi getSocial() {
        return this.social;
    }

    public ChatApi getChat() {
        return this.chat;
    }

    public StreamsApi getStreams() {
        return this.streams;
    }


    public SdkworkImClient setAuthToken(String token) {
        httpClient.setAuthToken(token);
        return this;
    }

    public SdkworkImClient setAccessToken(String token) {
        httpClient.setAccessToken(token);
        return this;
    }

    public SdkworkImClient setHeader(String key, String value) {
        httpClient.setHeader(key, value);
        return this;
    }

    public HttpClient getHttpClient() {
        return httpClient;
    }
}
