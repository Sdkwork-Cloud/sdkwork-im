package com.sdkwork.im.app.api.generated.api;

import com.fasterxml.jackson.core.type.TypeReference;
import com.sdkwork.im.app.api.generated.http.HttpClient;
import com.sdkwork.im.app.api.generated.model.*;
import java.util.List;
import java.util.Map;

public class RtcApi {
    private final HttpClient client;
    
    public RtcApi(HttpClient client) {
        this.client = client;
    }

    /** Map RTC provider callback */
    public Map<String, Object> providerCallbacksCreate() throws Exception {
        Object raw = client.post(ApiPaths.appPath("/rtc/provider_callbacks"), null);
        return client.convertValue(raw, new TypeReference<Map<String, Object>>() {});
    }

    /** Retrieve RTC provider health */
    public Map<String, Object> providerHealthRetrieve() throws Exception {
        Object raw = client.get(ApiPaths.appPath("/rtc/provider_health"));
        return client.convertValue(raw, new TypeReference<Map<String, Object>>() {});
    }




}
