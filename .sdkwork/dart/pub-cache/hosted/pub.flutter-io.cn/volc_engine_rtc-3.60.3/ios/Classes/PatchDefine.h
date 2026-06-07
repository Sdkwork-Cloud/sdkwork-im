// Copyright (c) 2026 Beijing Volcano Engine Technology Ltd.

//
//  PatchDefine.h
//  Pods
//
//  Created by ByteDance on 2026/1/22.
//
#import <Foundation/Foundation.h>
#import <VolcEngineRTC/VolcEngineRTC.h>
#import "ByteRTCPlugin.h"

NS_ASSUME_NONNULL_BEGIN

#pragma mark - ByteRTCFaceDetectionResult

/**
 * @brief 人脸检测结果
 */
@interface PatchFaceDetectionResult : NSObject

/**
 * @brief 人脸检测结果 <br>
 *        - 0：检测成功
 *        - !0：检测失败。详见[错误码](https:
 */
@property(assign, nonatomic) int detectResult;

/**
 * @brief 原始图片宽度(px)
 */
@property(assign, nonatomic) int imageWidth;

/**
 * @brief 原始图片高度(px)
 */
@property(assign, nonatomic) int imageHeight;

/**
 * @brief 识别到人脸的矩形框。数组的长度和检测到的人脸数量一致。参看 ByteRTCRectangle{@link #ByteRTCRectangle}。
 */
@property(nonatomic, copy) NSArray<ByteRTCRectangle *> * _Nullable faces;

/**
 * @brief 进行人脸识别的视频帧的时间戳。
 */
@property(assign, nonatomic) int64_t frameTimestamp;

@end


#pragma mark - ByteRTCLocalAudioPropertiesInfo

/**
 * @type keytype
 * @brief 本地音频属性信息
 */
@interface PatchLocalAudioPropertiesInfo : NSObject

/**
 * @brief 音频源。预留参数。
 */
@property(assign, nonatomic) id audioSource;

/**
 * @brief 音频属性信息，详见 ByteRTCAudioPropertiesInfo{@link #ByteRTCAudioPropertiesInfo}
 */
@property(strong, nonatomic) ByteRTCAudioPropertiesInfo *_Nonnull audioPropertiesInfo;

@end


NS_ASSUME_NONNULL_END
