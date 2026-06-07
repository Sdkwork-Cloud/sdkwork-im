// Copyright (c) 2026 Beijing Volcano Engine Technology Ltd.

//
//  DataTransformer.m
//  volc_engine_rtc
//
//  Provides transformation methods for ByteRTC mixed stream configurations
//

#import "DataTransformer.h"

@implementation DataTransformer

+ (ByteRTCPosition *)Position:(nullable NSDictionary *)obj {
    ByteRTCPosition *pos = [[ByteRTCPosition alloc] init];
    if (obj == nil) {
        pos.x = 0;
        pos.y = 0;
        pos.z = 0;
        return pos;
    }
    
    pos.x = [self getInt:obj forKey:@"x" withDefault:0];
    pos.y = [self getInt:obj forKey:@"y" withDefault:0];
    pos.z = [self getInt:obj forKey:@"z" withDefault:0];
    return pos;
}

+ (ByteRTCOrientation *)Orientation:(nullable NSDictionary *)obj 
                        withDefault:(ByteRTCOrientation *)defaultValue {
    if (obj == nil) {
        return defaultValue;
    }
    
    ByteRTCOrientation *orientation = [[ByteRTCOrientation alloc] init];
    orientation.x = [self getFloat:obj forKey:@"x" withDefault:defaultValue.x];
    orientation.y = [self getFloat:obj forKey:@"y" withDefault:defaultValue.y];
    orientation.z = [self getFloat:obj forKey:@"z" withDefault:defaultValue.z];
    return orientation;
}

+ (ByteRTCHumanOrientation *)HumanOrientation:(nullable NSDictionary *)obj {
    ByteRTCHumanOrientation *humanOrientation = [[ByteRTCHumanOrientation alloc] init];
    if (obj == nil) {
        return humanOrientation;
    }
    
    NSDictionary *forwardDict = [self getJSONObject:obj forKey:@"forward"];
    NSDictionary *rightDict = [self getJSONObject:obj forKey:@"right"];
    NSDictionary *upDict = [self getJSONObject:obj forKey:@"up"];
    
    humanOrientation.forward = [self Orientation:forwardDict withDefault:humanOrientation.forward];
    humanOrientation.right = [self Orientation:rightDict withDefault:humanOrientation.right];
    humanOrientation.up = [self Orientation:upDict withDefault:humanOrientation.up];
    
    return humanOrientation;
}

+ (ByteRTCSourceCropInfo *)SourceCrop:(nullable NSDictionary *)obj {
    ByteRTCSourceCropInfo *crop = [[ByteRTCSourceCropInfo alloc] init];
    if (obj == nil) {
        crop.locationX = 0.0;
        crop.locationY = 0.0;
        crop.widthProportion = 1.0;
        crop.heightProportion = 1.0;
        return crop;
    }
    
    crop.locationX = [self getDouble:obj forKey:@"locationX" withDefault:0.0];
    crop.locationY = [self getDouble:obj forKey:@"locationY" withDefault:0.0];
    crop.widthProportion = [self getDouble:obj forKey:@"widthProportion" withDefault:1.0];
    crop.heightProportion = [self getDouble:obj forKey:@"heightProportion" withDefault:1.0];
    return crop;
}

+ (ByteRTCMixedStreamLayoutRegionImageWaterMarkConfig *)MixedStreamLayoutRegionImageWaterMarkConfig:(nullable NSDictionary *)obj {
    ByteRTCMixedStreamLayoutRegionImageWaterMarkConfig *config = [[ByteRTCMixedStreamLayoutRegionImageWaterMarkConfig alloc] init];
    if (obj == nil) {
        config.imageWidth = 0;
        config.imageHeight = 0;
        return config;
    }
    
    config.imageWidth = [self getInt:obj forKey:@"imageWidth" withDefault:0];
    config.imageHeight = [self getInt:obj forKey:@"imageHeight" withDefault:0];
    return config;
}

+ (ByteRTCMixedStreamLayoutRegionConfig *)MixedStreamLayoutRegionConfig:(nullable NSDictionary *)obj {
    ByteRTCMixedStreamLayoutRegionConfig *config = [[ByteRTCMixedStreamLayoutRegionConfig alloc] init];
    if (obj == nil) {
        return config;
    }
    
    config.roomID = [self getString:obj forKey:@"roomId" withDefault:@""];
    config.userID = [self getString:obj forKey:@"userId" withDefault:@""];
    config.locationX = [self getInt:obj forKey:@"locationX" withDefault:0];
    config.locationY = [self getInt:obj forKey:@"locationY" withDefault:0];
    config.width = [self getInt:obj forKey:@"width" withDefault:360];
    config.height = [self getInt:obj forKey:@"height" withDefault:640];
    config.zOrder = [self getInt:obj forKey:@"zOrder" withDefault:0];
    config.alpha = [self getDouble:obj forKey:@"alpha" withDefault:1.0];
    config.cornerRadius = [self getDouble:obj forKey:@"cornerRadius" withDefault:0.0];
    
    // Enum mappings
    NSInteger mediaTypeIndex = [self getInt:obj forKey:@"mediaType" withDefault:0];
    config.mediaType = (ByteRTCMixedStreamMediaType)mediaTypeIndex;
    
    NSInteger renderModeIndex = [self getInt:obj forKey:@"renderMode" withDefault:0];
    config.renderMode = (ByteRTCMixedStreamRenderMode)(renderModeIndex + 1);
    
    config.isLocalUser = [self getBoolean:obj forKey:@"isLocalUser" withDefault:NO];
    
    NSInteger streamTypeIndex = [self getInt:obj forKey:@"streamType" withDefault:0];
    config.streamType = (ByteRTCMixedStreamVideoType)streamTypeIndex;
    
    NSInteger regionContentTypeIndex = [self getInt:obj forKey:@"regionContentType" withDefault:0];
    config.regionContentType = (ByteRTCMixedStreamLayoutRegionType)regionContentTypeIndex;
    
    config.imageWaterMark = [self getBytes:obj forKey:@"imageWaterMark" withDefault:nil];
    
    NSDictionary *imageWaterMarkConfigDict = [self getJSONObject:obj forKey:@"imageWaterMarkConfig"];
    config.imageWaterMarkConfig = [self MixedStreamLayoutRegionImageWaterMarkConfig:imageWaterMarkConfigDict];
    
    NSInteger alternateFillModeIndex = [self getInt:obj forKey:@"alternateImageFillMode" withDefault:0];
    config.alternateImageFillMode = (ByteRTCMixedStreamAlternateImageFillMode)alternateFillModeIndex;
    
    config.alternateImageUrl = [self getString:obj forKey:@"alternateImageUrl" withDefault:@""];
    
    NSDictionary *spatialPositionDict = [self getJSONObject:obj forKey:@"spatialPosition"];
    config.spatialPosition = [self Position:spatialPositionDict];
    
    config.applySpatialAudio = [self getBoolean:obj forKey:@"applySpatialAudio" withDefault:YES];
    
    NSDictionary *sourceCropDict = [self getJSONObject:obj forKey:@"sourceCrop"];
    config.sourceCrop = [self SourceCrop:sourceCropDict];
    
    return config;
}

+ (ByteRTCMixedStreamVideoConfig *)MixedStreamVideoConfig:(nullable NSDictionary *)obj {
    ByteRTCMixedStreamVideoConfig *config = [[ByteRTCMixedStreamVideoConfig alloc] init];
    if (obj == nil) {
        config.fps = 15;
        config.gop = 2;
        config.bitrate = 500;
        config.width = 360;
        config.height = 640;
        config.enableBFrame = NO;
        return config;
    }
    
    config.fps = [self getInt:obj forKey:@"fps" withDefault:15];
    config.gop = [self getInt:obj forKey:@"gop" withDefault:2];
    config.bitrate = [self getInt:obj forKey:@"bitrate" withDefault:500];
    config.width = [self getInt:obj forKey:@"width" withDefault:360];
    config.height = [self getInt:obj forKey:@"height" withDefault:640];
    config.enableBFrame = [self getBoolean:obj forKey:@"enableBframe" withDefault:NO];
    return config;
}

+ (ByteRTCMixedStreamAudioConfig *)MixedStreamAudioConfig:(nullable NSDictionary *)obj {
    ByteRTCMixedStreamAudioConfig *config = [[ByteRTCMixedStreamAudioConfig alloc] init];
    if (obj == nil) {
        config.bitrate = 64;
        config.sampleRate = 48000;
        config.channels = 2;
        config.audioProfile = ByteRTCMixedStreamAudioProfileLC;
        return config;
    }
    
    config.bitrate = [self getInt:obj forKey:@"bitrate" withDefault:64];
    config.sampleRate = [self getInt:obj forKey:@"sampleRate" withDefault:48000];
    config.channels = [self getInt:obj forKey:@"channels" withDefault:2];
    
    NSInteger audioProfileIndex = [self getInt:obj forKey:@"audioProfile" withDefault:0];
    config.audioProfile = (ByteRTCMixedStreamAudioProfile)audioProfileIndex;
    
    return config;
}

+ (ByteRTCMixedStreamControlConfig *)MixedStreamControlConfig:(nullable NSDictionary *)obj {
    ByteRTCMixedStreamControlConfig *config = [[ByteRTCMixedStreamControlConfig alloc] init];
    if (obj == nil) {
        config.enableVolumeIndication = NO;
        config.volumeIndicationInterval = 2.0f;
        config.talkVolume = 0;
        config.isAddVolumeValue = NO;
        config.seiContentMode = ByteRTCMixedStreamSEIContentModeDefault;
        config.seiPayloadType = 100;
        config.seiPayloadUUID = @"";
        config.mediaType = ByteRTCMixedStreamMediaTypeAudioAndVideo;
        config.pushStreamMode = ByteRTCMixedStreamPushModeOnStream;
        return config;
    }
    
    config.enableVolumeIndication = [self getBoolean:obj forKey:@"enableVolumeIndication" withDefault:NO];
    config.volumeIndicationInterval = [self getFloat:obj forKey:@"volumeIndicationInterval" withDefault:2.0f];
    config.talkVolume = [self getInt:obj forKey:@"talkVolume" withDefault:0];
    config.isAddVolumeValue = [self getBoolean:obj forKey:@"isAddVolumeValue" withDefault:NO];
    
    NSInteger seiContentModeIndex = [self getInt:obj forKey:@"seiContentMode" withDefault:0];
    config.seiContentMode = (ByteRTCMixedStreamSEIContentMode)seiContentModeIndex;
    
    config.seiPayloadType = [self getInt:obj forKey:@"seiPayloadType" withDefault:100];
    config.seiPayloadUUID = [self getString:obj forKey:@"seiPayloadUuid" withDefault:@""];
    
    NSInteger mediaTypeIndex = [self getInt:obj forKey:@"mediaType" withDefault:0];
    config.mediaType = (ByteRTCMixedStreamMediaType)mediaTypeIndex;
    
    NSInteger pushStreamModeIndex = [self getInt:obj forKey:@"pushStreamMode" withDefault:0];
    config.pushStreamMode = (ByteRTCMixedStreamPushMode)pushStreamModeIndex;
    
    return config;
}

+ (ByteRTCMixedStreamSpatialAudioConfig *)MixedStreamSpatialAudioConfig:(nullable NSDictionary *)obj {
    ByteRTCMixedStreamSpatialAudioConfig *config = [[ByteRTCMixedStreamSpatialAudioConfig alloc] init];
    if (obj == nil) {
        config.enableSpatialRender = NO;
        return config;
    }
    
    config.enableSpatialRender = [self getBoolean:obj forKey:@"enableSpatialRender" withDefault:NO];
    
    NSDictionary *audienceSpatialPositionDict = [self getJSONObject:obj forKey:@"audienceSpatialPosition"];
    config.audienceSpatialPosition = [self Position:audienceSpatialPositionDict];
    
    NSDictionary *audienceSpatialOrientationDict = [self getJSONObject:obj forKey:@"audienceSpatialOrientation"];
    config.audienceSpatialOrientation = [self HumanOrientation:audienceSpatialOrientationDict];
    
    return config;
}

+ (ByteRTCMixedStreamSyncControlConfig *)MixedStreamSyncControlConfig:(nullable NSDictionary *)obj {
    ByteRTCMixedStreamSyncControlConfig *config = [[ByteRTCMixedStreamSyncControlConfig alloc] init];
    if (obj == nil) {
        config.baseUserID = @"";
        config.syncStrategy = ByteRTCMixedStreamSyncStrategyNoSync;
        config.maxCacheTimeMs = 2000;
        config.videoNeedSdkMix = YES;
        return config;
    }
    
    config.baseUserID = [self getString:obj forKey:@"baseUserID" withDefault:@""];
    
    NSInteger syncStrategyIndex = [self getInt:obj forKey:@"syncStrategy" withDefault:0];
    config.syncStrategy = (ByteRTCMixedStreamSyncStrategy)syncStrategyIndex;
    
    config.maxCacheTimeMs = [self getInt:obj forKey:@"maxCacheTimeMs" withDefault:2000];
    config.videoNeedSdkMix = [self getBoolean:obj forKey:@"videoNeedSdkMix" withDefault:YES];
    
    return config;
}

+ (ByteRTCMixedStreamConfig *)MixedStreamConfig:(nullable NSDictionary *)obj {
    ByteRTCMixedStreamConfig *config = [ByteRTCMixedStreamConfig defaultMixedStreamConfig];
    if (obj == nil) {
        return config;
    }
    
    // Transform nested configurations
    NSDictionary *videoConfigDict = [self getJSONObject:obj forKey:@"videoConfig"];
    config.videoConfig = [self MixedStreamVideoConfig:videoConfigDict];
    
    NSDictionary *audioConfigDict = [self getJSONObject:obj forKey:@"audioConfig"];
    config.audioConfig = [self MixedStreamAudioConfig:audioConfigDict];
    
    NSDictionary *controlConfigDict = [self getJSONObject:obj forKey:@"controlConfig"];
    config.controlConfig = [self MixedStreamControlConfig:controlConfigDict];
    
    NSDictionary *syncControlConfigDict = [self getJSONObject:obj forKey:@"syncControlConfig"];
    config.syncControlConfig = [self MixedStreamSyncControlConfig:syncControlConfigDict];
    
    NSDictionary *spatialAudioConfigDict = [self getJSONObject:obj forKey:@"spatialAudioConfig"];
    config.spatialAudioConfig = [self MixedStreamSpatialAudioConfig:spatialAudioConfigDict];
    
    // Set basic properties
    config.roomID = [self getString:obj forKey:@"roomId" withDefault:@""];
    config.userID = [self getString:obj forKey:@"userId" withDefault:@""];
    
    config.userConfigExtraInfo = [self getString:obj forKey:@"userConfigExtraInfo" withDefault:@""];
    config.backgroundColor = [self getString:obj forKey:@"backgroundColor" withDefault:@"#000000"];
    config.backgroundImageURL = [self getString:obj forKey:@"backgroundImageUrl" withDefault:@""];
    
    NSInteger interpolationModeIndex = [self getInt:obj forKey:@"interpolationMode" withDefault:0];
    config.interpolationMode = (ByteRTCInterpolationMode)interpolationModeIndex;
    
    NSInteger pushTargetType = [self getInt:obj forKey:@"pushTargetType" withDefault:0];
    config.pushTargetType = (ByteRTCMixedStreamPushTargetType)pushTargetType;
    
    NSInteger layoutModeIndex = [self getInt:obj forKey:@"layoutMode" withDefault:0];
    config.layoutMode = (ByteRTCStreamLayoutMode)layoutModeIndex;
    
    // Handle advancedConfig and authInfo as dictionaries
    NSDictionary *advancedConfigDict = [self getJSONObject:obj forKey:@"advancedConfig"];
    if (advancedConfigDict != nil) {
        config.advancedConfig = advancedConfigDict;
    }
    
    NSDictionary *authInfoDict = [self getJSONObject:obj forKey:@"authInfo"];
    if (authInfoDict != nil) {
        config.authInfo = authInfoDict;
    }
    
    // Handle regions array
    NSArray *regionsArray = [self getJSONArray:obj forKey:@"regions"];
    if (regionsArray != nil && regionsArray.count > 0) {
        NSMutableArray<ByteRTCMixedStreamLayoutRegionConfig *> *regions = [NSMutableArray arrayWithCapacity:regionsArray.count];
        for (id regionObj in regionsArray) {
            if ([regionObj isKindOfClass:[NSDictionary class]]) {
                ByteRTCMixedStreamLayoutRegionConfig *regionConfig = [self MixedStreamLayoutRegionConfig:(NSDictionary *)regionObj];
                [regions addObject:regionConfig];
            }
        }
        config.regions = [regions copy];
    }
    
    return config;
}

@end
