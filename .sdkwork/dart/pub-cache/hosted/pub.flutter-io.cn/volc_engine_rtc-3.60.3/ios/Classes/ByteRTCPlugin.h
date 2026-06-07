// Copyright (c) 2026 Beijing Volcano Engine Technology Ltd.

#import <Flutter/Flutter.h>
#import <VolcApiEngine/VeEngine.h>
#import <VolcApiEngine/VolcEventObserver.h>

@interface ByteRTCPlugin : NSObject<FlutterPlugin, EventObserver>
@property (nonatomic, strong) VolcApiEngine *apiEngine;
@property (nonatomic, strong) FlutterMethodChannel *channel;

+ (instancetype) getInstance;
@end
