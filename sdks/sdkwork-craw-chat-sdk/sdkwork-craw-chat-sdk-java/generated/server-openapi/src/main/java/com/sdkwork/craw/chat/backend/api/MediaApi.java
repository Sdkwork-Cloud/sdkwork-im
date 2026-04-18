package com.sdkwork.craw.chat.backend.api;

import com.fasterxml.jackson.core.type.TypeReference;
import com.sdkwork.craw.chat.backend.http.HttpClient;
import com.sdkwork.craw.chat.backend.model.*;
import java.util.List;
import java.util.Map;

public class MediaApi {
    private final HttpClient client;
    
    public MediaApi(HttpClient client) {
        this.client = client;
    }

    /** Create a media upload record */
    public MediaAsset createMediaUpload(CreateUploadRequest body) throws Exception {
        Object raw = client.post(ApiPaths.backendPath("/media/uploads"), body, null, null, "application/json");
        return client.convertValue(raw, new TypeReference<MediaAsset>() {});
    }

    /** Complete a media upload */
    public MediaAsset completeMediaUpload(String mediaAssetId, CompleteUploadRequest body) throws Exception {
        Object raw = client.post(ApiPaths.backendPath("/media/uploads/" + mediaAssetId + "/complete"), body, null, null, "application/json");
        return client.convertValue(raw, new TypeReference<MediaAsset>() {});
    }

    /** Issue a signed media download URL */
    public MediaDownloadUrlResponse getMediaDownloadUrl(String mediaAssetId, Map<String, Object> params) throws Exception {
        Object raw = client.get(ApiPaths.backendPath("/media/" + mediaAssetId + "/download-url"), params);
        return client.convertValue(raw, new TypeReference<MediaDownloadUrlResponse>() {});
    }

    /** Get a media asset by id */
    public MediaAsset getMediaAsset(String mediaAssetId) throws Exception {
        Object raw = client.get(ApiPaths.backendPath("/media/" + mediaAssetId + ""));
        return client.convertValue(raw, new TypeReference<MediaAsset>() {});
    }

    /** Attach a ready media asset as a conversation message */
    public PostMessageResult attachMediaAsset(String mediaAssetId, AttachMediaRequest body) throws Exception {
        Object raw = client.post(ApiPaths.backendPath("/media/" + mediaAssetId + "/attach"), body, null, null, "application/json");
        return client.convertValue(raw, new TypeReference<PostMessageResult>() {});
    }
}
