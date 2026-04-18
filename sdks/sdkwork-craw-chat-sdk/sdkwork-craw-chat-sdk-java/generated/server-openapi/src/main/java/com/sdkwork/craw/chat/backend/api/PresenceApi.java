package com.sdkwork.craw.chat.backend.api;

import com.fasterxml.jackson.core.type.TypeReference;
import com.sdkwork.craw.chat.backend.http.HttpClient;
import com.sdkwork.craw.chat.backend.model.*;
import java.util.List;
import java.util.Map;

public class PresenceApi {
    private final HttpClient client;
    
    public PresenceApi(HttpClient client) {
        this.client = client;
    }

    /** Refresh device presence */
    public PresenceSnapshotView heartbeat(PresenceDeviceRequest body) throws Exception {
        Object raw = client.post(ApiPaths.backendPath("/presence/heartbeat"), body, null, null, "application/json");
        return client.convertValue(raw, new TypeReference<PresenceSnapshotView>() {});
    }

    /** Get current presence */
    public PresenceSnapshotView getPresenceMe() throws Exception {
        Object raw = client.get(ApiPaths.backendPath("/presence/me"));
        return client.convertValue(raw, new TypeReference<PresenceSnapshotView>() {});
    }
}
