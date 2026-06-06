package com.sdkwork.im.app.api.generated.api;

import com.fasterxml.jackson.core.type.TypeReference;
import com.sdkwork.im.app.api.generated.http.HttpClient;
import com.sdkwork.im.app.api.generated.model.*;
import java.util.List;
import java.util.Map;

public class IotApi {
    private final HttpClient client;

    public IotApi(HttpClient client) {
        this.client = client;
    }

    /** Retrieve IoT access provider health */
    public Map<String, Object> accessProviderHealthRetrieve() throws Exception {
        Object raw = client.get(ApiPaths.appPath("/iot/access/provider_health"));
        return client.convertValue(raw, new TypeReference<Map<String, Object>>() {});
    }

    /** Retrieve IoT protocol provider health */
    public Map<String, Object> protocolProviderHealthRetrieve() throws Exception {
        Object raw = client.get(ApiPaths.appPath("/iot/protocol/provider_health"));
        return client.convertValue(raw, new TypeReference<Map<String, Object>>() {});
    }

    /** Ingest IoT protocol uplink */
    public Map<String, Object> protocolUplinkCreate() throws Exception {
        Object raw = client.post(ApiPaths.appPath("/iot/protocol/uplink"), null);
        return client.convertValue(raw, new TypeReference<Map<String, Object>>() {});
    }

    /** Ingest IoT protocol downlink */
    public Map<String, Object> protocolDownlinkCreate() throws Exception {
        Object raw = client.post(ApiPaths.appPath("/iot/protocol/downlink"), null);
        return client.convertValue(raw, new TypeReference<Map<String, Object>>() {});
    }




}
