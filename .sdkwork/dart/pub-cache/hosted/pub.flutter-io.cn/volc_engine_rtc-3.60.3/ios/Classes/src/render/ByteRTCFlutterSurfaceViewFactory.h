// Copyright (c) 2026 Beijing Volcano Engine Technology Ltd.

#import <Foundation/Foundation.h>
#import <Flutter/Flutter.h>
#import <Flutter/FlutterPlatformViews.h>
#import "VolcApiEngine/VolcViewManager.h"
#import <VolcEngineRTC/objc/ByteRTCEngine.h>

NS_ASSUME_NONNULL_BEGIN

@interface ByteRTCFlutterSurfaceView: NSObject <FlutterPlatformView>

@property (nonatomic, strong) ByteRTCView *view;

- (instancetype)initWithMessager:(NSObject<FlutterBinaryMessenger>*)messenger
                           frame:(CGRect)frame
                  viewIdentifier:(int64_t)viewId
                       arguments:(id)args;

@end

@interface ByteRTCFlutterSurfaceViewFactory: NSObject <FlutterPlatformViewFactory>

- (instancetype)initWithMessenger:(NSObject<FlutterBinaryMessenger>*)messenger;

@end

NS_ASSUME_NONNULL_END
