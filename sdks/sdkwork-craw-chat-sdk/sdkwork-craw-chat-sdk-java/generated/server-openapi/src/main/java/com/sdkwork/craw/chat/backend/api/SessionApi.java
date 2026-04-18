package com.sdkwork.craw.chat.backend.api;

import com.fasterxml.jackson.core.type.TypeReference;
import com.sdkwork.craw.chat.backend.http.HttpClient;
import com.sdkwork.craw.chat.backend.model.*;
import java.util.List;
import java.util.Map;

public class SessionApi {
    private final HttpClient client;
    
    public SessionApi(HttpClient client) {
        this.client = client;
    }

    /** Resume the current app session */
    public SessionResumeView resume(ResumeSessionRequest body) throws Exception {
        Object raw = client.post(ApiPaths.backendPath("/sessions/resume"), body, null, null, "application/json");
        return client.convertValue(raw, new TypeReference<SessionResumeView>() {});
    }

    /** Disconnect the current app session device route */
    public PresenceSnapshotView disconnect(PresenceDeviceRequest body) throws Exception {
        Object raw = client.post(ApiPaths.backendPath("/sessions/disconnect"), body, null, null, "application/json");
        return client.convertValue(raw, new TypeReference<PresenceSnapshotView>() {});
    }
}
