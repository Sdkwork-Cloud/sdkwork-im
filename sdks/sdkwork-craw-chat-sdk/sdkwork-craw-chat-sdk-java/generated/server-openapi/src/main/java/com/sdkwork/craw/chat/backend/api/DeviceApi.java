package com.sdkwork.craw.chat.backend.api;

import com.fasterxml.jackson.core.type.TypeReference;
import com.sdkwork.craw.chat.backend.http.HttpClient;
import com.sdkwork.craw.chat.backend.model.*;
import java.util.List;
import java.util.Map;

public class DeviceApi {
    private final HttpClient client;
    
    public DeviceApi(HttpClient client) {
        this.client = client;
    }

    /** Register the current device */
    public RegisteredDeviceView register(RegisterDeviceRequest body) throws Exception {
        Object raw = client.post(ApiPaths.backendPath("/devices/register"), body, null, null, "application/json");
        return client.convertValue(raw, new TypeReference<RegisteredDeviceView>() {});
    }

    /** Get device sync feed entries */
    public DeviceSyncFeedResponse getDeviceSyncFeed(String deviceId, Map<String, Object> params) throws Exception {
        Object raw = client.get(ApiPaths.backendPath("/devices/" + deviceId + "/sync-feed"), params);
        return client.convertValue(raw, new TypeReference<DeviceSyncFeedResponse>() {});
    }
}
