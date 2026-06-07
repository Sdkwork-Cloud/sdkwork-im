// Copyright (c) 2026 Beijing Volcano Engine Technology Ltd.

package com.ss.bytertc.engine.flutter;

import com.ss.bytertc.engine.data.HumanOrientation;
import com.ss.bytertc.engine.data.Orientation;
import com.ss.bytertc.engine.data.Position;
import com.ss.bytertc.engine.live.MixedStreamAlternateImageFillMode;
import com.ss.bytertc.engine.live.MixedStreamAudioConfig;
import com.ss.bytertc.engine.live.MixedStreamAudioProfile;
import com.ss.bytertc.engine.live.MixedStreamConfig;
import com.ss.bytertc.engine.live.MixedStreamControlConfig;
import com.ss.bytertc.engine.live.MixedStreamLayoutRegionConfig;
import com.ss.bytertc.engine.live.MixedStreamLayoutRegionImageWaterMarkConfig;
import com.ss.bytertc.engine.live.MixedStreamLayoutRegionType;
import com.ss.bytertc.engine.live.MixedStreamMediaType;
import com.ss.bytertc.engine.live.MixedStreamPushMode;
import com.ss.bytertc.engine.live.MixedStreamRenderMode;
import com.ss.bytertc.engine.live.MixedStreamSEIContentMode;
import com.ss.bytertc.engine.live.MixedStreamSpatialAudioConfig;
import com.ss.bytertc.engine.live.MixedStreamVideoConfig;
import com.ss.bytertc.engine.live.MixedStreamVideoType;

import com.ss.bytertc.engine.live.SourceCrop;
import com.ss.bytertc.engine.live.MixedStreamSyncControlConfig;
import com.ss.bytertc.engine.live.MixedStreamSyncStrategy;
import com.ss.bytertc.engine.live.InterpolationMode;
import com.ss.bytertc.engine.live.StreamLayoutMode;

import org.json.JSONArray;
import org.json.JSONObject;

public class DataTransformer extends BasicTransformer {

    public static Position Position(JSONObject obj) {
        if (obj == null) return new Position();
        Position pos = new Position();
        pos.x = getInt(obj, "x", 0);
        pos.y = getInt(obj, "y", 0);
        pos.z = getInt(obj, "z", 0);
        return pos;
    }

    public static Orientation Orientation(JSONObject obj, Orientation defaultValue) {
        if (obj == null) return defaultValue;
        Orientation pos = new Orientation(
                getFloat(obj, "x", defaultValue.x),
                getFloat(obj, "y", defaultValue.y),
                getFloat(obj, "z", defaultValue.z)
        );
        return pos;
    }

    public static HumanOrientation HumanOrientation(JSONObject obj) {
        if (obj == null) return new HumanOrientation();
        HumanOrientation pos = new HumanOrientation();
        pos.forward = DataTransformer.Orientation(getJSONObject(obj, "forward"), pos.forward);
        pos.right = DataTransformer.Orientation(getJSONObject(obj, "right"), pos.right);
        pos.up = DataTransformer.Orientation(getJSONObject(obj, "up"), pos.up);
        return pos;
    }

    public static SourceCrop SourceCrop(JSONObject obj) {
        if (obj == null) return new SourceCrop();
        SourceCrop crop = new SourceCrop();
        crop.locationX = getDouble(obj, "locationX", 0.0);
        crop.locationY = getDouble(obj, "locationY", 0.0);
        crop.widthProportion = getDouble(obj, "widthProportion", 1.0);
        crop.heightProportion = getDouble(obj, "heightProportion", 1.0);
        return crop;
    }

    public static MixedStreamLayoutRegionImageWaterMarkConfig MixedStreamLayoutRegionImageWaterMarkConfig(JSONObject obj) {
        if (obj == null) return new MixedStreamLayoutRegionImageWaterMarkConfig(0, 0);
        MixedStreamLayoutRegionImageWaterMarkConfig config = new MixedStreamLayoutRegionImageWaterMarkConfig(0, 0);
        config.imageWidth = getInt(obj, "imageWidth", 0);
        config.imageHeight = getInt(obj, "imageHeight", 0);
        return config;
    }

    public static MixedStreamLayoutRegionConfig MixedStreamLayoutRegionConfig(JSONObject obj) {
        if (obj == null) return new MixedStreamLayoutRegionConfig();
        MixedStreamLayoutRegionConfig config = new MixedStreamLayoutRegionConfig();
        config.roomID = getString(obj, "roomId", "");
        config.userID = getString(obj, "userId", "");
        config.locationX = getInt(obj, "locationX", 0);
        config.locationY = getInt(obj, "locationY", 0);
        config.width = getInt(obj, "width", 360);
        config.height = getInt(obj, "height", 640);
        config.zOrder = getInt(obj, "zOrder", 0);
        config.alpha = getDouble(obj, "alpha", 1.0);
        config.cornerRadius = getDouble(obj, "cornerRadius", 0.0);
        config.mediaType = getEnumByIndex(obj, MixedStreamMediaType.class, "mediaType");
        config.renderMode = getEnumByIndex(obj, MixedStreamRenderMode.class, "renderMode");
        config.isLocalUser = getBoolean(obj, "isLocalUser", false);
        config.streamType = getEnumByIndex(obj, MixedStreamVideoType.class, "streamType");
        config.regionContentType = getEnumByIndex(obj, MixedStreamLayoutRegionType.class, "regionContentType");
        config.imageWaterMark = getBytes(obj, "imageWaterMark", null);
        config.imageWaterMarkConfig = DataTransformer.MixedStreamLayoutRegionImageWaterMarkConfig(getJSONObject(obj, "imageWaterMarkConfig"));
        config.alternateImageFillMode = getEnumByIndex(obj, MixedStreamAlternateImageFillMode.class, "alternateImageFillMode");
        config.alternateImageURL = getString(obj, "alternateImageUrl", "");
        config.spatialPosition = DataTransformer.Position(getJSONObject(obj, "spatialPosition"));
        config.applySpatialAudio = getBoolean(obj, "applySpatialAudio", true);
        config.sourceCrop = DataTransformer.SourceCrop(getJSONObject(obj, "sourceCrop"));
        return config;
    }

    public static MixedStreamVideoConfig MixedStreamVideoConfig(JSONObject obj) {
        if (obj == null) return new MixedStreamVideoConfig();
        MixedStreamVideoConfig config = new MixedStreamVideoConfig();
        config.fps = getInt(obj, "fps", 15);
        config.gop = getInt(obj, "gop", 2);
        config.bitrate = getInt(obj, "bitrate", 500);
        config.width = getInt(obj, "width", 360);
        config.height = getInt(obj, "height", 640);
        config.enableBframe = getBoolean(obj, "enableBframe", false);
        return config;
    }

    public static MixedStreamAudioConfig MixedStreamAudioConfig(JSONObject obj) {
        if (obj == null) return new MixedStreamAudioConfig();
        MixedStreamAudioConfig config = new MixedStreamAudioConfig();
        config.bitrate = getInt(obj, "bitrate", 64);
        config.sampleRate = getInt(obj, "sampleRate", 48000);
        config.channels = getInt(obj, "channels", 2);
        config.audioProfile = getEnumByIndex(obj, MixedStreamAudioProfile.class, "audioProfile");
        return config;
    }

    public static MixedStreamControlConfig MixedStreamControlConfig(JSONObject obj) {
        if (obj == null) return new MixedStreamControlConfig();
        MixedStreamControlConfig config = new MixedStreamControlConfig();
        config.enableVolumeIndication = getBoolean(obj, "enableVolumeIndication", false);
        config.volumeIndicationInterval = getFloat(obj, "volumeIndicationInterval", 2.0f);
        config.talkVolume = getInt(obj, "talkVolume", 0);
        config.isAddVolumeValue = getBoolean(obj, "isAddVolumeValue", false);
        config.seiContentMode = getEnumByIndex(obj, MixedStreamSEIContentMode.class, "seiContentMode");
        config.seiPayloadType = getInt(obj, "seiPayloadType", 100);
        config.seiPayloadUuid = getString(obj, "seiPayloadUuid", "");
        config.mediaType = getEnumByIndex(obj, MixedStreamMediaType.class, "mediaType");
        config.pushStreamMode = getEnumByIndex(obj, MixedStreamPushMode.class, "pushStreamMode");
        return config;
    }

    public static MixedStreamSpatialAudioConfig MixedStreamSpatialAudioConfig(JSONObject obj) {
        if (obj == null) return new MixedStreamSpatialAudioConfig();
        MixedStreamSpatialAudioConfig config = new MixedStreamSpatialAudioConfig();
        config.enableSpatialRender = getBoolean(obj, "enableSpatialRender", false);
        config.audienceSpatialPosition = DataTransformer.Position(getJSONObject(obj, "audienceSpatialPosition"));
        config.audienceSpatialOrientation = DataTransformer.HumanOrientation(getJSONObject(obj, "audienceSpatialOrientation"));
        return config;
    }

    public static MixedStreamSyncControlConfig MixedStreamSyncControlConfig(JSONObject obj) {
        if (obj == null) return new MixedStreamSyncControlConfig();
        MixedStreamSyncControlConfig config = new MixedStreamSyncControlConfig();
        config.baseUserID = getString(obj, "baseUserID", "");
        config.syncStrategy = getEnumByIndex(obj, MixedStreamSyncStrategy.class, "syncStrategy");
        config.maxCacheTimeMs = getInt(obj, "maxCacheTimeMs", 2000);
        config.videoNeedSdkMix = getBoolean(obj, "videoNeedSdkMix", true);
        return config;
    }

    public static MixedStreamConfig MixedStreamConfig(JSONObject obj) {
        if (obj == null) return MixedStreamConfig.defaultMixedStreamConfig();
        MixedStreamConfig config = MixedStreamConfig.defaultMixedStreamConfig();
        config.videoConfig = DataTransformer.MixedStreamVideoConfig(getJSONObject(obj, "videoConfig"));
        config.audioConfig = DataTransformer.MixedStreamAudioConfig(getJSONObject(obj, "audioConfig"));
        config.controlConfig = DataTransformer.MixedStreamControlConfig(getJSONObject(obj, "controlConfig"));
        config.syncControlConfig = DataTransformer.MixedStreamSyncControlConfig(getJSONObject(obj, "syncControlConfig"));
        config.spatialAudioConfig = DataTransformer.MixedStreamSpatialAudioConfig(getJSONObject(obj, "spatialAudioConfig"));
        config.roomID = getString(obj, "roomId", "");
        config.userID = getString(obj, "userId", "");
        config.syncControlConfig.baseUserID = config.userID;
        config.userConfigExtraInfo = getString(obj, "userConfigExtraInfo", "");
        config.backgroundColor = getString(obj, "backgroundColor", "#000000");
        config.backgroundImageURL = getString(obj, "backgroundImageUrl", "");
        config.interpolationMode = getEnumByIndex(obj, InterpolationMode.class, "interpolationMode");
        config.layoutMode = getEnumByIndex(obj, StreamLayoutMode.class, "layoutMode");
        config.advancedConfig = getJSONObject(obj, "advancedConfig");
        config.authInfo = getJSONObject(obj, "authInfo");
        JSONArray regionsList = getJSONArray(obj, "regions");
        if (regionsList != null) {
            config.regions = new MixedStreamLayoutRegionConfig[regionsList.length()];
            for (int i = 0; i < regionsList.length(); i++) {
                config.regions[i] = DataTransformer.MixedStreamLayoutRegionConfig(regionsList.optJSONObject(i));
            }
        }

        return config;
    }
}
