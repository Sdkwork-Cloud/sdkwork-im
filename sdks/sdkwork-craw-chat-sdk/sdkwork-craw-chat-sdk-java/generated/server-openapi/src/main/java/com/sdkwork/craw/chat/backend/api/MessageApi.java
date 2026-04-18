package com.sdkwork.craw.chat.backend.api;

import com.fasterxml.jackson.core.type.TypeReference;
import com.sdkwork.craw.chat.backend.http.HttpClient;
import com.sdkwork.craw.chat.backend.model.*;
import java.util.List;
import java.util.Map;

public class MessageApi {
    private final HttpClient client;
    
    public MessageApi(HttpClient client) {
        this.client = client;
    }

    /** Edit a posted message */
    public MessageMutationResult edit(String messageId, EditMessageRequest body) throws Exception {
        Object raw = client.post(ApiPaths.backendPath("/messages/" + messageId + "/edit"), body, null, null, "application/json");
        return client.convertValue(raw, new TypeReference<MessageMutationResult>() {});
    }

    /** Recall a posted message */
    public MessageMutationResult recall(String messageId) throws Exception {
        Object raw = client.post(ApiPaths.backendPath("/messages/" + messageId + "/recall"), null);
        return client.convertValue(raw, new TypeReference<MessageMutationResult>() {});
    }
}
