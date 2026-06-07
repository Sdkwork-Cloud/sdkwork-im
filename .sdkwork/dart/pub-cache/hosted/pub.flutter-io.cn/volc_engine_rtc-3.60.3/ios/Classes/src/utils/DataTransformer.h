// Copyright (c) 2026 Beijing Volcano Engine Technology Ltd.

//
//  DataTransformer.h
//  volc_engine_rtc
//
//  Provides transformation methods for ByteRTC mixed stream configurations
//

#import <Foundation/Foundation.h>
#import "BasicTransformer.h"
#import <VolcEngineRTC/objc/rtc/ByteRTCDefines.h>

NS_ASSUME_NONNULL_BEGIN

@interface DataTransformer : BasicTransformer

/**
 * @brief Transform dictionary to ByteRTCPosition
 * @param obj Source dictionary
 * @return ByteRTCPosition object
 */
+ (ByteRTCPosition *)Position:(nullable NSDictionary *)obj;

/**
 * @brief Transform dictionary to ByteRTCOrientation with default value
 * @param obj Source dictionary
 * @param defaultValue Default orientation value
 * @return ByteRTCOrientation object
 */
+ (ByteRTCOrientation *)Orientation:(nullable NSDictionary *)obj 
                        withDefault:(ByteRTCOrientation *)defaultValue;

/**
 * @brief Transform dictionary to ByteRTCHumanOrientation
 * @param obj Source dictionary
 * @return ByteRTCHumanOrientation object
 */
+ (ByteRTCHumanOrientation *)HumanOrientation:(nullable NSDictionary *)obj;

/**
 * @brief Transform dictionary to ByteRTCSourceCrop
 * @param obj Source dictionary
 * @return ByteRTCSourceCrop object
 */
+ (ByteRTCSourceCropInfo *)SourceCrop:(nullable NSDictionary *)obj;

/**
 * @brief Transform dictionary to ByteRTCMixedStreamLayoutRegionImageWaterMarkConfig
 * @param obj Source dictionary
 * @return ByteRTCMixedStreamLayoutRegionImageWaterMarkConfig object
 */
+ (ByteRTCMixedStreamLayoutRegionImageWaterMarkConfig *)MixedStreamLayoutRegionImageWaterMarkConfig:(nullable NSDictionary *)obj;

/**
 * @brief Transform dictionary to ByteRTCMixedStreamLayoutRegionConfig
 * @param obj Source dictionary
 * @return ByteRTCMixedStreamLayoutRegionConfig object
 */
+ (ByteRTCMixedStreamLayoutRegionConfig *)MixedStreamLayoutRegionConfig:(nullable NSDictionary *)obj;

/**
 * @brief Transform dictionary to ByteRTCMixedStreamVideoConfig
 * @param obj Source dictionary
 * @return ByteRTCMixedStreamVideoConfig object
 */
+ (ByteRTCMixedStreamVideoConfig *)MixedStreamVideoConfig:(nullable NSDictionary *)obj;

/**
 * @brief Transform dictionary to ByteRTCMixedStreamAudioConfig
 * @param obj Source dictionary
 * @return ByteRTCMixedStreamAudioConfig object
 */
+ (ByteRTCMixedStreamAudioConfig *)MixedStreamAudioConfig:(nullable NSDictionary *)obj;

/**
 * @brief Transform dictionary to ByteRTCMixedStreamControlConfig
 * @param obj Source dictionary
 * @return ByteRTCMixedStreamControlConfig object
 */
+ (ByteRTCMixedStreamControlConfig *)MixedStreamControlConfig:(nullable NSDictionary *)obj;

/**
 * @brief Transform dictionary to ByteRTCMixedStreamSpatialAudioConfig
 * @param obj Source dictionary
 * @return ByteRTCMixedStreamSpatialAudioConfig object
 */
+ (ByteRTCMixedStreamSpatialAudioConfig *)MixedStreamSpatialAudioConfig:(nullable NSDictionary *)obj;

/**
 * @brief Transform dictionary to ByteRTCMixedStreamSyncControlConfig
 * @param obj Source dictionary
 * @return ByteRTCMixedStreamSyncControlConfig object
 */
+ (ByteRTCMixedStreamSyncControlConfig *)MixedStreamSyncControlConfig:(nullable NSDictionary *)obj;

/**
 * @brief Transform dictionary to ByteRTCMixedStreamConfig (main transformer)
 * @param obj Source dictionary
 * @return ByteRTCMixedStreamConfig object
 */
+ (ByteRTCMixedStreamConfig *)MixedStreamConfig:(nullable NSDictionary *)obj;

@end

NS_ASSUME_NONNULL_END
