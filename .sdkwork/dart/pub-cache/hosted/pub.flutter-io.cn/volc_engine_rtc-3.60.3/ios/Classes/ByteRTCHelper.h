// Copyright (c) 2026 Beijing Volcano Engine Technology Ltd.

//
//  Pods
//

#import "VolcApiEngine/VolcViewManager.h"
#import <VolcEngineRTC/VolcEngineRTC.h>
#import <VolcEngineRTC/objc/ByteRTCEngine.h>
#import <VolcEngineRTC/objc/ByteRTCRoom.h>
#import "ByteRTCHelperDelegate.h"

@interface ByteRTCHelper : NSObject

@property (nonatomic, copy, nullable) NSString *groupId;

@property (nonatomic, copy, nullable) NSString *bundleId;

@property (nonatomic, strong) NSMutableDictionary * _Nonnull taskPaths;

@property (nonatomic, weak) id<ByteRTCHelperDelegate> delegate;

+ (instancetype _Nonnull) getInstance;

/**
 * @brief 设置 groupId 及 bundleId。
 * @note 如需使用屏幕共享, groupId、bundleId 必须设置。
 */
+ (void) setExtensionConfig:(NSString * _Nullable)groupId bundleId:(NSString * _Nullable)bundleId;

@end
