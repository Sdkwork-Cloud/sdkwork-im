// Copyright (c) 2026 Beijing Volcano Engine Technology Ltd.

#import "ByteRTCFlutterSurfaceViewFactory.h"
#import "VolcApiEngine/VolcViewManager.h"
#import <VolcEngineRTC/objc/ByteRTCEngine.h>
#import "ByteRTCHelper.h"

@implementation ByteRTCFlutterSurfaceView

- (instancetype)initWithMessager:(NSObject<FlutterBinaryMessenger>*)messenger
                           frame:(CGRect)frame
                  viewIdentifier:(int64_t)viewId
                       arguments:(id)args {
    self = [super init];
    if (self) {
        self.view = [[UIView alloc] initWithFrame:frame];
    }
    return self;
}

@end



@interface ByteRTCFlutterSurfaceViewFactory ()

@property(nonatomic, strong) NSObject<FlutterBinaryMessenger> *messenger;

@end

@implementation ByteRTCFlutterSurfaceViewFactory

- (instancetype)initWithMessenger:(NSObject<FlutterBinaryMessenger>*)messenger {
    self = [super init];
    if (self) {
        _messenger = messenger;
    }
    return self;
}

- (NSObject<FlutterMessageCodec>*)createArgsCodec {
    return [FlutterStandardMessageCodec sharedInstance];
}

- (NSObject<FlutterPlatformView> *)createWithFrame:(CGRect)frame
                                    viewIdentifier:(int64_t)viewId
                                         arguments:(id)args {
    ByteRTCFlutterSurfaceView * flutterView = [[ByteRTCFlutterSurfaceView alloc] initWithMessager:self.messenger
                                                                                          frame:frame
                                                                                 viewIdentifier:viewId
                                                                                      arguments:args];
    [VolcViewManager registerView:@(viewId).stringValue view:flutterView.view];
    return flutterView;
}

@end
