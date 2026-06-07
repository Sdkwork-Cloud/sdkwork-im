// Copyright (c) 2026 Beijing Volcano Engine Technology Ltd.

//
//  BasicTransformer.m
//  volc_engine_rtc
//
//  Provides basic utility methods for safe JSON/NSDictionary parsing
//

#import "BasicTransformer.h"

@implementation BasicTransformer

+ (NSString *)getString:(nullable NSDictionary *)source 
                 forKey:(NSString *)key 
            withDefault:(NSString *)defaultValue {
    if (source == nil || key == nil) {
        return defaultValue;
    }
    
    id value = source[key];
    if (value == nil || [value isKindOfClass:[NSNull class]]) {
        return defaultValue;
    }
    
    if ([value isKindOfClass:[NSString class]]) {
        return (NSString *)value;
    }
    
    // Try to convert to string if it's a number
    if ([value isKindOfClass:[NSNumber class]]) {
        return [(NSNumber *)value stringValue];
    }
    
    return defaultValue;
}

+ (NSInteger)getInt:(nullable NSDictionary *)source 
             forKey:(NSString *)key 
        withDefault:(NSInteger)defaultValue {
    if (source == nil || key == nil) {
        return defaultValue;
    }
    
    id value = source[key];
    if (value == nil || [value isKindOfClass:[NSNull class]]) {
        return defaultValue;
    }
    
    if ([value isKindOfClass:[NSNumber class]]) {
        return [(NSNumber *)value integerValue];
    }
    
    if ([value isKindOfClass:[NSString class]]) {
        return [(NSString *)value integerValue];
    }
    
    return defaultValue;
}

+ (double)getDouble:(nullable NSDictionary *)source 
             forKey:(NSString *)key 
        withDefault:(double)defaultValue {
    if (source == nil || key == nil) {
        return defaultValue;
    }
    
    id value = source[key];
    if (value == nil || [value isKindOfClass:[NSNull class]]) {
        return defaultValue;
    }
    
    if ([value isKindOfClass:[NSNumber class]]) {
        return [(NSNumber *)value doubleValue];
    }
    
    if ([value isKindOfClass:[NSString class]]) {
        return [(NSString *)value doubleValue];
    }
    
    return defaultValue;
}

+ (BOOL)getBoolean:(nullable NSDictionary *)source 
            forKey:(NSString *)key 
       withDefault:(BOOL)defaultValue {
    if (source == nil || key == nil) {
        return defaultValue;
    }
    
    id value = source[key];
    if (value == nil || [value isKindOfClass:[NSNull class]]) {
        return defaultValue;
    }
    
    if ([value isKindOfClass:[NSNumber class]]) {
        return [(NSNumber *)value boolValue];
    }
    
    if ([value isKindOfClass:[NSString class]]) {
        NSString *strValue = [(NSString *)value lowercaseString];
        return [strValue isEqualToString:@"true"] || [strValue isEqualToString:@"1"];
    }
    
    return defaultValue;
}

+ (float)getFloat:(nullable NSDictionary *)source 
           forKey:(NSString *)key 
      withDefault:(float)defaultValue {
    if (source == nil || key == nil) {
        return defaultValue;
    }
    
    id value = source[key];
    if (value == nil || [value isKindOfClass:[NSNull class]]) {
        return defaultValue;
    }
    
    if ([value isKindOfClass:[NSNumber class]]) {
        return [(NSNumber *)value floatValue];
    }
    
    if ([value isKindOfClass:[NSString class]]) {
        return [(NSString *)value floatValue];
    }
    
    return defaultValue;
}

+ (nullable NSData *)getBytes:(nullable NSDictionary *)source 
                       forKey:(NSString *)key 
                  withDefault:(nullable NSData *)defaultValue {
    if (source == nil || key == nil) {
        return defaultValue;
    }
    
    id value = source[key];
    if (value == nil || [value isKindOfClass:[NSNull class]]) {
        return defaultValue;
    }
    
    if ([value isKindOfClass:[NSData class]]) {
        return (NSData *)value;
    }
    
    // Try to convert from base64 string
    if ([value isKindOfClass:[NSString class]]) {
        NSData *data = [[NSData alloc] initWithBase64EncodedString:(NSString *)value options:0];
        return data ?: defaultValue;
    }
    
    return defaultValue;
}

+ (NSInteger)getEnumByIndex:(nullable NSDictionary *)source 
                     forKey:(NSString *)key 
                 enumValues:(NSArray<NSNumber *> *)enumValues {
    if (source == nil || key == nil || enumValues == nil) {
        return -1;
    }
    
    NSInteger index = [self getInt:source forKey:key withDefault:-1];
    
    if (index >= 0 && index < enumValues.count) {
        return [enumValues[index] integerValue];
    }
    
    return -1;
}

+ (nullable NSArray *)getJSONArray:(nullable NSDictionary *)source 
                            forKey:(NSString *)key {
    if (source == nil || key == nil) {
        return nil;
    }
    
    id value = source[key];
    if (value == nil || [value isKindOfClass:[NSNull class]]) {
        return nil;
    }
    
    if ([value isKindOfClass:[NSArray class]]) {
        return (NSArray *)value;
    }
    
    return nil;
}

+ (nullable NSDictionary *)getJSONObject:(nullable NSDictionary *)source 
                                  forKey:(NSString *)key {
    if (source == nil || key == nil) {
        return nil;
    }
    
    id value = source[key];
    if (value == nil || [value isKindOfClass:[NSNull class]]) {
        return nil;
    }
    
    if ([value isKindOfClass:[NSDictionary class]]) {
        return (NSDictionary *)value;
    }
    
    return nil;
}

@end
