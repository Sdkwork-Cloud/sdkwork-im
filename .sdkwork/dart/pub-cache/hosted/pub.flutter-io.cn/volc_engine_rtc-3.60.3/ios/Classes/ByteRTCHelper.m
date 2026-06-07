// Copyright (c) 2026 Beijing Volcano Engine Technology Ltd.

//
//  Helper
//

#import <Foundation/Foundation.h>
#import "ByteRTCPlugin.h"
#import "ByteRTCHelper.h"
#import <VolcEngineRTC/objc/ByteRTCEngine.h>
#import <VolcApiEngine/VolcJsonManager.h>
#import "ByteRTCHelperDelegate.h"
#import "src/utils/DataTransformer.h"

@implementation ByteRTCHelper

#pragma mark ByteRTCHelper

+ (instancetype)getInstance {
    static ByteRTCHelper *helper = nil;
    static dispatch_once_t onceToken;
    dispatch_once(&onceToken, ^{
        helper = [[ByteRTCHelper alloc] init];
    });
    return helper;
}


+ (bool)isUnionLiveModeEnable {
    return false;
}

- (UIView * _Nonnull)getView:(NSString * _Nonnull)viewId {
    if (!viewId || viewId.length == 0) {
        NSLog(@"Error: viewId is nil or empty");
        return nil;
    }
    
    UIView *view = [VolcViewManager.sharedInstance.viewMap objectForKey:viewId];
    if (!view) {
        NSLog(@"Warning: No view found for viewId: %@", viewId);
    }
    return view;
}

#pragma mark ByteRTCEngine

+ (void) setExtensionConfig:(NSString * _Nullable)groupId bundleId:(NSString * _Nullable)bundleId {
    ByteRTCHelper.getInstance.bundleId = bundleId;
    ByteRTCHelper.getInstance.groupId = groupId;
}

- (void) setExtensionConfig:(ByteRTCEngine * _Nonnull)rtc groupId:(NSString * _Nullable)groupId bundleId:(NSString * _Nullable)bundleId {
    ByteRTCHelper.getInstance.bundleId = bundleId;
    ByteRTCHelper.getInstance.groupId = groupId;
    [rtc setExtensionConfig:groupId];
}


- (int)updateRemoteStreamVideoCanvas:(ByteRTCEngine * _Nonnull)rtc streamId:(NSString * _Nonnull)streamId renderMode:(ByteRTCRenderMode)renderMode backgroundColor:(NSInteger)color {
    int res = [rtc updateRemoteStreamVideoCanvas:streamId
                                  withRenderMode:renderMode
                             withBackgroundColor:color];
    return res;
}

- (int)removeLocalVideo:(ByteRTCEngine * _Nonnull)rtc {
    int res = [rtc setLocalVideoCanvas:nil];
    return res;
}

- (int)removeRemoteVideo:(ByteRTCEngine * _Nonnull)rtc streamId:(NSString *)streamId streamIndex:(ByteRTCStreamIndex)streamIndex {
    int res = [rtc setRemoteVideoCanvas:streamId withCanvas:nil];
    return res;
}

- (int)startScreenCapture:(ByteRTCEngine * _Nonnull)rtc type:(int)sourceType {
    NSString * bundleId = ByteRTCHelper.getInstance.bundleId;
    if (bundleId == nil) {
        NSLog(@"[ByteRTCHelper] invoke error：bundleId 为空, 请先调用 setExtensionConfig 设置相关参数。");
        return -2;
    }
    int res = [rtc startScreenCapture:sourceType bundleId:bundleId];
    return res;
}

- (NSInteger)takeLocalSnapshot:(ByteRTCEngine * _Nonnull)rtc filePath:(NSString *)filePath {
    if (self.taskPaths == nil) {
        self.taskPaths = [NSMutableDictionary dictionary];
    }
    NSInteger taskId = [rtc takeLocalSnapshot:self];
    [self addFilePath:filePath forId:taskId];
    return taskId;
}

- (NSInteger)takeRemoteSnapshot:(ByteRTCEngine * _Nonnull)rtc streamId:(NSString * _Nonnull)streamId filePath:(NSString *)filePath {
    if (self.taskPaths == nil) {
        self.taskPaths = [NSMutableDictionary dictionary];
    }
    NSInteger taskId = [rtc takeRemoteSnapshot:streamId callback:self];
    [self addFilePath:filePath forId:taskId];
    return taskId;
}

- (int)startPushMixedStream:(ByteRTCEngine * _Nonnull)rtc taskId:(NSString *)taskId withPushTargetConfig:(ByteRTCMixedStreamPushTargetConfig *)withPushTargetConfig withMixedConfig:(NSDictionary *)withMixedConfig {
    @try {
        ByteRTCMixedStreamConfig *config = [DataTransformer MixedStreamConfig:withMixedConfig];
        int res = [rtc startPushMixedStream:taskId withPushTargetConfig:withPushTargetConfig withMixedConfig:config];
        return res;
    } @catch (NSException *exception) {
        NSLog(@"[ByteRTCHelper] startPushMixedStream failed: %@", exception.reason);
        return -1;
    }
}

- (int)updatePushMixedStream:(ByteRTCEngine * _Nonnull)rtc taskId:(NSString *)taskId withPushTargetConfig:(ByteRTCMixedStreamPushTargetConfig *)withPushTargetConfig withMixedConfig:(NSDictionary *)withMixedConfig {
    @try {
        ByteRTCMixedStreamConfig *config = [DataTransformer MixedStreamConfig:withMixedConfig];
        int res = [rtc updatePushMixedStream:taskId withPushTargetConfig:withPushTargetConfig withMixedConfig:config];
        return res;
    } @catch (NSException *exception) {
        NSLog(@"[ByteRTCHelper] updatePushMixedStream failed: %@", exception.reason);
        return -1;
    }
}

- (ByteRTCAudioRoute)getAudioRoute:(ByteRTCEngine * _Nonnull)rtc {
    @try {
        return [rtc getAudioRoute];
    } @catch (NSException *exception) {
        NSLog(@"[ByteRTCHelper] getAudioRoute failed: %@", exception.reason);
        return -1;
    }
}

- (ByteRTCAudioRoute)setVideoCaptureConfig:(ByteRTCEngine * _Nonnull)rtc videoCaptureConfig:(NSDictionary *)arguments{
    @try {
        ByteRTCVideoCaptureConfig* config = [ByteRTCVideoCaptureConfig alloc];
        CGSize size;
        size.width = [(NSNumber *)arguments[@"width"] floatValue];
        size.height = [(NSNumber *)arguments[@"height"] floatValue];
        config.videoSize = size;
        config.preference = [(NSNumber *)arguments[@"preference"] integerValue];
        config.frameRate = [(NSNumber *)arguments[@"frameRate"] integerValue];
        return [rtc setVideoCaptureConfig:config];
    } @catch (NSException *exception) {
        NSLog(@"[ByteRTCHelper] setVideoCaptureConfig failed: %@", exception.reason);
        return -1;
    }
}

- (int)feedback:(ByteRTCEngine * _Nonnull)rtc types:(NSArray<NSNumber *> * _Nonnull)types info:(ByteRTCProblemFeedbackInfo * _Nonnull)info {
    __block ByteRTCProblemFeedbackOption option = ByteRTCProblemFeedbackOptionNone;
    static NSArray *allOptions = nil;
    static dispatch_once_t onceToken;
    dispatch_once(&onceToken, ^{
        allOptions = @[
            @(ByteRTCProblemFeedbackOptionNone),
            @(ByteRTCProblemFeedbackOptionOtherMessage),
            @(ByteRTCProblemFeedbackOptionDisconnected),
            @(ByteRTCProblemFeedbackOptionEarBackDelay),
            @(ByteRTCProblemFeedbackOptionLocalNoise),
            @(ByteRTCProblemFeedbackOptionLocalAudioLagging),
            @(ByteRTCProblemFeedbackOptionLocalNoAudio),
            @(ByteRTCProblemFeedbackOptionLocalAudioStrength),
            @(ByteRTCProblemFeedbackOptionLocalEcho),
            @(ByteRTCProblemFeedbackOptionLocalVideoFuzzy),
            @(ByteRTCProblemFeedbackOptionLocalNotSync),
            @(ByteRTCProblemFeedbackOptionLocalVideoLagging),
            @(ByteRTCProblemFeedbackOptionLocalNoVideo),
            @(ByteRTCProblemFeedbackOptionRemoteNoise),
            @(ByteRTCProblemFeedbackOptionRemoteAudioLagging),
            @(ByteRTCProblemFeedbackOptionRemoteNoAudio),
            @(ByteRTCProblemFeedbackOptionRemoteAudioStrength),
            @(ByteRTCProblemFeedbackOptionRemoteEcho),
            @(ByteRTCProblemFeedbackOptionRemoteVideoFuzzy),
            @(ByteRTCProblemFeedbackOptionRemoteNotSync),
            @(ByteRTCProblemFeedbackOptionRemoteVideoLagging),
            @(ByteRTCProblemFeedbackOptionRemoteNoVideo)
        ];
    });
    [types enumerateObjectsUsingBlock:^(NSNumber *obj, NSUInteger idx, BOOL * _Nonnull stop) {
        NSInteger index = [obj integerValue];
        if (index >= 0 && index < allOptions.count) {
            ByteRTCProblemFeedbackOption enumValue = [allOptions[index] unsignedLongLongValue];
            option |= enumValue;
        }
    }];
    int res = [rtc feedback:option info:info];
    return res;
}

#pragma mark ByteRTCEngineEffect

- (int) enableVirtualBackground:(ByteRTCVideoEffect * _Nonnull)effect modelPath:(NSString *)modelPath arguments:(NSDictionary *)arguments {
    ByteRTCVirtualBackgroundSource * source = [ByteRTCVirtualBackgroundSource alloc];
    source.sourcePath = arguments[@"sourcePath"];
    source.sourceColor = arguments[@"sourceColor"];
    source.sourceType = arguments[@"sourceType"];
    return [effect enableVirtualBackground:modelPath withSource:source];
}

- (ByteRTCPlayerState) getState:(ByteRTCMediaPlayer * _Nonnull)player {
    return [player getState];
}

#pragma mark - Extra Tools

- (void)addFilePath:(NSString*)filePath
              forId:(NSInteger)taskId{
    self.taskPaths[@(taskId)] = filePath;
}

- (NSInteger)writeImageToFile:(ByteRTCImage * _Nullable)image
                    filePath:(NSString*)filePath
                   errorCode:(NSInteger)errorCode {
    if (image == nil){
        return errorCode;
    }

    NSData * data = UIImageJPEGRepresentation(image, 1);
    if (data == nil){
        return -103; // ERROR_IMAGE_FORMAT
    }
    NSError* err = nil;
    [data writeToFile:filePath options:NSDataWritingAtomic error:&err];
    if (err != nil) {
        NSLog(@"Exception in writeImageToFile: %@, might beacuse the path is not legal.", err);
        return -102; // WRITE_FILE_FAILED
    }
    return errorCode;
}

#pragma mark - SnapShot

- (int)addSnapshotEventHandler:(id<ByteRTCHelperDelegate> _Nonnull) delegate {
    @try {
        // coverable.
        self.delegate = delegate;
        return 0;
    } @catch(NSException *exception) {
        printf("AddSnapshotEventHandler failed, sdk inner error.");
    }
}



- (void) onTakeLocalSnapshotResult:(NSInteger) taskId
                       videoSource:(ByteRTCVideoSource *)videoSource
                            image:(ByteRTCImage * _Nullable)image
                        errorCode:(NSInteger)errorCode {
    NSString *filePath = self.taskPaths[@(taskId)];
    if (filePath == nil) {
        return;
    }
    NSInteger error = [self writeImageToFile:image
                                        filePath:filePath
                                       errorCode:errorCode];
    NSInteger width = 0;
    NSInteger height = 0;
    if (image != nil) {
        width = image.size.width;
        height = image.size.height;
    }
    if (self.delegate) {
        @try {
            [self.delegate onTakeLocalSnapshotResult:taskId
                                              width:width
                                             height:height
                                           filePath:filePath
                                           errorCode:error];
        } @catch(NSException *exception) {
            NSLog(@"Cannot find onTakeLocalSnapshotResult, sdk inner error.");
        }
    }
}

- (void) onTakeRemoteSnapshotResult:(NSInteger)taskId
                           streamId:(NSString * _Nonnull)streamId
                               info:(ByteRTCStreamInfo * _Nonnull)streamInfo
                              image:(ByteRTCImage * _Nullable)image
                          errorCode:(NSInteger)errorCode {
    NSString *filePath = self.taskPaths[@(taskId)];
    if (filePath == nil) {
        return;
    }
    NSInteger error = [self writeImageToFile:image
                                        filePath:filePath
                                       errorCode:errorCode];
    NSInteger width = 0;
    NSInteger height = 0;
    if (image != nil) {
        width = image.size.width;
        height = image.size.height;
    }
    if (self.delegate) {
        @try {
            [self.delegate onTakeRemoteSnapshotResult:taskId streamId:streamId streamInfo:streamInfo width:width height:height filePath:filePath errorCode:error];
        } @catch(NSException *exception) {
            NSLog(@"Cannot find onTakeRemoteSnapshotResult, sdk inner error.");
        }
    }
}
@end
