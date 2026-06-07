// Copyright (c) 2026 Beijing Volcano Engine Technology Ltd.

#import "ByteRTCPlugin.h"
#import <VolcEngineRTC/VolcEngineRTC.h>
#import "VolcApiEngine/VolcViewManager.h"
#import <VolcApiEngine/VolcJsonManager.h>
#import <VolcApiEngine/VolcApiEngineUtils.h>
#import "ByteRTCFlutterSurfaceViewFactory.h"
#import "ByteRTCHelperDelegate.h"
#import "ByteRTCHelper.h"
#import "PatchDefine.h"

@interface ByteRTCPlugin () <
    ByteRTCMediaPlayerAudioFrameObserver,
    ByteRTCFaceDetectionObserver,
    ByteRTCEncryptHandler,
    ByteRTCSingScoringDelegate,
    ByteRTCWTNStreamDelegate,
    ByteRTCAudioFrameObserver,
    ByteRtcScreenCapturerExtDelegate,
    ByteRTCKTVManagerDelegate,
    ByteRTCAudioEffectPlayerEventHandler,
    ByteRTCAudioFrameProcessor,
    ByteRTCRoomDelegate,
    ByteRTCExternalVideoEncoderEventHandler,
    ByteRTCVideoProcessorDelegate,
    ByteRTCGameRoomDelegate,
    ByteRTCRTSRoomDelegate,
    ByteRTCMediaPlayerEventHandler,
    ByteRTCEngineDelegate,
    ByteRTCLocalEncodedVideoFrameObserver,
    ByteRTCRemoteEncodedVideoFrameObserver,
    ByteRTCKTVPlayerDelegate,
    ByteRTCMediaPlayerCustomSourceProvider,
    ByteRTCFaceDetectionObserver,
    ByteRTCHelperDelegate
>
@end

@implementation ByteRTCPlugin

static ByteRTCPlugin *_instance = nil;

+ (instancetype)getInstance {
    static dispatch_once_t onceToken;
    dispatch_once(&onceToken, ^{
        _instance = [[ByteRTCPlugin alloc] init];
    });
    return _instance;
}

+ (void) enableDataClass {
    VolcJsonManager* shareInstance = [VolcJsonManager sharedInstance];
    [shareInstance enableDataClass: [ByteRTCRemoteStreamSwitchEvent class]];
    [shareInstance enableDataClass: [ByteRTCLocalAudioPropertiesInfo class]];
    [shareInstance enableDataClass: [ByteRTCRecordingInfo class]];
    [shareInstance enableDataClass: [ByteRTCRecordingProgress class]];
    [shareInstance enableDataClass: [ByteRTCRemoteAudioPropertiesInfo class]];
    [shareInstance enableDataClass: [ByteRTCRemoteStreamKey class]];
    [shareInstance enableDataClass: [ByteRTCStreamSyncInfoConfig class]];
    [shareInstance enableDataClass: [ByteRTCEngine class]];
    [shareInstance enableDataClass: [ByteRTCRoom class]];
    [shareInstance enableDataClass: [ByteRTCEngineEx class]];
    [shareInstance enableDataClass: [ByteRTCRoomEx class]];
    [shareInstance enableDataClass: [ByteRTCUser class]];
    [shareInstance enableDataClass: [ByteRTCStreamInfo class]];
    [shareInstance enableDataClass: [ByteRTCSourceWantedData class]];
    [shareInstance enableDataClass: [ByteRTCSubscribeConfig class]];
    [shareInstance enableDataClass: [ByteRTCUserInfo class]];
    [shareInstance enableDataClass: [ByteRTCForwardStreamEventInfo class]];
    [shareInstance enableDataClass: [ByteRTCForwardStreamStateInfo class]];
    [shareInstance enableDataClass: [ByteRTCLocalStreamStats class]];
    [shareInstance enableDataClass: [ByteRTCNetworkQualityStats class]];
    [shareInstance enableDataClass: [ByteRTCRoomStats class]];
    [shareInstance enableDataClass: [ByteRTCRemoteStreamStats class]];
    [shareInstance enableDataClass: [ByteRTCSubtitleMessage class]];
    [shareInstance enableDataClass: [ByteRTCLocalVideoStats class]];
    [shareInstance enableDataClass: [ByteRTCSysStats class]];
    [shareInstance enableDataClass: [ByteRTCRecordingInfo class]];
    [shareInstance enableDataClass: [ByteRTCRecordingProgress class]];
    [shareInstance enableDataClass: [ByteRTCRemoteAudioPropertiesInfo class]];
    [shareInstance enableDataClass: [ByteRTCRemoteStreamKey class]];
    [shareInstance enableDataClass: [ByteRTCVideoFrameInfo class]];
    [shareInstance enableDataClass: [ByteRTCRemoteAudioStats class]];
    [shareInstance enableDataClass: [ByteRTCUser class]];
    [shareInstance enableDataClass: [ByteRTCSourceWantedData class]];
    [shareInstance enableDataClass: [ByteRTCSubscribeConfig class]];
    [shareInstance enableDataClass: [ByteRTCForwardStreamEventInfo class]];
    [shareInstance enableDataClass: [ByteRTCForwardStreamStateInfo class]];
    [shareInstance enableDataClass: [ByteRTCLocalStreamStats class]];
    [shareInstance enableDataClass: [ByteRTCNetworkQualityStats class]];
    [shareInstance enableDataClass: [ByteRTCRemoteStreamStats class]];
    [shareInstance enableDataClass: [ByteRTCSubtitleMessage class]];
    [shareInstance enableDataClass: [ByteRTCRemoteVideoStats class]];
    [shareInstance enableDataClass: [ByteRTCLocalAudioStats class]];
    [shareInstance enableDataClass: [ByteRTCLocalVideoStats class]];
    [shareInstance enableDataClass: [ByteRTCMusicInfo class]];
    [shareInstance enableDataClass: [ByteRTCHotMusicInfo class]];
    [shareInstance enableDataClass: [ByteRTCDownloadResult class]];
    [shareInstance enableDataClass: [ByteRTCAudioPropertiesInfo class]];
    [shareInstance enableDataClass: [ByteRTCFaceDetectionResult class]];
    [shareInstance enableDataClass: [ByteRTCExpressionDetectResult class]];
}

+ (void)registerWithRegistrar:(NSObject<FlutterPluginRegistrar> *)registrar {
  ByteRTCPlugin *instance = [ByteRTCPlugin getInstance];
  instance.apiEngine = [[VolcApiEngine alloc] init];
  [instance.apiEngine setObserver:instance];
    
  [self enableDataClass];

  // Custom resolvers
  // Start ....
  [VolcUtils setCustomPropertyResolver:^id(id instance) {
      return [ByteRTCPlugin audioSourceResolver:instance];
  } forClass:[ByteRTCLocalAudioPropertiesInfo class]];
  [VolcUtils setCustomPropertyResolver:^id(id instance) {
      return [ByteRTCPlugin faceDetectionResultResolver:instance];
  } forClass:[ByteRTCFaceDetectionResult class]];
  // End ....

  FlutterMethodChannel *channel = [FlutterMethodChannel
      methodChannelWithName:@"com.volcengine.rtc.hybrid_runtime"
            binaryMessenger:[registrar messenger]];
  instance.channel = channel;

  [registrar addMethodCallDelegate:instance channel:channel];

  ByteRTCFlutterSurfaceView * factory = [[ByteRTCFlutterSurfaceViewFactory alloc] initWithMessenger:[registrar messenger]];
  [registrar registerViewFactory:factory withId:@"ByteRTCSurfaceView"];
}

- (void)handleMethodCall:(FlutterMethodCall *)call
                  result:(FlutterResult)result {
  if ([@"callApi" isEqualToString:call.method]) {
    NSDictionary *args = call.arguments;
    @try {
//      if ([args[@"params"] containsString:@"joinRoom"]) {
//           NSDictionary *response = [self.apiEngine callApi:args];
//           result(response);
//      } else {
//          NSDictionary *response = [self.apiEngine callApi:args];
//          result(response);
//      }
        NSDictionary *response = [self.apiEngine callApi:args];
        result(response);
    } @catch (NSException *exception) {
      result([FlutterError errorWithCode:@"CALL_API_ERROR"
                                 message:[exception reason]
                                 details:nil]);
    }
  } else {
    result(FlutterMethodNotImplemented);
  }
}

#pragma mark - EventObserver

- (void)onEvent:(NSString *)eventName data:(id)eventData {
    if (self.channel != nil) {
        @try {
            NSDictionary *event = @{@"event" : eventName, @"data" : eventData ?: @""};
            dispatch_async(dispatch_get_main_queue(), ^{
                @try {
                    NSDictionary *dataDict = (NSDictionary *)eventData;
                    NSString *methodName = dataDict[@"methodName"];
                    if ([methodName containsString:@"OnLocalAudioPropertiesReport"]) {
//                        event[@"data"][@"methodName"] = @"rtcEngine:onNetworkTimeSynchronized:";
//                        event[@"data"][@"serviceName"] = @"rtcEngine:onNetworkTimeSynchronized:";
                        NSLog(@"log");
                    }
                    [self.channel invokeMethod:@"onEvent" arguments:event];
                } @catch (NSException *exception) {
                    NSLog(@"Exception in channel invoke: %@", exception);
                }
            });
        } @catch (NSException *exception) {
            NSLog(@"Exception in event creation: %@", exception);
        }
    }
}

#pragma mark - Patch resolvers

+ (PatchLocalAudioPropertiesInfo *) audioSourceResolver:(ByteRTCLocalAudioPropertiesInfo *)arg {
    if (!arg) {
        return nil;
    }
    PatchLocalAudioPropertiesInfo* patch = [[PatchLocalAudioPropertiesInfo alloc] init];
    @synchronized(arg) {
        patch.audioSource = @{};
        patch.audioPropertiesInfo = arg.audioPropertiesInfo;
    }
    return patch;
}

+ (PatchFaceDetectionResult *) faceDetectionResultResolver:(ByteRTCFaceDetectionResult *)arg {
    if (!arg) {
        return nil;
    }
    PatchFaceDetectionResult* patch = [[PatchFaceDetectionResult alloc] init];
    @synchronized(arg) {
        CMTime timeInUs = CMTimeConvertScale(arg.frameTimestamp, 1000000, kCMTimeRoundingMethod_Default);
        patch.detectResult = arg.detectResult;
        patch.imageWidth = arg.imageWidth;
        patch.imageHeight = arg.imageHeight;
        patch.faces = arg.faces;
        patch.frameTimestamp = timeInUs.value;
    }
    return patch;
}

@end
